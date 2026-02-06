//! New Invoice API routes for HotelNew database
//!
//! - GET /api/new/checkins/:id/invoice - Get complete invoice data for a check-in

use axum::{
    extract::{Path, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::Row;

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
    let rows = sqlx::query(
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
                ci.cin_rate_per_night,
                (COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout)::date - ci.cin_checkin_time::date) as nights,

                -- Totals
                ci.cin_total_amount,
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
        )
        .bind(&cin_id)
        .fetch_all(pool)
        .await?;

    let row = rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    // Extract customer info
    let first_name = row.try_get::<String, _>("cust_firstname").unwrap_or_default();
    let last_name = row.try_get::<String, _>("cust_lastname").ok();
    let full_name = match &last_name {
        Some(ln) => format!("{} {}", first_name, ln),
        None => first_name.clone(),
    };

    let guest = InvoiceGuest {
        id: row.try_get::<i32, _>("cust_id").unwrap_or(0),
        first_name,
        last_name,
        full_name,
        email: row.try_get::<String, _>("cust_email").ok(),
        phone: row.try_get::<String, _>("cust_phone").ok(),
        address: row.try_get::<String, _>("cust_address").ok(),
        id_card: row.try_get::<String, _>("cust_idcard").ok(),
        passport: row.try_get::<String, _>("cust_passport").ok(),
    };

    // Extract room info
    let room = InvoiceRoom {
        room_id: row.try_get::<i32, _>("room_id").unwrap_or(0),
        room_no: row.try_get::<String, _>("room_no").unwrap_or_default(),
        room_type: row.try_get::<String, _>("room_type").ok(),
        floor: row.try_get::<i32, _>("room_floor").ok(),
    };

    // Extract rate info
    let rate_per_night = row.try_get::<f64, _>("cin_rate_per_night").unwrap_or(0.0);
    let nights = row.try_get::<i32, _>("nights").unwrap_or(1).max(1);
    let subtotal = rate_per_night * nights as f64;

    let rates = InvoiceRates {
        rate_per_night,
        nights,
        subtotal,
    };

    // Get total amount (use calculated if not stored)
    let total_amount = row.try_get::<f64, _>("cin_total_amount").unwrap_or(subtotal);

    let invoice = Invoice {
        checkin_id: row.try_get::<i32, _>("cin_id").unwrap_or(0),
        cin_no: row.try_get::<String, _>("cin_no").unwrap_or_default(),
        booking_id: row.try_get::<i32, _>("cin_book_id").ok(),
        booking_no: row.try_get::<String, _>("book_no").ok(),
        guest,
        room,
        check_in_time: row.try_get::<NaiveDateTime, _>("cin_checkin_time").ok(),
        check_out_time: row.try_get::<NaiveDateTime, _>("cin_checkout_time").ok(),
        expected_checkout: row.try_get::<NaiveDateTime, _>("cin_expected_checkout").ok(),
        adults: row.try_get::<i32, _>("cin_adults").unwrap_or(1),
        children: row.try_get::<i32, _>("cin_children").unwrap_or(0),
        rates,
        total_amount,
        payment_status: row.try_get::<String, _>("cin_payment_status").ok(),
        notes: row.try_get::<String, _>("cin_notes").ok(),
        created_at: row.try_get::<NaiveDateTime, _>("created_at").ok(),
    };

    Ok(Json(InvoiceResponse {
        success: true,
        invoice,
    }))
}
