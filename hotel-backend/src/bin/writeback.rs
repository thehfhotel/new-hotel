//! Writeback Worker Binary
//!
//! Per `docs/architecture.md` §3.6c (publication path), §6 (worked example),
//! §8 Phase 4b. Drains the `writeback_jobs` outbox into legacy MSSQL.
//!
//! ## Lifecycle
//!
//! 1. Parse env (WRITEBACK_ENABLED, WRITEBACK_POLL_INTERVAL_SECS, WRITEBACK_MAX_ATTEMPTS).
//! 2. If disabled → log + exit cleanly.
//! 3. Open PG + MSSQL pools.
//! 4. Verify legacy schema fingerprint (refuse to start on drift).
//! 5. `LISTEN writeback_channel` + 30-sec poll fallback.
//! 6. For each pending job:
//!    a. PG TX: claim row by atomic UPDATE-RETURNING.
//!    b. Resolve legacy IDs from `public.ht_*` (UUID → R/CH/C numbers).
//!    c. MSSQL TX: dispatch to recipe.
//!    d. On success: COMMIT MSSQL, mark `done`, write `legacy_ids` back, COMMIT PG.
//!    e. On failure: ROLLBACK MSSQL, mark `failed`, increment `attempts`, COMMIT PG.
//! 7. SIGTERM → drain in-flight, then exit.
//!
//! ## Why a dedicated binary
//!
//! Per architecture §1: "API can crash / restart without losing pending
//! writebacks." The worker also has its own resource limits and can be turned
//! off at the deployment level (docker-compose `profiles: [legacy]`) without
//! touching the API container.
//!
//! ## What this binary does NOT do
//!
//! - Does not run any HTTP server.
//! - Does not run the SSE broadcaster (lives in `bin/hotel-backend`).
//! - Does not run the CT watcher (lives in future `bin/sync.rs`).
//! - Does not auto-fix schema drift — fail loud, alert ops, wait for human.

use std::env;
use std::sync::Arc;
use std::time::Duration;

use sqlx::postgres::{PgListener, PgPoolOptions};
use sqlx::{PgPool, Row};
use tokio::sync::Notify;
use uuid::Uuid;

use hotel_backend::config::DbConfig;
use hotel_backend::db::{create_pool, DbPool};
use hotel_backend::outbox::intent::WritebackIntent;
use hotel_backend::writeback::{
    dispatch, verify_schema_fingerprint, DispatchContext, ResolvedJob, WritebackError,
};

/// PG channel name the service layer NOTIFYs after enqueueing a writeback job.
/// Future migration will wire a trigger to fire this on every INSERT into
/// `writeback_jobs` so the worker doesn't depend on application-level NOTIFY.
const WRITEBACK_CHANNEL: &str = "writeback_channel";

/// Default poll interval (fallback when NOTIFY misses or the channel is silent).
const DEFAULT_POLL_INTERVAL_SECS: u64 = 30;

