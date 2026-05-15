//! Background cron jobs for hotel management
//!
//! Jobs:
//! - Hourly report at minute 0 of every hour
//! - Poll for new check-ins every 2 minutes
//! - Poll for new checkouts every 2 minutes
//! - Poll for new bookings every 2 minutes

use chrono::NaiveDateTime;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::config::{SiteConfig, SlackConfig};
use crate::db::{DbPool, PgPool};
use crate::notifications::slack::{
    build_check_in_alert_message, build_check_out_alert_message, build_hourly_report_message,
    build_new_booking_alert_message, format_site_prefixed, SlackClient, SlackMessage,
};
use super::notification_state::{
    load_watermark, now_thai_local, save_watermark, NotificationType,
};
use super::sync;

/// Scheduler state for tracking last polled timestamps in-process.
///
/// These fields are a CACHE on top of `scheduler_notification_state`
/// (PG, migration 037). The first poll for each notification type after
/// a process restart hydrates from PG; subsequent polls write back to
/// PG every time the in-memory watermark advances. See
/// `scheduler::notification_state` for the why — TL;DR: the previous
/// in-memory-only design caused a ~45-message Slack replay storm on
/// every container redeploy because the seed value (UTC-now) didn't
/// match the Thai-local timezone of the MSSQL source columns.
struct SchedulerState {
    last_checkin_timestamp: Option<NaiveDateTime>,
    last_checkout_timestamp: Option<NaiveDateTime>,
    last_booking_timestamp: Option<NaiveDateTime>,
}

impl Default for SchedulerState {
    fn default() -> Self {
        Self {
            last_checkin_timestamp: None,
            last_checkout_timestamp: None,
            last_booking_timestamp: None,
        }
    }
}

