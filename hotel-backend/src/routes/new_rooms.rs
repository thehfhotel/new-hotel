//! New Room API routes for HotelNew database
//!
//! - GET /api/new/rooms - List rooms from HT_Rooms_New
//! - GET /api/new/rooms/:id - Get single room
//! - POST /api/new/rooms - Create room
//! - PUT /api/new/rooms/:id - Update room

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::mode::AppState;
use crate::error::{ApiError, ApiResult};
use crate::models::Pagination;

/// Room from HT_Rooms_New table
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRoom {
    pub id: i32,
    pub room_no: String,
    pub room_type_id: Option<i32>,
    pub room_type_name: Option<String>,
    pub floor: Option<i32>,
    pub status: String,
    pub is_clean: bool,
    pub is_maintenance: bool,
    pub price_weekday: Option<f64>,
    pub price_weekend: Option<f64>,
    pub price_special: Option<f64>,
    pub notes: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// Query parameters for rooms list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRoomsQuery {
    pub status: Option<String>,
    pub room_type_id: Option<i32>,
    pub floor: Option<i32>,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 50 }

/// Response for rooms list
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRoomsResponse {
    pub success: bool,
    pub data: Vec<NewRoom>,
    pub pagination: Pagination,
}

/// Response for single room
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewRoomResponse {
    pub success: bool,
    pub room: NewRoom,
}

/// Request body for creating/updating room
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateRoomRequest {
    pub room_no: String,
    pub room_type_id: Option<i32>,
    pub floor: Option<i32>,
    pub status: Option<String>,
    pub is_clean: Option<bool>,
    pub is_maintenance: Option<bool>,
    pub price_weekday: Option<f64>,
    pub price_weekend: Option<f64>,
    pub price_special: Option<f64>,
    pub notes: Option<String>,
}

/// Response for create/update operations
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

