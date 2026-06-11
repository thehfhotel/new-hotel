//! One-shot backfill: legacy `HT_CheckIn_H` / `HT_CheckIn_Ds` →
//! canonical `ht_checkin_rooms` junction rows for ACTIVE folios.
//!
//! Track B5 (`docs/coexistence/audit-2026-05-13.md` Theme 1, T1 CRIT-1
//! follow-on). The orchestration core lives in
//! [`hotel_backend::sync::backfill`] — this bin is a thin wrapper that
//! wires it up to a live MSSQL pool + active-folio list and prints
//! the summary report. The split lets the integration suite in
//! `tests/test_backfill_b5.rs` exercise the orchestration without a
//! live MSSQL pool.
//!
//! ## Why this is a separate bin (not a one-off migration)
//!
//! - It needs both legacy MSSQL (read aggregate) AND canonical PG
//!   (UPSERT junction) — `scripts/migrate.sh` only knows PG.
//! - Idempotent re-runs are valuable: receptionists can run it once,
//!   see the summary, and re-run after spot-checking a folio without
//!   risk.
//! - It explicitly REUSES the sync mapper
//!   (`sync::mappers::checkin::apply_checkin_aggregate`) — no
//!   duplicated projection logic.
//!
//! ## Usage
//!
//! ```text
//! # Dry run — log what would change, write nothing.
//! cd hotel-backend
//! DATABASE_URL=postgres://… DB_SERVER=… DB_USER=sa DB_PASSWORD=… DB_NAME=db \
//!   SITE_ID=hfhotel \
//!   cargo run --release --bin backfill_checkin_rooms -- --dry-run
//!
//! # Apply — same command without --dry-run.
//! cd hotel-backend
//! DATABASE_URL=postgres://… DB_SERVER=… DB_USER=sa DB_PASSWORD=… DB_NAME=db \
//!   SITE_ID=hfhotel \
//!   cargo run --release --bin backfill_checkin_rooms
//!
//! # One-shot historical backfill — include already-checked-out folios.
//! # Used 2026-05-18 to drain the residual cardinality-drift PKs that
//! # checked out before Track B5's junction-write path landed.
//! cargo run --release --bin backfill_checkin_rooms -- --include-inactive
//! ```
//!
//! ## Flags
//!
//! * `--dry-run`          — load aggregates but don't write to PG.
//! * `--include-inactive` — drop the active-folio filter. Scans every
//!                          non-cancelled `HT_CheckIn_H` header,
//!                          including already-checked-out folios. Use
//!                          for one-shot historical backfills only.
//!
//! See `docs/coexistence/RUNBOOK-b5-backfill.md` for the full
//! receptionist-coordinated apply procedure.

use std::env;
use std::sync::Arc;
use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::sync::Semaphore;

use hotel_backend::config::{DbConfig, SiteConfig};
use hotel_backend::db::{create_pool, DbPool};
use hotel_backend::sync::backfill::{
    backfill_one_folio_with_aggregate, BackfillSummary, FolioOutcome,
};
use hotel_backend::sync::parent_loader::load_checkin_aggregate;

/// Default in-flight folio cap. Empirically chosen — the PG pool caps
/// at 8 connections (`PG_POOL_MAX`) so anything beyond this just
/// blocks on pool acquisition. Override at runtime via
/// `BACKFILL_CONCURRENCY`.
const DEFAULT_BACKFILL_CONCURRENCY: usize = 8;

/// PG connection-pool size for this one-shot bin. Matches
/// `DEFAULT_BACKFILL_CONCURRENCY` so the semaphore never blocks on
/// sqlx pool acquisition; both knobs scale together if the operator
/// raises the env override.
const PG_POOL_MAX: u32 = 8;

/// Legacy MSSQL bb8 pool size. Same shape as the PG pool — one
/// concurrent connection per in-flight folio so neither side becomes
/// the bottleneck before the other.
const MSSQL_POOL_MAX: u32 = 8;

