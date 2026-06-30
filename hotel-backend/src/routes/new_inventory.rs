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
//!
//! Per `docs/architecture.md` §1, §6 (Phase 1b) the SQL has moved to
//! `repository::inventory`. Validation, derived totals, and DTO mapping stay
//! here.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::mode::{AppState, Branch};
use crate::error::{ApiError, ApiResult};
use crate::models::Pagination;
use crate::repository::inventory::{
    CategoryInsert, CategoryRow, InventoryRoomRollupRow, ItemRow, ItemWrite,
    RoomChecklistItemRow, TransactionRow,
};

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

impl Category {
    fn from_row(row: CategoryRow) -> Self {
        Self {
            id: row.cat_id,
            name: row.cat_name,
            description: row.cat_description,
            active: row.cat_active.unwrap_or(true),
            created_at: row.cat_created,
        }
    }
}

/// Inventory item
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i32,
    // The /inventory/items list UI reads itemCode/itemName/category/costPerUnit
    // (see types/inventory.ts). Without these explicit renames the list showed
    // blank code/name/category (only the already-matching minStock/currentStock/
    // unit rendered).
    #[serde(rename = "itemCode")]
    pub code: String,
    #[serde(rename = "itemName")]
    pub name: String,
    pub category_id: Option<i32>,
    #[serde(rename = "category")]
    pub category_name: Option<String>,
    pub unit: String,
    pub min_stock: i32,
    pub current_stock: i32,
    #[serde(rename = "costPerUnit")]
    pub cost: Option<f64>,
    pub active: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Item {
    fn from_row(row: ItemRow) -> Self {
        Self {
            id: row.item_id,
            code: row.item_code,
            name: row.item_name,
            category_id: row.item_category_id,
            category_name: row.cat_name,
            unit: row.item_unit,
            min_stock: row.item_min_stock.unwrap_or(0),
            current_stock: row.item_current_stock.unwrap_or(0),
            cost: row.item_cost,
            active: row.item_active.unwrap_or(true),
            created_at: row.item_created,
            updated_at: row.item_updated,
        }
    }
}

/// Room inventory assignment (legacy struct, kept for compatibility with admin tools)
#[allow(dead_code)]
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

impl Transaction {
    fn from_row(row: TransactionRow) -> Self {
        Self {
            id: row.trans_id,
            item_id: row.trans_item_id.unwrap_or(0),
            item_code: row.item_code,
            item_name: row.item_name,
            trans_type: row.trans_type,
            quantity: row.trans_quantity,
            room_id: row.trans_room_id,
            notes: row.trans_notes,
            date: row.trans_date,
            created_by: row.trans_by,
        }
    }
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
    /// Per-site read pool selector (`?branch=hfville`).
    pub branch: Option<Branch>,
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

/// A category reference accepted on the item create/update body: either a
/// numeric category id or a category name (resolved to an id server-side).
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CategoryRef {
    Id(i32),
    Name(String),
}

/// Request for creating/updating item.
///
/// Field aliases accept the frontend's camelCase payload shape
/// (`itemCode`/`itemName`/`costPerUnit`/`minStock`/`currentStock`) in addition
/// to the canonical names, so the form can POST/PUT without a 422.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateItemRequest {
    #[serde(alias = "itemCode")]
    pub code: String,
    #[serde(alias = "itemName")]
    pub name: String,
    pub category_id: Option<i32>,
    /// Optional category reference (id or name). When `category_id` is absent
    /// and this carries a name, it is resolved against `ht_inventory_categories`.
    pub category: Option<CategoryRef>,
    pub unit: String,
    #[serde(alias = "minStock")]
    pub min_stock: Option<i32>,
    #[serde(alias = "currentStock")]
    pub current_stock: Option<i32>,
    #[serde(alias = "costPerUnit")]
    pub cost: Option<f64>,
    pub active: Option<bool>,
}

/// Resolve the effective category id for an item write.
///
/// Precedence: an explicit `category_id` wins. Otherwise, if a `category`
/// reference was sent, a numeric id is taken as-is and a name is looked up in
/// `ht_inventory_categories` (case-insensitive exact match). An unknown name is
/// a clean 400 — categories are never auto-created here. A missing category is
/// allowed (`None`), matching the previous nullable behavior.
async fn resolve_category_id(
    pool: &crate::db::PgPool,
    category_id: Option<i32>,
    category: &Option<CategoryRef>,
) -> ApiResult<Option<i32>> {
    if let Some(id) = category_id {
        return Ok(Some(id));
    }
    match category {
        None => Ok(None),
        Some(CategoryRef::Id(id)) => Ok(Some(*id)),
        Some(CategoryRef::Name(name)) => {
            let name = name.trim();
            if name.is_empty() {
                return Ok(None);
            }
            let found: Option<i32> = sqlx::query_scalar(
                "SELECT cat_id FROM ht_inventory_categories WHERE LOWER(cat_name) = LOWER($1) LIMIT 1",
            )
            .bind(name)
            .fetch_optional(pool)
            .await?;
            match found {
                Some(id) => Ok(Some(id)),
                None => Err(ApiError::BadRequest(format!(
                    "Unknown category '{name}'"
                ))),
            }
        }
    }
}

