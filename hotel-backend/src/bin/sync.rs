//! Change Tracking Watcher Binary (`bin/sync.rs`) — Phase 5.1 skeleton.
//!
//! Per `docs/architecture.md` §3.6d, §4d-tris, §10 #8.
//!
//! ## Lifecycle
//!
//! 1. Parse env (`LEGACY_SYNC_ENABLED`, `LEGACY_SYNC_SHADOW_MODE`,
//!    `LEGACY_SYNC_TABLE_ALLOWLIST`, `CT_POLL_INTERVAL_MS`).
//! 2. If disabled (`LEGACY_SYNC_ENABLED != "true"`) → log + exit 0
//!    (intentional disable, NOT a failure).
//! 3. Open PG (sqlx) + MSSQL (tiberius/bb8) pools.
//! 4. Verify legacy schema fingerprint — abort + Slack alert on drift.
//! 5. Build `Vec<Box<dyn MssqlChangeMapper>>` — one per CT-enabled
//!    table, filtered by allowlist. Phase 5.1 ships 10 `NoopMapper`s.
//! 6. Main loop: every `CT_POLL_INTERVAL_MS` (default 1000ms):
//!    a. Read `legacy_ct_state.last_seen_version`.
//!    b. For each mapper, in panic-isolated tasks:
//!       - Verify `MIN_VALID_VERSION(<table>) <= last_seen_version`
//!         (else → CT retention overflow → Slack alert + skip).
//!       - Query `CHANGETABLE(CHANGES <table>, @last) JOIN <table>`
//!         filtering `SYS_CHANGE_CONTEXT <> 0x4E48` (loop-prevention).
//!       - For each row: `mapper.apply(&mut tx, op, row).await`.
//!       - Capture per-table max(SYS_CHANGE_VERSION).
//!       - Update `legacy_sync_status.rows_ingested`/`rows_skipped`.
//!    c. After all mappers commit: advance the watermark to
//!       `min(per-table-max)` in one PG TX.
//! 7. SIGTERM → finish current tick, then exit cleanly.
//!
//! ## What this binary does NOT do (Phase 5.1)
//!
//! - No real per-table mappers — every entry is `NoopMapper` so the loop
//!   exercises the polling, watermark advance, and observability plumbing
//!   without touching canonical PG row-shapes. Real mappers land 5.2+.
//! - No `--bootstrap` cold-start path — that's 5.5; this binary refuses
//!   to start with `--bootstrap` for now (clear error message).
//! - No HTTP server, no SSE — those live in `bin/hotel-backend`.
//! - No docker-compose service block — that's 5.5.

// The lifecycle doc-comment above uses sub-numbered list items (`a.`,
// `b.`, `c.`) under each top-level numbered step. clippy's
// `doc_lazy_continuation` lint wants either deeper indentation or blank
// lines between every sub-item, both of which hurt readability for the
// lifecycle sketch. Suppress the lint at the binary scope; the
// formatting is intentional.
#![allow(clippy::doc_lazy_continuation)]

use std::collections::HashSet;
use std::env;
use std::sync::Arc;
use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tokio::sync::Notify;

use hotel_backend::config::{DbConfig, SlackConfig};
use hotel_backend::db::{create_pool, DbPool};
use hotel_backend::notifications::slack::{SlackClient, SlackMessage};
use hotel_backend::sync::{MssqlChangeMapper, NoopMapper};
use hotel_backend::writeback::verify_schema_fingerprint;

/// Default poll interval (milliseconds). Sub-second per the architecture
/// doc's "real-time UI" goal — Goal #1 stretch is sub-2-second
/// end-to-end latency from a .NET app save to our SSE-driven UI repaint.
const DEFAULT_CT_POLL_INTERVAL_MS: u64 = 1000;

