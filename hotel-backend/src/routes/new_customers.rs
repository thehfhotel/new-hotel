//! New Customer API routes for HotelNew database
//!
//! - GET    /api/customers/{id}   - Get single customer (full record)
//! - POST   /api/customers        - Create customer
//! - PUT    /api/customers/{id}   - Update customer
//! - DELETE /api/customers/{id}   - Soft delete (set `cust_active=false`)
//! - GET    /api/customers/search - List customers from `ht_customers`
//!
//! Per `docs/architecture.md` §1, §6 (Phase 2.5) writes delegate through the
//! customer service. `create` keeps using the AppState-wired
//! `state.customers_service` (bound to the primary HF Hotel pool — its legacy
//! writeback is folded into the booking/check-in recipes). The `update`,
//! `delete` and single-`get` handlers are **branch-aware**: `?branch=hfville`
//! targets the `hotelville` canonical pool, everything else the HF Hotel pool —
//! mirroring `routes::housekeeping` / `routes::new_shifts`. A fresh
//! `CustomerService` is built per request bound to the resolved pool
//! (construction is cheap: Arc clones + a pool-handle clone). HF Ville
//! mutations stay gated by the `ville_write_guard` middleware in `main.rs`
//! until `HFVILLE_WRITES_ENABLED` is flipped — these handlers rely on that
//! existing safety net rather than re-checking the flag.
//!
//! The soft-delete handler stays on the repository **by design**: customer
//! deletes deliberately have no legacy writeback (iHOTEL's delete is a
//! destructive hard-DELETE + `'C0000'` cascade, cheatsheet §3.24 — see
//! `delete_customer` below).
//!
//! Path ids accept either the canonical integer `cust_id` or the legacy
//! `cust_no` (`C\d+`) the list endpoint emits — both resolve via
//! [`resolve_customer_id_int`](crate::routes::customers::resolve_customer_id_int).

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::mode::{AppState, Branch};
use crate::db::PgPool;
use crate::error::{ApiError, ApiResult};
use crate::models::Pagination;
use crate::outbox::event::EventSource;
use crate::repository::customer::CustomerRow;
use crate::routes::customers::resolve_customer_id_int;
use crate::service::{CreateCustomerCommand, CustomerService, UpdateCustomerCommand};

/// Customer from HT_Customers table
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomer {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub id_card: Option<String>,
    pub address: Option<String>,
    pub customer_type: Option<String>,
    pub notes: Option<String>,
    pub active: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl NewCustomer {
    fn from_row(row: CustomerRow) -> Self {
        Self {
            id: row.cust_id,
            first_name: Some(row.cust_firstname),
            last_name: row.cust_lastname,
            phone: row.cust_phone,
            email: row.cust_email,
            id_card: row.cust_idcard,
            address: row.cust_address,
            customer_type: row.cust_type,
            notes: row.cust_notes,
            active: row.cust_active.unwrap_or(true),
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

/// Query parameters for customers list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomersQuery {
    pub search: Option<String>,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    #[serde(default = "default_active_only")]
    pub active_only: bool,
    /// Branch selector: 'hfhotel' | 'hfville' | 'all' (HotelNew only contains hfhotel data)
    pub branch: Option<Branch>,
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 20 }
fn default_active_only() -> bool { true }

/// Response for customers list
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCustomersResponse {
    pub success: bool,
    pub data: Vec<NewCustomer>,
    pub pagination: Pagination,
}

/// Request body for creating/updating customer
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateCustomerRequest {
    pub first_name: String,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub id_card: Option<String>,
    pub address: Option<String>,
    pub customer_type: Option<String>,
    pub notes: Option<String>,
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

/// Branch selector for the single-customer CRUD handlers (`get` / `update` /
/// `delete`). Branch-only — the path id scopes the row, the branch scopes the
/// pool. Mirrors `routes::housekeeping::HousekeepingQuery`.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerMutationQuery {
    pub branch: Option<Branch>,
}

/// Resolve the canonical PG pool for a single-customer request. `All` is not
/// meaningful for one customer → default to HF Hotel. Returns an owned handle
/// (cheap `PgPool` clone) so the caller can both resolve the id and build a
/// per-request [`CustomerService`] bound to it.
fn customer_pool_for(state: &AppState, branch: Option<Branch>) -> ApiResult<PgPool> {
    // Delegate the Hfville→ville_pool decision to the unified write chokepoint.
    Ok(state.write_pool(branch)?.clone())
}

/// Build a [`CustomerService`] bound to `pool`, reusing the AppState
/// repository / outbox / event-bus handles (cheap Arc clones) — same
/// construction shape as `AppState::wire_services` and
/// `routes::housekeeping::service_for`.
fn customer_service_for(state: &AppState, pool: PgPool) -> CustomerService {
    CustomerService::new(
        state.customers.clone(),
        state.outbox.clone(),
        state.events.clone(),
        pool,
    )
}

/// GET /api/new/customers - List customers from HT_Customers
pub async fn list_customers(
    State(state): State<AppState>,
    Query(params): Query<NewCustomersQuery>,
) -> ApiResult<Json<NewCustomersResponse>> {
    // HotelNew DB only contains hfhotel data; hfville selector returns empty.
    if params.branch == Some(Branch::Hfville) {
        return Ok(Json(NewCustomersResponse {
            success: true,
            data: vec![],
            pagination: Pagination::new(params.page, params.limit, 0),
        }));
    }

    let (rows, total) = state
        .customers
        .list_with_count(&state.new_pool, &params)
        .await?;

    let customers: Vec<NewCustomer> = rows.into_iter().map(NewCustomer::from_row).collect();

    Ok(Json(NewCustomersResponse {
        success: true,
        data: customers,
        pagination: Pagination::new(params.page, params.limit, total),
    }))
}

/// POST /api/new/customers - Create customer
pub async fn create_customer(
    State(state): State<AppState>,
    Json(body): Json<CreateUpdateCustomerRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let outcome = state
        .customers_service
        .create(CreateCustomerCommand {
            first_name: body.first_name,
            last_name: body.last_name,
            phone: body.phone,
            email: body.email,
            id_card: body.id_card,
            address: body.address,
            customer_type: body.customer_type,
            notes: body.notes,
            // TODO: wire user_id from auth middleware
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
        })
        .await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Customer created successfully".to_string(),
        id: Some(outcome.customer_id),
    }))
}

