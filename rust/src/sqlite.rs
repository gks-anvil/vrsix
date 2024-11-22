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
        CREATE TABLE IF NOT EXISTS file_uris (
            id INTEGER PRIMARY KEY,
            uri TEXT UNIQUE
        );
        CREATE TABLE IF NOT EXISTS vrs_locations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            vrs_id TEXT NOT NULL,
            chr TEXT NOT NULL,
            pos INTEGER NOT NULL,
            uri_id INTEGER NOT NULL,
            FOREIGN KEY (uri_id) REFERENCES file_uris(id),
            UNIQUE(vrs_id, chr, pos, uri_id)
        );
        ",
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
    pub uri_id: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_setup_db() {
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let db_url = format!("sqlite://{}", temp_file.path().to_str().unwrap());
        setup_db(&db_url).await.expect("Setup DB failed");
    }
}