/// Default retry cap.
const DEFAULT_MAX_ATTEMPTS: i32 = 3;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "hotel_backend=info,writeback=info".into()),
        )
        .init();

    // 1. WRITEBACK_ENABLED — graceful no-op for State C
    let enabled = env::var("WRITEBACK_ENABLED")
        .ok()
        .map(|v| v != "false" && v != "0")
        .unwrap_or(true);
    if !enabled {
        tracing::warn!("WRITEBACK_ENABLED=false — worker exiting cleanly without polling");
        return Ok(());
    }

    let poll_interval = env::var("WRITEBACK_POLL_INTERVAL_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_POLL_INTERVAL_SECS);
    let max_attempts: i32 = env::var("WRITEBACK_MAX_ATTEMPTS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_MAX_ATTEMPTS);

    tracing::info!(
        poll_interval_secs = poll_interval,
        max_attempts,
        "Starting writeback worker"
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

    // 3. MSSQL pool — `create_pool` returns `Box<dyn Error>` (not Send+Sync) so
    // we map it to a string before bubbling up the typed error from `main`.
    let mssql_config = DbConfig::from_env();
    let mssql = create_pool(&mssql_config)
        .await
        .map_err(|e| format!("MSSQL pool init failed: {e}"))?;
    tracing::info!(server = %mssql_config.server, "Connected to legacy MSSQL");

    // 4. Schema fingerprint guard — refuse to start on drift
    verify_schema_fingerprint(&mssql).await.map_err(|e| {
        tracing::error!(error = %e, "Schema fingerprint check failed — refusing to start");
        format!("Schema fingerprint check failed: {e}")
    })?;

    // 5. NOTIFY listener + poll fallback
    let wakeup = Arc::new(Notify::new());
    let listener_wakeup = wakeup.clone();
    let pg_for_listener = pg.clone();
    let listener_handle = tokio::spawn(async move {
        if let Err(err) = run_listener(pg_for_listener, listener_wakeup).await {
            tracing::error!(error = %err, "PgListener task ended");
        }
    });

    // SIGTERM handling — drain in-flight then stop
    let shutdown = Arc::new(tokio::sync::Notify::new());
    let shutdown_clone = shutdown.clone();
    tokio::spawn(async move {
        let signal_kind = tokio::signal::unix::SignalKind::terminate();
        let mut sigterm: tokio::signal::unix::Signal =
            match tokio::signal::unix::signal(signal_kind) {
                Ok(s) => s,
                Err(err) => {
                    tracing::warn!(error = %err, "Could not register SIGTERM handler");
                    return;
                }
            };
        sigterm.recv().await;
        tracing::info!("SIGTERM received — draining pending jobs then exiting");
        shutdown_clone.notify_waiters();
    });

    // 6. Main loop — process jobs whenever NOTIFY wakes us OR every poll_interval
    loop {
        // Drain all pending jobs in this tick
        loop {
            match claim_next_job(&pg, max_attempts).await {
                Ok(Some(job)) => {
                    process_job(&pg, &mssql, job).await;
                }
                Ok(None) => break, // queue empty
                Err(err) => {
                    tracing::error!(error = %err, "claim_next_job failed; sleeping before retry");
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    break;
                }
            }
        }

        // Wait for either NOTIFY, poll tick, or shutdown
        tokio::select! {
            _ = wakeup.notified() => {
                tracing::trace!("wakeup from NOTIFY");
            }
            _ = tokio::time::sleep(Duration::from_secs(poll_interval)) => {
                tracing::trace!("poll-interval tick");
            }
            _ = shutdown.notified() => {
                tracing::info!("Shutdown signaled — exiting main loop");
                break;
            }
        }
    }

    listener_handle.abort();
    tracing::info!("Writeback worker exited cleanly");
    Ok(())
}

/// Claimed job — what we got from `writeback_jobs` after the atomic claim.
#[derive(Debug, Clone)]
struct ClaimedJob {
    id: i64,
    intent: WritebackIntent,
    aggregate_id: Uuid,
    attempts: i32,
}

/// Atomically claim the next pending (or retry-eligible failed) job.
///
/// Implemented as a single `UPDATE … RETURNING` so two worker instances can
/// race without producing duplicate processing.
async fn claim_next_job(
    pg: &PgPool,
    max_attempts: i32,
) -> Result<Option<ClaimedJob>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        UPDATE writeback_jobs
           SET status = 'in_progress',
               attempts = attempts + 1
         WHERE id = (
             SELECT id FROM writeback_jobs
              WHERE (status = 'pending')
                 OR (status = 'failed' AND attempts < $1)
              ORDER BY created_at
              FOR UPDATE SKIP LOCKED
              LIMIT 1
         )
        RETURNING id, intent, payload, aggregate_id, attempts
        "#,
    )
    .bind(max_attempts)
    .fetch_optional(pg)
    .await?;

    let Some(row) = row else { return Ok(None) };

    let id: i64 = row.try_get("id")?;
    let intent_name: String = row.try_get("intent")?;
    let payload: serde_json::Value = row.try_get("payload")?;
    let aggregate_id: Uuid = row.try_get("aggregate_id")?;
    let attempts: i32 = row.try_get("attempts")?;

    // Deserialize payload into the matching variant. The JSON shape is
    // produced by `serde(tag = "intent", content = "payload")` — the queue's
    // separate `intent` column is what the dispatcher uses, but the JSON
    // includes both for round-trip safety.
    let intent: WritebackIntent = serde_json::from_value(payload).map_err(|e| {
        sqlx::Error::Decode(
            format!("payload deserialize for job {id} ({intent_name}): {e}").into(),
        )
    })?;

    Ok(Some(ClaimedJob {
        id,
        intent,
        aggregate_id,
        attempts,
    }))
}

