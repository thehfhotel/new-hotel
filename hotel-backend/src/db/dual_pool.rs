//! Dual database connection pool for legacy and new_hotel databases

use crate::config::AppConfig;

use super::{DbPool, PgPool};

/// Holds connection pools for both legacy and new_hotel databases
#[derive(Clone)]
pub struct DualDbPool {
    /// Connection pool for the legacy database (SQL Server, shared with legacy application)
    pub legacy: DbPool,
    /// Connection pool for the new_hotel database (PostgreSQL, owned by this application)
    pub new_hotel: PgPool,
}

impl DualDbPool {
    /// Create a new DualDbPool with the given pools
    pub fn new(legacy: DbPool, new_hotel: PgPool) -> Self {
        Self { legacy, new_hotel }
    }
}

/// Create dual database connection pools from application configuration
pub async fn create_dual_pool(config: &AppConfig) -> Result<DualDbPool, Box<dyn std::error::Error>> {
    use super::pool::create_pool;
    use super::pg_pool::create_pg_pool;

    // Create legacy pool using the existing db config
    let legacy = create_pool(&config.db).await?;
    tracing::info!("Legacy database pool created for {}", config.db.database);

    // Create new_hotel pool using PostgreSQL
    let new_hotel = create_pg_pool(&config.new_db).await?;
    tracing::info!("New hotel database pool created for {}", config.new_db.database);

    Ok(DualDbPool::new(legacy, new_hotel))
}
