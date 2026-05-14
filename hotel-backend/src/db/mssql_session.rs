//! MSSQL session-scoped tagging for loop-prevention.
//!
//! Per `docs/architecture.md` §3.6c / §3.6d and the Phase 5 plan:
//! every writeback session starts by tagging itself with a 2-byte
//! `CONTEXT_INFO` value (`0x4E48` = ASCII `"NH"` for *New Hotel*).
//! SQL Server Change Tracking surfaces that value as
//! `SYS_CHANGE_CONTEXT` on every row mutated by the session.
//!
//! The CT watcher (`bin/sync.rs`) filters those rows out:
//! ```sql
//! WHERE ct.SYS_CHANGE_CONTEXT IS NULL
//!    OR ct.SYS_CHANGE_CONTEXT <> 0x4E48
//! ```
//! preventing a feedback loop where our writeback fires CT, the watcher
//! re-detects it, re-publishes the event, and the writeback fires again.
//!
//! Belt-and-suspenders: the mappers in 5.2+ are written as idempotent
//! UPSERTs against PG, so a missed tag costs at most one extra cycle of
//! work — never an infinite loop. This call is the cheap first line of
//! defence; the UPSERTs are the safety net.
//!
//! The chokepoint that calls this is `writeback::dispatcher::dispatch`
//! — it is the *single* entry point through which every recipe runs, so
//! the tag is set exactly once per writeback transaction with no chance
//! of a recipe forgetting it.

use bb8::PooledConnection;
use bb8_tiberius::ConnectionManager;

use crate::db::mssql_timeout::{simple_query_with_timeout_drop, MssqlOpKind};

/// Convenience alias matching `writeback::allocate::LegacyConn`.
type LegacyConn<'a> = PooledConnection<'a, ConnectionManager>;

/// Issue `SET CONTEXT_INFO 0x4E48` on the supplied connection.
///
/// Two bytes — `0x4E` = `'N'`, `0x48` = `'H'` — chosen to be human-
/// readable in `sys.dm_exec_sessions.context_info` and trivial to filter
/// from `CHANGETABLE(CHANGES …).SYS_CHANGE_CONTEXT`.
///
/// The value is session-scoped: it persists on the bb8 connection until
/// the connection is reset / dropped or another `SET CONTEXT_INFO` runs.
/// Calling it at the top of `dispatch` is enough to cover the entire
/// recipe — every subsequent statement on the same connection inherits
/// the tag.
pub async fn set_context_info(
    conn: &mut LegacyConn<'_>,
) -> Result<(), tiberius::error::Error> {
    // R2 (2026-05-14): wrap in per-op timeout. The SET runs at the
    // top of every writeback transaction — a stuck session here
    // would block the entire writeback worker indefinitely. Write
    // budget is appropriate: the call belongs to the surrounding
    // recipe's BEGIN TRAN envelope.
    simple_query_with_timeout_drop(conn, "SET CONTEXT_INFO 0x4E48", MssqlOpKind::Write).await
}
