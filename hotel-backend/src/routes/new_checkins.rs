//! New Check-in API routes for HotelNew database
//!
//! - GET /api/new/checkins - List check-ins from HT_CheckIns
//! - GET /api/new/checkins/:id - Get single check-in
//! - POST /api/new/checkins - Create check-in (walk-in or from booking)
//! - PUT /api/new/checkins/:id/checkout - Process check-out
//!
//! Guest Registry endpoints:
//! - GET /api/new/checkins/:id/guests - List guests for check-in
//! - POST /api/new/checkins/:id/guests - Add guest to check-in
//! - DELETE /api/new/checkins/:id/guests/:guestId - Remove guest from check-in
//!
//! Per `docs/architecture.md` §1, §6 (Phase 1b) the SQL has moved to
//! `repository::checkin`. The route still owns request validation, response
//! shaping, and the multi-step workflow choreography (which the Phase 2
//! service layer will absorb).

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use super::mode::AppState;
use crate::error::{ApiError, ApiResult};
use crate::models::Pagination;
use crate::repository::checkin::{
    CheckInDetailRow, CheckInInsert, CheckInListRow, CheckOutWrite, GuestInsert, GuestRow,
};

/// Check-in status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckInStatus {
    Active,
    CheckedOut,
    Cancelled,
}

impl CheckInStatus {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "checkedout" | "checked_out" => CheckInStatus::CheckedOut,
            "cancelled" | "canceled" => CheckInStatus::Cancelled,
            _ => CheckInStatus::Active,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            CheckInStatus::Active => "active",
            CheckInStatus::CheckedOut => "checkedout",
            CheckInStatus::Cancelled => "cancelled",
        }
    }
}

