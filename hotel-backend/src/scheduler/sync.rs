//! Background sync job: replicates legacy SQL Server data to PostgreSQL.
//!
//! ## Two modes (Phase 5.5+)
//!
//! Controlled by env var `LEGACY_SYNC_RECONCILE_MODE`:
//!
//! | Mode        | Behaviour                                                 | Default? |
//! |-------------|-----------------------------------------------------------|----------|
//! | `diff_only` | Compute hashes, log divergent rows to `ht_reconcile_log`. | ✅ yes   |
//! | `upsert`    | Original behaviour: UPSERT into `ht_*_legacy`.            | escape hatch |
//!
//! Phase 5.5 cutover: the CT watcher (`bin/sync.rs`) is now authoritative
//! for canonical PG state, so this job is demoted from a 5-min full-sync
//! UPSERT to a 15-min drift-detection tripwire. If the watcher misses a
//! row (CT retention overflow, transient mapper bug, schema regression),
//! the next reconcile tick lands a row in `ht_reconcile_log` for an
//! operator to investigate. `upsert` mode remains as an escape hatch in
//! case the watcher needs to be turned off operationally.
//!
//! Per docs/architecture.md §3.6d, §8 (Phase 5.5 row).
//!
//! ## What this job syncs
//!
//! 1. Customers (View_Customers -> ht_customers_legacy)
//! 2. Rooms (HT_Rooms -> ht_rooms_legacy)
//! 3. Bookings (View_Booking_Ds -> ht_bookings_legacy)
//! 4. Check-ins (View_CheckIn_Ds -> ht_checkins_legacy)
//!
//! Uses SHA256 hashing for change detection - unchanged rows are skipped
//! (in both modes). Note: the legacy mirror tables (`ht_*_legacy`) are
//! NOT the canonical state — that lives in `ht_*` proper, owned by the
//! CT watcher's mappers. The legacy mirror is now purely a hash cache
//! for drift detection.

use chrono::NaiveDateTime;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::env;
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

