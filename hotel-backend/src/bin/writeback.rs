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

use hotel_backend::config::{DbConfig, SlackConfig};
use hotel_backend::db::{create_pool, DbPool};
use hotel_backend::notifications::slack::{SlackClient, SlackMessage};
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

/// How long an `in_progress` claim is allowed to live before another worker
/// can re-claim it. Covers worker crashes mid-recipe and `mark_done` /
/// `mark_failed` PG failures. Set conservatively at 5 minutes — longer than
/// any realistic recipe execution but short enough that a stuck job recovers
/// within one operator coffee break.
const STUCK_IN_PROGRESS_TIMEOUT_SECS: i64 = 300;

/// Exponential backoff (in seconds) between retry attempts. Indexed by
/// `attempts` (0-based: backoff_secs(1) is the wait before attempt #2).
/// Caps at the last entry. Default schedule: 30s, 2min, 10min.
fn backoff_secs(attempts_so_far: i32) -> i64 {
    const BACKOFFS: &[i64] = &[30, 120, 600];
    let idx = (attempts_so_far as usize).saturating_sub(1).min(BACKOFFS.len() - 1);
    BACKOFFS[idx]
}

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

    // 4a. Slack notifier — best-effort alerts on schema drift + retry
    //     exhaustion. None = SLACK_WEBHOOK_URL not configured / disabled.
    let slack_config = SlackConfig::from_env();
    let slack: Option<SlackClient> = if slack_config.is_configured() {
        tracing::info!("Slack notifications enabled for writeback worker");
        Some(SlackClient::new(slack_config))
    } else {
        tracing::warn!(
            "Slack notifications NOT configured (set SLACK_WEBHOOK_URL); \
             retry-exhausted jobs will only surface in logs"
        );
        None
    };

    // 4b. Schema fingerprint guard — refuse to start on drift, but post
    //     a Slack alert first so the operator sees the failure even if
    //     they're not tailing logs. Sleep before returning so the Docker
    //     `restart: unless-stopped` policy backs off (without the sleep,
    //     the worker exits in ms, restarts, fingerprint fails again, fires
    //     another Slack — operator gets paged 6×/min until they intervene).
    if let Err(e) = verify_schema_fingerprint(&mssql).await {
        tracing::error!(error = %e, "Schema fingerprint check failed — refusing to start");
        if let Some(slack) = &slack {
            let msg = SlackMessage::with_text(format!(
                ":warning: *Writeback worker REFUSED TO START* :warning:\n\
                 Legacy MSSQL schema fingerprint mismatch.\n\
                 *Error:* `{e}`\n\
                 _The legacy DB columns drifted from the captured baseline. \
                 Run_ `./scripts/writeback-fingerprint.sh` _and follow the \
                 README to update the baseline before restarting the worker._"
            ));
            let _ = slack.send_message(&msg).await;
        }
        tracing::warn!(
            "Sleeping 60s before exit to throttle Docker restart cadence \
             and avoid Slack alert flood"
        );
        tokio::time::sleep(Duration::from_secs(60)).await;
        return Err(format!("Schema fingerprint check failed: {e}").into());
    }

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
                    // HIGH-3 fix: panic-isolate every job. A panic inside any
                    // recipe (or a tiberius driver bug) would otherwise crash
                    // the whole worker process. Docker would restart it but
                    // the in-flight job would sit `in_progress` for 5 min
                    // before the janitor re-claims — invisible to operator
                    // until the alert fires elsewhere. With panic isolation,
                    // a single bad job is marked exhausted with the panic
                    // message and the worker keeps draining the queue.
                    let job_id = job.id;
                    let intent_name = job.intent.intent_name();
                    let attempts = job.attempts;
                    let pg_inner = pg.clone();
                    let mssql_inner = mssql.clone();
                    let slack_inner = slack.clone();
                    let result = tokio::spawn(async move {
                        process_job(&pg_inner, &mssql_inner, max_attempts, &slack_inner, job)
                            .await;
                    })
                    .await;
                    if let Err(join_err) = result {
                        let panic_msg = if join_err.is_panic() {
                            // Try to recover the panic payload as a string —
                            // tokio gives us Box<dyn Any + Send>; common
                            // payload types are &'static str and String.
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
                            job_id,
                            intent = intent_name,
                            attempt = attempts,
                            panic = %panic_msg,
                            "Writeback job PANICKED — marking exhausted and continuing main loop"
                        );
                        // Force-exhaust the job so it doesn't sit stuck. The
                        // panic is unrecoverable for this payload — retrying
                        // wouldn't help. Slack alert fires inside mark_failed.
                        force_exhaust_job(
                            &pg,
                            job_id,
                            &slack,
                            &format!("PANIC: {panic_msg}"),
                        )
                        .await;
                    }
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

