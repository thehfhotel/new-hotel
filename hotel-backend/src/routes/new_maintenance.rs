//! Maintenance Request System API routes for HotelNew database
//!
//! Categories:
//! - GET /api/new/maintenance/categories - List all categories
//!
//! Requests:
//! - GET /api/new/maintenance/requests - List requests with filters (status, room, priority)
//! - POST /api/new/maintenance/requests - Create request (generates MReq_No as MR-YYMM-NNNN)
//! - GET /api/new/maintenance/requests/:id - Get single request
//! - PUT /api/new/maintenance/requests/:id - Update request
//! - PUT /api/new/maintenance/requests/:id/status - Quick status update

use axum::{
    extract::{Path, Query, State},
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

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 50 }

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

/// Request for creating a maintenance request
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequestInput {
    pub room_id: i32,
    pub category_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<i32>,
    pub assigned_to: Option<String>,
}

/// Request for updating a maintenance request
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRequestInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<i32>,
    pub status: Option<String>,
    pub assigned_to: Option<String>,
    pub resolution: Option<String>,
    pub cost: Option<f64>,
}

/// Request for quick status update
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusUpdateInput {
    pub status: String,
}

/// Generic mutation response
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_no: Option<String>,
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
    let count_q = match &params.status { Some(s) => count_q.bind(s), None => count_q };
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
    let data_q = match &params.status { Some(s) => data_q.bind(s), None => data_q };
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
            status: row.try_get::<String, _>("mreq_status").unwrap_or_else(|_| "open".to_string()),
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

/// POST /api/new/maintenance/requests - Create a new maintenance request
pub async fn create_request(
    State(state): State<AppState>,
    Query(query): Query<BranchQuery>,
    Json(body): Json<CreateRequestInput>,
) -> ApiResult<Json<MutationResponse>> {
    let title = body.title.trim();
    if title.is_empty() {
        return Err(ApiError::BadRequest("Title is required".to_string()));
    }

    let pool = resolve_pool(&state, query.branch)?;

    // Generate request number: MR-YYMM-NNNN
    let seq_rows = sqlx::query("SELECT nextval('sq_maintenance_no')::int AS seq_num")
        .fetch_all(pool)
        .await?;

    let seq_num: i32 = seq_rows
        .first()
        .and_then(|r| r.try_get::<i32, _>("seq_num").ok())
        .unwrap_or(1);

    let now = chrono::Local::now();
    let request_no = format!(
        "MR-{:02}{:02}-{:04}",
        now.format("%y"),
        now.format("%m"),
        seq_num
    );

    let priority = body.priority.unwrap_or(2);

    let rows = sqlx::query(
            r#"
            INSERT INTO ht_maintenance_requests (
                mreq_no, mreq_room_id, mreq_category_id, mreq_title,
                mreq_description, mreq_priority, mreq_assigned_to
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING mreq_id
            "#,
        )
        .bind(&request_no.as_str())
        .bind(&body.room_id)
        .bind(&body.category_id)
        .bind(&title)
        .bind(&body.description.as_deref())
        .bind(&priority)
        .bind(&body.assigned_to.as_deref())
        .fetch_all(pool)
        .await?;

    let id = rows
        .first()
        .and_then(|r| r.try_get::<i32, _>("mreq_id").ok())
        .ok_or_else(|| ApiError::Internal("Failed to create maintenance request".to_string()))?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Maintenance request created successfully".to_string(),
        id: Some(id),
        request_no: Some(request_no),
    }))
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
        status: row.try_get::<String, _>("mreq_status").unwrap_or_else(|_| "open".to_string()),
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

