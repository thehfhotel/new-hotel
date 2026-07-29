//! Database connection pool using tiberius and bb8

use std::ops::{Deref, DerefMut};
use std::time::Duration;

use bb8::{ManageConnection, Pool};
use bb8_tiberius::ConnectionManager as TiberiusManager;
use tiberius::Config;

use crate::config::DbConfig;

/// Type alias for the database connection pool
pub type DbPool = Pool<PoisonAwareManager>;

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

/// Wraps a pooled tiberius connection with a `poisoned` flag so a
/// per-operation timeout (`db::mssql_timeout`) can mark the connection
/// unusable instead of letting it silently return to the pool.
///
/// ## Why this exists (issue #274)
///
/// `tokio::time::timeout` around a `Client::simple_query` future drops
/// the future — including any partially-read TDS result-set — the
/// instant the budget elapses. The underlying TCP stream is left
/// mid-frame: whatever bytes the server was about to send for the
/// aborted query are still queued on the wire, so the *next* call on
/// the same connection reads garbage where it expects a fresh
/// response header.
///
/// `bb8-tiberius-0.16.0`'s `ManageConnection::has_broken` hardcodes
/// `false` (it has no way to know), so a desynced connection returns
/// to the idle queue looking healthy. bb8's `test_on_check_out` then
/// runs an unbounded `SELECT 1` on every checkout; against a desynced
/// stream that read hangs until bb8's `connection_timeout` (5s —
/// `POOL_CONNECTION_TIMEOUT` above) — producing the observed `Timed
/// out in bb8` bursts. Only the 10-minute `max_lifetime` reaper
/// eventually retires the connection, so one poisoned connection can
/// degrade a pool slot for up to 10 minutes.
///
/// `PoisonableConnection` closes that gap: `db::mssql_timeout` calls
/// [`mark_poisoned`](Self::mark_poisoned) in the timeout arm of every
/// `tokio::time::timeout` it wraps, and [`PoisonAwareManager`] reports
/// the flag back through `has_broken` — bb8 then closes the
/// connection on release (see bb8's `inner.rs::put_back`) instead of
/// re-queuing it, and the next `Pool::get` opens a fresh one.
///
/// Generic over the wrapped connection type `C` so the flag/Deref
/// mechanics are unit-testable without a live tiberius `Client` (see
/// the tests below) — `PoisonAwareManager` below fixes `C` to
/// `bb8_tiberius::rt::Client`.
///
/// `Deref`/`DerefMut` to `C` mean every existing call site
/// (`conn.simple_query(..)`, `conn.execute(..)`, …) needs no code
/// change — only the type-alias declarations that name the pool's
/// connection-manager type do (`LegacyConn` in `db::mssql_timeout`,
/// `db::mssql_session`, `writeback::allocate`, plus the couple of
/// one-shot `bin/*` admin binaries that spell out the pool type
/// explicitly instead of going through `db::DbPool`).
pub struct PoisonableConnection<C> {
    inner: C,
    poisoned: bool,
}

impl<C> PoisonableConnection<C> {
    fn new(inner: C) -> Self {
        Self {
            inner,
            poisoned: false,
        }
    }

    /// Mark this connection unusable. Call immediately after a
    /// `tokio::time::timeout` elapses on any wire call issued through
    /// it — see the struct docs for why the connection can't be
    /// trusted after that point. Idempotent.
    pub fn mark_poisoned(&mut self) {
        self.poisoned = true;
    }

    /// Whether [`mark_poisoned`](Self::mark_poisoned) has been called
    /// on this connection. [`PoisonAwareManager::has_broken`] reads
    /// this to decide whether bb8 should close the connection instead
    /// of returning it to the idle queue.
    pub fn is_poisoned(&self) -> bool {
        self.poisoned
    }
}

impl<C> Deref for PoisonableConnection<C> {
    type Target = C;

    fn deref(&self) -> &C {
        &self.inner
    }
}

impl<C> DerefMut for PoisonableConnection<C> {
    fn deref_mut(&mut self) -> &mut C {
        &mut self.inner
    }
}

/// `bb8::ManageConnection` for the legacy MSSQL pool.
///
/// Thin wrapper around `bb8_tiberius::ConnectionManager` (`connect` /
/// `is_valid` just delegate) that swaps in [`PoisonableConnection`] as
/// the pooled connection type so `has_broken` can act on the poison
/// flag instead of always returning `false` — see the
/// [`PoisonableConnection`] docs for the full issue #274 writeup.
///
/// `Error` stays `bb8_tiberius::Error` (unchanged) so every existing
/// `bb8::RunError<bb8_tiberius::Error>` call site (`ApiError`,
/// `WritebackError`, `SyncError`) keeps compiling unmodified — only
/// the manager/connection *type* changed, not the error type.
pub struct PoisonAwareManager {
    inner: TiberiusManager,
}

impl PoisonAwareManager {
    pub fn new(config: Config) -> Self {
        Self {
            inner: TiberiusManager::new(config),
        }
    }
}

