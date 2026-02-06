//! Dual database connection pool for legacy and new_hotel databases

use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use tiberius::Config;

use crate::config::{AppConfig, DbConfig, NewDbConfig};

use super::DbPool;

/// Holds connection pools for both legacy and new_hotel databases
#[derive(Clone)]
pub struct DualDbPool {
    /// Connection pool for the legacy database (shared with legacy application)
    pub legacy: DbPool,
    /// Connection pool for the new_hotel database (owned by this application)
    pub new_hotel: DbPool,
}

impl DualDbPool {
    /// Create a new DualDbPool with the given pools
    pub fn new(legacy: DbPool, new_hotel: DbPool) -> Self {
        Self { legacy, new_hotel }
    }
}

/// Create a pool from NewDbConfig (public for standalone HotelNew usage)
pub async fn create_new_pool(config: &NewDbConfig) -> Result<DbPool, Box<dyn std::error::Error>> {
    let mut tib_config = Config::new();

    tib_config.host(&config.server);
    tib_config.port(1433);
    tib_config.database(&config.database);
    tib_config.authentication(tiberius::AuthMethod::sql_server(
        &config.user,
        &config.password,
    ));
    tib_config.trust_cert();

    let manager = ConnectionManager::new(tib_config);

    let pool = Pool::builder()
        .max_size(config.pool_max)
        .build(manager)
        .await?;

    // Test the connection
    {
        let mut conn = pool.get().await?;
        let _ = conn.simple_query("SELECT 1").await?;
    }

    Ok(pool)
}

/// Create dual database connection pools from application configuration
///
/// This creates two separate connection pools:
/// - `legacy`: Connects to the legacy database (from DB_* env vars)
/// - `new_hotel`: Connects to the HotelNew database (from NEW_DB_* env vars)
pub async fn create_dual_pool(config: &AppConfig) -> Result<DualDbPool, Box<dyn std::error::Error>> {
    use super::pool::create_pool;

    // Create legacy pool using the existing db config
    let legacy = create_pool(&config.db).await?;
    tracing::info!("Legacy database pool created for {}", config.db.database);

    // Create new_hotel pool using the new_db config
    let new_hotel = create_new_pool(&config.new_db).await?;
    tracing::info!("New hotel database pool created for {}", config.new_db.database);

    Ok(DualDbPool::new(legacy, new_hotel))
}
