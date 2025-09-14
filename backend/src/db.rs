// backend/src/db.rs

use sqlx::{MySql, Pool};

pub type DbPool = Pool<MySql>;

pub async fn init_db() -> Result<DbPool, sqlx::Error> {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or("mysql://smrt:smrtpass@localhost/smrt_mcp".to_string());
    sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
}