/// Phase 5.5 diff-only path: record a divergence into `ht_reconcile_log`
/// instead of mutating canonical state. Best-effort — a failed insert
/// only degrades observability, so we never bubble it up to abort the
/// reconcile loop.
async fn record_divergence(
    pg_pool: &PgPool,
    table_name: &str,
    legacy_pk: &str,
    pg_hash: Option<&str>,
    mssql_hash: Option<&str>,
    mssql_row_json: serde_json::Value,
    pg_row_json: Option<serde_json::Value>,
) {
    let result = sqlx::query(
        "INSERT INTO ht_reconcile_log \
            (table_name, legacy_pk, pg_hash, mssql_hash, mssql_row_json, pg_row_json) \
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(table_name)
    .bind(legacy_pk)
    .bind(pg_hash)
    .bind(mssql_hash)
    .bind(mssql_row_json)
    .bind(pg_row_json)
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

        let hash_input = format!(
            "{}|{}|{}|{}|{}|{}",
            cust_no,
            cust_name.as_deref().unwrap_or(""),
            cust_type.as_deref().unwrap_or(""),
            cust_phone.as_deref().unwrap_or(""),
            cust_idcard.as_deref().unwrap_or(""),
            cust_address.as_deref().unwrap_or(""),
        );
        let hash = sha256(&hash_input);

        // Check if record exists and if hash changed
        let existing = sqlx::query_scalar::<_, Option<String>>(
            "SELECT sync_hash FROM ht_customers_legacy WHERE cust_no = $1"
        )
        .bind(&cust_no)
        .fetch_optional(pg_pool)
        .await?;

        match existing {
            Some(Some(existing_hash)) if existing_hash == hash => {
                unchanged += 1;
            }
            Some(prior_hash) => match mode {
                ReconcileMode::Upsert => {
                    // Exists but hash changed - update
                    sqlx::query(
                        r#"
                        UPDATE ht_customers_legacy
                        SET cust_name = $1, cust_type = $2, cust_phone = $3,
                            cust_idcard = $4, cust_address = $5,
                            sync_hash = $6, synced_at = NOW()
                        WHERE cust_no = $7
                        "#,
                    )
                    .bind(&cust_name)
                    .bind(&cust_type)
                    .bind(&cust_phone)
                    .bind(&cust_idcard)
                    .bind(&cust_address)
                    .bind(&hash)
                    .bind(&cust_no)
                    .execute(pg_pool)
                    .await?;
                    updated += 1;
                }
                ReconcileMode::DiffOnly => {
                    // Phase 5.5: log divergence; CT watcher owns canonical state.
                    let mssql_json = json!({
                        "Cust_no": cust_no,
                        "Cust_name": cust_name,
                        "Cust_Type": cust_type,
                        "Cust_Add_tel": cust_phone,
                        "Cust_IDcard": cust_idcard,
                        "C_Address": cust_address,
                    });
                    record_divergence(
                        pg_pool,
                        "customers",
                        &cust_no,
                        prior_hash.as_deref(),
                        Some(&hash),
                        mssql_json,
                        None,
                    )
                    .await;
                    // Acknowledge the divergence in the cache so the next
                    // reconcile tick doesn't re-flag this same row forever
                    // and spam the Phase 6 drift alert. Cache-only write —
                    // does NOT mutate canonical state. Best-effort: a
                    // failed cache update only re-fires the alert next
                    // tick, never a data-correctness issue.
                    let _ = sqlx::query(
                        "UPDATE ht_customers_legacy SET sync_hash = $1, synced_at = NOW() \
                         WHERE cust_no = $2",
                    )
                    .bind(&hash)
                    .bind(&cust_no)
                    .execute(pg_pool)
                    .await;
                    updated += 1;
                }
            },
            None => match mode {
                ReconcileMode::Upsert => {
                    // New record - insert
                    sqlx::query(
                        r#"
                        INSERT INTO ht_customers_legacy
                            (cust_no, cust_name, cust_type, cust_phone, cust_idcard, cust_address, sync_hash)
                        VALUES ($1, $2, $3, $4, $5, $6, $7)
                        "#,
                    )
                    .bind(&cust_no)
                    .bind(&cust_name)
                    .bind(&cust_type)
                    .bind(&cust_phone)
                    .bind(&cust_idcard)
                    .bind(&cust_address)
                    .bind(&hash)
                    .execute(pg_pool)
                    .await?;
                    added += 1;
                }
                ReconcileMode::DiffOnly => {
                    // PG-miss divergence: legacy row exists, canonical
                    // mirror does not yet. Logged with `pg_hash=NULL`.
                    let mssql_json = json!({
                        "Cust_no": cust_no,
                        "Cust_name": cust_name,
                        "Cust_Type": cust_type,
                        "Cust_Add_tel": cust_phone,
                        "Cust_IDcard": cust_idcard,
                        "C_Address": cust_address,
                    });
                    record_divergence(
                        pg_pool,
                        "customers",
                        &cust_no,
                        None,
                        Some(&hash),
                        mssql_json,
                        None,
                    )
                    .await;
                    added += 1;
                }
            },
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

        let hash_input = format!(
            "{}|{}|{}|{}|{}|{}|{}|{:?}|{:?}|{:?}|{}|{}|{:?}",
            room_no,
            room_type.as_deref().unwrap_or(""),
            room_details.as_deref().unwrap_or(""),
            room_clean.as_deref().unwrap_or(""),
            room_use.as_deref().unwrap_or(""),
            room_book.as_deref().unwrap_or(""),
            room_manternace.as_deref().unwrap_or(""),
            room_price_a,
            room_price_b,
            room_price_c,
            room_group.as_deref().unwrap_or(""),
            room_book_name.as_deref().unwrap_or(""),
            room_book_time,
        );
        let hash = sha256(&hash_input);

        let existing = sqlx::query_scalar::<_, Option<String>>(
            "SELECT sync_hash FROM ht_rooms_legacy WHERE room_no = $1"
        )
        .bind(&room_no)
        .fetch_optional(pg_pool)
        .await?;

        match existing {
            Some(Some(existing_hash)) if existing_hash == hash => {
                unchanged += 1;
            }
            Some(prior_hash) => match mode {
                ReconcileMode::Upsert => {
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
                    .bind(&room_type)
                    .bind(&room_details)
                    .bind(&room_clean)
                    .bind(&room_use)
                    .bind(&room_book)
                    .bind(&room_manternace)
                    .bind(&room_price_a)
                    .bind(&room_price_b)
                    .bind(&room_price_c)
                    .bind(&room_group)
                    .bind(&room_book_name)
                    .bind(&room_book_time)
                    .bind(&hash)
                    .bind(&room_no)
                    .execute(pg_pool)
                    .await?;
                    updated += 1;
                }
                ReconcileMode::DiffOnly => {
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
                    record_divergence(
                        pg_pool,
                        "rooms",
                        &room_no,
                        prior_hash.as_deref(),
                        Some(&hash),
                        mssql_json,
                        None,
                    )
                    .await;
                    // Phase 6 fix: ack the divergence in the cache so the
                    // next reconcile tick doesn't re-flag the same row.
                    // See sync_customers DiffOnly branch for the rationale.
                    let _ = sqlx::query(
                        "UPDATE ht_rooms_legacy SET sync_hash = $1, synced_at = NOW() \
                         WHERE room_no = $2",
                    )
                    .bind(&hash)
                    .bind(&room_no)
                    .execute(pg_pool)
                    .await;
                    updated += 1;
                }
            },
            None => match mode {
                ReconcileMode::Upsert => {
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
                    .bind(&room_no)
                    .bind(&room_type)
                    .bind(&room_details)
                    .bind(&room_clean)
                    .bind(&room_use)
                    .bind(&room_book)
                    .bind(&room_manternace)
                    .bind(&room_price_a)
                    .bind(&room_price_b)
                    .bind(&room_price_c)
                    .bind(&room_group)
                    .bind(&room_book_name)
                    .bind(&room_book_time)
                    .bind(&hash)
                    .execute(pg_pool)
                    .await?;
                    added += 1;
                }
                ReconcileMode::DiffOnly => {
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
                    record_divergence(
                        pg_pool,
                        "rooms",
                        &room_no,
                        None,
                        Some(&hash),
                        mssql_json,
                        None,
                    )
                    .await;
                    added += 1;
                }
            },
        }
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
        let hash = aggregate_booking_hash(&book_no, &room_type_key, &mut details);

        let existing = sqlx::query_scalar::<_, Option<String>>(
            "SELECT sync_hash FROM ht_bookings_legacy WHERE book_no = $1 AND COALESCE(book_room_type, '') = $2"
        )
        .bind(&book_no)
        .bind(&room_type_key)
        .fetch_optional(pg_pool)
        .await?;

        match existing {
            Some(Some(existing_hash)) if existing_hash == hash => {
                unchanged += 1;
            }
            Some(prior_hash) => match mode {
                ReconcileMode::Upsert => {
                    // Cache row is single-row per PK by schema; canonical
                    // multi-row data lives in `ht_bookings`. Pick the
                    // first deterministically-sorted detail as the
                    // representative row for the cache columns.
                    let canonical = &details[0];
                    sqlx::query(
                        r#"
                        UPDATE ht_bookings_legacy
                        SET book_date = $1, book_date_in = $2, book_date_out = $3,
                            book_cust_name = $4, book_cust_id = $5, book_status = $6,
                            sync_hash = $7, synced_at = NOW()
                        WHERE book_no = $8 AND COALESCE(book_room_type, '') = $9
                        "#,
                    )
                    .bind(&canonical.book_date)
                    .bind(&canonical.book_date_in)
                    .bind(&canonical.book_date_out)
                    .bind(&canonical.book_cust_name)
                    .bind(&canonical.book_cust_id)
                    .bind(&canonical.book_status)
                    .bind(&hash)
                    .bind(&book_no)
                    .bind(&room_type_key)
                    .execute(pg_pool)
                    .await?;
                    updated += 1;
                }
                ReconcileMode::DiffOnly => {
                    let mssql_json = booking_group_json(&book_no, &details);
                    let composite_pk = format!("{book_no}|{room_type_key}");
                    record_divergence(
                        pg_pool,
                        "bookings",
                        &composite_pk,
                        prior_hash.as_deref(),
                        Some(&hash),
                        mssql_json,
                        None,
                    )
                    .await;
                    // Phase 6 fix: ack the divergence in the cache so the
                    // next reconcile tick doesn't re-flag the same row.
                    // Composite PK on bookings — match the SELECT shape
                    // above. ONE UPDATE per PK now (was per-row, which
                    // race-tripped the spam under multi-row PKs).
                    let _ = sqlx::query(
                        "UPDATE ht_bookings_legacy SET sync_hash = $1, synced_at = NOW() \
                         WHERE book_no = $2 AND COALESCE(book_room_type, '') = $3",
                    )
                    .bind(&hash)
                    .bind(&book_no)
                    .bind(&room_type_key)
                    .execute(pg_pool)
                    .await;
                    updated += 1;
                }
            },
            None => match mode {
                ReconcileMode::Upsert => {
                    let canonical = &details[0];
                    sqlx::query(
                        r#"
                        INSERT INTO ht_bookings_legacy
                            (book_no, book_date, book_date_in, book_date_out,
                             book_cust_name, book_cust_id, book_status,
                             book_room_type, sync_hash)
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                        "#,
                    )
                    .bind(&book_no)
                    .bind(&canonical.book_date)
                    .bind(&canonical.book_date_in)
                    .bind(&canonical.book_date_out)
                    .bind(&canonical.book_cust_name)
                    .bind(&canonical.book_cust_id)
                    .bind(&canonical.book_status)
                    .bind(&canonical.book_room_type)
                    .bind(&hash)
                    .execute(pg_pool)
                    .await?;
                    added += 1;
                }
                ReconcileMode::DiffOnly => {
                    let mssql_json = booking_group_json(&book_no, &details);
                    let composite_pk = format!("{book_no}|{room_type_key}");
                    record_divergence(
                        pg_pool,
                        "bookings",
                        &composite_pk,
                        None,
                        Some(&hash),
                        mssql_json,
                        None,
                    )
                    .await;
                    added += 1;
                }
            },
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
        let hash = aggregate_checkin_hash(&cin_no, &mut details);

        let existing = sqlx::query_scalar::<_, Option<String>>(
            "SELECT sync_hash FROM ht_checkins_legacy WHERE cin_no = $1"
        )
        .bind(&cin_no)
        .fetch_optional(pg_pool)
        .await?;

        match existing {
            Some(Some(existing_hash)) if existing_hash == hash => {
                unchanged += 1;
            }
            Some(prior_hash) => match mode {
                ReconcileMode::Upsert => {
                    // Cache row is single-row per PK by schema; canonical
                    // multi-room data lives in `ht_checkins`. Pick the
                    // first deterministically-sorted detail as the
                    // representative row for the cache columns.
                    let canonical = &details[0];
                    sqlx::query(
                        r#"
                        UPDATE ht_checkins_legacy
                        SET cin_room_no = $1, cin_room_in = $2, cin_room_out = $3,
                            cin_cust_name = $4, cin_cust_no = $5, cin_status = $6,
                            sync_hash = $7, synced_at = NOW()
                        WHERE cin_no = $8
                        "#,
                    )
                    .bind(&canonical.room_no)
                    .bind(&canonical.room_in)
                    .bind(&canonical.room_out)
                    .bind(&canonical.cust_name)
                    .bind(&canonical.cust_no)
                    .bind(&canonical.status)
                    .bind(&hash)
                    .bind(&cin_no)
                    .execute(pg_pool)
                    .await?;
                    updated += 1;
                }
                ReconcileMode::DiffOnly => {
                    let mssql_json = checkin_group_json(&cin_no, &details);
                    record_divergence(
                        pg_pool,
                        "checkins",
                        &cin_no,
                        prior_hash.as_deref(),
                        Some(&hash),
                        mssql_json,
                        None,
                    )
                    .await;
                    // Phase 6 fix: ack the divergence in the cache so the
                    // next reconcile tick doesn't re-flag the same row.
                    // This is the dominant source of the 22-24k/hour drift
                    // alert spam observed 2026-04-29 — every reconcile
                    // re-detected the same ~3k checkin PKs forever. ONE
                    // UPDATE per PK (was per-row → re-flagged within tick
                    // for multi-row PKs).
                    let _ = sqlx::query(
                        "UPDATE ht_checkins_legacy SET sync_hash = $1, synced_at = NOW() \
                         WHERE cin_no = $2",
                    )
                    .bind(&hash)
                    .bind(&cin_no)
                    .execute(pg_pool)
                    .await;
                    updated += 1;
                }
            },
            None => match mode {
                ReconcileMode::Upsert => {
                    let canonical = &details[0];
                    sqlx::query(
                        r#"
                        INSERT INTO ht_checkins_legacy
                            (cin_no, cin_room_no, cin_room_in, cin_room_out,
                             cin_cust_name, cin_cust_no, cin_status, sync_hash)
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                        "#,
                    )
                    .bind(&cin_no)
                    .bind(&canonical.room_no)
                    .bind(&canonical.room_in)
                    .bind(&canonical.room_out)
                    .bind(&canonical.cust_name)
                    .bind(&canonical.cust_no)
                    .bind(&canonical.status)
                    .bind(&hash)
                    .execute(pg_pool)
                    .await?;
                    added += 1;
                }
                ReconcileMode::DiffOnly => {
                    let mssql_json = checkin_group_json(&cin_no, &details);
                    record_divergence(
                        pg_pool,
                        "checkins",
                        &cin_no,
                        None,
                        Some(&hash),
                        mssql_json,
                        None,
                    )
                    .await;
                    added += 1;
                }
            },
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
}
