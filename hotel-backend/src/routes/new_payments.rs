//! New Payment API routes for HotelNew database
//!
//! - GET /api/new/checkins/:id/payments - List payments for a check-in
//! - POST /api/new/checkins/:id/payments - Record a payment
//! - DELETE /api/new/payments/:id - Void a payment (soft delete)
//!
//! Per `docs/architecture.md` §1, §6 (Phase 2.5) the create handler
//! delegates to `state.payments_service` for the three legacy-known tender
//! methods (cash / credit / transfer) so the canonical write, outbox
//! enqueue, and event publish all happen atomically. The QR method has no
//! legacy counterpart yet (and no `PaymentMethod::Qr` variant in the
//! domain enum); that path stays on the repository so the wire contract
//! preserves the literal `"qr"` value in `ht_payments.pay_method`. The
//! list + void handlers stay on the repository (reads + a soft-delete
//! that has no service method yet).

use axum::{
    extract::{Path, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::mode::AppState;
use crate::domain::payment::PaymentMethod;
use crate::error::{ApiError, ApiResult};
use crate::outbox::event::EventSource;
use crate::outbox::intent::RecordPaymentReceipt;
use crate::repository::payment::{PaymentInsert, PaymentRow};
use crate::service::RecordPaymentCommand;

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
/// Cash / credit / transfer flow through [`crate::service::PaymentService::record_payment`]
/// for atomic canonical write + outbox enqueue + event publish. The legacy
/// "qr" tender has no `PaymentMethod` variant yet, so it falls back to a
/// direct repository insert — preserving the literal `"qr"` string value
/// in `ht_payments.pay_method` for the wire contract.
pub async fn create_payment(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
    Json(body): Json<CreatePaymentRequest>,
) -> ApiResult<Json<PaymentMutationResponse>> {
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
    if state.checkins.find_status(&state.new_pool, cin_id).await?.is_none() {
        return Err(ApiError::NotFound("Check-in not found".to_string()));
    }

    let pay_id = match parse_payment_method(&method) {
        Some(domain_method) => {
            let receipt = build_receipt_header(&state, cin_id).await?;
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
            let (price_per_night_baht, nights) = resolve_checkin_billing(&state, cin_id).await;
            let outcome = state
                .payments_service
                .record_payment(RecordPaymentCommand {
                    check_in_id: cin_id,
                    amount_satang: baht_f64_to_satang(body.amount),
                    method: domain_method,
                    reference: body.reference.clone(),
                    notes: body.notes.clone(),
                    created_by: body.created_by.clone(),
                    receipt,
                    checkin_ds_id,
                    price_per_night_baht,
                    nights,
                    // TODO: wire user_id from auth middleware
                    source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
                })
                .await?;
            outcome.pay_id
        }
        None => insert_qr_payment_directly(&state, cin_id, &body, &method).await?,
    };

    Ok(Json(PaymentMutationResponse {
        success: true,
        message: "Payment recorded successfully".to_string(),
        id: Some(pay_id),
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

// ---------- helpers ----------

fn parse_payment_method(method: &str) -> Option<PaymentMethod> {
    match method {
        "cash" => Some(PaymentMethod::Cash),
        "credit" => Some(PaymentMethod::Credit),
        "transfer" => Some(PaymentMethod::Transfer),
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
    cin_id: i32,
) -> ApiResult<RecordPaymentReceipt> {
    let Some(check_in) = state.checkins.get(&state.new_pool, cin_id).await? else {
        return Ok(RecordPaymentReceipt::default());
    };
    let Some(customer) = state
        .customers
        .get(&state.new_pool, check_in.cin_cust_id)
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
    cin_id: i32,
) -> (Option<f64>, Option<i32>) {
    match state.payments.check_in_billing(&state.new_pool, cin_id).await {
        Ok(Some(b)) => (b.cin_rate_per_night, b.nights.map(|n| n.max(1))),
        _ => (None, None),
    }
}

/// Direct repository insert for the "qr" tender method. Preserves the
/// literal column value (`pay_method = 'qr'`) since the domain
/// [`PaymentMethod`] enum has no `Qr` variant today.
async fn insert_qr_payment_directly(
    state: &AppState,
    cin_id: i32,
    body: &CreatePaymentRequest,
    method_lc: &str,
) -> ApiResult<i32> {
    let mut tx = state.new_pool.begin().await?;
    let pay_id = state
        .payments
        .insert(
            &mut tx,
            PaymentInsert {
                cin_id,
                amount: body.amount,
                method: method_lc,
                reference: body.reference.as_deref(),
                notes: body.notes.as_deref(),
                created_by: body.created_by.as_deref(),
            },
        )
        .await?;
    tx.commit().await?;
    Ok(pay_id)
}
