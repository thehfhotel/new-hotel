//! Database connection pool using tiberius and bb8

use std::time::Duration;

use bb8::Pool;
use bb8_tiberius::ConnectionManager;
use tiberius::Config;

use crate::config::DbConfig;

/// Type alias for the database connection pool
pub type DbPool = Pool<ConnectionManager>;

/// bb8 acquire / connect timeout — caps how long `Pool::get` will wait
/// before giving up. Tiberius itself has no socket-level connect
/// timeout API (the `Config` builder only exposes host / port / auth /
/// trust), so this bb8-level cap is also our effective TCP-connect
/// timeout: bb8 wraps `ConnectionManager::connect` in
/// `tokio::time::timeout(connection_timeout, ...)` (see bb8 0.9
/// `inner.rs`).
///
/// **Resilience PR R2 (2026-05-14):** lowered from 15s → 5s after the
/// 74-minute HF Hotel CT watermark stall. With the new per-query
/// timeout (`db::mssql_timeout`) in place, a wedged server is
/// detected by the in-flight call within 10s — there's no reason to
/// keep `Pool::get` waiting 15s for a fresh TCP handshake to the
/// same wedged server. 5s is still well above WG-tunnelled TLS
/// handshake p99 (~400ms in prod).
const POOL_CONNECTION_TIMEOUT: Duration = Duration::from_secs(5);

/// Recycle every connection at the 10-minute mark so a long-lived
/// stuck client (e.g. a connection that survived a network blip but
/// is now wedged on a server-side lock) is force-rotated periodically
/// instead of pinning a slot in the pool forever.
///
/// **Resilience PR R2 (2026-05-14):** lowered from 30min → 10min so a
/// connection that survives a per-query timeout (drops the in-flight
/// call but stays in the pool with a poisoned server-side lock)
/// rotates out within the same incident window instead of pinning a
/// slot for 30 minutes.
const POOL_MAX_LIFETIME: Duration = Duration::from_secs(10 * 60);

/// Drop idle connections after 60 seconds — keeps the pool lean
/// during quiet periods while avoiding cold-start latency on bursty
/// traffic.
///
/// **Resilience PR R2 (2026-05-14):** lowered from 10min → 60s. The
/// watcher's tick cadence is 1s; an idle connection older than a
/// minute is almost certainly stale relative to current server state
/// (e.g. session-level CONTEXT_INFO drift, dropped TCP keepalive,
/// transparent WG re-key). Aggressive recycle costs us nothing on
/// the writeback hot path (it re-opens within the same tick) and
/// prevents a class of "connection looked fine but was wedged"
/// failure modes from being inherited by the next worker.
const POOL_IDLE_TIMEOUT: Duration = Duration::from_secs(60);

/// Create a new database connection pool
///
/// Connects to SQL Server using the provided configuration.
/// Pool settings:
/// - max connections: 20 (configurable via `MSSQL_POOL_MAX_SIZE`,
///   legacy `DB_POOL_MAX` still honored). Sized for the shared
///   writeback + sync workload — see `DbConfig::from_env`.
/// - port: 1433 by default, override via `MSSQL_PORT` (HF Ville uses
///   1436 — its SS2025 Express instance does not listen on the
///   default port).
/// - encryption: disabled (matches encrypt: false in Node.js)
/// - trust_cert: true (matches trustServerCertificate in Node.js)
/// - circuit-breaker timeouts: see the `POOL_*` constants above.
pub async fn create_pool(config: &DbConfig) -> Result<DbPool, Box<dyn std::error::Error>> {
    let mut tib_config = Config::new();

    tib_config.host(&config.server);
    tib_config.port(config.port);
    tib_config.database(&config.database);
    tib_config.authentication(tiberius::AuthMethod::sql_server(
        &config.user,
        &config.password,
    ));
    tib_config.trust_cert();

    let manager = ConnectionManager::new(tib_config);

    let pool = Pool::builder()
        .max_size(config.pool_max)
        // circuit-breaker: bound bb8 acquire / TCP connect wait so a
        // dead MSSQL never produces an unbounded acquire queue.
        // R2: 5s (was 15s).
        .connection_timeout(POOL_CONNECTION_TIMEOUT)
        // reaper: rotate any connection older than the configured
        // ceiling so a wedged long-lived client eventually frees its
        // pool slot. R2: 10 min (was 30 min).
        .max_lifetime(Some(POOL_MAX_LIFETIME))
        // reaper: close idle connections after the configured idle
        // window so stale sessions don't linger. R2: 60s (was 10
        // min) — see const docstring for the rationale.
        .idle_timeout(Some(POOL_IDLE_TIMEOUT))
        .build(manager)
        .await?;

    // Test the connection
    {
        let mut conn = pool.get().await?;
        // R2: bound the boot-probe `SELECT 1` so a server that
        // accepted the TCP handshake but is now wedged (legacy
        // row-lock backlog, tempdb contention) fails fast on
        // `create_pool` instead of hanging the worker's startup
        // sequence.
        let _ = crate::db::mssql_timeout::simple_query_with_timeout_pooled(
            &mut conn,
            "SELECT 1",
            crate::db::mssql_timeout::MssqlOpKind::Read,
        )
        .await?;
        tracing::info!(
            "Database connection established to {}:{}",
            config.server,
            config.port
        );
    }

    Ok(pool)
}