/// Atomically claim the next pending / retry-eligible / stuck job.
///
/// Implemented as a single `UPDATE … RETURNING` so two worker instances can
/// race without producing duplicate processing. The selection covers three
/// retry-eligible cases:
///
/// 1. **`pending`** — never tried.
/// 2. **`failed`** — temporary failure; only re-claimable once
///    `next_retry_at <= NOW()` (exponential backoff set by `mark_failed`).
/// 3. **`in_progress` with stale `claimed_at`** — recovery from a worker
///    crash mid-recipe or a `mark_done`/`mark_failed` PG failure that left
///    the row stuck. Stale = claimed > `STUCK_IN_PROGRESS_TIMEOUT_SECS` ago.
///    Without this clause, stuck jobs would require manual SQL intervention.
///
/// `exhausted` rows are never re-claimed — they require operator triage and
/// a manual status reset. The Slack alert sent at the moment of exhaustion
/// is the operator's notification path.
async fn claim_next_job(
    pg: &PgPool,
    max_attempts: i32,
) -> Result<Option<ClaimedJob>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        UPDATE writeback_jobs
           SET status     = 'in_progress',
               attempts   = attempts + 1,
               claimed_at = NOW()
         WHERE id = (
             SELECT id FROM writeback_jobs
              WHERE (status = 'pending')
                 OR (status = 'failed'
                     AND attempts < $1
                     AND (next_retry_at IS NULL OR next_retry_at <= NOW()))
                 OR (status = 'in_progress'
                     AND attempts < $1
                     AND claimed_at IS NOT NULL
                     AND claimed_at < NOW() - make_interval(secs => $2))
              ORDER BY created_at
              FOR UPDATE SKIP LOCKED
              LIMIT 1
         )
        RETURNING id, intent, payload, aggregate_id, attempts
        "#,
    )
    .bind(max_attempts)
    .bind(STUCK_IN_PROGRESS_TIMEOUT_SECS)
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
async fn process_job(
    pg: &PgPool,
    mssql: &DbPool,
    max_attempts: i32,
    slack: &Option<SlackClient>,
    job: ClaimedJob,
) {
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
            mark_failed(pg, job_id, max_attempts, slack, &format!("resolve_legacy_ids: {err}")).await;
            return;
        }
    };

    // Acquire MSSQL connection and wrap the entire recipe in an explicit
    // transaction. This is mandatory: the recipes' `TABLOCKX, HOLDLOCK` MAX+1
    // ID allocation pattern only holds the table lock for the life of the
    // surrounding transaction. In autocommit mode (no `BEGIN TRAN`), MSSQL
    // releases the lock at the end of each statement — so the SELECT that
    // computes `MAX(...)+1` releases the lock before the corresponding INSERT
    // runs, and a concurrent worker / .NET client can read the same value.
    // That voids the spike §6 race-safety guarantee.
    //
    // The transaction also gives us atomic rollback: a recipe failure mid-batch
    // (e.g. statement 4 of 8 errors) un-does statements 1-3 instead of leaving
    // a half-applied booking in MSSQL.
    let mut conn = match mssql.get().await {
        Ok(c) => c,
        Err(err) => {
            tracing::error!(job_id, error = %err, "Failed to acquire MSSQL connection");
            mark_failed(pg, job_id, max_attempts, slack, &format!("mssql_acquire: {err}")).await;
            return;
        }
    };

    let ctx = DispatchContext {
        job_id,
        aggregate_id: job.aggregate_id,
    };

    let outcome = run_in_transaction(&mut conn, &job.intent, &resolved, ctx).await;
    drop(conn); // release back to pool before any further awaits

    match outcome {
        Ok(legacy_ids) => {
            tracing::info!(job_id, intent = intent_name, "Writeback succeeded");
            mark_done(
                pg,
                job_id,
                job.aggregate_id,
                &job.intent,
                legacy_ids.into_json(),
            )
            .await;
        }
        Err(err) => {
            let retryable = err.is_retryable();
            tracing::error!(
                job_id,
                error = %err,
                retryable,
                "Writeback recipe failed"
            );
            mark_failed_with_retryable(
                pg,
                job_id,
                max_attempts,
                slack,
                &err.to_string(),
                retryable,
            )
            .await;
        }
    }
}

