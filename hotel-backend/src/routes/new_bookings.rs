//! New Booking API routes for HotelNew database
//!
//! - GET /api/new/bookings - List bookings from HT_Bookings
//! - GET /api/new/bookings/:id - Get single booking with rooms
//! - POST /api/new/bookings - Create booking
//! - PUT /api/new/bookings/:id - Update booking
//! - PUT /api/new/bookings/:id/cancel - Cancel booking
//!
//! Per `docs/architecture.md` §1, §6 (Phase 2.5) writes delegate through
//! `state.bookings_service`. Reads keep calling `state.bookings` (the
//! repository) directly — the service layer's value is in writes (TX +
//! outbox + events), not in reads.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::mode::AppState;
use crate::domain::shared::{DateRange, Money};
use crate::error::{ApiError, ApiResult};
use crate::models::Pagination;
use crate::outbox::event::EventSource;
use crate::outbox::intent::BookingChanges;
use crate::repository::booking::{BookingDetailRow, BookingListRow, BookingRoomRow};
use crate::service::{
    BookingRoomCommand, BookingWritebackContext, CancelBookingCommand, CreateBookingCommand,
    ModifyBookingCommand,
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
///
/// Delegates the canonical write + outbox enqueue + event publish to
/// [`crate::service::BookingService::create`]. The route generates the
/// `book_no` (today/sequence based) since that's a presentation concern,
/// then constructs the [`CreateBookingCommand`] from the request body.
pub async fn create_booking(
    State(state): State<AppState>,
    Json(body): Json<CreateUpdateBookingRequest>,
) -> ApiResult<Json<MutationResponse>> {
    if body.rooms.is_empty() {
        return Err(ApiError::BadRequest("At least one room is required".to_string()));
    }

    let book_no = generate_book_no(&state).await?;
    let (check_in_date, check_out_date) = parse_stay_range(&body.check_in, &body.check_out)?;

    // Fetch customer + first room so the writeback context lands populated
    // (otherwise the .NET booking list shows blank Book_Cust_Name +
    // Book_Room_Type + Book_Room_Price — see the [2.28.0] CHANGELOG entry).
    let writeback_context =
        build_writeback_context(&state, &body, check_in_date, check_out_date).await?;

    let cmd = CreateBookingCommand {
        book_no: book_no.clone(),
        customer_id: body.customer_id,
        check_in: check_in_date,
        check_out: check_out_date,
        adults: body.adults.unwrap_or(1),
        children: body.children.unwrap_or(0),
        status: body.status.clone().unwrap_or_else(|| "pending".to_string()),
        source_label: body.source.clone(),
        total_amount: body.total_amount,
        deposit_amount: body.deposit_amount,
        notes: body.notes.clone(),
        rooms: body.rooms.iter().map(room_request_to_command).collect(),
        writeback_context,
        // TODO: wire user_id from auth middleware
        source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
    };

    let outcome = state.bookings_service.create(cmd).await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Booking created successfully".to_string(),
        id: Some(outcome.book_id),
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

    // Verify the booking exists (404 vs 400) and grab book_no for the response.
    let book_no = state
        .bookings
        .get_book_no(&state.new_pool, book_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Booking not found".to_string()))?;

    let (check_in_date, check_out_date) = parse_stay_range(&body.check_in, &body.check_out)?;

    let snapshot = build_snapshot_inputs(&body, check_in_date, check_out_date);

    let cmd = ModifyBookingCommand {
        book_id,
        customer_id: body.customer_id,
        check_in: check_in_date,
        check_out: check_out_date,
        adults: body.adults.unwrap_or(1),
        children: body.children.unwrap_or(0),
        status: body.status.clone().unwrap_or_else(|| "pending".to_string()),
        source_label: body.source.clone(),
        total_amount: body.total_amount,
        deposit_amount: body.deposit_amount,
        notes: body.notes.clone(),
        rooms: body.rooms.iter().map(room_request_to_command).collect(),
        // TODO: diff against the loaded prior row to populate per-field changes.
        changes: BookingChanges {
            new_stay: Some(DateRange::new(
                naive_date_to_utc(check_in_date),
                naive_date_to_utc(check_out_date),
            )),
            new_room_no: None,
            new_room_type: None,
            new_price: body.total_amount.map(money_from_baht_f64),
            new_state: None,
            new_notes: body.notes.clone(),
            new_customer_phone: None,
        },
        // TODO: load prior snapshot from repo for richer event payload.
        before_snapshot: None,
        after_snapshot: snapshot,
        // TODO: wire user_id from auth middleware
        source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
    };

    let outcome = state.bookings_service.modify(cmd).await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Booking updated successfully".to_string(),
        id: Some(outcome.book_id),
        book_no: Some(book_no),
    }))
}

