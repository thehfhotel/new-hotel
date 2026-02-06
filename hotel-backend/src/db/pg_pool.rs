//! PostgreSQL connection pool using sqlx for HotelNew database

use crate::config::NewDbConfig;

/// Type alias for the PostgreSQL connection pool
pub type PgPool = sqlx::PgPool;

/// Create a new PostgreSQL connection pool
pub async fn create_pg_pool(config: &NewDbConfig) -> Result<PgPool, Box<dyn std::error::Error>> {
    let conn_str = config.connection_string();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.pool_max)
        .connect(&conn_str)
        .await?;

    // Test the connection
    sqlx::query_scalar!("SELECT 1 as test").fetch_one(&pool).await?;
    tracing::info!(
        "PostgreSQL connection established to {}:{}",
        config.server,
        config.port
    );

    Ok(pool)
}
