//! Inventory Management API routes for HotelNew database
//!
//! Categories:
//! - GET /api/new/inventory/categories - List all categories
//! - POST /api/new/inventory/categories - Create category
//!
//! Items:
//! - GET /api/new/inventory/items - List items (with filters)
//! - POST /api/new/inventory/items - Create item
//! - GET /api/new/inventory/items/:id - Get item details
//! - PUT /api/new/inventory/items/:id - Update item
//! - DELETE /api/new/inventory/items/:id - Soft delete item
//!
//! Room Inventory:
//! - GET /api/new/inventory/rooms/:room_id - Get room's inventory
//! - PUT /api/new/inventory/rooms/:room_id - Update room's inventory
//!
//! Transactions:
//! - GET /api/new/inventory/transactions - List transactions
//! - POST /api/new/inventory/transactions - Create transaction
//!
//! Dashboard:
//! - GET /api/new/inventory/stats - Summary stats
//! - GET /api/new/inventory/low-stock - Items below minimum stock

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

// ============================================================================
// Data Structures
// ============================================================================

/// Inventory category
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub active: bool,
    pub created_at: Option<NaiveDateTime>,
}

/// Inventory item
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub category_id: Option<i32>,
    pub category_name: Option<String>,
    pub unit: String,
    pub min_stock: i32,
    pub current_stock: i32,
    pub cost: Option<f64>,
    pub active: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// Room inventory assignment
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInventoryItem {
    pub id: i32,
    pub room_id: i32,
    pub item_id: i32,
    pub item_code: Option<String>,
    pub item_name: Option<String>,
    pub quantity: i32,
    pub last_checked: Option<NaiveDateTime>,
}

/// Inventory transaction
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: i32,
    pub item_id: i32,
    pub item_code: Option<String>,
    pub item_name: Option<String>,
    pub trans_type: String,
    pub quantity: i32,
    pub room_id: Option<i32>,
    pub notes: Option<String>,
    pub date: Option<NaiveDateTime>,
    pub created_by: Option<String>,
}

/// Inventory stats for dashboard
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryStats {
    pub total_items: i32,
    pub total_categories: i32,
    pub low_stock_count: i32,
    pub out_of_stock_count: i32,
    pub total_stock_value: f64,
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Response for categories list
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoriesResponse {
    pub success: bool,
    pub data: Vec<Category>,
    pub total: i32,
}

/// Request for creating/updating category
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub active: Option<bool>,
}

/// Query parameters for items list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemsQuery {
    pub category_id: Option<i32>,
    pub low_stock: Option<bool>,
    pub search: Option<String>,
    pub active: Option<bool>,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 50 }

/// Response for items list
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemsResponse {
    pub success: bool,
    pub data: Vec<Item>,
    pub pagination: Pagination,
}

/// Response for single item
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemResponse {
    pub success: bool,
    pub item: Item,
}

/// Request for creating/updating item
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateItemRequest {
    pub code: String,
    pub name: String,
    pub category_id: Option<i32>,
    pub unit: String,
    pub min_stock: Option<i32>,
    pub current_stock: Option<i32>,
    pub cost: Option<f64>,
    pub active: Option<bool>,
}

/// Response for room inventory
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInventoryResponse {
    pub success: bool,
    pub room_id: i32,
    pub items: Vec<RoomInventoryItem>,
}

