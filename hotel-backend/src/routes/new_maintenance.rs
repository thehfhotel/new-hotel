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

use super::mode::AppState;
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
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 50 }

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
) -> ApiResult<Json<CategoriesResponse>> {
    let mut conn = state.new_pool.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT
                MCat_ID,
                MCat_Name,
                MCat_Name_En,
                MCat_Priority,
                MCat_Active
            FROM HT_Maintenance_Categories
            WHERE MCat_Active = 1
            ORDER BY MCat_Priority DESC, MCat_Name ASC
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let categories: Vec<MaintenanceCategory> = rows
        .iter()
        .map(|row| MaintenanceCategory {
            id: row.get::<i32, _>("MCat_ID").unwrap_or(0),
            name: row.get::<&str, _>("MCat_Name").unwrap_or_default().to_string(),
            name_en: row.get::<&str, _>("MCat_Name_En").map(String::from),
            priority: row.get::<i32, _>("MCat_Priority").unwrap_or(2),
            active: row.get::<bool, _>("MCat_Active").unwrap_or(true),
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
    let mut conn = state.new_pool.get().await?;

    let offset = (params.page - 1) * params.limit;

    // Build WHERE conditions
    let mut conditions: Vec<String> = Vec::new();

    if let Some(ref status) = params.status {
        let escaped = status.replace('\'', "''");
        conditions.push(format!("r.MReq_Status = '{}'", escaped));
    }

    if let Some(room_id) = params.room_id {
        conditions.push(format!("r.MReq_Room_ID = {}", room_id));
    }

    if let Some(category_id) = params.category_id {
        conditions.push(format!("r.MReq_Category_ID = {}", category_id));
    }

    if let Some(priority) = params.priority {
        conditions.push(format!("r.MReq_Priority = {}", priority));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count query
    let count_query = format!(
        "SELECT COUNT(*) as total FROM HT_Maintenance_Requests r {}",
        where_clause
    );

    let count_rows = conn
        .simple_query(&count_query)
        .await?
        .into_first_result()
        .await?;

    let total: i32 = count_rows
        .first()
        .and_then(|r| r.get::<i32, _>("total"))
        .unwrap_or(0);

    // Data query
    let data_query = format!(
        r#"
        SELECT
            r.MReq_ID,
            r.MReq_No,
            r.MReq_Room_ID,
            rm.Room_No,
            r.MReq_Category_ID,
            c.MCat_Name,
            r.MReq_Title,
            r.MReq_Description,
            r.MReq_Priority,
            r.MReq_Status,
            r.MReq_Assigned_To,
            r.MReq_Started_At,
            r.MReq_Completed_At,
            r.MReq_Resolution,
            r.MReq_Cost,
            r.MReq_Created_At,
            r.MReq_Updated_At
        FROM HT_Maintenance_Requests r
        LEFT JOIN HT_Rooms_New rm ON r.MReq_Room_ID = rm.Room_ID
        LEFT JOIN HT_Maintenance_Categories c ON r.MReq_Category_ID = c.MCat_ID
        {}
        ORDER BY
            CASE r.MReq_Status
                WHEN 'open' THEN 1
                WHEN 'in_progress' THEN 2
                WHEN 'completed' THEN 3
                WHEN 'cancelled' THEN 4
            END,
            r.MReq_Priority DESC,
            r.MReq_Created_At DESC
        OFFSET {} ROWS FETCH NEXT {} ROWS ONLY
        "#,
        where_clause, offset, params.limit
    );

    let rows = conn
        .simple_query(&data_query)
        .await?
        .into_first_result()
        .await?;

    let requests: Vec<MaintenanceRequest> = rows
        .iter()
        .map(|row| MaintenanceRequest {
            id: row.get::<i32, _>("MReq_ID").unwrap_or(0),
            request_no: row.get::<&str, _>("MReq_No").unwrap_or_default().to_string(),
            room_id: row.get::<i32, _>("MReq_Room_ID").unwrap_or(0),
            room_no: row.get::<&str, _>("Room_No").map(String::from),
            category_id: row.get::<i32, _>("MReq_Category_ID").unwrap_or(0),
            category_name: row.get::<&str, _>("MCat_Name").map(String::from),
            title: row.get::<&str, _>("MReq_Title").unwrap_or_default().to_string(),
            description: row.get::<&str, _>("MReq_Description").map(String::from),
            priority: row.get::<i32, _>("MReq_Priority").unwrap_or(2),
            status: row.get::<&str, _>("MReq_Status").unwrap_or("open").to_string(),
            assigned_to: row.get::<&str, _>("MReq_Assigned_To").map(String::from),
            started_at: row.get::<NaiveDateTime, _>("MReq_Started_At"),
            completed_at: row.get::<NaiveDateTime, _>("MReq_Completed_At"),
            resolution: row.get::<&str, _>("MReq_Resolution").map(String::from),
            cost: row.get::<f64, _>("MReq_Cost"),
            created_at: row.get::<NaiveDateTime, _>("MReq_Created_At"),
            updated_at: row.get::<NaiveDateTime, _>("MReq_Updated_At"),
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
    Json(body): Json<CreateRequestInput>,
) -> ApiResult<Json<MutationResponse>> {
    let title = body.title.trim();
    if title.is_empty() {
        return Err(ApiError::BadRequest("Title is required".to_string()));
    }

    let mut conn = state.new_pool.get().await?;

    // Generate request number: MR-YYMM-NNNN
    let seq_rows = conn
        .simple_query("SELECT NEXT VALUE FOR SQ_Maintenance_No AS seq_num")
        .await?
        .into_first_result()
        .await?;

    let seq_num: i32 = seq_rows
        .first()
        .and_then(|r| r.get::<i32, _>("seq_num"))
        .unwrap_or(1);

    let now = chrono::Local::now();
    let request_no = format!(
        "MR-{:02}{:02}-{:04}",
        now.format("%y"),
        now.format("%m"),
        seq_num
    );

    let priority = body.priority.unwrap_or(2);

    let rows = conn
        .query(
            r#"
            INSERT INTO HT_Maintenance_Requests (
                MReq_No, MReq_Room_ID, MReq_Category_ID, MReq_Title,
                MReq_Description, MReq_Priority, MReq_Assigned_To
            )
            OUTPUT INSERTED.MReq_ID
            VALUES (@P1, @P2, @P3, @P4, @P5, @P6, @P7)
            "#,
            &[
                &request_no.as_str(),
                &body.room_id,
                &body.category_id,
                &title,
                &body.description.as_deref(),
                &priority,
                &body.assigned_to.as_deref(),
            ],
        )
        .await?
        .into_first_result()
        .await?;

    let id = rows
        .first()
        .and_then(|r| r.get::<i32, _>("MReq_ID"))
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
) -> ApiResult<Json<RequestResponse>> {
    let mut conn = state.new_pool.get().await?;

    let rows = conn
        .query(
            r#"
            SELECT
                r.MReq_ID,
                r.MReq_No,
                r.MReq_Room_ID,
                rm.Room_No,
                r.MReq_Category_ID,
                c.MCat_Name,
                r.MReq_Title,
                r.MReq_Description,
                r.MReq_Priority,
                r.MReq_Status,
                r.MReq_Assigned_To,
                r.MReq_Started_At,
                r.MReq_Completed_At,
                r.MReq_Resolution,
                r.MReq_Cost,
                r.MReq_Created_At,
                r.MReq_Updated_At
            FROM HT_Maintenance_Requests r
            LEFT JOIN HT_Rooms_New rm ON r.MReq_Room_ID = rm.Room_ID
            LEFT JOIN HT_Maintenance_Categories c ON r.MReq_Category_ID = c.MCat_ID
            WHERE r.MReq_ID = @P1
            "#,
            &[&request_id],
        )
        .await?
        .into_first_result()
        .await?;

    let row = rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Maintenance request not found".to_string()))?;

    let request = MaintenanceRequest {
        id: row.get::<i32, _>("MReq_ID").unwrap_or(0),
        request_no: row.get::<&str, _>("MReq_No").unwrap_or_default().to_string(),
        room_id: row.get::<i32, _>("MReq_Room_ID").unwrap_or(0),
        room_no: row.get::<&str, _>("Room_No").map(String::from),
        category_id: row.get::<i32, _>("MReq_Category_ID").unwrap_or(0),
        category_name: row.get::<&str, _>("MCat_Name").map(String::from),
        title: row.get::<&str, _>("MReq_Title").unwrap_or_default().to_string(),
        description: row.get::<&str, _>("MReq_Description").map(String::from),
        priority: row.get::<i32, _>("MReq_Priority").unwrap_or(2),
        status: row.get::<&str, _>("MReq_Status").unwrap_or("open").to_string(),
        assigned_to: row.get::<&str, _>("MReq_Assigned_To").map(String::from),
        started_at: row.get::<NaiveDateTime, _>("MReq_Started_At"),
        completed_at: row.get::<NaiveDateTime, _>("MReq_Completed_At"),
        resolution: row.get::<&str, _>("MReq_Resolution").map(String::from),
        cost: row.get::<f64, _>("MReq_Cost"),
        created_at: row.get::<NaiveDateTime, _>("MReq_Created_At"),
        updated_at: row.get::<NaiveDateTime, _>("MReq_Updated_At"),
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
    Json(body): Json<UpdateRequestInput>,
) -> ApiResult<Json<MutationResponse>> {
    let mut conn = state.new_pool.get().await?;

    // Build dynamic UPDATE query
    let mut set_parts: Vec<String> = Vec::new();

    if let Some(ref title) = body.title {
        let escaped = title.replace('\'', "''");
        set_parts.push(format!("MReq_Title = N'{}'", escaped));
    }

    if let Some(ref description) = body.description {
        let escaped = description.replace('\'', "''");
        set_parts.push(format!("MReq_Description = N'{}'", escaped));
    }

    if let Some(priority) = body.priority {
        set_parts.push(format!("MReq_Priority = {}", priority));
    }

    if let Some(ref status) = body.status {
        let escaped = status.replace('\'', "''");
        set_parts.push(format!("MReq_Status = '{}'", escaped));

        // Automatically set timestamps based on status
        if status == "in_progress" {
            set_parts.push("MReq_Started_At = GETDATE()".to_string());
        } else if status == "completed" {
            set_parts.push("MReq_Completed_At = GETDATE()".to_string());
        }
    }

    if let Some(ref assigned_to) = body.assigned_to {
        let escaped = assigned_to.replace('\'', "''");
        set_parts.push(format!("MReq_Assigned_To = N'{}'", escaped));
    }

    if let Some(ref resolution) = body.resolution {
        let escaped = resolution.replace('\'', "''");
        set_parts.push(format!("MReq_Resolution = N'{}'", escaped));
    }

    if let Some(cost) = body.cost {
        set_parts.push(format!("MReq_Cost = {}", cost));
    }

    if set_parts.is_empty() {
        return Err(ApiError::BadRequest("No fields to update".to_string()));
    }

    set_parts.push("MReq_Updated_At = GETDATE()".to_string());

    let update_query = format!(
        "UPDATE HT_Maintenance_Requests SET {} WHERE MReq_ID = {}",
        set_parts.join(", "),
        request_id
    );

    let result = conn.execute(&update_query, &[]).await?;

    if result.total() == 0 {
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

    let mut conn = state.new_pool.get().await?;

    // Build update query with automatic timestamp handling
    let update_query = match status.as_str() {
        "in_progress" => format!(
            "UPDATE HT_Maintenance_Requests SET MReq_Status = '{}', MReq_Started_At = GETDATE(), MReq_Updated_At = GETDATE() WHERE MReq_ID = {}",
            status, request_id
        ),
        "completed" => format!(
            "UPDATE HT_Maintenance_Requests SET MReq_Status = '{}', MReq_Completed_At = GETDATE(), MReq_Updated_At = GETDATE() WHERE MReq_ID = {}",
            status, request_id
        ),
        _ => format!(
            "UPDATE HT_Maintenance_Requests SET MReq_Status = '{}', MReq_Updated_At = GETDATE() WHERE MReq_ID = {}",
            status, request_id
        ),
    };

    let result = conn.execute(&update_query, &[]).await?;

    if result.total() == 0 {
        return Err(ApiError::NotFound("Maintenance request not found".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: format!("Status updated to '{}'", status),
        id: Some(request_id),
        request_no: None,
    }))
}
