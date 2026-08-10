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

use std::collections::HashMap;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
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
use hotel_backend::outbox::legacy_stale::{self, StaleNote};
use hotel_backend::writeback::{
    dispatch, verify_legacy_collation_safety, verify_schema_fingerprint,
    verify_writeback_ledger_exists, DispatchContext, ResolvedJob, WritebackError,
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

/// Self-heal alert window (audit MED-4). Two jobs, both measured in TIME:
///
/// * the burst window — `SELF_HEAL_ALERT_THRESHOLD` events must land inside
///   it before anything fires, so a handful of expected salvages per hour
///   (CreateBooking↔CheckIn races) never pages;
/// * the minimum gap between two self-heal pages, so a sustained salvage
///   rate cannot page faster than once per window.
///
/// The second job used to be missing. `should_alert` zeroed the counter when
/// it fired and nothing else gated the next send, so the real meaning was
/// "one alert per {SELF_HEAL_ALERT_THRESHOLD} events" — at 5 salvages/second
/// (a broken back-population during a queue drain) that is one Slack POST
/// per second, with no floor on the interval at all. The floor is now
/// explicit: see `SelfHealCounter::last_alert_at`.
const SELF_HEAL_WINDOW_SECS: u64 = 300;

/// Collapse window for the `Writeback EXHAUSTED retries` page.
///
/// The alert is per-JOB, and two classes of failure exhaust a job on its
/// FIRST attempt without ever touching the retry budget:
///
/// * non-retryable errors (`Recipe` / `SchemaDrift` / `IntentMismatch` /
///   `Serde` / `Config` / `Disabled`) — routed straight to
///   `force_exhaust_job` by `mark_failed_with_retryable`;
/// * panics — force-exhausted by the main loop's `JoinError` arm.
///
/// So one bad recipe or one vendor schema change pages once per affected
/// row, at full drain speed, and every `await`ed Slack POST slows the drain
/// further. We collapse repeats of the same `(intent, error-class)` inside
/// this window into a single follow-up message carrying the suppressed
/// count. Sized to match `SELF_HEAL_WINDOW_SECS` — long enough to absorb a
/// full-queue drain of one bad class, short enough that a genuinely new
/// burst pages within a coffee break.
///
/// The FIRST occurrence of a class is never suppressed: it is the
/// actionable one, and it carries the full error text.
const EXHAUSTED_ALERT_WINDOW_SECS: u64 = 300;

/// Listener supervisor: how long the NOTIFY listener has to be
/// **continuously** down before the operator is paged (audit LOW-3, recalibrated).
///
/// This alert used to fire on a COUNT — 10 consecutive respawn failures —
/// with the counter zeroed on every send and no cooldown timestamp. At a 5s
/// backoff that is one page per ~105s (10×5s of retries + the 60s post-alert
/// backoff) for the entire duration of an outage, for a condition that is
/// self-recovering by design: the supervisor reconnects forever, and the
/// worker keeps draining the queue on its 30s poll the whole time. Nothing
/// is lost, nothing is stuck; only NOTIFY latency degrades (sub-second ⇒
/// ≤30s). That is a log line, not a page.
///
/// **Why 10 minutes.** Two independent grounds:
///
/// * *Every self-recovering cause clears well inside it.* The routine PG
///   interruptions here are a deploy (`run-deploy.sh` recreating containers
///   and running migrations), a `newdb` restart, a brief WireGuard blip, or
///   a `max_connections` spike — all seconds-to-low-minutes. Ten minutes is
///   past all of them, so a page at this point means reconnection is NOT
///   happening on its own.
/// * *It outlives every other self-healing mechanism in this binary.* It is
///   2× `STUCK_IN_PROGRESS_TIMEOUT_SECS` (the janitor's claim-steal window),
///   so an operator paged here is looking at something that already survived
///   the worker's own recovery paths. At `LISTENER_BACKOFF_SECS` that is
///   ~120 failed reconnects in a row — unambiguous.
///
/// Cost of waiting: the queue is drained on the 30s poll throughout, so the
/// whole delay buys at most ~20 extra polls of added latency and zero
/// durability risk. Bulk symptoms have their own faster page — the
/// queue-depth janitor fires on `pending > 500` regardless of this alert.
const LISTENER_SUSTAINED_OUTAGE_SECS: u64 = 600;

/// Listener supervisor: minimum gap between two pages inside ONE sustained
/// outage. Matches `QUEUE_DEPTH_ALERT_COOLDOWN_SECS` — the operator is
/// already engaged after the first page; re-stating it every 105s only
/// trains them to mute the channel.
const LISTENER_REPAGE_COOLDOWN_SECS: u64 = 1800;

/// Listener supervisor: how long a subscription must survive before the
/// session counts as HEALTHY and clears the outage clock. One poll interval
/// (30s) — long enough that a connect-then-instantly-drop flap keeps
/// accumulating toward the sustained threshold instead of resetting it on
/// every attempt, short enough that a genuinely recovered listener closes
/// the incident on its first good session.
const LISTENER_HEALTHY_SESSION_SECS: u64 = 30;

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

/// Listener supervisor: extended backoff once an outage has passed
/// `LISTENER_SUSTAINED_OUTAGE_SECS`. We don't give up — exiting would
/// leave the worker with no NOTIFY signal source, relying solely on the
/// 30s poll. We keep retrying but at a sustainable cadence so the operator
/// has time to investigate. Tied to the outage duration, NOT to whether a
/// page was sent: the cadence should slow because the outage is long, not
/// because Slack was told about it.
const LISTENER_BACKOFF_SUSTAINED_SECS: u64 = 60;

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

/// Env var gating the `legacy_stale` reception hint (ADR 0006). Ships DARK —
/// see [`legacy_stale_notify_enabled`]. Per-site: the `writeback-hfville`
/// compose service maps it from `HFVILLE_LEGACY_STALE_NOTIFY_ENABLED`, so the
/// two-service topology gives the canary rollout for free.
const LEGACY_STALE_NOTIFY_FLAG: &str = "LEGACY_STALE_NOTIFY_ENABLED";

/// Parse [`LEGACY_STALE_NOTIFY_FLAG`]. Truthy = `true` / `1` (trimmed,
/// case-insensitive); unset, empty, `false`, `0` or garbage ⇒ `false`.
///
/// Same liberal-on-input, default-OFF policy as `config::flag_enabled` (the
/// reader for every other ship-dark coexistence flag). Duplicated rather than
/// imported because that helper is private to `config.rs` and this binary
/// already parses its own env directly (`WRITEBACK_*` above).
fn legacy_stale_notify_enabled(raw: Option<String>) -> bool {
    match raw {
        Some(value) => {
            let normalized = value.trim().to_ascii_lowercase();
            normalized == "true" || normalized == "1"
        }
        None => false,
    }
}

/// Exponential backoff (in seconds) between retry attempts. Indexed by
/// `attempts` (0-based: backoff_secs(1) is the wait before attempt #2).
/// Caps at the last entry. Default schedule: 30s, 2min, 10min.
fn backoff_secs(attempts_so_far: i32) -> i64 {
    const BACKOFFS: &[i64] = &[30, 120, 600];
    let idx = (attempts_so_far as usize)
        .saturating_sub(1)
        .min(BACKOFFS.len() - 1);
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
    SITE_ID.get().map(String::as_str).unwrap_or("hfhotel")
}

// -----------------------------------------------------------------------
// Startup probes (collation / schema fingerprint / idempotency ledger)
//
// All three refuse to start the worker, and all three used to conflate two
// completely different situations:
//
//   * the probe's read came back and the ANSWER is bad — a real, permanent
//     configuration problem the operator has to fix;
//   * the probe never got an answer — legacy MSSQL slow or unreachable
//     (HF Ville over WireGuard at a quiet hour is the everyday case).
//
// The fingerprint probe was hardened for this in incident 2026-06-28: a
// timed-out catalog read that mis-reported as "schema drift" could lead an
// operator to re-baseline against a bad read. The collation and ledger
// probes still mapped ANY failure — including a bb8 pool timeout — onto
// "collation is case-sensitive" / "`dbo.ht_writeback_ledger` is missing",
// and they run FIRST, so on a tunnel blip they shadowed the hardened path
// entirely and sent the operator to re-collate a database or re-apply an
// already-applied migration. That is how "REFUSED TO START" gets learned as
// "probably the tunnel again" — including on the day it is real.
// -----------------------------------------------------------------------

/// Why a startup probe refused to let the worker boot. The distinction is
/// the whole point: it decides which of two mutually-exclusive stories the
/// Slack alert tells, and therefore whether the operator touches the legacy
/// database at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProbeFailureKind {
    /// The read COMPLETED and the value it returned is the bad one. Only
    /// here may the alert name the permanent cause and hand out a
    /// remediation that mutates legacy state.
    Confirmed,
    /// The read never completed, after every retry — timeout, pool
    /// exhaustion, dead tunnel. Nothing is known about the thing being
    /// probed, so the alert must say exactly that and tell the operator
    /// NOT to act on the permanent cause.
    Unreachable,
}

/// A startup probe's terminal failure, after retries.
struct ProbeFailure {
    kind: ProbeFailureKind,
    err: WritebackError,
    /// Attempts actually made — 1 for a confirmed failure (no point
    /// retrying a definite answer), `attempts` for an unreachable one.
    attempts: u32,
}

/// Default attempts per startup probe before refusing to start. Four
/// attempts with the 6/12/18s backoff below spans ~36s — longer than any
/// WireGuard re-handshake or `newdb`/legacy restart blip.
const DEFAULT_STARTUP_PROBE_ATTEMPTS: u32 = 4;

/// Attempt budget shared by all three startup probes.
/// `WRITEBACK_FINGERPRINT_ATTEMPTS` is still honoured — it was the
/// fingerprint probe's own knob before the other two got the same
/// treatment, and it may be set in a live `.env`.
fn startup_probe_attempts() -> u32 {
    parse_probe_attempts(
        env::var("WRITEBACK_STARTUP_PROBE_ATTEMPTS").ok().as_deref(),
        env::var("WRITEBACK_FINGERPRINT_ATTEMPTS").ok().as_deref(),
    )
}

/// Pure half of [`startup_probe_attempts`] — new name wins, old name is the
/// fallback, anything unparseable or `< 1` falls back to the default (a
/// zero would skip the probe entirely, which is not a thing we let an env
/// typo do).
fn parse_probe_attempts(primary: Option<&str>, legacy: Option<&str>) -> u32 {
    primary
        .or(legacy)
        .and_then(|v| v.trim().parse::<u32>().ok())
        .filter(|n| *n >= 1)
        .unwrap_or(DEFAULT_STARTUP_PROBE_ATTEMPTS)
}

/// Fingerprint probe: only a hash mismatch is a confirmed answer. Every
/// other failure means the catalog read itself did not land, and re-running
/// `writeback-fingerprint.sh` against a bad read is how you corrupt the
/// baseline (incident 2026-06-28).
fn fingerprint_failure_is_confirmed(err: &WritebackError) -> bool {
    matches!(err, WritebackError::SchemaDrift { .. })
}

/// Catalog probes (collation + idempotency ledger): both issue one tiny
/// `SELECT` and turn its ANSWER into `WritebackError::Config`. Every
/// transport failure — `pool.get()`, the per-op timeout, a driver error —
/// arrives as `Pool` / `Tiberius` / `Sqlx` instead, and means the answer
/// was never seen.
///
/// Written as an exhaustive match, not a `matches!`, so a new
/// `WritebackError` variant fails the build here and forces someone to
/// decide which side of this line it falls on.
fn catalog_probe_failure_is_confirmed(err: &WritebackError) -> bool {
    match err {
        // The SELECT came back; `Config` carries what it said (a `_CS_`
        // collation name, a NULL `OBJECT_ID`, an unreadable result shape).
        WritebackError::Config(_) => true,
        // The read never completed. Retry, then report connectivity.
        WritebackError::Tiberius(_) | WritebackError::Pool(_) | WritebackError::Sqlx(_) => false,
        // Not producible by either probe today. Fail loud rather than
        // retry-then-blame-the-network on something deterministic.
        WritebackError::SchemaDrift { .. }
        | WritebackError::Disabled
        | WritebackError::IntentMismatch(_)
        | WritebackError::Recipe(_)
        | WritebackError::Serde(_) => true,
    }
}

/// Run one startup probe, retrying transient failures with a 6/12/18s
/// backoff, and classify the terminal failure.
///
/// `is_confirmed` decides what "transient" means for this probe — it is
/// per-probe on purpose: a `Config` error is a definite answer from the
/// collation/ledger probes but merely a malformed catalog read from the
/// fingerprint probe, which retries it.
///
/// A confirmed failure short-circuits: re-asking a question that already
/// has a definite answer only delays the page.
async fn run_startup_probe<F, Fut>(
    site_id: &str,
    label: &str,
    attempts: u32,
    is_confirmed: fn(&WritebackError) -> bool,
    mut probe: F,
) -> Result<(), ProbeFailure>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<(), WritebackError>>,
{
    let attempts = attempts.max(1);
    // Overwritten on the first failing attempt; `attempts >= 1` makes the
    // sentinel unreachable, but it keeps this function panic-free.
    let mut last = WritebackError::Config(format!("{label}: probe never ran"));
    for attempt in 1..=attempts {
        match probe().await {
            Ok(()) => return Ok(()),
            Err(e) if is_confirmed(&e) => {
                return Err(ProbeFailure {
                    kind: ProbeFailureKind::Confirmed,
                    err: e,
                    attempts: attempt,
                });
            }
            Err(e) => {
                if attempt < attempts {
                    let backoff = Duration::from_secs((attempt as u64) * 6);
                    tracing::warn!(
                        site = %site_id,
                        probe = label,
                        attempt,
                        attempts,
                        backoff_secs = backoff.as_secs(),
                        error = %e,
                        "Startup probe read failed (transient) — retrying before refusing"
                    );
                    tokio::time::sleep(backoff).await;
                }
                last = e;
            }
        }
    }
    Err(ProbeFailure {
        kind: ProbeFailureKind::Unreachable,
        err: last,
        attempts,
    })
}

/// Slack body for the collation probe (W1).
///
/// The `Confirmed` branch deliberately does not assert *which* bad answer
/// came back — `verify_legacy_collation_safety` also fails when the probe
/// returns no rows or an unreadable column, and the error text says which.
/// What it does assert, and what the old unconditional message could not,
/// is that an answer WAS received: the operator is looking at a real
/// configuration problem, not at the tunnel.
fn collation_probe_alert_body(kind: ProbeFailureKind, attempts: u32, err: &str) -> String {
    match kind {
        ProbeFailureKind::Confirmed => format!(
            ":warning: *Writeback worker REFUSED TO START* :warning:\n\
             Legacy MSSQL collation check FAILED — the probe read succeeded, so this \
             is a real configuration problem, not connectivity.\n\
             *Error:* `{err}`\n\
             _Recipes pin every string literal to the case iHOTEL emits, so a \
             case-sensitive (`_CS_`) collation silently forks our SQL filters. \
             Expected `Thai_CI_AS` (or any `_CI_` collation) — restore the legacy DB \
             with a case-insensitive collation before retrying._"
        ),
        ProbeFailureKind::Unreachable => format!(
            ":warning: *Writeback worker could not start* :warning:\n\
             Could NOT read the legacy server collation after {attempts} attempts \
             (legacy MSSQL slow/unreachable — e.g. HF Ville over WireGuard).\n\
             *Error:* `{err}`\n\
             _This is a connectivity/timeout problem, NOT a collation problem — the \
             collation was never read, so nothing is known about it. Do NOT re-collate \
             or restore the legacy DB. The worker retries on restart; check the site \
             server / WireGuard if it persists._"
        ),
    }
}

/// Slack body for the schema-fingerprint probe (W3). Both texts are the
/// ones this probe already sent — it was the one guard that got this right,
/// and the other two are now modelled on it.
fn fingerprint_probe_alert_body(kind: ProbeFailureKind, attempts: u32, err: &str) -> String {
    match kind {
        ProbeFailureKind::Confirmed => format!(
            ":warning: *Writeback worker REFUSED TO START* :warning:\n\
             Legacy MSSQL schema fingerprint MISMATCH (real drift).\n\
             *Error:* `{err}`\n\
             _The legacy DB columns drifted from the captured baseline. \
             Run_ `./scripts/writeback-fingerprint.sh` _and follow the \
             README to update the baseline before restarting the worker._"
        ),
        ProbeFailureKind::Unreachable => format!(
            ":warning: *Writeback worker could not start* :warning:\n\
             Could NOT read the legacy schema after {attempts} attempts \
             (legacy MSSQL slow/unreachable — e.g. HF Ville over WireGuard).\n\
             *Error:* `{err}`\n\
             _This is a connectivity/timeout problem, NOT confirmed schema \
             drift — do NOT run `writeback-fingerprint.sh`. The worker retries \
             on restart; check the site server / WireGuard if it persists._"
        ),
    }
}

/// Slack body for the idempotency-ledger probe (W4).
fn ledger_probe_alert_body(kind: ProbeFailureKind, attempts: u32, err: &str) -> String {
    match kind {
        ProbeFailureKind::Confirmed => format!(
            ":warning: *Writeback worker REFUSED TO START* :warning:\n\
             Legacy idempotency ledger `dbo.ht_writeback_ledger` is MISSING — the probe \
             read succeeded and `OBJECT_ID` came back NULL.\n\
             *Error:* `{err}`\n\
             _Apply_ `migrations/legacy-mssql/024_writeback_ledger.sql` _(the deploy runs_ \
             `scripts/migrate-legacy-mssql.sh` _automatically — check its output / \
             `dbo.ht_legacy_migrations`). It is the crash-after-commit duplicate guard for \
             create recipes; the worker will not run without it._"
        ),
        ProbeFailureKind::Unreachable => format!(
            ":warning: *Writeback worker could not start* :warning:\n\
             Could NOT probe for `dbo.ht_writeback_ledger` after {attempts} attempts \
             (legacy MSSQL slow/unreachable — e.g. HF Ville over WireGuard).\n\
             *Error:* `{err}`\n\
             _This is a connectivity/timeout problem, NOT a missing table — the probe \
             never got an answer, so the ledger's presence is UNKNOWN. Do NOT re-apply_ \
             `024_writeback_ledger.sql` _chasing this; it is almost certainly already \
             applied (check `dbo.ht_legacy_migrations`). The worker retries on restart; \
             check the site server / WireGuard if it persists._"
        ),
    }
}