/// Request for updating room inventory
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoomInventoryRequest {
    pub items: Vec<RoomInventoryItemUpdate>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInventoryItemUpdate {
    pub item_id: i32,
    pub quantity: i32,
}

/// Query parameters for transactions list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsQuery {
    pub item_id: Option<i32>,
    #[serde(rename = "type")]
    pub trans_type: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

/// Response for transactions list
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionsResponse {
    pub success: bool,
    pub data: Vec<Transaction>,
    pub pagination: Pagination,
}

/// Request for creating transaction
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTransactionRequest {
    pub item_id: i32,
    #[serde(rename = "type")]
    pub trans_type: String,
    pub quantity: i32,
    pub room_id: Option<i32>,
    pub notes: Option<String>,
    pub created_by: Option<String>,
}

/// Response for stats
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsResponse {
    pub success: bool,
    pub stats: InventoryStats,
}

/// Response for low stock items
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LowStockResponse {
    pub success: bool,
    pub data: Vec<Item>,
    pub total: i32,
}

/// Generic mutation response
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

// ============================================================================
// Category Endpoints
// ============================================================================

/// GET /api/new/inventory/categories - List all categories
pub async fn list_categories(
    State(state): State<AppState>,
) -> ApiResult<Json<CategoriesResponse>> {
    let pool = &state.new_pool;

    let rows = sqlx::query(
            r#"
            SELECT
                cat_id,
                cat_name,
                cat_description,
                cat_active,
                cat_created
            FROM ht_inventory_categories
            ORDER BY cat_name ASC
            "#,
        )
        .fetch_all(pool)
        .await?;

    let categories: Vec<Category> = rows
        .iter()
        .map(|row| Category {
            id: row.try_get::<i32, _>("cat_id").unwrap_or(0),
            name: row.try_get::<String, _>("cat_name").unwrap_or_default(),
            description: row.try_get::<String, _>("cat_description").ok(),
            active: row.try_get::<bool, _>("cat_active").unwrap_or(true),
            created_at: row.try_get::<NaiveDateTime, _>("cat_created").ok(),
        })
        .collect();

    let total = categories.len() as i32;

    Ok(Json(CategoriesResponse {
        success: true,
        data: categories,
        total,
    }))
}

/// POST /api/new/inventory/categories - Create category
pub async fn create_category(
    State(state): State<AppState>,
    Json(body): Json<CreateCategoryRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let name = body.name.trim();
    if name.is_empty() {
        return Err(ApiError::BadRequest("Category name is required".to_string()));
    }

    let pool = &state.new_pool;

    let active = body.active.unwrap_or(true);

    let rows = sqlx::query(
            r#"
            INSERT INTO ht_inventory_categories (cat_name, cat_description, cat_active)
            VALUES ($1, $2, $3)
            RETURNING cat_id
            "#,
        )
        .bind(&name)
        .bind(&body.description.as_deref())
        .bind(&active)
        .fetch_all(pool)
        .await?;

    let id = rows
        .first()
        .and_then(|r| r.try_get::<i32, _>("cat_id").ok())
        .ok_or_else(|| ApiError::Internal("Failed to create category".to_string()))?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Category created successfully".to_string(),
        id: Some(id),
    }))
}

// ============================================================================
// Item Endpoints
// ============================================================================

