//! Background sync job: drift tripwire comparing legacy SQL Server state
//! against canonical PostgreSQL state.
//!
//! ## Two modes (Phase 5.5+)
//!
//! Controlled by env var `LEGACY_SYNC_RECONCILE_MODE`:
//!
//! | Mode        | Behaviour                                                 | Default? |
//! |-------------|-----------------------------------------------------------|----------|
//! | `diff_only` | Hash legacy + canonical rows, log divergence to `ht_reconcile_log`. | ✅ yes   |
//! | `upsert`    | Pre-5.5 escape hatch: UPSERT into `ht_*_legacy` mirror.   | dead code path |
//!
//! Phase 5.5 cutover: the CT watcher (`bin/sync.rs`) is now authoritative
//! for canonical PG state, so this job is demoted from a 5-min full-sync
//! UPSERT to a 15-min drift-detection tripwire. If the watcher misses a
//! row (CT retention overflow, transient mapper bug, schema regression),
//! the next reconcile tick lands a row in `ht_reconcile_log` for an
//! operator to investigate. `upsert` mode is preserved purely for forensic
//! rollback flexibility — it is **not exercised by any deployed code path**
//! post v2.63.0.
//!
//! Per docs/architecture.md §3.6d, §8 (Phase 5.5 row).
//!
//! ## What this job compares (v2.63.0+)
//!
//! 1. Customers: `View_Customers`  vs canonical `ht_customers`   (JOIN `legacy_cust_no`)
//! 2. Rooms:     `HT_Rooms`        vs canonical `ht_rooms_new`   (JOIN `legacy_room_no` / `room_no`)
//! 3. Bookings:  `View_Booking_Ds` vs canonical `ht_bookings`    (JOIN `legacy_book_id`)
//! 4. Check-ins: `View_CheckIn_Ds` vs canonical `ht_checkins`    (JOIN `legacy_cin_no`)
//!
//! Pre-v2.63.0 this job compared MSSQL hashes against `ht_*_legacy.sync_hash`
//! (the demoted mirror tables). After the 2026-04-28 cutover those mirrors
//! stopped getting their data columns refreshed — only `sync_hash` /
//! `synced_at` tick — so MSSQL-vs-mirror drift became cosmetic noise
//! (~2300+ unresolved entries observed). v2.63.0 switches the PG side
//! to canonical-table hashes so drift IS actionable (= real CT-mapper gap).
//!
//! The `ht_*_legacy` tables are retained as a **per-PK ack cache**: once a
//! divergence is logged, the current `mssql_hash` is written to
//! `ht_*_legacy.sync_hash` so the next tick short-circuits the same drift
//! instead of re-firing it. The mirror's data columns are intentionally
//! left stale — the CT watcher owns canonical state.

use chrono::NaiveDateTime;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::env;
use std::time::Instant;

use tiberius::Query;

use crate::db::{DbPool, PgPool};
use crate::notifications::slack::{SlackClient, SlackMessage};

/// Default per-table drift-count threshold above which a Slack alert is
/// fired on the next reconcile tick. 50 unresolved rows for a single
/// `table_name` in the last hour is well above the steady-state noise
/// floor (which should be 0) and below the noise level a genuine bulk
/// catch-up scenario would produce. Override at deploy time with
/// `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD`.
pub const DEFAULT_DRIFT_ALERT_THRESHOLD: i64 = 50;

/// Track D / T7 HIGH-1 — level-triggered drift digest cooldown (per
/// table). The edge-triggered alert above fires on a rolling-window
/// volume threshold (50 rows/hr); the level-triggered digest below
/// fires when a table has ANY unresolved divergence older than
/// `LEVEL_DRIFT_STALE_INTERVAL`, capped at one alert per table per
/// `LEVEL_DRIFT_COOLDOWN`. The two are complementary: the edge alert
/// catches bulk regressions, the level alert catches single-row
/// divergences that never trip 50/hr but still represent stuck state.
pub const LEVEL_DRIFT_STALE_INTERVAL_HOURS: i64 = 4;
pub const LEVEL_DRIFT_COOLDOWN_HOURS: i64 = 24;

/// Default per-tick CT watcher lag thresholds. Resolved at runtime via
/// [`ct_lag_thresholds_from_env`]. The version threshold catches a
/// watcher that's fallen behind the legacy CT stream (each MSSQL row
/// update bumps `CHANGE_TRACKING_CURRENT_VERSION()` by 1; 100 versions
/// is well above steady-state idle noise and well below a "the watcher
/// is wedged" scenario). The seconds threshold catches a watcher whose
/// `last_polled_at` row hasn't refreshed in a while — orthogonal to
/// version lag because a silent CT-stream gap also leaves
/// `last_polled_at` advancing while no new versions arrive.
///
/// Background: production incident 2026-05-18 — the CT watcher missed
/// 28+ UPDATEs to `HT_CheckIn_H` / `HT_CheckIn_Ds` between 2026-05-11
/// and 2026-05-15. The only detector was the reconcile sweep itself
/// (which was also broken at the time). This per-tick observation
/// closes the gap: a stuck watermark surfaces as a `[Sync] CT watcher
/// lag detected` log line every 15 minutes.
pub const DEFAULT_CT_LAG_WARN_VERSIONS: i64 = 100;
pub const DEFAULT_CT_LAG_WARN_SECONDS: i64 = 300;

/// Reconcile mode selected by env var `LEGACY_SYNC_RECONCILE_MODE`.
/// Default is `DiffOnly` per Phase 5.5 cutover.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReconcileMode {
    /// Phase 5.5 default. Compare row hashes; LOG divergent rows into
    /// `ht_reconcile_log` for operator investigation. Does NOT mutate
    /// canonical `ht_*` state — the CT watcher owns that.
    DiffOnly,
    /// Pre-5.5 behaviour, kept as an escape hatch. UPSERTs into
    /// `ht_*_legacy` on every divergence (the legacy reconcile path).
    /// Set explicitly only when the CT watcher is operationally
    /// disabled and we need the safety net to keep canonical state in
    /// sync.
    Upsert,
}

impl ReconcileMode {
    /// Read the mode from the env var. Default `DiffOnly`. Anything
    /// other than the two recognised strings logs a warning and falls
    /// back to the safe default.
    pub fn from_env() -> Self {
        match env::var("LEGACY_SYNC_RECONCILE_MODE")
            .as_deref()
            .map(str::trim)
        {
            Ok("upsert") => Self::Upsert,
            Ok("diff_only") | Err(_) => Self::DiffOnly,
            Ok(other) => {
                tracing::warn!(
                    value = other,
                    "Unknown LEGACY_SYNC_RECONCILE_MODE; defaulting to diff_only"
                );
                Self::DiffOnly
            }
        }
    }
}

/// Run a full sync cycle across all entity types.
/// Order: Customers -> Rooms -> Bookings -> CheckIns (respects dependencies).
///
/// Behaviour depends on `LEGACY_SYNC_RECONCILE_MODE` (see [`ReconcileMode`]).
///
/// Phase 6: pass an optional `SlackClient` to enable drift alerting.
/// When the per-table unresolved-drift count in the last hour exceeds
/// [`DEFAULT_DRIFT_ALERT_THRESHOLD`] (override:
/// `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD`, or per-site
/// `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_<SITE_ID_UPPER>`), a Slack
/// message is fired at the end of the cycle. Pass `None` to silence
/// alerts (e.g. from `bin/sync --bootstrap` where the Slack channel is
/// reserved for CT-watcher errors and bootstrap progress).
///
/// `site_id` is plumbed through so the drift-alert message names which
/// deployment fired (HF Hotel vs HF Ville share a Slack webhook from
/// Phase 5 onward) and so the site-specific threshold env var can be
/// looked up.
pub async fn run_sync(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
    slack: Option<&SlackClient>,
    site_id: &str,
) {
    let mode = ReconcileMode::from_env();
    tracing::info!(?mode, site = %site_id, "[Sync] Starting sync cycle...");

    if let Err(e) = sync_customers(legacy_pool, pg_pool).await {
        tracing::error!(site = %site_id, "[Sync] Customer sync failed: {}", e);
        record_error(pg_pool, "customers", &e.to_string()).await;
    }

    if let Err(e) = sync_rooms(legacy_pool, pg_pool).await {
        tracing::error!(site = %site_id, "[Sync] Room sync failed: {}", e);
        record_error(pg_pool, "rooms", &e.to_string()).await;
    }

    if let Err(e) = sync_bookings(legacy_pool, pg_pool).await {
        tracing::error!(site = %site_id, "[Sync] Booking sync failed: {}", e);
        record_error(pg_pool, "bookings", &e.to_string()).await;
    }

    if let Err(e) = sync_checkins(legacy_pool, pg_pool).await {
        tracing::error!(site = %site_id, "[Sync] Check-in sync failed: {}", e);
        record_error(pg_pool, "checkins", &e.to_string()).await;
    }

    // Phase 5.5a: full-table reload of legacy-only dimension tables into
    // legacy_mirror.*. Independent of the canonical reconcile above —
    // these tables are slow-changing reference data (pricing tiers,
    // hourly extension prices), reloaded in their own TX, and don't
    // interact with ReconcileMode (always full-reload).
    crate::scheduler::mirror::reload_mirror_dimensions(legacy_pool, pg_pool).await;

    // Track D / T7 follow-up — auto-resolve previously-recorded
    // divergences whose freshly-projected legacy hash matches the
    // freshly-projected canonical PG hash. BOTH sides are re-hashed
    // under the CURRENT projection so a change to a
    // `*_RECONCILE_PROJECTION` constant self-heals: the next sweep
    // tick re-evaluates rows whose stored `mssql_hash` was computed
    // under the now-superseded projection. Runs before the alert
    // queries so the alerts only surface drift that still persists.
    // Best-effort: a PG failure logs a warning and the alerts
    // proceed with stale state.
    if let Err(e) = auto_resolve_reconcile_log(legacy_pool, pg_pool, site_id).await {
        tracing::warn!(
            site = %site_id,
            error = %e,
            "[Sync] Auto-resolve sweep failed — alerts may include rows that have since converged"
        );
    }

    // Phase 6: drift-alert tripwire. Best-effort — degraded observability
    // never aborts the reconcile loop.
    check_drift_and_alert(pg_pool, slack, site_id).await;

    // Track D / T7 HIGH-1: level-triggered drift digest. Catches
    // long-lived single-row divergences that never breach the
    // edge-triggered 50/hr volume threshold. Per-table cooldown keeps
    // Slack from drowning during a known-bad cardinality migration
    // window.
    check_level_drift_and_alert(pg_pool, slack, site_id).await;

    // Incident 2026-05-18 follow-up — per-tick CT watcher health
    // observation. Compares the PG-side `legacy_ct_state.last_seen_version`
    // against MSSQL's `CHANGE_TRACKING_CURRENT_VERSION()` and warns on
    // lag. Log-only for now (no Slack); operators grep for the warning
    // text. Best-effort: any MSSQL/PG failure logs a warning and returns
    // without aborting the reconcile loop.
    check_ct_watcher_lag(legacy_pool, pg_pool, site_id).await;

    tracing::info!(site = %site_id, "[Sync] Sync cycle complete");
}

/// Phase 6: read the configured drift-alert threshold (default
/// [`DEFAULT_DRIFT_ALERT_THRESHOLD`]). Anything that doesn't parse to a
/// positive integer falls back to the safe default with a warning.
///
/// Task #69 — per-site override. Resolution order, first match wins:
///   1. `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_<SITE_ID_UPPER>` —
///      site-specific knob (e.g. `..._HFVILLE=20` to set a tighter
///      threshold for the smaller property).
///   2. `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD` — global, applies to
///      all sites that don't have a per-site override.
///   3. [`DEFAULT_DRIFT_ALERT_THRESHOLD`] — compiled-in fallback (50).
fn drift_alert_threshold_from_env(site_id: &str) -> i64 {
    let per_site_var =
        format!("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_{}", site_id.to_uppercase());
    parse_threshold_env(&per_site_var)
        .or_else(|| parse_threshold_env("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD"))
        .unwrap_or(DEFAULT_DRIFT_ALERT_THRESHOLD)
}

/// Inner helper: parse a single env var into a positive `i64` threshold.
/// Returns `None` if the var is unset OR parses to a non-positive /
/// non-numeric value (with a warning logged for the latter so an operator
/// notices a typo). Pulled out so the per-site / global fallback chain
/// in [`drift_alert_threshold_from_env`] reads as a single `or_else`.
fn parse_threshold_env(var_name: &str) -> Option<i64> {
    match env::var(var_name) {
        Ok(raw) => match raw.trim().parse::<i64>() {
            Ok(n) if n > 0 => Some(n),
            _ => {
                tracing::warn!(
                    var = var_name,
                    value = %raw,
                    "Invalid drift-alert threshold env var; ignoring"
                );
                None
            }
        },
        Err(_) => None,
    }
}

/// Pure decision function: given per-table unresolved-drift counts in
/// the alerting window, return the `(table_name, count)` pairs that
/// breached the threshold (count strictly greater than threshold).
///
/// Pulled out for unit testing — no PG dependency.
pub fn tables_breaching_threshold(
    counts: &[(String, i64)],
    threshold: i64,
) -> Vec<(String, i64)> {
    counts
        .iter()
        .filter(|(_, n)| *n > threshold)
        .cloned()
        .collect()
}

/// Phase 6 alerting: count unresolved `ht_reconcile_log` rows added in
/// the last hour, grouped by `table_name`. If any table breaches the
/// configured threshold, emit ONE Slack message listing the offenders.
/// Best-effort — a failed Slack POST or PG query only logs a warning.
///
/// Uses `idx_ht_reconcile_log_table_unresolved` (migration 019) for
/// the partial-index group-by.
///
/// **Cardinality excluded** (2026-05-18, post-incident #128): the
/// `cardinality` divergence kind is unack-able by design (see
/// `DivergenceKind::Cardinality` docstring + Track D / T7 CRIT-1) and
/// re-fires from `sync_checkins` every 15-min tick for every multi-room
/// folio that still exists in legacy MSSQL. On HF Hotel that's currently
/// ~766 unique PKs producing ~3000 inserts/hour, drowning any genuine
/// silenceable-drift signal under fixed-volume noise. Cardinality is
/// already covered by `check_level_drift_and_alert` (4h window, 24h
/// per-table cooldown — won't spam), so dropping it from the hourly
/// edge-trigger loses no observability. The hourly alert is now scoped
/// to kinds where re-detection actually means something changed:
/// `value`, `missing_pg`, `missing_mssql`.
async fn check_drift_and_alert(
    pg_pool: &PgPool,
    slack: Option<&SlackClient>,
    site_id: &str,
) {
    let threshold = drift_alert_threshold_from_env(site_id);

    let rows = sqlx::query_as::<_, (String, i64)>(
        "SELECT table_name, count(*) \
           FROM ht_reconcile_log \
          WHERE resolved_at IS NULL \
            AND divergence_kind IS NOT NULL \
            AND divergence_kind <> 'cardinality' \
            AND detected_at > now() - interval '1 hour' \
          GROUP BY table_name",
    )
    .fetch_all(pg_pool)
    .await;

    let counts = match rows {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                error = %e,
                "[Sync] Failed to query ht_reconcile_log for drift alert — observability degraded"
            );
            return;
        }
    };

    let breaches = tables_breaching_threshold(&counts, threshold);
    if breaches.is_empty() {
        tracing::debug!(
            site = %site_id,
            tables_observed = counts.len(),
            threshold,
            "[Sync] Drift alert: no tables breach threshold"
        );
        return;
    }

    // Always log; Slack is opportunistic.
    for (table, count) in &breaches {
        tracing::warn!(
            site = %site_id,
            table,
            count,
            threshold,
            "[Sync] Drift alert: table exceeds reconcile-log threshold in last hour"
        );
    }

    let Some(slack) = slack else {
        tracing::info!(
            site = %site_id,
            "[Sync] Slack not configured; drift alert logged only ({} table(s) breaching)",
            breaches.len()
        );
        return;
    };

    let body = breaches
        .iter()
        .map(|(t, n)| format!("• `{t}`: {n} unresolved rows in last hour"))
        .collect::<Vec<_>>()
        .join("\n");
    let msg = SlackMessage::with_site_text(
        site_id,
        format!(
            ":rotating_light: *Reconcile drift threshold exceeded* :rotating_light:\n\
             The drift-reconcile job recorded more than {threshold} unresolved \
             `ht_reconcile_log` rows for the following table(s) in the last hour:\n\
             {body}\n\
             _Investigate via `docs/runbook-sync.md` §9 (Phase 6 drift alert)._"
        ),
    );
    slack.send_message(&msg).await;
}

/// Track D / T7 HIGH-1 — pure decision function for the level-triggered
/// cooldown gate. Returns true iff the cooldown has elapsed (or there
/// is no prior alert record).
///
/// **Persistence (2026-05-16):** the cooldown state was migrated from
/// a process-local `Mutex<HashMap>` to the PG table
/// `ht_level_drift_alert_cooldowns` (migration 053). The in-memory
/// version was wiped on every backend restart — typical 2-5x/day during
/// active deploy cycles — and the next reconcile tick re-fired the
/// `:warning:` alert despite the documented 24h cooldown. The Slack
/// channel got the same alert repeatedly for the same unchanged
/// backlog. Production incident 2026-05-16.
///
/// This function stays pure (no PG dependency) so the unit tests can
/// inject `now` and `last_alerted_at` directly. The PG layer is in
/// `level_alert_eligible_pg` / `mark_level_alert_sent_pg`.
fn cooldown_elapsed(
    last_alerted_at: Option<chrono::DateTime<chrono::Utc>>,
    now: chrono::DateTime<chrono::Utc>,
    cooldown: std::time::Duration,
) -> bool {
    match last_alerted_at {
        None => true,
        // `signed_duration_since.to_std()` errors when `now < last` — a
        // clock-skew edge case. Treat as "eligible to alert" so we
        // don't get stuck refusing forever on a clock anomaly.
        Some(t) => now
            .signed_duration_since(t)
            .to_std()
            .map_or(true, |elapsed| elapsed >= cooldown),
    }
}

/// PG-backed eligibility check. Reads `last_alerted_at` for
/// `(site_id, table_name)` from `ht_level_drift_alert_cooldowns`,
/// applies the [`cooldown_elapsed`] decision. Returns `true` on PG
/// error so a transient DB blip doesn't silence a legitimate alert.
async fn level_alert_eligible_pg(
    pg_pool: &PgPool,
    site_id: &str,
    table_name: &str,
    cooldown: std::time::Duration,
) -> bool {
    let last = sqlx::query_scalar::<_, chrono::DateTime<chrono::Utc>>(
        "SELECT last_alerted_at FROM ht_level_drift_alert_cooldowns \
          WHERE site_id = $1 AND table_name = $2",
    )
    .bind(site_id)
    .bind(table_name)
    .fetch_optional(pg_pool)
    .await;

    match last {
        Ok(opt) => cooldown_elapsed(opt, chrono::Utc::now(), cooldown),
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                table = %table_name,
                error = %e,
                "[Sync] Failed to read level-drift cooldown row — \
                 defaulting to eligible to avoid silencing real alerts"
            );
            true
        }
    }
}

/// PG-backed cooldown mark. UPSERTs `(site_id, table_name) → NOW()`
/// in `ht_level_drift_alert_cooldowns`. Best-effort: a PG failure logs
/// a warning but doesn't block the alert from going out — the worst
/// case is one extra refire in 15 minutes.
async fn mark_level_alert_sent_pg(
    pg_pool: &PgPool,
    site_id: &str,
    table_name: &str,
) {
    let result = sqlx::query(
        "INSERT INTO ht_level_drift_alert_cooldowns \
            (site_id, table_name, last_alerted_at) \
         VALUES ($1, $2, NOW()) \
         ON CONFLICT (site_id, table_name) \
         DO UPDATE SET last_alerted_at = EXCLUDED.last_alerted_at",
    )
    .bind(site_id)
    .bind(table_name)
    .execute(pg_pool)
    .await;

    if let Err(e) = result {
        tracing::warn!(
            site = %site_id,
            table = %table_name,
            error = %e,
            "[Sync] Failed to persist level-drift cooldown — \
             next tick may refire the alert"
        );
    }
}