/// `HT_CheckIn_H.Cin_status` literal for an active (non-cancelled)
/// folio. Verbatim from the legacy schema per the user's standing
/// constraint — matches the cancelled-folio literal in
/// `sync::mappers::checkin` inversely.
const CIN_STATUS_ACTIVE: &str = "ปกติ";

/// `HT_CheckIn_H.Cin_status` literal for a cancelled folio. Mirrors the
/// private const in `sync::mappers::checkin`. Used only by the
/// `--include-inactive` path to exclude folios that `apply_cancelled`
/// would just delete anyway.
const CIN_STATUS_CANCELLED: &str = "ยกเลิก";

/// `HT_CheckIn_Ds.Cin_Room_Status` literal for a room that has
/// already checked out. Per-room — an active folio can carry a mix of
/// `'เข้าพัก'` (still occupying) and `'Check-Out'` (departed) rows.
/// Folios where EVERY Ds row is `'Check-Out'` are no longer active in
/// the operational sense even though the header may still say `'ปกติ'`.
const CIN_ROOM_STATUS_CHECKED_OUT: &str = "Check-Out";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    // Security audit 2026-05-14: hydrate Docker secret files into env vars
    // before any consumer reads them. See `hotel_backend::secrets`.
    hotel_backend::secrets::hydrate_env_from_secret_files();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backfill_checkin_rooms=info,hotel_backend=info".into()),
        )
        .init();

    let dry_run = env::args().any(|a| a == "--dry-run");
    let include_inactive = env::args().any(|a| a == "--include-inactive");
    let site = SiteConfig::from_env();
    let concurrency = parse_concurrency();

    tracing::info!(
        site = %site.id,
        dry_run,
        include_inactive,
        concurrency,
        "Track B5 backfill — starting one-shot sweep over legacy folios"
    );

    let pg = connect_pg().await?;
    let mssql = connect_legacy().await?;

    let cin_nos = fetch_candidate_cin_nos(&mssql, include_inactive).await?;
    tracing::info!(
        site = %site.id,
        active_folios = cin_nos.len(),
        "Fetched candidate folios from legacy HT_CheckIn_H"
    );

    let summary = run_backfill(&pg, &mssql, &cin_nos, concurrency, dry_run).await;

    tracing::info!(
        site = %site.id,
        scanned = summary.scanned,
        applied = summary.applied,
        skipped_idempotent = summary.skipped_idempotent,
        skipped_missing_pg = summary.skipped_missing_pg,
        errors = summary.errors,
        dry_run,
        "Track B5 backfill — done"
    );

    println!("{}", summary.to_report(&site.id, dry_run));

    Ok(())
}

// =============================================================================
// Orchestrator
// =============================================================================

async fn run_backfill(
    pg: &PgPool,
    mssql: &DbPool,
    cin_nos: &[String],
    concurrency: usize,
    dry_run: bool,
) -> BackfillSummary {
    let semaphore = Arc::new(Semaphore::new(concurrency.max(1)));
    let mut handles = Vec::with_capacity(cin_nos.len());

    for cin_no in cin_nos {
        let permit = semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("semaphore never closed");
        let pg = pg.clone();
        let mssql = mssql.clone();
        let cin_no = cin_no.clone();
        handles.push(tokio::spawn(async move {
            let _permit = permit;
            backfill_one_folio(&pg, &mssql, &cin_no, dry_run).await
        }));
    }

    let mut summary = BackfillSummary::default();
    for handle in handles {
        match handle.await {
            Ok(outcome) => summary.record(outcome),
            Err(join_err) => {
                tracing::error!(error = %join_err, "Backfill task panicked");
                summary.record(FolioOutcome::Error);
            }
        }
    }
    summary
}

