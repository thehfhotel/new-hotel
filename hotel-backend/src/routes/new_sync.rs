//! Sync status API routes
//!
//! - GET /api/new/sync/status - Get sync health and last sync times

use axum::{extract::State, Json};
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::error::ApiResult;
use crate::routes::mode::AppState;

/// Sync status for a single entity type
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncEntityStatus {
    pub entity_type: String,
    pub last_sync_at: Option<NaiveDateTime>,
    pub records_synced: i32,
    pub records_added: i32,
    pub records_updated: i32,
    pub records_unchanged: i32,
    pub sync_duration_ms: i32,
    pub last_error: Option<String>,
    pub last_error_at: Option<NaiveDateTime>,
    pub consecutive_failures: i32,
    pub health: String,
}

/// Overall sync status response
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SyncStatusResponse {
    pub success: bool,
    pub sync_enabled: bool,
    pub overall_health: String,
    pub entities: Vec<SyncEntityStatus>,
}

/// GET /api/new/sync/status - Get sync health and last sync times
pub async fn get_sync_status(
    State(state): State<AppState>,
) -> ApiResult<Json<SyncStatusResponse>> {
    let pool = &state.new_pool;

    let sync_enabled = std::env::var("SYNC_ENABLED")
        .map(|v| v != "false")
        .unwrap_or(true);

    let rows = sqlx::query(
        r#"
        SELECT
            entity_type,
            last_sync_at,
            records_synced,
            records_added,
            records_updated,
            records_unchanged,
            sync_duration_ms,
            last_error,
            last_error_at,
            consecutive_failures
        FROM sync_status
        ORDER BY entity_type
        "#,
    )
    .fetch_all(pool)
    .await;

    let entities: Vec<SyncEntityStatus> = match rows {
        Ok(rows) => {
            use sqlx::Row;
            rows.iter()
                .map(|row| {
                    let consecutive_failures: i32 =
                        row.try_get("consecutive_failures").unwrap_or(0);
                    let last_sync_at: Option<NaiveDateTime> =
                        row.try_get("last_sync_at").unwrap_or(None);

                    // Health logic: healthy if synced within last 15 minutes and no failures
                    let health = if !sync_enabled {
                        "disabled".to_string()
                    } else if consecutive_failures >= 3 {
                        "error".to_string()
                    } else if last_sync_at.is_none() {
                        "pending".to_string()
                    } else if consecutive_failures > 0 {
                        "warning".to_string()
                    } else {
                        // Check if last sync is stale (>15 min)
                        let now = chrono::Utc::now().naive_utc();
                        let stale_threshold = chrono::Duration::minutes(15);
                        if let Some(last) = last_sync_at {
                            if now - last > stale_threshold {
                                "stale".to_string()
                            } else {
                                "healthy".to_string()
                            }
                        } else {
                            "pending".to_string()
                        }
                    };

                    SyncEntityStatus {
                        entity_type: row.try_get("entity_type").unwrap_or_default(),
                        last_sync_at,
                        records_synced: row.try_get("records_synced").unwrap_or(0),
                        records_added: row.try_get("records_added").unwrap_or(0),
                        records_updated: row.try_get("records_updated").unwrap_or(0),
                        records_unchanged: row.try_get("records_unchanged").unwrap_or(0),
                        sync_duration_ms: row.try_get("sync_duration_ms").unwrap_or(0),
                        last_error: row.try_get("last_error").unwrap_or(None),
                        last_error_at: row.try_get("last_error_at").unwrap_or(None),
                        consecutive_failures,
                        health,
                    }
                })
                .collect()
        }
        Err(_) => Vec::new(),
    };

    // Overall health: worst of all entities
    let overall_health = if !sync_enabled {
        "disabled".to_string()
    } else if entities.iter().any(|e| e.health == "error") {
        "error".to_string()
    } else if entities.iter().any(|e| e.health == "warning" || e.health == "stale") {
        "warning".to_string()
    } else if entities.iter().all(|e| e.health == "healthy") {
        "healthy".to_string()
    } else {
        "pending".to_string()
    };

    Ok(Json(SyncStatusResponse {
        success: true,
        sync_enabled,
        overall_health,
        entities,
    }))
}
