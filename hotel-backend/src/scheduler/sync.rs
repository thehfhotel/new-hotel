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
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::sync::Mutex;
use std::time::Instant;

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

    // Phase 6: drift-alert tripwire. Best-effort — degraded observability
    // never aborts the reconcile loop.
    check_drift_and_alert(pg_pool, slack, site_id).await;

    // Track D / T7 HIGH-1: level-triggered drift digest. Catches
    // long-lived single-row divergences that never breach the
    // edge-triggered 50/hr volume threshold. Per-table cooldown keeps
    // Slack from drowning during a known-bad cardinality migration
    // window.
    check_level_drift_and_alert(pg_pool, slack, site_id).await;

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

/// Track D / T7 HIGH-1 — per-(site, table) cooldown for the
/// level-triggered drift digest. Keyed by `"{site_id}::{table_name}"`
/// so HF Hotel and HF Ville don't share state. Held as a process-global
/// Mutex because reconcile is serial per process and one watcher per
/// container — contention is zero in practice.
fn level_alert_cooldowns() -> &'static Mutex<HashMap<String, Instant>> {
    static COOLDOWNS: std::sync::OnceLock<Mutex<HashMap<String, Instant>>> =
        std::sync::OnceLock::new();
    COOLDOWNS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Track D / T7 HIGH-1 — pure decision function for the level-triggered
/// cooldown gate. Returns true iff the given table on the given site is
/// eligible to alert (cooldown elapsed or never alerted). On true, the
/// caller MUST call `mark_level_alert_sent` to start the next cooldown.
///
/// Pulled into a free function with explicit args so the unit test can
/// inject a `now` instead of relying on `Instant::now()`.
fn level_alert_eligible(
    state: &mut HashMap<String, Instant>,
    site_id: &str,
    table: &str,
    now: Instant,
    cooldown: std::time::Duration,
) -> bool {
    let key = format!("{site_id}::{table}");
    match state.get(&key) {
        Some(last_sent) => now.duration_since(*last_sent) >= cooldown,
        None => true,
    }
}

