use sqlx::{migrate::MigrateDatabase, Error, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

pub async fn get_db_connection() -> Result<SqlitePool, Error> {
    let db_pool = SqlitePool::connect(DB_URL).await?;
    Ok(db_pool)
}

pub async fn setup_db() -> Result<(), Error> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating DB {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Created DB"),
            Err(error) => panic!("Error: {}", error),
        }
    } else {
        println!("DB exists")
    }

    let db = get_db_connection().await?;
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
    .await
    .unwrap();
    println!("created table result: {:?}", result);
    Ok(())
}

#[derive(Debug)]
pub struct DbRow {
    pub vrs_id: String,
    pub chr: String,
    pub pos: i64,
}
