//! Database connection pool using tiberius and bb8

use std::future::Future;
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

/// Budget for the `SELECT 1` bb8 runs on every checkout
/// (`test_on_check_out`, see [`PoisonAwareManager::is_valid`]).
///
/// Must stay well under `POOL_CONNECTION_TIMEOUT`: bb8 wraps its whole
/// checkout loop — validation included — in
/// `timeout(connection_timeout, ..)` (bb8 0.9.1 `inner.rs:124`). If
/// *that* timer wins the race, the half-validated connection is dropped
/// with state `Present` (`api.rs:531-542`) and `put_back`
/// (`inner.rs:143-158`) re-queues it unless `has_broken` objects.
///
/// **2026-08-26 HF Ville incident:** a 115 s WireGuard break took
/// 5–8 min to clear because every dead idle connection was re-queued
/// exactly this way — its unbounded `SELECT 1` sat in TCP retransmit,
/// the 5 s outer timer fired first, nothing poisoned it, and only the
/// 10-min `max_lifetime` reaper or an eventual retransmit-back-off
/// error (RTO up to ~2 min) evicted it. A healthy `SELECT 1` answers
/// in milliseconds over the tunnel (p99 well under 100 ms), so 2 s is
/// generous on the success path and short enough that one `Pool::get`
/// can discard two dead connections and still open a fresh one inside
/// the 5 s outer budget.
const POOL_VALIDATE_TIMEOUT: Duration = Duration::from_secs(2);

/// Stable `event_name` for a checkout-validation probe that outran
/// [`POOL_VALIDATE_TIMEOUT`]. Same `sync.<snake_case>` taxonomy as
/// [`crate::db::mssql_timeout::EV_TIBERIUS_TIMEOUT`], and co-located
/// here for the same reason (library module; `bin/*` can't be
/// imported from). Expect a burst of at most `max_size` of these
/// right after a tunnel break — one per dead connection evicted.
pub const EV_POOL_VALIDATE_TIMEOUT: &str = "sync.pool_validate_timeout";

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
/// runs `SELECT 1` on every checkout; against a desynced stream that
/// read hangs — historically until bb8's `connection_timeout` (5s —
/// `POOL_CONNECTION_TIMEOUT` above), producing the observed `Timed
/// out in bb8` bursts, and since the 2026-08-26 fix until
/// `POOL_VALIDATE_TIMEOUT` (2s), after which the connection is
/// evicted (see [`PoisonAwareManager::is_valid`]). Without the poison
/// flag only the 10-minute `max_lifetime` reaper eventually retires
/// the connection, so one poisoned connection can degrade a pool slot
/// for up to 10 minutes.
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
/// Thin wrapper around `bb8_tiberius::ConnectionManager` (`connect`
/// delegates; `is_valid` delegates under a [`POOL_VALIDATE_TIMEOUT`]
/// budget) that swaps in [`PoisonableConnection`] as the pooled
/// connection type so `has_broken` can act on the poison flag instead
/// of always returning `false` — see the [`PoisonableConnection`] docs
/// for the full issue #274 writeup.
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

/// Run a checkout-validation `probe` under `budget`, tracking the
/// connection's poison flag *pessimistically*: the flag is raised
/// before the probe is polled and lowered (to its prior value) only
/// when the probe returns `Ok`. Every other outcome leaves it raised —
/// the probe erroring, the budget elapsing, **or this future being
/// cancelled mid-flight** by bb8's outer `connection_timeout` — so
/// `has_broken` reports `true` and bb8 closes the connection instead
/// of re-queuing it.
///
/// The order matters for the cancellation case: when bb8's own timer
/// wins the race (e.g. the third dead connection popped by one
/// `Pool::get` after two 2 s validation timeouts) it drops this future
/// with the guard still `Present`, and nothing after the `.await`
/// runs. Raising the flag first is the only way to cover that path.
/// Restoring the *prior* value on success (rather than clearing)
/// means a probe can never un-poison a connection that was poisoned
/// for another reason.
///
/// Generic over the error type and the probe so the tests below can
/// drive it with a pending / ready / erroring future and a bare
/// `bool`, without a live `tiberius::Client` (the crate offers no way
/// to construct one offline — same constraint as `db::mssql_timeout`).
async fn validate_within_budget<E>(
    poisoned: &mut bool,
    budget: Duration,
    probe: impl Future<Output = Result<(), E>>,
    on_elapsed: impl FnOnce() -> E,
) -> Result<(), E> {
    let prior = *poisoned;
    *poisoned = true;
    match tokio::time::timeout(budget, probe).await {
        Ok(Ok(())) => {
            *poisoned = prior;
            Ok(())
        }
        Ok(Err(e)) => Err(e),
        Err(_elapsed) => Err(on_elapsed()),
    }
}

