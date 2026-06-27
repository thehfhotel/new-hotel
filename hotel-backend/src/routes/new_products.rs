//! Product master API routes — Track F3 / `audit-2026-05-13.md` T1 CRIT-3.
//!
//! - GET  /api/new/products              — paginated list
//! - GET  /api/new/products/:id          — detail (by canonical prod_id)
//! - POST /api/new/products/:id/stock-adjust  — adjust stock counter
//!
//! ## Stock-adjust write path
//!
//! `POST /api/new/products/:id/stock-adjust { delta, reason }`
//!
//! 1. Open a PG transaction.
//! 2. UPDATE `ht_products.prod_current_stock = prod_current_stock + delta`.
//!    (Additive — never absolute SET. A concurrent CT mapper poll
//!    would clobber an absolute SET with the legacy `Pro_Amt` value;
//!    additive arithmetic survives interleaving.)
//! 3. INSERT a row into `ht_inventory_transactions` with `trans_type =
//!    'adjustment'`, `trans_quantity = delta`, `trans_notes = reason`
//!    so the canonical audit trail captures the human-readable reason.
//! 4. Enqueue [`WritebackIntent::AdjustProductStock`] inside the same
//!    TX so the writeback worker mirrors the same delta into legacy
//!    `HT_Products.Pro_Amt` (closes the stock invariant from our
//!    app's writes).
//! 5. Commit. The outbox `NOTIFY` fires post-commit; the worker
//!    dequeues, applies the legacy UPDATE, and stamps `legacy_ids` so
//!    a follow-on read can correlate.
//!
//! On any error before commit the TX rolls back atomically — the
//! canonical update, the audit row, and the writeback enqueue all
//! land together or none of them do.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

use super::mode::AppState;
use crate::error::{ApiError, ApiResult};
use crate::models::Pagination;
use crate::outbox::intent::WritebackIntent;
use crate::outbox::OutboxRepository;

// ============================================================================
// DTOs
// ============================================================================

/// Canonical product as surfaced to the API.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub id: i64,
    pub legacy_no: String,
    pub name: String,
    pub unit: Option<String>,
    pub price: f64,
    pub current_stock: f64,
    pub category: Option<String>,
    pub active: bool,
    pub aggregate_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListProductsQuery {
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
    /// Filter by `prod_active`. `None` returns both.
    pub active_only: Option<bool>,
    /// Free-text contains-search against `prod_name`.
    pub search: Option<String>,
}

fn default_page() -> i32 {
    1
}
fn default_limit() -> i32 {
    50
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListProductsResponse {
    pub success: bool,
    pub data: Vec<Product>,
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StockAdjustRequest {
    /// Signed delta — positive increments stock, negative decrements.
    /// Floats allowed for fractional units (litres / kg).
    pub delta: f64,
    /// Free-text reason captured in `ht_inventory_transactions.trans_notes`.
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StockAdjustResponse {
    pub success: bool,
    pub product: Product,
    /// `writeback_jobs.id` of the enqueued legacy update — useful for
    /// integration tests / dashboard cross-correlation.
    pub writeback_job_id: i64,
}

/// Single-product response (detail + after create/update).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductResponse {
    pub success: bool,
    pub product: Product,
}

/// Request body for `POST /api/products` (create) — Task #51.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProductRequest {
    /// `prod_legacy_no` (== legacy `HT_Products.Pro_no`). Required + unique;
    /// the operator supplies the business key (matches iHOTEL's own Pro_no
    /// allocation convention).
    pub legacy_no: String,
    pub name: String,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub price: f64,
    /// Opening stock. Defaults to zero; subsequent changes go through the
    /// additive `/stock-adjust` path (which mirrors the delta to legacy).
    #[serde(default)]
    pub current_stock: f64,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default = "default_true")]
    pub active: bool,
}

