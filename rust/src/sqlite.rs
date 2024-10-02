// TODO
// * double check that the is_empty function helps at all in light of ^
// * parallelism: https://github.com/zaeleus/noodles/issues/85
use futures::TryStreamExt;
use noodles_vcf::{
    self as vcf,
    variant::record::info::{self, field::Value as InfoValue},
};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::path::PathBuf;
use std::time::Instant;
use tokio::{fs::File as TkFile, io::BufReader};

const DB_URL: &str = "sqlite://sqlite.db";

async fn setup_db() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating DB {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Created DB"),
            Err(error) => panic!("Error: {}", error),
        }
    } else {
        println!("DB exists")
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let result = sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS vrs_locations (
            vrs_id VARCHAR(36)
            PRIMARY KEY NOT NULL,
            chr VARCHAR(5) NOT NULL,
            pos INTEGER NOT NULL
        );",
    )
    .execute(&db)
    .await
    .unwrap();
    println!("created table result: {:?}", result);
}

struct DbRow {
    vrs_id: String,
    chr: String,
    pos: i64,
}

async fn load_allele(db_row: DbRow, pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = pool.acquire().await?;
    sqlx::query!(
        r#"INSERT INTO vrs_locations (vrs_id, chr, pos) VALUES (?1, ?2, ?3)"#,
        db_row.vrs_id,
        db_row.chr,
        db_row.pos
    )
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

    setup_db().await;

    let mut reader = TkFile::open(vcf_path)
        .await
        .map(BufReader::new)
        .map(vcf::r#async::io::Reader::new)?;
    let header = reader.read_header().await?;

    let mut records = reader.records();

    let db_pool = SqlitePool::connect(DB_URL).await?;

    while let Some(record) = records.try_next().await? {
        let vrs_ids = get_vrs_ids(record.info(), &header).unwrap();
        let chrom = record.reference_sequence_name();
        let pos = record.variant_start().unwrap()?.get();

        for vrs_id in vrs_ids {
            let row = DbRow {
                vrs_id,
                chr: String::from(chrom),
                pos: pos.try_into().unwrap(),
            };
            load_allele(row, &db_pool).await?;
        }
    }

    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
    Ok(())
}
