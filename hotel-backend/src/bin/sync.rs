//! Change Tracking Watcher Binary (`bin/sync.rs`).
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
//!    table, filtered by allowlist. Phase 5.2 ships real mappers for
//!    `HT_Customers` / `HT_Rooms` / `HT_Room_Status`; the rest stay on
//!    `NoopMapper` until 5.3 / 5.4.
//! 6. Main loop: every `CT_POLL_INTERVAL_MS` (default 1000ms):
//!    a. Read `legacy_ct_state.last_seen_version`.
//!    b. For each mapper, in panic-isolated tasks:
//!       - Verify `MIN_VALID_VERSION(<table>) <= last_seen_version`
//!         (else → CT retention overflow → Slack alert + skip).
//!       - Query `CHANGETABLE(CHANGES <table>, @last) JOIN <table>`
//!         filtering `SYS_CHANGE_CONTEXT <> 0x4E48` (loop-prevention).
//!       - For each row: `mapper.apply(&mut tx, op, Some(&row)).await`
//!         (or `None` for D operations). On success, INSERT into
//!         `event_log` (live mode) or log "would publish" (shadow mode).
//!       - Capture per-table max(SYS_CHANGE_VERSION).
//!       - Update `legacy_sync_status.rows_ingested` /  `rows_skipped`.
//!       - Commit (live) / rollback (shadow) in the same TX.
//!       - Advance the watermark to per-table max.
//! 7. SIGTERM → finish current tick, then exit cleanly.
//!
//! ## Watermark advance — per-table, not min-of-all
//!
//! Phase 5.2 advances the watermark per-table at the end of each
//! table's own TX. Customer / room mappers are flat tables — one CT row
//! produces at most one mapper call and at most one event, so per-table
//! advance is equivalent to min-of-all and simpler.
//!
//! **Phase 5.3 keeps per-table watermark advance** even though the
//! booking aggregate spans HT_Book_H + HT_Book_Ds + HT_Book_Date.
//! `SYS_CHANGE_VERSION` is global-monotonic across the whole database,
//! so committing the per-table max version after a successful TX
//! cannot lose rows from any other table. The "one event per
//! aggregate per tick" guarantee is enforced one level up — by
//! coalescing CT rows in `poll_table` (see the per-row vs coalesced
//! branches there), not by watermark mechanics. Net effect: a booking
//! header + 2 line + 5 night change touches all three tables, but
//! produces exactly one DomainEvent (header tick emits the
//! BookingModified; the subsequent Ds + Date ticks load the same
//! aggregate, find the canonical row already matches, and skip the
//! event).

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
use hotel_backend::outbox::bus::EventBus;
use hotel_backend::outbox::event::DomainEvent;
use hotel_backend::sync::change_op::ChangeOp;
use hotel_backend::sync::mappers::{
    apply_booking_aggregate, apply_checkin_aggregate, apply_payment_aggregate,
    BookingDatesMapper, BookingHeaderMapper, BookingRoomsMapper, CheckInHeaderMapper,
    CheckInRoomsMapper, CustomerMapper, PaymentMapper, ReceiptMapper, RoomMasterMapper,
    RoomStatusMapper,
};
use hotel_backend::sync::parent_loader::{load_booking_aggregate, load_checkin_aggregate};
use hotel_backend::sync::{MssqlChangeMapper, NoopMapper};
use hotel_backend::writeback::verify_schema_fingerprint;

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

    let enabled = env::var("LEGACY_SYNC_ENABLED")
        .map(|v| v == "true")
        .unwrap_or(false);
    if !enabled {
        tracing::info!(
            "LEGACY_SYNC_ENABLED!=true — CT watcher exiting cleanly without polling"
        );
        return Ok(());
    }

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

    let shadow_mode = env::var("LEGACY_SYNC_SHADOW_MODE")
        .map(|v| v == "true")
        .unwrap_or(false);

    let allowlist = parse_allowlist(env::var("LEGACY_SYNC_TABLE_ALLOWLIST").ok());

    tracing::info!(
        poll_interval_ms,
        shadow_mode,
        allowlist = ?allowlist,
        "Starting CT watcher"
    );

    let pg_url = env::var("DATABASE_URL")
        .or_else(|_| env::var("NEW_DATABASE_URL"))
        .map_err(|_| "DATABASE_URL or NEW_DATABASE_URL must be set")?;
    let pg = PgPoolOptions::new()
        .max_connections(8)
        .connect(&pg_url)
        .await?;
    tracing::info!("Connected to PostgreSQL");

    let mssql_config = DbConfig::from_env();
    let mssql = create_pool(&mssql_config)
        .await
        .map_err(|e| format!("MSSQL pool init failed: {e}"))?;
    tracing::info!(server = %mssql_config.server, "Connected to legacy MSSQL");

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

    let mappers = build_mappers(&allowlist);
    tracing::info!(
        count = mappers.len(),
        "Mappers initialised (Phase 5.4: 10-table coverage — every CT-enabled \
         table has a real mapper or an intentional retired stub)"
    );

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

