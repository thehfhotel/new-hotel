//! MSSQL session-scoped tagging — observability marker, NOT a CT
//! filter input (corrected 2026-06-11, audit P1 #5).
//!
//! Every writeback session tags itself with a 2-byte `CONTEXT_INFO`
//! value (`0x4E48` = ASCII `"NH"` for *New Hotel*).
//!
//! ## What this does NOT do
//!
//! The original Phase 5 design assumed SQL Server Change Tracking
//! surfaces this value as `SYS_CHANGE_CONTEXT`, and the CT watcher
//! filtered on it for loop-prevention. That assumption was WRONG:
//! `SET CONTEXT_INFO` never populates `SYS_CHANGE_CONTEXT` — only the
//! per-statement `WITH CHANGE_TRACKING_CONTEXT (@ctx)` table hint
//! does, and nothing in this codebase uses it. Every writeback CT row
//! always arrived with `SYS_CHANGE_CONTEXT IS NULL` and passed the
//! filter; the predicate was inert and has been removed from the
//! watcher's queries (`bin/sync.rs::build_ct_changes_sql`).
//!
//! **Loop-prevention actually lives in the mappers**: every 5.2+ mapper
//! is an idempotent UPSERT against PG, so re-detecting our own write
//! converges to a no-op and emits no event. That was always the
//! load-bearing mechanism — the "belt-and-suspenders safety net" was
//! the only belt.
//!
//! Do NOT adopt `WITH CHANGE_TRACKING_CONTEXT` + a filter to "restore"
//! SQL-layer suppression: CT coalesces per-PK to the LATEST change's
//! context, so a genuine iHOTEL edit racing a writeback touch on the
//! same row would inherit our tag and be silently filtered — the exact
//! June-3 loss class, manufactured at the SQL layer.
//!
//! ## Why the tag is kept
//!
//! `sys.dm_exec_sessions.context_info` shows the human-readable `NH`
//! marker, letting operators distinguish our writeback sessions from
//! iHOTEL clients during incident triage on the shared server.
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
/// readable in `sys.dm_exec_sessions.context_info`. (It does NOT reach
/// `CHANGETABLE(CHANGES …).SYS_CHANGE_CONTEXT` — see the module doc;
/// the tag is an operator-triage marker only.)
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
