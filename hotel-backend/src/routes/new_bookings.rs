//! New Booking API routes for HotelNew database
//!
//! - GET /api/new/bookings - List bookings from HT_Bookings
//! - GET /api/new/bookings/:id - Get single booking with rooms
//! - POST /api/new/bookings - Create booking
//! - PUT /api/new/bookings/:id - Update booking
//! - PUT /api/new/bookings/:id/cancel - Cancel booking

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::mode::AppState;
use crate::error::{ApiError, ApiResult};
use crate::models::Pagination;

/// Booking status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BookingStatus {
    Pending,
    Confirmed,
    CheckedIn,
    Completed,
    Cancelled,
    NoShow,
}

impl BookingStatus {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "confirmed" => BookingStatus::Confirmed,
            "checkedin" | "checked_in" => BookingStatus::CheckedIn,
            "completed" => BookingStatus::Completed,
            "cancelled" | "canceled" => BookingStatus::Cancelled,
            "noshow" | "no_show" => BookingStatus::NoShow,
            _ => BookingStatus::Pending,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            BookingStatus::Pending => "pending",
            BookingStatus::Confirmed => "confirmed",
            BookingStatus::CheckedIn => "checkedin",
            BookingStatus::Completed => "completed",
            BookingStatus::Cancelled => "cancelled",
            BookingStatus::NoShow => "noshow",
        }
    }
}

/// Booking room from HT_Booking_Rooms table
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBookingRoom {
    pub id: i32,
    pub room_id: i32,
    pub room_no: Option<String>,
    pub room_type_name: Option<String>,
    pub price_per_night: Option<f64>,
    pub total_price: Option<f64>,
}

/// Booking from HT_Bookings table
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBooking {
    pub id: i32,
    pub book_no: String,
    pub customer_id: i32,
    pub customer_name: Option<String>,
    pub check_in: Option<NaiveDateTime>,
    pub check_out: Option<NaiveDateTime>,
    pub nights: Option<i32>,
    pub adults: Option<i32>,
    pub children: Option<i32>,
    pub status: String,
    pub source: Option<String>,
    pub total_amount: Option<f64>,
    pub deposit_amount: Option<f64>,
    pub notes: Option<String>,
    pub room_count: usize,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// Booking detail with rooms
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBookingDetail {
    #[serde(flatten)]
    pub booking: NewBooking,
    pub rooms: Vec<NewBookingRoom>,
}

/// Query parameters for bookings list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBookingsQuery {
    pub search: Option<String>,
    pub status: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub customer_id: Option<i32>,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 20 }

/// Response for bookings list
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBookingsResponse {
    pub success: bool,
    pub data: Vec<NewBooking>,
    pub pagination: Pagination,
}

/// Response for single booking
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBookingResponse {
    pub success: bool,
    pub booking: NewBookingDetail,
}

/// Room in create/update request
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookingRoomRequest {
    pub room_id: i32,
    pub price_per_night: Option<f64>,
}

/// Request body for creating/updating booking
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUpdateBookingRequest {
    pub customer_id: i32,
    pub check_in: String,
    pub check_out: String,
    pub adults: Option<i32>,
    pub children: Option<i32>,
    pub status: Option<String>,
    pub source: Option<String>,
    pub total_amount: Option<f64>,
    pub deposit_amount: Option<f64>,
    pub notes: Option<String>,
    pub rooms: Vec<BookingRoomRequest>,
}

/// Response for create/update/cancel operations
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub book_no: Option<String>,
}