/// Build the per-table mapper list, filtered by the allowlist. Phase
/// 5.4 completes 10-table coverage: every CT-enabled table now has a
/// real mapper or an intentional retired stub (`HT_Room_Status`).
fn build_mappers(allowlist: &Option<HashSet<String>>) -> Vec<Box<dyn MssqlChangeMapper>> {
    let allowed = |t: &str| allowlist.as_ref().map(|a| a.contains(t)).unwrap_or(true);

    let mut out: Vec<Box<dyn MssqlChangeMapper>> = Vec::with_capacity(CT_ENABLED_TABLES.len());

    for table in CT_ENABLED_TABLES {
        if !allowed(table) {
            continue;
        }
        let mapper: Box<dyn MssqlChangeMapper> = match *table {
            "HT_Customers" => Box::new(CustomerMapper),
            "HT_Rooms" => Box::new(RoomMasterMapper),
            "HT_Room_Status" => Box::new(RoomStatusMapper),
            "HT_Book_H" => Box::new(BookingHeaderMapper),
            "HT_Book_Ds" => Box::new(BookingRoomsMapper),
            "HT_Book_Date" => Box::new(BookingDatesMapper),
            "HT_CheckIn_H" => Box::new(CheckInHeaderMapper),
            "HT_CheckIn_Ds" => Box::new(CheckInRoomsMapper),
            "HT_CheckIn_Pay" => Box::new(PaymentMapper),
            "HT_Receipt_H" => Box::new(ReceiptMapper),
            other => Box::new(NoopMapper { table_name: other }),
        };
        out.push(mapper);
    }
    out
}

/// Process one watcher tick. Per-mapper failures are logged but don't
/// abort the tick — one bad table never blocks the others.
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
        let pk_cols = mapper.primary_key_cols();
        let select_sql = mapper.select_sql();

        // Run each table inside its own future; panics are isolated
        // via `tokio::spawn` further down for the per-row dispatch.
        if let Err(err) = poll_table(
            pg,
            mssql,
            slack,
            mapper.as_ref(),
            table,
            pk_cols,
            select_sql,
            last_seen,
            shadow_mode,
        )
        .await
        {
            tracing::error!(table, error = %err, "poll_table failed");
            let _ = record_table_error(pg, table, &err.to_string()).await;
        }
    }
}

