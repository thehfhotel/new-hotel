//! Room API routes
//!
//! - GET /api/rooms - List all rooms
//! - GET /api/rooms/:id - Get room details with current guest
//! - GET /api/rooms/status - Get room status history (calendar)
//! - GET /api/rooms/checkouts-today - Get rooms with checkout today
//!
//! `list_rooms`, `get_room`, and `get_checkouts_today` read from the
//! canonical PG tables (`ht_rooms_new`, `ht_checkins`, `ht_bookings`,
//! `ht_customers`). Previously read `ht_*_legacy` mirrors which stopped
//! receiving row-level updates after the Phase 5.5 cutover on 2026-04-28.
//!
//! `get_room_status_pg` (calendar) is intentionally left on the legacy
//! mirror tables for now — separate follow-up to migrate that route.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::Row;

use crate::error::{ApiError, ApiResult};
use crate::models::{
    CheckoutsTodayResponse, CurrentGuest, Room, RoomDetail, RoomDetailResponse, RoomStatus,
    RoomStatusResponse, RoomsResponse,
};
use crate::routes::mode::{AppState, Branch};

// ─────────────────────────────────────────────────────────────
// Canonical-table SQL fragments (DRY: same projection in list & detail)
// ─────────────────────────────────────────────────────────────

/// SQL projection mapping `ht_rooms_new` → the legacy-shaped `Room`/`RoomDetail`
/// API contract. Centralised so list and detail queries stay in sync.
///
/// Bool-to-yes/no string mapping preserves the frontend contract at
/// `app/page.tsx` (`getRoomStatus` reads `Room_Use === 'yes'` etc.).
///
/// `Room_Use` is true when an active, not-yet-checked-out checkin exists.
/// `Room_Book` returns the booking number of a currently active reservation
/// (today between book_checkin and book_checkout, confirmed/pending) when
/// the room is not currently in use — matching the legacy mirror semantics.
const ROOM_PROJECTION: &str = r#"
    r.room_no AS room_no,
    COALESCE(rt.type_name, '') AS room_type,
    COALESCE(r.room_notes, '') AS room_details,
    CASE WHEN r.room_clean THEN 'yes' ELSE 'no' END AS room_clean,
    CASE WHEN EXISTS (
        SELECT 1 FROM ht_checkins c
        WHERE c.cin_room_id = r.room_id
          AND c.cin_status = 'active'
          AND c.cin_checkout_time IS NULL
    ) THEN 'yes' ELSE 'no' END AS room_use,
    COALESCE((
        SELECT b.book_no::text
        FROM ht_booking_rooms br
        JOIN ht_bookings b ON b.book_id = br.br_book_id
        WHERE br.br_room_id = r.room_id
          AND b.book_status IN ('confirmed', 'pending')
          AND b.book_checkin <= CURRENT_DATE
          AND b.book_checkout > CURRENT_DATE
          AND NOT EXISTS (
              SELECT 1 FROM ht_checkins c2
              WHERE c2.cin_room_id = r.room_id
                AND c2.cin_status = 'active'
                AND c2.cin_checkout_time IS NULL
          )
        LIMIT 1
    ), '') AS room_book,
    CASE WHEN r.room_maintenance THEN 'yes' ELSE 'no' END AS room_manternace,
    r.room_price_weekday::float8 AS room_price_a,
    r.room_price_weekend::float8 AS room_price_b,
    r.room_price_special::float8 AS room_price_c
"#;

// ─────────────────────────────────────────────────────────────
// PostgreSQL helpers — canonical tables
// ─────────────────────────────────────────────────────────────

/// Map a result row from `ROOM_PROJECTION` to the API `Room` struct.
fn row_to_room(row: &sqlx::postgres::PgRow) -> Room {
    Room {
        room_no: row.get::<String, _>("room_no"),
        room_type: Some(row.get::<String, _>("room_type")),
        room_details: Some(row.get::<String, _>("room_details")),
        room_clean: Some(row.get::<String, _>("room_clean")),
        room_use: Some(row.get::<String, _>("room_use")),
        room_book: Some(row.get::<String, _>("room_book")),
        room_manternace: Some(row.get::<String, _>("room_manternace")),
        room_price_a: row.try_get::<f64, _>("room_price_a").ok(),
        room_price_b: row.try_get::<f64, _>("room_price_b").ok(),
        room_price_c: row.try_get::<f64, _>("room_price_c").ok(),
        // `room_group` and `room_book_name` are legacy-only metadata not
        // tracked in the canonical schema; surface as None for now.
        room_group: None,
        room_book_name: None,
    }
}

