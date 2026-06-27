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

use super::mode::{AppState, Branch};
use crate::error::{ApiError, ApiResult};
use crate::models::Pagination;

/// Resolve the canonical pool for a branch. HF Hotel reads/writes `new_pool`,
/// HF Ville reads/writes `ville_pool`; `All` is not meaningful for room-type
/// master ops (a per-database SERIAL `type_id` collides across the two logical
/// DBs) so it defaults to HF Hotel. HF Ville mutations are gated upstream by
/// `ville_write_guard`.
///
/// ## Legacy writeback — deliberately a TODO (Task #51)
///
/// Room-type master edits are **canonical-only** for now. The nearest legacy
/// table is `HT_SET_RoomType` (`id, id_full, name, Room_PriceA/B/C` — cheatsheet
/// §`HT_SET_RoomType`), but (a) iHOTEL edits it with a destructive
/// delete-then-reinsert (cheatsheet §1471 "master-data edit"), (b) our
/// `ht_room_types` shape (type_code / name_en / max_guests / bed_type / size)
/// does not map 1:1 onto it, and (c) nothing in `sync/mappers/` mirrors it
/// inbound, so there is no back-population anchor. Wiring a writeback would
/// require byte-shape verification we can't derive from the docs. The
/// price-bearing dimension iHOTEL actually consumes (`HT_Rooms_Price`) IS
/// mirrored — edit it via the rate-tier endpoints, which DO write back.
fn room_type_pool(state: &AppState, branch: Branch) -> ApiResult<&crate::db::PgPool> {
    // Delegate to the unified per-site write chokepoint.
    state.write_pool(Some(branch))
}

/// Branch selector for the single-room-type ops (get/create/update/delete).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomTypeBranchQuery {
    pub branch: Option<Branch>,
}

/// Room type from HT_Room_Types table
///
/// JSON field naming: `rename_all = "camelCase"` is the default, but
/// the frontend (`app/room-types/page.tsx`, `app/rates/page.tsx`,
/// `components/forms/RoomTypeForm.tsx`) drops the `type_` SQL-column
/// prefix on most fields (`basePrice`, `description`, `maxGuests`,
/// `bedType`, `sizeSqm`, `active`) while keeping it on the identifier
/// triple (`typeCode`, `typeName`, `typeNameEn`). Per-field `rename`
/// pins the API contract to match that convention; without it the
/// camelCase default produces `typeBasePrice` etc. and every consumer
/// reads `undefined`, crashing `.toLocaleString()` in the table render.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomType {
    pub id: i32,
    pub type_code: String,
    pub type_name: String,
    pub type_name_en: Option<String>,
    #[serde(rename = "description")]
    pub type_description: Option<String>,
    #[serde(rename = "basePrice")]
    pub type_base_price: Option<f64>,
    #[serde(rename = "maxGuests")]
    pub type_max_guests: Option<i32>,
    #[serde(rename = "bedType")]
    pub type_bed_type: Option<String>,
    #[serde(rename = "sizeSqm")]
    pub type_size_sqm: Option<f64>,
    #[serde(rename = "amenities")]
    pub type_amenities: Option<String>,
    #[serde(rename = "sortOrder")]
    pub type_sort_order: Option<i32>,
    #[serde(rename = "active")]
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
    /// Branch selector: 'hfhotel' (default) | 'hfville'. Selects which logical
    /// PG database the room-type list is read from.
    pub branch: Option<Branch>,
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