/// Poll one table for CT changes since `last_seen`. Per the lifecycle:
/// retention check → SELECT CT changes → for each row, dispatch to
/// mapper → INSERT event_log → bump counters → advance watermark.
#[allow(clippy::too_many_arguments)]
async fn poll_table(
    pg: &PgPool,
    mssql: &DbPool,
    slack: &Option<SlackClient>,
    mapper: &dyn MssqlChangeMapper,
    table: &str,
    pk_cols: &[&str],
    select_sql: &str,
    last_seen: i64,
    shadow_mode: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 1. Retention guard.
    if let Err(err) = check_retention(mssql, table, last_seen).await {
        tracing::error!(table, error = %err, "Retention check failed");
        let _ = record_table_error(pg, table, &err).await;
        if let Some(s) = slack {
            if err.contains("retention") {
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
        return Ok(()); // intentional skip — retention can't be repaired by retry
    }

    // 2. NoopMapper short-circuit — when select_sql is empty there's
    //    nothing to project, so just count rows for observability.
    if select_sql.is_empty() {
        let row_count = match count_ct_rows(mssql, table, last_seen).await {
            Ok(n) => n,
            Err(err) => {
                tracing::warn!(table, error = %err, "CT count query failed");
                let _ = record_table_error(pg, table, &err).await;
                return Ok(());
            }
        };
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
                "CT rows observed (NoopMapper — skipped, awaiting real mapper)"
            );
        }
        return Ok(());
    }

    // 3. Real-mapper path: fetch CT rows joined with the table.
    let rows = match fetch_ct_rows(mssql, table, pk_cols, select_sql, last_seen).await {
        Ok(rs) => rs,
        Err(err) => {
            tracing::warn!(table, error = %err, "CT fetch failed");
            let _ = record_table_error(pg, table, &err).await;
            return Ok(());
        }
    };

    if rows.is_empty() {
        return Ok(());
    }

    let row_count = rows.len() as i64;
    let mut max_version: i64 = last_seen;
    let mut ingested: i64 = 0;
    let mut skipped: i64 = 0;

    // 4. Open one PG TX per table-tick. Shadow mode rolls back; live
    //    mode commits.
    let mut tx = match pg.begin().await {
        Ok(t) => t,
        Err(err) => {
            tracing::error!(table, error = %err, "Failed to begin PG TX");
            return Ok(());
        }
    };

    // 4a. Aggregate-coalesced path (Phase 5.3): when the mapper opts
    //     in via `coalesce_key`, group rows by the aggregate root and
    //     dispatch each unique key exactly once per tick. Currently
    //     only the booking mappers (HT_Book_H / HT_Book_Ds /
    //     HT_Book_Date) opt in — see `sync::mappers::booking`.
    //
    //     The dispatch path branches on whether ANY row in the batch
    //     produced a Some-key. If so, the entire batch goes through
    //     the coalesced path; rows without a key (e.g. D rows on
    //     child tables where the parent FK isn't projected) are
    //     skipped with a debug log — a sibling header / line CT row
    //     in the same tick almost always covers the same booking and
    //     drives the canonical re-load.
    let any_coalesce_key = rows
        .iter()
        .any(|(_, _, r)| mapper.coalesce_key(r).is_some());

    if any_coalesce_key {
        for (version, op_char, _row) in &rows {
            // We still parse the op code to surface unknown operations
            // loudly (matches the per-row path's behaviour). Beyond
            // that, the op itself is informational for the aggregate
            // path — the parent re-load supersedes per-row I/U/D
            // semantics.
            if let Err(err) = ChangeOp::try_from(op_char.as_str()) {
                tracing::warn!(
                    table,
                    sys_change_operation = %op_char,
                    error = %err,
                    "Unknown CT operation code — skipping row"
                );
                skipped += 1;
                continue;
            }
            if *version > max_version {
                max_version = *version;
            }
        }

        // Group all rows by aggregate root key (de-dup by HashSet).
        let mut keys: std::collections::HashSet<String> = std::collections::HashSet::new();
        for (_v, _op, row) in &rows {
            if let Some(k) = mapper.coalesce_key(row) {
                keys.insert(k);
            }
        }

        for key in &keys {
            // Route the coalesced apply by table. Three aggregate
            // shapes ship today:
            //
            // * booking_*  → load_booking_aggregate + apply_booking_aggregate
            // * checkin_*  → load_checkin_aggregate + apply_checkin_aggregate
            // * payment_*  → apply_payment_aggregate (loads internally)
            let result = match table {
                "HT_Book_H" | "HT_Book_Ds" | "HT_Book_Date" => {
                    match load_booking_aggregate(mssql, key).await {
                        Ok(a) => apply_booking_aggregate(&mut tx, &a, key).await,
                        Err(err) => {
                            tracing::warn!(
                                table,
                                key = %key,
                                error = %err,
                                "Failed to load booking aggregate; recording and continuing"
                            );
                            let _ = record_table_error(pg, table, &err.to_string()).await;
                            skipped += 1;
                            continue;
                        }
                    }
                }
                "HT_CheckIn_H" | "HT_CheckIn_Ds" => {
                    match load_checkin_aggregate(mssql, key).await {
                        Ok(a) => apply_checkin_aggregate(&mut tx, Some(mssql), &a, key).await,
                        Err(err) => {
                            tracing::warn!(
                                table,
                                key = %key,
                                error = %err,
                                "Failed to load checkin aggregate; recording and continuing"
                            );
                            let _ = record_table_error(pg, table, &err.to_string()).await;
                            skipped += 1;
                            continue;
                        }
                    }
                }
                "HT_CheckIn_Pay" => apply_payment_aggregate(&mut tx, mssql, key).await,
                other => {
                    tracing::warn!(
                        table = other,
                        "Unknown coalesced aggregate table — skipping"
                    );
                    skipped += 1;
                    continue;
                }
            };

            match result {
                Ok(Some(event)) => {
                    if shadow_mode {
                        tracing::info!(
                            table,
                            key = %key,
                            event_type = event.type_name(),
                            "would publish (shadow mode)"
                        );
                        skipped += 1;
                    } else if let Err(err) = persist_event(&mut tx, &event).await {
                        tracing::error!(
                            table,
                            key = %key,
                            error = %err,
                            "Failed to persist event_log row"
                        );
                        skipped += 1;
                    } else {
                        ingested += 1;
                    }
                }
                Ok(None) => {
                    // Idempotent skip — canonical row already matches.
                    skipped += 1;
                }
                Err(err) => {
                    tracing::warn!(
                        table,
                        key = %key,
                        error = %err,
                        "aggregate apply error — recording and continuing"
                    );
                    let _ = record_table_error(pg, table, &err.to_string()).await;
                    skipped += 1;
                }
            }
        }
    } else {
        // 4b. Legacy per-row dispatch path (Phase 5.2). Customer / room /
        //     room_status mappers stay here.
        for (version, op_char, row) in &rows {
            let op = match ChangeOp::try_from(op_char.as_str()) {
                Ok(o) => o,
                Err(err) => {
                    tracing::warn!(
                        table,
                        sys_change_operation = %op_char,
                        error = %err,
                        "Unknown CT operation code — skipping row"
                    );
                    skipped += 1;
                    continue;
                }
            };

            // For Delete, the joined row is NULL but the PK columns are
            // still in the projection (CT carries them). Pass `Some(&row)`
            // either way and let the mapper decide.
            let result = mapper.apply(&mut tx, op, Some(row)).await;

            match result {
                Ok(Some(event)) => {
                    if shadow_mode {
                        tracing::info!(
                            table,
                            version,
                            op = ?op,
                            event_type = event.type_name(),
                            "would publish (shadow mode)"
                        );
                        skipped += 1;
                    } else if let Err(err) = persist_event(&mut tx, &event).await {
                        tracing::error!(
                            table,
                            version,
                            op = ?op,
                            error = %err,
                            "Failed to persist event_log row"
                        );
                        skipped += 1;
                    } else {
                        ingested += 1;
                    }
                }
                Ok(None) => {
                    // Idempotent skip / D-event with no event payload.
                    skipped += 1;
                }
                Err(err) => {
                    tracing::warn!(
                        table,
                        version,
                        op = ?op,
                        error = %err,
                        "Mapper error — recording and continuing"
                    );
                    let _ = record_table_error(pg, table, &err.to_string()).await;
                    skipped += 1;
                }
            }

            if *version > max_version {
                max_version = *version;
            }
        }
    }

    // 5. Commit (live) or rollback (shadow).
    if shadow_mode {
        if let Err(err) = tx.rollback().await {
            tracing::warn!(table, error = %err, "Shadow-mode rollback failed");
        }
        // Bump skipped counter to mirror the noop path's behavior.
        let _ = bump_skipped(pg, table, row_count).await;
        return Ok(());
    }

    if let Err(err) = tx.commit().await {
        tracing::error!(table, error = %err, "PG TX commit failed");
        let _ = record_table_error(pg, table, &err.to_string()).await;
        return Ok(());
    }

    // 6. Counters + watermark advance (live mode only).
    if let Err(err) = bump_counters(pg, table, ingested, skipped).await {
        tracing::warn!(table, error = %err, "Failed to bump counters");
    }

    if max_version > last_seen {
        if let Err(err) =
            hotel_backend::sync::watermark::advance(pg, max_version).await
        {
            tracing::error!(
                table,
                new_version = max_version,
                error = %err,
                "Failed to advance CT watermark"
            );
        } else {
            tracing::info!(
                table,
                from = last_seen,
                to = max_version,
                ingested,
                skipped,
                "Advanced CT watermark"
            );
        }
    }

    Ok(())
}

