//! One-shot operator backfill: populate `ht_customers.legacy_id` for
//! rows mirrored BEFORE migration 055.
//!
//! ## Why this exists
//!
//! Migration 055 (2026-06-11, coexistence audit P1 #6) added
//! `ht_customers.legacy_id` mirroring the legacy `HT_Customers.id`
//! SERIAL PK — the ONLY key a Change Tracking D-row carries, and
//! therefore the only way `apply_soft_delete` can resolve an iHOTEL
//! hard-delete (FrmManageCustomersNew). The customer CT mapper
//! populates the column on every I/U apply going forward, but the
//! ~21-22k canonical rows mirrored pre-055 carry NULL until their next
//! CT touch — and most customers are never edited again. Until then,
//! an iHOTEL delete of such a row only logs a loud WARN ("unresolvable
//! by legacy_id, Cust_no NULL on D") instead of soft-deleting it.
//!
//! This bin closes the gap in one pass: read `(id, Cust_no)` from
//! legacy `HT_Customers` (READ-ONLY — no MSSQL writes whatsoever) and
//! stamp each id onto the matching canonical row:
//!
//! ```sql
//! UPDATE ht_customers SET legacy_id = $1
//!  WHERE legacy_cust_no = $2 AND legacy_id IS NULL
//! ```
//!
//! `legacy_id IS NULL` makes the bin idempotent AND non-destructive:
//! rows the CT mapper already stamped are never touched, so a re-run
//! (or a run racing the live watcher) can't flip an id. A canonical
//! row whose stored `legacy_id` DISAGREES with the legacy pair is
//! counted + warned, never overwritten (operator decides — it is
//! either legacy id-reuse or an earlier mis-mirror).
//!
//! ## Usage
//!
//! ```text
//! cd hotel-backend
//! DATABASE_URL=postgres://… DB_SERVER=… DB_USER=sa DB_PASSWORD=… DB_NAME=db \
//!   cargo run --release --bin backfill_customer_legacy_ids -- --dry-run
//!
//! # Live run (drop --dry-run).
//! cargo run --release --bin backfill_customer_legacy_ids
//! ```
//!
//! Run once per site (HF Hotel `hotelnew`, HF Ville `hotelville` via
//! `NEW_DB_NAME` / `DATABASE_URL` + the Ville MSSQL env), after the
//! migration-055 deploy.
//!
//! ## Flags
//!
//! * `--dry-run` — classify and report; no PG writes.
//! * `--chunk=N` — PG transaction batch size. Default: 500.
//! * `--site-id=ID` — informational tag in log output. Defaults to
//!   `SITE_ID` env var or `"hfhotel"`.
//!
//! ## Summary semantics
//!
//! * `updated` — canonical row found with NULL legacy_id; stamped.
//! * `already_set` — canonical row already carries the same id
//!   (CT mapper or a previous run got there first).
//! * `mismatch` — canonical row carries a DIFFERENT id; left
//!   alone, logged loudly per row.
//! * `no_canonical_row` — legacy Cust_no has no canonical counterpart
//!   (never mirrored — e.g. pre-bootstrap deletes).
//! * `skipped_null_cust` — legacy row with NULL/empty Cust_no (cannot
//!   be matched; D-resolution for those was already impossible
//!   pre-055).

use std::env;
use std::time::Instant;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

const PG_POOL_MAX: u32 = 4;
const DEFAULT_CHUNK: usize = 500;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    hotel_backend::secrets::hydrate_env_from_secret_files();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "backfill_customer_legacy_ids=info,hotel_backend=info".into()
            }),
        )
        .init();

    let dry_run = env::args().any(|a| a == "--dry-run");
    let chunk: usize = env::args()
        .find_map(|a| a.strip_prefix("--chunk=").and_then(|n| n.parse::<usize>().ok()))
        .filter(|n| *n > 0)
        .unwrap_or(DEFAULT_CHUNK);
    let site_id = env::args()
        .find_map(|a| a.strip_prefix("--site-id=").map(str::to_string))
        .or_else(|| env::var("SITE_ID").ok())
        .unwrap_or_else(|| "hfhotel".to_string());

    tracing::info!(site = %site_id, dry_run, chunk, "backfill_customer_legacy_ids — starting");

    let pg = connect_pg().await?;
    let (pairs, skipped_null_cust) = fetch_legacy_pairs().await?;
    tracing::info!(site = %site_id, legacy_rows = pairs.len(), "fetched legacy (id, Cust_no) pairs");

    let start = Instant::now();
    let mut stats = Stats {
        skipped_null_cust,
        ..Stats::default()
    };
    let total = pairs.len();

    for (i, batch) in pairs.chunks(chunk).enumerate() {
        run_chunk(&pg, batch, dry_run, &mut stats).await?;
        let done = (i * chunk + batch.len()).min(total);
        if done < total {
            tracing::info!(
                progress = format!("{done}/{total}"),
                updated = stats.updated,
                already_set = stats.already_set,
                mismatch = stats.mismatch,
                no_canonical_row = stats.no_canonical_row,
                elapsed_ms = start.elapsed().as_millis() as u64,
                "in-flight summary"
            );
        }
    }

    let duration = start.elapsed();
    tracing::info!(
        site = %site_id,
        dry_run,
        legacy_rows = total,
        updated = stats.updated,
        already_set = stats.already_set,
        mismatch = stats.mismatch,
        no_canonical_row = stats.no_canonical_row,
        skipped_null_cust = stats.skipped_null_cust,
        duration_secs = duration.as_secs(),
        "backfill_customer_legacy_ids — done"
    );

    println!();
    println!("=== Backfill Summary (ht_customers.legacy_id) ===");
    println!("Site:                   {site_id}");
    println!("Mode:                   {}", if dry_run { "DRY RUN" } else { "LIVE" });
    println!("Legacy rows seen:       {total}");
    println!(
        "Updated:                {}{}",
        stats.updated,
        if dry_run { " (would update)" } else { "" }
    );
    println!("Already set:            {}", stats.already_set);
    println!("Mismatch (left alone):  {}", stats.mismatch);
    println!("No canonical row:       {}", stats.no_canonical_row);
    println!("Skipped (NULL Cust_no): {}", stats.skipped_null_cust);
    println!("Duration:               {duration:?}");
    if stats.mismatch > 0 {
        println!();
        println!(
            "WARNING: {} canonical row(s) hold a DIFFERENT legacy_id than the \
             live legacy pair — see the per-row warnings above. Likely legacy \
             id-reuse; resolve manually before trusting D-row deletes for them.",
            stats.mismatch
        );
    }

    Ok(())
}