/// GET /api/customers/{id} - Fetch a single customer's full record.
///
/// Branch-aware via `?branch=`. Returns every editable column (first/last
/// name, phone, email, id card, address, customer type, notes) so the edit
/// form loads the complete row — the canonical UPDATE overwrites all of these
/// columns, so the form must round-trip the un-edited ones (the list endpoint
/// only carries the display subset).
pub async fn get_customer(
    State(state): State<AppState>,
    Path(path_id): Path<String>,
    Query(query): Query<CustomerMutationQuery>,
) -> ApiResult<Json<NewCustomer>> {
    let pool = customer_pool_for(&state, query.branch)?;
    let cust_id = resolve_customer_id_int(&pool, &path_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Customer not found".to_string()))?;

    let row = state
        .customers
        .get(&pool, cust_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Customer not found".to_string()))?;

    Ok(Json(NewCustomer::from_row(row)))
}

/// PUT /api/customers/{id} - Update customer.
///
/// Branch-aware: builds a per-request [`CustomerService`] bound to the branch
/// pool so the canonical UPDATE + the legacy `UpdateCustomer` re-save enqueue
/// (audit 2026-06-11 P2) commit in one transaction on the right site.
pub async fn update_customer(
    State(state): State<AppState>,
    Path(path_id): Path<String>,
    Query(query): Query<CustomerMutationQuery>,
    Json(body): Json<CreateUpdateCustomerRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let pool = customer_pool_for(&state, query.branch)?;
    let cust_id = resolve_customer_id_int(&pool, &path_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Customer not found".to_string()))?;

    let outcome = customer_service_for(&state, pool)
        .update(UpdateCustomerCommand {
            customer_id: cust_id,
            first_name: body.first_name,
            last_name: body.last_name,
            phone: body.phone,
            email: body.email,
            id_card: body.id_card,
            address: body.address,
            customer_type: body.customer_type,
            notes: body.notes,
            // TODO: wire user_id from auth middleware
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
        })
        .await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Customer updated successfully".to_string(),
        id: Some(outcome.customer_id),
    }))
}

/// DELETE /api/new/customers/:id - Soft delete (set `cust_active=false`)
///
/// **Deliberately has NO legacy writeback** (decision recorded with the
/// 2026-06-11 audit P2 `UpdateCustomer` gap-close): iHOTEL's only delete
/// shape is a destructive hard-DELETE of the `HT_Customers` row +
/// `Tb_Save_Image` photos plus a 6-statement `'C0000'` cascade across
/// every FK-like reference (cheatsheet §3.24) — irreversible by
/// construction, while our soft-delete is a reversible flag. Mirroring it
/// would let a mis-click here permanently destroy legacy data. The
/// soft-deleted customer therefore remains visible in iHOTEL; see the
/// fuller rationale in `writeback/recipes/update_customer.rs`.
///
/// Stays on the repository (not the service): with the writeback
/// deliberately out of scope there is nothing for a service method to
/// compose beyond the single UPDATE.
///
/// Branch-aware via `?branch=` — the soft-delete runs against the branch
/// pool. (It is a canonical-only write, so HF Ville's lack of a writeback is
/// moot; ville mutations stay blocked by the `ville_write_guard` regardless.)
pub async fn delete_customer(
    State(state): State<AppState>,
    Path(path_id): Path<String>,
    Query(query): Query<CustomerMutationQuery>,
) -> ApiResult<Json<MutationResponse>> {
    let pool = customer_pool_for(&state, query.branch)?;
    let cust_id = resolve_customer_id_int(&pool, &path_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Customer not found".to_string()))?;

    let mut tx = pool.begin().await?;
    let rows_affected = state.customers.soft_delete(&mut tx, cust_id).await?;

    if rows_affected == 0 {
        tx.rollback().await?;
        return Err(ApiError::NotFound("Customer not found".to_string()));
    }
    tx.commit().await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Customer deleted successfully".to_string(),
        id: Some(cust_id),
    }))
}