/// Thin wrapper that loads the legacy aggregate for one folio then
/// forwards to [`backfill_one_folio_with_aggregate`]. The split keeps
/// the MSSQL-load step out of the integration suite (which builds
/// aggregates by hand).
async fn backfill_one_folio(
    pg: &PgPool,
    mssql: &DbPool,
    cin_no: &str,
    dry_run: bool,
) -> FolioOutcome {
    let aggregate = match load_checkin_aggregate(mssql, cin_no).await {
        Ok(a) => a,
        Err(err) => {
            tracing::warn!(
                cin_no,
                error = %err,
                "Skipping folio: failed to load legacy aggregate"
            );
            return FolioOutcome::Error;
        }
    };
    backfill_one_folio_with_aggregate(pg, Some(mssql), cin_no, &aggregate, dry_run).await
}

// =============================================================================
// Legacy reads
// =============================================================================

/// Pull the list of active folios from legacy MSSQL. An active folio
/// is one whose `HT_CheckIn_H.Cin_status` is the verbatim Thai literal
/// `'ปกติ'` AND that has at least one `HT_CheckIn_Ds` row whose
/// `Cin_Room_Status` is NOT `'Check-Out'` (i.e. at least one room is
/// still occupying). Folios where every Ds row already checked out
/// are excluded — backfilling them adds rows the receptionist never
/// sees (no in-flight stay) and the dashboard read-path correctly
/// ignores them anyway.
/// Fetch candidate Cin_no values from legacy `HT_CheckIn_H`.
///
/// Default (`include_inactive=false`): the original active-folio filter
/// — status = active AND at least one room not yet checked-out. This is
/// what the receptionist sees in iHOTEL as "in-flight stays".
///
/// `include_inactive=true`: drop both predicates and return EVERY
/// non-cancelled header. Used for one-shot historical backfills of
/// already-checked-out folios that flagged as `divergence_kind=cardinality`
/// because no per-room CT events were emitted before Track B's
/// junction-write path landed. 2026-05-18 incident remediation —
/// without this flag the residual 4 PKs (CH26-005340/5350/5377/5430)
/// stay flagged forever. Cancellation status is still excluded because
/// `apply_cancelled` would just delete the canonical row anyway.
async fn fetch_candidate_cin_nos(
    mssql: &DbPool,
    include_inactive: bool,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = mssql.get().await?;
    let sql = if include_inactive {
        format!(
            r#"
            SELECT DISTINCT h.Cin_no
              FROM HT_CheckIn_H h
             WHERE h.Cin_status <> '{cancelled}'
               AND EXISTS (SELECT 1 FROM HT_CheckIn_Ds d WHERE d.Cin_No = h.Cin_no)
             ORDER BY h.Cin_no
            "#,
            cancelled = CIN_STATUS_CANCELLED,
        )
    } else {
        format!(
            r#"
            SELECT DISTINCT h.Cin_no
              FROM HT_CheckIn_H h
             WHERE h.Cin_status = '{active}'
               AND EXISTS (
                   SELECT 1
                     FROM HT_CheckIn_Ds d
                    WHERE d.Cin_No = h.Cin_no
                      AND d.Cin_Room_Status <> '{checked_out}'
               )
             ORDER BY h.Cin_no
            "#,
            active = CIN_STATUS_ACTIVE,
            checked_out = CIN_ROOM_STATUS_CHECKED_OUT,
        )
    };

    let rows = conn.simple_query(&sql).await?.into_first_result().await?;

    let mut cin_nos = Vec::with_capacity(rows.len());
    for row in &rows {
        if let Some(cin_no) = row.get::<&str, _>("Cin_no") {
            let trimmed = cin_no.trim();
            if !trimmed.is_empty() {
                cin_nos.push(trimmed.to_string());
            }
        }
    }
    Ok(cin_nos)
}

// =============================================================================
// Connection helpers
// =============================================================================