#[derive(Default, Debug)]
struct Stats {
    updated: u64,
    already_set: u64,
    mismatch: u64,
    no_canonical_row: u64,
    skipped_null_cust: u64,
}

/// Process one batch inside a single PG transaction (chunked commits
/// so an interruption loses at most one batch of progress; re-running
/// re-classifies the committed rows as `already_set`).
async fn run_chunk(
    pg: &PgPool,
    batch: &[(i32, String)],
    dry_run: bool,
    stats: &mut Stats,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut tx = pg.begin().await?;

    for (legacy_id, cust_no) in batch {
        // Classify first so dry-run reports identically to a live run.
        let existing: Option<(Option<i32>,)> =
            sqlx::query_as("SELECT legacy_id FROM ht_customers WHERE legacy_cust_no = $1")
                .bind(cust_no)
                .fetch_optional(&mut *tx)
                .await?;

        match existing {
            None => stats.no_canonical_row += 1,
            Some((Some(stored),)) if stored == *legacy_id => stats.already_set += 1,
            Some((Some(stored),)) => {
                stats.mismatch += 1;
                tracing::warn!(
                    cust_no,
                    legacy_id,
                    stored_legacy_id = stored,
                    "canonical row already carries a DIFFERENT legacy_id — \
                     leaving it alone (legacy id-reuse?); resolve manually"
                );
            }
            Some((None,)) => {
                if dry_run {
                    stats.updated += 1;
                } else {
                    let affected = sqlx::query(
                        "UPDATE ht_customers SET legacy_id = $1 \
                          WHERE legacy_cust_no = $2 AND legacy_id IS NULL",
                    )
                    .bind(legacy_id)
                    .bind(cust_no)
                    .execute(&mut *tx)
                    .await?
                    .rows_affected();
                    if affected > 0 {
                        stats.updated += affected;
                    } else {
                        // Raced the live CT watcher between SELECT and
                        // UPDATE — the mapper stamped it first. Count
                        // as already-set; the guard kept us harmless.
                        stats.already_set += 1;
                    }
                }
            }
        }
    }

    if dry_run {
        tx.rollback().await?;
    } else {
        tx.commit().await?;
    }
    Ok(())
}

/// Read `(id, Cust_no)` from legacy `HT_Customers` — READ-ONLY.
/// Returns the matchable pairs plus the count of rows with NULL id /
/// NULL-or-empty `Cust_no` (those can never be matched to a canonical
/// row, which is keyed on `legacy_cust_no`).
async fn fetch_legacy_pairs(
) -> Result<(Vec<(i32, String)>, u64), Box<dyn std::error::Error + Send + Sync>> {
    let config = hotel_backend::config::DbConfig::from_env();
    let server = config.server.clone();
    let port = config.port;
    let pool = hotel_backend::db::create_pool(&config)
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { e.to_string().into() })?;
    tracing::info!("Connecting to legacy SQL Server at {server}:{port}");

    let mut conn = pool.get().await?;
    let rows = conn
        .simple_query("SELECT id, Cust_no FROM HT_Customers ORDER BY id")
        .await?
        .into_first_result()
        .await?;

    let mut out = Vec::with_capacity(rows.len());
    let mut skipped_null = 0u64;
    for row in &rows {
        let Some(id) = row.get::<i32, _>("id") else {
            skipped_null += 1;
            continue;
        };
        let cust_no = row
            .get::<&str, _>("Cust_no")
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let Some(cust_no) = cust_no else {
            skipped_null += 1;
            continue;
        };
        out.push((id, cust_no.to_string()));
    }
    if skipped_null > 0 {
        tracing::warn!(
            skipped_null,
            "legacy rows with NULL id / NULL-or-empty Cust_no skipped \
             (cannot be matched to a canonical row)"
        );
    }
    Ok((out, skipped_null))
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