/// Initialize the cron job scheduler.
///
/// `site` is plumbed through so every Slack message and tracing span
/// carries `site=<id>` (task #69). HF Hotel and HF Ville share a single
/// Slack webhook from Phase 5 onward, so without this prefix an on-call
/// operator can't tell which deployment fired the alert.
pub async fn init_scheduler(
    pool: DbPool,
    pg_pool: Option<PgPool>,
    slack_config: SlackConfig,
    site: SiteConfig,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing::info!(site = %site.id, "[Scheduler] Starting cron jobs...");

    let scheduler = JobScheduler::new().await?;
    let state = Arc::new(Mutex::new(SchedulerState::default()));

    // Legacy reconcile job — Phase 6 (built on Phase 5.5 demotion).
    //
    // Cadence: minute 0 of every 15 minutes (00, 15, 30, 45). Quarters
    // of the hour are easier for operators to correlate with logs than
    // an arbitrary 15-min phase. We deliberately do NOT poll faster:
    // the CT watcher (`bin/sync.rs`) is the real-time path with sub-
    // second latency; this reconcile is a slower safety net that
    // surfaces rows the watcher missed (CT retention overflow,
    // transient mapper bug, schema regression). Polling more often
    // would only add legacy-MSSQL load without changing the recovery
    // posture — operator response time to a Slack alert dominates.
    //
    // Mode: `LEGACY_SYNC_RECONCILE_MODE` defaults to `diff_only` (see
    // `ReconcileMode::from_env`). `diff_only` LOGS divergent rows into
    // `ht_reconcile_log` instead of mutating canonical state, which
    // the CT watcher owns. `upsert` remains as an escape hatch.
    //
    // Phase 6 alerting: at the end of each tick the job counts
    // unresolved `ht_reconcile_log` rows in the last hour, grouped by
    // `table_name`, and fires a `:rotating_light:` Slack alert if any
    // table breaches `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD`
    // (default 50). Threshold pure-function logic is unit-tested in
    // `scheduler::sync::tests`.
    //
    // Per docs/architecture.md §3.6d, §8 (Phase 6 row); runbook §9.
    if let Some(ref pg) = pg_pool {
        let sync_legacy = pool.clone();
        let sync_pg = pg.clone();
        let sync_slack: Option<SlackClient> = if slack_config.is_configured() {
            Some(SlackClient::new(slack_config.clone()))
        } else {
            None
        };

        let sync_enabled = std::env::var("SYNC_ENABLED")
            .map(|v| v != "false")
            .unwrap_or(true);

        if sync_enabled {
            let sync_site_id = site.id.clone();
            let sync_job = Job::new_async("0 */15 * * * *", move |_uuid, _l| {
                let legacy = sync_legacy.clone();
                let pg = sync_pg.clone();
                let slack = sync_slack.clone();
                let site_id = sync_site_id.clone();
                Box::pin(async move {
                    let span = tracing::info_span!("reconcile_tick", site = %site_id);
                    let _enter = span.enter();
                    sync::run_sync(&legacy, &pg, slack.as_ref(), &site_id).await;
                })
            })?;
            scheduler.add(sync_job).await?;
            tracing::info!(
                site = %site.id,
                "[Scheduler] - Legacy reconcile: every 15 minutes (Phase 6 — diff-only safety net + drift alerts)"
            );
        } else {
            tracing::info!("[Scheduler] - Legacy reconcile: DISABLED (SYNC_ENABLED=false)");
        }
    }

    // Slack notification jobs only run if Slack is configured
    if !slack_config.is_configured() {
        tracing::info!("[Scheduler] Slack not configured, notification jobs not started");
        scheduler.start().await?;
        return Ok(());
    }

    // Clone resources for each job
    let pool_hourly = pool.clone();
    let slack_hourly = SlackClient::new(slack_config.clone());

    // PG pool is OPTIONAL for the polling jobs — if absent, the
    // scheduler falls back to the legacy in-memory-only watermark
    // behavior (which has the post-redeploy replay bug). In all
    // deployment shapes today PG is available, but we keep the option
    // to match the existing init_scheduler signature.
    let pool_checkins = pool.clone();
    let slack_checkins = SlackClient::new(slack_config.clone());
    let state_checkins = state.clone();
    let pg_checkins = pg_pool.clone();

    let pool_checkouts = pool.clone();
    let slack_checkouts = SlackClient::new(slack_config.clone());
    let state_checkouts = state.clone();
    let pg_checkouts = pg_pool.clone();

    let pool_bookings = pool.clone();
    let slack_bookings = SlackClient::new(slack_config.clone());
    let state_bookings = state.clone();
    let pg_bookings = pg_pool.clone();

    // Site id is small + Clone-cheap so we just clone it into each
    // closure rather than wrapping in an Arc.
    let hourly_site = site.id.clone();
    let checkin_site = site.id.clone();
    let checkout_site = site.id.clone();
    let booking_site = site.id.clone();

    // Hourly report at minute 0 of every hour.
    //
    // Feature flag (2026-05-15): `HOURLY_REPORT_ENABLED=false` skips
    // registration entirely so the cron never fires. Set during the
    // CT-watcher backfill window (`backfill_legacy_checkins` +
    // `backfill_legacy_bookings`) to suppress the hourly occupancy
    // summary while we're churning canonical state and the numbers
    // would mislead the receptionist channel. Default: enabled.
    let hourly_enabled = std::env::var("HOURLY_REPORT_ENABLED")
        .map(|v| v != "false")
        .unwrap_or(true);
    if hourly_enabled {
        let hourly_job = Job::new_async("0 0 * * * *", move |_uuid, _l| {
            let pool = pool_hourly.clone();
            let slack = slack_hourly.clone();
            let site_id = hourly_site.clone();
            Box::pin(async move {
                if let Err(e) = send_hourly_report(&pool, &slack, &site_id).await {
                    tracing::error!(site = %site_id, "[Scheduler] Error in hourly report: {}", e);
                }
            })
        })?;
        scheduler.add(hourly_job).await?;
    } else {
        tracing::info!(
            "[Scheduler] - Hourly report: DISABLED (HOURLY_REPORT_ENABLED=false)"
        );
    }

    // Poll for check-ins every 2 minutes.
    //
    // Feature flag (2026-05-15): `CHECKIN_NOTIFICATIONS_ENABLED=false`
    // skips registration entirely so the new-checkin Slack alert
    // doesn't fire. Set during the CT-watcher backfill window — the
    // backfill binaries deliberately do not publish DomainEvents, but
    // operators may still want a hard kill-switch on the poll-based
    // alert path while batch-importing historical state. Default:
    // enabled.
    let checkin_notifications_enabled = std::env::var("CHECKIN_NOTIFICATIONS_ENABLED")
        .map(|v| v != "false")
        .unwrap_or(true);
    if checkin_notifications_enabled {
        let checkin_job = Job::new_async("0 */2 * * * *", move |_uuid, _l| {
            let pool = pool_checkins.clone();
            let slack = slack_checkins.clone();
            let state = state_checkins.clone();
            let site_id = checkin_site.clone();
            let pg = pg_checkins.clone();
            Box::pin(async move {
                if let Err(e) = poll_checkins(&pool, pg.as_ref(), &slack, &state, &site_id).await
                {
                    tracing::error!(site = %site_id, "[Scheduler] Error polling check-ins: {}", e);
                }
            })
        })?;
        scheduler.add(checkin_job).await?;
    } else {
        tracing::info!(
            "[Scheduler] - Check-in polling: DISABLED (CHECKIN_NOTIFICATIONS_ENABLED=false)"
        );
    }

    // Poll for checkouts every 2 minutes
    let checkout_job = Job::new_async("0 */2 * * * *", move |_uuid, _l| {
        let pool = pool_checkouts.clone();
        let slack = slack_checkouts.clone();
        let state = state_checkouts.clone();
        let site_id = checkout_site.clone();
        let pg = pg_checkouts.clone();
        Box::pin(async move {
            if let Err(e) = poll_checkouts(&pool, pg.as_ref(), &slack, &state, &site_id).await {
                tracing::error!(site = %site_id, "[Scheduler] Error polling checkouts: {}", e);
            }
        })
    })?;
    scheduler.add(checkout_job).await?;

    // Poll for new bookings every 2 minutes
    let booking_job = Job::new_async("0 */2 * * * *", move |_uuid, _l| {
        let pool = pool_bookings.clone();
        let slack = slack_bookings.clone();
        let state = state_bookings.clone();
        let site_id = booking_site.clone();
        let pg = pg_bookings.clone();
        Box::pin(async move {
            if let Err(e) = poll_new_bookings(&pool, pg.as_ref(), &slack, &state, &site_id).await {
                tracing::error!(site = %site_id, "[Scheduler] Error polling bookings: {}", e);
            }
        })
    })?;
    scheduler.add(booking_job).await?;

    scheduler.start().await?;

    tracing::info!("[Scheduler] Cron jobs started");
    if hourly_enabled {
        tracing::info!("[Scheduler] - Hourly report: minute 0 of every hour");
    }
    if checkin_notifications_enabled {
        tracing::info!("[Scheduler] - Check-in polling: every 2 minutes");
    }
    tracing::info!("[Scheduler] - Checkout polling: every 2 minutes");
    tracing::info!("[Scheduler] - Booking polling: every 2 minutes");

    Ok(())
}