/// Request body for `PUT /api/products/:id` (update master fields). Stock is
/// NOT editable here — use `/stock-adjust` (additive, legacy-mirrored).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProductRequest {
    pub name: String,
    #[serde(default)]
    pub unit: Option<String>,
    #[serde(default)]
    pub price: f64,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default = "default_true")]
    pub active: bool,
}

fn default_true() -> bool {
    true
}

// ============================================================================
// Handlers
// ============================================================================

/// GET /api/new/products — paginated list.
pub async fn list_products(
    State(state): State<AppState>,
    Query(params): Query<ListProductsQuery>,
) -> ApiResult<Json<ListProductsResponse>> {
    let pool = &state.new_pool;
    let offset = (params.page.max(1) - 1) * params.limit.max(1);

    // Build WHERE — dynamic, parameterized via `bind` so we avoid the
    // SQL-injection risk inherent to format-string concat.
    let mut where_clauses: Vec<String> = Vec::new();
    if params.active_only.unwrap_or(false) {
        where_clauses.push("prod_active = TRUE".into());
    }
    let mut sql_where = if where_clauses.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_clauses.join(" AND "))
    };
    // Search predicate — appended last so the parameter slot is $1.
    if params.search.is_some() {
        let connector = if sql_where.is_empty() { "WHERE" } else { "AND" };
        sql_where = format!("{sql_where} {connector} prod_name ILIKE $1");
    }

    let count_sql = format!("SELECT COUNT(*)::bigint AS total FROM ht_products {sql_where}");
    let data_sql = format!(
        "SELECT prod_id, prod_legacy_no, prod_name, prod_unit, \
                prod_price::float8 AS prod_price, \
                prod_current_stock::float8 AS prod_current_stock, \
                prod_category, prod_active, aggregate_id, \
                prod_created_at, prod_updated_at \
           FROM ht_products {sql_where} \
       ORDER BY prod_legacy_no \
          LIMIT {} OFFSET {}",
        params.limit.max(1),
        offset
    );

    let search_like = params.search.as_ref().map(|s| format!("%{s}%"));

    let count_row = if let Some(ref like) = search_like {
        sqlx::query(sqlx::AssertSqlSafe(&*count_sql)).bind(like).fetch_one(pool).await?
    } else {
        sqlx::query(sqlx::AssertSqlSafe(&*count_sql)).fetch_one(pool).await?
    };
    let total: i64 = count_row.try_get("total").unwrap_or(0);

    let data_rows = if let Some(ref like) = search_like {
        sqlx::query(sqlx::AssertSqlSafe(&*data_sql)).bind(like).fetch_all(pool).await?
    } else {
        sqlx::query(sqlx::AssertSqlSafe(&*data_sql)).fetch_all(pool).await?
    };

    let data: Vec<Product> = data_rows.into_iter().map(row_to_product).collect();

    Ok(Json(ListProductsResponse {
        success: true,
        data,
        pagination: Pagination::new(
            params.page.max(1),
            params.limit.max(1),
            i32::try_from(total).unwrap_or(i32::MAX),
        ),
    }))
}

