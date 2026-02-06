//! Booking API routes
//!
//! - GET /api/bookings - List bookings (paginated) - uses legacy DB
//! - GET /api/bookings/:id - Get booking details - uses legacy DB for booking, HotelNew for notes
//! - GET /api/bookings/:id/notes - Get booking notes - uses HotelNew DB
//! - POST /api/bookings/:id/notes - Add booking note - uses HotelNew DB
//! - DELETE /api/bookings/:id/notes - Delete booking note - uses HotelNew DB
//!
//! Note: HT_Booking_Notes table is stored in HotelNew database to ensure
//! the legacy database remains read-only.

use std::collections::HashMap;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::db::DbPool;
use crate::error::{ApiError, ApiResult};
use crate::models::{
    get_status_code, map_status, Booking, BookingCustomer, BookingCustomerDetail, BookingDetail,
    BookingDetailResponse, BookingRoom, BookingRoomDetail, BookingsResponse,
    CreateNoteRequest, CreateNoteResponse, DeleteNoteResponse, Note, NotesResponse, Pagination,
};
use crate::routes::mode::AppState;

/// Query parameters for bookings list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookingsQuery {
    pub search: Option<String>,
    pub status: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
    #[serde(default = "default_sort_by")]
    pub sort_by: String,
    #[serde(default = "default_sort_order")]
    pub sort_order: String,
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 20 }
fn default_sort_by() -> String { "bookDate".to_string() }
fn default_sort_order() -> String { "desc".to_string() }