/// Aggregated room inventory item (shape used by RoomInventoryChecklist frontend)
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInventoryChecklistItem {
    pub item_id: i32,
    pub item_code: String,
    pub item_name: String,
    pub category: String,
    pub unit: String,
    pub assigned_quantity: i32,
    pub verified_quantity: Option<i32>,
    pub last_verified: Option<NaiveDateTime>,
}

impl RoomInventoryChecklistItem {
    fn from_row(row: RoomChecklistItemRow) -> Self {
        Self {
            item_id: row.item_id,
            item_code: row.item_code,
            item_name: row.item_name,
            category: row.category,
            unit: row.item_unit,
            assigned_quantity: row.assigned_quantity,
            // Original behavior: verified_quantity defaults to the assigned count.
            verified_quantity: Some(row.assigned_quantity),
            last_verified: row.last_checked,
        }
    }
}

/// Inner data block for the checklist response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInventoryData {
    pub room_id: i32,
    pub room_number: String,
    pub room_type: String,
    pub items: Vec<RoomInventoryChecklistItem>,
}

/// Response for room inventory (frontend shape: { success, data: { ..., items } })
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInventoryResponse {
    pub success: bool,
    pub data: RoomInventoryData,
}

/// Room rollup for the inventory rooms list page
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryRoomRollup {
    pub id: i32,
    pub room_number: String,
    pub room_type: String,
    pub status: String,
    pub inventory_count: i32,
    pub last_checked: Option<NaiveDateTime>,
    pub missing_items: i32,
}

impl InventoryRoomRollup {
    fn from_row(row: InventoryRoomRollupRow) -> Self {
        Self {
            id: row.room_id,
            room_number: row.room_no,
            room_type: row.room_type,
            status: row.room_status,
            inventory_count: row.inventory_count,
            last_checked: row.last_checked,
            missing_items: row.missing_items,
        }
    }
}

/// Response for the inventory rooms collection endpoint
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryRoomsResponse {
    pub success: bool,
    pub data: Vec<InventoryRoomRollup>,
}

/// Query parameters for inventory rooms list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InventoryRoomsQuery {
    pub search: Option<String>,
    /// 'all' | 'missing' | 'checked'
    pub filter: Option<String>,
    pub branch: Option<Branch>,
}

/// Request body for room inventory check (POST /rooms/:room_id/check)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInventoryCheckRequest {
    pub items: Vec<RoomInventoryCheckItem>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInventoryCheckItem {
    pub item_id: i32,
    pub verified: bool,
    pub actual_quantity: i32,
}

/// Request body for room inventory replenish (POST /rooms/:room_id/replenish)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInventoryReplenishRequest {
    pub items: Vec<RoomInventoryReplenishItem>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomInventoryReplenishItem {
    pub item_id: i32,
    pub quantity: i32,
}

