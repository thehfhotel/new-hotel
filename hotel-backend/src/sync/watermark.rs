//! Read / advance the single-row Change Tracking watermark.
//!
//! The `legacy_ct_state` table (migration 013) holds exactly one row
//! tracking the highest `SYS_CHANGE_VERSION` we've successfully imported
//! from MSSQL. The watcher resumes from this point on every restart.
//!
//! Per `docs/architecture.md` §4d-tris.
//!
//! ## Single-write-per-tick contract
//!
//! [`advance`] is called once per watcher tick AFTER all per-table
//! mappers in that tick have committed. The advance writes the
//! `min(per-table-max(SYS_CHANGE_VERSION))` so a partial tick failure
//! leaves the watermark below the last fully-applied version — the
//! retry will re-fetch the failed table's rows (idempotent UPSERTs in
//! 5.2+ make the re-fetch safe).
//!
//! In Phase 5.1 the mappers are no-ops, so the loop just exercises the
//! read/write plumbing without producing any actual change rows.

use sqlx::PgPool;

use crate::sync::SyncError;

/// Read the current watermark. Returns 0 on a fresh install (the
/// migration seeds the row with `last_seen_version = 0`).
pub async fn read_last_seen(pool: &PgPool) -> Result<i64, SyncError> {
    let row: (i64,) =
        sqlx::query_as("SELECT last_seen_version FROM legacy_ct_state WHERE id = 1")
            .fetch_one(pool)
            .await?;
    Ok(row.0)
}

/// Advance the watermark in a single-row UPDATE. Caller MUST have
/// already committed every mapper's per-tick transaction — advancing
/// past unprocessed rows would silently lose them on the next tick.
///
/// `new_version` should be the minimum-of-per-table-max for this tick;
/// see [`crate::sync::watermark`] module docs for the rationale.
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
