//! PostgreSQL connection pool using sqlx for HotelNew database

use std::time::Duration;

use crate::config::NewDbConfig;

/// Type alias for the PostgreSQL connection pool
pub type PgPool = sqlx::PgPool;

/// How long a caller waits for a free pool connection before giving up.
///
/// sqlx's default is **30 seconds**, and it was never overridden — which is
/// what turned the 2026-07-29 SSE incident from a slowdown into an outage.
/// Every open /v2 tab held 1–2 real pool slots for the life of its
/// `EventSource` (`PgListener::connect_with(pool)`), so a few tabs exhausted
/// the 10-slot hotelnew / 5-slot hotelville pools. Saturation then meant:
/// `/api/stats|checkins|bookings` blocked the full 30s before returning
/// `PoolTimedOut` (→ HTTP 500), and the SSE handler's serial auth + two
/// listener acquires ran ~90s **with zero bytes written**, long enough for
/// Cloudflare to 524 the stream and for `EventSource` to reconnect onto the
/// same starved pool.
///
/// `routes::events` removed the cause (listeners are now standalone
/// connections shared via a broadcast fan-out). This bound is the backstop: a
/// future saturation fails fast and loudly instead of hanging silently for
/// half a minute. Deliberately a plain constant — a pool-acquire ceiling is a
/// safety property, not a per-environment tuning knob.
pub const PG_ACQUIRE_TIMEOUT: Duration = Duration::from_secs(5);

/// Shared `PgPoolOptions` for every canonical PostgreSQL pool this process
/// builds — HotelNew via [`create_pg_pool`], HF Ville inline in `main.rs`.
///
/// Exists so the acquire bound can't be applied to one pool and forgotten on
/// the other (the Ville pool is built separately because `VilleDbConfig` is a
/// distinct type).
pub fn pg_pool_options(max_connections: u32) -> sqlx::postgres::PgPoolOptions {
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(PG_ACQUIRE_TIMEOUT)
}

/// Create a new PostgreSQL connection pool
pub async fn create_pg_pool(config: &NewDbConfig) -> Result<PgPool, Box<dyn std::error::Error>> {
    let conn_str = config.connection_string();

    let pool = pg_pool_options(config.pool_max).connect(&conn_str).await?;

    // Test the connection
    sqlx::query_scalar!("SELECT 1 as test").fetch_one(&pool).await?;
    tracing::info!(
        "PostgreSQL connection established to {}:{}",
        config.server,
        config.port
    );

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both canonical pools must carry the explicit acquire bound. Without it
    /// sqlx silently uses 30s, which is the difference between a fast logged
    /// `PoolTimedOut` and the 30s-hang → 500 → CF-524 spiral of 2026-07-29.
    #[test]
    fn pool_options_bound_the_acquire_wait() {
        let options = pg_pool_options(20);
        assert_eq!(options.get_acquire_timeout(), PG_ACQUIRE_TIMEOUT);
        assert_eq!(options.get_max_connections(), 20);
        assert!(
            options.get_acquire_timeout() < Duration::from_secs(30),
            "must be tighter than sqlx's 30s default, or the bound is pointless",
        );
    }

    /// The caller's `pool_max` is passed through untouched — compose sets
    /// `NEW_DB_POOL_MAX` / `VILLE_DB_POOL_MAX` and config.rs defaults to 10/5.
    #[test]
    fn pool_options_respect_the_configured_max() {
        assert_eq!(pg_pool_options(5).get_max_connections(), 5);
        assert_eq!(pg_pool_options(10).get_max_connections(), 10);
    }
}
