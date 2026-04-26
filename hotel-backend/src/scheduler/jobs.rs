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

use crate::config::SlackConfig;
use crate::db::{DbPool, PgPool};
use crate::notifications::slack::{
    build_check_in_alert_message, build_check_out_alert_message, build_hourly_report_message,
    build_new_booking_alert_message, SlackClient,
};
use super::sync;

/// Scheduler state for tracking last polled timestamps
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

/// Initialize the cron job scheduler
pub async fn init_scheduler(
    pool: DbPool,
    pg_pool: Option<PgPool>,
    slack_config: SlackConfig,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing::info!("[Scheduler] Starting cron jobs...");

    let scheduler = JobScheduler::new().await?;
    let state = Arc::new(Mutex::new(SchedulerState::default()));

    // Legacy reconcile job — Phase 5.5: demoted from 5-min full UPSERT
    // to a 15-min diff-only safety net. The CT watcher (`bin/sync.rs`)
    // is now authoritative for canonical PG state; this job's role is
    // to LOG drift into `ht_reconcile_log` so an operator can investigate
    // (mode controlled by env var `LEGACY_SYNC_RECONCILE_MODE`).
    //
    // Per docs/architecture.md §3.6d, §8 (Phase 5.5 row).
    if let Some(ref pg) = pg_pool {
        let sync_legacy = pool.clone();
        let sync_pg = pg.clone();

        let sync_enabled = std::env::var("SYNC_ENABLED")
            .map(|v| v != "false")
            .unwrap_or(true);

        if sync_enabled {
            // Cron: minute 0 of every 15 minutes (00, 15, 30, 45). Quarters of
            // the hour are easier for operators to correlate with logs than
            // an arbitrary 15-min phase.
            let sync_job = Job::new_async("0 */15 * * * *", move |_uuid, _l| {
                let legacy = sync_legacy.clone();
                let pg = sync_pg.clone();
                Box::pin(async move {
                    sync::run_sync(&legacy, &pg).await;
                })
            })?;
            scheduler.add(sync_job).await?;
            tracing::info!(
                "[Scheduler] - Legacy reconcile: every 15 minutes (Phase 5.5 — diff-only safety net)"
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

    let pool_checkins = pool.clone();
    let slack_checkins = SlackClient::new(slack_config.clone());
    let state_checkins = state.clone();

    let pool_checkouts = pool.clone();
    let slack_checkouts = SlackClient::new(slack_config.clone());
    let state_checkouts = state.clone();

    let pool_bookings = pool.clone();
    let slack_bookings = SlackClient::new(slack_config.clone());
    let state_bookings = state.clone();

    // Hourly report at minute 0 of every hour
    let hourly_job = Job::new_async("0 0 * * * *", move |_uuid, _l| {
        let pool = pool_hourly.clone();
        let slack = slack_hourly.clone();
        Box::pin(async move {
            if let Err(e) = send_hourly_report(&pool, &slack).await {
                tracing::error!("[Scheduler] Error in hourly report: {}", e);
            }
        })
    })?;
    scheduler.add(hourly_job).await?;

    // Poll for check-ins every 2 minutes
    let checkin_job = Job::new_async("0 */2 * * * *", move |_uuid, _l| {
        let pool = pool_checkins.clone();
        let slack = slack_checkins.clone();
        let state = state_checkins.clone();
        Box::pin(async move {
            if let Err(e) = poll_checkins(&pool, &slack, &state).await {
                tracing::error!("[Scheduler] Error polling check-ins: {}", e);
            }
        })
    })?;
    scheduler.add(checkin_job).await?;

    // Poll for checkouts every 2 minutes
    let checkout_job = Job::new_async("0 */2 * * * *", move |_uuid, _l| {
        let pool = pool_checkouts.clone();
        let slack = slack_checkouts.clone();
        let state = state_checkouts.clone();
        Box::pin(async move {
            if let Err(e) = poll_checkouts(&pool, &slack, &state).await {
                tracing::error!("[Scheduler] Error polling checkouts: {}", e);
            }
        })
    })?;
    scheduler.add(checkout_job).await?;

    // Poll for new bookings every 2 minutes
    let booking_job = Job::new_async("0 */2 * * * *", move |_uuid, _l| {
        let pool = pool_bookings.clone();
        let slack = slack_bookings.clone();
        let state = state_bookings.clone();
        Box::pin(async move {
            if let Err(e) = poll_new_bookings(&pool, &slack, &state).await {
                tracing::error!("[Scheduler] Error polling bookings: {}", e);
            }
        })
    })?;
    scheduler.add(booking_job).await?;

    scheduler.start().await?;

    tracing::info!("[Scheduler] Cron jobs started");
    tracing::info!("[Scheduler] - Hourly report: minute 0 of every hour");
    tracing::info!("[Scheduler] - Check-in polling: every 2 minutes");
    tracing::info!("[Scheduler] - Checkout polling: every 2 minutes");
    tracing::info!("[Scheduler] - Booking polling: every 2 minutes");

    Ok(())
}

/// Send hourly report to Slack
async fn send_hourly_report(
    pool: &DbPool,
    slack: &SlackClient,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing::info!("[Scheduler] Running hourly report...");

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

    let message = build_hourly_report_message(occupied_rooms, total_rooms, today_bookings);
    slack.send_message(&message).await;

    tracing::info!("[Scheduler] Hourly report sent successfully");
    Ok(())
}

/// Poll for new check-ins and send alerts
async fn poll_checkins(
    pool: &DbPool,
    slack: &SlackClient,
    state: &Arc<Mutex<SchedulerState>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing::debug!("[Scheduler] Polling for new check-ins...");

    let mut conn = pool.get().await?;
    let mut state = state.lock().await;

    // Initialize timestamp on first run
    if state.last_checkin_timestamp.is_none() {
        state.last_checkin_timestamp = Some(chrono::Utc::now().naive_utc());
        tracing::info!(
            "[Scheduler] Initialized check-in polling from: {:?}",
            state.last_checkin_timestamp
        );
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
                let message = build_check_in_alert_message(&guest_name, &room_no, time);
                slack.send_message(&message).await;

                // Update last checked timestamp
                if time > state.last_checkin_timestamp.unwrap() {
                    state.last_checkin_timestamp = Some(time);
                }
            }
        }
    } else {
        tracing::debug!("[Scheduler] No new check-ins found");
    }

    Ok(())
}

/// Poll for new checkouts and send alerts
async fn poll_checkouts(
    pool: &DbPool,
    slack: &SlackClient,
    state: &Arc<Mutex<SchedulerState>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing::debug!("[Scheduler] Polling for new checkouts...");

    let mut conn = pool.get().await?;
    let mut state = state.lock().await;

    // Initialize timestamp on first run
    if state.last_checkout_timestamp.is_none() {
        state.last_checkout_timestamp = Some(chrono::Utc::now().naive_utc());
        tracing::info!(
            "[Scheduler] Initialized checkout polling from: {:?}",
            state.last_checkout_timestamp
        );
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
                let message = build_check_out_alert_message(&guest_name, &room_no, time);
                slack.send_message(&message).await;

                // Update last checked timestamp
                if time > state.last_checkout_timestamp.unwrap() {
                    state.last_checkout_timestamp = Some(time);
                }
            }
        }
    } else {
        tracing::debug!("[Scheduler] No new checkouts found");
    }

    Ok(())
}

/// Poll for new bookings and send alerts
async fn poll_new_bookings(
    pool: &DbPool,
    slack: &SlackClient,
    state: &Arc<Mutex<SchedulerState>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing::debug!("[Scheduler] Polling for new bookings...");

    let mut conn = pool.get().await?;
    let mut state = state.lock().await;

    // Initialize timestamp on first run
    if state.last_booking_timestamp.is_none() {
        state.last_booking_timestamp = Some(chrono::Utc::now().naive_utc());
        tracing::info!(
            "[Scheduler] Initialized booking polling from: {:?}",
            state.last_booking_timestamp
        );
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
                let message = build_new_booking_alert_message(&guest_name, &room_type, cin, cout);
                slack.send_message(&message).await;

                // Update last checked timestamp
                if let Some(time) = booking_time {
                    if time > state.last_booking_timestamp.unwrap() {
                        state.last_booking_timestamp = Some(time);
                    }
                }
            }
        }
    } else {
        tracing::debug!("[Scheduler] No new bookings found");
    }

    Ok(())
}
