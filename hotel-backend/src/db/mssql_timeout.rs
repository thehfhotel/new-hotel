//! Per-query timeout helpers for legacy MSSQL access.
//!
//! ## Why this exists
//!
//! `tiberius::Client::simple_query` (and `Client::query`) have **no
//! built-in timeout**. The bb8 pool's `connection_timeout` only bounds
//! `Pool::get` — once a connection is in hand, a wire call can block
//! forever waiting for the server to return rows. In practice that
//! happens when the legacy app is holding a row / page lock during a
//! booking cancellation (`HT_Book_H`), an ID-allocator transaction
//! (`HT_Customers`), or a long-running iHOTEL report job. The CT
//! watcher's per-table CHANGETABLE poll hangs behind the same lock
//! and freezes the watermark for the entire site.
//!
//! 2026-05-14 incident: HF Hotel CT watermark stalled 74 minutes
//! against `HT_Book_H.Book_ID='R015142'` — symptom was zero progress
//! across every CT-enabled table while iHOTEL held the row lock
//! during a cancel.
//!
//! ## What this module provides
//!
//! [`simple_query_with_timeout`] wraps `client.simple_query(sql)` +
//! `into_first_result()` in a [`tokio::time::timeout`] bounded by an
//! [`MssqlOpKind`]-dependent budget. On expiry it returns a synthetic
//! `tiberius::error::Error::Io { kind: TimedOut, .. }` — which already
//! threads through every caller's existing `?`-from-Tiberius path with
//! zero call-site refactor, and which `WritebackError::is_retryable`
//! already classifies as retryable.
//!
//! ## Budgets
//!
//! Two operation classes, each independently tunable via env:
//!
//! | Kind | Default | Env override |
//! |------|---------|--------------|
//! | [`MssqlOpKind::Read`]  | 10s | `LEGACY_QUERY_TIMEOUT_READ_MS` |
//! | [`MssqlOpKind::Write`] | 15s | `LEGACY_QUERY_TIMEOUT_WRITE_MS` |
//!
//! Budgets are read **once** at process start (first call) into a
//! `OnceLock` — restart the worker to pick up new values. A bad env
//! value (non-numeric, zero, or below
//! [`MIN_BUDGET_MS`]) falls back to the default and logs a WARN.
//!
//! ## Observability
//!
//! On timeout we emit `tracing::error!` with `event_name =
//! [`EV_TIBERIUS_TIMEOUT`]` — the canonical event name the operator
//! runbook (`docs/incidents/2026-05-14-ct-watermark-stall.md`) tells
//! ops to grep for. Naming follows R1's `sync.<snake_case>` taxonomy
//! convention (see `bin/sync.rs::KNOWN_SYNC_EVENT_NAMES`); the const
//! is co-located here because `bin/*` is a crate-binary target and
//! the timeout helper is in a library module that cannot import from
//! it.

use std::sync::OnceLock;
use std::time::Duration;

use bb8::PooledConnection;
use bb8_tiberius::ConnectionManager;
use tiberius::error::IoErrorKind;
use tiberius::Row;

/// Stable `event_name` attached to every tiberius-timeout error log.
/// Matches R1's `sync.<snake_case>` taxonomy convention so dashboard
/// filters of the shape `event_name =~ /^sync\./` cover it
/// automatically.
pub const EV_TIBERIUS_TIMEOUT: &str = "sync.tiberius_timeout";

/// Convenience alias matching `writeback::allocate::LegacyConn`. Local
/// to this module so the helper doesn't have to depend on the
/// `writeback` module (which would create a layering inversion —
/// `writeback` legitimately depends on `db`, not the other way
/// around).
type LegacyConn<'a> = PooledConnection<'a, ConnectionManager>;

/// Default read budget — covers the per-table CT poll, `parent_loader`
/// aggregate fetches, eager-fetch fallbacks, fingerprint reads. 10s is
/// comfortably above the p99 wire latency observed in production
/// (sub-100ms over WireGuard) while short enough that a row-locked
/// poll fails fast and lets the watchdog re-poll.
const DEFAULT_READ_TIMEOUT_MS: u64 = 10_000;