/// Apply the task #69 site-id prefix to a Block-Kit Slack message in
/// place. The `text` field is what shows in Slack notification previews
/// and channel summaries — prefixing it lets an operator triage which
/// site fired the alert without expanding the full block payload.
fn prefix_message_with_site(message: &mut SlackMessage, site_id: &str) {
    let prefixed = format_site_prefixed(site_id, &message.text);
    message.text = prefixed;
}

/// Send hourly report to Slack
async fn send_hourly_report(
    pool: &DbPool,
    slack: &SlackClient,
    site_id: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing::info!(site = %site_id, "[Scheduler] Running hourly report...");

    let mut conn = pool.get().await?;

    // Get occupied rooms count
    let occupied_rows = conn
        .simple_query(
            r#"
            SELECT COUNT(*) as count FROM HT_Rooms
            WHERE Room_Use = 'yes' OR Room_Book = 'yes'
            "#,
        )
        .await?
        .into_first_result()
        .await?;
    let occupied_rooms: i32 = occupied_rows
        .first()
        .and_then(|r| r.get::<i32, _>("count"))
        .unwrap_or(0);

    // Get total rooms count
    let total_rows = conn
        .simple_query("SELECT COUNT(*) as count FROM HT_Rooms")
        .await?
        .into_first_result()
        .await?;
    let total_rooms: i32 = total_rows
        .first()
        .and_then(|r| r.get::<i32, _>("count"))
        .unwrap_or(0);

    // Get today's new bookings count
    let bookings_rows = conn
        .simple_query(
            r#"
            SELECT COUNT(*) as count FROM View_Booking_Ds
            WHERE CAST(Book_Date AS DATE) = CAST(GETDATE() AS DATE)
            "#,
        )
        .await?
        .into_first_result()
        .await?;
    let today_bookings: i32 = bookings_rows
        .first()
        .and_then(|r| r.get::<i32, _>("count"))
        .unwrap_or(0);

    let mut message = build_hourly_report_message(occupied_rooms, total_rooms, today_bookings);
    prefix_message_with_site(&mut message, site_id);
    slack.send_message(&message).await;

    tracing::info!(site = %site_id, "[Scheduler] Hourly report sent successfully");
    Ok(())
}

