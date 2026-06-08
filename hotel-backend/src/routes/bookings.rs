//! Booking API routes
//!
//! - GET /api/bookings - List bookings (paginated). Reads canonical `ht_bookings` +
//!   `ht_customers` + `ht_booking_rooms` (junction) + `ht_rooms_new` / `ht_room_types`.
//! - GET /api/bookings/:id - Get booking details. Same canonical sources.
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
    get_canonical_status_from_thai, map_canonical_status_to_thai, Booking, BookingCustomer,
    BookingCustomerDetail, BookingDetail, BookingDetailResponse, BookingRoom, BookingRoomDetail,
    BookingsResponse, CreateNoteRequest, CreateNoteResponse, DeleteNoteResponse, Note,
    NotesResponse, Pagination,
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

/// List bookings from canonical `ht_bookings` + `ht_customers` + the
/// `ht_booking_rooms` junction.
///
/// Track J-style canonical rewrite (2026-05-16): the pre-rewrite path
/// read from `ht_bookings_legacy` (a cache mirror) which the v2.63.0
/// reconcile-mode demotion stopped populating with data columns — new
/// bookings landed in cache as ack-only rows (`sync_hash` + `synced_at`
/// only, all data columns NULL), so the dashboard rendered them as
/// empty rows. Canonical is the authoritative source and is populated
/// by the CT booking mapper for every event.
///
/// SQL-level pagination + sort + filter (the pre-rewrite path loaded
/// every matching row into memory and sorted in Rust). The aux query
/// `fetch_rooms_for_bookings` resolves the rooms array per book_id.
async fn list_bookings_pg(
    pool: &PgPool,
    params: &BookingsQuery,
) -> ApiResult<Json<BookingsResponse>> {
    // WHERE conditions — all parameterized to avoid injection.
    let mut conditions: Vec<String> = Vec::new();
    let mut bind_idx: usize = 0;
    let mut search_pattern: Option<String> = None;
    let mut canonical_status: Option<&'static str> = None;
    let mut start_date_val: Option<String> = None;
    let mut end_date_val: Option<String> = None;

    if let Some(ref search) = params.search {
        if !search.is_empty() {
            bind_idx += 1;
            let i = bind_idx;
            // Match book_no OR the constructed customer full name.
            // ILIKE is case-insensitive — matches the prior behavior of
            // hitting `book_cust_name` (Thai text where case folding is
            // a no-op but stays compatible with mixed-case Latin names).
            conditions.push(format!(
                "(b.book_no ILIKE ${i} OR \
                  COALESCE(c.cust_firstname, '') || ' ' || COALESCE(c.cust_lastname, '') ILIKE ${i})"
            ));
            search_pattern = Some(format!("%{}%", search));
        }
    }

    if let Some(ref status) = params.status {
        if let Some(canonical) = get_canonical_status_from_thai(status) {
            bind_idx += 1;
            conditions.push(format!("b.book_status = ${}", bind_idx));
            canonical_status = Some(canonical);
        }
    }

    if let Some(ref start_date) = params.start_date {
        if !start_date.is_empty() {
            bind_idx += 1;
            // Canonical `book_checkout` is a DATE column — direct
            // comparison without casting.
            conditions.push(format!("b.book_checkout >= ${}::date", bind_idx));
            start_date_val = Some(start_date.clone());
        }
    }

    if let Some(ref end_date) = params.end_date {
        if !end_date.is_empty() {
            bind_idx += 1;
            conditions.push(format!("b.book_checkin <= ${}::date", bind_idx));
            end_date_val = Some(end_date.clone());
        }
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // sort_by → fixed ORDER BY fragment. Whitelist style: arbitrary
    // sort_by input never reaches the SQL — only the keys this match
    // accepts can produce a column reference.
    let order_clause = match params.sort_by.as_str() {
        "bookNo" => "b.book_no",
        "status" => "b.book_status",
        "customer" => {
            "LOWER(COALESCE(c.cust_firstname, '') || ' ' || COALESCE(c.cust_lastname, ''))"
        }
        "checkIn" => "b.book_checkin",
        "checkOut" => "b.book_checkout",
        "roomCount" => "room_count",
        _ => "b.book_date", // default
    };
    let order_direction = if params.sort_order.to_lowercase() == "asc" { "ASC" } else { "DESC" };

    // Count — distinct book_ids (canonical has 1 row per book_no so
    // the DISTINCT is conservative — works whether we LEFT JOIN
    // junction rows in the future or not).
    let count_query = format!(
        "SELECT COUNT(*)::int AS total \
         FROM ht_bookings b \
         LEFT JOIN ht_customers c ON c.cust_id = b.book_cust_id \
         {where_clause}"
    );

    let mut count_q = sqlx::query(sqlx::AssertSqlSafe(&*count_query));
    if let Some(ref pat) = search_pattern {
        count_q = count_q.bind(pat);
    }
    if let Some(canon) = canonical_status {
        count_q = count_q.bind(canon);
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

    // Data — paginated at the SQL level. `room_count` subquery is a
    // correlated count against the junction; with LIMIT 20 the overhead
    // is ~20 trivial index lookups per page.
    let offset = ((params.page - 1) * params.limit).max(0);
    let data_query = format!(
        r#"
        SELECT
            b.book_id,
            b.book_no,
            b.book_date,
            b.book_checkin                                              AS book_checkin,
            b.book_checkout                                             AS book_checkout,
            b.book_status                                               AS book_status_canonical,
            NULLIF(TRIM(BOTH ' ' FROM
                COALESCE(c.cust_firstname, '') || ' ' || COALESCE(c.cust_lastname, '')
            ), '')                                                      AS cust_full_name,
            (SELECT COUNT(*) FROM ht_booking_rooms br WHERE br.br_book_id = b.book_id)::int
                                                                        AS room_count
        FROM ht_bookings b
        LEFT JOIN ht_customers c ON c.cust_id = b.book_cust_id
        {where_clause}
        ORDER BY {order_clause} {order_direction} NULLS LAST, b.book_id {order_direction}
        OFFSET {offset} LIMIT {limit}
        "#,
        limit = params.limit
    );

    let mut data_q = sqlx::query(sqlx::AssertSqlSafe(&*data_query));
    if let Some(ref pat) = search_pattern {
        data_q = data_q.bind(pat);
    }
    if let Some(canon) = canonical_status {
        data_q = data_q.bind(canon);
    }
    if let Some(ref sd) = start_date_val {
        data_q = data_q.bind(sd);
    }
    if let Some(ref ed) = end_date_val {
        data_q = data_q.bind(ed);
    }

    let rows = data_q.fetch_all(pool).await?;

    let mut bookings: Vec<Booking> = Vec::with_capacity(rows.len());
    let mut book_ids_for_rooms: Vec<i32> = Vec::with_capacity(rows.len());
    let mut book_no_by_id: HashMap<i32, String> = HashMap::with_capacity(rows.len());

    for row in &rows {
        let book_id: i32 = row.try_get("book_id").unwrap_or(0);
        let book_no: String = row.try_get("book_no").unwrap_or_default();
        let book_date: Option<NaiveDateTime> = row.try_get("book_date").ok();
        let check_in_date: Option<chrono::NaiveDate> = row.try_get("book_checkin").ok();
        let check_out_date: Option<chrono::NaiveDate> = row.try_get("book_checkout").ok();
        let status_canonical: Option<String> = row.try_get("book_status_canonical").ok();
        let cust_full_name: String = row.try_get("cust_full_name").unwrap_or_default();
        let room_count: i32 = row.try_get("room_count").unwrap_or(0);

        bookings.push(Booking {
            book_no: book_no.clone(),
            book_date: book_date.map(|dt| dt.and_utc()),
            check_in: check_in_date.map(|d| d.and_hms_opt(0, 0, 0).unwrap_or_default().and_utc()),
            check_out: check_out_date.map(|d| d.and_hms_opt(0, 0, 0).unwrap_or_default().and_utc()),
            customer: BookingCustomer { name: cust_full_name },
            status: map_canonical_status_to_thai(status_canonical.as_deref()),
            rooms: Vec::new(),
            room_count: room_count as usize,
        });
        book_ids_for_rooms.push(book_id);
        book_no_by_id.insert(book_id, book_no);
    }

    // Aux query — populate rooms[] for the page.
    if !book_ids_for_rooms.is_empty() {
        let mut rooms_by_book_no =
            fetch_rooms_for_bookings(pool, &book_ids_for_rooms, &book_no_by_id).await?;
        for b in bookings.iter_mut() {
            if let Some(rs) = rooms_by_book_no.remove(&b.book_no) {
                b.rooms = rs;
            }
        }
    }

    Ok(Json(BookingsResponse {
        success: true,
        data: bookings,
        pagination: Pagination::new(params.page, params.limit, total),
    }))
}

/// Aux query for [`list_bookings_pg`]: fetch the `ht_booking_rooms`
/// rows for a page's `book_id` set in one round-trip. Returns
/// `book_no → Vec<BookingRoom>` so the caller doesn't have to keep the
/// book_id mapping around. Bookings with no junction rows simply don't
/// appear in the map (caller leaves `rooms` empty).
async fn fetch_rooms_for_bookings(
    pool: &PgPool,
    book_ids: &[i32],
    book_no_by_id: &HashMap<i32, String>,
) -> ApiResult<HashMap<String, Vec<BookingRoom>>> {
    let rows = sqlx::query(
        r#"
        SELECT br.br_book_id, r.legacy_room_no AS room_no, COALESCE(rt.type_name, '-') AS room_type
          FROM ht_booking_rooms br
          JOIN ht_rooms_new r ON r.room_id = br.br_room_id
          LEFT JOIN ht_room_types rt ON rt.type_id = br.br_room_type_id
         WHERE br.br_book_id = ANY($1)
         ORDER BY br.br_book_id, r.legacy_room_no
        "#,
    )
    .bind(book_ids)
    .fetch_all(pool)
    .await?;

    let mut out: HashMap<String, Vec<BookingRoom>> = HashMap::new();
    for row in &rows {
        let book_id: i32 = row.try_get("br_book_id").unwrap_or(0);
        let room_no: String = row.try_get::<Option<String>, _>("room_no")
            .ok()
            .flatten()
            .unwrap_or_else(|| "-".to_string());
        let room_type: String = row.try_get("room_type").unwrap_or_else(|_| "-".to_string());
        if let Some(book_no) = book_no_by_id.get(&book_id) {
            out.entry(book_no.clone())
                .or_default()
                .push(BookingRoom { room_no, room_type });
        }
    }
    Ok(out)
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

/// Get booking details from canonical `ht_bookings` + JOINs.
/// Canonical rewrite — see `list_bookings_pg` for the rationale.
///
/// `status_code` in the response is a legacy-INT artefact kept for
/// frontend compatibility. We map canonical English back to the legacy
/// 1/2/3/4 the frontend may key off of, so existing UI logic keeps
/// working.
async fn get_booking_pg(
    pool: &crate::db::PgPool,
    book_no: &str,
) -> ApiResult<BookingDetail> {
    let header = sqlx::query(
        r#"
        SELECT
            b.book_id,
            b.book_no,
            b.book_date,
            b.book_checkin,
            b.book_checkout,
            b.book_status                                                AS book_status_canonical,
            COALESCE(b.book_total_amount, 0)::float8                    AS book_total_amount,
            NULLIF(TRIM(BOTH ' ' FROM
                COALESCE(c.cust_firstname, '') || ' ' || COALESCE(c.cust_lastname, '')
            ), '')                                                       AS cust_full_name
        FROM ht_bookings b
        LEFT JOIN ht_customers c ON c.cust_id = b.book_cust_id
        WHERE b.book_no = $1 OR b.legacy_book_id = $1
        LIMIT 1
        "#,
    )
    .bind(book_no)
    .fetch_optional(pool)
    .await?;

    let header = header.ok_or_else(|| ApiError::NotFound("Booking not found".to_string()))?;

    let book_id: i32 = header.try_get("book_id").unwrap_or(0);
    let book_no_canonical: String = header.try_get("book_no").unwrap_or_default();
    let book_date: Option<NaiveDateTime> = header.try_get("book_date").ok();
    let check_in_date: Option<chrono::NaiveDate> = header.try_get("book_checkin").ok();
    let check_out_date: Option<chrono::NaiveDate> = header.try_get("book_checkout").ok();
    let status_canonical: Option<String> = header.try_get("book_status_canonical").ok();
    let total_amount: f64 = header.try_get("book_total_amount").unwrap_or(0.0);
    let cust_full_name: String = header.try_get("cust_full_name").unwrap_or_default();

    let room_rows = sqlx::query(
        r#"
        SELECT r.legacy_room_no                          AS room_no,
               COALESCE(rt.type_name, '-')               AS room_type,
               COALESCE(br.br_price_per_night, 0)::float8 AS price_per_night
          FROM ht_booking_rooms br
          JOIN ht_rooms_new r ON r.room_id = br.br_room_id
          LEFT JOIN ht_room_types rt ON rt.type_id = br.br_room_type_id
         WHERE br.br_book_id = $1
         ORDER BY r.legacy_room_no
        "#,
    )
    .bind(book_id)
    .fetch_all(pool)
    .await?;

    let rooms: Vec<BookingRoomDetail> = room_rows
        .iter()
        .map(|r| BookingRoomDetail {
            room_no: r.try_get::<Option<String>, _>("room_no").ok().flatten()
                .unwrap_or_else(|| "-".to_string()),
            room_type: r.try_get::<String, _>("room_type").unwrap_or_else(|_| "-".to_string()),
            total: r.try_get::<f64, _>("price_per_night").unwrap_or(0.0),
        })
        .collect();

    // Frontend keys some logic off the legacy INT status code. Map
    // canonical English → legacy INT to preserve compatibility.
    let status_code: Option<i32> = match status_canonical.as_deref() {
        Some("confirmed") => Some(1),
        Some("checked_in") => Some(2),
        Some("completed") => Some(3),
        Some("cancelled") => Some(4),
        _ => None,
    };

    Ok(BookingDetail {
        book_no: book_no_canonical,
        book_date: book_date.map(|dt| dt.and_utc()),
        check_in: check_in_date.map(|d| d.and_hms_opt(0, 0, 0).unwrap_or_default().and_utc()),
        check_out: check_out_date.map(|d| d.and_hms_opt(0, 0, 0).unwrap_or_default().and_utc()),
        status: map_canonical_status_to_thai(status_canonical.as_deref()),
        status_code,
        customer: BookingCustomerDetail {
            full_name: cust_full_name,
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