async fn connect_legacy() -> Result<DbPool, Box<dyn std::error::Error + Send + Sync>> {
    // Inherit MSSQL_PORT / DB_PASSWORD / circuit-breaker semantics from
    // the centralised DbConfig — same pattern as `bin/sync.rs` and
    // `bin/backfill_rooms.rs`.
    let mut config = DbConfig::from_env();
    // The bb8 pool's default `pool_max` (20) is wider than we need for
    // this one-shot bin; cap it at MSSQL_POOL_MAX so we don't gratuitously
    // open more legacy connections than there are concurrent folios.
    config.pool_max = MSSQL_POOL_MAX;
    let server = config.server.clone();
    let pool = create_pool(&config)
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { e.to_string().into() })?;
    {
        let mut conn = pool.get().await?;
        let _ = conn.simple_query("SELECT 1").await?;
    }
    tracing::info!(
        server = %server,
        port = config.port,
        "Connected to legacy SQL Server"
    );
    Ok(pool)
}

async fn connect_pg() -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {
    let url = env::var("DATABASE_URL")
        .or_else(|_| env::var("NEW_DATABASE_URL"))
        .map_err(|_| "DATABASE_URL or NEW_DATABASE_URL must be set")?;
    let pool = PgPoolOptions::new()
        .max_connections(PG_POOL_MAX)
        .acquire_timeout(Duration::from_secs(15))
        .connect(&url)
        .await?;
    sqlx::query("SELECT 1").execute(&pool).await?;
    tracing::info!("Connected to PostgreSQL");
    Ok(pool)
}

fn parse_concurrency() -> usize {
    env::var("BACKFILL_CONCURRENCY")
        .ok()
        .and_then(|v| v.parse().ok())
        .filter(|n: &usize| *n > 0)
        .unwrap_or(DEFAULT_BACKFILL_CONCURRENCY)
}

// =============================================================================
// Tests — bin-local helpers only. Orchestration tests live in
// `sync::backfill` (unit) + `tests/test_backfill_b5.rs` (integration).
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concurrency_defaults_when_env_unset() {
        // Snapshot + restore to avoid clobbering other tests' env state.
        // CI runs --test-threads=1 (see `tests/common/mod.rs`) so env
        // mutation is safe; the restore at the bottom mirrors a unit-test
        // pattern used elsewhere in the crate (`config.rs` SITE_ID tests).
        let prev = env::var("BACKFILL_CONCURRENCY").ok();
        env::remove_var("BACKFILL_CONCURRENCY");
        assert_eq!(parse_concurrency(), DEFAULT_BACKFILL_CONCURRENCY);

        env::set_var("BACKFILL_CONCURRENCY", "16");
        assert_eq!(parse_concurrency(), 16);

        // Garbage value falls back to default.
        env::set_var("BACKFILL_CONCURRENCY", "not-a-number");
        assert_eq!(parse_concurrency(), DEFAULT_BACKFILL_CONCURRENCY);

        // Zero is rejected (a 0-permit semaphore deadlocks immediately).
        env::set_var("BACKFILL_CONCURRENCY", "0");
        assert_eq!(parse_concurrency(), DEFAULT_BACKFILL_CONCURRENCY);

        match prev {
            Some(v) => env::set_var("BACKFILL_CONCURRENCY", v),
            None => env::remove_var("BACKFILL_CONCURRENCY"),
        }
    }

    #[test]
    fn active_status_literal_matches_legacy_constant() {
        // Lock the Thai literals against accidental rename. The
        // mapper uses 'ยกเลิก' as the cancelled literal; everything
        // else is active. The bin selects on 'ปกติ' explicitly because
        // the legacy app may use other non-cancelled values in the
        // future and we don't want to backfill rows the sync mapper
        // would ignore.
        assert_eq!(CIN_STATUS_ACTIVE, "ปกติ");
        assert_eq!(CIN_ROOM_STATUS_CHECKED_OUT, "Check-Out");
    }
}
