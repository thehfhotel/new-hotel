//! Room API routes
//!
//! - GET /api/rooms - List all rooms
//! - GET /api/rooms/:id - Get room details with current guest
//! - GET /api/rooms/status - Get room status history (calendar)
//! - GET /api/rooms/checkouts-today - Get rooms with checkout today
//!
//! All read paths use the canonical PG tables (`ht_rooms_new`,
//! `ht_checkins`, `ht_bookings`, `ht_booking_rooms`, `ht_room_types`,
//! `ht_customers`). Previously read `ht_*_legacy` mirrors which stopped
//! receiving row-level updates after the Phase 5.5 cutover on 2026-04-28.
//! The calendar route (`get_room_status_pg`) was the final holdout and
//! was migrated alongside the v2.63.0 dashboard fix family.

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
///
/// ## Track B3 / T3 CRIT-2 — multi-room junction reads
///
/// Pre-B3 the "room is in use?" predicate only joined on
/// `ht_checkins.cin_room_id`, which collapsed every multi-room folio to
/// its first room (Theme 1 of `audit-2026-05-13.md`). B3 adds an OR-clause
/// against the canonical `ht_checkin_rooms` junction (one row per
/// `HT_CheckIn_Ds` row, populated by B2's mapper) so secondary rooms 2..N
/// of a multi-room folio also flip to `room_use = 'yes'`.
///
/// The legacy `cin_room_id` branch is kept additive: per CARDINALITY_MAP
/// the column is DEPRECATED but stays in place until B5 backfills every
/// historical folio into the junction. Until then it remains the only
/// signal for any pre-B2 row that hasn't been re-synced through the new
/// mapper.
const ROOM_PROJECTION: &str = r#"
    r.room_no AS room_no,
    COALESCE(rt.type_name, '') AS room_type,
    COALESCE(r.room_notes, '') AS room_details,
    CASE WHEN r.room_clean THEN 'yes' ELSE 'no' END AS room_clean,
    CASE WHEN EXISTS (
        SELECT 1 FROM ht_checkins c
        WHERE c.cin_status = 'active'
          AND c.cin_checkout_time IS NULL
          AND (
              c.cin_room_id = r.room_id
              OR EXISTS (
                  SELECT 1 FROM ht_checkin_rooms cr
                  WHERE cr.cr_cin_id = c.cin_id
                    AND cr.cr_room_id = r.room_id
              )
          )
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
              WHERE c2.cin_status = 'active'
                AND c2.cin_checkout_time IS NULL
                AND (
                    c2.cin_room_id = r.room_id
                    OR EXISTS (
                        SELECT 1 FROM ht_checkin_rooms cr2
                        WHERE cr2.cr_cin_id = c2.cin_id
                          AND cr2.cr_room_id = r.room_id
                    )
                )
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
/// checkin whose expected checkout is "today in Bangkok", surfaced after
/// 06:00 BKK so the morning crew can process departures. Coexistence
/// audit T3 MED-1 + MED-2: the prior `CURRENT_DATE` / `EXTRACT(HOUR FROM
/// NOW())` shape evaluated in UTC, which delayed the morning flip until
/// 13:00 BKK.
///
/// ## Track B3 / T3 CRIT-2 — multi-room junction reads
///
/// Walks through `ht_checkin_rooms` so a 2-room folio surfaces BOTH room
/// numbers (pre-B3 the `JOIN ON cin_room_id` shape collapsed to the first
/// room only). The `LEFT JOIN` on the junction with `COALESCE` falls back
/// to the legacy `cin_room_id` for any pre-B2 row that hasn't been
/// re-synced through the new mapper yet — see CARDINALITY_MAP's
/// `ht_checkins.cin_room_id` DEPRECATED note. After B5's backfill the
/// fallback becomes dead code and the column drops.
async fn get_checkouts_today_pg(pool: &crate::db::PgPool) -> ApiResult<Vec<String>> {
    let rows = sqlx::query(&format!(
        r#"
        SELECT DISTINCT r.room_no AS room_no
        FROM ht_checkins c
        LEFT JOIN ht_checkin_rooms cr ON cr.cr_cin_id = c.cin_id
        JOIN ht_rooms_new r ON r.room_id = COALESCE(cr.cr_room_id, c.cin_room_id)
        WHERE c.cin_status = 'active'
          AND c.cin_expected_checkout = {today}
          AND c.cin_checkout_time IS NULL
          AND {hour} >= 6
        ORDER BY r.room_no
        "#,
        today = crate::routes::stats::BANGKOK_TODAY_SQL,
        hour = crate::routes::stats::BANGKOK_HOUR_SQL,
    ))
    .fetch_all(pool)
    .await?;

    let room_numbers: Vec<String> = rows
        .iter()
        .map(|row| row.get::<String, _>("room_no"))
        .collect();

    Ok(room_numbers)
}

/// Get room status calendar from canonical PG tables.
///
/// Generates a per-room-per-date matrix between `start_date` and `end_date`
/// (inclusive) and reports one of `'เข้าพัก'` (occupied checkin),
/// `'จอง'` (active booking, no overriding checkin), or `'ว่าง'` (vacant).
///
/// Sources:
/// - `ht_rooms_new` filtered to `room_active = true` — active inventory only
/// - `ht_room_types` for the displayed `room_type` (`type_name`)
/// - `ht_checkins` joined through the `ht_checkin_rooms` junction (Track
///   B3 / T3 CRIT-2). `COALESCE(cr.cr_room_id, ci.cin_room_id)` resolves
///   the room identity: when the junction has any rows for the folio,
///   they are authoritative (one row per actual room); otherwise the
///   header-level `cin_room_id` is the fallback for pre-B2 folios that
///   haven't been re-synced through the new mapper. Occupancy window
///   is `[cin_checkin_time::date, COALESCE(cin_checkout_time::date,
///   cin_expected_checkout))`. Restricted to non-cancelled rows
///   (`cin_status IN ('active','checkedout')`).
/// - `ht_bookings` joined via `ht_booking_rooms` on `br_room_id`. Reservation
///   window is `[book_checkin, book_checkout)`. Restricted to
///   `book_status IN ('confirmed','pending','checkedin')`. Checkin takes
///   precedence (`ci.cin_id IS NULL` guard on the booking LEFT JOIN).
///
/// Output shape (`room_no`, `room_date`, `room_status`, `room_details`,
/// `room_checkin_no`, `room_type`) preserved verbatim for the calendar UI.
/// The legacy `cin_checkin_no` column maps to canonical `cin_no`.
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
        WITH checkin_rooms AS (
            -- One row per (folio, room) pair. The junction takes
            -- precedence when present; falls back to header-level
            -- `cin_room_id` for pre-B2 folios. After B5 backfill the
            -- LEFT JOIN always yields a non-null cr_room_id and the
            -- COALESCE branch becomes dead.
            SELECT
                ci.cin_id,
                ci.cin_no,
                ci.cin_status,
                ci.cin_checkin_time,
                ci.cin_checkout_time,
                ci.cin_expected_checkout,
                COALESCE(cr.cr_room_id, ci.cin_room_id) AS room_id
            FROM ht_checkins ci
            LEFT JOIN ht_checkin_rooms cr ON cr.cr_cin_id = ci.cin_id
            WHERE ci.cin_status IN ('active', 'checkedout')
        )
        SELECT
            r.room_no,
            d.dt::timestamp AS room_date,
            CASE
                WHEN ci.cin_no IS NOT NULL THEN 'เข้าพัก'
                WHEN b.book_no IS NOT NULL THEN 'จอง'
                ELSE 'ว่าง'
            END AS room_status,
            COALESCE(r.room_notes, '') AS room_details,
            ci.cin_no AS room_checkin_no,
            COALESCE(rt.type_name, '') AS room_type
        FROM ht_rooms_new r
        LEFT JOIN ht_room_types rt ON rt.type_id = r.room_type_id
        CROSS JOIN generate_series($1::date, $2::date, '1 day'::interval) AS d(dt)
        LEFT JOIN checkin_rooms ci
            ON ci.room_id = r.room_id
            AND d.dt::date >= ci.cin_checkin_time::date
            AND d.dt::date < COALESCE(ci.cin_checkout_time::date, ci.cin_expected_checkout)
        LEFT JOIN ht_booking_rooms br
            ON br.br_room_id = r.room_id
            AND ci.cin_id IS NULL
        LEFT JOIN ht_bookings b
            ON b.book_id = br.br_book_id
            AND b.book_status IN ('confirmed', 'pending', 'checkedin')
            AND d.dt::date >= b.book_checkin
            AND d.dt::date < b.book_checkout
        WHERE r.room_active = true
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

/// GET /api/rooms/status - Get room status history (calendar)
///
/// Reads from canonical PG tables (`ht_rooms_new` + `ht_room_types` +
/// `ht_checkins` + `ht_bookings` + `ht_booking_rooms`).
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
