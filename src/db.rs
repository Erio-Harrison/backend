use crate::config::Config;
use crate::errors::AppError;
use sqlx::{PgPool, Row};
use sqlx::postgres::PgPoolOptions;

// SQLx connection pool
pub type DbPool = PgPool;

pub async fn init_db(config: &Config) -> Result<DbPool, AppError> {
    let pool = PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .min_connections(config.database_min_connections)
        .connect(&config.database_url)
        .await
        .map_err(|e| AppError::DatabaseError(format!("PostgreSQL connection failed: {}", e)))?;

    let row = sqlx::query("SELECT 1 as test")
        .fetch_one(&pool)
        .await
        .map_err(|e| AppError::DatabaseError(format!("PostgreSQL connection check failed: {}", e)))?;

    let test_value: i32 = row.get("test");
    if test_value == 1 {
        log::info!("PostgreSQL connected successfully");
    }

    Ok(pool)
}