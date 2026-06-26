//! One-shot backfill: legacy MSSQL `HT_CheckIn_Pay` → canonical PG
//! `ht_payment_ledger` (Track J7e).
//!
//! ## Why this exists
//!
//! `ht_payment_ledger` (migration 057) is the per-line, per-tender mirror of
//! `HT_CheckIn_Pay` that the round-close reconciliation + shift report
//! (`GET /api/shifts/{id}/report`) sum. It is populated by the live
//! `PaymentMapper` coalesced path — but only **forward** (when a check-in's
//! payments next change after the J7a deploy). Rounds containing check-ins
//! that were paid-in-full *before* the deploy and never touched again would
//! under-report. This bin closes that gap in one pass: read the recent
//! `HT_CheckIn_Pay` lines (READ-ONLY — no MSSQL writes whatsoever) and mirror
//! them via the EXACT same code path the live watcher uses
//! (`load_checkin_aggregate` + `sync::mappers::payment::mirror_payment_ledger`),
//! so there is one source of truth for the projection + upsert.
//!
//! Idempotent + non-destructive: `mirror_payment_ledger` does a
//! delete-then-insert per `Cin_No` from the CURRENT legacy line set, so a
//! re-run (or a run racing the live watcher) converges to the same state.
//!
//! ## Usage
//!
//! ```text
//! # HF Hotel (default env), recent 90 days:
//! docker compose --profile backfill run --rm backfill-payment-ledger
//! # HF Ville:
//! docker compose --profile backfill run --rm backfill-payment-ledger-hfville
//!
//! # Locally / flags:
//! DATABASE_URL=postgres://… DB_SERVER=… DB_USER=sa DB_PASSWORD=… DB_NAME=db \
//!   cargo run --release --bin backfill_payment_ledger -- --dry-run --days=120
//! ```
//!
//! Run once per site (HF Hotel `hotelnew`, HF Ville `hotelville`).
//!
//! ## Flags
//!
//! * `--dry-run`     — load + count; no PG writes.
//! * `--days=N`      — only `Cin_No`s with a payment line in the last N days
//!                     (by `Cin_Pay_Date`). Default 90 — covers all current /
//!                     recent rounds while bounding the sweep. Generous because
//!                     a long-stay guest's early payment still matters.
//! * `--all`         — no date filter; mirror every `Cin_No` that has payments.
//! * `--site-id=ID`  — informational tag in log output. Defaults to `SITE_ID`.

use std::env;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use hotel_backend::config::{DbConfig, SiteConfig};
use hotel_backend::db::{create_pool, DbPool};
use hotel_backend::sync::mappers::payment::mirror_payment_ledger;
use hotel_backend::sync::parent_loader::load_checkin_aggregate;

const DEFAULT_DAYS: i64 = 90;
const PG_POOL_MAX: u32 = 4;
const MSSQL_POOL_MAX: u32 = 4;