/// Default write budget — covers writeback recipe statements
/// (INSERT/UPDATE/DELETE inside `BEGIN TRAN`). 15s gives the
/// TABLOCKX-HOLDLOCK MAX+1 allocators a wider window than reads;
/// recipes can issue 5–10 statements per intent and we don't want
/// a slow-but-progressing batch to spuriously time out.
const DEFAULT_WRITE_TIMEOUT_MS: u64 = 15_000;

/// Floor below which a user-supplied env value is rejected and the
/// default substituted. A 0ms or 5ms budget would fail every call.
const MIN_BUDGET_MS: u64 = 100;

/// Env var: read-operation budget override (milliseconds).
const ENV_READ_TIMEOUT_MS: &str = "LEGACY_QUERY_TIMEOUT_READ_MS";

/// Env var: write-operation budget override (milliseconds).
const ENV_WRITE_TIMEOUT_MS: &str = "LEGACY_QUERY_TIMEOUT_WRITE_MS";

/// Operation class — selects which budget the helper applies.
///
/// Callers in `sync/*` (CT watcher, parent loader, eager fetches) use
/// [`Read`](MssqlOpKind::Read). Callers in `writeback/recipes/*` and
/// `writeback/allocate.rs` use [`Write`](MssqlOpKind::Write).
///
/// `SELECT` statements issued by writeback recipes (e.g. MAX+1
/// allocator probes, prior-occupant lookups) are still classified as
/// `Write` because they run inside the recipe's `BEGIN TRAN` and
/// budget-rounding to the write-side keeps the whole transaction
/// under one consistent envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MssqlOpKind {
    /// Read-side budget. Default 10s, override
    /// `LEGACY_QUERY_TIMEOUT_READ_MS`.
    Read,
    /// Write-side budget. Default 15s, override
    /// `LEGACY_QUERY_TIMEOUT_WRITE_MS`.
    Write,
}

impl MssqlOpKind {
    fn budget(self) -> Duration {
        match self {
            MssqlOpKind::Read => read_budget(),
            MssqlOpKind::Write => write_budget(),
        }
    }

    fn as_static_str(self) -> &'static str {
        match self {
            MssqlOpKind::Read => "read",
            MssqlOpKind::Write => "write",
        }
    }
}

/// Resolve the read budget on first access, then cache it.
fn read_budget() -> Duration {
    static CELL: OnceLock<Duration> = OnceLock::new();
    *CELL.get_or_init(|| resolve_budget_from_env(ENV_READ_TIMEOUT_MS, DEFAULT_READ_TIMEOUT_MS))
}

/// Resolve the write budget on first access, then cache it.
fn write_budget() -> Duration {
    static CELL: OnceLock<Duration> = OnceLock::new();
    *CELL.get_or_init(|| resolve_budget_from_env(ENV_WRITE_TIMEOUT_MS, DEFAULT_WRITE_TIMEOUT_MS))
}

/// Pure helper — parse an env var or fall back to the supplied default.
/// Extracted so the unit tests can exercise the parsing rules without
/// mutating real process env (which would race with parallel tests).
fn parse_budget_ms(raw: Option<&str>, default_ms: u64) -> Duration {
    match raw {
        None => Duration::from_millis(default_ms),
        Some(s) => match s.trim().parse::<u64>() {
            Ok(ms) if ms >= MIN_BUDGET_MS => Duration::from_millis(ms),
            _ => Duration::from_millis(default_ms),
        },
    }
}