/// Track D / T7 HIGH-1 — level-triggered drift digest. Complements the
/// edge-triggered `check_drift_and_alert` above: that one fires on
/// volume (50 rows/hr in a single table), this one fires on persistence
/// (ANY unresolved row older than 4 hours per table). The edge alert
/// catches bulk regressions; the level alert catches single-row
/// divergences that never trip the volume threshold.
///
/// Behaviour:
/// - Counts unresolved rows per `table_name` where `detected_at` is
///   older than `LEVEL_DRIFT_STALE_INTERVAL_HOURS` (default 4h).
/// - For each table with ≥1 such row, emits a Slack alert if the
///   per-table cooldown (`LEVEL_DRIFT_COOLDOWN_HOURS`, default 24h) has
///   elapsed since the last level alert for that table+site.
/// - Best-effort: a failed PG query or Slack POST only logs a warning.
async fn check_level_drift_and_alert(
    pg_pool: &PgPool,
    slack: Option<&SlackClient>,
    site_id: &str,
) {
    let rows = sqlx::query_as::<_, (String, i64)>(
        &format!(
            "SELECT table_name, count(*) \
               FROM ht_reconcile_log \
              WHERE resolved_at IS NULL \
                AND divergence_kind IS NOT NULL \
                AND detected_at < now() - interval '{LEVEL_DRIFT_STALE_INTERVAL_HOURS} hours' \
              GROUP BY table_name"
        ),
    )
    .fetch_all(pg_pool)
    .await;

    let counts = match rows {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                error = %e,
                "[Sync] Failed to query ht_reconcile_log for level-triggered drift digest"
            );
            return;
        }
    };

    if counts.is_empty() {
        tracing::debug!(
            site = %site_id,
            "[Sync] Level drift digest: no tables with unresolved rows older than 4h"
        );
        return;
    }

    let cooldown = std::time::Duration::from_secs((LEVEL_DRIFT_COOLDOWN_HOURS * 3600) as u64);
    let mut to_alert: Vec<(String, i64)> = Vec::new();
    for (table, count) in &counts {
        if level_alert_eligible_pg(pg_pool, site_id, table, cooldown).await {
            to_alert.push((table.clone(), *count));
            mark_level_alert_sent_pg(pg_pool, site_id, table).await;
        } else {
            tracing::debug!(
                site = %site_id,
                table,
                count,
                "[Sync] Level drift alert suppressed by cooldown"
            );
        }
    }

    for (table, count) in &to_alert {
        tracing::warn!(
            site = %site_id,
            table,
            count,
            stale_hours = LEVEL_DRIFT_STALE_INTERVAL_HOURS,
            "[Sync] Level drift alert: table has unresolved divergence older than threshold"
        );
    }

    if to_alert.is_empty() {
        return;
    }

    let Some(slack) = slack else {
        tracing::info!(
            site = %site_id,
            "[Sync] Slack not configured; level drift digest logged only ({} table(s))",
            to_alert.len()
        );
        return;
    };

    let body = to_alert
        .iter()
        .map(|(t, n)| format!("• `{t}`: {n} unresolved row(s)"))
        .collect::<Vec<_>>()
        .join("\n");
    let msg = SlackMessage::with_site_text(
        site_id,
        format!(
            ":warning: *Reconcile drift unresolved >{LEVEL_DRIFT_STALE_INTERVAL_HOURS}h* :warning:\n\
             One or more tables have `ht_reconcile_log` rows that have been \
             unresolved for over {LEVEL_DRIFT_STALE_INTERVAL_HOURS} hours:\n\
             {body}\n\
             _Single-row divergences don't trip the volume threshold but still \
             represent stuck canonical state. Investigate + set \
             `resolved_at = now()` after fixing. Per-table cooldown \
             {LEVEL_DRIFT_COOLDOWN_HOURS}h._"
        ),
    );
    slack.send_message(&msg).await;
}

/// Resolved CT-lag thresholds (versions + seconds) for a reconcile tick.
/// Produced by [`ct_lag_thresholds_from_env`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CtLagThresholds {
    /// `current_version - last_seen_version` greater than this triggers
    /// a warn-level log instead of debug.
    version_lag: i64,
    /// `now() - last_polled_at` (in seconds) greater than this triggers
    /// a warn-level log instead of debug.
    poll_age_seconds: i64,
}

/// Resolve the per-tick CT lag thresholds from env. Mirrors the per-site
/// override pattern used by [`drift_alert_threshold_from_env`]:
/// `LEGACY_CT_LAG_WARN_VERSIONS_<SITE_ID_UPPER>` and
/// `LEGACY_CT_LAG_WARN_SECONDS_<SITE_ID_UPPER>` take precedence over the
/// global `LEGACY_CT_LAG_WARN_VERSIONS` / `LEGACY_CT_LAG_WARN_SECONDS`,
/// which in turn fall back to the compiled-in defaults
/// ([`DEFAULT_CT_LAG_WARN_VERSIONS`] / [`DEFAULT_CT_LAG_WARN_SECONDS`]).
fn ct_lag_thresholds_from_env(site_id: &str) -> CtLagThresholds {
    let site_upper = site_id.to_uppercase();
    let per_site_version = format!("LEGACY_CT_LAG_WARN_VERSIONS_{site_upper}");
    let per_site_seconds = format!("LEGACY_CT_LAG_WARN_SECONDS_{site_upper}");
    let version_lag = parse_threshold_env(&per_site_version)
        .or_else(|| parse_threshold_env("LEGACY_CT_LAG_WARN_VERSIONS"))
        .unwrap_or(DEFAULT_CT_LAG_WARN_VERSIONS);
    let poll_age_seconds = parse_threshold_env(&per_site_seconds)
        .or_else(|| parse_threshold_env("LEGACY_CT_LAG_WARN_SECONDS"))
        .unwrap_or(DEFAULT_CT_LAG_WARN_SECONDS);
    CtLagThresholds {
        version_lag,
        poll_age_seconds,
    }
}

/// Pure decision function for [`check_ct_watcher_lag`]. Returns `true`
/// when the observed lag breaches either threshold (strictly greater
/// than) — i.e. the operator should see a `warn` log instead of `debug`.
///
/// Inputs are kept explicit (no env reads, no clock) so the unit tests
/// can drive the truth table without spinning a tokio runtime or mocking
/// PG/MSSQL.
fn ct_lag_is_warning(
    version_lag: i64,
    poll_age_seconds: i64,
    thresholds: CtLagThresholds,
) -> bool {
    version_lag > thresholds.version_lag || poll_age_seconds > thresholds.poll_age_seconds
}

/// Snapshot of the PG-side CT watermark for the per-tick health check.
#[derive(Debug, Clone, Copy)]
struct PgCtWatermark {
    last_seen_version: i64,
    last_polled_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Read `legacy_ct_state.last_seen_version` + `last_polled_at` from PG.
/// Single-row table (`WHERE id = 1`). A missing row (pre-bootstrap) is
/// surfaced as a `sqlx::Error::RowNotFound` for the caller to log and
/// skip — the watcher hasn't started yet so there's nothing to compare.
async fn read_pg_ct_watermark(pg_pool: &PgPool) -> Result<PgCtWatermark, sqlx::Error> {
    let row = sqlx::query_as::<_, (i64, Option<chrono::DateTime<chrono::Utc>>)>(
        "SELECT last_seen_version, last_polled_at FROM legacy_ct_state WHERE id = 1",
    )
    .fetch_one(pg_pool)
    .await?;
    Ok(PgCtWatermark {
        last_seen_version: row.0,
        last_polled_at: row.1,
    })
}

/// Read `SELECT CHANGE_TRACKING_CURRENT_VERSION()` from legacy MSSQL.
/// Returns `Ok(None)` when CT is not enabled on the database (the
/// function returns NULL in that case); returns `Err` for connection /
/// query failures. The caller logs both as a warning and skips the
/// comparison — neither is fatal to the reconcile tick.
async fn read_mssql_ct_current_version(
    legacy_pool: &DbPool,
) -> Result<Option<i64>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;
    let rows = Query::new("SELECT CHANGE_TRACKING_CURRENT_VERSION() AS v")
        .query(&mut conn)
        .await?
        .into_first_result()
        .await?;
    let Some(row) = rows.first() else {
        return Ok(None);
    };
    Ok(row.get::<i64, _>("v"))
}

/// Incident 2026-05-18 follow-up — per-reconcile-tick CT watcher health
/// observation. Compares the PG-side `legacy_ct_state.last_seen_version`
/// against MSSQL's `CHANGE_TRACKING_CURRENT_VERSION()` and emits a
/// `warn`-level log line if either the version lag or the poll-age
/// exceeds the configured thresholds.
///
/// Log-only by design — no Slack alert. The CT-watcher binary
/// (`bin/sync.rs`) already pages on stuck watermarks via the
/// `watermark_stall_alert_eligible` path; this reconcile-tick check is
/// the SECOND line of defence for the case where the watcher process
/// itself is silent (no logs, no alerts) yet falling behind. Operators
/// grep the backend logs for `[Sync] CT watcher lag detected` to find
/// it. If repeated warns become a pattern we can layer a Slack alert
/// on top later.
///
/// Best-effort: a failed MSSQL or PG query logs a warning and returns
/// without aborting the reconcile loop. Same posture as
/// [`check_drift_and_alert`] / [`check_level_drift_and_alert`] —
/// degraded observability must never take down the rest of the tick.
///
/// **TODO (R3 per-table watermark cutover):** this check reads only the
/// global `legacy_ct_state WHERE id = 1` row. When `SYNC_PER_TABLE_WATERMARK`
/// is enabled (per-table mode, planned for R3), the global row stops
/// advancing and per-table watermarks live in `legacy_ct_state_per_table`.
/// In that mode this check will produce a stuck `version_lag` warning every
/// tick (`last_seen_version` frozen at bootstrap value), masking the real
/// per-table lag. When the per-table flag is flipped, fan this comparison
/// out per tracked table. The watchdog in `bin/sync.rs::watermark_stall_alert_eligible`
/// has the same gap — track both in one follow-up.
async fn check_ct_watcher_lag(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
    site_id: &str,
) {
    let watermark = match read_pg_ct_watermark(pg_pool).await {
        Ok(w) => w,
        Err(sqlx::Error::RowNotFound) => {
            tracing::debug!(
                site = %site_id,
                "[Sync] CT watcher health: legacy_ct_state has no row yet — \
                 watcher pre-bootstrap, skipping lag check"
            );
            return;
        }
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                error = %e,
                "[Sync] CT watcher health: failed to read legacy_ct_state — \
                 observability degraded for this tick"
            );
            return;
        }
    };

    let current_version = match read_mssql_ct_current_version(legacy_pool).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            tracing::warn!(
                site = %site_id,
                "[Sync] CT watcher health: CHANGE_TRACKING_CURRENT_VERSION() \
                 returned NULL — CT not enabled on legacy DB?"
            );
            return;
        }
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                error = %e,
                "[Sync] CT watcher health: failed to probe \
                 CHANGE_TRACKING_CURRENT_VERSION() on legacy MSSQL — \
                 observability degraded for this tick"
            );
            return;
        }
    };

    let last_seen_version = watermark.last_seen_version;
    // Saturating sub: a watermark ahead of `current_version` is a CT
    // anomaly (the version is monotonic) but a defensive zero-lag here
    // is better than a panic on underflow.
    let version_lag = current_version.saturating_sub(last_seen_version);
    let poll_age_seconds = watermark
        .last_polled_at
        .map(|polled| {
            chrono::Utc::now()
                .signed_duration_since(polled)
                .num_seconds()
                .max(0)
        })
        // No `last_polled_at` row yet → watcher hasn't ticked. Treat as
        // "infinitely old" (i64::MAX) so the warn branch fires and the
        // operator sees the never-polled state in the logs.
        .unwrap_or(i64::MAX);

    let thresholds = ct_lag_thresholds_from_env(site_id);
    if ct_lag_is_warning(version_lag, poll_age_seconds, thresholds) {
        tracing::warn!(
            site = %site_id,
            current_version,
            last_seen_version,
            version_lag,
            poll_age_seconds,
            version_threshold = thresholds.version_lag,
            seconds_threshold = thresholds.poll_age_seconds,
            "[Sync] CT watcher lag detected"
        );
    } else {
        tracing::debug!(
            site = %site_id,
            current_version,
            last_seen_version,
            version_lag,
            poll_age_seconds,
            "[Sync] CT watcher healthy"
        );
    }
}

/// Compute SHA256 hash of a string
fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

// =============================================================================
// Multi-row PK aggregation (Phase 6 hotfix 2026-04-29)
// =============================================================================
//
// `View_CheckIn_Ds` and `View_Booking_Ds` both project multiple rows
// per logical PK (one per booked detail/room). Hashing each row
// independently against the single-row-per-PK `ht_*_legacy` cache
// caused the reconcile job to flag the same PKs as drifted on every
// tick (~22-24k/hour Slack spam observed 2026-04-29).
//
// Fix: aggregate rows by PK into a deterministic group, sort the group
// by stable discriminating fields, and hash a deterministic
// concatenation of the entire group. One hash per PK, one
// `record_divergence` per PK, one cache UPDATE per PK.
//
// Helpers below are pure (no PG, no env, no time) so they're
// unit-testable in `mod tests`.

/// One detail-row of a check-in. `View_CheckIn_Ds` returns 41-45 of
/// these per `Cin_no` (one per booked room).
#[derive(Debug, Clone)]
struct CheckinDetail {
    room_no: Option<String>,
    room_in: Option<NaiveDateTime>,
    room_out: Option<NaiveDateTime>,
    cust_name: Option<String>,
    cust_no: Option<String>,
    status: Option<String>,
}

/// One detail-row of a booking. `View_Booking_Ds` returns up to 3 of
/// these per composite PK `(Book_No, Book_Room_Type)`.
#[derive(Debug, Clone)]
struct BookingDetail {
    book_date: Option<NaiveDateTime>,
    book_date_in: Option<NaiveDateTime>,
    book_date_out: Option<NaiveDateTime>,
    book_cust_name: Option<String>,
    book_cust_id: Option<String>,
    book_status: Option<i32>,
    book_room_type: Option<String>,
}

/// Render an `Option<NaiveDateTime>` deterministically. Empty string
/// for `None` so it's distinguishable from the `Debug` `"None"` literal
/// only by absence — both encodings are stable, but using a fixed
/// sentinel avoids the `Some(...)` wrapper noise and matches what the
/// JSON projection emits.
fn fmt_dt(dt: &Option<NaiveDateTime>) -> String {
    dt.map(|d| d.to_string()).unwrap_or_default()
}

/// Render an `Option<String>` deterministically.
fn fmt_str(s: &Option<String>) -> String {
    s.clone().unwrap_or_default()
}