/// Request body for stock adjustment (POST /adjustments)
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockAdjustmentRequest {
    pub item_id: i32,
    /// 'add' | 'remove' | 'set'
    pub adjustment_type: String,
    pub quantity: i32,
    pub notes: Option<String>,
    pub created_by: Option<String>,
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

/// Branch selector for the inventory READ endpoints (list_categories, get_item)
/// that don't already carry an ItemsQuery. Resolves the per-site read pool so
/// HF Ville reads its own inventory.
#[derive(Debug, Deserialize)]
pub struct InvBranchQuery {
    pub branch: Option<Branch>,
}

/// GET /api/new/inventory/categories - List all categories
pub async fn list_categories(
    State(state): State<AppState>,
    Query(bq): Query<InvBranchQuery>,
) -> ApiResult<Json<CategoriesResponse>> {
    let rows = state
        .inventory
        .list_categories(state.write_pool(bq.branch)?)
        .await?;
    let categories: Vec<Category> = rows.into_iter().map(Category::from_row).collect();
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

    let active = body.active.unwrap_or(true);

    let mut tx = state.new_pool.begin().await?;
    let id = state
        .inventory
        .insert_category(
            &mut tx,
            CategoryInsert {
                name,
                description: body.description.as_deref(),
                active,
            },
        )
        .await?;
    tx.commit().await?;

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
    // Branch-aware read: HF Ville must list its own inventory, not HF Hotel's
    // (the list previously always read state.new_pool, so switching branch never
    // changed the items).
    let pool = state.write_pool(params.branch)?;
    let (rows, total) = state
        .inventory
        .list_items_with_count(pool, &params)
        .await?;
    let items: Vec<Item> = rows.into_iter().map(Item::from_row).collect();

    Ok(Json(ItemsResponse {
        success: true,
        data: items,
        pagination: Pagination::new(params.page, params.limit, total),
    }))
}

/// POST /api/new/inventory/items - Create item
pub async fn create_item(
    State(state): State<AppState>,
    Query(bq): Query<InvBranchQuery>,
    Json(body): Json<CreateUpdateItemRequest>,
) -> ApiResult<Json<MutationResponse>> {
    // Branch-aware write: the code-uniqueness check, category resolution AND the
    // insert all run against the per-site pool, so a Ville item isn't rejected
    // by an HF-Hotel code clash (and isn't inserted into HF Hotel). HF Ville
    // mutations stay 403'd by ville_write_guard until HFVILLE_WRITES_ENABLED.
    let pool = state.write_pool(bq.branch)?;
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

    if state
        .inventory
        .find_item_id_by_code(pool, code)
        .await?
        .is_some()
    {
        return Err(ApiError::BadRequest("Item code already exists".to_string()));
    }

    let min_stock = body.min_stock.unwrap_or(0);
    let current_stock = body.current_stock.unwrap_or(0);
    let active = body.active.unwrap_or(true);
    let category_id = resolve_category_id(pool, body.category_id, &body.category).await?;

    let mut tx = pool.begin().await?;
    let id = state
        .inventory
        .insert_item(
            &mut tx,
            ItemWrite {
                code,
                name,
                category_id,
                unit,
                min_stock,
                current_stock,
                cost: body.cost,
                active,
            },
        )
        .await?;
    tx.commit().await?;

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
    Query(bq): Query<InvBranchQuery>,
) -> ApiResult<Json<ItemResponse>> {
    let row = state
        .inventory
        .get_item(state.write_pool(bq.branch)?, item_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Item not found".to_string()))?;

    Ok(Json(ItemResponse {
        success: true,
        item: Item::from_row(row),
    }))
}

/// PUT /api/new/inventory/items/:id - Update item
pub async fn update_item(
    State(state): State<AppState>,
    Path(item_id): Path<i32>,
    Query(bq): Query<InvBranchQuery>,
    Json(body): Json<CreateUpdateItemRequest>,
) -> ApiResult<Json<MutationResponse>> {
    // Branch-aware write (see create_item): code check, category resolve + update
    // all run on the per-site pool.
    let pool = state.write_pool(bq.branch)?;
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

    if state
        .inventory
        .find_item_id_by_code_excluding(pool, code, item_id)
        .await?
        .is_some()
    {
        return Err(ApiError::BadRequest("Item code already exists".to_string()));
    }

    let min_stock = body.min_stock.unwrap_or(0);
    let current_stock = body.current_stock.unwrap_or(0);
    let active = body.active.unwrap_or(true);
    let category_id = resolve_category_id(pool, body.category_id, &body.category).await?;

    let mut tx = pool.begin().await?;
    let rows_affected = state
        .inventory
        .update_item(
            &mut tx,
            item_id,
            ItemWrite {
                code,
                name,
                category_id,
                unit,
                min_stock,
                current_stock,
                cost: body.cost,
                active,
            },
        )
        .await?;

    if rows_affected == 0 {
        tx.rollback().await?;
        return Err(ApiError::NotFound("Item not found".to_string()));
    }
    tx.commit().await?;

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
    let mut tx = state.new_pool.begin().await?;
    let rows_affected = state.inventory.deactivate_item(&mut tx, item_id).await?;

    if rows_affected == 0 {
        tx.rollback().await?;
        return Err(ApiError::NotFound("Item not found".to_string()));
    }
    tx.commit().await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Item deactivated successfully".to_string(),
        id: Some(item_id),
    }))
}

// ============================================================================
// Room Inventory Endpoints
// ============================================================================

