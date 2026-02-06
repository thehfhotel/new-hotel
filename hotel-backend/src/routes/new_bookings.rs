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
use sqlx::Row;

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
    let pool = &state.new_pool;

    let offset = (params.page - 1) * params.limit;
    let sort_order = params
        .sort_order
        .as_ref()
        .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
        .unwrap_or("DESC");

    // Map frontend column names to SQL columns
    let order_by_column = match params.sort_by.as_deref() {
        Some("bookNo") => "b.book_no",
        Some("customer") => "c.cust_firstname",
        Some("checkIn") => "b.book_checkin",
        Some("checkOut") => "b.book_checkout",
        Some("status") => "b.book_status",
        Some("totalAmount") => "b.book_total_amount",
        _ => "b.created_at",
    };

    // Build WHERE conditions
    let mut conditions: Vec<String> = Vec::new();

    if let Some(ref search) = params.search {
        let escaped = search.replace('\'', "''");
        conditions.push(format!(
            "(b.book_no LIKE '%{}%' OR c.cust_firstname LIKE '%{}%' OR c.cust_lastname LIKE '%{}%' OR c.cust_phone LIKE '%{}%')",
            escaped, escaped, escaped, escaped
        ));
    }

    if let Some(ref status) = params.status {
        let escaped = status.replace('\'', "''");
        conditions.push(format!("b.book_status = '{}'", escaped));
    }

    // Date range filter: find bookings that OVERLAP the given range
    if let Some(ref start_date) = params.start_date {
        conditions.push(format!("b.book_checkout::date >= '{}'", start_date.replace('\'', "''")));
    }

    if let Some(ref end_date) = params.end_date {
        conditions.push(format!("b.book_checkin::date <= '{}'", end_date.replace('\'', "''")));
    }

    if let Some(cust_id) = params.customer_id {
        conditions.push(format!("b.book_cust_id = {}", cust_id));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count query
    let count_query = format!(
        r#"
        SELECT COUNT(DISTINCT b.book_id)::int as total
        FROM ht_bookings b
        LEFT JOIN ht_customers c ON b.book_cust_id = c.cust_id
        {}
        "#,
        where_clause
    );

    let count_rows = sqlx::query(&count_query)
        .fetch_all(pool)
        .await?;

    let total: i32 = count_rows
        .first()
        .map(|r| r.try_get::<i32, _>("total").unwrap_or(0))
        .unwrap_or(0);

    // Data query
    let data_query = format!(
        r#"
        SELECT
            b.book_id,
            b.book_no,
            b.book_cust_id,
            CONCAT(c.cust_firstname, ' ', COALESCE(c.cust_lastname, '')) as customer_name,
            b.book_checkin,
            b.book_checkout,
            b.book_nights,
            b.book_adults,
            b.book_children,
            b.book_status,
            b.book_source,
            b.book_total_amount,
            b.book_deposit_amount,
            b.book_notes,
            b.created_at,
            b.updated_at,
            (SELECT COUNT(*)::int FROM ht_booking_rooms br WHERE br.br_book_id = b.book_id) as room_count
        FROM ht_bookings b
        LEFT JOIN ht_customers c ON b.book_cust_id = c.cust_id
        {}
        ORDER BY {} {}
        LIMIT {} OFFSET {}
        "#,
        where_clause, order_by_column, sort_order, params.limit, offset
    );

    let rows = sqlx::query(&data_query)
        .fetch_all(pool)
        .await?;

    let bookings: Vec<NewBooking> = rows
        .iter()
        .map(|row| NewBooking {
            id: row.try_get::<i32, _>("book_id").unwrap_or(0),
            book_no: row.try_get::<String, _>("book_no").unwrap_or_default(),
            customer_id: row.try_get::<i32, _>("book_cust_id").unwrap_or(0),
            customer_name: row.try_get::<String, _>("customer_name").ok(),
            check_in: row.try_get::<NaiveDateTime, _>("book_checkin").ok(),
            check_out: row.try_get::<NaiveDateTime, _>("book_checkout").ok(),
            nights: row.try_get::<i32, _>("book_nights").ok(),
            adults: row.try_get::<i32, _>("book_adults").ok(),
            children: row.try_get::<i32, _>("book_children").ok(),
            status: row.try_get::<String, _>("book_status").unwrap_or_else(|_| "pending".to_string()),
            source: row.try_get::<String, _>("book_source").ok(),
            total_amount: row.try_get::<f64, _>("book_total_amount").ok(),
            deposit_amount: row.try_get::<f64, _>("book_deposit_amount").ok(),
            notes: row.try_get::<String, _>("book_notes").ok(),
            room_count: row.try_get::<i32, _>("room_count").unwrap_or(0) as usize,
            created_at: row.try_get::<NaiveDateTime, _>("created_at").ok(),
            updated_at: row.try_get::<NaiveDateTime, _>("updated_at").ok(),
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
    let pool = &state.new_pool;

    // Get booking
    let booking_rows = sqlx::query(
            r#"
            SELECT
                b.book_id,
                b.book_no,
                b.book_cust_id,
                CONCAT(c.cust_firstname, ' ', COALESCE(c.cust_lastname, '')) as customer_name,
                b.book_checkin,
                b.book_checkout,
                b.book_nights,
                b.book_adults,
                b.book_children,
                b.book_status,
                b.book_source,
                b.book_total_amount,
                b.book_deposit_amount,
                b.book_notes,
                b.created_at,
                b.updated_at
            FROM ht_bookings b
            LEFT JOIN ht_customers c ON b.book_cust_id = c.cust_id
            WHERE b.book_id = $1
            "#,
        )
        .bind(&book_id)
        .fetch_all(pool)
        .await?;

    let booking_row = booking_rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Booking not found".to_string()))?;

    // Get booking rooms
    let room_rows = sqlx::query(
            r#"
            SELECT
                br.br_id,
                br.br_room_id,
                r.room_no,
                rt.type_name,
                br.br_price_per_night,
                br.br_total_price
            FROM ht_booking_rooms br
            LEFT JOIN ht_rooms_new r ON br.br_room_id = r.room_id
            LEFT JOIN ht_room_types rt ON r.room_type_id = rt.type_id
            WHERE br.br_book_id = $1
            "#,
        )
        .bind(&book_id)
        .fetch_all(pool)
        .await?;

    let rooms: Vec<NewBookingRoom> = room_rows
        .iter()
        .map(|row| NewBookingRoom {
            id: row.try_get::<i32, _>("br_id").unwrap_or(0),
            room_id: row.try_get::<i32, _>("br_room_id").unwrap_or(0),
            room_no: row.try_get::<String, _>("room_no").ok(),
            room_type_name: row.try_get::<String, _>("type_name").ok(),
            price_per_night: row.try_get::<f64, _>("br_price_per_night").ok(),
            total_price: row.try_get::<f64, _>("br_total_price").ok(),
        })
        .collect();

    let booking = NewBooking {
        id: booking_row.try_get::<i32, _>("book_id").unwrap_or(0),
        book_no: booking_row.try_get::<String, _>("book_no").unwrap_or_default(),
        customer_id: booking_row.try_get::<i32, _>("book_cust_id").unwrap_or(0),
        customer_name: booking_row.try_get::<String, _>("customer_name").ok(),
        check_in: booking_row.try_get::<NaiveDateTime, _>("book_checkin").ok(),
        check_out: booking_row.try_get::<NaiveDateTime, _>("book_checkout").ok(),
        nights: booking_row.try_get::<i32, _>("book_nights").ok(),
        adults: booking_row.try_get::<i32, _>("book_adults").ok(),
        children: booking_row.try_get::<i32, _>("book_children").ok(),
        status: booking_row.try_get::<String, _>("book_status").unwrap_or_else(|_| "pending".to_string()),
        source: booking_row.try_get::<String, _>("book_source").ok(),
        total_amount: booking_row.try_get::<f64, _>("book_total_amount").ok(),
        deposit_amount: booking_row.try_get::<f64, _>("book_deposit_amount").ok(),
        notes: booking_row.try_get::<String, _>("book_notes").ok(),
        room_count: rooms.len(),
        created_at: booking_row.try_get::<NaiveDateTime, _>("created_at").ok(),
        updated_at: booking_row.try_get::<NaiveDateTime, _>("updated_at").ok(),
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

    let pool = &state.new_pool;

    // Generate booking number (YYYYMMDD-NNNN format)
    let book_no_rows = sqlx::query(
            r#"
            SELECT book_no
            FROM ht_bookings
            WHERE book_no LIKE TO_CHAR(NOW(), 'YYYYMMDD') || '-%'
            ORDER BY book_no DESC
            LIMIT 1
            "#,
        )
        .fetch_all(pool)
        .await?;

    let next_seq = if let Some(row) = book_no_rows.first() {
        if let Some(last_no) = row.try_get::<String, _>("book_no").ok() {
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
    let date_rows = sqlx::query("SELECT TO_CHAR(NOW(), 'YYYYMMDD') as today")
        .fetch_all(pool)
        .await?;

    let today = date_rows
        .first()
        .map(|r| r.try_get::<String, _>("today").unwrap_or_else(|_| "00000000".to_string()))
        .unwrap_or_else(|| "00000000".to_string());

    let book_no = format!("{}-{:04}", today, next_seq);

    let status = body.status.as_deref().unwrap_or("pending");
    let adults = body.adults.unwrap_or(1);
    let children = body.children.unwrap_or(0);

    // Insert booking
    let booking_rows = sqlx::query(
            r#"
            INSERT INTO ht_bookings (
                book_no,
                book_cust_id,
                book_checkin,
                book_checkout,
                book_adults,
                book_children,
                book_status,
                book_source,
                book_total_amount,
                book_deposit_amount,
                book_notes
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING book_id
            "#,
        )
        .bind(&book_no.as_str())
        .bind(&body.customer_id)
        .bind(&body.check_in.as_str())
        .bind(&body.check_out.as_str())
        .bind(&adults)
        .bind(&children)
        .bind(&status)
        .bind(&body.source.as_deref())
        .bind(&body.total_amount)
        .bind(&body.deposit_amount)
        .bind(&body.notes.as_deref())
        .fetch_all(pool)
        .await?;

    let book_id = booking_rows
        .first()
        .and_then(|r| r.try_get::<i32, _>("book_id").ok())
        .ok_or_else(|| ApiError::Internal("Failed to create booking".to_string()))?;

    // Insert booking rooms
    for room in &body.rooms {
        sqlx::query(
            r#"
            INSERT INTO ht_booking_rooms (br_book_id, br_room_id, br_price_per_night)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(&book_id)
        .bind(&room.room_id)
        .bind(&room.price_per_night)
        .execute(pool)
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

    let pool = &state.new_pool;

    // Check if booking exists
    let check_rows = sqlx::query(
            "SELECT book_no FROM ht_bookings WHERE book_id = $1",
        )
        .bind(&book_id)
        .fetch_all(pool)
        .await?;

    let book_no = check_rows
        .first()
        .and_then(|r| r.try_get::<String, _>("book_no").ok())
        .ok_or_else(|| ApiError::NotFound("Booking not found".to_string()))?;

    let status = body.status.as_deref().unwrap_or("pending");
    let adults = body.adults.unwrap_or(1);
    let children = body.children.unwrap_or(0);

    // Update booking
    sqlx::query(
        r#"
        UPDATE ht_bookings
        SET book_cust_id = $1,
            book_checkin = $2,
            book_checkout = $3,
            book_adults = $4,
            book_children = $5,
            book_status = $6,
            book_source = $7,
            book_total_amount = $8,
            book_deposit_amount = $9,
            book_notes = $10,
            updated_at = NOW()
        WHERE book_id = $11
        "#,
    )
    .bind(&body.customer_id)
    .bind(&body.check_in.as_str())
    .bind(&body.check_out.as_str())
    .bind(&adults)
    .bind(&children)
    .bind(&status)
    .bind(&body.source.as_deref())
    .bind(&body.total_amount)
    .bind(&body.deposit_amount)
    .bind(&body.notes.as_deref())
    .bind(&book_id)
    .execute(pool)
    .await?;

    // Delete existing booking rooms and insert new ones
    sqlx::query(
        "DELETE FROM ht_booking_rooms WHERE br_book_id = $1",
    )
    .bind(&book_id)
    .execute(pool)
    .await?;

    for room in &body.rooms {
        sqlx::query(
            r#"
            INSERT INTO ht_booking_rooms (br_book_id, br_room_id, br_price_per_night)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(&book_id)
        .bind(&room.room_id)
        .bind(&room.price_per_night)
        .execute(pool)
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
    let pool = &state.new_pool;

    let result = sqlx::query(
            r#"
            UPDATE ht_bookings
            SET book_status = 'cancelled',
                updated_at = NOW()
            WHERE book_id = $1 AND book_status NOT IN ('completed', 'cancelled')
            "#,
        )
        .bind(&book_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::BadRequest("Booking not found or cannot be cancelled".to_string()));
    }

    Ok(Json(MutationResponse {
        success: true,
        message: "Booking cancelled successfully".to_string(),
        id: Some(book_id),
        book_no: None,
    }))
}
