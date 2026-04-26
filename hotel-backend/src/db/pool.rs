//! Database connection pool using tiberius and bb8

use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use tiberius::Config;

use crate::config::DbConfig;

/// Type alias for the database connection pool
pub type DbPool = Pool<ConnectionManager>;

/// Create a new database connection pool
///
/// Connects to SQL Server using the provided configuration.
/// Pool settings:
/// - max connections: 20 (configurable via `MSSQL_POOL_MAX_SIZE`,
///   legacy `DB_POOL_MAX` still honored). Sized for the shared
///   writeback + sync + ville-sync workload — see
///   `DbConfig::from_env`.
/// - encryption: disabled (matches encrypt: false in Node.js)
/// - trust_cert: true (matches trustServerCertificate in Node.js)
pub async fn create_pool(config: &DbConfig) -> Result<DbPool, Box<dyn std::error::Error>> {
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
        tracing::info!(
            "Database connection established to {}",
            config.server
        );
    }

    Ok(pool)
}