/// GET /api/new/inventory/rooms/:room_id - Get room's inventory (checklist shape)
///
/// Response: `{ success, data: { roomId, roomNumber, roomType, items: [{ itemId, itemCode, itemName, category, unit, assignedQuantity, verifiedQuantity, lastVerified }] } }`
pub async fn get_room_inventory(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
) -> ApiResult<Json<RoomInventoryResponse>> {
    let room = state
        .inventory
        .get_room_lite(&state.new_pool, room_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Room not found".to_string()))?;

    let rows = state
        .inventory
        .list_room_checklist(&state.new_pool, room_id)
        .await?;
    let items: Vec<RoomInventoryChecklistItem> = rows
        .into_iter()
        .map(RoomInventoryChecklistItem::from_row)
        .collect();

    Ok(Json(RoomInventoryResponse {
        success: true,
        data: RoomInventoryData {
            room_id,
            room_number: room.room_no,
            room_type: room.type_name,
            items,
        },
    }))
}

/// GET /api/new/inventory/rooms - List rooms with inventory rollup
///
/// Response: `{ success, data: [{ id, roomNumber, roomType, status, inventoryCount, lastChecked, missingItems }] }`
pub async fn list_inventory_rooms(
    State(state): State<AppState>,
    Query(params): Query<InventoryRoomsQuery>,
) -> ApiResult<Json<InventoryRoomsResponse>> {
    // The new HotelNew DB only contains hfhotel rooms. Treat hfville as empty.
    if params.branch == Some(Branch::Hfville) {
        return Ok(Json(InventoryRoomsResponse { success: true, data: vec![] }));
    }

    let rows = state
        .inventory
        .list_inventory_rooms(&state.new_pool, &params)
        .await?;
    let mut data: Vec<InventoryRoomRollup> = rows.into_iter().map(InventoryRoomRollup::from_row).collect();

    // Apply filter post-aggregation (matches the previous in-process filter).
    if let Some(ref filter) = params.filter {
        match filter.as_str() {
            "missing" => data.retain(|r| r.missing_items > 0),
            "checked" => {
                let now = chrono::Utc::now().naive_utc();
                data.retain(|r| match r.last_checked {
                    Some(ts) => (now - ts).num_hours() < 24,
                    None => false,
                });
            }
            _ => {}
        }
    }

    Ok(Json(InventoryRoomsResponse { success: true, data }))
}

/// POST /api/new/inventory/rooms/:room_id/check
/// Records an inventory check for a room. Updates `ri_last_checked` for verified items
/// and creates a transaction log entry per item to capture the check event.
pub async fn check_room_inventory(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Json(body): Json<RoomInventoryCheckRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let mut tx = state.new_pool.begin().await?;

    for item in &body.items {
        state
            .inventory
            .touch_room_item_last_checked(&mut tx, room_id, item.item_id)
            .await?;
    }

    let notes = body.notes.unwrap_or_else(|| "Room inventory check".to_string());
    // Best-effort log entry (matches the route's prior `.ok()`-swallowed call).
    let _ = state
        .inventory
        .log_room_check(&mut tx, room_id, notes.as_str())
        .await;

    tx.commit().await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Room inventory check recorded".to_string(),
        id: Some(room_id),
    }))
}

/// POST /api/new/inventory/rooms/:room_id/replenish
/// Replenishes a room's inventory by deducting from main stock and bumping ri_last_checked.
pub async fn replenish_room_inventory(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Json(body): Json<RoomInventoryReplenishRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let notes = body.notes.unwrap_or_else(|| "Room replenish".to_string());

    let mut tx = state.new_pool.begin().await?;

    for item in &body.items {
        if item.quantity <= 0 {
            continue;
        }

        // Deduct from main stock (allow going to zero, refuse to go negative).
        let current = state
            .inventory
            .get_item_current_stock(&state.new_pool, item.item_id)
            .await?
            .and_then(|stock| stock)
            .unwrap_or(0);
        let new_stock = (current - item.quantity).max(0);

        state
            .inventory
            .set_item_current_stock(&mut tx, item.item_id, new_stock)
            .await?;

        state
            .inventory
            .log_replenish_out(&mut tx, item.item_id, item.quantity, room_id, notes.as_str())
            .await?;

        state
            .inventory
            .touch_room_item_last_checked(&mut tx, room_id, item.item_id)
            .await?;
    }

    tx.commit().await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Room inventory replenished".to_string(),
        id: Some(room_id),
    }))
}