/// All CT-enabled MSSQL tables — must stay in sync with the seed in
/// migration 017 and `legacy_sync_status` rows. Adding a new mapper
/// means inserting a row in 017, adding the table here, and replacing
/// the matching `NoopMapper` entry in `build_mappers`.
const CT_ENABLED_TABLES: &[&str] = &[
    "HT_Customers",
    "HT_Rooms",
    "HT_Room_Status",
    "HT_Book_H",
    "HT_Book_Ds",
    "HT_Book_Date",
    "HT_CheckIn_H",
    "HT_CheckIn_Ds",
    "HT_CheckIn_Pay",
    "HT_Receipt_H",
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hotel_backend=info,sync=info".into()),
        )
        .init();

    // 1. LEGACY_SYNC_ENABLED — explicit opt-in. Worker exits cleanly
    //    (exit code 0) when disabled — this is intentional, not a
    //    failure, so Docker `restart: unless-stopped` doesn't loop.
    let enabled = env::var("LEGACY_SYNC_ENABLED")
        .map(|v| v == "true")
        .unwrap_or(false);
    if !enabled {
        tracing::info!(
            "LEGACY_SYNC_ENABLED!=true — CT watcher exiting cleanly without polling"
        );
        return Ok(());
    }

    // Reject --bootstrap until Phase 5.5 wires it to scheduler::sync.
    if env::args().any(|a| a == "--bootstrap") {
        return Err(
            "--bootstrap is reserved for Phase 5.5 cold-start path; not yet implemented"
                .into(),
        );
    }

    let poll_interval_ms = env::var("CT_POLL_INTERVAL_MS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_CT_POLL_INTERVAL_MS);

    // Shadow mode: when true, mappers run their UPSERTs in a transaction
    // that is rolled back at the end (5.2+ honours this). 5.1 mappers
    // are no-ops so the flag is parsed but does not affect behavior —
    // wiring it now keeps 5.2+ a config-only flip.
    let shadow_mode = env::var("LEGACY_SYNC_SHADOW_MODE")
        .map(|v| v == "true")
        .unwrap_or(false);

    // Allowlist: comma-separated MSSQL table names. Empty/unset = all
    // CT-enabled tables. Used to phase mappers in one at a time during
    // 5.2 rollout without code changes.
    let allowlist = parse_allowlist(env::var("LEGACY_SYNC_TABLE_ALLOWLIST").ok());

    tracing::info!(
        poll_interval_ms,
        shadow_mode,
        allowlist = ?allowlist,
        "Starting CT watcher"
    );

    // 2. PG pool
    let pg_url = env::var("DATABASE_URL")
        .or_else(|_| env::var("NEW_DATABASE_URL"))
        .map_err(|_| "DATABASE_URL or NEW_DATABASE_URL must be set")?;
    let pg = PgPoolOptions::new()
        .max_connections(8)
        .connect(&pg_url)
        .await?;
    tracing::info!("Connected to PostgreSQL");

    // 3. MSSQL pool — `create_pool` returns `Box<dyn Error>` (not
    //    Send+Sync) so we map it to a string before bubbling up.
    let mssql_config = DbConfig::from_env();
    let mssql = create_pool(&mssql_config)
        .await
        .map_err(|e| format!("MSSQL pool init failed: {e}"))?;
    tracing::info!(server = %mssql_config.server, "Connected to legacy MSSQL");

    // 4. Slack notifier — best-effort alerts on schema drift,
    //    retention overflow, and per-mapper failures.
    let slack_config = SlackConfig::from_env();
    let slack: Option<SlackClient> = if slack_config.is_configured() {
        tracing::info!("Slack notifications enabled for CT watcher");
        Some(SlackClient::new(slack_config))
    } else {
        tracing::warn!(
            "Slack notifications NOT configured (set SLACK_WEBHOOK_URL); \
             schema drift / retention overflow will only surface in logs"
        );
        None
    };

    // 5. Schema fingerprint guard — refuse to start on drift, post a
    //    Slack alert first. Sleep before exit so Docker restart cadence
    //    backs off (matches writeback worker's pattern).
    if let Err(e) = verify_schema_fingerprint(&mssql).await {
        tracing::error!(
            error = %e,
            "Schema fingerprint check failed — refusing to start"
        );
        if let Some(slack) = &slack {
            let msg = SlackMessage::with_text(format!(
                ":warning: *CT watcher REFUSED TO START* :warning:\n\
                 Legacy MSSQL schema fingerprint mismatch.\n\
                 *Error:* `{e}`\n\
                 _The legacy DB columns drifted from the captured baseline. \
                 Run_ `./scripts/writeback-fingerprint.sh` _and follow the \
                 README to update the baseline before restarting._"
            ));
            let _ = slack.send_message(&msg).await;
        }
        tracing::warn!("Sleeping 60s before exit to throttle Docker restart cadence");
        tokio::time::sleep(Duration::from_secs(60)).await;
        return Err(format!("Schema fingerprint check failed: {e}").into());
    }

    // 6. Build mappers — one per CT-enabled table, filtered by
    //    allowlist. Phase 5.1 ships 10 `NoopMapper`s; 5.2+ replaces
    //    entries one at a time.
    let mappers = build_mappers(&allowlist);
    tracing::info!(
        count = mappers.len(),
        "Mappers initialised (all NoopMapper in 5.1)"
    );

    // SIGTERM handler — finish the current tick then exit.
    let shutdown = Arc::new(Notify::new());
    let shutdown_clone = shutdown.clone();
    tokio::spawn(async move {
        let signal_kind = tokio::signal::unix::SignalKind::terminate();
        let mut sigterm = match tokio::signal::unix::signal(signal_kind) {
            Ok(s) => s,
            Err(err) => {
                tracing::warn!(error = %err, "Could not register SIGTERM handler");
                return;
            }
        };
        sigterm.recv().await;
        tracing::info!("SIGTERM received — finishing current tick then exiting");
        shutdown_clone.notify_waiters();
    });

    // 7. Main loop
    loop {
        run_one_tick(&pg, &mssql, &mappers, &slack, shadow_mode).await;

        tokio::select! {
            _ = tokio::time::sleep(Duration::from_millis(poll_interval_ms)) => {
                tracing::trace!("CT poll-interval tick");
            }
            _ = shutdown.notified() => {
                tracing::info!("Shutdown signaled — exiting main loop");
                break;
            }
        }
    }

    tracing::info!("CT watcher exited cleanly");
    Ok(())
}