/// List all rooms from canonical PG tables.
async fn list_rooms_pg(pool: &crate::db::PgPool) -> ApiResult<Vec<Room>> {
    let sql = format!(
        r#"
        SELECT {projection}
        FROM ht_rooms_new r
        LEFT JOIN ht_room_types rt ON rt.type_id = r.room_type_id
        WHERE r.room_active = true
        ORDER BY r.room_no
        "#,
        projection = ROOM_PROJECTION,
    );

    let rows = sqlx::query(&sql).fetch_all(pool).await?;
    Ok(rows.iter().map(row_to_room).collect())
}

/// HF Ville variant. Schema is now identical post-Ville-upgrade
/// (see CLAUDE memory: ville_constraint upgraded 2026-04-29), so it
/// delegates to `list_rooms_pg`. Kept as a named function to preserve
/// the call sites and signal intent at the dispatch layer.
async fn list_rooms_legacy_only(pool: &crate::db::PgPool) -> ApiResult<Vec<Room>> {
    list_rooms_pg(pool).await
}

/// Get a single room detail from canonical PG tables with current guest info.
async fn get_room_pg(pool: &crate::db::PgPool, room_no: &str) -> ApiResult<RoomDetail> {
    let sql = format!(
        r#"
        SELECT
            {projection},
            r.room_id AS room_id
        FROM ht_rooms_new r
        LEFT JOIN ht_room_types rt ON rt.type_id = r.room_type_id
        WHERE r.room_no = $1 AND r.room_active = true
        "#,
        projection = ROOM_PROJECTION,
    );

    let room_row = sqlx::query(&sql)
        .bind(room_no)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| ApiError::NotFound("Room not found".to_string()))?;

    let room_id: i32 = room_row.get("room_id");

    // Most recent checkin for this room (any status). The
    // current-guest section in the UI shows the latest stay even
    // after checkout, matching the prior legacy-mirror behaviour.
    let checkin_row = sqlx::query(
        r#"
        SELECT
            TRIM(COALESCE(cu.cust_firstname, '') || ' ' || COALESCE(cu.cust_lastname, '')) AS cust_name,
            c.cin_checkin_time,
            c.cin_checkout_time
        FROM ht_checkins c
        LEFT JOIN ht_customers cu ON cu.cust_id = c.cin_cust_id
        WHERE c.cin_room_id = $1
        ORDER BY c.cin_checkin_time DESC
        LIMIT 1
        "#,
    )
    .bind(room_id)
    .fetch_optional(pool)
    .await?;

    let current_guest = checkin_row.map(|row| CurrentGuest {
        name: row.try_get::<String, _>("cust_name").ok(),
        check_in: row
            .try_get::<NaiveDateTime, _>("cin_checkin_time")
            .ok()
            .map(|dt| dt.and_utc()),
        check_out: row
            .try_get::<NaiveDateTime, _>("cin_checkout_time")
            .ok()
            .map(|dt| dt.and_utc()),
    });

    let room = RoomDetail {
        room_no: room_row.get::<String, _>("room_no"),
        room_type: Some(room_row.get::<String, _>("room_type")),
        room_details: Some(room_row.get::<String, _>("room_details")),
        room_clean: Some(room_row.get::<String, _>("room_clean")),
        room_use: Some(room_row.get::<String, _>("room_use")),
        room_book: Some(room_row.get::<String, _>("room_book")),
        room_manternace: Some(room_row.get::<String, _>("room_manternace")),
        room_price_a: room_row.try_get::<f64, _>("room_price_a").ok(),
        room_price_b: room_row.try_get::<f64, _>("room_price_b").ok(),
        room_price_c: room_row.try_get::<f64, _>("room_price_c").ok(),
        room_group: None,
        room_book_name: None,
        // `Room_Book_Time` isn't tracked on the canonical booking row
        // (booking has a date range, not a single "book at" instant).
        // Leave None for now; future enhancement could surface
        // `ht_bookings.created_at` if/when that column lands.
        room_book_time: None,
        current_guest,
    };

    Ok(room)
}

