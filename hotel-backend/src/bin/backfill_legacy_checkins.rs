//! One-shot operator backfill: create canonical `ht_checkins` rows for
//! legacy check-ins flagged as `missing_pg` in `ht_reconcile_log`.
//!
//! ## When to use this
//!
//! Run when the reconcile drift digest shows a backlog of
//! `divergence_kind = 'missing_pg'` entries for `table_name = 'checkins'`.
//! Typical cause: the CT watcher's initial bootstrap only seeded
//! "currently active" check-ins, and historical check-ins that were
//! already closed at bootstrap time never produced a subsequent CT
//! event to backfill them.
//!
//! The 2026-05-15 incident: PR-#114 (Bug B reconcile-hash projection
//! alignment) restored visibility on ~19k pre-bootstrap CH21–CH26
//! check-ins whose MSSQL hashes had been stably ack'd in
//! `ht_checkins_legacy.sync_hash` under the old projection, masking the
//! canonical gap. This bin closes the gap by replaying the legacy
//! aggregate through the CT mapper's INSERT path.
//!
//! ## What it does
//!
//! 1. Reads `DISTINCT legacy_pk` from `ht_reconcile_log` where
//!    `table_name = 'checkins' AND divergence_kind = 'missing_pg' AND
//!     resolved_at IS NULL`.
//! 2. For each PK, calls `parent_loader::load_checkin_aggregate(...)`
//!    to fetch the MSSQL `HT_CheckIn_H` / `HT_CheckIn_Ds` /
//!    `HT_CheckIn_Pay` aggregate.
//! 3. Calls `sync::mappers::apply_checkin_aggregate(...)` to project +
//!    INSERT into canonical `ht_checkins` + `ht_checkin_rooms` ONLY —
//!    it does NOT touch `ht_payments` (stale claim corrected 2026-07-31,
//!    issue #278 side-finding; `ht_payments` is populated separately by
//!    the `HT_Receipt_H` mapper — see `bin/backfill_receipt_payments`
//!    for that table's own backfill-ordering-race gap). The mapper is
//!    idempotent — re-runs short-circuit.
//! 4. On successful apply, marks every `ht_reconcile_log` row for that
//!    PK as `resolved_at = NOW()`.
//!
//! Domain events (`CheckInCreated`) are intentionally NOT published —
//! backfilled historical check-ins are not new events from a domain
//! perspective. Matches the no-event semantics of
//! `sync::backfill::backfill_one_folio_with_aggregate`.
//!
//! ## Usage
//!
//! ```text
//! cd hotel-backend
//! DATABASE_URL=postgres://… DB_SERVER=… DB_USER=sa DB_PASSWORD=… DB_NAME=db \
//!   cargo run --release --bin backfill_legacy_checkins -- --dry-run --limit=10
//!
//! # Live run (drop --dry-run); --limit is optional, defaults to all.
//! cargo run --release --bin backfill_legacy_checkins -- --limit=200
//! ```
//!
//! ## Flags
//!
//! * `--dry-run`       — load aggregates but don't write to PG. Reports
//!                       what would be applied.
//! * `--limit=N`       — process at most N distinct PKs. Default: all.
//! * `--site-id=ID`    — informational tag in log output. Defaults to
//!                       `SITE_ID` env var or `"hfhotel"`.
//! * `--include-value` — also drain `divergence_kind = 'value'` rows
//!                       (CT-watcher missed-UPDATE remediation). Default
//!                       off: only `missing_pg`. The same idempotent
//!                       `apply_checkin_aggregate` UPSERT path
//!                       re-projects canonical from current MSSQL state.
//!
//! ## Failure handling
//!
//! Per-PK best-effort. A failure to load the legacy aggregate, or a
//! mapper `SyncError` (defer-on-missing FK, etc.), logs and continues
//! to the next PK. The reconcile_log row stays unresolved so the next
//! sweep tick re-evaluates it (and so the alert keeps surfacing it
//! until the operator addresses the underlying issue).

use std::env;
use std::time::Instant;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use hotel_backend::config::DbConfig;
use hotel_backend::db::{create_pool, DbPool};
use hotel_backend::sync::mappers::{apply_booking_aggregate, apply_checkin_aggregate};
use hotel_backend::sync::parent_loader::{load_booking_aggregate, load_checkin_aggregate};
use hotel_backend::sync::MappableRow;

