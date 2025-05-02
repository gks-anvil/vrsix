use crate::parse::{get_int_info_field, get_str_info_field, VrsVcfField};
use crate::sqlite::{get_db_connection, setup_db, DbRow};
use crate::{FiletypeError, SqliteFileError, VcfError, VrsixDbError, VRSIX_SCHEMA_VERSION};
use futures::TryStreamExt;
use itertools::izip;
use log::{error, info};
use noodles_bgzf::r#async::Reader as BgzfReader;
use noodles_vcf as vcf;
use noodles_vcf::r#async::io::Reader as VcfReader;
use pyo3::{exceptions, prelude::*};
use regex::Regex;
use sqlx::{
    error::DatabaseError, error::Error as SqlxError, sqlite::SqliteError, sqlite::SqliteRow, Row,
    SqlitePool,
};
use std::path::PathBuf;
use std::time::Instant;
use tokio::{
    fs::File as TkFile,
    io::{AsyncBufRead, BufReader},
};

async fn load_allele(db_row: DbRow, pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.acquire().await?;
    let result =
        sqlx::query("INSERT INTO vrs_locations (vrs_id, chr, pos, vrs_start, vrs_end, vrs_state, uri_id) VALUES (?, ?, ?, ?, ?, ?, ?);")
            .bind(db_row.vrs_id)
            .bind(db_row.chr)
            .bind(db_row.pos)
            .bind(db_row.vrs_start)
            .bind(db_row.vrs_end)
            .bind(db_row.vrs_state)
            .bind(db_row.uri_id)
            .execute(&mut *conn)
            .await;
    if let Err(err) = result {
        if let Some(db_error) = err.as_database_error() {
            if let Some(sqlite_error) = db_error.try_downcast_ref::<SqliteError>() {
                if sqlite_error
                    .code()
                    .map(|code| code == "2067")
                    .unwrap_or(false)
                {
                    error!("duplicate");
                    return Ok(());
                }
            }
        }
        return Err(err.into());
    }
    Ok(())
}

async fn get_reader(
    vcf_path: PathBuf,
) -> Result<VcfReader<Box<dyn tokio::io::AsyncBufRead + Unpin + Send>>, PyErr> {
    let file = TkFile::open(vcf_path.clone()).await.map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyOSError, _>(format!("Failed to open file: {}", e))
    })?;
    let ext = vcf_path.extension().and_then(|ext| ext.to_str());
    match ext {
        Some("gz") => {
            let reader = Box::new(BgzfReader::new(file)) as Box<dyn AsyncBufRead + Unpin + Send>;
            Ok(VcfReader::new(reader))
        }
        Some("vcf") => {
            let reader = Box::new(BufReader::new(file)) as Box<dyn AsyncBufRead + Unpin + Send>;
            Ok(VcfReader::new(reader))
        }
        _ => {
            error!(
                "Unexpected file extension `{:?}` for input file `{:?}`",
                ext, vcf_path
            );
            Err(PyErr::new::<FiletypeError, _>(format!(
                "Unsupported file extension: {:?}",
                ext
            )))
        }
    }
}

/// Populate the `file_uris` table with a row corresponding to the incoming file.
///
/// VRS/VRS Python version columns are nullable, so those options are just passed as
/// null if they don't exist
async fn load_file_uri(
    uri: &str,
    version_info: VcfVrsVersion,
    pool: &SqlitePool,
) -> Result<i64, Box<dyn std::error::Error>> {
    let mut conn = pool.acquire().await?;

    let insert_result = sqlx::query(
        "INSERT OR IGNORE INTO file_uris (uri, vrs_version, vrs_python_version) VALUES (?, ?, ?);",
    )
    .bind(uri)
    .bind(version_info.vrs_version)
    .bind(version_info.vrs_python_version)
    .execute(&mut *conn)
    .await?;
    if insert_result.rows_affected() > 0 {
        Ok(insert_result.last_insert_rowid())
    } else {
        // if, for whatever reason, the file already existed, we go back and fetch the
        // URI ID. We could also throw an error if we think that'd be better.
        let row_id: (i64,) = sqlx::query_as("SELECT id FROM file_uris WHERE uri = ?;")
            .bind(uri)
            .fetch_one(&mut *conn)
            .await?;
        Ok(row_id.0)
    }
}

/// Output from schema lookup query
#[derive(Debug)]
struct SchemaResult {
    pub vrsix_schema_version: String,
}

impl From<SqliteRow> for SchemaResult {
    fn from(row: SqliteRow) -> Self {
        let version: String = row.get("vrsix_schema_version");
        SchemaResult {
            vrsix_schema_version: version,
        }
    }
}

/// Check whether VRSIX schema in provided file matches the vrsix library schema value
///
/// The library hard-codes a schema version value. This is currently pretty naive, just a
/// simple string value. If the value differs from what's in the vrsix DB, this function
/// emits a warning and returns false.
async fn schema_matches_library(pool: &SqlitePool, file_uri: &str) -> Result<bool, SqlxError> {
    let result: SchemaResult = sqlx::query("SELECT vrsix_schema_version FROM vrsix_schema;")
        .fetch_one(pool)
        .await?
        .into();
    if result.vrsix_schema_version != VRSIX_SCHEMA_VERSION {
        error!("vrsix schema in {} is {}; differs from vrsix library schema {}. Import may be unsuccessful, migration is recommended.", file_uri, result.vrsix_schema_version, VRSIX_SCHEMA_VERSION);
        Ok(false)
    } else {
        Ok(true)
    }
}

