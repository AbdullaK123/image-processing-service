use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env::var;
use std::time::Duration;
use anyhow::Result;

pub async fn create_pool() -> Result<PgPool> {
    let db_url = var("DATABASE_URL").expect("Database URI must be set");

    let pool =
        PgPoolOptions::new()
            .max_connections(10)
            .min_connections(1)
            .acquire_timeout(Duration::from_secs(10))
            .idle_timeout(Duration::from_secs(60))
            .max_lifetime(Duration::from_secs(1800))
            .connect(&db_url)
            .await?;

    Ok(pool)
}