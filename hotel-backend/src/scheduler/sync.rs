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
// Issue #204 (bug #2): the durable self-healing arm of the auto-resolve
// sweep re-drives the EXISTING CT upsert path, so it reaches for the same
// mappers / row-abstraction / op-enum the watcher uses rather than writing
// canonical fields by hand.
use crate::notifications::slack::{SlackClient, SlackMessage};
use crate::sync::change_op::ChangeOp;
use crate::sync::mapper::MssqlChangeMapper;
use crate::sync::mappers::{CustomerMapper, RoomMasterMapper};
use crate::sync::row::MappableRow;

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
    let per_site_var = format!(
        "LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD_{}",
        site_id.to_uppercase()
    );
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
pub fn tables_breaching_threshold(counts: &[(String, i64)], threshold: i64) -> Vec<(String, i64)> {
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
async fn check_drift_and_alert(pg_pool: &PgPool, slack: Option<&SlackClient>, site_id: &str) {
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
            ":rotating_light: *Sync lag burst — threshold exceeded* :rotating_light:\n\
             The reconcile sweep observed more than {threshold} unconverged \
             `ht_reconcile_log` row(s) for the following table(s) in the last hour. \
             Most clear on their own as the CT watcher / writeback catch up; this \
             alert surfaces a burst that may indicate a real backlog:\n\
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
async fn mark_level_alert_sent_pg(pg_pool: &PgPool, site_id: &str, table_name: &str) {
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

/// Read every cooldown key currently recorded for `site_id` in
/// `ht_level_drift_alert_cooldowns`. This is the "we alerted about this
/// at some point and never told anyone it cleared" set that the recovery
/// notification (see [`check_level_drift_recovery_and_notify`]) diffs
/// against the tables that are still unconverged.
///
/// Fail-soft in the same style as [`level_alert_eligible_pg`]: a PG error
/// yields an empty set (no all-clear fired this tick) plus a warning, and
/// never aborts the sweep.
async fn level_alert_cooldown_keys_pg(pg_pool: &PgPool, site_id: &str) -> Vec<String> {
    let rows = sqlx::query_scalar::<_, String>(
        "SELECT table_name FROM ht_level_drift_alert_cooldowns WHERE site_id = $1",
    )
    .bind(site_id)
    .fetch_all(pg_pool)
    .await;

    match rows {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                error = %e,
                "[Sync] Failed to read level-drift cooldown keys — \
                 skipping the sync-lag all-clear this tick"
            );
            Vec::new()
        }
    }
}

/// Drop the `(site_id, table_name)` cooldown row so a RECURRENCE alerts
/// on the very next tick instead of being swallowed by a stale 24h
/// window. Called only after the paired `:white_check_mark:` all-clear
/// has been emitted for that table.
///
/// Best-effort: a PG failure logs a warning and leaves the row in place —
/// the worst case is one duplicate all-clear on the next tick.
async fn clear_level_alert_cooldown_pg(pg_pool: &PgPool, site_id: &str, table_name: &str) {
    let result = sqlx::query(
        "DELETE FROM ht_level_drift_alert_cooldowns \
          WHERE site_id = $1 AND table_name = $2",
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
            "[Sync] Failed to clear level-drift cooldown after all-clear — \
             a recurrence may stay suppressed until the 24h window lapses"
        );
    }
}

/// Cooldown keys that live in `ht_level_drift_alert_cooldowns` but are
/// NOT `ht_reconcile_log` table names. The cooldown table is shared: the
/// stale-checkin tripwire parks its own key there
/// ([`STALE_CHECKIN_COOLDOWN_KEY`]) so it inherits the same 24h
/// per-site window. Those keys have no unconverged-row count to compare
/// against, so the sync-lag all-clear must never claim them recovered or
/// clear their cooldown — doing so would let the stale-checkin alert
/// refire every 15 minutes.
const NON_RECONCILE_COOLDOWN_KEYS: &[&str] = &[STALE_CHECKIN_COOLDOWN_KEY];

/// Pure decision helper for the sync-lag all-clear. Given the cooldown
/// keys recorded for a site and the tables that STILL have unconverged
/// `ht_reconcile_log` rows past the stale threshold, return the tables
/// that have recovered — i.e. we alerted about them at some point and
/// they now have zero stale rows.
///
/// Non-reconcile cooldown keys ([`NON_RECONCILE_COOLDOWN_KEYS`]) are
/// excluded: they are parked in the same table by other tripwires and
/// carry no reconcile-row semantics. Output is de-duplicated and sorted
/// so the Slack body and the log lines are deterministic.
///
/// Kept free of PG so the state-transition tests are plain unit tests.
fn tables_recovered(cooldown_keys: &[String], still_stale_tables: &[String]) -> Vec<String> {
    let mut recovered: Vec<String> = cooldown_keys
        .iter()
        .filter(|k| !NON_RECONCILE_COOLDOWN_KEYS.contains(&k.as_str()))
        .filter(|k| !still_stale_tables.iter().any(|s| s == *k))
        .cloned()
        .collect();
    recovered.sort();
    recovered.dedup();
    recovered
}

/// Paired recovery notification for [`check_level_drift_and_alert`].
///
/// The level-triggered alert was fire-and-forget: an operator who fixed
/// the lag got silence, and a recurrence inside the 24h cooldown was
/// silent too. This closes both gaps — when a table that currently HAS a
/// cooldown row no longer has any unconverged rows older than the stale
/// threshold, emit ONE `:white_check_mark:` all-clear naming the site and
/// the table(s), then drop those cooldown rows so a recurrence alerts
/// immediately.
///
/// Matches the two recovery-notification precedents in this codebase:
/// `bin/writeback.rs::send_resolved_alert` (exhausted job RESOLVED) and
/// `bin/sync.rs::format_recovery_message` (CT watermark RECOVERED).
///
/// Alert hygiene only — reads and writes NOTHING but the cooldown table,
/// and is deliberately NOT behind any data-write feature flag.
/// Best-effort throughout: a PG or Slack failure logs and continues.
async fn check_level_drift_recovery_and_notify(
    pg_pool: &PgPool,
    slack: Option<&SlackClient>,
    site_id: &str,
    still_stale_tables: &[String],
) {
    let cooldown_keys = level_alert_cooldown_keys_pg(pg_pool, site_id).await;
    let recovered = tables_recovered(&cooldown_keys, still_stale_tables);

    if recovered.is_empty() {
        tracing::debug!(
            site = %site_id,
            cooldown_keys = cooldown_keys.len(),
            still_stale = still_stale_tables.len(),
            "[Sync] Sync-lag all-clear: nothing recovered this tick"
        );
        return;
    }

    for table in &recovered {
        tracing::info!(
            site = %site_id,
            table,
            stale_hours = LEVEL_DRIFT_STALE_INTERVAL_HOURS,
            "[Sync] Sync-lag all-clear: table has no unconverged rows past threshold — \
             clearing level-alert cooldown"
        );
    }

    if let Some(slack) = slack {
        let body = recovered
            .iter()
            .map(|t| format!("• `{t}`"))
            .collect::<Vec<_>>()
            .join("\n");
        let msg = SlackMessage::with_site_text(
            site_id,
            format!(
                ":white_check_mark: *Reconcile rows CONVERGED* :white_check_mark:\n\
                 Every `ht_reconcile_log` row older than \
                 {LEVEL_DRIFT_STALE_INTERVAL_HOURS}h has converged for:\n\
                 {body}\n\
                 _Closure of the_ `:warning:` _unconverged alert sent earlier. The \
                 per-table {LEVEL_DRIFT_COOLDOWN_HOURS}h cooldown is reset, so a \
                 recurrence alerts on the next tick instead of waiting out a stale \
                 window._"
            ),
        );
        slack.send_message(&msg).await;
    } else {
        tracing::info!(
            site = %site_id,
            "[Sync] Slack not configured; sync-lag all-clear logged only ({} table(s))",
            recovered.len()
        );
    }

    for table in &recovered {
        clear_level_alert_cooldown_pg(pg_pool, site_id, table).await;
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
/// - Fires the paired all-clear for any table that HAS a cooldown row but
///   no longer has stale rows (see
///   [`check_level_drift_recovery_and_notify`]) — the alert used to be
///   fire-and-forget, so an operator who fixed the lag got silence and a
///   recurrence inside the 24h window was silent too.
/// - Best-effort: a failed PG query or Slack POST only logs a warning.
async fn check_level_drift_and_alert(pg_pool: &PgPool, slack: Option<&SlackClient>, site_id: &str) {
    let rows = sqlx::query_as::<_, (String, i64)>(sqlx::AssertSqlSafe(format!(
        "SELECT table_name, count(*) \
           FROM ht_reconcile_log \
          WHERE resolved_at IS NULL \
            AND divergence_kind IS NOT NULL \
            AND detected_at < now() - interval '{LEVEL_DRIFT_STALE_INTERVAL_HOURS} hours' \
          GROUP BY table_name"
    )))
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

    // Paired recovery notification — MUST run before the `counts.is_empty()`
    // early return below, because "no table has stale rows any more" is
    // exactly the everything-recovered case an operator needs to hear about.
    let still_stale_tables: Vec<String> = counts.iter().map(|(t, _)| t.clone()).collect();
    check_level_drift_recovery_and_notify(pg_pool, slack, site_id, &still_stale_tables).await;

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
            ":warning: *Reconcile rows unconverged >{LEVEL_DRIFT_STALE_INTERVAL_HOURS}h* :warning:\n\
             `ht_reconcile_log` row(s) the auto-resolve sweep has not closed in \
             over {LEVEL_DRIFT_STALE_INTERVAL_HOURS} hours. This is NOT sync lag — \
             past this threshold it will not clear on its own:\n\
             {body}\n\
             _Check `divergence_kind` first. `missing_pg` with a live legacy row is a \
             *dropped legacy change*: the record is absent from our app entirely and \
             no tick will fix it. Do NOT blanket-set `resolved_at` — that closes rows \
             whether or not canonical landed. Triage: docs/runbook-sync.md §9b. \
             Per-table cooldown {LEVEL_DRIFT_COOLDOWN_HOURS}h; an all-clear fires \
             when the table clears._"
        ),
    );
    slack.send_message(&msg).await;
}

/// Default days-past-expected-checkout before an `active` check-in is
/// flagged stale. Overridable via `STALE_CHECKIN_ALERT_DAYS`.
const STALE_CHECKIN_ALERT_DAYS_DEFAULT: i32 = 2;

/// Cooldown key (in `ht_level_drift_alert_cooldowns`, the `table_name`
/// column) for the stale-checkin tripwire — reuses the level-drift cooldown
/// table so the alert fires at most once per `LEVEL_DRIFT_COOLDOWN_HOURS`
/// (default 24h) per site, even while a backlog persists.
const STALE_CHECKIN_COOLDOWN_KEY: &str = "stale_active_checkin";

/// Resolve the stale-checkin threshold (days) from `STALE_CHECKIN_ALERT_DAYS`,
/// clamped to a sane floor of 1 day. Falls back to the default on a missing
/// or unparseable value.
fn stale_checkin_alert_days() -> i32 {
    std::env::var("STALE_CHECKIN_ALERT_DAYS")
        .ok()
        .and_then(|v| v.trim().parse::<i32>().ok())
        .filter(|d| *d >= 1)
        .unwrap_or(STALE_CHECKIN_ALERT_DAYS_DEFAULT)
}

/// P0 stale-checkin tripwire (task #66). A pure-PG safety net, INDEPENDENT
/// of the MSSQL-backed reconcile sweep: a dropped checkout leaves the
/// canonical check-in `active` with `cin_expected_checkout` in the past
/// indefinitely (the CT event that would flip it is gone after retention,
/// and the reconcile hash historically didn't cover the per-room checkout
/// flip — the 2026-06-28 room-114 / cin 19906 incident, see
/// [`checkin_canonical_hash`]). This catches that class on ANY site, even
/// one whose reconcile sweep doesn't run or whose legacy MSSQL is
/// unreachable — because it only reads canonical PG.
///
/// Flags `cin_status='active'` rows whose `cin_expected_checkout` is more
/// than `STALE_CHECKIN_ALERT_DAYS` (default 2) days in the past, fires ONE
/// Slack alert per site gated by the shared 24h level-drift cooldown, and is
/// best-effort throughout (a PG or Slack failure only logs a warning).
pub async fn check_stale_active_checkins_and_alert(
    pg_pool: &PgPool,
    slack: Option<&SlackClient>,
    site_id: &str,
) {
    let days = stale_checkin_alert_days();

    let rows = sqlx::query_as::<_, (String, chrono::DateTime<chrono::Utc>)>(
        "SELECT cin_no, cin_expected_checkout \
           FROM ht_checkins \
          WHERE cin_status = 'active' \
            AND cin_expected_checkout IS NOT NULL \
            AND cin_expected_checkout < now() - make_interval(days => $1) \
          ORDER BY cin_expected_checkout \
          LIMIT 100",
    )
    .bind(days)
    .fetch_all(pg_pool)
    .await;

    let stale = match rows {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                error = %e,
                "[Sync] Failed to query ht_checkins for stale-checkin tripwire — observability degraded"
            );
            return;
        }
    };

    if stale.is_empty() {
        tracing::debug!(
            site = %site_id,
            threshold_days = days,
            "[Sync] Stale-checkin tripwire: no active check-ins past expected checkout"
        );
        return;
    }

    tracing::warn!(
        site = %site_id,
        count = stale.len(),
        threshold_days = days,
        "[Sync] Stale-checkin tripwire: active check-ins long past expected checkout (likely dropped checkout)"
    );

    // Cooldown-gate the Slack alert (reuse the level-drift cooldown table so
    // a persistent backlog doesn't refire every tick). Check eligibility AND
    // mark in the same branch — if we're going to alert, we mark.
    let cooldown = std::time::Duration::from_secs((LEVEL_DRIFT_COOLDOWN_HOURS * 3600) as u64);
    if !level_alert_eligible_pg(pg_pool, site_id, STALE_CHECKIN_COOLDOWN_KEY, cooldown).await {
        tracing::debug!(
            site = %site_id,
            "[Sync] Stale-checkin alert suppressed by cooldown"
        );
        return;
    }

    let Some(slack) = slack else {
        tracing::info!(
            site = %site_id,
            "[Sync] Slack not configured; stale-checkin tripwire logged only ({} row(s))",
            stale.len()
        );
        return;
    };

    let now = chrono::Utc::now();
    let shown = stale.len().min(15);
    let body = stale
        .iter()
        .take(shown)
        .map(|(cin_no, exp)| {
            let overdue_days = (now - *exp).num_days();
            format!("• `{cin_no}` — {overdue_days}d past expected checkout")
        })
        .collect::<Vec<_>>()
        .join("\n");
    let more = if stale.len() > shown {
        format!("\n…and {} more", stale.len() - shown)
    } else {
        String::new()
    };

    let msg = SlackMessage::with_site_text(
        site_id,
        format!(
            ":hourglass_flowing_sand: *Stale active check-in(s) — likely dropped checkout* :hourglass_flowing_sand:\n\
             {count} canonical check-in(s) are still `active` more than {days} day(s) past their \
             expected checkout. This usually means a checkout CT event was dropped (past MSSQL \
             retention, so it won't self-heal) — the room shows occupied in the new app while \
             iHOTEL has it checked out:\n\
             {body}{more}\n\
             _Reconcile the row(s) to match iHOTEL (see the 2026-06-28 cin 19906 / room 114 \
             playbook). Pure-PG tripwire; per-site cooldown {cooldown_h}h._",
            count = stale.len(),
            cooldown_h = LEVEL_DRIFT_COOLDOWN_HOURS,
        ),
    );
    slack.send_message(&msg).await;
    mark_level_alert_sent_pg(pg_pool, site_id, STALE_CHECKIN_COOLDOWN_KEY).await;
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
fn ct_lag_is_warning(version_lag: i64, poll_age_seconds: i64, thresholds: CtLagThresholds) -> bool {
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

/// Resilience PR R3 — is the CT watcher holding per-table watermarks?
///
/// Mirrors the exact idiom `bin/sync.rs` uses to read the same flag
/// (strict `== "true"`, default OFF) so the reconcile-tick health check
/// and the watcher binary can never disagree about which watermark table
/// is authoritative. When this is on, the single-row `legacy_ct_state`
/// stops advancing and `legacy_ct_state_per_table` carries the real
/// progress — see `crate::sync::watermark` for the dual-mode contract.
fn per_table_watermark_enabled() -> bool {
    env::var("SYNC_PER_TABLE_WATERMARK")
        .map(|v| v == "true")
        .unwrap_or(false)
}

/// One row of `legacy_ct_state_per_table`, as read by the per-tick CT
/// health check in per-table-watermark mode.
#[derive(Debug, Clone)]
struct PerTableWatermark {
    table_name: String,
    last_seen_version: i64,
    last_polled_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Read every `legacy_ct_state_per_table` row. An empty result means the
/// per-table watermarks haven't been seeded yet (pre-migration-050 or
/// pre-bootstrap) — the caller logs and skips, exactly like the global
/// path's `RowNotFound` branch.
async fn read_pg_ct_watermarks_per_table(
    pg_pool: &PgPool,
) -> Result<Vec<PerTableWatermark>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String, i64, Option<chrono::DateTime<chrono::Utc>>)>(
        "SELECT table_name, last_seen_version, last_polled_at \
           FROM legacy_ct_state_per_table",
    )
    .fetch_all(pg_pool)
    .await?;
    Ok(rows
        .into_iter()
        .map(
            |(table_name, last_seen_version, last_polled_at)| PerTableWatermark {
                table_name,
                last_seen_version,
                last_polled_at,
            },
        )
        .collect())
}