#[derive(Default, Debug)]
struct Summary {
    cin_nos: usize,
    lines_mirrored: usize,
    cin_nos_with_lines: usize,
    load_errors: usize,
    mirror_errors: usize,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();
    hotel_backend::secrets::hydrate_env_from_secret_files();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backfill_payment_ledger=info,hotel_backend=info".into()),
        )
        .init();

    let dry_run = env::args().any(|a| a == "--dry-run");
    let all = env::args().any(|a| a == "--all");
    let days = env::args()
        .find_map(|a| a.strip_prefix("--days=").map(|s| s.to_string()))
        .and_then(|s| s.parse::<i64>().ok())
        .filter(|n| *n > 0)
        .unwrap_or(DEFAULT_DAYS);
    let site = SiteConfig::from_env();

    tracing::info!(
        site = %site.id, dry_run, all, days,
        "Track J7e — backfilling ht_payment_ledger from legacy HT_CheckIn_Pay"
    );

    let pg = connect_pg().await?;
    let mssql = connect_legacy().await?;

    let cin_nos = fetch_candidate_cin_nos(&mssql, all, days).await?;
    tracing::info!(site = %site.id, candidates = cin_nos.len(), "Fetched distinct Cin_No with payments");

    let mut s = Summary { cin_nos: cin_nos.len(), ..Default::default() };
    for cin_no in &cin_nos {
        let agg = match load_checkin_aggregate(&mssql, cin_no).await {
            Ok(a) => a,
            Err(err) => {
                tracing::warn!(cin_no, error = %err, "load_checkin_aggregate failed — skipping");
                s.load_errors += 1;
                continue;
            }
        };
        if agg.payments.is_empty() {
            continue;
        }
        if dry_run {
            s.cin_nos_with_lines += 1;
            s.lines_mirrored += agg.payments.len();
            continue;
        }
        let mut tx = match pg.begin().await {
            Ok(t) => t,
            Err(err) => {
                tracing::warn!(cin_no, error = %err, "pg.begin failed — skipping");
                s.mirror_errors += 1;
                continue;
            }
        };
        match mirror_payment_ledger(&mut tx, cin_no, &agg.payments).await {
            Ok(()) => match tx.commit().await {
                Ok(()) => {
                    s.cin_nos_with_lines += 1;
                    s.lines_mirrored += agg.payments.len();
                }
                Err(err) => {
                    tracing::warn!(cin_no, error = %err, "commit failed — skipping");
                    s.mirror_errors += 1;
                }
            },
            Err(err) => {
                let _ = tx.rollback().await;
                tracing::warn!(cin_no, error = %err, "mirror_payment_ledger failed — skipping");
                s.mirror_errors += 1;
            }
        }
    }

    tracing::info!(
        site = %site.id,
        cin_nos = s.cin_nos,
        cin_nos_with_lines = s.cin_nos_with_lines,
        lines_mirrored = s.lines_mirrored,
        load_errors = s.load_errors,
        mirror_errors = s.mirror_errors,
        dry_run,
        "Track J7e — backfill done"
    );
    println!(
        "backfill_payment_ledger [{}]{}: {} Cin_No scanned, {} with lines, {} lines mirrored, {} load-err, {} mirror-err",
        site.id,
        if dry_run { " (dry-run)" } else { "" },
        s.cin_nos, s.cin_nos_with_lines, s.lines_mirrored, s.load_errors, s.mirror_errors
    );
    Ok(())
}

/// Distinct `Cin_No` values that have at least one `HT_CheckIn_Pay` line
/// (optionally within the last `days`). READ-ONLY.
async fn fetch_candidate_cin_nos(
    mssql: &DbPool,
    all: bool,
    days: i64,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = mssql.get().await?;
    // `days` is a parsed positive integer (no user-string interpolation).
    let where_date = if all {
        String::new()
    } else {
        format!(" AND Cin_Pay_Date >= DATEADD(day, -{days}, GETDATE())")
    };
    let sql = format!(
        "SELECT DISTINCT Cin_No FROM HT_CheckIn_Pay \
          WHERE Cin_No IS NOT NULL AND Cin_No <> ''{where_date} \
          ORDER BY Cin_No"
    );
    let rows = conn.simple_query(&sql).await?.into_first_result().await?;
    let mut out = Vec::with_capacity(rows.len());
    for row in &rows {
        if let Some(c) = row.get::<&str, _>("Cin_No") {
            let t = c.trim();
            if !t.is_empty() {
                out.push(t.to_string());
            }
        }
    }
    Ok(out)
}

async fn connect_pg() -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {
    let url = env::var("DATABASE_URL")
        .or_else(|_| env::var("NEW_DATABASE_URL"))
        .map_err(|_| "DATABASE_URL or NEW_DATABASE_URL must be set")?;
    let pool = PgPoolOptions::new()
        .max_connections(PG_POOL_MAX)
        .connect(&url)
        .await?;
    Ok(pool)
}

async fn connect_legacy() -> Result<DbPool, Box<dyn std::error::Error + Send + Sync>> {
    let mut config = DbConfig::from_env();
    config.pool_max = MSSQL_POOL_MAX;
    let server = config.server.clone();
    let pool = create_pool(&config)
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send + Sync> { e.to_string().into() })?;
    {
        let mut conn = pool.get().await?;
        let _ = conn.simple_query("SELECT 1").await?;
    }
    tracing::info!(server = %server, port = config.port, "Connected to legacy MSSQL (read-only)");
    Ok(pool)
}