/// GET /api/new/inventory/items - List items with filters
pub async fn list_items(
    State(state): State<AppState>,
    Query(params): Query<ItemsQuery>,
) -> ApiResult<Json<ItemsResponse>> {
    let pool = &state.new_pool;

    let offset = (params.page - 1) * params.limit;

    // Build WHERE conditions
    let mut conditions: Vec<String> = Vec::new();

    if let Some(cat_id) = params.category_id {
        conditions.push(format!("i.item_category_id = {}", cat_id));
    }

    if params.low_stock == Some(true) {
        conditions.push("i.item_current_stock <= i.item_min_stock".to_string());
    }

    if let Some(ref search) = params.search {
        let escaped = search.replace('\'', "''");
        conditions.push(format!(
            "(i.item_code LIKE '%{}%' OR i.item_name LIKE '%{}%')",
            escaped, escaped
        ));
    }

    if let Some(active) = params.active {
        conditions.push(format!("i.item_active = {}", if active { "true" } else { "false" }));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count query
    let count_query = format!(
        "SELECT COUNT(*)::int as total FROM ht_inventory_items i {}",
        where_clause
    );

    let count_rows = sqlx::query(&count_query)
        .fetch_all(pool)
        .await?;

    let total: i32 = count_rows
        .first()
        .and_then(|r| r.try_get::<i32, _>("total").ok())
        .unwrap_or(0);

    // Data query
    let data_query = format!(
        r#"
        SELECT
            i.item_id,
            i.item_code,
            i.item_name,
            i.item_category_id,
            c.cat_name,
            i.item_unit,
            i.item_min_stock,
            i.item_current_stock,
            i.item_cost,
            i.item_active,
            i.item_created,
            i.item_updated
        FROM ht_inventory_items i
        LEFT JOIN ht_inventory_categories c ON i.item_category_id = c.cat_id
        {}
        ORDER BY i.item_name ASC
        LIMIT {} OFFSET {}
        "#,
        where_clause, params.limit, offset
    );

    let rows = sqlx::query(&data_query)
        .fetch_all(pool)
        .await?;

    let items: Vec<Item> = rows
        .iter()
        .map(|row| Item {
            id: row.try_get::<i32, _>("item_id").unwrap_or(0),
            code: row.try_get::<String, _>("item_code").unwrap_or_default(),
            name: row.try_get::<String, _>("item_name").unwrap_or_default(),
            category_id: row.try_get::<i32, _>("item_category_id").ok(),
            category_name: row.try_get::<String, _>("cat_name").ok(),
            unit: row.try_get::<String, _>("item_unit").unwrap_or_default(),
            min_stock: row.try_get::<i32, _>("item_min_stock").unwrap_or(0),
            current_stock: row.try_get::<i32, _>("item_current_stock").unwrap_or(0),
            cost: row.try_get::<f64, _>("item_cost").ok(),
            active: row.try_get::<bool, _>("item_active").unwrap_or(true),
            created_at: row.try_get::<NaiveDateTime, _>("item_created").ok(),
            updated_at: row.try_get::<NaiveDateTime, _>("item_updated").ok(),
        })
        .collect();

    Ok(Json(ItemsResponse {
        success: true,
        data: items,
        pagination: Pagination::new(params.page, params.limit, total),
    }))
}

/// POST /api/new/inventory/items - Create item
pub async fn create_item(
    State(state): State<AppState>,
    Json(body): Json<CreateUpdateItemRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let code = body.code.trim();
    let name = body.name.trim();
    let unit = body.unit.trim();

    if code.is_empty() {
        return Err(ApiError::BadRequest("Item code is required".to_string()));
    }
    if name.is_empty() {
        return Err(ApiError::BadRequest("Item name is required".to_string()));
    }
    if unit.is_empty() {
        return Err(ApiError::BadRequest("Item unit is required".to_string()));
    }

    let pool = &state.new_pool;

    // Check for duplicate code
    let check_rows = sqlx::query(
            "SELECT item_id FROM ht_inventory_items WHERE item_code = $1",
        )
        .bind(&code)
        .fetch_all(pool)
        .await?;

    if !check_rows.is_empty() {
        return Err(ApiError::BadRequest("Item code already exists".to_string()));
    }

    let min_stock = body.min_stock.unwrap_or(0);
    let current_stock = body.current_stock.unwrap_or(0);
    let active = body.active.unwrap_or(true);

    let rows = sqlx::query(
            r#"
            INSERT INTO ht_inventory_items (
                item_code, item_name, item_category_id, item_unit,
                item_min_stock, item_current_stock, item_cost, item_active
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING item_id
            "#,
        )
        .bind(&code)
        .bind(&name)
        .bind(&body.category_id)
        .bind(&unit)
        .bind(&min_stock)
        .bind(&current_stock)
        .bind(&body.cost)
        .bind(&active)
        .fetch_all(pool)
        .await?;

    let id = rows
        .first()
        .and_then(|r| r.try_get::<i32, _>("item_id").ok())
        .ok_or_else(|| ApiError::Internal("Failed to create item".to_string()))?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Item created successfully".to_string(),
        id: Some(id),
    }))
}

