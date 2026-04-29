//! Booking API routes
//!
//! - GET /api/bookings - List bookings (paginated). Reads from PG (`ht_bookings_legacy` cache, fed by drift-reconcile + CT mappers).
//! - GET /api/bookings/:id - Get booking details. Reads from PG (`ht_bookings_legacy` cache, fed by drift-reconcile + CT mappers).
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
use sqlx::Row;

use crate::db::PgPool;
use crate::error::{ApiError, ApiResult};
use crate::models::{
    get_status_code, map_status, Booking, BookingCustomer, BookingCustomerDetail, BookingDetail,
    BookingDetailResponse, BookingRoom, BookingRoomDetail, BookingsResponse,
    CreateNoteRequest, CreateNoteResponse, DeleteNoteResponse, Note, NotesResponse, Pagination,
};
use crate::routes::mode::{AppState, Branch};

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
    pub branch: Option<Branch>,
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 20 }
fn default_sort_by() -> String { "bookDate".to_string() }
fn default_sort_order() -> String { "desc".to_string() }

/// GET /api/bookings - List bookings (paginated)
///
/// Reads from PG (`ht_bookings_legacy` cache, fed by drift-reconcile + CT mappers).
pub async fn list_bookings(
    State(state): State<AppState>,
    Query(params): Query<BookingsQuery>,
) -> ApiResult<Json<BookingsResponse>> {
    let branch = params.branch.unwrap_or_default();

    match branch {
        Branch::Hfhotel => list_bookings_pg(&state.new_pool, &params).await,
        Branch::Hfville => list_bookings_pg(state.ville_pool()?, &params).await,
        Branch::All => {
            // For "All", we merge results from both branches
            let hf_result = list_bookings_pg(&state.new_pool, &params).await?;
            // Try to get HF Ville data
            if let Ok(vp) = state.ville_pool() {
                if let Ok(ville_result) = list_bookings_pg(vp, &params).await {
                    let Json(mut hf) = hf_result;
                    let Json(ville) = ville_result;
                    hf.data.extend(ville.data);
                    hf.pagination.total += ville.pagination.total;
                    return Ok(Json(hf));
                }
            }
            Ok(hf_result)
        }
    }
}