/// Poll for new check-ins and send alerts
async fn poll_checkins(
    pool: &DbPool,
    pg: Option<&PgPool>,
    slack: &SlackClient,
    state: &Arc<Mutex<SchedulerState>>,
    site_id: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing::debug!(site = %site_id, "[Scheduler] Polling for new check-ins...");

    let mut conn = pool.get().await?;
    let mut state = state.lock().await;

    // Hydrate the watermark on first poll after process start. PG is
    // authoritative — if a previous backend container already paged
    // an event we MUST resume past it, otherwise we re-emit (the bug
    // this whole subsystem exists to fix).
    if state.last_checkin_timestamp.is_none() {
        let initial = hydrate_or_seed_watermark(
            pg,
            site_id,
            NotificationType::Checkin,
            "check-in",
        )
        .await;
        state.last_checkin_timestamp = Some(initial);
        return Ok(());
    }

    let last_checked = state.last_checkin_timestamp.unwrap();
    let last_checked_str = last_checked.format("%Y-%m-%d %H:%M:%S").to_string();

    let rows = conn
        .query(
            r#"
            SELECT Cin_no, Cin_Room_No, Cin_Room_In, Cin_cust_name
            FROM View_CheckIn_Ds
            WHERE Cin_Room_In > @P1
            ORDER BY Cin_Room_In ASC
            "#,
            &[&last_checked_str.as_str()],
        )
        .await?
        .into_first_result()
        .await?;

    if !rows.is_empty() {
        tracing::info!("[Scheduler] Found {} new check-in(s)", rows.len());

        let mut advanced_to: Option<NaiveDateTime> = None;
        for row in &rows {
            let guest_name = row
                .get::<&str, _>("Cin_cust_name")
                .unwrap_or("Unknown")
                .to_string();
            let room_no = row
                .get::<&str, _>("Cin_Room_No")
                .unwrap_or("Unknown")
                .to_string();
            let check_in_time = row.get::<NaiveDateTime, _>("Cin_Room_In");

            if let Some(time) = check_in_time {
                let mut message = build_check_in_alert_message(&guest_name, &room_no, time);
                prefix_message_with_site(&mut message, site_id);
                slack.send_message(&message).await;

                // Update last checked timestamp
                if time > state.last_checkin_timestamp.unwrap() {
                    state.last_checkin_timestamp = Some(time);
                    advanced_to = Some(time);
                }
            }
        }
        if let (Some(ts), Some(pg)) = (advanced_to, pg) {
            persist_watermark(pg, site_id, NotificationType::Checkin, ts, "check-in").await;
        }
    } else {
        tracing::debug!("[Scheduler] No new check-ins found");
    }

    Ok(())
}

