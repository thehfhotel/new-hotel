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
/// `LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD`), a Slack message is fired
/// at the end of the cycle. Pass `None` to silence alerts (e.g. from
/// `bin/sync --bootstrap` where the Slack channel is reserved for
/// CT-watcher errors and bootstrap progress).
pub async fn run_sync(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
    slack: Option<&SlackClient>,
) {
    let mode = ReconcileMode::from_env();
    tracing::info!(?mode, "[Sync] Starting sync cycle...");

    if let Err(e) = sync_customers(legacy_pool, pg_pool).await {
        tracing::error!("[Sync] Customer sync failed: {}", e);
        record_error(pg_pool, "customers", &e.to_string()).await;
    }

    if let Err(e) = sync_rooms(legacy_pool, pg_pool).await {
        tracing::error!("[Sync] Room sync failed: {}", e);
        record_error(pg_pool, "rooms", &e.to_string()).await;
    }

    if let Err(e) = sync_bookings(legacy_pool, pg_pool).await {
        tracing::error!("[Sync] Booking sync failed: {}", e);
        record_error(pg_pool, "bookings", &e.to_string()).await;
    }

    if let Err(e) = sync_checkins(legacy_pool, pg_pool).await {
        tracing::error!("[Sync] Check-in sync failed: {}", e);
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
    check_drift_and_alert(pg_pool, slack).await;

    tracing::info!("[Sync] Sync cycle complete");
}

/// Phase 6: read the configured drift-alert threshold (default
/// [`DEFAULT_DRIFT_ALERT_THRESHOLD`]). Anything that doesn't parse to a
/// positive integer falls back to the safe default with a warning.
fn drift_alert_threshold_from_env() -> i64 {
    match env::var("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD") {
        Ok(raw) => match raw.trim().parse::<i64>() {
            Ok(n) if n > 0 => n,
            _ => {
                tracing::warn!(
                    value = %raw,
                    default = DEFAULT_DRIFT_ALERT_THRESHOLD,
                    "Invalid LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD; using default"
                );
                DEFAULT_DRIFT_ALERT_THRESHOLD
            }
        },
        Err(_) => DEFAULT_DRIFT_ALERT_THRESHOLD,
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
async fn check_drift_and_alert(pg_pool: &PgPool, slack: Option<&SlackClient>) {
    let threshold = drift_alert_threshold_from_env();

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
                error = %e,
                "[Sync] Failed to query ht_reconcile_log for drift alert — observability degraded"
            );
            return;
        }
    };

    let breaches = tables_breaching_threshold(&counts, threshold);
    if breaches.is_empty() {
        tracing::debug!(
            tables_observed = counts.len(),
            threshold,
            "[Sync] Drift alert: no tables breach threshold"
        );
        return;
    }

    // Always log; Slack is opportunistic.
    for (table, count) in &breaches {
        tracing::warn!(
            table,
            count,
            threshold,
            "[Sync] Drift alert: table exceeds reconcile-log threshold in last hour"
        );
    }

    let Some(slack) = slack else {
        tracing::info!(
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
    let msg = SlackMessage::with_text(format!(
        ":rotating_light: *Reconcile drift threshold exceeded* :rotating_light:\n\
         The drift-reconcile job recorded more than {threshold} unresolved \
         `ht_reconcile_log` rows for the following table(s) in the last hour:\n\
         {body}\n\
         _Investigate via `docs/runbook-sync.md` §9 (Phase 6 drift alert)._"
    ));
    slack.send_message(&msg).await;
}

/// Compute SHA256 hash of a string
fn sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
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

    for row in &rows {
        let book_no = row.get::<&str, _>("Book_No").unwrap_or_default().to_string();
        let book_date: Option<NaiveDateTime> = row.try_get("Book_Date").unwrap_or(None);
        let book_date_in: Option<NaiveDateTime> = row.try_get("Book_Date_in").unwrap_or(None);
        let book_date_out: Option<NaiveDateTime> = row.try_get("Book_Date_out").unwrap_or(None);
        let book_cust_name = row.get::<&str, _>("Book_Cust_Name").map(String::from);
        let book_cust_id = row.get::<&str, _>("Book_Cust_ID").map(String::from);
        let book_status = row.get::<i32, _>("Book_Status");
        let book_room_type = row.get::<&str, _>("Book_Room_Type").map(String::from);

        let hash_input = format!(
            "{}|{:?}|{:?}|{:?}|{}|{}|{:?}|{}",
            book_no,
            book_date,
            book_date_in,
            book_date_out,
            book_cust_name.as_deref().unwrap_or(""),
            book_cust_id.as_deref().unwrap_or(""),
            book_status,
            book_room_type.as_deref().unwrap_or(""),
        );
        let hash = sha256(&hash_input);

        // Composite key: book_no + room_type
        let room_type_key = book_room_type.as_deref().unwrap_or("");

        let existing = sqlx::query_scalar::<_, Option<String>>(
            "SELECT sync_hash FROM ht_bookings_legacy WHERE book_no = $1 AND COALESCE(book_room_type, '') = $2"
        )
        .bind(&book_no)
        .bind(room_type_key)
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
                        UPDATE ht_bookings_legacy
                        SET book_date = $1, book_date_in = $2, book_date_out = $3,
                            book_cust_name = $4, book_cust_id = $5, book_status = $6,
                            sync_hash = $7, synced_at = NOW()
                        WHERE book_no = $8 AND COALESCE(book_room_type, '') = $9
                        "#,
                    )
                    .bind(&book_date)
                    .bind(&book_date_in)
                    .bind(&book_date_out)
                    .bind(&book_cust_name)
                    .bind(&book_cust_id)
                    .bind(&book_status)
                    .bind(&hash)
                    .bind(&book_no)
                    .bind(room_type_key)
                    .execute(pg_pool)
                    .await?;
                    updated += 1;
                }
                ReconcileMode::DiffOnly => {
                    let mssql_json = json!({
                        "Book_No": book_no,
                        "Book_Date": book_date.map(|d| d.to_string()),
                        "Book_Date_in": book_date_in.map(|d| d.to_string()),
                        "Book_Date_out": book_date_out.map(|d| d.to_string()),
                        "Book_Cust_Name": book_cust_name,
                        "Book_Cust_ID": book_cust_id,
                        "Book_Status": book_status,
                        "Book_Room_Type": book_room_type,
                    });
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
                    updated += 1;
                }
            },
            None => match mode {
                ReconcileMode::Upsert => {
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
                    .bind(&book_date)
                    .bind(&book_date_in)
                    .bind(&book_date_out)
                    .bind(&book_cust_name)
                    .bind(&book_cust_id)
                    .bind(&book_status)
                    .bind(&book_room_type)
                    .bind(&hash)
                    .execute(pg_pool)
                    .await?;
                    added += 1;
                }
                ReconcileMode::DiffOnly => {
                    let mssql_json = json!({
                        "Book_No": book_no,
                        "Book_Date": book_date.map(|d| d.to_string()),
                        "Book_Date_in": book_date_in.map(|d| d.to_string()),
                        "Book_Date_out": book_date_out.map(|d| d.to_string()),
                        "Book_Cust_Name": book_cust_name,
                        "Book_Cust_ID": book_cust_id,
                        "Book_Status": book_status,
                        "Book_Room_Type": book_room_type,
                    });
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

    for row in &rows {
        let cin_no = row.get::<&str, _>("Cin_no").unwrap_or_default().to_string();
        let cin_room_no = row.get::<&str, _>("Cin_Room_No").map(String::from);
        let cin_room_in: Option<NaiveDateTime> = row.try_get("Cin_Room_In").unwrap_or(None);
        let cin_room_out: Option<NaiveDateTime> = row.try_get("Cin_Room_Out").unwrap_or(None);
        let cin_cust_name = row.get::<&str, _>("Cin_cust_name").map(String::from);
        let cin_cust_no = row.get::<&str, _>("Cin_cust_no").map(String::from);
        let cin_status = row.get::<&str, _>("Cin_status").map(String::from);

        let hash_input = format!(
            "{}|{}|{:?}|{:?}|{}|{}|{}",
            cin_no,
            cin_room_no.as_deref().unwrap_or(""),
            cin_room_in,
            cin_room_out,
            cin_cust_name.as_deref().unwrap_or(""),
            cin_cust_no.as_deref().unwrap_or(""),
            cin_status.as_deref().unwrap_or(""),
        );
        let hash = sha256(&hash_input);

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
                    sqlx::query(
                        r#"
                        UPDATE ht_checkins_legacy
                        SET cin_room_no = $1, cin_room_in = $2, cin_room_out = $3,
                            cin_cust_name = $4, cin_cust_no = $5, cin_status = $6,
                            sync_hash = $7, synced_at = NOW()
                        WHERE cin_no = $8
                        "#,
                    )
                    .bind(&cin_room_no)
                    .bind(&cin_room_in)
                    .bind(&cin_room_out)
                    .bind(&cin_cust_name)
                    .bind(&cin_cust_no)
                    .bind(&cin_status)
                    .bind(&hash)
                    .bind(&cin_no)
                    .execute(pg_pool)
                    .await?;
                    updated += 1;
                }
                ReconcileMode::DiffOnly => {
                    let mssql_json = json!({
                        "Cin_no": cin_no,
                        "Cin_Room_No": cin_room_no,
                        "Cin_Room_In": cin_room_in.map(|d| d.to_string()),
                        "Cin_Room_Out": cin_room_out.map(|d| d.to_string()),
                        "Cin_cust_name": cin_cust_name,
                        "Cin_cust_no": cin_cust_no,
                        "Cin_status": cin_status,
                    });
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
                    updated += 1;
                }
            },
            None => match mode {
                ReconcileMode::Upsert => {
                    sqlx::query(
                        r#"
                        INSERT INTO ht_checkins_legacy
                            (cin_no, cin_room_no, cin_room_in, cin_room_out,
                             cin_cust_name, cin_cust_no, cin_status, sync_hash)
                        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                        "#,
                    )
                    .bind(&cin_no)
                    .bind(&cin_room_no)
                    .bind(&cin_room_in)
                    .bind(&cin_room_out)
                    .bind(&cin_cust_name)
                    .bind(&cin_cust_no)
                    .bind(&cin_status)
                    .bind(&hash)
                    .execute(pg_pool)
                    .await?;
                    added += 1;
                }
                ReconcileMode::DiffOnly => {
                    let mssql_json = json!({
                        "Cin_no": cin_no,
                        "Cin_Room_No": cin_room_no,
                        "Cin_Room_In": cin_room_in.map(|d| d.to_string()),
                        "Cin_Room_Out": cin_room_out.map(|d| d.to_string()),
                        "Cin_cust_name": cin_cust_name,
                        "Cin_cust_no": cin_cust_no,
                        "Cin_status": cin_status,
                    });
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
    /// as `with_mode_env` above.
    fn with_threshold_env<F: FnOnce() -> i64>(value: Option<&str>, f: F) -> i64 {
        use std::sync::Mutex;
        static LOCK: Mutex<()> = Mutex::new(());
        let _g = LOCK.lock().unwrap();
        let prior = env::var("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD").ok();
        match value {
            Some(v) => env::set_var("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD", v),
            None => env::remove_var("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD"),
        }
        let out = f();
        match prior {
            Some(v) => env::set_var("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD", v),
            None => env::remove_var("LEGACY_RECONCILE_DRIFT_ALERT_THRESHOLD"),
        }
        out
    }

    #[test]
    fn threshold_defaults_when_env_unset() {
        let v = with_threshold_env(None, drift_alert_threshold_from_env);
        assert_eq!(v, DEFAULT_DRIFT_ALERT_THRESHOLD);
    }

    #[test]
    fn threshold_parses_custom_positive_value() {
        let v = with_threshold_env(Some("125"), drift_alert_threshold_from_env);
        assert_eq!(v, 125);
    }

    #[test]
    fn threshold_falls_back_on_zero_or_negative() {
        let v = with_threshold_env(Some("0"), drift_alert_threshold_from_env);
        assert_eq!(v, DEFAULT_DRIFT_ALERT_THRESHOLD);
        let v = with_threshold_env(Some("-5"), drift_alert_threshold_from_env);
        assert_eq!(v, DEFAULT_DRIFT_ALERT_THRESHOLD);
    }

    #[test]
    fn threshold_falls_back_on_garbage() {
        let v = with_threshold_env(Some("not-a-number"), drift_alert_threshold_from_env);
        assert_eq!(v, DEFAULT_DRIFT_ALERT_THRESHOLD);
    }
}