const PG_POOL_MAX: u32 = 4;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    hotel_backend::secrets::hydrate_env_from_secret_files();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backfill_legacy_checkins=info,hotel_backend=info".into()),
        )
        .init();

    let dry_run = env::args().any(|a| a == "--dry-run");
    let include_value = env::args().any(|a| a == "--include-value");
    let limit: i64 = env::args()
        .find_map(|a| a.strip_prefix("--limit=").map(|n| n.parse::<i64>().ok()).flatten())
        .unwrap_or(i64::MAX);
    let site_id = env::args()
        .find_map(|a| a.strip_prefix("--site-id=").map(str::to_string))
        .or_else(|| env::var("SITE_ID").ok())
        .unwrap_or_else(|| "hfhotel".to_string());

    tracing::info!(
        site = %site_id, dry_run, include_value, limit,
        "backfill_legacy_checkins — starting"
    );

    let pg = connect_pg().await?;
    let mssql = connect_legacy().await?;

    let pks = fetch_candidate_pks(&pg, limit, include_value).await?;
    tracing::info!(site = %site_id, candidate_pks = pks.len(), "fetched candidate PKs");

    let start = Instant::now();
    let mut applied = 0usize;
    let mut skipped_no_mssql = 0usize;
    let mut deferred = 0usize;
    let mut errored = 0usize;
    let total = pks.len();

    for (i, cin_no) in pks.iter().enumerate() {
        if i > 0 && i % 100 == 0 {
            tracing::info!(
                progress = format!("{i}/{total}"),
                applied,
                errored,
                skipped_no_mssql,
                deferred,
                elapsed_ms = start.elapsed().as_millis() as u64,
                "in-flight summary"
            );
        }

        let aggregate = match load_checkin_aggregate(&mssql, cin_no).await {
            Ok(a) => a,
            Err(err) => {
                tracing::warn!(cin_no, error = %err, "load_checkin_aggregate failed");
                errored += 1;
                continue;
            }
        };

        if !aggregate.is_present() {
            tracing::info!(
                cin_no,
                "MSSQL aggregate empty (header missing) — legacy row may have been deleted; \
                 leaving reconcile_log unresolved for operator review"
            );
            skipped_no_mssql += 1;
            continue;
        }

        if dry_run {
            applied += 1;
            continue;
        }

        let mut tx = match pg.begin().await {
            Ok(t) => t,
            Err(err) => {
                tracing::warn!(cin_no, error = %err, "pg.begin failed");
                errored += 1;
                continue;
            }
        };

        match apply_checkin_aggregate(&mut tx, Some(&mssql), &aggregate, cin_no).await {
            Ok(_event) => {
                // `Ok` is ambiguous: it covers both real INSERT/UPDATE
                // success AND defer-on-missing-FK (the mapper logs a
                // warning + returns `Ok(None)` when a parent booking or
                // room isn't mirrored yet — see `apply_checkin_aggregate`
                // line ~341 and ~362). Mark the reconcile_log row
                // resolved ONLY if a canonical `ht_checkins` row exists
                // for this PK after the apply. Otherwise the canonical
                // row is still missing and the next reconcile tick will
                // correctly re-flag it as missing_pg.
                let canonical_present: bool = sqlx::query_scalar::<_, bool>(
                    "SELECT EXISTS(SELECT 1 FROM ht_checkins WHERE legacy_cin_no = $1)",
                )
                .bind(cin_no)
                .fetch_one(&mut *tx)
                .await
                .unwrap_or(false);

                if !canonical_present {
                    // Cascade attempt (2026-05-18 root-cause fix for the 18 stuck
                    // missing_pg orphan-booking-header check-ins). When the apply
                    // defers on a missing parent booking FK, extract Cin_Book_no
                    // from the loaded MSSQL header, load + apply the parent
                    // booking aggregate (header-only is now an accepted shape per
                    // `apply_booking_aggregate` doc contract), then retry the
                    // checkin apply. Idempotent: if the parent is already in
                    // canonical or MSSQL has no row, the cascade is a no-op and
                    // we still fall through to the original defer-and-skip path.
                    let parent_book_no = aggregate.header.as_ref().and_then(|h| {
                        h.try_get_str("Cin_Book_no")
                            .ok()
                            .flatten()
                            .filter(|s| !s.is_empty())
                            .map(|s| s.to_string())
                    });
                    let mut cascade_applied = false;
                    if let Some(book_no) = parent_book_no.as_deref() {
                        match load_booking_aggregate(&mssql, book_no).await {
                            Ok(book_agg) if book_agg.is_present() => {
                                if let Err(err) = apply_booking_aggregate(
                                    &mut tx,
                                    Some(&mssql),
                                    &book_agg,
                                    book_no,
                                )
                                .await
                                {
                                    tracing::warn!(
                                        cin_no,
                                        book_no,
                                        error = %err,
                                        "cascade: apply_booking_aggregate failed"
                                    );
                                } else {
                                    tracing::info!(
                                        cin_no,
                                        book_no,
                                        "cascade: applied parent booking; retrying checkin"
                                    );
                                    cascade_applied = true;
                                }
                            }
                            Ok(_) => {
                                tracing::info!(
                                    cin_no,
                                    book_no,
                                    "cascade: parent booking has no MSSQL header; cannot mirror"
                                );
                            }
                            Err(err) => {
                                tracing::warn!(
                                    cin_no,
                                    book_no,
                                    error = %err,
                                    "cascade: load_booking_aggregate failed"
                                );
                            }
                        }
                    }

                    if cascade_applied {
                        if let Err(err) = apply_checkin_aggregate(
                            &mut tx, Some(&mssql), &aggregate, cin_no,
                        )
                        .await
                        {
                            tracing::warn!(
                                cin_no,
                                error = %err,
                                "cascade-retry: apply_checkin_aggregate failed"
                            );
                        }
                    }

                    let canonical_present_after: bool = sqlx::query_scalar::<_, bool>(
                        "SELECT EXISTS(SELECT 1 FROM ht_checkins WHERE legacy_cin_no = $1)",
                    )
                    .bind(cin_no)
                    .fetch_one(&mut *tx)
                    .await
                    .unwrap_or(false);

                    if !canonical_present_after {
                        tracing::info!(
                            cin_no,
                            parent_book_no,
                            "apply returned Ok but canonical row still missing — \
                             cascade did not resolve; leaving reconcile_log unresolved \
                             for operator review"
                        );
                        let _ = tx.rollback().await;
                        deferred += 1;
                        continue;
                    }
                    // Fall through to the resolve-update path: cascade
                    // succeeded, canonical row now exists, commit + mark
                    // reconcile_log resolved as if the apply had worked first try.
                }

                // Mark every unresolved reconcile_log row for this PK
                // resolved. We do this inside the same TX as the apply
                // so a rollback below also rolls back the resolution.
                if let Err(err) = sqlx::query(
                    "UPDATE ht_reconcile_log \
                        SET resolved_at = NOW() \
                      WHERE table_name = 'checkins' \
                        AND legacy_pk = $1 \
                        AND resolved_at IS NULL",
                )
                .bind(cin_no)
                .execute(&mut *tx)
                .await
                {
                    tracing::warn!(cin_no, error = %err, "resolve update failed; rolling back");
                    let _ = tx.rollback().await;
                    errored += 1;
                    continue;
                }

                if let Err(err) = tx.commit().await {
                    tracing::warn!(cin_no, error = %err, "commit failed");
                    errored += 1;
                    continue;
                }

                applied += 1;
            }
            Err(err) => {
                let _ = tx.rollback().await;
                tracing::warn!(cin_no, error = %err, "apply_checkin_aggregate failed");
                errored += 1;
            }
        }
    }

    let duration = start.elapsed();
    tracing::info!(
        site = %site_id,
        dry_run,
        candidate_pks = total,
        applied,
        skipped_no_mssql,
        deferred,
        errored,
        duration_secs = duration.as_secs(),
        "backfill_legacy_checkins — done"
    );

    println!();
    println!("=== Backfill Summary ===");
    println!("Site:                  {site_id}");
    println!("Mode:                  {}", if dry_run { "DRY RUN" } else { "LIVE" });
    println!("Candidate PKs:         {total}");
    println!("Applied:               {applied}");
    println!("Skipped (no MSSQL):    {skipped_no_mssql}");
    println!("Deferred (FK missing): {deferred}");
    println!("Errored:               {errored}");
    println!("Duration:              {:?}", duration);

    Ok(())
}