/// GET /api/new/inventory/items/:id - Get item details
pub async fn get_item(
    State(state): State<AppState>,
    Path(item_id): Path<i32>,
) -> ApiResult<Json<ItemResponse>> {
    let pool = &state.new_pool;

    let rows = sqlx::query(
            r#"
            SELECT
                i.item_id,
                i.item_code,
                i.item_name,
                i.item_category_id,
                c.cat_name,
                i.item_unit,
                i.item_min_stock,
                i.item_current_stock,
                i.item_cost,
                i.item_active,
                i.item_created,
                i.item_updated
            FROM ht_inventory_items i
            LEFT JOIN ht_inventory_categories c ON i.item_category_id = c.cat_id
            WHERE i.item_id = $1
            "#,
        )
        .bind(&item_id)
        .fetch_all(pool)
        .await?;

    let row = rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Item not found".to_string()))?;

    let item = Item {
        id: row.try_get::<i32, _>("item_id").unwrap_or(0),
        code: row.try_get::<String, _>("item_code").unwrap_or_default(),
        name: row.try_get::<String, _>("item_name").unwrap_or_default(),
        category_id: row.try_get::<i32, _>("item_category_id").ok(),
        category_name: row.try_get::<String, _>("cat_name").ok(),
        unit: row.try_get::<String, _>("item_unit").unwrap_or_default(),
        min_stock: row.try_get::<i32, _>("item_min_stock").unwrap_or(0),
        current_stock: row.try_get::<i32, _>("item_current_stock").unwrap_or(0),
        cost: row.try_get::<f64, _>("item_cost").ok(),
        active: row.try_get::<bool, _>("item_active").unwrap_or(true),
        created_at: row.try_get::<NaiveDateTime, _>("item_created").ok(),
        updated_at: row.try_get::<NaiveDateTime, _>("item_updated").ok(),
    };

    Ok(Json(ItemResponse { success: true, item }))
}

/// PUT /api/new/inventory/items/:id - Update item
pub async fn update_item(
    State(state): State<AppState>,
    Path(item_id): Path<i32>,
    Json(body): Json<CreateUpdateItemRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let code = body.code.trim();
    let name = body.name.trim();
    let unit = body.unit.trim();

    if code.is_empty() {
        return Err(ApiError::BadRequest("Item code is required".to_string()));
    }
    if name.is_empty() {
        return Err(ApiError::BadRequest("Item name is required".to_string()));
    }
    if unit.is_empty() {
        return Err(ApiError::BadRequest("Item unit is required".to_string()));
    }

    let pool = &state.new_pool;

    // Check for duplicate code (excluding current item)
    let check_rows = sqlx::query(
            "SELECT item_id FROM ht_inventory_items WHERE item_code = $1 AND item_id != $2",
        )
        .bind(&code)
        .bind(&item_id)
        .fetch_all(pool)
        .await?;

    if !check_rows.is_empty() {
        return Err(ApiError::BadRequest("Item code already exists".to_string()));
    }

    let min_stock = body.min_stock.unwrap_or(0);
    let current_stock = body.current_stock.unwrap_or(0);
    let active = body.active.unwrap_or(true);

    let result = sqlx::query(
            r#"
            UPDATE ht_inventory_items
            SET item_code = $1,
                item_name = $2,
                item_category_id = $3,
                item_unit = $4,
                item_min_stock = $5,
                item_current_stock = $6,
                item_cost = $7,
                item_active = $8,
                item_updated = NOW()
            WHERE item_id = $9
            "#,
        )
        .bind(&code)
        .bind(&name)
        .bind(&body.category_id)
        .bind(&unit)
        .bind(&min_stock)
        .bind(&current_stock)
        .bind(&body.cost)
        .bind(&active)
        .bind(&item_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Item not found".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Item updated successfully".to_string(),
        id: Some(item_id),
    }))
}

/// DELETE /api/new/inventory/items/:id - Soft delete item
pub async fn delete_item(
    State(state): State<AppState>,
    Path(item_id): Path<i32>,
) -> ApiResult<Json<MutationResponse>> {
    let pool = &state.new_pool;

    let result = sqlx::query(
            "UPDATE ht_inventory_items SET item_active = false, item_updated = NOW() WHERE item_id = $1",
        )
        .bind(&item_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Item not found".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Item deactivated successfully".to_string(),
        id: Some(item_id),
    }))
}

// ============================================================================
// Room Inventory Endpoints
// ============================================================================