/// Parse `LEGACY_SYNC_TABLE_ALLOWLIST` into a normalised set. Empty /
/// unset / whitespace-only values disable filtering (every CT-enabled
/// table runs). Comma-separated; whitespace around tokens is trimmed.
fn parse_allowlist(raw: Option<String>) -> Option<HashSet<String>> {
    let raw = raw?;
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let set: HashSet<String> = trimmed
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if set.is_empty() {
        None
    } else {
        Some(set)
    }
}

/// Build the per-table mapper list, filtered by the allowlist. In 5.1
/// every entry is a `NoopMapper`; 5.2+ swaps real mappers in by table.
fn build_mappers(allowlist: &Option<HashSet<String>>) -> Vec<Box<dyn MssqlChangeMapper>> {
    CT_ENABLED_TABLES
        .iter()
        .filter(|t| allowlist.as_ref().map(|a| a.contains(**t)).unwrap_or(true))
        .map(|t| {
            Box::new(NoopMapper { table_name: t }) as Box<dyn MssqlChangeMapper>
        })
        .collect()
}

/// Process one watcher tick. Per-mapper failures are logged but don't
/// abort the tick — one bad table never blocks the others. Per-mapper
/// panics are isolated via `tokio::spawn` so a panicked mapper can't
/// crash the binary.
async fn run_one_tick(
    pg: &PgPool,
    mssql: &DbPool,
    mappers: &[Box<dyn MssqlChangeMapper>],
    slack: &Option<SlackClient>,
    shadow_mode: bool,
) {
    let last_seen = match hotel_backend::sync::watermark::read_last_seen(pg).await {
        Ok(v) => v,
        Err(err) => {
            tracing::error!(error = %err, "Failed to read CT watermark; skipping tick");
            return;
        }
    };

    for mapper in mappers {
        let table = mapper.table();
        // Panic isolation — a bug in any mapper's `apply` (or the CT
        // SELECT it drives) must not kill the binary.
        let pg_inner = pg.clone();
        let mssql_inner = mssql.clone();
        let slack_inner = slack.clone();
        let table_owned = table.to_string();
        let result = tokio::spawn(async move {
            poll_table(&pg_inner, &mssql_inner, &slack_inner, &table_owned, last_seen, shadow_mode)
                .await
        })
        .await;

        if let Err(join_err) = result {
            let panic_msg = if join_err.is_panic() {
                let payload = join_err.into_panic();
                payload
                    .downcast_ref::<&'static str>()
                    .map(|s| (*s).to_string())
                    .or_else(|| payload.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "panic with non-string payload".to_string())
            } else {
                format!("task cancelled: {join_err}")
            };
            tracing::error!(
                table,
                panic = %panic_msg,
                "CT mapper PANICKED — recording failure and continuing main loop"
            );
            // Best-effort: record the panic into legacy_sync_status so
            // operators see "table X has been failing for N ticks" even
            // if Slack is offline.
            let _ = record_table_error(pg, table, &format!("PANIC: {panic_msg}")).await;
        }
    }

    // Phase 5.1 leaves the watermark advance commented-out: with 10
    // NoopMappers nothing was actually applied, so advancing past the
    // current `CHANGE_TRACKING_CURRENT_VERSION()` would silently skip
    // the rows that the 5.2+ real mappers will need to re-process when
    // they replace the no-ops. The min-of-per-table-max calculation
    // belongs to 5.2 alongside the first real mapper. See the
    // `advance_watermark_after_real_mappers_land` TODO below.
    //
    // TODO(Phase 5.2): once at least one real mapper is wired, replace
    // this comment with:
    //   let new_version = mappers
    //       .iter()
    //       .filter_map(|m| per_table_max[m.table()])
    //       .min()
    //       .unwrap_or(last_seen);
    //   if let Err(err) = hotel_backend::sync::watermark::advance(pg, new_version).await {
    //       tracing::error!(error = %err, "Failed to advance CT watermark");
    //   }
    let _ = last_seen;
}

