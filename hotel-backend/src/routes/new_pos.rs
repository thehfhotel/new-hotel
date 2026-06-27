//! POS routes — Task #45 (walk-up sale + void).
//!
//! Two write paths that the folio POS in `new_checkins.rs` doesn't cover:
//!
//! - `POST /api/new/pos/walkup-sale` — a roomless sale (no active
//!   check-in). Writes a standalone receipt (`ht_pos_receipts` +
//!   `ht_pos_receipt_lines`) → legacy `HT_Receipt_H`/`Ds`.
//! - `POST /api/new/pos/sales/:id/void` — void a folio POS line
//!   (`ht_pos_sales.sale_status='voided'` + stock restore + legacy
//!   `HT_CheckIn_Product` reversal).
//!
//! **Branch-aware:** `?branch=hfville` targets the `hotelville` canonical
//! pool, everything else the primary HF Hotel pool — same `service_for`
//! pattern as `routes::housekeeping`. A fresh branch-bound [`PosService`]
//! is built per request (cheap Arc clones + a pool handle clone); the
//! AppState-wired instance is hardwired to the primary pool at startup. HF
//! Ville mutations stay gated by the `ville_write_guard` middleware until
//! `HFVILLE_WRITES_ENABLED` is flipped — this module relies on that
//! existing safety net rather than re-checking the flag.
//!
//! **Actor:** the cashier comes from the `Extension<User>` injected by the
//! auth middleware when present; auth ships dark, so the handlers fall back
//! to a body-supplied `soldBy`/`by` then a stable sentinel — same shape as
//! `routes::new_payments::create_payment`.

use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::{Deserialize, Serialize};

use super::mode::{AppState, Branch};
use crate::domain::user::User;
use crate::error::{ApiError, ApiResult};
use crate::service::error::ServiceError;
use crate::service::{
    PosService, RecordWalkupSaleCommand, RecordWalkupSaleOutcome, VoidSaleCommand, WalkupSaleLine,
};

/// Operator label stamped onto a sale when no auth user / body value is
/// supplied. Mirrors `routes::housekeeping::DEFAULT_BY`.
const DEFAULT_BY: &str = "Front Desk";

/// Branch selector shared by the POS routes.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PosQuery {
    pub branch: Option<Branch>,
}

/// Build a [`PosService`] bound to the branch's pool. Reuses the AppState
/// outbox / event-bus handles (cheap Arc clones) and the resolved pool —
/// same construction shape as `routes::housekeeping::service_for`.
fn service_for(state: &AppState, branch: Option<Branch>) -> ApiResult<PosService> {
    let pool = match branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool()?.clone(),
        Branch::Hfhotel | Branch::All => state.new_pool.clone(),
    };
    Ok(PosService::new(
        state.outbox.clone(),
        state.events.clone(),
        pool,
    ))
}

/// Resolve the operator: prefer the authenticated user, then the body
/// value, then [`DEFAULT_BY`].
fn resolve_actor(actor: Option<&User>, body: Option<&str>) -> String {
    actor
        .map(|u| u.username.clone())
        .or_else(|| {
            body.map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
        })
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_BY.to_string())
}

// ============================================================================
// Walk-up sale
// ============================================================================

/// One line of the walk-up sale request body.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalkupSaleLineRequest {
    pub product_id: i64,
    pub qty: f64,
    #[serde(default)]
    pub unit_price_override: Option<f64>,
    #[serde(default)]
    pub discount: Option<f64>,
}

/// `POST /api/new/pos/walkup-sale` request body.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalkupSaleRequest {
    pub lines: Vec<WalkupSaleLineRequest>,
    #[serde(default)]
    pub customer_no: Option<String>,
    #[serde(default)]
    pub customer_name: Option<String>,
    #[serde(default)]
    pub customer_address: Option<String>,
    #[serde(default)]
    pub customer_tel: Option<String>,
    #[serde(default)]
    pub tax_id: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub vat_percent: Option<i32>,
    /// Header-level discount in baht (on top of per-line discounts).
    #[serde(default)]
    pub discount: Option<f64>,
    #[serde(default)]
    pub payment_method: Option<String>,
    /// Direct-pay amount. Absent ⇒ settled in full (defaults to total).
    #[serde(default)]
    pub paid: Option<f64>,
    /// Cashier fallback when auth is dark.
    #[serde(default)]
    pub sold_by: Option<String>,
}