/// Wrap `dispatch` in an explicit MSSQL transaction.
///
/// **Why this is mandatory:** the recipes' `TABLOCKX, HOLDLOCK` MAX+1 ID
/// allocation pattern only holds the table lock for the life of the
/// surrounding transaction. In autocommit mode (no `BEGIN TRAN`), MSSQL
/// releases the lock at the end of each statement — so the SELECT that
/// computes `MAX(...)+1` releases its lock before the corresponding INSERT
/// runs, and a concurrent writeback worker / .NET client can read the same
/// value. That voids the spike §6 race-safety guarantee.
///
/// The transaction also gives us atomic rollback: a recipe failure mid-batch
/// (e.g. statement 4 of 8 errors) un-does statements 1-3 instead of leaving
/// a half-applied booking in MSSQL.
async fn run_in_transaction(
    conn: &mut hotel_backend::writeback::allocate::LegacyConn<'_>,
    intent: &WritebackIntent,
    resolved: &ResolvedJob,
    ctx: DispatchContext,
) -> Result<hotel_backend::writeback::LegacyIds, WritebackError> {
    {
        let begin = conn.simple_query("BEGIN TRAN").await?;
        drop(begin);
    }

    let dispatch_result = dispatch(conn, intent, resolved, ctx).await;

    match dispatch_result {
        Ok(legacy_ids) => {
            let commit = conn.simple_query("COMMIT TRAN").await?;
            drop(commit);
            Ok(legacy_ids)
        }
        Err(err) => {
            // Best-effort rollback. If ROLLBACK itself fails, the connection
            // is poisoned and bb8 will discard it on next acquire — the data
            // remains safe because nothing was committed.
            match conn.simple_query("ROLLBACK TRAN").await {
                Ok(stream) => drop(stream),
                Err(rb_err) => tracing::warn!(
                    rollback_error = %rb_err,
                    "ROLLBACK TRAN failed — connection will be dropped from pool"
                ),
            }
            Err(err)
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

    // The intent's `*_id` field carries the deterministic UUID derived from
    // PG's SERIAL via `service::ids::aggregate_uuid` — the canonical row was
    // stamped with the same UUID at insert time (migration 014). So we
    // resolve the legacy_* fields by joining on `aggregate_id`, not on the
    // SERIAL primary key.
    //
    // Resolution is two-layered:
    //   1. **Cache** — the canonical `ht_*` row has `legacy_*` columns
    //      back-populated by `mark_done` after each successful writeback.
    //   2. **Self-heal** — if the cache says NULL (e.g. back-population
    //      previously failed due to a transient PG hiccup), fall back to
    //      `writeback_jobs.legacy_ids` JSONB, which is the **source of
    //      truth** (it's what the recipe returned, persisted atomically with
    //      the `status='done'` write). This makes the system recover from
    //      back-population failures without operator intervention.
    match &job.intent {
        ModifyBooking { booking_id, .. } | CancelBooking { booking_id } => {
            if let Some(row) = sqlx::query(
                "SELECT legacy_book_id FROM ht_bookings WHERE aggregate_id = $1",
            )
            .bind(booking_id)
            .fetch_optional(pg)
            .await?
            {
                resolved.legacy_book_id = row.try_get("legacy_book_id").ok();
            }
            // Self-heal from writeback_jobs audit log if cache missed.
            if resolved.legacy_book_id.is_none() {
                let salvaged = salvage_legacy_ids(pg, *booking_id).await?;
                resolved.legacy_book_id = salvaged.book_id;
                if resolved.legacy_cust_no.is_none() {
                    resolved.legacy_cust_no = salvaged.cust_no;
                }
            }
        }
        CancelCheckIn { check_in_id, .. }
        | ExtendStay { check_in_id, .. }
        | CheckOut { check_in_id }
        | RecordPayment { check_in_id, .. } => {
            if let Some(row) = sqlx::query(
                "SELECT legacy_cin_no, legacy_room_no, legacy_cust_no, legacy_checkin_ds_id \
                 FROM ht_checkins WHERE aggregate_id = $1",
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
            // Self-heal from writeback_jobs audit log per missing field.
            if resolved.legacy_cin_no.is_none()
                || resolved.legacy_room_no.is_none()
                || resolved.legacy_cust_no.is_none()
                || resolved.legacy_checkin_ds_id.is_none()
            {
                let salvaged = salvage_legacy_ids(pg, *check_in_id).await?;
                resolved.legacy_cin_no = resolved.legacy_cin_no.or(salvaged.cin_no);
                resolved.legacy_room_no = resolved.legacy_room_no.or(salvaged.room_no);
                resolved.legacy_cust_no = resolved.legacy_cust_no.or(salvaged.cust_no);
                resolved.legacy_checkin_ds_id =
                    resolved.legacy_checkin_ds_id.or(salvaged.checkin_ds_id);
            }
        }
        MarkRoomClean { room_id, .. } => {
            if let Some(row) = sqlx::query(
                "SELECT legacy_room_no, legacy_room_id_int \
                 FROM ht_rooms_new WHERE aggregate_id = $1",
            )
            .bind(room_id)
            .fetch_optional(pg)
            .await?
            {
                resolved.legacy_room_no = row.try_get("legacy_room_no").ok();
                resolved.legacy_room_id_int = row.try_get("legacy_room_id_int").ok();
            }
            // No self-heal source for rooms — they're populated by the
            // backfill_rooms binary, not by recipe writebacks.
        }
        // CreateBooking carries everything in its payload — no resolution.
        CreateBooking { .. } => {}
        // CreateCheckIn (linked-to-booking variant) needs the linked
        // booking's legacy_book_id. Normally the payload supplies it, but
        // when the booking was created via UI moments earlier and its
        // CreateBooking writeback hasn't completed yet (back-population
        // race or partial failure), the payload's `linked_legacy_book_id`
        // is None and the dispatcher must fall back to a PG lookup. Walkin
        // (linked_booking_id=None) needs nothing here.
        CreateCheckIn { payload, .. } => {
            if let (Some(linked_booking_id), None) = (
                payload.linked_booking_id,
                payload.linked_legacy_book_id.as_deref(),
            ) {
                if let Some(row) = sqlx::query(
                    "SELECT legacy_book_id FROM ht_bookings WHERE aggregate_id = $1",
                )
                .bind(linked_booking_id)
                .fetch_optional(pg)
                .await?
                {
                    resolved.legacy_book_id = row.try_get("legacy_book_id").ok();
                }
                // Self-heal from writeback_jobs audit log if cache missed.
                if resolved.legacy_book_id.is_none() {
                    let salvaged = salvage_legacy_ids(pg, linked_booking_id).await?;
                    resolved.legacy_book_id = salvaged.book_id;
                }
            }
        }
    }
    Ok(resolved)
}

/// Identifiers salvaged from a prior successful writeback's
/// `writeback_jobs.legacy_ids` JSONB. Used by the resolver as a self-healing
/// fallback when the canonical `ht_*.legacy_*` cache columns are NULL —
/// typically because `mark_done`'s back-population step failed transiently.
#[derive(Default, Debug)]
struct SalvagedLegacyIds {
    book_id: Option<String>,
    cust_no: Option<String>,
    cin_no: Option<String>,
    room_no: Option<String>,
    checkin_ds_id: Option<i32>,
}

/// Pull the most recently successful writeback's allocated legacy IDs for
/// `aggregate_id` out of the audit log. Tolerant of missing fields and
/// missing rows — every field is `Option`.
///
/// Why this is safe to use as a fallback: `writeback_jobs.legacy_ids` is
/// written in the same `mark_done` UPDATE that flips `status='done'`, so a
/// row with `status='done'` and a non-NULL `legacy_ids` is a strict superset
/// of what a successful back-population would have written to `ht_*`. If
/// back-population fails, this audit row is the source of truth.
async fn salvage_legacy_ids(
    pg: &PgPool,
    aggregate_id: Uuid,
) -> Result<SalvagedLegacyIds, sqlx::Error> {
    let row = sqlx::query(
        "SELECT legacy_ids FROM writeback_jobs \
         WHERE aggregate_id = $1 AND status = 'done' AND legacy_ids IS NOT NULL \
         ORDER BY completed_at DESC NULLS LAST LIMIT 1",
    )
    .bind(aggregate_id)
    .fetch_optional(pg)
    .await?;

    let mut out = SalvagedLegacyIds::default();
    if let Some(row) = row {
        let legacy: Option<serde_json::Value> = row.try_get("legacy_ids").ok();
        if let Some(json) = legacy {
            out.book_id = json.get("book_id").and_then(|v| v.as_str()).map(String::from);
            out.cust_no = json.get("cust_no").and_then(|v| v.as_str()).map(String::from);
            out.cin_no = json.get("cin_no").and_then(|v| v.as_str()).map(String::from);
            out.room_no = json.get("room_no").and_then(|v| v.as_str()).map(String::from);
            out.checkin_ds_id = json
                .get("checkin_ds_id")
                .and_then(|v| v.as_i64())
                .map(|n| n as i32);
            tracing::warn!(
                %aggregate_id,
                "Self-healed missing legacy_* from writeback_jobs audit log; \
                 ht_* cache row likely needs re-stamping"
            );
        }
    }
    Ok(out)
}

/// Mark the job done, persist allocated legacy IDs into the writeback_jobs
/// audit row, AND back-populate the canonical PG row's `legacy_*` columns so
/// subsequent intents on the same aggregate can resolve immediately.
///
/// The back-population is essential for the second-and-later writebacks on
/// the same aggregate. Example:
///
///   1. CreateBooking → allocates `R014812` in MSSQL → mark_done writes
///      `legacy_book_id='R014812'` into ht_bookings.
///   2. ModifyBooking on same aggregate_id → resolver finds 'R014812' and
///      passes it to the recipe.
///
/// Without step 1, step 2 fails with "ModifyBooking requires resolved
/// legacy_book_id".
async fn mark_done(
    pg: &PgPool,
    job_id: i64,
    aggregate_id: Uuid,
    intent: &WritebackIntent,
    legacy_ids: serde_json::Value,
) {
    let res = sqlx::query(
        "UPDATE writeback_jobs SET status='done', completed_at=NOW(), legacy_ids=$2 \
         WHERE id=$1",
    )
    .bind(job_id)
    .bind(&legacy_ids)
    .execute(pg)
    .await;
    if let Err(err) = res {
        tracing::error!(job_id, error = %err, "Failed to mark job done");
    }

    // Bounded retry — three attempts with exponential backoff. If all fail,
    // the resolver's self-heal path (`salvage_legacy_ids`) recovers from the
    // writeback_jobs audit row at the next intent. So the worst case is one
    // extra SELECT per future resolution, not data loss.
    let mut delay_ms = 100u64;
    let mut last_err: Option<sqlx::Error> = None;
    for attempt in 1..=3 {
        match back_populate_legacy_ids(pg, aggregate_id, intent, &legacy_ids).await {
            Ok(()) => {
                last_err = None;
                break;
            }
            Err(err) => {
                tracing::warn!(
                    job_id,
                    %aggregate_id,
                    attempt,
                    error = %err,
                    "Back-population attempt failed; will retry"
                );
                last_err = Some(err);
                tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                delay_ms = delay_ms.saturating_mul(4);
            }
        }
    }
    if let Some(err) = last_err {
        // All retries exhausted. Resolver's self-heal will pick up the
        // legacy_ids from this writeback_jobs row at the next intent — so
        // no data loss, just a slower lookup.
        tracing::error!(
            job_id,
            %aggregate_id,
            error = %err,
            "Back-population failed after retries; resolver will self-heal \
             from writeback_jobs.legacy_ids at next intent"
        );
    }
}

/// Write the recipe's allocated legacy identifiers (book_id, cin_no, etc.)
/// back to the canonical PG row. Used after a successful writeback so the
/// next intent on the same aggregate can resolve immediately. Quietly skips
/// fields the intent variant doesn't produce.
async fn back_populate_legacy_ids(
    pg: &PgPool,
    aggregate_id: Uuid,
    intent: &WritebackIntent,
    legacy_ids: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    let book_id = legacy_ids.get("book_id").and_then(|v| v.as_str());
    let cust_no = legacy_ids.get("cust_no").and_then(|v| v.as_str());
    let cin_no = legacy_ids.get("cin_no").and_then(|v| v.as_str());
    let room_no = legacy_ids.get("room_no").and_then(|v| v.as_str());
    let checkin_ds_id = legacy_ids.get("checkin_ds_id").and_then(|v| v.as_i64()).map(|n| n as i32);

    use WritebackIntent::*;
    match intent {
        CreateBooking { .. } | ModifyBooking { .. } | CancelBooking { .. } => {
            if book_id.is_some() || cust_no.is_some() {
                sqlx::query(
                    "UPDATE ht_bookings SET \
                       legacy_book_id = COALESCE($2, legacy_book_id), \
                       legacy_cust_no = COALESCE($3, legacy_cust_no), \
                       updated_at = NOW() \
                     WHERE aggregate_id = $1",
                )
                .bind(aggregate_id)
                .bind(book_id)
                .bind(cust_no)
                .execute(pg)
                .await?;
            }
        }
        CreateCheckIn { .. } | CancelCheckIn { .. } | ExtendStay { .. } | CheckOut { .. } | RecordPayment { .. } => {
            if cin_no.is_some() || room_no.is_some() || cust_no.is_some() || checkin_ds_id.is_some() {
                sqlx::query(
                    "UPDATE ht_checkins SET \
                       legacy_cin_no        = COALESCE($2, legacy_cin_no), \
                       legacy_room_no       = COALESCE($3, legacy_room_no), \
                       legacy_cust_no       = COALESCE($4, legacy_cust_no), \
                       legacy_checkin_ds_id = COALESCE($5, legacy_checkin_ds_id), \
                       updated_at = NOW() \
                     WHERE aggregate_id = $1",
                )
                .bind(aggregate_id)
                .bind(cin_no)
                .bind(room_no)
                .bind(cust_no)
                .bind(checkin_ds_id)
                .execute(pg)
                .await?;
            }
        }
        MarkRoomClean { .. } => {
            // mark_clean doesn't allocate any new legacy IDs.
        }
    }
    Ok(())
}

/// Force a job into the terminal `exhausted` state, regardless of attempt
/// count. Used for panics (no point retrying — the recipe code is broken or
/// the payload is unparseable) and other deterministic failures where the
/// retry budget would just delay operator visibility.
async fn force_exhaust_job(
    pg: &PgPool,
    job_id: i64,
    slack: &Option<SlackClient>,
    err_msg: &str,
) {
    let row = sqlx::query(
        "UPDATE writeback_jobs SET status='exhausted', last_error=$2, next_retry_at=NULL \
         WHERE id=$1 RETURNING attempts, intent, aggregate_id",
    )
    .bind(job_id)
    .bind(err_msg)
    .fetch_one(pg)
    .await;

    match row {
        Ok(row) => {
            let attempts: i32 = row.try_get("attempts").unwrap_or(0);
            let intent: String = row.try_get("intent").unwrap_or_default();
            let aggregate_id: Option<Uuid> = row.try_get("aggregate_id").ok();
            tracing::error!(
                job_id, attempts, intent = %intent, ?aggregate_id,
                "Writeback job force-exhausted"
            );
            if let Some(slack) = slack {
                send_exhausted_alert(slack, job_id, &intent, aggregate_id, attempts, err_msg)
                    .await;
            }
        }
        Err(err) => {
            tracing::error!(
                job_id, error = %err,
                "Failed to force-exhaust job — janitor will reset it in {STUCK_IN_PROGRESS_TIMEOUT_SECS}s"
            );
        }
    }
}

/// Mark the job failed. If the underlying error is `retryable=false`
/// (deterministic — schema drift, intent mismatch, recipe business-rule
/// failure, payload deserialize error), skip the retry budget entirely and
/// go straight to `exhausted` so the operator gets a Slack alert immediately
/// instead of waiting for 12 minutes of wasted retries on the same failure
/// (audit HIGH-2).
///
/// Otherwise, schedule a retry (`failed` + `next_retry_at` set via
/// exponential backoff) or transition to `exhausted` once `attempts >=
/// max_attempts`. Fires a Slack alert on the exhaustion transition so the
/// operator sees the failure within seconds, not whenever they next look at
/// the queue.
async fn mark_failed_with_retryable(
    pg: &PgPool,
    job_id: i64,
    max_attempts: i32,
    slack: &Option<SlackClient>,
    err_msg: &str,
    retryable: bool,
) {
    if !retryable {
        force_exhaust_job(pg, job_id, slack, err_msg).await;
        return;
    }
    mark_failed(pg, job_id, max_attempts, slack, err_msg).await;
}

/// See [`mark_failed_with_retryable`]. Convenience wrapper for callsites
/// that don't have a typed error in hand (e.g. PG resolve failures).
/// Defaults to retryable=true.
async fn mark_failed(
    pg: &PgPool,
    job_id: i64,
    max_attempts: i32,
    slack: &Option<SlackClient>,
    err_msg: &str,
) {
    // Bump status conditionally. RETURNING gives us the post-UPDATE state so
    // we know whether this attempt exhausted the budget.
    let row = sqlx::query(
        r#"
        UPDATE writeback_jobs
           SET status        = CASE WHEN attempts >= $2 THEN 'exhausted' ELSE 'failed' END,
               last_error    = $3,
               next_retry_at = CASE
                                   WHEN attempts >= $2 THEN NULL
                                   ELSE NOW() + make_interval(secs => $4)
                               END
         WHERE id = $1
        RETURNING attempts, status, intent, aggregate_id
        "#,
    )
    .bind(job_id)
    .bind(max_attempts)
    .bind(err_msg)
    .bind(backoff_secs(get_attempts_for_backoff(pg, job_id).await))
    .fetch_one(pg)
    .await;

    let row = match row {
        Ok(r) => r,
        Err(err) => {
            tracing::error!(
                job_id,
                error = %err,
                "Failed to mark job failed — job will be picked up by stuck-in-progress janitor in {STUCK_IN_PROGRESS_TIMEOUT_SECS}s"
            );
            return;
        }
    };

    let attempts: i32 = row.try_get("attempts").unwrap_or(0);
    let status: String = row.try_get("status").unwrap_or_default();
    let intent: String = row.try_get("intent").unwrap_or_default();
    let aggregate_id: Option<Uuid> = row.try_get("aggregate_id").ok();

    if status == "exhausted" {
        tracing::error!(
            job_id,
            attempts,
            intent = %intent,
            ?aggregate_id,
            "Writeback job EXHAUSTED retries — manual intervention required"
        );
        if let Some(slack) = slack {
            send_exhausted_alert(slack, job_id, &intent, aggregate_id, attempts, err_msg).await;
        }
    } else {
        tracing::warn!(
            job_id,
            attempts,
            "Writeback job failed; will retry after backoff"
        );
    }
}

/// Read just `attempts` so we can compute the right backoff. Two queries
/// instead of one (we already issued the UPDATE) trades a tiny extra round
/// trip for keeping the UPDATE statement readable; could be folded in via a
/// CTE if it ever shows up in a profile.
async fn get_attempts_for_backoff(pg: &PgPool, job_id: i64) -> i32 {
    sqlx::query_scalar::<_, i32>("SELECT attempts FROM writeback_jobs WHERE id = $1")
        .bind(job_id)
        .fetch_one(pg)
        .await
        .unwrap_or(1)
}

/// Post a Slack alert when a writeback job exhausts its retry budget.
/// Best-effort — Slack failures are logged inside `send_message` but never
/// propagated. Avoids blocking the writeback main loop on Slack timeouts.
async fn send_exhausted_alert(
    slack: &SlackClient,
    job_id: i64,
    intent: &str,
    aggregate_id: Option<Uuid>,
    attempts: i32,
    err_msg: &str,
) {
    let aggregate_id_str = aggregate_id
        .map(|u| u.to_string())
        .unwrap_or_else(|| "(unknown)".into());
    // Head+tail truncation — tiberius/sqlx errors put the actually-useful
    // row context at the END (e.g. "in row 23, column foo: <value>"). A
    // pure head truncation would lose it. Slice on character boundaries
    // (Thai messages are multi-byte) by walking with `char_indices`.
    let truncated_err = truncate_head_tail(err_msg, 200, 300);
    let text = format!(
        ":rotating_light: *Writeback EXHAUSTED retries* :rotating_light:\n\
         *Job ID:* `{job_id}`\n\
         *Intent:* `{intent}`\n\
         *Aggregate:* `{aggregate_id_str}`\n\
         *Attempts:* {attempts}\n\
         *Last error:*\n```\n{truncated_err}\n```\n\
         _Manual intervention required. Inspect_ \
         `SELECT * FROM writeback_jobs WHERE id = {job_id}` _and either fix \
         the underlying cause + manually reset_ \
         `UPDATE writeback_jobs SET status='pending', attempts=0, \
         next_retry_at=NULL WHERE id={job_id}` _to retry, \
         or delete the row if the writeback is no longer needed._"
    );
    let msg = SlackMessage::with_text(text);
    let _ = slack.send_message(&msg).await;
}

/// Truncate `s` to at most `head_chars + tail_chars` chars, keeping the
/// head and tail with `…` between them. Walks `char_indices` so multi-byte
/// (Thai, Chinese) text never splits in the middle of a code point.
fn truncate_head_tail(s: &str, head_chars: usize, tail_chars: usize) -> String {
    let total = s.chars().count();
    if total <= head_chars + tail_chars {
        return s.to_string();
    }
    let mut chars = s.char_indices();
    let head_end = chars.nth(head_chars).map(|(i, _)| i).unwrap_or(s.len());
    let tail_start = s
        .char_indices()
        .rev()
        .nth(tail_chars.saturating_sub(1))
        .map(|(i, _)| i)
        .unwrap_or(s.len());
    format!("{}…{}", &s[..head_end], &s[tail_start..])
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Backoff schedule: 30s, 2min, 10min — matches the constants and
    /// guards against an accidental edit that could collapse the schedule
    /// (e.g. all 0s would re-enable the previous thrashing behavior).
    #[test]
    fn backoff_schedule_matches_documented_values() {
        assert_eq!(backoff_secs(1), 30);
        assert_eq!(backoff_secs(2), 120);
        assert_eq!(backoff_secs(3), 600);
    }

    /// The function is called with `attempts` from the post-update row.
    /// `attempts == 0` is the pre-claim state — should never reach the
    /// backoff path, but if it does, must not panic and must produce a
    /// non-zero wait.
    #[test]
    fn backoff_for_zero_attempts_does_not_panic() {
        let n = backoff_secs(0);
        assert!(n > 0, "backoff_secs(0) should be safe and non-zero, got {n}");
    }

    /// Anything past the schedule (would only happen if max_attempts is
    /// raised mid-flight) caps at the longest backoff. Prevents an
    /// off-by-one panic.
    #[test]
    fn backoff_caps_at_longest_for_overflow() {
        assert_eq!(backoff_secs(10), 600);
        assert_eq!(backoff_secs(i32::MAX), 600);
    }

    /// Stuck-in-progress timeout must be longer than the longest realistic
    /// recipe execution but shorter than an operator coffee break — guards
    /// against accidentally setting it to a value that would let stuck
    /// jobs sit for hours, OR letting a slow recipe get re-claimed mid-run.
    #[test]
    fn stuck_in_progress_timeout_is_in_safe_range() {
        assert!(STUCK_IN_PROGRESS_TIMEOUT_SECS >= 60, "less than 1 min risks racing slow recipes");
        assert!(STUCK_IN_PROGRESS_TIMEOUT_SECS <= 1800, "more than 30 min is too slow to recover");
    }

    /// Short error messages pass through unchanged.
    #[test]
    fn truncate_head_tail_preserves_short_strings() {
        assert_eq!(truncate_head_tail("hello", 200, 300), "hello");
        assert_eq!(truncate_head_tail("", 200, 300), "");
    }

    /// Long messages keep both ends — important for tiberius errors that
    /// put row context at the end.
    #[test]
    fn truncate_head_tail_keeps_both_ends_for_long_strings() {
        let s = "A".repeat(200) + &"B".repeat(500) + &"C".repeat(300);
        let out = truncate_head_tail(&s, 200, 300);
        assert!(out.starts_with(&"A".repeat(200)), "head missing");
        assert!(out.ends_with(&"C".repeat(300)), "tail missing");
        assert!(out.contains('…'), "ellipsis missing");
    }

    /// Multi-byte (Thai) text must not split inside a code point — would
    /// panic in the underlying str slice if the boundary is wrong.
    #[test]
    fn truncate_head_tail_safe_on_thai_multibyte() {
        let thai = "เข้าพัก".repeat(100); // each char is 3 bytes
        let out = truncate_head_tail(&thai, 5, 5);
        // No panic = pass. Sanity: result must be a valid String shorter
        // than the input.
        assert!(out.chars().count() < thai.chars().count());
    }
}