/// PUT /api/new/bookings/:id/cancel - Cancel booking
pub async fn cancel_booking(
    State(state): State<AppState>,
    Path(book_id): Path<i32>,
) -> ApiResult<Json<MutationResponse>> {
    let outcome = state
        .bookings_service
        .cancel(CancelBookingCommand {
            book_id,
            reason: None,
            // TODO: wire user_id from auth middleware
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
        })
        .await
        .map_err(map_cancel_error)?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Booking cancelled successfully".to_string(),
        id: Some(outcome.book_id),
        book_no: None,
    }))
}

// ---------- helpers (presentation glue) ----------

/// Generate `YYYYMMDD-NNNN` booking number from the latest sequence today.
async fn generate_book_no(state: &AppState) -> ApiResult<String> {
    let last_book_no = state.bookings.latest_book_no_today(&state.new_pool).await?;
    let next_seq = last_book_no
        .as_deref()
        .and_then(|s| s.split('-').nth(1))
        .and_then(|n| n.parse::<i32>().ok())
        .map(|n| n + 1)
        .unwrap_or(1);

    let today = state.bookings.today_yyyymmdd(&state.new_pool).await?;
    Ok(format!("{}-{:04}", today, next_seq))
}

fn parse_stay_range(check_in: &str, check_out: &str) -> ApiResult<(NaiveDate, NaiveDate)> {
    let check_in_date = NaiveDate::parse_from_str(check_in, "%Y-%m-%d").map_err(|_| {
        ApiError::BadRequest("Invalid check-in date format (expected YYYY-MM-DD)".to_string())
    })?;
    let check_out_date = NaiveDate::parse_from_str(check_out, "%Y-%m-%d").map_err(|_| {
        ApiError::BadRequest("Invalid check-out date format (expected YYYY-MM-DD)".to_string())
    })?;
    Ok((check_in_date, check_out_date))
}

fn room_request_to_command(req: &BookingRoomRequest) -> BookingRoomCommand {
    BookingRoomCommand {
        room_id: req.room_id,
        price_per_night: req.price_per_night,
    }
}