/// One CT row returned by [`fetch_ct_rows`]: the version, the
/// single-character op code, and the joined data wrapped in our test
/// fixture (which trivially backs onto `tiberius::Row`).
type CtRow = (
    i64,
    String,
    hotel_backend::sync::row::test_support::HashMapRow,
);
// Note: above type alias intentionally points at HashMapRow only because
// that's the test impl; we re-shape tiberius rows into the same
// HashMap-backed type below so the dispatch path works against
// `MappableRow` uniformly. This keeps a single code path for both
// production and tests, at the cost of one extra copy per row (small
// — column counts are <16, all cells are short strings or i32s).

#[cfg(test)]
mod sync_row_alias_compile_check {
    // Compile-time guard that the alias above stays in sync with the
    // public `test_support` re-export the watcher binary depends on.
    use super::CtRow;
    fn _assert_send(_v: CtRow) {}
}

/// Fetch CT rows joined with the table, filtered by loop-prevention
/// `SYS_CHANGE_CONTEXT <> 0x4E48`, ordered by `SYS_CHANGE_VERSION` for
/// monotonic processing.
async fn fetch_ct_rows(
    mssql: &DbPool,
    table: &str,
    pk_cols: &[&str],
    select_sql: &str,
    last_seen: i64,
) -> Result<Vec<CtRow>, String> {
    let mut conn = mssql.get().await.map_err(|e| e.to_string())?;

    // Build the JOIN condition: `t.pk1 = ct.pk1 AND t.pk2 = ct.pk2 …`.
    // For a single-PK table (our 5.2 mappers) this is trivial.
    let join_clause = if pk_cols.is_empty() {
        // Fallback that should never happen for a real mapper; the
        // NoopMapper short-circuit upstream owns that path.
        return Err("real mapper must declare primary_key_cols".into());
    } else {
        pk_cols
            .iter()
            .map(|c| format!("t.{c} = ct.{c}"))
            .collect::<Vec<_>>()
            .join(" AND ")
    };

    // Build the PK projection list for the SELECT (we always include
    // the PK columns from CT itself so D rows still carry them even
    // when `t.*` is NULL).
    let pk_projection = pk_cols
        .iter()
        .map(|c| format!("ct.{c} AS pk_{c}"))
        .collect::<Vec<_>>()
        .join(", ");

    let sql = format!(
        "SELECT ct.SYS_CHANGE_VERSION AS sys_change_version, \
                ct.SYS_CHANGE_OPERATION AS sys_change_operation, \
                {pk_projection}, \
                {select_sql} \
           FROM CHANGETABLE(CHANGES {table}, {last_seen}) AS ct \
           LEFT JOIN {table} AS t ON {join_clause} \
          WHERE ct.SYS_CHANGE_CONTEXT IS NULL \
             OR ct.SYS_CHANGE_CONTEXT <> 0x4E48 \
          ORDER BY ct.SYS_CHANGE_VERSION ASC"
    );

    let stream = conn.simple_query(&sql).await.map_err(|e| e.to_string())?;
    let rows = stream
        .into_first_result()
        .await
        .map_err(|e| e.to_string())?;

    let mut out: Vec<CtRow> = Vec::with_capacity(rows.len());
    for r in rows {
        let version: i64 = r.get("sys_change_version").unwrap_or(0);
        // SYS_CHANGE_OPERATION is always one of 'I' / 'U' / 'D' (single
        // char); tiberius surfaces it as `&str`.
        let op_char: String = r
            .get::<&str, _>("sys_change_operation")
            .unwrap_or("?")
            .to_string();
        out.push((version, op_char, materialise_row(&r, pk_cols, select_sql)));
    }
    Ok(out)
}