/// GET /api/new/inventory/rooms/:room_id - Get room's inventory
pub async fn get_room_inventory(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
) -> ApiResult<Json<RoomInventoryResponse>> {
    let pool = &state.new_pool;

    let rows = sqlx::query(
            r#"
            SELECT
                ri.ri_id,
                ri.ri_room_id,
                ri.ri_item_id,
                i.item_code,
                i.item_name,
                ri.ri_quantity,
                ri.ri_last_checked
            FROM ht_room_inventory ri
            LEFT JOIN ht_inventory_items i ON ri.ri_item_id = i.item_id
            WHERE ri.ri_room_id = $1
            ORDER BY i.item_name ASC
            "#,
        )
        .bind(&room_id)
        .fetch_all(pool)
        .await?;

    let items: Vec<RoomInventoryItem> = rows
        .iter()
        .map(|row| RoomInventoryItem {
            id: row.try_get::<i32, _>("ri_id").unwrap_or(0),
            room_id: row.try_get::<i32, _>("ri_room_id").unwrap_or(0),
            item_id: row.try_get::<i32, _>("ri_item_id").unwrap_or(0),
            item_code: row.try_get::<String, _>("item_code").ok(),
            item_name: row.try_get::<String, _>("item_name").ok(),
            quantity: row.try_get::<i32, _>("ri_quantity").unwrap_or(0),
            last_checked: row.try_get::<NaiveDateTime, _>("ri_last_checked").ok(),
        })
        .collect();

    Ok(Json(RoomInventoryResponse {
        success: true,
        room_id,
        items,
    }))
}

/// PUT /api/new/inventory/rooms/:room_id - Update room's inventory
pub async fn update_room_inventory(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Json(body): Json<UpdateRoomInventoryRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let pool = &state.new_pool;

    // Delete existing room inventory entries
    sqlx::query(
        "DELETE FROM ht_room_inventory WHERE ri_room_id = $1",
    )
    .bind(&room_id)
    .execute(pool)
    .await?;

    // Insert new entries
    for item in &body.items {
        if item.quantity > 0 {
            sqlx::query(
                r#"
                INSERT INTO ht_room_inventory (ri_room_id, ri_item_id, ri_quantity, ri_last_checked)
                VALUES ($1, $2, $3, NOW())
                "#,
            )
            .bind(&room_id)
            .bind(&item.item_id)
            .bind(&item.quantity)
            .execute(pool)
            .await?;
        }
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Room inventory updated successfully".to_string(),
        id: Some(room_id),
    }))
}

// ============================================================================
// Transaction Endpoints
// ============================================================================

