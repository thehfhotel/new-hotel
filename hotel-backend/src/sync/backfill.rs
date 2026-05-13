//! Track B5 — one-shot backfill orchestration for the
//! `ht_checkin_rooms` junction.
//!
//! The thin bin (`bin/backfill_checkin_rooms.rs`) wires this module up
//! to a live MSSQL pool + active-folio list and reports the summary.
//! The orchestration itself lives here so the integration suite in
//! `tests/test_backfill_b5.rs` can drive it with hand-built
//! [`CheckInAggregate`] fixtures and an optional MSSQL pool.
//!
//! Per `docs/coexistence/audit-2026-05-13.md` Theme 1 (T1 CRIT-1
//! follow-on). Track B2 (commit 79f8276) made the CT sync mapper emit
//! one `ht_checkin_rooms` row per `HT_CheckIn_Ds` row whenever the
//! watcher re-syncs a folio, but folios that haven't been edited
//! since the B2 deploy still carry only the deprecated header-level
//! `ht_checkins.cin_room_id` and NO junction rows. B5 closes that
//! window by sweeping every still-active legacy folio once and
//! materialising the junction rows via the same mapper the CT watcher
//! uses (`sync::mappers::checkin::apply_checkin_aggregate`).
//!
//! ## Idempotency
//!
//! `apply_checkin_aggregate` short-circuits when the canonical row and
//! the per-room set already match the legacy aggregate
//! (via the mapper's `existing_matches` and `rooms_match` checks).
//! The orchestration in this module re-counts the junction rows pre-
//! and post-apply inside the same tx so it can report "Applied" vs
//! "SkippedIdempotent" cleanly without scraping mapper log lines.

use sqlx::PgPool;

use crate::db::DbPool;
use crate::sync::mappers::apply_checkin_aggregate;
use crate::sync::parent_loader::CheckInAggregate;

/// Per-run counters reported on stdout. Public to the integration
/// suite so per-folio outcomes can be rolled into a summary without
/// scraping logs.
#[derive(Debug, Default, Clone, Copy)]
pub struct BackfillSummary {
    /// Total `Cin_no` values pulled from `HT_CheckIn_H` and processed.
    pub scanned: usize,
    /// Folios where the mapper actually wrote junction rows (or would
    /// have, in `--dry-run`).
    pub applied: usize,
    /// Folios where canonical state already matches legacy — the
    /// mapper's `existing_matches` + `rooms_match` short-circuit
    /// fired. Expected outcome of a re-run.
    pub skipped_idempotent: usize,
    /// Folios where no `ht_checkins.legacy_cin_no = …` row exists in
    /// PG. The CT watcher hasn't seen this folio yet — backfill must
    /// wait for the canonical row to land. Warning, not error.
    pub skipped_missing_pg: usize,
    /// Folios where the mapper returned an error. Each error is
    /// logged via `tracing::warn!`; this just rolls up the count so
    /// operators can see at a glance whether anything failed.
    pub errors: usize,
}

impl BackfillSummary {
    /// Roll a single [`FolioOutcome`] into the running totals. Always
    /// increments `scanned`.
    pub fn record(&mut self, outcome: FolioOutcome) {
        self.scanned += 1;
        match outcome {
            FolioOutcome::Applied => self.applied += 1,
            FolioOutcome::SkippedIdempotent => self.skipped_idempotent += 1,
            FolioOutcome::SkippedMissingPg => self.skipped_missing_pg += 1,
            FolioOutcome::Error => self.errors += 1,
        }
    }

    /// Human-readable stdout report — receptionists see this after
    /// the apply window. The fixed-column layout makes diffing apply
    /// vs dry-run runs trivial in a chat paste.
    pub fn to_report(&self, site_id: &str, dry_run: bool) -> String {
        let mode = if dry_run { "DRY-RUN" } else { "APPLY" };
        format!(
            "
==============================================
 Track B5 backfill summary [{site}] [{mode}]
==============================================
 Folios scanned         : {scanned}
 Junction rows applied  : {applied}
 Skipped (already match): {skipped_idempotent}
 Skipped (no PG row)    : {skipped_missing_pg}
 Errors                 : {errors}
==============================================
",
            site = site_id,
            mode = mode,
            scanned = self.scanned,
            applied = self.applied,
            skipped_idempotent = self.skipped_idempotent,
            skipped_missing_pg = self.skipped_missing_pg,
            errors = self.errors,
        )
    }
}