/// Aggregate all detail rows for one `Cin_no` into a single
/// deterministic SHA256 hash. Sorts `details` in place by
/// `(room_no, room_in, room_out)` so the hash is independent of the
/// order MSSQL returned the rows.
///
/// **Retired in v2.63.0** — the multi-row aggregate hash is no longer
/// used by either reconcile mode. Kept for the unit tests in
/// `mod tests` which still pin the determinism contract (useful if
/// the helper is ever resurrected, and useful as a reference for the
/// post-v2.63.0 `sort_checkin_details` extraction).
#[allow(dead_code)]
fn aggregate_checkin_hash(cin_no: &str, details: &mut Vec<CheckinDetail>) -> String {
    details.sort_by(|a, b| {
        (
            fmt_str(&a.room_no),
            fmt_dt(&a.room_in),
            fmt_dt(&a.room_out),
        )
            .cmp(&(
                fmt_str(&b.room_no),
                fmt_dt(&b.room_in),
                fmt_dt(&b.room_out),
            ))
    });

    let body = details
        .iter()
        .map(|d| {
            format!(
                "{}|{}|{}|{}|{}|{}",
                fmt_str(&d.room_no),
                fmt_dt(&d.room_in),
                fmt_dt(&d.room_out),
                fmt_str(&d.cust_name),
                fmt_str(&d.cust_no),
                fmt_str(&d.status),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    sha256(&format!("{cin_no}|{body}"))
}

/// Aggregate all detail rows for one `(Book_No, Book_Room_Type)` PK
/// into a single deterministic SHA256 hash. Sorts `details` in place
/// by `(book_date, book_date_in, book_date_out, book_cust_name,
/// book_cust_id, book_status)` — every non-key field — so the hash is
/// independent of the order MSSQL returned the rows.
///
/// **Retired in v2.63.0** — see [`aggregate_checkin_hash`] for context.
#[allow(dead_code)]
fn aggregate_booking_hash(
    book_no: &str,
    room_type_key: &str,
    details: &mut Vec<BookingDetail>,
) -> String {
    details.sort_by(|a, b| {
        (
            fmt_dt(&a.book_date),
            fmt_dt(&a.book_date_in),
            fmt_dt(&a.book_date_out),
            fmt_str(&a.book_cust_name),
            fmt_str(&a.book_cust_id),
            a.book_status.unwrap_or(i32::MIN),
        )
            .cmp(&(
                fmt_dt(&b.book_date),
                fmt_dt(&b.book_date_in),
                fmt_dt(&b.book_date_out),
                fmt_str(&b.book_cust_name),
                fmt_str(&b.book_cust_id),
                b.book_status.unwrap_or(i32::MIN),
            ))
    });

    let body = details
        .iter()
        .map(|d| {
            format!(
                "{}|{}|{}|{}|{}|{:?}|{}",
                fmt_dt(&d.book_date),
                fmt_dt(&d.book_date_in),
                fmt_dt(&d.book_date_out),
                fmt_str(&d.book_cust_name),
                fmt_str(&d.book_cust_id),
                d.book_status,
                fmt_str(&d.book_room_type),
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    sha256(&format!("{book_no}|{room_type_key}|{body}"))
}

/// Build the `mssql_row_json` payload for a check-in PK group. Includes
/// the full sorted details array so an operator can see what changed
/// across all rooms on the booking — not just one row.
fn checkin_group_json(cin_no: &str, details: &[CheckinDetail]) -> serde_json::Value {
    let rows: Vec<serde_json::Value> = details
        .iter()
        .map(|d| {
            json!({
                "Cin_Room_No": d.room_no,
                // JSON key reflects the projection's actual source column
                // (Bug C fix 2026-05-15) so operators investigating drift
                // see what was hashed, not what they might assume from the
                // schema name.
                "Cin_Date_in": d.room_in.map(|t| t.to_string()),
                "Cin_Room_Out": d.room_out.map(|t| t.to_string()),
                "Cin_cust_name": d.cust_name,
                "Cin_cust_no": d.cust_no,
                "Cin_status": d.status,
            })
        })
        .collect();
    json!({
        "Cin_no": cin_no,
        "details": rows,
    })
}

/// Build the `mssql_row_json` payload for a booking PK group. Includes
/// the full sorted details array.
fn booking_group_json(book_no: &str, details: &[BookingDetail]) -> serde_json::Value {
    let rows: Vec<serde_json::Value> = details
        .iter()
        .map(|d| {
            json!({
                "Book_Date": d.book_date.map(|t| t.to_string()),
                "Book_Date_in": d.book_date_in.map(|t| t.to_string()),
                "Book_Date_out": d.book_date_out.map(|t| t.to_string()),
                "Book_Cust_Name": d.book_cust_name,
                "Book_Cust_ID": d.book_cust_id,
                "Book_Status": d.book_status,
                "Book_Room_Type": d.book_room_type,
            })
        })
        .collect();
    json!({
        "Book_No": book_no,
        "details": rows,
    })
}

// =============================================================================
// Canonical-projection helpers (v2.63.0)
// =============================================================================
//
// The DiffOnly hot path computes TWO hashes per PK and compares them:
//   * `mssql_hash`     — legacy row(s) projected into canonical shape
//   * `canonical_hash` — canonical `ht_*` row(s) projected into the same shape
//
// Both hashes use the SAME field set + SAME serialisation template, so they
// match iff the CT watcher's mapper has faithfully mirrored MSSQL into
// canonical. Drift = actionable mapper gap.
//
// Field set is deliberately narrowed to columns the CT mapper actually
// projects to canonical. Untracked legacy fields (e.g. `Room_Group`,
// `Room_Book_Time`, `Book_Cust_Name` denormalisations) are excluded from
// both hashes — including them would create systematic drift on rows
// where canonical has no corresponding storage, defeating the tripwire's
// signal-to-noise ratio.

/// Reverse of `sync::mappers::room::legacy_yesno_to_bool`. Project a
/// canonical `BOOLEAN` back into the legacy `'yes' | 'no' | ""` literal
/// for hashing. `None → ""` matches what the legacy MSSQL projection
/// emits for a NULL column via `.as_deref().unwrap_or("")`.
fn bool_to_yesno(b: Option<bool>) -> &'static str {
    match b {
        Some(true) => "yes",
        Some(false) => "no",
        None => "",
    }
}

/// Map an MSSQL legacy `Room_Clean`/`Room_Manternace` literal to the
/// canonical-shape `'yes' | 'no' | ""` token. Mirrors the CT room
/// mapper's `legacy_yesno_to_bool`: anything other than the two known
/// literals collapses to `""` so the hashes line up.
fn legacy_yesno_canonical(s: Option<&str>) -> &'static str {
    match s {
        Some("yes") => "yes",
        Some("no") => "no",
        _ => "",
    }
}

/// Hash inputs for the canonical-shape customer projection. Single-row
/// per PK on both sides. Order + separator must stay byte-identical
/// between the MSSQL and PG paths or the hashes won't line up.
fn customer_canonical_hash(
    legacy_cust_no: &str,
    cust_firstname: &str,
    cust_type: Option<&str>,
    cust_phone: Option<&str>,
    cust_idcard: Option<&str>,
    cust_address: Option<&str>,
) -> String {
    sha256(&format!(
        "{}|{}|{}|{}|{}|{}",
        legacy_cust_no,
        cust_firstname,
        cust_type.unwrap_or(""),
        cust_phone.unwrap_or(""),
        cust_idcard.unwrap_or(""),
        cust_address.unwrap_or(""),
    ))
}

/// Hash inputs for the canonical-shape room projection. Narrowed to
/// fields the CT room mapper actually writes back (room_clean,
/// room_maintenance, room_notes) — prices and other legacy-only
/// columns are excluded because canonical doesn't mirror them.
fn room_canonical_hash(
    room_no: &str,
    room_clean_yesno: &str,
    room_maintenance_yesno: &str,
    room_notes: Option<&str>,
) -> String {
    sha256(&format!(
        "{}|{}|{}|{}",
        room_no,
        room_clean_yesno,
        room_maintenance_yesno,
        room_notes.unwrap_or(""),
    ))
}

/// Hash inputs for one canonical-shape booking row. Single-row per
/// `legacy_book_id` (canonical doesn't multi-row by `Book_Room_Type`).
///
/// `book_status` is deliberately excluded: legacy `View_Booking_Ds.Book_Status`
/// is an integer ledger code while canonical `ht_bookings.book_status` is a
/// translated English literal sourced from `HT_Book_H.Book_Status` — different
/// fields. Status changes are surfaced by the CT watcher's domain events.
fn booking_canonical_hash(
    legacy_book_id: &str,
    book_checkin_date: Option<&str>,
    book_checkout_date: Option<&str>,
    legacy_cust_no: Option<&str>,
) -> String {
    sha256(&format!(
        "{}|{}|{}|{}",
        legacy_book_id,
        book_checkin_date.unwrap_or(""),
        book_checkout_date.unwrap_or(""),
        legacy_cust_no.unwrap_or(""),
    ))
}

/// Hash inputs for one canonical-shape check-in row. The CT checkin
/// mapper denormalises only the FIRST room (`legacy_room_no`) into
/// `ht_checkins`, so we hash that single representative row here too.
///
/// `cin_status` is deliberately excluded: `View_CheckIn_Ds.Cin_status`
/// is a per-room ledger state, whereas canonical `ht_checkins.cin_status`
/// is the header-derived aggregate — different fields.
fn checkin_canonical_hash(
    legacy_cin_no: &str,
    legacy_room_no: Option<&str>,
    cin_checkin_time: Option<&str>,
    cin_checkout_time: Option<&str>,
    legacy_cust_no: Option<&str>,
) -> String {
    sha256(&format!(
        "{}|{}|{}|{}|{}",
        legacy_cin_no,
        legacy_room_no.unwrap_or(""),
        cin_checkin_time.unwrap_or(""),
        cin_checkout_time.unwrap_or(""),
        legacy_cust_no.unwrap_or(""),
    ))
}

/// Track D / T7 CRIT-1 — discriminator for `ht_reconcile_log.divergence_kind`.
/// Pure enum so the reconcile loop and the (never-silenced) ack guard
/// agree on the same vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DivergenceKind {
    /// Same row count on both sides; content drift (a CT-mapper bug
    /// projected a column wrong, an operator hand-edited canonical,
    /// etc.). Acked by hash — subsequent ticks short-circuit once the
    /// `mssql_hash` stops changing.
    Value,
    /// Row counts differ between MSSQL and PG. Canonical example: a
    /// multi-room folio (3 rows in `View_CheckIn_Ds`) collapsed into 1
    /// `ht_checkins` row by the CT mapper's `first_room_no`
    /// denormalisation. Hashes will never match while the cardinality
    /// asymmetry exists — Track D / T7 CRIT-1 says this case must
    /// NEVER be acked, so every tick re-fires until operator action
    /// (Track B junction-table migration, or a hand-applied fix).
    Cardinality,
    /// Canonical PG row is missing entirely (`pg_hash IS NULL`). The CT
    /// watcher hasn't yet projected this PK into canonical — could be
    /// a transient watermark lag, a mapper bug, or a CT retention
    /// overflow. Highest-signal divergence; never silenced.
    MissingPg,
    /// Legacy MSSQL row is missing for a PK that exists in canonical.
    /// Should be impossible under normal flow (writeback owns the PG-to-
    /// MSSQL direction) but kept for symmetry — if it ever fires, the
    /// writeback is broken and operator needs to know.
    MissingMssql,
}

impl DivergenceKind {
    /// String literal stored in `ht_reconcile_log.divergence_kind`.
    /// Schema constraint isn't enforced via CHECK so the column can
    /// hold any TEXT — but readers (alerts, dashboards) expect these
    /// exact values.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Value => "value",
            Self::Cardinality => "cardinality",
            Self::MissingPg => "missing_pg",
            Self::MissingMssql => "missing_mssql",
        }
    }

    /// Track D / T7 CRIT-1 invariant: the cache ack must NEVER silence
    /// a cardinality drift or a canonical-missing divergence. Acking
    /// these would write the MSSQL hash into `ht_*_legacy.sync_hash`,
    /// short-circuiting subsequent ticks — but the underlying drift is
    /// not repaired by hash alone, so the alert would go quiet while
    /// canonical stayed wrong forever.
    pub fn is_silenceable(self) -> bool {
        matches!(self, Self::Value | Self::MissingMssql)
    }
}

/// Track D / T7 CRIT-1 — classify a divergence by hash + row-count
/// comparison. Pure function so the unit tests can exercise the truth
/// table without a PG pool.
///
/// * `pg_hash = None` ⇒ `MissingPg` (highest signal).
/// * `mssql_hash = None` ⇒ `MissingMssql`.
/// * `legacy_row_count != pg_row_count` ⇒ `Cardinality` (multi-room
///   folio collapse, junction-table gap).
/// * otherwise ⇒ `Value` (hash mismatch with matching cardinality).
///
/// The caller has already determined that the two hashes differ; this
/// helper just sub-classifies the drift for ack-silencing purposes.
pub fn classify_divergence(
    pg_hash: Option<&str>,
    mssql_hash: Option<&str>,
    legacy_row_count: i32,
    pg_row_count: i32,
) -> DivergenceKind {
    if pg_hash.is_none() {
        return DivergenceKind::MissingPg;
    }
    if mssql_hash.is_none() {
        return DivergenceKind::MissingMssql;
    }
    if legacy_row_count != pg_row_count {
        return DivergenceKind::Cardinality;
    }
    DivergenceKind::Value
}

/// Phase 5.5 diff-only path: record a divergence into `ht_reconcile_log`
/// instead of mutating canonical state. Best-effort — a failed insert
/// only degrades observability, so we never bubble it up to abort the
/// reconcile loop.
///
/// Track D / T7 CRIT-1: every row now carries `divergence_kind` +
/// `legacy_row_count` + `pg_row_count` so cardinality drift (e.g.
/// multi-room folio collapsed by the CT mapper) is distinguishable
/// from value drift, and the ack-cache (`ht_*_legacy.sync_hash`)
/// silencing rule can refuse to silence non-`value` kinds.
#[allow(clippy::too_many_arguments)]
async fn record_divergence(
    pg_pool: &PgPool,
    table_name: &str,
    legacy_pk: &str,
    pg_hash: Option<&str>,
    mssql_hash: Option<&str>,
    mssql_row_json: serde_json::Value,
    pg_row_json: Option<serde_json::Value>,
    divergence_kind: DivergenceKind,
    legacy_row_count: i32,
    pg_row_count: i32,
) {
    let result = sqlx::query(
        "INSERT INTO ht_reconcile_log \
            (table_name, legacy_pk, pg_hash, mssql_hash, \
             mssql_row_json, pg_row_json, \
             divergence_kind, legacy_row_count, pg_row_count) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(table_name)
    .bind(legacy_pk)
    .bind(pg_hash)
    .bind(mssql_hash)
    .bind(mssql_row_json)
    .bind(pg_row_json)
    .bind(divergence_kind.as_str())
    .bind(legacy_row_count)
    .bind(pg_row_count)
    .execute(pg_pool)
    .await;
    if let Err(e) = result {
        tracing::warn!(
            table_name,
            legacy_pk,
            error = %e,
            "[Sync] Failed to record divergence in ht_reconcile_log — observability degraded"
        );
    }
}

/// Pure decision helper for the auto-resolve sweep. A row in
/// `ht_reconcile_log` may be auto-resolved iff a freshly-projected
/// legacy (MSSQL) hash matches a freshly-computed canonical PG hash.
/// Both inputs must be present non-empty strings — a `None` on either
/// side is an intentional skip (missing-PG cases must persist until
/// canonical actually catches up; missing-MSSQL cases need operator
/// review).
///
/// Pulled into a free function so the unit tests can exercise the
/// truth table without a live PG pool.
fn should_auto_resolve(
    current_legacy_hash: Option<&str>,
    current_pg_hash: Option<&str>,
) -> bool {
    match (current_legacy_hash, current_pg_hash) {
        (Some(legacy), Some(pg)) if !legacy.is_empty() && !pg.is_empty() => legacy == pg,
        _ => false,
    }
}

/// Parse the composite booking PK as stored in `ht_reconcile_log.legacy_pk`.
/// Bookings serialise as `"{book_no}|{room_type_key}"`; pre-Phase-6-hotfix
/// rows lack the separator and are interpreted as `(legacy_pk, "")`. Pure
/// so the booking-fetch dispatch + the canonical-fetch dispatch agree on
/// the parse rule.
fn parse_booking_legacy_pk(legacy_pk: &str) -> (&str, &str) {
    legacy_pk.split_once('|').unwrap_or((legacy_pk, ""))
}

/// Re-compute the canonical PG hash for a single `ht_reconcile_log`
/// row's `(table_name, legacy_pk)` pair. Returns `Ok(None)` if no
/// canonical row exists today (still drifted), or `Ok(Some(hash))`
/// when canonical has converged.
///
/// Dispatches on the same table-name vocabulary the reconcile loop
/// writes into `ht_reconcile_log.table_name` ("customers", "bookings",
/// "checkins"). Other table names (e.g. "rooms") are not currently
/// auto-resolvable — they return `Ok(None)` so the row stays in the
/// queue for operator review.
async fn compute_current_pg_hash(
    pg_pool: &PgPool,
    table_name: &str,
    legacy_pk: &str,
) -> Result<Option<String>, sqlx::Error> {
    match table_name {
        "customers" => {
            let canonical = fetch_canonical_customer(pg_pool, legacy_pk).await?;
            Ok(canonical.map(|c| {
                customer_canonical_hash(
                    legacy_pk,
                    &c.cust_firstname,
                    c.cust_type.as_deref(),
                    c.cust_phone.as_deref(),
                    c.cust_idcard.as_deref(),
                    c.cust_address.as_deref(),
                )
            }))
        }
        "bookings" => {
            // Composite PK serialised as "{book_no}|{room_type_key}";
            // canonical hash is keyed by `book_no` (the legacy_book_id)
            // — matches `sync_bookings`' canonical-side hash inputs.
            let (book_no, _room_type_key) = parse_booking_legacy_pk(legacy_pk);
            let canonical = fetch_canonical_booking(pg_pool, book_no).await?;
            Ok(canonical.map(|c| {
                let checkin_str = c.book_checkin.map(|d| d.to_string());
                let checkout_str = c.book_checkout.map(|d| d.to_string());
                booking_canonical_hash(
                    book_no,
                    checkin_str.as_deref(),
                    checkout_str.as_deref(),
                    c.legacy_cust_no.as_deref(),
                )
            }))
        }
        "checkins" => {
            let canonical = fetch_canonical_checkin(pg_pool, legacy_pk).await?;
            Ok(canonical.map(|c| {
                let effective_checkout = c.effective_checkout_date().map(|d| d.to_string());
                checkin_canonical_hash(
                    legacy_pk,
                    c.legacy_room_no.as_deref(),
                    c.cin_checkin_time.map(|t| t.to_string()).as_deref(),
                    effective_checkout.as_deref(),
                    c.legacy_cust_no.as_deref(),
                )
            }))
        }
        _ => Ok(None),
    }
}

/// Re-project a single MSSQL row/group by PK under the CURRENT
/// `*_RECONCILE_PROJECTION` constants and produce a fresh `mssql_hash`.
/// Returns `Ok(None)` when the row no longer exists on the legacy side
/// (treated as "still drifted — leave for operator review"), or when
/// `table_name` is outside the dispatched set ("rooms" / future entities
/// that don't yet have a legacy-side fetch path).
///
/// Mirrors `compute_current_pg_hash` so the auto-resolve sweep can
/// compare like-for-like under the current projection. The whole point
/// of re-fetching MSSQL — rather than trusting `ht_reconcile_log.mssql_hash`
/// — is that a projection change invalidates every pre-fix stored hash.
///
/// **Checkins (2026-05-16):** uses the CT mapper's `project_aggregate`
/// directly — loads the full MSSQL aggregate via `load_checkin_aggregate`
/// then projects through `crate::sync::mappers::project_checkin_aggregate`,
/// producing a `CanonicalCheckIn` whose hash inputs are identical to what
/// the mapper would write into PG. By construction, the resulting hash is
/// what canonical SHOULD be, so a mismatch with the actual canonical row
/// can only signal real exogenous drift (CT miss, hand-edited PG), not
/// parallel-projection-pipeline divergence. Bug B / C / D class becomes
/// impossible at this layer.
///
/// Customers + bookings still use their light-weight per-PK projections
/// (`fetch_legacy_customer_hash`, `fetch_legacy_booking_hash`) — the
/// same unification is a follow-on for those entities.
async fn compute_current_legacy_hash(
    legacy_pool: &DbPool,
    table_name: &str,
    legacy_pk: &str,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    match table_name {
        "customers" => fetch_legacy_customer_hash(legacy_pool, legacy_pk).await,
        "bookings" => {
            let (book_no, room_type_key) = parse_booking_legacy_pk(legacy_pk);
            fetch_legacy_booking_hash(legacy_pool, book_no, room_type_key).await
        }
        "checkins" => compute_legacy_checkin_hash_via_mapper(legacy_pool, legacy_pk).await,
        _ => Ok(None),
    }
}

/// Unified-projection MSSQL hash for a single check-in PK. Loads the
/// full legacy aggregate (header + Ds + Pay) and runs it through the
/// CT mapper's `project_aggregate`, then hashes from the resulting
/// `CanonicalCheckIn` using the same `effective_checkout_date` rule
/// the canonical-side projection uses (see
/// `CanonicalCheckinRow::effective_checkout_date`).
///
/// Cost: 3 MSSQL queries per call (header / Ds / Pay), versus 1 for
/// the old `fetch_legacy_checkin_hash`. The sweep is bounded to 500
/// rows per 15-min tick, so the steady-state load is ~1.7 MSSQL
/// queries/sec — well below saturation on the same-LAN legacy DB.
/// The architectural correctness payoff (no parallel projection
/// pipeline) outweighs the per-call cost.
async fn compute_legacy_checkin_hash_via_mapper(
    legacy_pool: &DbPool,
    cin_no: &str,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let aggregate =
        crate::sync::parent_loader::load_checkin_aggregate(legacy_pool, cin_no).await?;
    if !aggregate.is_present() {
        return Ok(None);
    }
    let canonical = crate::sync::mappers::project_checkin_aggregate(&aggregate, cin_no)?;
    // Effective checkout: prefer actual departure when set (Bug D
    // alignment with canonical-side `effective_checkout_date()`).
    let effective_checkout = canonical
        .cin_checkout_time
        .map(|dt| dt.date())
        .unwrap_or(canonical.cin_expected_checkout)
        .to_string();
    let hash = checkin_canonical_hash(
        &canonical.legacy_cin_no,
        canonical.legacy_room_no.as_deref(),
        Some(canonical.cin_checkin_time.to_string()).as_deref(),
        Some(effective_checkout).as_deref(),
        canonical.legacy_cust_no.as_deref(),
    );
    Ok(Some(hash))
}

/// Single-PK MSSQL re-projection for customers. Mirrors `sync_customers`'
/// per-row hash construction so a projection change in
/// `CUSTOMERS_RECONCILE_PROJECTION` self-heals on the next sweep tick.
async fn fetch_legacy_customer_hash(
    legacy_pool: &DbPool,
    cust_no: &str,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;
    let sql = format!(
        "SELECT {projection} FROM HT_Customers WHERE Cust_no = @P1",
        projection = CUSTOMERS_RECONCILE_PROJECTION,
    );
    let mut q = Query::new(sql);
    q.bind(cust_no);
    let rows = q.query(&mut conn).await?.into_first_result().await?;
    let Some(row) = rows.first() else {
        return Ok(None);
    };
    let row_cust_no = row.get::<&str, _>("Cust_no").unwrap_or_default().to_string();
    let cust_name = row.get::<&str, _>("Cust_name").map(String::from);
    let cust_type = row.get::<&str, _>("Cust_Type_Main").map(String::from);
    let cust_phone = row.get::<&str, _>("Cust_Add_tel").map(String::from);
    let cust_idcard = row.get::<&str, _>("Cust_IDcard").map(String::from);
    let cust_address = row.get::<&str, _>("Cust_Add_no").map(String::from);
    Ok(Some(customer_canonical_hash(
        &row_cust_no,
        cust_name.as_deref().unwrap_or(""),
        cust_type.as_deref(),
        cust_phone.as_deref(),
        cust_idcard.as_deref(),
        cust_address.as_deref(),
    )))
}

/// Single-composite-PK MSSQL re-projection for bookings. `View_Booking_Ds`
/// returns up to 3 rows per `(Book_No, Book_Room_Type)`; we group exactly
/// the same way `sync_bookings` does and take the deterministic
/// representative so the hash inputs match. `Ok(None)` when no matching
/// group exists today (legacy-side deletion since detection).
async fn fetch_legacy_booking_hash(
    legacy_pool: &DbPool,
    book_no: &str,
    room_type_key: &str,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;
    let sql = format!(
        "SELECT {projection} FROM View_Booking_Ds WHERE Book_No = @P1",
        projection = BOOKINGS_RECONCILE_PROJECTION.join(", "),
    );
    let mut q = Query::new(sql);
    q.bind(book_no);
    let rows = q.query(&mut conn).await?.into_first_result().await?;

    let mut groups: BTreeMap<(String, String), Vec<BookingDetail>> = BTreeMap::new();
    for row in &rows {
        let row_book_no = row.get::<&str, _>("Book_No").unwrap_or_default().to_string();
        let row_room_type = row.get::<&str, _>("Book_Room_Type").map(String::from);
        let detail = BookingDetail {
            book_date: row.try_get("Book_Date").unwrap_or(None),
            book_date_in: row.try_get("Book_Date_in").unwrap_or(None),
            book_date_out: row.try_get("Book_Date_out").unwrap_or(None),
            book_cust_name: row.get::<&str, _>("Book_Cust_Name").map(String::from),
            book_cust_id: row.get::<&str, _>("Book_Cust_ID").map(String::from),
            book_status: row.get::<i32, _>("Book_Status"),
            book_room_type: row_room_type.clone(),
        };
        let key = row_room_type.unwrap_or_default();
        groups.entry((row_book_no, key)).or_default().push(detail);
    }

    let Some(mut details) = groups.remove(&(book_no.to_string(), room_type_key.to_string()))
    else {
        return Ok(None);
    };
    sort_booking_details(&mut details);
    let representative = details.first();
    let book_checkin_date = representative
        .and_then(|d| d.book_date_in.map(|dt| dt.date().to_string()));
    let book_checkout_date = representative
        .and_then(|d| d.book_date_out.map(|dt| dt.date().to_string()));
    let book_cust_id_owned = representative.and_then(|d| d.book_cust_id.clone());
    Ok(Some(booking_canonical_hash(
        book_no,
        book_checkin_date.as_deref(),
        book_checkout_date.as_deref(),
        book_cust_id_owned.as_deref(),
    )))
}

/// Track D / T7 follow-up — auto-resolve sweep. Walks unresolved
/// `ht_reconcile_log` rows whose `divergence_kind` was classified by
/// the post-migration-032 reconcile loop, re-projects BOTH the legacy
/// MSSQL row and the canonical PG row under the CURRENT projections,
/// and marks the row resolved when the two fresh hashes converge.
/// Returns the number of rows resolved.
///
/// Rationale: alerts should fire only on drift that still persists.
/// Re-projecting the legacy side (rather than trusting the row's
/// stored `mssql_hash`) is what makes this sweep robust to changes
/// in a `*_RECONCILE_PROJECTION` constant — a fix that renames or
/// re-sources a hashed column would otherwise leave every pre-fix
/// row permanently unresolvable, since the stored hash was computed
/// over a now-defunct field set.
///
/// Bounded to 500 rows per tick so a backlog can't stall the
/// reconcile loop. Best-effort per row — a single MSSQL or PG
/// failure logs and continues to the next.
async fn auto_resolve_reconcile_log(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
    site_id: &str,
) -> Result<usize, sqlx::Error> {
    let rows = sqlx::query_as::<_, (i64, String, String)>(
        "SELECT id, table_name, legacy_pk \
           FROM ht_reconcile_log \
          WHERE resolved_at IS NULL \
            AND divergence_kind IS NOT NULL \
          LIMIT 500",
    )
    .fetch_all(pg_pool)
    .await?;

    let mut resolved = 0usize;
    for (id, table_name, legacy_pk) in rows {
        let current_legacy_hash =
            match compute_current_legacy_hash(legacy_pool, &table_name, &legacy_pk).await {
                Ok(opt) => opt,
                Err(e) => {
                    tracing::warn!(
                        site = %site_id,
                        id,
                        table_name = %table_name,
                        legacy_pk = %legacy_pk,
                        error = %e,
                        "[Sync] Auto-resolve sweep: failed to re-fetch legacy hash, skipping row"
                    );
                    continue;
                }
            };

        let current_pg_hash =
            match compute_current_pg_hash(pg_pool, &table_name, &legacy_pk).await {
                Ok(opt) => opt,
                Err(e) => {
                    tracing::warn!(
                        site = %site_id,
                        id,
                        table_name = %table_name,
                        legacy_pk = %legacy_pk,
                        error = %e,
                        "[Sync] Auto-resolve sweep: failed to re-fetch canonical hash, skipping row"
                    );
                    continue;
                }
            };

        if !should_auto_resolve(current_legacy_hash.as_deref(), current_pg_hash.as_deref()) {
            // Per-row visibility into stuck rows (prod debug 2026-05-18):
            // operators need to see WHY each persistent reconcile_log
            // row isn't converging — (a) legacy hash missing, (b)
            // canonical hash missing, or (c) hashes computed but
            // genuinely don't match. Kept at debug level so it doesn't
            // flood at info; the same field-style as the converged-row
            // debug! below for grep symmetry.
            tracing::debug!(
                site = %site_id,
                id,
                table_name = %table_name,
                legacy_pk = %legacy_pk,
                current_legacy_hash = ?current_legacy_hash,
                current_pg_hash = ?current_pg_hash,
                "[Sync] Auto-resolve sweep: hashes did not converge, leaving row open"
            );
            continue;
        }

        let update = sqlx::query(
            "UPDATE ht_reconcile_log SET resolved_at = NOW() \
              WHERE id = $1 AND resolved_at IS NULL",
        )
        .bind(id)
        .execute(pg_pool)
        .await;

        match update {
            Ok(r) if r.rows_affected() == 1 => {
                resolved += 1;
                tracing::debug!(
                    site = %site_id,
                    id,
                    table_name = %table_name,
                    legacy_pk = %legacy_pk,
                    "[Sync] Auto-resolve sweep: hashes converged, marked resolved"
                );
            }
            Ok(_) => {
                // Row was already resolved by a concurrent path — fine.
            }
            Err(e) => {
                tracing::warn!(
                    site = %site_id,
                    id,
                    error = %e,
                    "[Sync] Auto-resolve sweep: UPDATE failed, leaving row unresolved"
                );
            }
        }
    }

    if resolved > 0 {
        tracing::info!(site = %site_id, resolved, "[Sync] Auto-resolve sweep: rows reconciled");
    }
    Ok(resolved)
}

/// Record a sync error in the sync_status table.
///
/// Phase 6 status: kept for the bootstrap path (`bin/sync --bootstrap`
/// in `Upsert` mode) and for the rare operator-toggled
/// `LEGACY_SYNC_RECONCILE_MODE=upsert` rollback path. The `DiffOnly`
/// hot path also calls it on hard errors so dashboards still surface a
/// per-entity failure counter — observability, not control flow.
async fn record_error(pg_pool: &PgPool, entity: &str, error: &str) {
    let _ = sqlx::query(
        r#"
        UPDATE sync_status
        SET last_error = $1,
            last_error_at = NOW(),
            consecutive_failures = consecutive_failures + 1
        WHERE entity_type = $2
        "#,
    )
    .bind(error)
    .bind(entity)
    .execute(pg_pool)
    .await;
}

/// Update sync_status after a successful sync.
///
/// Phase 6 status: continues to update `sync_status` rows (records
/// counts + duration) for both `Upsert` and `DiffOnly` modes — the
/// dashboard that reads `sync_status` to show "last reconcile tick
/// touched N rows in Mms" depends on this. Do NOT remove without first
/// updating the operator dashboard.
async fn record_success(
    pg_pool: &PgPool,
    entity: &str,
    added: i32,
    updated: i32,
    unchanged: i32,
    duration_ms: i32,
) {
    let total = added + updated + unchanged;
    let _ = sqlx::query(
        r#"
        UPDATE sync_status
        SET last_sync_at = NOW(),
            records_synced = $1,
            records_added = $2,
            records_updated = $3,
            records_unchanged = $4,
            sync_duration_ms = $5,
            consecutive_failures = 0
        WHERE entity_type = $6
        "#,
    )
    .bind(total)
    .bind(added)
    .bind(updated)
    .bind(unchanged)
    .bind(duration_ms)
    .bind(entity)
    .execute(pg_pool)
    .await;
}

// =============================================================================
// Customer Sync
// =============================================================================

/// Canonical-side projection of a customer row for hashing. Loaded by
/// `legacy_cust_no` join on `ht_customers`. Returns `None` when the CT
/// watcher hasn't yet projected this PK into canonical — that PG-miss
/// case is itself a drift signal recorded with `pg_hash=NULL`.
struct CanonicalCustomerRow {
    cust_firstname: String,
    cust_type: Option<String>,
    cust_phone: Option<String>,
    cust_idcard: Option<String>,
    cust_address: Option<String>,
}

async fn fetch_canonical_customer(
    pg_pool: &PgPool,
    legacy_cust_no: &str,
) -> Result<Option<CanonicalCustomerRow>, sqlx::Error> {
    sqlx::query_as::<_, (String, Option<String>, Option<String>, Option<String>, Option<String>)>(
        "SELECT cust_firstname, cust_type, cust_phone, cust_idcard, cust_address \
           FROM ht_customers \
          WHERE legacy_cust_no = $1 \
          LIMIT 1",
    )
    .bind(legacy_cust_no)
    .fetch_optional(pg_pool)
    .await
    .map(|opt| {
        opt.map(|(firstname, type_, phone, idcard, address)| CanonicalCustomerRow {
            cust_firstname: firstname,
            cust_type: type_,
            cust_phone: phone,
            cust_idcard: idcard,
            cust_address: address,
        })
    })
}

/// Best-effort cache UPDATE: ack the most recent `mssql_hash` for this
/// PK so subsequent ticks short-circuit before re-querying canonical.
/// Cache-only — does NOT mutate canonical state. A failed write merely
/// re-fires the alert next tick.
async fn ack_customer_mirror(pg_pool: &PgPool, cust_no: &str, mssql_hash: &str) {
    let updated = sqlx::query(
        "UPDATE ht_customers_legacy SET sync_hash = $1, synced_at = NOW() \
         WHERE cust_no = $2",
    )
    .bind(mssql_hash)
    .bind(cust_no)
    .execute(pg_pool)
    .await
    .map(|r| r.rows_affected())
    .unwrap_or(0);

    if updated == 0 {
        let _ = sqlx::query(
            "INSERT INTO ht_customers_legacy (cust_no, sync_hash, synced_at) \
             VALUES ($1, $2, NOW()) \
             ON CONFLICT (cust_no) DO UPDATE SET sync_hash = EXCLUDED.sync_hash, \
                                                   synced_at = EXCLUDED.synced_at",
        )
        .bind(cust_no)
        .bind(mssql_hash)
        .execute(pg_pool)
        .await;
    }
}

/// Escape-hatch path (mode=upsert): UPSERT the cache mirror's data
/// columns from the MSSQL projection. Not exercised in production after
/// v2.63.0 — preserved for forensic rollback flexibility only.
#[allow(clippy::too_many_arguments)]
async fn upsert_customer_mirror(
    pg_pool: &PgPool,
    cust_no: &str,
    cust_name: &Option<String>,
    cust_type: &Option<String>,
    cust_phone: &Option<String>,
    cust_idcard: &Option<String>,
    cust_address: &Option<String>,
    mssql_hash: &str,
    added: &mut i32,
    updated: &mut i32,
    unchanged: &mut i32,
) -> Result<(), sqlx::Error> {
    let existing = sqlx::query_scalar::<_, Option<String>>(
        "SELECT sync_hash FROM ht_customers_legacy WHERE cust_no = $1",
    )
    .bind(cust_no)
    .fetch_optional(pg_pool)
    .await?;

    match existing {
        Some(Some(existing_hash)) if existing_hash == mssql_hash => {
            *unchanged += 1;
        }
        Some(_) => {
            sqlx::query(
                r#"
                UPDATE ht_customers_legacy
                SET cust_name = $1, cust_type = $2, cust_phone = $3,
                    cust_idcard = $4, cust_address = $5,
                    sync_hash = $6, synced_at = NOW()
                WHERE cust_no = $7
                "#,
            )
            .bind(cust_name)
            .bind(cust_type)
            .bind(cust_phone)
            .bind(cust_idcard)
            .bind(cust_address)
            .bind(mssql_hash)
            .bind(cust_no)
            .execute(pg_pool)
            .await?;
            *updated += 1;
        }
        None => {
            sqlx::query(
                r#"
                INSERT INTO ht_customers_legacy
                    (cust_no, cust_name, cust_type, cust_phone, cust_idcard, cust_address, sync_hash)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                "#,
            )
            .bind(cust_no)
            .bind(cust_name)
            .bind(cust_type)
            .bind(cust_phone)
            .bind(cust_idcard)
            .bind(cust_address)
            .bind(mssql_hash)
            .execute(pg_pool)
            .await?;
            *added += 1;
        }
    }
    Ok(())
}

/// Legacy projection for the customer reconcile hash. Must mirror the
/// CT mapper's column semantics (see `sync::mappers::customer`) so the
/// MSSQL-side hash and the canonical-side hash project the SAME six
/// fields. Drift between this projection and the mapper's writes
/// produces systematic false-positive `ht_reconcile_log` rows that the
/// auto-resolve sweep cannot ever clear.
///
/// **Why `HT_Customers` and NOT `View_Customers`:** the legacy view
/// concatenates `Cust_perfix + Cust_name + ' ' + Cust_name2` into a
/// single `Cust_name` column and builds `C_Address` from eight address
/// components. The CT mapper writes only the base `Cust_name` into
/// `cust_firstname` and only the door number (`Cust_Add_no`) into
/// `cust_address`. Hashing the view's collapsed values against the
/// mapper's component values guarantees a mismatch on every customer
/// that has either a prefix, a secondary name, or a multi-line
/// address.
///
/// **Why `Cust_Type_Main` and NOT `Cust_Type`:** the CT mapper writes
/// `Cust_Type_Main` (the customer-category literal, e.g.
/// `'บุคคลธรรมดา'`) into PG `cust_type`. The view's `Cust_Type` column
/// is actually the *rate-tier* label (e.g. `'ราคาปกติ'`) which the
/// mapper writes into the separate `cust_price_tier` column. The two
/// frequently differ.
const CUSTOMERS_RECONCILE_PROJECTION: &str = "Cust_no, \
                                              Cust_name, \
                                              Cust_Type_Main, \
                                              Cust_Add_tel, \
                                              Cust_IDcard, \
                                              Cust_Add_no";

async fn sync_customers(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    let mode = ReconcileMode::from_env();
    tracing::info!(?mode, "[Sync] Syncing customers...");

    let mut conn = legacy_pool.get().await?;

    let select_sql = format!(
        "SELECT {projection} FROM HT_Customers",
        projection = CUSTOMERS_RECONCILE_PROJECTION,
    );
    let rows = conn
        .simple_query(&select_sql)
        .await?
        .into_first_result()
        .await?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    for row in &rows {
        let cust_no = row.get::<&str, _>("Cust_no").unwrap_or_default().to_string();
        let cust_name = row.get::<&str, _>("Cust_name").map(String::from);
        // `Cust_Type_Main` (not `Cust_Type`) is the column the CT mapper
        // mirrors into PG `cust_type`. See doc on
        // `CUSTOMERS_RECONCILE_PROJECTION` for the rate-tier vs.
        // customer-category distinction.
        let cust_type = row.get::<&str, _>("Cust_Type_Main").map(String::from);
        let cust_phone = row.get::<&str, _>("Cust_Add_tel").map(String::from);
        let cust_idcard = row.get::<&str, _>("Cust_IDcard").map(String::from);
        // `Cust_Add_no` (the door number, not the view's concatenated
        // `C_Address`) is what the CT mapper mirrors into PG
        // `cust_address`. See doc on `CUSTOMERS_RECONCILE_PROJECTION`.
        let cust_address = row.get::<&str, _>("Cust_Add_no").map(String::from);

        // v2.63.0: canonical-shape hash of the MSSQL projection. Same
        // field set + serialisation as `customer_canonical_hash`, so a
        // faithful CT-mapper projection lands an identical hash on the
        // canonical side.
        let mssql_hash = customer_canonical_hash(
            &cust_no,
            cust_name.as_deref().unwrap_or(""),
            cust_type.as_deref(),
            cust_phone.as_deref(),
            cust_idcard.as_deref(),
            cust_address.as_deref(),
        );

        match mode {
            ReconcileMode::Upsert => {
                upsert_customer_mirror(
                    pg_pool,
                    &cust_no,
                    &cust_name,
                    &cust_type,
                    &cust_phone,
                    &cust_idcard,
                    &cust_address,
                    &mssql_hash,
                    &mut added,
                    &mut updated,
                    &mut unchanged,
                )
                .await?;
            }
            ReconcileMode::DiffOnly => {
                let last_ack = sqlx::query_scalar::<_, Option<String>>(
                    "SELECT sync_hash FROM ht_customers_legacy WHERE cust_no = $1",
                )
                .bind(&cust_no)
                .fetch_optional(pg_pool)
                .await?;

                // Dedupe: identical `mssql_hash` as last acknowledged
                // means drift (if any) is already in `ht_reconcile_log`.
                if matches!(&last_ack, Some(Some(prev)) if *prev == mssql_hash) {
                    unchanged += 1;
                    continue;
                }

                let canonical = fetch_canonical_customer(pg_pool, &cust_no).await?;
                let canonical_hash = canonical.as_ref().map(|c| {
                    customer_canonical_hash(
                        &cust_no,
                        &c.cust_firstname,
                        c.cust_type.as_deref(),
                        c.cust_phone.as_deref(),
                        c.cust_idcard.as_deref(),
                        c.cust_address.as_deref(),
                    )
                });

                if canonical_hash.as_deref() == Some(mssql_hash.as_str()) {
                    // Canonical matches legacy. Ack so we skip the
                    // SELECT-join next tick on this stable PK.
                    ack_customer_mirror(pg_pool, &cust_no, &mssql_hash).await;
                    unchanged += 1;
                    continue;
                }

                // Drift: canonical PG-miss (None) or hash mismatch.
                let mssql_json = json!({
                    "Cust_no": cust_no,
                    "Cust_name": cust_name,
                    "Cust_Type": cust_type,
                    "Cust_Add_tel": cust_phone,
                    "Cust_IDcard": cust_idcard,
                    "C_Address": cust_address,
                });
                let pg_json = canonical.as_ref().map(|c| {
                    json!({
                        "cust_firstname": c.cust_firstname,
                        "cust_type": c.cust_type,
                        "cust_phone": c.cust_phone,
                        "cust_idcard": c.cust_idcard,
                        "cust_address": c.cust_address,
                    })
                });
                // Track D / T7 CRIT-1: customers are flat 1:1 PKs on
                // both sides — `legacy_row_count` and `pg_row_count`
                // are 0 or 1 by construction. The pg_row_count == 0
                // case is the `canonical.is_none()` MissingPg path.
                let legacy_row_count: i32 = 1;
                let pg_row_count: i32 = if canonical.is_some() { 1 } else { 0 };
                let kind = classify_divergence(
                    canonical_hash.as_deref(),
                    Some(&mssql_hash),
                    legacy_row_count,
                    pg_row_count,
                );
                record_divergence(
                    pg_pool,
                    "customers",
                    &cust_no,
                    canonical_hash.as_deref(),
                    Some(&mssql_hash),
                    mssql_json,
                    pg_json,
                    kind,
                    legacy_row_count,
                    pg_row_count,
                )
                .await;
                // Track D / T7 CRIT-1: only silence the alert via the
                // cache UPDATE when the kind is silenceable
                // (value-drift). Cardinality + missing_pg refire every
                // tick until operator action.
                if kind.is_silenceable() {
                    ack_customer_mirror(pg_pool, &cust_no, &mssql_hash).await;
                }
                if canonical.is_none() {
                    added += 1;
                } else {
                    updated += 1;
                }
            }
        }
    }

    let duration_ms = start.elapsed().as_millis() as i32;
    tracing::info!(
        "[Sync] Customers ({:?}): {} added, {} updated, {} unchanged in {}ms",
        mode, added, updated, unchanged, duration_ms
    );
    record_success(pg_pool, "customers", added, updated, unchanged, duration_ms).await;

    Ok(())
}

// =============================================================================
// Room Sync
// =============================================================================

/// Canonical-side projection of a room row for hashing. Resolved by
/// `legacy_room_no` first (writeback's preferred key), falling back to
/// `room_no` so rooms that predate the writeback resolver still get
/// compared.
struct CanonicalRoomRow {
    room_clean: Option<bool>,
    room_maintenance: Option<bool>,
    room_notes: Option<String>,
}

async fn fetch_canonical_room(
    pg_pool: &PgPool,
    legacy_room_no: &str,
) -> Result<Option<CanonicalRoomRow>, sqlx::Error> {
    sqlx::query_as::<_, (Option<bool>, Option<bool>, Option<String>)>(
        "SELECT room_clean, room_maintenance, room_notes \
           FROM ht_rooms_new \
          WHERE legacy_room_no = $1 \
             OR room_no = $1 \
          ORDER BY (legacy_room_no = $1) DESC \
          LIMIT 1",
    )
    .bind(legacy_room_no)
    .fetch_optional(pg_pool)
    .await
    .map(|opt| {
        opt.map(|(clean, maintenance, notes)| CanonicalRoomRow {
            room_clean: clean,
            room_maintenance: maintenance,
            room_notes: notes,
        })
    })
}

async fn ack_room_mirror(pg_pool: &PgPool, room_no: &str, mssql_hash: &str) {
    let updated = sqlx::query(
        "UPDATE ht_rooms_legacy SET sync_hash = $1, synced_at = NOW() \
         WHERE room_no = $2",
    )
    .bind(mssql_hash)
    .bind(room_no)
    .execute(pg_pool)
    .await
    .map(|r| r.rows_affected())
    .unwrap_or(0);

    if updated == 0 {
        let _ = sqlx::query(
            "INSERT INTO ht_rooms_legacy (room_no, sync_hash, synced_at) \
             VALUES ($1, $2, NOW()) \
             ON CONFLICT (room_no) DO UPDATE SET sync_hash = EXCLUDED.sync_hash, \
                                                  synced_at = EXCLUDED.synced_at",
        )
        .bind(room_no)
        .bind(mssql_hash)
        .execute(pg_pool)
        .await;
    }
}

#[allow(clippy::too_many_arguments)]
async fn upsert_room_mirror(
    pg_pool: &PgPool,
    room_no: &str,
    room_type: &Option<String>,
    room_details: &Option<String>,
    room_clean: &Option<String>,
    room_use: &Option<String>,
    room_book: &Option<String>,
    room_manternace: &Option<String>,
    room_price_a: Option<f64>,
    room_price_b: Option<f64>,
    room_price_c: Option<f64>,
    room_group: &Option<String>,
    room_book_name: &Option<String>,
    room_book_time: &Option<NaiveDateTime>,
    mssql_hash: &str,
    added: &mut i32,
    updated: &mut i32,
    unchanged: &mut i32,
) -> Result<(), sqlx::Error> {
    let existing = sqlx::query_scalar::<_, Option<String>>(
        "SELECT sync_hash FROM ht_rooms_legacy WHERE room_no = $1",
    )
    .bind(room_no)
    .fetch_optional(pg_pool)
    .await?;

    match existing {
        Some(Some(existing_hash)) if existing_hash == mssql_hash => {
            *unchanged += 1;
        }
        Some(_) => {
            sqlx::query(
                r#"
                UPDATE ht_rooms_legacy
                SET room_type = $1, room_details = $2, room_clean = $3,
                    room_use = $4, room_book = $5, room_manternace = $6,
                    room_price_a = $7::float8, room_price_b = $8::float8,
                    room_price_c = $9::float8, room_group = $10,
                    room_book_name = $11, room_book_time = $12,
                    sync_hash = $13, synced_at = NOW()
                WHERE room_no = $14
                "#,
            )
            .bind(room_type)
            .bind(room_details)
            .bind(room_clean)
            .bind(room_use)
            .bind(room_book)
            .bind(room_manternace)
            .bind(room_price_a)
            .bind(room_price_b)
            .bind(room_price_c)
            .bind(room_group)
            .bind(room_book_name)
            .bind(room_book_time)
            .bind(mssql_hash)
            .bind(room_no)
            .execute(pg_pool)
            .await?;
            *updated += 1;
        }
        None => {
            sqlx::query(
                r#"
                INSERT INTO ht_rooms_legacy
                    (room_no, room_type, room_details, room_clean, room_use,
                     room_book, room_manternace, room_price_a, room_price_b,
                     room_price_c, room_group, room_book_name, room_book_time, sync_hash)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8::float8, $9::float8,
                        $10::float8, $11, $12, $13, $14)
                "#,
            )
            .bind(room_no)
            .bind(room_type)
            .bind(room_details)
            .bind(room_clean)
            .bind(room_use)
            .bind(room_book)
            .bind(room_manternace)
            .bind(room_price_a)
            .bind(room_price_b)
            .bind(room_price_c)
            .bind(room_group)
            .bind(room_book_name)
            .bind(room_book_time)
            .bind(mssql_hash)
            .execute(pg_pool)
            .await?;
            *added += 1;
        }
    }
    Ok(())
}

/// Legacy `HT_Rooms` projection for the room reconcile hash. Held as a
/// module-private const so Track J1's projection-lock test can pin
/// every column against the authoritative HF Hotel schema dump.
///
/// `Room_Manternace` is the legacy schema's verbatim spelling (sic —
/// typo for "Maintenance"). Renaming it client-side would break the
/// SELECT.
const ROOMS_RECONCILE_PROJECTION: &[&str] = &[
    "Room_no",
    "Room_Type",
    "Room_Details",
    "Room_Clean",
    "Room_Use",
    "Room_Book",
    "Room_Manternace",
    "Room_PriceA",
    "Room_PriceB",
    "Room_PriceC",
    "Room_Group",
    "Room_Book_Name",
    "Room_Book_Time",
];

async fn sync_rooms(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    let mode = ReconcileMode::from_env();
    tracing::info!(?mode, "[Sync] Syncing rooms...");

    let mut conn = legacy_pool.get().await?;

    let rooms_select_sql = format!(
        "SELECT {projection} FROM HT_Rooms ORDER BY Room_no",
        projection = ROOMS_RECONCILE_PROJECTION.join(", "),
    );
    let rows = conn
        .simple_query(&rooms_select_sql)
        .await?
        .into_first_result()
        .await?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    for row in &rows {
        let room_no = row.get::<&str, _>("Room_no").unwrap_or_default().to_string();
        // MSSQL projection captured even for fields excluded from the
        // canonical-shape hash — operators reading `ht_reconcile_log`
        // want the full row payload to investigate.
        let room_type = row.get::<&str, _>("Room_Type").map(String::from);
        let room_details = row.get::<&str, _>("Room_Details").map(String::from);
        let room_clean = row.get::<&str, _>("Room_Clean").map(String::from);
        let room_use = row.get::<&str, _>("Room_Use").map(String::from);
        let room_book = row.get::<&str, _>("Room_Book").map(String::from);
        let room_manternace = row.get::<&str, _>("Room_Manternace").map(String::from);
        let room_price_a = row.get::<f64, _>("Room_PriceA");
        let room_price_b = row.get::<f64, _>("Room_PriceB");
        let room_price_c = row.get::<f64, _>("Room_PriceC");
        let room_group = row.get::<&str, _>("Room_Group").map(String::from);
        let room_book_name = row.get::<&str, _>("Room_Book_Name").map(String::from);
        let room_book_time: Option<NaiveDateTime> = row.try_get("Room_Book_Time").unwrap_or(None);

        // v2.63.0: canonical-shape hash over the CT-tracked fields only
        // (room_clean, room_maintenance, room_notes/details). Legacy
        // yes/no literals are folded through `legacy_yesno_canonical`
        // so they line up with the canonical `BOOLEAN` columns via
        // `bool_to_yesno`.
        let mssql_hash = room_canonical_hash(
            &room_no,
            legacy_yesno_canonical(room_clean.as_deref()),
            legacy_yesno_canonical(room_manternace.as_deref()),
            room_details.as_deref(),
        );

        match mode {
            ReconcileMode::Upsert => {
                upsert_room_mirror(
                    pg_pool,
                    &room_no,
                    &room_type,
                    &room_details,
                    &room_clean,
                    &room_use,
                    &room_book,
                    &room_manternace,
                    room_price_a,
                    room_price_b,
                    room_price_c,
                    &room_group,
                    &room_book_name,
                    &room_book_time,
                    &mssql_hash,
                    &mut added,
                    &mut updated,
                    &mut unchanged,
                )
                .await?;
            }
            ReconcileMode::DiffOnly => {
                let last_ack = sqlx::query_scalar::<_, Option<String>>(
                    "SELECT sync_hash FROM ht_rooms_legacy WHERE room_no = $1",
                )
                .bind(&room_no)
                .fetch_optional(pg_pool)
                .await?;

                if matches!(&last_ack, Some(Some(prev)) if *prev == mssql_hash) {
                    unchanged += 1;
                    continue;
                }

                let canonical = fetch_canonical_room(pg_pool, &room_no).await?;
                let canonical_hash = canonical.as_ref().map(|c| {
                    room_canonical_hash(
                        &room_no,
                        bool_to_yesno(c.room_clean),
                        bool_to_yesno(c.room_maintenance),
                        c.room_notes.as_deref(),
                    )
                });

                if canonical_hash.as_deref() == Some(mssql_hash.as_str()) {
                    ack_room_mirror(pg_pool, &room_no, &mssql_hash).await;
                    unchanged += 1;
                    continue;
                }

                let mssql_json = json!({
                    "Room_no": room_no,
                    "Room_Type": room_type,
                    "Room_Details": room_details,
                    "Room_Clean": room_clean,
                    "Room_Use": room_use,
                    "Room_Book": room_book,
                    "Room_Manternace": room_manternace,
                    "Room_PriceA": room_price_a,
                    "Room_PriceB": room_price_b,
                    "Room_PriceC": room_price_c,
                    "Room_Group": room_group,
                    "Room_Book_Name": room_book_name,
                });
                let pg_json = canonical.as_ref().map(|c| {
                    json!({
                        "room_clean": c.room_clean,
                        "room_maintenance": c.room_maintenance,
                        "room_notes": c.room_notes,
                    })
                });
                // Track D / T7 CRIT-1: rooms are flat 1:1 PKs.
                let legacy_row_count: i32 = 1;
                let pg_row_count: i32 = if canonical.is_some() { 1 } else { 0 };
                let kind = classify_divergence(
                    canonical_hash.as_deref(),
                    Some(&mssql_hash),
                    legacy_row_count,
                    pg_row_count,
                );
                record_divergence(
                    pg_pool,
                    "rooms",
                    &room_no,
                    canonical_hash.as_deref(),
                    Some(&mssql_hash),
                    mssql_json,
                    pg_json,
                    kind,
                    legacy_row_count,
                    pg_row_count,
                )
                .await;
                if kind.is_silenceable() {
                    ack_room_mirror(pg_pool, &room_no, &mssql_hash).await;
                }
                if canonical.is_none() {
                    added += 1;
                } else {
                    updated += 1;
                }
            }
        }
        // `room_book_time` is only consumed by the Upsert branch; the
        // explicit reference silences any over-zealous "unused
        // binding" lint in DiffOnly-only execution paths.
        let _ = &room_book_time;
    }

    let duration_ms = start.elapsed().as_millis() as i32;
    tracing::info!(
        "[Sync] Rooms ({:?}): {} added, {} updated, {} unchanged in {}ms",
        mode, added, updated, unchanged, duration_ms
    );
    record_success(pg_pool, "rooms", added, updated, unchanged, duration_ms).await;

    Ok(())
}

// =============================================================================
// Booking Sync
// =============================================================================

/// Legacy `View_Booking_Ds` projection for the booking reconcile hash.
/// Held as a module-private const so Track J1's projection-lock test
/// can pin every column.
///
/// `View_Booking_Ds` joins `HT_Book_H` (header) with `HT_Book_Ds`
/// (per-room detail), so every column in this projection must exist on
/// one of those two base tables.
const BOOKINGS_RECONCILE_PROJECTION: &[&str] = &[
    "Book_No",
    "Book_Date",
    "Book_Date_in",
    "Book_Date_out",
    "Book_Cust_Name",
    "Book_Cust_ID",
    "Book_Status",
    "Book_Room_Type",
];

async fn sync_bookings(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    let mode = ReconcileMode::from_env();
    tracing::info!(?mode, "[Sync] Syncing bookings...");

    let mut conn = legacy_pool.get().await?;

    let bookings_select_sql = format!(
        "SELECT {projection} FROM View_Booking_Ds",
        projection = BOOKINGS_RECONCILE_PROJECTION.join(", "),
    );
    let rows = conn
        .simple_query(&bookings_select_sql)
        .await?
        .into_first_result()
        .await?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    // Phase 6 hotfix (2026-04-29): `View_Booking_Ds` returns up to 3
    // rows per `(Book_No, Book_Room_Type)` composite PK. The previous
    // per-row loop computed a different hash for each iteration of the
    // same PK and re-flagged divergence forever. Aggregate by composite
    // PK first, then hash the whole group deterministically — one
    // record_divergence + one cache UPDATE per PK.
    let mut groups: BTreeMap<(String, String), Vec<BookingDetail>> = BTreeMap::new();
    for row in &rows {
        let book_no = row.get::<&str, _>("Book_No").unwrap_or_default().to_string();
        let book_room_type = row.get::<&str, _>("Book_Room_Type").map(String::from);
        let detail = BookingDetail {
            book_date: row.try_get("Book_Date").unwrap_or(None),
            book_date_in: row.try_get("Book_Date_in").unwrap_or(None),
            book_date_out: row.try_get("Book_Date_out").unwrap_or(None),
            book_cust_name: row.get::<&str, _>("Book_Cust_Name").map(String::from),
            book_cust_id: row.get::<&str, _>("Book_Cust_ID").map(String::from),
            book_status: row.get::<i32, _>("Book_Status"),
            book_room_type: book_room_type.clone(),
        };
        let room_type_key = book_room_type.unwrap_or_default();
        groups.entry((book_no, room_type_key)).or_default().push(detail);
    }

    for ((book_no, room_type_key), mut details) in groups {
        // Deterministic legacy multi-row → single-row collapse: sort by
        // every non-key field (matching `aggregate_booking_hash`'s sort
        // contract) and pick the first detail row as the canonical-shape
        // representative. Canonical `ht_bookings` has one row per
        // `legacy_book_id`, so a single representative is enough.
        sort_booking_details(&mut details);
        let representative = details.first();

        // v2.63.0: canonical-shape hash of the legacy projection.
        // `book_status` intentionally omitted (see `booking_canonical_hash`
        // docs). Dates: legacy returns DATETIME; canonical stores DATE.
        // Drop the time component so both sides hash the same YYYY-MM-DD
        // string (legacy `Book_Date_in/out` are stored at midnight per
        // the booking-create recipe — see `sync::mappers::booking` docs).
        let book_checkin_date = representative
            .and_then(|d| d.book_date_in.map(|dt| dt.date().to_string()));
        let book_checkout_date = representative
            .and_then(|d| d.book_date_out.map(|dt| dt.date().to_string()));
        let book_cust_id_owned = representative.and_then(|d| d.book_cust_id.clone());
        let mssql_hash = booking_canonical_hash(
            &book_no,
            book_checkin_date.as_deref(),
            book_checkout_date.as_deref(),
            book_cust_id_owned.as_deref(),
        );

        match mode {
            ReconcileMode::Upsert => {
                upsert_booking_mirror(
                    pg_pool,
                    &book_no,
                    &room_type_key,
                    representative,
                    &mssql_hash,
                    &mut added,
                    &mut updated,
                    &mut unchanged,
                )
                .await?;
            }
            ReconcileMode::DiffOnly => {
                let last_ack = sqlx::query_scalar::<_, Option<String>>(
                    "SELECT sync_hash FROM ht_bookings_legacy \
                      WHERE book_no = $1 AND COALESCE(book_room_type, '') = $2",
                )
                .bind(&book_no)
                .bind(&room_type_key)
                .fetch_optional(pg_pool)
                .await?;

                if matches!(&last_ack, Some(Some(prev)) if *prev == mssql_hash) {
                    unchanged += 1;
                    continue;
                }

                let canonical = fetch_canonical_booking(pg_pool, &book_no).await?;
                let canonical_hash = canonical.as_ref().map(|c| {
                    let checkin_str = c.book_checkin.map(|d| d.to_string());
                    let checkout_str = c.book_checkout.map(|d| d.to_string());
                    booking_canonical_hash(
                        &book_no,
                        checkin_str.as_deref(),
                        checkout_str.as_deref(),
                        c.legacy_cust_no.as_deref(),
                    )
                });

                if canonical_hash.as_deref() == Some(mssql_hash.as_str()) {
                    ack_booking_mirror(pg_pool, &book_no, &room_type_key, &mssql_hash).await;
                    unchanged += 1;
                    continue;
                }

                let mssql_json = booking_group_json(&book_no, &details);
                let pg_json = canonical.as_ref().map(|c| {
                    json!({
                        "book_checkin": c.book_checkin.map(|d| d.to_string()),
                        "book_checkout": c.book_checkout.map(|d| d.to_string()),
                        "legacy_cust_no": c.legacy_cust_no,
                    })
                });
                let composite_pk = format!("{book_no}|{room_type_key}");
                // Track D / T7 CRIT-1: bookings — `View_Booking_Ds` can
                // return up to 3 rows per composite PK (booking with
                // multiple room types within the same Book_No);
                // canonical `ht_bookings` collapses to one row per
                // `legacy_book_id`. legacy_row_count exposes the raw
                // legacy cardinality so a 3-row Book_No vs 1-row PG
                // mismatch lights up as `Cardinality` instead of being
                // silenced as a value drift.
                let legacy_row_count: i32 = details.len() as i32;
                let pg_row_count: i32 = if canonical.is_some() { 1 } else { 0 };
                let kind = classify_divergence(
                    canonical_hash.as_deref(),
                    Some(&mssql_hash),
                    legacy_row_count,
                    pg_row_count,
                );
                record_divergence(
                    pg_pool,
                    "bookings",
                    &composite_pk,
                    canonical_hash.as_deref(),
                    Some(&mssql_hash),
                    mssql_json,
                    pg_json,
                    kind,
                    legacy_row_count,
                    pg_row_count,
                )
                .await;
                if kind.is_silenceable() {
                    ack_booking_mirror(pg_pool, &book_no, &room_type_key, &mssql_hash).await;
                }
                if canonical.is_none() {
                    added += 1;
                } else {
                    updated += 1;
                }
            }
        }
    }

    let duration_ms = start.elapsed().as_millis() as i32;
    tracing::info!(
        "[Sync] Bookings ({:?}): {} added, {} updated, {} unchanged in {}ms",
        mode, added, updated, unchanged, duration_ms
    );
    record_success(pg_pool, "bookings", added, updated, unchanged, duration_ms).await;

    Ok(())
}

/// Sort a booking PK group's detail rows deterministically. Mirrors the
/// sort contract in [`aggregate_booking_hash`] so the "first" row is
/// stable across reconcile ticks regardless of the order MSSQL returned
/// the detail rows.
fn sort_booking_details(details: &mut [BookingDetail]) {
    details.sort_by(|a, b| {
        (
            fmt_dt(&a.book_date),
            fmt_dt(&a.book_date_in),
            fmt_dt(&a.book_date_out),
            fmt_str(&a.book_cust_name),
            fmt_str(&a.book_cust_id),
            a.book_status.unwrap_or(i32::MIN),
        )
            .cmp(&(
                fmt_dt(&b.book_date),
                fmt_dt(&b.book_date_in),
                fmt_dt(&b.book_date_out),
                fmt_str(&b.book_cust_name),
                fmt_str(&b.book_cust_id),
                b.book_status.unwrap_or(i32::MIN),
            ))
    });
}

/// Canonical-side projection of a booking row for hashing. Resolved by
/// `legacy_book_id`. `legacy_cust_no` is read directly from
/// `ht_bookings.legacy_cust_no` (denormalised by the writeback
/// resolver) rather than joined through `ht_customers` — keeps the
/// drift check resilient to a transient FK gap between bookings and
/// customers in the canonical store.
struct CanonicalBookingRow {
    book_checkin: Option<chrono::NaiveDate>,
    book_checkout: Option<chrono::NaiveDate>,
    legacy_cust_no: Option<String>,
}

async fn fetch_canonical_booking(
    pg_pool: &PgPool,
    legacy_book_id: &str,
) -> Result<Option<CanonicalBookingRow>, sqlx::Error> {
    sqlx::query_as::<_, (Option<chrono::NaiveDate>, Option<chrono::NaiveDate>, Option<String>)>(
        "SELECT book_checkin, book_checkout, legacy_cust_no \
           FROM ht_bookings \
          WHERE legacy_book_id = $1 \
          LIMIT 1",
    )
    .bind(legacy_book_id)
    .fetch_optional(pg_pool)
    .await
    .map(|opt| {
        opt.map(|(checkin, checkout, legacy_cust_no)| CanonicalBookingRow {
            book_checkin: checkin,
            book_checkout: checkout,
            legacy_cust_no,
        })
    })
}

async fn ack_booking_mirror(
    pg_pool: &PgPool,
    book_no: &str,
    room_type_key: &str,
    mssql_hash: &str,
) {
    let updated = sqlx::query(
        "UPDATE ht_bookings_legacy SET sync_hash = $1, synced_at = NOW() \
         WHERE book_no = $2 AND COALESCE(book_room_type, '') = $3",
    )
    .bind(mssql_hash)
    .bind(book_no)
    .bind(room_type_key)
    .execute(pg_pool)
    .await
    .map(|r| r.rows_affected())
    .unwrap_or(0);

    if updated == 0 {
        let _ = sqlx::query(
            "INSERT INTO ht_bookings_legacy (book_no, book_room_type, sync_hash, synced_at) \
             VALUES ($1, $2, $3, NOW()) \
             ON CONFLICT (book_no, book_room_type) DO UPDATE \
               SET sync_hash = EXCLUDED.sync_hash, synced_at = EXCLUDED.synced_at",
        )
        .bind(book_no)
        .bind(room_type_key)
        .bind(mssql_hash)
        .execute(pg_pool)
        .await;
    }
}

#[allow(clippy::too_many_arguments)]
async fn upsert_booking_mirror(
    pg_pool: &PgPool,
    book_no: &str,
    room_type_key: &str,
    representative: Option<&BookingDetail>,
    mssql_hash: &str,
    added: &mut i32,
    updated: &mut i32,
    unchanged: &mut i32,
) -> Result<(), sqlx::Error> {
    let existing = sqlx::query_scalar::<_, Option<String>>(
        "SELECT sync_hash FROM ht_bookings_legacy \
          WHERE book_no = $1 AND COALESCE(book_room_type, '') = $2",
    )
    .bind(book_no)
    .bind(room_type_key)
    .fetch_optional(pg_pool)
    .await?;

    // Empty groups should not occur (we only enter the loop with at
    // least one detail row), but guard so the Upsert path never panics
    // under a degenerate input.
    let Some(rep) = representative else {
        return Ok(());
    };

    match existing {
        Some(Some(existing_hash)) if existing_hash == mssql_hash => {
            *unchanged += 1;
        }
        Some(_) => {
            sqlx::query(
                r#"
                UPDATE ht_bookings_legacy
                SET book_date = $1, book_date_in = $2, book_date_out = $3,
                    book_cust_name = $4, book_cust_id = $5, book_status = $6,
                    sync_hash = $7, synced_at = NOW()
                WHERE book_no = $8 AND COALESCE(book_room_type, '') = $9
                "#,
            )
            .bind(rep.book_date)
            .bind(rep.book_date_in)
            .bind(rep.book_date_out)
            .bind(&rep.book_cust_name)
            .bind(&rep.book_cust_id)
            .bind(rep.book_status)
            .bind(mssql_hash)
            .bind(book_no)
            .bind(room_type_key)
            .execute(pg_pool)
            .await?;
            *updated += 1;
        }
        None => {
            sqlx::query(
                r#"
                INSERT INTO ht_bookings_legacy
                    (book_no, book_date, book_date_in, book_date_out,
                     book_cust_name, book_cust_id, book_status,
                     book_room_type, sync_hash)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                "#,
            )
            .bind(book_no)
            .bind(rep.book_date)
            .bind(rep.book_date_in)
            .bind(rep.book_date_out)
            .bind(&rep.book_cust_name)
            .bind(&rep.book_cust_id)
            .bind(rep.book_status)
            .bind(&rep.book_room_type)
            .bind(mssql_hash)
            .execute(pg_pool)
            .await?;
            *added += 1;
        }
    }
    Ok(())
}

// =============================================================================
// Check-in Sync
// =============================================================================

/// Base-table-qualified projection for the check-in reconcile hash.
/// Held as a module-private const so Track J1's projection-lock test
/// can pin every column to its source table.
///
/// **Why qualified columns + direct JOIN instead of `View_CheckIn_Ds`:**
/// Prod incident 2026-05-15 (242 consecutive `sync_checkins` failures,
/// `Token error: 'Invalid column name 'Cin_Date_in''`). The view's
/// SELECT-list happens to expose `HT_CheckIn_H.Cin_Date` (record-created
/// timestamp) but NOT `HT_CheckIn_H.Cin_Date_in` (guest-arrival
/// timestamp), even though both columns exist on the base header table.
/// The previous projection added `Cin_Date_in` to a `FROM View_CheckIn_Ds`
/// query and every reconcile tick failed at the driver. Reading directly
/// from `HT_CheckIn_H h INNER JOIN HT_CheckIn_Ds d ON h.Cin_no = d.Cin_No`
/// bypasses the view's projection choices entirely — every column on
/// either base table is reachable, and the projection-lock test
/// (`checkins_reconcile_projection_columns_resolve_on_their_tables`)
/// pins each entry to its declared table at CI time.
///
/// Mixed casing is verbatim from the base-table schema dump
/// (`docs/legacy-spike/schema/01-baseline-schema.txt`, lines 208-248):
/// header `HT_CheckIn_H` exposes lowercase `Cin_no` / `Cin_cust_no` /
/// `Cin_status` / `Cin_Date_in`; detail `HT_CheckIn_Ds` exposes
/// capital-N `Cin_No` / `Cin_Room_No` / `Cin_Room_Out`. The JOIN's
/// `ON` clause uses both casings deliberately — SQL Server's default
/// CI collation resolves them to the same identity at runtime but the
/// projection-lock test reads them as literals.
///
/// **Why `Cin_Date_in` and NOT `Cin_Room_In`:** canonical
/// `cin_checkin_time` is sourced from header `HT_CheckIn_H.Cin_Date_in`
/// (via `derive_stay_range` in the CT checkin mapper). Detail
/// `Cin_Room_In` is when the guest physically moved into that
/// specific room — usually equal to `Cin_Date_in` for single-room
/// flows but lags when a room is assigned/added later (we observed
/// ~50min gaps in production audit 2026-05-15). Hashing the detail
/// timestamp against the canonical header timestamp produced
/// systematic false-positive value drift for every check-in with a
/// late room assignment.
///
/// **Why no `Cin_cust_name`:** that column was a view-derived alias
/// from `View_CheckIn_Ds`'s join with `View_Customers` — not a bare
/// column on either base table. It was never part of
/// `checkin_canonical_hash`'s inputs (the hash uses only legacy_pk +
/// legacy_room_no + cin_checkin_time + cin_checkout_time +
/// legacy_cust_no), so dropping it from the projection eliminates the
/// view-only-column dependency without changing the hash. The
/// `CheckinDetail.cust_name` field is preserved as `None` so the
/// upsert-mode mirror's `cin_cust_name` column keeps NULLing rather
/// than disappearing — DiffOnly is the hot path; Upsert is the
/// bootstrap fallback and a NULL there is acceptable.
///
/// Counterpart of the Bug B fix that swapped `Cin_Room_Out` → date-only
/// to align with canonical `cin_expected_checkout`. Both projections
/// now read header columns for the timestamps the CT mapper mirrors.
const CHECKINS_RECONCILE_PROJECTION: &[(&str, &str)] = &[
    ("h", "Cin_no"),
    ("d", "Cin_Room_No"),
    ("h", "Cin_Date_in"),
    ("d", "Cin_Room_Out"),
    ("h", "Cin_cust_no"),
    ("h", "Cin_status"),
];

/// Base-table FROM clause for the check-in reconcile SELECT. Pulled out
/// as a const so the projection-lock test can introspect the JOIN
/// shape without re-parsing the SQL string.
///
/// Note the WHERE-column casing per cheatsheet §3.4 / schema dump:
/// `HT_CheckIn_H.Cin_no` (lowercase n) joins `HT_CheckIn_Ds.Cin_No`
/// (capital N — the discrepancy is verbatim from the legacy schema).
/// `parent_loader::load_checkin_aggregate` documents the same
/// case-asymmetry contract.
const CHECKINS_RECONCILE_FROM_CLAUSE: &str =
    "HT_CheckIn_H h INNER JOIN HT_CheckIn_Ds d ON h.Cin_no = d.Cin_No";

/// Build the `(select_sql, from_clause, projection)` triple for the
/// check-in reconcile read. Pure helper so the projection-lock test
/// can assert the same SQL the production path runs, without
/// duplicating the string-format template.
fn build_checkins_reconcile_select() -> (String, &'static str, &'static [(&'static str, &'static str)]) {
    let projection = CHECKINS_RECONCILE_PROJECTION
        .iter()
        .map(|(alias, col)| format!("{alias}.{col}"))
        .collect::<Vec<_>>()
        .join(", ");
    let sql = format!(
        "SELECT {projection} FROM {from}",
        from = CHECKINS_RECONCILE_FROM_CLAUSE,
    );
    (sql, CHECKINS_RECONCILE_FROM_CLAUSE, CHECKINS_RECONCILE_PROJECTION)
}

async fn sync_checkins(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    let mode = ReconcileMode::from_env();
    tracing::info!(?mode, "[Sync] Syncing check-ins...");

    let mut conn = legacy_pool.get().await?;

    let (checkins_select_sql, _, _) = build_checkins_reconcile_select();
    let rows = conn
        .simple_query(&checkins_select_sql)
        .await?
        .into_first_result()
        .await?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    // Phase 6 hotfix (2026-04-29): the `HT_CheckIn_H ⋈ HT_CheckIn_Ds`
    // join returns 41-45 rows per `Cin_no` (one per booked room/detail).
    // Aggregate by PK first, then compute one deterministic hash per
    // PK, so we record one divergence + one cache UPDATE per PK instead
    // of per-row.
    //
    // tiberius returns each column under its unqualified name (the
    // alias prefix `h.` / `d.` only scopes resolution at parse time);
    // `row.get("Cin_no")` reaches the JOIN output without re-prefixing.
    let mut groups: BTreeMap<String, Vec<CheckinDetail>> = BTreeMap::new();
    for row in &rows {
        let cin_no = row.get::<&str, _>("Cin_no").unwrap_or_default().to_string();
        let detail = CheckinDetail {
            room_no: row.get::<&str, _>("Cin_Room_No").map(String::from),
            // Bug C fix (2026-05-15) + prod hotfix (2026-05-18): read
            // header `Cin_Date_in` directly from `HT_CheckIn_H` — see
            // doc comment on `CHECKINS_RECONCILE_PROJECTION` for the
            // why-not-the-view rationale. The struct field stays
            // `room_in` for now; renaming is a separate cleanup.
            room_in: row.try_get("Cin_Date_in").unwrap_or(None),
            room_out: row.try_get("Cin_Room_Out").unwrap_or(None),
            // `Cin_cust_name` was a view-derived alias and is no longer
            // projected. The field stays so upsert-mode keeps writing
            // `ht_checkins_legacy.cin_cust_name` (as NULL) and the
            // JSON dump shape is unchanged.
            cust_name: None,
            cust_no: row.get::<&str, _>("Cin_cust_no").map(String::from),
            status: row.get::<&str, _>("Cin_status").map(String::from),
        };
        groups.entry(cin_no).or_default().push(detail);
    }

    for (cin_no, mut details) in groups {
        // Same single-row-representative collapse as bookings — see
        // `sort_checkin_details` for the determinism contract.
        sort_checkin_details(&mut details);
        let representative = details.first();

        // v2.63.0: canonical-shape hash of the legacy projection.
        // `cin_status` intentionally omitted (see `checkin_canonical_hash`
        // docs). `cin_checkin_time` keeps `NaiveDateTime::to_string()`
        // (`YYYY-MM-DD HH:MM:SS`) on both sides. `Cin_Room_Out` drops the
        // time component to align with canonical `cin_expected_checkout`
        // (a `DATE` — see `CanonicalCheckinRow` doc comment for the
        // field-semantic rationale).
        let room_in_str = representative.and_then(|d| d.room_in.map(|t| t.to_string()));
        let room_out_date_str =
            representative.and_then(|d| d.room_out.map(|t| t.date().to_string()));
        let mssql_hash = checkin_canonical_hash(
            &cin_no,
            representative.and_then(|d| d.room_no.as_deref()),
            room_in_str.as_deref(),
            room_out_date_str.as_deref(),
            representative.and_then(|d| d.cust_no.as_deref()),
        );

        match mode {
            ReconcileMode::Upsert => {
                upsert_checkin_mirror(
                    pg_pool,
                    &cin_no,
                    representative,
                    &mssql_hash,
                    &mut added,
                    &mut updated,
                    &mut unchanged,
                )
                .await?;
            }
            ReconcileMode::DiffOnly => {
                let last_ack = sqlx::query_scalar::<_, Option<String>>(
                    "SELECT sync_hash FROM ht_checkins_legacy WHERE cin_no = $1",
                )
                .bind(&cin_no)
                .fetch_optional(pg_pool)
                .await?;

                if matches!(&last_ack, Some(Some(prev)) if *prev == mssql_hash) {
                    unchanged += 1;
                    continue;
                }

                let canonical = fetch_canonical_checkin(pg_pool, &cin_no).await?;
                let canonical_hash = canonical.as_ref().map(|c| {
                    let checkin_str = c.cin_checkin_time.map(|t| t.to_string());
                    let effective_checkout_str =
                        c.effective_checkout_date().map(|d| d.to_string());
                    checkin_canonical_hash(
                        &cin_no,
                        c.legacy_room_no.as_deref(),
                        checkin_str.as_deref(),
                        effective_checkout_str.as_deref(),
                        c.legacy_cust_no.as_deref(),
                    )
                });

                if canonical_hash.as_deref() == Some(mssql_hash.as_str()) {
                    ack_checkin_mirror(pg_pool, &cin_no, &mssql_hash).await;
                    unchanged += 1;
                    continue;
                }

                let mssql_json = checkin_group_json(&cin_no, &details);
                let pg_json = canonical.as_ref().map(|c| {
                    json!({
                        "legacy_room_no": c.legacy_room_no,
                        "cin_checkin_time": c.cin_checkin_time.map(|t| t.to_string()),
                        // Bug D fix (2026-05-16) — surface BOTH the actual and
                        // expected checkout dates for operator triage. Hash uses
                        // `effective_checkout_date()` (actual if set, else
                        // expected); JSON exposes both so the operator can see
                        // which one matched / mismatched the legacy
                        // `Cin_Room_Out` shown in mssql_row_json.
                        "cin_checkout_time": c.cin_checkout_time.map(|t| t.to_string()),
                        "cin_expected_checkout": c.cin_expected_checkout.map(|d| d.to_string()),
                        "legacy_cust_no": c.legacy_cust_no,
                    })
                });
                // Track D / T7 CRIT-1: check-ins are the headline
                // cardinality-drift case. `View_CheckIn_Ds` returns
                // 41-45 rows per `Cin_no` (one per booked room) but
                // canonical `ht_checkins` denormalises only the first
                // room into a single row — Track B is the schema fix
                // (junction table) but until then the row-count delta
                // is the actionable signal. legacy_row_count =
                // details.len() exposes "this folio has N rooms";
                // pg_row_count = 0 or 1 reflects today's denormalised
                // canonical.
                let legacy_row_count: i32 = details.len() as i32;
                let pg_row_count: i32 = if canonical.is_some() { 1 } else { 0 };
                let kind = classify_divergence(
                    canonical_hash.as_deref(),
                    Some(&mssql_hash),
                    legacy_row_count,
                    pg_row_count,
                );
                record_divergence(
                    pg_pool,
                    "checkins",
                    &cin_no,
                    canonical_hash.as_deref(),
                    Some(&mssql_hash),
                    mssql_json,
                    pg_json,
                    kind,
                    legacy_row_count,
                    pg_row_count,
                )
                .await;
                if kind.is_silenceable() {
                    ack_checkin_mirror(pg_pool, &cin_no, &mssql_hash).await;
                }
                if canonical.is_none() {
                    added += 1;
                } else {
                    updated += 1;
                }
            }
        }
    }

    let duration_ms = start.elapsed().as_millis() as i32;
    tracing::info!(
        "[Sync] Check-ins ({:?}): {} added, {} updated, {} unchanged in {}ms",
        mode, added, updated, unchanged, duration_ms
    );
    record_success(pg_pool, "checkins", added, updated, unchanged, duration_ms).await;

    Ok(())
}

/// Sort a check-in PK group's detail rows deterministically. Mirrors
/// the sort contract in [`aggregate_checkin_hash`] so the "first" row
/// is stable across reconcile ticks regardless of MSSQL row order.
fn sort_checkin_details(details: &mut [CheckinDetail]) {
    details.sort_by(|a, b| {
        (
            fmt_str(&a.room_no),
            fmt_dt(&a.room_in),
            fmt_dt(&a.room_out),
        )
            .cmp(&(
                fmt_str(&b.room_no),
                fmt_dt(&b.room_in),
                fmt_dt(&b.room_out),
            ))
    });
}

/// Canonical-side projection of a check-in row for hashing. Resolved
/// by `legacy_cin_no`. `legacy_room_no` is the writeback-resolved
/// denormalised FIRST room (matches the CT mapper's `first_room_no`
/// denormalisation in `derive_room_state`).
///
/// **Checkout-date projection** is two-phase to mirror legacy
/// `Cin_Room_Out`'s dual semantic:
/// * Active stay: `cin_checkout_time IS NULL`, `cin_expected_checkout`
///   = max(Cin_Room_Out among non-checked-out rooms) per
///   `derive_stay_range`. Legacy `Cin_Room_Out` carries the booked
///   future date.
/// * Checked-out stay: `cin_checkout_time` = max(Cin_Room_Out across
///   all rooms) per `derive_room_state` (the actual departure time).
///   Legacy `Cin_Room_Out` is now the actual departure too — extended
///   then completed stays land here.
///
/// Hashing on `cin_expected_checkout` ALONE (Bug B fix 2026-05-15)
/// matches active stays but produces systematic false-positive value
/// drift for the 3,382 completed-extended stays in the HF Hotel
/// audit 2026-05-16: canonical's `cin_expected_checkout` froze at
/// the original booked date while legacy's `Cin_Room_Out` advanced
/// to the actual departure. Bug D fix: hash on
/// `COALESCE(cin_checkout_time::date, cin_expected_checkout)` so
/// the canonical side mirrors legacy's "best-known checkout date"
/// for the stay regardless of status.
struct CanonicalCheckinRow {
    legacy_room_no: Option<String>,
    cin_checkin_time: Option<NaiveDateTime>,
    cin_expected_checkout: Option<chrono::NaiveDate>,
    cin_checkout_time: Option<NaiveDateTime>,
    legacy_cust_no: Option<String>,
}

impl CanonicalCheckinRow {
    /// Effective checkout date used in the reconcile hash. Mirrors
    /// MSSQL `Cin_Room_Out.date()`: prefers actual departure
    /// (`cin_checkout_time`) when set, else falls back to expected
    /// (`cin_expected_checkout`).
    fn effective_checkout_date(&self) -> Option<chrono::NaiveDate> {
        self.cin_checkout_time
            .map(|dt| dt.date())
            .or(self.cin_expected_checkout)
    }
}

async fn fetch_canonical_checkin(
    pg_pool: &PgPool,
    legacy_cin_no: &str,
) -> Result<Option<CanonicalCheckinRow>, sqlx::Error> {
    sqlx::query_as::<_, (
        Option<String>,
        Option<NaiveDateTime>,
        Option<chrono::NaiveDate>,
        Option<NaiveDateTime>,
        Option<String>,
    )>(
        "SELECT legacy_room_no, cin_checkin_time, cin_expected_checkout, \
                cin_checkout_time, legacy_cust_no \
           FROM ht_checkins \
          WHERE legacy_cin_no = $1 \
          LIMIT 1",
    )
    .bind(legacy_cin_no)
    .fetch_optional(pg_pool)
    .await
    .map(|opt| {
        opt.map(|(room, checkin, expected_checkout, actual_checkout, cust)| {
            CanonicalCheckinRow {
                legacy_room_no: room,
                cin_checkin_time: checkin,
                cin_expected_checkout: expected_checkout,
                cin_checkout_time: actual_checkout,
                legacy_cust_no: cust,
            }
        })
    })
}

async fn ack_checkin_mirror(pg_pool: &PgPool, cin_no: &str, mssql_hash: &str) {
    let updated = sqlx::query(
        "UPDATE ht_checkins_legacy SET sync_hash = $1, synced_at = NOW() \
         WHERE cin_no = $2",
    )
    .bind(mssql_hash)
    .bind(cin_no)
    .execute(pg_pool)
    .await
    .map(|r| r.rows_affected())
    .unwrap_or(0);

    if updated == 0 {
        let _ = sqlx::query(
            "INSERT INTO ht_checkins_legacy (cin_no, sync_hash, synced_at) \
             VALUES ($1, $2, NOW()) \
             ON CONFLICT (cin_no) DO UPDATE \
               SET sync_hash = EXCLUDED.sync_hash, synced_at = EXCLUDED.synced_at",
        )
        .bind(cin_no)
        .bind(mssql_hash)
        .execute(pg_pool)
        .await;
    }
}

#[allow(clippy::too_many_arguments)]
async fn upsert_checkin_mirror(
    pg_pool: &PgPool,
    cin_no: &str,
    representative: Option<&CheckinDetail>,
    mssql_hash: &str,
    added: &mut i32,
    updated: &mut i32,
    unchanged: &mut i32,
) -> Result<(), sqlx::Error> {
    let existing = sqlx::query_scalar::<_, Option<String>>(
        "SELECT sync_hash FROM ht_checkins_legacy WHERE cin_no = $1",
    )
    .bind(cin_no)
    .fetch_optional(pg_pool)
    .await?;

    let Some(rep) = representative else {
        return Ok(());
    };

    match existing {
        Some(Some(existing_hash)) if existing_hash == mssql_hash => {
            *unchanged += 1;
        }
        Some(_) => {
            sqlx::query(
                r#"
                UPDATE ht_checkins_legacy
                SET cin_room_no = $1, cin_room_in = $2, cin_room_out = $3,
                    cin_cust_name = $4, cin_cust_no = $5, cin_status = $6,
                    sync_hash = $7, synced_at = NOW()
                WHERE cin_no = $8
                "#,
            )
            .bind(&rep.room_no)
            .bind(rep.room_in)
            .bind(rep.room_out)
            .bind(&rep.cust_name)
            .bind(&rep.cust_no)
            .bind(&rep.status)
            .bind(mssql_hash)
            .bind(cin_no)
            .execute(pg_pool)
            .await?;
            *updated += 1;
        }
        None => {
            sqlx::query(
                r#"
                INSERT INTO ht_checkins_legacy
                    (cin_no, cin_room_no, cin_room_in, cin_room_out,
                     cin_cust_name, cin_cust_no, cin_status, sync_hash)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                "#,
            )
            .bind(cin_no)
            .bind(&rep.room_no)
            .bind(rep.room_in)
            .bind(rep.room_out)
            .bind(&rep.cust_name)
            .bind(&rep.cust_no)
            .bind(&rep.status)
            .bind(mssql_hash)
            .execute(pg_pool)
            .await?;
            *added += 1;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    //! Pure unit tests for the Phase 5.5 mode-parsing logic. The
    //! integration tests that exercise the full reconcile + diff-log
    //! path live in `tests/test_scheduler_sync_diff_only.rs`.
    use super::*;

    /// Env-var manipulation across parallel cargo tests would race; we
    /// serialise these tests behind a Mutex and restore the prior value.
    /// The lock is process-wide because `set_var` is too.
    fn with_mode_env<F: FnOnce() -> ReconcileMode>(value: Option<&str>, f: F) -> ReconcileMode {
        use std::sync::Mutex;
        static LOCK: Mutex<()> = Mutex::new(());
        let _g = LOCK.lock().unwrap();
        let prior = env::var("LEGACY_SYNC_RECONCILE_MODE").ok();
        match value {
            Some(v) => env::set_var("LEGACY_SYNC_RECONCILE_MODE", v),
            None => env::remove_var("LEGACY_SYNC_RECONCILE_MODE"),
        }
        let out = f();
        match prior {
            Some(v) => env::set_var("LEGACY_SYNC_RECONCILE_MODE", v),
            None => env::remove_var("LEGACY_SYNC_RECONCILE_MODE"),
        }
        out
    }

    #[test]
    fn from_env_defaults_to_diff_only_when_unset() {
        let mode = with_mode_env(None, ReconcileMode::from_env);
        assert_eq!(mode, ReconcileMode::DiffOnly);
    }

    #[test]
    fn from_env_recognises_diff_only_literal() {
        let mode = with_mode_env(Some("diff_only"), ReconcileMode::from_env);
        assert_eq!(mode, ReconcileMode::DiffOnly);
    }

    #[test]
    fn from_env_recognises_upsert_literal() {
        let mode = with_mode_env(Some("upsert"), ReconcileMode::from_env);
        assert_eq!(mode, ReconcileMode::Upsert);
    }

    #[test]
    fn from_env_falls_back_to_diff_only_on_unknown_value() {
        let mode = with_mode_env(Some("garbage"), ReconcileMode::from_env);
        assert_eq!(
            mode,
            ReconcileMode::DiffOnly,
            "unknown values must default to the safe (non-mutating) mode"
        );
    }

    // -------------------------------------------------------------------
    // Phase 6 — drift-alert threshold tests
    // -------------------------------------------------------------------

    fn counts(rows: &[(&str, i64)]) -> Vec<(String, i64)> {
        rows.iter().map(|(t, n)| ((*t).to_string(), *n)).collect()
    }

    #[test]
    fn breach_filter_returns_empty_when_no_table_exceeds_threshold() {
        let input = counts(&[("customers", 10), ("rooms", 49), ("bookings", 0)]);
        assert!(tables_breaching_threshold(&input, 50).is_empty());
    }

    #[test]
    fn breach_filter_is_strict_greater_than_not_equal() {
        // Threshold 50, count 50 → must NOT alert. Spec wording: "above
        // 50 in an hour".
        let input = counts(&[("customers", 50)]);
        assert!(
            tables_breaching_threshold(&input, 50).is_empty(),
            "exactly-at-threshold must not alert"
        );
    }

    #[test]
    fn breach_filter_returns_only_breaching_tables() {
        let input = counts(&[
            ("customers", 51),
            ("rooms", 49),
            ("bookings", 5_000),
            ("checkins", 50),
        ]);
        let mut breaches = tables_breaching_threshold(&input, 50);
        breaches.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(
            breaches,
            vec![("bookings".to_string(), 5_000), ("customers".to_string(), 51)]
        );
    }

    #[test]
    fn breach_filter_handles_empty_input() {
        let input: Vec<(String, i64)> = Vec::new();
        assert!(tables_breaching_threshold(&input, 50).is_empty());
    }

    #[test]
    fn breach_filter_respects_custom_threshold() {
        let input = counts(&[("customers", 5), ("rooms", 11)]);
        // Threshold 10 → only `rooms` (11 > 10).
        let breaches = tables_breaching_threshold(&input, 10);
        assert_eq!(breaches, vec![("rooms".to_string(), 11)]);
    }

    /// Env-isolation helper for the threshold-parsing tests. Same shape
    /// as `with_mode_env` above. Tracks BOTH the global var and the
    /// per-site override (task #69) so the per-site fallback chain can
    /// be exercised without leaking state across tests.
    fn with_threshold_envs<F: FnOnce() -> i64>(
        global: Option<&str>,
        per_site_var: Option<(&str, &str)>,
        f: F,
    ) -> i64 {
        use std::sync::Mutex;
        static LOCK: Mutex<()> = Mutex::new(());
        let _g = LOCK.lock().unwrap();
        let prior_global = env::var("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD").ok();
        let prior_per_site = per_site_var.map(|(name, _)| (name.to_string(), env::var(name).ok()));
        match global {
            Some(v) => env::set_var("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD", v),
            None => env::remove_var("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD"),
        }
        if let Some((name, value)) = per_site_var {
            env::set_var(name, value);
        }
        let out = f();
        match prior_global {
            Some(v) => env::set_var("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD", v),
            None => env::remove_var("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD"),
        }
        if let Some((name, prior)) = prior_per_site {
            match prior {
                Some(v) => env::set_var(&name, v),
                None => env::remove_var(&name),
            }
        }
        out
    }

    /// Back-compat shim for tests that only care about the global var.
    fn with_threshold_env<F: FnOnce() -> i64>(value: Option<&str>, f: F) -> i64 {
        with_threshold_envs(value, None, f)
    }

    #[test]
    fn threshold_defaults_when_env_unset() {
        let v = with_threshold_env(None, || drift_alert_threshold_from_env("hfhotel"));
        assert_eq!(v, DEFAULT_DRIFT_ALERT_THRESHOLD);
    }

    #[test]
    fn threshold_parses_custom_positive_value() {
        let v = with_threshold_env(Some("125"), || drift_alert_threshold_from_env("hfhotel"));
        assert_eq!(v, 125);
    }

    #[test]
    fn threshold_falls_back_on_zero_or_negative() {
        let v = with_threshold_env(Some("0"), || drift_alert_threshold_from_env("hfhotel"));
        assert_eq!(v, DEFAULT_DRIFT_ALERT_THRESHOLD);
        let v = with_threshold_env(Some("-5"), || drift_alert_threshold_from_env("hfhotel"));
        assert_eq!(v, DEFAULT_DRIFT_ALERT_THRESHOLD);
    }

    #[test]
    fn threshold_falls_back_on_garbage() {
        let v = with_threshold_env(Some("not-a-number"), || drift_alert_threshold_from_env("hfhotel"));
        assert_eq!(v, DEFAULT_DRIFT_ALERT_THRESHOLD);
    }

    // -------------------------------------------------------------------
    // Task #69 — per-site drift threshold overrides
    // -------------------------------------------------------------------

    /// Global env honored when the per-site override is unset (the
    /// HF Hotel back-compat path: existing
    /// `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD=80` keeps working
    /// unchanged after task #69 lands).
    #[test]
    fn threshold_global_used_when_per_site_unset() {
        let v = with_threshold_envs(Some("80"), None, || {
            drift_alert_threshold_from_env("hfhotel")
        });
        assert_eq!(v, 80);
    }

    /// Per-site override wins when both are set (the HF Ville path:
    /// operator wants a tighter alert threshold for the smaller
    /// property without disturbing HF Hotel's tuning).
    #[test]
    fn threshold_per_site_overrides_global() {
        let v = with_threshold_envs(
            Some("80"),
            Some(("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_HFVILLE", "20")),
            || drift_alert_threshold_from_env("hfville"),
        );
        assert_eq!(v, 20, "per-site override must take precedence over global");
    }

    /// Per-site override is namespaced by the site id — an HF Hotel
    /// tick must NOT pick up `..._HFVILLE` and vice versa.
    #[test]
    fn threshold_per_site_does_not_leak_across_sites() {
        let v = with_threshold_envs(
            Some("80"),
            Some(("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_HFVILLE", "20")),
            || drift_alert_threshold_from_env("hfhotel"),
        );
        assert_eq!(
            v, 80,
            "HF Hotel tick must use the global threshold, not HF Ville's"
        );
    }

    /// Garbage in the per-site override falls through to the global
    /// (instead of crashing) — operator typo doesn't take down alerts.
    #[test]
    fn threshold_per_site_garbage_falls_through_to_global() {
        let v = with_threshold_envs(
            Some("80"),
            Some(("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_HFVILLE", "abc")),
            || drift_alert_threshold_from_env("hfville"),
        );
        assert_eq!(v, 80, "invalid per-site value must fall through to the global");
    }

    // -------------------------------------------------------------------
    // Phase 6 hotfix (2026-04-29) — multi-row PK aggregation determinism
    // -------------------------------------------------------------------
    //
    // `View_CheckIn_Ds` returns 41-45 rows per `Cin_no`; `View_Booking_Ds`
    // returns up to 3 rows per `(Book_No, Book_Room_Type)`. Hashing each
    // row independently against a single-row-per-PK cache caused a
    // ~22-24k/hour drift-alert spam loop. These tests pin the
    // determinism + sensitivity contract of the aggregation helpers.

    use chrono::NaiveDate;

    fn dt(y: i32, m: u32, d: u32, h: u32, min: u32) -> Option<NaiveDateTime> {
        Some(
            NaiveDate::from_ymd_opt(y, m, d)
                .unwrap()
                .and_hms_opt(h, min, 0)
                .unwrap(),
        )
    }

    fn checkin_detail(
        room: &str,
        in_dt: Option<NaiveDateTime>,
        out_dt: Option<NaiveDateTime>,
    ) -> CheckinDetail {
        CheckinDetail {
            room_no: Some(room.to_string()),
            room_in: in_dt,
            room_out: out_dt,
            cust_name: Some("Somchai".to_string()),
            cust_no: Some("C001".to_string()),
            status: Some("OPEN".to_string()),
        }
    }

    #[test]
    fn aggregate_checkin_hash_is_order_independent() {
        let a = checkin_detail("101", dt(2026, 4, 1, 14, 0), dt(2026, 4, 3, 12, 0));
        let b = checkin_detail("102", dt(2026, 4, 1, 14, 0), dt(2026, 4, 3, 12, 0));
        let c = checkin_detail("103", dt(2026, 4, 1, 14, 0), dt(2026, 4, 3, 12, 0));

        let mut forward = vec![a.clone(), b.clone(), c.clone()];
        let mut reversed = vec![c, b, a];

        let h1 = aggregate_checkin_hash("CIN-1", &mut forward);
        let h2 = aggregate_checkin_hash("CIN-1", &mut reversed);
        assert_eq!(h1, h2, "row order must not affect the aggregate hash");
    }

    #[test]
    fn aggregate_checkin_hash_changes_when_a_field_changes() {
        let a = checkin_detail("101", dt(2026, 4, 1, 14, 0), dt(2026, 4, 3, 12, 0));
        let mut original = vec![a.clone()];
        let mut mutated = vec![CheckinDetail {
            status: Some("CLOSED".to_string()),
            ..a
        }];

        let h1 = aggregate_checkin_hash("CIN-1", &mut original);
        let h2 = aggregate_checkin_hash("CIN-1", &mut mutated);
        assert_ne!(h1, h2, "field change must produce a different hash");
    }

    #[test]
    fn aggregate_checkin_hash_handles_single_row_pk() {
        // Smoke test for the common case: 1:1 PKs (which `Cin_no`
        // technically is for the older customer subset). Helper must
        // not panic and must return a stable hash.
        let a = checkin_detail("101", dt(2026, 4, 1, 14, 0), dt(2026, 4, 3, 12, 0));
        let mut once = vec![a.clone()];
        let mut twice = vec![a];

        let h1 = aggregate_checkin_hash("CIN-1", &mut once);
        let h2 = aggregate_checkin_hash("CIN-1", &mut twice);
        assert_eq!(h1, h2);
        assert!(!h1.is_empty());
    }

    #[test]
    fn aggregate_checkin_hash_distinguishes_pk() {
        let a = checkin_detail("101", dt(2026, 4, 1, 14, 0), dt(2026, 4, 3, 12, 0));
        let mut g1 = vec![a.clone()];
        let mut g2 = vec![a];

        let h1 = aggregate_checkin_hash("CIN-1", &mut g1);
        let h2 = aggregate_checkin_hash("CIN-2", &mut g2);
        assert_ne!(h1, h2, "PK must be part of the hashed material");
    }

    fn booking_detail(room_type: &str, status: i32) -> BookingDetail {
        BookingDetail {
            book_date: dt(2026, 4, 1, 10, 0),
            book_date_in: dt(2026, 4, 5, 14, 0),
            book_date_out: dt(2026, 4, 7, 12, 0),
            book_cust_name: Some("Somchai".to_string()),
            book_cust_id: Some("ID-001".to_string()),
            book_status: Some(status),
            book_room_type: Some(room_type.to_string()),
        }
    }

    #[test]
    fn aggregate_booking_hash_is_order_independent() {
        let a = booking_detail("DELUXE", 1);
        let b = BookingDetail {
            book_cust_name: Some("Anan".to_string()),
            ..booking_detail("DELUXE", 2)
        };
        let c = BookingDetail {
            book_cust_id: Some("ID-002".to_string()),
            ..booking_detail("DELUXE", 1)
        };

        let mut forward = vec![a.clone(), b.clone(), c.clone()];
        let mut reversed = vec![c, b, a];

        let h1 = aggregate_booking_hash("BK-1", "DELUXE", &mut forward);
        let h2 = aggregate_booking_hash("BK-1", "DELUXE", &mut reversed);
        assert_eq!(h1, h2, "row order must not affect the aggregate hash");
    }

    #[test]
    fn aggregate_booking_hash_changes_when_a_field_changes() {
        let a = booking_detail("DELUXE", 1);
        let mut original = vec![a.clone()];
        let mut mutated = vec![BookingDetail {
            book_status: Some(2),
            ..a
        }];

        let h1 = aggregate_booking_hash("BK-1", "DELUXE", &mut original);
        let h2 = aggregate_booking_hash("BK-1", "DELUXE", &mut mutated);
        assert_ne!(h1, h2, "field change must produce a different hash");
    }

    #[test]
    fn aggregate_booking_hash_handles_single_row_pk() {
        let a = booking_detail("DELUXE", 1);
        let mut once = vec![a.clone()];
        let mut twice = vec![a];

        let h1 = aggregate_booking_hash("BK-1", "DELUXE", &mut once);
        let h2 = aggregate_booking_hash("BK-1", "DELUXE", &mut twice);
        assert_eq!(h1, h2);
        assert!(!h1.is_empty());
    }

    #[test]
    fn aggregate_booking_hash_distinguishes_composite_pk() {
        // (BK-1, DELUXE) vs (BK-1, STANDARD) — the room_type half of
        // the composite PK must be part of the hash so two PKs
        // sharing a Book_No don't collide.
        let a = booking_detail("DELUXE", 1);
        let b = booking_detail("STANDARD", 1);
        let mut g1 = vec![a];
        let mut g2 = vec![b];

        let h1 = aggregate_booking_hash("BK-1", "DELUXE", &mut g1);
        let h2 = aggregate_booking_hash("BK-1", "STANDARD", &mut g2);
        assert_ne!(h1, h2, "composite PK key half must be part of the hash");
    }

    // -------------------------------------------------------------------
    // v2.63.0 — canonical-shape hash helpers (PG vs MSSQL alignment)
    // -------------------------------------------------------------------
    //
    // These tests pin the contract that an MSSQL row + a faithful
    // canonical projection of that row hash to the SAME value. If a CT
    // mapper regression breaks the projection, the canonical hash
    // diverges and the reconciler logs drift — exactly the actionable
    // signal the v2.63.0 migration is designed to surface.

    #[test]
    fn customer_canonical_hash_matches_when_canonical_mirrors_legacy() {
        let mssql = customer_canonical_hash(
            "C001",
            "Somchai",
            Some("walk-in"),
            Some("0812345678"),
            Some("1234567890123"),
            Some("123 Sukhumvit"),
        );
        // Canonical row that the CT mapper would have produced.
        let canonical = customer_canonical_hash(
            "C001",
            "Somchai",
            Some("walk-in"),
            Some("0812345678"),
            Some("1234567890123"),
            Some("123 Sukhumvit"),
        );
        assert_eq!(mssql, canonical);
    }

    #[test]
    fn customer_canonical_hash_diverges_on_phone_drift() {
        let mssql = customer_canonical_hash(
            "C001",
            "Somchai",
            None,
            Some("0812345678"),
            None,
            None,
        );
        // CT mapper stored an older phone (drift the operator should fix).
        let canonical = customer_canonical_hash(
            "C001",
            "Somchai",
            None,
            Some("0899999999"),
            None,
            None,
        );
        assert_ne!(mssql, canonical);
    }

    #[test]
    fn customer_canonical_hash_treats_none_as_empty() {
        // `Cust_Type = NULL` on the MSSQL side hashes identically to
        // `cust_type = NULL` on the canonical side — both project to
        // empty string before hashing.
        let h1 = customer_canonical_hash("C001", "Anan", None, None, None, None);
        let h2 = customer_canonical_hash("C001", "Anan", Some(""), Some(""), Some(""), Some(""));
        assert_eq!(h1, h2, "None and empty-string must canonicalise the same way");
    }

    /// Locks the legacy `HT_Customers` projection used by the reconcile
    /// hash against the CT mapper's column semantics. The six columns
    /// here must mirror `sync::mappers::customer::CustomerMapper`
    /// faithfully — otherwise every customer with a prefix, secondary
    /// name, multi-line address, or distinct rate-tier-vs-category
    /// surfaces as systematic `value` drift that the auto-resolve
    /// sweep can never clear.
    ///
    /// **Forbidden columns (regression tripwires):**
    /// - `View_Customers` concatenates `Cust_perfix + Cust_name + ' ' +
    ///   Cust_name2`, hiding the actual `Cust_name` the CT mapper
    ///   writes. Hash against `HT_Customers.Cust_name` only.
    /// - `View_Customers.C_Address` is a multi-line concatenation; PG
    ///   `cust_address` mirrors only the door number from
    ///   `Cust_Add_no`. Hash against `Cust_Add_no` only.
    /// - `View_Customers.Cust_Type` is the rate-tier label (mapped to
    ///   `cust_price_tier` in PG); the CT mapper writes
    ///   `Cust_Type_Main` into PG `cust_type`. Hash against
    ///   `Cust_Type_Main` only.
    #[test]
    fn customers_reconcile_projection_locks_mapper_compatible_columns() {
        for col in [
            "Cust_no",
            "Cust_name",
            "Cust_Type_Main",
            "Cust_Add_tel",
            "Cust_IDcard",
            "Cust_Add_no",
        ] {
            assert!(
                CUSTOMERS_RECONCILE_PROJECTION.contains(col),
                "CUSTOMERS_RECONCILE_PROJECTION missing required column '{col}' — \
                 reconcile hash will diverge from CT mapper's canonical writes"
            );
        }
        // Forbidden columns — these are the View_Customers collapsed
        // surfaces that produce systematic false-positive drift.
        for forbidden in ["C_Address", "Cust_Type "] {
            assert!(
                !CUSTOMERS_RECONCILE_PROJECTION.contains(forbidden),
                "CUSTOMERS_RECONCILE_PROJECTION must not project '{forbidden}' — \
                 hashes will not align with the CT mapper's writes"
            );
        }
        // `Cust_Type` is a substring of `Cust_Type_Main`, so the
        // contains-check above uses a trailing space. Pin the exact
        // bare-column absence via a token split.
        let has_bare_cust_type = CUSTOMERS_RECONCILE_PROJECTION
            .split(',')
            .map(str::trim)
            .any(|tok| tok == "Cust_Type");
        assert!(
            !has_bare_cust_type,
            "CUSTOMERS_RECONCILE_PROJECTION must not project bare 'Cust_Type' \
             (rate-tier, mapped to cust_price_tier) — use Cust_Type_Main instead"
        );
    }

    // -------------------------------------------------------------------
    // Track J1 — projection-lock guards for the remaining reconcile
    // projections. `CUSTOMERS_RECONCILE_PROJECTION` keeps its hand-rolled
    // lock test above because that test additionally pins forbidden-
    // column tripwires (View_Customers's `Cust_Type` rate-tier label).
    // The three projections below have no such forbidden columns —
    // a baseline-subset check is the right granularity.
    //
    // PR #90 (`HT_CheckIn_Ds` 9h drop) and PR #101 (`Cust_Type` vs
    // `Cust_Type_Main` reconcile drift) were both single typos that
    // shipped because tiberius validates column names only at runtime.
    // A baseline-subset lock test catches that class at CI time before
    // it can ever reach the watcher / reconcile loop.
    // -------------------------------------------------------------------

    #[test]
    fn rooms_reconcile_projection_is_subset_of_legacy_schema() {
        crate::assert_projection_slice_subset!(ROOMS_RECONCILE_PROJECTION, "HT_Rooms");
    }

    #[test]
    fn bookings_reconcile_projection_is_subset_of_legacy_schema() {
        // `View_Booking_Ds` joins HT_Book_H (header) with HT_Book_Ds
        // (per-room detail). Every projected column must appear on
        // one of the two underlying base tables.
        crate::assert_projection_slice_subset_of_two_tables!(
            BOOKINGS_RECONCILE_PROJECTION,
            "HT_Book_H",
            "HT_Book_Ds",
            "View_Booking_Ds"
        );
    }

    #[test]
    fn checkins_reconcile_projection_columns_resolve_on_their_tables() {
        // Prod hotfix 2026-05-18 — the precursor test only asserted
        // "this column is on one of the two base tables", which the
        // failing `Cin_Date_in` projection passed (it's on
        // `HT_CheckIn_H`) even though it was being SELECTed from
        // `View_CheckIn_Ds`, which doesn't expose it. The new
        // contract: every projection entry is a `(alias, column)`
        // pair, and the column MUST exist on the table the alias
        // resolves to per the FROM clause. An unqualified column or
        // a mis-aliased column fails CI.

        use crate::sync::projection_guard::parse_baseline_columns;

        // 1. Extract the alias-to-table mapping from the canonical
        //    FROM clause. Format is
        //    `HT_CheckIn_H h INNER JOIN HT_CheckIn_Ds d ON …` — split
        //    on whitespace, every `<TABLE> <alias>` pair is two
        //    consecutive tokens immediately after the
        //    `(FROM|JOIN)` keyword.
        let from_clause = CHECKINS_RECONCILE_FROM_CLAUSE;
        let tokens: Vec<&str> = from_clause.split_whitespace().collect();
        let mut alias_to_table: std::collections::HashMap<&str, &str> =
            std::collections::HashMap::new();
        // First two tokens after the implicit FROM: `HT_CheckIn_H h`.
        if tokens.len() >= 2 {
            alias_to_table.insert(tokens[1], tokens[0]);
        }
        // Any `JOIN <TABLE> <alias>` triple later in the clause.
        for window in tokens.windows(3) {
            if window[0].eq_ignore_ascii_case("JOIN") {
                alias_to_table.insert(window[2], window[1]);
            }
        }
        assert!(
            alias_to_table.contains_key("h"),
            "alias `h` not found in FROM clause `{from_clause}`",
        );
        assert!(
            alias_to_table.contains_key("d"),
            "alias `d` not found in FROM clause `{from_clause}`",
        );

        // 2. For each projection entry, look up the declared alias
        //    and assert the column exists on that specific table per
        //    the baseline schema dump.
        for (alias, col) in CHECKINS_RECONCILE_PROJECTION.iter() {
            assert!(
                !alias.is_empty(),
                "projection entry `{col}` has empty alias — every column \
                 MUST declare which base table it lives on (prod hotfix \
                 2026-05-18: an unqualified `Cin_Date_in` resolved at the \
                 wrong table caused 242 consecutive reconcile failures)",
            );
            let table = alias_to_table.get(alias).unwrap_or_else(|| {
                panic!(
                    "projection entry `{alias}.{col}` references unknown \
                     alias `{alias}` — declared aliases in FROM clause: {:?}",
                    alias_to_table.keys().collect::<Vec<_>>(),
                )
            });
            let allowed = parse_baseline_columns(table);
            assert!(
                !allowed.is_empty(),
                "no baseline columns found for `{table}` — schema dump \
                 may be missing it",
            );
            assert!(
                allowed.contains(&col.to_lowercase()),
                "projection entry `{alias}.{col}` references column `{col}` \
                 which is NOT on `{table}` per the HF Hotel legacy schema \
                 baseline (docs/legacy-spike/schema/01-baseline-schema.txt). \
                 Either the alias is wrong or the column doesn't exist. \
                 Prod hotfix 2026-05-18: this is the exact bug class \
                 `edc600a` shipped — `Cin_Date_in` was added to a \
                 projection that targeted the wrong table (the view), \
                 and every reconcile tick failed at the driver.",
            );
        }

        // 3. The production code path MUST use the same projection +
        //    FROM clause this test introspects. Assert the helper
        //    returns the lock-tested values verbatim.
        let (sql, from, projection) = build_checkins_reconcile_select();
        assert_eq!(from, CHECKINS_RECONCILE_FROM_CLAUSE);
        assert_eq!(
            projection.len(),
            CHECKINS_RECONCILE_PROJECTION.len(),
            "helper must return the same projection slice the lock test pins",
        );
        for (returned, expected) in
            projection.iter().zip(CHECKINS_RECONCILE_PROJECTION.iter())
        {
            assert_eq!(
                returned, expected,
                "helper projection drift — expected {expected:?}, got {returned:?}",
            );
        }
        assert!(
            sql.contains(CHECKINS_RECONCILE_FROM_CLAUSE),
            "built SQL must include the locked FROM clause: {sql}",
        );
        for (alias, col) in CHECKINS_RECONCILE_PROJECTION.iter() {
            assert!(
                sql.contains(&format!("{alias}.{col}")),
                "built SQL must include qualified column `{alias}.{col}`: {sql}",
            );
        }
    }

    #[test]
    fn bool_to_yesno_round_trip_via_legacy_yesno_canonical() {
        // The two halves of the room-status translation must be each
        // other's inverse for the canonical-hash to align with the
        // MSSQL projection.
        assert_eq!(legacy_yesno_canonical(Some("yes")), bool_to_yesno(Some(true)));
        assert_eq!(legacy_yesno_canonical(Some("no")), bool_to_yesno(Some(false)));
        // NULL → "" on both sides, matching how nullable BOOLEAN
        // columns canonicalise.
        assert_eq!(legacy_yesno_canonical(None), bool_to_yesno(None));
        // Unknown legacy literals fall back to "" — matches the CT
        // mapper's behaviour (`legacy_yesno_to_bool` returns None for
        // anything other than yes/no).
        assert_eq!(legacy_yesno_canonical(Some("maybe")), "");
    }

    #[test]
    fn room_canonical_hash_matches_when_canonical_mirrors_legacy() {
        // Legacy "yes" → canonical true → reverse to "yes". Hashes align.
        let mssql = room_canonical_hash("101", "yes", "no", Some("ocean view"));
        let canonical = room_canonical_hash(
            "101",
            bool_to_yesno(Some(true)),
            bool_to_yesno(Some(false)),
            Some("ocean view"),
        );
        assert_eq!(mssql, canonical);
    }

    #[test]
    fn room_canonical_hash_diverges_when_canonical_clean_lags_behind() {
        // Operator marked the room dirty in legacy ("no") but the CT
        // mapper hasn't yet flipped canonical.room_clean to false →
        // drift fires.
        let mssql = room_canonical_hash("101", "no", "no", None);
        let canonical = room_canonical_hash(
            "101",
            bool_to_yesno(Some(true)),
            bool_to_yesno(Some(false)),
            None,
        );
        assert_ne!(mssql, canonical);
    }

    #[test]
    fn booking_canonical_hash_aligns_legacy_datetime_and_canonical_date() {
        // Legacy stores `Book_Date_in` as DATETIME-at-midnight. After
        // `.date().to_string()` it serialises to "2026-04-01" — the
        // exact format `chrono::NaiveDate::to_string()` emits on the
        // canonical side.
        let mssql = booking_canonical_hash(
            "BK001",
            Some("2026-04-01"),
            Some("2026-04-03"),
            Some("C001"),
        );
        let canonical = booking_canonical_hash(
            "BK001",
            Some("2026-04-01"),
            Some("2026-04-03"),
            Some("C001"),
        );
        assert_eq!(mssql, canonical);
    }

    #[test]
    fn booking_canonical_hash_diverges_on_checkout_date_drift() {
        let mssql = booking_canonical_hash(
            "BK001",
            Some("2026-04-01"),
            Some("2026-04-03"),
            Some("C001"),
        );
        let canonical = booking_canonical_hash(
            "BK001",
            Some("2026-04-01"),
            Some("2026-04-05"), // canonical extended by two nights (drift)
            Some("C001"),
        );
        assert_ne!(mssql, canonical);
    }

    #[test]
    fn checkin_canonical_hash_aligns_legacy_first_room_and_canonical_legacy_room_no() {
        // CT checkin mapper denormalises the FIRST sorted room into
        // `ht_checkins.legacy_room_no`. The reconciler picks the same
        // first room from the sorted legacy multi-row group, so both
        // hashes use the identical room_no token.
        let mssql = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            None,
            Some("C001"),
        );
        let canonical = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            None,
            Some("C001"),
        );
        assert_eq!(mssql, canonical);
    }

    #[test]
    fn checkin_canonical_hash_diverges_on_room_drift() {
        let mssql = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            None,
            None,
            None,
        );
        // CT mapper resolved the wrong room — drift the operator
        // should investigate via `ht_reconcile_log`.
        let canonical = checkin_canonical_hash(
            "CIN001",
            Some("102"),
            None,
            None,
            None,
        );
        assert_ne!(mssql, canonical);
    }

    #[test]
    fn checkin_canonical_hash_handles_open_checkin_with_no_checkout() {
        // Both-NULL is now a vacuous case (Bug B fix: canonical reads
        // `cin_expected_checkout` which is NOT NULL per derive_stay_range,
        // and legacy `Cin_Room_Out` is populated at check-in time). Kept
        // as a degenerate-input guard — the hash function itself still
        // matches when both sides pass None.
        let mssql = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            None,
            Some("C001"),
        );
        let canonical = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            None,
            Some("C001"),
        );
        assert_eq!(mssql, canonical);
    }

    /// Bug B fix (2026-05-15): for an active stay, MSSQL `Cin_Room_Out` is the
    /// booked checkout *timestamp* (e.g. `2026-05-18 11:59:59`) while
    /// canonical `cin_expected_checkout` is a *date* (`2026-05-18`). The
    /// reconcile-side caller now drops the time component on the MSSQL side
    /// and reads `cin_expected_checkout` (not `cin_checkout_time`) on the
    /// canonical side, so both inputs land on the same `"YYYY-MM-DD"`
    /// string and the hashes converge. Without this alignment every active
    /// stay produced a systematic false-positive value-drift row.
    #[test]
    fn checkin_canonical_hash_aligns_booked_checkout_across_sides_after_bug_b_fix() {
        let mssql = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            Some("2026-05-18"), // MSSQL caller now drops time via .date().to_string()
            Some("C001"),
        );
        let canonical = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            Some("2026-05-18"), // canonical cin_expected_checkout.to_string()
            Some("C001"),
        );
        assert_eq!(mssql, canonical);
    }

    /// Regression guard for Bug B. Pre-fix the MSSQL side hashed
    /// `Cin_Room_Out` as a full datetime while the canonical side hashed
    /// `cin_checkout_time` (NULL on active stays). Re-introducing either
    /// half of that mismatch must fail this test. The two inputs below
    /// model the pre-fix call shape; the new code never produces them.
    #[test]
    fn checkin_canonical_hash_pre_bug_b_inputs_must_diverge() {
        let pre_fix_mssql_datetime = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            Some("2026-05-18 11:59:59"),
            Some("C001"),
        );
        let pre_fix_canonical_null_checkout = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            None,
            Some("C001"),
        );
        assert_ne!(
            pre_fix_mssql_datetime, pre_fix_canonical_null_checkout,
            "pre-Bug-B inputs MUST diverge — this is the bug the fix addresses"
        );
    }

    /// Bug D fix (2026-05-16): `effective_checkout_date()` prefers
    /// `cin_checkout_time::date()` over `cin_expected_checkout`. Pre-fix
    /// the hash always used `cin_expected_checkout`, which froze at the
    /// original booked date for completed-extended stays — 3,382
    /// false-positive value-drift rows on HF Hotel as of audit
    /// 2026-05-16 (e.g. CH26-005335 was booked checkout 2026-05-08 but
    /// the guest actually departed 2026-05-10; reconcile flagged
    /// canonical 2026-05-08 vs MSSQL 2026-05-10 forever).
    #[test]
    fn effective_checkout_prefers_actual_when_present() {
        use chrono::NaiveDate;
        let row = CanonicalCheckinRow {
            legacy_room_no: Some("518".into()),
            cin_checkin_time: None,
            cin_expected_checkout: Some(NaiveDate::from_ymd_opt(2026, 5, 8).unwrap()),
            cin_checkout_time: Some(
                NaiveDate::from_ymd_opt(2026, 5, 10).unwrap().and_hms_opt(12, 8, 0).unwrap(),
            ),
            legacy_cust_no: None,
        };
        // Completed-extended stay: actual departure date wins over original
        // booking. Matches the legacy `Cin_Room_Out` value the MSSQL side
        // of the reconcile hashes.
        assert_eq!(
            row.effective_checkout_date(),
            Some(NaiveDate::from_ymd_opt(2026, 5, 10).unwrap())
        );
    }

    /// Active stay: `cin_checkout_time IS NULL`, fall back to
    /// `cin_expected_checkout` so the hash matches legacy
    /// `Cin_Room_Out` (which holds the booked-future date until
    /// checkout).
    #[test]
    fn effective_checkout_falls_back_to_expected_when_not_yet_checked_out() {
        use chrono::NaiveDate;
        let row = CanonicalCheckinRow {
            legacy_room_no: Some("404".into()),
            cin_checkin_time: None,
            cin_expected_checkout: Some(NaiveDate::from_ymd_opt(2026, 5, 18).unwrap()),
            cin_checkout_time: None,
            legacy_cust_no: None,
        };
        assert_eq!(
            row.effective_checkout_date(),
            Some(NaiveDate::from_ymd_opt(2026, 5, 18).unwrap())
        );
    }

    /// Edge case: row exists but no checkout dates yet (transient
    /// mid-edit state) — None passes through to the hash function's
    /// "" sentinel.
    #[test]
    fn effective_checkout_returns_none_when_neither_set() {
        let row = CanonicalCheckinRow {
            legacy_room_no: None,
            cin_checkin_time: None,
            cin_expected_checkout: None,
            cin_checkout_time: None,
            legacy_cust_no: None,
        };
        assert_eq!(row.effective_checkout_date(), None);
    }

    // -------------------------------------------------------------------
    // Track D / T7 CRIT-1 — cardinality-aware reconcile
    // -------------------------------------------------------------------

    #[test]
    fn classify_divergence_returns_missing_pg_when_pg_hash_absent() {
        let kind = classify_divergence(None, Some("abc"), 1, 0);
        assert_eq!(kind, DivergenceKind::MissingPg);
    }

    #[test]
    fn classify_divergence_returns_missing_mssql_when_mssql_hash_absent() {
        let kind = classify_divergence(Some("abc"), None, 0, 1);
        assert_eq!(kind, DivergenceKind::MissingMssql);
    }

    #[test]
    fn classify_divergence_returns_cardinality_when_row_counts_differ() {
        // Multi-room folio: 3 legacy `View_CheckIn_Ds` rows collapsed
        // into 1 canonical row. Hashes will differ forever — the kind
        // discriminator surfaces the actionable root cause.
        let kind = classify_divergence(Some("pg-hash"), Some("mssql-hash"), 3, 1);
        assert_eq!(kind, DivergenceKind::Cardinality);
    }

    #[test]
    fn classify_divergence_returns_value_when_counts_match_and_hashes_differ() {
        // Pre-condition: caller has already determined hashes differ.
        // Same row count on both sides ⇒ pure content drift.
        let kind = classify_divergence(Some("pg-hash"), Some("mssql-hash"), 1, 1);
        assert_eq!(kind, DivergenceKind::Value);
    }

    /// Track D / T7 CRIT-1 invariant — cardinality divergences must NEVER
    /// be silenced via the ack cache. The reconcile loop reads
    /// `is_silenceable()` before calling `ack_*_mirror`; if this test
    /// regresses, multi-room folios would re-acquire the silent-failure
    /// behaviour the post-mortem exposed.
    #[test]
    fn cardinality_kind_never_silenced() {
        assert!(
            !DivergenceKind::Cardinality.is_silenceable(),
            "Cardinality drift must never be silenced — the underlying \
             schema asymmetry isn't repaired by hash alone."
        );
    }

    /// Track D / T7 HIGH-1 corollary — `pg_hash IS NULL` (canonical
    /// missing) is the highest-signal divergence and must never silence.
    /// Compounds with `cardinality_kind_never_silenced` to lock the two
    /// non-silenceable kinds.
    #[test]
    fn missing_pg_kind_never_silenced() {
        assert!(
            !DivergenceKind::MissingPg.is_silenceable(),
            "Canonical-missing rows must never be silenced — they're the \
             highest-signal divergence the reconciler surfaces."
        );
    }

    #[test]
    fn value_and_missing_mssql_kinds_are_silenceable() {
        // Value drift acks-on-hash by design — a one-shot CT regression
        // becomes silent once the canonical mapper catches up.
        assert!(DivergenceKind::Value.is_silenceable());
        // MissingMssql means writeback dropped the row — the alert
        // fires once at detection; subsequent ticks are silent until
        // writeback regenerates the row.
        assert!(DivergenceKind::MissingMssql.is_silenceable());
    }

    #[test]
    fn divergence_kind_as_str_matches_schema_constraint_values() {
        // The migration 032 column doc lists these four exact strings;
        // the alerting layer + dashboards depend on them.
        assert_eq!(DivergenceKind::Value.as_str(), "value");
        assert_eq!(DivergenceKind::Cardinality.as_str(), "cardinality");
        assert_eq!(DivergenceKind::MissingPg.as_str(), "missing_pg");
        assert_eq!(DivergenceKind::MissingMssql.as_str(), "missing_mssql");
    }

    // -------------------------------------------------------------------
    // Track D / T7 HIGH-1 — level-triggered drift digest cooldown
    // -------------------------------------------------------------------

    #[test]
    fn cooldown_elapsed_returns_true_when_no_prior_alert() {
        let now = chrono::Utc::now();
        let cooldown = std::time::Duration::from_secs(86_400);
        assert!(cooldown_elapsed(None, now, cooldown));
    }

    #[test]
    fn cooldown_elapsed_returns_false_inside_window() {
        let now = chrono::Utc::now();
        let cooldown = std::time::Duration::from_secs(86_400);
        // Just fired → 0 seconds elapsed → not eligible.
        assert!(!cooldown_elapsed(Some(now), now, cooldown));
    }

    #[test]
    fn cooldown_elapsed_returns_true_after_window() {
        let now = chrono::Utc::now();
        let cooldown = std::time::Duration::from_secs(86_400); // 24h
        // Fired 25 hours ago — window elapsed.
        let last = now - chrono::Duration::hours(25);
        assert!(cooldown_elapsed(Some(last), now, cooldown));
    }

    /// Edge case: clock-skew anomaly where `last_alerted_at` is
    /// somehow in the future relative to `now`. The `to_std()` cast
    /// errors; we treat it as eligible so we don't get stuck refusing
    /// alerts forever on a clock anomaly. Per-site/per-table
    /// uniqueness is enforced by the PG primary key, so the
    /// in-memory key-construction tests from the pre-2026-05-16
    /// implementation are no longer needed.
    #[test]
    fn cooldown_elapsed_handles_future_last_alerted_via_clock_skew() {
        let now = chrono::Utc::now();
        let cooldown = std::time::Duration::from_secs(86_400);
        let future = now + chrono::Duration::hours(1);
        assert!(cooldown_elapsed(Some(future), now, cooldown));
    }

    // -------------------------------------------------------------------
    // Auto-resolve sweep — pure decision helper
    // -------------------------------------------------------------------

    /// The sweep MUST mark a row resolved when the freshly-computed
    /// canonical PG hash equals the recorded legacy hash. Pre-condition
    /// for the auto-resolve UPDATE — if this regresses, the sweep
    /// silently stops draining the queue.
    #[test]
    fn auto_resolve_sweep_marks_resolved_when_hashes_match() {
        let recorded_legacy_hash = Some("hash-X");
        let current_pg_hash = Some("hash-X");
        assert!(should_auto_resolve(recorded_legacy_hash, current_pg_hash));
    }

    /// The sweep MUST leave the row untouched when canonical still
    /// diverges from the recorded legacy hash. Drift that persists is
    /// exactly what the alerts are supposed to surface.
    #[test]
    fn auto_resolve_sweep_skips_when_drift_persists() {
        let recorded_legacy_hash = Some("hash-X");
        let current_pg_hash = Some("hash-Y");
        assert!(!should_auto_resolve(recorded_legacy_hash, current_pg_hash));
    }

    /// Composite-PK parse contract: a `"book_no|room_type"` PK splits
    /// on the first `|`. The booking-fetch dispatch + the
    /// canonical-fetch dispatch must agree on this shape or the
    /// auto-resolve sweep compares hashes from different groups.
    #[test]
    fn parse_booking_legacy_pk_splits_composite() {
        assert_eq!(parse_booking_legacy_pk("B12345|A"), ("B12345", "A"));
    }

    /// Empty `room_type_key` (the `unwrap_or_default()` case in
    /// `sync_bookings`) round-trips through the PK serialisation as
    /// `"{book_no}|"`. Must still parse cleanly so an empty-key group
    /// can be re-fetched and resolved.
    #[test]
    fn parse_booking_legacy_pk_handles_empty_room_type() {
        assert_eq!(parse_booking_legacy_pk("B12345|"), ("B12345", ""));
    }

    /// Pre-Phase-6-hotfix `ht_reconcile_log` rows lack the `|` separator
    /// (older format stored just `book_no`). Treat those as
    /// `(legacy_pk, "")` so the sweep doesn't error on legacy data.
    #[test]
    fn parse_booking_legacy_pk_legacy_format_has_no_separator() {
        assert_eq!(parse_booking_legacy_pk("B12345"), ("B12345", ""));
    }

    // -------------------------------------------------------------------
    // Incident 2026-05-18 follow-up — per-tick CT watcher lag observation
    // -------------------------------------------------------------------
    //
    // `check_ct_watcher_lag` is the live MSSQL+PG observation; the
    // tests below cover only the pure decision helper
    // (`ct_lag_is_warning`) and the env-resolution helper
    // (`ct_lag_thresholds_from_env`). The plumbing through the
    // best-effort PG/MSSQL queries is exercised by integration tests
    // when the legacy spike harness is wired up — out of scope here.

    fn default_ct_thresholds() -> CtLagThresholds {
        CtLagThresholds {
            version_lag: DEFAULT_CT_LAG_WARN_VERSIONS,
            poll_age_seconds: DEFAULT_CT_LAG_WARN_SECONDS,
        }
    }

    /// Both lag dimensions below threshold → debug-level log (warning
    /// branch must NOT fire). The hot path: a healthy watcher in the
    /// steady-state idle period between CT bumps.
    #[test]
    fn ct_lag_under_both_thresholds_does_not_warn() {
        let t = default_ct_thresholds();
        assert!(!ct_lag_is_warning(0, 0, t));
        assert!(!ct_lag_is_warning(50, 60, t));
        // Exactly at threshold ⇒ debug (strict greater-than, mirrors
        // `tables_breaching_threshold`).
        assert!(!ct_lag_is_warning(
            DEFAULT_CT_LAG_WARN_VERSIONS,
            DEFAULT_CT_LAG_WARN_SECONDS,
            t,
        ));
    }

    /// Version lag breaches alone are sufficient to warn — a CT-watcher
    /// that's behind on rows is the actionable case even if it's still
    /// polling fast enough to keep `last_polled_at` fresh.
    #[test]
    fn ct_lag_warns_when_only_version_lag_breaches() {
        let t = default_ct_thresholds();
        assert!(ct_lag_is_warning(
            DEFAULT_CT_LAG_WARN_VERSIONS + 1,
            0,
            t
        ));
    }

    /// Poll-age breaches alone are sufficient to warn — a watcher whose
    /// poll-loop is wedged still keeps `last_seen_version` advancing
    /// from the in-flight transaction, but `last_polled_at` stops
    /// refreshing (incident-2026-05-18 root-cause shape).
    #[test]
    fn ct_lag_warns_when_only_poll_age_breaches() {
        let t = default_ct_thresholds();
        assert!(ct_lag_is_warning(
            0,
            DEFAULT_CT_LAG_WARN_SECONDS + 1,
            t
        ));
    }

    /// Both dimensions breached → warn. Defensive smoke test — if either
    /// arm of the `||` regresses to `&&`, this catches it together with
    /// the two "only X breaches" tests above.
    #[test]
    fn ct_lag_warns_when_both_dimensions_breach() {
        let t = default_ct_thresholds();
        assert!(ct_lag_is_warning(
            DEFAULT_CT_LAG_WARN_VERSIONS + 1,
            DEFAULT_CT_LAG_WARN_SECONDS + 1,
            t
        ));
    }

    /// Custom thresholds are honored — proves the helper doesn't
    /// hardwire the defaults. Matches the runtime behaviour where
    /// `LEGACY_CT_LAG_WARN_VERSIONS` / `LEGACY_CT_LAG_WARN_SECONDS`
    /// override the compiled-in constants.
    #[test]
    fn ct_lag_respects_custom_thresholds() {
        let tight = CtLagThresholds {
            version_lag: 10,
            poll_age_seconds: 30,
        };
        assert!(!ct_lag_is_warning(10, 30, tight));
        assert!(ct_lag_is_warning(11, 0, tight));
        assert!(ct_lag_is_warning(0, 31, tight));
    }

    /// Env-isolation helper for CT-lag threshold tests. Same shape as
    /// `with_threshold_envs` above. Tracks BOTH the global vars and the
    /// per-site overrides so the fallback chain can be exercised
    /// without leaking state across tests.
    fn with_ct_lag_envs<F: FnOnce() -> CtLagThresholds>(
        global_versions: Option<&str>,
        global_seconds: Option<&str>,
        per_site_versions: Option<(&str, &str)>,
        per_site_seconds: Option<(&str, &str)>,
        f: F,
    ) -> CtLagThresholds {
        use std::sync::Mutex;
        static LOCK: Mutex<()> = Mutex::new(());
        let _g = LOCK.lock().unwrap();

        let prior_gv = env::var("LEGACY_CT_LAG_WARN_VERSIONS").ok();
        let prior_gs = env::var("LEGACY_CT_LAG_WARN_SECONDS").ok();
        let prior_psv = per_site_versions.map(|(n, _)| (n.to_string(), env::var(n).ok()));
        let prior_pss = per_site_seconds.map(|(n, _)| (n.to_string(), env::var(n).ok()));

        match global_versions {
            Some(v) => env::set_var("LEGACY_CT_LAG_WARN_VERSIONS", v),
            None => env::remove_var("LEGACY_CT_LAG_WARN_VERSIONS"),
        }
        match global_seconds {
            Some(v) => env::set_var("LEGACY_CT_LAG_WARN_SECONDS", v),
            None => env::remove_var("LEGACY_CT_LAG_WARN_SECONDS"),
        }
        if let Some((name, value)) = per_site_versions {
            env::set_var(name, value);
        }
        if let Some((name, value)) = per_site_seconds {
            env::set_var(name, value);
        }

        let out = f();

        match prior_gv {
            Some(v) => env::set_var("LEGACY_CT_LAG_WARN_VERSIONS", v),
            None => env::remove_var("LEGACY_CT_LAG_WARN_VERSIONS"),
        }
        match prior_gs {
            Some(v) => env::set_var("LEGACY_CT_LAG_WARN_SECONDS", v),
            None => env::remove_var("LEGACY_CT_LAG_WARN_SECONDS"),
        }
        if let Some((name, prior)) = prior_psv {
            match prior {
                Some(v) => env::set_var(&name, v),
                None => env::remove_var(&name),
            }
        }
        if let Some((name, prior)) = prior_pss {
            match prior {
                Some(v) => env::set_var(&name, v),
                None => env::remove_var(&name),
            }
        }
        out
    }

    #[test]
    fn ct_lag_thresholds_default_when_envs_unset() {
        let t = with_ct_lag_envs(None, None, None, None, || {
            ct_lag_thresholds_from_env("hfhotel")
        });
        assert_eq!(t.version_lag, DEFAULT_CT_LAG_WARN_VERSIONS);
        assert_eq!(t.poll_age_seconds, DEFAULT_CT_LAG_WARN_SECONDS);
    }

    #[test]
    fn ct_lag_thresholds_use_global_envs_when_set() {
        let t = with_ct_lag_envs(Some("250"), Some("900"), None, None, || {
            ct_lag_thresholds_from_env("hfhotel")
        });
        assert_eq!(t.version_lag, 250);
        assert_eq!(t.poll_age_seconds, 900);
    }

    /// Per-site override wins when both are set — same pattern as the
    /// drift-alert threshold per-site override (task #69).
    #[test]
    fn ct_lag_thresholds_per_site_override_wins() {
        let t = with_ct_lag_envs(
            Some("250"),
            Some("900"),
            Some(("LEGACY_CT_LAG_WARN_VERSIONS_HFVILLE", "20")),
            Some(("LEGACY_CT_LAG_WARN_SECONDS_HFVILLE", "60")),
            || ct_lag_thresholds_from_env("hfville"),
        );
        assert_eq!(t.version_lag, 20);
        assert_eq!(t.poll_age_seconds, 60);
    }

    /// Per-site override is namespaced by site id — an HF Hotel tick
    /// must NOT pick up `..._HFVILLE` (mirrors the
    /// `threshold_per_site_does_not_leak_across_sites` invariant).
    #[test]
    fn ct_lag_thresholds_per_site_does_not_leak_across_sites() {
        let t = with_ct_lag_envs(
            Some("250"),
            Some("900"),
            Some(("LEGACY_CT_LAG_WARN_VERSIONS_HFVILLE", "20")),
            Some(("LEGACY_CT_LAG_WARN_SECONDS_HFVILLE", "60")),
            || ct_lag_thresholds_from_env("hfhotel"),
        );
        assert_eq!(t.version_lag, 250);
        assert_eq!(t.poll_age_seconds, 900);
    }

    /// Garbage in any env falls through to the next layer (per-site →
    /// global → default). Operator typo on the override must not
    /// silence the observation.
    #[test]
    fn ct_lag_thresholds_garbage_falls_through_to_default() {
        let t = with_ct_lag_envs(Some("abc"), Some("-1"), None, None, || {
            ct_lag_thresholds_from_env("hfhotel")
        });
        assert_eq!(t.version_lag, DEFAULT_CT_LAG_WARN_VERSIONS);
        assert_eq!(t.poll_age_seconds, DEFAULT_CT_LAG_WARN_SECONDS);
    }

}