/// Read distinct PKs from `ht_reconcile_log` for the bin to backfill.
///
/// Default: only `missing_pg` (the original missing-canonical-row drain).
/// With `include_value=true`: also includes `value` drift — for cases
/// where the CT watcher missed an UPDATE and canonical PG holds stale
/// state. The same `apply_checkin_aggregate` path is idempotent: it
/// UPSERTs based on existing canonical state, so re-applying for a value
/// drift updates canonical to match MSSQL. Used 2026-05-18 to drain 24
/// stuck value drifts from the CT-silent-drop incident (see PR #128).
async fn fetch_candidate_pks(
    pg: &PgPool,
    limit: i64,
    include_value: bool,
) -> Result<Vec<String>, sqlx::Error> {
    let kind_filter = if include_value {
        "AND divergence_kind IN ('missing_pg', 'value')"
    } else {
        "AND divergence_kind = 'missing_pg'"
    };
    let sql = format!(
        "SELECT DISTINCT legacy_pk \
           FROM ht_reconcile_log \
          WHERE table_name = 'checkins' \
            {kind_filter} \
            AND resolved_at IS NULL \
          ORDER BY legacy_pk \
          LIMIT $1"
    );
    sqlx::query_scalar::<_, String>(sqlx::AssertSqlSafe(&*sql))
        .bind(limit)
        .fetch_all(pg)
        .await
}

async fn connect_pg() -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {
    let url = env::var("DATABASE_URL")
        .or_else(|_| env::var("NEW_DATABASE_URL"))
        .map_err(|_| "DATABASE_URL or NEW_DATABASE_URL must be set")?;
    let pool = PgPoolOptions::new()
        .max_connections(PG_POOL_MAX)
        .connect(&url)
        .await?;
    tracing::info!("Connected to PostgreSQL");
    Ok(pool)
}

async fn connect_legacy() -> Result<DbPool, Box<dyn std::error::Error + Send + Sync>> {
    let mut config = DbConfig::from_env();
    config.pool_max = 4;
    let server = config.server.clone();
    let pool = create_pool(&config).await.map_err(|e| format!("MSSQL pool: {e}"))?;
    tracing::info!(server = %server, "Connected to legacy MSSQL");
    Ok(pool)
}
