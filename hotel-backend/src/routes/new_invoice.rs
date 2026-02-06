//! New Invoice API routes for HotelNew database
//!
//! - GET /api/new/checkins/:id/invoice - Get complete invoice data for a check-in

use axum::{
    extract::{Path, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::Serialize;

use super::mode::AppState;
use crate::error::{ApiError, ApiResult};

/// Guest details for invoice
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceGuest {
    pub id: i32,
    pub first_name: String,
    pub last_name: Option<String>,
    pub full_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub id_card: Option<String>,
    pub passport: Option<String>,
}

/// Room assignment for invoice
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceRoom {
    pub room_id: i32,
    pub room_no: String,
    pub room_type: Option<String>,
    pub floor: Option<i32>,
}

/// Rate details for invoice
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceRates {
    pub rate_per_night: f64,
    pub nights: i32,
    pub subtotal: f64,
}

/// Complete invoice data
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    // Check-in info
    pub checkin_id: i32,
    pub cin_no: String,
    pub booking_id: Option<i32>,
    pub booking_no: Option<String>,

    // Guest details
    pub guest: InvoiceGuest,

    // Room assignment
    pub room: InvoiceRoom,

    // Stay details
    pub check_in_time: Option<NaiveDateTime>,
    pub check_out_time: Option<NaiveDateTime>,
    pub expected_checkout: Option<NaiveDateTime>,
    pub adults: i32,
    pub children: i32,

    // Rate calculations
    pub rates: InvoiceRates,

    // Totals
    pub total_amount: f64,
    pub payment_status: Option<String>,

    // Notes
    pub notes: Option<String>,

    // Timestamps
    pub created_at: Option<NaiveDateTime>,
}

/// Response for invoice
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceResponse {
    pub success: bool,
    pub invoice: Invoice,
}

/// GET /api/new/checkins/:id/invoice - Get complete invoice data
pub async fn get_invoice(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
) -> ApiResult<Json<InvoiceResponse>> {
    let pool = &state.new_pool;

    // Get check-in with all related data
    let rec = sqlx::query!(
            r#"
            SELECT
                -- Check-in info
                ci.cin_id,
                ci.cin_no,
                ci.cin_book_id,
                b.book_no,

                -- Customer/Guest info
                c.cust_id,
                c.cust_firstname,
                c.cust_lastname,
                c.cust_email,
                c.cust_phone,
                c.cust_address,
                c.cust_idcard,
                c.cust_passport,

                -- Room info
                r.room_id,
                r.room_no,
                r.room_floor,
                rt.type_name as room_type,

                -- Stay details
                ci.cin_checkin_time,
                ci.cin_checkout_time,
                ci.cin_expected_checkout,
                ci.cin_adults,
                ci.cin_children,

                -- Rate info
                ci.cin_rate_per_night::float8 as cin_rate_per_night,
                (COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout)::date - ci.cin_checkin_time::date) as nights,

                -- Totals
                ci.cin_total_amount::float8 as cin_total_amount,
                ci.cin_payment_status,

                -- Notes and timestamps
                ci.cin_notes,
                ci.created_at
            FROM ht_checkins ci
            LEFT JOIN ht_customers c ON ci.cin_cust_id = c.cust_id
            LEFT JOIN ht_rooms_new r ON ci.cin_room_id = r.room_id
            LEFT JOIN ht_room_types rt ON r.room_type_id = rt.type_id
            LEFT JOIN ht_bookings b ON ci.cin_book_id = b.book_id
            WHERE ci.cin_id = $1
            "#,
            cin_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    // Extract customer info
    let first_name = rec.cust_firstname;
    let last_name = rec.cust_lastname.clone();
    let full_name = match &last_name {
        Some(ln) => format!("{} {}", first_name, ln),
        None => first_name.clone(),
    };

    let guest = InvoiceGuest {
        id: rec.cust_id,
        first_name,
        last_name,
        full_name,
        email: rec.cust_email,
        phone: rec.cust_phone,
        address: rec.cust_address,
        id_card: rec.cust_idcard,
        passport: rec.cust_passport,
    };

    // Extract room info
    let room = InvoiceRoom {
        room_id: rec.room_id,
        room_no: rec.room_no,
        room_type: Some(rec.room_type),
        floor: rec.room_floor,
    };

    // Extract rate info
    let rate_per_night = rec.cin_rate_per_night.unwrap_or(0.0);
    let nights = rec.nights.unwrap_or(1).max(1);
    let subtotal = rate_per_night * nights as f64;

    let rates = InvoiceRates {
        rate_per_night,
        nights,
        subtotal,
    };

    // Get total amount (use calculated if not stored)
    let total_amount = rec.cin_total_amount.unwrap_or(subtotal);

    let invoice = Invoice {
        checkin_id: rec.cin_id,
        cin_no: rec.cin_no,
        booking_id: rec.cin_book_id,
        booking_no: Some(rec.book_no),
        guest,
        room,
        check_in_time: Some(rec.cin_checkin_time),
        check_out_time: rec.cin_checkout_time,
        expected_checkout: Some(rec.cin_expected_checkout.and_hms_opt(0, 0, 0).unwrap()),
        adults: rec.cin_adults.unwrap_or(1),
        children: rec.cin_children.unwrap_or(0),
        rates,
        total_amount,
        payment_status: rec.cin_payment_status,
        notes: rec.cin_notes,
        created_at: rec.created_at,
    };

    Ok(Json(InvoiceResponse {
        success: true,
        invoice,
    }))
}
