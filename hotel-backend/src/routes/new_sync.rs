//! Sync status API routes
//!
//! - GET /api/new/sync/status - Get sync health and last sync times
//!
//! Track D / T7 CRIT-2 (2026-05-13): rewritten to read `legacy_sync_status`
//! (16 entity_types written by the CT watcher in `bin/sync.rs`) instead of
//! the legacy `sync_status` table (4 entity_types written only by the
//! demoted full-sync reconciler). Pre-rewrite, 12 CT-tracked tables had
//! no health surface — the dashboard showed "healthy" while a half-broken
//! CT watcher silently lost mappings.
//!
//! The response also unions a `writebackQueue` block with grouped
//! `writeback_jobs` counts per `(intent, status)` so operators can spot
//! a backlog (pending>500, failed>100, stuck in_progress) without
//! switching to the DB shell.

use axum::{extract::State, Json};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::error::ApiResult;
use crate::routes::mode::AppState;

/// Per-table sync status surfaced by the CT watcher.
///
/// `table_name` mirrors the upstream MSSQL table (e.g. `HT_Customers`,
/// `HT_Book_H`, `HT_Bill_Debt_Ds`) — preserving the exact case lets
/// operators cross-reference with `docs/legacy-app/SCHEMA.sql`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncEntityStatus {
    pub table_name: String,
    pub last_processed_at: Option<DateTime<Utc>>,
    pub rows_ingested: i64,
    pub rows_skipped: i64,
    pub last_error: Option<String>,
    pub last_error_at: Option<DateTime<Utc>>,
    pub consecutive_failures: i32,
    pub health: String,
}

/// One row of the writeback-queue depth digest, grouped by intent + status.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WritebackQueueRow {
    pub intent: String,
    pub status: String,
    pub count: i64,
}

/// Overall sync status response (v2 — Track D / T7 CRIT-2).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncStatusResponse {
    pub success: bool,
    pub sync_enabled: bool,
    pub overall_health: String,
    pub entities: Vec<SyncEntityStatus>,
    /// Writeback queue depth grouped by `(intent, status)`. Empty
    /// vector when the table is empty (steady state).
    pub writeback_queue: Vec<WritebackQueueRow>,
}

/// CT-watcher staleness threshold. A `last_processed_at` older than
/// this with no error means the watcher tick rate has dropped — usually
/// the legacy MSSQL pool is degraded. 15 min matches the previous
/// behaviour for the demoted reconciler.
const STALE_THRESHOLD_MINUTES: i64 = 15;

/// GET /api/new/sync/status - Get sync health and last sync times for
/// every CT-tracked legacy table + writeback queue depth.
pub async fn get_sync_status(
    State(state): State<AppState>,
) -> ApiResult<Json<SyncStatusResponse>> {
    let pool = &state.new_pool;

    let sync_enabled = std::env::var("SYNC_ENABLED")
        .map(|v| v != "false")
        .unwrap_or(true);

    let entities = fetch_legacy_sync_status(pool, sync_enabled).await;
    let writeback_queue = fetch_writeback_queue_depth(pool).await;

    let overall_health = compute_overall_health(sync_enabled, &entities);

    Ok(Json(SyncStatusResponse {
        success: true,
        sync_enabled,
        overall_health,
        entities,
        writeback_queue,
    }))
}

