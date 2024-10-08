use futures::TryStreamExt;
use noodles_vcf::{
    self as vcf,
    variant::record::info::{self, field::Value as InfoValue},
};
use sqlx::SqlitePool;
use std::path::PathBuf;
use std::time::Instant;
use tokio::{fs::File as TkFile, io::BufReader};
use crate::sqlite::{DbRow, setup_db, get_db_connection};

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

fn get_vrs_ids(
    info: vcf::record::Info,
    header: &vcf::Header,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if let Some(Ok(Some(InfoValue::Array(array)))) = info.get(header, "VRS_Allele_IDs") {
        if let info::field::value::Array::String(array_elements) = array {
            let vec = array_elements
                .iter()
                .map(|cow_str| cow_str.unwrap().unwrap_or_default().to_string())
                .collect();
            return Ok(vec);
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Expected string Array variant",
            )))
        }
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Expected Array variant",
        )))
    }
}

pub async fn vcf_to_sqlite(vcf_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    setup_db().await?;

    let mut reader = TkFile::open(vcf_path)
        .await
        .map(BufReader::new)
        .map(vcf::r#async::io::Reader::new)?;
    let header = reader.read_header().await?;

    let mut records = reader.records();

    let db_pool = get_db_connection().await?;

    while let Some(record) = records.try_next().await? {
        let vrs_ids = get_vrs_ids(record.info(), &header).unwrap();
        let chrom = record.reference_sequence_name();
        let pos = record.variant_start().unwrap()?.get();

        for vrs_id in vrs_ids {
            let row = DbRow {
                vrs_id: vrs_id.strip_prefix("ga4gh:VA.").unwrap_or(&vrs_id).to_string(),
                chr: chrom.strip_prefix("chr").unwrap_or(chrom).to_string(),
                pos: pos.try_into().unwrap(),
            };
            load_allele(row, &db_pool).await?;
        }
    }

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
    Ok(())
}