/// Description of VRS schema/library versioning taken from annotated VCF.
struct VcfVrsVersion {
    vrs_version: Option<String>,
    vrs_python_version: Option<String>,
}

/// Extract VRS schema/library versioning from VCF.
///
/// If VCF doesn't have this info (i.e. it was made from an older VRS-Python release)
/// then it'll return a struct with None values. Otherwise, it should just pull
/// them out as they're given.
fn get_vrs_version(header: &vcf::Header) -> Result<VcfVrsVersion, VcfError> {
    let description = header.infos().get("VRS_Allele_IDs").unwrap().description();
    let re = Regex::new(r"\[VRS version=(.*);VRS-Python version=(.*)\]").unwrap();
    match re.captures(description) {
        Some(caps) => {
            let vrs_version = caps.get(1).map(|m| m.as_str().to_string());
            let vrs_python_version = caps.get(2).map(|m| m.as_str().to_string());
            Ok(VcfVrsVersion {
                vrs_version,
                vrs_python_version,
            })
        }
        None => Ok(VcfVrsVersion {
            vrs_version: None,
            vrs_python_version: None,
        }),
    }
}

/// Load a VRS-annotated VCF into the given VRSIX db.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// let vcf_path = PathBuf::from(r"path/to/my/vcf.vcf.gz");
/// let db_url = String::from("file:///usr/local/share/index.db");
/// let uri = db_url.to_string();
/// let _ = load_vcf(vcf_path, &db_url, uri_value)).await?;
/// ```
pub async fn load_vcf(vcf_path: PathBuf, db_url: &str, uri: String) -> PyResult<()> {
    let start = Instant::now();

    if !vcf_path.exists() || !vcf_path.is_file() {
        error!("Input file `{:?}` does not appear to exist", vcf_path);
        return Err(exceptions::PyFileNotFoundError::new_err(
            "Input path does not lead to an existing file",
        ));
    }

    setup_db(db_url).await.map_err(|_| {
        error!("Unable to open input file `{:?}` into sqlite", db_url);
        SqliteFileError::new_err("Unable to open DB file -- is it a valid sqlite file?")
    })?;

    let mut reader = get_reader(vcf_path).await?;
    let header = reader.read_header().await?;

    let mut records = reader.records();

    let db_pool = get_db_connection(db_url).await.map_err(|e| {
        error!("DB connection failed: {}", e);
        VrsixDbError::new_err(format!("Failed database connection/call: {}", e))
    })?;

    // TODO: see #43 for details on extra row written on vrsix schema mismatch
    if !schema_matches_library(&db_pool, &db_url).await.map_err(|_| SqliteFileError::new_err(format!("Unable to get VRSIX schema version from {} -- this might indicate a schema mismatch", &db_url)))? {
        return Err(SqliteFileError::new_err(format!("Found schema mismatch between VRSIX library and {}", &db_url)))
    };

    let file_vrs_versioning = get_vrs_version(&header)
        .map_err(|_| {
            VcfError::new_err(
                "Failed to parse VRS versioning from VCF allele IDs INFO field description",
            )
        })
        .unwrap();
    let uri_id = load_file_uri(&uri, file_vrs_versioning, &db_pool)
        .await
        .map_err(|e| VrsixDbError::new_err(format!("Failed to insert file URI `{uri}`: {e}")))?;

    while let Some(record) = records.try_next().await? {
        let vrs_ids = get_str_info_field(record.info(), &header, VrsVcfField::VrsAlleleIds)?;
        let vrs_starts = get_int_info_field(record.info(), &header, VrsVcfField::VrsStarts)?;
        let vrs_ends = get_int_info_field(record.info(), &header, VrsVcfField::VrsEnds)?;
        let vrs_states = get_str_info_field(record.info(), &header, VrsVcfField::VrsStates)?;
        let chrom = record.reference_sequence_name();
        let pos = record.variant_start().unwrap()?.get();

        for (vrs_id, vrs_start, vrs_end, vrs_state) in
            izip!(vrs_ids, vrs_starts, vrs_ends, vrs_states)
        {
            let row = DbRow {
                vrs_id: vrs_id
                    .strip_prefix("ga4gh:VA.")
                    .unwrap_or(&vrs_id)
                    .to_string(),
                chr: chrom.to_string(),
                pos: pos.try_into().unwrap(),
                vrs_start,
                vrs_end,
                vrs_state,
                uri_id,
            };
            load_allele(row, &db_pool).await.map_err(|e| {
                error!("Failed to load row {:?}", e);
                VrsixDbError::new_err(format!("Failed to load row: {}", e))
            })?;
        }
    }

    let duration = start.elapsed();
    info!("Time taken: {:?}", duration);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_load_file_uri() {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let db_url = format!("sqlite://{}", temp_file.path().to_str().unwrap());
        crate::sqlite::setup_db(&db_url).await.unwrap();
        let db_pool = get_db_connection(&db_url).await.unwrap();
        let versions = VcfVrsVersion {
            vrs_version: None,
            vrs_python_version: None,
        };
        let uri_id = load_file_uri("file:///arbitrary/file/location.vcf", versions, &db_pool)
            .await
            .unwrap();
        assert!(uri_id == 1);
    }
}
