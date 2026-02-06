//! Room Types API routes for HotelNew database
//!
//! - GET /api/new/room-types - List all room types
//! - GET /api/new/room-types/:id - Get single room type
//! - POST /api/new/room-types - Create room type
//! - PUT /api/new/room-types/:id - Update room type
//! - DELETE /api/new/room-types/:id - Delete room type

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Row;

use super::mode::AppState;
use crate::error::{ApiError, ApiResult};
use crate::models::Pagination;

/// Room type from HT_Room_Types table
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomType {
    pub id: i32,
    pub type_code: String,
    pub type_name: String,
    pub type_name_en: Option<String>,
    pub type_description: Option<String>,
    pub type_base_price: Option<f64>,
    pub type_max_guests: Option<i32>,
    pub type_bed_type: Option<String>,
    pub type_size_sqm: Option<f64>,
    pub type_amenities: Option<String>,
    pub type_sort_order: Option<i32>,
    pub type_active: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// Query parameters for room types list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomTypesQuery {
    pub active_only: Option<bool>,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 50 }

/// Response for room types list
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomTypesResponse {
    pub success: bool,
    pub data: Vec<RoomType>,
    pub pagination: Pagination,
}

/// Response for single room type
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomTypeResponse {
    pub success: bool,
    pub room_type: RoomType,
}

/// Request body for creating/updating room type
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateRoomTypeRequest {
    pub type_code: String,
    pub type_name: String,
    pub type_name_en: Option<String>,
    pub type_description: Option<String>,
    pub type_base_price: Option<f64>,
    pub type_max_guests: Option<i32>,
    pub type_bed_type: Option<String>,
    pub type_size_sqm: Option<f64>,
    pub type_amenities: Option<String>,
    pub type_sort_order: Option<i32>,
    pub type_active: Option<bool>,
}

/// Response for create/update/delete operations
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

/// GET /api/new/room-types - List all room types
pub async fn list_room_types(
    State(state): State<AppState>,
    Query(params): Query<RoomTypesQuery>,
) -> ApiResult<Json<RoomTypesResponse>> {
    let pool = &state.new_pool;

    let offset = (params.page - 1) * params.limit;
    let sort_order = params
        .sort_order
        .as_ref()
        .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
        .unwrap_or("ASC");

    // Map frontend column names to SQL columns
    let order_by_column = match params.sort_by.as_deref() {
        Some("typeCode") => "type_code",
        Some("typeName") => "type_name",
        Some("typeBasePrice") => "type_base_price",
        Some("typeSortOrder") => "type_sort_order",
        _ => "type_sort_order",
    };

    // Build WHERE conditions
    let where_clause = if params.active_only.unwrap_or(false) {
        "WHERE type_active = true"
    } else {
        ""
    };

    // Count query
    let count_query = format!(
        "SELECT COUNT(*)::int as total FROM ht_room_types {}",
        where_clause
    );

    let count_rows = sqlx::query(&count_query).fetch_all(pool).await?;

    let total: i32 = count_rows
        .first()
        .map(|r| r.try_get::<i32, _>("total").unwrap_or(0))
        .unwrap_or(0);

    // Data query
    let data_query = format!(
        r#"
        SELECT
            type_id,
            type_code,
            type_name,
            type_name_en,
            type_description,
            type_base_price,
            type_max_guests,
            type_bed_type,
            type_size_sqm,
            type_amenities,
            type_sort_order,
            type_active,
            type_created_at,
            type_updated_at
        FROM ht_room_types
        {}
        ORDER BY {} {}
        LIMIT {} OFFSET {}
        "#,
        where_clause, order_by_column, sort_order, params.limit, offset
    );

    let rows = sqlx::query(&data_query).fetch_all(pool).await?;

    let room_types: Vec<RoomType> = rows
        .iter()
        .map(|row| RoomType {
            id: row.try_get::<i32, _>("type_id").unwrap_or(0),
            type_code: row.try_get::<String, _>("type_code").unwrap_or_default(),
            type_name: row.try_get::<String, _>("type_name").unwrap_or_default(),
            type_name_en: row.try_get::<String, _>("type_name_en").ok(),
            type_description: row.try_get::<String, _>("type_description").ok(),
            type_base_price: row.try_get::<f64, _>("type_base_price").ok(),
            type_max_guests: row.try_get::<i32, _>("type_max_guests").ok(),
            type_bed_type: row.try_get::<String, _>("type_bed_type").ok(),
            type_size_sqm: row.try_get::<f64, _>("type_size_sqm").ok(),
            type_amenities: row.try_get::<String, _>("type_amenities").ok(),
            type_sort_order: row.try_get::<i32, _>("type_sort_order").ok(),
            type_active: row.try_get::<bool, _>("type_active").unwrap_or(true),
            created_at: row.try_get::<NaiveDateTime, _>("type_created_at").ok(),
            updated_at: row.try_get::<NaiveDateTime, _>("type_updated_at").ok(),
        })
        .collect();

    Ok(Json(RoomTypesResponse {
        success: true,
        data: room_types,
        pagination: Pagination::new(params.page, params.limit, total),
    }))
}