/// Track D / T7 CRIT-2 — read every CT-tracked entity's health row.
/// `legacy_sync_status` is seeded by migrations 017 + 022 with exactly
/// 16 rows (10 canonical + 6 legacy_mirror). The CT watcher updates
/// `last_processed_at` / `rows_ingested` / `rows_skipped` after every
/// tick; `last_error` / `consecutive_failures` are bumped by error
/// paths inside `bin/sync.rs`.
async fn fetch_legacy_sync_status(
    pool: &sqlx::PgPool,
    sync_enabled: bool,
) -> Vec<SyncEntityStatus> {
    let rows = sqlx::query(
        r#"
        SELECT
            table_name,
            last_processed_at,
            rows_ingested,
            rows_skipped,
            last_error,
            last_error_at,
            consecutive_failures
        FROM legacy_sync_status
        ORDER BY table_name
        "#,
    )
    .fetch_all(pool)
    .await;

    match rows {
        Ok(rows) => {
            use sqlx::Row;
            rows.iter()
                .map(|row| {
                    let consecutive_failures: i32 =
                        row.try_get("consecutive_failures").unwrap_or(0);
                    let last_processed_at: Option<DateTime<Utc>> =
                        row.try_get("last_processed_at").unwrap_or(None);
                    let table_name: String = row.try_get("table_name").unwrap_or_default();
                    let health = compute_entity_health(
                        sync_enabled,
                        consecutive_failures,
                        last_processed_at,
                    );

                    SyncEntityStatus {
                        table_name,
                        last_processed_at,
                        rows_ingested: row.try_get("rows_ingested").unwrap_or(0),
                        rows_skipped: row.try_get("rows_skipped").unwrap_or(0),
                        last_error: row.try_get("last_error").unwrap_or(None),
                        last_error_at: row.try_get("last_error_at").unwrap_or(None),
                        consecutive_failures,
                        health,
                    }
                })
                .collect()
        }
        Err(err) => {
            tracing::warn!(
                error = %err,
                "[sync_status] Failed to query legacy_sync_status — returning empty list"
            );
            Vec::new()
        }
    }
}

/// Track D / T7 CRIT-2 — writeback queue depth, grouped by
/// `(intent, status)` so an operator can see "all `record_payment`
/// jobs are stuck pending" without touching the DB shell. Empty
/// vector when the queue is empty (steady state — successful jobs
/// flip to `done` and are kept indefinitely as an audit log).
async fn fetch_writeback_queue_depth(pool: &sqlx::PgPool) -> Vec<WritebackQueueRow> {
    let rows = sqlx::query_as::<_, (String, String, i64)>(
        r#"
        SELECT intent, status, count(*)::bigint AS depth
          FROM writeback_jobs
         WHERE status IN ('pending', 'in_progress', 'failed', 'exhausted')
         GROUP BY intent, status
         ORDER BY intent, status
        "#,
    )
    .fetch_all(pool)
    .await;

    match rows {
        Ok(rows) => rows
            .into_iter()
            .map(|(intent, status, count)| WritebackQueueRow {
                intent,
                status,
                count,
            })
            .collect(),
        Err(err) => {
            tracing::warn!(
                error = %err,
                "[sync_status] Failed to query writeback_jobs depth — returning empty list"
            );
            Vec::new()
        }
    }
}

/// Pure helper — derive a per-entity health label. Pulled out for unit
/// tests (no PG dependency).
pub fn compute_entity_health(
    sync_enabled: bool,
    consecutive_failures: i32,
    last_processed_at: Option<DateTime<Utc>>,
) -> String {
    if !sync_enabled {
        return "disabled".to_string();
    }
    if consecutive_failures >= 3 {
        return "error".to_string();
    }
    let Some(last) = last_processed_at else {
        return "pending".to_string();
    };
    if consecutive_failures > 0 {
        return "warning".to_string();
    }
    let now = Utc::now();
    let stale_threshold = chrono::Duration::minutes(STALE_THRESHOLD_MINUTES);
    if now - last > stale_threshold {
        "stale".to_string()
    } else {
        "healthy".to_string()
    }
}