impl ManageConnection for PoisonAwareManager {
    type Connection = PoisonableConnection<bb8_tiberius::rt::Client>;
    type Error = bb8_tiberius::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let client = self.inner.connect().await?;
        Ok(PoisonableConnection::new(client))
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        self.inner.is_valid(&mut conn.inner).await
    }

    fn has_broken(&self, conn: &mut Self::Connection) -> bool {
        // Straight passthrough onto the poison flag — kept as a
        // one-liner deliberately. We can't exercise this trait impl
        // directly in a unit test: `Self::Connection` wraps a live
        // `tiberius::Client`, which the crate offers no way to
        // construct without a real TCP+TDS handshake (the same
        // constraint `db::mssql_timeout`'s test module documents).
        // The `poisonable_connection_*` tests below cover the actual
        // flag semantics this delegates to.
        conn.is_poisoned()
    }
}

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

    let manager = PoisonAwareManager::new(tib_config);

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

        let manager = PoisonAwareManager::new(tib_config);

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

    // --- Issue #274: poisoned-connection tests ---
    //
    // `PoisonableConnection` is generic over the wrapped connection
    // type specifically so these can run against a plain `i32` stand-
    // in instead of a live `tiberius::Client` — the crate offers no
    // way to construct one without a real TCP+TDS handshake (same
    // constraint documented in `db::mssql_timeout`'s test module).
    // `PoisonAwareManager::has_broken` is a one-line passthrough onto
    // `is_poisoned()` (see its doc comment), so these fully cover the
    // semantics bb8's `put_back` will observe.

    #[test]
    fn poisonable_connection_starts_unpoisoned() {
        let conn = PoisonableConnection::new(42i32);
        assert!(!conn.is_poisoned());
    }

    #[test]
    fn poisonable_connection_mark_poisoned_sets_flag() {
        let mut conn = PoisonableConnection::new(42i32);
        conn.mark_poisoned();
        assert!(conn.is_poisoned());
    }

    #[test]
    fn poisonable_connection_mark_poisoned_is_idempotent() {
        let mut conn = PoisonableConnection::new(42i32);
        conn.mark_poisoned();
        conn.mark_poisoned();
        assert!(conn.is_poisoned());
    }

    #[test]
    fn poisonable_connection_deref_exposes_inner() {
        let conn = PoisonableConnection::new(String::from("legacy"));
        // `simple_query`/`execute` call sites rely on exactly this:
        // methods on the wrapped type resolve straight through the
        // wrapper via auto-deref.
        assert_eq!(conn.len(), 6);
        assert_eq!(conn.as_str(), "legacy");
    }

    #[test]
    fn poisonable_connection_deref_mut_allows_inner_mutation() {
        let mut conn = PoisonableConnection::new(vec![1, 2, 3]);
        conn.push(4);
        assert_eq!(*conn, vec![1, 2, 3, 4]);
        // Mutating through the wrapper doesn't implicitly poison it —
        // only an explicit `mark_poisoned()` call does.
        assert!(!conn.is_poisoned());
    }

    /// Core wiring test: reproduces the exact shape `db::mssql_timeout`
    /// uses — a mutably-borrowing future wrapped in
    /// `tokio::time::timeout`, with `conn.mark_poisoned()` called in
    /// the `Err(_elapsed)` arm once the timeout has fired and the
    /// future (and its borrow of `conn`) has been dropped. Proves the
    /// borrow-checker shape compiles and that a connection is left
    /// poisoned after a timed-out operation.
    #[tokio::test(start_paused = true)]
    async fn timeout_elapsing_then_marking_poisons_the_connection() {
        let mut conn = PoisonableConnection::new(0i32);
        let budget = Duration::from_millis(200);

        let fut = async {
            // Borrows `conn` mutably for the duration of the future,
            // exactly like `conn.simple_query(sql)` does in
            // `simple_query_with_timeout`.
            let _ = &mut conn;
            std::future::pending::<()>().await
        };

        match tokio::time::timeout(budget, fut).await {
            Ok(()) => panic!("pending future must not resolve before the timeout"),
            Err(_elapsed) => {
                conn.mark_poisoned();
            }
        }

        assert!(
            conn.is_poisoned(),
            "connection must be poisoned after its operation timed out"
        );
    }

    /// Companion: a connection whose operation completes within budget
    /// is never poisoned — documents the success path so a regression
    /// that poisons unconditionally can't slip through.
    #[tokio::test(start_paused = true)]
    async fn completing_within_budget_leaves_connection_unpoisoned() {
        let mut conn = PoisonableConnection::new(0i32);
        let budget = Duration::from_millis(200);

        let fut = async {
            let _ = &mut conn;
            42
        };

        let result = tokio::time::timeout(budget, fut).await;
        assert_eq!(result.expect("must not elapse"), 42);
        assert!(!conn.is_poisoned());
    }
}
