//! Maintenance Request System API routes for HotelNew database.
//!
//! ## RETIRED as an intake (wave-5, 2026-08-16) — reads only
//!
//! The PMS's own แจ้งซ่อม flow duplicated the Housekeeping ops app, which the
//! owner made the system of record for maintenance WORK ORDERS. The three
//! write routes now answer `410 Gone` with a Thai body naming the housekeeping
//! app, and the PMS Kanban UI that drove them is deleted. Same disposition as
//! `ht_hk_broken_reports` (retired 2026-08-11): the table and the GETs stay so
//! existing history remains readable, and nothing silently disappears.
//!
//! `ht_maintenance_requests` held **0 rows at both properties** when this
//! landed (verified read-only against production, both `hotelnew` and
//! `hotelville`), so there was nothing to migrate and history-only is a
//! complete answer rather than a compromise.
//!
//! Categories:
//! - GET /api/maintenance/categories - List all categories
//!
//! Requests:
//! - GET /api/maintenance/requests - List requests with filters (status, room, priority)
//! - GET /api/maintenance/requests/:id - Get single request
//! - ~~POST /api/maintenance/requests~~ - **410 Gone**
//! - ~~PUT /api/maintenance/requests/:id~~ - **410 Gone**
//! - ~~PUT /api/maintenance/requests/:id/status~~ - **410 Gone**

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Row;

use super::mode::{AppState, Branch};
use crate::error::{ApiError, ApiResult};
use crate::models::Pagination;

// ============================================================================
// Data Structures
// ============================================================================

/// Maintenance category
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceCategory {
    pub id: i32,
    pub name: String,
    pub name_en: Option<String>,
    pub priority: i32,
    pub active: bool,
}

/// Maintenance request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaintenanceRequest {
    pub id: i32,
    pub request_no: String,
    pub room_id: i32,
    pub room_no: Option<String>,
    pub category_id: i32,
    pub category_name: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub priority: i32,
    pub status: String,
    pub assigned_to: Option<String>,
    pub started_at: Option<NaiveDateTime>,
    pub completed_at: Option<NaiveDateTime>,
    pub resolution: Option<String>,
    pub cost: Option<f64>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Response for categories list
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoriesResponse {
    pub success: bool,
    pub data: Vec<MaintenanceCategory>,
    pub total: i32,
}

/// Query parameters for requests list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestsQuery {
    pub status: Option<String>,
    pub room_id: Option<i32>,
    pub category_id: Option<i32>,
    pub priority: Option<i32>,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
    /// Branch selector: 'hfhotel' | 'hfville' | 'all'. Resolves the per-site pool.
    pub branch: Option<Branch>,
}

fn default_page() -> i32 {
    1
}
fn default_limit() -> i32 {
    50
}

/// Branch selector for maintenance handlers without a list query. `branchFetch`
/// appends `?branch=`; absent ⇒ HF Hotel.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchQuery {
    pub branch: Option<Branch>,
}

/// Resolve the per-site canonical pool via the unified write chokepoint
/// (`Branch::Hfville` → hotelville, else the primary pool).
fn resolve_pool(state: &AppState, branch: Option<Branch>) -> ApiResult<&crate::db::PgPool> {
    state.write_pool(branch)
}

/// Response for requests list
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestsResponse {
    pub success: bool,
    pub data: Vec<MaintenanceRequest>,
    pub pagination: Pagination,
}

/// Response for single request
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestResponse {
    pub success: bool,
    pub request: MaintenanceRequest,
}

/// Body of a `410 Gone` answer from a retired write route. Same
/// `{success:false, error}` shape `ApiError`'s `IntoResponse` emits, so a
/// client's existing error handling needs no special case. Mirrors
/// `routes::hk::GoneResponse` — the established precedent for a retired
/// endpoint in this codebase (`ApiError` has no `Gone` variant, and adding one
/// for two call sites would be the bigger change).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GoneResponse {
    pub success: bool,
    pub error: String,
}

/// The one Thai sentence every retired maintenance write answers with. Names
/// the app that took over, so a receptionist reading a stale tab knows where to
/// go rather than just that something broke.
const RETIRED_MESSAGE: &str =
    "ระบบแจ้งซ่อมย้ายไปที่แอปแม่บ้านแล้ว: แจ้งซ่อมและติดตามงานที่ housekeeping.thehfhotel.org \
     (ข้อมูลเดิมยังดูย้อนหลังได้ในระบบนี้)";

