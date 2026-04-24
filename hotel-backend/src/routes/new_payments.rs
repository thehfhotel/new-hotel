//! New Payment API routes for HotelNew database
//!
//! - GET /api/new/checkins/:id/payments - List payments for a check-in
//! - POST /api/new/checkins/:id/payments - Record a payment
//! - DELETE /api/new/payments/:id - Void a payment (soft delete)
//!
//! Per `docs/architecture.md` §1, §6 (Phase 1b) the SQL has moved to
//! `repository::payment`. Validation, derived totals, and DTO mapping stay
//! here; the multi-step business logic moves to the service layer in Phase 2.

use axum::{
    extract::{Path, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::mode::AppState;
use crate::error::{ApiError, ApiResult};
use crate::repository::payment::{PaymentInsert, PaymentRow};

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

    // Verify check-in exists. Mirrors the route's prior `find_status`-shaped
    // existence check.
    if state.checkins.find_status(&state.new_pool, cin_id).await?.is_none() {
        return Err(ApiError::NotFound("Check-in not found".to_string()));
    }

    let mut tx = state.new_pool.begin().await?;
    let pay_id = state
        .payments
        .insert(
            &mut tx,
            PaymentInsert {
                cin_id,
                amount: body.amount,
                method: method.as_str(),
                reference: body.reference.as_deref(),
                notes: body.notes.as_deref(),
                created_by: body.created_by.as_deref(),
            },
        )
        .await?;
    tx.commit().await?;

    Ok(Json(PaymentMutationResponse {
        success: true,
        message: "Payment recorded successfully".to_string(),
        id: Some(pay_id),
    }))
}

/// DELETE /api/new/payments/:id - Void a payment (soft delete)
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