/// Disposition for a single folio. The orchestrator uses this to roll
/// into a [`BackfillSummary`]. Public so the integration suite can
/// assert per-folio outcomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FolioOutcome {
    /// Mapper inserted / updated junction rows (or would have, in
    /// dry-run).
    Applied,
    /// Canonical state already matches legacy — no changes made.
    SkippedIdempotent,
    /// No `ht_checkins.legacy_cin_no` row exists in PG; the CT
    /// watcher must catch up first.
    SkippedMissingPg,
    /// Tx or mapper failure — already logged at warn level.
    Error,
}

/// Backfill one folio given an already-loaded [`CheckInAggregate`].
///
/// Factored out of the bin's production path so the integration suite
/// can exercise it without a live MSSQL pool. The bin's
/// `backfill_one_folio` is a thin wrapper that loads the aggregate
/// then forwards to this function.
///
/// `mssql` is forwarded to `apply_checkin_aggregate` for the optional
/// parent-booking re-projection side-effect on full check-out. Tests
/// that don't exercise that path pass `None`; the side-effect is a
/// no-op for walk-in folios.
///
/// `dry_run` rolls back the tx after re-counting the post-apply
/// junction rows, so the orchestrator can report "would land N rows"
/// without writing any.
pub async fn backfill_one_folio_with_aggregate(
    pg: &PgPool,
    mssql: Option<&DbPool>,
    cin_no: &str,
    aggregate: &CheckInAggregate,
    dry_run: bool,
) -> FolioOutcome {
    // 1. Confirm a canonical row exists. If not, the CT watcher
    //    hasn't seen this folio yet — backfill must wait for it.
    let pg_present = canonical_checkin_exists(pg, cin_no).await.unwrap_or(false);
    if !pg_present {
        tracing::warn!(
            cin_no,
            "Skipping folio: no ht_checkins.legacy_cin_no row in PG \
             (CT watcher hasn't ingested this folio yet — re-run \
             after the watcher catches up)"
        );
        return FolioOutcome::SkippedMissingPg;
    }

    // 2. Count existing junction rows before the apply tx. Tells us
    //    "Applied" vs "SkippedIdempotent" by comparison after the
    //    mapper has run.
    let existing_room_count = match count_existing_junction_rooms(pg, cin_no).await {
        Ok(n) => n,
        Err(err) => {
            tracing::warn!(
                cin_no,
                error = %err,
                "Skipping folio: failed to count existing junction rows"
            );
            return FolioOutcome::Error;
        }
    };
    let legacy_room_count = aggregate.rooms.len();

    // 3. Fast-path dry-run hint — if cardinality already matches we
    //    can skip the BEGIN…ROLLBACK round trip. We don't trust this
    //    alone in live mode (per-room status fields can still differ
    //    even with matching cardinality), but in dry-run it saves an
    //    unnecessary tx.
    if dry_run && existing_room_count == legacy_room_count && existing_room_count > 0 {
        tracing::debug!(
            cin_no,
            existing_room_count,
            legacy_room_count,
            "Dry-run: row counts already match — would be no-op"
        );
        return FolioOutcome::SkippedIdempotent;
    }

    // 4. Apply through the canonical sync mapper. Same code path the
    //    CT watcher uses; idempotency is enforced inside.
    let mut tx = match pg.begin().await {
        Ok(t) => t,
        Err(err) => {
            tracing::warn!(
                cin_no,
                error = %err,
                "Skipping folio: failed to begin PG transaction"
            );
            return FolioOutcome::Error;
        }
    };

    if let Err(err) = apply_checkin_aggregate(&mut tx, mssql, aggregate, cin_no).await {
        tracing::warn!(
            cin_no,
            error = %err,
            "Skipping folio: mapper returned error"
        );
        let _ = tx.rollback().await;
        return FolioOutcome::Error;
    }

    // 5. Re-count from inside the tx so we don't race another writer.
    let post_count = match count_junction_rooms_in_tx(&mut tx, cin_no).await {
        Ok(n) => n,
        Err(err) => {
            tracing::warn!(
                cin_no,
                error = %err,
                "Skipping folio: failed to re-count junction rows post-apply"
            );
            let _ = tx.rollback().await;
            return FolioOutcome::Error;
        }
    };

    let outcome = if post_count != existing_room_count {
        FolioOutcome::Applied
    } else {
        FolioOutcome::SkippedIdempotent
    };

    if dry_run {
        if let Err(err) = tx.rollback().await {
            tracing::warn!(
                cin_no,
                error = %err,
                "Dry-run: rollback failed (state was never committed)"
            );
        }
        tracing::info!(
            cin_no,
            existing_room_count,
            legacy_room_count,
            post_count,
            "Dry-run: {} junction rows would land",
            post_count.saturating_sub(existing_room_count)
        );
    } else {
        if let Err(err) = tx.commit().await {
            tracing::warn!(
                cin_no,
                error = %err,
                "Failed to commit folio backfill — junction state may be partial"
            );
            return FolioOutcome::Error;
        }
        if outcome == FolioOutcome::Applied {
            tracing::info!(
                cin_no,
                inserted = post_count.saturating_sub(existing_room_count),
                total_rooms = post_count,
                "Backfilled folio"
            );
        }
    }

    outcome
}