/// Request body for creating/updating room type. Field names mirror
/// the response shape — see [`RoomType`] for the rationale.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateRoomTypeRequest {
    pub type_code: String,
    pub type_name: String,
    pub type_name_en: Option<String>,
    #[serde(rename = "description")]
    pub type_description: Option<String>,
    #[serde(rename = "basePrice")]
    pub type_base_price: Option<f64>,
    #[serde(rename = "maxGuests")]
    pub type_max_guests: Option<i32>,
    #[serde(rename = "bedType")]
    pub type_bed_type: Option<String>,
    #[serde(rename = "sizeSqm")]
    pub type_size_sqm: Option<f64>,
    #[serde(rename = "amenities")]
    pub type_amenities: Option<String>,
    #[serde(rename = "sortOrder")]
    pub type_sort_order: Option<i32>,
    #[serde(rename = "active")]
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
    let pool = room_type_pool(&state, params.branch.unwrap_or_default())?;

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

    // Count query (DYNAMIC - uses format!)
    let count_query = format!(
        "SELECT COUNT(*)::int as total FROM ht_room_types {}",
        where_clause
    );

    let count_rows = sqlx::query(sqlx::AssertSqlSafe(&*count_query)).fetch_all(pool).await?;

    let total: i32 = count_rows
        .first()
        .map(|r| r.try_get::<i32, _>("total").unwrap_or(0))
        .unwrap_or(0);

    // Data query (DYNAMIC - uses format!)
    let data_query = format!(
        r#"
        SELECT
            type_id,
            type_code,
            type_name,
            type_name_en,
            type_description,
            type_base_price::float8 as type_base_price,
            type_max_guests,
            type_bed_type,
            type_size_sqm::float8 as type_size_sqm,
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

    let rows = sqlx::query(sqlx::AssertSqlSafe(&*data_query)).fetch_all(pool).await?;

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
    Query(bq): Query<RoomTypeBranchQuery>,
) -> ApiResult<Json<RoomTypeResponse>> {
    let pool = room_type_pool(&state, bq.branch.unwrap_or_default())?;

    let rec = sqlx::query!(
        r#"SELECT type_id, type_code, type_name, type_name_en, type_description,
            type_base_price::float8 as type_base_price, type_max_guests, type_bed_type,
            type_size_sqm::float8 as type_size_sqm, type_amenities, type_sort_order,
            type_active, type_created_at, type_updated_at
        FROM ht_room_types WHERE type_id = $1"#,
        type_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Room type not found".to_string()))?;

    let room_type = RoomType {
        id: rec.type_id,
        type_code: rec.type_code,
        type_name: rec.type_name,
        type_name_en: rec.type_name_en,
        type_description: rec.type_description,
        type_base_price: rec.type_base_price,
        type_max_guests: rec.type_max_guests,
        type_bed_type: rec.type_bed_type,
        type_size_sqm: rec.type_size_sqm,
        type_amenities: rec.type_amenities,
        type_sort_order: rec.type_sort_order,
        type_active: rec.type_active.unwrap_or(true),
        created_at: rec.type_created_at,
        updated_at: rec.type_updated_at,
    };

    Ok(Json(RoomTypeResponse {
        success: true,
        room_type,
    }))
}

/// POST /api/new/room-types - Create room type
pub async fn create_room_type(
    State(state): State<AppState>,
    Query(bq): Query<RoomTypeBranchQuery>,
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

    // Canonical-only (see `room_type_pool` doc — legacy HT_SET_RoomType
    // writeback is a deliberate TODO pending byte-shape verification).
    let pool = room_type_pool(&state, bq.branch.unwrap_or_default())?;

    // Check for duplicate type code
    let existing = sqlx::query!(
        "SELECT type_id FROM ht_room_types WHERE type_code = $1",
        type_code
    )
    .fetch_optional(pool)
    .await?;

    if existing.is_some() {
        return Err(ApiError::BadRequest("Type code already exists".to_string()));
    }

    let type_active = body.type_active.unwrap_or(true);
    let type_sort_order = body.type_sort_order.unwrap_or(0);
    let type_max_guests = body.type_max_guests.unwrap_or(2);

    let rec = sqlx::query!(
        r#"INSERT INTO ht_room_types (type_code, type_name, type_name_en, type_description, type_base_price, type_max_guests, type_bed_type, type_size_sqm, type_amenities, type_sort_order, type_active)
        VALUES ($1, $2, $3, $4, $5::float8, $6, $7, $8::float8, $9, $10, $11)
        RETURNING type_id"#,
        type_code,
        type_name,
        body.type_name_en.as_deref(),
        body.type_description.as_deref(),
        body.type_base_price,
        type_max_guests,
        body.type_bed_type.as_deref(),
        body.type_size_sqm,
        body.type_amenities.as_deref(),
        type_sort_order,
        type_active
    )
    .fetch_one(pool)
    .await?;

    let id = rec.type_id;

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
    Query(bq): Query<RoomTypeBranchQuery>,
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

    // Canonical-only (legacy HT_SET_RoomType writeback is a TODO — see
    // `room_type_pool`). The price dimension iHOTEL reads lives in
    // HT_Rooms_Price; edit that via the rate-tier endpoints.
    let pool = room_type_pool(&state, bq.branch.unwrap_or_default())?;

    // Check for duplicate type code (excluding current type)
    let existing = sqlx::query!(
        "SELECT type_id FROM ht_room_types WHERE type_code = $1 AND type_id != $2",
        type_code,
        type_id
    )
    .fetch_optional(pool)
    .await?;

    if existing.is_some() {
        return Err(ApiError::BadRequest("Type code already exists".to_string()));
    }

    let type_active = body.type_active.unwrap_or(true);
    let type_sort_order = body.type_sort_order.unwrap_or(0);
    let type_max_guests = body.type_max_guests.unwrap_or(2);

    let result = sqlx::query!(
        r#"UPDATE ht_room_types SET type_code = $1, type_name = $2, type_name_en = $3, type_description = $4, type_base_price = $5::float8, type_max_guests = $6, type_bed_type = $7, type_size_sqm = $8::float8, type_amenities = $9, type_sort_order = $10, type_active = $11, type_updated_at = NOW()
        WHERE type_id = $12"#,
        type_code,
        type_name,
        body.type_name_en.as_deref(),
        body.type_description.as_deref(),
        body.type_base_price,
        type_max_guests,
        body.type_bed_type.as_deref(),
        body.type_size_sqm,
        body.type_amenities.as_deref(),
        type_sort_order,
        type_active,
        type_id
    )
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
    Query(bq): Query<RoomTypeBranchQuery>,
) -> ApiResult<Json<MutationResponse>> {
    let pool = room_type_pool(&state, bq.branch.unwrap_or_default())?;

    // Check if room type is in use by any rooms
    let rec = sqlx::query!(
        "SELECT COUNT(*)::int as count FROM ht_rooms_new WHERE room_type_id = $1",
        type_id
    )
    .fetch_one(pool)
    .await?;

    let usage_count = rec.count.unwrap_or(0);

    if usage_count > 0 {
        return Err(ApiError::BadRequest(format!(
            "Cannot delete room type: {} room(s) are using this type",
            usage_count
        )));
    }

    let result = sqlx::query!(
        "DELETE FROM ht_room_types WHERE type_id = $1",
        type_id
    )
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