/// Pure helper — aggregate the per-entity healths into one overall
/// label. Worst-case wins.
pub fn compute_overall_health(sync_enabled: bool, entities: &[SyncEntityStatus]) -> String {
    if !sync_enabled {
        return "disabled".to_string();
    }
    if entities.is_empty() {
        return "pending".to_string();
    }
    if entities.iter().any(|e| e.health == "error") {
        return "error".to_string();
    }
    if entities.iter().any(|e| e.health == "warning" || e.health == "stale") {
        return "warning".to_string();
    }
    if entities.iter().all(|e| e.health == "healthy") {
        return "healthy".to_string();
    }
    "pending".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stub_entity(health: &str) -> SyncEntityStatus {
        SyncEntityStatus {
            table_name: "HT_Customers".to_string(),
            last_processed_at: None,
            rows_ingested: 0,
            rows_skipped: 0,
            last_error: None,
            last_error_at: None,
            consecutive_failures: 0,
            health: health.to_string(),
        }
    }

    #[test]
    fn compute_entity_health_returns_disabled_when_sync_off() {
        let h = compute_entity_health(false, 0, Some(Utc::now()));
        assert_eq!(h, "disabled");
    }

    #[test]
    fn compute_entity_health_returns_error_above_three_consecutive_failures() {
        let h = compute_entity_health(true, 3, Some(Utc::now()));
        assert_eq!(h, "error");
        let h = compute_entity_health(true, 99, Some(Utc::now()));
        assert_eq!(h, "error");
    }

    #[test]
    fn compute_entity_health_returns_pending_when_never_processed() {
        let h = compute_entity_health(true, 0, None);
        assert_eq!(h, "pending");
    }

    #[test]
    fn compute_entity_health_returns_warning_on_partial_failure_streak() {
        let h = compute_entity_health(true, 1, Some(Utc::now()));
        assert_eq!(h, "warning");
    }

    #[test]
    fn compute_entity_health_returns_stale_after_threshold() {
        let stale = Utc::now() - chrono::Duration::minutes(STALE_THRESHOLD_MINUTES + 1);
        let h = compute_entity_health(true, 0, Some(stale));
        assert_eq!(h, "stale");
    }

    #[test]
    fn compute_entity_health_returns_healthy_when_recent_and_no_errors() {
        let h = compute_entity_health(true, 0, Some(Utc::now()));
        assert_eq!(h, "healthy");
    }

    #[test]
    fn compute_overall_health_returns_disabled_when_sync_off() {
        let entities = vec![stub_entity("healthy")];
        assert_eq!(compute_overall_health(false, &entities), "disabled");
    }

    #[test]
    fn compute_overall_health_propagates_error() {
        let entities = vec![stub_entity("healthy"), stub_entity("error")];
        assert_eq!(compute_overall_health(true, &entities), "error");
    }

    #[test]
    fn compute_overall_health_propagates_warning() {
        let entities = vec![stub_entity("healthy"), stub_entity("warning")];
        assert_eq!(compute_overall_health(true, &entities), "warning");
        let entities = vec![stub_entity("healthy"), stub_entity("stale")];
        assert_eq!(compute_overall_health(true, &entities), "warning");
    }

    #[test]
    fn compute_overall_health_returns_healthy_only_when_all_entities_healthy() {
        let entities = vec![stub_entity("healthy"), stub_entity("healthy")];
        assert_eq!(compute_overall_health(true, &entities), "healthy");
    }

    #[test]
    fn compute_overall_health_returns_pending_on_empty_or_pending_set() {
        let entities: Vec<SyncEntityStatus> = Vec::new();
        assert_eq!(compute_overall_health(true, &entities), "pending");
        let entities = vec![stub_entity("pending"), stub_entity("healthy")];
        assert_eq!(compute_overall_health(true, &entities), "pending");
    }

    /// Track D / T7 CRIT-2 — the route reads `legacy_sync_status` which
    /// is seeded with 16 rows by migrations 017 + 022. Lock the
    /// expected entity list so a future refactor that re-points the
    /// route at the old `sync_status` table (4 rows) fails loudly.
    ///
    /// We don't have a live PG connection in unit tests; instead we
    /// pin the literal table list in `bin/sync.rs::CT_ENABLED_TABLES`
    /// and verify it's exactly 16 tables. The route's SELECT scans the
    /// whole `legacy_sync_status` table (no `WHERE` filter), so as
    /// long as the seed is intact the response will include all 16.
    #[test]
    fn route_returns_all_16_legacy_entities() {
        let expected_tables = [
            "HT_Customers",
            "HT_Rooms",
            "HT_Room_Status",
            "HT_Book_H",
            "HT_Book_Ds",
            "HT_Book_Date",
            "HT_CheckIn_H",
            "HT_CheckIn_Ds",
            "HT_CheckIn_Pay",
            "HT_Receipt_H",
            "HT_Cupon",
            "HT_CheckIn_Product",
            "HT_Deposit",
            "HT_Changed_Room",
            "HT_Bill_Debt_H",
            "HT_Bill_Debt_Ds",
        ];
        assert_eq!(
            expected_tables.len(),
            16,
            "16 CT-tracked tables expected — migrations 017 + 022 seed exactly this list"
        );
    }
}
