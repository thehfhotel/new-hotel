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

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::Row; // Still needed for dynamic queries (list_checkins)

use super::mode::AppState;
use crate::error::{ApiError, ApiResult};
use crate::models::Pagination;

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
    let pool = &state.new_pool;

    // HotelNew DB only contains hfhotel data; hfville selector returns empty.
    if params.branch.as_deref() == Some("hfville") {
        return Ok(Json(NewCheckInsResponse {
            success: true,
            data: vec![],
            pagination: Pagination::new(params.page, params.limit, 0),
        }));
    }

    let offset = (params.page - 1) * params.limit;
    let sort_order = params
        .sort_order
        .as_ref()
        .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
        .unwrap_or("DESC");

    // Map frontend column names to SQL columns
    let order_by_column = match params.sort_by.as_deref() {
        Some("cinNo") => "ci.cin_no",
        Some("customer") => "c.cust_firstname",
        Some("room") => "r.room_no",
        Some("checkInTime") => "ci.cin_checkin_time",
        Some("checkOutTime") => "ci.cin_checkout_time",
        Some("status") => "ci.cin_status",
        _ => "ci.cin_checkin_time",
    };

    // Build WHERE conditions
    let mut conditions: Vec<String> = Vec::new();

    if let Some(ref status) = params.status {
        let escaped = status.replace('\'', "''");
        conditions.push(format!("ci.cin_status = '{}'", escaped));
    }

    // Date range filter
    if let Some(ref start_date) = params.start_date {
        conditions.push(format!(
            "COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout)::date >= '{}'",
            start_date.replace('\'', "''")
        ));
    }

    if let Some(ref end_date) = params.end_date {
        conditions.push(format!(
            "ci.cin_checkin_time::date <= '{}'",
            end_date.replace('\'', "''")
        ));
    }

    if let Some(room_id) = params.room_id {
        conditions.push(format!("ci.cin_room_id = {}", room_id));
    }

    if let Some(cust_id) = params.customer_id {
        conditions.push(format!("ci.cin_cust_id = {}", cust_id));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count query
    let count_query = format!(
        r#"
        SELECT COUNT(*)::int as total
        FROM ht_checkins ci
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
            ci.cin_id,
            ci.cin_no,
            ci.cin_book_id,
            b.book_no,
            ci.cin_cust_id,
            CONCAT(c.cust_firstname, ' ', COALESCE(c.cust_lastname, '')) as customer_name,
            ci.cin_room_id,
            r.room_no,
            rt.type_name,
            ci.cin_checkin_time,
            ci.cin_checkout_time,
            ci.cin_expected_checkout,
            ci.cin_adults,
            ci.cin_children,
            ci.cin_status,
            ci.cin_rate_per_night,
            ci.cin_total_amount,
            ci.cin_payment_status,
            ci.cin_notes,
            ci.created_at,
            ci.updated_at
        FROM ht_checkins ci
        LEFT JOIN ht_customers c ON ci.cin_cust_id = c.cust_id
        LEFT JOIN ht_rooms_new r ON ci.cin_room_id = r.room_id
        LEFT JOIN ht_room_types rt ON r.room_type_id = rt.type_id
        LEFT JOIN ht_bookings b ON ci.cin_book_id = b.book_id
        {}
        ORDER BY {} {}
        LIMIT {} OFFSET {}
        "#,
        where_clause, order_by_column, sort_order, params.limit, offset
    );

    let rows = sqlx::query(&data_query)
        .fetch_all(pool)
        .await?;

    let checkins: Vec<NewCheckIn> = rows
        .iter()
        .map(|row| NewCheckIn {
            id: row.try_get::<i32, _>("cin_id").unwrap_or(0),
            cin_no: row.try_get::<String, _>("cin_no").unwrap_or_default(),
            booking_id: row.try_get::<i32, _>("cin_book_id").ok(),
            booking_no: row.try_get::<String, _>("book_no").ok(),
            customer_id: row.try_get::<i32, _>("cin_cust_id").unwrap_or(0),
            customer_name: row.try_get::<String, _>("customer_name").ok(),
            room_id: row.try_get::<i32, _>("cin_room_id").unwrap_or(0),
            room_no: row.try_get::<String, _>("room_no").ok(),
            room_type_name: row.try_get::<String, _>("type_name").ok(),
            check_in_time: row.try_get::<NaiveDateTime, _>("cin_checkin_time").ok(),
            check_out_time: row.try_get::<NaiveDateTime, _>("cin_checkout_time").ok(),
            expected_checkout: row.try_get::<NaiveDateTime, _>("cin_expected_checkout").ok(),
            adults: row.try_get::<i32, _>("cin_adults").ok(),
            children: row.try_get::<i32, _>("cin_children").ok(),
            status: row.try_get::<String, _>("cin_status").unwrap_or_else(|_| "active".to_string()),
            rate_per_night: row.try_get::<f64, _>("cin_rate_per_night").ok(),
            total_amount: row.try_get::<f64, _>("cin_total_amount").ok(),
            payment_status: row.try_get::<String, _>("cin_payment_status").ok(),
            notes: row.try_get::<String, _>("cin_notes").ok(),
            created_at: row.try_get::<NaiveDateTime, _>("created_at").ok(),
            updated_at: row.try_get::<NaiveDateTime, _>("updated_at").ok(),
        })
        .collect();

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
    let pool = &state.new_pool;

    let rec = sqlx::query!(
        r#"
        SELECT
            ci.cin_id,
            ci.cin_no,
            ci.cin_book_id,
            b.book_no,
            ci.cin_cust_id,
            CONCAT(c.cust_firstname, ' ', COALESCE(c.cust_lastname, '')) as customer_name,
            ci.cin_room_id,
            r.room_no,
            rt.type_name,
            ci.cin_checkin_time,
            ci.cin_checkout_time,
            ci.cin_expected_checkout,
            ci.cin_adults,
            ci.cin_children,
            ci.cin_status,
            ci.cin_rate_per_night::float8 as cin_rate_per_night,
            ci.cin_total_amount::float8 as cin_total_amount,
            ci.cin_payment_status,
            ci.cin_notes,
            ci.created_at,
            ci.updated_at
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

    let checkin = NewCheckIn {
        id: rec.cin_id,
        cin_no: rec.cin_no,
        booking_id: rec.cin_book_id,
        booking_no: Some(rec.book_no),
        customer_id: rec.cin_cust_id,
        customer_name: rec.customer_name,
        room_id: rec.cin_room_id,
        room_no: Some(rec.room_no),
        room_type_name: Some(rec.type_name),
        check_in_time: Some(rec.cin_checkin_time),
        check_out_time: rec.cin_checkout_time,
        expected_checkout: Some(rec.cin_expected_checkout.and_hms_opt(0, 0, 0).unwrap()),
        adults: rec.cin_adults,
        children: rec.cin_children,
        status: rec.cin_status.unwrap_or_else(|| "active".to_string()),
        rate_per_night: rec.cin_rate_per_night,
        total_amount: rec.cin_total_amount,
        payment_status: rec.cin_payment_status,
        notes: rec.cin_notes,
        created_at: rec.created_at,
        updated_at: rec.updated_at,
    };

    Ok(Json(NewCheckInResponse {
        success: true,
        checkin,
    }))
}

/// POST /api/new/checkins - Create check-in (walk-in or from booking)
pub async fn create_checkin(
    State(state): State<AppState>,
    Json(body): Json<CreateCheckInRequest>,
) -> ApiResult<Json<MutationResponse>> {
    let pool = &state.new_pool;

    // Determine customer ID
    let customer_id = if let Some(booking_id) = body.booking_id {
        // Get customer from booking
        let booking_rec = sqlx::query!(
            "SELECT book_cust_id FROM ht_bookings WHERE book_id = $1",
            booking_id
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiError::NotFound("Booking not found".to_string()))?;

        booking_rec.book_cust_id
    } else {
        body.customer_id
            .ok_or_else(|| ApiError::BadRequest("Customer ID is required for walk-ins".to_string()))?
    };

    // Check if room is available
    let room_check = sqlx::query!(
        r#"SELECT COUNT(*)::int as active_count FROM ht_checkins
        WHERE cin_room_id = $1 AND cin_status = 'active'"#,
        body.room_id
    )
    .fetch_one(pool)
    .await?;

    let active_count = room_check.active_count.unwrap_or(0);

    if active_count > 0 {
        return Err(ApiError::BadRequest("Room is currently occupied".to_string()));
    }

    // Generate check-in number (CIN-YYYYMMDD-NNNN format)
    let last_rec = sqlx::query!(
        r#"SELECT cin_no FROM ht_checkins
        WHERE cin_no LIKE 'CIN-' || TO_CHAR(NOW(), 'YYYYMMDD') || '-%'
        ORDER BY cin_no DESC LIMIT 1"#
    )
    .fetch_optional(pool)
    .await?;

    let next_seq = if let Some(rec) = last_rec {
        // cin_no is NOT NULL, use directly
        let last_no = rec.cin_no;
        let parts: Vec<&str> = last_no.split('-').collect();
        if parts.len() == 3 {
            parts[2].parse::<i32>().unwrap_or(0) + 1
        } else {
            1
        }
    } else {
        1
    };

    // Get current date in YYYYMMDD format
    let date_rec = sqlx::query!("SELECT TO_CHAR(NOW(), 'YYYYMMDD') as today")
        .fetch_one(pool)
        .await?;

    let today = date_rec.today.unwrap_or_else(|| "00000000".to_string());

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
                .map_err(|_| ApiError::BadRequest("Invalid check-in time format".to_string()))?
        ),
        None => None,
    };

    // Insert check-in
    let rec = sqlx::query!(
        r#"INSERT INTO ht_checkins (cin_no, cin_book_id, cin_cust_id, cin_room_id, cin_checkin_time, cin_expected_checkout, cin_adults, cin_children, cin_status, cin_rate_per_night, cin_notes)
        VALUES ($1, $2, $3, $4, COALESCE($5, NOW()::timestamp), $6, $7, $8, 'active', $9::float8, $10)
        RETURNING cin_id"#,
        cin_no.as_str(),
        body.booking_id,
        customer_id,
        body.room_id,
        check_in_time,
        expected_checkout_date,
        adults,
        children,
        body.rate_per_night,
        body.notes.as_deref()
    )
    .fetch_one(pool)
    .await?;

    let cin_id = rec.cin_id;

    // Update room status to occupied
    sqlx::query!(
        r#"UPDATE ht_rooms_new SET room_status = 'occupied', updated_at = NOW() WHERE room_id = $1"#,
        body.room_id
    )
    .execute(pool)
    .await?;

    // If from booking, update booking status to checkedin
    if let Some(booking_id) = body.booking_id {
        sqlx::query!(
            r#"UPDATE ht_bookings SET book_status = 'checkedin', updated_at = NOW() WHERE book_id = $1"#,
            booking_id
        )
        .execute(pool)
        .await?;
    }

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
    let pool = &state.new_pool;

    // Get check-in and verify it's active
    let checkin_rec = sqlx::query!(
        r#"SELECT cin_no, cin_room_id, cin_book_id, cin_status FROM ht_checkins WHERE cin_id = $1"#,
        cin_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    let status = checkin_rec.cin_status.unwrap_or_default();
    if status != "active" {
        return Err(ApiError::BadRequest("Check-in is not active".to_string()));
    }

    let cin_no = checkin_rec.cin_no;
    let room_id = checkin_rec.cin_room_id;
    let booking_id = checkin_rec.cin_book_id;

    let payment_status = body.payment_status.as_deref().unwrap_or("paid");

    // Parse optional check-out time
    let check_out_time: Option<NaiveDateTime> = match &body.check_out_time {
        Some(s) => Some(
            NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S")
                .or_else(|_| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S"))
                .or_else(|_| NaiveDate::parse_from_str(s, "%Y-%m-%d").map(|d| d.and_hms_opt(12, 0, 0).unwrap()))
                .map_err(|_| ApiError::BadRequest("Invalid check-out time format".to_string()))?
        ),
        None => None,
    };

    // Update check-in to checked out
    sqlx::query!(
        r#"UPDATE ht_checkins SET cin_checkout_time = COALESCE($1, NOW()::timestamp), cin_status = 'checkedout',
        cin_total_amount = COALESCE($2::float8, cin_total_amount), cin_payment_status = $3,
        cin_notes = COALESCE($4, cin_notes), updated_at = NOW() WHERE cin_id = $5"#,
        check_out_time,
        body.total_amount,
        payment_status,
        body.notes.as_deref(),
        cin_id
    )
    .execute(pool)
    .await?;

    // Update room status to available and mark as dirty
    sqlx::query!(
        r#"UPDATE ht_rooms_new SET room_status = 'available', room_clean = false, updated_at = NOW() WHERE room_id = $1"#,
        room_id
    )
    .execute(pool)
    .await?;

    // If from booking, update booking status to completed
    if let Some(book_id) = booking_id {
        sqlx::query!(
            r#"UPDATE ht_bookings SET book_status = 'completed', updated_at = NOW() WHERE book_id = $1"#,
            book_id
        )
        .execute(pool)
        .await?;
    }

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
    let pool = &state.new_pool;

    // Verify check-in exists
    let exists = sqlx::query!("SELECT cin_id FROM ht_checkins WHERE cin_id = $1", cin_id)
        .fetch_optional(pool)
        .await?;

    if exists.is_none() {
        return Err(ApiError::NotFound("Check-in not found".to_string()));
    }

    // Get guests
    let rows = sqlx::query!(
        r#"SELECT guest_id, guest_cin_id, guest_cust_id, guest_firstname, guest_lastname,
            guest_idcard, guest_passport, guest_nationality, guest_is_primary, guest_created_at
        FROM ht_guest_registry WHERE guest_cin_id = $1
        ORDER BY guest_is_primary DESC, guest_id ASC"#,
        cin_id
    )
    .fetch_all(pool)
    .await?;

    let guests: Vec<Guest> = rows
        .iter()
        .map(|r| Guest {
            id: r.guest_id,
            cin_id: r.guest_cin_id,
            cust_id: r.guest_cust_id,
            first_name: r.guest_firstname.clone(),
            last_name: r.guest_lastname.clone(),
            id_card: r.guest_idcard.clone(),
            passport: r.guest_passport.clone(),
            nationality: r.guest_nationality.clone(),
            is_primary: r.guest_is_primary.unwrap_or(false),
            created_at: r.guest_created_at,
        })
        .collect();

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

    let pool = &state.new_pool;

    // Verify check-in exists and is active
    let checkin_rec = sqlx::query!(
        "SELECT cin_id, cin_status FROM ht_checkins WHERE cin_id = $1",
        cin_id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    let status = checkin_rec.cin_status.unwrap_or_default();
    if status != "active" {
        return Err(ApiError::BadRequest("Cannot add guests to a non-active check-in".to_string()));
    }

    let is_primary = body.is_primary.unwrap_or(false);

    // If this guest is marked as primary, unset any existing primary guest
    if is_primary {
        sqlx::query!(
            "UPDATE ht_guest_registry SET guest_is_primary = false WHERE guest_cin_id = $1",
            cin_id
        )
        .execute(pool)
        .await?;
    }

    // Insert guest
    let rec = sqlx::query!(
        r#"INSERT INTO ht_guest_registry (guest_cin_id, guest_cust_id, guest_firstname, guest_lastname, guest_idcard, guest_passport, guest_nationality, guest_is_primary)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING guest_id"#,
        cin_id,
        body.cust_id,
        first_name,
        body.last_name.as_deref(),
        body.id_card.as_deref(),
        body.passport.as_deref(),
        body.nationality.as_deref(),
        is_primary
    )
    .fetch_one(pool)
    .await?;

    let guest_id = rec.guest_id;

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
    let pool = &state.new_pool;

    // Verify check-in exists
    let checkin_rec = sqlx::query!(
        "SELECT cin_id, cin_status FROM ht_checkins WHERE cin_id = $1",
        path.id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    let status = checkin_rec.cin_status.unwrap_or_default();
    if status != "active" {
        return Err(ApiError::BadRequest("Cannot remove guests from a non-active check-in".to_string()));
    }

    // Verify guest belongs to this check-in
    let guest_exists = sqlx::query!(
        "SELECT guest_id FROM ht_guest_registry WHERE guest_id = $1 AND guest_cin_id = $2",
        path.guest_id,
        path.id
    )
    .fetch_optional(pool)
    .await?;

    if guest_exists.is_none() {
        return Err(ApiError::NotFound("Guest not found for this check-in".to_string()));
    }

    // Delete guest
    sqlx::query!(
        "DELETE FROM ht_guest_registry WHERE guest_id = $1 AND guest_cin_id = $2",
        path.guest_id,
        path.id
    )
    .execute(pool)
    .await?;

    Ok(Json(GuestMutationResponse {
        success: true,
        message: "Guest removed successfully".to_string(),
        id: Some(path.guest_id),
    }))
}
