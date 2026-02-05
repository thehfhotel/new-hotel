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
    let mut conn = state.new_pool.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT
                Cat_ID,
                Cat_Name,
                Cat_Description,
                Cat_Active,
                Cat_Created
            FROM HT_Inventory_Categories
            ORDER BY Cat_Name ASC
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let categories: Vec<Category> = rows
        .iter()
        .map(|row| Category {
            id: row.get::<i32, _>("Cat_ID").unwrap_or(0),
            name: row.get::<&str, _>("Cat_Name").unwrap_or_default().to_string(),
            description: row.get::<&str, _>("Cat_Description").map(String::from),
            active: row.get::<bool, _>("Cat_Active").unwrap_or(true),
            created_at: row.get::<NaiveDateTime, _>("Cat_Created"),
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

    let mut conn = state.new_pool.get().await?;

    let active = body.active.unwrap_or(true);

    let rows = conn
        .query(
            r#"
            INSERT INTO HT_Inventory_Categories (Cat_Name, Cat_Description, Cat_Active)
            OUTPUT INSERTED.Cat_ID
            VALUES (@P1, @P2, @P3)
            "#,
            &[&name, &body.description.as_deref(), &active],
        )
        .await?
        .into_first_result()
        .await?;

    let id = rows
        .first()
        .and_then(|r| r.get::<i32, _>("Cat_ID"))
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
    let mut conn = state.new_pool.get().await?;

    let offset = (params.page - 1) * params.limit;

    // Build WHERE conditions
    let mut conditions: Vec<String> = Vec::new();

    if let Some(cat_id) = params.category_id {
        conditions.push(format!("i.Item_Category_ID = {}", cat_id));
    }

    if params.low_stock == Some(true) {
        conditions.push("i.Item_Current_Stock <= i.Item_Min_Stock".to_string());
    }

    if let Some(ref search) = params.search {
        let escaped = search.replace('\'', "''");
        conditions.push(format!(
            "(i.Item_Code LIKE '%{}%' OR i.Item_Name LIKE '%{}%')",
            escaped, escaped
        ));
    }

    if let Some(active) = params.active {
        conditions.push(format!("i.Item_Active = {}", if active { 1 } else { 0 }));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count query
    let count_query = format!(
        "SELECT COUNT(*) as total FROM HT_Inventory_Items i {}",
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
            i.Item_ID,
            i.Item_Code,
            i.Item_Name,
            i.Item_Category_ID,
            c.Cat_Name,
            i.Item_Unit,
            i.Item_Min_Stock,
            i.Item_Current_Stock,
            i.Item_Cost,
            i.Item_Active,
            i.Item_Created,
            i.Item_Updated
        FROM HT_Inventory_Items i
        LEFT JOIN HT_Inventory_Categories c ON i.Item_Category_ID = c.Cat_ID
        {}
        ORDER BY i.Item_Name ASC
        OFFSET {} ROWS FETCH NEXT {} ROWS ONLY
        "#,
        where_clause, offset, params.limit
    );

    let rows = conn
        .simple_query(&data_query)
        .await?
        .into_first_result()
        .await?;

    let items: Vec<Item> = rows
        .iter()
        .map(|row| Item {
            id: row.get::<i32, _>("Item_ID").unwrap_or(0),
            code: row.get::<&str, _>("Item_Code").unwrap_or_default().to_string(),
            name: row.get::<&str, _>("Item_Name").unwrap_or_default().to_string(),
            category_id: row.get::<i32, _>("Item_Category_ID"),
            category_name: row.get::<&str, _>("Cat_Name").map(String::from),
            unit: row.get::<&str, _>("Item_Unit").unwrap_or_default().to_string(),
            min_stock: row.get::<i32, _>("Item_Min_Stock").unwrap_or(0),
            current_stock: row.get::<i32, _>("Item_Current_Stock").unwrap_or(0),
            cost: row.get::<f64, _>("Item_Cost"),
            active: row.get::<bool, _>("Item_Active").unwrap_or(true),
            created_at: row.get::<NaiveDateTime, _>("Item_Created"),
            updated_at: row.get::<NaiveDateTime, _>("Item_Updated"),
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

    let mut conn = state.new_pool.get().await?;

    // Check for duplicate code
    let check_rows = conn
        .query(
            "SELECT Item_ID FROM HT_Inventory_Items WHERE Item_Code = @P1",
            &[&code],
        )
        .await?
        .into_first_result()
        .await?;

    if !check_rows.is_empty() {
        return Err(ApiError::BadRequest("Item code already exists".to_string()));
    }

    let min_stock = body.min_stock.unwrap_or(0);
    let current_stock = body.current_stock.unwrap_or(0);
    let active = body.active.unwrap_or(true);

    let rows = conn
        .query(
            r#"
            INSERT INTO HT_Inventory_Items (
                Item_Code, Item_Name, Item_Category_ID, Item_Unit,
                Item_Min_Stock, Item_Current_Stock, Item_Cost, Item_Active
            )
            OUTPUT INSERTED.Item_ID
            VALUES (@P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8)
            "#,
            &[
                &code,
                &name,
                &body.category_id,
                &unit,
                &min_stock,
                &current_stock,
                &body.cost,
                &active,
            ],
        )
        .await?
        .into_first_result()
        .await?;

    let id = rows
        .first()
        .and_then(|r| r.get::<i32, _>("Item_ID"))
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
    let mut conn = state.new_pool.get().await?;

    let rows = conn
        .query(
            r#"
            SELECT
                i.Item_ID,
                i.Item_Code,
                i.Item_Name,
                i.Item_Category_ID,
                c.Cat_Name,
                i.Item_Unit,
                i.Item_Min_Stock,
                i.Item_Current_Stock,
                i.Item_Cost,
                i.Item_Active,
                i.Item_Created,
                i.Item_Updated
            FROM HT_Inventory_Items i
            LEFT JOIN HT_Inventory_Categories c ON i.Item_Category_ID = c.Cat_ID
            WHERE i.Item_ID = @P1
            "#,
            &[&item_id],
        )
        .await?
        .into_first_result()
        .await?;

    let row = rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Item not found".to_string()))?;

    let item = Item {
        id: row.get::<i32, _>("Item_ID").unwrap_or(0),
        code: row.get::<&str, _>("Item_Code").unwrap_or_default().to_string(),
        name: row.get::<&str, _>("Item_Name").unwrap_or_default().to_string(),
        category_id: row.get::<i32, _>("Item_Category_ID"),
        category_name: row.get::<&str, _>("Cat_Name").map(String::from),
        unit: row.get::<&str, _>("Item_Unit").unwrap_or_default().to_string(),
        min_stock: row.get::<i32, _>("Item_Min_Stock").unwrap_or(0),
        current_stock: row.get::<i32, _>("Item_Current_Stock").unwrap_or(0),
        cost: row.get::<f64, _>("Item_Cost"),
        active: row.get::<bool, _>("Item_Active").unwrap_or(true),
        created_at: row.get::<NaiveDateTime, _>("Item_Created"),
        updated_at: row.get::<NaiveDateTime, _>("Item_Updated"),
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

    let mut conn = state.new_pool.get().await?;

    // Check for duplicate code (excluding current item)
    let check_rows = conn
        .query(
            "SELECT Item_ID FROM HT_Inventory_Items WHERE Item_Code = @P1 AND Item_ID != @P2",
            &[&code, &item_id],
        )
        .await?
        .into_first_result()
        .await?;

    if !check_rows.is_empty() {
        return Err(ApiError::BadRequest("Item code already exists".to_string()));
    }

    let min_stock = body.min_stock.unwrap_or(0);
    let current_stock = body.current_stock.unwrap_or(0);
    let active = body.active.unwrap_or(true);

    let result = conn
        .execute(
            r#"
            UPDATE HT_Inventory_Items
            SET Item_Code = @P1,
                Item_Name = @P2,
                Item_Category_ID = @P3,
                Item_Unit = @P4,
                Item_Min_Stock = @P5,
                Item_Current_Stock = @P6,
                Item_Cost = @P7,
                Item_Active = @P8,
                Item_Updated = GETDATE()
            WHERE Item_ID = @P9
            "#,
            &[
                &code,
                &name,
                &body.category_id,
                &unit,
                &min_stock,
                &current_stock,
                &body.cost,
                &active,
                &item_id,
            ],
        )
        .await?;

    if result.total() == 0 {
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
    let mut conn = state.new_pool.get().await?;

    let result = conn
        .execute(
            "UPDATE HT_Inventory_Items SET Item_Active = 0, Item_Updated = GETDATE() WHERE Item_ID = @P1",
            &[&item_id],
        )
        .await?;

    if result.total() == 0 {
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
    let mut conn = state.new_pool.get().await?;

    let rows = conn
        .query(
            r#"
            SELECT
                ri.RI_ID,
                ri.RI_Room_ID,
                ri.RI_Item_ID,
                i.Item_Code,
                i.Item_Name,
                ri.RI_Quantity,
                ri.RI_Last_Checked
            FROM HT_Room_Inventory ri
            LEFT JOIN HT_Inventory_Items i ON ri.RI_Item_ID = i.Item_ID
            WHERE ri.RI_Room_ID = @P1
            ORDER BY i.Item_Name ASC
            "#,
            &[&room_id],
        )
        .await?
        .into_first_result()
        .await?;

    let items: Vec<RoomInventoryItem> = rows
        .iter()
        .map(|row| RoomInventoryItem {
            id: row.get::<i32, _>("RI_ID").unwrap_or(0),
            room_id: row.get::<i32, _>("RI_Room_ID").unwrap_or(0),
            item_id: row.get::<i32, _>("RI_Item_ID").unwrap_or(0),
            item_code: row.get::<&str, _>("Item_Code").map(String::from),
            item_name: row.get::<&str, _>("Item_Name").map(String::from),
            quantity: row.get::<i32, _>("RI_Quantity").unwrap_or(0),
            last_checked: row.get::<NaiveDateTime, _>("RI_Last_Checked"),
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
    let mut conn = state.new_pool.get().await?;

    // Delete existing room inventory entries
    conn.execute(
        "DELETE FROM HT_Room_Inventory WHERE RI_Room_ID = @P1",
        &[&room_id],
    )
    .await?;

    // Insert new entries
    for item in &body.items {
        if item.quantity > 0 {
            conn.execute(
                r#"
                INSERT INTO HT_Room_Inventory (RI_Room_ID, RI_Item_ID, RI_Quantity, RI_Last_Checked)
                VALUES (@P1, @P2, @P3, GETDATE())
                "#,
                &[&room_id, &item.item_id, &item.quantity],
            )
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
    let mut conn = state.new_pool.get().await?;

    let offset = (params.page - 1) * params.limit;

    // Build WHERE conditions
    let mut conditions: Vec<String> = Vec::new();

    if let Some(item_id) = params.item_id {
        conditions.push(format!("t.Trans_Item_ID = {}", item_id));
    }

    if let Some(ref trans_type) = params.trans_type {
        let escaped = trans_type.replace('\'', "''").to_uppercase();
        conditions.push(format!("t.Trans_Type = '{}'", escaped));
    }

    if let Some(ref from) = params.from {
        let escaped = from.replace('\'', "''");
        conditions.push(format!("t.Trans_Date >= '{}'", escaped));
    }

    if let Some(ref to) = params.to {
        let escaped = to.replace('\'', "''");
        conditions.push(format!("t.Trans_Date <= '{}'", escaped));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count query
    let count_query = format!(
        "SELECT COUNT(*) as total FROM HT_Inventory_Transactions t {}",
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
            t.Trans_ID,
            t.Trans_Item_ID,
            i.Item_Code,
            i.Item_Name,
            t.Trans_Type,
            t.Trans_Quantity,
            t.Trans_Room_ID,
            t.Trans_Notes,
            t.Trans_Date,
            t.Trans_By
        FROM HT_Inventory_Transactions t
        LEFT JOIN HT_Inventory_Items i ON t.Trans_Item_ID = i.Item_ID
        {}
        ORDER BY t.Trans_Date DESC
        OFFSET {} ROWS FETCH NEXT {} ROWS ONLY
        "#,
        where_clause, offset, params.limit
    );

    let rows = conn
        .simple_query(&data_query)
        .await?
        .into_first_result()
        .await?;

    let transactions: Vec<Transaction> = rows
        .iter()
        .map(|row| Transaction {
            id: row.get::<i32, _>("Trans_ID").unwrap_or(0),
            item_id: row.get::<i32, _>("Trans_Item_ID").unwrap_or(0),
            item_code: row.get::<&str, _>("Item_Code").map(String::from),
            item_name: row.get::<&str, _>("Item_Name").map(String::from),
            trans_type: row.get::<&str, _>("Trans_Type").unwrap_or_default().to_string(),
            quantity: row.get::<i32, _>("Trans_Quantity").unwrap_or(0),
            room_id: row.get::<i32, _>("Trans_Room_ID"),
            notes: row.get::<&str, _>("Trans_Notes").map(String::from),
            date: row.get::<NaiveDateTime, _>("Trans_Date"),
            created_by: row.get::<&str, _>("Trans_By").map(String::from),
        })
        .collect();

    Ok(Json(TransactionsResponse {
        success: true,
        data: transactions,
        pagination: Pagination::new(params.page, params.limit, total),
    }))
}

/// POST /api/new/inventory/transactions - Create transaction
/// Also updates Item_Current_Stock based on transaction type:
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

    let mut conn = state.new_pool.get().await?;

    // Check if item exists
    let item_rows = conn
        .query(
            "SELECT Item_ID, Item_Current_Stock FROM HT_Inventory_Items WHERE Item_ID = @P1",
            &[&body.item_id],
        )
        .await?
        .into_first_result()
        .await?;

    if item_rows.is_empty() {
        return Err(ApiError::NotFound("Item not found".to_string()));
    }

    let current_stock: i32 = item_rows
        .first()
        .and_then(|r| r.get::<i32, _>("Item_Current_Stock"))
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
    let rows = conn
        .query(
            r#"
            INSERT INTO HT_Inventory_Transactions (
                Trans_Item_ID, Trans_Type, Trans_Quantity, Trans_Room_ID, Trans_Notes, Trans_By
            )
            OUTPUT INSERTED.Trans_ID
            VALUES (@P1, @P2, @P3, @P4, @P5, @P6)
            "#,
            &[
                &body.item_id,
                &trans_type.as_str(),
                &body.quantity,
                &body.room_id,
                &body.notes.as_deref(),
                &body.created_by.as_deref(),
            ],
        )
        .await?
        .into_first_result()
        .await?;

    let trans_id = rows
        .first()
        .and_then(|r| r.get::<i32, _>("Trans_ID"))
        .ok_or_else(|| ApiError::Internal("Failed to create transaction".to_string()))?;

    // Update item stock (except for MOVE type)
    if trans_type != "MOVE" {
        conn.execute(
            "UPDATE HT_Inventory_Items SET Item_Current_Stock = @P1, Item_Updated = GETDATE() WHERE Item_ID = @P2",
            &[&new_stock, &body.item_id],
        )
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
    let mut conn = state.new_pool.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT
                (SELECT COUNT(*) FROM HT_Inventory_Items WHERE Item_Active = 1) as total_items,
                (SELECT COUNT(*) FROM HT_Inventory_Categories WHERE Cat_Active = 1) as total_categories,
                (SELECT COUNT(*) FROM HT_Inventory_Items WHERE Item_Active = 1 AND Item_Current_Stock <= Item_Min_Stock AND Item_Current_Stock > 0) as low_stock_count,
                (SELECT COUNT(*) FROM HT_Inventory_Items WHERE Item_Active = 1 AND Item_Current_Stock = 0) as out_of_stock_count,
                (SELECT ISNULL(SUM(Item_Current_Stock * ISNULL(Item_Cost, 0)), 0) FROM HT_Inventory_Items WHERE Item_Active = 1) as total_stock_value
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let row = rows.first();

    let stats = InventoryStats {
        total_items: row.and_then(|r| r.get::<i32, _>("total_items")).unwrap_or(0),
        total_categories: row.and_then(|r| r.get::<i32, _>("total_categories")).unwrap_or(0),
        low_stock_count: row.and_then(|r| r.get::<i32, _>("low_stock_count")).unwrap_or(0),
        out_of_stock_count: row.and_then(|r| r.get::<i32, _>("out_of_stock_count")).unwrap_or(0),
        total_stock_value: row.and_then(|r| r.get::<f64, _>("total_stock_value")).unwrap_or(0.0),
    };

    Ok(Json(StatsResponse { success: true, stats }))
}

/// GET /api/new/inventory/low-stock - Items below minimum stock
pub async fn get_low_stock(
    State(state): State<AppState>,
) -> ApiResult<Json<LowStockResponse>> {
    let mut conn = state.new_pool.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT
                i.Item_ID,
                i.Item_Code,
                i.Item_Name,
                i.Item_Category_ID,
                c.Cat_Name,
                i.Item_Unit,
                i.Item_Min_Stock,
                i.Item_Current_Stock,
                i.Item_Cost,
                i.Item_Active,
                i.Item_Created,
                i.Item_Updated
            FROM HT_Inventory_Items i
            LEFT JOIN HT_Inventory_Categories c ON i.Item_Category_ID = c.Cat_ID
            WHERE i.Item_Active = 1 AND i.Item_Current_Stock <= i.Item_Min_Stock
            ORDER BY (i.Item_Current_Stock - i.Item_Min_Stock) ASC, i.Item_Name ASC
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let items: Vec<Item> = rows
        .iter()
        .map(|row| Item {
            id: row.get::<i32, _>("Item_ID").unwrap_or(0),
            code: row.get::<&str, _>("Item_Code").unwrap_or_default().to_string(),
            name: row.get::<&str, _>("Item_Name").unwrap_or_default().to_string(),
            category_id: row.get::<i32, _>("Item_Category_ID"),
            category_name: row.get::<&str, _>("Cat_Name").map(String::from),
            unit: row.get::<&str, _>("Item_Unit").unwrap_or_default().to_string(),
            min_stock: row.get::<i32, _>("Item_Min_Stock").unwrap_or(0),
            current_stock: row.get::<i32, _>("Item_Current_Stock").unwrap_or(0),
            cost: row.get::<f64, _>("Item_Cost"),
            active: row.get::<bool, _>("Item_Active").unwrap_or(true),
            created_at: row.get::<NaiveDateTime, _>("Item_Created"),
            updated_at: row.get::<NaiveDateTime, _>("Item_Updated"),
        })
        .collect();

    let total = items.len() as i32;

    Ok(Json(LowStockResponse {
        success: true,
        data: items,
        total,
    }))
}