/// The shared `410` answer. `410` not `404` is deliberate: the resource existed
/// and is permanently gone, which tells a stale cached client to STOP retrying
/// rather than treat it as a transient routing error.
fn retired() -> (StatusCode, Json<GoneResponse>) {
    (
        StatusCode::GONE,
        Json(GoneResponse {
            success: false,
            error: RETIRED_MESSAGE.to_string(),
        }),
    )
}

// ============================================================================
// Category Endpoints
// ============================================================================

/// GET /api/new/maintenance/categories - List all categories
pub async fn list_categories(
    State(state): State<AppState>,
    Query(query): Query<BranchQuery>,
) -> ApiResult<Json<CategoriesResponse>> {
    let pool = resolve_pool(&state, query.branch)?;

    let rows = sqlx::query(
        r#"
            SELECT
                mcat_id,
                mcat_name,
                mcat_name_en,
                mcat_priority,
                mcat_active
            FROM ht_maintenance_categories
            WHERE mcat_active = true
            ORDER BY mcat_priority DESC, mcat_name ASC
            "#,
    )
    .fetch_all(pool)
    .await?;

    let categories: Vec<MaintenanceCategory> = rows
        .iter()
        .map(|row| MaintenanceCategory {
            id: row.try_get::<i32, _>("mcat_id").unwrap_or(0),
            name: row.try_get::<String, _>("mcat_name").unwrap_or_default(),
            name_en: row.try_get::<String, _>("mcat_name_en").ok(),
            priority: row.try_get::<i32, _>("mcat_priority").unwrap_or(2),
            active: row.try_get::<bool, _>("mcat_active").unwrap_or(true),
        })
        .collect();

    let total = categories.len() as i32;

    Ok(Json(CategoriesResponse {
        success: true,
        data: categories,
        total,
    }))
}

// ============================================================================
// Request Endpoints
// ============================================================================