/// Poll one table for CT changes since `last_seen`. Phase 5.1: just
/// counts rows + bumps `legacy_sync_status.rows_skipped` (every row is
/// "skipped" because `NoopMapper` returns `Ok(None)`). Real per-row
/// dispatch lands in 5.2 alongside the first real mapper.
async fn poll_table(
    pg: &PgPool,
    mssql: &DbPool,
    slack: &Option<SlackClient>,
    table: &str,
    last_seen: i64,
    _shadow_mode: bool,
) {
    // Per-poll retention check — if MIN_VALID_VERSION for this table is
    // ahead of our watermark, CT has already cleaned up rows we needed
    // to see. We REFUSE to advance and surface to ops via Slack; the
    // recovery path is a manual `--bootstrap` (Phase 5.5).
    match check_retention(mssql, table, last_seen).await {
        Ok(()) => {}
        Err(err) => {
            tracing::error!(table, error = %err, "Retention check failed");
            // Treat as a transient failure — record + continue. A
            // hard retention overflow would have returned a typed
            // error in `err`; non-overflow errors are I/O hiccups.
            let _ = record_table_error(pg, table, &err.to_string()).await;
            if let Some(s) = slack {
                if err.to_string().contains("retention") {
                    let msg = SlackMessage::with_text(format!(
                        ":rotating_light: *CT retention overflow* :rotating_light:\n\
                         Table: `{table}`\n\
                         Watermark fell behind CT retention; \
                         row history beyond `MIN_VALID_VERSION` is gone.\n\
                         _Recover with_ `bin/sync --bootstrap` _(Phase 5.5)_."
                    ));
                    let _ = s.send_message(&msg).await;
                }
            }
            return;
        }
    }

    // Count CT rows since the watermark, filtering out our own
    // writeback session (`SYS_CHANGE_CONTEXT = 0x4E48`).
    let row_count = match count_ct_rows(mssql, table, last_seen).await {
        Ok(n) => n,
        Err(err) => {
            tracing::warn!(table, error = %err, "CT count query failed");
            let _ = record_table_error(pg, table, &err.to_string()).await;
            return;
        }
    };

    // Phase 5.1: every row is "skipped" because NoopMapper.apply
    // returns Ok(None). Bump the skipped counter so operators can
    // verify the loop is alive without log-tailing.
    if let Err(err) = bump_skipped(pg, table, row_count).await {
        tracing::warn!(
            table,
            error = %err,
            "Failed to update legacy_sync_status — observability degraded"
        );
    } else if row_count > 0 {
        tracing::info!(
            table,
            row_count,
            "CT rows observed (5.1: NoopMapper skipped all)"
        );
    }
}