/// Record the time of the most recent level-triggered alert for
/// `(site_id, table)`. Pairs with [`level_alert_eligible`].
fn mark_level_alert_sent(
    state: &mut HashMap<String, Instant>,
    site_id: &str,
    table: &str,
    now: Instant,
) {
    let key = format!("{site_id}::{table}");
    state.insert(key, now);
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
    let now = Instant::now();
    let mut to_alert: Vec<(String, i64)> = Vec::new();
    {
        let mut state = match level_alert_cooldowns().lock() {
            Ok(g) => g,
            Err(poisoned) => poisoned.into_inner(),
        };
        for (table, count) in &counts {
            if level_alert_eligible(&mut state, site_id, table, now, cooldown) {
                to_alert.push((table.clone(), *count));
                mark_level_alert_sent(&mut state, site_id, table, now);
            } else {
                tracing::debug!(
                    site = %site_id,
                    table,
                    count,
                    "[Sync] Level drift alert suppressed by cooldown"
                );
            }
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
                "Cin_Room_In": d.room_in.map(|t| t.to_string()),
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

async fn sync_customers(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    let mode = ReconcileMode::from_env();
    tracing::info!(?mode, "[Sync] Syncing customers...");

    let mut conn = legacy_pool.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT
                Cust_no,
                Cust_name,
                Cust_Type,
                Cust_Add_tel,
                Cust_IDcard,
                C_Address
            FROM View_Customers
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    for row in &rows {
        let cust_no = row.get::<&str, _>("Cust_no").unwrap_or_default().to_string();
        let cust_name = row.get::<&str, _>("Cust_name").map(String::from);
        let cust_type = row.get::<&str, _>("Cust_Type").map(String::from);
        let cust_phone = row.get::<&str, _>("Cust_Add_tel").map(String::from);
        let cust_idcard = row.get::<&str, _>("Cust_IDcard").map(String::from);
        let cust_address = row.get::<&str, _>("C_Address").map(String::from);

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

async fn sync_rooms(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    let mode = ReconcileMode::from_env();
    tracing::info!(?mode, "[Sync] Syncing rooms...");

    let mut conn = legacy_pool.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT
                Room_no,
                Room_Type,
                Room_Details,
                Room_Clean,
                Room_Use,
                Room_Book,
                Room_Manternace,
                Room_PriceA,
                Room_PriceB,
                Room_PriceC,
                Room_Group,
                Room_Book_Name,
                Room_Book_Time
            FROM HT_Rooms
            ORDER BY Room_no
            "#,
        )
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

async fn sync_bookings(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    let mode = ReconcileMode::from_env();
    tracing::info!(?mode, "[Sync] Syncing bookings...");

    let mut conn = legacy_pool.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT
                Book_No,
                Book_Date,
                Book_Date_in,
                Book_Date_out,
                Book_Cust_Name,
                Book_Cust_ID,
                Book_Status,
                Book_Room_Type
            FROM View_Booking_Ds
            "#,
        )
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

async fn sync_checkins(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let start = Instant::now();
    let mode = ReconcileMode::from_env();
    tracing::info!(?mode, "[Sync] Syncing check-ins...");

    let mut conn = legacy_pool.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT
                Cin_no,
                Cin_Room_No,
                Cin_Room_In,
                Cin_Room_Out,
                Cin_cust_name,
                Cin_cust_no,
                Cin_status
            FROM View_CheckIn_Ds
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let mut added = 0i32;
    let mut updated = 0i32;
    let mut unchanged = 0i32;

    // Phase 6 hotfix (2026-04-29): `View_CheckIn_Ds` returns 41-45 rows
    // per `Cin_no` (one per booked room/detail). Aggregate by PK first,
    // then compute one deterministic hash per PK, so we record one
    // divergence + one cache UPDATE per PK instead of per-row.
    let mut groups: BTreeMap<String, Vec<CheckinDetail>> = BTreeMap::new();
    for row in &rows {
        let cin_no = row.get::<&str, _>("Cin_no").unwrap_or_default().to_string();
        let detail = CheckinDetail {
            room_no: row.get::<&str, _>("Cin_Room_No").map(String::from),
            room_in: row.try_get("Cin_Room_In").unwrap_or(None),
            room_out: row.try_get("Cin_Room_Out").unwrap_or(None),
            cust_name: row.get::<&str, _>("Cin_cust_name").map(String::from),
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
        // docs). Timestamps use `NaiveDateTime::to_string()` so both
        // sides serialise as YYYY-MM-DD HH:MM:SS.
        let room_in_str = representative.and_then(|d| d.room_in.map(|t| t.to_string()));
        let room_out_str = representative.and_then(|d| d.room_out.map(|t| t.to_string()));
        let mssql_hash = checkin_canonical_hash(
            &cin_no,
            representative.and_then(|d| d.room_no.as_deref()),
            room_in_str.as_deref(),
            room_out_str.as_deref(),
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
                    let checkout_str = c.cin_checkout_time.map(|t| t.to_string());
                    checkin_canonical_hash(
                        &cin_no,
                        c.legacy_room_no.as_deref(),
                        checkin_str.as_deref(),
                        checkout_str.as_deref(),
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
                        "cin_checkout_time": c.cin_checkout_time.map(|t| t.to_string()),
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
struct CanonicalCheckinRow {
    legacy_room_no: Option<String>,
    cin_checkin_time: Option<NaiveDateTime>,
    cin_checkout_time: Option<NaiveDateTime>,
    legacy_cust_no: Option<String>,
}

async fn fetch_canonical_checkin(
    pg_pool: &PgPool,
    legacy_cin_no: &str,
) -> Result<Option<CanonicalCheckinRow>, sqlx::Error> {
    sqlx::query_as::<_, (Option<String>, Option<NaiveDateTime>, Option<NaiveDateTime>, Option<String>)>(
        "SELECT legacy_room_no, cin_checkin_time, cin_checkout_time, legacy_cust_no \
           FROM ht_checkins \
          WHERE legacy_cin_no = $1 \
          LIMIT 1",
    )
    .bind(legacy_cin_no)
    .fetch_optional(pg_pool)
    .await
    .map(|opt| {
        opt.map(|(room, checkin, checkout, cust)| CanonicalCheckinRow {
            legacy_room_no: room,
            cin_checkin_time: checkin,
            cin_checkout_time: checkout,
            legacy_cust_no: cust,
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
        // Active stays have `cin_checkout_time IS NULL` (canonical) /
        // `Cin_Room_Out IS NULL` (legacy). Both serialise to "" so the
        // hashes line up while the guest is still in residence.
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
    fn level_alert_eligible_returns_true_for_fresh_table() {
        let mut state: HashMap<String, Instant> = HashMap::new();
        let now = Instant::now();
        let cooldown = std::time::Duration::from_secs(86_400);
        assert!(level_alert_eligible(
            &mut state,
            "hfhotel",
            "checkins",
            now,
            cooldown
        ));
    }

    #[test]
    fn level_alert_eligible_returns_false_inside_cooldown_window() {
        let mut state: HashMap<String, Instant> = HashMap::new();
        let now = Instant::now();
        let cooldown = std::time::Duration::from_secs(86_400);
        mark_level_alert_sent(&mut state, "hfhotel", "checkins", now);
        // Same instant → cooldown not yet elapsed.
        assert!(!level_alert_eligible(
            &mut state,
            "hfhotel",
            "checkins",
            now,
            cooldown
        ));
    }

    #[test]
    fn level_alert_eligible_does_not_leak_across_sites() {
        let mut state: HashMap<String, Instant> = HashMap::new();
        let now = Instant::now();
        let cooldown = std::time::Duration::from_secs(86_400);
        mark_level_alert_sent(&mut state, "hfhotel", "checkins", now);
        // HF Hotel cooldown active — HF Ville must still be eligible.
        assert!(level_alert_eligible(
            &mut state,
            "hfville",
            "checkins",
            now,
            cooldown
        ));
    }

    #[test]
    fn level_alert_eligible_does_not_leak_across_tables() {
        let mut state: HashMap<String, Instant> = HashMap::new();
        let now = Instant::now();
        let cooldown = std::time::Duration::from_secs(86_400);
        mark_level_alert_sent(&mut state, "hfhotel", "checkins", now);
        // Different table on same site → still eligible.
        assert!(level_alert_eligible(
            &mut state,
            "hfhotel",
            "customers",
            now,
            cooldown
        ));
    }
}