/// GET /api/bookings - List bookings (paginated)
pub async fn list_bookings(
    State(pool): State<DbPool>,
    Query(params): Query<BookingsQuery>,
) -> ApiResult<Json<BookingsResponse>> {
    let mut conn = pool.get().await?;

    // Build WHERE conditions with direct value interpolation
    let mut conditions: Vec<String> = Vec::new();

    if let Some(ref search) = params.search {
        let escaped = search.replace('\'', "''");
        conditions.push(format!(
            "(Book_No LIKE '%{}%' OR Book_Cust_Name LIKE '%{}%')",
            escaped, escaped
        ));
    }

    if let Some(ref status) = params.status {
        if let Some(code) = get_status_code(status) {
            conditions.push(format!("Book_Status = {}", code));
        }
    }

    // Date range filter: find bookings that OVERLAP the given range
    // A booking overlaps if it starts before the end AND ends after the start
    // Use CAST to compare dates without time component
    if let Some(ref start_date) = params.start_date {
        conditions.push(format!("CAST(Book_Date_out AS DATE) >= '{}'", start_date.replace('\'', "''")));
    }

    if let Some(ref end_date) = params.end_date {
        conditions.push(format!("CAST(Book_Date_in AS DATE) <= '{}'", end_date.replace('\'', "''")));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count distinct bookings
    let count_query = format!(
        "SELECT COUNT(DISTINCT Book_No) as total FROM View_Booking_Ds {}",
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

    // Get all data (group and paginate in Rust like the Node.js version)
    let data_query = format!(
        r#"
        SELECT
            Book_No,
            Book_Date,
            Book_Date_in,
            Book_Date_out,
            Book_Cust_Name,
            Book_Status,
            Book_Room_Type
        FROM View_Booking_Ds
        {}
        ORDER BY Book_Date DESC
        "#,
        where_clause
    );

    let rows = conn
        .simple_query(&data_query)
        .await?
        .into_first_result()
        .await?;

    // Group records by Book_No
    let mut grouped: HashMap<String, Booking> = HashMap::new();

    for row in &rows {
        let book_no = row
            .get::<&str, _>("Book_No")
            .unwrap_or_default()
            .to_string();

        let entry = grouped.entry(book_no.clone()).or_insert_with(|| Booking {
            book_no: book_no.clone(),
            book_date: row.get::<NaiveDateTime, _>("Book_Date"),
            check_in: row.get::<NaiveDateTime, _>("Book_Date_in"),
            check_out: row.get::<NaiveDateTime, _>("Book_Date_out"),
            customer: BookingCustomer {
                name: row
                    .get::<&str, _>("Book_Cust_Name")
                    .unwrap_or_default()
                    .to_string(),
            },
            status: map_status(row.get::<i32, _>("Book_Status")),
            rooms: Vec::new(),
            room_count: 0,
        });

        entry.rooms.push(BookingRoom {
            room_no: "-".to_string(),
            room_type: row
                .get::<&str, _>("Book_Room_Type")
                .unwrap_or("-")
                .to_string(),
        });
        entry.room_count = entry.rooms.len();
    }

    let mut all_grouped: Vec<Booking> = grouped.into_values().collect();

    // Sort by requested field
    let sort_asc = params.sort_order.to_lowercase() == "asc";
    all_grouped.sort_by(|a, b| {
        let cmp = match params.sort_by.as_str() {
            "bookNo" => a.book_no.cmp(&b.book_no),
            "status" => a.status.cmp(&b.status),
            "customer" => a.customer.name.to_lowercase().cmp(&b.customer.name.to_lowercase()),
            "checkIn" => a.check_in.cmp(&b.check_in),
            "checkOut" => a.check_out.cmp(&b.check_out),
            "roomCount" => a.room_count.cmp(&b.room_count),
            _ => a.book_date.cmp(&b.book_date), // bookDate default
        };
        if sort_asc { cmp } else { cmp.reverse() }
    });

    // Paginate
    let start_idx = ((params.page - 1) * params.limit) as usize;
    let paginated_data: Vec<Booking> = all_grouped
        .into_iter()
        .skip(start_idx)
        .take(params.limit as usize)
        .collect();

    Ok(Json(BookingsResponse {
        success: true,
        data: paginated_data,
        pagination: Pagination::new(params.page, params.limit, total),
    }))
}

/// GET /api/bookings/:id - Get booking details (uses legacy DB for booking, HotelNew for notes)
///
/// In New-only mode (when legacy DB is unavailable), this will try to fetch
/// from the new_pool which may not have View_Booking_Ds. The route will return
/// a 404 in that case.
pub async fn get_booking(
    State(state): State<AppState>,
    Path(book_no): Path<String>,
) -> ApiResult<Json<BookingDetailResponse>> {
    // Use legacy pool for booking data (read-only)
    // Note: In new-only mode, legacy_pool == new_pool, so this query will fail
    // gracefully since View_Booking_Ds doesn't exist in HotelNew
    let mut legacy_conn = state.legacy_pool.get().await?;

    // Fetch booking with all rooms from legacy database
    let booking_rows = match legacy_conn
        .query(
            r#"
            SELECT
                Book_No,
                Book_Date,
                Book_Date_in,
                Book_Date_out,
                Book_Cust_Name,
                Book_Status,
                Book_Room_Type
            FROM View_Booking_Ds
            WHERE Book_No = @P1
            ORDER BY Book_Room_Type
            "#,
            &[&book_no],
        )
        .await
    {
        Ok(result) => result.into_first_result().await?,
        Err(_) => {
            // View_Booking_Ds doesn't exist (likely running in new-only mode)
            // Return 404 since legacy bookings are not available
            return Err(ApiError::NotFound(
                "Legacy booking not available (running in new-only mode)".to_string()
            ));
        }
    };

    if booking_rows.is_empty() {
        return Err(ApiError::NotFound("Booking not found".to_string()));
    }

    let first_record = &booking_rows[0];

    // Use new_pool for notes (HotelNew database)
    let mut new_conn = state.new_pool.get().await?;

    // Ensure notes table exists in HotelNew
    ensure_notes_table(&mut new_conn).await?;

    // Try to fetch notes from HotelNew database
    let notes = match new_conn
        .query(
            r#"
            SELECT Note_ID, Note_Text, Created_At, Updated_At
            FROM HT_Booking_Notes
            WHERE Book_No = @P1
            ORDER BY Created_At DESC
            "#,
            &[&book_no],
        )
        .await
    {
        Ok(result) => {
            let note_rows = result.into_first_result().await.unwrap_or_default();
            note_rows
                .iter()
                .map(|row| Note {
                    id: row.get::<i32, _>("Note_ID").unwrap_or(0),
                    text: row
                        .get::<&str, _>("Note_Text")
                        .unwrap_or_default()
                        .to_string(),
                    created_at: row.get::<NaiveDateTime, _>("Created_At"),
                    updated_at: row.get::<NaiveDateTime, _>("Updated_At"),
                })
                .collect()
        }
        Err(_) => Vec::new(), // Table doesn't exist yet
    };

    // Build rooms array
    let rooms: Vec<BookingRoomDetail> = booking_rows
        .iter()
        .map(|r| BookingRoomDetail {
            room_no: "-".to_string(),
            room_type: r.get::<&str, _>("Book_Room_Type").unwrap_or("-").to_string(),
            total: 0.0,
        })
        .collect();

    let booking = BookingDetail {
        book_no: first_record
            .get::<&str, _>("Book_No")
            .unwrap_or_default()
            .to_string(),
        book_date: first_record.get::<NaiveDateTime, _>("Book_Date"),
        check_in: first_record.get::<NaiveDateTime, _>("Book_Date_in"),
        check_out: first_record.get::<NaiveDateTime, _>("Book_Date_out"),
        status: map_status(first_record.get::<i32, _>("Book_Status")),
        status_code: first_record.get::<i32, _>("Book_Status"),
        customer: BookingCustomerDetail {
            full_name: first_record
                .get::<&str, _>("Book_Cust_Name")
                .unwrap_or_default()
                .to_string(),
        },
        room_count: rooms.len(),
        rooms,
        total_amount: 0.0,
        notes,
    };

    Ok(Json(BookingDetailResponse {
        success: true,
        booking,
    }))
}

/// GET /api/bookings/:id/notes - Get booking notes (uses HotelNew DB)
pub async fn get_notes(
    State(state): State<AppState>,
    Path(book_no): Path<String>,
) -> ApiResult<Json<NotesResponse>> {
    // Use new_pool for notes (HotelNew database - writable)
    let mut conn = state.new_pool.get().await?;

    // Ensure notes table exists
    ensure_notes_table(&mut conn).await?;

    let rows = conn
        .query(
            r#"
            SELECT Note_ID, Note_Text, Created_At, Updated_At
            FROM HT_Booking_Notes
            WHERE Book_No = @P1
            ORDER BY Created_At DESC
            "#,
            &[&book_no],
        )
        .await?
        .into_first_result()
        .await?;

    let notes: Vec<Note> = rows
        .iter()
        .map(|row| Note {
            id: row.get::<i32, _>("Note_ID").unwrap_or(0),
            text: row
                .get::<&str, _>("Note_Text")
                .unwrap_or_default()
                .to_string(),
            created_at: row.get::<NaiveDateTime, _>("Created_At"),
            updated_at: row.get::<NaiveDateTime, _>("Updated_At"),
        })
        .collect();

    Ok(Json(NotesResponse {
        success: true,
        notes,
    }))
}

/// POST /api/bookings/:id/notes - Add booking note (uses HotelNew DB)
pub async fn create_note(
    State(state): State<AppState>,
    Path(book_no): Path<String>,
    Json(body): Json<CreateNoteRequest>,
) -> ApiResult<Json<CreateNoteResponse>> {
    let text = body.text.trim();
    if text.is_empty() {
        return Err(ApiError::BadRequest("Note text is required".to_string()));
    }

    // Use new_pool for notes (HotelNew database - writable)
    let mut conn = state.new_pool.get().await?;

    // Ensure notes table exists
    ensure_notes_table(&mut conn).await?;

    let rows = conn
        .query(
            r#"
            INSERT INTO HT_Booking_Notes (Book_No, Note_Text)
            OUTPUT INSERTED.Note_ID, INSERTED.Note_Text, INSERTED.Created_At, INSERTED.Updated_At
            VALUES (@P1, @P2)
            "#,
            &[&book_no, &text],
        )
        .await?
        .into_first_result()
        .await?;

    let row = rows
        .first()
        .ok_or_else(|| ApiError::Internal("Failed to create note".to_string()))?;

    let note = Note {
        id: row.get::<i32, _>("Note_ID").unwrap_or(0),
        text: row
            .get::<&str, _>("Note_Text")
            .unwrap_or_default()
            .to_string(),
        created_at: row.get::<NaiveDateTime, _>("Created_At"),
        updated_at: row.get::<NaiveDateTime, _>("Updated_At"),
    };

    Ok(Json(CreateNoteResponse {
        success: true,
        note,
    }))
}

/// Query parameters for delete note
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteNoteQuery {
    pub note_id: i32,
}

/// DELETE /api/bookings/:id/notes - Delete booking note (uses HotelNew DB)
pub async fn delete_note(
    State(state): State<AppState>,
    Path(book_no): Path<String>,
    Query(params): Query<DeleteNoteQuery>,
) -> ApiResult<Json<DeleteNoteResponse>> {
    // Use new_pool for notes (HotelNew database - writable)
    let mut conn = state.new_pool.get().await?;

    let result = conn
        .execute(
            r#"
            DELETE FROM HT_Booking_Notes
            WHERE Note_ID = @P1 AND Book_No = @P2
            "#,
            &[&params.note_id, &book_no],
        )
        .await?;

    if result.total() == 0 {
        return Err(ApiError::NotFound("Note not found".to_string()));
    }

    Ok(Json(DeleteNoteResponse {
        success: true,
        message: "Note deleted".to_string(),
    }))
}

/// Ensure the notes table exists (idempotent)
async fn ensure_notes_table(
    conn: &mut bb8::PooledConnection<'_, bb8_tiberius::ConnectionManager>,
) -> ApiResult<()> {
    conn.simple_query(
        r#"
        IF NOT EXISTS (SELECT * FROM sysobjects WHERE name='HT_Booking_Notes' AND xtype='U')
        CREATE TABLE HT_Booking_Notes (
            Note_ID INT IDENTITY(1,1) PRIMARY KEY,
            Book_No NVARCHAR(50) NOT NULL,
            Note_Text NVARCHAR(MAX) NOT NULL,
            Created_At DATETIME DEFAULT GETDATE(),
            Updated_At DATETIME DEFAULT GETDATE()
        )
        "#,
    )
    .await?
    .into_results()
    .await?;

    conn.simple_query(
        r#"
        IF NOT EXISTS (SELECT * FROM sys.indexes WHERE name='IX_Booking_Notes_BookNo')
        CREATE INDEX IX_Booking_Notes_BookNo ON HT_Booking_Notes(Book_No)
        "#,
    )
    .await?
    .into_results()
    .await?;

    Ok(())
}
