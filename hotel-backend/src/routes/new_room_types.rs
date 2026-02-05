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
    let mut conn = state.new_pool.get().await?;

    let offset = (params.page - 1) * params.limit;
    let sort_order = params
        .sort_order
        .as_ref()
        .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
        .unwrap_or("ASC");

    // Map frontend column names to SQL columns
    let order_by_column = match params.sort_by.as_deref() {
        Some("typeCode") => "Type_Code",
        Some("typeName") => "Type_Name",
        Some("typeBasePrice") => "Type_Base_Price",
        Some("typeSortOrder") => "Type_Sort_Order",
        _ => "Type_Sort_Order",
    };

    // Build WHERE conditions
    let where_clause = if params.active_only.unwrap_or(false) {
        "WHERE Type_Active = 1"
    } else {
        ""
    };

    // Count query
    let count_query = format!(
        "SELECT COUNT(*) as total FROM HT_Room_Types {}",
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
            Type_ID,
            Type_Code,
            Type_Name,
            Type_Name_En,
            Type_Description,
            Type_Base_Price,
            Type_Max_Guests,
            Type_Bed_Type,
            Type_Size_SQM,
            Type_Amenities,
            Type_Sort_Order,
            Type_Active,
            Type_Created_At,
            Type_Updated_At
        FROM HT_Room_Types
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

    let room_types: Vec<RoomType> = rows
        .iter()
        .map(|row| RoomType {
            id: row.get::<i32, _>("Type_ID").unwrap_or(0),
            type_code: row.get::<&str, _>("Type_Code").unwrap_or_default().to_string(),
            type_name: row.get::<&str, _>("Type_Name").unwrap_or_default().to_string(),
            type_name_en: row.get::<&str, _>("Type_Name_En").map(String::from),
            type_description: row.get::<&str, _>("Type_Description").map(String::from),
            type_base_price: row.get::<f64, _>("Type_Base_Price"),
            type_max_guests: row.get::<i32, _>("Type_Max_Guests"),
            type_bed_type: row.get::<&str, _>("Type_Bed_Type").map(String::from),
            type_size_sqm: row.get::<f64, _>("Type_Size_SQM"),
            type_amenities: row.get::<&str, _>("Type_Amenities").map(String::from),
            type_sort_order: row.get::<i32, _>("Type_Sort_Order"),
            type_active: row.get::<bool, _>("Type_Active").unwrap_or(true),
            created_at: row.get::<NaiveDateTime, _>("Type_Created_At"),
            updated_at: row.get::<NaiveDateTime, _>("Type_Updated_At"),
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
    let mut conn = state.new_pool.get().await?;

    let rows = conn
        .query(
            r#"
            SELECT
                Type_ID,
                Type_Code,
                Type_Name,
                Type_Name_En,
                Type_Description,
                Type_Base_Price,
                Type_Max_Guests,
                Type_Bed_Type,
                Type_Size_SQM,
                Type_Amenities,
                Type_Sort_Order,
                Type_Active,
                Type_Created_At,
                Type_Updated_At
            FROM HT_Room_Types
            WHERE Type_ID = @P1
            "#,
            &[&type_id],
        )
        .await?
        .into_first_result()
        .await?;

    let row = rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Room type not found".to_string()))?;

    let room_type = RoomType {
        id: row.get::<i32, _>("Type_ID").unwrap_or(0),
        type_code: row.get::<&str, _>("Type_Code").unwrap_or_default().to_string(),
        type_name: row.get::<&str, _>("Type_Name").unwrap_or_default().to_string(),
        type_name_en: row.get::<&str, _>("Type_Name_En").map(String::from),
        type_description: row.get::<&str, _>("Type_Description").map(String::from),
        type_base_price: row.get::<f64, _>("Type_Base_Price"),
        type_max_guests: row.get::<i32, _>("Type_Max_Guests"),
        type_bed_type: row.get::<&str, _>("Type_Bed_Type").map(String::from),
        type_size_sqm: row.get::<f64, _>("Type_Size_SQM"),
        type_amenities: row.get::<&str, _>("Type_Amenities").map(String::from),
        type_sort_order: row.get::<i32, _>("Type_Sort_Order"),
        type_active: row.get::<bool, _>("Type_Active").unwrap_or(true),
        created_at: row.get::<NaiveDateTime, _>("Type_Created_At"),
        updated_at: row.get::<NaiveDateTime, _>("Type_Updated_At"),
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

    let mut conn = state.new_pool.get().await?;

    // Check for duplicate type code
    let check_rows = conn
        .query(
            "SELECT Type_ID FROM HT_Room_Types WHERE Type_Code = @P1",
            &[&type_code],
        )
        .await?
        .into_first_result()
        .await?;

    if !check_rows.is_empty() {
        return Err(ApiError::BadRequest("Type code already exists".to_string()));
    }

    let type_active = body.type_active.unwrap_or(true);
    let type_sort_order = body.type_sort_order.unwrap_or(0);
    let type_max_guests = body.type_max_guests.unwrap_or(2);

    let rows = conn
        .query(
            r#"
            INSERT INTO HT_Room_Types (
                Type_Code,
                Type_Name,
                Type_Name_En,
                Type_Description,
                Type_Base_Price,
                Type_Max_Guests,
                Type_Bed_Type,
                Type_Size_SQM,
                Type_Amenities,
                Type_Sort_Order,
                Type_Active
            )
            OUTPUT INSERTED.Type_ID
            VALUES (@P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8, @P9, @P10, @P11)
            "#,
            &[
                &type_code,
                &type_name,
                &body.type_name_en.as_deref(),
                &body.type_description.as_deref(),
                &body.type_base_price,
                &type_max_guests,
                &body.type_bed_type.as_deref(),
                &body.type_size_sqm,
                &body.type_amenities.as_deref(),
                &type_sort_order,
                &type_active,
            ],
        )
        .await?
        .into_first_result()
        .await?;

    let id = rows
        .first()
        .and_then(|r| r.get::<i32, _>("Type_ID"))
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

    let mut conn = state.new_pool.get().await?;

    // Check for duplicate type code (excluding current type)
    let check_rows = conn
        .query(
            "SELECT Type_ID FROM HT_Room_Types WHERE Type_Code = @P1 AND Type_ID != @P2",
            &[&type_code, &type_id],
        )
        .await?
        .into_first_result()
        .await?;

    if !check_rows.is_empty() {
        return Err(ApiError::BadRequest("Type code already exists".to_string()));
    }

    let type_active = body.type_active.unwrap_or(true);
    let type_sort_order = body.type_sort_order.unwrap_or(0);
    let type_max_guests = body.type_max_guests.unwrap_or(2);

    let result = conn
        .execute(
            r#"
            UPDATE HT_Room_Types
            SET Type_Code = @P1,
                Type_Name = @P2,
                Type_Name_En = @P3,
                Type_Description = @P4,
                Type_Base_Price = @P5,
                Type_Max_Guests = @P6,
                Type_Bed_Type = @P7,
                Type_Size_SQM = @P8,
                Type_Amenities = @P9,
                Type_Sort_Order = @P10,
                Type_Active = @P11,
                Type_Updated_At = GETDATE()
            WHERE Type_ID = @P12
            "#,
            &[
                &type_code,
                &type_name,
                &body.type_name_en.as_deref(),
                &body.type_description.as_deref(),
                &body.type_base_price,
                &type_max_guests,
                &body.type_bed_type.as_deref(),
                &body.type_size_sqm,
                &body.type_amenities.as_deref(),
                &type_sort_order,
                &type_active,
                &type_id,
            ],
        )
        .await?;

    if result.total() == 0 {
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
    let mut conn = state.new_pool.get().await?;

    // Check if room type is in use by any rooms
    let usage_rows = conn
        .query(
            "SELECT COUNT(*) as count FROM HT_Rooms_New WHERE Room_Type_ID = @P1",
            &[&type_id],
        )
        .await?
        .into_first_result()
        .await?;

    let usage_count: i32 = usage_rows
        .first()
        .and_then(|r| r.get::<i32, _>("count"))
        .unwrap_or(0);

    if usage_count > 0 {
        return Err(ApiError::BadRequest(format!(
            "Cannot delete room type: {} room(s) are using this type",
            usage_count
        )));
    }

    let result = conn
        .execute(
            "DELETE FROM HT_Room_Types WHERE Type_ID = @P1",
            &[&type_id],
        )
        .await?;

    if result.total() == 0 {
        return Err(ApiError::NotFound("Room type not found".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Room type deleted successfully".to_string(),
        id: Some(type_id),
    }))
}
