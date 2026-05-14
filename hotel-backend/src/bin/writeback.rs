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

use hotel_backend::config::{DbConfig, SiteConfig, SlackConfig};
use hotel_backend::db::mssql_timeout::{simple_query_with_timeout_drop, MssqlOpKind};
use hotel_backend::db::{create_pool, DbPool};
use hotel_backend::notifications::slack::{SlackClient, SlackMessage};
use hotel_backend::outbox::intent::WritebackIntent;
use hotel_backend::writeback::{
    dispatch, verify_legacy_collation_safety, verify_schema_fingerprint, DispatchContext,
    ResolvedJob, WritebackError,
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

/// Audit H11 — defensive sentinel run on every legacy-conn checkout to clear
/// any open transaction the previous worker may have left behind after a
/// failed `ROLLBACK TRAN`. Idempotent: zero-cost when @@TRANCOUNT is 0 (the
/// normal case), heals the connection when it isn't. See the call site in
/// `process_job` for the full rationale.
const RESET_TRANCOUNT_SQL: &str = "IF @@TRANCOUNT > 0 ROLLBACK";

/// Listener supervisor: extended backoff after exceeding
/// `LISTENER_MAX_CONSECUTIVE_FAILURES`. We don't give up — exiting would
/// leave the worker with no NOTIFY signal source, relying solely on the
/// 30s poll. We keep retrying but at a sustainable cadence so the operator
/// has time to investigate.
const LISTENER_BACKOFF_AFTER_ALERT_SECS: u64 = 60;

/// Track D / T7 HIGH-2 — queue-depth janitor poll interval (60s).
/// Reads `writeback_jobs` grouped by status and pages when any
/// threshold below is breached.
const QUEUE_DEPTH_POLL_INTERVAL_SECS: u64 = 60;

/// Track D / T7 HIGH-2 — backlog thresholds. Per-condition cooldown
/// (`QUEUE_DEPTH_ALERT_COOLDOWN_SECS`) so a known-bad MSSQL outage
/// doesn't flood Slack.
///
/// * `pending > 500` — NOTIFY backlog (worker can't keep up).
/// * `failed > 100` — recipe-level errors stacking; usually a vendor
///   schema drift / collation issue / network partition. The
///   exhausted-alert path covers individual jobs; the threshold here
///   catches the bulk case before it hits the retry cap.
/// * `in_progress > 5 with claimed_at older than 10 min` — stuck
///   claims that the janitor's own steal hasn't reclaimed yet (the
///   `STUCK_IN_PROGRESS_TIMEOUT_SECS` window is 5 min; 10 min is double
///   that to skip the steady-state recipe-in-flight noise floor).
const QUEUE_PENDING_ALERT_THRESHOLD: i64 = 500;
const QUEUE_FAILED_ALERT_THRESHOLD: i64 = 100;
const QUEUE_STUCK_IN_PROGRESS_THRESHOLD: i64 = 5;
/// Bound directly into `make_interval(mins => $1)` in
/// `fetch_queue_depth`. PostgreSQL's `make_interval` is overloaded
/// only on `int` (i32) — a `bigint` (i64) bind raises
/// `function make_interval(mins => bigint) does not exist`, which
/// silently disables the queue-depth alert. `i32::MAX` minutes is
/// ~4083 years so the narrower type costs nothing.
/// Track D regression caught 2026-05-13 production verification.
const QUEUE_STUCK_IN_PROGRESS_AGE_MINS: i32 = 10;

/// Track D / T7 HIGH-2 — minimum gap between queue-depth Slack pages
/// per condition. 30 min — operator gets one ping per breach window.
const QUEUE_DEPTH_ALERT_COOLDOWN_SECS: u64 = 1800;

/// Exponential backoff (in seconds) between retry attempts. Indexed by
/// `attempts` (0-based: backoff_secs(1) is the wait before attempt #2).
/// Caps at the last entry. Default schedule: 30s, 2min, 10min.
fn backoff_secs(attempts_so_far: i32) -> i64 {
    const BACKOFFS: &[i64] = &[30, 120, 600];
    let idx = (attempts_so_far as usize).saturating_sub(1).min(BACKOFFS.len() - 1);
    BACKOFFS[idx]
}

/// Process-global SITE_ID, captured from `SiteConfig::from_env` at the
/// top of `main`. Stored in a `OnceLock` so the many free-standing alert
/// helpers (`send_exhausted_alert`, `send_resolved_alert`,
/// `send_listener_alert`, `send_self_heal_alert`) don't each need a
/// `site_id: &str` parameter — task #69 just needs the prefix in the
/// message text, and threading the value through 6+ deeply-nested
/// callers would balloon the diff for no readability gain. The string
/// is set exactly once during startup; reads are lock-free after that.
///
/// The fallback to `"hfhotel"` only kicks in if some test harness
/// constructs a `SlackMessage` via these helpers without calling
/// `init_site_id` first — production code paths always set it.
static SITE_ID: OnceLock<String> = OnceLock::new();

/// Set the process-wide SITE_ID. Called exactly once at startup, after
/// `SiteConfig::from_env` has validated the env var.
fn init_site_id(id: &str) {
    let _ = SITE_ID.set(id.to_string());
}

/// Read the process-wide SITE_ID; defaults to `"hfhotel"` if uninit.
fn current_site_id() -> &'static str {
    SITE_ID
        .get()
        .map(String::as_str)
        .unwrap_or("hfhotel")
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

    // Task #69: parse SITE_ID once at startup. Panics on a typo so a
    // misconfigured deploy fails loud before the worker pulls jobs.
    let site = SiteConfig::from_env();
    init_site_id(&site.id);
    tracing::info!(site = %site.id, "Writeback worker: site identity resolved");

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

    // 4b1. Wave 6 LOW item 8 — Ville cutover safety: refuse to start on a
    //      case-sensitive collation. Recipes pin every string literal to
    //      the case the .NET app emits; a `_CS_` collation would silently
    //      fork our SQL filters on a fresh Ville cutover. Cheap one-row
    //      SELECT — runs before the fingerprint check so a misconfigured
    //      Ville fails fast at startup.
    if let Err(e) = verify_legacy_collation_safety(&mssql).await {
        tracing::error!(
            site = %site.id,
            error = %e,
            "Legacy MSSQL collation check failed — refusing to start"
        );
        if let Some(slack) = &slack {
            let msg = SlackMessage::with_site_text(
                &site.id,
                format!(
                    ":warning: *Writeback worker REFUSED TO START* :warning:\n\
                     Legacy MSSQL collation is case-sensitive.\n\
                     *Error:* `{e}`\n\
                     _Recipes assume `Thai_CI_AS` (or any `_CI_` collation). \
                     Restore the legacy DB with a case-insensitive collation \
                     before retrying._"
                ),
            );
            let _ = slack.send_message(&msg).await;
        }
        tracing::warn!(
            site = %site.id,
            "Sleeping 60s before exit to throttle Docker restart cadence"
        );
        tokio::time::sleep(Duration::from_secs(60)).await;
        return Err(format!("Legacy collation check failed: {e}").into());
    }

    // 4b. Schema fingerprint guard — refuse to start on drift, but post
    //     a Slack alert first so the operator sees the failure even if
    //     they're not tailing logs. Sleep before returning so the Docker
    //     `restart: unless-stopped` policy backs off (without the sleep,
    //     the worker exits in ms, restarts, fingerprint fails again, fires
    //     another Slack — operator gets paged 6×/min until they intervene).
    if let Err(e) = verify_schema_fingerprint(&mssql).await {
        tracing::error!(
            site = %site.id,
            error = %e,
            "Schema fingerprint check failed — refusing to start"
        );
        if let Some(slack) = &slack {
            let msg = SlackMessage::with_site_text(
                &site.id,
                format!(
                    ":warning: *Writeback worker REFUSED TO START* :warning:\n\
                     Legacy MSSQL schema fingerprint mismatch.\n\
                     *Error:* `{e}`\n\
                     _The legacy DB columns drifted from the captured baseline. \
                     Run_ `./scripts/writeback-fingerprint.sh` _and follow the \
                     README to update the baseline before restarting the worker._"
                ),
            );
            let _ = slack.send_message(&msg).await;
        }
        tracing::warn!(
            site = %site.id,
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

    // Track D / T7 HIGH-2 — queue-depth janitor. Background task that
    // polls `writeback_jobs` every 60s and pages on:
    //   pending > 500, failed > 100, stuck in_progress > 5
    // Per-condition cooldown so a known-bad MSSQL outage doesn't flood.
    let janitor_pg = pg.clone();
    let janitor_slack = slack.clone();
    let janitor_shutdown = shutdown.clone();
    tokio::spawn(async move {
        run_queue_depth_janitor(janitor_pg, janitor_slack, janitor_shutdown).await;
    });

    // Task #69: wrap the main loop in a tracing span so every log line
    // emitted from inside the worker (job claim, dispatch outcome,
    // panic recovery) carries `site=<id>`. Same purpose as the watcher
    // span in `bin/sync.rs`.
    let worker_span = tracing::info_span!("writeback_worker", site = %site.id);
    let _worker_guard = worker_span.enter();

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

    // Audit H11: defensive `IF @@TRANCOUNT > 0 ROLLBACK` at acquisition.
    //
    // If a previous job's `run_in_transaction` failed and its `ROLLBACK TRAN`
    // itself errored (network blip mid-rollback, MSSQL hiccup, etc.) the
    // connection returns to the bb8 pool with an open transaction. bb8-tiberius
    // has no per-checkout `is_valid` hook, so the next worker that checks out
    // that connection inherits the open tran — the next `BEGIN TRAN` nests it
    // (T-SQL `BEGIN TRAN` does NOT start a new outer tran when one is already
    // open, it bumps @@TRANCOUNT), and the next TABLOCKX hangs or commits
    // against the wrong tran scope.
    //
    // This sentinel runs against every checkout and is idempotent: when
    // @@TRANCOUNT is 0 (the normal case) the IF body is skipped — zero wire
    // overhead beyond the round-trip. When it's >0 we ROLLBACK to clear the
    // poison; the connection is then safe for `BEGIN TRAN` below.
    //
    // Lower-blast-radius choice: we do this here (in the worker's hot path)
    // rather than in `db::pool::create_pool` or a bb8 `on_release` /
    // `is_valid` hook, because the legacy pool is shared with other consumers
    // (sync.rs, backfill_rooms.rs, API routes) that don't open explicit
    // transactions — adding a sentinel everywhere would be overhead for no
    // benefit. Only the writeback worker opens `BEGIN TRAN` against this
    // pool, so only the writeback worker needs to clean up poisoned conns.
    // R2 (2026-05-14): wrap defensive @@TRANCOUNT reset in write-budget
    // timeout. If the previous worker's connection returned with a
    // poisoned tran AND the server is now wedged on the table that
    // tran holds locks against, the bare `simple_query` here would
    // hang the dispatcher indefinitely. The timeout converts that
    // into a retryable failure (Tiberius::Io{TimedOut}).
    let trancount_reset =
        simple_query_with_timeout_drop(&mut conn, RESET_TRANCOUNT_SQL, MssqlOpKind::Write).await;
    if let Err(err) = trancount_reset {
        tracing::warn!(
            job_id,
            error = %err,
            "defensive @@TRANCOUNT rollback failed — dropping conn"
        );
        drop(conn);
        mark_failed(
            pg,
            job_id,
            job.attempts,
            job.claimed_at,
            max_attempts,
            slack,
            &format!("trancount_reset: {err}"),
        )
        .await;
        return;
    }

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
        // R2: BEGIN TRAN itself is fast on MSSQL, but wrap it for
        // symmetry with the rest of the writeback path so the whole
        // transaction lives under one consistent timeout envelope.
        simple_query_with_timeout_drop(conn, "BEGIN TRAN", MssqlOpKind::Write).await?;
    }

    let dispatch_result = dispatch(conn, intent, resolved, ctx).await;

    match dispatch_result {
        Ok(legacy_ids) => {
            // R2: COMMIT TRAN. A wedged commit (e.g. log-flush stall on
            // legacy MSSQL) would leave the recipe in a "did it land or
            // not?" twilight zone — the timeout flips that into an
            // explicit retryable failure.
            simple_query_with_timeout_drop(conn, "COMMIT TRAN", MssqlOpKind::Write).await?;
            Ok(legacy_ids)
        }
        Err(err) => {
            // Best-effort rollback. If ROLLBACK itself fails, the connection
            // is poisoned and bb8 will discard it on next acquire — the data
            // remains safe because nothing was committed.
            //
            // R2: also wrap in timeout — a stuck ROLLBACK on a poisoned
            // connection was the exact symptom of the 2026-05-14
            // incident. We log + drop the conn; the timeout makes that
            // path reach the drop instead of hanging forever.
            match simple_query_with_timeout_drop(conn, "ROLLBACK TRAN", MssqlOpKind::Write).await
            {
                Ok(()) => {}
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
        | CheckOut { check_in_id, .. }
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
        // Track G2 / T4 CRIT-1 — `RefundPayment` resolves the same
        // check-in identifiers as `RecordPayment` plus the original
        // payment's legacy_pay_no (for the audit note in
        // `HT_CheckIn_Pay.Cin_Pay_Note`). The original is looked up via
        // its aggregate id; None when the payment row doesn't exist or
        // back-population is still pending — the recipe degrades to a
        // generic `REFUND` note prefix.
        RefundPayment {
            check_in_id,
            original_payment_aggregate_id,
            ..
        } => {
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
            if resolved.legacy_cin_no.is_none()
                || resolved.legacy_room_no.is_none()
                || resolved.legacy_cust_no.is_none()
            {
                let salvaged = salvage_legacy_ids(pg, slack, *check_in_id).await?;
                resolved.legacy_cin_no = resolved.legacy_cin_no.or(salvaged.cin_no);
                resolved.legacy_room_no = resolved.legacy_room_no.or(salvaged.room_no);
                resolved.legacy_cust_no = resolved.legacy_cust_no.or(salvaged.cust_no);
            }
            if let Some(orig_aggregate) = original_payment_aggregate_id {
                if let Some(row) = sqlx::query(
                    "SELECT legacy_pay_no FROM ht_payments WHERE aggregate_id = $1",
                )
                .bind(orig_aggregate)
                .fetch_optional(pg)
                .await?
                {
                    resolved.legacy_original_pay_no = row.try_get("legacy_pay_no").ok();
                }
            }
        }
        // Track G4 / T4 HIGH-3 — `RoomChange`. Load the full canonical
        // `ht_room_changes` row keyed by `rc_id`, then resolve the
        // legacy `cin_no` + both `room_no`s via FK joins. The recipe
        // consumes the hydrated struct only.
        RoomChange { rc_id, .. } => {
            use hotel_backend::writeback::dispatcher::ResolvedRoomChange;
            if let Some(row) = sqlx::query(
                "SELECT \
                    rc.rc_id, \
                    COALESCE(c.legacy_cin_no, '') AS legacy_cin_no, \
                    COALESCE(rf.room_no, '')      AS from_room_no, \
                    COALESCE(rt.room_no, '')      AS to_room_no, \
                    COALESCE(rc.rc_reason, '')    AS reason, \
                    rc.rc_changed_at, \
                    COALESCE(rc.rc_changed_by, '')AS changed_by, \
                    rc.rc_room_before_price::float8 AS room_before_price_baht, \
                    COALESCE(rc.rc_to_price, '')  AS to_price \
                 FROM ht_room_changes rc \
                 JOIN ht_checkins  c  ON c.cin_id  = rc.rc_cin_id \
                 JOIN ht_rooms_new rf ON rf.room_id = rc.rc_from_room_id \
                 JOIN ht_rooms_new rt ON rt.room_id = rc.rc_to_room_id \
                 WHERE rc.rc_id = $1",
            )
            .bind(*rc_id)
            .fetch_optional(pg)
            .await?
            {
                resolved.room_change = Some(ResolvedRoomChange {
                    rc_id: row.try_get("rc_id").unwrap_or(*rc_id),
                    legacy_cin_no: row.try_get("legacy_cin_no").unwrap_or_default(),
                    from_room_no: row.try_get("from_room_no").unwrap_or_default(),
                    to_room_no: row.try_get("to_room_no").unwrap_or_default(),
                    reason: row.try_get("reason").unwrap_or_default(),
                    changed_at: row.try_get("rc_changed_at").unwrap_or_else(|_| Utc::now()),
                    changed_by: row.try_get("changed_by").unwrap_or_default(),
                    room_before_price_baht: row
                        .try_get("room_before_price_baht")
                        .unwrap_or(0.0),
                    to_price: row.try_get("to_price").unwrap_or_default(),
                });
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
        // Track F3 — AdjustProductStock carries the legacy `Pro_no`
        // business key directly in its payload (product master is keyed
        // on the same value on both sides). No PG lookup or self-heal
        // is required at this layer.
        AdjustProductStock { .. } => {}
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
    let msg = SlackMessage::with_site_text(current_site_id(), text);
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
            // Wave 5a item 4: when the `mark_done` UPDATE itself errors we
            // can't distinguish "status flip never landed" from "status
            // flipped but the RETURNING read errored" — and we may be
            // racing the janitor's stolen claim in another worker. The
            // safe choice is to skip back-population so we don't clobber
            // a stolen-claim winner's legacy_ids with our (potentially
            // stale) values. The resolver's self-heal path
            // (`salvage_legacy_ids`) will fish the ids out of
            // `writeback_jobs.legacy_ids` JSONB at the next intent on the
            // same aggregate, so no data loss — just a slower lookup.
            tracing::error!(
                job_id,
                error = %err,
                "Failed to mark job done; skipping back-population to avoid \
                 clobbering a stolen-claim winner. Resolver self-heal will \
                 recover legacy_ids from writeback_jobs at next intent."
            );
            return;
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
    let pay_no = legacy_ids.get("pay_no").and_then(|v| v.as_str());
    let receipt_no = legacy_ids.get("receipt_no").and_then(|v| v.as_str());
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
        CreateCheckIn { .. } | CancelCheckIn { .. } | ExtendStay { .. } | CheckOut { .. } => {
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
            // Track B4 — for multi-room folios the recipe returns one
            // `HT_CheckIn_Ds.id` per junction room in
            // `checkin_ds_ids_by_room`. Stamp each one onto the matching
            // `ht_checkin_rooms` row so the next intent on the same
            // folio (ExtendStay, RecordPayment by-room, CancelCheckIn
            // by-room) can target the correct legacy row by id rather
            // than re-deriving it from `(Cin_no, Cin_Room_No)`.
            //
            // Best-effort: a `room_no` that no longer exists in the
            // junction (e.g. the orchestrator already dropped it as
            // part of an edit) is a no-op. Single-room folios surface
            // an empty array so the WHERE-NOT-NULL guard skips the
            // UPDATE entirely.
            if let Some(pairs) =
                legacy_ids.get("checkin_ds_ids_by_room").and_then(|v| v.as_array())
            {
                for pair in pairs {
                    let arr = match pair.as_array() {
                        Some(a) if a.len() == 2 => a,
                        _ => continue,
                    };
                    let pair_room_no = match arr[0].as_str() {
                        Some(s) => s,
                        None => continue,
                    };
                    let pair_ds_id = match arr[1].as_i64() {
                        Some(n) => n as i32,
                        None => continue,
                    };
                    sqlx::query(
                        "UPDATE ht_checkin_rooms cr \
                            SET cr_legacy_ds_id = $3, cr_updated_at = NOW() \
                          FROM ht_checkins c, ht_rooms_new r \
                          WHERE cr.cr_cin_id = c.cin_id \
                            AND cr.cr_room_id = r.room_id \
                            AND c.aggregate_id = $1 \
                            AND r.room_no = $2",
                    )
                    .bind(aggregate_id)
                    .bind(pair_room_no)
                    .bind(pair_ds_id)
                    .execute(pg)
                    .await?;
                }
            }
        }
        // Track G2 / T4 CRIT-1 — refund back-population. Mirrors
        // RecordPayment but the recipe doesn't allocate a Receipt_no
        // (refunds don't emit HT_Receipt_H rows), so only legacy_pay_no
        // matters here. The aggregate_id passed in by the caller is the
        // check-in's; the new refund row's aggregate id lives in the
        // intent's `payment_aggregate_id`.
        RefundPayment { payment_aggregate_id, .. } => {
            if cin_no.is_some() || room_no.is_some() || cust_no.is_some() {
                sqlx::query(
                    "UPDATE ht_checkins SET \
                       legacy_cin_no  = COALESCE($2, legacy_cin_no), \
                       legacy_room_no = COALESCE($3, legacy_room_no), \
                       legacy_cust_no = COALESCE($4, legacy_cust_no), \
                       updated_at = NOW() \
                     WHERE aggregate_id = $1",
                )
                .bind(aggregate_id)
                .bind(cin_no)
                .bind(room_no)
                .bind(cust_no)
                .execute(pg)
                .await?;
            }
            if let (Some(refund_aggregate), true) =
                (payment_aggregate_id, pay_no.is_some())
            {
                sqlx::query(
                    "UPDATE ht_payments SET legacy_pay_no = COALESCE($2, legacy_pay_no) \
                     WHERE aggregate_id = $1",
                )
                .bind(refund_aggregate)
                .bind(pay_no)
                .execute(pg)
                .await?;
            }
        }
        RecordPayment { payment_aggregate_id, .. } => {
            // Payment back-population is split-target: ht_checkins keeps the
            // check-in identifiers (the aggregate_id passed in is the
            // check-in's), and ht_payments gets the freshly-allocated
            // legacy_pay_no / legacy_receipt_no keyed off the payment's own
            // aggregate_id (Wave 5a item 3). Both UPDATEs are independent —
            // either can land without the other.
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
            if let (Some(pay_aggregate), true) =
                (payment_aggregate_id, pay_no.is_some() || receipt_no.is_some())
            {
                sqlx::query(
                    "UPDATE ht_payments SET \
                       legacy_pay_no     = COALESCE($2, legacy_pay_no), \
                       legacy_receipt_no = COALESCE($3, legacy_receipt_no) \
                     WHERE aggregate_id = $1",
                )
                .bind(pay_aggregate)
                .bind(pay_no)
                .bind(receipt_no)
                .execute(pg)
                .await?;
            }
        }
        MarkRoomClean { .. } => {
            // mark_clean doesn't allocate any new legacy IDs.
        }
        // Track G4 / T4 HIGH-3 — RoomChange back-populates the freshly
        // allocated HT_Changed_Room.id onto ht_room_changes.rc_legacy_id
        // so the next read-side join can cross-reference the legacy
        // audit row. The recipe stuffs the id into legacy_ids.extra
        // (keys `rc_id` + `ht_changed_room_id`); we pull both out here
        // and run the targeted UPDATE on the canonical row.
        RoomChange { .. } => {
            let rc_id = legacy_ids
                .get("extra")
                .and_then(|v| v.get("rc_id"))
                .and_then(|v| v.as_i64());
            let legacy_id = legacy_ids
                .get("extra")
                .and_then(|v| v.get("ht_changed_room_id"))
                .and_then(|v| v.as_i64())
                .map(|n| n as i32);
            if let (Some(rc_id), Some(legacy_id)) = (rc_id, legacy_id) {
                sqlx::query(
                    "UPDATE ht_room_changes \
                        SET rc_legacy_id = $2, rc_updated_at = NOW() \
                      WHERE rc_id = $1 AND rc_legacy_id IS NULL",
                )
                .bind(rc_id)
                .bind(legacy_id)
                .execute(pg)
                .await?;
            }
            // ht_checkins legacy_cin_no may also surface here when this is
            // the first writeback after a fresh walkin; mirror the existing
            // check-in branch so the cached cin_no doesn't decay.
            if cin_no.is_some() {
                sqlx::query(
                    "UPDATE ht_checkins SET \
                       legacy_cin_no = COALESCE($2, legacy_cin_no), \
                       updated_at = NOW() \
                     WHERE aggregate_id = $1",
                )
                .bind(aggregate_id)
                .bind(cin_no)
                .execute(pg)
                .await?;
            }
        }
        // Track F3 — AdjustProductStock targets an existing
        // `HT_Products` row by `Pro_no` (already in the payload). The
        // recipe never allocates a new legacy id; the canonical
        // `ht_products.legacy_*` fields are populated by the sync
        // mapper, not by writebacks. Nothing to back-populate here.
        AdjustProductStock { .. } => {}
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
    let msg = SlackMessage::with_site_text(current_site_id(), text);
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
    let msg = SlackMessage::with_site_text(current_site_id(), text);
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
    let msg = SlackMessage::with_site_text(current_site_id(), text);
    let _ = slack.send_message(&msg).await;
}

/// Track D / T7 HIGH-2 — snapshot of writeback queue depth pulled by
/// the janitor's group-by-status query. The three counts cover every
/// alertable condition; `done` rows are omitted because they're the
/// happy path.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct QueueDepthSnapshot {
    pub pending: i64,
    pub failed: i64,
    /// `in_progress` rows whose `claimed_at` is older than
    /// `QUEUE_STUCK_IN_PROGRESS_AGE_MINS` minutes. NOT the total
    /// `in_progress` count — steady-state recipes run for seconds and
    /// don't count as stuck.
    pub stuck_in_progress: i64,
}

/// Track D / T7 HIGH-2 — pure decision function. Given a snapshot,
/// returns the human-readable reasons for each breached condition.
/// Empty vec ⇒ everything within tolerance.
pub fn queue_depth_breaches(snapshot: &QueueDepthSnapshot) -> Vec<String> {
    let mut out = Vec::new();
    if snapshot.pending > QUEUE_PENDING_ALERT_THRESHOLD {
        out.push(format!(
            "pending={} > {}",
            snapshot.pending, QUEUE_PENDING_ALERT_THRESHOLD
        ));
    }
    if snapshot.failed > QUEUE_FAILED_ALERT_THRESHOLD {
        out.push(format!(
            "failed={} > {}",
            snapshot.failed, QUEUE_FAILED_ALERT_THRESHOLD
        ));
    }
    if snapshot.stuck_in_progress > QUEUE_STUCK_IN_PROGRESS_THRESHOLD {
        out.push(format!(
            "stuck in_progress={} > {} (claimed > {}m ago)",
            snapshot.stuck_in_progress,
            QUEUE_STUCK_IN_PROGRESS_THRESHOLD,
            QUEUE_STUCK_IN_PROGRESS_AGE_MINS,
        ));
    }
    out
}

/// Track D / T7 HIGH-2 — janitor that polls `writeback_jobs` every
/// 60s and pages the operator if any threshold is breached. One alert
/// per condition per `QUEUE_DEPTH_ALERT_COOLDOWN_SECS` to avoid
/// flooding. Best-effort: a failed PG query only logs a warning.
async fn run_queue_depth_janitor(
    pg: PgPool,
    slack: Option<SlackClient>,
    shutdown: Arc<Notify>,
) {
    let mut last_alerted_pending: Option<Instant> = None;
    let mut last_alerted_failed: Option<Instant> = None;
    let mut last_alerted_stuck: Option<Instant> = None;
    let cooldown = Duration::from_secs(QUEUE_DEPTH_ALERT_COOLDOWN_SECS);

    tracing::info!(
        pending_threshold = QUEUE_PENDING_ALERT_THRESHOLD,
        failed_threshold = QUEUE_FAILED_ALERT_THRESHOLD,
        stuck_threshold = QUEUE_STUCK_IN_PROGRESS_THRESHOLD,
        stuck_age_mins = QUEUE_STUCK_IN_PROGRESS_AGE_MINS,
        "[janitor] Queue-depth janitor starting"
    );

    loop {
        tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(QUEUE_DEPTH_POLL_INTERVAL_SECS)) => {}
            _ = shutdown.notified() => {
                tracing::info!("[janitor] Shutdown — exiting");
                return;
            }
        }

        let snapshot = match fetch_queue_depth(&pg).await {
            Ok(s) => s,
            Err(err) => {
                tracing::warn!(
                    error = %err,
                    "[janitor] Failed to fetch writeback_jobs depth — observability degraded"
                );
                continue;
            }
        };

        let breaches = queue_depth_breaches(&snapshot);
        if breaches.is_empty() {
            tracing::trace!(?snapshot, "[janitor] Queue depth within thresholds");
            continue;
        }

        for reason in &breaches {
            tracing::warn!(
                pending = snapshot.pending,
                failed = snapshot.failed,
                stuck_in_progress = snapshot.stuck_in_progress,
                reason,
                "[janitor] Queue-depth breach detected"
            );
        }

        let now = Instant::now();
        // Per-condition cooldown — fire only the kinds whose cooldown
        // has elapsed; the message lists every active breach for
        // operator visibility.
        let pending_due = snapshot.pending > QUEUE_PENDING_ALERT_THRESHOLD
            && last_alerted_pending
                .map(|t| now.duration_since(t) >= cooldown)
                .unwrap_or(true);
        let failed_due = snapshot.failed > QUEUE_FAILED_ALERT_THRESHOLD
            && last_alerted_failed
                .map(|t| now.duration_since(t) >= cooldown)
                .unwrap_or(true);
        let stuck_due = snapshot.stuck_in_progress > QUEUE_STUCK_IN_PROGRESS_THRESHOLD
            && last_alerted_stuck
                .map(|t| now.duration_since(t) >= cooldown)
                .unwrap_or(true);

        if !(pending_due || failed_due || stuck_due) {
            tracing::debug!("[janitor] All breached conditions still in cooldown");
            continue;
        }

        if let Some(s) = slack.as_ref() {
            let body = breaches
                .iter()
                .map(|r| format!("• {r}"))
                .collect::<Vec<_>>()
                .join("\n");
            let payload = SlackMessage::with_site_text(
                current_site_id(),
                format!(
                    ":warning: *Writeback queue depth breach* :warning:\n\
                     {body}\n\
                     _Cooldown {QUEUE_DEPTH_ALERT_COOLDOWN_SECS}s per condition. \
                     Inspect with_ `SELECT intent, status, count(*) FROM \
                     writeback_jobs GROUP BY intent, status;` _and the \
                     dashboard at_ `/api/new/sync/status` _(writebackQueue \
                     section)._"
                ),
            );
            let _ = s.send_message(&payload).await;
        }

        if pending_due {
            last_alerted_pending = Some(now);
        }
        if failed_due {
            last_alerted_failed = Some(now);
        }
        if stuck_due {
            last_alerted_stuck = Some(now);
        }
    }
}

/// Track D / T7 HIGH-2 — one round-trip to PG for the three counts.
async fn fetch_queue_depth(pg: &PgPool) -> Result<QueueDepthSnapshot, sqlx::Error> {
    // One query for all three numbers. Cheap on the existing
    // ix_writeback_jobs_claim partial index (status IN
    // ('pending','failed','in_progress')).
    let row = sqlx::query_as::<_, (i64, i64, i64)>(
        r#"
        SELECT
            COUNT(*) FILTER (WHERE status = 'pending'),
            COUNT(*) FILTER (WHERE status = 'failed'),
            COUNT(*) FILTER (
                WHERE status = 'in_progress'
                  AND claimed_at IS NOT NULL
                  AND claimed_at < now() - make_interval(mins => $1)
            )
        FROM writeback_jobs
        WHERE status IN ('pending', 'failed', 'in_progress')
        "#,
    )
    .bind(QUEUE_STUCK_IN_PROGRESS_AGE_MINS)
    .fetch_one(pg)
    .await?;
    Ok(QueueDepthSnapshot {
        pending: row.0,
        failed: row.1,
        stuck_in_progress: row.2,
    })
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

    /// Track D regression (2026-05-13 production verification):
    /// `QUEUE_STUCK_IN_PROGRESS_AGE_MINS` is bound directly into
    /// `make_interval(mins => $1)` in `fetch_queue_depth`. PostgreSQL's
    /// `make_interval(mins => ...)` is overloaded only on `int` (i32) — a
    /// `bigint` (i64) bind raises
    /// `function make_interval(mins => bigint) does not exist`, which
    /// silently disables the queue-depth alert (the janitor catches and
    /// logs the error every 60s but never pages).
    ///
    /// This test pins the type at the const site. If a future edit widens
    /// it back to i64 the `let bind_value: i32 = ...` line stops
    /// type-checking at compile time. i32::MAX minutes is ~4083 years so
    /// the narrower type costs nothing.
    #[test]
    fn queue_stuck_in_progress_age_mins_is_i32_for_make_interval() {
        // Compile-time: this assignment only type-checks while the const
        // is exactly i32. A widening to i64 turns this into an error.
        let bind_value: i32 = QUEUE_STUCK_IN_PROGRESS_AGE_MINS;
        assert!(bind_value > 0, "must be positive for make_interval");
        assert!(bind_value < 60 * 24, "more than 1 day is too slow to alert");
    }

    /// Audit H11: the defensive sentinel SQL MUST gate the ROLLBACK on
    /// `@@TRANCOUNT > 0` so it's a no-op when the connection is clean
    /// (the normal case). An unconditional `ROLLBACK` would error with
    /// "no transaction is active" on every checkout — slow + noisy.
    ///
    /// We assert the literal form here because a typo (`@@TRANCOUNT >= 0`,
    /// or omitting the IF) would silently regress the invariant in a way
    /// no integration test would catch on a healthy pool.
    #[test]
    fn reset_trancount_sql_is_guarded_idempotent_form() {
        assert_eq!(RESET_TRANCOUNT_SQL, "IF @@TRANCOUNT > 0 ROLLBACK");
        let upper = RESET_TRANCOUNT_SQL.to_ascii_uppercase();
        assert!(upper.contains("IF @@TRANCOUNT > 0"));
        assert!(upper.contains("ROLLBACK"));
        assert!(!upper.contains("BEGIN"), "must not BEGIN a tran");
        assert!(!upper.contains("COMMIT"), "must not COMMIT");
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
                nights: None,
                room_price_total: None,
                product_total: None,
                net_total: None,
                pay_total: None,
                balance: None,
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

    /// Wave 5a item 4 — the `Err(_)` arm of `mark_done`'s row-match
    /// must skip back-population entirely. Since the function takes
    /// a live `&PgPool` argument we can't unit-test the dispatch
    /// runtime, but we can structurally pin the source-code shape:
    /// the `Err(err) =>` arm in the match must contain an early
    /// `return` BEFORE the back_populate_legacy_ids retry loop.
    /// A regression that drops the `return` would silently let a
    /// stale `legacy_ids` clobber a stolen-claim winner's row.
    #[test]
    fn mark_done_err_arm_returns_before_back_population() {
        let source = include_str!("writeback.rs");
        // Find the `mark_done` function body.
        let fn_start = source
            .find("async fn mark_done(")
            .expect("mark_done must exist");
        // Find the back-pop retry loop.
        let back_pop_pos = source[fn_start..]
            .find("back_populate_legacy_ids(pg, aggregate_id, intent, &legacy_ids)")
            .expect("mark_done must call back_populate_legacy_ids");
        // Find the `Err(err) =>` arm of the match.
        let err_arm_pos = source[fn_start..]
            .find("Err(err) => {")
            .expect("mark_done must have an Err(err) match arm");
        assert!(
            err_arm_pos < back_pop_pos,
            "Err(err) arm must precede the back-pop call (it's inside the \
             match that classifies the mark_done UPDATE result)"
        );
        // From the Err arm, locate the next `return;` and the next `}` so
        // we can assert the early-return sits inside the arm.
        let from_err = &source[fn_start + err_arm_pos..];
        let return_pos = from_err
            .find("return;")
            .expect("Err arm must contain `return;` per Wave 5a item 4");
        // The back-pop loop is far below the Err arm (after the closing
        // `}` of the match). We need the `return;` to come BEFORE the
        // back-pop call to guarantee the Err path skips it.
        let abs_return = fn_start + err_arm_pos + return_pos;
        let abs_back_pop = fn_start + back_pop_pos;
        assert!(
            abs_return < abs_back_pop,
            "the Err arm's `return;` must execute before the back-pop \
             retry loop — otherwise a failed status-flip could still \
             clobber a stolen-claim winner's legacy_ids"
        );
    }

    // -------------------------------------------------------------------
    // Track D / T7 HIGH-2 — queue-depth janitor
    // -------------------------------------------------------------------

    #[test]
    fn queue_depth_alert_fires_above_threshold() {
        // pending=600 > 500 → one breach reason.
        let snap = QueueDepthSnapshot {
            pending: 600,
            failed: 0,
            stuck_in_progress: 0,
        };
        let breaches = queue_depth_breaches(&snap);
        assert_eq!(breaches.len(), 1);
        assert!(breaches[0].contains("pending=600"));
    }

    #[test]
    fn queue_depth_failed_threshold_fires_above_100() {
        let snap = QueueDepthSnapshot {
            pending: 0,
            failed: 101,
            stuck_in_progress: 0,
        };
        let breaches = queue_depth_breaches(&snap);
        assert_eq!(breaches.len(), 1);
        assert!(breaches[0].contains("failed=101"));
    }

    #[test]
    fn queue_depth_stuck_in_progress_fires_above_5() {
        let snap = QueueDepthSnapshot {
            pending: 0,
            failed: 0,
            stuck_in_progress: 6,
        };
        let breaches = queue_depth_breaches(&snap);
        assert_eq!(breaches.len(), 1);
        assert!(breaches[0].contains("stuck in_progress=6"));
    }

    #[test]
    fn queue_depth_no_breach_at_or_below_thresholds() {
        // Strict-greater semantics — exactly-at-threshold must NOT alert.
        let snap = QueueDepthSnapshot {
            pending: 500,
            failed: 100,
            stuck_in_progress: 5,
        };
        assert!(queue_depth_breaches(&snap).is_empty());

        let snap = QueueDepthSnapshot {
            pending: 0,
            failed: 0,
            stuck_in_progress: 0,
        };
        assert!(queue_depth_breaches(&snap).is_empty());
    }

    #[test]
    fn queue_depth_multiple_simultaneous_breaches() {
        let snap = QueueDepthSnapshot {
            pending: 1000,
            failed: 200,
            stuck_in_progress: 50,
        };
        let breaches = queue_depth_breaches(&snap);
        assert_eq!(
            breaches.len(),
            3,
            "all three conditions must produce one reason each"
        );
    }
}