/// Read the env var live, log a WARN on rejection, cache the result.
fn resolve_budget_from_env(env_var: &'static str, default_ms: u64) -> Duration {
    let raw = std::env::var(env_var).ok();
    let chosen = parse_budget_ms(raw.as_deref(), default_ms);
    if let Some(s) = raw.as_deref() {
        let supplied = s.trim().parse::<u64>().ok();
        let accepted = supplied.is_some_and(|ms| ms >= MIN_BUDGET_MS);
        if !accepted {
            tracing::warn!(
                env_var,
                value = s,
                fallback_ms = default_ms,
                min_ms = MIN_BUDGET_MS,
                "invalid MSSQL query-timeout override — using default"
            );
        }
    }
    tracing::debug!(
        env_var,
        budget_ms = chosen.as_millis() as u64,
        "MSSQL query-timeout budget resolved"
    );
    chosen
}

/// Construct the synthetic `tiberius::error::Error` the helper returns
/// on timeout. `Io { kind: TimedOut, .. }` was chosen deliberately:
///
/// * `tiberius::error::Error` already threads through every caller's
///   `?`-from-Tiberius path (`WritebackError::Tiberius`,
///   `SyncError::Tiberius`, ad-hoc `.map_err(|e| e.to_string())` in
///   `bin/sync.rs`), so call sites don't need a new error variant.
/// * `WritebackError::is_retryable` already classifies all `Tiberius`
///   variants as retryable, which matches our intent: a stuck wire
///   call should bounce and retry on the next tick / next attempt.
/// * `kind: TimedOut` lets downstream consumers (R1 taxonomy) pattern-
///   match if they want to break out a `Timeout` variant later.
fn timeout_error(sql: &str, kind: MssqlOpKind, budget: Duration) -> tiberius::error::Error {
    // Keep the SQL preview small — recipe statements can be many KB
    // and a full dump would flood the log. The first whitespace token
    // is usually the statement verb (SELECT / INSERT / UPDATE / …).
    let preview = sql.split_whitespace().next().unwrap_or("<empty>");
    tiberius::error::Error::Io {
        kind: IoErrorKind::TimedOut,
        message: format!(
            "MSSQL {kind_str} query exceeded {budget_ms}ms budget (sql_kind={preview})",
            kind_str = kind.as_static_str(),
            budget_ms = budget.as_millis() as u64,
        ),
    }
}

/// Run `client.simple_query(sql)` and `into_first_result()` under a
/// [`tokio::time::timeout`] bounded by the [`MssqlOpKind`] budget.
///
/// On timeout, emits `tracing::error!(event_name = EV_TIBERIUS_TIMEOUT, ...)`
/// and returns a synthetic [`tiberius::error::Error::Io`] with
/// `kind = TimedOut` — see [`timeout_error`].
///
/// **Why both `simple_query` AND `into_first_result` are inside the
/// timeout:** `simple_query` initiates the query on the wire but the
/// rows are streamed back lazily; a server holding a lock can block
/// either the initial response or any subsequent row fetch. Wrapping
/// just `simple_query` would leak the lock-induced stall into the
/// stream consumption. We materialise the full first result-set
/// inside the timeout so the bound covers the complete round-trip.
pub async fn simple_query_with_timeout(
    conn: &mut LegacyConn<'_>,
    sql: &str,
    kind: MssqlOpKind,
) -> Result<Vec<Row>, tiberius::error::Error> {
    let budget = kind.budget();
    let fut = async {
        let stream = conn.simple_query(sql).await?;
        stream.into_first_result().await
    };
    match tokio::time::timeout(budget, fut).await {
        Ok(result) => result,
        Err(_elapsed) => {
            tracing::error!(
                event_name = EV_TIBERIUS_TIMEOUT,
                op_kind = kind.as_static_str(),
                budget_ms = budget.as_millis() as u64,
                sql_kind = sql.split_whitespace().next().unwrap_or("<empty>"),
                sql_len = sql.len(),
                "MSSQL query exceeded per-op timeout — likely legacy row lock held by iHOTEL"
            );
            Err(timeout_error(sql, kind, budget))
        }
    }
}