/// POST /api/new/inventory/adjustments
/// Generic stock adjustment endpoint used by StockAdjustmentModal.
pub async fn create_stock_adjustment(
    State(state): State<AppState>,
    Json(body): Json<StockAdjustmentRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let kind = body.adjustment_type.trim().to_lowercase();
    if !["add", "remove", "set"].contains(&kind.as_str()) {
        return Err(ApiError::BadRequest(format!(
            "Invalid adjustment type '{}' (must be add|remove|set)",
            body.adjustment_type
        )));
    }
    if body.quantity < 0 {
        return Err(ApiError::BadRequest("Quantity must be non-negative".to_string()));
    }

    let stock_opt = state
        .inventory
        .get_item_current_stock(&state.new_pool, body.item_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Item not found".to_string()))?;
    let current = stock_opt.unwrap_or(0);

    let (new_stock, trans_type) = match kind.as_str() {
        "add" => (current + body.quantity, "IN"),
        "remove" => {
            if body.quantity > current {
                return Err(ApiError::BadRequest(format!(
                    "Insufficient stock. Current: {}, Requested: {}",
                    current, body.quantity
                )));
            }
            (current - body.quantity, "OUT")
        }
        "set" => (body.quantity, "ADJUST"),
        _ => unreachable!(),
    };

    let mut tx = state.new_pool.begin().await?;
    state
        .inventory
        .set_item_current_stock(&mut tx, body.item_id, new_stock)
        .await?;

    let trans_id = state
        .inventory
        .insert_stock_adjustment_transaction(
            &mut tx,
            body.item_id,
            trans_type,
            body.quantity,
            body.notes.as_deref(),
            body.created_by.as_deref(),
        )
        .await?;
    tx.commit().await?;

    Ok(Json(MutationResponse {
        success: true,
        message: format!("Stock adjusted to {}", new_stock),
        id: Some(trans_id),
    }))
}

/// PUT /api/new/inventory/rooms/:room_id - Update room's inventory
pub async fn update_room_inventory(
    State(state): State<AppState>,
    Path(room_id): Path<i32>,
    Json(body): Json<UpdateRoomInventoryRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let mut tx = state.new_pool.begin().await?;

    state.inventory.delete_room_inventory(&mut tx, room_id).await?;

    for item in &body.items {
        if item.quantity > 0 {
            state
                .inventory
                .insert_room_inventory(&mut tx, room_id, item.item_id, item.quantity)
                .await?;
        }
    }

    tx.commit().await?;

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
    let (rows, total) = state
        .inventory
        .list_transactions_with_count(&state.new_pool, &params)
        .await?;

    let transactions: Vec<Transaction> = rows.into_iter().map(Transaction::from_row).collect();

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

    let stock_opt = state
        .inventory
        .get_item_current_stock(&state.new_pool, body.item_id)
        .await?;

    if stock_opt.is_none() {
        return Err(ApiError::NotFound("Item not found".to_string()));
    }
    let current_stock = stock_opt.and_then(|s| s).unwrap_or(0);

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
        "ADJUST" => body.quantity,
        "MOVE" => current_stock,
        _ => current_stock,
    };

    let mut tx = state.new_pool.begin().await?;

    let trans_id = state
        .inventory
        .insert_transaction(
            &mut tx,
            body.item_id,
            trans_type.as_str(),
            body.quantity,
            body.room_id,
            body.notes.as_deref(),
            body.created_by.as_deref(),
        )
        .await?;

    if trans_type != "MOVE" {
        state
            .inventory
            .set_item_current_stock(&mut tx, body.item_id, new_stock)
            .await?;
    }

    tx.commit().await?;

    Ok(Json(MutationResponse {
        success: true,
        message: format!(
            "Transaction created. Stock {} to {}",
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
    let row = state.inventory.get_stats(&state.new_pool).await?;

    let stats = InventoryStats {
        total_items: row.total_items.unwrap_or(0),
        total_categories: row.total_categories.unwrap_or(0),
        low_stock_count: row.low_stock_count.unwrap_or(0),
        out_of_stock_count: row.out_of_stock_count.unwrap_or(0),
        total_stock_value: row.total_stock_value.unwrap_or(0.0),
    };

    Ok(Json(StatsResponse { success: true, stats }))
}

/// GET /api/new/inventory/low-stock - Items below minimum stock
pub async fn get_low_stock(
    State(state): State<AppState>,
) -> ApiResult<Json<LowStockResponse>> {
    let rows = state.inventory.list_low_stock(&state.new_pool).await?;
    let items: Vec<Item> = rows.into_iter().map(Item::from_row).collect();
    let total = items.len() as i32;

    Ok(Json(LowStockResponse {
        success: true,
        data: items,
        total,
    }))
}