/// HF Ville variant — see `list_rooms_legacy_only` rationale.
async fn get_room_legacy_only(pool: &crate::db::PgPool, room_no: &str) -> ApiResult<RoomDetail> {
    get_room_pg(pool, room_no).await
}

/// Get rooms checking out today from canonical PG tables.
///
/// Same flip rule as `stats::get_stats_pg`: only rooms with an active
/// checkin whose `cin_expected_checkout = CURRENT_DATE`, surfaced after
/// 06:00 local so the morning crew can process departures.
async fn get_checkouts_today_pg(pool: &crate::db::PgPool) -> ApiResult<Vec<String>> {
    let rows = sqlx::query(
        r#"
        SELECT DISTINCT r.room_no AS room_no
        FROM ht_rooms_new r
        JOIN ht_checkins c ON c.cin_room_id = r.room_id
        WHERE c.cin_status = 'active'
          AND c.cin_expected_checkout = CURRENT_DATE
          AND c.cin_checkout_time IS NULL
          AND EXTRACT(HOUR FROM NOW()) >= 6
        ORDER BY r.room_no
        "#,
    )
    .fetch_all(pool)
    .await?;

    let room_numbers: Vec<String> = rows
        .iter()
        .map(|row| row.get::<String, _>("room_no"))
        .collect();

    Ok(room_numbers)
}

/// Get room status from PostgreSQL (PG mirror tables).
///
/// NOTE: This route still reads the legacy mirror tables. Tracked as
/// a separate follow-up — migrating the calendar query requires
/// careful date-range handling and isn't in scope for the dashboard fix.
async fn get_room_status_pg(
    pool: &crate::db::PgPool,
    params: &RoomStatusQuery,
) -> ApiResult<Vec<RoomStatus>> {
    // Default date range: today to +30 days if not specified
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let thirty_days = (chrono::Local::now() + chrono::Duration::days(30)).format("%Y-%m-%d").to_string();

    let start = params.start_date.as_deref().unwrap_or(&today);
    let end = params.end_date.as_deref().unwrap_or(&thirty_days);

    let rows = sqlx::query(
        r#"
        SELECT
            r.room_no,
            d.dt::timestamp AS room_date,
            CASE
                WHEN ci.cin_no IS NOT NULL THEN 'เข้าพัก'
                WHEN bl.book_no IS NOT NULL THEN 'จอง'
                ELSE 'ว่าง'
            END AS room_status,
            COALESCE(r.room_details, '') AS room_details,
            ci.cin_checkin_no AS room_checkin_no,
            r.room_type
        FROM ht_rooms_legacy r
        CROSS JOIN generate_series($1::date, $2::date, '1 day'::interval) AS d(dt)
        LEFT JOIN ht_checkins_legacy ci
            ON ci.cin_room_no = r.room_no
            AND d.dt::date >= ci.cin_room_in::date
            AND d.dt::date < ci.cin_room_out::date
        LEFT JOIN ht_bookings_legacy bl
            ON bl.book_room_no = r.room_no
            AND d.dt::date >= bl.book_date_in::date
            AND d.dt::date < bl.book_date_out::date
            AND bl.book_status = 1
            AND ci.cin_no IS NULL
        ORDER BY r.room_no, d.dt
        "#,
    )
    .bind(start)
    .bind(end)
    .fetch_all(pool)
    .await?;

    let statuses: Vec<RoomStatus> = rows
        .iter()
        .map(|row| RoomStatus {
            room_no: row.try_get::<String, _>("room_no").unwrap_or_default(),
            room_date: row.try_get::<NaiveDateTime, _>("room_date").ok().map(|dt| dt.and_utc()),
            room_status: row.try_get::<String, _>("room_status").ok(),
            room_details: row.try_get::<String, _>("room_details").ok(),
            room_checkin_no: row.try_get::<String, _>("room_checkin_no").ok(),
            room_type: row.try_get::<String, _>("room_type").ok(),
        })
        .collect();

    Ok(statuses)
}

// ─────────────────────────────────────────────────────────────
// Public route handlers
// ─────────────────────────────────────────────────────────────