/// GET /api/new/room-types/:id - Get single room type
pub async fn get_room_type(
    State(state): State<AppState>,
    Path(type_id): Path<i32>,
) -> ApiResult<Json<RoomTypeResponse>> {
    let pool = &state.new_pool;

    let rows = sqlx::query(
            r#"
            SELECT
                type_id,
                type_code,
                type_name,
                type_name_en,
                type_description,
                type_base_price,
                type_max_guests,
                type_bed_type,
                type_size_sqm,
                type_amenities,
                type_sort_order,
                type_active,
                type_created_at,
                type_updated_at
            FROM ht_room_types
            WHERE type_id = $1
            "#,
        )
        .bind(&type_id)
        .fetch_all(pool)
        .await?;

    let row = rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Room type not found".to_string()))?;

    let room_type = RoomType {
        id: row.try_get::<i32, _>("type_id").unwrap_or(0),
        type_code: row.try_get::<String, _>("type_code").unwrap_or_default(),
        type_name: row.try_get::<String, _>("type_name").unwrap_or_default(),
        type_name_en: row.try_get::<String, _>("type_name_en").ok(),
        type_description: row.try_get::<String, _>("type_description").ok(),
        type_base_price: row.try_get::<f64, _>("type_base_price").ok(),
        type_max_guests: row.try_get::<i32, _>("type_max_guests").ok(),
        type_bed_type: row.try_get::<String, _>("type_bed_type").ok(),
        type_size_sqm: row.try_get::<f64, _>("type_size_sqm").ok(),
        type_amenities: row.try_get::<String, _>("type_amenities").ok(),
        type_sort_order: row.try_get::<i32, _>("type_sort_order").ok(),
        type_active: row.try_get::<bool, _>("type_active").unwrap_or(true),
        created_at: row.try_get::<NaiveDateTime, _>("type_created_at").ok(),
        updated_at: row.try_get::<NaiveDateTime, _>("type_updated_at").ok(),
    };

    Ok(Json(RoomTypeResponse {
        success: true,
        room_type,
    }))
}

/// POST /api/new/room-types - Create room type
pub async fn create_room_type(
    State(state): State<AppState>,
    Json(body): Json<CreateUpdateRoomTypeRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let type_code = body.type_code.trim();
    if type_code.is_empty() {
        return Err(ApiError::BadRequest("Type code is required".to_string()));
    }

    let type_name = body.type_name.trim();
    if type_name.is_empty() {
        return Err(ApiError::BadRequest("Type name is required".to_string()));
    }

    let pool = &state.new_pool;

    // Check for duplicate type code
    let check_rows = sqlx::query(
            "SELECT type_id FROM ht_room_types WHERE type_code = $1",
        )
        .bind(&type_code)
        .fetch_all(pool)
        .await?;

    if !check_rows.is_empty() {
        return Err(ApiError::BadRequest("Type code already exists".to_string()));
    }

    let type_active = body.type_active.unwrap_or(true);
    let type_sort_order = body.type_sort_order.unwrap_or(0);
    let type_max_guests = body.type_max_guests.unwrap_or(2);

    let rows = sqlx::query(
            r#"
            INSERT INTO ht_room_types (
                type_code,
                type_name,
                type_name_en,
                type_description,
                type_base_price,
                type_max_guests,
                type_bed_type,
                type_size_sqm,
                type_amenities,
                type_sort_order,
                type_active
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING type_id
            "#,
        )
        .bind(&type_code)
        .bind(&type_name)
        .bind(&body.type_name_en.as_deref())
        .bind(&body.type_description.as_deref())
        .bind(&body.type_base_price)
        .bind(&type_max_guests)
        .bind(&body.type_bed_type.as_deref())
        .bind(&body.type_size_sqm)
        .bind(&body.type_amenities.as_deref())
        .bind(&type_sort_order)
        .bind(&type_active)
        .fetch_all(pool)
        .await?;

    let id = rows
        .first()
        .map(|r| r.try_get::<i32, _>("type_id").ok())
        .flatten()
        .ok_or_else(|| ApiError::Internal("Failed to create room type".to_string()))?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Room type created successfully".to_string(),
        id: Some(id),
    }))
}

