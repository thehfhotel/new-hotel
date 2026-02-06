//! Calendar API route with hybrid database support
//!
//! - GET /api/calendar - Hybrid calendar endpoint fetching from both databases

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::error::{ApiError, ApiResult};
use super::mode::{AppState, SystemMode};

/// Query parameters for calendar endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarQuery {
    pub start_date: String,
    pub end_date: String,
}

/// Source database for a calendar entry
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DataSource {
    Legacy,
    New,
}

/// A booking entry for the calendar
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarBooking {
    pub id: String,
    pub booking_no: String,
    pub customer_name: Option<String>,
    pub room_no: Option<String>,
    pub check_in: Option<NaiveDateTime>,
    pub check_out: Option<NaiveDateTime>,
    pub status: Option<String>,
    pub source: DataSource,
}

/// A check-in entry for the calendar
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarCheckin {
    pub id: String,
    pub checkin_no: String,
    pub customer_name: Option<String>,
    pub room_no: Option<String>,
    pub check_in: Option<NaiveDateTime>,
    pub check_out: Option<NaiveDateTime>,
    pub source: DataSource,
}

/// Calendar data combining bookings and check-ins
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarData {
    pub bookings: Vec<CalendarBooking>,
    pub checkins: Vec<CalendarCheckin>,
}

/// Calendar response
#[derive(Debug, Serialize)]
pub struct CalendarResponse {
    pub success: bool,
    pub data: CalendarData,
    pub mode: SystemMode,
}

/// GET /api/calendar - Hybrid calendar endpoint
///
/// Fetches calendar data from both legacy and new databases.
/// In legacy mode, only legacy data is returned.
/// In new mode, data from both databases is returned with source indicators.
pub async fn get_calendar(
    State(state): State<AppState>,
    Query(params): Query<CalendarQuery>,
) -> ApiResult<Json<CalendarResponse>> {
    let mode = state.current_mode();

    // Validate date parameters
    if params.start_date.is_empty() || params.end_date.is_empty() {
        return Err(ApiError::BadRequest("startDate and endDate are required".to_string()));
    }

    let mut all_bookings: Vec<CalendarBooking> = Vec::new();
    let mut all_checkins: Vec<CalendarCheckin> = Vec::new();

    // Always fetch from legacy database
    let (legacy_bookings, legacy_checkins) = fetch_legacy_calendar_data(
        &state.legacy_pool,
        &params.start_date,
        &params.end_date,
    ).await?;

    all_bookings.extend(legacy_bookings);
    all_checkins.extend(legacy_checkins);

    // In new mode, also fetch from new_hotel database
    if mode == SystemMode::New {
        let (new_bookings, new_checkins) = fetch_new_calendar_data(
            &state.new_pool,
            &params.start_date,
            &params.end_date,
        ).await?;

        all_bookings.extend(new_bookings);
        all_checkins.extend(new_checkins);
    }

    Ok(Json(CalendarResponse {
        success: true,
        data: CalendarData {
            bookings: all_bookings,
            checkins: all_checkins,
        },
        mode,
    }))
}

/// Fetch calendar data from legacy database
async fn fetch_legacy_calendar_data(
    pool: &crate::db::DbPool,
    start_date: &str,
    end_date: &str,
) -> ApiResult<(Vec<CalendarBooking>, Vec<CalendarCheckin>)> {
    let mut conn = pool.get().await?;

    // Fetch bookings from legacy database
    let booking_query = format!(
        r#"
        SELECT
            Book_No,
            Book_Cust_Name,
            Book_Room_No,
            Book_Date_in,
            Book_Date_out,
            Book_Status
        FROM View_Booking_Ds
        WHERE CAST(Book_Date_out AS DATE) >= '{}'
          AND CAST(Book_Date_in AS DATE) <= '{}'
        ORDER BY Book_Date_in
        "#,
        start_date.replace('\'', "''"),
        end_date.replace('\'', "''")
    );

    let booking_rows = conn
        .simple_query(&booking_query)
        .await?
        .into_first_result()
        .await?;

    let bookings: Vec<CalendarBooking> = booking_rows
        .iter()
        .map(|row| {
            let book_no = row.get::<&str, _>("Book_No").unwrap_or_default().to_string();
            CalendarBooking {
                id: format!("legacy-booking-{}", book_no),
                booking_no: book_no,
                customer_name: row.get::<&str, _>("Book_Cust_Name").map(String::from),
                room_no: row.get::<&str, _>("Book_Room_No").map(String::from),
                check_in: row.get::<NaiveDateTime, _>("Book_Date_in"),
                check_out: row.get::<NaiveDateTime, _>("Book_Date_out"),
                status: row.get::<i32, _>("Book_Status").map(|s| map_legacy_status(s)),
                source: DataSource::Legacy,
            }
        })
        .collect();

    // Fetch check-ins from legacy database
    let checkin_query = format!(
        r#"
        SELECT
            Cin_CheckIn_No,
            Cin_Cust_Name,
            Cin_Room_No,
            Cin_Room_In,
            Cin_Room_Out
        FROM View_CheckIn_Ds
        WHERE CAST(Cin_Room_Out AS DATE) >= '{}'
          AND CAST(Cin_Room_In AS DATE) <= '{}'
        ORDER BY Cin_Room_In
        "#,
        start_date.replace('\'', "''"),
        end_date.replace('\'', "''")
    );

    let checkin_rows = conn
        .simple_query(&checkin_query)
        .await?
        .into_first_result()
        .await?;

    let checkins: Vec<CalendarCheckin> = checkin_rows
        .iter()
        .map(|row| {
            let cin_no = row.get::<&str, _>("Cin_CheckIn_No").unwrap_or_default().to_string();
            CalendarCheckin {
                id: format!("legacy-checkin-{}", cin_no),
                checkin_no: cin_no,
                customer_name: row.get::<&str, _>("Cin_Cust_Name").map(String::from),
                room_no: row.get::<&str, _>("Cin_Room_No").map(String::from),
                check_in: row.get::<NaiveDateTime, _>("Cin_Room_In"),
                check_out: row.get::<NaiveDateTime, _>("Cin_Room_Out"),
                source: DataSource::Legacy,
            }
        })
        .collect();

    Ok((bookings, checkins))
}