/// Process one claimed job: open MSSQL conn, dispatch, persist outcome.
async fn process_job(pg: &PgPool, mssql: &DbPool, job: ClaimedJob) {
    let job_id = job.id;
    let intent_name = job.intent.intent_name();
    tracing::info!(
        job_id,
        intent = intent_name,
        attempt = job.attempts,
        "Processing writeback job"
    );

    // Resolve legacy IDs from PG canonical tables.
    let resolved = match resolve_legacy_ids(pg, &job).await {
        Ok(r) => r,
        Err(err) => {
            tracing::error!(job_id, error = %err, "Failed to resolve legacy IDs");
            mark_failed(pg, job_id, &format!("resolve_legacy_ids: {err}")).await;
            return;
        }
    };

    // Acquire MSSQL connection. We do NOT open `BEGIN TRAN` at the
    // connection level — the recipes themselves must wrap their statements in
    // `BEGIN TRAN ... COMMIT` because tiberius doesn't expose a typed
    // transaction handle the way sqlx does. The `TABLOCKX, HOLDLOCK` in the
    // allocate helpers needs to live inside an explicit transaction; recipes
    // that allocate IDs already start one.
    //
    // For now we issue the statements outside an explicit transaction wrapper
    // — each recipe's allocate call holds the table-level lock for the
    // duration of the connection's batch. This matches the spike §6 verified
    // pattern (auto-commit per statement; the lock holds until the connection
    // returns to the pool).
    let mut conn = match mssql.get().await {
        Ok(c) => c,
        Err(err) => {
            tracing::error!(job_id, error = %err, "Failed to acquire MSSQL connection");
            mark_failed(pg, job_id, &format!("mssql_acquire: {err}")).await;
            return;
        }
    };

    let ctx = DispatchContext {
        job_id,
        aggregate_id: job.aggregate_id,
    };

    let dispatch_result = dispatch(&mut conn, &job.intent, &resolved, ctx).await;
    drop(conn); // release back to pool

    match dispatch_result {
        Ok(legacy_ids) => {
            tracing::info!(job_id, intent = intent_name, "Writeback succeeded");
            mark_done(pg, job_id, legacy_ids.into_json()).await;
        }
        Err(err) => {
            let retryable = err.is_retryable();
            tracing::error!(
                job_id,
                error = %err,
                retryable,
                "Writeback recipe failed"
            );
            mark_failed(pg, job_id, &err.to_string()).await;
        }
    }
}