/// PUT /api/new/maintenance/requests/:id - Update a maintenance request
pub async fn update_request(
    State(state): State<AppState>,
    Path(request_id): Path<i32>,
    Query(query): Query<BranchQuery>,
    Json(body): Json<UpdateRequestInput>,
) -> ApiResult<Json<MutationResponse>> {
    let pool = resolve_pool(&state, query.branch)?;

    // Build dynamic UPDATE query with parameterized placeholders. Each text/
    // numeric column that takes a user-supplied value reserves the next `$N`
    // slot; the values are bound below in the same order via sqlx, so they
    // cannot be interpreted as SQL. Integer columns are still safe to inline
    // because `i32`/`f64` cannot encode SQL syntax.
    let mut set_parts: Vec<String> = Vec::new();
    let mut next_param_index: i32 = 1;

    if body.title.is_some() {
        set_parts.push(format!("mreq_title = ${}", next_param_index));
        next_param_index += 1;
    }

    if body.description.is_some() {
        set_parts.push(format!("mreq_description = ${}", next_param_index));
        next_param_index += 1;
    }

    if let Some(priority) = body.priority {
        set_parts.push(format!("mreq_priority = {}", priority));
    }

    if let Some(ref status) = body.status {
        set_parts.push(format!("mreq_status = ${}", next_param_index));
        next_param_index += 1;

        // Automatically set timestamps based on status
        if status == "in_progress" {
            set_parts.push("mreq_started_at = NOW()".to_string());
        } else if status == "completed" {
            set_parts.push("mreq_completed_at = NOW()".to_string());
        }
    }

    if body.assigned_to.is_some() {
        set_parts.push(format!("mreq_assigned_to = ${}", next_param_index));
        next_param_index += 1;
    }

    if body.resolution.is_some() {
        set_parts.push(format!("mreq_resolution = ${}", next_param_index));
        next_param_index += 1;
    }

    if let Some(cost) = body.cost {
        set_parts.push(format!("mreq_cost = {}", cost));
    }

    if set_parts.is_empty() {
        return Err(ApiError::BadRequest("No fields to update".to_string()));
    }

    set_parts.push("mreq_updated_at = NOW()".to_string());

    // mreq_id is the LAST bind so it occupies the final `$N` slot regardless
    // of which optional columns appeared above.
    let id_param_index = next_param_index;

    let update_query = format!(
        "UPDATE ht_maintenance_requests SET {} WHERE mreq_id = ${}",
        set_parts.join(", "),
        id_param_index
    );

    // Bind values in the same order the placeholders were assigned above, then
    // bind `request_id` last to match the WHERE-clause `$id_param_index`.
    let q = sqlx::query(sqlx::AssertSqlSafe(&*update_query));
    let q = match &body.title { Some(v) => q.bind(v), None => q };
    let q = match &body.description { Some(v) => q.bind(v), None => q };
    let q = match &body.status { Some(v) => q.bind(v), None => q };
    let q = match &body.assigned_to { Some(v) => q.bind(v), None => q };
    let q = match &body.resolution { Some(v) => q.bind(v), None => q };
    let q = q.bind(request_id);

    let result = q.execute(pool).await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Maintenance request not found".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Maintenance request updated successfully".to_string(),
        id: Some(request_id),
        request_no: None,
    }))
}

/// PUT /api/new/maintenance/requests/:id/status - Quick status update
pub async fn update_request_status(
    State(state): State<AppState>,
    Path(request_id): Path<i32>,
    Query(query): Query<BranchQuery>,
    Json(body): Json<StatusUpdateInput>,
) -> ApiResult<Json<MutationResponse>> {
    let status = body.status.trim().to_lowercase();

    // Validate status
    let valid_statuses = ["open", "in_progress", "completed", "cancelled"];
    if !valid_statuses.contains(&status.as_str()) {
        return Err(ApiError::BadRequest(format!(
            "Invalid status '{}'. Must be one of: {}",
            status,
            valid_statuses.join(", ")
        )));
    }

    let pool = resolve_pool(&state, query.branch)?;

    // Build update query with automatic timestamp handling
    let update_query = match status.as_str() {
        "in_progress" => format!(
            "UPDATE ht_maintenance_requests SET mreq_status = '{}', mreq_started_at = NOW(), mreq_updated_at = NOW() WHERE mreq_id = {}",
            status, request_id
        ),
        "completed" => format!(
            "UPDATE ht_maintenance_requests SET mreq_status = '{}', mreq_completed_at = NOW(), mreq_updated_at = NOW() WHERE mreq_id = {}",
            status, request_id
        ),
        _ => format!(
            "UPDATE ht_maintenance_requests SET mreq_status = '{}', mreq_updated_at = NOW() WHERE mreq_id = {}",
            status, request_id
        ),
    };

    let result = sqlx::query(sqlx::AssertSqlSafe(&*update_query)).execute(pool).await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Maintenance request not found".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: format!("Status updated to '{}'", status),
        id: Some(request_id),
        request_no: None,
    }))
}
