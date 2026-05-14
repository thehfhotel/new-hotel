//! Read / advance the Change Tracking watermark.
//!
//! Two storage shapes coexist while Resilience PR R3 rolls out behind
//! the `SYNC_PER_TABLE_WATERMARK` env flag:
//!
//! 1. **Global single-row** (`legacy_ct_state`, migration 013).
//!    Holds exactly one row tracking the highest `SYS_CHANGE_VERSION`
//!    we've successfully imported from MSSQL across ALL CT-enabled
//!    tables. The watcher resumes from this point on every restart.
//!    Operational on `SYNC_PER_TABLE_WATERMARK=false` (default).
//!    See [`read_last_seen`] / [`advance`].
//!
//! 2. **Per-table** (`legacy_ct_state_per_table`, migration 050).
//!    One row per CT-tracked table, each carrying its own
//!    `last_seen_version`. A row-lock wedge on one hot table
//!    (canonical: `HT_Book_H` on `Book_ID='R015142'`,
//!    74-min stall observed 2026-05-14) freezes only that row
//!    instead of gating every table's advance. Operational on
//!    `SYNC_PER_TABLE_WATERMARK=true`. See [`read_per_table`] /
//!    [`advance_per_table`] / [`record_per_table_error`].
//!
//! Per `docs/architecture.md` §4d-tris.
//!
//! ## Single-write-per-tick contract (global path)
//!
//! [`advance`] is called once per watcher tick AFTER all per-table
//! mappers in that tick have committed. The advance writes the
//! `min(per-table-max(SYS_CHANGE_VERSION))` so a partial tick failure
//! leaves the watermark below the last fully-applied version — the
//! retry will re-fetch the failed table's rows (idempotent UPSERTs in
//! 5.2+ make the re-fetch safe).
//!
//! ## Single-write-per-table-tick contract (per-table path)
//!
//! [`advance_per_table`] is called once per (table, tick) AFTER that
//! table's mapper TX has committed. The advance writes the per-table
//! `max(SYS_CHANGE_VERSION)` so a failure on table X leaves X's
//! watermark behind; tables Y..Z that succeeded in the same tick keep
//! their independent forward progress.

use std::collections::HashMap;

use sqlx::PgPool;

use crate::sync::SyncError;

/// Read the current GLOBAL watermark. Returns 0 on a fresh install
/// (migration 013 seeds the row with `last_seen_version = 0`).
///
/// Operational only when `SYNC_PER_TABLE_WATERMARK=false`. Callers
/// gating on the feature flag should use [`read_per_table`] instead
/// when the flag is on.
pub async fn read_last_seen(pool: &PgPool) -> Result<i64, SyncError> {
    let row: (i64,) =
        sqlx::query_as("SELECT last_seen_version FROM legacy_ct_state WHERE id = 1")
            .fetch_one(pool)
            .await?;
    Ok(row.0)
}

/// Advance the GLOBAL watermark in a single-row UPDATE. Caller MUST
/// have already committed every mapper's per-tick transaction —
/// advancing past unprocessed rows would silently lose them on the
/// next tick.
///
/// `new_version` should be the minimum-of-per-table-max for this tick;
/// see module docs for the rationale.
pub async fn advance(pool: &PgPool, new_version: i64) -> Result<(), SyncError> {
    sqlx::query(
        "UPDATE legacy_ct_state \
            SET last_seen_version = $1, \
                last_polled_at    = now() \
          WHERE id = 1 \
            AND last_seen_version <= $1",
    )
    .bind(new_version)
    .execute(pool)
    .await?;
    Ok(())
}

/// Read every per-table watermark in one query. Returns a map keyed
/// on `table_name` so the caller can look up each mapper's resume
/// point in O(1). Tables absent from the map (e.g. a fresh deploy
/// where migration 050 hasn't seeded a row yet) should be treated as
/// `0` by the caller — same default semantics as
/// [`read_last_seen`].
///
/// Operational only when `SYNC_PER_TABLE_WATERMARK=true`.
pub async fn read_per_table(pool: &PgPool) -> Result<HashMap<String, i64>, SyncError> {
    let rows: Vec<(String, i64)> =
        sqlx::query_as("SELECT table_name, last_seen_version FROM legacy_ct_state_per_table")
            .fetch_all(pool)
            .await?;
    Ok(rows.into_iter().collect())
}