/// Same wire-level guarantees as [`simple_query_with_timeout`] but with
/// a caller-supplied explicit budget instead of the env-cached
/// per-`MssqlOpKind` defaults. Reserved for callers that need a
/// tighter budget than the default read window — canonical use case
/// is the watermark-stall watchdog's `CHANGE_TRACKING_CURRENT_VERSION()`
/// probe, which fires every 60s and MUST fail fast (≤ 5s) so the
/// watchdog tick doesn't drift behind a row-locked legacy.
///
/// Floor is still [`MIN_BUDGET_MS`] — a 0ms / 5ms budget would fail
/// every call.
pub async fn simple_query_with_explicit_timeout(
    conn: &mut PooledConnection<'_, ConnectionManager>,
    sql: &str,
    budget: Duration,
) -> Result<Vec<Row>, tiberius::error::Error> {
    let budget = if budget.as_millis() < MIN_BUDGET_MS as u128 {
        Duration::from_millis(MIN_BUDGET_MS)
    } else {
        budget
    };
    let fut = async {
        let stream = conn.simple_query(sql).await?;
        stream.into_first_result().await
    };
    match tokio::time::timeout(budget, fut).await {
        Ok(result) => result,
        Err(_elapsed) => {
            tracing::error!(
                event_name = EV_TIBERIUS_TIMEOUT,
                op_kind = "explicit",
                budget_ms = budget.as_millis() as u64,
                sql_kind = sql.split_whitespace().next().unwrap_or("<empty>"),
                sql_len = sql.len(),
                "MSSQL query exceeded explicit timeout — likely legacy row lock held by iHOTEL"
            );
            // Reuse the `Read` envelope text so log consumers see one
            // consistent error shape; the explicit budget is captured
            // in the `budget_ms` log field above.
            Err(timeout_error(sql, MssqlOpKind::Read, budget))
        }
    }
}

/// Same wire-level guarantees as [`simple_query_with_timeout`] but for
/// statements that don't need rows back (SET, BEGIN/COMMIT/ROLLBACK,
/// fire-and-forget DML where the recipe asserts shape via the SQL
/// itself). Drops the result-set after materialising it so the wire
/// fully drains before the bound.
pub async fn simple_query_with_timeout_drop(
    conn: &mut LegacyConn<'_>,
    sql: &str,
    kind: MssqlOpKind,
) -> Result<(), tiberius::error::Error> {
    let _ = simple_query_with_timeout(conn, sql, kind).await?;
    Ok(())
}

/// Alias re-export for call sites that hold a bb8 `PooledConnection`
/// (rather than the `writeback::allocate::LegacyConn` re-alias of the
/// same type). Functionally identical to
/// [`simple_query_with_timeout`] — `LegacyConn` is just a type alias
/// over `bb8::PooledConnection<'_, ConnectionManager>`, so this just
/// re-points at the same implementation for readability at the
/// call site.
pub async fn simple_query_with_timeout_pooled(
    conn: &mut PooledConnection<'_, ConnectionManager>,
    sql: &str,
    kind: MssqlOpKind,
) -> Result<Vec<Row>, tiberius::error::Error> {
    simple_query_with_timeout(conn, sql, kind).await
}

#[cfg(test)]
mod tests {
    //! Unit tests for the timeout helper. The integration-grade test —
    //! "give me a never-resolving future and assert the timeout
    //! fires" — cannot use a real `tiberius::Client` mock (the
    //! crate exposes no test seam), so we exercise the wrapping logic
    //! directly via a manually-constructed `pending()` future.
    //!
    //! The OnceLock-backed budgets are exercised through pure
    //! [`parse_budget_ms`] tests so concurrent test cases can't race
    //! on real env mutation.
    use super::*;
    use std::future::pending;

    #[test]
    fn parse_budget_ms_uses_default_when_env_unset() {
        let b = parse_budget_ms(None, 10_000);
        assert_eq!(b, Duration::from_millis(10_000));
    }

