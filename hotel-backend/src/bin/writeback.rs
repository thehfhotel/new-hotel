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
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
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

/// Self-heal alert threshold (audit MED-4). When `salvage_legacy_ids` recovers
/// IDs from the writeback_jobs audit log this many times within
/// `SELF_HEAL_WINDOW_SECS`, fire a single Slack alert. The threshold is
/// deliberately above zero so a one-off race (e.g. CreateBooking + immediate
/// CheckIn before back-population finishes) doesn't page the operator. A
/// sustained burst means back-population is broken (PG perms regression,
/// migration drift, etc.) and needs investigation.
const SELF_HEAL_ALERT_THRESHOLD: u32 = 5;

/// Self-heal alert window (audit MED-4). Sized to be longer than the
/// "expected" salvages (a handful per hour from CreateBooking↔CheckIn races)
/// but short enough that an operator gets the page within one coffee break of
/// a real regression. After firing, the counter resets — back-to-back bursts
/// produce back-to-back alerts (every 5 min, not every event).
const SELF_HEAL_WINDOW_SECS: u64 = 300;

/// Listener supervisor: max consecutive immediate failures before we slow
/// down + page the operator (audit LOW-3). Matches the ~10 retries-in-50s
/// budget below; past this point the listener is broken in a way that
/// reconnecting won't fix (PG down, network partition, auth revoked).
const LISTENER_MAX_CONSECUTIVE_FAILURES: u32 = 10;

/// Listener supervisor: base sleep between respawn attempts. Short enough
/// that a transient PG conn drop is invisible to the operator (5s gap in
/// NOTIFY ⇒ poll fallback covers it), long enough that a hard failure
/// doesn't spin the CPU.
const LISTENER_BACKOFF_SECS: u64 = 5;

