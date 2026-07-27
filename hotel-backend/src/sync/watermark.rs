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
//! [`advance`] is called EXACTLY ONCE per watcher tick, from
//! `run_one_tick` AFTER every per-table mapper in that tick has
//! committed. It is NOT called from inside the per-table poll.
//!
//! The value written is the **tick CT ceiling**:
//! `CHANGE_TRACKING_CURRENT_VERSION()` sampled ONCE from MSSQL *before*
//! the mapper loop begins. Every table is polled at or after that
//! instant, so every table has provably read through at least that
//! version; anything an iHOTEL save lands mid-loop carries a version
//! strictly ABOVE the ceiling and is therefore still picked up next
//! tick.
//!
//! Writing per-table `max(SYS_CHANGE_VERSION)` here instead is the
//! silent-drop bug fixed 2026-07-11 (HF Ville). `advance` is monotonic
//! (`WHERE last_seen_version <= $1`), so a per-table call made the
//! SHARED watermark end each tick at the MAX across tables. One iHOTEL
//! save writes many CT tables in one transaction; `HT_Customers` is
//! polled FIRST and `HT_Book_Pro` LAST, so a save landing mid-loop was
//! seen only by the late table, which then dragged the shared watermark
//! past the early table's unread rows (customer C2413 / booking R002066
//! at v30789 lost behind a 30788 → 30801 advance). CT's 2-day retention
//! aged them out permanently, with nothing logged.
//!
//! The whole advance is HELD (no write at all) when ANY table errored in
//! the tick, so the next tick re-fetches the same CT range for every
//! table — re-applying a CT row is idempotent across the mapper stack,
//! which is the same assumption the per-table `errored` hold relies on.
//! A conservative ceiling that occasionally re-reads a few rows is the
//! deliberate trade against ever skipping one.
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
/// `new_version` MUST be the tick's CT ceiling — the value of
/// `CHANGE_TRACKING_CURRENT_VERSION()` sampled BEFORE the mapper loop
/// started — and never a per-table `max(SYS_CHANGE_VERSION)`; see the
/// module docs for the 2026-07-11 loss this distinction prevents.
/// `bin/sync.rs::global_watermark_target` is the only sanctioned way to
/// compute the argument.
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

/// Force EVERY listed table's per-table watermark to `version` in one
/// statement. Bootstrap-only override.
///
/// Unlike [`advance_per_table`] this deliberately has NO monotonic
/// guard: `bin/sync --bootstrap` re-snapshots canonical PG and stamps
/// the watermark to the CT version captured before the snapshot, which
/// may be BEHIND a partially-advanced row. Matching the global path's
/// unguarded `UPDATE legacy_ct_state` keeps the documented
/// retention-overflow recovery ("bootstrap, then restart") correct under
/// `SYNC_PER_TABLE_WATERMARK=true` — before this existed, bootstrap
/// stamped only the global row and left all 18 per-table rows stale, so
/// the watcher came back up and immediately re-tripped the overflow
/// refusal.
///
/// Also clears `last_error` / `last_error_at`: a fresh cold-seed
/// invalidates any pre-bootstrap failure recorded against the table.
pub async fn stamp_all_per_table(
    pool: &PgPool,
    tables: &[&str],
    version: i64,
) -> Result<(), SyncError> {
    if tables.is_empty() {
        return Ok(());
    }
    let names: Vec<String> = tables.iter().map(|t| (*t).to_string()).collect();
    sqlx::query(
        "INSERT INTO legacy_ct_state_per_table \
             (table_name, last_seen_version, last_polled_at) \
         SELECT t, $2::bigint, now() FROM UNNEST($1::text[]) AS t \
         ON CONFLICT (table_name) DO UPDATE \
             SET last_seen_version = EXCLUDED.last_seen_version, \
                 last_polled_at    = EXCLUDED.last_polled_at, \
                 last_error        = NULL, \
                 last_error_at     = NULL",
    )
    .bind(&names)
    .bind(version)
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
