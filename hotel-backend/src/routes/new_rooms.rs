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
use sqlx::Row; // Keep for dynamic queries in list_rooms

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
    let pool = &state.new_pool;

    let offset = (params.page - 1) * params.limit;
    let sort_order = params
        .sort_order
        .as_ref()
        .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
        .unwrap_or("ASC");

    // Map frontend column names to SQL columns
    let order_by_column = match params.sort_by.as_deref() {
        Some("roomNo") => "r.room_no",
        Some("floor") => "r.room_floor",
        Some("status") => "r.room_status",
        Some("roomType") => "rt.type_name",
        _ => "r.room_no",
    };

    // Build WHERE conditions
    let mut conditions: Vec<String> = Vec::new();

    if let Some(ref status) = params.status {
        let escaped = status.replace('\'', "''");
        conditions.push(format!("r.room_status = '{}'", escaped));
    }

    if let Some(type_id) = params.room_type_id {
        conditions.push(format!("r.room_type_id = {}", type_id));
    }

    if let Some(floor) = params.floor {
        conditions.push(format!("r.room_floor = {}", floor));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count query
    let count_query = format!(
        "SELECT COUNT(*)::int as total FROM ht_rooms_new r {}",
        where_clause
    );

    let count_rows = sqlx::query(&count_query)
        .fetch_all(pool)
        .await?;

    let total: i32 = count_rows
        .first()
        .map(|r| r.try_get::<i32, _>("total").unwrap_or(0))
        .unwrap_or(0);

    // Data query with LEFT JOIN to room types
    let data_query = format!(
        r#"
        SELECT
            r.room_id,
            r.room_no,
            r.room_type_id,
            rt.type_name,
            r.room_floor,
            r.room_status,
            r.room_clean,
            r.room_maintenance,
            r.room_price_weekday,
            r.room_price_weekend,
            r.room_price_special,
            r.room_notes,
            r.created_at,
            r.updated_at
        FROM ht_rooms_new r
        LEFT JOIN ht_room_types rt ON r.room_type_id = rt.type_id
        {}
        ORDER BY {} {}
        LIMIT {} OFFSET {}
        "#,
        where_clause, order_by_column, sort_order, params.limit, offset
    );

    let rows = sqlx::query(&data_query)
        .fetch_all(pool)
        .await?;

    let rooms: Vec<NewRoom> = rows
        .iter()
        .map(|row| NewRoom {
            id: row.try_get::<i32, _>("room_id").unwrap_or(0),
            room_no: row.try_get::<String, _>("room_no").unwrap_or_default(),
            room_type_id: row.try_get::<i32, _>("room_type_id").ok(),
            room_type_name: row.try_get::<String, _>("type_name").ok(),
            floor: row.try_get::<i32, _>("room_floor").ok(),
            status: row.try_get::<String, _>("room_status").unwrap_or_else(|_| "available".to_string()),
            is_clean: row.try_get::<bool, _>("room_clean").unwrap_or(true),
            is_maintenance: row.try_get::<bool, _>("room_maintenance").unwrap_or(false),
            price_weekday: row.try_get::<f64, _>("room_price_weekday").ok(),
            price_weekend: row.try_get::<f64, _>("room_price_weekend").ok(),
            price_special: row.try_get::<f64, _>("room_price_special").ok(),
            notes: row.try_get::<String, _>("room_notes").ok(),
            created_at: row.try_get::<NaiveDateTime, _>("created_at").ok(),
            updated_at: row.try_get::<NaiveDateTime, _>("updated_at").ok(),
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
    let pool = &state.new_pool;

    let rec = sqlx::query!(
        r#"SELECT
            r.room_id, r.room_no, r.room_type_id, rt.type_name as "type_name?",
            r.room_floor, r.room_status, r.room_clean, r.room_maintenance,
            r.room_price_weekday::float8 as "room_price_weekday?",
            r.room_price_weekend::float8 as "room_price_weekend?",
            r.room_price_special::float8 as "room_price_special?",
            r.room_notes, r.created_at, r.updated_at
        FROM ht_rooms_new r
        LEFT JOIN ht_room_types rt ON r.room_type_id = rt.type_id
        WHERE r.room_id = $1"#,
        room_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Room not found".to_string()))?;

    let room = NewRoom {
        id: rec.room_id,
        room_no: rec.room_no,
        room_type_id: rec.room_type_id,
        room_type_name: rec.type_name,
        floor: rec.room_floor,
        status: rec.room_status.unwrap_or_else(|| "available".to_string()),
        is_clean: rec.room_clean.unwrap_or(true),
        is_maintenance: rec.room_maintenance.unwrap_or(false),
        price_weekday: rec.room_price_weekday,
        price_weekend: rec.room_price_weekend,
        price_special: rec.room_price_special,
        notes: rec.room_notes,
        created_at: rec.created_at,
        updated_at: rec.updated_at,
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

    let pool = &state.new_pool;

    // Check for duplicate room number
    let existing = sqlx::query!(
        "SELECT room_id FROM ht_rooms_new WHERE room_no = $1",
        room_no
    )
    .fetch_optional(pool)
    .await?;

    if existing.is_some() {
        return Err(ApiError::BadRequest("Room number already exists".to_string()));
    }

    let status = body.status.as_deref().unwrap_or("available");
    let is_clean = body.is_clean.unwrap_or(true);
    let is_maintenance = body.is_maintenance.unwrap_or(false);

    let rec = sqlx::query!(
        r#"INSERT INTO ht_rooms_new (room_no, room_type_id, room_floor, room_status, room_clean, room_maintenance, room_price_weekday, room_price_weekend, room_price_special, room_notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7::float8, $8::float8, $9::float8, $10)
        RETURNING room_id"#,
        room_no,
        body.room_type_id,
        body.floor,
        status,
        is_clean,
        is_maintenance,
        body.price_weekday,
        body.price_weekend,
        body.price_special,
        body.notes.as_deref()
    )
    .fetch_one(pool)
    .await?;

    let id = rec.room_id;

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

    let pool = &state.new_pool;

    // Check for duplicate room number (excluding current room)
    let existing = sqlx::query!(
        "SELECT room_id FROM ht_rooms_new WHERE room_no = $1 AND room_id != $2",
        room_no,
        room_id
    )
    .fetch_optional(pool)
    .await?;

    if existing.is_some() {
        return Err(ApiError::BadRequest("Room number already exists".to_string()));
    }

    let status = body.status.as_deref().unwrap_or("available");
    let is_clean = body.is_clean.unwrap_or(true);
    let is_maintenance = body.is_maintenance.unwrap_or(false);

    let result = sqlx::query!(
        r#"UPDATE ht_rooms_new SET room_no = $1, room_type_id = $2, room_floor = $3, room_status = $4, room_clean = $5, room_maintenance = $6, room_price_weekday = $7::float8, room_price_weekend = $8::float8, room_price_special = $9::float8, room_notes = $10, updated_at = NOW()
        WHERE room_id = $11"#,
        room_no,
        body.room_type_id,
        body.floor,
        status,
        is_clean,
        is_maintenance,
        body.price_weekday,
        body.price_weekend,
        body.price_special,
        body.notes.as_deref(),
        room_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
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

    let pool = &state.new_pool;

    let result = sqlx::query!(
        r#"UPDATE ht_rooms_new SET room_status = $1, updated_at = NOW() WHERE room_id = $2"#,
        status.as_str(),
        room_id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Room not found".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: format!("Room status updated to '{}'", status),
        id: Some(room_id),
    }))
}