/// Check-in from HT_CheckIns table
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCheckIn {
    pub id: i32,
    pub cin_no: String,
    pub booking_id: Option<i32>,
    pub booking_no: Option<String>,
    pub customer_id: i32,
    pub customer_name: Option<String>,
    pub room_id: i32,
    pub room_no: Option<String>,
    pub room_type_name: Option<String>,
    pub check_in_time: Option<NaiveDateTime>,
    pub check_out_time: Option<NaiveDateTime>,
    pub expected_checkout: Option<NaiveDateTime>,
    pub adults: Option<i32>,
    pub children: Option<i32>,
    pub status: String,
    pub rate_per_night: Option<f64>,
    pub total_amount: Option<f64>,
    pub payment_status: Option<String>,
    pub notes: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl NewCheckIn {
    fn from_list_row(row: CheckInListRow) -> Self {
        Self {
            id: row.cin_id,
            cin_no: row.cin_no,
            booking_id: row.cin_book_id,
            booking_no: row.book_no,
            customer_id: row.cin_cust_id,
            customer_name: row.customer_name,
            room_id: row.cin_room_id,
            room_no: row.room_no,
            room_type_name: row.type_name,
            check_in_time: row.cin_checkin_time,
            check_out_time: row.cin_checkout_time,
            expected_checkout: row.cin_expected_checkout,
            adults: row.cin_adults,
            children: row.cin_children,
            status: row.cin_status,
            rate_per_night: row.cin_rate_per_night,
            total_amount: row.cin_total_amount,
            payment_status: row.cin_payment_status,
            notes: row.cin_notes,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }

    fn from_detail_row(row: CheckInDetailRow) -> Self {
        Self {
            id: row.cin_id,
            cin_no: row.cin_no,
            booking_id: row.cin_book_id,
            booking_no: Some(row.book_no),
            customer_id: row.cin_cust_id,
            customer_name: row.customer_name,
            room_id: row.cin_room_id,
            room_no: Some(row.room_no),
            room_type_name: Some(row.type_name),
            check_in_time: Some(row.cin_checkin_time),
            check_out_time: row.cin_checkout_time,
            expected_checkout: Some(row.cin_expected_checkout.and_hms_opt(0, 0, 0).unwrap()),
            adults: row.cin_adults,
            children: row.cin_children,
            status: row.cin_status.unwrap_or_else(|| "active".to_string()),
            rate_per_night: row.cin_rate_per_night,
            total_amount: row.cin_total_amount,
            payment_status: row.cin_payment_status,
            notes: row.cin_notes,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

/// Query parameters for check-ins list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCheckInsQuery {
    pub status: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub room_id: Option<i32>,
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

/// Response for check-ins list
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCheckInsResponse {
    pub success: bool,
    pub data: Vec<NewCheckIn>,
    pub pagination: Pagination,
}

/// Response for single check-in
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewCheckInResponse {
    pub success: bool,
    pub checkin: NewCheckIn,
}

/// Request body for creating check-in
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCheckInRequest {
    /// Booking ID if checking in from booking (optional for walk-ins)
    pub booking_id: Option<i32>,
    /// Customer ID (required for walk-ins, optional if booking_id provided)
    pub customer_id: Option<i32>,
    /// Room ID to check into
    pub room_id: i32,
    /// Check-in time (defaults to now)
    pub check_in_time: Option<String>,
    /// Expected check-out date/time
    pub expected_checkout: String,
    pub adults: Option<i32>,
    pub children: Option<i32>,
    pub rate_per_night: Option<f64>,
    pub notes: Option<String>,
}

/// Request body for checkout
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckOutRequest {
    pub check_out_time: Option<String>,
    pub total_amount: Option<f64>,
    pub payment_status: Option<String>,
    pub notes: Option<String>,
}

/// Response for create/checkout operations
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MutationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cin_no: Option<String>,
}

/// GET /api/new/checkins - List check-ins from HT_CheckIns
pub async fn list_checkins(
    State(state): State<AppState>,
    Query(params): Query<NewCheckInsQuery>,
) -> ApiResult<Json<NewCheckInsResponse>> {
    // HotelNew DB only contains hfhotel data; hfville selector returns empty.
    if params.branch.as_deref() == Some("hfville") {
        return Ok(Json(NewCheckInsResponse {
            success: true,
            data: vec![],
            pagination: Pagination::new(params.page, params.limit, 0),
        }));
    }

    let (rows, total) = state
        .checkins
        .list_with_count(&state.new_pool, &params)
        .await?;

    let checkins: Vec<NewCheckIn> = rows.into_iter().map(NewCheckIn::from_list_row).collect();

    Ok(Json(NewCheckInsResponse {
        success: true,
        data: checkins,
        pagination: Pagination::new(params.page, params.limit, total),
    }))
}

/// GET /api/new/checkins/:id - Get single check-in
pub async fn get_checkin(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
) -> ApiResult<Json<NewCheckInResponse>> {
    let row = state
        .checkins
        .get(&state.new_pool, cin_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    Ok(Json(NewCheckInResponse {
        success: true,
        checkin: NewCheckIn::from_detail_row(row),
    }))
}

/// POST /api/new/checkins - Create check-in (walk-in or from booking)
pub async fn create_checkin(
    State(state): State<AppState>,
    Json(body): Json<CreateCheckInRequest>,
) -> ApiResult<Json<MutationResponse>> {
    // Determine customer ID
    let customer_id = if let Some(booking_id) = body.booking_id {
        state
            .checkins
            .get_booking_customer_id(&state.new_pool, booking_id)
            .await?
            .ok_or_else(|| ApiError::NotFound("Booking not found".to_string()))?
    } else {
        body.customer_id
            .ok_or_else(|| ApiError::BadRequest("Customer ID is required for walk-ins".to_string()))?
    };

    // Check if room is available
    let active_count = state
        .checkins
        .count_active_for_room(&state.new_pool, body.room_id)
        .await?;
    if active_count > 0 {
        return Err(ApiError::BadRequest("Room is currently occupied".to_string()));
    }

    // Generate check-in number (CIN-YYYYMMDD-NNNN format)
    let last_cin_no = state.checkins.latest_cin_no_today(&state.new_pool).await?;
    let next_seq = if let Some(last) = last_cin_no {
        let parts: Vec<&str> = last.split('-').collect();
        if parts.len() == 3 {
            parts[2].parse::<i32>().unwrap_or(0) + 1
        } else {
            1
        }
    } else {
        1
    };

    let today = state.checkins.today_yyyymmdd(&state.new_pool).await?;
    let cin_no = format!("CIN-{}-{:04}", today, next_seq);

    let adults = body.adults.unwrap_or(1);
    let children = body.children.unwrap_or(0);

    // Parse expected checkout date
    let expected_checkout_date = NaiveDate::parse_from_str(&body.expected_checkout, "%Y-%m-%d")
        .map_err(|_| ApiError::BadRequest("Invalid expected checkout date format (expected YYYY-MM-DD)".to_string()))?;

    // Parse optional check-in time
    let check_in_time: Option<NaiveDateTime> = match &body.check_in_time {
        Some(s) => Some(
            NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
                .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S"))
                .or_else(|_| NaiveDate::parse_from_str(s, "%Y-%m-%d").map(|d| d.and_hms_opt(14, 0, 0).unwrap()))
                .map_err(|_| ApiError::BadRequest("Invalid check-in time format".to_string()))?,
        ),
        None => None,
    };

    let mut tx = state.new_pool.begin().await?;
    let cin_id = state
        .checkins
        .insert(
            &mut tx,
            CheckInInsert {
                cin_no: cin_no.as_str(),
                booking_id: body.booking_id,
                customer_id,
                room_id: body.room_id,
                check_in_time,
                expected_checkout: expected_checkout_date,
                adults,
                children,
                rate_per_night: body.rate_per_night,
                notes: body.notes.as_deref(),
            },
        )
        .await?;

    state.checkins.mark_room_occupied(&mut tx, body.room_id).await?;

    if let Some(booking_id) = body.booking_id {
        state.checkins.set_booking_checkedin(&mut tx, booking_id).await?;
    }

    tx.commit().await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Check-in created successfully".to_string(),
        id: Some(cin_id),
        cin_no: Some(cin_no),
    }))
}

/// PUT /api/new/checkins/:id/checkout - Process check-out
pub async fn checkout(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
    Json(body): Json<CheckOutRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let status_snap = state
        .checkins
        .find_status(&state.new_pool, cin_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    let status = status_snap.cin_status.unwrap_or_default();
    if status != "active" {
        return Err(ApiError::BadRequest("Check-in is not active".to_string()));
    }

    let cin_no = status_snap.cin_no;
    let room_id = status_snap.cin_room_id;
    let booking_id = status_snap.cin_book_id;

    let payment_status = body.payment_status.as_deref().unwrap_or("paid");

    // Parse optional check-out time
    let check_out_time: Option<NaiveDateTime> = match &body.check_out_time {
        Some(s) => Some(
            NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
                .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S"))
                .or_else(|_| NaiveDate::parse_from_str(s, "%Y-%m-%d").map(|d| d.and_hms_opt(12, 0, 0).unwrap()))
                .map_err(|_| ApiError::BadRequest("Invalid check-out time format".to_string()))?,
        ),
        None => None,
    };

    let mut tx = state.new_pool.begin().await?;
    state
        .checkins
        .apply_checkout(
            &mut tx,
            cin_id,
            CheckOutWrite {
                check_out_time,
                total_amount: body.total_amount,
                payment_status,
                notes: body.notes.as_deref(),
            },
        )
        .await?;

    state.checkins.mark_room_available_dirty(&mut tx, room_id).await?;

    if let Some(book_id) = booking_id {
        state.checkins.set_booking_completed(&mut tx, book_id).await?;
    }

    tx.commit().await?;

    Ok(Json(MutationResponse {
        success: true,
        message: "Check-out completed successfully".to_string(),
        id: Some(cin_id),
        cin_no: Some(cin_no),
    }))
}

// =============================================================================
// Guest Registry Types and Endpoints
// =============================================================================

/// Guest from HT_Guest_Registry table
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Guest {
    pub id: i32,
    pub cin_id: i32,
    pub cust_id: Option<i32>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub id_card: Option<String>,
    pub passport: Option<String>,
    pub nationality: Option<String>,
    pub is_primary: bool,
    pub created_at: Option<NaiveDateTime>,
}

impl Guest {
    fn from_row(row: GuestRow) -> Self {
        Self {
            id: row.guest_id,
            cin_id: row.guest_cin_id,
            cust_id: row.guest_cust_id,
            first_name: row.guest_firstname,
            last_name: row.guest_lastname,
            id_card: row.guest_idcard,
            passport: row.guest_passport,
            nationality: row.guest_nationality,
            is_primary: row.guest_is_primary.unwrap_or(false),
            created_at: row.guest_created_at,
        }
    }
}

/// Response for guests list
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestsResponse {
    pub success: bool,
    pub data: Vec<Guest>,
    pub total: i32,
}

/// Response for single guest
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestResponse {
    pub success: bool,
    pub guest: Guest,
}

/// Request body for creating a guest
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateGuestRequest {
    /// Customer ID if linking to existing customer (optional)
    pub cust_id: Option<i32>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub id_card: Option<String>,
    pub passport: Option<String>,
    pub nationality: Option<String>,
    pub is_primary: Option<bool>,
}

/// Response for guest mutation operations
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GuestMutationResponse {
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

/// GET /api/new/checkins/:id/guests - List guests for check-in
pub async fn list_guests(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
) -> ApiResult<Json<GuestsResponse>> {
    if !state.checkins.exists(&state.new_pool, cin_id).await? {
        return Err(ApiError::NotFound("Check-in not found".to_string()));
    }

    let rows = state.checkins.list_guests(&state.new_pool, cin_id).await?;
    let guests: Vec<Guest> = rows.into_iter().map(Guest::from_row).collect();
    let total = guests.len() as i32;

    Ok(Json(GuestsResponse {
        success: true,
        data: guests,
        total,
    }))
}

/// POST /api/new/checkins/:id/guests - Add guest to check-in
pub async fn create_guest(
    State(state): State<AppState>,
    Path(cin_id): Path<i32>,
    Json(body): Json<CreateGuestRequest>,
) -> ApiResult<Json<GuestMutationResponse>> {
    let first_name = body.first_name.trim();
    if first_name.is_empty() {
        return Err(ApiError::BadRequest("Guest first name is required".to_string()));
    }

    let status_snap = state
        .checkins
        .find_status(&state.new_pool, cin_id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    let status = status_snap.cin_status.unwrap_or_default();
    if status != "active" {
        return Err(ApiError::BadRequest("Cannot add guests to a non-active check-in".to_string()));
    }

    let is_primary = body.is_primary.unwrap_or(false);

    let mut tx = state.new_pool.begin().await?;

    // If this guest is marked as primary, unset any existing primary guest
    if is_primary {
        state.checkins.unset_primary_guests(&mut tx, cin_id).await?;
    }

    let guest_id = state
        .checkins
        .insert_guest(
            &mut tx,
            GuestInsert {
                cin_id,
                cust_id: body.cust_id,
                first_name,
                last_name: body.last_name.as_deref(),
                id_card: body.id_card.as_deref(),
                passport: body.passport.as_deref(),
                nationality: body.nationality.as_deref(),
                is_primary,
            },
        )
        .await?;

    tx.commit().await?;

    Ok(Json(GuestMutationResponse {
        success: true,
        message: "Guest added successfully".to_string(),
        id: Some(guest_id),
    }))
}

/// Path parameters for guest operations
#[derive(Debug, Deserialize)]
pub struct GuestPath {
    pub id: i32,
    pub guest_id: i32,
}

/// DELETE /api/new/checkins/:id/guests/:guestId - Remove guest from check-in
pub async fn delete_guest(
    State(state): State<AppState>,
    Path(path): Path<GuestPath>,
) -> ApiResult<Json<GuestMutationResponse>> {
    let status_snap = state
        .checkins
        .find_status(&state.new_pool, path.id)
        .await?
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    let status = status_snap.cin_status.unwrap_or_default();
    if status != "active" {
        return Err(ApiError::BadRequest("Cannot remove guests from a non-active check-in".to_string()));
    }

    let exists = state
        .checkins
        .find_guest_in_checkin(&state.new_pool, path.id, path.guest_id)
        .await?;
    if exists.is_none() {
        return Err(ApiError::NotFound("Guest not found for this check-in".to_string()));
    }

    let mut tx = state.new_pool.begin().await?;
    state
        .checkins
        .delete_guest(&mut tx, path.id, path.guest_id)
        .await?;
    tx.commit().await?;

    Ok(Json(GuestMutationResponse {
        success: true,
        message: "Guest removed successfully".to_string(),
        id: Some(path.guest_id),
    }))
}