/// Build a [`BookingWritebackContext`] from the request body + PG lookups
/// for the customer (`ht_customers`) and the first assigned room
/// (`ht_rooms_new`).
///
/// The legacy `HT_Book_H` row needs `Book_Cust_Name`, `Book_Cust_Tel`, and
/// `Book_Room_Type` (which actually stores the room number per spike §3b).
/// Without populating these the .NET booking list shows blank cells and the
/// receptionist can't identify the booking — the failure mode the [2.28.0]
/// CHANGELOG flagged.
///
/// Multi-room bookings: `HT_Book_Ds` only carries one room number per row in
/// the legacy schema. We pick the first room's price/number for the recipe
/// payload — additional rooms in `body.rooms` still get persisted in
/// `ht_booking_rooms` (PG canonical), but the legacy view only sees the first.
/// This matches the .NET app's own behavior for multi-room bookings (it
/// surfaces them as one HT_Book_Ds row).
async fn build_writeback_context(
    state: &AppState,
    body: &CreateUpdateBookingRequest,
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> ApiResult<BookingWritebackContext> {
    use crate::service::{aggregate_uuid, AggregateKind};

    let stay = DateRange::new(naive_date_to_utc(check_in), naive_date_to_utc(check_out));

    let customer = state.customers.get(&state.new_pool, body.customer_id).await?;
    let (customer_name, customer_phone) = match customer {
        Some(c) => (
            full_customer_name(&c.cust_firstname, c.cust_lastname.as_deref()),
            c.cust_phone,
        ),
        // Customer was deleted between the form submit and the route — fall
        // back to empty so the booking still lands. The receipt will show
        // blank for this booking; matches the legacy app's tolerance.
        None => (String::new(), None),
    };

    let primary_room = body.rooms.first();
    let (room_no, room_type, room_price_baht) = match primary_room {
        Some(req) => {
            let room = state.rooms.get(&state.new_pool, req.room_id).await?;
            let (room_no, room_type, default_weekday) = match room {
                Some(r) => (
                    r.room_no,
                    r.type_name.unwrap_or_default(),
                    r.room_price_weekday,
                ),
                None => (String::new(), String::new(), None),
            };
            // Per-room override on the request beats the room's default; if
            // neither is set, fall back to the booking total (existing
            // behavior for one-room bookings).
            let price = req
                .price_per_night
                .or(default_weekday)
                .or(body.total_amount)
                .unwrap_or(0.0);
            (room_no, room_type, price)
        }
        None => (String::new(), String::new(), body.total_amount.unwrap_or(0.0)),
    };

    Ok(BookingWritebackContext {
        customer_aggregate_id: aggregate_uuid(AggregateKind::Customer, body.customer_id),
        legacy_cust_no: None,
        customer_name,
        customer_phone,
        stay,
        room_no,
        room_type,
        price: money_from_baht_f64(room_price_baht),
        created_by: String::new(),
        notes: body.notes.clone(),
    })
}

/// Join `cust_firstname` + `cust_lastname` with a single space, dropping the
/// trailing space when the last name is missing or empty. Mirrors the
/// `HT_Customers.Cust_name` shape (the legacy schema stores the joined value).
fn full_customer_name(first: &str, last: Option<&str>) -> String {
    match last {
        Some(last) if !last.is_empty() => format!("{} {}", first, last),
        _ => first.to_string(),
    }
}

fn build_snapshot_inputs(
    body: &CreateUpdateBookingRequest,
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> crate::service::BookingSnapshotInputs {
    use crate::domain::booking::BookingState;

    let state = match body.status.as_deref().unwrap_or("pending") {
        "active" | "confirmed" => BookingState::Active,
        "checkedin" | "checked_in" | "checked-in" => BookingState::CheckedIn,
        "completed" => BookingState::Completed,
        "cancelled" | "canceled" => BookingState::Cancelled,
        _ => BookingState::Pending,
    };

    crate::service::BookingSnapshotInputs {
        legacy_book_id: None,
        state,
        stay_start: naive_date_to_utc(check_in),
        stay_end: naive_date_to_utc(check_out),
        room_no: None,
        price: body.total_amount.map(money_from_baht_f64).unwrap_or(Money::ZERO),
    }
}

fn naive_date_to_utc(date: NaiveDate) -> chrono::DateTime<Utc> {
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).expect("hardcoded midnight is valid");
    Utc.from_utc_datetime(&date.and_time(midnight))
}

/// Convert a baht-denominated `f64` from the request DTO to the integer-cent
/// [`Money`] type the service speaks. Mirrors `Money::from_baht` semantics.
fn money_from_baht_f64(baht: f64) -> Money {
    let satang = (baht * 100.0).round() as i64;
    Money::from_satang(satang)
}

/// Translate the service's `Conflict` outcome (booking missing or already
/// terminal) to the route's prior 400 message so the wire contract is
/// preserved verbatim.
fn map_cancel_error(err: crate::service::ServiceError) -> ApiError {
    match err {
        crate::service::ServiceError::Conflict(_)
        | crate::service::ServiceError::NotFound(_) => {
            ApiError::BadRequest("Booking not found or cannot be cancelled".to_string())
        }
        other => other.into(),
    }
}