/// Poll for new checkouts and send alerts
async fn poll_checkouts(
    pool: &DbPool,
    pg: Option<&PgPool>,
    slack: &SlackClient,
    state: &Arc<Mutex<SchedulerState>>,
    site_id: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing::debug!(site = %site_id, "[Scheduler] Polling for new checkouts...");

    let mut conn = pool.get().await?;
    let mut state = state.lock().await;

    // Hydrate the watermark on first poll after process start. PG is
    // authoritative — if a previous backend container already paged
    // an event we MUST resume past it, otherwise we re-emit ~45
    // historical events (the production-verified bug 2026-05-13).
    if state.last_checkout_timestamp.is_none() {
        let initial = hydrate_or_seed_watermark(
            pg,
            site_id,
            NotificationType::Checkout,
            "checkout",
        )
        .await;
        state.last_checkout_timestamp = Some(initial);
        return Ok(());
    }

    let last_checked = state.last_checkout_timestamp.unwrap();
    let last_checked_str = last_checked.format("%Y-%m-%d %H:%M:%S").to_string();

    let rows = conn
        .query(
            r#"
            SELECT Cin_no, Cin_Room_No, Cin_Room_Out, Cin_cust_name
            FROM View_CheckIn_Ds
            WHERE Cin_Room_Out > @P1
            ORDER BY Cin_Room_Out ASC
            "#,
            &[&last_checked_str.as_str()],
        )
        .await?
        .into_first_result()
        .await?;

    if !rows.is_empty() {
        tracing::info!("[Scheduler] Found {} new checkout(s)", rows.len());

        let mut advanced_to: Option<NaiveDateTime> = None;
        for row in &rows {
            let guest_name = row
                .get::<&str, _>("Cin_cust_name")
                .unwrap_or("Unknown")
                .to_string();
            let room_no = row
                .get::<&str, _>("Cin_Room_No")
                .unwrap_or("Unknown")
                .to_string();
            let checkout_time = row.get::<NaiveDateTime, _>("Cin_Room_Out");

            if let Some(time) = checkout_time {
                let mut message = build_check_out_alert_message(&guest_name, &room_no, time);
                prefix_message_with_site(&mut message, site_id);
                slack.send_message(&message).await;

                // Update last checked timestamp
                if time > state.last_checkout_timestamp.unwrap() {
                    state.last_checkout_timestamp = Some(time);
                    advanced_to = Some(time);
                }
            }
        }
        if let (Some(ts), Some(pg)) = (advanced_to, pg) {
            persist_watermark(pg, site_id, NotificationType::Checkout, ts, "checkout").await;
        }
    } else {
        tracing::debug!("[Scheduler] No new checkouts found");
    }

    Ok(())
}

/// Poll for new bookings and send alerts
async fn poll_new_bookings(
    pool: &DbPool,
    pg: Option<&PgPool>,
    slack: &SlackClient,
    state: &Arc<Mutex<SchedulerState>>,
    site_id: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing::debug!(site = %site_id, "[Scheduler] Polling for new bookings...");

    let mut conn = pool.get().await?;
    let mut state = state.lock().await;

    // Hydrate the watermark on first poll after process start. PG is
    // authoritative — see poll_checkouts for the full rationale.
    if state.last_booking_timestamp.is_none() {
        let initial = hydrate_or_seed_watermark(
            pg,
            site_id,
            NotificationType::Booking,
            "booking",
        )
        .await;
        state.last_booking_timestamp = Some(initial);
        return Ok(());
    }

    let last_checked = state.last_booking_timestamp.unwrap();
    let last_checked_str = last_checked.format("%Y-%m-%d %H:%M:%S").to_string();

    let rows = conn
        .query(
            r#"
            SELECT Book_no, Book_Cust_Name, Book_Room_Type, Book_Date_in, Book_Date_out, Book_Date
            FROM View_Booking_Ds
            WHERE Book_Date > @P1
            ORDER BY Book_Date ASC
            "#,
            &[&last_checked_str.as_str()],
        )
        .await?
        .into_first_result()
        .await?;

    if !rows.is_empty() {
        tracing::info!("[Scheduler] Found {} new booking(s)", rows.len());

        let mut advanced_to: Option<NaiveDateTime> = None;
        for row in &rows {
            let guest_name = row
                .get::<&str, _>("Book_Cust_Name")
                .unwrap_or("Unknown")
                .to_string();
            let room_type = row
                .get::<&str, _>("Book_Room_Type")
                .unwrap_or("Unknown")
                .to_string();
            let check_in_date = row.get::<NaiveDateTime, _>("Book_Date_in");
            let check_out_date = row.get::<NaiveDateTime, _>("Book_Date_out");
            let booking_time = row.get::<NaiveDateTime, _>("Book_Date");

            if let (Some(cin), Some(cout)) = (check_in_date, check_out_date) {
                let mut message = build_new_booking_alert_message(&guest_name, &room_type, cin, cout);
                prefix_message_with_site(&mut message, site_id);
                slack.send_message(&message).await;

                // Update last checked timestamp
                if let Some(time) = booking_time {
                    if time > state.last_booking_timestamp.unwrap() {
                        state.last_booking_timestamp = Some(time);
                        advanced_to = Some(time);
                    }
                }
            }
        }
        if let (Some(ts), Some(pg)) = (advanced_to, pg) {
            persist_watermark(pg, site_id, NotificationType::Booking, ts, "booking").await;
        }
    } else {
        tracing::debug!("[Scheduler] No new bookings found");
    }

    Ok(())
}