/// Copy a tiberius row's columns into a `HashMapRow` so the rest of the
/// dispatch path can use the `MappableRow` trait uniformly. Strictly a
/// boundary translator — both sides of the column list (PK + projection)
/// are addressed by the original column names the mapper requested.
fn materialise_row(
    row: &tiberius::Row,
    pk_cols: &[&str],
    select_sql: &str,
) -> hotel_backend::sync::row::test_support::HashMapRow {
    use hotel_backend::sync::row::test_support::{HashMapRow, MockValue};

    let mut h = HashMapRow::new("ct_row");

    // PK columns: surfaced under both `pk_<name>` (the SELECT alias)
    // and `<name>` (what the mapper looks up). For D rows the joined
    // table column is NULL but the CT-side `pk_<name>` is populated.
    for col in pk_cols {
        let pk_alias = format!("pk_{col}");
        if let Some(v) = read_cell(row, &pk_alias) {
            h.cells.insert((*col).to_string(), v);
        }
    }

    // Projection columns: the mapper specified them as `t.<col>` in
    // select_sql; tiberius exposes them under just `<col>` in the
    // result row. We pull every identifier mentioned in select_sql
    // (after a `t.`) and copy whatever value tiberius surfaces.
    for col in extract_projection_columns(select_sql) {
        if let Some(v) = read_cell(row, &col) {
            h.cells.insert(col, v);
        } else {
            // Column was NULL — record it so the mapper sees an
            // explicit None instead of "missing column".
            h.cells.insert(col, MockValue::Null);
        }
    }

    h
}