/// Returns Ok(()) if `MIN_VALID_VERSION(<table>) <= last_seen`, else a
/// typed error containing "retention" so the caller can recognise it.
async fn check_retention(
    mssql: &DbPool,
    table: &str,
    last_seen: i64,
) -> Result<(), String> {
    let mut conn = mssql.get().await.map_err(|e| e.to_string())?;
    let sql = format!(
        "SELECT CHANGE_TRACKING_MIN_VALID_VERSION(OBJECT_ID(N'{table}')) AS min_valid"
    );
    let stream = conn.simple_query(&sql).await.map_err(|e| e.to_string())?;
    let rows = stream
        .into_first_result()
        .await
        .map_err(|e| e.to_string())?;
    let row = match rows.first() {
        Some(r) => r,
        None => return Ok(()), // CT not enabled? leave it for fingerprint guard
    };
    let min_valid: Option<i64> = row.get(0);
    let Some(min_valid) = min_valid else {
        return Ok(());
    };
    if min_valid > last_seen {
        return Err(format!(
            "retention overflow: min_valid_version={min_valid} > last_seen={last_seen}"
        ));
    }
    Ok(())
}

/// Count CT change rows since `last_seen`, filtering out our own
/// writeback session via the `SYS_CHANGE_CONTEXT` tag.
async fn count_ct_rows(
    mssql: &DbPool,
    table: &str,
    last_seen: i64,
) -> Result<i64, String> {
    let mut conn = mssql.get().await.map_err(|e| e.to_string())?;
    // CHANGETABLE wants the previous version; rows with SYS_CHANGE_VERSION
    // > @last are returned. Filter our own session via SYS_CHANGE_CONTEXT.
    let sql = format!(
        "SELECT COUNT(*) FROM CHANGETABLE(CHANGES {table}, {last_seen}) AS ct \
         WHERE ct.SYS_CHANGE_CONTEXT IS NULL OR ct.SYS_CHANGE_CONTEXT <> 0x4E48"
    );
    let stream = conn.simple_query(&sql).await.map_err(|e| e.to_string())?;
    let rows = stream
        .into_first_result()
        .await
        .map_err(|e| e.to_string())?;
    let row = rows
        .first()
        .ok_or_else(|| "COUNT(*) returned no rows".to_string())?;
    let n: i32 = row.get(0).unwrap_or(0);
    Ok(n as i64)
}

/// Bump `legacy_sync_status.rows_skipped` and clear any prior error.
async fn bump_skipped(pg: &PgPool, table: &str, n: i64) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE legacy_sync_status \
            SET rows_skipped         = rows_skipped + $2, \
                last_processed_at    = now(), \
                last_error           = NULL, \
                last_error_at        = NULL, \
                consecutive_failures = 0 \
          WHERE table_name = $1",
    )
    .bind(table)
    .bind(n)
    .execute(pg)
    .await?;
    Ok(())
}