/// PUT /api/new/room-types/:id - Update room type
pub async fn update_room_type(
    State(state): State<AppState>,
    Path(type_id): Path<i32>,
    Json(body): Json<CreateUpdateRoomTypeRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let type_code = body.type_code.trim();
    if type_code.is_empty() {
        return Err(ApiError::BadRequest("Type code is required".to_string()));
    }

    let type_name = body.type_name.trim();
    if type_name.is_empty() {
        return Err(ApiError::BadRequest("Type name is required".to_string()));
    }

    let pool = &state.new_pool;

    // Check for duplicate type code (excluding current type)
    let check_rows = sqlx::query(
            "SELECT type_id FROM ht_room_types WHERE type_code = $1 AND type_id != $2",
        )
        .bind(&type_code)
        .bind(&type_id)
        .fetch_all(pool)
        .await?;

    if !check_rows.is_empty() {
        return Err(ApiError::BadRequest("Type code already exists".to_string()));
    }

    let type_active = body.type_active.unwrap_or(true);
    let type_sort_order = body.type_sort_order.unwrap_or(0);
    let type_max_guests = body.type_max_guests.unwrap_or(2);

    let result = sqlx::query(
            r#"
            UPDATE ht_room_types
            SET type_code = $1,
                type_name = $2,
                type_name_en = $3,
                type_description = $4,
                type_base_price = $5,
                type_max_guests = $6,
                type_bed_type = $7,
                type_size_sqm = $8,
                type_amenities = $9,
                type_sort_order = $10,
                type_active = $11,
                type_updated_at = NOW()
            WHERE type_id = $12
            "#,
        )
        .bind(&type_code)
        .bind(&type_name)
        .bind(&body.type_name_en.as_deref())
        .bind(&body.type_description.as_deref())
        .bind(&body.type_base_price)
        .bind(&type_max_guests)
        .bind(&body.type_bed_type.as_deref())
        .bind(&body.type_size_sqm)
        .bind(&body.type_amenities.as_deref())
        .bind(&type_sort_order)
        .bind(&type_active)
        .bind(&type_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Room type not found".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Room type updated successfully".to_string(),
        id: Some(type_id),
    }))
}

/// DELETE /api/new/room-types/:id - Delete room type
pub async fn delete_room_type(
    State(state): State<AppState>,
    Path(type_id): Path<i32>,
) -> ApiResult<Json<MutationResponse>> {
    let pool = &state.new_pool;

    // Check if room type is in use by any rooms
    let usage_rows = sqlx::query(
            "SELECT COUNT(*)::int as count FROM ht_rooms_new WHERE room_type_id = $1",
        )
        .bind(&type_id)
        .fetch_all(pool)
        .await?;

    let usage_count: i32 = usage_rows
        .first()
        .map(|r| r.try_get::<i32, _>("count").unwrap_or(0))
        .unwrap_or(0);

    if usage_count > 0 {
        return Err(ApiError::BadRequest(format!(
            "Cannot delete room type: {} room(s) are using this type",
            usage_count
        )));
    }

    let result = sqlx::query(
            "DELETE FROM ht_room_types WHERE type_id = $1",
        )
        .bind(&type_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Room type not found".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Room type deleted successfully".to_string(),
        id: Some(type_id),
    }))
}