/// Fetch calendar data from new_hotel database (HotelNew)
///
/// Uses the schema defined in migrations/002_create_new_hotel_database.sql
async fn fetch_new_calendar_data(
    pool: &crate::db::PgPool,
    start_date: &str,
    end_date: &str,
) -> ApiResult<(Vec<CalendarBooking>, Vec<CalendarCheckin>)> {
    // Fetch bookings from HotelNew database (ht_bookings + ht_customers + ht_booking_rooms + ht_rooms_new)
    let booking_query = format!(
        r#"
        SELECT
            b.book_id,
            b.book_no,
            CONCAT(c.cust_firstname, ' ', COALESCE(c.cust_lastname, '')) AS cust_name,
            r.room_no,
            b.book_checkin,
            b.book_checkout,
            b.book_status
        FROM ht_bookings b
        INNER JOIN ht_customers c ON b.book_cust_id = c.cust_id
        LEFT JOIN ht_booking_rooms br ON b.book_id = br.br_book_id
        LEFT JOIN ht_rooms_new r ON br.br_room_id = r.room_id
        WHERE b.book_checkout >= '{}'
          AND b.book_checkin <= '{}'
          AND b.book_status != 'cancelled'
        ORDER BY b.book_checkin
        "#,
        start_date.replace('\'', "''"),
        end_date.replace('\'', "''")
    );

    let booking_rows = sqlx::query(&booking_query)
        .fetch_all(pool)
        .await?;

    let bookings: Vec<CalendarBooking> = booking_rows
        .iter()
        .map(|row| {
            let id = row.try_get::<i32, _>("book_id").unwrap_or_default();
            let booking_no = row.try_get::<String, _>("book_no").unwrap_or_default();
            CalendarBooking {
                id: format!("new-booking-{}", id),
                booking_no,
                customer_name: row.try_get::<String, _>("cust_name").ok(),
                room_no: row.try_get::<String, _>("room_no").ok(),
                check_in: row.try_get::<NaiveDateTime, _>("book_checkin").ok(),
                check_out: row.try_get::<NaiveDateTime, _>("book_checkout").ok(),
                status: row.try_get::<String, _>("book_status").ok(),
                source: DataSource::New,
            }
        })
        .collect();

    // Fetch check-ins from HotelNew database (ht_checkins + ht_customers + ht_rooms_new)
    let checkin_query = format!(
        r#"
        SELECT
            ci.cin_id,
            ci.cin_no,
            CONCAT(c.cust_firstname, ' ', COALESCE(c.cust_lastname, '')) AS cust_name,
            r.room_no,
            ci.cin_checkin_time,
            COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout) AS cin_checkout
        FROM ht_checkins ci
        INNER JOIN ht_customers c ON ci.cin_cust_id = c.cust_id
        INNER JOIN ht_rooms_new r ON ci.cin_room_id = r.room_id
        WHERE COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout) >= '{}'
          AND ci.cin_checkin_time <= '{}'
        ORDER BY ci.cin_checkin_time
        "#,
        start_date.replace('\'', "''"),
        end_date.replace('\'', "''")
    );

    let checkin_rows = sqlx::query(&checkin_query)
        .fetch_all(pool)
        .await?;

    let checkins: Vec<CalendarCheckin> = checkin_rows
        .iter()
        .map(|row| {
            let id = row.try_get::<i32, _>("cin_id").unwrap_or_default();
            let cin_no = row.try_get::<String, _>("cin_no").unwrap_or_default();
            CalendarCheckin {
                id: format!("new-checkin-{}", id),
                checkin_no: cin_no,
                customer_name: row.try_get::<String, _>("cust_name").ok(),
                room_no: row.try_get::<String, _>("room_no").ok(),
                check_in: row.try_get::<NaiveDateTime, _>("cin_checkin_time").ok(),
                check_out: row.try_get::<NaiveDateTime, _>("cin_checkout").ok(),
                source: DataSource::New,
            }
        })
        .collect();

    Ok((bookings, checkins))
}

/// Map legacy booking status code to string
fn map_legacy_status(code: i32) -> String {
    match code {
        0 => "pending".to_string(),
        1 => "confirmed".to_string(),
        2 => "cancelled".to_string(),
        3 => "checked_in".to_string(),
        4 => "completed".to_string(),
        _ => "unknown".to_string(),
    }
}