/// Resolve PG aggregate UUIDs → legacy identifiers (R\d{6}, CH\d{2}-\d{6},
/// numeric `HT_Rooms.id`, etc.). Each intent has different requirements; we
/// best-effort-fetch every column the recipes might need.
///
/// Reads only — does not modify PG.
async fn resolve_legacy_ids(
    pg: &PgPool,
    job: &ClaimedJob,
) -> Result<ResolvedJob, sqlx::Error> {
    use WritebackIntent::*;
    let mut resolved = ResolvedJob::default();

    match &job.intent {
        ModifyBooking { booking_id, .. } | CancelBooking { booking_id } => {
            if let Some(row) = sqlx::query(
                "SELECT legacy_book_id FROM ht_bookings WHERE id = $1",
            )
            .bind(booking_id)
            .fetch_optional(pg)
            .await?
            {
                resolved.legacy_book_id = row.try_get("legacy_book_id").ok();
            }
        }
        CancelCheckIn { check_in_id, .. }
        | ExtendStay { check_in_id, .. }
        | CheckOut { check_in_id }
        | RecordPayment { check_in_id, .. } => {
            if let Some(row) = sqlx::query(
                "SELECT legacy_cin_no, legacy_room_no, legacy_cust_no, legacy_checkin_ds_id \
                 FROM ht_checkins WHERE id = $1",
            )
            .bind(check_in_id)
            .fetch_optional(pg)
            .await?
            {
                resolved.legacy_cin_no = row.try_get("legacy_cin_no").ok();
                resolved.legacy_room_no = row.try_get("legacy_room_no").ok();
                resolved.legacy_cust_no = row.try_get("legacy_cust_no").ok();
                resolved.legacy_checkin_ds_id = row.try_get("legacy_checkin_ds_id").ok();
            }
        }
        MarkRoomClean { room_id, .. } => {
            if let Some(row) = sqlx::query(
                "SELECT legacy_room_no, legacy_room_id_int \
                 FROM ht_rooms_new WHERE id = $1",
            )
            .bind(room_id)
            .fetch_optional(pg)
            .await?
            {
                resolved.legacy_room_no = row.try_get("legacy_room_no").ok();
                resolved.legacy_room_id_int = row.try_get("legacy_room_id_int").ok();
            }
        }
        // CreateBooking / CreateCheckIn don't need PG-side IDs — the payload
        // carries the canonical data and the recipe allocates legacy IDs
        // itself with TABLOCKX.
        CreateBooking { .. } | CreateCheckIn { .. } => {}
    }
    Ok(resolved)
}

/// Mark the job done + persist allocated legacy IDs.
async fn mark_done(pg: &PgPool, job_id: i64, legacy_ids: serde_json::Value) {
    let res = sqlx::query(
        "UPDATE writeback_jobs SET status='done', completed_at=NOW(), legacy_ids=$2 \
         WHERE id=$1",
    )
    .bind(job_id)
    .bind(legacy_ids)
    .execute(pg)
    .await;
    if let Err(err) = res {
        tracing::error!(job_id, error = %err, "Failed to mark job done");
    }
}

/// Mark the job failed (will retry until attempts >= max_attempts).
async fn mark_failed(pg: &PgPool, job_id: i64, err_msg: &str) {
    let res = sqlx::query(
        "UPDATE writeback_jobs SET status='failed', last_error=$2 WHERE id=$1",
    )
    .bind(job_id)
    .bind(err_msg)
    .execute(pg)
    .await;
    if let Err(err) = res {
        tracing::error!(job_id, error = %err, "Failed to mark job failed");
    }
}

/// Long-lived PG LISTEN connection; signals the main loop on every NOTIFY.
async fn run_listener(pg: PgPool, wakeup: Arc<Notify>) -> Result<(), sqlx::Error> {
    let mut listener = PgListener::connect_with(&pg).await?;
    listener.listen(WRITEBACK_CHANNEL).await?;
    tracing::info!(channel = WRITEBACK_CHANNEL, "PgListener subscribed");
    loop {
        match listener.recv().await {
            Ok(_notification) => {
                tracing::trace!("NOTIFY received — waking main loop");
                wakeup.notify_one();
            }
            Err(err) => {
                tracing::warn!(error = %err, "PgListener recv() error; exiting listener task");
                return Err(err);
            }
        }
    }
}

// Suppress unused import warning when WritebackError isn't directly referenced
// in error returns. (`is_retryable` covers the visible path.)
#[allow(dead_code)]
fn _suppress_unused_writeback_error_import(_: WritebackError) {}
