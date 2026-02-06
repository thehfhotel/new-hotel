//! New Payment API routes for HotelNew database
//!
//! - GET /api/new/checkins/:id/payments - List payments for a check-in
//! - POST /api/new/checkins/:id/payments - Record a payment
//! - DELETE /api/new/payments/:id - Void a payment (soft delete)

use axum::{
    extract::{Path, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::mode::AppState;
use crate::error::{ApiError, ApiResult};

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
    let pool = &state.new_pool;

    // Verify check-in exists and get total amount
    let checkin_rec = sqlx::query!(
        r#"SELECT cin_id, cin_total_amount::float8 as cin_total_amount, cin_rate_per_night::float8 as cin_rate_per_night,
            (COALESCE(cin_checkout_time, cin_expected_checkout)::date - cin_checkin_time::date) as nights
        FROM ht_checkins WHERE cin_id = $1"#,
        cin_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    // Calculate total amount
    let stored_total = checkin_rec.cin_total_amount;
    let rate_per_night = checkin_rec.cin_rate_per_night.unwrap_or(0.0);
    let nights = checkin_rec.nights.unwrap_or(1).max(1);
    let calculated_total = rate_per_night * nights as f64;
    let total_amount = stored_total.unwrap_or(calculated_total);

    // Get all non-voided payments
    let rows = sqlx::query!(
        r#"SELECT pay_id, pay_cin_id, pay_amount::float8 as pay_amount, pay_method, pay_reference,
            pay_notes, pay_date, pay_created_by, pay_voided, pay_voided_at, pay_voided_by, created_at
        FROM ht_payments WHERE pay_cin_id = $1
        ORDER BY pay_date DESC, pay_id DESC"#,
        cin_id
    )
    .fetch_all(pool)
    .await?;

    let payments: Vec<Payment> = rows
        .iter()
        .map(|r| Payment {
            id: r.pay_id,
            cin_id: r.pay_cin_id,
            amount: r.pay_amount.unwrap_or(0.0),
            method: r.pay_method.clone(),
            reference: r.pay_reference.clone(),
            notes: r.pay_notes.clone(),
            pay_date: r.pay_date,
            created_by: r.pay_created_by.clone(),
            voided: r.pay_voided.unwrap_or(false),
            voided_at: r.pay_voided_at,
            voided_by: r.pay_voided_by.clone(),
            created_at: r.created_at,
        })
        .collect();

    // Calculate total paid (excluding voided payments)
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
    // Validate amount
    if body.amount <= 0.0 {
        return Err(ApiError::BadRequest("Payment amount must be greater than 0".to_string()));
    }

    // Validate method
    let valid_methods = ["cash", "credit", "transfer", "qr"];
    let method = body.method.to_lowercase();
    if !valid_methods.contains(&method.as_str()) {
        return Err(ApiError::BadRequest(format!(
            "Invalid payment method. Valid methods: {}",
            valid_methods.join(", ")
        )));
    }

    let pool = &state.new_pool;

    // Verify check-in exists
    let exists = sqlx::query!(
        "SELECT cin_id, cin_status FROM ht_checkins WHERE cin_id = $1",
        cin_id
    )
    .fetch_optional(pool)
    .await?;

    if exists.is_none() {
        return Err(ApiError::NotFound("Check-in not found".to_string()));
    }

    // Insert payment
    let rec = sqlx::query!(
        r#"INSERT INTO ht_payments (pay_cin_id, pay_amount, pay_method, pay_reference, pay_notes, pay_created_by)
        VALUES ($1, $2::float8, $3, $4, $5, $6) RETURNING pay_id"#,
        cin_id,
        body.amount,
        method.as_str(),
        body.reference.as_deref(),
        body.notes.as_deref(),
        body.created_by.as_deref()
    )
    .fetch_one(pool)
    .await?;

    let pay_id = rec.pay_id;

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
    let pool = &state.new_pool;

    // Verify payment exists and is not already voided
    let payment_rec = sqlx::query!(
        "SELECT pay_id, pay_voided FROM ht_payments WHERE pay_id = $1",
        pay_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Payment not found".to_string()))?;

    let already_voided = payment_rec.pay_voided.unwrap_or(false);
    if already_voided {
        return Err(ApiError::BadRequest("Payment is already voided".to_string()));
    }

    // Void the payment (soft delete)
    sqlx::query!(
        r#"UPDATE ht_payments SET pay_voided = true, pay_voided_at = NOW() WHERE pay_id = $1"#,
        pay_id
    )
    .execute(pool)
    .await?;

    Ok(Json(PaymentMutationResponse {
        success: true,
        message: "Payment voided successfully".to_string(),
        id: Some(pay_id),
    }))
}