/// Listener supervisor: extended backoff after exceeding
/// `LISTENER_MAX_CONSECUTIVE_FAILURES`. We don't give up — exiting would
/// leave the worker with no NOTIFY signal source, relying solely on the
/// 30s poll. We keep retrying but at a sustainable cadence so the operator
/// has time to investigate.
const LISTENER_BACKOFF_AFTER_ALERT_SECS: u64 = 60;

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

    // 5. NOTIFY listener + poll fallback. The listener is wrapped in a
    //    supervisor (audit LOW-3) so a transient PG conn drop respawns
    //    automatically; without this the worker would silently degrade
    //    to 30s polling forever after a single recv() error.
    let wakeup = Arc::new(Notify::new());
    let listener_wakeup = wakeup.clone();
    let pg_for_listener = pg.clone();
    let slack_for_listener = slack.clone();
    let listener_handle = tokio::spawn(async move {
        run_listener_supervised(pg_for_listener, listener_wakeup, slack_for_listener).await;
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
                        // wouldn't help. Slack alert fires inside
                        // force_exhaust_job. We pass `attempts` (the
                        // post-claim value preserved from `ClaimedJob`) so
                        // the alert reports the correct number even if a
                        // janitor steal would have bumped the row's
                        // counter mid-panic. Note: no claim-gate on this
                        // path (audit MED-2) — the panic recovery must
                        // terminate the row regardless of who currently
                        // holds the claim, otherwise a panicked recipe +
                        // concurrent janitor steal would leak a stuck row.
                        force_exhaust_job(
                            &pg,
                            job_id,
                            attempts,
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
///
/// `claimed_at` is the exact `NOW()` the claim UPDATE stamped onto the row.
/// It travels with the job so `mark_done` / `mark_failed` can pass it back
/// into their UPDATE WHERE clause as a claim-gate (audit MED-2): if a slow
/// recipe runs past `STUCK_IN_PROGRESS_TIMEOUT_SECS`, the janitor in another
/// worker may have already re-claimed the row (bumping `claimed_at`); in
/// that case the gate ensures the original worker silently discards its
/// result instead of double-writing back-population columns with possibly-
/// different `legacy_*` values than the new claim's recipe will produce.
#[derive(Debug, Clone)]
struct ClaimedJob {
    id: i64,
    intent: WritebackIntent,
    aggregate_id: Uuid,
    attempts: i32,
    claimed_at: DateTime<Utc>,
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
        RETURNING id, intent, payload, aggregate_id, attempts, claimed_at
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
    // claimed_at is set by this very UPDATE (NOW()); guaranteed NOT NULL
    // on the returned row. Used downstream to gate mark_done / mark_failed
    // against a parallel janitor steal (audit MED-2).
    let claimed_at: DateTime<Utc> = row.try_get("claimed_at")?;

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
        claimed_at,
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

    // Resolve legacy IDs from PG canonical tables. `slack` is plumbed in
    // for the MED-4 throttled self-heal alert, which fires from inside
    // `salvage_legacy_ids` when back-population is silently broken.
    let resolved = match resolve_legacy_ids(pg, slack, &job).await {
        Ok(r) => r,
        Err(err) => {
            tracing::error!(job_id, error = %err, "Failed to resolve legacy IDs");
            mark_failed(
                pg,
                job_id,
                job.attempts,
                job.claimed_at,
                max_attempts,
                slack,
                &format!("resolve_legacy_ids: {err}"),
            )
            .await;
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
            mark_failed(
                pg,
                job_id,
                job.attempts,
                job.claimed_at,
                max_attempts,
                slack,
                &format!("mssql_acquire: {err}"),
            )
            .await;
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
                job.claimed_at,
                job.aggregate_id,
                &job.intent,
                slack,
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
                job.attempts,
                job.claimed_at,
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
/// Reads only — does not modify PG. `slack` is plumbed through solely so
/// the self-heal fallback (`salvage_legacy_ids`) can fire the throttled
/// MED-4 alert on sustained back-population failure; passing `None` is
/// safe (the throttle still tracks events for log-grep visibility).
async fn resolve_legacy_ids(
    pg: &PgPool,
    slack: &Option<SlackClient>,
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
                let salvaged = salvage_legacy_ids(pg, slack, *booking_id).await?;
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
                let salvaged = salvage_legacy_ids(pg, slack, *check_in_id).await?;
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
                    let salvaged = salvage_legacy_ids(pg, slack, linked_booking_id).await?;
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

/// Process-local throttle state for self-heal Slack alerts (audit MED-4).
///
/// Tracks how many self-heal events fired since `window_start`, plus the
/// list of `aggregate_id`s involved so the Slack message can name them for
/// the operator. Bounded — we drop excess IDs from the inspection list
/// to keep the Slack message under the 40k-char webhook limit.
#[derive(Debug)]
struct SelfHealCounter {
    /// When the current counting window opened. None = no events yet.
    window_start: Option<Instant>,
    /// Events observed inside the current window.
    count: u32,
    /// Aggregate IDs that triggered self-heal in this window. Capped at
    /// `SELF_HEAL_ALERT_THRESHOLD * 2` so Slack body stays small even if
    /// the threshold is bumped in env config later.
    aggregates: Vec<Uuid>,
}

impl SelfHealCounter {
    const fn new() -> Self {
        Self {
            window_start: None,
            count: 0,
            aggregates: Vec::new(),
        }
    }
}

/// Outcome of recording a self-heal event — separates the throttle decision
/// (pure, easy to test) from the Slack send (impure, hard to test).
#[derive(Debug, PartialEq, Eq)]
struct AlertDecision {
    /// True ⇒ caller should fire Slack and reset the counter.
    fire: bool,
    /// Counter value at decision time (for logging + Slack body).
    count: u32,
    /// Width of the window the counter has been accumulating in.
    window_secs: u64,
}

/// Process-global counter — `OnceLock` initialized lazily. A static keeps
/// the call-site change in `salvage_legacy_ids` to a single
/// `record_self_heal()` call instead of threading another arg through the
/// resolver tree.
static SELF_HEAL_COUNTER: OnceLock<Arc<Mutex<SelfHealCounter>>> = OnceLock::new();

/// Lazily get-or-init the process-global self-heal counter. The `OnceLock`
/// guarantees one allocation across all worker threads.
fn self_heal_counter() -> &'static Arc<Mutex<SelfHealCounter>> {
    SELF_HEAL_COUNTER.get_or_init(|| Arc::new(Mutex::new(SelfHealCounter::new())))
}

/// Pure throttle decision — extracted from the IO path so the threshold and
/// window logic can be unit-tested without spinning up Slack or PG.
///
/// Rules:
///   - First event in a new window opens the window at `now` and counts 1.
///   - Subsequent events inside the same window bump the count.
///   - When count reaches `threshold`, return `fire=true` AND reset the
///     window so the next event opens a fresh one (no spam — exactly one
///     alert per `window_secs` per burst).
///   - When an event arrives after the window has expired, the window
///     resets to `now` with count=1 (no spurious alert from a stale count).
fn should_alert(
    state: &mut SelfHealCounter,
    now: Instant,
    threshold: u32,
    window: Duration,
) -> AlertDecision {
    // Roll the window if it's expired (or never opened).
    let window_alive = state
        .window_start
        .map(|start| now.duration_since(start) < window)
        .unwrap_or(false);

    if !window_alive {
        state.window_start = Some(now);
        state.count = 0;
        state.aggregates.clear();
    }

    state.count = state.count.saturating_add(1);

    let fire = state.count >= threshold;
    let decision = AlertDecision {
        fire,
        count: state.count,
        window_secs: window.as_secs(),
    };

    if fire {
        // Reset for the next window so we don't re-fire on every event past
        // the threshold (audit-mandated throttle).
        state.window_start = None;
        state.count = 0;
        state.aggregates.clear();
    }
    decision
}

/// Record a single self-heal event and, if the burst threshold is breached,
/// fire one Slack alert. Logs at warn-level on every event (so log-grep
/// also catches what Slack does) and at error-level on the alert.
///
/// `slack` is `&Option<SlackClient>` for parity with the rest of the file —
/// when Slack isn't configured the throttle still runs, the operator just
/// sees the warn/error log lines.
async fn record_self_heal(slack: &Option<SlackClient>, aggregate_id: Uuid) {
    let counter = self_heal_counter();
    // Compute decision under the lock — short critical section, no awaits.
    // If the lock is poisoned (some prior call panicked) we recover and
    // keep going: this is a reporting path, not a correctness path.
    let (decision, aggregates_at_alert) = {
        let mut guard = match counter.lock() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };
        // Track the aggregate ID for the Slack body — bounded so a sustained
        // outage doesn't blow up the message size.
        let cap = (SELF_HEAL_ALERT_THRESHOLD as usize).saturating_mul(2);
        if guard.aggregates.len() < cap {
            guard.aggregates.push(aggregate_id);
        }
        // Snapshot aggregates BEFORE should_alert (which clears them on
        // fire=true). The snapshot is what populates the Slack message.
        let snapshot = guard.aggregates.clone();
        let decision = should_alert(
            &mut guard,
            Instant::now(),
            SELF_HEAL_ALERT_THRESHOLD,
            Duration::from_secs(SELF_HEAL_WINDOW_SECS),
        );
        (decision, snapshot)
    };

    // Per-event log (warn) so a log-grep alert can catch sustained drift
    // even if Slack is offline.
    tracing::warn!(
        %aggregate_id,
        count = decision.count,
        window_secs = decision.window_secs,
        threshold = SELF_HEAL_ALERT_THRESHOLD,
        "Self-heal event recorded"
    );

    if !decision.fire {
        return;
    }

    tracing::error!(
        count = decision.count,
        window_secs = decision.window_secs,
        threshold = SELF_HEAL_ALERT_THRESHOLD,
        aggregates = ?aggregates_at_alert,
        "Self-heal threshold breached — back-population may be broken"
    );

    if let Some(slack) = slack {
        send_self_heal_alert(
            slack,
            decision.count,
            decision.window_secs,
            &aggregates_at_alert,
        )
        .await;
    }
}

/// Post a Slack alert when the self-heal counter trips its threshold within
/// the throttle window (audit MED-4). Best-effort: failures are swallowed
/// inside `send_message` so a Slack outage never blocks the writeback loop.
async fn send_self_heal_alert(
    slack: &SlackClient,
    count: u32,
    window_secs: u64,
    aggregates: &[Uuid],
) {
    // Format aggregate UUIDs as a comma-separated SQL `IN (…)` list the
    // operator can paste into the inspection query. Capped already by the
    // counter so this stays under a few hundred bytes.
    let in_list = if aggregates.is_empty() {
        "/* no aggregates captured */".to_string()
    } else {
        aggregates
            .iter()
            .map(|u| format!("'{u}'"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let last_aggregate = aggregates
        .last()
        .map(|u| u.to_string())
        .unwrap_or_else(|| "(none)".to_string());

    let text = format!(
        ":warning: *Writeback self-heal threshold breached* :warning:\n\
         *Events:* {count} self-heals in the last {window_secs}s \
         (threshold: {SELF_HEAL_ALERT_THRESHOLD})\n\
         *Last aggregate:* `{last_aggregate}`\n\
         _Back-population to `ht_*.legacy_*` cache may be broken — likely a \
         PG perms regression, schema drift on the `ht_*` tables, or a \
         mark_done failure pattern. Inspect with:_\n\
         ```\n\
         SELECT id, intent, aggregate_id, last_error\n\
         FROM writeback_jobs\n\
         WHERE status = 'exhausted'\n\
            OR aggregate_id IN ({in_list})\n\
         ORDER BY completed_at DESC NULLS LAST\n\
         LIMIT 50;\n\
         ```\n\
         _Then verify_ \
         `SELECT legacy_book_id, legacy_cin_no FROM ht_bookings JOIN ht_checkins …` \
         _has values and `mark_done`'s UPDATE is succeeding for that intent class._"
    );
    let msg = SlackMessage::with_text(text);
    let _ = slack.send_message(&msg).await;
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
///
/// Side-effect: every successful salvage feeds `record_self_heal`, which
/// drives the throttled MED-4 Slack alert on sustained back-population
/// failure. `slack` is plumbed in solely for that path; passing `None` is
/// safe (the throttle still tracks events for log-grep visibility).
async fn salvage_legacy_ids(
    pg: &PgPool,
    slack: &Option<SlackClient>,
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
            // Audit MED-4: feed the throttled alert path. Operator gets one
            // Slack ping per `SELF_HEAL_WINDOW_SECS` if these fire in bursts.
            record_self_heal(slack, aggregate_id).await;
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
///
/// Audit LOW-2: the UPDATE captures the *prior* status via a CTE so we can
/// detect the `exhausted → done` transition (operator manually fixed +
/// reset the row to `pending`, the next attempt succeeded). On that
/// transition we post a `:white_check_mark:` Slack so the operator sees
/// closure, not just the original `:rotating_light:` alarm.
///
/// **Claim-gating (audit MED-2):** the UPDATE matches only when
/// `status='in_progress' AND claimed_at = $X`. If a slow recipe ran past
/// `STUCK_IN_PROGRESS_TIMEOUT_SECS`, the janitor in another worker may
/// have already re-claimed the row (bumping `claimed_at`) — in which case
/// THIS worker's MSSQL transaction has already been honored on the legacy
/// side but the new claim's recipe will run a second time on top of it
/// (the MSSQL `TABLOCKX` serializes execution so the duplicate recipe
/// gets clean `R-numbers`). The 0-row response is the signal: log a loud
/// warning and skip back-population so we don't race the new claim's
/// `mark_done` to write possibly-different `legacy_*` values into the
/// canonical row.
#[allow(clippy::too_many_arguments)]
async fn mark_done(
    pg: &PgPool,
    job_id: i64,
    claimed_at: DateTime<Utc>,
    aggregate_id: Uuid,
    intent: &WritebackIntent,
    slack: &Option<SlackClient>,
    legacy_ids: serde_json::Value,
) {
    // CTE pattern keeps prior-status capture atomic with the status flip —
    // no race between SELECT and UPDATE in case another worker / janitor
    // touches the row mid-call. The MED-2 claim-gate (status + claimed_at)
    // lives on the UPDATE, not on the prev SELECT — we still want to read
    // the row's prior_status for the LOW-2 closure alert even if the gate
    // would otherwise reject our update.
    let row = sqlx::query(
        r#"
        WITH prev AS (
            SELECT id, status AS prior_status FROM writeback_jobs WHERE id = $1
        )
        UPDATE writeback_jobs wj
           SET status       = 'done',
               completed_at = NOW(),
               legacy_ids   = $2
          FROM prev
         WHERE wj.id = prev.id
           AND wj.status = 'in_progress'
           AND wj.claimed_at = $3
        RETURNING wj.attempts, wj.intent, wj.aggregate_id, prev.prior_status
        "#,
    )
    .bind(job_id)
    .bind(&legacy_ids)
    .bind(claimed_at)
    .fetch_optional(pg)
    .await;

    match &row {
        Ok(Some(r)) => {
            let prior_status: String = r.try_get("prior_status").unwrap_or_default();
            // LOW-2: closure alert on operator-driven recovery.
            if prior_status == "exhausted" {
                let attempts: i32 = r.try_get("attempts").unwrap_or(0);
                let intent_name: String = r.try_get("intent").unwrap_or_default();
                let agg: Option<Uuid> = r.try_get("aggregate_id").ok();
                tracing::warn!(
                    job_id,
                    attempts,
                    intent = %intent_name,
                    ?agg,
                    "Writeback job RESOLVED on retry after prior exhaustion"
                );
                if let Some(slack) = slack {
                    send_resolved_alert(slack, job_id, &intent_name, agg, attempts).await;
                }
            }
        }
        Ok(None) => {
            // 0 rows updated means the original claim was stolen by the
            // stuck-in-progress janitor in another worker (audit MED-2).
            // The other worker will re-run the recipe and write its own
            // legacy_ids; ours would race and possibly clobber theirs with
            // stale values. Discard quietly with a loud log so the operator
            // can spot the race in their dashboards. Skip back-population
            // — the new claim's mark_done will handle that.
            tracing::warn!(
                job_id,
                "Job {job_id} was re-claimed by another worker before mark_done; discarding result"
            );
            return;
        }
        Err(err) => {
            tracing::error!(job_id, error = %err, "Failed to mark job done");
            // Don't bail — still attempt back-population so subsequent
            // intents on the same aggregate can resolve. The stuck-in-
            // progress janitor will eventually retry this job's status row
            // if the UPDATE truly never landed.
        }
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
///
/// **No claim-gate here (intentional, audit MED-2):** the panic recovery
/// path needs to terminate the row regardless of who currently holds the
/// claim. If a recipe panics AND the janitor in another worker has already
/// stolen the claim, both would otherwise leak — gating this UPDATE would
/// leave a stuck `in_progress` row.
///
/// `attempts` is passed in from the caller's `ClaimedJob` (the post-claim
/// value) so the Slack alert reports the correct number even though we no
/// longer rely on the row's stored `attempts` — that would have been the
/// *new* claim's incremented attempts if the row was stolen, which is
/// misleading in the alert.
async fn force_exhaust_job(
    pg: &PgPool,
    job_id: i64,
    attempts: i32,
    slack: &Option<SlackClient>,
    err_msg: &str,
) {
    let row = sqlx::query(
        "UPDATE writeback_jobs SET status='exhausted', last_error=$2, next_retry_at=NULL \
         WHERE id=$1 RETURNING intent, aggregate_id",
    )
    .bind(job_id)
    .bind(err_msg)
    .fetch_one(pg)
    .await;

    match row {
        Ok(row) => {
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
#[allow(clippy::too_many_arguments)]
async fn mark_failed_with_retryable(
    pg: &PgPool,
    job_id: i64,
    attempts: i32,
    claimed_at: DateTime<Utc>,
    max_attempts: i32,
    slack: &Option<SlackClient>,
    err_msg: &str,
    retryable: bool,
) {
    if !retryable {
        force_exhaust_job(pg, job_id, attempts, slack, err_msg).await;
        return;
    }
    mark_failed(pg, job_id, attempts, claimed_at, max_attempts, slack, err_msg).await;
}

/// See [`mark_failed_with_retryable`]. Convenience wrapper for callsites
/// that don't have a typed error in hand (e.g. PG resolve failures).
/// Defaults to retryable=true.
///
/// **HIGH-1 fix:** this used to do an UPDATE then a separate SELECT
/// (`get_attempts_for_backoff`) to compute the backoff seconds. Between
/// the two queries, the stuck-in-progress janitor in another worker could
/// re-claim the row and bump `attempts`, so the backoff was computed
/// against the wrong number. Folded into a single statement: the
/// post-claim `attempts` (preserved on `ClaimedJob`) is passed in by the
/// caller and `backoff_secs` is computed client-side, so the round-trip
/// is gone and the value is always consistent with the row we actually
/// claimed.
///
/// **MED-2 fix:** the UPDATE is gated on
/// `status='in_progress' AND claimed_at = $X` so a slow-recipe + janitor-
/// steal race (where another worker already re-claimed the row) silently
/// discards instead of clobbering the new claim's status. `RETURNING`
/// signals: 0 rows = our claim was stolen.
#[allow(clippy::too_many_arguments)]
async fn mark_failed(
    pg: &PgPool,
    job_id: i64,
    attempts: i32,
    claimed_at: DateTime<Utc>,
    max_attempts: i32,
    slack: &Option<SlackClient>,
    err_msg: &str,
) {
    // HIGH-1: backoff is computed from the post-claim `attempts` carried on
    // ClaimedJob, NOT from a separate SELECT after the UPDATE. That removes
    // the read-modify-write race where a janitor steal between the two
    // queries would have skewed the backoff value.
    let backoff = backoff_secs(attempts);
    // Single statement: bumps status conditionally (CASE on attempts vs
    // max_attempts) AND gates on the claim still being ours (audit MED-2).
    // RETURNING gives us the post-UPDATE state so we can fire the Slack
    // alert exactly on the transition into `exhausted`.
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
           AND status = 'in_progress'
           AND claimed_at = $5
        RETURNING attempts, status, intent, aggregate_id
        "#,
    )
    .bind(job_id)
    .bind(max_attempts)
    .bind(err_msg)
    .bind(backoff)
    .bind(claimed_at)
    .fetch_optional(pg)
    .await;

    let row = match row {
        Ok(Some(r)) => r,
        Ok(None) => {
            // 0 rows updated — our claim was stolen by the stuck-in-progress
            // janitor in another worker (audit MED-2). The new claim will
            // run the recipe again and call its own mark_failed/mark_done.
            // Discard quietly with a loud log so the race is visible.
            tracing::warn!(
                job_id,
                "Job {job_id} was re-claimed by another worker before mark_failed; discarding result"
            );
            return;
        }
        Err(err) => {
            tracing::error!(
                job_id,
                error = %err,
                "Failed to mark job failed — job will be picked up by stuck-in-progress janitor in {STUCK_IN_PROGRESS_TIMEOUT_SECS}s"
            );
            return;
        }
    };

    let post_attempts: i32 = row.try_get("attempts").unwrap_or(attempts);
    let status: String = row.try_get("status").unwrap_or_default();
    let intent: String = row.try_get("intent").unwrap_or_default();
    let aggregate_id: Option<Uuid> = row.try_get("aggregate_id").ok();

    if status == "exhausted" {
        tracing::error!(
            job_id,
            attempts = post_attempts,
            intent = %intent,
            ?aggregate_id,
            "Writeback job EXHAUSTED retries — manual intervention required"
        );
        if let Some(slack) = slack {
            send_exhausted_alert(slack, job_id, &intent, aggregate_id, post_attempts, err_msg)
                .await;
        }
    } else {
        tracing::warn!(
            job_id,
            attempts = post_attempts,
            "Writeback job failed; will retry after backoff"
        );
    }
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

/// Post a Slack closure alert when an `exhausted` job is recovered (audit
/// LOW-2). Fires once per resolution from `mark_done`. The operator
/// previously got a `:rotating_light:` alarm via `send_exhausted_alert`;
/// this `:white_check_mark:` lets them confirm their fix worked without
/// having to query the queue manually. Best-effort like the other alerts.
async fn send_resolved_alert(
    slack: &SlackClient,
    job_id: i64,
    intent: &str,
    aggregate_id: Option<Uuid>,
    attempts: i32,
) {
    let aggregate_id_str = aggregate_id
        .map(|u| u.to_string())
        .unwrap_or_else(|| "(unknown)".into());
    let text = format!(
        ":white_check_mark: *Writeback exhausted job RESOLVED* :white_check_mark:\n\
         *Job ID:* `{job_id}`\n\
         *Intent:* `{intent}`\n\
         *Aggregate:* `{aggregate_id_str}`\n\
         *Resolved on attempt:* {attempts}\n\
         _The previously-exhausted job succeeded after operator intervention. \
         Closure of the_ `:rotating_light:` _alert sent earlier for this job._"
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

/// Supervisor for `run_listener` (audit LOW-3). Respawns the listener on
/// every error with a 5s backoff; if `LISTENER_MAX_CONSECUTIVE_FAILURES`
/// happen back-to-back, fires a Slack alert and slows the retry cadence
/// to `LISTENER_BACKOFF_AFTER_ALERT_SECS` (one alert per burst — same
/// throttle pattern as MED-4) but never gives up.
///
/// Why we keep retrying instead of exiting: the worker has two signal
/// sources — NOTIFY and the 30s poll. If we exit the listener task entirely
/// the worker still functions (it just sees jobs ~30s late). But an
/// operator under time pressure during the live test won't realize sync
/// silently degraded. Persistent reconnect + Slack alert preserves both
/// liveness AND visibility.
async fn run_listener_supervised(
    pg: PgPool,
    wakeup: Arc<Notify>,
    slack: Option<SlackClient>,
) {
    let mut consecutive_failures: u32 = 0;
    loop {
        let pg_inner = pg.clone();
        let wakeup_inner = wakeup.clone();
        match run_listener(pg_inner, wakeup_inner).await {
            Ok(()) => {
                // Listener returned Ok — the only path is `loop {}` exit,
                // which currently can't happen. Treated as success: reset
                // the failure counter and respawn after the standard backoff.
                tracing::warn!("PgListener returned Ok unexpectedly — respawning");
                consecutive_failures = 0;
            }
            Err(err) => {
                consecutive_failures = consecutive_failures.saturating_add(1);
                tracing::error!(
                    error = %err,
                    consecutive_failures,
                    "PgListener task ended; will respawn after backoff"
                );
            }
        }

        let sleep_secs = if consecutive_failures >= LISTENER_MAX_CONSECUTIVE_FAILURES {
            // First time we cross the threshold (or every threshold-th
            // failure after that): page the operator, then back off.
            // Counter is reset post-alert so we get one alert per burst,
            // not one per attempt past the threshold.
            tracing::error!(
                consecutive_failures,
                threshold = LISTENER_MAX_CONSECUTIVE_FAILURES,
                "PgListener supervisor: alert threshold breached — paging operator + slowing respawn"
            );
            if let Some(slack) = &slack {
                send_listener_alert(slack, consecutive_failures).await;
            }
            consecutive_failures = 0;
            LISTENER_BACKOFF_AFTER_ALERT_SECS
        } else {
            LISTENER_BACKOFF_SECS
        };

        tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
    }
}

/// Post a Slack alert when the PG NOTIFY listener has failed to stay up
/// across `LISTENER_MAX_CONSECUTIVE_FAILURES` consecutive respawn attempts
/// (audit LOW-3). The worker is still functional via the 30s poll fallback,
/// but sync latency has degraded from sub-second to ~30s — the operator
/// needs to know.
async fn send_listener_alert(slack: &SlackClient, consecutive_failures: u32) {
    let text = format!(
        ":warning: *Writeback PG NOTIFY listener UNHEALTHY* :warning:\n\
         *Consecutive failures:* {consecutive_failures} \
         (threshold: {LISTENER_MAX_CONSECUTIVE_FAILURES})\n\
         _The worker is still draining the queue via 30s poll fallback, but \
         sync latency has degraded from sub-second to ~30s. Likely causes: \
         PG down, network partition, role missing LISTEN privilege, or \
         max_connections exhausted. Inspect:_\n\
         ```\n\
         SELECT * FROM pg_stat_activity WHERE query LIKE '%LISTEN%';\n\
         SELECT count(*) FROM pg_stat_activity;\n\
         ```\n\
         _The supervisor will keep retrying every \
         {LISTENER_BACKOFF_AFTER_ALERT_SECS}s — fix the underlying issue and \
         the next reconnect will succeed automatically._"
    );
    let msg = SlackMessage::with_text(text);
    let _ = slack.send_message(&msg).await;
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

    /// MED-4 throttle: a single self-heal event must NOT trip the alert —
    /// CreateBooking↔CheckIn races are normal and shouldn't page the
    /// operator. The decision must report fire=false until the threshold
    /// is reached.
    #[test]
    fn should_alert_below_threshold_does_not_fire() {
        let mut state = SelfHealCounter::new();
        let now = Instant::now();
        let decision = should_alert(&mut state, now, 5, Duration::from_secs(300));
        assert!(!decision.fire, "first event must not fire");
        assert_eq!(decision.count, 1);
        assert_eq!(decision.window_secs, 300);
    }

    /// MED-4 throttle: the Nth event inside the window fires exactly once,
    /// then resets the counter so the next event opens a fresh window.
    #[test]
    fn should_alert_at_threshold_fires_once_then_resets() {
        let mut state = SelfHealCounter::new();
        let now = Instant::now();
        let window = Duration::from_secs(300);

        // Events 1..=4 must not fire.
        for expected_count in 1..=4 {
            let d = should_alert(&mut state, now, 5, window);
            assert!(!d.fire, "event {expected_count} must not fire");
            assert_eq!(d.count, expected_count);
        }

        // Event 5 fires and resets.
        let fifth = should_alert(&mut state, now, 5, window);
        assert!(fifth.fire, "5th event must fire");
        assert_eq!(fifth.count, 5);

        // Event 6 (immediately after) must NOT fire — counter was reset,
        // so it reopens a fresh window with count=1. Operator gets exactly
        // one alert per threshold-burst, not one per event past the threshold.
        let sixth = should_alert(&mut state, now, 5, window);
        assert!(!sixth.fire, "6th event must not fire (counter just reset)");
        assert_eq!(sixth.count, 1);
    }

    /// MED-4 throttle: events arriving past the window boundary reset the
    /// counter without firing — a stale partial-burst from yesterday must
    /// not contribute to today's count.
    #[test]
    fn should_alert_window_expiry_resets_counter() {
        let mut state = SelfHealCounter::new();
        let t0 = Instant::now();
        let window = Duration::from_secs(60);

        // Two events open the window with count=2.
        let _ = should_alert(&mut state, t0, 5, window);
        let _ = should_alert(&mut state, t0, 5, window);
        assert_eq!(state.count, 2);

        // Jump past the window — the next event must reset to count=1
        // and not fire (because threshold is 5, not 1).
        let t1 = t0 + Duration::from_secs(120);
        let decision = should_alert(&mut state, t1, 5, window);
        assert!(!decision.fire, "event after window expiry must not fire");
        assert_eq!(decision.count, 1, "counter must reset after window expiry");
    }

    /// MED-4 throttle: threshold of 1 fires immediately on every event —
    /// edge case but the math should still be safe (no off-by-one panic).
    #[test]
    fn should_alert_threshold_of_one_fires_every_event() {
        let mut state = SelfHealCounter::new();
        let now = Instant::now();
        let window = Duration::from_secs(60);

        for _ in 0..3 {
            let d = should_alert(&mut state, now, 1, window);
            assert!(d.fire, "threshold=1 must fire every event");
            assert_eq!(d.count, 1, "counter resets after each fire");
        }
    }

    /// MED-4 constants must form a sensible throttle: threshold > 1 (or
    /// every event would page) and window > 0 (or we'd divide by zero
    /// somewhere reading these). Guards against an env-driven config error
    /// shipping a no-op throttle.
    #[test]
    fn self_heal_constants_are_in_safe_range() {
        assert!(
            SELF_HEAL_ALERT_THRESHOLD >= 2,
            "threshold of 1 means every salvage pages — likely a misconfig"
        );
        assert!(
            SELF_HEAL_WINDOW_SECS >= 60,
            "window <60s makes the throttle useless against bursts"
        );
        assert!(
            SELF_HEAL_WINDOW_SECS <= 3600,
            "window >1h hides regressions for too long"
        );
    }

    /// LOW-3 listener constants must form a usable supervisor: backoff
    /// short enough to be invisible normally, long enough not to spin;
    /// alert threshold high enough to absorb transient flaps but low
    /// enough to page within a minute on a real outage.
    #[test]
    fn listener_supervisor_constants_are_in_safe_range() {
        assert!(LISTENER_BACKOFF_SECS >= 1, "<1s would spin CPU");
        assert!(LISTENER_BACKOFF_SECS <= 30, ">30s defeats the point of NOTIFY");
        assert!(
            LISTENER_MAX_CONSECUTIVE_FAILURES >= 3,
            "<3 would page on every flap"
        );
        assert!(
            LISTENER_BACKOFF_AFTER_ALERT_SECS > LISTENER_BACKOFF_SECS,
            "post-alert backoff must be longer than normal backoff"
        );
    }

    /// HIGH-1 contract: the backoff fed into `mark_failed`'s UPDATE must
    /// be derived from the post-claim `attempts` carried on `ClaimedJob`,
    /// NOT from a separate SELECT after the UPDATE (which could read a
    /// re-claimed `attempts` if the janitor in another worker got there
    /// first). This mirrors the in-function call
    /// `let backoff = backoff_secs(attempts);` — if the function ever
    /// reverts to a second PG round-trip the read could observe a stale
    /// or post-steal value, and this test won't directly catch that, but
    /// it locks in the *expected* mapping so a regression in the schedule
    /// surfaces immediately.
    #[test]
    fn backoff_is_consistent_with_post_claim_attempts() {
        // Worker claimed a row that's now on attempt #2; the backoff
        // written to next_retry_at must be the 2nd schedule entry (120s),
        // regardless of any concurrent re-claim that might have bumped
        // the row's attempts to 3 between the UPDATE and a hypothetical
        // re-SELECT.
        assert_eq!(backoff_secs(2), 120);
    }

    /// MED-2 round-trip: `ClaimedJob.claimed_at` is a required
    /// `DateTime<Utc>` field that survives `Clone` unmodified — the worker
    /// effectively clones the field every time it copies `job.claimed_at`
    /// into a `mark_failed` / `mark_done` callsite inside `process_job`.
    /// If someone changes the type to `Option<...>`, removes the field,
    /// or strips `Clone` from `ClaimedJob`, this test fails fast at
    /// compile time rather than at the next live writeback. The `intent`
    /// value is arbitrary — we're asserting the metadata round-trip, not
    /// anything about the recipe.
    #[test]
    fn claimed_job_carries_claimed_at_through_clone() {
        let claimed_at: DateTime<Utc> =
            DateTime::parse_from_rfc3339("2026-04-25T12:00:00Z")
                .unwrap()
                .with_timezone(&Utc);
        let job = ClaimedJob {
            id: 42,
            intent: WritebackIntent::CheckOut {
                check_in_id: Uuid::nil(),
            },
            aggregate_id: Uuid::nil(),
            attempts: 1,
            claimed_at,
        };
        let cloned = job.clone();
        assert_eq!(cloned.claimed_at, claimed_at);
        assert_eq!(cloned.id, 42);
        assert_eq!(cloned.attempts, 1);
    }
}