/// Resolve the initial watermark for a notification type on cold start.
///
/// Precedence:
///   1. PG `scheduler_notification_state` row if one exists — resume
///      exactly where the previous process left off. This is the path
///      that fixes the replay-on-redeploy bug.
///   2. `now_thai_local()` if PG is reachable but has no row yet, OR
///      if `pg` is `None` (no canonical pool configured). Stored so
///      subsequent polls don't re-seed.
///
/// Always returns a `NaiveDateTime` so the caller can unconditionally
/// `Some(...)` the in-memory cache without an extra error branch. If
/// PG IO fails we LOG and seed with `now_thai_local` — silently
/// re-introducing the legacy in-memory behavior for this tick is
/// better than crashing the polling loop.
async fn hydrate_or_seed_watermark(
    pg: Option<&PgPool>,
    site_id: &str,
    notification_type: NotificationType,
    label_for_log: &str,
) -> NaiveDateTime {
    let Some(pg) = pg else {
        let seed = now_thai_local();
        tracing::info!(
            site = %site_id,
            "[Scheduler] No PG pool — seeding {} watermark in-memory at {:?} (replay-on-restart bug NOT mitigated)",
            label_for_log, seed
        );
        return seed;
    };
    match load_watermark(pg, site_id, notification_type).await {
        Ok(Some(ts)) => {
            tracing::info!(
                site = %site_id,
                "[Scheduler] Resumed {} watermark from PG: {:?}",
                label_for_log, ts
            );
            ts
        }
        Ok(None) => {
            let seed = now_thai_local();
            // Persist immediately so a crash between this seed and the
            // next event-emitting poll doesn't re-seed (which would
            // re-introduce the 7-hour bug if Utc::now was reused).
            if let Err(e) = save_watermark(pg, site_id, notification_type, seed).await {
                tracing::warn!(
                    site = %site_id,
                    "[Scheduler] Failed to persist initial {} watermark to PG: {} — \
                     in-memory only for this process",
                    label_for_log, e
                );
            } else {
                tracing::info!(
                    site = %site_id,
                    "[Scheduler] Seeded {} watermark in PG at {:?} (no prior row)",
                    label_for_log, seed
                );
            }
            seed
        }
        Err(e) => {
            let seed = now_thai_local();
            tracing::warn!(
                site = %site_id,
                "[Scheduler] Failed to load {} watermark from PG: {} — \
                 seeding in-memory at {:?}",
                label_for_log, e, seed
            );
            seed
        }
    }
}

/// Persist an advanced watermark to PG. Log + swallow IO errors so the
/// polling loop continues; the in-memory cache still has the new value
/// so this tick won't re-emit, and the next successful save will catch
/// PG up.
async fn persist_watermark(
    pg: &PgPool,
    site_id: &str,
    notification_type: NotificationType,
    advanced_to: NaiveDateTime,
    label_for_log: &str,
) {
    if let Err(e) = save_watermark(pg, site_id, notification_type, advanced_to).await {
        tracing::warn!(
            site = %site_id,
            "[Scheduler] Failed to persist {} watermark ({:?}) to PG: {} — \
             will retry on next advance",
            label_for_log, advanced_to, e
        );
    }
}
