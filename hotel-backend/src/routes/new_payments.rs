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
    let mut conn = state.new_pool.get().await?;

    // Verify check-in exists and get total amount
    let checkin_rows = conn
        .query(
            r#"
            SELECT Cin_ID, Cin_Total_Amount, Cin_Rate_Per_Night,
                   DATEDIFF(day, Cin_CheckIn_Time,
                       COALESCE(Cin_CheckOut_Time, Cin_Expected_Checkout)) as Nights
            FROM HT_CheckIns
            WHERE Cin_ID = @P1
            "#,
            &[&cin_id],
        )
        .await?
        .into_first_result()
        .await?;

    let checkin_row = checkin_rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    // Calculate total amount
    let stored_total = checkin_row.get::<f64, _>("Cin_Total_Amount");
    let rate_per_night = checkin_row.get::<f64, _>("Cin_Rate_Per_Night").unwrap_or(0.0);
    let nights = checkin_row.get::<i32, _>("Nights").unwrap_or(1).max(1);
    let calculated_total = rate_per_night * nights as f64;
    let total_amount = stored_total.unwrap_or(calculated_total);

    // Get all non-voided payments
    let rows = conn
        .query(
            r#"
            SELECT
                Pay_ID,
                Pay_Cin_ID,
                Pay_Amount,
                Pay_Method,
                Pay_Reference,
                Pay_Notes,
                Pay_Date,
                Pay_Created_By,
                Pay_Voided,
                Pay_Voided_At,
                Pay_Voided_By,
                Created_At
            FROM HT_Payments
            WHERE Pay_Cin_ID = @P1
            ORDER BY Pay_Date DESC, Pay_ID DESC
            "#,
            &[&cin_id],
        )
        .await?
        .into_first_result()
        .await?;

    let payments: Vec<Payment> = rows
        .iter()
        .map(|row| Payment {
            id: row.get::<i32, _>("Pay_ID").unwrap_or(0),
            cin_id: row.get::<i32, _>("Pay_Cin_ID").unwrap_or(0),
            amount: row.get::<f64, _>("Pay_Amount").unwrap_or(0.0),
            method: row.get::<&str, _>("Pay_Method").unwrap_or_default().to_string(),
            reference: row.get::<&str, _>("Pay_Reference").map(String::from),
            notes: row.get::<&str, _>("Pay_Notes").map(String::from),
            pay_date: row.get::<NaiveDateTime, _>("Pay_Date"),
            created_by: row.get::<&str, _>("Pay_Created_By").map(String::from),
            voided: row.get::<bool, _>("Pay_Voided").unwrap_or(false),
            voided_at: row.get::<NaiveDateTime, _>("Pay_Voided_At"),
            voided_by: row.get::<&str, _>("Pay_Voided_By").map(String::from),
            created_at: row.get::<NaiveDateTime, _>("Created_At"),
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

    let mut conn = state.new_pool.get().await?;

    // Verify check-in exists
    let checkin_rows = conn
        .query(
            "SELECT Cin_ID, Cin_Status FROM HT_CheckIns WHERE Cin_ID = @P1",
            &[&cin_id],
        )
        .await?
        .into_first_result()
        .await?;

    if checkin_rows.is_empty() {
        return Err(ApiError::NotFound("Check-in not found".to_string()));
    }

    // Insert payment
    let rows = conn
        .query(
            r#"
            INSERT INTO HT_Payments (
                Pay_Cin_ID,
                Pay_Amount,
                Pay_Method,
                Pay_Reference,
                Pay_Notes,
                Pay_Created_By
            )
            OUTPUT INSERTED.Pay_ID
            VALUES (@P1, @P2, @P3, @P4, @P5, @P6)
            "#,
            &[
                &cin_id,
                &body.amount,
                &method.as_str(),
                &body.reference.as_deref(),
                &body.notes.as_deref(),
                &body.created_by.as_deref(),
            ],
        )
        .await?
        .into_first_result()
        .await?;

    let pay_id = rows
        .first()
        .and_then(|r| r.get::<i32, _>("Pay_ID"))
        .ok_or_else(|| ApiError::Internal("Failed to create payment".to_string()))?;

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
    let mut conn = state.new_pool.get().await?;

    // Verify payment exists and is not already voided
    let payment_rows = conn
        .query(
            "SELECT Pay_ID, Pay_Voided FROM HT_Payments WHERE Pay_ID = @P1",
            &[&pay_id],
        )
        .await?
        .into_first_result()
        .await?;

    let payment_row = payment_rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Payment not found".to_string()))?;

    let already_voided = payment_row.get::<bool, _>("Pay_Voided").unwrap_or(false);
    if already_voided {
        return Err(ApiError::BadRequest("Payment is already voided".to_string()));
    }

    // Void the payment (soft delete)
    conn.execute(
        r#"
        UPDATE HT_Payments
        SET Pay_Voided = 1,
            Pay_Voided_At = GETDATE()
        WHERE Pay_ID = @P1
        "#,
        &[&pay_id],
    )
    .await?;

    Ok(Json(PaymentMutationResponse {
        success: true,
        message: "Payment voided successfully".to_string(),
        id: Some(pay_id),
    }))
}
