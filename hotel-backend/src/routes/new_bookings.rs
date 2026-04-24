//! New Booking API routes for HotelNew database
//!
//! - GET /api/new/bookings - List bookings from HT_Bookings
//! - GET /api/new/bookings/:id - Get single booking with rooms
//! - POST /api/new/bookings - Create booking
//! - PUT /api/new/bookings/:id - Update booking
//! - PUT /api/new/bookings/:id/cancel - Cancel booking
//!
//! Per `docs/architecture.md` §1, §6 (Phase 1b) the SQL has moved to
//! `repository::booking`. This file now owns request validation, response
//! shaping, and translates between repository row shapes and the wire DTOs.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use super::mode::AppState;
use crate::error::{ApiError, ApiResult};
use crate::models::Pagination;
use crate::repository::booking::{
    BookingDetailRow, BookingListRow, BookingRoomAssignment, BookingRoomRow, BookingWrite,
};

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

impl NewBookingRoom {
    fn from_row(row: BookingRoomRow) -> Self {
        Self {
            id: row.br_id,
            room_id: row.br_room_id,
            room_no: Some(row.room_no),
            room_type_name: Some(row.type_name),
            price_per_night: row.br_price_per_night,
            total_price: None,
        }
    }
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

impl NewBooking {
    fn from_list_row(row: BookingListRow) -> Self {
        Self {
            id: row.book_id,
            book_no: row.book_no,
            customer_id: row.book_cust_id,
            customer_name: row.customer_name,
            check_in: row.book_checkin,
            check_out: row.book_checkout,
            nights: row.book_nights,
            adults: row.book_adults,
            children: row.book_children,
            status: row.book_status,
            source: row.book_source,
            total_amount: row.book_total_amount,
            deposit_amount: row.book_deposit_amount,
            notes: row.book_notes,
            room_count: row.room_count.max(0) as usize,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }

    fn from_detail_row(row: BookingDetailRow, room_count: usize) -> Self {
        Self {
            id: row.book_id,
            book_no: row.book_no,
            customer_id: row.book_cust_id,
            customer_name: row.customer_name,
            check_in: Some(row.book_checkin.and_hms_opt(0, 0, 0).unwrap()),
            check_out: Some(row.book_checkout.and_hms_opt(0, 0, 0).unwrap()),
            nights: row.book_nights,
            adults: row.book_adults,
            children: row.book_children,
            status: row.book_status.unwrap_or_else(|| "pending".to_string()),
            source: row.book_source,
            total_amount: row.book_total_amount,
            deposit_amount: row.book_deposit_amount,
            notes: row.book_notes,
            room_count,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
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
    /// Branch selector: 'hfhotel' | 'hfville' | 'all' (HotelNew only contains hfhotel data)
    pub branch: Option<String>,
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
    // HotelNew only stores HF Hotel data; HF Ville request -> empty list.
    if params.branch.as_deref() == Some("hfville") {
        return Ok(Json(NewBookingsResponse {
            success: true,
            data: vec![],
            pagination: Pagination::new(params.page, params.limit, 0),
        }));
    }

    let (rows, total) = state
        .bookings
        .list_with_count(&state.new_pool, &params)
        .await?;

    let bookings: Vec<NewBooking> = rows.into_iter().map(NewBooking::from_list_row).collect();

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
    let detail = state
        .bookings
        .get(&state.new_pool, book_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Booking not found".to_string()))?;

    let room_rows = state.bookings.list_rooms(&state.new_pool, book_id).await?;
    let rooms: Vec<NewBookingRoom> = room_rows.into_iter().map(NewBookingRoom::from_row).collect();
    let room_count = rooms.len();

    let booking = NewBooking::from_detail_row(detail, room_count);

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

    // Generate booking number (YYYYMMDD-NNNN format).
    let last_book_no = state.bookings.latest_book_no_today(&state.new_pool).await?;

    let next_seq = if let Some(last) = last_book_no {
        let parts: Vec<&str> = last.split('-').collect();
        if parts.len() == 2 {
            parts[1].parse::<i32>().unwrap_or(0) + 1
        } else {
            1
        }
    } else {
        1
    };

    let today = state.bookings.today_yyyymmdd(&state.new_pool).await?;

    let book_no = format!("{}-{:04}", today, next_seq);

    let status = body.status.as_deref().unwrap_or("pending");
    let adults = body.adults.unwrap_or(1);
    let children = body.children.unwrap_or(0);

    let check_in_date = NaiveDate::parse_from_str(&body.check_in, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid check-in date format (expected YYYY-MM-DD)".to_string()))?;
    let check_out_date = NaiveDate::parse_from_str(&body.check_out, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid check-out date format (expected YYYY-MM-DD)".to_string()))?;

    let mut tx = state.new_pool.begin().await?;
    let book_id = state
        .bookings
        .insert_booking(
            &mut tx,
            BookingWrite {
                book_no: book_no.as_str(),
                customer_id: body.customer_id,
                check_in: check_in_date,
                check_out: check_out_date,
                adults,
                children,
                status,
                source: body.source.as_deref(),
                total_amount: body.total_amount,
                deposit_amount: body.deposit_amount,
                notes: body.notes.as_deref(),
            },
        )
        .await?;

    for room in &body.rooms {
        state
            .bookings
            .insert_booking_room(
                &mut tx,
                book_id,
                BookingRoomAssignment {
                    room_id: room.room_id,
                    price_per_night: room.price_per_night,
                },
            )
            .await?;
    }

    tx.commit().await?;

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

    // Check if booking exists (and grab the existing book_no for the response)
    let book_no = state
        .bookings
        .get_book_no(&state.new_pool, book_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Booking not found".to_string()))?;

    let status = body.status.as_deref().unwrap_or("pending");
    let adults = body.adults.unwrap_or(1);
    let children = body.children.unwrap_or(0);

    let check_in_date = NaiveDate::parse_from_str(&body.check_in, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid check-in date format (expected YYYY-MM-DD)".to_string()))?;
    let check_out_date = NaiveDate::parse_from_str(&body.check_out, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid check-out date format (expected YYYY-MM-DD)".to_string()))?;

    let mut tx = state.new_pool.begin().await?;
    state
        .bookings
        .update_booking(
            &mut tx,
            book_id,
            BookingWrite {
                book_no: book_no.as_str(),
                customer_id: body.customer_id,
                check_in: check_in_date,
                check_out: check_out_date,
                adults,
                children,
                status,
                source: body.source.as_deref(),
                total_amount: body.total_amount,
                deposit_amount: body.deposit_amount,
                notes: body.notes.as_deref(),
            },
        )
        .await?;

    state.bookings.delete_booking_rooms(&mut tx, book_id).await?;

    for room in &body.rooms {
        state
            .bookings
            .insert_booking_room(
                &mut tx,
                book_id,
                BookingRoomAssignment {
                    room_id: room.room_id,
                    price_per_night: room.price_per_night,
                },
            )
            .await?;
    }

    tx.commit().await?;

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
    let mut tx = state.new_pool.begin().await?;
    let rows_affected = state.bookings.cancel(&mut tx, book_id).await?;

    if rows_affected == 0 {
        tx.rollback().await?;
        return Err(ApiError::BadRequest("Booking not found or cannot be cancelled".to_string()));
    }

    tx.commit().await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Booking cancelled successfully".to_string(),
        id: Some(book_id),
        book_no: None,
    }))
}