/// True when an `ht_checkins.legacy_cin_no = $1` row exists.
async fn canonical_checkin_exists(pg: &PgPool, cin_no: &str) -> Result<bool, sqlx::Error> {
    let row: Option<(i32,)> = sqlx::query_as(
        "SELECT cin_id FROM ht_checkins WHERE legacy_cin_no = $1 LIMIT 1",
    )
    .bind(cin_no)
    .fetch_optional(pg)
    .await?;
    Ok(row.is_some())
}

/// Count `ht_checkin_rooms` rows for a folio (resolved by
/// `legacy_cin_no`, NOT `cin_id`, so the integration suite can pass
/// the legacy string without an extra round-trip).
async fn count_existing_junction_rooms(pg: &PgPool, cin_no: &str) -> Result<usize, sqlx::Error> {
    let (count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*)::bigint \
           FROM ht_checkin_rooms cr \
           JOIN ht_checkins c ON c.cin_id = cr.cr_cin_id \
          WHERE c.legacy_cin_no = $1",
    )
    .bind(cin_no)
    .fetch_one(pg)
    .await?;
    Ok(count as usize)
}

/// Tx-scoped variant — needed for the post-apply re-count so we don't
/// race another writer between the mapper's COMMIT and our SELECT.
async fn count_junction_rooms_in_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    cin_no: &str,
) -> Result<usize, sqlx::Error> {
    let (count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*)::bigint \
           FROM ht_checkin_rooms cr \
           JOIN ht_checkins c ON c.cin_id = cr.cr_cin_id \
          WHERE c.legacy_cin_no = $1",
    )
    .bind(cin_no)
    .fetch_one(&mut **tx)
    .await?;
    Ok(count as usize)
}

// =============================================================================
// Tests — pure helpers. Integration tests cover the orchestration.
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn summary_report_includes_all_counters_and_mode() {
        let s = BackfillSummary {
            scanned: 11,
            applied: 5,
            skipped_idempotent: 3,
            skipped_missing_pg: 2,
            errors: 1,
        };
        let dry = s.to_report("hfhotel", true);
        assert!(dry.contains("DRY-RUN"), "must mark dry-run mode");
        assert!(dry.contains("hfhotel"), "must carry site id");
        assert!(dry.contains("Folios scanned         : 11"));
        assert!(dry.contains("Junction rows applied  : 5"));
        assert!(dry.contains("Skipped (already match): 3"));
        assert!(dry.contains("Skipped (no PG row)    : 2"));
        assert!(dry.contains("Errors                 : 1"));

        let live = s.to_report("hfville", false);
        assert!(live.contains("APPLY"), "must mark apply mode");
        assert!(live.contains("hfville"));
    }

    #[test]
    fn summary_record_increments_correct_buckets() {
        let mut s = BackfillSummary::default();
        s.record(FolioOutcome::Applied);
        s.record(FolioOutcome::Applied);
        s.record(FolioOutcome::SkippedIdempotent);
        s.record(FolioOutcome::SkippedMissingPg);
        s.record(FolioOutcome::Error);
        assert_eq!(s.scanned, 5);
        assert_eq!(s.applied, 2);
        assert_eq!(s.skipped_idempotent, 1);
        assert_eq!(s.skipped_missing_pg, 1);
        assert_eq!(s.errors, 1);
    }
}
