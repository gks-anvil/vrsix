use log::info;
use sqlx::{migrate::MigrateDatabase, Error, Sqlite, SqlitePool};

pub async fn get_db_connection(db_url: &str) -> Result<SqlitePool, Error> {
    let db_pool = SqlitePool::connect(db_url).await?;
    Ok(db_pool)
}

pub async fn setup_db(db_url: &str) -> Result<(), Error> {
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        info!("Creating DB {}", db_url);
        match Sqlite::create_database(db_url).await {
            Ok(_) => info!("Created DB"),
            Err(error) => return Err(error),
        }
    } else {
        info!("DB exists")
    }

    let db = get_db_connection(db_url).await?;
    let result = sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS vrs_locations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            vrs_id TEXT NOT NULL,
            chr TEXT NOT NULL,
            pos INTEGER NOT NULL
        );",
    )
    .execute(&db)
    .await?;
    info!("created table result: {:?}", result);
    Ok(())
}

#[derive(Debug)]
pub struct DbRow {
    pub vrs_id: String,
    pub chr: String,
    pub pos: i64,
}