/// GET /api/new/inventory/transactions - List transactions
pub async fn list_transactions(
    State(state): State<AppState>,
    Query(params): Query<TransactionsQuery>,
) -> ApiResult<Json<TransactionsResponse>> {
    let pool = &state.new_pool;

    let offset = (params.page - 1) * params.limit;

    // Build WHERE conditions
    let mut conditions: Vec<String> = Vec::new();

    if let Some(item_id) = params.item_id {
        conditions.push(format!("t.trans_item_id = {}", item_id));
    }

    if let Some(ref trans_type) = params.trans_type {
        let escaped = trans_type.replace('\'', "''").to_uppercase();
        conditions.push(format!("t.trans_type = '{}'", escaped));
    }

    if let Some(ref from) = params.from {
        let escaped = from.replace('\'', "''");
        conditions.push(format!("t.trans_date >= '{}'", escaped));
    }

    if let Some(ref to) = params.to {
        let escaped = to.replace('\'', "''");
        conditions.push(format!("t.trans_date <= '{}'", escaped));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count query
    let count_query = format!(
        "SELECT COUNT(*)::int as total FROM ht_inventory_transactions t {}",
        where_clause
    );

    let count_rows = sqlx::query(&count_query)
        .fetch_all(pool)
        .await?;

    let total: i32 = count_rows
        .first()
        .and_then(|r| r.try_get::<i32, _>("total").ok())
        .unwrap_or(0);

    // Data query
    let data_query = format!(
        r#"
        SELECT
            t.trans_id,
            t.trans_item_id,
            i.item_code,
            i.item_name,
            t.trans_type,
            t.trans_quantity,
            t.trans_room_id,
            t.trans_notes,
            t.trans_date,
            t.trans_by
        FROM ht_inventory_transactions t
        LEFT JOIN ht_inventory_items i ON t.trans_item_id = i.item_id
        {}
        ORDER BY t.trans_date DESC
        LIMIT {} OFFSET {}
        "#,
        where_clause, params.limit, offset
    );

    let rows = sqlx::query(&data_query)
        .fetch_all(pool)
        .await?;

    let transactions: Vec<Transaction> = rows
        .iter()
        .map(|row| Transaction {
            id: row.try_get::<i32, _>("trans_id").unwrap_or(0),
            item_id: row.try_get::<i32, _>("trans_item_id").unwrap_or(0),
            item_code: row.try_get::<String, _>("item_code").ok(),
            item_name: row.try_get::<String, _>("item_name").ok(),
            trans_type: row.try_get::<String, _>("trans_type").unwrap_or_default(),
            quantity: row.try_get::<i32, _>("trans_quantity").unwrap_or(0),
            room_id: row.try_get::<i32, _>("trans_room_id").ok(),
            notes: row.try_get::<String, _>("trans_notes").ok(),
            date: row.try_get::<NaiveDateTime, _>("trans_date").ok(),
            created_by: row.try_get::<String, _>("trans_by").ok(),
        })
        .collect();

    Ok(Json(TransactionsResponse {
        success: true,
        data: transactions,
        pagination: Pagination::new(params.page, params.limit, total),
    }))
}

/// POST /api/new/inventory/transactions - Create transaction
/// Also updates item_current_stock based on transaction type:
/// - IN: Add quantity to stock
/// - OUT: Subtract quantity from stock
/// - ADJUST: Set stock to absolute value (quantity becomes the new stock)
/// - MOVE: Log only (for room transfers, doesn't affect main stock)
pub async fn create_transaction(
    State(state): State<AppState>,
    Json(body): Json<CreateTransactionRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let trans_type = body.trans_type.trim().to_uppercase();

    // Validate transaction type
    let valid_types = ["IN", "OUT", "ADJUST", "MOVE"];
    if !valid_types.contains(&trans_type.as_str()) {
        return Err(ApiError::BadRequest(format!(
            "Invalid transaction type '{}'. Must be one of: {}",
            trans_type,
            valid_types.join(", ")
        )));
    }

    if body.quantity < 0 && trans_type != "ADJUST" {
        return Err(ApiError::BadRequest("Quantity must be non-negative".to_string()));
    }

    let pool = &state.new_pool;

    // Check if item exists
    let item_rows = sqlx::query(
            "SELECT item_id, item_current_stock FROM ht_inventory_items WHERE item_id = $1",
        )
        .bind(&body.item_id)
        .fetch_all(pool)
        .await?;

    if item_rows.is_empty() {
        return Err(ApiError::NotFound("Item not found".to_string()));
    }

    let current_stock: i32 = item_rows
        .first()
        .and_then(|r| r.try_get::<i32, _>("item_current_stock").ok())
        .unwrap_or(0);

    // Calculate new stock based on transaction type
    let new_stock = match trans_type.as_str() {
        "IN" => current_stock + body.quantity,
        "OUT" => {
            let result = current_stock - body.quantity;
            if result < 0 {
                return Err(ApiError::BadRequest(format!(
                    "Insufficient stock. Current: {}, Requested: {}",
                    current_stock, body.quantity
                )));
            }
            result
        }
        "ADJUST" => body.quantity, // Set to absolute value
        "MOVE" => current_stock,   // No change to main stock
        _ => current_stock,
    };

    // Insert transaction record
    let rows = sqlx::query(
            r#"
            INSERT INTO ht_inventory_transactions (
                trans_item_id, trans_type, trans_quantity, trans_room_id, trans_notes, trans_by
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING trans_id
            "#,
        )
        .bind(&body.item_id)
        .bind(&trans_type.as_str())
        .bind(&body.quantity)
        .bind(&body.room_id)
        .bind(&body.notes.as_deref())
        .bind(&body.created_by.as_deref())
        .fetch_all(pool)
        .await?;

    let trans_id = rows
        .first()
        .and_then(|r| r.try_get::<i32, _>("trans_id").ok())
        .ok_or_else(|| ApiError::Internal("Failed to create transaction".to_string()))?;

    // Update item stock (except for MOVE type)
    if trans_type != "MOVE" {
        sqlx::query(
            "UPDATE ht_inventory_items SET item_current_stock = $1, item_updated = NOW() WHERE item_id = $2",
        )
        .bind(&new_stock)
        .bind(&body.item_id)
        .execute(pool)
        .await?;
    }

    Ok(Json(MutationResponse {
        success: true,
        message: format!("Transaction created. Stock {} to {}",
            match trans_type.as_str() {
                "IN" => "increased",
                "OUT" => "decreased",
                "ADJUST" => "adjusted",
                "MOVE" => "unchanged (move)",
                _ => "updated",
            },
            new_stock
        ),
        id: Some(trans_id),
    }))
}

// ============================================================================
// Dashboard Endpoints
// ============================================================================

/// GET /api/new/inventory/stats - Summary statistics
pub async fn get_stats(
    State(state): State<AppState>,
) -> ApiResult<Json<StatsResponse>> {
    let pool = &state.new_pool;

    let rows = sqlx::query(
            r#"
            SELECT
                (SELECT COUNT(*)::int FROM ht_inventory_items WHERE item_active = true) as total_items,
                (SELECT COUNT(*)::int FROM ht_inventory_categories WHERE cat_active = true) as total_categories,
                (SELECT COUNT(*)::int FROM ht_inventory_items WHERE item_active = true AND item_current_stock <= item_min_stock AND item_current_stock > 0) as low_stock_count,
                (SELECT COUNT(*)::int FROM ht_inventory_items WHERE item_active = true AND item_current_stock = 0) as out_of_stock_count,
                (SELECT COALESCE(SUM(item_current_stock * COALESCE(item_cost, 0)), 0) FROM ht_inventory_items WHERE item_active = true) as total_stock_value
            "#,
        )
        .fetch_all(pool)
        .await?;

    let row = rows.first();

    let stats = InventoryStats {
        total_items: row.and_then(|r| r.try_get::<i32, _>("total_items").ok()).unwrap_or(0),
        total_categories: row.and_then(|r| r.try_get::<i32, _>("total_categories").ok()).unwrap_or(0),
        low_stock_count: row.and_then(|r| r.try_get::<i32, _>("low_stock_count").ok()).unwrap_or(0),
        out_of_stock_count: row.and_then(|r| r.try_get::<i32, _>("out_of_stock_count").ok()).unwrap_or(0),
        total_stock_value: row.and_then(|r| r.try_get::<f64, _>("total_stock_value").ok()).unwrap_or(0.0),
    };

    Ok(Json(StatsResponse { success: true, stats }))
}

/// GET /api/new/inventory/low-stock - Items below minimum stock
pub async fn get_low_stock(
    State(state): State<AppState>,
) -> ApiResult<Json<LowStockResponse>> {
    let pool = &state.new_pool;

    let rows = sqlx::query(
            r#"
            SELECT
                i.item_id,
                i.item_code,
                i.item_name,
                i.item_category_id,
                c.cat_name,
                i.item_unit,
                i.item_min_stock,
                i.item_current_stock,
                i.item_cost,
                i.item_active,
                i.item_created,
                i.item_updated
            FROM ht_inventory_items i
            LEFT JOIN ht_inventory_categories c ON i.item_category_id = c.cat_id
            WHERE i.item_active = true AND i.item_current_stock <= i.item_min_stock
            ORDER BY (i.item_current_stock - i.item_min_stock) ASC, i.item_name ASC
            "#,
        )
        .fetch_all(pool)
        .await?;

    let items: Vec<Item> = rows
        .iter()
        .map(|row| Item {
            id: row.try_get::<i32, _>("item_id").unwrap_or(0),
            code: row.try_get::<String, _>("item_code").unwrap_or_default(),
            name: row.try_get::<String, _>("item_name").unwrap_or_default(),
            category_id: row.try_get::<i32, _>("item_category_id").ok(),
            category_name: row.try_get::<String, _>("cat_name").ok(),
            unit: row.try_get::<String, _>("item_unit").unwrap_or_default(),
            min_stock: row.try_get::<i32, _>("item_min_stock").unwrap_or(0),
            current_stock: row.try_get::<i32, _>("item_current_stock").unwrap_or(0),
            cost: row.try_get::<f64, _>("item_cost").ok(),
            active: row.try_get::<bool, _>("item_active").unwrap_or(true),
            created_at: row.try_get::<NaiveDateTime, _>("item_created").ok(),
            updated_at: row.try_get::<NaiveDateTime, _>("item_updated").ok(),
        })
        .collect();

    let total = items.len() as i32;

    Ok(Json(LowStockResponse {
        success: true,
        data: items,
        total,
    }))
}
