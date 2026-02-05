//! Calendar API route with hybrid database support
//!
//! - GET /api/calendar - Hybrid calendar endpoint fetching from both databases

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

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
    pool: &crate::db::DbPool,
    start_date: &str,
    end_date: &str,
) -> ApiResult<(Vec<CalendarBooking>, Vec<CalendarCheckin>)> {
    let mut conn = pool.get().await?;

    // Fetch bookings from HotelNew database (HT_Bookings + HT_Customers + HT_Booking_Rooms + HT_Rooms_New)
    let booking_query = format!(
        r#"
        SELECT
            b.Book_ID,
            b.Book_No,
            CONCAT(c.Cust_FirstName, ' ', ISNULL(c.Cust_LastName, '')) AS Cust_Name,
            r.Room_No,
            b.Book_CheckIn,
            b.Book_CheckOut,
            b.Book_Status
        FROM HT_Bookings b
        INNER JOIN HT_Customers c ON b.Book_Cust_ID = c.Cust_ID
        LEFT JOIN HT_Booking_Rooms br ON b.Book_ID = br.BR_Book_ID
        LEFT JOIN HT_Rooms_New r ON br.BR_Room_ID = r.Room_ID
        WHERE b.Book_CheckOut >= '{}'
          AND b.Book_CheckIn <= '{}'
          AND b.Book_Status != 'cancelled'
        ORDER BY b.Book_CheckIn
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
            let id = row.get::<i32, _>("Book_ID").unwrap_or_default();
            let booking_no = row.get::<&str, _>("Book_No").unwrap_or_default().to_string();
            CalendarBooking {
                id: format!("new-booking-{}", id),
                booking_no,
                customer_name: row.get::<&str, _>("Cust_Name").map(String::from),
                room_no: row.get::<&str, _>("Room_No").map(String::from),
                check_in: row.get::<NaiveDateTime, _>("Book_CheckIn"),
                check_out: row.get::<NaiveDateTime, _>("Book_CheckOut"),
                status: row.get::<&str, _>("Book_Status").map(String::from),
                source: DataSource::New,
            }
        })
        .collect();

    // Fetch check-ins from HotelNew database (HT_CheckIns + HT_Customers + HT_Rooms_New)
    let checkin_query = format!(
        r#"
        SELECT
            ci.Cin_ID,
            ci.Cin_No,
            CONCAT(c.Cust_FirstName, ' ', ISNULL(c.Cust_LastName, '')) AS Cust_Name,
            r.Room_No,
            ci.Cin_CheckIn_Time,
            ISNULL(ci.Cin_CheckOut_Time, CAST(ci.Cin_Expected_Out AS DATETIME)) AS Cin_CheckOut
        FROM HT_CheckIns ci
        INNER JOIN HT_Customers c ON ci.Cin_Cust_ID = c.Cust_ID
        INNER JOIN HT_Rooms_New r ON ci.Cin_Room_ID = r.Room_ID
        WHERE ISNULL(ci.Cin_CheckOut_Time, ci.Cin_Expected_Out) >= '{}'
          AND ci.Cin_CheckIn_Time <= '{}'
        ORDER BY ci.Cin_CheckIn_Time
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
            let id = row.get::<i32, _>("Cin_ID").unwrap_or_default();
            let cin_no = row.get::<&str, _>("Cin_No").unwrap_or_default().to_string();
            CalendarCheckin {
                id: format!("new-checkin-{}", id),
                checkin_no: cin_no,
                customer_name: row.get::<&str, _>("Cust_Name").map(String::from),
                room_no: row.get::<&str, _>("Room_No").map(String::from),
                check_in: row.get::<NaiveDateTime, _>("Cin_CheckIn_Time"),
                check_out: row.get::<NaiveDateTime, _>("Cin_CheckOut"),
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