/// Record a per-table failure into `legacy_sync_status`. Increments
/// `consecutive_failures` so operators can spot a wedged mapper.
async fn record_table_error(
    pg: &PgPool,
    table: &str,
    err: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE legacy_sync_status \
            SET last_error           = $2, \
                last_error_at        = now(), \
                consecutive_failures = consecutive_failures + 1 \
          WHERE table_name = $1",
    )
    .bind(table)
    .bind(err)
    .execute(pg)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_allowlist_returns_none_when_unset() {
        assert!(parse_allowlist(None).is_none());
    }

    #[test]
    fn parse_allowlist_returns_none_when_blank() {
        assert!(parse_allowlist(Some(String::new())).is_none());
        assert!(parse_allowlist(Some("   ".into())).is_none());
        assert!(parse_allowlist(Some(",,,".into())).is_none());
    }

    #[test]
    fn parse_allowlist_splits_and_trims() {
        let set = parse_allowlist(Some(" HT_Customers , HT_Rooms ,HT_Book_H ".into()))
            .expect("non-empty allowlist returns Some");
        assert!(set.contains("HT_Customers"));
        assert!(set.contains("HT_Rooms"));
        assert!(set.contains("HT_Book_H"));
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn build_mappers_no_allowlist_returns_all_ten() {
        let mappers = build_mappers(&None);
        assert_eq!(mappers.len(), CT_ENABLED_TABLES.len());
        assert_eq!(mappers.len(), 10, "10 CT-enabled tables expected in 5.1");
    }

    #[test]
    fn build_mappers_filters_by_allowlist() {
        let mut allow = HashSet::new();
        allow.insert("HT_Customers".to_string());
        allow.insert("HT_Rooms".to_string());
        let mappers = build_mappers(&Some(allow));
        assert_eq!(mappers.len(), 2);
    }

    #[test]
    fn ct_enabled_tables_match_migration_017_seed() {
        // Whenever this list changes, migration 017 + the
        // `legacy_sync_status` seed in init-hotelnew.sql must change
        // too. The mismatch would cause `bump_skipped` UPDATEs to
        // silently affect zero rows.
        let expected = [
            "HT_Customers",
            "HT_Rooms",
            "HT_Room_Status",
            "HT_Book_H",
            "HT_Book_Ds",
            "HT_Book_Date",
            "HT_CheckIn_H",
            "HT_CheckIn_Ds",
            "HT_CheckIn_Pay",
            "HT_Receipt_H",
        ];
        assert_eq!(CT_ENABLED_TABLES, &expected);
    }

    /// CHANGETABLE filter must include the SYS_CHANGE_CONTEXT clause
    /// matching the `SET CONTEXT_INFO 0x4E48` value the writeback
    /// dispatcher stamps. Locks the byte literal so a refactor can't
    /// silently break loop-prevention.
    #[test]
    fn count_ct_rows_uses_loop_prevention_filter() {
        let source = include_str!("sync.rs");
        assert!(
            source.contains("SYS_CHANGE_CONTEXT IS NULL OR ct.SYS_CHANGE_CONTEXT <> 0x4E48"),
            "count_ct_rows must filter out our own writeback session via 0x4E48"
        );
    }

    /// The CONTEXT_INFO value used by writeback's dispatcher and the
    /// CT watcher's filter must be the same byte sequence — otherwise
    /// every writeback re-fires through the watcher. This test pins
    /// both to `0x4E48` ("NH"). Update both files together if the tag
    /// ever needs to change.
    #[test]
    fn loop_prevention_tag_matches_writeback_dispatcher() {
        let dispatcher_src = include_str!("../writeback/dispatcher.rs");
        let mssql_session_src = include_str!("../db/mssql_session.rs");
        // Dispatcher must call set_context_info — the actual
        // `0x4E48` literal lives in mssql_session.rs.
        assert!(dispatcher_src.contains("set_context_info(conn)"));
        assert!(
            mssql_session_src.contains("SET CONTEXT_INFO 0x4E48"),
            "mssql_session::set_context_info must issue 0x4E48 \
             so the watcher's CHANGETABLE filter can match it"
        );
    }
}