/// Pull `t.<column>` identifiers out of a select clause like
/// `"t.Cust_no, t.Cust_name, t.Cust_perfix"`. Tolerant of whitespace
/// and trailing commas; ignores anything that isn't `t.<ident>`.
fn extract_projection_columns(select_sql: &str) -> Vec<String> {
    select_sql
        .split(',')
        .filter_map(|p| {
            let p = p.trim();
            p.strip_prefix("t.").map(|s| s.trim().to_string())
        })
        .collect()
}

/// Read one cell from a tiberius row, choosing a wrapper based on the
/// type tiberius surfaces. Probes types in the order our 5.2 mappers
/// actually use (str → i32 → i64 → f64 → datetime). Returns `None` if
/// the cell is SQL NULL.
fn read_cell(
    row: &tiberius::Row,
    col: &str,
) -> Option<hotel_backend::sync::row::test_support::MockValue> {
    use hotel_backend::sync::row::test_support::MockValue;

    if let Ok(Some(s)) = tiberius::Row::try_get::<&str, _>(row, col) {
        return Some(MockValue::Str(s.to_string()));
    }
    if let Ok(Some(n)) = tiberius::Row::try_get::<i32, _>(row, col) {
        return Some(MockValue::I32(n));
    }
    if let Ok(Some(n)) = tiberius::Row::try_get::<i64, _>(row, col) {
        return Some(MockValue::I64(n));
    }
    if let Ok(Some(n)) = tiberius::Row::try_get::<f64, _>(row, col) {
        return Some(MockValue::F64(n));
    }
    if let Ok(Some(d)) = tiberius::Row::try_get::<chrono::NaiveDateTime, _>(row, col) {
        return Some(MockValue::DateTime(d));
    }
    None
}

/// Persist a `DomainEvent` into `event_log` inside the caller's TX.
/// Wraps `EventBus::publish` so the watcher and the service layer take
/// the same code path.
async fn persist_event(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    event: &DomainEvent,
) -> Result<(), sqlx::Error> {
    let _id = EventBus::publish(tx, event).await?;
    Ok(())
}

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
        None => return Ok(()),
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