/// Query parameters for rooms list (branch support)
#[derive(Debug, Deserialize)]
pub struct RoomsQuery {
    pub branch: Option<Branch>,
}

/// GET /api/rooms - List all rooms
///
/// Reads from canonical PG tables (`ht_rooms_new` + `ht_checkins` + `ht_bookings`).
pub async fn list_rooms(
    State(state): State<AppState>,
    Query(params): Query<RoomsQuery>,
) -> ApiResult<Json<RoomsResponse>> {
    let branch = params.branch.unwrap_or_default();

    let rooms = match branch {
        Branch::Hfhotel => list_rooms_pg(&state.new_pool).await?,
        Branch::Hfville => list_rooms_legacy_only(state.ville_pool()?).await?,
        Branch::All => {
            let mut all = list_rooms_pg(&state.new_pool).await?;
            if let Ok(vp) = state.ville_pool() {
                all.extend(list_rooms_legacy_only(vp).await?);
            }
            all
        }
    };

    let total = rooms.len();

    Ok(Json(RoomsResponse {
        success: true,
        data: rooms,
        total,
    }))
}

/// Query parameters for get room (branch support)
#[derive(Debug, Deserialize)]
pub struct GetRoomQuery {
    pub branch: Option<Branch>,
}

/// GET /api/rooms/:id - Get room details with current guest
///
/// Reads from canonical PG tables (`ht_rooms_new` + `ht_checkins` + `ht_customers`).
pub async fn get_room(
    State(state): State<AppState>,
    Path(room_no): Path<String>,
    Query(params): Query<GetRoomQuery>,
) -> ApiResult<Json<RoomDetailResponse>> {
    let branch = params.branch.unwrap_or_default();

    let room = match branch {
        Branch::Hfhotel | Branch::All => get_room_pg(&state.new_pool, &room_no).await?,
        Branch::Hfville => get_room_legacy_only(state.ville_pool()?, &room_no).await?,
    };

    Ok(Json(RoomDetailResponse {
        success: true,
        room,
    }))
}

/// Query parameters for room status
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomStatusQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub branch: Option<Branch>,
}

/// GET /api/rooms/status - Get room status history
///
/// NOTE: Still reads legacy mirror tables — separate follow-up to migrate.
pub async fn get_room_status(
    State(state): State<AppState>,
    Query(params): Query<RoomStatusQuery>,
) -> ApiResult<Json<RoomStatusResponse>> {
    let branch = params.branch.unwrap_or_default();

    let statuses = match branch {
        Branch::Hfhotel => get_room_status_pg(&state.new_pool, &params).await?,
        Branch::Hfville => get_room_status_pg(state.ville_pool()?, &params).await?,
        Branch::All => {
            let mut all = get_room_status_pg(&state.new_pool, &params).await?;
            if let Ok(vp) = state.ville_pool() {
                all.extend(get_room_status_pg(vp, &params).await?);
            }
            all
        }
    };
    let total = statuses.len();

    Ok(Json(RoomStatusResponse {
        success: true,
        data: statuses,
        total,
    }))
}

/// Query parameters for checkouts-today (branch support)
#[derive(Debug, Deserialize)]
pub struct CheckoutsTodayQuery {
    pub branch: Option<Branch>,
}

/// GET /api/rooms/checkouts-today - Get rooms with checkout today
///
/// Reads from canonical PG tables (`ht_rooms_new` + `ht_checkins`).
pub async fn get_checkouts_today(
    State(state): State<AppState>,
    Query(params): Query<CheckoutsTodayQuery>,
) -> ApiResult<Json<CheckoutsTodayResponse>> {
    let branch = params.branch.unwrap_or_default();

    let room_numbers = match branch {
        Branch::Hfhotel => get_checkouts_today_pg(&state.new_pool).await?,
        Branch::Hfville => get_checkouts_today_pg(state.ville_pool()?).await?,
        Branch::All => {
            let mut all = get_checkouts_today_pg(&state.new_pool).await?;
            if let Ok(vp) = state.ville_pool() {
                all.extend(get_checkouts_today_pg(vp).await?);
            }
            all
        }
    };

    Ok(Json(CheckoutsTodayResponse {
        success: true,
        data: room_numbers,
    }))
}