/// Shared fail-loud envelope for a startup probe that refused the boot:
/// log, post `body` to Slack, then sleep before exiting so Docker's
/// `restart: unless-stopped` backs off instead of re-paging 6×/min.
/// Returns the string `main` bubbles up as its error.
async fn refuse_to_start(
    slack: &Option<SlackClient>,
    site_id: &str,
    what: &str,
    failure: &ProbeFailure,
    body: String,
) -> String {
    tracing::error!(
        site = %site_id,
        probe = what,
        error = %failure.err,
        confirmed = failure.kind == ProbeFailureKind::Confirmed,
        attempts = failure.attempts,
        "Startup probe failed — refusing to start"
    );
    if let Some(slack) = slack {
        let _ = slack
            .send_message(&SlackMessage::with_site_text(site_id, body))
            .await;
    }
    tracing::warn!(
        site = %site_id,
        "Sleeping 60s before exit to throttle Docker restart cadence \
         and avoid Slack alert flood"
    );
    tokio::time::sleep(Duration::from_secs(60)).await;
    format!("{what} failed: {}", failure.err)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    hotel_backend::secrets::hydrate_env_from_secret_files();
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

    // ADR 0006 — ship DARK. When off, nothing is built and nothing is
    // notified; the drain loop behaves exactly as before. Read once here so
    // the hot path never touches the environment.
    let stale_notify = legacy_stale_notify_enabled(env::var(LEGACY_STALE_NOTIFY_FLAG).ok());

    tracing::info!(
        poll_interval_secs = poll_interval,
        max_attempts,
        legacy_stale_notify = stale_notify,
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

    // 4b0. All three startup probes share one retry + classification
    //      envelope (see `run_startup_probe`). Each retries a read that did
    //      not land, and only names its permanent cause when the read
    //      actually came back with a bad answer.
    let probe_attempts = startup_probe_attempts();
    let mssql_probe = &mssql;

    // 4b1. Wave 6 LOW item 8 — Ville cutover safety: refuse to start on a
    //      case-sensitive collation. Recipes pin every string literal to
    //      the case the .NET app emits; a `_CS_` collation would silently
    //      fork our SQL filters on a fresh Ville cutover. Cheap one-row
    //      SELECT — runs before the fingerprint check so a misconfigured
    //      Ville fails fast at startup.
    //
    //      Because it runs FIRST, an unclassified failure here shadows the
    //      fingerprint probe's careful transient/permanent split: a pool
    //      timeout on the very first legacy round-trip of the process used
    //      to be reported as "collation is case-sensitive".
    if let Err(f) = run_startup_probe(
        &site.id,
        "legacy collation",
        probe_attempts,
        catalog_probe_failure_is_confirmed,
        || verify_legacy_collation_safety(mssql_probe),
    )
    .await
    {
        let body = collation_probe_alert_body(f.kind, f.attempts, &f.err.to_string());
        return Err(refuse_to_start(&slack, &site.id, "Legacy collation check", &f, body)
            .await
            .into());
    }

    // 4b. Schema fingerprint guard — refuse to start on drift, but post
    //     a Slack alert first so the operator sees the failure even if
    //     they're not tailing logs. Sleep before returning so the Docker
    //     `restart: unless-stopped` policy backs off (without the sleep,
    //     the worker exits in ms, restarts, fingerprint fails again, fires
    //     another Slack — operator gets paged 6×/min until they intervene).
    // Retry transient read failures before refusing. A timed-out / I/O-failed
    // catalog SELECT (legacy MSSQL slow or unreachable — e.g. HF Ville over
    // WireGuard at a quiet hour) is NOT confirmed schema drift: only a real
    // `SchemaDrift` (read succeeded, hash differs) should tell the operator to
    // run `writeback-fingerprint.sh`. Mis-firing that on a timeout could lead to
    // updating the baseline against a bad read and corrupting it (incident
    // 2026-06-28).
    if let Err(f) = run_startup_probe(
        &site.id,
        "schema fingerprint",
        probe_attempts,
        fingerprint_failure_is_confirmed,
        || verify_schema_fingerprint(mssql_probe),
    )
    .await
    {
        let body = fingerprint_probe_alert_body(f.kind, f.attempts, &f.err.to_string());
        return Err(
            refuse_to_start(&slack, &site.id, "Schema fingerprint check", &f, body)
                .await
                .into(),
        );
    }

    // 4c. Idempotency-ledger guard — refuse to start if dbo.ht_writeback_ledger
    //     is absent. Without it, the crash-after-commit duplicate protection for
    //     the create recipes is silently gone (a missing table reads as a
    //     retryable Tiberius error -> retry-storm with zero protection). Same
    //     fail-loud + Slack + throttle-sleep envelope as the fingerprint guard.
    if let Err(f) = run_startup_probe(
        &site.id,
        "writeback ledger",
        probe_attempts,
        catalog_probe_failure_is_confirmed,
        || verify_writeback_ledger_exists(mssql_probe),
    )
    .await
    {
        let body = ledger_probe_alert_body(f.kind, f.attempts, &f.err.to_string());
        return Err(
            refuse_to_start(&slack, &site.id, "Writeback ledger check", &f, body)
                .await
                .into(),
        );
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
    let janitor_mssql = mssql.clone();
    let janitor_slack = slack.clone();
    let janitor_shutdown = shutdown.clone();
    tokio::spawn(async move {
        run_queue_depth_janitor(janitor_pg, janitor_mssql, janitor_slack, janitor_shutdown).await;
    });

    // Task #69: wrap the main loop in a tracing span so every log line
    // emitted from inside the worker (job claim, dispatch outcome,
    // panic recovery) carries `site=<id>`. Same purpose as the watcher
    // span in `bin/sync.rs`.
    let worker_span = tracing::info_span!("writeback_worker", site = %site.id);
    let _worker_guard = worker_span.enter();

    // 6. Main loop — process jobs whenever NOTIFY wakes us OR every poll_interval
    loop {
        // ADR 0006 — one `legacy_stale` hint per DRAIN TICK, not per job.
        // No timer is needed because the inner loop already runs to queue
        // exhaustion: a booking that emits 3 intents enqueues all 3 rows in
        // one PG transaction, the notify trigger wakes us once, and all 3
        // drain here ⇒ one signal with `count: 3`. A slow trickle produces
        // one signal each, which is correct — those ARE separate events, and
        // suppressing them into one toast is the middleware latch's job.
        //
        // Stays empty whenever `LEGACY_STALE_NOTIFY_ENABLED` is off:
        // `process_job` doesn't even build a note then.
        let mut stale_notes: Vec<StaleNote> = Vec::new();

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
                        process_job(
                            &pg_inner,
                            &mssql_inner,
                            max_attempts,
                            &slack_inner,
                            job,
                            stale_notify,
                        )
                        .await
                    })
                    .await;
                    // The spawn wrapper's `Ok` value used to be discarded; it
                    // now carries the reception hint for jobs that landed.
                    // Borrowed so the panic-recovery arm below is untouched.
                    if let Ok(Some(note)) = &result {
                        stale_notes.push(note.clone());
                    }
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

        // The queue is drained (or the claim query broke — the jobs that DID
        // land are still worth announcing). One coalesced hint, then back to
        // waiting. A failure here must never fail or retry a job whose MSSQL
        // transaction already committed: log a warn and carry on.
        if !stale_notes.is_empty() {
            let signal = legacy_stale::coalesce(current_site_id(), &stale_notes);
            tracing::debug!(
                count = signal.count(),
                summary = signal.summary(),
                "Publishing legacy_stale hint for this drain tick"
            );
            if let Err(err) = legacy_stale::publish(&pg, &signal).await {
                tracing::warn!(
                    error = %err,
                    count = signal.count(),
                    "legacy_stale notify failed; the legacy writes are committed \
                     and unaffected — reception just won't be told this time"
                );
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

/// What the queue row looked like in the instant BEFORE this claim flipped
/// it to `in_progress`.
///
/// **Why this has to be captured at claim time.** `mark_done` used to read
/// the "prior" status itself, via a `WITH prev AS (SELECT status …)` CTE on
/// its own UPDATE. That never worked: `claim_next_job` commits the flip to
/// `in_progress` in an EARLIER statement, so by the time `mark_done` runs
/// the pre-image is long gone and its CTE could only ever observe
/// `in_progress` (and its UPDATE gate requires exactly that anyway). The
/// `exhausted → done` branch was unreachable, which silently killed the
/// `:white_check_mark:` closure alert for the single most actionable page
/// in this binary. The claim statement is the last place the pre-image
/// exists, so it is captured there and carried in memory.
///
/// **Why this is a classification, not a raw status string.** An
/// `exhausted` row is terminal — `claim_next_job` deliberately never
/// selects one (operator triage is the only way out). The documented
/// recovery, printed in `send_exhausted_alert`'s own remediation text, is
/// `UPDATE writeback_jobs SET status='pending', attempts=0,
/// next_retry_at=NULL`. So even at claim time the literal status of a
/// recovered job reads `pending`, never `exhausted`, and a naive
/// `prior_status == "exhausted"` comparison would stay dead. We recognise
/// the *shape* the operator's reset leaves behind instead — see
/// [`classify_prior_disposition`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PriorDisposition {
    /// Enqueued and never attempted: `pending` with no error residue.
    Fresh,
    /// A normal retry — either `failed` with a scheduled `next_retry_at`,
    /// or a stale `in_progress` claim stolen back from a crashed worker.
    /// No operator was ever paged for this row.
    Retrying,
    /// The row had reached the terminal `exhausted` state — meaning
    /// `send_exhausted_alert` paged an operator for it — and was put back
    /// into the queue by hand. Success on this attempt is the closure of
    /// that page.
    RecoveredFromExhausted,
}

/// Classify the claim-time pre-image of a queue row. Pure — the whole
/// point is that the `exhausted → done` transition can be unit-tested
/// without a database.
///
/// Recognised shapes:
///
/// * `exhausted` — a literal terminal pre-image. Unreachable while
///   `claim_next_job` excludes the state, but classified correctly so the
///   detection does not silently die again if that predicate is ever
///   widened.
/// * `pending` + carries a `last_error` + has NO scheduled retry — the
///   fingerprint of an operator reset from `exhausted`, and it is
///   unambiguous in this schema:
///     - enqueue INSERTs `pending` with `last_error` NULL (migration 011
///       default), so a fresh row never carries an error;
///     - `mark_failed`'s non-terminal branch always writes
///       `next_retry_at = NOW() + backoff`, so a retrying row always has
///       one scheduled AND sits in `failed`, not `pending`;
///     - only `mark_failed`'s terminal branch and `force_exhaust_job`
///       produce (`last_error` set, `next_retry_at` NULL) — and both write
///       `exhausted`;
///     - nothing in this codebase writes `pending` after the initial
///       INSERT.
///
///   Residual over-fire: an operator who hand-resets a merely `failed` row
///   with the same SQL gets a closure alert without a preceding
///   `:rotating_light:`. The statement it makes ("a job that was in an
///   error state has now succeeded") is still true, so this is left as-is.
/// * anything else — an ordinary retry.
fn classify_prior_disposition(
    prior_status: &str,
    prior_had_error: bool,
    prior_retry_scheduled: bool,
) -> PriorDisposition {
    match prior_status {
        "exhausted" => PriorDisposition::RecoveredFromExhausted,
        "pending" if prior_had_error && !prior_retry_scheduled => {
            PriorDisposition::RecoveredFromExhausted
        }
        "pending" => PriorDisposition::Fresh,
        _ => PriorDisposition::Retrying,
    }
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
    /// Frozen at enqueue (`writeback_jobs.idempotency_key`, immutable —
    /// `claim_next_job` mutates only status/attempts/claimed_at). This is the
    /// key the legacy idempotency ledger (`dbo.ht_writeback_ledger`) is keyed
    /// on. MUST be the STORED value, never a recomputed `uuid5(intent,
    /// aggregate_id)`: ExtendStay/RoomChange enqueue a per-event RANDOM
    /// discriminator key, so recomputing would false-skip distinct events that
    /// share an aggregate.
    idempotency_key: Uuid,
    attempts: i32,
    claimed_at: DateTime<Utc>,
    /// The row's state in the instant before THIS claim flipped it to
    /// `in_progress`, captured from the claim statement's own pre-image.
    /// Threaded to `mark_done` so it can detect the `exhausted → done`
    /// recovery and post the closure alert. See [`PriorDisposition`] for
    /// why it cannot be re-read later.
    prior: PriorDisposition,
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
///
/// **Pre-image capture.** The victim sub-select is a CTE rather than a bare
/// scalar sub-query so it can also project the row's PRE-claim
/// `status` / `last_error` / `next_retry_at`. Every CTE and the main query
/// share one statement snapshot, so `victim` sees the row as it was BEFORE
/// this UPDATE's own flip to `in_progress` — the last point at which that
/// state is observable. `mark_done` cannot re-derive it (the flip has
/// committed by then), so it is classified here and carried on
/// `ClaimedJob`. See [`PriorDisposition`].
async fn claim_next_job(pg: &PgPool, max_attempts: i32) -> Result<Option<ClaimedJob>, sqlx::Error> {
    let row = sqlx::query(
        r#"
        WITH victim AS (
            SELECT id,
                   status                      AS prior_status,
                   (last_error IS NOT NULL)    AS prior_had_error,
                   (next_retry_at IS NOT NULL) AS prior_retry_scheduled
              FROM writeback_jobs
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
        UPDATE writeback_jobs wj
           SET status     = 'in_progress',
               attempts   = wj.attempts + 1,
               claimed_at = NOW()
          FROM victim v
         WHERE wj.id = v.id
        RETURNING wj.id, wj.intent, wj.payload, wj.aggregate_id, wj.idempotency_key,
                  wj.attempts, wj.claimed_at,
                  v.prior_status, v.prior_had_error, v.prior_retry_scheduled
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
    let idempotency_key: Uuid = row.try_get("idempotency_key")?;
    let attempts: i32 = row.try_get("attempts")?;
    // claimed_at is set by this very UPDATE (NOW()); guaranteed NOT NULL
    // on the returned row. Used downstream to gate mark_done / mark_failed
    // against a parallel janitor steal (audit MED-2).
    let claimed_at: DateTime<Utc> = row.try_get("claimed_at")?;

    // Pre-claim state, projected by the `victim` CTE from the same snapshot
    // (i.e. before this statement's own flip). Classified here because this
    // is the last place the information exists — `mark_done` runs after the
    // flip has committed and can only ever see `in_progress`.
    let prior_status: String = row.try_get("prior_status")?;
    let prior_had_error: bool = row.try_get("prior_had_error")?;
    let prior_retry_scheduled: bool = row.try_get("prior_retry_scheduled")?;
    let prior = classify_prior_disposition(&prior_status, prior_had_error, prior_retry_scheduled);
    if prior == PriorDisposition::RecoveredFromExhausted {
        tracing::info!(
            job_id = id,
            prior_status = %prior_status,
            "Claimed a job that was previously exhausted (operator reset) — \
             a closure alert will fire if this attempt succeeds"
        );
    }

    // Deserialize payload into the matching variant. The JSON shape is
    // produced by `serde(tag = "intent", content = "payload")` — the queue's
    // separate `intent` column is what the dispatcher uses, but the JSON
    // includes both for round-trip safety.
    let intent: WritebackIntent = serde_json::from_value(payload).map_err(|e| {
        sqlx::Error::Decode(format!("payload deserialize for job {id} ({intent_name}): {e}").into())
    })?;

    Ok(Some(ClaimedJob {
        id,
        intent,
        aggregate_id,
        idempotency_key,
        attempts,
        claimed_at,
        prior,
    }))
}

/// Process one claimed job: open MSSQL conn, dispatch, persist outcome.
///
/// Returns `Some(note)` exactly when a row landed in legacy MSSQL **and** this
/// worker owned the completion (see [`mark_done`]'s return contract) — the
/// drain loop accumulates those and publishes one coalesced `legacy_stale`
/// signal per tick (ADR 0006). Every failure path returns `None`.
///
/// `stale_notify` is the `LEGACY_STALE_NOTIFY_ENABLED` flag, read once at
/// startup and threaded in so that when the feature is dark we don't even
/// build the label.
async fn process_job(
    pg: &PgPool,
    mssql: &DbPool,
    max_attempts: i32,
    slack: &Option<SlackClient>,
    job: ClaimedJob,
    stale_notify: bool,
) -> Option<StaleNote> {
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
            return None;
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
            return None;
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
        return None;
    }

    let ctx = DispatchContext {
        job_id,
        aggregate_id: job.aggregate_id,
        idempotency_key: job.idempotency_key,
    };

    let outcome = run_in_transaction(&mut conn, &job.intent, &resolved, ctx).await;
    drop(conn); // release back to pool before any further awaits

    match outcome {
        Ok(legacy_ids) => {
            tracing::info!(job_id, intent = intent_name, "Writeback succeeded");
            // Built BEFORE `into_json()` consumes `legacy_ids`. The room number
            // is whatever we already have in hand — the recipe-minted one wins
            // (walk-in / checkin-to-booking allocate it), falling back to the
            // one the resolver read from PG before dispatch. No extra query.
            let note = stale_notify.then(|| {
                let room_no = legacy_ids
                    .room_no
                    .as_deref()
                    .or(resolved.legacy_room_no.as_deref());
                StaleNote::for_intent(&job.intent, room_no)
            });
            let landed = mark_done(
                pg,
                job_id,
                job.claimed_at,
                job.aggregate_id,
                &job.intent,
                slack,
                job.prior,
                legacy_ids.into_json(),
            )
            .await;
            // A stolen claim means the OTHER worker owns this job's completion
            // and will emit its own hint — see `mark_done`'s return contract.
            if landed {
                note
            } else {
                None
            }
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
            None
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
            match simple_query_with_timeout_drop(conn, "ROLLBACK TRAN", MssqlOpKind::Write).await {
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
            if let Some(row) =
                sqlx::query("SELECT legacy_book_id FROM ht_bookings WHERE aggregate_id = $1")
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
            // #75 — PER-ROOM (partial) checkout: a CheckOut intent carrying a
            // `cr_id` releases ONE room of a multi-room stay. Override the
            // header's single room_no + HT_CheckIn_Ds.id with THAT room's values
            // (cin_no stays the header's). Mirrors the RefundDeposit per-cr
            // resolution. We RESET both first so a lookup miss OR a NULL
            // cr_legacy_ds_id (the room's CreateCheckIn writeback hasn't
            // back-populated it yet) leaves them None — the dispatcher's
            // `ok_or_else` then EXHAUSTS the job (Recipe error is non-retryable
            // → dead-letter + Slack "manual intervention"), the correct
            // fail-loud, and never falls back to mis-writing the header's room.
            if let CheckOut {
                cr_id: Some(cr_id), ..
            } = &job.intent
            {
                resolved.legacy_room_no = None;
                resolved.legacy_checkin_ds_id = None;
                if let Some(row) = sqlx::query(
                    "SELECT cr.cr_legacy_ds_id, r.room_no \
                     FROM ht_checkin_rooms cr JOIN ht_rooms_new r ON r.room_id = cr.cr_room_id \
                     WHERE cr.cr_id = $1",
                )
                .bind(*cr_id)
                .fetch_optional(pg)
                .await?
                {
                    resolved.legacy_room_no = row.try_get("room_no").ok();
                    resolved.legacy_checkin_ds_id = row.try_get("cr_legacy_ds_id").ok();
                }
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
                if let Some(row) =
                    sqlx::query("SELECT legacy_pay_no FROM ht_payments WHERE aggregate_id = $1")
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
                    room_before_price_baht: row.try_get("room_before_price_baht").unwrap_or(0.0),
                    to_price: row.try_get("to_price").unwrap_or_default(),
                });
            }
        }
        // MarkRoomDirty (audit 2026-06-11 P2) and SetRoomMaintenance share
        // MarkRoomClean's resolution query: all three key `HT_Rooms` by
        // the numeric internal `id` (spike §3j critical finding), fetched
        // here as `legacy_room_id_int`. Only `MarkRoomClean` still needs
        // the display `room_no` too, for its `HT_Housewife` audit row +
        // prior-occupant lookup (mark_clean.rs). Since issue #276,
        // `MarkRoomDirty` no longer writes an `HT_Housewife` row or does
        // a prior-occupant lookup — iHOTEL itself never inserts one on a
        // standalone dirty flip (mark_dirty.rs module doc) — so its
        // `room_no` is resolved here (cheap, shared query) but unused by
        // the recipe; `SetRoomMaintenance` never needed `room_no` at all
        // (set_maintenance.rs keys by `id` only).
        MarkRoomClean { room_id, .. }
        | MarkRoomDirty { room_id, .. }
        | SetRoomMaintenance { room_id, .. } => {
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
        // Admin room master-data edit. The payload carries the legacy
        // `Room_no` business key directly (resolved by the route layer
        // before enqueue), so no PG lookup or self-heal is required at
        // this layer. Mirrors the `AdjustProductStock` resolution shape.
        UpdateRoom { .. } => {}
        // Board-move payloads carry `Room_no` business keys directly —
        // nothing to resolve. Mirrors `UpdateRoom`.
        MoveRoomTiles { .. } => {}
        // Audit 2026-06-11 P2 — standalone customer-edit re-save. The
        // payload's `resave.legacy_cust_no` carries the `Cust_no`
        // business key directly (hydrated by `service::customer::update`
        // before enqueue — the intent is only emitted for customers that
        // already exist on the legacy side). Mirrors `UpdateRoom`.
        UpdateCustomer { .. } => {}
        // Track G6 — `RecordPosSale`. Load the canonical `ht_pos_sales`
        // row joined with `ht_products` so the recipe consumes plain
        // fields (no sqlx). The check-in identifiers (`cin_no`, room)
        // come from the parent `ht_checkins` row, resolved here too
        // because the intent only carries the parent aggregate id.
        RecordPosSale { sale_id, .. } => {
            use hotel_backend::writeback::dispatcher::ResolvedPosSale;
            if let Some(row) = sqlx::query(
                "SELECT \
                    s.sale_id, \
                    COALESCE(c.legacy_cin_no, '') AS legacy_cin_no, \
                    COALESCE(c.legacy_room_no, '') AS legacy_room_no, \
                    COALESCE(p.prod_legacy_no, '') AS prod_legacy_no, \
                    COALESCE(p.prod_name, '')      AS prod_name, \
                    COALESCE(p.prod_unit, '')      AS prod_unit, \
                    s.sale_qty::float8             AS qty, \
                    s.sale_unit_price::float8      AS unit_price_baht, \
                    s.sale_total::float8           AS total_baht, \
                    COALESCE(s.sale_note, '')      AS note, \
                    s.sale_sold_at \
                 FROM ht_pos_sales s \
                 JOIN ht_checkins  c ON c.cin_id  = s.sale_cin_id \
                 JOIN ht_products  p ON p.prod_id = s.sale_product_id \
                 WHERE s.sale_id = $1",
            )
            .bind(*sale_id)
            .fetch_optional(pg)
            .await?
            {
                resolved.pos_sale = Some(ResolvedPosSale {
                    sale_id: row.try_get("sale_id").unwrap_or(*sale_id),
                    legacy_cin_no: row.try_get("legacy_cin_no").unwrap_or_default(),
                    legacy_room_no: row.try_get("legacy_room_no").unwrap_or_default(),
                    prod_legacy_no: row.try_get("prod_legacy_no").unwrap_or_default(),
                    prod_name: row.try_get("prod_name").unwrap_or_default(),
                    prod_unit: row.try_get("prod_unit").unwrap_or_default(),
                    qty: row.try_get("qty").unwrap_or(0.0),
                    unit_price_baht: row.try_get("unit_price_baht").unwrap_or(0.0),
                    total_baht: row.try_get("total_baht").unwrap_or(0.0),
                    note: row.try_get("note").unwrap_or_default(),
                    sold_at: row.try_get("sale_sold_at").unwrap_or_else(|_| Utc::now()),
                });
            }
        }
        // Task #45 — RecordReceipt. Load the canonical `ht_pos_receipts`
        // header + its `ht_pos_receipt_lines` (the line snapshot already
        // carries `line_product_no`/`name`/`unit_name`, so no join to
        // `ht_products` is needed). The recipe consumes plain fields only.
        RecordReceipt { receipt_id, .. } => {
            use hotel_backend::writeback::dispatcher::{ResolvedReceipt, ResolvedReceiptLine};
            if let Some(h) = sqlx::query(
                "SELECT \
                    receipt_id, \
                    COALESCE(receipt_customer_no, 'C0000') AS customer_no, \
                    COALESCE(receipt_customer_name, '')    AS customer_name, \
                    COALESCE(receipt_customer_addr, '')    AS customer_addr, \
                    COALESCE(receipt_customer_tel, '')     AS customer_tel, \
                    COALESCE(receipt_tax_id, '')           AS tax_id, \
                    receipt_total::float8                  AS total_baht, \
                    receipt_discount::float8               AS discount_baht, \
                    receipt_vat_percent                    AS vat_percent, \
                    COALESCE(receipt_note, '')             AS note, \
                    receipt_sold_at \
                 FROM ht_pos_receipts WHERE receipt_id = $1",
            )
            .bind(*receipt_id)
            .fetch_optional(pg)
            .await?
            {
                let line_rows = sqlx::query(
                    "SELECT \
                        COALESCE(line_product_no, '')   AS prod_legacy_no, \
                        COALESCE(line_product_name, '') AS prod_name, \
                        COALESCE(line_unit_name, '')    AS unit_name, \
                        line_qty::float8                AS qty, \
                        line_unit_price::float8         AS unit_price_baht, \
                        line_total::float8              AS total_baht, \
                        line_discount::float8           AS discount_baht \
                     FROM ht_pos_receipt_lines \
                     WHERE line_receipt_id = $1 \
                     ORDER BY line_id",
                )
                .bind(*receipt_id)
                .fetch_all(pg)
                .await?;
                let lines: Vec<ResolvedReceiptLine> = line_rows
                    .into_iter()
                    .map(|r| ResolvedReceiptLine {
                        prod_legacy_no: r.try_get("prod_legacy_no").unwrap_or_default(),
                        prod_name: r.try_get("prod_name").unwrap_or_default(),
                        unit_name: r.try_get("unit_name").unwrap_or_default(),
                        qty: r.try_get("qty").unwrap_or(0.0),
                        unit_price_baht: r.try_get("unit_price_baht").unwrap_or(0.0),
                        total_baht: r.try_get("total_baht").unwrap_or(0.0),
                        discount_baht: r.try_get("discount_baht").unwrap_or(0.0),
                    })
                    .collect();
                resolved.receipt = Some(ResolvedReceipt {
                    receipt_id: h.try_get("receipt_id").unwrap_or(*receipt_id),
                    customer_no: h.try_get("customer_no").unwrap_or_default(),
                    customer_name: h.try_get("customer_name").unwrap_or_default(),
                    customer_address: h.try_get("customer_addr").unwrap_or_default(),
                    customer_tel: h.try_get("customer_tel").unwrap_or_default(),
                    tax_id: h.try_get("tax_id").unwrap_or_default(),
                    total_baht: h.try_get("total_baht").unwrap_or(0.0),
                    discount_baht: h.try_get("discount_baht").unwrap_or(0.0),
                    vat_percent: h.try_get("vat_percent").unwrap_or(0),
                    note: h.try_get("note").unwrap_or_default(),
                    sold_at: h.try_get("receipt_sold_at").unwrap_or_else(|_| Utc::now()),
                    lines,
                });
            }
        }
        // Task #45 — VoidPosSale. Load the sale's back-populated
        // `sale_legacy_id` (== `HT_CheckIn_Product.id`) + the joined
        // `prod_legacy_no` fallback. `legacy_id = None` ⇒ the dispatcher
        // defers the void until the original RecordPosSale back-populated it.
        VoidPosSale { sale_id, .. } => {
            use hotel_backend::writeback::dispatcher::ResolvedPosVoid;
            if let Some(row) = sqlx::query(
                "SELECT \
                    s.sale_id, \
                    s.sale_legacy_id, \
                    COALESCE(p.prod_legacy_no, '') AS prod_legacy_no \
                 FROM ht_pos_sales s \
                 JOIN ht_products  p ON p.prod_id = s.sale_product_id \
                 WHERE s.sale_id = $1",
            )
            .bind(*sale_id)
            .fetch_optional(pg)
            .await?
            {
                resolved.pos_void = Some(ResolvedPosVoid {
                    sale_id: row.try_get("sale_id").unwrap_or(*sale_id),
                    legacy_id: row
                        .try_get::<Option<i32>, _>("sale_legacy_id")
                        .unwrap_or(None),
                    prod_legacy_no: row.try_get("prod_legacy_no").unwrap_or_default(),
                });
            }
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
                if let Some(row) =
                    sqlx::query("SELECT legacy_book_id FROM ht_bookings WHERE aggregate_id = $1")
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
        // Track G5 — IssueCoupon / RedeemCoupon load the full
        // canonical `ht_coupons` row keyed by `coupon_id`, then thread
        // the legacy folio + room context through to the recipe.
        IssueCoupon { coupon_id, .. } | RedeemCoupon { coupon_id, .. } => {
            use hotel_backend::writeback::dispatcher::ResolvedCoupon;
            if let Some(row) = sqlx::query(
                "SELECT \
                    c.coupon_id, \
                    c.legacy_cupon_no, \
                    COALESCE(c.coupon_for_cin_no, '') AS coupon_for_cin_no, \
                    COALESCE(ck.legacy_room_no, '')   AS coupon_for_room_no, \
                    c.coupon_issued_at, \
                    COALESCE(c.coupon_issued_by, '')  AS issued_by \
                 FROM ht_coupons c \
                 LEFT JOIN ht_checkins ck \
                        ON ck.legacy_cin_no = c.coupon_for_cin_no \
                 WHERE c.coupon_id = $1",
            )
            .bind(*coupon_id)
            .fetch_optional(pg)
            .await?
            {
                resolved.coupon = Some(ResolvedCoupon {
                    coupon_id: row.try_get("coupon_id").unwrap_or(*coupon_id),
                    legacy_cupon_no: row.try_get("legacy_cupon_no").ok(),
                    coupon_for_cin_no: row.try_get("coupon_for_cin_no").unwrap_or_default(),
                    coupon_for_room_no: row.try_get("coupon_for_room_no").unwrap_or_default(),
                    issued_at: row
                        .try_get("coupon_issued_at")
                        .unwrap_or_else(|_| Utc::now()),
                    issued_by: row.try_get("issued_by").unwrap_or_default(),
                });
            }
        }
        // Track J6 — OpenRound / CloseRound carry the full `HT_Round_Bill`
        // shape in their payload (explicit legacy id, price, cashier,
        // timestamp). No canonical `legacy_*` cache to resolve or self-heal.
        OpenRound { .. } | CloseRound { .. } => {}
        // Task #49 — RefundDeposit. Resolve the specific per-room folio
        // line's legacy `HT_CheckIn_Ds.id` (back-populated onto
        // `ht_checkin_rooms.cr_legacy_ds_id` by the room's CreateCheckIn
        // writeback) keyed on the intent's `cr_id`. None ⇒ not yet
        // back-populated; the recipe no-ops with a WARN.
        RefundDeposit { cr_id, .. } => {
            if let Some(row) =
                sqlx::query("SELECT cr_legacy_ds_id FROM ht_checkin_rooms WHERE cr_id = $1")
                    .bind(*cr_id)
                    .fetch_optional(pg)
                    .await?
            {
                resolved.legacy_dep_ds_id = row.try_get("cr_legacy_ds_id").ok();
            }
        }
        // Task #47 — CreateNote carries the legacy target key (room_no /
        // username) directly in its payload; nothing to resolve. MarkNoteRead
        // needs the legacy `SMS_ID`, cached on `ht_notes.note_legacy_id` and
        // resolved here by the note's aggregate id. NULL ⇒ the dispatcher
        // defers (CreateNote back-population still pending).
        CreateNote { .. } => {}
        MarkNoteRead {
            note_aggregate_id, ..
        } => {
            if let Some(row) =
                sqlx::query("SELECT note_legacy_id FROM ht_notes WHERE aggregate_id = $1")
                    .bind(note_aggregate_id)
                    .fetch_optional(pg)
                    .await?
            {
                resolved.note_legacy_id = row.try_get("note_legacy_id").ok();
            }
        }
        // Task #51 — UpsertRatePrice carries the full `HT_Rooms_Price` row
        // (composite key + prices) in its payload; nothing to resolve from PG.
        // Mirrors the `UpdateRoom` / `AdjustProductStock` resolution shape.
        UpsertRatePrice { .. } => {}
        // Phase 2 — MirrorGuestImage needs the raw `doc_image` bytea, loaded
        // from `ht_guest_documents` by `doc_id` so the recipe stays PG-pure and
        // binds it as a varbinary param. Dynamic sqlx (bytea via
        // `try_get::<Vec<u8>>`) — no `query!` macro / no `.sqlx` cache. NULL /
        // missing ⇒ the dispatcher errors and the job retries.
        MirrorGuestImage { doc_id, .. } => {
            if let Some(row) =
                sqlx::query("SELECT doc_image FROM ht_guest_documents WHERE doc_id = $1")
                    .bind(doc_id)
                    .fetch_optional(pg)
                    .await?
            {
                resolved.guest_document_image = row.try_get::<Vec<u8>, _>("doc_image").ok();
            }
        }
        // Phase 4 — MirrorCompanion / MirrorCompanionList carry the legacy
        // `Cin_no` (+ name/country, or the full list) in their payload (resolved
        // by the route before enqueue); nothing to resolve from PG. Mirrors the
        // `UpdateCustomer` payload-carries-key shape. CompanionDelete likewise
        // carries the legacy `Cin_no` + the row's known legacy id directly.
        MirrorCompanion { .. } | MirrorCompanionList { .. } | CompanionDelete { .. } => {}
        // Convergent companion mirror — CompanionAdd carries the legacy
        // `Cin_no` in its payload, but the dispatcher needs to know whether
        // the canonical `ht_guest_registry` row still exists (the guest may
        // have been deleted while the job sat queued). A vanished row ⇒ skip
        // the legacy INSERT entirely: with nothing to stamp
        // `guest_legacy_id` onto, the CT echo could never be adopted and
        // the orphan legacy row would re-import as a duplicate.
        CompanionAdd { guest_id, .. } => {
            let exists: Option<i32> =
                sqlx::query_scalar("SELECT 1 FROM ht_guest_registry WHERE guest_id = $1")
                    .bind(*guest_id)
                    .fetch_optional(pg)
                    .await?;
            resolved.companion_guest_exists = exists.is_some();
        }
        // Issue #202 — CreateCashEntry is standalone: the payload carries
        // everything the recipe needs directly, no legacy FK to resolve from
        // PG (same shape as CreateBooking / MirrorCompanion).
        CreateCashEntry { .. } => {}
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
    /// When the last self-heal alert was SENT. This is what makes the
    /// throttle a time window rather than a counter: without it, zeroing
    /// `count` on fire is the only thing standing between a sustained
    /// salvage rate and one Slack POST per `SELF_HEAL_ALERT_THRESHOLD`
    /// events, however fast those arrive.
    last_alert_at: Option<Instant>,
}

impl SelfHealCounter {
    const fn new() -> Self {
        Self {
            window_start: None,
            count: 0,
            aggregates: Vec::new(),
            last_alert_at: None,
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
/// `window` does two things, and both are measured in TIME:
///
///   - **Burst window.** `threshold` events have to land inside it before
///     anything fires. Events arriving after it lapses open a fresh window
///     at count=1, so a stale partial burst from an hour ago never
///     contributes to a page now.
///   - **Alert floor.** Two alerts are never less than `window` apart, no
///     matter the event rate — `last_alert_at` gates the send.
///
/// **The floor is the fix.** The original implementation only zeroed the
/// counter on fire, which reads as a throttle but isn't one: the meaning was
/// "one alert per `threshold` events", so at a sustained salvage rate (a
/// broken back-population while the queue drains) the interval between
/// pages collapsed to whatever `threshold` events cost — potentially
/// sub-second, with nothing bounding it. The counter still resets on fire
/// so the next page reports only what happened since the last one; the
/// floor is what stops the storm.
///
/// Events that arrive while the floor is closed are still counted (and
/// still logged at warn by the caller) — suppression here is never silent.
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

    // Time floor: an alert may only go out if we haven't sent one inside
    // `window`. First ever alert (None) is always allowed through.
    let floor_clear = state
        .last_alert_at
        .map(|sent| now.duration_since(sent) >= window)
        .unwrap_or(true);

    let fire = state.count >= threshold && floor_clear;
    let decision = AlertDecision {
        fire,
        count: state.count,
        window_secs: window.as_secs(),
    };

    if fire {
        // Open the floor's cooldown and start counting again from zero, so
        // the next page reports the volume accumulated since THIS one.
        state.last_alert_at = Some(now);
        state.window_start = Some(now);
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
    // even if Slack is offline. `page_held` marks the events that WOULD
    // have paged but for the time floor — suppression by the throttle is
    // never invisible, it just isn't a Slack message.
    tracing::warn!(
        %aggregate_id,
        count = decision.count,
        window_secs = decision.window_secs,
        threshold = SELF_HEAL_ALERT_THRESHOLD,
        page_held = !decision.fire && decision.count >= SELF_HEAL_ALERT_THRESHOLD,
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
            out.book_id = json
                .get("book_id")
                .and_then(|v| v.as_str())
                .map(String::from);
            out.cust_no = json
                .get("cust_no")
                .and_then(|v| v.as_str())
                .map(String::from);
            out.cin_no = json
                .get("cin_no")
                .and_then(|v| v.as_str())
                .map(String::from);
            out.room_no = json
                .get("room_no")
                .and_then(|v| v.as_str())
                .map(String::from);
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
/// Audit LOW-2: on an `exhausted → done` transition (operator manually
/// fixed the cause + put the row back in the queue, and this attempt
/// succeeded) we post a `:white_check_mark:` Slack so the operator sees
/// closure, not just the original `:rotating_light:` alarm.
///
/// **The transition is detected from `prior`, not from PG.** This UPDATE
/// used to carry a `WITH prev AS (SELECT status …)` CTE for that purpose,
/// which was dead code from the day it was written: `claim_next_job`
/// commits the flip to `in_progress` in an earlier statement, so the CTE
/// read a post-claim snapshot and `prior_status` was invariably
/// `in_progress` — doubly so, since the UPDATE's own gate below requires
/// exactly that value. The pre-image is now captured by the claim
/// statement and threaded in as [`PriorDisposition`].
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
///
/// **Returns `true` only when this worker's claim-gated UPDATE actually
/// matched a row**, i.e. when THIS worker is the one that terminated the job.
/// The stolen-claim (`Ok(None)`) and error paths return `false`. The caller
/// uses that to decide whether to emit a `legacy_stale` hint: if another
/// worker stole the claim, IT will re-run the recipe and notify, and a
/// duplicate signal from here would inflate reception's toast count for a
/// single real change.
#[allow(clippy::too_many_arguments)]
async fn mark_done(
    pg: &PgPool,
    job_id: i64,
    claimed_at: DateTime<Utc>,
    aggregate_id: Uuid,
    intent: &WritebackIntent,
    slack: &Option<SlackClient>,
    prior: PriorDisposition,
    legacy_ids: serde_json::Value,
) -> bool {
    // No prior-status CTE here — see the doc comment. The pre-image is
    // carried in `prior`; this statement only needs the MED-2 claim-gate
    // (status + claimed_at) and the columns the closure alert reports.
    let row = sqlx::query(
        r#"
        UPDATE writeback_jobs wj
           SET status       = 'done',
               completed_at = NOW(),
               legacy_ids   = $2
         WHERE wj.id = $1
           AND wj.status = 'in_progress'
           AND wj.claimed_at = $3
        RETURNING wj.attempts, wj.intent, wj.aggregate_id
        "#,
    )
    .bind(job_id)
    .bind(&legacy_ids)
    .bind(claimed_at)
    .fetch_optional(pg)
    .await;

    match &row {
        Ok(Some(r)) => {
            // LOW-2: closure alert on operator-driven recovery. `prior` was
            // classified from the claim statement's pre-image (the only
            // place it is observable) and carried here on `ClaimedJob`.
            if prior == PriorDisposition::RecoveredFromExhausted {
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
            return false;
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
            return false;
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

    // The status flip landed and it was OURS — back-population is a
    // best-effort follow-up (self-healing at the next intent), so its
    // outcome does not change who owns the completion.
    true
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
    let checkin_ds_id = legacy_ids
        .get("checkin_ds_id")
        .and_then(|v| v.as_i64())
        .map(|n| n as i32);
    // Issue #202 — see the `CreateCashEntry` arm below.
    let cash_legacy_id = legacy_ids
        .get("cash_legacy_id")
        .and_then(|v| v.as_i64())
        .map(|n| n as i32);

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
            if cin_no.is_some() || room_no.is_some() || cust_no.is_some() || checkin_ds_id.is_some()
            {
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
            if let Some(pairs) = legacy_ids
                .get("checkin_ds_ids_by_room")
                .and_then(|v| v.as_array())
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
        RefundPayment {
            payment_aggregate_id,
            ..
        } => {
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
            if let (Some(refund_aggregate), true) = (payment_aggregate_id, pay_no.is_some()) {
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
        RecordPayment {
            payment_aggregate_id,
            ..
        } => {
            // Payment back-population is split-target: ht_checkins keeps the
            // check-in identifiers (the aggregate_id passed in is the
            // check-in's), and ht_payments gets the freshly-allocated
            // legacy_pay_no / legacy_receipt_no keyed off the payment's own
            // aggregate_id (Wave 5a item 3). Both UPDATEs are independent —
            // either can land without the other.
            if cin_no.is_some() || room_no.is_some() || cust_no.is_some() || checkin_ds_id.is_some()
            {
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
            if let (Some(pay_aggregate), true) = (
                payment_aggregate_id,
                pay_no.is_some() || receipt_no.is_some(),
            ) {
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
        MarkRoomClean { .. } | MarkRoomDirty { .. } | SetRoomMaintenance { .. } => {
            // The housekeeping / maintenance flag recipes don't allocate
            // any new legacy IDs.
        }
        UpdateRoom { .. } => {
            // update_room doesn't allocate any new legacy IDs — the
            // legacy `HT_Rooms` row already exists and we only shift its
            // column values. The canonical `ht_rooms_new.legacy_*`
            // back-link columns are populated by `backfill_rooms`, not
            // by recipe writebacks, so nothing to back-populate here.
        }
        MoveRoomTiles { .. } => {
            // Board moves only SET Room_X/Room_y on existing rows —
            // no legacy IDs allocated, nothing to back-populate.
        }
        UpdateCustomer { .. } => {
            // update_customer doesn't allocate any new legacy IDs — the
            // legacy `HT_Customers` row already exists (the intent is
            // only emitted when `legacy_cust_no` is already known) and
            // the recipe only shifts its column values.
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
        // Track G5 — IssueCoupon back-populates the freshly allocated
        // `HT_Cupon.cupon_no` onto `ht_coupons.legacy_cupon_no`. The
        // canonical row is keyed by the intent's `coupon_aggregate_id`
        // (== the row's `aggregate_id`). RedeemCoupon doesn't allocate
        // new legacy IDs (it just flips `cupon_print`); the legacy
        // back-pointer is already set from the prior IssueCoupon.
        IssueCoupon { .. } | RedeemCoupon { .. } => {
            let cupon_no = legacy_ids
                .get("cupon_no")
                .and_then(|v| v.as_i64())
                .map(|n| n as i32);
            if let Some(cupon_no) = cupon_no {
                sqlx::query(
                    "UPDATE ht_coupons SET \
                       legacy_cupon_no = COALESCE($2, legacy_cupon_no), \
                       updated_at = NOW() \
                     WHERE aggregate_id = $1",
                )
                .bind(aggregate_id)
                .bind(cupon_no)
                .execute(pg)
                .await?;
            }
        }
        // Track G6 — RecordPosSale back-populates the freshly-
        // allocated `HT_CheckIn_Product.id` onto
        // `ht_pos_sales.sale_legacy_id` so the reverse-sync mapper
        // can match legacy-origin rows to canonical ones and so the
        // reconcile job differentiates one-sided lines. The recipe
        // stuffs both ids into `legacy_ids.extra` (`sale_id` +
        // `ht_checkin_product_id`).
        RecordPosSale { .. } => {
            let sale_id = legacy_ids
                .get("extra")
                .and_then(|v| v.get("sale_id"))
                .and_then(|v| v.as_i64());
            let legacy_id = legacy_ids
                .get("extra")
                .and_then(|v| v.get("ht_checkin_product_id"))
                .and_then(|v| v.as_i64())
                .map(|n| n as i32);
            if let (Some(sale_id), Some(legacy_id)) = (sale_id, legacy_id) {
                sqlx::query(
                    "UPDATE ht_pos_sales \
                        SET sale_legacy_id = $2, updated_at = NOW() \
                      WHERE sale_id = $1 AND sale_legacy_id IS NULL",
                )
                .bind(sale_id)
                .bind(legacy_id)
                .execute(pg)
                .await?;
            }
            // The legacy_cin_no self-heal mirrors the RoomChange branch
            // — a fresh walkin's first writeback may surface the
            // cin_no here. Keeping the cached cin_no fresh avoids
            // forcing the next intent through the resolver fallback.
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
        // Task #45 — RecordReceipt back-populates the freshly-allocated
        // `HT_Receipt_H.id` + `Receipt_no` onto
        // `ht_pos_receipts.receipt_legacy_id` / `receipt_legacy_no`, keyed
        // by the receipt's aggregate id (== the intent's
        // `receipt_aggregate_id`). The recipe stuffs `receipt_h_id` into
        // `legacy_ids.extra` and the `Receipt_no` into `receipt_no`.
        RecordReceipt { .. } => {
            let receipt_h_id = legacy_ids
                .get("extra")
                .and_then(|v| v.get("receipt_h_id"))
                .and_then(|v| v.as_i64())
                .map(|n| n as i32);
            if receipt_h_id.is_some() || receipt_no.is_some() {
                sqlx::query(
                    "UPDATE ht_pos_receipts SET \
                       receipt_legacy_id = COALESCE($2, receipt_legacy_id), \
                       receipt_legacy_no = COALESCE($3, receipt_legacy_no), \
                       updated_at = NOW() \
                     WHERE aggregate_id = $1",
                )
                .bind(aggregate_id)
                .bind(receipt_h_id)
                .bind(receipt_no)
                .execute(pg)
                .await?;
            }
        }
        // Task #45 — VoidPosSale removes a legacy line by its existing
        // `sale_legacy_id` and allocates no new legacy id. The canonical
        // row is already `sale_status='voided'`. Nothing to back-populate.
        VoidPosSale { .. } => {}
        // Track J6 — OpenRound allocates its legacy id app-side (in
        // `open_shift`, already stamped onto `ht_shifts.shift_legacy_round_id`)
        // and CloseRound allocates nothing. Neither returns a legacy id to
        // back-populate.
        OpenRound { .. } | CloseRound { .. } => {}
        // Task #49 — RefundDeposit targets an existing `HT_CheckIn_Ds` row by
        // its already-resolved `cr_legacy_ds_id` and only flips the deposit
        // status flag. It allocates no new legacy id — nothing to
        // back-populate.
        RefundDeposit { .. } => {}
        // Task #47 — CreateNote back-populates the freshly-allocated
        // `HT_Room_SMS`/`HT_EMP_SMS.SMS_ID` (IDENTITY, captured via the recipe's
        // `OUTPUT INSERTED.SMS_ID`) onto `ht_notes.note_legacy_id`, keyed by the
        // note's aggregate id. MarkNoteRead flips a flag and allocates nothing.
        CreateNote { .. } => {
            let sms_id = legacy_ids
                .get("sms_id")
                .and_then(|v| v.as_i64())
                .map(|n| n as i32);
            if let Some(sms_id) = sms_id {
                sqlx::query(
                    "UPDATE ht_notes SET \
                       note_legacy_id = COALESCE($2, note_legacy_id), \
                       note_updated_at = NOW() \
                     WHERE aggregate_id = $1",
                )
                .bind(aggregate_id)
                .bind(sms_id)
                .execute(pg)
                .await?;
            }
        }
        MarkNoteRead { .. } => {}
        // Task #51 — UpsertRatePrice targets `HT_Rooms_Price` by the composite
        // `(Room_Type, Room_CustType)` key and allocates no new legacy id (the
        // INSERT branch's IDENTITY is re-pinned onto `rate_tier_legacy_id` by
        // the 15-minute mirror poll, not by this writeback). Nothing to
        // back-populate here.
        UpsertRatePrice { .. } => {}
        // Phase 2 — MirrorGuestImage writes a PROVISIONAL `Tb_Save_Image` row
        // (IDENTITY id, not captured for the provisional shape); the check-in
        // writeback later stamps `cin_no`/`cust_no` by `tmp_no`. Nothing to
        // back-populate onto the canonical row today (doc_legacy_id capture is a
        // documented follow-up).
        MirrorGuestImage { .. } => {}
        // Phase 4 — MirrorCompanion / MirrorCompanionList write
        // `HT_CheckIn_Other_People` rows (IDENTITY ids) and carry no canonical
        // back-pointer column. Nothing to back-populate. CompanionDelete
        // removes a legacy row and allocates nothing.
        MirrorCompanion { .. } | MirrorCompanionList { .. } | CompanionDelete { .. } => {}
        // Convergent companion mirror — stamp the captured
        // `HT_CheckIn_Other_People.id` (recipe puts it in
        // `legacy_ids.extra.other_people_id`) onto
        // `ht_guest_registry.guest_legacy_id` so the CT echo of our own
        // INSERT lands on the mapper's ON CONFLICT (guest_legacy_id) upsert
        // with identical values instead of inserting a duplicate row.
        // Guarded on IS NULL: if the mapper's echo-adoption already stamped
        // it (CT won the race), this is a no-op.
        CompanionAdd { guest_id, .. } => {
            let other_people_id = legacy_ids
                .get("extra")
                .and_then(|v| v.get("other_people_id"))
                .and_then(|v| v.as_i64())
                .map(|n| n as i32);
            if let Some(other_people_id) = other_people_id {
                sqlx::query(
                    "UPDATE ht_guest_registry SET guest_legacy_id = $1 \
                     WHERE guest_id = $2 AND guest_legacy_id IS NULL",
                )
                .bind(other_people_id)
                .bind(*guest_id)
                .execute(pg)
                .await?;
            }
        }
        // Issue #202 — CreateCashEntry back-populates the freshly-allocated
        // `TB_Pay_History.id` onto `ht_cash_ledger.cash_legacy_id`, keyed by
        // the row's `aggregate_id` (migration 085). This is the SAME column
        // `sync_cash_history`'s `ON CONFLICT (cash_legacy_id)` UPSERT dedups
        // on (`bin/sync.rs::CASH_HISTORY_UPSERT_SQL`, consumed at
        // `bin/sync.rs::sync_cash_history`) — closing the echo gap this
        // issue named: without this stamp, an app-originated cash entry's
        // `cash_legacy_id` stays NULL forever and every re-import tick
        // inserts a genuine duplicate row (pinned by
        // `bin/sync.rs::cash_sync_tests::reimport_without_backpopulation_still_duplicates`).
        CreateCashEntry { .. } => {
            if let Some(cash_legacy_id) = cash_legacy_id {
                sqlx::query(
                    "UPDATE ht_cash_ledger SET \
                       cash_legacy_id = COALESCE($2, cash_legacy_id) \
                     WHERE aggregate_id = $1",
                )
                .bind(aggregate_id)
                .bind(cash_legacy_id)
                .execute(pg)
                .await?;
            }
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
                send_exhausted_alert(slack, job_id, &intent, aggregate_id, attempts, err_msg).await;
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
    mark_failed(
        pg,
        job_id,
        attempts,
        claimed_at,
        max_attempts,
        slack,
        err_msg,
    )
    .await;
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

/// Coarse failure class for an exhausted job, derived from the message text.
///
/// Deliberately string-based rather than typed: `send_exhausted_alert` is
/// reached from four call sites (`mark_failed`'s terminal branch,
/// `force_exhaust_job` via the non-retryable route, the panic arm of the
/// main loop, and the resolver/pool/trancount pre-dispatch failures), and
/// only one of them still holds a `WritebackError`. Classifying the
/// rendered message keeps every path on the same key without threading a
/// typed error through call sites that never had one.
///
/// The prefixes are the `#[error(...)]` Display forms in
/// `writeback::error::WritebackError` plus the four messages this binary
/// synthesises itself. Returns `&'static str` so the throttle map's key
/// space stays bounded by construction (intents are a fixed enum, classes
/// a fixed list) — no unbounded growth from attacker- or vendor-controlled
/// error text.
fn classify_error_kind(err_msg: &str) -> &'static str {
    // Worker-synthesised prefixes first — these wrap an inner error whose
    // own prefix would otherwise win the match.
    const PREFIXES: &[(&str, &str)] = &[
        ("PANIC:", "panic"),
        ("resolve_legacy_ids:", "resolve_legacy_ids"),
        ("mssql_acquire:", "mssql_acquire"),
        ("trancount_reset:", "trancount_reset"),
        ("legacy schema drift:", "schema_drift"),
        ("writeback disabled by", "disabled"),
        ("intent payload mismatch:", "intent_mismatch"),
        ("recipe error:", "recipe"),
        ("legacy connection pool:", "pool"),
        ("payload deserialize:", "serde"),
        ("tiberius:", "tiberius"),
        ("sqlx:", "sqlx"),
        ("config:", "config"),
    ];
    for (prefix, kind) in PREFIXES {
        if err_msg.starts_with(prefix) {
            return kind;
        }
    }
    "other"
}

/// Throttle key: one collapse window per `(intent, error-class)` pair.
/// Keeping the intent in the key means a recipe broken for `CreateBooking`
/// does not mask an unrelated `CheckOut` failure that starts during the
/// same window.
type ExhaustedAlertKey = (String, &'static str);

/// Open collapse window for one [`ExhaustedAlertKey`].
#[derive(Debug)]
struct ExhaustedAlertWindow {
    /// When the alert that opened this window was SENT.
    opened_at: Instant,
    /// Alerts collapsed into this window since then (excludes the one that
    /// opened it).
    suppressed: u32,
}

/// Outcome of the throttle check — pure decision, split out from the Slack
/// POST so the collapse rules are unit-testable without a webhook.
#[derive(Debug, PartialEq, Eq)]
enum ExhaustedAlertDecision {
    /// Post to Slack. `collapsed` is how many alerts for this key were
    /// suppressed since the previous send — 0 on a first occurrence, >0 on
    /// the first send after a window that absorbed repeats.
    Send { collapsed: u32 },
    /// Do not post. `collapsed` is the running suppressed count inside the
    /// currently-open window.
    Suppress { collapsed: u32, window_secs: u64 },
}

/// Open collapse windows, keyed by `(intent, error-class)`.
type ExhaustedAlertWindows = HashMap<ExhaustedAlertKey, ExhaustedAlertWindow>;

/// Process-global collapse state. `OnceLock` + `Mutex` mirrors
/// `SELF_HEAL_COUNTER` — keeps the call-site change to a single lookup
/// instead of threading throttle state through `mark_failed` /
/// `force_exhaust_job` / the panic arm.
static EXHAUSTED_ALERT_WINDOWS: OnceLock<Arc<Mutex<ExhaustedAlertWindows>>> = OnceLock::new();

/// Lazily get-or-init the process-global exhausted-alert collapse state.
fn exhausted_alert_windows() -> &'static Arc<Mutex<ExhaustedAlertWindows>> {
    EXHAUSTED_ALERT_WINDOWS.get_or_init(|| Arc::new(Mutex::new(ExhaustedAlertWindows::new())))
}

/// Pure collapse decision for the exhausted-job page.
///
/// Rules — chosen so the alert stays trustworthy under a bad-recipe drain
/// while never hiding the actionable first signal:
///
///   - No open window for the key ⇒ **send immediately**, open a window.
///     The first occurrence is the one an operator acts on and it carries
///     the full error text.
///   - Inside an open window ⇒ **suppress**, bump the count. This is the
///     bad-recipe drain case: N identical pages become one line of context
///     on the next send instead of N webhook round-trips in the hot path.
///   - Window expired ⇒ **send**, reporting how many were collapsed while
///     it was open, and reopen. A sustained outage therefore pages once per
///     `window`, each time stating the true volume.
///
/// Expired windows that absorbed nothing are dropped, so the map stays at
/// the size of the currently-failing key set rather than every pair ever
/// seen. (Dropping them is behaviour-preserving: a fresh insert and an
/// expired-with-zero window both yield `Send { collapsed: 0 }`.)
fn decide_exhausted_alert(
    windows: &mut ExhaustedAlertWindows,
    key: ExhaustedAlertKey,
    now: Instant,
    window: Duration,
) -> ExhaustedAlertDecision {
    windows.retain(|k, w| {
        k == &key || w.suppressed > 0 || now.duration_since(w.opened_at) < window
    });

    match windows.get_mut(&key) {
        Some(open) if now.duration_since(open.opened_at) < window => {
            open.suppressed = open.suppressed.saturating_add(1);
            ExhaustedAlertDecision::Suppress {
                collapsed: open.suppressed,
                window_secs: window.as_secs(),
            }
        }
        Some(expired) => {
            let collapsed = expired.suppressed;
            expired.opened_at = now;
            expired.suppressed = 0;
            ExhaustedAlertDecision::Send { collapsed }
        }
        None => {
            windows.insert(
                key,
                ExhaustedAlertWindow {
                    opened_at: now,
                    suppressed: 0,
                },
            );
            ExhaustedAlertDecision::Send { collapsed: 0 }
        }
    }
}

/// Post a Slack alert when a writeback job exhausts its retry budget.
/// Best-effort — Slack failures are logged inside `send_message` but never
/// propagated. Avoids blocking the writeback main loop on Slack timeouts.
///
/// Repeats of the same `(intent, error-class)` within
/// `EXHAUSTED_ALERT_WINDOW_SECS` are collapsed (see
/// [`decide_exhausted_alert`]). Suppression is never silent: every
/// suppressed job logs at `warn` with its job_id, so a log grep still sees
/// one line per affected row even though Slack sees one message per class
/// per window. The unconditional per-job `tracing::error!` at both call
/// sites is untouched.
async fn send_exhausted_alert(
    slack: &SlackClient,
    job_id: i64,
    intent: &str,
    aggregate_id: Option<Uuid>,
    attempts: i32,
    err_msg: &str,
) {
    let error_kind = classify_error_kind(err_msg);
    let decision = {
        // Short critical section, no awaits held across the lock. Poisoned
        // lock ⇒ recover and continue: this is alert hygiene, not a
        // correctness path.
        let windows = exhausted_alert_windows();
        let mut guard = match windows.lock() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };
        decide_exhausted_alert(
            &mut guard,
            (intent.to_string(), error_kind),
            Instant::now(),
            Duration::from_secs(EXHAUSTED_ALERT_WINDOW_SECS),
        )
    };

    let collapsed = match decision {
        ExhaustedAlertDecision::Suppress {
            collapsed,
            window_secs,
        } => {
            tracing::warn!(
                job_id,
                intent,
                error_kind,
                collapsed,
                window_secs,
                "Writeback EXHAUSTED alert collapsed — same (intent, error class) \
                 already paged inside the window; the job itself is still \
                 exhausted and still needs triage"
            );
            return;
        }
        ExhaustedAlertDecision::Send { collapsed } => collapsed,
    };

    let aggregate_id_str = aggregate_id
        .map(|u| u.to_string())
        .unwrap_or_else(|| "(unknown)".into());
    // Head+tail truncation — tiberius/sqlx errors put the actually-useful
    // row context at the END (e.g. "in row 23, column foo: <value>"). A
    // pure head truncation would lose it. Slice on character boundaries
    // (Thai messages are multi-byte) by walking with `char_indices`.
    let truncated_err = truncate_head_tail(err_msg, 200, 300);
    // Only present on a follow-up send, so the common single-failure page
    // reads exactly as it always has.
    let collapsed_line = if collapsed > 0 {
        format!(
            "*Also suppressed:* {collapsed} further `{intent}` / `{error_kind}` \
             exhaustion(s) in the last {EXHAUSTED_ALERT_WINDOW_SECS}s — \
             `SELECT * FROM writeback_jobs WHERE status='exhausted' AND intent='{intent}'`\n"
        )
    } else {
        String::new()
    };
    let text = format!(
        ":rotating_light: *Writeback EXHAUSTED retries* :rotating_light:\n\
         *Job ID:* `{job_id}`\n\
         *Intent:* `{intent}`\n\
         *Aggregate:* `{aggregate_id_str}`\n\
         *Attempts:* {attempts}\n\
         *Error class:* `{error_kind}`\n\
         {collapsed_line}\
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
///
/// `subscribed` is set the moment the `LISTEN` lands, so the supervisor can
/// tell "never got a connection" apart from "was live and then dropped".
/// Without that distinction there is no way to know whether an outage is
/// still going: a listener that reconnects cleanly and runs for hours, then
/// hits one `recv()` error, is indistinguishable from one that has never
/// come up.
async fn run_listener(
    pg: PgPool,
    wakeup: Arc<Notify>,
    subscribed: &AtomicBool,
) -> Result<(), sqlx::Error> {
    let mut listener = PgListener::connect_with(&pg).await?;
    listener.listen(WRITEBACK_CHANNEL).await?;
    subscribed.store(true, Ordering::Relaxed);
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

/// Rolling health of the NOTIFY listener, as seen by its supervisor.
///
/// The old version of this was a bare `consecutive_failures` counter that
/// paged at 10 and zeroed itself on every send. Two things were wrong with
/// it, and both made the page less trustworthy the longer an outage ran:
/// the counter had no notion of "the listener has been fine for six hours"
/// (nothing reset it on a healthy session, so ten unrelated `recv()` errors
/// spread over days added up to a page), and zeroing on fire with no
/// timestamp made re-firing a function of the retry cadence — ~105s during
/// a sustained outage.
#[derive(Debug, Default)]
struct ListenerHealth {
    /// Start of the current uninterrupted outage. `None` = no outage in
    /// progress (only true before the first failure).
    outage_started: Option<Instant>,
    /// Failed sessions since the last healthy one. Reported for context;
    /// it is no longer what decides the page.
    consecutive_failures: u32,
    /// When the last page for this outage went out — the re-page floor.
    last_paged_at: Option<Instant>,
    /// Whether this outage has already paged, i.e. whether recovery owes
    /// the operator an all-clear.
    paged_this_outage: bool,
}

/// What the supervisor should do after one listener session ended.
#[derive(Debug, PartialEq, Eq)]
enum ListenerAction {
    /// Log it and reconnect. The overwhelming majority — a reconnect loop
    /// doing its job is not news.
    LogOnly,
    /// The listener has been continuously down for
    /// `LISTENER_SUSTAINED_OUTAGE_SECS`. Reconnection is not happening on
    /// its own; page.
    Page { outage_secs: u64, consecutive_failures: u32 },
    /// A healthy session closed an outage we had paged for — post the
    /// all-clear so the channel doesn't keep a stale alarm open.
    Recovered { outage_secs: u64 },
}

/// Action + how long to wait before respawning.
#[derive(Debug, PartialEq, Eq)]
struct ListenerDecision {
    action: ListenerAction,
    backoff_secs: u64,
}

/// Pure supervisor policy — called once per ended listener session.
///
/// `healthy_session` means the subscription actually came up AND survived
/// `LISTENER_HEALTHY_SESSION_SECS`; a connect-then-instantly-drop flap is
/// NOT healthy and keeps accumulating toward the sustained threshold.
///
/// The page is gated on elapsed outage TIME, not on a failure count, and
/// re-pages inside one outage are floored at `repage_cooldown`. Backoff
/// tracks the outage duration rather than the alert: the retry cadence
/// should slow because the outage is long, not because Slack was told.
fn decide_listener_action(
    state: &mut ListenerHealth,
    now: Instant,
    healthy_session: bool,
    sustained: Duration,
    repage_cooldown: Duration,
) -> ListenerDecision {
    if healthy_session {
        // The listener was live for a meaningful stretch, so whatever
        // outage preceded it is over — even though this session has just
        // ended and a new one may be starting.
        let recovered = state.paged_this_outage.then(|| {
            state
                .outage_started
                .map(|start| now.duration_since(start).as_secs())
                .unwrap_or(0)
        });
        // This session ended with an error too, so a fresh outage clock
        // starts now; if the next session is healthy it clears silently.
        *state = ListenerHealth {
            outage_started: Some(now),
            consecutive_failures: 1,
            last_paged_at: None,
            paged_this_outage: false,
        };
        return ListenerDecision {
            action: recovered
                .map(|outage_secs| ListenerAction::Recovered { outage_secs })
                .unwrap_or(ListenerAction::LogOnly),
            backoff_secs: LISTENER_BACKOFF_SECS,
        };
    }

    state.consecutive_failures = state.consecutive_failures.saturating_add(1);
    let started = *state.outage_started.get_or_insert(now);
    let outage = now.duration_since(started);
    let is_sustained = outage >= sustained;

    let backoff_secs = if is_sustained {
        LISTENER_BACKOFF_SUSTAINED_SECS
    } else {
        LISTENER_BACKOFF_SECS
    };

    let repage_clear = state
        .last_paged_at
        .map(|sent| now.duration_since(sent) >= repage_cooldown)
        .unwrap_or(true);

    if is_sustained && repage_clear {
        state.last_paged_at = Some(now);
        state.paged_this_outage = true;
        return ListenerDecision {
            action: ListenerAction::Page {
                outage_secs: outage.as_secs(),
                consecutive_failures: state.consecutive_failures,
            },
            backoff_secs,
        };
    }

    ListenerDecision {
        action: ListenerAction::LogOnly,
        backoff_secs,
    }
}

/// Supervisor for `run_listener` (audit LOW-3, recalibrated). Respawns the
/// listener forever with a 5s backoff, and pages only once the listener has
/// been continuously down for `LISTENER_SUSTAINED_OUTAGE_SECS` — see that
/// constant for why a reconnect loop below that bar is a log line, not a
/// page.
///
/// Why we keep retrying instead of exiting: the worker has two signal
/// sources — NOTIFY and the 30s poll. If we exit the listener task entirely
/// the worker still functions (it just sees jobs ~30s late). But an
/// operator under time pressure during the live test won't realize sync
/// silently degraded. Persistent reconnect + a *sustained* Slack alert
/// preserves both liveness AND visibility, without spending the operator's
/// attention on a condition that fixes itself.
async fn run_listener_supervised(pg: PgPool, wakeup: Arc<Notify>, slack: Option<SlackClient>) {
    let mut health = ListenerHealth::default();
    let subscribed = AtomicBool::new(false);
    let sustained = Duration::from_secs(LISTENER_SUSTAINED_OUTAGE_SECS);
    let repage_cooldown = Duration::from_secs(LISTENER_REPAGE_COOLDOWN_SECS);

    loop {
        let pg_inner = pg.clone();
        let wakeup_inner = wakeup.clone();
        subscribed.store(false, Ordering::Relaxed);
        let session_started = Instant::now();
        let outcome = run_listener(pg_inner, wakeup_inner, &subscribed).await;
        // "Healthy" = the LISTEN actually landed and the subscription then
        // held for at least one poll interval. A connect that fails, or one
        // that drops immediately, does not clear the outage clock.
        let healthy_session = subscribed.load(Ordering::Relaxed)
            && session_started.elapsed() >= Duration::from_secs(LISTENER_HEALTHY_SESSION_SECS);

        match &outcome {
            Ok(()) => {
                // Listener returned Ok — the only path is `loop {}` exit,
                // which currently can't happen.
                tracing::warn!("PgListener returned Ok unexpectedly — respawning");
            }
            Err(err) => {
                tracing::warn!(
                    error = %err,
                    healthy_session,
                    session_secs = session_started.elapsed().as_secs(),
                    "PgListener task ended; will respawn after backoff"
                );
            }
        }

        let decision = decide_listener_action(
            &mut health,
            Instant::now(),
            healthy_session,
            sustained,
            repage_cooldown,
        );

        match decision.action {
            ListenerAction::LogOnly => {
                // The demotion. A reconnect loop that is doing its job is a
                // log line: the queue keeps draining on the 30s poll, so
                // nothing is stuck and nothing is lost.
                tracing::info!(
                    consecutive_failures = health.consecutive_failures,
                    outage_secs = health
                        .outage_started
                        .map(|s| s.elapsed().as_secs())
                        .unwrap_or(0),
                    sustained_threshold_secs = LISTENER_SUSTAINED_OUTAGE_SECS,
                    backoff_secs = decision.backoff_secs,
                    "PgListener down — reconnecting (worker still drains via the 30s poll; \
                     no page until the outage is sustained)"
                );
            }
            ListenerAction::Page {
                outage_secs,
                consecutive_failures,
            } => {
                tracing::error!(
                    outage_secs,
                    consecutive_failures,
                    sustained_threshold_secs = LISTENER_SUSTAINED_OUTAGE_SECS,
                    "PgListener supervisor: SUSTAINED outage — paging operator"
                );
                if let Some(slack) = &slack {
                    send_listener_alert(slack, outage_secs, consecutive_failures).await;
                }
            }
            ListenerAction::Recovered { outage_secs } => {
                tracing::info!(
                    outage_secs,
                    "PgListener recovered after a paged outage — posting all-clear"
                );
                if let Some(slack) = &slack {
                    send_listener_recovered_alert(slack, outage_secs).await;
                }
            }
        }

        tokio::time::sleep(Duration::from_secs(decision.backoff_secs)).await;
    }
}

/// Post a Slack alert when the PG NOTIFY listener has been continuously
/// down for `LISTENER_SUSTAINED_OUTAGE_SECS` (audit LOW-3, recalibrated).
/// The worker is still functional via the 30s poll fallback — this is a
/// latency-degradation page, which is exactly why it waits for the outage
/// to prove it is not self-recovering before spending the operator's
/// attention.
async fn send_listener_alert(slack: &SlackClient, outage_secs: u64, consecutive_failures: u32) {
    let outage_mins = outage_secs / 60;
    let text = format!(
        ":warning: *Writeback PG NOTIFY listener UNHEALTHY* :warning:\n\
         *Down for:* {outage_mins}m ({outage_secs}s continuous, \
         {consecutive_failures} failed reconnects)\n\
         _Past the {LISTENER_SUSTAINED_OUTAGE_SECS}s sustained threshold, so this is NOT \
         a self-recovering blip. The worker is still draining the queue via the 30s poll \
         fallback — nothing is lost — but sync latency has degraded from sub-second to \
         ~30s. Likely causes: PG down, network partition, role missing LISTEN privilege, \
         or max_connections exhausted. Inspect:_\n\
         ```\n\
         SELECT * FROM pg_stat_activity WHERE query LIKE '%LISTEN%';\n\
         SELECT count(*) FROM pg_stat_activity;\n\
         ```\n\
         _The supervisor keeps retrying every {LISTENER_BACKOFF_SUSTAINED_SECS}s and will \
         post an all-clear when it reconnects; you will not be re-paged for this outage \
         more than once per {LISTENER_REPAGE_COOLDOWN_SECS}s._"
    );
    let msg = SlackMessage::with_site_text(current_site_id(), text);
    let _ = slack.send_message(&msg).await;
}

/// All-clear for a listener outage that was paged. Same pairing rule as the
/// exhausted-job `:white_check_mark:`: a failure alert that never closes
/// trains the operator to ignore the channel.
async fn send_listener_recovered_alert(slack: &SlackClient, outage_secs: u64) {
    let outage_mins = outage_secs / 60;
    let text = format!(
        ":white_check_mark: *Writeback PG NOTIFY listener RECOVERED* \
         :white_check_mark:\n\
         *Outage duration:* {outage_mins}m ({outage_secs}s)\n\
         _The listener reconnected and held the subscription for at least \
         {LISTENER_HEALTHY_SESSION_SECS}s. NOTIFY-driven wakeups are back to sub-second; \
         closure of the_ `:warning:` _sent earlier for this outage._"
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
    mssql: DbPool,
    slack: Option<SlackClient>,
    shutdown: Arc<Notify>,
) {
    let mut last_alerted_pending: Option<Instant> = None;
    let mut last_alerted_failed: Option<Instant> = None;
    let mut last_alerted_stuck: Option<Instant> = None;
    // Ledger retention prune cadence (~6h); None ⇒ prune on the first tick.
    let mut last_pruned: Option<Instant> = None;
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

        // Ledger retention prune (~6h cadence). Placed BEFORE the breach-driven
        // early-returns below so quiet ticks don't skip it. Trims
        // dbo.ht_writeback_ledger rows older than 90 days — idempotency markers
        // far outlive a retry (which happens within the minutes-long lease).
        // Best-effort + per-site (this worker's mssql pool targets its own
        // legacy server); on failure we just wait for the next ~6h window.
        if last_pruned.map_or(true, |t| t.elapsed() >= Duration::from_secs(6 * 3600)) {
            match mssql.get().await {
                Ok(mut conn) => {
                    if let Err(e) = simple_query_with_timeout_drop(
                        &mut conn,
                        "DELETE FROM dbo.ht_writeback_ledger \
                         WHERE applied_at < DATEADD(day, -90, GETDATE())",
                        MssqlOpKind::Write,
                    )
                    .await
                    {
                        tracing::warn!(error = %e, "[janitor] ledger prune failed; next ~6h window");
                    } else {
                        tracing::debug!("[janitor] ledger prune ran (rows older than 90d removed)");
                    }
                    last_pruned = Some(Instant::now());
                }
                Err(e) => {
                    tracing::warn!(error = %e, "[janitor] ledger prune: no MSSQL conn this tick")
                }
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
        assert!(
            n > 0,
            "backoff_secs(0) should be safe and non-zero, got {n}"
        );
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
        assert!(
            STUCK_IN_PROGRESS_TIMEOUT_SECS >= 60,
            "less than 1 min risks racing slow recipes"
        );
        assert!(
            STUCK_IN_PROGRESS_TIMEOUT_SECS <= 1800,
            "more than 30 min is too slow to recover"
        );
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
    /// (The time floor added later is what keeps the *next* burst from
    /// paging immediately — see `should_alert_enforces_time_floor_*`.)
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

    /// MED-4 throttle, recalibrated: even at `threshold = 1` — the
    /// degenerate config where every single event is alert-worthy — the
    /// time floor still holds. This test previously asserted the opposite
    /// ("threshold=1 must fire every event"), which is exactly the
    /// counter-reset semantics the floor replaces: with no `last_alert_at`,
    /// "one alert per 1 event" meant one Slack POST per salvage.
    #[test]
    fn should_alert_threshold_of_one_still_honours_the_time_floor() {
        let mut state = SelfHealCounter::new();
        let t0 = Instant::now();
        let window = Duration::from_secs(60);

        let first = should_alert(&mut state, t0, 1, window);
        assert!(first.fire, "the first event must always page");

        // Same instant, and every second after it inside the window: the
        // floor holds them all.
        for offset in [0, 1, 5, 30, 59] {
            let d = should_alert(&mut state, t0 + Duration::from_secs(offset), 1, window);
            assert!(
                !d.fire,
                "event at +{offset}s must be held by the {}s floor",
                window.as_secs()
            );
        }

        // Once the floor lapses, the next event pages again.
        let after = should_alert(&mut state, t0 + Duration::from_secs(60), 1, window);
        assert!(after.fire, "the floor must open again after the window");
    }

    /// THE W5 regression test. Under a sustained salvage rate the old
    /// implementation paged once per `SELF_HEAL_ALERT_THRESHOLD` events —
    /// counter semantics wearing a window's name — so an hour of one
    /// salvage per second produced ~720 Slack messages. The floor makes the
    /// interval a genuine function of time.
    #[test]
    fn should_alert_enforces_time_floor_under_sustained_rate() {
        let mut state = SelfHealCounter::new();
        let t0 = Instant::now();
        let window = Duration::from_secs(300);

        // One self-heal every second for an hour.
        let mut fire_times: Vec<u64> = Vec::new();
        for sec in 0..3600u64 {
            let d = should_alert(&mut state, t0 + Duration::from_secs(sec), 5, window);
            if d.fire {
                fire_times.push(sec);
            }
        }

        // Old behaviour: 3600 events / 5 per alert = 720 pages.
        assert!(
            fire_times.len() <= 13,
            "3600 events in 1h must not produce {} pages — the window is a \
             time window, not an event counter",
            fire_times.len()
        );
        assert!(
            !fire_times.is_empty(),
            "a sustained salvage rate must still page at least once"
        );
        for pair in fire_times.windows(2) {
            assert!(
                pair[1] - pair[0] >= window.as_secs(),
                "pages at {}s and {}s are closer than the {}s floor",
                pair[0],
                pair[1],
                window.as_secs()
            );
        }
    }

    /// Suppression by the floor must not be a black hole: events keep being
    /// counted while it is closed, so the caller's per-event warn log (and
    /// the next page) still reflect the real volume.
    #[test]
    fn should_alert_keeps_counting_while_the_floor_is_closed() {
        let mut state = SelfHealCounter::new();
        let t0 = Instant::now();
        let window = Duration::from_secs(300);

        for _ in 0..5 {
            let _ = should_alert(&mut state, t0, 5, window);
        }
        // The 5th fired and zeroed the count; the next three are held by
        // the floor but still counted.
        for expected in 1..=3 {
            let d = should_alert(&mut state, t0 + Duration::from_secs(1), 5, window);
            assert!(!d.fire, "floor must hold this page");
            assert_eq!(d.count, expected, "held events must still be counted");
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
    /// short enough to be invisible normally, long enough not to spin; and
    /// a sustained-outage threshold that is unambiguously past the
    /// self-recovering blips this alert used to fire on.
    #[test]
    fn listener_supervisor_constants_are_in_safe_range() {
        // Bound through locals — same idiom as
        // `queue_stuck_in_progress_age_mins_is_i32_for_make_interval`, so
        // the assertions aren't compile-time constants and a type change
        // at the const site fails here rather than silently.
        let backoff: u64 = LISTENER_BACKOFF_SECS;
        let backoff_sustained: u64 = LISTENER_BACKOFF_SUSTAINED_SECS;
        let sustained: u64 = LISTENER_SUSTAINED_OUTAGE_SECS;
        let repage: u64 = LISTENER_REPAGE_COOLDOWN_SECS;
        let healthy: u64 = LISTENER_HEALTHY_SESSION_SECS;
        let stuck_claim: u64 = STUCK_IN_PROGRESS_TIMEOUT_SECS as u64;

        assert!(backoff >= 1, "<1s would spin CPU");
        assert!(backoff <= 30, ">30s defeats the point of NOTIFY");
        assert!(
            backoff_sustained > backoff,
            "sustained-outage backoff must be longer than normal backoff"
        );
        // The demotion's load-bearing number. Below ~5 min we are back to
        // paging on deploys and tunnel re-handshakes; the justification in
        // the constant's doc leans on it being 2x the janitor's
        // claim-steal window.
        assert!(
            sustained >= 2 * stuck_claim,
            "a page must outlive every other self-healing mechanism here"
        );
        assert!(
            sustained <= 3600,
            ">1h of degraded NOTIFY latency should not pass unreported"
        );
        assert!(
            repage >= sustained,
            "re-paging faster than the outage threshold rebuilds the storm"
        );
        assert!(
            healthy < sustained,
            "a session can never prove itself healthy otherwise"
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
        let claimed_at: DateTime<Utc> = DateTime::parse_from_rfc3339("2026-04-25T12:00:00Z")
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
                cr_id: None,
                room_ds_price_total: None,
                room_ds_nights: None,
                room_ds_pay_total: None,
            },
            aggregate_id: Uuid::nil(),
            idempotency_key: Uuid::nil(),
            attempts: 1,
            claimed_at,
            prior: PriorDisposition::RecoveredFromExhausted,
        };
        let cloned = job.clone();
        assert_eq!(cloned.claimed_at, claimed_at);
        assert_eq!(cloned.id, 42);
        assert_eq!(cloned.attempts, 1);
        // The LOW-2 closure alert depends on this field surviving the copy
        // from `claim_next_job` to `mark_done` — it is the ONLY carrier of
        // the pre-claim status (see `PriorDisposition`).
        assert_eq!(cloned.prior, PriorDisposition::RecoveredFromExhausted);
    }

    // -------------------------------------------------------------------
    // Audit LOW-2 — `exhausted → done` detection
    //
    // Regression cover for the dead `:white_check_mark:` closure alert:
    // `mark_done` read the "prior" status AFTER `claim_next_job` had
    // already committed the flip to `in_progress`, so the transition test
    // could never be true. The pre-image is now captured by the claim and
    // classified by this pure helper.
    // -------------------------------------------------------------------

    /// THE regression test. The documented operator recovery — printed in
    /// `send_exhausted_alert`'s own remediation text — is
    /// `SET status='pending', attempts=0, next_retry_at=NULL`. That leaves
    /// a `pending` row still carrying the `last_error` stamped when it
    /// exhausted, and with no scheduled retry. That shape MUST classify as
    /// a recovery, otherwise the closure alert stays dead exactly the way
    /// it was before this fix (a literal `prior_status == "exhausted"`
    /// comparison never matches, because a reset row no longer says
    /// `exhausted` and `claim_next_job` refuses to claim one that does).
    #[test]
    fn prior_disposition_detects_operator_reset_from_exhausted() {
        assert_eq!(
            classify_prior_disposition("pending", true, false),
            PriorDisposition::RecoveredFromExhausted,
            "operator-reset-from-exhausted must be detected — this is the \
             transition the RESOLVED alert exists for"
        );
    }

    /// A literal `exhausted` pre-image also classifies as a recovery.
    /// Unreachable today (the claim predicate excludes the state) but
    /// pinned so widening that predicate can't silently kill detection a
    /// second time.
    #[test]
    fn prior_disposition_detects_literal_exhausted_pre_image() {
        assert_eq!(
            classify_prior_disposition("exhausted", true, false),
            PriorDisposition::RecoveredFromExhausted
        );
        // Residue flags must not override an explicit terminal status.
        assert_eq!(
            classify_prior_disposition("exhausted", false, true),
            PriorDisposition::RecoveredFromExhausted
        );
    }

    /// A never-attempted job is `pending` with no error residue — it must
    /// NOT produce a closure alert, or every ordinary writeback would post
    /// a `:white_check_mark:` and the signal would be worthless.
    #[test]
    fn prior_disposition_fresh_enqueue_is_not_a_recovery() {
        assert_eq!(
            classify_prior_disposition("pending", false, false),
            PriorDisposition::Fresh
        );
    }

    /// Ordinary retries must not produce a closure alert either. Covers
    /// both retry shapes: `failed` with a scheduled backoff, and a stale
    /// `in_progress` claim stolen back from a crashed worker.
    #[test]
    fn prior_disposition_ordinary_retries_are_not_recoveries() {
        assert_eq!(
            classify_prior_disposition("failed", true, true),
            PriorDisposition::Retrying,
            "a backoff retry never paged an operator — no closure to send"
        );
        assert_eq!(
            classify_prior_disposition("in_progress", true, false),
            PriorDisposition::Retrying,
            "a janitor steal of a stuck claim is not an operator recovery"
        );
        assert_eq!(
            classify_prior_disposition("done", false, false),
            PriorDisposition::Retrying
        );
    }

    /// Structural pin: `mark_done` must NOT reintroduce a post-claim read
    /// of the row's status. The original bug was precisely a
    /// `WITH prev AS (SELECT … status AS prior_status …)` CTE inside
    /// `mark_done`, which ran after `claim_next_job` had committed the flip
    /// and therefore could only ever observe `in_progress`. The pre-image
    /// must come from the claim statement.
    #[test]
    fn mark_done_does_not_reread_prior_status_from_pg() {
        let source = include_str!("writeback.rs");
        let fn_start = source
            .find("async fn mark_done(")
            .expect("mark_done must exist");
        let fn_body = &source[fn_start..];
        let body_end = fn_body
            .find("\n/// Write the recipe's allocated legacy identifiers")
            .expect("mark_done must be followed by back_populate_legacy_ids' doc comment");
        let body = &fn_body[..body_end];
        assert!(
            !body.contains("prior_status"),
            "mark_done must not read prior_status from PG — by the time it \
             runs, claim_next_job has already committed status='in_progress', \
             so any such read is dead code. Use ClaimedJob.prior instead."
        );
        assert!(
            body.contains("PriorDisposition::RecoveredFromExhausted"),
            "mark_done must gate the closure alert on the carried \
             PriorDisposition"
        );
    }

    /// The claim statement is the only place the pre-image is observable,
    /// so it must project all three inputs the classifier needs.
    #[test]
    fn claim_next_job_projects_the_pre_image() {
        let source = include_str!("writeback.rs");
        let fn_start = source
            .find("async fn claim_next_job(")
            .expect("claim_next_job must exist");
        let body = &source[fn_start..fn_start + 3000];
        for col in [
            "prior_status",
            "prior_had_error",
            "prior_retry_scheduled",
        ] {
            assert!(
                body.contains(col),
                "claim_next_job must capture `{col}` — the pre-image cannot \
                 be recovered after the claim commits"
            );
        }
        assert!(
            body.contains("FOR UPDATE SKIP LOCKED"),
            "the pre-image CTE must keep the concurrent-claim guard"
        );
    }

    // -------------------------------------------------------------------
    // EXHAUSTED-alert collapse guard
    //
    // Non-retryable errors and panics bypass the retry budget and exhaust
    // on the FIRST attempt, so one bad recipe paged once per affected row
    // at full drain speed — each send `await`ed in the hot path.
    // -------------------------------------------------------------------

    /// First occurrence of a class is always immediate and un-collapsed —
    /// it is the genuinely actionable page and carries the full error text.
    #[test]
    fn exhausted_alert_first_occurrence_sends_immediately() {
        let mut windows = HashMap::new();
        let now = Instant::now();
        let decision = decide_exhausted_alert(
            &mut windows,
            ("create_booking".to_string(), "recipe"),
            now,
            Duration::from_secs(300),
        );
        assert_eq!(decision, ExhaustedAlertDecision::Send { collapsed: 0 });
    }

    /// An immediate repeat of the same `(intent, class)` is collapsed
    /// rather than posted — this is the bad-recipe drain case.
    #[test]
    fn exhausted_alert_repeat_within_window_is_collapsed() {
        let mut windows = HashMap::new();
        let now = Instant::now();
        let key = ("create_booking".to_string(), "recipe");
        let window = Duration::from_secs(300);

        let first = decide_exhausted_alert(&mut windows, key.clone(), now, window);
        assert_eq!(first, ExhaustedAlertDecision::Send { collapsed: 0 });

        // 50 more rows fail the same way while the window is open.
        for expected in 1..=50u32 {
            let d = decide_exhausted_alert(&mut windows, key.clone(), now, window);
            assert_eq!(
                d,
                ExhaustedAlertDecision::Suppress {
                    collapsed: expected,
                    window_secs: 300,
                },
                "repeat #{expected} must be collapsed, not posted"
            );
        }
    }

    /// A DIFFERENT error class must not be collapsed behind an open window
    /// — a schema drift starting during a recipe-failure drain is new
    /// information and has to page.
    #[test]
    fn exhausted_alert_different_error_class_is_not_collapsed() {
        let mut windows = HashMap::new();
        let now = Instant::now();
        let window = Duration::from_secs(300);

        let _ = decide_exhausted_alert(
            &mut windows,
            ("create_booking".to_string(), "recipe"),
            now,
            window,
        );
        // Same intent, different class → sends.
        let drift = decide_exhausted_alert(
            &mut windows,
            ("create_booking".to_string(), "schema_drift"),
            now,
            window,
        );
        assert_eq!(
            drift,
            ExhaustedAlertDecision::Send { collapsed: 0 },
            "a new error class must page even mid-drain of another class"
        );
        // Same class, different intent → also sends.
        let other_intent = decide_exhausted_alert(
            &mut windows,
            ("check_out".to_string(), "recipe"),
            now,
            window,
        );
        assert_eq!(
            other_intent,
            ExhaustedAlertDecision::Send { collapsed: 0 },
            "a broken recipe for one intent must not mask another intent"
        );
    }

    /// After the window expires the next occurrence sends AND reports how
    /// many it absorbed, so nothing is silently dropped from Slack either.
    #[test]
    fn exhausted_alert_window_expiry_sends_with_collapsed_count() {
        let mut windows = HashMap::new();
        let t0 = Instant::now();
        let key = ("create_booking".to_string(), "recipe");
        let window = Duration::from_secs(60);

        let _ = decide_exhausted_alert(&mut windows, key.clone(), t0, window);
        for _ in 0..7 {
            let _ = decide_exhausted_alert(&mut windows, key.clone(), t0, window);
        }

        let t1 = t0 + Duration::from_secs(61);
        let d = decide_exhausted_alert(&mut windows, key.clone(), t1, window);
        assert_eq!(
            d,
            ExhaustedAlertDecision::Send { collapsed: 7 },
            "the follow-up page must state the true suppressed volume"
        );

        // And the counter resets for the new window.
        let d2 = decide_exhausted_alert(&mut windows, key, t1, window);
        assert_eq!(
            d2,
            ExhaustedAlertDecision::Suppress {
                collapsed: 1,
                window_secs: 60,
            }
        );
    }

    /// The key space must stay bounded — idle keys that absorbed nothing
    /// are dropped so a long-lived worker doesn't accumulate one entry per
    /// `(intent, class)` pair ever seen.
    #[test]
    fn exhausted_alert_windows_do_not_grow_unbounded() {
        let mut windows = HashMap::new();
        let t0 = Instant::now();
        let window = Duration::from_secs(60);

        for i in 0..25 {
            let _ = decide_exhausted_alert(
                &mut windows,
                (format!("intent_{i}"), "recipe"),
                t0,
                window,
            );
        }
        assert_eq!(windows.len(), 25);

        // Long after everything expired, one new key prunes the idle ones.
        let t1 = t0 + Duration::from_secs(600);
        let _ = decide_exhausted_alert(&mut windows, ("fresh".to_string(), "panic"), t1, window);
        assert_eq!(
            windows.len(),
            1,
            "expired windows with nothing suppressed must be pruned"
        );
    }

    /// A window that absorbed repeats must survive expiry until its count
    /// has actually been reported — pruning it would silently discard the
    /// suppressed volume.
    #[test]
    fn exhausted_alert_pending_counts_survive_pruning() {
        let mut windows = HashMap::new();
        let t0 = Instant::now();
        let key = ("create_booking".to_string(), "recipe");
        let window = Duration::from_secs(60);

        let _ = decide_exhausted_alert(&mut windows, key.clone(), t0, window);
        let _ = decide_exhausted_alert(&mut windows, key.clone(), t0, window);

        // An unrelated key at a much later time triggers the prune.
        let t1 = t0 + Duration::from_secs(600);
        let _ = decide_exhausted_alert(&mut windows, ("other".to_string(), "panic"), t1, window);

        let d = decide_exhausted_alert(&mut windows, key, t1, window);
        assert_eq!(
            d,
            ExhaustedAlertDecision::Send { collapsed: 1 },
            "the suppressed count must not be lost to pruning"
        );
    }

    /// The collapse key depends on classifying the rendered error message,
    /// so the prefixes must track `WritebackError`'s Display forms and the
    /// messages this binary synthesises. A misclassification would collapse
    /// two unrelated failure modes into one page.
    #[test]
    fn classify_error_kind_separates_the_non_retryable_classes() {
        // These six bypass the retry budget entirely (is_retryable == false)
        // and so exhaust on the FIRST attempt — the flood this guard exists
        // for. Each must get its own key.
        assert_eq!(
            classify_error_kind("recipe error: no prior occupant for room 301"),
            "recipe"
        );
        assert_eq!(
            classify_error_kind("legacy schema drift: expected fingerprint a, got b"),
            "schema_drift"
        );
        assert_eq!(
            classify_error_kind("intent payload mismatch: CheckOut"),
            "intent_mismatch"
        );
        assert_eq!(
            classify_error_kind("payload deserialize: missing field `nights`"),
            "serde"
        );
        assert_eq!(classify_error_kind("config: NEW_DB_NAME unset"), "config");
        assert_eq!(
            classify_error_kind("writeback disabled by WRITEBACK_ENABLED env var"),
            "disabled"
        );
        // Panics skip the budget too (force_exhaust_job from the main loop).
        assert_eq!(classify_error_kind("PANIC: index out of bounds"), "panic");
    }

    /// Retryable/wrapper classes must stay distinct from each other, and a
    /// worker-synthesised wrapper must win over the inner error's prefix.
    #[test]
    fn classify_error_kind_wrapper_prefixes_win_over_inner() {
        assert_eq!(classify_error_kind("tiberius: connection reset"), "tiberius");
        assert_eq!(classify_error_kind("sqlx: pool timed out"), "sqlx");
        assert_eq!(
            classify_error_kind("legacy connection pool: timed out"),
            "pool"
        );
        // The binary wraps these before they reach the alert — the wrapper
        // is the useful class, not the inner driver error.
        assert_eq!(
            classify_error_kind("resolve_legacy_ids: sqlx: row not found"),
            "resolve_legacy_ids"
        );
        assert_eq!(
            classify_error_kind("mssql_acquire: legacy connection pool: timed out"),
            "mssql_acquire"
        );
        assert_eq!(
            classify_error_kind("trancount_reset: tiberius: broken pipe"),
            "trancount_reset"
        );
        assert_eq!(classify_error_kind("something unexpected"), "other");
        assert_eq!(classify_error_kind(""), "other");
    }

    /// The collapse window must be a real throttle but not a black hole.
    /// Bound through a local (same idiom as
    /// `queue_stuck_in_progress_age_mins_is_i32_for_make_interval`) so the
    /// assertion isn't a compile-time constant, and so a type change at the
    /// const site fails here rather than silently.
    #[test]
    fn exhausted_alert_window_is_in_safe_range() {
        let window_secs: u64 = EXHAUSTED_ALERT_WINDOW_SECS;
        assert!(
            window_secs >= 60,
            "<60s barely dents a full-speed queue drain"
        );
        assert!(
            window_secs <= 3600,
            ">1h would hide a genuinely new failure class for too long"
        );
    }

    /// Wave 5a item 4 — the `Err(_)` arm of `mark_done`'s row-match
    /// must skip back-population entirely. Since the function takes
    /// a live `&PgPool` argument we can't unit-test the dispatch
    /// runtime, but we can structurally pin the source-code shape:
    /// the `Err(err) =>` arm in the match must contain an early
    /// `return` BEFORE the back_populate_legacy_ids retry loop.
    /// A regression that drops the `return` would silently let a
    /// stale `legacy_ids` clobber a stolen-claim winner's row.
    ///
    /// ADR 0006 tightened the literal from `return;` to `return false;`
    /// when `mark_done` gained its bool return — which also pins the
    /// second half of the contract: an errored status-flip must not
    /// claim the completion, or the drain loop would announce a
    /// `legacy_stale` hint for a job it may not own.
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
            .find("return false;")
            .expect("Err arm must contain `return false;` per Wave 5a item 4 + ADR 0006");
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

    // -------------------------------------------------------------------
    // W1 / W4 — startup probes must not name a permanent cause on a
    // transient read
    //
    // The collation probe runs FIRST, before the hardened fingerprint
    // guard, and mapped ANY failure (including a bb8 pool timeout on the
    // very first legacy round-trip of the process) onto "Legacy MSSQL
    // collation is case-sensitive". The ledger probe did the same with
    // "`dbo.ht_writeback_ledger` is missing", sending the operator to
    // re-apply a migration that was already applied.
    // -------------------------------------------------------------------

    /// A pool-get timeout / driver error means the read never landed. It
    /// must be retried, and it must never be reported as a bad value.
    #[test]
    fn catalog_probe_transport_failures_are_not_confirmed() {
        assert!(
            !catalog_probe_failure_is_confirmed(&WritebackError::Pool(bb8::RunError::TimedOut)),
            "a bb8 acquire timeout is the tunnel, not the collation"
        );
        assert!(
            !catalog_probe_failure_is_confirmed(&WritebackError::Sqlx(sqlx::Error::PoolTimedOut)),
            "wire-level failures are transient"
        );
    }

    /// `Config` is the only thing the two catalog probes synthesise from an
    /// answer they actually received — a `_CS_` collation name, a NULL
    /// `OBJECT_ID`, an unreadable result shape. That, and only that, may
    /// refuse the boot on the first try.
    #[test]
    fn catalog_probe_bad_answer_is_confirmed() {
        assert!(catalog_probe_failure_is_confirmed(&WritebackError::Config(
            "Legacy server collation is case-sensitive (Thai_CS_AS)".into()
        )));
        assert!(catalog_probe_failure_is_confirmed(&WritebackError::Config(
            "dbo.ht_writeback_ledger missing".into()
        )));
    }

    /// The fingerprint probe keeps its own, narrower rule (W3, incident
    /// 2026-06-28): only a hash mismatch is confirmed. A malformed catalog
    /// read is retried, because re-baselining against a bad read corrupts
    /// the baseline.
    #[test]
    fn fingerprint_probe_confirms_only_a_hash_mismatch() {
        assert!(fingerprint_failure_is_confirmed(
            &WritebackError::SchemaDrift {
                expected: "a".into(),
                actual: "b".into(),
            }
        ));
        assert!(!fingerprint_failure_is_confirmed(&WritebackError::Pool(
            bb8::RunError::TimedOut
        )));
        assert!(
            !fingerprint_failure_is_confirmed(&WritebackError::Config(
                "TABLE_NAME column missing".into()
            )),
            "a malformed catalog read is not drift — this is the 2026-06-28 rule"
        );
    }

    /// A transient collation probe must produce the connectivity message
    /// and must NOT tell the operator to touch the database's collation.
    #[test]
    fn collation_alert_on_transient_does_not_blame_the_collation() {
        let body = collation_probe_alert_body(
            ProbeFailureKind::Unreachable,
            4,
            "legacy connection pool: Timed out in bb8",
        );
        assert!(
            body.contains("could not start"),
            "must not shout REFUSED TO START for a read that never landed"
        );
        assert!(!body.contains("REFUSED TO START"));
        assert!(body.contains("connectivity/timeout problem"));
        assert!(body.contains("NOT a collation problem"));
        assert!(
            body.contains("Do NOT re-collate"),
            "the remediation must be explicitly negated"
        );
        assert!(
            !body.contains("Thai_CI_AS"),
            "naming the expected collation invites a restore that fixes nothing"
        );
        assert!(body.contains("4 attempts"), "must state the retry budget");
    }

    /// A confirmed collation failure still says everything it used to.
    #[test]
    fn collation_alert_on_confirmed_names_the_configuration_problem() {
        let body = collation_probe_alert_body(
            ProbeFailureKind::Confirmed,
            1,
            "config: Legacy server collation is case-sensitive (Thai_CS_AS)",
        );
        assert!(body.contains("REFUSED TO START"));
        assert!(body.contains("the probe read succeeded"));
        assert!(body.contains("Thai_CI_AS"));
        assert!(body.contains("case-insensitive"));
        assert!(body.contains("Thai_CS_AS"), "error text must be carried");
    }

    /// A transient ledger probe must not claim the table is missing — the
    /// probe never got an answer, so its presence is unknown.
    #[test]
    fn ledger_alert_on_transient_does_not_claim_the_table_is_missing() {
        let body = ledger_probe_alert_body(
            ProbeFailureKind::Unreachable,
            4,
            "tiberius: connection reset by peer",
        );
        assert!(body.contains("could not start"));
        assert!(!body.contains("REFUSED TO START"));
        assert!(
            !body.contains("is MISSING"),
            "the claim the operator acts on must not be made on a timeout"
        );
        assert!(body.contains("UNKNOWN"));
        assert!(
            body.contains("Do NOT re-apply"),
            "re-applying an already-applied migration is the wrong action"
        );
        assert!(body.contains("connectivity/timeout problem"));
    }

    /// A confirmed missing ledger still routes to the migration.
    #[test]
    fn ledger_alert_on_confirmed_routes_to_the_migration() {
        let body = ledger_probe_alert_body(
            ProbeFailureKind::Confirmed,
            1,
            "config: dbo.ht_writeback_ledger missing",
        );
        assert!(body.contains("REFUSED TO START"));
        assert!(body.contains("is MISSING"));
        assert!(body.contains("024_writeback_ledger.sql"));
        assert!(body.contains("OBJECT_ID"));
        assert!(!body.contains("Do NOT re-apply"));
    }

    /// W3's two messages are the model the other two now follow — pin them
    /// so a future edit can't quietly re-merge the two stories.
    #[test]
    fn fingerprint_alert_keeps_its_transient_and_drift_split() {
        let drift = fingerprint_probe_alert_body(ProbeFailureKind::Confirmed, 1, "hash a != b");
        assert!(drift.contains("REFUSED TO START"));
        assert!(drift.contains("real drift"));
        assert!(drift.contains("writeback-fingerprint.sh"));

        let transient =
            fingerprint_probe_alert_body(ProbeFailureKind::Unreachable, 4, "tiberius: timeout");
        assert!(transient.contains("could not start"));
        assert!(transient.contains("NOT confirmed schema"));
        assert!(transient.contains("do NOT run `writeback-fingerprint.sh`"));
    }

    /// End-to-end on the retry envelope: a read that fails transiently and
    /// then lands must NOT refuse the boot. `start_paused` auto-advances
    /// the 6s/12s backoff sleeps.
    #[tokio::test(start_paused = true)]
    async fn startup_probe_retries_a_transient_read_then_succeeds() {
        let calls = std::cell::Cell::new(0u32);
        let calls_ref = &calls;
        let out = run_startup_probe(
            "hfhotel",
            "test probe",
            4,
            catalog_probe_failure_is_confirmed,
            move || async move {
                calls_ref.set(calls_ref.get() + 1);
                if calls_ref.get() < 3 {
                    Err(WritebackError::Pool(bb8::RunError::TimedOut))
                } else {
                    Ok(())
                }
            },
        )
        .await;
        assert!(out.is_ok(), "a recovered transient must not refuse the boot");
        assert_eq!(calls.get(), 3);
    }

    /// A read that never lands exhausts the budget and reports
    /// `Unreachable` — the branch that must not name a permanent cause.
    #[tokio::test(start_paused = true)]
    async fn startup_probe_exhausts_transients_as_unreachable() {
        let calls = std::cell::Cell::new(0u32);
        let calls_ref = &calls;
        let out = run_startup_probe(
            "hfhotel",
            "test probe",
            3,
            catalog_probe_failure_is_confirmed,
            move || async move {
                calls_ref.set(calls_ref.get() + 1);
                Err(WritebackError::Pool(bb8::RunError::TimedOut))
            },
        )
        .await;
        let failure = out.expect_err("must fail");
        assert_eq!(failure.kind, ProbeFailureKind::Unreachable);
        assert_eq!(failure.attempts, 3);
        assert_eq!(calls.get(), 3, "every attempt must be made");
        // And the message the operator gets is the connectivity one.
        let body = collation_probe_alert_body(failure.kind, failure.attempts, "…");
        assert!(body.contains("NOT a collation problem"));
    }

    /// A definite bad answer short-circuits: re-asking a question that is
    /// already answered only delays the page.
    #[tokio::test(start_paused = true)]
    async fn startup_probe_short_circuits_a_confirmed_failure() {
        let calls = std::cell::Cell::new(0u32);
        let calls_ref = &calls;
        let out = run_startup_probe(
            "hfhotel",
            "test probe",
            4,
            catalog_probe_failure_is_confirmed,
            move || async move {
                calls_ref.set(calls_ref.get() + 1);
                Err(WritebackError::Config("dbo.ht_writeback_ledger missing".into()))
            },
        )
        .await;
        let failure = out.expect_err("must fail");
        assert_eq!(failure.kind, ProbeFailureKind::Confirmed);
        assert_eq!(failure.attempts, 1);
        assert_eq!(calls.get(), 1, "a definite answer must not be retried");
    }

    /// The attempt budget: new env var wins, the fingerprint probe's
    /// original name still works, garbage and zero fall back to the
    /// default (a `0` would skip the probe entirely).
    #[test]
    fn probe_attempts_parse_precedence_and_floor() {
        assert_eq!(parse_probe_attempts(Some("7"), Some("2")), 7);
        assert_eq!(parse_probe_attempts(None, Some("2")), 2);
        assert_eq!(parse_probe_attempts(Some(" 3 "), None), 3);
        assert_eq!(
            parse_probe_attempts(None, None),
            DEFAULT_STARTUP_PROBE_ATTEMPTS
        );
        assert_eq!(
            parse_probe_attempts(Some("0"), None),
            DEFAULT_STARTUP_PROBE_ATTEMPTS,
            "0 attempts would disable the probe"
        );
        assert_eq!(
            parse_probe_attempts(Some("nope"), None),
            DEFAULT_STARTUP_PROBE_ATTEMPTS
        );
    }

    // -------------------------------------------------------------------
    // W8 — listener UNHEALTHY is a log line until the outage is sustained
    //
    // The supervisor reconnects forever and the worker keeps draining on
    // the 30s poll, so a reconnect loop is self-recovering by design. The
    // old alert fired on 10 consecutive failures, zeroed its counter on
    // fire, and had no cooldown timestamp — one page per ~105s for the
    // whole duration of an outage.
    // -------------------------------------------------------------------

    /// Helper: one failed listener session at `t`.
    fn listener_fail(state: &mut ListenerHealth, t: Instant) -> ListenerDecision {
        decide_listener_action(
            state,
            t,
            false,
            Duration::from_secs(600),
            Duration::from_secs(1800),
        )
    }

    /// Nine minutes of failed reconnects at the 5s cadence — 108 sessions,
    /// which under the old rule would have been ~10 pages — must produce
    /// no Slack traffic at all.
    #[test]
    fn listener_does_not_page_below_the_sustained_threshold() {
        let mut state = ListenerHealth::default();
        let t0 = Instant::now();
        for sec in (0..540).step_by(5) {
            let d = listener_fail(&mut state, t0 + Duration::from_secs(sec));
            assert_eq!(
                d.action,
                ListenerAction::LogOnly,
                "a reconnect loop at +{sec}s is a log line, not a page"
            );
            assert_eq!(
                d.backoff_secs, LISTENER_BACKOFF_SECS,
                "cadence stays fast while the outage may still self-recover"
            );
        }
        assert!(state.consecutive_failures > 100, "sanity: many failures");
    }

    /// Past the threshold it pages exactly once, then holds the re-page
    /// floor for the whole cooldown.
    #[test]
    fn listener_pages_once_when_sustained_then_holds_the_floor() {
        let mut state = ListenerHealth::default();
        let t0 = Instant::now();
        let _ = listener_fail(&mut state, t0);

        // Just before the threshold: still silent.
        let before = listener_fail(&mut state, t0 + Duration::from_secs(599));
        assert_eq!(before.action, ListenerAction::LogOnly);

        // At the threshold: one page.
        let at = listener_fail(&mut state, t0 + Duration::from_secs(600));
        match at.action {
            ListenerAction::Page { outage_secs, .. } => assert_eq!(outage_secs, 600),
            other => panic!("expected a page at the sustained threshold, got {other:?}"),
        }
        assert_eq!(
            at.backoff_secs, LISTENER_BACKOFF_SUSTAINED_SECS,
            "cadence slows once the outage is sustained"
        );

        // Every 60s for the next half hour: silent.
        for sec in (660..2400).step_by(60) {
            let d = listener_fail(&mut state, t0 + Duration::from_secs(sec));
            assert_eq!(
                d.action,
                ListenerAction::LogOnly,
                "re-page floor must hold at +{sec}s"
            );
        }

        // Past the cooldown, one more page restates the outage.
        let repage = listener_fail(&mut state, t0 + Duration::from_secs(2400));
        assert!(
            matches!(repage.action, ListenerAction::Page { .. }),
            "a still-broken listener restates itself once per cooldown"
        );
    }

    /// A healthy session clears the outage clock with no Slack traffic when
    /// nothing was ever paged — the ordinary "PG restarted during a deploy"
    /// case.
    #[test]
    fn listener_healthy_session_clears_the_outage_silently() {
        let mut state = ListenerHealth::default();
        let t0 = Instant::now();
        for sec in (0..120).step_by(5) {
            let _ = listener_fail(&mut state, t0 + Duration::from_secs(sec));
        }
        let recovered = decide_listener_action(
            &mut state,
            t0 + Duration::from_secs(300),
            true,
            Duration::from_secs(600),
            Duration::from_secs(1800),
        );
        assert_eq!(
            recovered.action,
            ListenerAction::LogOnly,
            "an outage nobody was paged for needs no all-clear"
        );
        assert!(!state.paged_this_outage);

        // And the clock restarted: the next page is 600s away from the
        // recovery, not from the original failure.
        let d = listener_fail(&mut state, t0 + Duration::from_secs(899));
        assert_eq!(d.action, ListenerAction::LogOnly);
    }

    /// If the outage DID page, recovery closes it — same pairing rule as
    /// the exhausted-job `:white_check_mark:`.
    #[test]
    fn listener_recovery_posts_an_all_clear_only_after_a_page() {
        let mut state = ListenerHealth::default();
        let t0 = Instant::now();
        let _ = listener_fail(&mut state, t0);
        let paged = listener_fail(&mut state, t0 + Duration::from_secs(700));
        assert!(matches!(paged.action, ListenerAction::Page { .. }));

        let recovered = decide_listener_action(
            &mut state,
            t0 + Duration::from_secs(900),
            true,
            Duration::from_secs(600),
            Duration::from_secs(1800),
        );
        assert_eq!(
            recovered.action,
            ListenerAction::Recovered { outage_secs: 900 }
        );
        // The incident is closed; a later short outage starts from scratch.
        assert!(!state.paged_this_outage);
        assert!(state.last_paged_at.is_none());
    }

    /// A connect-then-instantly-drop flap is NOT a healthy session (the
    /// supervisor gates that on `LISTENER_HEALTHY_SESSION_SECS`), so it
    /// keeps accumulating toward the threshold instead of resetting the
    /// clock on every attempt — otherwise a flapping listener could never
    /// page at all.
    #[test]
    fn listener_flapping_still_reaches_the_sustained_threshold() {
        let mut state = ListenerHealth::default();
        let t0 = Instant::now();
        let mut paged = false;
        for sec in (0..700).step_by(7) {
            // healthy_session=false — the session came up but died in <30s.
            let action = listener_fail(&mut state, t0 + Duration::from_secs(sec)).action;
            if matches!(action, ListenerAction::Page { .. }) {
                paged = true;
            }
        }
        assert!(
            paged,
            "a listener that flaps for 11 minutes is still an outage"
        );
    }

    // -------------------------------------------------------------------
    // ADR 0006 — legacy_stale emission
    // -------------------------------------------------------------------

    /// Ships DARK: unset, empty, `false`, `0` and garbage are all off. Only
    /// an explicit truthy value arms the reception hint.
    #[test]
    fn legacy_stale_flag_defaults_off_and_only_accepts_truthy() {
        assert!(!legacy_stale_notify_enabled(None));
        for off in ["", "  ", "false", "FALSE", "0", "no", "yes please"] {
            assert!(
                !legacy_stale_notify_enabled(Some(off.to_string())),
                "{off:?} must not arm the flag",
            );
        }
        for on in ["true", "TRUE", " True ", "1"] {
            assert!(
                legacy_stale_notify_enabled(Some(on.to_string())),
                "{on:?} must arm the flag",
            );
        }
    }

    /// The flag name is the one `docker-compose.yml` sets (and the only place
    /// flags live — ADR 0004). A rename here silently disables the feature.
    #[test]
    fn legacy_stale_flag_name_matches_compose() {
        assert_eq!(LEGACY_STALE_NOTIFY_FLAG, "LEGACY_STALE_NOTIFY_ENABLED");
    }

    /// `mark_done` must report FALSE when its claim-gated UPDATE matches no
    /// row — the stolen-claim case. That return value is what stops two
    /// workers emitting two `legacy_stale` hints for one real change (the
    /// stealer re-runs the recipe and notifies from its own `mark_done`).
    ///
    /// Needs PG; skipped when `DATABASE_URL` is unset (same convention as
    /// `tests/test_permissions_g7.rs`).
    #[tokio::test]
    async fn mark_done_reports_false_when_the_claim_was_stolen() {
        let Ok(url) = env::var("DATABASE_URL") else {
            eprintln!("skipping: DATABASE_URL unset");
            return;
        };
        let pg = PgPool::connect(&url).await.expect("connect");

        let aggregate_id = Uuid::new_v4();
        let intent = WritebackIntent::MarkRoomDirty {
            room_id: aggregate_id,
            by: "TEST_mark_done_stolen_claim".into(),
        };
        let our_claim = Utc::now();
        let their_claim = our_claim + chrono::Duration::seconds(30);

        // The row as the JANITOR left it: still in_progress, but re-claimed
        // (claimed_at bumped) by another worker while our recipe ran.
        let job_id: i64 = sqlx::query_scalar(
            "INSERT INTO writeback_jobs \
                 (intent, payload, aggregate_id, idempotency_key, status, claimed_at) \
             VALUES ($1, $2, $3, $4, 'in_progress', $5) \
             RETURNING id",
        )
        .bind(intent.intent_name())
        .bind(serde_json::to_value(&intent).unwrap())
        .bind(aggregate_id)
        .bind(Uuid::new_v4())
        .bind(their_claim)
        .fetch_one(&pg)
        .await
        .expect("insert fixture job");

        let landed = mark_done(
            &pg,
            job_id,
            our_claim,
            aggregate_id,
            &intent,
            &None,
            PriorDisposition::Fresh,
            serde_json::json!({}),
        )
        .await;

        assert!(
            !landed,
            "a stolen claim must not report the completion as ours"
        );

        let status: String = sqlx::query_scalar("SELECT status FROM writeback_jobs WHERE id = $1")
            .bind(job_id)
            .fetch_one(&pg)
            .await
            .expect("re-read fixture job");
        assert_eq!(
            status, "in_progress",
            "the stealer's claim must be left alone for it to finish"
        );

        // Same call, matching claim → this worker owns the completion.
        let landed = mark_done(
            &pg,
            job_id,
            their_claim,
            aggregate_id,
            &intent,
            &None,
            PriorDisposition::Fresh,
            serde_json::json!({}),
        )
        .await;
        assert!(landed, "the claim holder's mark_done must report true");

        sqlx::query("DELETE FROM writeback_jobs WHERE id = $1")
            .bind(job_id)
            .execute(&pg)
            .await
            .ok();
    }

    // -------------------------------------------------------------------
    // Issue #202 — CreateCashEntry back-population
    // -------------------------------------------------------------------

    fn cash_entry_intent(cash_aggregate_id: Uuid, amount: f64) -> WritebackIntent {
        WritebackIntent::CreateCashEntry {
            cash_aggregate_id,
            payload: hotel_backend::outbox::intent::CreateCashEntryPayload {
                site_id: "hfhotel".into(),
                entry_date: Utc::now(),
                program_date: None,
                amount,
                legacy_pay_type: "รายจ่าย".into(),
                bill_no: None,
                payee: None,
                note: None,
                group: None,
                account: None,
            },
        }
    }

    /// "mark_done stamps the right row": `back_populate_legacy_ids` must
    /// stamp `cash_legacy_id` on the `ht_cash_ledger` row whose `aggregate_id`
    /// matches the intent — and leave an unrelated row (decoy) untouched.
    ///
    /// Needs PG; skipped when `DATABASE_URL` is unset (same convention as
    /// `mark_done_reports_false_when_the_claim_was_stolen`).
    #[tokio::test]
    async fn back_populate_stamps_cash_legacy_id_on_the_matching_row_only() {
        let Ok(url) = env::var("DATABASE_URL") else {
            eprintln!("skipping: DATABASE_URL unset");
            return;
        };
        let pg = PgPool::connect(&url).await.expect("connect");

        let target_agg = Uuid::new_v4();
        let decoy_agg = Uuid::new_v4();

        let target_row: i64 = sqlx::query_scalar(
            "INSERT INTO ht_cash_ledger (cash_kind, cash_amount, cash_source, aggregate_id) \
             VALUES ('expense', 100.00, 'app', $1) RETURNING cash_id",
        )
        .bind(target_agg)
        .fetch_one(&pg)
        .await
        .expect("insert target fixture row");

        let decoy_row: i64 = sqlx::query_scalar(
            "INSERT INTO ht_cash_ledger (cash_kind, cash_amount, cash_source, aggregate_id) \
             VALUES ('expense', 200.00, 'app', $1) RETURNING cash_id",
        )
        .bind(decoy_agg)
        .fetch_one(&pg)
        .await
        .expect("insert decoy fixture row");

        let intent = cash_entry_intent(target_agg, 100.0);
        back_populate_legacy_ids(
            &pg,
            target_agg,
            &intent,
            &serde_json::json!({"cash_legacy_id": 900_100_001}),
        )
        .await
        .expect("back-populate must succeed");

        let target_legacy: Option<i32> =
            sqlx::query_scalar("SELECT cash_legacy_id FROM ht_cash_ledger WHERE cash_id = $1")
                .bind(target_row)
                .fetch_one(&pg)
                .await
                .expect("re-read target row");
        assert_eq!(
            target_legacy,
            Some(900_100_001),
            "the aggregate_id-matched row must be stamped"
        );

        let decoy_legacy: Option<i32> =
            sqlx::query_scalar("SELECT cash_legacy_id FROM ht_cash_ledger WHERE cash_id = $1")
                .bind(decoy_row)
                .fetch_one(&pg)
                .await
                .expect("re-read decoy row");
        assert_eq!(
            decoy_legacy, None,
            "an unrelated row (different aggregate_id) must NOT be stamped"
        );

        sqlx::query("DELETE FROM ht_cash_ledger WHERE cash_id = $1")
            .bind(target_row)
            .execute(&pg)
            .await
            .ok();
        sqlx::query("DELETE FROM ht_cash_ledger WHERE cash_id = $1")
            .bind(decoy_row)
            .execute(&pg)
            .await
            .ok();
    }

    /// "stolen-claim path does not stamp": mirrors
    /// `mark_done_reports_false_when_the_claim_was_stolen`'s stolen-claim
    /// setup, but asserts the CONCRETE side effect on `ht_cash_ledger`
    /// rather than only the boolean — a stolen claim must leave
    /// `cash_legacy_id` NULL; the re-claimant's own `mark_done` owns the
    /// stamp.
    ///
    /// Needs PG; skipped when `DATABASE_URL` is unset.
    #[tokio::test]
    async fn mark_done_stolen_claim_does_not_stamp_cash_legacy_id() {
        let Ok(url) = env::var("DATABASE_URL") else {
            eprintln!("skipping: DATABASE_URL unset");
            return;
        };
        let pg = PgPool::connect(&url).await.expect("connect");

        let aggregate_id = Uuid::new_v4();
        let row_id: i64 = sqlx::query_scalar(
            "INSERT INTO ht_cash_ledger (cash_kind, cash_amount, cash_source, aggregate_id) \
             VALUES ('income', 300.00, 'app', $1) RETURNING cash_id",
        )
        .bind(aggregate_id)
        .fetch_one(&pg)
        .await
        .expect("insert fixture row");

        let intent = cash_entry_intent(aggregate_id, 300.0);
        let our_claim = Utc::now();
        let their_claim = our_claim + chrono::Duration::seconds(30);

        let job_id: i64 = sqlx::query_scalar(
            "INSERT INTO writeback_jobs \
                 (intent, payload, aggregate_id, idempotency_key, status, claimed_at) \
             VALUES ($1, $2, $3, $4, 'in_progress', $5) \
             RETURNING id",
        )
        .bind(intent.intent_name())
        .bind(serde_json::to_value(&intent).unwrap())
        .bind(aggregate_id)
        .bind(Uuid::new_v4())
        .bind(their_claim)
        .fetch_one(&pg)
        .await
        .expect("insert fixture job");

        let landed = mark_done(
            &pg,
            job_id,
            our_claim,
            aggregate_id,
            &intent,
            &None,
            PriorDisposition::Fresh,
            serde_json::json!({"cash_legacy_id": 900_100_002}),
        )
        .await;

        assert!(
            !landed,
            "a stolen claim must not report the completion as ours"
        );

        let legacy: Option<i32> =
            sqlx::query_scalar("SELECT cash_legacy_id FROM ht_cash_ledger WHERE cash_id = $1")
                .bind(row_id)
                .fetch_one(&pg)
                .await
                .expect("re-read fixture row");
        assert_eq!(
            legacy, None,
            "a stolen claim's mark_done must not stamp cash_legacy_id — the \
             re-claimant's own mark_done owns that"
        );

        sqlx::query("DELETE FROM ht_cash_ledger WHERE cash_id = $1")
            .bind(row_id)
            .execute(&pg)
            .await
            .ok();
        sqlx::query("DELETE FROM writeback_jobs WHERE id = $1")
            .bind(job_id)
            .execute(&pg)
            .await
            .ok();
    }

    /// "echo-recognition": once back-population has stamped `cash_legacy_id`,
    /// a legacy-side re-import of THAT SAME id must UPDATE the existing row,
    /// not insert a duplicate. This reproduces the exact `ON CONFLICT
    /// (cash_legacy_id)` target `sync_cash_history`'s
    /// `CASH_HISTORY_UPSERT_SQL` upserts on (`hotel-backend/src/bin/sync.rs`,
    /// the `const` at line 6192 / `ON CONFLICT` at line 6197, consumed by
    /// `sync_cash_history` at line 6547) against the real `ht_cash_ledger`
    /// UNIQUE constraint — it does NOT call or modify `bin/sync.rs` (out of
    /// this task's ownership; the dedup itself is proven correct there by
    /// `cash_sync_tests::reimport_without_backpopulation_still_duplicates`,
    /// which pins the CONVERSE case — no back-population ⇒ a real duplicate).
    ///
    /// Needs PG; skipped when `DATABASE_URL` is unset.
    #[tokio::test]
    async fn backpopulated_row_absorbs_a_legacy_reimport_instead_of_duplicating() {
        let Ok(url) = env::var("DATABASE_URL") else {
            eprintln!("skipping: DATABASE_URL unset");
            return;
        };
        let pg = PgPool::connect(&url).await.expect("connect");

        let aggregate_id = Uuid::new_v4();
        let legacy_id = 900_100_003;

        let row_id: i64 = sqlx::query_scalar(
            "INSERT INTO ht_cash_ledger (cash_kind, cash_amount, cash_source, aggregate_id) \
             VALUES ('expense', 250.00, 'app', $1) RETURNING cash_id",
        )
        .bind(aggregate_id)
        .fetch_one(&pg)
        .await
        .expect("insert app-originated fixture row");

        let intent = cash_entry_intent(aggregate_id, 250.0);
        back_populate_legacy_ids(
            &pg,
            aggregate_id,
            &intent,
            &serde_json::json!({"cash_legacy_id": legacy_id}),
        )
        .await
        .expect("back-populate must succeed");

        // Simulate the mirror poll re-importing the SAME legacy row under the
        // SAME `ON CONFLICT (cash_legacy_id)` target `CASH_HISTORY_UPSERT_SQL`
        // uses — not the importer function itself.
        sqlx::query(
            "INSERT INTO ht_cash_ledger (cash_legacy_id, cash_kind, cash_amount, cash_source) \
             VALUES ($1, 'expense', 250.00, 'legacy') \
             ON CONFLICT (cash_legacy_id) DO UPDATE SET cash_synced_at = NOW()",
        )
        .bind(legacy_id)
        .execute(&pg)
        .await
        .expect("simulated re-import upsert");

        let total: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM ht_cash_ledger WHERE cash_legacy_id = $1")
                .bind(legacy_id)
                .fetch_one(&pg)
                .await
                .expect("count rows for this legacy id");
        assert_eq!(
            total, 1,
            "back-population must make ON CONFLICT (cash_legacy_id) target the \
             SAME row — otherwise the re-import lands as a duplicate (the gap \
             bin/sync.rs::cash_sync_tests::reimport_without_backpopulation_still_duplicates pins)"
        );

        sqlx::query("DELETE FROM ht_cash_ledger WHERE cash_id = $1")
            .bind(row_id)
            .execute(&pg)
            .await
            .ok();
    }
}
