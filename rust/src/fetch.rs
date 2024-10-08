use crate::sqlite::{get_db_connection, DbRow};
use sqlx::Row;

pub async fn fetch_by_vrs_id(vrs_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let db = get_db_connection().await?;

    let result = sqlx::query(
        r#"
        SELECT vrs_id, chr, pos
        FROM vrs_locations
        WHERE vrs_id = ?
        "#,
    )
    .bind(vrs_id.clone())
    .fetch_one(&db)
    .await?;
    let result = DbRow {
        vrs_id: result.get("vrs_id"),
        chr: result.get("chr"),
        pos: result.get("pos"),
    };
    println!("{:?}", result);
    Ok(())
}

pub async fn fetch_by_range(
    chr: String,
    start: i64,
    end: i64,
) -> Result<Vec<DbRow>, Box<dyn std::error::Error>> {
    println!("{:?}, {:?}, {:?}", chr, start, end);
    let db = get_db_connection().await?;
    let result = sqlx::query(
        r#"
        SELECT vrs_id, chr, pos
        FROM vrs_locations
        WHERE chr = ? AND pos BETWEEN ? AND ?
        "#,
    )
    .bind(chr.clone())
    .bind(start)
    .bind(end)
    .fetch_all(&db)
    .await?;

    let rows: Vec<DbRow> = result
        .into_iter()
        .map(|r| DbRow {
            vrs_id: r.get("vrs_id"),
            chr: r.get("chr"),
            pos: r.get("pos"),
        })
        .collect();

    println!("{:?}", rows);
    Ok(rows)
}