/// GET /api/new/maintenance/requests - List requests with filters
pub async fn list_requests(
    State(state): State<AppState>,
    Query(params): Query<RequestsQuery>,
) -> ApiResult<Json<RequestsResponse>> {
    let pool = resolve_pool(&state, params.branch)?;

    let offset = (params.page - 1) * params.limit;

    // Build WHERE conditions. The `status` filter is parameterized (sqlx bind)
    // because it accepts arbitrary user input; the integer filters are safely
    // formatted in-place since `i32` cannot encode SQL syntax.
    let mut conditions: Vec<String> = Vec::new();
    let mut next_param_index: i32 = 1;

    if params.status.is_some() {
        conditions.push(format!("r.mreq_status = ${}", next_param_index));
        next_param_index += 1;
    }

    if let Some(room_id) = params.room_id {
        conditions.push(format!("r.mreq_room_id = {}", room_id));
    }

    if let Some(category_id) = params.category_id {
        conditions.push(format!("r.mreq_category_id = {}", category_id));
    }

    if let Some(priority) = params.priority {
        conditions.push(format!("r.mreq_priority = {}", priority));
    }

    // Silence unused-assignment warning if no further binds are added.
    let _ = next_param_index;

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count query
    let count_query = format!(
        "SELECT COUNT(*)::int as total FROM ht_maintenance_requests r {}",
        where_clause
    );

    let count_q = sqlx::query(sqlx::AssertSqlSafe(&*count_query));
    let count_q = match &params.status {
        Some(s) => count_q.bind(s),
        None => count_q,
    };
    let count_rows = count_q.fetch_all(pool).await?;

    let total: i32 = count_rows
        .first()
        .and_then(|r| r.try_get::<i32, _>("total").ok())
        .unwrap_or(0);

    // Data query
    let data_query = format!(
        r#"
        SELECT
            r.mreq_id,
            r.mreq_no,
            r.mreq_room_id,
            rm.room_no,
            r.mreq_category_id,
            c.mcat_name,
            r.mreq_title,
            r.mreq_description,
            r.mreq_priority,
            r.mreq_status,
            r.mreq_assigned_to,
            r.mreq_started_at,
            r.mreq_completed_at,
            r.mreq_resolution,
            r.mreq_cost,
            r.mreq_created_at,
            r.mreq_updated_at
        FROM ht_maintenance_requests r
        LEFT JOIN ht_rooms_new rm ON r.mreq_room_id = rm.room_id
        LEFT JOIN ht_maintenance_categories c ON r.mreq_category_id = c.mcat_id
        {}
        ORDER BY
            CASE r.mreq_status
                WHEN 'open' THEN 1
                WHEN 'in_progress' THEN 2
                WHEN 'completed' THEN 3
                WHEN 'cancelled' THEN 4
            END,
            r.mreq_priority DESC,
            r.mreq_created_at DESC
        LIMIT {} OFFSET {}
        "#,
        where_clause, params.limit, offset
    );

    let data_q = sqlx::query(sqlx::AssertSqlSafe(&*data_query));
    let data_q = match &params.status {
        Some(s) => data_q.bind(s),
        None => data_q,
    };
    let rows = data_q.fetch_all(pool).await?;

    let requests: Vec<MaintenanceRequest> = rows
        .iter()
        .map(|row| MaintenanceRequest {
            id: row.try_get::<i32, _>("mreq_id").unwrap_or(0),
            request_no: row.try_get::<String, _>("mreq_no").unwrap_or_default(),
            room_id: row.try_get::<i32, _>("mreq_room_id").unwrap_or(0),
            room_no: row.try_get::<String, _>("room_no").ok(),
            category_id: row.try_get::<i32, _>("mreq_category_id").unwrap_or(0),
            category_name: row.try_get::<String, _>("mcat_name").ok(),
            title: row.try_get::<String, _>("mreq_title").unwrap_or_default(),
            description: row.try_get::<String, _>("mreq_description").ok(),
            priority: row.try_get::<i32, _>("mreq_priority").unwrap_or(2),
            status: row
                .try_get::<String, _>("mreq_status")
                .unwrap_or_else(|_| "open".to_string()),
            assigned_to: row.try_get::<String, _>("mreq_assigned_to").ok(),
            started_at: row.try_get::<NaiveDateTime, _>("mreq_started_at").ok(),
            completed_at: row.try_get::<NaiveDateTime, _>("mreq_completed_at").ok(),
            resolution: row.try_get::<String, _>("mreq_resolution").ok(),
            cost: row.try_get::<f64, _>("mreq_cost").ok(),
            created_at: row.try_get::<NaiveDateTime, _>("mreq_created_at").ok(),
            updated_at: row.try_get::<NaiveDateTime, _>("mreq_updated_at").ok(),
        })
        .collect();

    Ok(Json(RequestsResponse {
        success: true,
        data: requests,
        pagination: Pagination::new(params.page, params.limit, total),
    }))
}

/// POST /api/maintenance/requests — **RETIRED (410 Gone)**.
///
/// Maintenance intake is the Housekeeping ops app's job now (owner decision,
/// wave-5): it owns the work-order lifecycle, and running a second intake here
/// meant two queues nobody reconciled. Takes NO extractors on purpose — a
/// stale client's body is never parsed or validated, so it cannot 400 on its
/// way to being told the endpoint is gone.
pub async fn retired_create_request() -> (StatusCode, Json<GoneResponse>) {
    retired()
}