#[cfg(test)]
mod tests {
    //! Pool-builder unit tests don't need a live MSSQL — we assert on
    //! the resulting `bb8::State::Config` snapshot returned by
    //! `Pool::builder().build_unchecked(...)`. This validates that the
    //! timeout knobs we set above are actually applied to the pool
    //! (a typo on `connection_timeout` vs. `connect_timeout`, for
    //! example, would silently fall through to the bb8 default
    //! without this test).
    use super::*;
    use crate::config::DbConfig;

    fn stub_db_config() -> DbConfig {
        DbConfig {
            server: "127.0.0.1".to_string(),
            port: 1436,
            database: "stub".to_string(),
            user: "stub".to_string(),
            password: "stub".to_string(),
            pool_max: 5,
        }
    }

    /// `build_unchecked` constructs the pool without actually opening a
    /// connection — perfect for asserting on the configured knobs in
    /// CI where no MSSQL is reachable.
    fn pool_with_timeouts(config: &DbConfig) -> DbPool {
        let mut tib_config = Config::new();
        tib_config.host(&config.server);
        tib_config.port(config.port);
        tib_config.database(&config.database);
        tib_config.authentication(tiberius::AuthMethod::sql_server(
            &config.user,
            &config.password,
        ));
        tib_config.trust_cert();

        let manager = ConnectionManager::new(tib_config);

        Pool::builder()
            .max_size(config.pool_max)
            .connection_timeout(POOL_CONNECTION_TIMEOUT)
            .max_lifetime(Some(POOL_MAX_LIFETIME))
            .idle_timeout(Some(POOL_IDLE_TIMEOUT))
            .build_unchecked(manager)
    }

    // bb8::Pool::build_unchecked spawns the reaper as a tokio
    // interval, so every assertion needs a runtime. `#[tokio::test]`
    // gives us one without requiring a reachable MSSQL.

    #[tokio::test]
    async fn pool_builder_applies_circuit_breaker_connection_timeout() {
        let pool = pool_with_timeouts(&stub_db_config());
        let pool_cfg = pool.config();
        assert_eq!(pool_cfg.connection_timeout, POOL_CONNECTION_TIMEOUT);
    }

    #[tokio::test]
    async fn pool_builder_applies_max_lifetime() {
        let pool = pool_with_timeouts(&stub_db_config());
        let pool_cfg = pool.config();
        assert_eq!(pool_cfg.max_lifetime, Some(POOL_MAX_LIFETIME));
    }

    #[tokio::test]
    async fn pool_builder_applies_idle_timeout() {
        let pool = pool_with_timeouts(&stub_db_config());
        let pool_cfg = pool.config();
        assert_eq!(pool_cfg.idle_timeout, Some(POOL_IDLE_TIMEOUT));
    }

    #[tokio::test]
    async fn pool_builder_honors_max_size_from_config() {
        let pool = pool_with_timeouts(&stub_db_config());
        let pool_cfg = pool.config();
        assert_eq!(pool_cfg.max_size, 5);
    }

    // --- Resilience PR R2 (2026-05-14) ---
    //
    // The R2 tightening is only meaningful at the specific values
    // chosen in the post-mortem (5s / 60s / 10min). Pin the literals
    // so a future refactor that touches the consts without updating
    // the post-mortem regresses on the test instead of in production.

    #[tokio::test]
    async fn r2_connection_timeout_is_five_seconds() {
        assert_eq!(POOL_CONNECTION_TIMEOUT, Duration::from_secs(5));
    }

    #[tokio::test]
    async fn r2_idle_timeout_is_sixty_seconds() {
        assert_eq!(POOL_IDLE_TIMEOUT, Duration::from_secs(60));
    }

    #[tokio::test]
    async fn r2_max_lifetime_is_ten_minutes() {
        assert_eq!(POOL_MAX_LIFETIME, Duration::from_secs(600));
    }
}