/// Pure picker for the per-table CT health check: given every per-table
/// watermark row, the current legacy CT version, and `now`, return the
/// STALEST table together with its `(version_lag, poll_age_seconds)`.
///
/// Ranking, in order: largest `version_lag` (i.e. smallest
/// `last_seen_version`) first, then largest poll age (i.e. oldest
/// `last_polled_at`), then `table_name` so the pick is deterministic when
/// every table is equally healthy. A `NULL` `last_polled_at` is treated as
/// infinitely old (`i64::MAX`), matching the global path's never-polled
/// handling.
///
/// Returns `None` only for an empty input (per-table watermarks not seeded
/// yet). Kept free of PG / env / clock so the unit tests can drive the
/// ranking directly.
fn stalest_per_table_watermark(
    rows: &[PerTableWatermark],
    current_version: i64,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<(&PerTableWatermark, i64, i64)> {
    rows.iter()
        .map(|w| {
            // A watermark AHEAD of `current_version` is a CT anomaly (the
            // version is monotonic). `saturating_sub` alone does not help
            // here — on a signed `i64` it saturates at `i64::MIN`, not at
            // zero — so clamp explicitly. Without the clamp the log line
            // would report a nonsensical negative lag. (The global arm keeps
            // its pre-existing unclamped form so its behaviour stays
            // byte-identical; it is unreachable in practice for the same
            // monotonicity reason.)
            let version_lag = current_version.saturating_sub(w.last_seen_version).max(0);
            let poll_age_seconds = w
                .last_polled_at
                .map(|polled| now.signed_duration_since(polled).num_seconds().max(0))
                .unwrap_or(i64::MAX);
            (w, version_lag, poll_age_seconds)
        })
        .max_by(|a, b| {
            a.1.cmp(&b.1)
                .then_with(|| a.2.cmp(&b.2))
                // `max_by` keeps the LAST maximum, so invert the name
                // comparison to land on the alphabetically-first table.
                .then_with(|| b.0.table_name.cmp(&a.0.table_name))
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
/// **Per-table watermark mode (R3).** `SYNC_PER_TABLE_WATERMARK` selects
/// which watermark table is authoritative. With the flag OFF (default)
/// this reads the global `legacy_ct_state WHERE id = 1` row — behaviour
/// unchanged. With the flag ON the global row stops advancing (it freezes
/// at its bootstrap value), so reading it would emit a permanently-stuck
/// `version_lag` warning every tick and mask the real per-table lag;
/// instead we read every `legacy_ct_state_per_table` row and report the
/// STALEST table by name (see [`stalest_per_table_watermark`]). The
/// sibling gap in `bin/sync.rs::watermark_stall_alert_eligible` lives in
/// the CT-watcher binary and is tracked with that file.
async fn check_ct_watcher_lag(legacy_pool: &DbPool, pg_pool: &PgPool, site_id: &str) {
    if per_table_watermark_enabled() {
        check_ct_watcher_lag_per_table(legacy_pool, pg_pool, site_id).await;
    } else {
        check_ct_watcher_lag_global(legacy_pool, pg_pool, site_id).await;
    }
}

/// Global-watermark arm of [`check_ct_watcher_lag`] — the default path,
/// byte-identical to the pre-R3 behaviour.
async fn check_ct_watcher_lag_global(legacy_pool: &DbPool, pg_pool: &PgPool, site_id: &str) {
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

/// Per-table-watermark arm of [`check_ct_watcher_lag`]. Reads every
/// `legacy_ct_state_per_table` row, ranks them with
/// [`stalest_per_table_watermark`], and reports the worst one WITH ITS
/// TABLE NAME so an operator can see which table is behind rather than a
/// meaningless global figure. Same warn/debug message text as the global
/// arm (plus a `table` field) so existing log greps keep working.
async fn check_ct_watcher_lag_per_table(legacy_pool: &DbPool, pg_pool: &PgPool, site_id: &str) {
    let watermarks = match read_pg_ct_watermarks_per_table(pg_pool).await {
        Ok(w) => w,
        Err(e) => {
            tracing::warn!(
                site = %site_id,
                error = %e,
                "[Sync] CT watcher health: failed to read legacy_ct_state_per_table — \
                 observability degraded for this tick"
            );
            return;
        }
    };

    if watermarks.is_empty() {
        tracing::debug!(
            site = %site_id,
            "[Sync] CT watcher health: legacy_ct_state_per_table has no rows yet — \
             watcher pre-bootstrap, skipping lag check"
        );
        return;
    }

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

    let Some((stalest, version_lag, poll_age_seconds)) =
        stalest_per_table_watermark(&watermarks, current_version, chrono::Utc::now())
    else {
        // Unreachable — `watermarks` is non-empty by the guard above.
        return;
    };

    let last_seen_version = stalest.last_seen_version;
    let thresholds = ct_lag_thresholds_from_env(site_id);
    if ct_lag_is_warning(version_lag, poll_age_seconds, thresholds) {
        tracing::warn!(
            site = %site_id,
            table = %stalest.table_name,
            tables_tracked = watermarks.len(),
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
            table = %stalest.table_name,
            tables_tracked = watermarks.len(),
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

/// Aggregate all detail rows for one `(Book_No, Book_Room_Type)` PK
/// into a single deterministic SHA256 hash. Sorts `details` in place
/// by `(book_date, book_date_in, book_date_out, book_cust_name,
/// book_cust_id, book_status)` — every non-key field — so the hash is
/// independent of the order MSSQL returned the rows.
///
/// **Retired in v2.63.0** — the multi-row aggregate hash is no longer
/// used by either reconcile mode (the bookings sweep takes a single
/// representative row per composite PK). Kept for the unit tests in
/// `mod tests` which still pin the determinism contract, useful both
/// if the helper is ever resurrected and as a reference for the
/// post-v2.63.0 `sort_booking_details` extraction.
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

/// Count DISTINCT non-empty `Cin_Room_No` values across a check-in's
/// `HT_CheckIn_Ds` rows.
///
/// Why: `ht_checkin_rooms` carries a `UNIQUE (cr_cin_id, cr_room_id)`
/// constraint, so "distinct rooms in folio" IS the canonical truth.
/// Raw `aggregate.rooms.len()` overcounts whenever iHOTEL records
/// multiple HT_CheckIn_Ds detail rows for the same room (extends,
/// re-keys, deposit returns), which then surfaces as spurious
/// `cardinality` drift in `ht_reconcile_log`.
///
/// Concrete trigger (CH22-000722, 2026-05-19): 3 HT_CheckIn_Ds rows,
/// all `Cin_Room_No='417'`, same `Cin_cust_no`, only `Cin_Room_Out`
/// differs across rows. Raw len → 3, junction count → 1, classifier
/// → `cardinality` (false positive). With this helper: distinct → 1,
/// matches the junction.
///
/// Skip semantics mirror [`crate::sync::mappers::checkin::project_rooms`]
/// (file `mappers/checkin.rs`, around the `Cin_Room_No` guards) —
/// rows whose `Cin_Room_No` is NULL or empty are dropped from the
/// junction projection, so they must also be dropped here so the two
/// counts compare apples-to-apples.
///
/// Errors from `try_get_str` are swallowed (the row is treated as
/// having no room number) rather than bubbled. Rationale: this helper
/// runs inside the reconcile loop where a hard error on a single
/// malformed row would abort the entire tick. The mapper-side
/// projection (`project_rooms`) propagates the same errors loudly so
/// real schema regressions still surface; here, conservative
/// "skip-on-probe-failure" beats blocking the tick.
fn count_distinct_legacy_checkin_rooms(
    rooms: &[crate::sync::row::test_support::HashMapRow],
) -> i32 {
    use crate::sync::row::MappableRow;
    use std::collections::HashSet;

    let mut distinct: HashSet<String> = HashSet::new();
    for r in rooms {
        let Ok(Some(room_no)) = r.try_get_str("Cin_Room_No") else {
            continue;
        };
        if room_no.is_empty() {
            continue;
        }
        distinct.insert(room_no.to_string());
    }
    distinct.len() as i32
}

/// Build the `mssql_row_json` payload for a check-in PK aggregate.
/// Reads the legacy header (`HT_CheckIn_H`) once for the cross-row
/// fields (`Cin_Date_in`, `Cin_cust_no`, `Cin_status`) and pairs them
/// with each `HT_CheckIn_Ds` row's `Cin_Room_No` / `Cin_Room_Out` so
/// an operator triaging the divergence sees what changed across every
/// booked room — not just the first.
///
/// JSON shape is preserved across the 2026-05-18 unification refactor
/// (v2.63.x → mapper-backed) so dashboards that parse the column keys
/// continue to work. Every detail row repeats the header-derived
/// fields verbatim (same redundancy the per-row JOIN projection used
/// to produce), and `Cin_cust_name` is left `null` because the
/// column was always populated from a view-derived alias that the
/// parent loader does not project (kept under the same key for shape
/// stability — see the original `CheckinDetail.cust_name = None`
/// rationale in the prior projection's doc comment).
fn checkin_aggregate_json(
    cin_no: &str,
    aggregate: &crate::sync::parent_loader::CheckInAggregate,
) -> serde_json::Value {
    use crate::sync::row::MappableRow;

    let header_date_in = aggregate
        .header
        .as_ref()
        .and_then(|h| h.try_get_datetime("Cin_Date_in").ok().flatten())
        .map(|dt| dt.to_string());
    let header_cust_no = aggregate
        .header
        .as_ref()
        .and_then(|h| h.try_get_str("Cin_cust_no").ok().flatten())
        .map(str::to_string);
    let header_status = aggregate
        .header
        .as_ref()
        .and_then(|h| h.try_get_str("Cin_status").ok().flatten())
        .map(str::to_string);

    let rows: Vec<serde_json::Value> = aggregate
        .rooms
        .iter()
        .map(|r| {
            let room_no = r
                .try_get_str("Cin_Room_No")
                .ok()
                .flatten()
                .map(str::to_string);
            let room_out = r
                .try_get_datetime("Cin_Room_Out")
                .ok()
                .flatten()
                .map(|dt| dt.to_string());
            json!({
                "Cin_Room_No": room_no,
                // JSON key reflects the hashed source column
                // (`HT_CheckIn_H.Cin_Date_in`, not the per-room
                // `HT_CheckIn_Ds.Cin_Room_In`) so operators see what
                // actually fed the hash.
                "Cin_Date_in": header_date_in,
                "Cin_Room_Out": room_out,
                "Cin_cust_name": serde_json::Value::Null,
                "Cin_cust_no": header_cust_no,
                "Cin_status": header_status,
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

/// Project canonical `room_clean` (true = IS clean) back to the legacy
/// `Room_Clean` NEEDS-CLEANING literal for drift hashing. Legacy semantics are
/// inverted from canonical: 'no' = clean, 'yes' = dirty (needs cleaning). MUST
/// match the CT room mapper's `new_clean` inversion or every checked-out /
/// cleaned room would hash as drifted. Maintenance/use keep `bool_to_yesno`
/// ('yes' = the named state).
fn clean_bool_to_legacy_yesno(b: Option<bool>) -> &'static str {
    match b {
        Some(true) => "no",   // canonical clean -> legacy "no cleaning needed"
        Some(false) => "yes", // canonical dirty -> legacy "needs cleaning"
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
/// Full `cin_status` is deliberately excluded from the active-stay hash
/// shape: `View_CheckIn_Ds.Cin_status` is a per-room ledger state,
/// whereas canonical `ht_checkins.cin_status` is the header-derived
/// aggregate — different fields. The `checked_out` boolean below is the
/// ONE status bit we DO hash (see next paragraph).
///
/// **Checked-out flag (task #68, 2026-06-28).** `cin_checkout_time` alone
/// does NOT distinguish active-from-checked-out, because the hash feeds it
/// the *effective* checkout date — `actual` when checked out, else
/// `expected` — and the two coincide whenever a guest departs on the
/// booked date. A dropped checkout then leaves canonical `active`
/// (effective = expected) while legacy is checked-out (effective = actual)
/// with IDENTICAL dates → identical hashes → invisible drift. That is
/// exactly the 2026-06-28 room-114 / cin 19906 incident (out 05-16 =
/// expected 05-16). The `checked_out` bit — `cin_checkout_time.is_some()`
/// on BOTH sides — makes the active↔checked-out transition a first-class
/// hash input without importing the non-comparable full `cin_status`.
/// Agreeing rows still match (both `true` or both `false`); only a genuine
/// status divergence (dropped checkout/checkin) flips it.
///
/// **Cancelled folios (2026-05-19 — reconcile cleanup PR B).** When
/// iHOTEL cancels a check-in (`HT_CheckIn_H.Cin_status='ยกเลิก'`) it
/// also deletes the per-room `HT_CheckIn_Ds` rows. The CT mapper's
/// `derive_room_state` honours that by emitting
/// `canonical_status='cancelled'` with `first_room_no=None`.
/// Canonical PG, however, retains the original `legacy_room_no` on
/// the existing `ht_checkins` row so operators can still see WHICH
/// room was cancelled. Hashing room context on both sides therefore
/// diverges forever: legacy emits `""`, PG emits `Some("301")`. Six
/// stuck `value` drifts in production (CH26-005252, CH26-005270,
/// CH26-005487, CH26-005524, CH26-005527, CH26-005543) are the
/// live manifestation as of the 2026-05-19 audit.
///
/// When `cancelled = true`, this function returns a sentinel
/// `sha256("CANCELLED|{legacy_cin_no}")` and IGNORES every other
/// input. Both the legacy and canonical reconcile paths call with
/// the same `cancelled` decision (`cin_status == "cancelled"`), so
/// the sentinel collapses the parity gap deterministically. When
/// `cancelled = false`, the active-stay 5-field shape is unchanged
/// — pre-2026-05-19 hash bytes are preserved bit-for-bit.
fn checkin_canonical_hash(
    legacy_cin_no: &str,
    legacy_room_no: Option<&str>,
    cin_checkin_time: Option<&str>,
    cin_checkout_time: Option<&str>,
    legacy_cust_no: Option<&str>,
    checked_out: bool,
    cancelled: bool,
) -> String {
    if cancelled {
        return sha256(&format!("CANCELLED|{}", legacy_cin_no));
    }
    sha256(&format!(
        "{}|{}|{}|{}|{}|co={}",
        legacy_cin_no,
        legacy_room_no.unwrap_or(""),
        cin_checkin_time.unwrap_or(""),
        cin_checkout_time.unwrap_or(""),
        legacy_cust_no.unwrap_or(""),
        checked_out,
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
/// Insert a fresh `ht_reconcile_log` row UNLESS an unresolved row
/// already exists for the same `(table_name, legacy_pk, divergence_kind,
/// mssql_hash)` tuple. This dedupe prevents unbounded log growth for
/// kinds that re-fire every tick (most notably `cardinality`, which by
/// design never silences via the cache-ack path — see
/// `DivergenceKind::is_silenceable`).
///
/// **Why mssql_hash is part of the dedupe key:** for `value` drift, a
/// new MSSQL UPDATE produces a new `mssql_hash`, so a fresh row IS
/// recorded (new state worth investigating) — that's the intended
/// semantic. For `cardinality`, the MSSQL hash is stable across ticks
/// while the asymmetry persists, so only one unresolved row per PK
/// accumulates. For `missing_pg`, the canonical-missing predicate
/// produces a stable mssql_hash (the actual legacy hash for the PK),
/// same dedupe behavior.
///
/// 2026-05-18 incident driver: after PR #128 restored sync_checkins,
/// HF Hotel was inserting ~766 cardinality rows per 15-min tick (one
/// per multi-room folio in `HT_CheckIn_Ds`), projecting `ht_reconcile_log`
/// past 100k unresolved rows within 18-24 hours and degrading the
/// partial-index `idx_ht_reconcile_log_table_unresolved` query plan.
/// Cardinality is real drift but it doesn't get materially more drifted
/// with every re-detection — one row per (PK, hash) is enough.
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
    // Conditional INSERT: skip when an unresolved row with the same
    // (table_name, legacy_pk, divergence_kind, mssql_hash) already
    // exists. NULL-safe comparison via IS NOT DISTINCT FROM so
    // missing_pg rows (mssql_hash = legacy hash) and missing_mssql
    // (mssql_hash NULL) both dedupe correctly.
    let result = sqlx::query(
        "INSERT INTO ht_reconcile_log \
            (table_name, legacy_pk, pg_hash, mssql_hash, \
             mssql_row_json, pg_row_json, \
             divergence_kind, legacy_row_count, pg_row_count) \
         SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9 \
         WHERE NOT EXISTS ( \
             SELECT 1 FROM ht_reconcile_log \
              WHERE table_name = $1 \
                AND legacy_pk = $2 \
                AND divergence_kind = $7 \
                AND mssql_hash IS NOT DISTINCT FROM $4 \
                AND resolved_at IS NULL \
         )",
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
/// `ht_reconcile_log` may be auto-resolved when:
///
/// 1. **Primary convergence** — a freshly-projected legacy (MSSQL) hash
///    and a freshly-computed canonical PG hash are both present, non-empty,
///    and equal. A `None` on either side is normally an intentional skip
///    (missing-PG cases must persist until canonical actually catches up;
///    missing-MSSQL cases need operator review). This helper NEVER resolves
///    a missing-PG row on its own — the 2026-07-27 re-ingest arm makes
///    canonical appear FIRST (see [`reingest_missing_pg_eligible`]) and only
///    then re-tests through this same function, so the convergence contract
///    below stays the single place a row can be closed.
///
/// 2. **Stale-ghost convergence (bookings only)** — the legacy composite
///    key has since *disappeared* (`current_legacy_hash == None`) yet
///    canonical PG now exists and matches the legacy state recorded at
///    detection time (`recorded_mssql_hash`). A booking's legacy PK is
///    `{book_no}|{room_type}`, and the room-type half churns routinely
///    over a booking's life (room reassignment, multi-room edits). A row
///    first logged as `missing_pg` during the brief window before a
///    NEW-app booking links its `legacy_book_id` becomes permanently
///    unresolvable once that initial room-type line is swapped out: the
///    legacy detail row for that exact key is gone, so the primary arm —
///    which needs BOTH sides present — can never fire. Because the
///    booking-level hash is room-type-independent, a match against the
///    recorded `mssql_hash` proves the booking itself is fully reconciled;
///    the per-room-type key simply churned away. This arm is restricted
///    to bookings: a vanished legacy key for rooms / customers / checkins
///    is a genuine anomaly that must stay open for operator review.
///    (Live evidence 2026-06-24: bookings row `R015423|501`.)
///
/// The auto-resolve sweep only reaches this helper with a `None`
/// `current_legacy_hash` when the legacy re-fetch returned `Ok(None)`
/// (row genuinely absent) — a re-fetch *error* skips the row earlier, so
/// a transient MSSQL outage can never be mistaken for a churned key.
///
/// Pulled into a free function so the unit tests can exercise the
/// truth table without a live PG pool.
fn should_auto_resolve(
    table_name: &str,
    current_legacy_hash: Option<&str>,
    current_pg_hash: Option<&str>,
    recorded_mssql_hash: Option<&str>,
) -> bool {
    match (current_legacy_hash, current_pg_hash) {
        (Some(legacy), Some(pg)) if !legacy.is_empty() && !pg.is_empty() => legacy == pg,
        (None, Some(pg)) if table_name == "bookings" && !pg.is_empty() => {
            matches!(recorded_mssql_hash, Some(rec) if !rec.is_empty() && rec == pg)
        }
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
/// "checkins", "rooms"). Other table names return `Ok(None)` so the
/// row stays in the queue for operator review.
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
                let cancelled = c.is_cancelled();
                let checked_out = c.is_checked_out();
                checkin_canonical_hash(
                    legacy_pk,
                    c.legacy_room_no.as_deref(),
                    c.cin_checkin_time.map(|t| t.to_string()).as_deref(),
                    effective_checkout.as_deref(),
                    c.legacy_cust_no.as_deref(),
                    checked_out,
                    cancelled,
                )
            }))
        }
        "rooms" => {
            // Canonical-side re-hash from `ht_rooms_new` under the SAME
            // narrowed projection `sync_rooms` uses on the legacy side
            // (room_no + clean + maintenance + notes). Without this arm
            // every rooms drift row sat open forever — the post-detection
            // sweep fell through to `_ => Ok(None)` even after the
            // underlying state had converged (live A2-1 evidence,
            // 2026-05-18: current_legacy_hash=None current_pg_hash=None).
            let canonical = fetch_canonical_room(pg_pool, legacy_pk).await?;
            Ok(canonical.map(|c| {
                room_canonical_hash(
                    legacy_pk,
                    clean_bool_to_legacy_yesno(c.room_clean),
                    bool_to_yesno(c.room_maintenance),
                    c.room_notes.as_deref(),
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
/// `table_name` is outside the dispatched set (future entities that
/// don't yet have a legacy-side fetch path).
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
/// Customers + bookings + rooms still use their light-weight per-PK
/// projections (`fetch_legacy_customer_hash`, `fetch_legacy_booking_hash`,
/// `fetch_legacy_room_hash`) — the same unification is a follow-on for
/// those entities.
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
        "rooms" => fetch_legacy_room_hash(legacy_pool, legacy_pk).await,
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
    let aggregate = crate::sync::parent_loader::load_checkin_aggregate(legacy_pool, cin_no).await?;
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
        canonical.cin_checkout_time.is_some(),
        canonical.cin_status == "cancelled",
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
    let row_cust_no = row
        .get::<&str, _>("Cust_no")
        .unwrap_or_default()
        .to_string();
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
        let row_book_no = row
            .get::<&str, _>("Book_No")
            .unwrap_or_default()
            .to_string();
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

    let Some(mut details) = groups.remove(&(book_no.to_string(), room_type_key.to_string())) else {
        return Ok(None);
    };
    sort_booking_details(&mut details);
    let representative = details.first();
    let book_checkin_date =
        representative.and_then(|d| d.book_date_in.map(|dt| dt.date().to_string()));
    let book_checkout_date =
        representative.and_then(|d| d.book_date_out.map(|dt| dt.date().to_string()));
    let book_cust_id_owned = representative.and_then(|d| d.book_cust_id.clone());
    Ok(Some(booking_canonical_hash(
        book_no,
        book_checkin_date.as_deref(),
        book_checkout_date.as_deref(),
        book_cust_id_owned.as_deref(),
    )))
}

/// Single-PK MSSQL re-projection for rooms. Mirrors `sync_rooms`'
/// per-row hash construction (room_no + clean + maintenance + notes,
/// with `legacy_yesno_canonical` collapsing legacy literals) so the
/// auto-resolve sweep can re-fetch a room's CURRENT legacy hash and
/// compare against canonical. `Ok(None)` when the row no longer
/// exists on the legacy side (treated as "still drifted — leave for
/// operator review").
async fn fetch_legacy_room_hash(
    legacy_pool: &DbPool,
    room_no: &str,
) -> Result<Option<String>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;
    let sql = format!(
        "SELECT {projection} FROM HT_Rooms WHERE Room_no = @P1",
        projection = ROOMS_RECONCILE_PROJECTION.join(", "),
    );
    let mut q = Query::new(sql);
    q.bind(room_no);
    let rows = q.query(&mut conn).await?.into_first_result().await?;
    let Some(row) = rows.first() else {
        return Ok(None);
    };
    let row_room_no = row
        .get::<&str, _>("Room_no")
        .unwrap_or_default()
        .to_string();
    let room_clean = row.get::<&str, _>("Room_Clean").map(String::from);
    let room_manternace = row.get::<&str, _>("Room_Manternace").map(String::from);
    let room_details = row.get::<&str, _>("Room_Details").map(String::from);
    Ok(Some(room_canonical_hash(
        &row_room_no,
        legacy_yesno_canonical(room_clean.as_deref()),
        legacy_yesno_canonical(room_manternace.as_deref()),
        room_details.as_deref(),
    )))
}

/// Issue #204 (bug #2) — is the durable self-healing arm of the
/// auto-resolve sweep enabled?
///
/// Default **OFF** (ship dark). When unset / not `"true"`, the sweep stays
/// observational-only and performs ZERO canonical writes — behaviour is
/// byte-for-byte identical to its pre-#204 form. Flip to `"true"` only after
/// reception-coordinated verification (see MEMORY: flag flips are never "just
/// config").
fn reconcile_force_converge_enabled() -> bool {
    env::var("RECONCILE_FORCE_CONVERGE_ENABLED")
        .map(|v| v == "true")
        .unwrap_or(false)
}

/// Is the `missing_pg` re-ingest arm of the auto-resolve sweep enabled?
///
/// Deliberately a SEPARATE flag from [`reconcile_force_converge_enabled`]:
/// that one is already `true` in production on `sync-hfville`, so folding a
/// brand-new canonical-write class into it would ship this ON with no
/// coordinated flip. Re-ingesting a whole booking aggregate is materially
/// more consequential than the customers/rooms value-converge that flag was
/// scoped to, so it ships dark on its own switch.
///
/// Default **OFF**. The `== "true"` comparison is strict on purpose —
/// `"TRUE"`, `"1"` and `" true"` all evaluate false, matching every other
/// feature flag in the sync path. Flip to `"true"` only after
/// reception-coordinated verification (a flag flip is never "just config").
fn reconcile_reingest_missing_pg_enabled() -> bool {
    env::var("RECONCILE_REINGEST_MISSING_PG_ENABLED")
        .map(|v| v == "true")
        .unwrap_or(false)
}

/// Issue #204 (bug #2) — minimum age (seconds) an unresolved
/// `ht_reconcile_log` row must have before the force-converge arm will touch
/// it. A younger row is likely just waiting on an in-flight CT event, so we
/// leave it for the normal converge-on-its-own path. 3600s ≈ 4 sweep ticks
/// at the default 15-min cadence, i.e. only rows that have resisted several
/// observational sweeps qualify ("seen across 2+ sweeps").
const FORCE_CONVERGE_MIN_AGE_SECS: f64 = 3600.0;

/// Issue #204 (bug #2) — re-fetch a single `HT_Customers` base row by its
/// business key (`Cust_no`, which is what the reconcile loop stores as
/// `ht_reconcile_log.legacy_pk` for customers). The projection is the full
/// CT eager-fetch column set so the returned `tiberius::Row` carries every
/// column `CustomerMapper`'s `project` / `apply_upsert` reads. `Ok(None)`
/// when the legacy row no longer exists.
async fn fetch_legacy_customer_base_row(
    legacy_pool: &DbPool,
    cust_no: &str,
) -> Result<Option<tiberius::Row>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;
    let sql = format!(
        "SELECT {projection} FROM HT_Customers WHERE Cust_no = @P1",
        projection = crate::sync::mappers::customer::EAGER_FETCH_COLUMNS.join(", "),
    );
    let mut q = Query::new(sql);
    q.bind(cust_no);
    let rows = q.query(&mut conn).await?.into_first_result().await?;
    Ok(rows.into_iter().next())
}

/// Issue #204 (bug #2) — re-fetch a single `HT_Rooms` base row by `Room_no`
/// (the reconcile loop's `legacy_pk` for rooms). The column set mirrors
/// `room.rs`'s `ROOMS_SELECT_COLS` minus the CT-JOIN `t.` alias and MUST
/// cover every field `RoomMasterMapper::project_room` reads — `id` is the CT
/// PK but is also a real `HT_Rooms` column, re-projected here. `Ok(None)`
/// when the legacy row no longer exists.
async fn fetch_legacy_room_base_row(
    legacy_pool: &DbPool,
    room_no: &str,
) -> Result<Option<tiberius::Row>, Box<dyn std::error::Error + Send + Sync>> {
    let mut conn = legacy_pool.get().await?;
    let sql = "SELECT id, Room_no, Room_Type, Room_Clean, Room_Use, \
               Room_Manternace, Room_Details, Room_Use_Count, Room_X, Room_Y, \
               Room_Group, Room_Power_OPEN, Room_Power_CLOSE, Room_Power_STATUS, \
               Room_Polity FROM HT_Rooms WHERE Room_no = @P1"
        .to_string();
    let mut q = Query::new(sql);
    q.bind(room_no);
    let rows = q.query(&mut conn).await?.into_first_result().await?;
    Ok(rows.into_iter().next())
}

/// Issue #204 (bug #2) — durable self-heal for a single `customers` / `rooms`
/// reconcile row that has resisted observational convergence.
///
/// Re-fetches the CURRENT legacy base row by its PK and re-projects it
/// through the EXISTING CT upsert path — the very same `mapper.apply(...)`
/// the watcher uses — INSIDE a fresh PG transaction. Canonical fields are
/// therefore written by the mapper, never by hand here. The mapper's I/U
/// branch is an UPSERT that updates the existing canonical row in place
/// (verified: `customer::apply_upsert` / `room::apply_room_upsert`), and its
/// own idempotency guard collapses a no-op to no write — so this is safe to
/// re-drive.
///
/// The returned `DomainEvent` is intentionally DROPPED: the value we just
/// wrote came FROM legacy, so there is nothing to write back — emitting an
/// outbox event would only produce a converging no-op echo against MSSQL.
/// The sweep is a silent backstop; a genuine subsequent CT edit re-emits the
/// normal `CustomerModified` / room event.
///
/// Returns `Ok(true)` when a re-projection was attempted and committed,
/// `Ok(false)` when the legacy row no longer exists (or the table is outside
/// the supported set) so there is nothing to project from.
///
/// `op` is the `ChangeOp` handed to the mapper. The value-drift caller passes
/// `ChangeOp::Update` (the canonical row exists, we're correcting its
/// fields); the `missing_pg` re-ingest caller passes `ChangeOp::Insert` (no
/// canonical row at all). Both land in the same mapper UPSERT branch — the op
/// only shapes the `DomainEvent`, which we drop — but the honest value keeps
/// the intent readable at the call site.
///
/// **Bookings (2026-07-27, `missing_pg` re-ingest).** The bookings arm
/// re-drives the CT watcher's own aggregate path: `load_booking_aggregate`
/// re-reads `HT_Book_H` + `HT_Book_Ds` + `HT_Book_Date` for the `book_no`,
/// then `apply_booking_aggregate` projects it into canonical in a fresh PG
/// tx. The legacy pool is passed through so the mapper's customer
/// eager-mirror fallback works (an unresolvable customer FK errors rather
/// than silently skipping — the 2026-06-03 silent-drop class). Still a
/// PG-write-only path: the legacy side is read exclusively.
async fn force_converge_reconcile_row(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
    table_name: &str,
    legacy_pk: &str,
    op: ChangeOp,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    match table_name {
        "customers" => {
            let Some(row) = fetch_legacy_customer_base_row(legacy_pool, legacy_pk).await? else {
                return Ok(false);
            };
            let mut tx = pg_pool.begin().await?;
            // apply runs the full UPSERT + the mapper's idempotency check, so
            // it inserts when canonical is absent and updates in place when
            // it is not.
            let _evt = CustomerMapper
                .apply(&mut tx, op, Some(&row as &dyn MappableRow))
                .await?;
            tx.commit().await?;
            Ok(true)
        }
        "rooms" => {
            let Some(row) = fetch_legacy_room_base_row(legacy_pool, legacy_pk).await? else {
                return Ok(false);
            };
            let mut tx = pg_pool.begin().await?;
            let _evt = RoomMasterMapper
                .apply(&mut tx, op, Some(&row as &dyn MappableRow))
                .await?;
            tx.commit().await?;
            Ok(true)
        }
        "bookings" => {
            // `ht_reconcile_log.legacy_pk` for bookings is the composite
            // "{book_no}|{room_type}"; the aggregate is keyed on `book_no`
            // alone, so several reconcile rows for one booking all re-drive
            // the SAME aggregate. That is safe: `apply_booking_aggregate` is
            // idempotent and returns `Ok(None)` once canonical already
            // mirrors legacy, so the 2nd and 3rd row of a multi-room-type
            // booking are no-ops that still get their convergence re-tested.
            let (book_no, _room_type_key) = parse_booking_legacy_pk(legacy_pk);
            let aggregate =
                crate::sync::parent_loader::load_booking_aggregate(legacy_pool, book_no).await?;
            if !aggregate.is_present() {
                return Ok(false);
            }
            let mut tx = pg_pool.begin().await?;
            let _evt = crate::sync::mappers::apply_booking_aggregate(
                &mut tx,
                Some(legacy_pool),
                &aggregate,
                book_no,
            )
            .await?;
            tx.commit().await?;
            Ok(true)
        }
        // checkins are multi-row aggregates whose self-heal is still out of
        // scope — leave them to the normal paths / operator review.
        _ => Ok(false),
    }
}

/// Tables the #204 value-drift force-converge arm will repair. Single-PK
/// mappers whose `apply` is safe to re-drive idempotently.
const FORCE_CONVERGE_VALUE_DRIFT_TABLES: &[&str] = &["customers", "rooms"];

/// Tables the `missing_pg` re-ingest arm will repair. Customers first-class
/// (flat single-PK mapper), bookings via the aggregate loader. `rooms` is
/// excluded because a room absent from canonical is a provisioning gap, not
/// a dropped CT event; `checkins` is excluded until its aggregate re-ingest
/// has been verified the same way.
const REINGEST_MISSING_PG_TABLES: &[&str] = &["customers", "bookings"];

/// Pure gate for the #204 value-drift force-converge arm — BOTH hashes
/// present (a genuine value drift, not a missing-side case), an eligible
/// table, past the min-age threshold, flag on.
///
/// Extracted so the "value drift still routes to the existing arm,
/// unchanged" invariant is pinned by a unit test rather than by reading the
/// sweep's control flow.
fn force_converge_value_drift_eligible(
    table_name: &str,
    current_legacy_hash: Option<&str>,
    current_pg_hash: Option<&str>,
    age_secs: f64,
    enabled: bool,
) -> bool {
    enabled
        && FORCE_CONVERGE_VALUE_DRIFT_TABLES.contains(&table_name)
        && current_legacy_hash.is_some()
        && current_pg_hash.is_some()
        && age_secs >= FORCE_CONVERGE_MIN_AGE_SECS
}

/// Pure gate for the `missing_pg` re-ingest arm (2026-07-27).
///
/// Repairs a DROPPED INGEST: legacy still has the row, canonical never got
/// it, and CT's 2-day retention has long since aged the event out so the
/// watcher can never redeliver it. Live shape that motivated this: customer
/// `C2413` + bookings `R002066|110` / `|112` / `|217`, unconverged for 16
/// days after a 2026-07-11 cross-table watermark clobber on HF Ville.
///
/// Conditions, ALL required:
/// * flag on (`RECONCILE_REINGEST_MISSING_PG_ENABLED`, default off);
/// * `table_name` in [`REINGEST_MISSING_PG_TABLES`];
/// * legacy hash PRESENT — a VANISHED legacy row must never trip this arm.
///   That is the genuine-anomaly case [`should_auto_resolve`] deliberately
///   protects and `rooms_dispatch_missing_legacy_row_does_not_auto_resolve`
///   pins; re-ingesting nothing would be meaningless and closing the row
///   would hide a real deletion;
/// * canonical hash ABSENT — this arm only inserts what is missing; a value
///   drift belongs to [`force_converge_value_drift_eligible`];
/// * past [`FORCE_CONVERGE_MIN_AGE_SECS`] so an in-flight CT event isn't
///   raced.
fn reingest_missing_pg_eligible(
    table_name: &str,
    current_legacy_hash: Option<&str>,
    current_pg_hash: Option<&str>,
    age_secs: f64,
    enabled: bool,
) -> bool {
    enabled
        && REINGEST_MISSING_PG_TABLES.contains(&table_name)
        && current_legacy_hash.is_some_and(|h| !h.is_empty())
        && current_pg_hash.is_none()
        && age_secs >= FORCE_CONVERGE_MIN_AGE_SECS
}

/// One unresolved `ht_reconcile_log` candidate as fetched by the
/// auto-resolve sweep: `(id, table_name, legacy_pk, mssql_hash, age_secs)`.
type ReconcileCandidate = (i64, String, String, Option<String>, f64);

/// FK-dependency rank for the auto-resolve sweep's candidate ordering.
/// Lower runs first, so a parent is always re-ingested before anything that
/// points at it: customers → rooms → bookings → checkins.
///
/// **This ordering is load-bearing, not cosmetic.** `apply_booking_aggregate`
/// needs the booking's customer to exist in canonical (it eager-mirrors on a
/// miss and ERRORS if even that fails); check-ins point at rooms and
/// bookings. Healing a dependent before its parent within one sweep pass
/// turns a repairable row into an error.
fn reconcile_table_fk_rank(table_name: &str) -> u8 {
    match table_name {
        "customers" => 0,
        "rooms" => 1,
        "bookings" => 2,
        "checkins" => 3,
        _ => 4,
    }
}

/// Order the sweep's candidate batch: FK parents first, then oldest first,
/// then `id` as a deterministic tie-break.
///
/// "Oldest first" is expressed as DESCENDING `age_secs` because `age_secs`
/// is `NOW() - detected_at` — a larger age IS an earlier `detected_at`.
///
/// This deliberately does NOT mirror the SQL `ORDER BY` in
/// [`auto_resolve_reconcile_log`], and the split is the point. SQL selects
/// the batch purely by age, which is what fixes the unordered-`LIMIT 500`
/// starvation hazard (with a backlog >500 rows PG returned an arbitrary
/// subset, so a row could age past 4h forever without ever being retested).
/// The FK-rank leg lives here, applied AFTER the fetch, so parents are
/// processed before dependents within the batch without letting a large
/// parent backlog starve `checkins` out of the selection entirely. Keeping
/// FK rank out of the query is intentional — do not "restore" it there.
fn sort_reconcile_candidates(rows: &mut [ReconcileCandidate]) {
    rows.sort_by(|a, b| {
        reconcile_table_fk_rank(&a.1)
            .cmp(&reconcile_table_fk_rank(&b.1))
            .then_with(|| b.4.total_cmp(&a.4))
            .then_with(|| a.0.cmp(&b.0))
    });
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
///
/// **Issue #204 (bug #2) — durable self-healing arm (ship dark).** By
/// default this sweep is observational-only: it resolves a row ONLY when
/// legacy and canonical ALREADY agree, so a durable value drift that will
/// never get a CT event (migration-born, or a hand-edit) is observed
/// forever but never repaired. When `RECONCILE_FORCE_CONVERGE_ENABLED` is
/// `"true"`, a long-lived `customers` / `rooms` value-drift row is repaired
/// by re-projecting the CURRENT legacy row through the EXISTING CT upsert
/// path (`force_converge_reconcile_row`) before the convergence re-test.
/// With the flag OFF the behaviour is identical to the pre-#204 sweep — no
/// canonical writes.
///
/// **`missing_pg` re-ingest arm (2026-07-27, ship dark).** A second,
/// separately-flagged arm (`RECONCILE_REINGEST_MISSING_PG_ENABLED`, default
/// off) repairs the DROPPED-INGEST shape: legacy row present, canonical row
/// absent, past the min-age threshold. Those rows have no automated path to
/// closure — the observational test needs both hashes, and CT's 2-day
/// retention means the watcher can never redeliver the event — so they sit
/// unconverged forever while the >4h level alert refires every 24h. The arm
/// re-runs the NORMAL mapper for the key and closes the row only if the two
/// sides then converge. A VANISHED legacy row never trips it: that is the
/// genuine anomaly `should_auto_resolve` protects. See
/// [`reingest_missing_pg_eligible`].
async fn auto_resolve_reconcile_log(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
    site_id: &str,
) -> Result<usize, sqlx::Error> {
    // `age_secs` (issue #204 bug #2) gates the durable force-converge arm —
    // only rows that have resisted convergence for a while qualify.
    //
    // The `ORDER BY` is strictly age-based, and that is deliberate. Before
    // this it was absent entirely, so `LIMIT 500` returned an arbitrary
    // (heap-order, therefore stable) subset — rows outside it were never
    // retested and could age past the 4h level alert forever. Oldest-first
    // makes the cap a fair queue instead of a lottery.
    //
    // The FK guarantee (parents before dependents — a booking whose customer
    // is still missing hits `Ok(None)` in the mapper) is supplied in Rust by
    // [`sort_reconcile_candidates`] over the fetched batch, NOT here. Ranking
    // by table in SQL would reintroduce starvation from the other side: with
    // >500 unresolved `customers`+`rooms`+`bookings` rows, `checkins` would
    // never be swept at all. Sorting after the fetch gets both properties —
    // fair selection, correct ordering within the batch. Worst case, a
    // dependent lands in a batch whose parent did not; it stays open and the
    // next tick retries it, which is the same self-correcting path the
    // FK-defer class already relies on.
    let mut rows = sqlx::query_as::<_, ReconcileCandidate>(
        "SELECT id, table_name, legacy_pk, mssql_hash, \
                EXTRACT(EPOCH FROM (NOW() - detected_at))::float8 AS age_secs \
           FROM ht_reconcile_log \
          WHERE resolved_at IS NULL \
            AND divergence_kind IS NOT NULL \
          ORDER BY detected_at ASC, \
                   id ASC \
          LIMIT 500",
    )
    .fetch_all(pg_pool)
    .await?;
    // Belt-and-braces: re-apply the same order in Rust so the FK guarantee
    // survives a future edit to the query above, and so it is unit-testable.
    sort_reconcile_candidates(&mut rows);

    // Read the self-heal flags ONCE per sweep. Both default OFF ⇒ the sweep
    // stays observational-only and never writes canonical state.
    let force_converge_enabled = reconcile_force_converge_enabled();
    let reingest_missing_pg_enabled = reconcile_reingest_missing_pg_enabled();

    let mut resolved = 0usize;
    for (id, table_name, legacy_pk, recorded_mssql_hash, age_secs) in rows {
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

        let current_pg_hash = match compute_current_pg_hash(pg_pool, &table_name, &legacy_pk).await
        {
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

        if !should_auto_resolve(
            &table_name,
            current_legacy_hash.as_deref(),
            current_pg_hash.as_deref(),
            recorded_mssql_hash.as_deref(),
        ) {
            // ---------------------------------------------------------------
            // Issue #204 (bug #2) — durable self-healing arm.
            //
            // The convergence test above is observational-only: it resolves a
            // row ONLY when legacy and canonical ALREADY agree. A value drift
            // that will never receive a CT event (migration-born, or a
            // hand-edit on one side) is therefore observed forever but never
            // repaired. When the flag is ON and the row is a long-lived
            // `customers` / `rooms` *value* drift (both sides present, just
            // unequal), re-project the CURRENT legacy row through the EXISTING
            // CT upsert path, then re-hash; if it now converges, fall through
            // to mark it resolved.
            //
            // Conservative guards:
            //   * flag OFF → branch never taken; the `else` below logs and
            //     leaves the row open exactly as before — ZERO canonical writes.
            //   * customers/rooms only — single-PK mappers whose apply is safe
            //     to re-drive idempotently. bookings/checkins are multi-row
            //     aggregates, out of scope for #204.
            //   * row older than FORCE_CONVERGE_MIN_AGE_SECS so we don't race
            //     an in-flight CT event for a fresh divergence.
            //   * BOTH hashes present — a genuine value drift, not a
            //     missing_pg / missing_mssql case (those stay open for the
            //     normal paths / operator review).
            if force_converge_value_drift_eligible(
                &table_name,
                current_legacy_hash.as_deref(),
                current_pg_hash.as_deref(),
                age_secs,
                force_converge_enabled,
            ) {
                match force_converge_reconcile_row(
                    legacy_pool,
                    pg_pool,
                    &table_name,
                    &legacy_pk,
                    // The canonical row already exists — this is a value
                    // drift, not a miss.
                    ChangeOp::Update,
                )
                .await
                {
                    Ok(true) => {
                        // The mapper re-projected the current legacy row into
                        // canonical. The legacy hash is unchanged (we projected
                        // FROM it), so only the canonical hash can have moved —
                        // re-fetch it and re-test convergence.
                        let reprojected_pg_hash =
                            match compute_current_pg_hash(pg_pool, &table_name, &legacy_pk).await {
                                Ok(opt) => opt,
                                Err(e) => {
                                    tracing::warn!(
                                        site = %site_id,
                                        id,
                                        table_name = %table_name,
                                        legacy_pk = %legacy_pk,
                                        error = %e,
                                        "[Sync] Force-converge (#204): re-hash of canonical \
                                         failed after re-projection, leaving row open"
                                    );
                                    continue;
                                }
                            };
                        if should_auto_resolve(
                            &table_name,
                            current_legacy_hash.as_deref(),
                            reprojected_pg_hash.as_deref(),
                            recorded_mssql_hash.as_deref(),
                        ) {
                            tracing::info!(
                                site = %site_id,
                                id,
                                table_name = %table_name,
                                legacy_pk = %legacy_pk,
                                "[Sync] Force-converge (#204): re-projected current legacy \
                                 row into canonical; hashes now converge — marking resolved"
                            );
                            // Fall through (no `continue`) to the resolved UPDATE.
                        } else {
                            tracing::warn!(
                                site = %site_id,
                                id,
                                table_name = %table_name,
                                legacy_pk = %legacy_pk,
                                current_legacy_hash = ?current_legacy_hash,
                                reprojected_pg_hash = ?reprojected_pg_hash,
                                "[Sync] Force-converge (#204): canonical re-projected but \
                                 hashes still diverge — leaving row open for operator review"
                            );
                            continue;
                        }
                    }
                    Ok(false) => {
                        // Legacy row no longer exists (or unsupported table) —
                        // nothing to project from.
                        tracing::debug!(
                            site = %site_id,
                            id,
                            table_name = %table_name,
                            legacy_pk = %legacy_pk,
                            "[Sync] Force-converge (#204): skipped (legacy row absent), \
                             leaving row open"
                        );
                        continue;
                    }
                    Err(e) => {
                        tracing::warn!(
                            site = %site_id,
                            id,
                            table_name = %table_name,
                            legacy_pk = %legacy_pk,
                            error = %e,
                            "[Sync] Force-converge (#204): re-projection failed, leaving \
                             row open"
                        );
                        continue;
                    }
                }
            } else if reingest_missing_pg_eligible(
                &table_name,
                current_legacy_hash.as_deref(),
                current_pg_hash.as_deref(),
                age_secs,
                reingest_missing_pg_enabled,
            ) {
                // ---------------------------------------------------------------
                // `missing_pg` re-ingest arm (2026-07-27, ship dark behind
                // RECONCILE_REINGEST_MISSING_PG_ENABLED).
                //
                // Legacy still has the row, canonical never got it: a DROPPED
                // INGEST. CT's 2-day retention aged the event out, so the
                // watcher can never redeliver it and no observational sweep
                // can ever close the row — `should_auto_resolve` needs both
                // hashes and `(Some(legacy), None)` falls to `_ => false`.
                // Repair by re-running the NORMAL mapper for that key (the
                // same `apply` the CT watcher uses), then re-hash and only
                // close the row if the two sides actually converge.
                //
                // PG-write-only: legacy MSSQL is read, never written.
                // FK ordering is guaranteed by the sweep's candidate sort —
                // every `customers` row is healed before any `bookings` row
                // in the same pass.
                match force_converge_reconcile_row(
                    legacy_pool,
                    pg_pool,
                    &table_name,
                    &legacy_pk,
                    // No canonical row exists — this is an insert-if-absent.
                    ChangeOp::Insert,
                )
                .await
                {
                    Ok(true) => {
                        // Only the canonical side can have moved (we projected
                        // FROM the legacy row), so re-fetch it and re-test.
                        let reprojected_pg_hash =
                            match compute_current_pg_hash(pg_pool, &table_name, &legacy_pk).await {
                                Ok(opt) => opt,
                                Err(e) => {
                                    tracing::warn!(
                                        site = %site_id,
                                        id,
                                        table_name = %table_name,
                                        legacy_pk = %legacy_pk,
                                        error = %e,
                                        "[Sync] Re-ingest (missing_pg): re-hash of canonical \
                                         failed after re-ingest, leaving row open"
                                    );
                                    continue;
                                }
                            };
                        if should_auto_resolve(
                            &table_name,
                            current_legacy_hash.as_deref(),
                            reprojected_pg_hash.as_deref(),
                            recorded_mssql_hash.as_deref(),
                        ) {
                            tracing::info!(
                                site = %site_id,
                                id,
                                table_name = %table_name,
                                legacy_pk = %legacy_pk,
                                age_secs,
                                "[Sync] Re-ingest (missing_pg): re-ran the mapper for a \
                                 dropped ingest; canonical now present and hashes \
                                 converge — marking resolved"
                            );
                            // Fall through (no `continue`) to the resolved UPDATE.
                        } else {
                            tracing::warn!(
                                site = %site_id,
                                id,
                                table_name = %table_name,
                                legacy_pk = %legacy_pk,
                                current_legacy_hash = ?current_legacy_hash,
                                reprojected_pg_hash = ?reprojected_pg_hash,
                                "[Sync] Re-ingest (missing_pg): canonical re-ingested but \
                                 hashes still unconverged — leaving row open for operator \
                                 review"
                            );
                            continue;
                        }
                    }
                    Ok(false) => {
                        // The legacy row vanished between the hash probe and
                        // this re-fetch (or the aggregate header is gone).
                        // Distinct message on purpose — `/diagnose-alert`
                        // greps this to tell "legacy row genuinely absent"
                        // apart from "heal attempted and failed".
                        tracing::warn!(
                            site = %site_id,
                            id,
                            table_name = %table_name,
                            legacy_pk = %legacy_pk,
                            "[Sync] Re-ingest (missing_pg): legacy row absent at re-fetch \
                             — nothing to project from, leaving row open"
                        );
                        continue;
                    }
                    Err(e) => {
                        tracing::warn!(
                            site = %site_id,
                            id,
                            table_name = %table_name,
                            legacy_pk = %legacy_pk,
                            error = %e,
                            "[Sync] Re-ingest (missing_pg): re-ingest failed, leaving row \
                             open"
                        );
                        continue;
                    }
                }
            } else {
                // Observational-only behaviour (also the flag-OFF path).
                //
                // Per-row visibility into stuck rows (prod debug 2026-05-18):
                // operators need to see WHY each persistent reconcile_log
                // row isn't converging — (a) legacy hash missing, (b)
                // canonical hash missing, or (c) hashes computed but
                // genuinely don't match. Kept at debug level so it doesn't
                // flood at info; the same field-style as the converged-row
                // debug! below for grep symmetry.
                //
                // `outcome` classifies the three shapes explicitly because
                // `ht_reconcile_log` has no status column beyond
                // `resolved_at` — any outcome distinction has to live in
                // structured logs. `legacy_row_absent` in particular is the
                // genuine-anomaly case that no self-heal arm will ever touch.
                let outcome = match (current_legacy_hash.as_deref(), current_pg_hash.as_deref()) {
                    (None, _) => "legacy_row_absent",
                    (Some(_), None) => "canonical_row_absent",
                    _ => "hashes_unconverged",
                };
                tracing::debug!(
                    site = %site_id,
                    id,
                    table_name = %table_name,
                    legacy_pk = %legacy_pk,
                    outcome,
                    current_legacy_hash = ?current_legacy_hash,
                    current_pg_hash = ?current_pg_hash,
                    recorded_mssql_hash = ?recorded_mssql_hash,
                    "[Sync] Auto-resolve sweep: hashes did not converge, leaving row open"
                );
                continue;
            }
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
    sqlx::query_as::<
        _,
        (
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
        ),
    >(
        "SELECT cust_firstname, cust_type, cust_phone, cust_idcard, cust_address \
           FROM ht_customers \
          WHERE legacy_cust_no = $1 \
          LIMIT 1",
    )
    .bind(legacy_cust_no)
    .fetch_optional(pg_pool)
    .await
    .map(|opt| {
        opt.map(
            |(firstname, type_, phone, idcard, address)| CanonicalCustomerRow {
                cust_firstname: firstname,
                cust_type: type_,
                cust_phone: phone,
                cust_idcard: idcard,
                cust_address: address,
            },
        )
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
        let cust_no = row
            .get::<&str, _>("Cust_no")
            .unwrap_or_default()
            .to_string();
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
        mode,
        added,
        updated,
        unchanged,
        duration_ms
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
        let room_no = row
            .get::<&str, _>("Room_no")
            .unwrap_or_default()
            .to_string();
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
                        // `room_clean` is INVERTED between the two sides —
                        // legacy 'yes' means NEEDS cleaning, canonical `true`
                        // means IS clean. Commit 0303c98 added
                        // `clean_bool_to_legacy_yesno` for exactly this but
                        // only applied it to `compute_current_pg_hash`'s copy
                        // of this projection, leaving THIS one on the
                        // uninverted `bool_to_yesno`. The two disagreed, so
                        // every room with a non-NULL `Room_Clean` was detected
                        // as `value` drift and then immediately closed by the
                        // auto-resolve sweep (which uses the corrected
                        // inverse) — pure churn, and a row that misses the
                        // sweep's LIMIT 500 can age past the 4h level alert.
                        // Maintenance is NOT inverted and keeps `bool_to_yesno`.
                        clean_bool_to_legacy_yesno(c.room_clean),
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
        mode,
        added,
        updated,
        unchanged,
        duration_ms
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
        let book_no = row
            .get::<&str, _>("Book_No")
            .unwrap_or_default()
            .to_string();
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
        groups
            .entry((book_no, room_type_key))
            .or_default()
            .push(detail);
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
        let book_checkin_date =
            representative.and_then(|d| d.book_date_in.map(|dt| dt.date().to_string()));
        let book_checkout_date =
            representative.and_then(|d| d.book_date_out.map(|dt| dt.date().to_string()));
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
        mode,
        added,
        updated,
        unchanged,
        duration_ms
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
    sqlx::query_as::<
        _,
        (
            Option<chrono::NaiveDate>,
            Option<chrono::NaiveDate>,
            Option<String>,
        ),
    >(
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

/// Compute the MSSQL-side reconcile hash for a check-in aggregate via
/// the CT mapper's authoritative projection. Mirrors
/// [`compute_legacy_checkin_hash_via_mapper`] (auto-resolve sweep path)
/// so the bulk reconcile loop and the per-PK auto-resolver agree on
/// `(legacy_room_no, cin_checkin_time, effective_checkout_date,
/// legacy_cust_no)` to the byte.
///
/// **Why factored out:** the bulk sweep and the auto-resolver used to
/// project the legacy aggregate independently — the auto-resolver via
/// the mapper, the sweep via an in-Rust group/sort/representative pick
/// over a `HT_CheckIn_H ⋈ HT_CheckIn_Ds` join. The two pipelines
/// disagreed on multi-room folios (mapper picks the lowest-`Cin_Ds.id`
/// row per `derive_room_state`; the sweep picked the alphabetical-first
/// `Cin_Room_No`). Production audit 2026-05-18: 4 of 5 sampled
/// multi-room PKs picked different first rooms, producing ~545 spurious
/// `value` drifts on HF Hotel per tick. Unifying on the mapper here
/// makes "Bug B / C / D class" structurally impossible in the bulk
/// path too (cf. `compute_legacy_checkin_hash_via_mapper`'s doc).
fn checkin_hash_from_canonical(
    canonical: &crate::sync::mappers::checkin::CanonicalCheckIn,
) -> String {
    let effective_checkout: chrono::NaiveDate = canonical
        .cin_checkout_time
        .map(|dt: NaiveDateTime| dt.date())
        .unwrap_or(canonical.cin_expected_checkout);
    let effective_checkout_str = effective_checkout.to_string();
    checkin_canonical_hash(
        &canonical.legacy_cin_no,
        canonical.legacy_room_no.as_deref(),
        Some(canonical.cin_checkin_time.to_string()).as_deref(),
        Some(effective_checkout_str).as_deref(),
        canonical.legacy_cust_no.as_deref(),
        canonical.cin_checkout_time.is_some(),
        canonical.cin_status == "cancelled",
    )
}

/// Pure decision helper for the bulk reconcile loop: given the legacy
/// (MSSQL) hash freshly projected via the mapper, the canonical PG
/// hash, and the legacy/canonical row counts, classify the drift kind
/// (or return `None` if the two hashes match and there's nothing to
/// record).
///
/// Pulled out as a free function so the unit tests can pin the
/// "multi-room folio whose mapper-projected hash matches canonical
/// must NOT report value drift" contract without standing up a PG
/// pool or running the full async loop.
fn classify_checkin_drift(
    canonical_hash: Option<&str>,
    mssql_hash: &str,
    legacy_row_count: i32,
    pg_row_count: i32,
) -> Option<DivergenceKind> {
    if canonical_hash == Some(mssql_hash) && legacy_row_count == pg_row_count {
        return None;
    }
    Some(classify_divergence(
        canonical_hash,
        Some(mssql_hash),
        legacy_row_count,
        pg_row_count,
    ))
}

async fn sync_checkins(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    let mode = ReconcileMode::from_env();
    tracing::info!(?mode, "[Sync] Syncing check-ins...");

    // 1. Pull the PK list with a cheap single-column scan. The
    //    previous shape did one bulk JOIN that brought 41-45 rows back
    //    per folio (~800k rows on HF Hotel), then re-implemented the
    //    mapper's first-room projection in Rust. That parallel
    //    projection disagreed with the CT mapper on multi-room folios
    //    (the CT mapper picks the lowest-`Cin_Ds.id` row via
    //    `derive_room_state`; the bulk loop picked the alphabetical-
    //    first `Cin_Room_No`), producing ~545 spurious `value` drift
    //    rows per tick on HF Hotel (2026-05-18 audit). Unifying on the
    //    mapper eliminates the bug class — see
    //    `compute_legacy_checkin_hash_via_mapper`'s doc comment.
    // The EXISTS filter restores the row-set semantics of the pre-#134
    // bulk JOIN query (`FROM HT_CheckIn_Ds INNER JOIN HT_CheckIn_H`),
    // which implicitly excluded Ds-less header rows. 2026-05-18 prod
    // audit found 201 such rows on HF Hotel — iHOTEL "ghosts" from
    // failed walk-ins / cancelled-before-room-assignment / test data
    // that aren't representable in canonical (`ht_checkins.cin_room_id`
    // is NOT NULL). Without this filter the reconcile sweep flags every
    // ghost as `missing_pg`, producing 200+ false-positive drift rows
    // per tick.
    let mut conn = legacy_pool.get().await?;
    let pk_rows = conn
        .simple_query(
            "SELECT DISTINCT h.Cin_no FROM HT_CheckIn_H h \
              WHERE EXISTS ( \
                  SELECT 1 FROM HT_CheckIn_Ds d WHERE d.Cin_No = h.Cin_no \
              ) \
              ORDER BY h.Cin_no",
        )
        .await?
        .into_first_result()
        .await?;
    let cin_nos: Vec<String> = pk_rows
        .iter()
        .filter_map(|r| r.get::<&str, _>("Cin_no").map(String::from))
        .collect();
    // Free the pool slot before per-PK loads grab their own connection.
    drop(conn);

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;
    let mut load_errors = 0i32;
    let mut project_errors = 0i32;

    // 2. For each PK: load the legacy aggregate (header + Ds + Pay)
    //    and run it through the CT mapper's projection function. The
    //    resulting `CanonicalCheckIn` is what canonical PG SHOULD look
    //    like for this PK, so a hash mismatch with the actual canonical
    //    row can only signal real exogenous drift (CT miss, hand-edited
    //    PG) — never parallel-projection-pipeline divergence.
    //
    //    Cost: ~3 MSSQL queries per PK × 19k PKs (HF Hotel) ≈ 57k
    //    queries per tick, vs 1 bulk query previously. Still well
    //    within the 15-min cron cadence (steady-state ~63 q/s, well
    //    below same-LAN saturation).
    for cin_no in &cin_nos {
        let aggregate =
            match crate::sync::parent_loader::load_checkin_aggregate(legacy_pool, cin_no).await {
                Ok(a) => a,
                Err(e) => {
                    load_errors += 1;
                    tracing::warn!(
                        cin_no = %cin_no,
                        error = %e,
                        "[Sync] sync_checkins: load_checkin_aggregate failed; skipping PK"
                    );
                    continue;
                }
            };
        // Cancelled / deleted folio — the CT watcher emits
        // `CheckInCancelled` on its own; nothing for the sweep to do.
        if !aggregate.is_present() {
            continue;
        }
        let canonical_proj =
            match crate::sync::mappers::project_checkin_aggregate(&aggregate, cin_no) {
                Ok(p) => p,
                Err(e) => {
                    project_errors += 1;
                    tracing::warn!(
                        cin_no = %cin_no,
                        error = %e,
                        "[Sync] sync_checkins: project_checkin_aggregate failed; skipping PK"
                    );
                    continue;
                }
            };

        // Hash inputs are byte-identical to
        // `compute_legacy_checkin_hash_via_mapper` (auto-resolve sweep)
        // so the two paths produce the same string for the same
        // aggregate. The ack-cache (`ht_checkins_legacy.sync_hash`)
        // was stamped under the prior per-row projection, but the
        // chosen first-room row is determined by the mapper's
        // `derive_room_state` (lowest `Cin_Ds.id`) — which is also
        // what canonical-side `ht_checkins.legacy_room_no` holds. So
        // for any folio where canonical is correct, the new hash
        // equals what canonical hashes to today, and the ack cache
        // converges on the next tick.
        let mssql_hash = checkin_hash_from_canonical(&canonical_proj);

        match mode {
            ReconcileMode::Upsert => {
                upsert_checkin_mirror(
                    pg_pool,
                    cin_no,
                    &canonical_proj,
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
                .bind(cin_no)
                .fetch_optional(pg_pool)
                .await?;

                if matches!(&last_ack, Some(Some(prev)) if *prev == mssql_hash) {
                    unchanged += 1;
                    continue;
                }

                let canonical = fetch_canonical_checkin(pg_pool, cin_no).await?;
                let canonical_hash = canonical.as_ref().map(|c| {
                    let checkin_str = c.cin_checkin_time.map(|t| t.to_string());
                    let effective_checkout_str = c.effective_checkout_date().map(|d| d.to_string());
                    checkin_canonical_hash(
                        cin_no,
                        c.legacy_room_no.as_deref(),
                        checkin_str.as_deref(),
                        effective_checkout_str.as_deref(),
                        c.legacy_cust_no.as_deref(),
                        c.is_checked_out(),
                        c.is_cancelled(),
                    )
                });

                // Track D / T7 CRIT-1 / Track B last-mile (2026-05-18):
                // `pg_row_count` reads the ACTUAL per-room junction
                // count (`ht_checkin_rooms`). On a query failure fall
                // back to `1` (pre-Track-B observation) rather than
                // `0` — degraded observability beats spurious
                // `missing_pg` misclassification — and warn-log.
                //
                // 2026-05-19 dedup fix: count DISTINCT `Cin_Room_No`
                // values instead of raw `aggregate.rooms.len()`.
                // iHOTEL routinely writes >1 HT_CheckIn_Ds row per
                // room (extends / re-keys / deposit returns) — see
                // CH22-000722 trigger documented on
                // [`count_distinct_legacy_checkin_rooms`]. The
                // `ht_checkin_rooms` junction enforces
                // `UNIQUE (cr_cin_id, cr_room_id)`, so "distinct
                // rooms" IS the canonical truth that `pg_row_count`
                // already reflects.
                let legacy_row_count: i32 = count_distinct_legacy_checkin_rooms(&aggregate.rooms);
                let pg_row_count: i32 = if canonical.is_some() {
                    match count_canonical_checkin_rooms(pg_pool, cin_no).await {
                        Ok(n) => n,
                        Err(e) => {
                            tracing::warn!(
                                cin_no = %cin_no,
                                error = %e,
                                "[Sync] count_canonical_checkin_rooms failed; \
                                 falling back to pre-Track-B pg_row_count=1 \
                                 to avoid spurious missing_pg classification"
                            );
                            1
                        }
                    }
                } else {
                    0
                };

                let drift_kind = classify_checkin_drift(
                    canonical_hash.as_deref(),
                    &mssql_hash,
                    legacy_row_count,
                    pg_row_count,
                );

                let Some(kind) = drift_kind else {
                    // Hashes match AND row counts agree — converged.
                    // Ack the cache so the next tick short-circuits at
                    // the `last_ack` compare above.
                    ack_checkin_mirror(pg_pool, cin_no, &mssql_hash).await;
                    unchanged += 1;
                    continue;
                };

                let mssql_json = checkin_aggregate_json(cin_no, &aggregate);
                let pg_json = canonical.as_ref().map(|c| {
                    json!({
                        "legacy_room_no": c.legacy_room_no,
                        "cin_checkin_time": c.cin_checkin_time.map(|t| t.to_string()),
                        // Bug D fix (2026-05-16) — surface BOTH the actual
                        // and expected checkout dates for operator triage.
                        // Hash uses `effective_checkout_date()` (actual if
                        // set, else expected); JSON exposes both so the
                        // operator can see which one matched / mismatched
                        // the legacy `Cin_Room_Out` shown in mssql_row_json.
                        "cin_checkout_time": c.cin_checkout_time.map(|t| t.to_string()),
                        "cin_expected_checkout": c.cin_expected_checkout.map(|d| d.to_string()),
                        "legacy_cust_no": c.legacy_cust_no,
                    })
                });
                record_divergence(
                    pg_pool,
                    "checkins",
                    cin_no,
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
                    ack_checkin_mirror(pg_pool, cin_no, &mssql_hash).await;
                }
                if canonical.is_none() {
                    added += 1;
                } else {
                    updated += 1;
                }
            }
        }
    }

    if load_errors > 0 || project_errors > 0 {
        tracing::warn!(
            load_errors,
            project_errors,
            "[Sync] sync_checkins: {} aggregate loads / {} projections failed (per-PK skipped, see warn logs)",
            load_errors,
            project_errors,
        );
    }

    let duration_ms = start.elapsed().as_millis() as i32;
    tracing::info!(
        "[Sync] Check-ins ({:?}): {} added, {} updated, {} unchanged in {}ms",
        mode,
        added,
        updated,
        unchanged,
        duration_ms
    );
    record_success(pg_pool, "checkins", added, updated, unchanged, duration_ms).await;

    Ok(())
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
    /// Header-derived aggregate status. PG enum literals
    /// (`'active'` / `'checkedout'` / `'cancelled'`) — see
    /// `crate::sync::mappers::checkin::legacy_status_to_pg`.
    ///
    /// Used as the gate for the `cancelled` sentinel branch of
    /// `checkin_canonical_hash` so the legacy and canonical hashes
    /// converge on cancelled folios (whose per-room `HT_CheckIn_Ds`
    /// rows iHOTEL deletes — see the function-level docs on
    /// `checkin_canonical_hash` for the six stuck CH26-* PKs that
    /// motivated this).
    cin_status: Option<String>,
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

    /// `true` when the header-derived aggregate status is `'cancelled'`
    /// — gate for the `checkin_canonical_hash` cancelled sentinel.
    /// Treats NULL/missing as not cancelled (the active-stay 5-field
    /// hash path is strictly safer than the sentinel path: it
    /// preserves all the existing field-by-field drift detection).
    fn is_cancelled(&self) -> bool {
        self.cin_status.as_deref() == Some("cancelled")
    }

    /// `true` when an ACTUAL checkout has been recorded (`cin_checkout_time`
    /// is set) — the `checked_out` bit hashed by [`checkin_canonical_hash`].
    /// Mirrors the legacy side's `cin_checkout_time.is_some()` so a dropped
    /// checkout (canonical still active, legacy checked-out) diverges even
    /// when the actual/expected dates coincide (task #68).
    fn is_checked_out(&self) -> bool {
        self.cin_checkout_time.is_some()
    }
}

async fn fetch_canonical_checkin(
    pg_pool: &PgPool,
    legacy_cin_no: &str,
) -> Result<Option<CanonicalCheckinRow>, sqlx::Error> {
    sqlx::query_as::<
        _,
        (
            Option<String>,
            Option<NaiveDateTime>,
            Option<chrono::NaiveDate>,
            Option<NaiveDateTime>,
            Option<String>,
            Option<String>,
        ),
    >(
        "SELECT legacy_room_no, cin_checkin_time, cin_expected_checkout, \
                cin_checkout_time, legacy_cust_no, cin_status \
           FROM ht_checkins \
          WHERE legacy_cin_no = $1 \
          LIMIT 1",
    )
    .bind(legacy_cin_no)
    .fetch_optional(pg_pool)
    .await
    .map(|opt| {
        opt.map(
            |(room, checkin, expected_checkout, actual_checkout, cust, status)| {
                CanonicalCheckinRow {
                    legacy_room_no: room,
                    cin_checkin_time: checkin,
                    cin_expected_checkout: expected_checkout,
                    cin_checkout_time: actual_checkout,
                    legacy_cust_no: cust,
                    cin_status: status,
                }
            },
        )
    })
}

/// Count canonical check-in room rows for a given `legacy_cin_no`,
/// joining `ht_checkin_rooms` (the Track B junction table created in
/// migration `043_create_ht_checkin_rooms.sql`) against `ht_checkins`.
///
/// **Why this exists:** the reconcile sweep historically hardcoded
/// `pg_row_count = 1` whenever `ht_checkins` had a row (since canonical
/// only stored the first room denormalised). After Track B landed the
/// per-room junction, the CT mapper (`mappers/checkin.rs`) and the
/// `backfill_checkin_rooms` bin populate `ht_checkin_rooms` with one
/// row per booked room. This helper reads that actual count so
/// `classify_divergence` can compare like-for-like with
/// `View_CheckIn_Ds.details.len()`, collapsing the 756 multi-room
/// cardinality drifts observed on HF Hotel on 2026-05-18 from
/// "always Cardinality" to "Value (matching) ⇒ silent ack".
///
/// Follow-up (deferred): the canonical-side hash still hashes only the
/// FIRST room (see `checkin_canonical_hash`). Once row counts match,
/// drift on a SECONDARY room would still be invisible to the hash. A
/// future PR that rotates the canonical hash format to hash ALL
/// junction rows (mirroring `aggregate_booking_hash`'s shape) would
/// close that blind spot, but is out of scope here because rotating
/// the hash
/// invalidates every `ht_checkins_legacy.sync_hash` entry and would
/// trigger a deploy-time alert flood (cf. 2026-05-15).
async fn count_canonical_checkin_rooms(
    pg_pool: &PgPool,
    legacy_cin_no: &str,
) -> Result<i32, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        "SELECT count(*) FROM ht_checkin_rooms cr \
           JOIN ht_checkins c ON c.cin_id = cr.cr_cin_id \
          WHERE c.legacy_cin_no = $1",
    )
    .bind(legacy_cin_no)
    .fetch_one(pg_pool)
    .await
    .map(|n| n as i32)
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

/// Pre-Phase-5.5 escape-hatch path: UPSERT the `ht_checkins_legacy`
/// mirror's data columns from the CT mapper's canonical projection.
/// Not exercised in production after v2.63.0 — preserved for forensic
/// rollback flexibility only.
///
/// Field mapping mirrors the prior per-row `CheckinDetail`-backed
/// shape so the mirror schema is unchanged:
/// * `cin_room_no`  ← `canonical.legacy_room_no` (mapper's first-room
///                     denormalisation per `derive_room_state`)
/// * `cin_room_in`  ← `canonical.cin_checkin_time` (header `Cin_Date_in`)
/// * `cin_room_out` ← `canonical.cin_checkout_time` when set,
///                     else midnight of `cin_expected_checkout`
///                     (preserves the `NaiveDateTime` column type — the
///                     mapper's date-only `cin_expected_checkout`
///                     would require a column-type change otherwise).
/// * `cin_cust_name` ← `NULL` (this column was always populated from
///                     a view-derived alias the parent loader does
///                     not project; left NULL pre-2026-05-18 too).
/// * `cin_cust_no`  ← `canonical.legacy_cust_no`
/// * `cin_status`   ← `canonical.cin_status` (mapper's translated
///                     literal — `active` / `checkedout` / `cancelled`).
#[allow(clippy::too_many_arguments)]
async fn upsert_checkin_mirror(
    pg_pool: &PgPool,
    cin_no: &str,
    canonical: &crate::sync::mappers::checkin::CanonicalCheckIn,
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

    let room_in: Option<NaiveDateTime> = Some(canonical.cin_checkin_time);
    let room_out: Option<NaiveDateTime> = canonical.cin_checkout_time.or_else(|| {
        chrono::NaiveTime::from_hms_opt(0, 0, 0)
            .map(|t| canonical.cin_expected_checkout.and_time(t))
    });
    let cust_name: Option<&str> = None;

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
            .bind(&canonical.legacy_room_no)
            .bind(room_in)
            .bind(room_out)
            .bind(cust_name)
            .bind(&canonical.legacy_cust_no)
            .bind(&canonical.cin_status)
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
            .bind(&canonical.legacy_room_no)
            .bind(room_in)
            .bind(room_out)
            .bind(cust_name)
            .bind(&canonical.legacy_cust_no)
            .bind(&canonical.cin_status)
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
            vec![
                ("bookings".to_string(), 5_000),
                ("customers".to_string(), 51)
            ]
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
        let v = with_threshold_env(Some("not-a-number"), || {
            drift_alert_threshold_from_env("hfhotel")
        });
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
        assert_eq!(
            v, 80,
            "invalid per-site value must fall through to the global"
        );
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

    // -------------------------------------------------------------------
    // sync_checkins mapper-unification (2026-05-18)
    // -------------------------------------------------------------------
    //
    // The pre-2026-05-18 bulk sweep grouped `HT_CheckIn_H ⋈ HT_CheckIn_Ds`
    // rows in Rust and picked the alphabetical-first `Cin_Room_No` as
    // the representative. The CT mapper picks the lowest-`Cin_Ds.id`
    // row (via `derive_room_state`). On multi-room folios where those
    // pick different rooms, every reconcile tick inserted a spurious
    // `value` drift. The pinned tests below cover the unified shape:
    // `classify_checkin_drift` returns `None` when the mapper-projected
    // legacy hash matches canonical, regardless of room ordering.

    /// Build the hash inputs a single-folio check-in produces when its
    /// canonical PG projection picks `legacy_room_no` as the
    /// representative room. Mirrors the byte-shape of
    /// [`checkin_hash_from_canonical`] for the given inputs without
    /// requiring a full `CanonicalCheckIn` construction (which has
    /// private fields the mapper guards). Hash inputs are stable
    /// across both the bulk sweep and the auto-resolve sweep — what
    /// this test fixture pins is the cross-pipeline equality, not a
    /// new format.
    fn projected_hash(cin_no: &str, legacy_room_no: &str, cust_no: &str) -> String {
        let checkin_dt = NaiveDate::from_ymd_opt(2026, 4, 1)
            .unwrap()
            .and_hms_opt(14, 0, 0)
            .unwrap();
        let expected_out = NaiveDate::from_ymd_opt(2026, 4, 3).unwrap();
        checkin_canonical_hash(
            cin_no,
            Some(legacy_room_no),
            Some(checkin_dt.to_string()).as_deref(),
            Some(expected_out.to_string()).as_deref(),
            Some(cust_no),
            false,
            false,
        )
    }

    /// Regression guard for the headline bug class
    /// (`docs/coexistence/data-hygiene-ch25-002081.md`): a multi-room
    /// folio whose canonical `legacy_room_no` matches the mapper's
    /// `derive_room_state` first-room pick MUST classify as "no drift".
    /// Pre-fix the bulk sweep re-implemented the projection in Rust
    /// and picked the alphabetical-first `Cin_Room_No` instead of the
    /// lowest-`Cin_Ds.id` row the CT mapper picks, disagreeing with
    /// canonical on every multi-room folio whose lowest-id row wasn't
    /// also alphabetically first — ~545 spurious `value` drift rows
    /// per tick on HF Hotel (2026-05-18 audit).
    ///
    /// Post-fix: both the bulk sweep and canonical resolve via the
    /// same `project_checkin_aggregate`, so the representative pick
    /// agrees by construction and no drift fires.
    #[test]
    fn multi_room_pk_with_matching_first_room_reports_no_drift() {
        // Both sides project the same first room (the mapper's pick).
        let mapper_hash = projected_hash("CIN-1", "203", "C001");
        let canonical_hash = projected_hash("CIN-1", "203", "C001");

        let drift = classify_checkin_drift(
            Some(&canonical_hash),
            &mapper_hash,
            3, // legacy rooms
            3, // pg junction rooms
        );

        assert!(
            drift.is_none(),
            "mapper-projected hash for the first room MUST equal canonical's \
             stored first room (both are the lowest-Cin_Ds.id row); the \
             bulk sweep no longer disagrees on which room is representative — \
             pre-fix this returned DivergenceKind::Value on every multi-room \
             folio where alphabetical-first ≠ lowest-id"
        );
    }

    /// The exact bug class we eliminated: pre-fix the bulk sweep
    /// picked the alphabetical-first room ("101") while canonical PG
    /// stored "203" (the mapper's lowest-id pick). The hash mismatch
    /// re-fired `DivergenceKind::Value` on every tick. This test
    /// pins the pre-fix behaviour AS A REGRESSION SHAPE and asserts
    /// the post-fix shape (both sides project via the mapper)
    /// converges.
    #[test]
    fn pre_fix_alphabetical_vs_lowest_id_pick_diverges_but_unified_converges() {
        let alpha_first_hash = projected_hash("CIN-1", "101", "C001");
        let mapper_pick_hash = projected_hash("CIN-1", "203", "C001");

        // Pre-fix shape: sweep hashed "101", canonical stored "203" →
        // spurious value drift every tick.
        let pre_fix_drift =
            classify_checkin_drift(Some(&mapper_pick_hash), &alpha_first_hash, 3, 3);
        assert_eq!(
            pre_fix_drift,
            Some(DivergenceKind::Value),
            "pre-fix sweep picked '101' alphabetically, canonical PG stored \
             '203' from the mapper — this hash mismatch was the spurious \
             drift signal the unification eliminates"
        );

        // Post-fix shape: both sides project via the mapper → same
        // first room → no drift.
        let post_fix_drift =
            classify_checkin_drift(Some(&mapper_pick_hash), &mapper_pick_hash, 3, 3);
        assert!(
            post_fix_drift.is_none(),
            "unifying the sweep on the CT mapper eliminates the parallel- \
             projection mismatch by construction"
        );
    }

    /// Negative guard: real value drift (e.g. CT-watcher missed the
    /// update, canonical stuck on an old cust_no) must still classify
    /// as `Value`. The fix narrows false positives, not signal.
    #[test]
    fn genuine_value_drift_still_classifies_as_value() {
        // Mapper sees new cust_no in legacy; canonical PG is stale.
        let mapper_hash = projected_hash("CIN-1", "203", "C001-NEW");
        let canonical_stale_hash = projected_hash("CIN-1", "203", "C001-OLD");

        let drift = classify_checkin_drift(Some(&canonical_stale_hash), &mapper_hash, 3, 3);

        assert_eq!(
            drift,
            Some(DivergenceKind::Value),
            "hash mismatch with matching cardinality must remain Value drift"
        );
    }

    /// Cardinality drift (junction row count short of legacy) MUST
    /// keep classifying as `Cardinality` regardless of hash match.
    #[test]
    fn cardinality_drift_classifies_even_when_hashes_match() {
        let hash = projected_hash("CIN-1", "203", "C001");

        let drift = classify_checkin_drift(
            Some(&hash),
            &hash,
            3, // legacy rooms
            2, // pg junction rooms short
        );

        assert_eq!(
            drift,
            Some(DivergenceKind::Cardinality),
            "row-count asymmetry must surface as Cardinality drift, \
             even when first-room hashes converge"
        );
    }

    /// `missing_pg` drift (canonical has nothing for this PK) must
    /// surface even when the mapper produces a valid hash.
    #[test]
    fn missing_canonical_classifies_as_missing_pg() {
        let mapper_hash = projected_hash("CIN-1", "203", "C001");

        let drift = classify_checkin_drift(
            None, // canonical absent
            &mapper_hash,
            3, // legacy rooms
            0, // no canonical rooms
        );

        assert_eq!(
            drift,
            Some(DivergenceKind::MissingPg),
            "canonical-absent must remain a MissingPg signal — the highest- \
             priority drift kind operators page on"
        );
    }

    // -------------------------------------------------------------------
    // Track B last-mile (2026-05-18) — junction-aware pg_row_count
    // -------------------------------------------------------------------
    //
    // These exercise the `classify_divergence` decision the reconcile
    // loop now feeds with the actual `ht_checkin_rooms` row count
    // (instead of the pre-Track-B hardcoded 1). The wire-up itself is
    // an `await` in the loop body — the testable surface is the pure
    // classifier and its truth-table response to the new inputs.

    /// Regression guard: post-Track-B, a multi-room folio whose
    /// junction-room count matches legacy `View_CheckIn_Ds.details.len()`
    /// must classify as `Value` (and become silenceable via
    /// `is_silenceable()` once hashes align too). Pre-fix this case
    /// returned `Cardinality` for every multi-room folio because
    /// `pg_row_count` was hardcoded to 1.
    #[test]
    fn cardinality_kind_when_junction_count_matches_legacy_returns_value_not_cardinality() {
        let kind = classify_divergence(Some("pg-hash"), Some("mssql-hash"), 3, 3);
        assert_eq!(
            kind,
            DivergenceKind::Value,
            "matching junction count must drop the kind to Value so the \
             ack-cache can silence it once the first-room hashes align"
        );
    }

    /// Regression guard against an over-eager fix: if the junction
    /// has FEWER rows than legacy (e.g. backfill skipped an inactive
    /// folio), we must still classify as `Cardinality` so the
    /// operator-visible signal is preserved.
    #[test]
    fn cardinality_kind_when_junction_count_differs_returns_cardinality() {
        let kind = classify_divergence(Some("pg-hash"), Some("mssql-hash"), 3, 2);
        assert_eq!(
            kind,
            DivergenceKind::Cardinality,
            "junction count short of legacy must remain Cardinality so \
             the gap surfaces in ht_reconcile_log instead of being silenced"
        );
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
        let mssql =
            customer_canonical_hash("C001", "Somchai", None, Some("0812345678"), None, None);
        // CT mapper stored an older phone (drift the operator should fix).
        let canonical =
            customer_canonical_hash("C001", "Somchai", None, Some("0899999999"), None, None);
        assert_ne!(mssql, canonical);
    }

    #[test]
    fn customer_canonical_hash_treats_none_as_empty() {
        // `Cust_Type = NULL` on the MSSQL side hashes identically to
        // `cust_type = NULL` on the canonical side — both project to
        // empty string before hashing.
        let h1 = customer_canonical_hash("C001", "Anan", None, None, None, None);
        let h2 = customer_canonical_hash("C001", "Anan", Some(""), Some(""), Some(""), Some(""));
        assert_eq!(
            h1, h2,
            "None and empty-string must canonicalise the same way"
        );
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

    // sync_checkins no longer maintains its own MSSQL projection — it
    // delegates per-PK to `parent_loader::load_checkin_aggregate` +
    // `mappers::project_checkin_aggregate` (the CT mapper's own
    // pipeline). The Track J1 projection-lock test for those columns
    // lives at `sync::parent_loader::tests::checkin_h_projection_*`
    // and `sync::mappers::checkin::tests::checkin_ds_projection_*`
    // respectively. Nothing in this module owns a parallel projection
    // any more, so the local lock test is retired.

    #[test]
    fn bool_to_yesno_round_trip_via_legacy_yesno_canonical() {
        // The two halves of the room-status translation must be each
        // other's inverse for the canonical-hash to align with the
        // MSSQL projection.
        assert_eq!(
            legacy_yesno_canonical(Some("yes")),
            bool_to_yesno(Some(true))
        );
        assert_eq!(
            legacy_yesno_canonical(Some("no")),
            bool_to_yesno(Some(false))
        );
        // NULL → "" on both sides, matching how nullable BOOLEAN
        // columns canonicalise.
        assert_eq!(legacy_yesno_canonical(None), bool_to_yesno(None));
        // Unknown legacy literals fall back to "" — matches the CT
        // mapper's behaviour (`legacy_yesno_to_bool` returns None for
        // anything other than yes/no).
        assert_eq!(legacy_yesno_canonical(Some("maybe")), "");
    }

    /// Regression: the DETECTION projection (`sync_rooms`) and the
    /// AUTO-RESOLVE projection (`compute_current_pg_hash`) must agree on
    /// `room_clean`, which is INVERTED between the two sides — legacy 'yes'
    /// means NEEDS cleaning, canonical `true` means IS clean.
    ///
    /// Commit 0303c98 introduced `clean_bool_to_legacy_yesno` for this but
    /// applied it to only one of the two call sites. The result was a false
    /// `value` divergence recorded for every room with a non-NULL
    /// `Room_Clean`, immediately closed again by the sweep — churn that can
    /// age past the 4h level alert if a row misses the sweep's LIMIT 500.
    ///
    /// The `bool_to_yesno` assertion at the bottom is the important half: it
    /// fails if anyone "simplifies" the inverted helper back to the plain one.
    #[test]
    fn room_clean_projections_agree_across_detection_and_auto_resolve() {
        // A dirty room: legacy says "needs cleaning", canonical says not clean.
        let legacy_dirty = room_canonical_hash("512", legacy_yesno_canonical(Some("yes")), "no", None);
        let canonical_dirty =
            room_canonical_hash("512", clean_bool_to_legacy_yesno(Some(false)), "no", None);
        assert_eq!(
            legacy_dirty, canonical_dirty,
            "canonical room_clean=false MUST hash like legacy Room_Clean='yes' (needs cleaning)"
        );

        // A clean room: legacy "no cleaning needed", canonical clean.
        let legacy_clean = room_canonical_hash("512", legacy_yesno_canonical(Some("no")), "no", None);
        let canonical_clean =
            room_canonical_hash("512", clean_bool_to_legacy_yesno(Some(true)), "no", None);
        assert_eq!(
            legacy_clean, canonical_clean,
            "canonical room_clean=true MUST hash like legacy Room_Clean='no'"
        );

        // And the uninverted helper must NOT agree — this is what the bug was.
        let canonical_dirty_uninverted =
            room_canonical_hash("512", bool_to_yesno(Some(false)), "no", None);
        assert_ne!(
            legacy_dirty, canonical_dirty_uninverted,
            "bool_to_yesno on room_clean reintroduces the 0303c98 half-fix"
        );
    }

    #[test]
    fn room_canonical_hash_matches_when_canonical_mirrors_legacy() {
        // room_clean is INVERTED: legacy "no" = no cleaning needed = canonical
        // true (IS clean). room_maintenance is NOT inverted. Pairing canonical
        // `true` with legacy "yes" — as this test did before 20edf18 — encodes
        // the very polarity bug that fix removed.
        let mssql = room_canonical_hash("101", "no", "no", Some("ocean view"));
        let canonical = room_canonical_hash(
            "101",
            clean_bool_to_legacy_yesno(Some(true)),
            bool_to_yesno(Some(false)),
            Some("ocean view"),
        );
        assert_eq!(mssql, canonical);
    }

    #[test]
    fn room_canonical_hash_diverges_when_canonical_clean_lags_behind() {
        // Legacy says the room NEEDS cleaning ("yes" = dirty) but the CT
        // mapper hasn't yet flipped canonical.room_clean to false, so
        // canonical still claims clean → divergence fires.
        let mssql = room_canonical_hash("101", "yes", "no", None);
        let canonical = room_canonical_hash(
            "101",
            clean_bool_to_legacy_yesno(Some(true)),
            bool_to_yesno(Some(false)),
            None,
        );
        assert_ne!(mssql, canonical);
    }

    // -------------------------------------------------------------------
    // PR C (2026-05-19) — auto-resolve sweep wires rooms into both
    // dispatch tables (`compute_current_pg_hash` and
    // `compute_current_legacy_hash`). The dispatch arms are 1-liners
    // that delegate to `fetch_canonical_room → room_canonical_hash`
    // and `fetch_legacy_room_hash` respectively; the substantive
    // behaviour lives in those helpers. The composition tests below
    // pin the hash-input contract that both arms construct, mirroring
    // the customer/booking pattern (the live A2-1 evidence of
    // 2026-05-18 was `current_legacy_hash=None current_pg_hash=None`
    // — the `_ => Ok(None)` arm firing on both sides).
    // -------------------------------------------------------------------

    /// Pinpoints the canonical-side arm's composition: a `CanonicalRoomRow`
    /// projects through `bool_to_yesno + room_canonical_hash` into
    /// exactly the same byte string the legacy-side projection emits
    /// for the converged equivalent. When the dispatch arm runs against
    /// a real PG row, this is the hash the sweep compares against
    /// `fetch_legacy_room_hash`'s output.
    #[test]
    fn rooms_pg_dispatch_composition_converges_with_legacy_projection() {
        // A canonical row the CT mapper would have written after the
        // legacy state converged: legacy Room_Clean="no" (no cleaning
        // needed) → canonical room_clean=true; maintenance "no" → false.
        let canonical_row = CanonicalRoomRow {
            room_clean: Some(true),
            room_maintenance: Some(false),
            room_notes: None,
        };
        let pg_hash = room_canonical_hash(
            "A2-1",
            clean_bool_to_legacy_yesno(canonical_row.room_clean),
            bool_to_yesno(canonical_row.room_maintenance),
            canonical_row.room_notes.as_deref(),
        );

        // The legacy side runs `legacy_yesno_canonical` over the raw
        // MSSQL literal ("yes"/"no"/NULL). After convergence those
        // collapse to the same `'yes' | 'no' | ""` tokens used above.
        let legacy_hash = room_canonical_hash(
            "A2-1",
            legacy_yesno_canonical(Some("no")),
            legacy_yesno_canonical(Some("no")),
            None,
        );

        assert_eq!(
            pg_hash, legacy_hash,
            "auto-resolve sweep relies on both arms producing byte-identical \
             hashes when the underlying state has converged"
        );
    }

    /// Round-trips every legal `(canonical bool, legacy literal)` pair for
    /// the NON-inverted fields. Lock test for `bool_to_yesno` ↔
    /// `legacy_yesno_canonical`.
    ///
    /// This pairing is correct for `room_maintenance` and WRONG for
    /// `room_clean`, which is inverted (legacy "yes" = needs cleaning =
    /// canonical false) and uses `clean_bool_to_legacy_yesno` — see
    /// `room_clean_projections_agree_across_detection_and_auto_resolve`.
    /// Do not cite this test as licence to use `bool_to_yesno` on clean.
    #[test]
    fn rooms_dispatch_yesno_round_trip_is_total() {
        for (canonical, legacy_literal) in [
            (Some(true), Some("yes")),
            (Some(false), Some("no")),
            (None, None),
        ] {
            assert_eq!(
                bool_to_yesno(canonical),
                legacy_yesno_canonical(legacy_literal),
                "rooms dispatch loses parity when canonical {:?} ↔ legacy {:?}",
                canonical,
                legacy_literal,
            );
        }
    }

    /// Operator-edited the notes in legacy but the CT mapper hasn't
    /// caught up → both dispatch arms produce different hashes and the
    /// reconcile_log row stays open. Mirrors the
    /// `room_canonical_hash_diverges_when_canonical_clean_lags_behind`
    /// pattern for the notes field, which is the third hash input.
    #[test]
    fn rooms_pg_dispatch_composition_diverges_when_canonical_notes_lag() {
        let canonical_row = CanonicalRoomRow {
            room_clean: Some(true),
            room_maintenance: Some(false),
            room_notes: Some("old note".to_string()),
        };
        let pg_hash = room_canonical_hash(
            "A2-1",
            clean_bool_to_legacy_yesno(canonical_row.room_clean),
            bool_to_yesno(canonical_row.room_maintenance),
            canonical_row.room_notes.as_deref(),
        );
        let legacy_hash = room_canonical_hash(
            "A2-1",
            legacy_yesno_canonical(Some("yes")),
            legacy_yesno_canonical(Some("no")),
            Some("new note"),
        );
        assert_ne!(
            pg_hash, legacy_hash,
            "post-detection sweep must NOT auto-resolve while canonical \
             notes still trail the legacy state"
        );
    }

    /// `fetch_legacy_room_hash` returns `Ok(None)` when the legacy
    /// row has been deleted since the drift was detected. This pure
    /// test pins the convention used by every `fetch_legacy_*_hash`
    /// helper: a missing row is "still drifted — leave for operator
    /// review" rather than "converged to absent". For non-booking
    /// entities the dispatch arm must NOT auto-resolve a vanished
    /// legacy key — even when canonical matches the recorded legacy
    /// state — because a deleted room/customer/checkin is a genuine
    /// anomaly. The stale-ghost arm is bookings-only.
    #[test]
    fn rooms_dispatch_missing_legacy_row_does_not_auto_resolve() {
        // Simulate `fetch_legacy_room_hash` returning Ok(None) for a
        // legacy-deleted row, alongside a still-present canonical hash
        // that EQUALS the recorded legacy hash. For bookings this would
        // trip the stale-ghost arm; for rooms it must stay open.
        let recorded = room_canonical_hash("A2-1", "yes", "no", None);
        let legacy_hash: Option<String> = None;
        let pg_hash: Option<String> = Some(recorded.clone());

        assert!(
            !should_auto_resolve(
                "rooms",
                legacy_hash.as_deref(),
                pg_hash.as_deref(),
                Some(recorded.as_str()),
            ),
            "a None legacy hash (room deleted) must keep the reconcile_log \
             row open for operator review, never auto-resolve — the \
             stale-ghost arm is restricted to bookings"
        );
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
            false,
            false,
        );
        let canonical = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            None,
            Some("C001"),
            false,
            false,
        );
        assert_eq!(mssql, canonical);
    }

    #[test]
    fn checkin_canonical_hash_diverges_on_room_drift() {
        let mssql = checkin_canonical_hash("CIN001", Some("101"), None, None, None, false, false);
        // CT mapper resolved the wrong room — drift the operator
        // should investigate via `ht_reconcile_log`.
        let canonical =
            checkin_canonical_hash("CIN001", Some("102"), None, None, None, false, false);
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
            false,
            false,
        );
        let canonical = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            None,
            Some("C001"),
            false,
            false,
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
            false,
            false,
        );
        let canonical = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            Some("2026-05-18"), // canonical cin_expected_checkout.to_string()
            Some("C001"),
            false,
            false,
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
            false,
            false,
        );
        let pre_fix_canonical_null_checkout = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            None,
            Some("C001"),
            false,
            false,
        );
        assert_ne!(
            pre_fix_mssql_datetime, pre_fix_canonical_null_checkout,
            "pre-Bug-B inputs MUST diverge — this is the bug the fix addresses"
        );
    }

    /// Task #68 regression guard for the 2026-06-28 room-114 / cin 19906
    /// dropped-checkout class. A guest who departs ON the expected date leaves
    /// `effective_checkout` IDENTICAL on both sides (actual == expected), so the
    /// only signal of the dropped checkout is the `checked_out` bit. Canonical
    /// still `active` (checked_out=false) vs legacy checked-out
    /// (checked_out=true) MUST diverge despite every OTHER field matching —
    /// otherwise the sweep is blind to this exact incident.
    #[test]
    fn checkin_canonical_hash_diverges_on_checked_out_flag_when_dates_coincide() {
        let canonical_active = checkin_canonical_hash(
            "CH26-001158",
            Some("114"),
            Some("2026-05-15 15:00:00"),
            Some("2026-05-16"), // effective = expected (active, no actual checkout)
            Some("C019906"),
            false, // checked_out: canonical still active (the dropped checkout)
            false,
        );
        let legacy_checked_out = checkin_canonical_hash(
            "CH26-001158",
            Some("114"),
            Some("2026-05-15 15:00:00"),
            Some("2026-05-16"), // effective = actual = SAME date as expected
            Some("C019906"),
            true, // checked_out: legacy departed on the expected date
            false,
        );
        assert_ne!(
            canonical_active, legacy_checked_out,
            "a dropped checkout (active vs checked-out) MUST diverge even when actual==expected"
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
                NaiveDate::from_ymd_opt(2026, 5, 10)
                    .unwrap()
                    .and_hms_opt(12, 8, 0)
                    .unwrap(),
            ),
            legacy_cust_no: None,
            cin_status: None,
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
            cin_status: None,
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
            cin_status: None,
        };
        assert_eq!(row.effective_checkout_date(), None);
    }

    // -------------------------------------------------------------------
    // Cancelled-folio sentinel (2026-05-19 — reconcile cleanup PR B)
    // -------------------------------------------------------------------
    //
    // When iHOTEL cancels a check-in it deletes the per-room
    // `HT_CheckIn_Ds` rows, so the CT mapper's `derive_room_state`
    // emits `canonical_status='cancelled'` with `first_room_no=None`.
    // Canonical PG keeps the original `legacy_room_no` on the
    // existing `ht_checkins` row. Hashing room context on both sides
    // therefore diverges forever; the live audit on 2026-05-19 found
    // six stuck `value` drifts on HF Hotel (CH26-005252, CH26-005270,
    // CH26-005487, CH26-005524, CH26-005527, CH26-005543) all of
    // which share that exact shape.
    //
    // The `cancelled` flag on `checkin_canonical_hash` collapses both
    // sides onto a sentinel that ignores room/time/customer context.
    // These tests pin (a) the sentinel format, (b) the cross-side
    // equality the production fix delivers, and (c) the regression
    // guarantee that the active-stay path keeps its existing
    // 5-field-shape semantics.

    #[test]
    fn checkin_canonical_hash_returns_cancelled_sentinel_independent_of_room() {
        // With `cancelled = true` the function must IGNORE every input
        // except `legacy_cin_no` — garbage in any other slot must not
        // change the bytes. Pin the exact sentinel format so a future
        // refactor that drops the `CANCELLED|` prefix or swaps the
        // PK position trips this test.
        let expected = sha256("CANCELLED|CH26-005252");

        let with_room_and_dates = checkin_canonical_hash(
            "CH26-005252",
            Some("301"),
            Some("2026-04-01 14:00:00"),
            Some("2026-04-03"),
            Some("C001"),
            false,
            true,
        );
        assert_eq!(with_room_and_dates, expected);

        // Same PK, completely different room/time/customer garbage —
        // sentinel still wins (checked_out flag is ignored too).
        let with_other_garbage = checkin_canonical_hash(
            "CH26-005252",
            Some("999"),
            Some("1999-01-01 00:00:00"),
            Some("2099-12-31"),
            Some("CXXXX"),
            true,
            true,
        );
        assert_eq!(with_other_garbage, expected);

        // All-None on the ignored slots still produces the same hash.
        let with_all_none =
            checkin_canonical_hash("CH26-005252", None, None, None, None, false, true);
        assert_eq!(with_all_none, expected);
    }

    #[test]
    fn cancelled_folio_hashes_match_across_legacy_and_pg_sides() {
        // Models the exact production scenario for the six stuck
        // CH26-* PKs: iHOTEL has deleted the per-room
        // `HT_CheckIn_Ds`, so the legacy projection lands with
        // `legacy_room_no = None`; canonical PG retains
        // `legacy_room_no = Some("301")` on the original
        // `ht_checkins` row. Pre-fix the two hashes differed on the
        // empty-vs-room slot forever. With `cancelled = true` both
        // sides collapse onto the sentinel and converge.
        let legacy_side = checkin_canonical_hash(
            "CH26-005252",
            None, // post-cancel: HT_CheckIn_Ds deleted, no first_room_no
            Some("2026-04-01 14:00:00"),
            Some("2026-04-03"),
            Some("C001"),
            false,
            true,
        );
        let pg_side = checkin_canonical_hash(
            "CH26-005252",
            Some("301"), // canonical kept the original room for triage
            Some("2026-04-01 14:00:00"),
            Some("2026-04-03"),
            Some("C001"),
            false,
            true,
        );
        assert_eq!(
            legacy_side, pg_side,
            "cancelled sentinel must collapse the legacy/PG room-context gap"
        );
        // Pin the exact sentinel bytes — a future refactor that
        // breaks the format will trip this assertion alongside the
        // cross-side equality.
        assert_eq!(legacy_side, sha256("CANCELLED|CH26-005252"));
    }

    #[test]
    fn active_folio_hash_unaffected_by_cancelled_flag_being_false() {
        // Regression guard: with `cancelled = false` the function
        // must keep its existing pre-2026-05-19 5-field hash shape
        // bit-for-bit. Recomputed here from the legacy `format!`
        // string the implementation uses so the test fails loudly if
        // the active-stay path is ever refactored to a different
        // shape.
        let actual = checkin_canonical_hash(
            "CIN001",
            Some("101"),
            Some("2026-04-01 14:00:00"),
            Some("2026-04-03"),
            Some("C001"),
            false,
            false,
        );
        let expected = sha256("CIN001|101|2026-04-01 14:00:00|2026-04-03|C001|co=false");
        assert_eq!(
            actual, expected,
            "active-stay hash shape must be preserved (5 fields + co= checked_out flag) when cancelled=false"
        );
        // And the active hash MUST NOT equal the cancelled sentinel
        // for the same PK — otherwise the cancelled-vs-active
        // distinction collapses and we lose state-change drift
        // detection on the transition itself.
        assert_ne!(actual, sha256("CANCELLED|CIN001"));
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
    /// canonical PG hash equals the freshly-projected legacy hash.
    /// Pre-condition for the auto-resolve UPDATE — if this regresses,
    /// the sweep silently stops draining the queue.
    #[test]
    fn auto_resolve_sweep_marks_resolved_when_hashes_match() {
        let current_legacy_hash = Some("hash-X");
        let current_pg_hash = Some("hash-X");
        assert!(should_auto_resolve(
            "bookings",
            current_legacy_hash,
            current_pg_hash,
            None,
        ));
    }

    /// The sweep MUST leave the row untouched when canonical still
    /// diverges from the freshly-projected legacy hash. Drift that
    /// persists is exactly what the alerts are supposed to surface.
    #[test]
    fn auto_resolve_sweep_skips_when_drift_persists() {
        let current_legacy_hash = Some("hash-X");
        let current_pg_hash = Some("hash-Y");
        assert!(!should_auto_resolve(
            "bookings",
            current_legacy_hash,
            current_pg_hash,
            None,
        ));
    }

    /// Stale-ghost convergence: a `bookings` row whose legacy composite
    /// key has vanished (`current_legacy_hash == None`) but whose
    /// canonical PG hash now matches the legacy state recorded at
    /// detection (`recorded_mssql_hash`) MUST auto-resolve. This is the
    /// `R015423|501` class — a `missing_pg` row logged before the
    /// new-app booking linked `legacy_book_id`, stranded once its
    /// initial room-type line was swapped out. Without this it sits
    /// open forever and trips the >4h `level_drift_alert`.
    #[test]
    fn auto_resolve_sweep_resolves_booking_stale_ghost() {
        let recorded = booking_canonical_hash(
            "R015423",
            Some("2026-07-04"),
            Some("2026-07-05"),
            Some("C22381"),
        );
        let current_pg_hash = recorded.clone();
        assert!(should_auto_resolve(
            "bookings",
            None, // legacy room-type line is gone
            Some(current_pg_hash.as_str()),
            Some(recorded.as_str()),
        ));
    }

    /// Negative: a genuine, persistent `missing_pg` booking (canonical
    /// still absent today) MUST NOT be swept closed by the stale-ghost
    /// arm. The legacy side recorded a hash, but PG never caught up —
    /// this is real, un-reconciled drift.
    #[test]
    fn auto_resolve_sweep_keeps_open_persistent_missing_pg_booking() {
        assert!(!should_auto_resolve(
            "bookings",
            None,           // legacy key gone
            None,           // canonical still missing
            Some("hash-X"), // legacy recorded a hash at detection
        ));
    }

    /// Negative: the stale-ghost arm fires only when canonical matches
    /// the RECORDED legacy state. A `bookings` row where the legacy key
    /// vanished but canonical PG diverges from what legacy had recorded
    /// is real drift (the booking's dates/customer changed in PG
    /// independently) and must stay open.
    #[test]
    fn auto_resolve_sweep_keeps_open_booking_ghost_with_mismatched_canonical() {
        assert!(!should_auto_resolve(
            "bookings",
            None,
            Some("hash-current-pg"),
            Some("hash-recorded-legacy"),
        ));
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
        assert!(ct_lag_is_warning(DEFAULT_CT_LAG_WARN_VERSIONS + 1, 0, t));
    }

    /// Poll-age breaches alone are sufficient to warn — a watcher whose
    /// poll-loop is wedged still keeps `last_seen_version` advancing
    /// from the in-flight transaction, but `last_polled_at` stops
    /// refreshing (incident-2026-05-18 root-cause shape).
    #[test]
    fn ct_lag_warns_when_only_poll_age_breaches() {
        let t = default_ct_thresholds();
        assert!(ct_lag_is_warning(0, DEFAULT_CT_LAG_WARN_SECONDS + 1, t));
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

    // -------------------------------------------------------------------
    // 2026-05-19 — `count_distinct_legacy_checkin_rooms` dedup invariants.
    //
    // Backs the CH22-000722 cardinality false-positive fix: iHOTEL's
    // HT_CheckIn_Ds table commonly carries multiple detail rows for the
    // same room (extends, re-keys, deposit returns). The
    // `ht_checkin_rooms` junction's `UNIQUE (cr_cin_id, cr_room_id)`
    // already collapses them; the cardinality check has to as well or
    // every such folio shows up as drift.
    // -------------------------------------------------------------------
    use crate::sync::row::test_support::{
        HashMapRow as TestHashMapRow, MockValue as TestMockValue,
    };

    fn ds_row_with_room(room_no: TestMockValue) -> TestHashMapRow {
        TestHashMapRow::new("HT_CheckIn_Ds").with("Cin_Room_No", room_no)
    }

    #[test]
    fn cardinality_dedups_duplicate_room_in_legacy_ds() {
        // CH22-000722 shape: three Ds rows, all room 417. Distinct
        // count must collapse to 1 to match the junction.
        let rooms = vec![
            ds_row_with_room(TestMockValue::Str("417".into())),
            ds_row_with_room(TestMockValue::Str("417".into())),
            ds_row_with_room(TestMockValue::Str("417".into())),
        ];
        assert_eq!(count_distinct_legacy_checkin_rooms(&rooms), 1);
    }

    #[test]
    fn cardinality_preserves_distinct_multi_room() {
        // A real multi-room folio (rooms 417 + 418) still surfaces as
        // distinct = 2, so a short junction (only one room mirrored)
        // still flags `cardinality` drift correctly.
        let rooms = vec![
            ds_row_with_room(TestMockValue::Str("417".into())),
            ds_row_with_room(TestMockValue::Str("418".into())),
        ];
        assert_eq!(count_distinct_legacy_checkin_rooms(&rooms), 2);
    }

    #[test]
    fn cardinality_skips_null_and_empty_room_no() {
        // Mirrors `project_rooms` skip semantic: NULL and empty
        // `Cin_Room_No` are dropped from the junction projection, so
        // they must also be dropped from the legacy cardinality count.
        let rooms = vec![
            ds_row_with_room(TestMockValue::Null),
            ds_row_with_room(TestMockValue::Str("".into())),
            ds_row_with_room(TestMockValue::Str("417".into())),
        ];
        assert_eq!(count_distinct_legacy_checkin_rooms(&rooms), 1);
    }

    // -------------------------------------------------------------------
    // 2026-07-27 — `missing_pg` re-ingest arm of the auto-resolve sweep.
    //
    // Live shape that motivated the arm: a 2026-07-11 cross-table
    // watermark clobber on HF Ville dropped a CT event, leaving customer
    // `C2413` and bookings `R002066|110` / `|112` / `|217` unconverged for
    // 16 days. Legacy still had every row; canonical had none. No existing
    // path could ever close them — `should_auto_resolve` needs BOTH hashes,
    // and CT's 2-day retention means the watcher can never redeliver.
    // -------------------------------------------------------------------

    /// Env-isolation helper for the two self-heal feature flags. Same
    /// save/restore shape as `with_mode_env` / `with_threshold_envs` above;
    /// one process-wide lock because `set_var` is process-wide.
    /// Takes a SLICE of vars rather than one, so a test that needs two flags
    /// set at once passes them together. **Do NOT nest calls** — the guard is
    /// a plain non-reentrant `Mutex`, so a nested call self-deadlocks (the
    /// test binary hangs forever rather than failing).
    fn with_env_vars<T, F: FnOnce() -> T>(vars: &[(&str, Option<&str>)], f: F) -> T {
        use std::sync::Mutex;
        static LOCK: Mutex<()> = Mutex::new(());
        let _g = LOCK.lock().unwrap();
        let priors: Vec<(String, Option<String>)> = vars
            .iter()
            .map(|(name, _)| ((*name).to_string(), env::var(name).ok()))
            .collect();
        for (name, value) in vars {
            match value {
                Some(v) => env::set_var(name, v),
                None => env::remove_var(name),
            }
        }
        let out = f();
        for (name, prior) in priors {
            match prior {
                Some(v) => env::set_var(&name, v),
                None => env::remove_var(&name),
            }
        }
        out
    }

    /// Single-var convenience wrapper over [`with_env_vars`].
    fn with_self_heal_flag_env<T, F: FnOnce() -> T>(
        var_name: &str,
        value: Option<&str>,
        f: F,
    ) -> T {
        with_env_vars(&[(var_name, value)], f)
    }

    const REINGEST_FLAG: &str = "RECONCILE_REINGEST_MISSING_PG_ENABLED";
    const FORCE_CONVERGE_FLAG: &str = "RECONCILE_FORCE_CONVERGE_ENABLED";

    #[test]
    fn reingest_flag_defaults_off_when_env_unset() {
        let on =
            with_self_heal_flag_env(REINGEST_FLAG, None, reconcile_reingest_missing_pg_enabled);
        assert!(
            !on,
            "the missing_pg re-ingest arm writes canonical state — it must ship dark"
        );
    }

    #[test]
    fn reingest_flag_on_for_exact_true_literal() {
        let on = with_self_heal_flag_env(REINGEST_FLAG, Some("true"), {
            reconcile_reingest_missing_pg_enabled
        });
        assert!(on);
    }

    /// The `== "true"` comparison is strict on purpose. An operator who
    /// types `TRUE` / `1` / ` true` gets the SAFE (off) behaviour rather
    /// than a silently-enabled canonical-write path.
    #[test]
    fn reingest_flag_is_strict_about_the_true_literal() {
        for value in ["TRUE", "True", "1", "yes", " true", "true ", ""] {
            let on = with_self_heal_flag_env(REINGEST_FLAG, Some(value), {
                reconcile_reingest_missing_pg_enabled
            });
            assert!(!on, "{value:?} must NOT enable the re-ingest arm");
        }
    }

    /// The re-ingest flag must be independent of the force-converge flag:
    /// `RECONCILE_FORCE_CONVERGE_ENABLED` is already `true` in production
    /// on `sync-hfville`, so sharing it would have shipped the new
    /// canonical-write class ON with no coordinated flip.
    #[test]
    fn reingest_flag_is_independent_of_force_converge_flag() {
        // Both vars go through ONE `with_env_vars` call — nesting two
        // guarded calls would self-deadlock on the shared Mutex.
        let on = with_env_vars(
            &[(FORCE_CONVERGE_FLAG, Some("true")), (REINGEST_FLAG, None)],
            reconcile_reingest_missing_pg_enabled,
        );
        assert!(
            !on,
            "force-converge being ON must never imply the missing_pg re-ingest arm is ON"
        );
    }

    /// Pre-existing coverage gap: the force-converge flag reader had no
    /// test at all. Pin its default and its strictness too.
    #[test]
    fn force_converge_flag_defaults_off_and_is_strict() {
        let unset = with_self_heal_flag_env(FORCE_CONVERGE_FLAG, None, {
            reconcile_force_converge_enabled
        });
        assert!(!unset);
        let loose = with_self_heal_flag_env(FORCE_CONVERGE_FLAG, Some("TRUE"), {
            reconcile_force_converge_enabled
        });
        assert!(!loose);
        let exact = with_self_heal_flag_env(FORCE_CONVERGE_FLAG, Some("true"), {
            reconcile_force_converge_enabled
        });
        assert!(exact);
    }

    const LEGACY_HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const PG_HASH: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    /// Comfortably past `FORCE_CONVERGE_MIN_AGE_SECS` (3600s) — the live
    /// rows had aged 16 days.
    const OLD_ENOUGH_SECS: f64 = FORCE_CONVERGE_MIN_AGE_SECS + 1.0;

    #[test]
    fn reingest_arm_eligible_when_legacy_present_and_canonical_absent() {
        // The exact live shape: customer C2413 / booking R002066|110.
        for table in ["customers", "bookings"] {
            assert!(
                reingest_missing_pg_eligible(table, Some(LEGACY_HASH), None, OLD_ENOUGH_SECS, true,),
                "{table}: legacy present + canonical absent + past min age must be eligible"
            );
        }
    }

    /// The genuine-anomaly guard. A VANISHED legacy row must never trip the
    /// re-ingest arm — there is nothing to project from, and closing the
    /// row would hide a real deletion. Same invariant
    /// `rooms_dispatch_missing_legacy_row_does_not_auto_resolve` pins for
    /// the observational path.
    #[test]
    fn reingest_arm_not_eligible_when_legacy_row_vanished() {
        for table in ["customers", "bookings"] {
            assert!(
                !reingest_missing_pg_eligible(table, None, None, OLD_ENOUGH_SECS, true),
                "{table}: a vanished legacy row must stay open for operator review"
            );
        }
        // An empty-string legacy hash is treated the same as absent.
        assert!(!reingest_missing_pg_eligible(
            "customers",
            Some(""),
            None,
            OLD_ENOUGH_SECS,
            true,
        ));
        // …and the row must also stay open on the observational path.
        assert!(!should_auto_resolve(
            "customers",
            None,
            None,
            Some(LEGACY_HASH)
        ));
    }

    #[test]
    fn reingest_arm_not_eligible_below_min_age() {
        assert!(
            !reingest_missing_pg_eligible(
                "customers",
                Some(LEGACY_HASH),
                None,
                FORCE_CONVERGE_MIN_AGE_SECS - 1.0,
                true,
            ),
            "a fresh divergence is probably an in-flight CT event — don't race it"
        );
        // Exactly at the threshold IS eligible (`>=`).
        assert!(reingest_missing_pg_eligible(
            "customers",
            Some(LEGACY_HASH),
            None,
            FORCE_CONVERGE_MIN_AGE_SECS,
            true,
        ));
    }

    #[test]
    fn reingest_arm_not_eligible_when_flag_off() {
        assert!(
            !reingest_missing_pg_eligible(
                "bookings",
                Some(LEGACY_HASH),
                None,
                OLD_ENOUGH_SECS,
                false,
            ),
            "flag OFF ⇒ zero canonical writes"
        );
    }

    /// `rooms` / `checkins` are outside the arm: a room absent from
    /// canonical is a provisioning gap rather than a dropped ingest, and
    /// the check-in aggregate re-ingest hasn't been verified the same way.
    #[test]
    fn reingest_arm_not_eligible_for_unsupported_tables() {
        for table in ["rooms", "checkins", "payments"] {
            assert!(
                !reingest_missing_pg_eligible(
                    table,
                    Some(LEGACY_HASH),
                    None,
                    OLD_ENOUGH_SECS,
                    true,
                ),
                "{table} must not be re-ingested by this arm"
            );
        }
    }

    /// Value drift (BOTH hashes present) keeps routing to the pre-existing
    /// force-converge arm, and must NOT be picked up by the new arm.
    #[test]
    fn value_drift_still_routes_to_force_converge_arm_unchanged() {
        for table in ["customers", "rooms"] {
            assert!(
                force_converge_value_drift_eligible(
                    table,
                    Some(LEGACY_HASH),
                    Some(PG_HASH),
                    OLD_ENOUGH_SECS,
                    true,
                ),
                "{table} value drift must still reach the force-converge arm"
            );
        }
        assert!(
            !reingest_missing_pg_eligible(
                "customers",
                Some(LEGACY_HASH),
                Some(PG_HASH),
                OLD_ENOUGH_SECS,
                true,
            ),
            "a value drift is not a dropped ingest — the re-ingest arm must ignore it"
        );
    }

    /// Symmetric guard: the force-converge arm keeps its "BOTH hashes
    /// present" precondition, so the missing_pg shape never reaches it.
    #[test]
    fn force_converge_arm_ignores_the_missing_pg_shape() {
        assert!(!force_converge_value_drift_eligible(
            "customers",
            Some(LEGACY_HASH),
            None,
            OLD_ENOUGH_SECS,
            true,
        ));
        assert!(!force_converge_value_drift_eligible(
            "bookings",
            Some(LEGACY_HASH),
            Some(PG_HASH),
            OLD_ENOUGH_SECS,
            true,
        ));
    }

    // -------------------------------------------------------------------
    // Auto-resolve sweep candidate ordering — the FK guarantee.
    // -------------------------------------------------------------------

    fn candidate(id: i64, table: &str, pk: &str, age_secs: f64) -> ReconcileCandidate {
        (id, table.to_string(), pk.to_string(), None, age_secs)
    }

    #[test]
    fn reconcile_fk_rank_orders_parents_before_dependents() {
        assert!(reconcile_table_fk_rank("customers") < reconcile_table_fk_rank("bookings"));
        assert!(reconcile_table_fk_rank("rooms") < reconcile_table_fk_rank("bookings"));
        assert!(reconcile_table_fk_rank("bookings") < reconcile_table_fk_rank("checkins"));
        assert!(reconcile_table_fk_rank("checkins") < reconcile_table_fk_rank("something_new"));
    }

    /// The live FK shape: booking `R002066` references customer `C2413`.
    /// Every `customers` row must be healed before ANY `bookings` row in
    /// the same sweep pass, regardless of insertion order or age.
    #[test]
    fn reconcile_candidates_sort_customers_before_bookings() {
        let mut rows = vec![
            candidate(11, "bookings", "R002066|217", 1_400_000.0),
            candidate(12, "bookings", "R002066|110", 1_400_000.0),
            // The customer row was detected LATER (smaller age) — it must
            // still be processed first.
            candidate(13, "customers", "C2413", 1_000.0),
        ];
        sort_reconcile_candidates(&mut rows);
        let tables: Vec<&str> = rows.iter().map(|r| r.1.as_str()).collect();
        assert_eq!(
            tables,
            vec!["customers", "bookings", "bookings"],
            "FK parents must run first — a booking healed before its customer errors out"
        );
    }

    /// `age_secs` is `NOW() - detected_at`, so oldest-first == largest age
    /// first. This is what stops the `LIMIT 500` from starving old rows out
    /// of an arbitrary subset once the backlog exceeds the cap.
    #[test]
    fn reconcile_candidates_sort_oldest_detected_at_first_within_a_table() {
        let mut rows = vec![
            candidate(1, "bookings", "R000001|110", 3_600.0),
            candidate(2, "bookings", "R000002|110", 1_382_400.0), // 16 days
            candidate(3, "bookings", "R000003|110", 86_400.0),    // 1 day
        ];
        sort_reconcile_candidates(&mut rows);
        let ids: Vec<i64> = rows.iter().map(|r| r.0).collect();
        assert_eq!(
            ids,
            vec![2, 3, 1],
            "oldest detected_at (largest age_secs) must be retested first"
        );
    }

    #[test]
    fn reconcile_candidates_tie_break_on_id_is_deterministic() {
        let mut rows = vec![
            candidate(9, "customers", "C0009", 7_200.0),
            candidate(4, "customers", "C0004", 7_200.0),
            candidate(7, "customers", "C0007", 7_200.0),
        ];
        sort_reconcile_candidates(&mut rows);
        assert_eq!(rows.iter().map(|r| r.0).collect::<Vec<_>>(), vec![4, 7, 9]);
    }

    // -------------------------------------------------------------------
    // Level-drift recovery notification (paired all-clear).
    // -------------------------------------------------------------------

    fn owned(values: &[&str]) -> Vec<String> {
        values.iter().map(|v| (*v).to_string()).collect()
    }

    #[test]
    fn level_drift_recovery_fires_when_cooldown_table_has_no_stale_rows() {
        // `bookings` alerted at some point (cooldown row exists) and now
        // has zero unconverged rows past the threshold ⇒ all-clear.
        let recovered = tables_recovered(&owned(&["bookings"]), &owned(&[]));
        assert_eq!(recovered, owned(&["bookings"]));
    }

    #[test]
    fn level_drift_recovery_suppressed_while_table_still_stale() {
        let recovered = tables_recovered(&owned(&["bookings"]), &owned(&["bookings"]));
        assert!(
            recovered.is_empty(),
            "a table that still has unconverged rows past the threshold has NOT recovered"
        );
    }

    #[test]
    fn level_drift_recovery_reports_only_the_converged_tables() {
        let recovered = tables_recovered(
            &owned(&["bookings", "customers", "checkins"]),
            &owned(&["checkins"]),
        );
        assert_eq!(
            recovered,
            owned(&["bookings", "customers"]),
            "output is sorted + deduped so the Slack body is deterministic"
        );
    }

    /// The cooldown table is shared with the stale-checkin tripwire. Its
    /// key has no unconverged-row count to compare against, so the sync-lag
    /// all-clear must never claim it recovered or clear its cooldown —
    /// doing so would let that alert refire every 15 minutes.
    #[test]
    fn level_drift_recovery_never_claims_non_reconcile_cooldown_keys() {
        let recovered = tables_recovered(
            &owned(&[STALE_CHECKIN_COOLDOWN_KEY, "customers"]),
            &owned(&[]),
        );
        assert_eq!(recovered, owned(&["customers"]));
    }

    #[test]
    fn level_drift_recovery_is_silent_without_cooldown_rows() {
        // Never alerted ⇒ nothing to close, even with zero stale rows.
        assert!(tables_recovered(&owned(&[]), &owned(&[])).is_empty());
    }

    // -------------------------------------------------------------------
    // Per-table CT watermark health check (R3 TODO resolution).
    // -------------------------------------------------------------------

    /// `now` is threaded in explicitly (rather than each row reaching for
    /// `Utc::now()`) so `poll_age_seconds` comes out as an exact integer —
    /// otherwise sub-second construction skew truncates 4000s to 3999s and
    /// the assertions flake.
    fn per_table_at(
        now: chrono::DateTime<chrono::Utc>,
        name: &str,
        version: i64,
        polled_secs_ago: Option<i64>,
    ) -> PerTableWatermark {
        PerTableWatermark {
            table_name: name.to_string(),
            last_seen_version: version,
            last_polled_at: polled_secs_ago.map(|s| now - chrono::Duration::seconds(s)),
        }
    }

    #[test]
    fn per_table_watermark_flag_defaults_off() {
        let on = with_self_heal_flag_env("SYNC_PER_TABLE_WATERMARK", None, {
            per_table_watermark_enabled
        });
        assert!(!on, "global-watermark mode stays the default");
        let on = with_self_heal_flag_env("SYNC_PER_TABLE_WATERMARK", Some("true"), {
            per_table_watermark_enabled
        });
        assert!(on);
    }

    #[test]
    fn stalest_per_table_watermark_returns_none_for_empty_input() {
        assert!(stalest_per_table_watermark(&[], 100, chrono::Utc::now()).is_none());
    }

    /// The point of the whole exercise: report WHICH table is behind, not a
    /// global figure that is frozen at its bootstrap value in per-table mode.
    #[test]
    fn stalest_per_table_watermark_picks_largest_version_lag() {
        let now = chrono::Utc::now();
        let rows = vec![
            per_table_at(now, "HT_Customers", 990, Some(10)),
            per_table_at(now, "HT_Book_H", 500, Some(10)),
            per_table_at(now, "HT_Rooms", 1_000, Some(10)),
        ];
        let (stalest, version_lag, _) = stalest_per_table_watermark(&rows, 1_000, now).unwrap();
        assert_eq!(stalest.table_name, "HT_Book_H");
        assert_eq!(version_lag, 500);
    }

    #[test]
    fn stalest_per_table_watermark_breaks_version_ties_on_oldest_poll() {
        let now = chrono::Utc::now();
        let rows = vec![
            per_table_at(now, "HT_Customers", 1_000, Some(30)),
            per_table_at(now, "HT_CheckIn_H", 1_000, Some(4_000)),
        ];
        let (stalest, version_lag, poll_age) =
            stalest_per_table_watermark(&rows, 1_000, now).unwrap();
        assert_eq!(stalest.table_name, "HT_CheckIn_H");
        assert_eq!(version_lag, 0);
        assert_eq!(poll_age, 4_000);
    }

    /// A never-polled table is infinitely old — same convention the global
    /// arm uses so the warn branch fires and the operator sees it.
    #[test]
    fn stalest_per_table_watermark_treats_never_polled_as_infinitely_old() {
        let now = chrono::Utc::now();
        let rows = vec![
            per_table_at(now, "HT_Customers", 1_000, Some(10)),
            per_table_at(now, "HT_Book_Pro", 1_000, None),
        ];
        let (stalest, _, poll_age) = stalest_per_table_watermark(&rows, 1_000, now).unwrap();
        assert_eq!(stalest.table_name, "HT_Book_Pro");
        assert_eq!(poll_age, i64::MAX);
        assert!(ct_lag_is_warning(0, poll_age, default_ct_thresholds()));
    }

    /// A watermark ahead of `CHANGE_TRACKING_CURRENT_VERSION()` is a CT
    /// anomaly; saturating-sub keeps it at zero lag rather than panicking.
    #[test]
    fn stalest_per_table_watermark_saturates_on_watermark_ahead_of_current() {
        let now = chrono::Utc::now();
        let rows = vec![per_table_at(now, "HT_Rooms", 5_000, Some(1))];
        let (_, version_lag, _) = stalest_per_table_watermark(&rows, 1_000, now).unwrap();
        assert_eq!(version_lag, 0);
    }

    /// Equally-healthy tables must produce a stable pick so the log line
    /// doesn't flap between tables every tick.
    #[test]
    fn stalest_per_table_watermark_is_deterministic_on_a_full_tie() {
        let now = chrono::Utc::now();
        let rows = vec![
            per_table_at(now, "HT_Rooms", 1_000, Some(5)),
            per_table_at(now, "HT_Customers", 1_000, Some(5)),
        ];
        let (first, _, _) = stalest_per_table_watermark(&rows, 1_000, now).unwrap();
        assert_eq!(first.table_name, "HT_Customers");
    }
}
