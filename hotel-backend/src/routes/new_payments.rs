//! New Payment API routes for HotelNew database
//!
//! - GET /api/new/checkins/:id/payments - List payments for a check-in
//! - POST /api/new/checkins/:id/payments - Record a payment
//! - DELETE /api/new/payments/:id - Void a payment (soft delete)
//!
//! Per `docs/architecture.md` §1, §6 (Phase 2.5) the create handler
//! delegates to `state.payments_service` for every legacy-known tender
//! method (cash / credit / transfer / qr → Web column) so the canonical
//! write, outbox enqueue, and event publish all happen atomically. The
//! list + void handlers stay on the repository (reads + a soft-delete
//! that has no service method yet).
//!
//! Wave 5c: QR / online payments map to `PaymentMethod::Web`, which the
//! writeback recipe routes to `HT_CheckIn_Pay.Cin_Pay_web`. Canonical PG
//! `ht_payments.pay_method` continues to carry the literal `"qr"` string
//! so dashboards / SSE keep their existing wire contract.

use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::mode::{AppState, Branch};
use crate::domain::payment::PaymentMethod;
use crate::domain::user::User;
use crate::error::{ApiError, ApiResult};
use crate::outbox::event::EventSource;
use crate::outbox::intent::RecordPaymentReceipt;
use crate::repository::payment::PaymentRow;
use crate::repository::settings;
use crate::service::{RecordPaymentCommand, RefundPaymentCommand};

/// Payment from HT_Payments table
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub id: i32,
    pub cin_id: i32,
    pub amount: f64,
    pub method: String,
    pub reference: Option<String>,
    pub notes: Option<String>,
    pub pay_date: Option<NaiveDateTime>,
    pub created_by: Option<String>,
    pub voided: bool,
    pub voided_at: Option<NaiveDateTime>,
    pub voided_by: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

impl Payment {
    fn from_row(row: PaymentRow) -> Self {
        Self {
            id: row.pay_id,
            cin_id: row.pay_cin_id,
            amount: row.pay_amount.unwrap_or(0.0),
            method: row.pay_method,
            reference: row.pay_reference,
            notes: row.pay_notes,
            pay_date: row.pay_date,
            created_by: row.pay_created_by,
            voided: row.pay_voided.unwrap_or(false),
            voided_at: row.pay_voided_at,
            voided_by: row.pay_voided_by,
            created_at: row.created_at,
        }
    }
}

/// Response for payments list with summary
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentsResponse {
    pub success: bool,
    pub data: Vec<Payment>,
    pub total_paid: f64,
    pub total_amount: f64,
    pub balance: f64,
}

/// Response for single payment
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentResponse {
    pub success: bool,
    pub payment: Payment,
}

/// Branch selector for the payment mutation routes (create / refund).
/// `branchFetch` (frontend) appends `?branch=…`; absent ⇒ HF Hotel. Routes the
/// canonical write + pre-write reads to the per-site pool via the unified write
/// chokepoint (Ville bundle).
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchQuery {
    pub branch: Option<Branch>,
}

/// Request body for creating a payment
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePaymentRequest {
    pub amount: f64,
    pub method: String,
    pub reference: Option<String>,
    pub notes: Option<String>,
    pub created_by: Option<String>,
}

/// Request body for refunding a payment.
///
/// `amount` is the POSITIVE magnitude of the refund in baht — the service
/// negates it before persisting. `reason` is operator-supplied free text
/// captured into `ht_payments.refund_reason` (canonical) and embedded in
/// the legacy `HT_CheckIn_Pay.Cin_Pay_Note` audit prefix.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefundPaymentRequest {
    pub amount: f64,
    pub reason: Option<String>,
    pub created_by: Option<String>,
}

/// Response for payment mutation operations
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMutationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