/// One persisted line in the walk-up sale response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalkupSaleLineResponse {
    pub line_id: i64,
    pub product_id: i64,
    pub product_legacy_no: String,
    pub product_name: String,
    pub unit_name: String,
    pub qty: f64,
    pub unit_price_baht: f64,
    pub discount_baht: f64,
    pub total_baht: f64,
}

/// `POST /api/new/pos/walkup-sale` response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalkupSaleResponse {
    pub success: bool,
    pub message: String,
    pub receipt_id: i64,
    pub aggregate_id: uuid::Uuid,
    pub customer_no: String,
    pub subtotal_baht: f64,
    pub discount_baht: f64,
    pub before_vat_baht: f64,
    pub vat_baht: f64,
    pub vat_percent: i32,
    pub total_baht: f64,
    pub paid_baht: f64,
    pub payment_method: String,
    pub sold_at: chrono::DateTime<chrono::Utc>,
    pub lines: Vec<WalkupSaleLineResponse>,
}

/// `POST /api/new/pos/walkup-sale` — ring up a roomless sale.
pub async fn create_walkup_sale(
    State(state): State<AppState>,
    Query(query): Query<PosQuery>,
    actor: Option<Extension<User>>,
    Json(body): Json<WalkupSaleRequest>,
) -> ApiResult<Json<WalkupSaleResponse>> {
    if body.lines.is_empty() {
        return Err(ApiError::BadRequest(
            "a walk-up sale must have at least one line".to_string(),
        ));
    }
    let svc = service_for(&state, query.branch)?;
    let sold_by = resolve_actor(actor.as_deref(), body.sold_by.as_deref());

    let command = RecordWalkupSaleCommand {
        lines: body
            .lines
            .iter()
            .map(|l| WalkupSaleLine {
                product_id: l.product_id,
                qty: l.qty,
                unit_price_override_baht: l.unit_price_override,
                discount_baht: l.discount.unwrap_or(0.0),
            })
            .collect(),
        customer_no: body.customer_no.unwrap_or_default(),
        customer_name: body.customer_name.unwrap_or_default(),
        customer_address: body.customer_address.unwrap_or_default(),
        customer_tel: body.customer_tel.unwrap_or_default(),
        tax_id: body.tax_id.unwrap_or_default(),
        note: body.note.unwrap_or_default(),
        vat_percent: body.vat_percent.unwrap_or(0),
        discount_baht: body.discount.unwrap_or(0.0),
        payment_method: body
            .payment_method
            .map(|m| m.trim().to_lowercase())
            .filter(|m| !m.is_empty())
            .unwrap_or_else(|| "cash".to_string()),
        paid_baht: body.paid,
        actor: sold_by,
    };

    let outcome = svc
        .record_walkup_sale(command)
        .await
        .map_err(map_pos_error)?;
    Ok(Json(build_walkup_response(outcome)))
}

/// PURE — translate the service outcome to the HTTP response body.
pub(crate) fn build_walkup_response(outcome: RecordWalkupSaleOutcome) -> WalkupSaleResponse {
    WalkupSaleResponse {
        success: true,
        message: "Walk-up sale recorded".to_string(),
        receipt_id: outcome.receipt_id,
        aggregate_id: outcome.aggregate_id,
        customer_no: outcome.customer_no,
        subtotal_baht: outcome.subtotal_baht,
        discount_baht: outcome.discount_baht,
        before_vat_baht: outcome.before_vat_baht,
        vat_baht: outcome.vat_baht,
        vat_percent: outcome.vat_percent,
        total_baht: outcome.total_baht,
        paid_baht: outcome.paid_baht,
        payment_method: outcome.payment_method,
        sold_at: outcome.sold_at,
        lines: outcome
            .lines
            .into_iter()
            .map(|l| WalkupSaleLineResponse {
                line_id: l.line_id,
                product_id: l.product_id,
                product_legacy_no: l.product_legacy_no,
                product_name: l.product_name,
                unit_name: l.unit_name,
                qty: l.qty,
                unit_price_baht: l.unit_price_baht,
                discount_baht: l.discount_baht,
                total_baht: l.total_baht,
            })
            .collect(),
    }
}