/// Advance ONE table's watermark. UPSERTs so a fresh deploy that
/// hasn't run migration 050's backfill yet still records progress
/// (defensive — the backfill is part of the same migration, so this
/// branch shouldn't trip in practice).
///
/// The `last_seen_version <= EXCLUDED.last_seen_version` guard mirrors
/// [`advance`]'s monotonic constraint: a stale tick that somehow
/// re-races with a newer one can't roll the watermark backward. Also
/// clears `last_error` / `last_error_at` on success so the per-table
/// watchdog can age "currently healthy" rows by `last_polled_at`
/// alone.
pub async fn advance_per_table(
    pool: &PgPool,
    table: &str,
    new_version: i64,
) -> Result<(), SyncError> {
    sqlx::query(
        "INSERT INTO legacy_ct_state_per_table \
             (table_name, last_seen_version, last_polled_at, last_error, last_error_at) \
         VALUES ($1, $2, now(), NULL, NULL) \
         ON CONFLICT (table_name) DO UPDATE \
             SET last_seen_version = EXCLUDED.last_seen_version, \
                 last_polled_at    = EXCLUDED.last_polled_at, \
                 last_error        = NULL, \
                 last_error_at     = NULL \
           WHERE legacy_ct_state_per_table.last_seen_version <= EXCLUDED.last_seen_version",
    )
    .bind(table)
    .bind(new_version)
    .execute(pool)
    .await?;
    Ok(())
}

/// Touch ONE table's `last_polled_at` without advancing its watermark.
/// Used after a successful empty-fetch tick so the per-table watchdog
/// can tell "table is healthy but quiet" from "table is wedged".
///
/// Does NOT clear `last_error` — a transient error followed by an
/// empty tick should keep the error visible until a real advance
/// resolves it (consistent with `legacy_sync_status` semantics).
pub async fn touch_per_table(pool: &PgPool, table: &str) -> Result<(), SyncError> {
    sqlx::query(
        "INSERT INTO legacy_ct_state_per_table (table_name, last_seen_version, last_polled_at) \
         VALUES ($1, 0, now()) \
         ON CONFLICT (table_name) DO UPDATE \
             SET last_polled_at = now()",
    )
    .bind(table)
    .execute(pool)
    .await?;
    Ok(())
}

/// Record a per-table failure. The watcher's main `record_table_error`
/// already writes to `legacy_sync_status`; this is the per-table
/// watermark companion that lets a watchdog page on "this specific
/// table's watermark hasn't advanced AND its last_error is fresh"
/// without joining across two tables.
///
/// Error messages are truncated to 1024 chars to keep the row small;
/// the full trace lives in `legacy_sync_status.last_error` and the
/// container logs.
pub async fn record_per_table_error(
    pool: &PgPool,
    table: &str,
    message: &str,
) -> Result<(), SyncError> {
    let truncated: String = message.chars().take(1024).collect();
    sqlx::query(
        "INSERT INTO legacy_ct_state_per_table \
             (table_name, last_seen_version, last_polled_at, last_error, last_error_at) \
         VALUES ($1, 0, now(), $2, now()) \
         ON CONFLICT (table_name) DO UPDATE \
             SET last_error    = EXCLUDED.last_error, \
                 last_error_at = EXCLUDED.last_error_at",
    )
    .bind(table)
    .bind(truncated)
    .execute(pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    //! Integration-style tests that need a live PostgreSQL pool live in
    //! `tests/test_sync_watermark.rs`. The unit tests here are pure
    //! string-shape assertions that don't require a runtime — they lock
    //! in the SQL we emit so a refactor doesn't silently regress the
    //! single-row WHERE clause.
    //!
    //! `read_last_seen` against a fresh `legacy_ct_state` row (returns
    //! `0`) is the smoke test specified in the Phase 5.1 plan; it lives
    //! in `tests/test_sync_watermark.rs` so it can use the real pool
    //! infrastructure that the integration suite already wires up.
}
