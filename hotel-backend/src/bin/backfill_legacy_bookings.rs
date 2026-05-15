//! One-shot operator backfill: create canonical `ht_bookings` rows for
//! legacy bookings that exist in `ht_bookings_legacy` (the cache mirror)
//! but have no canonical counterpart.
//!
//! ## When to use this
//!
//! Run when canonical `ht_bookings` is materially smaller than the legacy
//! cache `ht_bookings_legacy`. Typical cause is the same as for the
//! sibling `backfill_legacy_checkins`: the CT-watcher bootstrap in April
//! 2026 only seeded "active" rows (a status filter on HT_Book_H), so
//! every booking that had completed/cancelled before bootstrap was
//! skipped, and those rows never produce a subsequent CT event for the
//! watcher to backfill on.
//!
//! Unlike checkins, the booking gap is NOT currently surfaced in
//! `ht_reconcile_log` because the reconcile-hash projection for bookings
//! still matches the cached old-projection hash — the cache short-circuit
//! is doing its job hiding the gap. We therefore source missing PKs
//! directly from the cache-vs-canonical anti-join.
//!
//! The 2026-05-15 audit found ~16,241 cache-only `R*` book_no values
//! spanning 2021–2027 (1,639 from 2026 alone, 5,313 since 2025).
//!
//! ## What it does
//!
//! 1. Reads cache-only PKs:
//!    `SELECT DISTINCT bl.book_no FROM ht_bookings_legacy bl
//!     WHERE NOT EXISTS (SELECT 1 FROM ht_bookings b
//!                       WHERE b.legacy_book_id = bl.book_no)`.
//! 2. For each PK, `parent_loader::load_booking_aggregate(mssql, book_no)`
//!    fetches the legacy aggregate.
//! 3. `sync::mappers::apply_booking_aggregate(tx, &agg, book_no)` projects
//!    + INSERTs into canonical `ht_bookings`.
//!
//! Domain events (`BookingCreated`/`BookingModified`/`BookingCancelled`)
//! are intentionally NOT published — backfilled historical bookings are
//! not new domain events. Matches the no-event semantics of
//! `sync::backfill::backfill_one_folio_with_aggregate` and the sibling
//! `backfill_legacy_checkins`.
//!
//! ## Usage
//!
//! ```text
//! cd hotel-backend
//! DATABASE_URL=postgres://… DB_SERVER=… DB_USER=sa DB_PASSWORD=… DB_NAME=db \
//!   cargo run --release --bin backfill_legacy_bookings -- --dry-run --limit=10
//!
//! # Live run; --limit is optional, defaults to all.
//! cargo run --release --bin backfill_legacy_bookings -- --limit=200
//! ```
//!
//! ## Flags
//!
//! * `--dry-run`     — load aggregates but don't write to PG.
//! * `--limit=N`     — process at most N distinct PKs. Default: all.
//! * `--site-id=ID`  — informational tag in log output. Defaults to
//!                     `SITE_ID` env var or `"hfhotel"`.
//!
//! ## Failure handling
//!
//! Per-PK best-effort. A failure to load the legacy aggregate (no MSSQL
//! row), or a mapper `SyncError` (defer-on-missing customer FK, header
//! NULL fields, etc.), logs a warning and continues to the next PK so a
//! single bad row doesn't stall the run.

use std::env;
use std::time::Instant;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use hotel_backend::config::DbConfig;
use hotel_backend::db::{create_pool, DbPool};
use hotel_backend::sync::mappers::apply_booking_aggregate;
use hotel_backend::sync::parent_loader::load_booking_aggregate;

const PG_POOL_MAX: u32 = 4;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    hotel_backend::secrets::hydrate_env_from_secret_files();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backfill_legacy_bookings=info,hotel_backend=info".into()),
        )
        .init();

    let dry_run = env::args().any(|a| a == "--dry-run");
    let limit: i64 = env::args()
        .find_map(|a| a.strip_prefix("--limit=").map(|n| n.parse::<i64>().ok()).flatten())
        .unwrap_or(i64::MAX);
    let site_id = env::args()
        .find_map(|a| a.strip_prefix("--site-id=").map(str::to_string))
        .or_else(|| env::var("SITE_ID").ok())
        .unwrap_or_else(|| "hfhotel".to_string());

    tracing::info!(site = %site_id, dry_run, limit, "backfill_legacy_bookings — starting");

    let pg = connect_pg().await?;
    let mssql = connect_legacy().await?;

    let pks = fetch_missing_pks(&pg, limit).await?;
    tracing::info!(site = %site_id, candidate_pks = pks.len(), "fetched candidate PKs");

    let start = Instant::now();
    let mut applied = 0usize;
    let mut skipped_no_mssql = 0usize;
    let mut errored = 0usize;
    let total = pks.len();

    for (i, book_no) in pks.iter().enumerate() {
        if i > 0 && i % 100 == 0 {
            tracing::info!(
                progress = format!("{i}/{total}"),
                applied,
                errored,
                skipped_no_mssql,
                elapsed_ms = start.elapsed().as_millis() as u64,
                "in-flight summary"
            );
        }

        let aggregate = match load_booking_aggregate(&mssql, book_no).await {
            Ok(a) => a,
            Err(err) => {
                tracing::warn!(book_no, error = %err, "load_booking_aggregate failed");
                errored += 1;
                continue;
            }
        };

        if !aggregate.is_present() {
            tracing::info!(
                book_no,
                "MSSQL aggregate empty (header missing) — legacy row may have been deleted; \
                 leaving cache row in place for operator review"
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
                tracing::warn!(book_no, error = %err, "pg.begin failed");
                errored += 1;
                continue;
            }
        };

        match apply_booking_aggregate(&mut tx, &aggregate, book_no).await {
            Ok(_event) => {
                if let Err(err) = tx.commit().await {
                    tracing::warn!(book_no, error = %err, "commit failed");
                    errored += 1;
                    continue;
                }
                applied += 1;
            }
            Err(err) => {
                let _ = tx.rollback().await;
                tracing::warn!(book_no, error = %err, "apply_booking_aggregate failed");
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
        errored,
        duration_secs = duration.as_secs(),
        "backfill_legacy_bookings — done"
    );

    println!();
    println!("=== Backfill Summary ===");
    println!("Site:                  {site_id}");
    println!("Mode:                  {}", if dry_run { "DRY RUN" } else { "LIVE" });
    println!("Candidate PKs:         {total}");
    println!("Applied:               {applied}");
    println!("Skipped (no MSSQL):    {skipped_no_mssql}");
    println!("Errored:               {errored}");
    println!("Duration:              {:?}", duration);

    Ok(())
}

/// Cache-vs-canonical anti-join. We don't read from `ht_reconcile_log`
/// because the bookings missing-PG signal is currently hidden by the
/// stable-hash cache short-circuit (see module docs). DISTINCT because
/// `ht_bookings_legacy` is keyed on `(book_no, book_room_type)` and we
/// only want one apply per `book_no`.
async fn fetch_missing_pks(pg: &PgPool, limit: i64) -> Result<Vec<String>, sqlx::Error> {
    sqlx::query_scalar::<_, String>(
        "SELECT DISTINCT bl.book_no \
           FROM ht_bookings_legacy bl \
          WHERE NOT EXISTS ( \
                SELECT 1 FROM ht_bookings b \
                 WHERE b.legacy_book_id = bl.book_no \
          ) \
          ORDER BY bl.book_no \
          LIMIT $1",
    )
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