/// GET /api/new/rooms - List rooms from HT_Rooms_New
pub async fn list_rooms(
    State(state): State<AppState>,
    Query(params): Query<NewRoomsQuery>,
) -> ApiResult<Json<NewRoomsResponse>> {
    let mut conn = state.new_pool.get().await?;

    let offset = (params.page - 1) * params.limit;
    let sort_order = params
        .sort_order
        .as_ref()
        .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
        .unwrap_or("ASC");

    // Map frontend column names to SQL columns
    let order_by_column = match params.sort_by.as_deref() {
        Some("roomNo") => "r.Room_No",
        Some("floor") => "r.Room_Floor",
        Some("status") => "r.Room_Status",
        Some("roomType") => "rt.Type_Name",
        _ => "r.Room_No",
    };

    // Build WHERE conditions
    let mut conditions: Vec<String> = Vec::new();

    if let Some(ref status) = params.status {
        let escaped = status.replace('\'', "''");
        conditions.push(format!("r.Room_Status = '{}'", escaped));
    }

    if let Some(type_id) = params.room_type_id {
        conditions.push(format!("r.Room_Type_ID = {}", type_id));
    }

    if let Some(floor) = params.floor {
        conditions.push(format!("r.Room_Floor = {}", floor));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count query
    let count_query = format!(
        "SELECT COUNT(*) as total FROM HT_Rooms_New r {}",
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

    // Data query with LEFT JOIN to room types
    let data_query = format!(
        r#"
        SELECT
            r.Room_ID,
            r.Room_No,
            r.Room_Type_ID,
            rt.Type_Name,
            r.Room_Floor,
            r.Room_Status,
            r.Room_Clean,
            r.Room_Maintenance,
            r.Room_Price_Weekday,
            r.Room_Price_Weekend,
            r.Room_Price_Special,
            r.Room_Notes,
            r.Created_At,
            r.Updated_At
        FROM HT_Rooms_New r
        LEFT JOIN HT_Room_Types rt ON r.Room_Type_ID = rt.Type_ID
        {}
        ORDER BY {} {}
        OFFSET {} ROWS FETCH NEXT {} ROWS ONLY
        "#,
        where_clause, order_by_column, sort_order, offset, params.limit
    );

    let rows = conn
        .simple_query(&data_query)
        .await?
        .into_first_result()
        .await?;

    let rooms: Vec<NewRoom> = rows
        .iter()
        .map(|row| NewRoom {
            id: row.get::<i32, _>("Room_ID").unwrap_or(0),
            room_no: row.get::<&str, _>("Room_No").unwrap_or_default().to_string(),
            room_type_id: row.get::<i32, _>("Room_Type_ID"),
            room_type_name: row.get::<&str, _>("Type_Name").map(String::from),
            floor: row.get::<i32, _>("Room_Floor"),
            status: row.get::<&str, _>("Room_Status").unwrap_or("available").to_string(),
            is_clean: row.get::<bool, _>("Room_Clean").unwrap_or(true),
            is_maintenance: row.get::<bool, _>("Room_Maintenance").unwrap_or(false),
            price_weekday: row.get::<f64, _>("Room_Price_Weekday"),
            price_weekend: row.get::<f64, _>("Room_Price_Weekend"),
            price_special: row.get::<f64, _>("Room_Price_Special"),
            notes: row.get::<&str, _>("Room_Notes").map(String::from),
            created_at: row.get::<NaiveDateTime, _>("Created_At"),
            updated_at: row.get::<NaiveDateTime, _>("Updated_At"),
        })
        .collect();

    Ok(Json(NewRoomsResponse {
        success: true,
        data: rooms,
        pagination: Pagination::new(params.page, params.limit, total),
    }))
}

/// GET /api/new/rooms/:id - Get single room
pub async fn get_room(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
) -> ApiResult<Json<NewRoomResponse>> {
    let mut conn = state.new_pool.get().await?;

    let rows = conn
        .query(
            r#"
            SELECT
                r.Room_ID,
                r.Room_No,
                r.Room_Type_ID,
                rt.Type_Name,
                r.Room_Floor,
                r.Room_Status,
                r.Room_Clean,
                r.Room_Maintenance,
                r.Room_Price_Weekday,
                r.Room_Price_Weekend,
                r.Room_Price_Special,
                r.Room_Notes,
                r.Created_At,
                r.Updated_At
            FROM HT_Rooms_New r
            LEFT JOIN HT_Room_Types rt ON r.Room_Type_ID = rt.Type_ID
            WHERE r.Room_ID = @P1
            "#,
            &[&room_id],
        )
        .await?
        .into_first_result()
        .await?;

    let row = rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Room not found".to_string()))?;

    let room = NewRoom {
        id: row.get::<i32, _>("Room_ID").unwrap_or(0),
        room_no: row.get::<&str, _>("Room_No").unwrap_or_default().to_string(),
        room_type_id: row.get::<i32, _>("Room_Type_ID"),
        room_type_name: row.get::<&str, _>("Type_Name").map(String::from),
        floor: row.get::<i32, _>("Room_Floor"),
        status: row.get::<&str, _>("Room_Status").unwrap_or("available").to_string(),
        is_clean: row.get::<bool, _>("Room_Clean").unwrap_or(true),
        is_maintenance: row.get::<bool, _>("Room_Maintenance").unwrap_or(false),
        price_weekday: row.get::<f64, _>("Room_Price_Weekday"),
        price_weekend: row.get::<f64, _>("Room_Price_Weekend"),
        price_special: row.get::<f64, _>("Room_Price_Special"),
        notes: row.get::<&str, _>("Room_Notes").map(String::from),
        created_at: row.get::<NaiveDateTime, _>("Created_At"),
        updated_at: row.get::<NaiveDateTime, _>("Updated_At"),
    };

    Ok(Json(NewRoomResponse {
        success: true,
        room,
    }))
}

/// POST /api/new/rooms - Create room
pub async fn create_room(
    State(state): State<AppState>,
    Json(body): Json<CreateUpdateRoomRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let room_no = body.room_no.trim();
    if room_no.is_empty() {
        return Err(ApiError::BadRequest("Room number is required".to_string()));
    }

    let mut conn = state.new_pool.get().await?;

    // Check for duplicate room number
    let check_rows = conn
        .query(
            "SELECT Room_ID FROM HT_Rooms_New WHERE Room_No = @P1",
            &[&room_no],
        )
        .await?
        .into_first_result()
        .await?;

    if !check_rows.is_empty() {
        return Err(ApiError::BadRequest("Room number already exists".to_string()));
    }

    let status = body.status.as_deref().unwrap_or("available");
    let is_clean = body.is_clean.unwrap_or(true);
    let is_maintenance = body.is_maintenance.unwrap_or(false);

    let rows = conn
        .query(
            r#"
            INSERT INTO HT_Rooms_New (
                Room_No,
                Room_Type_ID,
                Room_Floor,
                Room_Status,
                Room_Clean,
                Room_Maintenance,
                Room_Price_Weekday,
                Room_Price_Weekend,
                Room_Price_Special,
                Room_Notes
            )
            OUTPUT INSERTED.Room_ID
            VALUES (@P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8, @P9, @P10)
            "#,
            &[
                &room_no,
                &body.room_type_id,
                &body.floor,
                &status,
                &is_clean,
                &is_maintenance,
                &body.price_weekday,
                &body.price_weekend,
                &body.price_special,
                &body.notes.as_deref(),
            ],
        )
        .await?
        .into_first_result()
        .await?;

    let id = rows
        .first()
        .and_then(|r| r.get::<i32, _>("Room_ID"))
        .ok_or_else(|| ApiError::Internal("Failed to create room".to_string()))?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Room created successfully".to_string(),
        id: Some(id),
    }))
}