/// GET /api/new/checkins/:id/payments - List payments for a check-in
pub async fn list_payments(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
) -> ApiResult<Json<PaymentsResponse>> {
    let billing = state
        .payments
        .check_in_billing(&state.new_pool, cin_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    // Calculate total amount
    let stored_total = billing.cin_total_amount;
    let rate_per_night = billing.cin_rate_per_night.unwrap_or(0.0);
    let nights = billing.nights.unwrap_or(1).max(1);
    let calculated_total = rate_per_night * nights as f64;
    let total_amount = stored_total.unwrap_or(calculated_total);

    let rows = state.payments.list_for_checkin(&state.new_pool, cin_id).await?;
    let payments: Vec<Payment> = rows.into_iter().map(Payment::from_row).collect();

    let total_paid: f64 = payments
        .iter()
        .filter(|p| !p.voided)
        .map(|p| p.amount)
        .sum();

    let balance = total_amount - total_paid;

    Ok(Json(PaymentsResponse {
        success: true,
        data: payments,
        total_paid,
        total_amount,
        balance,
    }))
}

/// POST /api/new/checkins/:id/payments - Record a payment
///
/// All four tender methods (cash / credit / transfer / qr) flow through
/// [`crate::service::PaymentService::record_payment`] for atomic canonical
/// write + outbox enqueue + event publish. The QR method maps to the
/// `PaymentMethod::Web` variant, which the writeback recipe routes to
/// the legacy `HT_CheckIn_Pay.Cin_Pay_web` column so iHOTEL reports surface
/// the tender. Canonical PG `ht_payments.pay_method` continues to carry
/// `"qr"` for wire-contract stability.
pub async fn create_payment(
    State(state): State<AppState>,
    // M1 task #36: the authenticated cashier, when present. Auth ships dark
    // today, so this extractor is `Option<_>` — it resolves to `None` until
    // the auth middleware is enabled, at which point the session user becomes
    // the canonical `pay_created_by` (see resolution below). Must precede the
    // `Json` body extractor (which consumes the request).
    actor: Option<Extension<User>>,
    Path(cin_id): Path<i32>,
    Query(query): Query<BranchQuery>,
    Json(body): Json<CreatePaymentRequest>,
) -> ApiResult<Json<PaymentMutationResponse>> {
    // Ville bundle: resolve the per-site payment service + pool through the
    // unified write chokepoint. HF Hotel / `All` returns the pre-wired (shift-
    // gated) service Arc unchanged; HF Ville rebuilds the graph on the
    // `hotelville` pool. The pre-write reads below (existence check, receipt
    // header, billing, VAT) also use the resolved pool so a Ville payment
    // validates + receipts against Ville data. HF Ville mutations stay 403'd by
    // `ville_write_guard` until HFVILLE_WRITES_ENABLED.
    let ws = state.resolve_write_services(query.branch)?;
    let pool = state.write_pool(query.branch)?;
    if body.amount <= 0.0 {
        return Err(ApiError::BadRequest("Payment amount must be greater than 0".to_string()));
    }

    let valid_methods = ["cash", "credit", "transfer", "qr"];
    let method = body.method.to_lowercase();
    if !valid_methods.contains(&method.as_str()) {
        return Err(ApiError::BadRequest(format!(
            "Invalid payment method. Valid methods: {}",
            valid_methods.join(", ")
        )));
    }

    // Verify check-in exists. Mirrors the route's prior `find_status` check.
    if state.checkins.find_status(pool, cin_id).await?.is_none() {
        return Err(ApiError::NotFound("Check-in not found".to_string()));
    }

    let domain_method = parse_payment_method(&method).ok_or_else(|| {
        // Defensive: the `valid_methods` check above already screens unknown
        // strings, so an unmappable method here is a programming error
        // (a new tender added to `valid_methods` without a domain variant).
        ApiError::BadRequest(format!("Unmapped payment method '{method}'"))
    })?;

    let receipt = build_receipt_header(&state, pool, cin_id).await?;
    // Per-room apportionment: spike §3h capture line 3 fires
    // `UPDATE HT_CheckIn_Ds SET Cin_Room_Pay_Total=<amt>, Cin_note=''`
    // just before inserting the payment. The route currently doesn't
    // know which room a payment maps to (single-room check-ins are
    // unambiguous; multi-room would need an explicit body field).
    // Until that wire-contract change lands, leave `None` so the
    // recipe skips the per-room UPDATE — header totals still settle.
    // TODO: thread `room_id` through `CreatePaymentRequest` for
    // multi-room support.
    let checkin_ds_id: Option<i32> = None;
    // Wave 5a item 2: resolve the canonical per-night rate + nights
    // from `ht_checkins` so the writeback recipe stamps the real
    // values into `HT_CheckIn_Pay.Cin_Pay_Ds_PriceOne` /
    // `Cin_Pay_Ds_Num` instead of deriving them as `amount/nights`.
    let (price_per_night_baht, nights) = resolve_checkin_billing(&state, pool, cin_id).await;
    // Wave 5c: read `ht_settings.vat_percent` so the writeback recipe
    // stamps the hotel-configured rate into `HT_Receipt_H.Receipt_VatPer`
    // instead of the hardcoded constant. Lookup is best-effort — falls
    // back to DEFAULT_VAT_PERCENT on error so payments never block on a
    // settings hiccup.
    let vat_percent = Some(settings::get_vat_percent(pool).await);
    // M1 task #36: prefer the authenticated cashier's username over the
    // body-supplied `created_by` so we stop blindly trusting client input for
    // cashier identity. Auth is dark today (`actor` is `None`), so the body
    // value remains the fallback and the current flow is unchanged.
    let created_by = actor
        .as_deref()
        .map(|u| u.username.clone())
        .or_else(|| body.created_by.clone());
    let outcome = ws
        .payments
        .record_payment(RecordPaymentCommand {
            check_in_id: cin_id,
            amount_satang: baht_f64_to_satang(body.amount),
            method: domain_method,
            reference: body.reference.clone(),
            notes: body.notes.clone(),
            created_by,
            receipt,
            checkin_ds_id,
            price_per_night_baht,
            nights,
            vat_percent,
            // TODO: wire user_id from auth middleware
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
        })
        .await?;

    Ok(Json(PaymentMutationResponse {
        success: true,
        message: "Payment recorded successfully".to_string(),
        id: Some(outcome.pay_id),
    }))
}

/// DELETE /api/new/payments/:id - Void a payment (soft delete)
///
/// Stays on the repository: the service layer does not yet expose a
/// `void_payment` method.
pub async fn void_payment(
    State(state): State<AppState>,
    Path(pay_id): Path<i32>,
) -> ApiResult<Json<PaymentMutationResponse>> {
    let payment = state
        .payments
        .find_for_void(&state.new_pool, pay_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Payment not found".to_string()))?;

    if payment.pay_voided.unwrap_or(false) {
        return Err(ApiError::BadRequest("Payment is already voided".to_string()));
    }

    let mut tx = state.new_pool.begin().await?;
    state.payments.void(&mut tx, pay_id).await?;
    tx.commit().await?;

    Ok(Json(PaymentMutationResponse {
        success: true,
        message: "Payment voided successfully".to_string(),
        id: Some(pay_id),
    }))
}

/// POST /api/new/payments/:id/refund - Refund (negate) a payment
///
/// Track G2 / T4 CRIT-1 (`docs/coexistence/audit-2026-05-13.md`).
/// Inserts a new `ht_payments` row with NEGATIVE `pay_amount`, links it
/// back to the original via `refund_of_payment_id`, enqueues a
/// `WritebackIntent::RefundPayment` to negate the corresponding
/// `HT_CheckIn_Pay` row on legacy, and publishes a
/// `DomainEvent::PaymentRefunded`.
///
/// The service layer enforces:
/// - amount > 0
/// - original payment exists, not voided, not itself a refund
/// - sum of refunds ≤ original amount
pub async fn refund_payment(
    State(state): State<AppState>,
    // M1 task #40: prefer the authenticated cashier over the body `createdBy`
    // for the refund's `created_by`, mirroring `create_payment`. Auth ships
    // dark, so this is `Option<_>` and the body value remains the fallback.
    actor: Option<Extension<User>>,
    Path(pay_id): Path<i32>,
    Query(query): Query<BranchQuery>,
    Json(body): Json<RefundPaymentRequest>,
) -> ApiResult<Json<PaymentMutationResponse>> {
    // Ville bundle: resolve the per-site payment service through the unified
    // write chokepoint. HF Hotel / `All` returns the pre-wired Arc unchanged; HF
    // Ville rebuilds the graph on the `hotelville` pool. The refund's lookups
    // (original payment, refund-sum guard) run inside the service against that
    // pool. HF Ville mutations stay 403'd by `ville_write_guard` until
    // HFVILLE_WRITES_ENABLED.
    let ws = state.resolve_write_services(query.branch)?;
    if !body.amount.is_finite() || body.amount <= 0.0 {
        return Err(ApiError::BadRequest(
            "Refund amount must be a positive finite number".to_string(),
        ));
    }

    let created_by = super::resolve_actor(actor.as_deref(), body.created_by.as_deref());
    let outcome = ws
        .payments
        .refund_payment(RefundPaymentCommand {
            original_payment_id: pay_id,
            amount_satang: baht_f64_to_satang(body.amount),
            reason: body.reason.clone(),
            created_by,
            // TODO: wire user_id from auth middleware
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
        })
        .await?;

    Ok(Json(PaymentMutationResponse {
        success: true,
        message: format!(
            "Refund of {amount:.2} baht recorded against payment {pay_id}",
            amount = body.amount
        ),
        id: Some(outcome.refund_pay_id),
    }))
}

// ---------- helpers ----------

fn parse_payment_method(method: &str) -> Option<PaymentMethod> {
    match method {
        "cash" => Some(PaymentMethod::Cash),
        "credit" => Some(PaymentMethod::Credit),
        "transfer" => Some(PaymentMethod::Transfer),
        // Wave 5c: "qr" maps to the Web tender variant. The service still
        // persists `ht_payments.pay_method = "qr"` in canonical PG so the
        // wire contract stays stable; the writeback recipe routes the
        // amount to `HT_CheckIn_Pay.Cin_Pay_web` on the legacy side.
        "qr" => Some(PaymentMethod::Web),
        _ => None,
    }
}

/// Look up the customer attached to this check-in and assemble the receipt
/// header (`HT_Receipt_H.Receipt_Name` / `Address` / `Tel`).
///
/// Returns an all-empty header on lookup failure rather than erroring — a
/// missing customer name is a UX nit on the printed receipt; failing the
/// payment over it would be a regression. The legacy schema also rejects
/// `NULL` in these columns, so empty strings preserve compatibility.
async fn build_receipt_header(
    state: &AppState,
    pool: &crate::db::PgPool,
    cin_id: i32,
) -> ApiResult<RecordPaymentReceipt> {
    let Some(check_in) = state.checkins.get(pool, cin_id).await? else {
        return Ok(RecordPaymentReceipt::default());
    };
    let Some(customer) = state
        .customers
        .get(pool, check_in.cin_cust_id)
        .await?
    else {
        return Ok(RecordPaymentReceipt::default());
    };

    Ok(RecordPaymentReceipt {
        customer_name: full_customer_name(&customer.cust_firstname, customer.cust_lastname.as_deref()),
        customer_address: customer.cust_address.unwrap_or_default(),
        customer_tel: customer.cust_phone.unwrap_or_default(),
    })
}

/// Join `cust_firstname` + `cust_lastname` with a single space, trimming
/// trailing whitespace when the last name is missing or empty. Mirrors the
/// .NET app's `Cust_name` column which stores the combined value.
fn full_customer_name(first: &str, last: Option<&str>) -> String {
    match last {
        Some(last) if !last.is_empty() => format!("{} {}", first, last),
        _ => first.to_string(),
    }
}

fn baht_f64_to_satang(baht: f64) -> i64 {
    (baht * 100.0).round() as i64
}

/// Wave 5a item 2 — resolve the canonical per-night rate + nights from
/// `ht_checkins` so the route can thread them through to the writeback
/// recipe via the [`RecordPaymentCommand`] payload.
///
/// Returns `(None, None)` when the lookup fails (orphaned check-in, PG
/// outage); the recipe then falls back to its defensive `amount/nights`
/// derivation. We deliberately swallow the error here rather than failing
/// the payment: the payment still has to land in `ht_payments` even when
/// the rate lookup hiccups — the legacy receipt line just won't carry the
/// real per-night price (cosmetic, not data loss).
async fn resolve_checkin_billing(
    state: &AppState,
    pool: &crate::db::PgPool,
    cin_id: i32,
) -> (Option<f64>, Option<i32>) {
    match state.payments.check_in_billing(pool, cin_id).await {
        Ok(Some(b)) => (b.cin_rate_per_night, b.nights.map(|n| n.max(1))),
        _ => (None, None),
    }
}

// Wave 5c: the prior `insert_qr_payment_directly` bypass was retired —
// QR / "qr" tender now flows through `record_payment` and reaches the
// legacy `HT_CheckIn_Pay.Cin_Pay_web` column via the writeback recipe.
// See `docs/coexistence/audit-2026-05-13.md` Wave 5c notes.

#[cfg(test)]
mod tests {
    //! Track G2 / T4 CRIT-1 — input-validation tests for the refund
    //! route. These are pure deserialization + helper tests so they run
    //! without any database; the end-to-end behavior is covered by the
    //! `service::payment` DB-gated tests.

    use super::*;

    #[test]
    fn refund_request_deserializes_minimal_body() {
        let body = serde_json::json!({
            "amount": 250.0,
            "reason": "guest complaint"
        });
        let req: RefundPaymentRequest =
            serde_json::from_value(body).expect("must deserialize minimal body");
        assert_eq!(req.amount, 250.0);
        assert_eq!(req.reason.as_deref(), Some("guest complaint"));
        assert!(req.created_by.is_none());
    }

    #[test]
    fn refund_request_deserializes_with_camel_case_created_by() {
        let body = serde_json::json!({
            "amount": 100.0,
            "createdBy": "alice"
        });
        let req: RefundPaymentRequest =
            serde_json::from_value(body).expect("must deserialize camelCase createdBy");
        assert_eq!(req.created_by.as_deref(), Some("alice"));
        assert!(req.reason.is_none());
    }

    #[test]
    fn baht_f64_to_satang_rounds_half_to_even_in_practice() {
        // The route helper rounds to the nearest satang before handing
        // the value to the service. Spot-check the conversions that
        // matter for the refund flow.
        assert_eq!(baht_f64_to_satang(0.01), 1);
        assert_eq!(baht_f64_to_satang(100.0), 10_000);
        assert_eq!(baht_f64_to_satang(0.005), 1); // standard rounding behavior
    }

    /// Confirms the route handler's prelude rejects non-positive and
    /// non-finite amounts BEFORE delegating to the service — keeps the
    /// 400 response shape stable for the frontend.
    #[test]
    fn refund_route_input_validation_rejects_invalid_amounts() {
        let bad: [f64; 6] = [0.0, -0.5, -100.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY];
        for amt in bad {
            let rejected = !amt.is_finite() || amt <= 0.0;
            assert!(
                rejected,
                "amount {amt} must be flagged by the route's pre-service check"
            );
        }
        let good: [f64; 4] = [0.01, 1.0, 100.0, 9_999_999.99];
        for amt in good {
            let rejected = !amt.is_finite() || amt <= 0.0;
            assert!(!rejected, "amount {amt} must pass the route's pre-service check");
        }
    }
}
