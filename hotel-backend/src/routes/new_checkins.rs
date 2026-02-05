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
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

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
    let mut conn = state.new_pool.get().await?;

    let offset = (params.page - 1) * params.limit;
    let sort_order = params
        .sort_order
        .as_ref()
        .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
        .unwrap_or("DESC");

    // Map frontend column names to SQL columns
    let order_by_column = match params.sort_by.as_deref() {
        Some("cinNo") => "ci.Cin_No",
        Some("customer") => "c.Cust_FirstName",
        Some("room") => "r.Room_No",
        Some("checkInTime") => "ci.Cin_CheckIn_Time",
        Some("checkOutTime") => "ci.Cin_CheckOut_Time",
        Some("status") => "ci.Cin_Status",
        _ => "ci.Cin_CheckIn_Time",
    };

    // Build WHERE conditions
    let mut conditions: Vec<String> = Vec::new();

    if let Some(ref status) = params.status {
        let escaped = status.replace('\'', "''");
        conditions.push(format!("ci.Cin_Status = '{}'", escaped));
    }

    // Date range filter
    if let Some(ref start_date) = params.start_date {
        conditions.push(format!(
            "CAST(COALESCE(ci.Cin_CheckOut_Time, ci.Cin_Expected_Checkout) AS DATE) >= '{}'",
            start_date.replace('\'', "''")
        ));
    }

    if let Some(ref end_date) = params.end_date {
        conditions.push(format!(
            "CAST(ci.Cin_CheckIn_Time AS DATE) <= '{}'",
            end_date.replace('\'', "''")
        ));
    }

    if let Some(room_id) = params.room_id {
        conditions.push(format!("ci.Cin_Room_ID = {}", room_id));
    }

    if let Some(cust_id) = params.customer_id {
        conditions.push(format!("ci.Cin_Cust_ID = {}", cust_id));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count query
    let count_query = format!(
        r#"
        SELECT COUNT(*) as total
        FROM HT_CheckIns ci
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
            ci.Cin_ID,
            ci.Cin_No,
            ci.Cin_Book_ID,
            b.Book_No,
            ci.Cin_Cust_ID,
            CONCAT(c.Cust_FirstName, ' ', COALESCE(c.Cust_LastName, '')) as Customer_Name,
            ci.Cin_Room_ID,
            r.Room_No,
            rt.Type_Name,
            ci.Cin_CheckIn_Time,
            ci.Cin_CheckOut_Time,
            ci.Cin_Expected_Checkout,
            ci.Cin_Adults,
            ci.Cin_Children,
            ci.Cin_Status,
            ci.Cin_Rate_Per_Night,
            ci.Cin_Total_Amount,
            ci.Cin_Payment_Status,
            ci.Cin_Notes,
            ci.Created_At,
            ci.Updated_At
        FROM HT_CheckIns ci
        LEFT JOIN HT_Customers c ON ci.Cin_Cust_ID = c.Cust_ID
        LEFT JOIN HT_Rooms_New r ON ci.Cin_Room_ID = r.Room_ID
        LEFT JOIN HT_Room_Types rt ON r.Room_Type_ID = rt.Type_ID
        LEFT JOIN HT_Bookings b ON ci.Cin_Book_ID = b.Book_ID
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

    let checkins: Vec<NewCheckIn> = rows
        .iter()
        .map(|row| NewCheckIn {
            id: row.get::<i32, _>("Cin_ID").unwrap_or(0),
            cin_no: row.get::<&str, _>("Cin_No").unwrap_or_default().to_string(),
            booking_id: row.get::<i32, _>("Cin_Book_ID"),
            booking_no: row.get::<&str, _>("Book_No").map(String::from),
            customer_id: row.get::<i32, _>("Cin_Cust_ID").unwrap_or(0),
            customer_name: row.get::<&str, _>("Customer_Name").map(String::from),
            room_id: row.get::<i32, _>("Cin_Room_ID").unwrap_or(0),
            room_no: row.get::<&str, _>("Room_No").map(String::from),
            room_type_name: row.get::<&str, _>("Type_Name").map(String::from),
            check_in_time: row.get::<NaiveDateTime, _>("Cin_CheckIn_Time"),
            check_out_time: row.get::<NaiveDateTime, _>("Cin_CheckOut_Time"),
            expected_checkout: row.get::<NaiveDateTime, _>("Cin_Expected_Checkout"),
            adults: row.get::<i32, _>("Cin_Adults"),
            children: row.get::<i32, _>("Cin_Children"),
            status: row.get::<&str, _>("Cin_Status").unwrap_or("active").to_string(),
            rate_per_night: row.get::<f64, _>("Cin_Rate_Per_Night"),
            total_amount: row.get::<f64, _>("Cin_Total_Amount"),
            payment_status: row.get::<&str, _>("Cin_Payment_Status").map(String::from),
            notes: row.get::<&str, _>("Cin_Notes").map(String::from),
            created_at: row.get::<NaiveDateTime, _>("Created_At"),
            updated_at: row.get::<NaiveDateTime, _>("Updated_At"),
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
    let mut conn = state.new_pool.get().await?;

    let rows = conn
        .query(
            r#"
            SELECT
                ci.Cin_ID,
                ci.Cin_No,
                ci.Cin_Book_ID,
                b.Book_No,
                ci.Cin_Cust_ID,
                CONCAT(c.Cust_FirstName, ' ', COALESCE(c.Cust_LastName, '')) as Customer_Name,
                ci.Cin_Room_ID,
                r.Room_No,
                rt.Type_Name,
                ci.Cin_CheckIn_Time,
                ci.Cin_CheckOut_Time,
                ci.Cin_Expected_Checkout,
                ci.Cin_Adults,
                ci.Cin_Children,
                ci.Cin_Status,
                ci.Cin_Rate_Per_Night,
                ci.Cin_Total_Amount,
                ci.Cin_Payment_Status,
                ci.Cin_Notes,
                ci.Created_At,
                ci.Updated_At
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

    let checkin = NewCheckIn {
        id: row.get::<i32, _>("Cin_ID").unwrap_or(0),
        cin_no: row.get::<&str, _>("Cin_No").unwrap_or_default().to_string(),
        booking_id: row.get::<i32, _>("Cin_Book_ID"),
        booking_no: row.get::<&str, _>("Book_No").map(String::from),
        customer_id: row.get::<i32, _>("Cin_Cust_ID").unwrap_or(0),
        customer_name: row.get::<&str, _>("Customer_Name").map(String::from),
        room_id: row.get::<i32, _>("Cin_Room_ID").unwrap_or(0),
        room_no: row.get::<&str, _>("Room_No").map(String::from),
        room_type_name: row.get::<&str, _>("Type_Name").map(String::from),
        check_in_time: row.get::<NaiveDateTime, _>("Cin_CheckIn_Time"),
        check_out_time: row.get::<NaiveDateTime, _>("Cin_CheckOut_Time"),
        expected_checkout: row.get::<NaiveDateTime, _>("Cin_Expected_Checkout"),
        adults: row.get::<i32, _>("Cin_Adults"),
        children: row.get::<i32, _>("Cin_Children"),
        status: row.get::<&str, _>("Cin_Status").unwrap_or("active").to_string(),
        rate_per_night: row.get::<f64, _>("Cin_Rate_Per_Night"),
        total_amount: row.get::<f64, _>("Cin_Total_Amount"),
        payment_status: row.get::<&str, _>("Cin_Payment_Status").map(String::from),
        notes: row.get::<&str, _>("Cin_Notes").map(String::from),
        created_at: row.get::<NaiveDateTime, _>("Created_At"),
        updated_at: row.get::<NaiveDateTime, _>("Updated_At"),
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
    let mut conn = state.new_pool.get().await?;

    // Determine customer ID
    let customer_id = if let Some(booking_id) = body.booking_id {
        // Get customer from booking
        let booking_rows = conn
            .query(
                "SELECT Book_Cust_ID FROM HT_Bookings WHERE Book_ID = @P1",
                &[&booking_id],
            )
            .await?
            .into_first_result()
            .await?;

        booking_rows
            .first()
            .and_then(|r| r.get::<i32, _>("Book_Cust_ID"))
            .ok_or_else(|| ApiError::NotFound("Booking not found".to_string()))?
    } else {
        body.customer_id
            .ok_or_else(|| ApiError::BadRequest("Customer ID is required for walk-ins".to_string()))?
    };

    // Check if room is available
    let room_check_rows = conn
        .query(
            r#"
            SELECT COUNT(*) as active_count
            FROM HT_CheckIns
            WHERE Cin_Room_ID = @P1 AND Cin_Status = 'active'
            "#,
            &[&body.room_id],
        )
        .await?
        .into_first_result()
        .await?;

    let active_count: i32 = room_check_rows
        .first()
        .and_then(|r| r.get::<i32, _>("active_count"))
        .unwrap_or(0);

    if active_count > 0 {
        return Err(ApiError::BadRequest("Room is currently occupied".to_string()));
    }

    // Generate check-in number (CIN-YYYYMMDD-NNNN format)
    let cin_no_rows = conn
        .simple_query(
            r#"
            SELECT TOP 1 Cin_No
            FROM HT_CheckIns
            WHERE Cin_No LIKE CONCAT('CIN-', FORMAT(GETDATE(), 'yyyyMMdd'), '-%')
            ORDER BY Cin_No DESC
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let next_seq = if let Some(row) = cin_no_rows.first() {
        if let Some(last_no) = row.get::<&str, _>("Cin_No") {
            // Extract sequence number and increment
            let parts: Vec<&str> = last_no.split('-').collect();
            if parts.len() == 3 {
                parts[2].parse::<i32>().unwrap_or(0) + 1
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

    let cin_no = format!("CIN-{}-{:04}", today, next_seq);

    let adults = body.adults.unwrap_or(1);
    let children = body.children.unwrap_or(0);

    // Insert check-in
    let checkin_rows = conn
        .query(
            r#"
            INSERT INTO HT_CheckIns (
                Cin_No,
                Cin_Book_ID,
                Cin_Cust_ID,
                Cin_Room_ID,
                Cin_CheckIn_Time,
                Cin_Expected_Checkout,
                Cin_Adults,
                Cin_Children,
                Cin_Status,
                Cin_Rate_Per_Night,
                Cin_Notes
            )
            OUTPUT INSERTED.Cin_ID
            VALUES (
                @P1, @P2, @P3, @P4,
                COALESCE(@P5, GETDATE()),
                @P6, @P7, @P8, 'active', @P9, @P10
            )
            "#,
            &[
                &cin_no.as_str(),
                &body.booking_id,
                &customer_id,
                &body.room_id,
                &body.check_in_time.as_deref(),
                &body.expected_checkout.as_str(),
                &adults,
                &children,
                &body.rate_per_night,
                &body.notes.as_deref(),
            ],
        )
        .await?
        .into_first_result()
        .await?;

    let cin_id = checkin_rows
        .first()
        .and_then(|r| r.get::<i32, _>("Cin_ID"))
        .ok_or_else(|| ApiError::Internal("Failed to create check-in".to_string()))?;

    // Update room status to occupied
    conn.execute(
        r#"
        UPDATE HT_Rooms_New
        SET Room_Status = 'occupied',
            Updated_At = GETDATE()
        WHERE Room_ID = @P1
        "#,
        &[&body.room_id],
    )
    .await?;

    // If from booking, update booking status to checkedin
    if let Some(booking_id) = body.booking_id {
        conn.execute(
            r#"
            UPDATE HT_Bookings
            SET Book_Status = 'checkedin',
                Updated_At = GETDATE()
            WHERE Book_ID = @P1
            "#,
            &[&booking_id],
        )
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
    let mut conn = state.new_pool.get().await?;

    // Get check-in and verify it's active
    let checkin_rows = conn
        .query(
            r#"
            SELECT Cin_No, Cin_Room_ID, Cin_Book_ID, Cin_Status
            FROM HT_CheckIns
            WHERE Cin_ID = @P1
            "#,
            &[&cin_id],
        )
        .await?
        .into_first_result()
        .await?;

    let checkin_row = checkin_rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    let status = checkin_row.get::<&str, _>("Cin_Status").unwrap_or_default();
    if status != "active" {
        return Err(ApiError::BadRequest("Check-in is not active".to_string()));
    }

    let cin_no = checkin_row.get::<&str, _>("Cin_No").unwrap_or_default().to_string();
    let room_id: i32 = checkin_row.get::<i32, _>("Cin_Room_ID").unwrap_or(0);
    let booking_id: Option<i32> = checkin_row.get::<i32, _>("Cin_Book_ID");

    let payment_status = body.payment_status.as_deref().unwrap_or("paid");

    // Update check-in to checked out
    conn.execute(
        r#"
        UPDATE HT_CheckIns
        SET Cin_CheckOut_Time = COALESCE(@P1, GETDATE()),
            Cin_Status = 'checkedout',
            Cin_Total_Amount = COALESCE(@P2, Cin_Total_Amount),
            Cin_Payment_Status = @P3,
            Cin_Notes = COALESCE(@P4, Cin_Notes),
            Updated_At = GETDATE()
        WHERE Cin_ID = @P5
        "#,
        &[
            &body.check_out_time.as_deref(),
            &body.total_amount,
            &payment_status,
            &body.notes.as_deref(),
            &cin_id,
        ],
    )
    .await?;

    // Update room status to available and mark as dirty
    conn.execute(
        r#"
        UPDATE HT_Rooms_New
        SET Room_Status = 'available',
            Room_Clean = 0,
            Updated_At = GETDATE()
        WHERE Room_ID = @P1
        "#,
        &[&room_id],
    )
    .await?;

    // If from booking, update booking status to completed
    if let Some(book_id) = booking_id {
        conn.execute(
            r#"
            UPDATE HT_Bookings
            SET Book_Status = 'completed',
                Updated_At = GETDATE()
            WHERE Book_ID = @P1
            "#,
            &[&book_id],
        )
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
    let mut conn = state.new_pool.get().await?;

    // Verify check-in exists
    let checkin_rows = conn
        .query(
            "SELECT Cin_ID FROM HT_CheckIns WHERE Cin_ID = @P1",
            &[&cin_id],
        )
        .await?
        .into_first_result()
        .await?;

    if checkin_rows.is_empty() {
        return Err(ApiError::NotFound("Check-in not found".to_string()));
    }

    // Get guests
    let rows = conn
        .query(
            r#"
            SELECT
                Guest_ID,
                Guest_Cin_ID,
                Guest_Cust_ID,
                Guest_FirstName,
                Guest_LastName,
                Guest_IDCard,
                Guest_Passport,
                Guest_Nationality,
                Guest_Is_Primary,
                Guest_Created_At
            FROM HT_Guest_Registry
            WHERE Guest_Cin_ID = @P1
            ORDER BY Guest_Is_Primary DESC, Guest_ID ASC
            "#,
            &[&cin_id],
        )
        .await?
        .into_first_result()
        .await?;

    let guests: Vec<Guest> = rows
        .iter()
        .map(|row| Guest {
            id: row.get::<i32, _>("Guest_ID").unwrap_or(0),
            cin_id: row.get::<i32, _>("Guest_Cin_ID").unwrap_or(0),
            cust_id: row.get::<i32, _>("Guest_Cust_ID"),
            first_name: row.get::<&str, _>("Guest_FirstName").unwrap_or_default().to_string(),
            last_name: row.get::<&str, _>("Guest_LastName").map(String::from),
            id_card: row.get::<&str, _>("Guest_IDCard").map(String::from),
            passport: row.get::<&str, _>("Guest_Passport").map(String::from),
            nationality: row.get::<&str, _>("Guest_Nationality").map(String::from),
            is_primary: row.get::<bool, _>("Guest_Is_Primary").unwrap_or(false),
            created_at: row.get::<NaiveDateTime, _>("Guest_Created_At"),
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

    let mut conn = state.new_pool.get().await?;

    // Verify check-in exists and is active
    let checkin_rows = conn
        .query(
            "SELECT Cin_ID, Cin_Status FROM HT_CheckIns WHERE Cin_ID = @P1",
            &[&cin_id],
        )
        .await?
        .into_first_result()
        .await?;

    let checkin_row = checkin_rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    let status = checkin_row.get::<&str, _>("Cin_Status").unwrap_or_default();
    if status != "active" {
        return Err(ApiError::BadRequest("Cannot add guests to a non-active check-in".to_string()));
    }

    let is_primary = body.is_primary.unwrap_or(false);

    // If this guest is marked as primary, unset any existing primary guest
    if is_primary {
        conn.execute(
            "UPDATE HT_Guest_Registry SET Guest_Is_Primary = 0 WHERE Guest_Cin_ID = @P1",
            &[&cin_id],
        )
        .await?;
    }

    // Insert guest
    let rows = conn
        .query(
            r#"
            INSERT INTO HT_Guest_Registry (
                Guest_Cin_ID,
                Guest_Cust_ID,
                Guest_FirstName,
                Guest_LastName,
                Guest_IDCard,
                Guest_Passport,
                Guest_Nationality,
                Guest_Is_Primary
            )
            OUTPUT INSERTED.Guest_ID
            VALUES (@P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8)
            "#,
            &[
                &cin_id,
                &body.cust_id,
                &first_name,
                &body.last_name.as_deref(),
                &body.id_card.as_deref(),
                &body.passport.as_deref(),
                &body.nationality.as_deref(),
                &is_primary,
            ],
        )
        .await?
        .into_first_result()
        .await?;

    let guest_id = rows
        .first()
        .and_then(|r| r.get::<i32, _>("Guest_ID"))
        .ok_or_else(|| ApiError::Internal("Failed to create guest".to_string()))?;

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
    let mut conn = state.new_pool.get().await?;

    // Verify check-in exists
    let checkin_rows = conn
        .query(
            "SELECT Cin_ID, Cin_Status FROM HT_CheckIns WHERE Cin_ID = @P1",
            &[&path.id],
        )
        .await?
        .into_first_result()
        .await?;

    let checkin_row = checkin_rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Check-in not found".to_string()))?;

    let status = checkin_row.get::<&str, _>("Cin_Status").unwrap_or_default();
    if status != "active" {
        return Err(ApiError::BadRequest("Cannot remove guests from a non-active check-in".to_string()));
    }

    // Verify guest belongs to this check-in
    let guest_rows = conn
        .query(
            "SELECT Guest_ID FROM HT_Guest_Registry WHERE Guest_ID = @P1 AND Guest_Cin_ID = @P2",
            &[&path.guest_id, &path.id],
        )
        .await?
        .into_first_result()
        .await?;

    if guest_rows.is_empty() {
        return Err(ApiError::NotFound("Guest not found for this check-in".to_string()));
    }

    // Delete guest
    conn.execute(
        "DELETE FROM HT_Guest_Registry WHERE Guest_ID = @P1 AND Guest_Cin_ID = @P2",
        &[&path.guest_id, &path.id],
    )
    .await?;

    Ok(Json(GuestMutationResponse {
        success: true,
        message: "Guest removed successfully".to_string(),
        id: Some(path.guest_id),
    }))
}
