use crate::sqlite::{get_db_connection, setup_db, DbRow};
use crate::{FiletypeError, SqliteFileError, VcfError, VrsixDbError};
use futures::TryStreamExt;
use noodles_bgzf::r#async::Reader as BgzfReader;
use noodles_vcf::{
    self as vcf,
    r#async::io::Reader as VcfReader,
    variant::record::info::{self, field::Value as InfoValue},
};
use pyo3::{exceptions, prelude::*};
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::time::Instant;
use tokio::{
    fs::File as TkFile,
    io::{AsyncBufRead, BufReader},
};

async fn load_allele(db_row: DbRow, pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.acquire().await?;
    sqlx::query("INSERT INTO vrs_locations (vrs_id, chr, pos) VALUES (?, ?, ?)")
        .bind(db_row.vrs_id)
        .bind(db_row.chr)
        .bind(db_row.pos)
        .execute(&mut *conn)
        .await?;
    Ok(())
}

fn get_vrs_ids(info: vcf::record::Info, header: &vcf::Header) -> Result<Vec<String>, PyErr> {
    if let Some(Ok(Some(InfoValue::Array(array)))) = info.get(header, "VRS_Allele_IDs") {
        if let info::field::value::Array::String(array_elements) = array {
            let vec = array_elements
                .iter()
                .map(|cow_str| cow_str.unwrap().unwrap_or_default().to_string())
                .collect();
            return Ok(vec);
        } else {
            Err(VcfError::new_err("expected string array variant"))
        }
    } else {
        Err(VcfError::new_err("Expected Array variant"))
    }
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
        _ => Err(PyErr::new::<FiletypeError, _>(format!(
            "Unsupported file extension: {:?}",
            ext
        ))),
    }
}

pub async fn load_vcf(vcf_path: PathBuf, db_url: &str) -> PyResult<()> {
    let start = Instant::now();

    if !vcf_path.exists() || !vcf_path.is_file() {
        return Err(exceptions::PyFileNotFoundError::new_err(
            "Input path does not lead to an existing file",
        ));
    }

    setup_db(db_url).await.map_err(|_| {
        SqliteFileError::new_err("Unable to open DB file -- is it a valid sqlite file?")
    })?;

    let mut reader = get_reader(vcf_path).await?;
    let header = reader.read_header().await?;

    let mut records = reader.records();

    let db_pool = get_db_connection(db_url)
        .await
        .map_err(|e| VrsixDbError::new_err(format!("Failed database connection/call: {}", e)))?;

    while let Some(record) = records.try_next().await? {
        let vrs_ids = get_vrs_ids(record.info(), &header)?;
        let chrom = record.reference_sequence_name();
        let pos = record.variant_start().unwrap()?.get();

        for vrs_id in vrs_ids {
            let row = DbRow {
                vrs_id: vrs_id
                    .strip_prefix("ga4gh:VA.")
                    .unwrap_or(&vrs_id)
                    .to_string(),
                chr: chrom.strip_prefix("chr").unwrap_or(chrom).to_string(),
                pos: pos.try_into().unwrap(),
            };
            load_allele(row, &db_pool)
                .await
                .map_err(|e| VrsixDbError::new_err(format!("Failed to load row: {}", e)))?;
        }
    }

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
    Ok(())
}