/// GET /api/new/bookings - List bookings from HT_Bookings
pub async fn list_bookings(
    State(state): State<AppState>,
    Query(params): Query<NewBookingsQuery>,
) -> ApiResult<Json<NewBookingsResponse>> {
    let mut conn = state.new_pool.get().await?;

    let offset = (params.page - 1) * params.limit;
    let sort_order = params
        .sort_order
        .as_ref()
        .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
        .unwrap_or("DESC");

    // Map frontend column names to SQL columns
    let order_by_column = match params.sort_by.as_deref() {
        Some("bookNo") => "b.Book_No",
        Some("customer") => "c.Cust_FirstName",
        Some("checkIn") => "b.Book_CheckIn",
        Some("checkOut") => "b.Book_CheckOut",
        Some("status") => "b.Book_Status",
        Some("totalAmount") => "b.Book_Total_Amount",
        _ => "b.Created_At",
    };

    // Build WHERE conditions
    let mut conditions: Vec<String> = Vec::new();

    if let Some(ref search) = params.search {
        let escaped = search.replace('\'', "''");
        conditions.push(format!(
            "(b.Book_No LIKE '%{}%' OR c.Cust_FirstName LIKE '%{}%' OR c.Cust_LastName LIKE '%{}%' OR c.Cust_Phone LIKE '%{}%')",
            escaped, escaped, escaped, escaped
        ));
    }

    if let Some(ref status) = params.status {
        let escaped = status.replace('\'', "''");
        conditions.push(format!("b.Book_Status = '{}'", escaped));
    }

    // Date range filter: find bookings that OVERLAP the given range
    if let Some(ref start_date) = params.start_date {
        conditions.push(format!("CAST(b.Book_CheckOut AS DATE) >= '{}'", start_date.replace('\'', "''")));
    }

    if let Some(ref end_date) = params.end_date {
        conditions.push(format!("CAST(b.Book_CheckIn AS DATE) <= '{}'", end_date.replace('\'', "''")));
    }

    if let Some(cust_id) = params.customer_id {
        conditions.push(format!("b.Book_Cust_ID = {}", cust_id));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count query
    let count_query = format!(
        r#"
        SELECT COUNT(DISTINCT b.Book_ID) as total
        FROM HT_Bookings b
        LEFT JOIN HT_Customers c ON b.Book_Cust_ID = c.Cust_ID
        {}
        "#,
        where_clause
    );

    let count_rows = conn
        .simple_query(&count_query)
        .await?
        .into_first_result()
        .await?;

    let total: i32 = count_rows
        .first()
        .and_then(|r| r.get::<i32, _>("total"))
        .unwrap_or(0);

    // Data query
    let data_query = format!(
        r#"
        SELECT
            b.Book_ID,
            b.Book_No,
            b.Book_Cust_ID,
            CONCAT(c.Cust_FirstName, ' ', COALESCE(c.Cust_LastName, '')) as Customer_Name,
            b.Book_CheckIn,
            b.Book_CheckOut,
            b.Book_Nights,
            b.Book_Adults,
            b.Book_Children,
            b.Book_Status,
            b.Book_Source,
            b.Book_Total_Amount,
            b.Book_Deposit_Amount,
            b.Book_Notes,
            b.Created_At,
            b.Updated_At,
            (SELECT COUNT(*) FROM HT_Booking_Rooms br WHERE br.BR_Book_ID = b.Book_ID) as Room_Count
        FROM HT_Bookings b
        LEFT JOIN HT_Customers c ON b.Book_Cust_ID = c.Cust_ID
        {}
        ORDER BY {} {}
        OFFSET {} ROWS FETCH NEXT {} ROWS ONLY
        "#,
        where_clause, order_by_column, sort_order, offset, params.limit
    );

    let rows = conn
        .simple_query(&data_query)
        .await?
        .into_first_result()
        .await?;

    let bookings: Vec<NewBooking> = rows
        .iter()
        .map(|row| NewBooking {
            id: row.get::<i32, _>("Book_ID").unwrap_or(0),
            book_no: row.get::<&str, _>("Book_No").unwrap_or_default().to_string(),
            customer_id: row.get::<i32, _>("Book_Cust_ID").unwrap_or(0),
            customer_name: row.get::<&str, _>("Customer_Name").map(String::from),
            check_in: row.get::<NaiveDateTime, _>("Book_CheckIn"),
            check_out: row.get::<NaiveDateTime, _>("Book_CheckOut"),
            nights: row.get::<i32, _>("Book_Nights"),
            adults: row.get::<i32, _>("Book_Adults"),
            children: row.get::<i32, _>("Book_Children"),
            status: row.get::<&str, _>("Book_Status").unwrap_or("pending").to_string(),
            source: row.get::<&str, _>("Book_Source").map(String::from),
            total_amount: row.get::<f64, _>("Book_Total_Amount"),
            deposit_amount: row.get::<f64, _>("Book_Deposit_Amount"),
            notes: row.get::<&str, _>("Book_Notes").map(String::from),
            room_count: row.get::<i32, _>("Room_Count").unwrap_or(0) as usize,
            created_at: row.get::<NaiveDateTime, _>("Created_At"),
            updated_at: row.get::<NaiveDateTime, _>("Updated_At"),
        })
        .collect();

    Ok(Json(NewBookingsResponse {
        success: true,
        data: bookings,
        pagination: Pagination::new(params.page, params.limit, total),
    }))
}

/// GET /api/new/bookings/:id - Get single booking with rooms
pub async fn get_booking(
    State(state): State<AppState>,
    Path(book_id): Path<i32>,
) -> ApiResult<Json<NewBookingResponse>> {
    let mut conn = state.new_pool.get().await?;

    // Get booking
    let booking_rows = conn
        .query(
            r#"
            SELECT
                b.Book_ID,
                b.Book_No,
                b.Book_Cust_ID,
                CONCAT(c.Cust_FirstName, ' ', COALESCE(c.Cust_LastName, '')) as Customer_Name,
                b.Book_CheckIn,
                b.Book_CheckOut,
                b.Book_Nights,
                b.Book_Adults,
                b.Book_Children,
                b.Book_Status,
                b.Book_Source,
                b.Book_Total_Amount,
                b.Book_Deposit_Amount,
                b.Book_Notes,
                b.Created_At,
                b.Updated_At
            FROM HT_Bookings b
            LEFT JOIN HT_Customers c ON b.Book_Cust_ID = c.Cust_ID
            WHERE b.Book_ID = @P1
            "#,
            &[&book_id],
        )
        .await?
        .into_first_result()
        .await?;

    let booking_row = booking_rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Booking not found".to_string()))?;

    // Get booking rooms
    let room_rows = conn
        .query(
            r#"
            SELECT
                br.BR_ID,
                br.BR_Room_ID,
                r.Room_No,
                rt.Type_Name,
                br.BR_Price_Per_Night,
                br.BR_Total_Price
            FROM HT_Booking_Rooms br
            LEFT JOIN HT_Rooms_New r ON br.BR_Room_ID = r.Room_ID
            LEFT JOIN HT_Room_Types rt ON r.Room_Type_ID = rt.Type_ID
            WHERE br.BR_Book_ID = @P1
            "#,
            &[&book_id],
        )
        .await?
        .into_first_result()
        .await?;

    let rooms: Vec<NewBookingRoom> = room_rows
        .iter()
        .map(|row| NewBookingRoom {
            id: row.get::<i32, _>("BR_ID").unwrap_or(0),
            room_id: row.get::<i32, _>("BR_Room_ID").unwrap_or(0),
            room_no: row.get::<&str, _>("Room_No").map(String::from),
            room_type_name: row.get::<&str, _>("Type_Name").map(String::from),
            price_per_night: row.get::<f64, _>("BR_Price_Per_Night"),
            total_price: row.get::<f64, _>("BR_Total_Price"),
        })
        .collect();

    let booking = NewBooking {
        id: booking_row.get::<i32, _>("Book_ID").unwrap_or(0),
        book_no: booking_row.get::<&str, _>("Book_No").unwrap_or_default().to_string(),
        customer_id: booking_row.get::<i32, _>("Book_Cust_ID").unwrap_or(0),
        customer_name: booking_row.get::<&str, _>("Customer_Name").map(String::from),
        check_in: booking_row.get::<NaiveDateTime, _>("Book_CheckIn"),
        check_out: booking_row.get::<NaiveDateTime, _>("Book_CheckOut"),
        nights: booking_row.get::<i32, _>("Book_Nights"),
        adults: booking_row.get::<i32, _>("Book_Adults"),
        children: booking_row.get::<i32, _>("Book_Children"),
        status: booking_row.get::<&str, _>("Book_Status").unwrap_or("pending").to_string(),
        source: booking_row.get::<&str, _>("Book_Source").map(String::from),
        total_amount: booking_row.get::<f64, _>("Book_Total_Amount"),
        deposit_amount: booking_row.get::<f64, _>("Book_Deposit_Amount"),
        notes: booking_row.get::<&str, _>("Book_Notes").map(String::from),
        room_count: rooms.len(),
        created_at: booking_row.get::<NaiveDateTime, _>("Created_At"),
        updated_at: booking_row.get::<NaiveDateTime, _>("Updated_At"),
    };

    Ok(Json(NewBookingResponse {
        success: true,
        booking: NewBookingDetail { booking, rooms },
    }))
}

/// POST /api/new/bookings - Create booking
pub async fn create_booking(
    State(state): State<AppState>,
    Json(body): Json<CreateUpdateBookingRequest>,
) -> ApiResult<Json<MutationResponse>> {
    if body.rooms.is_empty() {
        return Err(ApiError::BadRequest("At least one room is required".to_string()));
    }

    let mut conn = state.new_pool.get().await?;

    // Generate booking number (YYYYMMDD-NNNN format)
    let book_no_rows = conn
        .simple_query(
            r#"
            SELECT TOP 1 Book_No
            FROM HT_Bookings
            WHERE Book_No LIKE CONCAT(FORMAT(GETDATE(), 'yyyyMMdd'), '-%')
            ORDER BY Book_No DESC
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let next_seq = if let Some(row) = book_no_rows.first() {
        if let Some(last_no) = row.get::<&str, _>("Book_No") {
            // Extract sequence number and increment
            let parts: Vec<&str> = last_no.split('-').collect();
            if parts.len() == 2 {
                parts[1].parse::<i32>().unwrap_or(0) + 1
            } else {
                1
            }
        } else {
            1
        }
    } else {
        1
    };

    // Get current date in YYYYMMDD format
    let date_rows = conn
        .simple_query("SELECT FORMAT(GETDATE(), 'yyyyMMdd') as today")
        .await?
        .into_first_result()
        .await?;

    let today = date_rows
        .first()
        .and_then(|r| r.get::<&str, _>("today"))
        .unwrap_or("00000000");

    let book_no = format!("{}-{:04}", today, next_seq);

    let status = body.status.as_deref().unwrap_or("pending");
    let adults = body.adults.unwrap_or(1);
    let children = body.children.unwrap_or(0);

    // Insert booking
    let booking_rows = conn
        .query(
            r#"
            INSERT INTO HT_Bookings (
                Book_No,
                Book_Cust_ID,
                Book_CheckIn,
                Book_CheckOut,
                Book_Adults,
                Book_Children,
                Book_Status,
                Book_Source,
                Book_Total_Amount,
                Book_Deposit_Amount,
                Book_Notes
            )
            OUTPUT INSERTED.Book_ID
            VALUES (@P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8, @P9, @P10, @P11)
            "#,
            &[
                &book_no.as_str(),
                &body.customer_id,
                &body.check_in.as_str(),
                &body.check_out.as_str(),
                &adults,
                &children,
                &status,
                &body.source.as_deref(),
                &body.total_amount,
                &body.deposit_amount,
                &body.notes.as_deref(),
            ],
        )
        .await?
        .into_first_result()
        .await?;

    let book_id = booking_rows
        .first()
        .and_then(|r| r.get::<i32, _>("Book_ID"))
        .ok_or_else(|| ApiError::Internal("Failed to create booking".to_string()))?;

    // Insert booking rooms
    for room in &body.rooms {
        conn.execute(
            r#"
            INSERT INTO HT_Booking_Rooms (BR_Book_ID, BR_Room_ID, BR_Price_Per_Night)
            VALUES (@P1, @P2, @P3)
            "#,
            &[&book_id, &room.room_id, &room.price_per_night],
        )
        .await?;
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Booking created successfully".to_string(),
        id: Some(book_id),
        book_no: Some(book_no),
    }))
}

