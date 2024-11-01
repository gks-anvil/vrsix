use crate::sqlite::{get_db_connection, setup_db, DbRow};
use crate::{SqliteFileError, VcfError, VrsixDbError};
use futures::TryStreamExt;
use noodles_vcf::{
    self as vcf,
    variant::record::info::{self, field::Value as InfoValue},
};
use pyo3::{exceptions, prelude::*};
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::time::Instant;
use tokio::{fs::File as TkFile, io::BufReader};

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

pub async fn load_vcf(vcf_path: PathBuf, db_url: &str) -> PyResult<()> {
    let start = Instant::now();

    if !vcf_path.exists() || !vcf_path.is_file() {
        return Err(exceptions::PyFileNotFoundError::new_err(
            "Input path does not lead to an exist",
        ));
    }

    setup_db(db_url).await.map_err(|_| {
        SqliteFileError::new_err("Unable to open DB file -- is it a valid sqlite file?")
    })?;

    let mut reader = TkFile::open(vcf_path)
        .await
        .map(BufReader::new)
        .map(vcf::r#async::io::Reader::new)?;
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