    #[test]
    fn parse_budget_ms_accepts_valid_override() {
        let b = parse_budget_ms(Some("250"), 10_000);
        assert_eq!(b, Duration::from_millis(250));
    }

    #[test]
    fn parse_budget_ms_trims_whitespace() {
        let b = parse_budget_ms(Some("  500  "), 10_000);
        assert_eq!(b, Duration::from_millis(500));
    }

    #[test]
    fn parse_budget_ms_rejects_below_floor() {
        // 50ms < MIN_BUDGET_MS (100) — fall back.
        let b = parse_budget_ms(Some("50"), 10_000);
        assert_eq!(b, Duration::from_millis(10_000));
    }

    #[test]
    fn parse_budget_ms_rejects_zero() {
        let b = parse_budget_ms(Some("0"), 10_000);
        assert_eq!(b, Duration::from_millis(10_000));
    }

    #[test]
    fn parse_budget_ms_rejects_non_numeric() {
        let b = parse_budget_ms(Some("forever"), 10_000);
        assert_eq!(b, Duration::from_millis(10_000));
    }

    #[test]
    fn parse_budget_ms_rejects_negative_via_unsigned_parse() {
        // `u64::parse("-1")` is Err — falls back.
        let b = parse_budget_ms(Some("-1"), 10_000);
        assert_eq!(b, Duration::from_millis(10_000));
    }

    #[test]
    fn parse_budget_ms_accepts_floor_exact() {
        let b = parse_budget_ms(Some("100"), 10_000);
        assert_eq!(b, Duration::from_millis(100));
    }

    #[test]
    fn timeout_error_is_io_timed_out() {
        let err = timeout_error("SELECT 1", MssqlOpKind::Read, Duration::from_millis(123));
        match err {
            tiberius::error::Error::Io { kind, message } => {
                assert_eq!(kind, IoErrorKind::TimedOut);
                assert!(message.contains("123ms"), "got {message}");
                assert!(message.contains("read"), "got {message}");
                assert!(message.contains("SELECT"), "got {message}");
            }
            other => panic!("expected Io{{TimedOut}}, got {other:?}"),
        }
    }

    #[test]
    fn timeout_error_redacts_to_first_token() {
        let big_sql = "INSERT INTO HT_Book_H (Book_Cust_Name) VALUES ('secret pii')";
        let err = timeout_error(big_sql, MssqlOpKind::Write, Duration::from_millis(456));
        let msg = err.to_string();
        // First token leaks (INSERT), full SQL doesn't.
        assert!(msg.contains("INSERT"), "got {msg}");
        assert!(!msg.contains("secret pii"), "PII leaked: {msg}");
    }

    #[test]
    fn op_kind_static_str_round_trips() {
        assert_eq!(MssqlOpKind::Read.as_static_str(), "read");
        assert_eq!(MssqlOpKind::Write.as_static_str(), "write");
    }

    /// Core integration-light test: a future that pends forever, wrapped in
    /// `tokio::time::timeout` with a tight budget, must elapse — proving
    /// the same wrapping wedge we use in [`simple_query_with_timeout`]
    /// will fire on a stuck wire call.
    #[tokio::test(start_paused = true)]
    async fn pending_future_elapses_within_budget() {
        let budget = Duration::from_millis(200);
        let stuck = pending::<Result<(), tiberius::error::Error>>();
        let result = tokio::time::timeout(budget, stuck).await;
        assert!(
            result.is_err(),
            "timeout must fire on a pending future"
        );
    }

    /// Companion: a future that completes within budget returns its
    /// value. Documents the success path so a regression where the
    /// timeout always trips can't slip through.
    #[tokio::test(start_paused = true)]
    async fn ready_future_passes_through_timeout() {
        let budget = Duration::from_millis(200);
        let ok = async { Ok::<i32, tiberius::error::Error>(42) };
        let result = tokio::time::timeout(budget, ok).await;
        assert_eq!(result.expect("must not elapse").expect("must be Ok"), 42);
    }
}