/// GET /api/new/maintenance/requests/:id - Get a single maintenance request
pub async fn get_request(
    State(state): State<AppState>,
    Path(request_id): Path<i32>,
    Query(query): Query<BranchQuery>,
) -> ApiResult<Json<RequestResponse>> {
    let pool = resolve_pool(&state, query.branch)?;

    let rows = sqlx::query(
        r#"
            SELECT
                r.mreq_id,
                r.mreq_no,
                r.mreq_room_id,
                rm.room_no,
                r.mreq_category_id,
                c.mcat_name,
                r.mreq_title,
                r.mreq_description,
                r.mreq_priority,
                r.mreq_status,
                r.mreq_assigned_to,
                r.mreq_started_at,
                r.mreq_completed_at,
                r.mreq_resolution,
                r.mreq_cost,
                r.mreq_created_at,
                r.mreq_updated_at
            FROM ht_maintenance_requests r
            LEFT JOIN ht_rooms_new rm ON r.mreq_room_id = rm.room_id
            LEFT JOIN ht_maintenance_categories c ON r.mreq_category_id = c.mcat_id
            WHERE r.mreq_id = $1
            "#,
    )
    .bind(&request_id)
    .fetch_all(pool)
    .await?;

    let row = rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Maintenance request not found".to_string()))?;

    let request = MaintenanceRequest {
        id: row.try_get::<i32, _>("mreq_id").unwrap_or(0),
        request_no: row.try_get::<String, _>("mreq_no").unwrap_or_default(),
        room_id: row.try_get::<i32, _>("mreq_room_id").unwrap_or(0),
        room_no: row.try_get::<String, _>("room_no").ok(),
        category_id: row.try_get::<i32, _>("mreq_category_id").unwrap_or(0),
        category_name: row.try_get::<String, _>("mcat_name").ok(),
        title: row.try_get::<String, _>("mreq_title").unwrap_or_default(),
        description: row.try_get::<String, _>("mreq_description").ok(),
        priority: row.try_get::<i32, _>("mreq_priority").unwrap_or(2),
        status: row
            .try_get::<String, _>("mreq_status")
            .unwrap_or_else(|_| "open".to_string()),
        assigned_to: row.try_get::<String, _>("mreq_assigned_to").ok(),
        started_at: row.try_get::<NaiveDateTime, _>("mreq_started_at").ok(),
        completed_at: row.try_get::<NaiveDateTime, _>("mreq_completed_at").ok(),
        resolution: row.try_get::<String, _>("mreq_resolution").ok(),
        cost: row.try_get::<f64, _>("mreq_cost").ok(),
        created_at: row.try_get::<NaiveDateTime, _>("mreq_created_at").ok(),
        updated_at: row.try_get::<NaiveDateTime, _>("mreq_updated_at").ok(),
    };

    Ok(Json(RequestResponse {
        success: true,
        request,
    }))
}

/// PUT /api/maintenance/requests/:id — **RETIRED (410 Gone)**.
///
/// See [`retired_create_request`]. Editing a request is part of the work-order
/// lifecycle that moved wholesale; `GET /api/maintenance/requests/:id` still
/// serves the row for history.
pub async fn retired_update_request() -> (StatusCode, Json<GoneResponse>) {
    retired()
}

/// PUT /api/maintenance/requests/:id/status — **RETIRED (410 Gone)**.
///
/// See [`retired_create_request`]. This was the Kanban board's drag-to-column
/// action; the board is deleted and the column it wrote lives in the
/// housekeeping app.
pub async fn retired_update_request_status() -> (StatusCode, Json<GoneResponse>) {
    retired()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// All three retired writes answer 410 (permanently gone), not 404
    /// (transient/unknown) — so a stale cached tab stops retrying instead of
    /// looking like a routing blip.
    #[tokio::test]
    async fn every_retired_write_answers_410() {
        for (name, (status, Json(body))) in [
            ("create", retired_create_request().await),
            ("update", retired_update_request().await),
            ("status", retired_update_request_status().await),
        ] {
            assert_eq!(status, StatusCode::GONE, "{name} must be 410");
            assert!(!body.success, "{name}");
        }
    }

    /// The body must NAME the app that took over, in Thai. A bare "gone" tells
    /// a receptionist that something broke; this tells her where to go.
    #[test]
    fn the_thai_body_names_the_housekeeping_app() {
        assert!(
            RETIRED_MESSAGE.contains("housekeeping.thehfhotel.org"),
            "{RETIRED_MESSAGE}"
        );
        assert!(RETIRED_MESSAGE.contains("แอปแม่บ้าน"), "{RETIRED_MESSAGE}");
        // History stays readable via the surviving GETs — say so, or the
        // message reads as "your data is gone".
        assert!(RETIRED_MESSAGE.contains("ย้อนหลัง"), "{RETIRED_MESSAGE}");
    }

    /// The wire shape matches `ApiError`'s `IntoResponse` (`success` + `error`),
    /// so existing client error handling needs no special case.
    #[test]
    fn the_gone_body_matches_the_house_error_shape() {
        let (_, Json(body)) = retired();
        let json = serde_json::to_string(&body).expect("serializes");
        assert!(json.contains("\"success\":false"), "{json}");
        assert!(json.contains("\"error\""), "{json}");
    }
}