/// PUT /api/new/rooms/:id - Update room
pub async fn update_room(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Json(body): Json<CreateUpdateRoomRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let room_no = body.room_no.trim();
    if room_no.is_empty() {
        return Err(ApiError::BadRequest("Room number is required".to_string()));
    }

    let mut conn = state.new_pool.get().await?;

    // Check for duplicate room number (excluding current room)
    let check_rows = conn
        .query(
            "SELECT Room_ID FROM HT_Rooms_New WHERE Room_No = @P1 AND Room_ID != @P2",
            &[&room_no, &room_id],
        )
        .await?
        .into_first_result()
        .await?;

    if !check_rows.is_empty() {
        return Err(ApiError::BadRequest("Room number already exists".to_string()));
    }

    let status = body.status.as_deref().unwrap_or("available");
    let is_clean = body.is_clean.unwrap_or(true);
    let is_maintenance = body.is_maintenance.unwrap_or(false);

    let result = conn
        .execute(
            r#"
            UPDATE HT_Rooms_New
            SET Room_No = @P1,
                Room_Type_ID = @P2,
                Room_Floor = @P3,
                Room_Status = @P4,
                Room_Clean = @P5,
                Room_Maintenance = @P6,
                Room_Price_Weekday = @P7,
                Room_Price_Weekend = @P8,
                Room_Price_Special = @P9,
                Room_Notes = @P10,
                Updated_At = GETDATE()
            WHERE Room_ID = @P11
            "#,
            &[
                &room_no,
                &body.room_type_id,
                &body.floor,
                &status,
                &is_clean,
                &is_maintenance,
                &body.price_weekday,
                &body.price_weekend,
                &body.price_special,
                &body.notes.as_deref(),
                &room_id,
            ],
        )
        .await?;

    if result.total() == 0 {
        return Err(ApiError::NotFound("Room not found".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Room updated successfully".to_string(),
        id: Some(room_id),
    }))
}

/// Request body for updating room status only
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoomStatusRequest {
    pub status: String,
}

/// PATCH /api/new/rooms/:id/status - Update room status only
pub async fn update_room_status(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Json(body): Json<UpdateRoomStatusRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let status = body.status.trim().to_lowercase();

    // Validate status
    let valid_statuses = ["available", "occupied", "maintenance", "cleaning"];
    if !valid_statuses.contains(&status.as_str()) {
        return Err(ApiError::BadRequest(format!(
            "Invalid status '{}'. Must be one of: {}",
            status,
            valid_statuses.join(", ")
        )));
    }

    let mut conn = state.new_pool.get().await?;

    let result = conn
        .execute(
            r#"
            UPDATE HT_Rooms_New
            SET Room_Status = @P1,
                Updated_At = GETDATE()
            WHERE Room_ID = @P2
            "#,
            &[&status.as_str(), &room_id],
        )
        .await?;

    if result.total() == 0 {
        return Err(ApiError::NotFound("Room not found".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: format!("Room status updated to '{}'", status),
        id: Some(room_id),
    }))
}