/// PUT /api/new/bookings/:id - Update booking
pub async fn update_booking(
    State(state): State<AppState>,
    Path(book_id): Path<i32>,
    Json(body): Json<CreateUpdateBookingRequest>,
) -> ApiResult<Json<MutationResponse>> {
    if body.rooms.is_empty() {
        return Err(ApiError::BadRequest("At least one room is required".to_string()));
    }

    let mut conn = state.new_pool.get().await?;

    // Check if booking exists
    let check_rows = conn
        .query(
            "SELECT Book_No FROM HT_Bookings WHERE Book_ID = @P1",
            &[&book_id],
        )
        .await?
        .into_first_result()
        .await?;

    let book_no = check_rows
        .first()
        .and_then(|r| r.get::<&str, _>("Book_No").map(String::from))
        .ok_or_else(|| ApiError::NotFound("Booking not found".to_string()))?;

    let status = body.status.as_deref().unwrap_or("pending");
    let adults = body.adults.unwrap_or(1);
    let children = body.children.unwrap_or(0);

    // Update booking
    conn.execute(
        r#"
        UPDATE HT_Bookings
        SET Book_Cust_ID = @P1,
            Book_CheckIn = @P2,
            Book_CheckOut = @P3,
            Book_Adults = @P4,
            Book_Children = @P5,
            Book_Status = @P6,
            Book_Source = @P7,
            Book_Total_Amount = @P8,
            Book_Deposit_Amount = @P9,
            Book_Notes = @P10,
            Updated_At = GETDATE()
        WHERE Book_ID = @P11
        "#,
        &[
            &body.customer_id,
            &body.check_in.as_str(),
            &body.check_out.as_str(),
            &adults,
            &children,
            &status,
            &body.source.as_deref(),
            &body.total_amount,
            &body.deposit_amount,
            &body.notes.as_deref(),
            &book_id,
        ],
    )
    .await?;

    // Delete existing booking rooms and insert new ones
    conn.execute(
        "DELETE FROM HT_Booking_Rooms WHERE BR_Book_ID = @P1",
        &[&book_id],
    )
    .await?;

    for room in &body.rooms {
        conn.execute(
            r#"
            INSERT INTO HT_Booking_Rooms (BR_Book_ID, BR_Room_ID, BR_Price_Per_Night)
            VALUES (@P1, @P2, @P3)
            "#,
            &[&book_id, &room.room_id, &room.price_per_night],
        )
        .await?;
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Booking updated successfully".to_string(),
        id: Some(book_id),
        book_no: Some(book_no),
    }))
}

/// PUT /api/new/bookings/:id/cancel - Cancel booking
pub async fn cancel_booking(
    State(state): State<AppState>,
    Path(book_id): Path<i32>,
) -> ApiResult<Json<MutationResponse>> {
    let mut conn = state.new_pool.get().await?;

    let result = conn
        .execute(
            r#"
            UPDATE HT_Bookings
            SET Book_Status = 'cancelled',
                Updated_At = GETDATE()
            WHERE Book_ID = @P1 AND Book_Status NOT IN ('completed', 'cancelled')
            "#,
            &[&book_id],
        )
        .await?;

    if result.total() == 0 {
        return Err(ApiError::BadRequest("Booking not found or cannot be cancelled".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Booking cancelled successfully".to_string(),
        id: Some(book_id),
        book_no: None,
    }))
}