// ============================================================================
// Void a sale line
// ============================================================================

/// `POST /api/new/pos/sales/:id/void` request body.
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct VoidSaleRequest {
    pub reason: Option<String>,
    /// Operator fallback when auth is dark.
    pub by: Option<String>,
}

/// `POST /api/new/pos/sales/:id/void` response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VoidSaleResponse {
    pub success: bool,
    pub message: String,
    pub sale_id: i64,
    pub qty_restored: f64,
}

/// `POST /api/new/pos/sales/:id/void` — void a folio POS line.
pub async fn void_sale(
    State(state): State<AppState>,
    Path(sale_id): Path<i64>,
    Query(query): Query<PosQuery>,
    actor: Option<Extension<User>>,
    body: Option<Json<VoidSaleRequest>>,
) -> ApiResult<Json<VoidSaleResponse>> {
    let svc = service_for(&state, query.branch)?;
    let Json(body) = body.unwrap_or_default();
    let by = resolve_actor(actor.as_deref(), body.by.as_deref());

    let outcome = svc
        .void_sale(VoidSaleCommand {
            sale_id,
            by,
            reason: body.reason,
        })
        .await
        .map_err(map_pos_error)?;

    Ok(Json(VoidSaleResponse {
        success: true,
        message: "Sale voided".to_string(),
        sale_id: outcome.sale_id,
        qty_restored: outcome.qty_restored,
    }))
}

/// Translate service outcomes to HTTP. Validation + Conflict surface as 400
/// with explicit wording so the cashier UI can show what went wrong.
fn map_pos_error(err: ServiceError) -> ApiError {
    match err {
        ServiceError::Conflict(msg) => ApiError::BadRequest(msg),
        ServiceError::Validation(msg) => ApiError::BadRequest(msg),
        other => other.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_actor_prefers_auth_then_body_then_default() {
        // Body fallback when no auth user.
        assert_eq!(resolve_actor(None, Some("Nok")), "Nok");
        // Default when both absent / blank.
        assert_eq!(resolve_actor(None, None), DEFAULT_BY);
        assert_eq!(resolve_actor(None, Some("   ")), DEFAULT_BY);
    }

    #[test]
    fn walkup_request_parses_minimal_body() {
        let body: WalkupSaleRequest = serde_json::from_str(
            r#"{"lines":[{"productId":1,"qty":2}]}"#,
        )
        .unwrap();
        assert_eq!(body.lines.len(), 1);
        assert_eq!(body.lines[0].product_id, 1);
        assert_eq!(body.lines[0].qty, 2.0);
        assert!(body.payment_method.is_none());
        assert!(body.vat_percent.is_none());
    }

    #[test]
    fn walkup_request_parses_full_body() {
        let body: WalkupSaleRequest = serde_json::from_str(
            r#"{"lines":[{"productId":3,"qty":1,"unitPriceOverride":25.0,"discount":5.0}],
                "customerName":"ACME","taxId":"012","vatPercent":7,"discount":10.0,
                "paymentMethod":"QR","paid":100.0,"soldBy":"Nok"}"#,
        )
        .unwrap();
        assert_eq!(body.lines[0].unit_price_override, Some(25.0));
        assert_eq!(body.lines[0].discount, Some(5.0));
        assert_eq!(body.vat_percent, Some(7));
        assert_eq!(body.discount, Some(10.0));
        assert_eq!(body.payment_method.as_deref(), Some("QR"));
        assert_eq!(body.sold_by.as_deref(), Some("Nok"));
    }

    #[test]
    fn void_request_accepts_empty_object() {
        let body: VoidSaleRequest = serde_json::from_str("{}").unwrap();
        assert!(body.reason.is_none());
        assert!(body.by.is_none());
    }
}
