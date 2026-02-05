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
    let mut conn = state.new_pool.get().await?;

    // Get check-in with all related data
    let rows = conn
        .query(
            r#"
            SELECT
                -- Check-in info
                ci.Cin_ID,
                ci.Cin_No,
                ci.Cin_Book_ID,
                b.Book_No,

                -- Customer/Guest info
                c.Cust_ID,
                c.Cust_FirstName,
                c.Cust_LastName,
                c.Cust_Email,
                c.Cust_Phone,
                c.Cust_Address,
                c.Cust_IDCard,
                c.Cust_Passport,

                -- Room info
                r.Room_ID,
                r.Room_No,
                r.Room_Floor,
                rt.Type_Name as Room_Type,

                -- Stay details
                ci.Cin_CheckIn_Time,
                ci.Cin_CheckOut_Time,
                ci.Cin_Expected_Checkout,
                ci.Cin_Adults,
                ci.Cin_Children,

                -- Rate info
                ci.Cin_Rate_Per_Night,
                DATEDIFF(day, ci.Cin_CheckIn_Time,
                    COALESCE(ci.Cin_CheckOut_Time, ci.Cin_Expected_Checkout)) as Nights,

                -- Totals
                ci.Cin_Total_Amount,
                ci.Cin_Payment_Status,

                -- Notes and timestamps
                ci.Cin_Notes,
                ci.Created_At
            FROM HT_CheckIns ci
            LEFT JOIN HT_Customers c ON ci.Cin_Cust_ID = c.Cust_ID
            LEFT JOIN HT_Rooms_New r ON ci.Cin_Room_ID = r.Room_ID
            LEFT JOIN HT_Room_Types rt ON r.Room_Type_ID = rt.Type_ID
            LEFT JOIN HT_Bookings b ON ci.Cin_Book_ID = b.Book_ID
            WHERE ci.Cin_ID = @P1
            "#,
            &[&cin_id],
        )
        .await?
        .into_first_result()
        .await?;

    let row = rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    // Extract customer info
    let first_name = row.get::<&str, _>("Cust_FirstName").unwrap_or_default().to_string();
    let last_name = row.get::<&str, _>("Cust_LastName").map(String::from);
    let full_name = match &last_name {
        Some(ln) => format!("{} {}", first_name, ln),
        None => first_name.clone(),
    };

    let guest = InvoiceGuest {
        id: row.get::<i32, _>("Cust_ID").unwrap_or(0),
        first_name,
        last_name,
        full_name,
        email: row.get::<&str, _>("Cust_Email").map(String::from),
        phone: row.get::<&str, _>("Cust_Phone").map(String::from),
        address: row.get::<&str, _>("Cust_Address").map(String::from),
        id_card: row.get::<&str, _>("Cust_IDCard").map(String::from),
        passport: row.get::<&str, _>("Cust_Passport").map(String::from),
    };

    // Extract room info
    let room = InvoiceRoom {
        room_id: row.get::<i32, _>("Room_ID").unwrap_or(0),
        room_no: row.get::<&str, _>("Room_No").unwrap_or_default().to_string(),
        room_type: row.get::<&str, _>("Room_Type").map(String::from),
        floor: row.get::<i32, _>("Room_Floor"),
    };

    // Extract rate info
    let rate_per_night = row.get::<f64, _>("Cin_Rate_Per_Night").unwrap_or(0.0);
    let nights = row.get::<i32, _>("Nights").unwrap_or(1).max(1);
    let subtotal = rate_per_night * nights as f64;

    let rates = InvoiceRates {
        rate_per_night,
        nights,
        subtotal,
    };

    // Get total amount (use calculated if not stored)
    let total_amount = row.get::<f64, _>("Cin_Total_Amount").unwrap_or(subtotal);

    let invoice = Invoice {
        checkin_id: row.get::<i32, _>("Cin_ID").unwrap_or(0),
        cin_no: row.get::<&str, _>("Cin_No").unwrap_or_default().to_string(),
        booking_id: row.get::<i32, _>("Cin_Book_ID"),
        booking_no: row.get::<&str, _>("Book_No").map(String::from),
        guest,
        room,
        check_in_time: row.get::<NaiveDateTime, _>("Cin_CheckIn_Time"),
        check_out_time: row.get::<NaiveDateTime, _>("Cin_CheckOut_Time"),
        expected_checkout: row.get::<NaiveDateTime, _>("Cin_Expected_Checkout"),
        adults: row.get::<i32, _>("Cin_Adults").unwrap_or(1),
        children: row.get::<i32, _>("Cin_Children").unwrap_or(0),
        rates,
        total_amount,
        payment_status: row.get::<&str, _>("Cin_Payment_Status").map(String::from),
        notes: row.get::<&str, _>("Cin_Notes").map(String::from),
        created_at: row.get::<NaiveDateTime, _>("Created_At"),
    };

    Ok(Json(InvoiceResponse {
        success: true,
        invoice,
    }))
}