/// Synthetic error for a validation probe that outran its budget —
/// the same `Io { kind: TimedOut }` shape
/// `db::mssql_timeout::timeout_error` returns, so anything that
/// already classifies tiberius timeouts sees one more of the same.
/// bb8 hands it to its error sink (a no-op by default) and continues
/// the checkout loop; the WARN in `is_valid` is the operator signal.
fn validation_timeout_error(budget: Duration) -> bb8_tiberius::Error {
    bb8_tiberius::Error::Tiberius(tiberius::error::Error::Io {
        kind: tiberius::error::IoErrorKind::TimedOut,
        message: format!(
            "MSSQL pool checkout validation (SELECT 1) exceeded {}ms budget",
            budget.as_millis()
        ),
    })
}

impl ManageConnection for PoisonAwareManager {
    type Connection = PoisonableConnection<bb8_tiberius::rt::Client>;
    type Error = bb8_tiberius::Error;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        let client = self.inner.connect().await?;
        Ok(PoisonableConnection::new(client))
    }

    /// Checkout validation (`test_on_check_out`), bounded by
    /// [`POOL_VALIDATE_TIMEOUT`] and poison-tracked by
    /// [`validate_within_budget`] so a dead connection is discarded on
    /// its *first* checkout instead of being re-queued.
    ///
    /// bb8 reacts to `Err` here by marking the connection `Invalid` and
    /// moving on to the next idle one (or a fresh `connect`) within the
    /// remaining `connection_timeout` (bb8 0.9.1 `inner.rs:110-116`);
    /// the `Invalid` guard is then closed by `put_back`. The poison flag
    /// is belt-and-braces for the case bb8 can't see: its outer timer
    /// cancelling this future mid-probe.
    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        // Disjoint borrows: the probe holds `inner`, the helper holds
        // the flag — no `&mut conn` overlap across the `.await`.
        let PoisonableConnection { inner, poisoned } = conn;
        validate_within_budget(
            poisoned,
            POOL_VALIDATE_TIMEOUT,
            self.inner.is_valid(inner),
            || {
                tracing::warn!(
                    event_name = EV_POOL_VALIDATE_TIMEOUT,
                    budget_ms = POOL_VALIDATE_TIMEOUT.as_millis() as u64,
                    "legacy MSSQL pool: checkout validation (SELECT 1) exceeded \
                     budget — evicting connection instead of re-queuing it"
                );
                validation_timeout_error(POOL_VALIDATE_TIMEOUT)
            },
        )
        .await
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
/// - checkout validation: `SELECT 1` bounded by `POOL_VALIDATE_TIMEOUT`,
///   dead connections evicted rather than re-queued.
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
        // validate every checkout with `is_valid` (bb8's default, but
        // load-bearing: the bounded, poison-tracked `SELECT 1` in
        // `PoisonAwareManager::is_valid` is what evicts a connection
        // that died during a tunnel break — spelled out so nobody
        // "optimises" it away).
        .test_on_check_out(true)
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
            .test_on_check_out(true)
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

    // --- 2026-08-26: bounded checkout validation ---
    //
    // `validate_within_budget` is the whole mechanism; `is_valid` only
    // feeds it the real `SELECT 1` future and the tiberius error. Same
    // stand-in approach as the #274 tests above: a bare `bool` for the
    // flag and pending / ready / erroring futures for the probe.

    #[tokio::test]
    async fn validate_timeout_is_two_seconds_and_under_the_outer_budget() {
        assert_eq!(POOL_VALIDATE_TIMEOUT, Duration::from_secs(2));
        // If the outer bb8 timer could fire first, a dead connection
        // would be re-queued with state `Present` — the exact bug.
        assert!(
            POOL_VALIDATE_TIMEOUT < POOL_CONNECTION_TIMEOUT,
            "validation budget must be strictly inside bb8's connection_timeout"
        );
    }

    #[tokio::test]
    async fn pool_builder_validates_on_check_out() {
        let pool = pool_with_timeouts(&stub_db_config());
        assert!(
            pool.config().test_on_check_out,
            "checkout validation is what evicts dead connections — must stay on"
        );
    }

    /// (a) A validation that exceeds the budget → error, connection
    /// poisoned, so `has_broken` evicts it at `put_back`.
    #[tokio::test(start_paused = true)]
    async fn validation_exceeding_budget_poisons_the_connection() {
        let mut poisoned = false;
        let probe = std::future::pending::<Result<(), &'static str>>();

        let result =
            validate_within_budget(&mut poisoned, Duration::from_millis(200), probe, || {
                "elapsed"
            })
            .await;

        assert_eq!(result, Err("elapsed"));
        assert!(
            poisoned,
            "a timed-out validation must poison the connection"
        );
    }

    /// (b) A healthy validation → `Ok`, connection left unpoisoned —
    /// the success path is byte-for-byte the pre-fix behaviour.
    #[tokio::test(start_paused = true)]
    async fn healthy_validation_leaves_connection_unpoisoned() {
        let mut poisoned = false;
        let probe = std::future::ready(Ok::<(), &'static str>(()));

        let result =
            validate_within_budget(&mut poisoned, Duration::from_millis(200), probe, || {
                "elapsed"
            })
            .await;

        assert_eq!(result, Ok(()));
        assert!(
            !poisoned,
            "a healthy validation must not poison the connection"
        );
    }

    /// A probe that fails outright (server RST, TDS error) is passed
    /// through untouched and also poisons — bb8 discards on `Err`
    /// anyway, but the flag keeps `has_broken` consistent.
    #[tokio::test(start_paused = true)]
    async fn validation_error_is_propagated_and_poisons() {
        let mut poisoned = false;
        let probe = std::future::ready(Err::<(), &'static str>("connection reset"));

        let result =
            validate_within_budget(&mut poisoned, Duration::from_millis(200), probe, || {
                "elapsed"
            })
            .await;

        assert_eq!(result, Err("connection reset"));
        assert!(poisoned);
    }

    /// The case the plain "timeout then mark" shape misses: bb8's outer
    /// `connection_timeout` cancels the validation future mid-probe
    /// (nothing after the `.await` runs). Pessimistic flagging must
    /// leave the connection poisoned so `put_back(Present)` evicts it.
    #[tokio::test(start_paused = true)]
    async fn validation_cancelled_mid_flight_stays_poisoned() {
        let mut poisoned = false;
        let probe = std::future::pending::<Result<(), &'static str>>();

        // Inner budget 1 s, outer (stand-in for bb8) 100 ms — the outer
        // timer wins and drops the whole validation future.
        let outer = tokio::time::timeout(
            Duration::from_millis(100),
            validate_within_budget(&mut poisoned, Duration::from_secs(1), probe, || "elapsed"),
        )
        .await;

        assert!(
            outer.is_err(),
            "outer timer must have cancelled the validation"
        );
        assert!(
            poisoned,
            "a validation cancelled by the outer timer must leave the connection poisoned"
        );
    }

    /// A probe can never un-poison a connection poisoned for another
    /// reason (e.g. a desynced stream that happens to answer `SELECT 1`
    /// with leftover bytes — issue #274).
    #[tokio::test(start_paused = true)]
    async fn successful_probe_does_not_clear_prior_poison() {
        let mut poisoned = true;
        let probe = std::future::ready(Ok::<(), &'static str>(()));

        let result =
            validate_within_budget(&mut poisoned, Duration::from_millis(200), probe, || {
                "elapsed"
            })
            .await;

        assert_eq!(result, Ok(()));
        assert!(poisoned, "prior poison must survive a successful probe");
    }

    #[test]
    fn validation_timeout_error_is_io_timed_out() {
        match validation_timeout_error(Duration::from_millis(2000)) {
            bb8_tiberius::Error::Tiberius(tiberius::error::Error::Io { kind, message }) => {
                assert_eq!(kind, tiberius::error::IoErrorKind::TimedOut);
                assert!(message.contains("2000ms"), "got {message}");
                assert!(message.contains("SELECT 1"), "got {message}");
            }
            other => panic!("expected Tiberius(Io{{TimedOut}}), got {other:?}"),
        }
    }
}