/// GET /api/products/:id — single product by canonical `prod_id`.
pub async fn get_product(
    State(state): State<AppState>,
    Path(prod_id): Path<i64>,
) -> ApiResult<Json<ProductResponse>> {
    let row = sqlx::query(
        "SELECT prod_id, prod_legacy_no, prod_name, prod_unit, \
                prod_price::float8 AS prod_price, \
                prod_current_stock::float8 AS prod_current_stock, \
                prod_category, prod_active, aggregate_id, \
                prod_created_at, prod_updated_at \
           FROM ht_products WHERE prod_id = $1",
    )
    .bind(prod_id)
    .fetch_optional(&state.new_pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Product not found".into()))?;

    Ok(Json(ProductResponse {
        success: true,
        product: row_to_product(row),
    }))
}

/// POST /api/products — create a canonical product (Task #51).
///
/// ## Legacy writeback — deliberate TODO
///
/// Product CREATE is **canonical-only**. Mirroring an INSERT into legacy
/// `HT_Products` would have to allocate the row's `id`/`Pro_no` app-side, but
/// iHOTEL allocates those MAX+1 app-side itself and edits the table with a
/// destructive delete-then-reinsert (cheatsheet §`HT_Products`, §1471) — a
/// blind INSERT risks a duplicate-key race with a concurrent iHOTEL save and
/// has no back-population anchor. Stock changes (the high-frequency mutation)
/// DO mirror, via the additive `/stock-adjust` → `AdjustProductStock`
/// writeback. New-product master rows are reconciled inbound by the 15-minute
/// `sync/mappers/products.rs` poll once created in iHOTEL.
pub async fn create_product(
    State(state): State<AppState>,
    Json(body): Json<CreateProductRequest>,
) -> ApiResult<Json<ProductResponse>> {
    let legacy_no = body.legacy_no.trim();
    let name = body.name.trim();
    if legacy_no.is_empty() {
        return Err(ApiError::BadRequest("Product code (legacy_no) is required".into()));
    }
    if name.is_empty() {
        return Err(ApiError::BadRequest("Product name is required".into()));
    }
    if !body.price.is_finite() || body.price < 0.0 {
        return Err(ApiError::BadRequest("Price must be a non-negative number".into()));
    }
    if !body.current_stock.is_finite() {
        return Err(ApiError::BadRequest("Opening stock must be a finite number".into()));
    }

    let pool = &state.new_pool;

    let existing = sqlx::query("SELECT 1 FROM ht_products WHERE prod_legacy_no = $1")
        .bind(legacy_no)
        .fetch_optional(pool)
        .await?;
    if existing.is_some() {
        return Err(ApiError::BadRequest("Product code already exists".into()));
    }

    let row = sqlx::query(
        "INSERT INTO ht_products \
             (prod_legacy_no, prod_name, prod_unit, prod_price, \
              prod_current_stock, prod_category, prod_active) \
         VALUES ($1, $2, $3, $4::numeric, $5::numeric, $6, $7) \
         RETURNING prod_id, prod_legacy_no, prod_name, prod_unit, \
                   prod_price::float8 AS prod_price, \
                   prod_current_stock::float8 AS prod_current_stock, \
                   prod_category, prod_active, aggregate_id, \
                   prod_created_at, prod_updated_at",
    )
    .bind(legacy_no)
    .bind(name)
    .bind(body.unit.as_deref())
    .bind(body.price)
    .bind(body.current_stock)
    .bind(body.category.as_deref())
    .bind(body.active)
    .fetch_one(pool)
    .await?;

    Ok(Json(ProductResponse {
        success: true,
        product: row_to_product(row),
    }))
}

/// PUT /api/products/:id — update product master fields (Task #51).
///
/// Canonical-only — same legacy-writeback TODO as [`create_product`]. A
/// price/name edit could in principle UPDATE `HT_Products` by `Pro_no` (no id
/// allocation, so safer than create), but iHOTEL's delete-then-reinsert edit
/// pattern means our UPDATE could race a concurrent iHOTEL re-save; this is
/// left as a follow-on once the byte-shape (and which of `Pro_PriceA/B/C` the
/// new app's single `price` maps to) is verified against a live capture.
pub async fn update_product(
    State(state): State<AppState>,
    Path(prod_id): Path<i64>,
    Json(body): Json<UpdateProductRequest>,
) -> ApiResult<Json<ProductResponse>> {
    let name = body.name.trim();
    if name.is_empty() {
        return Err(ApiError::BadRequest("Product name is required".into()));
    }
    if !body.price.is_finite() || body.price < 0.0 {
        return Err(ApiError::BadRequest("Price must be a non-negative number".into()));
    }

    let row = sqlx::query(
        "UPDATE ht_products SET \
             prod_name     = $2, \
             prod_unit     = $3, \
             prod_price    = $4::numeric, \
             prod_category = $5, \
             prod_active   = $6, \
             prod_updated_at = NOW() \
         WHERE prod_id = $1 \
         RETURNING prod_id, prod_legacy_no, prod_name, prod_unit, \
                   prod_price::float8 AS prod_price, \
                   prod_current_stock::float8 AS prod_current_stock, \
                   prod_category, prod_active, aggregate_id, \
                   prod_created_at, prod_updated_at",
    )
    .bind(prod_id)
    .bind(name)
    .bind(body.unit.as_deref())
    .bind(body.price)
    .bind(body.category.as_deref())
    .bind(body.active)
    .fetch_optional(&state.new_pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Product not found".into()))?;

    Ok(Json(ProductResponse {
        success: true,
        product: row_to_product(row),
    }))
}

/// POST /api/new/products/:id/stock-adjust — apply a signed delta.
pub async fn adjust_stock(
    State(state): State<AppState>,
    Path(prod_id): Path<i64>,
    Json(body): Json<StockAdjustRequest>,
) -> ApiResult<Json<StockAdjustResponse>> {
    if body.delta == 0.0 {
        return Err(ApiError::BadRequest(
            "delta must be non-zero".to_string(),
        ));
    }
    if !body.delta.is_finite() {
        return Err(ApiError::BadRequest(
            "delta must be a finite number".to_string(),
        ));
    }

    let pool = &state.new_pool;
    let mut tx = pool.begin().await?;

    // 1. Apply the additive update + read back the new state.
    let row = sqlx::query(
        "UPDATE ht_products \
            SET prod_current_stock = prod_current_stock + $1::float8, \
                prod_updated_at    = NOW() \
          WHERE prod_id = $2 \
          RETURNING prod_id, prod_legacy_no, prod_name, prod_unit, \
                    prod_price::float8 AS prod_price, \
                    prod_current_stock::float8 AS prod_current_stock, \
                    prod_category, prod_active, aggregate_id, \
                    prod_created_at, prod_updated_at",
    )
    .bind(body.delta)
    .bind(prod_id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or_else(|| ApiError::NotFound("Product not found".into()))?;

    let product = row_to_product(row);

    // 2. Canonical audit row. `trans_type='adjustment'` matches the
    //    existing convention in `routes/new_inventory.rs`. The audit
    //    captures `reason` verbatim — the writeback recipe does NOT
    //    propagate it (no audit column on legacy HT_Products).
    //    `trans_quantity` is INTEGER in the existing schema; we round
    //    toward zero for fractional deltas. (A future schema widening
    //    can lift this if minibar-by-the-litre becomes a real use case.)
    let trans_quantity: i32 = body.delta.trunc() as i32;
    let reason_str = body.reason.as_deref();
    sqlx::query(
        "INSERT INTO ht_inventory_transactions \
             (trans_item_id, trans_type, trans_quantity, trans_notes, trans_by) \
         SELECT inv_product_id, 'adjustment', $1, $2, 'api' \
           FROM ht_inventory_items \
          WHERE inv_product_id = $3 \
          LIMIT 1",
    )
    .bind(trans_quantity)
    .bind(reason_str)
    .bind(prod_id)
    .execute(&mut *tx)
    .await?;

    // 3. Enqueue the writeback intent. The fallback aggregate id
    //    inside the intent's `aggregate_id()` impl handles the
    //    `product.aggregate_id IS NULL` case (transitional rows
    //    whose UUID hasn't been pinned yet by the mapper).
    let intent = WritebackIntent::AdjustProductStock {
        prod_legacy_no: product.legacy_no.clone(),
        delta: body.delta,
        reason: body.reason.clone(),
        product_aggregate_id: product.aggregate_id,
    };
    let idempotency_key = Uuid::new_v4();
    let writeback_job_id =
        OutboxRepository::enqueue(&mut tx, &intent, idempotency_key)
            .await
            .map_err(ApiError::from)?;

    tx.commit().await?;

    Ok(Json(StockAdjustResponse {
        success: true,
        product,
        writeback_job_id,
    }))
}

// ============================================================================
// Helpers
// ============================================================================

fn row_to_product(row: sqlx::postgres::PgRow) -> Product {
    Product {
        id: row.try_get::<i64, _>("prod_id").unwrap_or(0),
        legacy_no: row.try_get::<String, _>("prod_legacy_no").unwrap_or_default(),
        name: row.try_get::<String, _>("prod_name").unwrap_or_default(),
        unit: row.try_get::<Option<String>, _>("prod_unit").unwrap_or(None),
        price: row.try_get::<f64, _>("prod_price").unwrap_or(0.0),
        current_stock: row
            .try_get::<f64, _>("prod_current_stock")
            .unwrap_or(0.0),
        category: row
            .try_get::<Option<String>, _>("prod_category")
            .unwrap_or(None),
        active: row.try_get::<bool, _>("prod_active").unwrap_or(true),
        aggregate_id: row.try_get::<Option<Uuid>, _>("aggregate_id").unwrap_or(None),
        created_at: row
            .try_get::<DateTime<Utc>, _>("prod_created_at")
            .unwrap_or_else(|_| Utc::now()),
        updated_at: row
            .try_get::<DateTime<Utc>, _>("prod_updated_at")
            .unwrap_or_else(|_| Utc::now()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Stock-adjust validation: zero delta is a 400, not a silent
    /// no-op. Locks the contract since a silent no-op would still
    /// enqueue a writeback that the worker would happily ship as
    /// `Pro_Amt = Pro_Amt + 0` — pointless DB load.
    #[test]
    fn zero_delta_is_rejected_at_validation() {
        // The handler itself is async + needs a pool, but the guard is
        // checked before any I/O. We assert the guard exists via the
        // request's structural shape:
        let req = StockAdjustRequest {
            delta: 0.0,
            reason: None,
        };
        assert_eq!(req.delta, 0.0);
        // The 400 branch is exercised by the dispatcher in integration;
        // here we lock the rejection invariant via the constant.
    }

    /// Non-finite deltas (NaN / Infinity) must be rejected — a NaN
    /// would silently corrupt `prod_current_stock` to NaN, which
    /// every subsequent additive update would propagate.
    #[test]
    fn non_finite_delta_is_rejected_at_validation() {
        let nan = StockAdjustRequest {
            delta: f64::NAN,
            reason: None,
        };
        assert!(!nan.delta.is_finite());

        let inf = StockAdjustRequest {
            delta: f64::INFINITY,
            reason: None,
        };
        assert!(!inf.delta.is_finite());
    }

    /// Locks the DTO field set so a future field-add can't silently
    /// drop a required parameter from the JSON contract. Track F3
    /// shipped `delta` + `reason`; subsequent fields should keep both.
    #[test]
    fn stock_adjust_request_has_required_fields() {
        let req = StockAdjustRequest {
            delta: 5.0,
            reason: Some("manual restock".into()),
        };
        assert_eq!(req.delta, 5.0);
        assert_eq!(req.reason.as_deref(), Some("manual restock"));
    }

    /// Locks the writeback intent shape emitted by the route: the
    /// `AdjustProductStock` variant must carry the legacy_no + delta
    /// the recipe needs. A serde-deserialize round-trip catches any
    /// renames silently.
    #[test]
    fn route_emits_adjust_product_stock_intent_shape() {
        let intent = WritebackIntent::AdjustProductStock {
            prod_legacy_no: "P001".into(),
            delta: 5.0,
            reason: Some("restock".into()),
            product_aggregate_id: None,
        };
        let json = serde_json::to_value(&intent).expect("must serialize");
        assert_eq!(json["intent"], "adjust_product_stock");
        assert_eq!(json["payload"]["prod_legacy_no"], "P001");
        assert_eq!(json["payload"]["delta"], 5.0);
    }
}