async fn count_ct_rows(
    mssql: &DbPool,
    table: &str,
    last_seen: i64,
) -> Result<i64, String> {
    let mut conn = mssql.get().await.map_err(|e| e.to_string())?;
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

async fn bump_counters(
    pg: &PgPool,
    table: &str,
    ingested: i64,
    skipped: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE legacy_sync_status \
            SET rows_ingested        = rows_ingested + $2, \
                rows_skipped         = rows_skipped + $3, \
                last_processed_at    = now(), \
                last_error           = NULL, \
                last_error_at        = NULL, \
                consecutive_failures = 0 \
          WHERE table_name = $1",
    )
    .bind(table)
    .bind(ingested)
    .bind(skipped)
    .execute(pg)
    .await?;
    Ok(())
}

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
        assert_eq!(mappers.len(), 10, "10 CT-enabled tables expected");
    }

    #[test]
    fn build_mappers_filters_by_allowlist() {
        let mut allow = HashSet::new();
        allow.insert("HT_Customers".to_string());
        allow.insert("HT_Rooms".to_string());
        let mappers = build_mappers(&Some(allow));
        assert_eq!(mappers.len(), 2);
    }

    /// Phase 5.2: customer + room are now real mappers; the rest are
    /// still NoopMapper. The mapper for `HT_Customers` must not be a
    /// NoopMapper anymore — locks the wiring so a refactor doesn't
    /// silently regress the customer mapper to no-op.
    #[test]
    fn build_mappers_wires_customer_to_customer_mapper() {
        let mut allow = HashSet::new();
        allow.insert("HT_Customers".to_string());
        let mappers = build_mappers(&Some(allow));
        assert_eq!(mappers.len(), 1);
        // The CustomerMapper has primary_key_cols == &["id"]; NoopMapper
        // has &[]. Use that as the structural assertion.
        assert_eq!(
            mappers[0].primary_key_cols(),
            &["id"],
            "HT_Customers must be wired to CustomerMapper, not NoopMapper"
        );
    }

    #[test]
    fn build_mappers_wires_rooms_to_room_master_mapper() {
        let mut allow = HashSet::new();
        allow.insert("HT_Rooms".to_string());
        let mappers = build_mappers(&Some(allow));
        assert_eq!(mappers.len(), 1);
        assert_eq!(mappers[0].primary_key_cols(), &["id"]);
        assert!(mappers[0].select_sql().contains("Room_Clean"));
    }

    #[test]
    fn build_mappers_wires_room_status_to_room_status_mapper() {
        let mut allow = HashSet::new();
        allow.insert("HT_Room_Status".to_string());
        let mappers = build_mappers(&Some(allow));
        assert_eq!(mappers.len(), 1);
        assert_eq!(mappers[0].primary_key_cols(), &["id"]);
    }

    /// Phase 5.3: HT_Book_H + HT_Book_Ds + HT_Book_Date are now real
    /// mappers (BookingHeaderMapper / BookingRoomsMapper /
    /// BookingDatesMapper). Locks the wiring so a refactor doesn't
    /// silently regress them to NoopMapper.
    #[test]
    fn build_mappers_wires_booking_header_to_booking_header_mapper() {
        let mut allow = HashSet::new();
        allow.insert("HT_Book_H".to_string());
        let mappers = build_mappers(&Some(allow));
        assert_eq!(mappers.len(), 1);
        assert_eq!(
            mappers[0].primary_key_cols(),
            &["Book_ID"],
            "HT_Book_H must be wired to BookingHeaderMapper (PK=Book_ID), not NoopMapper"
        );
    }

    #[test]
    fn build_mappers_wires_booking_ds_to_booking_rooms_mapper() {
        let mut allow = HashSet::new();
        allow.insert("HT_Book_Ds".to_string());
        let mappers = build_mappers(&Some(allow));
        assert_eq!(mappers.len(), 1);
        assert_eq!(mappers[0].primary_key_cols(), &["id"]);
        assert!(mappers[0].select_sql().contains("Book_Room_Type"));
    }

    #[test]
    fn build_mappers_wires_booking_date_to_booking_dates_mapper() {
        let mut allow = HashSet::new();
        allow.insert("HT_Book_Date".to_string());
        let mappers = build_mappers(&Some(allow));
        assert_eq!(mappers.len(), 1);
        assert_eq!(mappers[0].primary_key_cols(), &["id"]);
        assert!(mappers[0].select_sql().contains("Book_ok"));
    }

    /// 5.4 wired the four checkin/receipt tables to real mappers.
    /// Locks the wiring so a refactor can't silently regress them
    /// back to NoopMapper.
    #[test]
    fn build_mappers_wires_5_4_tables_to_real_mappers() {
        let cases: &[(&str, &[&str])] = &[
            ("HT_CheckIn_H", &["Cin_no"]),
            ("HT_CheckIn_Ds", &["id"]),
            ("HT_CheckIn_Pay", &["id"]),
            ("HT_Receipt_H", &["id"]),
        ];
        for (t, expected_pk) in cases {
            let mut allow = HashSet::new();
            allow.insert((*t).to_string());
            let mappers = build_mappers(&Some(allow));
            assert_eq!(mappers.len(), 1, "{t}: expected one mapper");
            assert_eq!(
                mappers[0].primary_key_cols(),
                *expected_pk,
                "{t} must be wired to its real 5.4 mapper, not NoopMapper"
            );
        }
    }

    #[test]
    fn ct_enabled_tables_match_migration_017_seed() {
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

    #[test]
    fn extract_projection_columns_strips_t_prefix() {
        let cols = extract_projection_columns(
            "t.Cust_no, t.Cust_name, t.Cust_Add_tel",
        );
        assert_eq!(cols, vec!["Cust_no", "Cust_name", "Cust_Add_tel"]);
    }

    #[test]
    fn extract_projection_columns_tolerates_whitespace_and_trailing_comma() {
        let cols = extract_projection_columns(
            "  t.id,  t.Room_no ,  t.Room_Type ,",
        );
        assert_eq!(cols, vec!["id", "Room_no", "Room_Type"]);
    }

    #[test]
    fn extract_projection_columns_skips_non_t_qualified_entries() {
        let cols = extract_projection_columns(
            "t.id, ct.SYS_CHANGE_VERSION AS v, t.Room_no",
        );
        assert_eq!(cols, vec!["id", "Room_no"]);
    }

    /// CHANGETABLE filter must include the SYS_CHANGE_CONTEXT clause
    /// matching the `SET CONTEXT_INFO 0x4E48` value the writeback
    /// dispatcher stamps. Locks the byte literal so a refactor can't
    /// silently break loop-prevention. The check is a substring match
    /// on `0x4E48` near `SYS_CHANGE_CONTEXT` — both fetch_ct_rows and
    /// count_ct_rows compose the clause across multiple source lines,
    /// so an exact-string match would be fragile.
    #[test]
    fn fetch_ct_rows_uses_loop_prevention_filter() {
        let source = include_str!("sync.rs");
        // Both occurrences of the literal must be paired with the
        // SYS_CHANGE_CONTEXT predicate — this guards both the JOIN
        // SELECT and the COUNT SELECT.
        let ctx_count = source.matches("SYS_CHANGE_CONTEXT").count();
        let tag_count = source.matches("0x4E48").count();
        assert!(
            ctx_count >= 2 && tag_count >= 2,
            "expected ≥2 occurrences of SYS_CHANGE_CONTEXT + 0x4E48 (JOIN + COUNT paths)"
        );
    }

    /// The CONTEXT_INFO value used by writeback's dispatcher and the
    /// CT watcher's filter must be the same byte sequence — otherwise
    /// every writeback re-fires through the watcher.
    #[test]
    fn loop_prevention_tag_matches_writeback_dispatcher() {
        let dispatcher_src = include_str!("../writeback/dispatcher.rs");
        let mssql_session_src = include_str!("../db/mssql_session.rs");
        assert!(dispatcher_src.contains("set_context_info(conn)"));
        assert!(
            mssql_session_src.contains("SET CONTEXT_INFO 0x4E48"),
            "mssql_session::set_context_info must issue 0x4E48 \
             so the watcher's CHANGETABLE filter can match it"
        );
    }
}