/// List bookings from PostgreSQL `ht_bookings_legacy` table
async fn list_bookings_pg(
    pool: &PgPool,
    params: &BookingsQuery,
) -> ApiResult<Json<BookingsResponse>> {
    // Build WHERE conditions with parameterized queries
    let mut conditions: Vec<String> = Vec::new();
    let mut bind_idx: usize = 0;
    // We'll collect bind values and apply them later
    // Since we're using dynamic SQL, track parameter indices
    let mut search_pattern: Option<String> = None;
    let mut status_code: Option<i32> = None;
    let mut start_date_val: Option<String> = None;
    let mut end_date_val: Option<String> = None;

    if let Some(ref search) = params.search {
        if !search.is_empty() {
            bind_idx += 1;
            let search_idx = bind_idx;
            conditions.push(format!(
                "(book_no LIKE ${} OR book_cust_name LIKE ${})",
                search_idx, search_idx
            ));
            search_pattern = Some(format!("%{}%", search));
        }
    }

    if let Some(ref status) = params.status {
        if let Some(code) = get_status_code(status) {
            bind_idx += 1;
            conditions.push(format!("book_status = ${}", bind_idx));
            status_code = Some(code);
        }
    }

    if let Some(ref start_date) = params.start_date {
        if !start_date.is_empty() {
            bind_idx += 1;
            conditions.push(format!("book_date_out::date >= ${}", bind_idx));
            start_date_val = Some(start_date.clone());
        }
    }

    if let Some(ref end_date) = params.end_date {
        if !end_date.is_empty() {
            bind_idx += 1;
            conditions.push(format!("book_date_in::date <= ${}", bind_idx));
            end_date_val = Some(end_date.clone());
        }
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count distinct bookings
    let count_query = format!(
        "SELECT COUNT(DISTINCT book_no)::int as total FROM ht_bookings_legacy {}",
        where_clause
    );

    // Build count query with binds
    let mut count_q = sqlx::query(&count_query);
    if let Some(ref pat) = search_pattern {
        count_q = count_q.bind(pat);
    }
    if let Some(code) = status_code {
        count_q = count_q.bind(code);
    }
    if let Some(ref sd) = start_date_val {
        count_q = count_q.bind(sd);
    }
    if let Some(ref ed) = end_date_val {
        count_q = count_q.bind(ed);
    }

    let count_rows = count_q.fetch_all(pool).await?;
    let total: i32 = count_rows
        .first()
        .map(|r| r.try_get::<i32, _>("total").unwrap_or(0))
        .unwrap_or(0);

    // Get all data (group and paginate in Rust like the Node.js version)
    let data_query = format!(
        r#"
        SELECT
            book_no,
            book_date,
            book_date_in,
            book_date_out,
            book_cust_name,
            book_status,
            book_room_type
        FROM ht_bookings_legacy
        {}
        ORDER BY book_date DESC
        "#,
        where_clause
    );

    let mut data_q = sqlx::query(&data_query);
    if let Some(ref pat) = search_pattern {
        data_q = data_q.bind(pat);
    }
    if let Some(code) = status_code {
        data_q = data_q.bind(code);
    }
    if let Some(ref sd) = start_date_val {
        data_q = data_q.bind(sd);
    }
    if let Some(ref ed) = end_date_val {
        data_q = data_q.bind(ed);
    }

    let rows = data_q.fetch_all(pool).await?;

    // Group records by book_no
    let mut grouped: HashMap<String, Booking> = HashMap::new();

    for row in &rows {
        let book_no: String = row.try_get("book_no").unwrap_or_default();

        let entry = grouped.entry(book_no.clone()).or_insert_with(|| {
            let book_date: Option<NaiveDateTime> = row.try_get("book_date").ok();
            let check_in: Option<NaiveDateTime> = row.try_get("book_date_in").ok();
            let check_out: Option<NaiveDateTime> = row.try_get("book_date_out").ok();
            let cust_name: String = row.try_get("book_cust_name").unwrap_or_default();
            let status: Option<i32> = row.try_get("book_status").ok();

            Booking {
                book_no: book_no.clone(),
                book_date: book_date.map(|dt| dt.and_utc()),
                check_in: check_in.map(|dt| dt.and_utc()),
                check_out: check_out.map(|dt| dt.and_utc()),
                customer: BookingCustomer {
                    name: cust_name,
                },
                status: map_status(status),
                rooms: Vec::new(),
                room_count: 0,
            }
        });

        let room_type: String = row
            .try_get("book_room_type")
            .unwrap_or_else(|_| "-".to_string());

        entry.rooms.push(BookingRoom {
            room_no: "-".to_string(),
            room_type,
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

/// GET /api/bookings/:id - Get booking details
///
/// Reads from PG (`ht_bookings_legacy` cache, fed by drift-reconcile + CT mappers).
/// Notes are fetched from HotelNew PostgreSQL.
pub async fn get_booking(
    State(state): State<AppState>,
    Path(book_no): Path<String>,
) -> ApiResult<Json<BookingDetailResponse>> {
    let new_pool = &state.new_pool;

    // Ensure notes table exists
    ensure_notes_table(new_pool).await?;

    // Fetch notes from HotelNew (always PG)
    let notes = match sqlx::query!(
            r#"
            SELECT note_id, note_text, created_at, updated_at
            FROM ht_booking_notes
            WHERE book_no = $1
            ORDER BY created_at DESC
            "#,
            book_no
        )
        .fetch_all(new_pool)
        .await
    {
        Ok(note_rows) => note_rows
            .iter()
            .map(|r| Note {
                id: r.note_id,
                text: r.note_text.clone(),
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect(),
        Err(_) => Vec::new(),
    };

    let mut booking = get_booking_pg(new_pool, &book_no).await?;

    // Attach notes
    booking.notes = notes;

    Ok(Json(BookingDetailResponse {
        success: true,
        booking,
    }))
}

/// Get booking details from PostgreSQL mirror tables
async fn get_booking_pg(
    pool: &crate::db::PgPool,
    book_no: &str,
) -> ApiResult<BookingDetail> {
    let rows = sqlx::query(
        r#"
        SELECT book_no, book_date, book_date_in, book_date_out,
               book_cust_name, book_status, book_room_type,
               COALESCE(book_total, 0)::float8 AS book_total
        FROM ht_bookings_legacy
        WHERE book_no = $1
        ORDER BY book_room_type
        "#,
    )
    .bind(book_no)
    .fetch_all(pool)
    .await?;

    if rows.is_empty() {
        return Err(ApiError::NotFound("Booking not found".to_string()));
    }

    let first = &rows[0];

    let rooms: Vec<BookingRoomDetail> = rows
        .iter()
        .map(|r| BookingRoomDetail {
            room_no: "-".to_string(),
            room_type: r.try_get::<String, _>("book_room_type").unwrap_or_else(|_| "-".to_string()),
            total: r.try_get::<f64, _>("book_total").unwrap_or(0.0),
        })
        .collect();

    let status_code = first.try_get::<i32, _>("book_status").ok();
    let total_amount: f64 = rooms.iter().map(|r| r.total).sum();

    Ok(BookingDetail {
        book_no: first.try_get::<String, _>("book_no").unwrap_or_default(),
        book_date: first.try_get::<NaiveDateTime, _>("book_date").ok().map(|dt| dt.and_utc()),
        check_in: first.try_get::<NaiveDateTime, _>("book_date_in").ok().map(|dt| dt.and_utc()),
        check_out: first.try_get::<NaiveDateTime, _>("book_date_out").ok().map(|dt| dt.and_utc()),
        status: map_status(status_code),
        status_code,
        customer: BookingCustomerDetail {
            full_name: first.try_get::<String, _>("book_cust_name").unwrap_or_default(),
        },
        room_count: rooms.len(),
        rooms,
        total_amount,
        notes: Vec::new(),
    })
}

/// GET /api/bookings/:id/notes - Get booking notes (uses HotelNew DB)
pub async fn get_notes(
    State(state): State<AppState>,
    Path(book_no): Path<String>,
) -> ApiResult<Json<NotesResponse>> {
    // Use new_pool for notes (HotelNew database - writable via sqlx/PostgreSQL)
    let pool = &state.new_pool;

    // Ensure notes table exists
    ensure_notes_table(pool).await?;

    let rows = sqlx::query!(
            r#"
            SELECT note_id, note_text, created_at, updated_at
            FROM ht_booking_notes
            WHERE book_no = $1
            ORDER BY created_at DESC
            "#,
            book_no
        )
        .fetch_all(pool)
        .await?;

    let notes: Vec<Note> = rows
        .iter()
        .map(|r| Note {
            id: r.note_id,
            text: r.note_text.clone(),
            created_at: r.created_at,
            updated_at: r.updated_at,
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

    // Use new_pool for notes (HotelNew database - writable via sqlx/PostgreSQL)
    let pool = &state.new_pool;

    // Ensure notes table exists
    ensure_notes_table(pool).await?;

    let rec = sqlx::query!(
            r#"
            INSERT INTO ht_booking_notes (book_no, note_text)
            VALUES ($1, $2)
            RETURNING note_id, note_text, created_at, updated_at
            "#,
            book_no,
            text
        )
        .fetch_one(pool)
        .await
        .map_err(|_| ApiError::Internal("Failed to create note".to_string()))?;

    let note = Note {
        id: rec.note_id,
        text: rec.note_text,
        created_at: rec.created_at,
        updated_at: rec.updated_at,
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
    // Use new_pool for notes (HotelNew database - writable via sqlx/PostgreSQL)
    let pool = &state.new_pool;

    let result = sqlx::query!(
            r#"
            DELETE FROM ht_booking_notes
            WHERE note_id = $1 AND book_no = $2
            "#,
            params.note_id,
            book_no
        )
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Note not found".to_string()));
    }

    Ok(Json(DeleteNoteResponse {
        success: true,
        message: "Note deleted".to_string(),
    }))
}

/// Ensure the notes table exists (idempotent)
async fn ensure_notes_table(pool: &crate::db::PgPool) -> ApiResult<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS ht_booking_notes (
            note_id SERIAL PRIMARY KEY,
            book_no VARCHAR(50) NOT NULL,
            note_text TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT NOW(),
            updated_at TIMESTAMP DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS ix_booking_notes_bookno ON ht_booking_notes(book_no)"
    )
    .execute(pool)
    .await?;

    Ok(())
}
