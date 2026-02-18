//! Room API routes
//!
//! - GET /api/rooms - List all rooms
//! - GET /api/rooms/:id - Get room details with current guest
//! - GET /api/rooms/status - Get room status history
//! - GET /api/rooms/checkouts-today - Get rooms with checkout today
//!
//! Supports dual read source via LEGACY_READ_SOURCE env var:
//! - "pg" (default): Read from PostgreSQL tables (ht_rooms_legacy, ht_checkins_legacy)
//! - "sqlserver": Read from SQL Server (legacy tiberius path)

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::Row;
use tiberius::Row as TiberiusRow;

use crate::error::{ApiError, ApiResult};
use crate::models::{
    CheckoutsTodayResponse, CurrentGuest, Room, RoomDetail, RoomDetailResponse, RoomStatus,
    RoomStatusResponse, RoomsResponse,
};
use crate::routes::mode::AppState;

/// Check if we should read from PostgreSQL (default) or SQL Server
fn use_pg_source() -> bool {
    std::env::var("LEGACY_READ_SOURCE")
        .unwrap_or_else(|_| "pg".to_string())
        .to_lowercase()
        != "sqlserver"
}

// ─────────────────────────────────────────────────────────────
// PostgreSQL helpers
// ─────────────────────────────────────────────────────────────

/// List all rooms from PostgreSQL (ht_rooms_legacy + ht_rooms_new)
async fn list_rooms_pg(pool: &crate::db::PgPool) -> ApiResult<Vec<Room>> {
    // Primary: ht_rooms_legacy with optional price overrides from ht_rooms_new.
    // Also include rooms that exist only in ht_rooms_new (via UNION ALL).
    // Note: ht_rooms_new has different column names (room_clean is boolean, room_type_id
    // is FK to ht_room_types, etc.), so we map them to the legacy string format.
    let rows = sqlx::query(
        r#"
        SELECT
            l.room_no,
            l.room_type,
            l.room_details,
            l.room_clean,
            l.room_use,
            l.room_book,
            l.room_manternace,
            COALESCE(n.room_price_weekday::float8, l.room_price_a::float8) AS room_price_a,
            COALESCE(n.room_price_weekend::float8, l.room_price_b::float8) AS room_price_b,
            l.room_price_c::float8 AS room_price_c,
            l.room_group,
            l.room_book_name
        FROM ht_rooms_legacy l
        LEFT JOIN ht_rooms_new n ON n.room_no = l.room_no

        UNION ALL

        SELECT
            n.room_no,
            COALESCE(rt.type_name, n.room_status) AS room_type,
            n.room_notes AS room_details,
            CASE WHEN n.room_clean THEN 'yes' ELSE 'no' END AS room_clean,
            CASE WHEN n.room_status = 'occupied' THEN 'yes' ELSE 'no' END AS room_use,
            'no' AS room_book,
            CASE WHEN n.room_maintenance THEN 'yes' ELSE 'no' END AS room_manternace,
            n.room_price_weekday::float8 AS room_price_a,
            n.room_price_weekend::float8 AS room_price_b,
            n.room_price_special::float8 AS room_price_c,
            NULL AS room_group,
            NULL AS room_book_name
        FROM ht_rooms_new n
        LEFT JOIN ht_room_types rt ON rt.type_id = n.room_type_id
        WHERE n.room_active = true
          AND NOT EXISTS (
              SELECT 1 FROM ht_rooms_legacy l WHERE l.room_no = n.room_no
          )

        ORDER BY room_no
        "#,
    )
    .fetch_all(pool)
    .await?;

    let rooms: Vec<Room> = rows
        .iter()
        .map(|row| Room {
            room_no: row.get::<String, _>("room_no"),
            room_type: row.get::<Option<String>, _>("room_type"),
            room_details: row.get::<Option<String>, _>("room_details"),
            room_clean: row.get::<Option<String>, _>("room_clean"),
            room_use: row.get::<Option<String>, _>("room_use"),
            room_book: row.get::<Option<String>, _>("room_book"),
            room_manternace: row.get::<Option<String>, _>("room_manternace"),
            room_price_a: row.get::<Option<f64>, _>("room_price_a"),
            room_price_b: row.get::<Option<f64>, _>("room_price_b"),
            room_price_c: row.get::<Option<f64>, _>("room_price_c"),
            room_group: row.get::<Option<String>, _>("room_group"),
            room_book_name: row.get::<Option<String>, _>("room_book_name"),
        })
        .collect();

    Ok(rooms)
}

/// Get a single room detail from PostgreSQL with current guest info
async fn get_room_pg(pool: &crate::db::PgPool, room_no: &str) -> ApiResult<RoomDetail> {
    // Try legacy first (with new-system price overrides), then new-only
    let room_row = sqlx::query(
        r#"
        SELECT
            l.room_no,
            l.room_type,
            l.room_details,
            l.room_clean,
            l.room_use,
            l.room_book,
            l.room_manternace,
            COALESCE(n.room_price_weekday::float8, l.room_price_a::float8) AS room_price_a,
            COALESCE(n.room_price_weekend::float8, l.room_price_b::float8) AS room_price_b,
            l.room_price_c::float8 AS room_price_c,
            l.room_group,
            l.room_book_name,
            l.room_book_time
        FROM ht_rooms_legacy l
        LEFT JOIN ht_rooms_new n ON n.room_no = l.room_no
        WHERE l.room_no = $1
        "#,
    )
    .bind(room_no)
    .fetch_optional(pool)
    .await?;

    // If not found in legacy, try new-only rooms
    let room_row = match room_row {
        Some(row) => row,
        None => {
            sqlx::query(
                r#"
                SELECT
                    n.room_no,
                    COALESCE(rt.type_name, n.room_status) AS room_type,
                    n.room_notes AS room_details,
                    CASE WHEN n.room_clean THEN 'yes' ELSE 'no' END AS room_clean,
                    CASE WHEN n.room_status = 'occupied' THEN 'yes' ELSE 'no' END AS room_use,
                    'no' AS room_book,
                    CASE WHEN n.room_maintenance THEN 'yes' ELSE 'no' END AS room_manternace,
                    n.room_price_weekday::float8 AS room_price_a,
                    n.room_price_weekend::float8 AS room_price_b,
                    n.room_price_special::float8 AS room_price_c,
                    NULL::varchar AS room_group,
                    NULL::varchar AS room_book_name,
                    NULL::timestamp AS room_book_time
                FROM ht_rooms_new n
                LEFT JOIN ht_room_types rt ON rt.type_id = n.room_type_id
                WHERE n.room_no = $1 AND n.room_active = true
                "#,
            )
            .bind(room_no)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| ApiError::NotFound("Room not found".to_string()))?
        }
    };

    // Get current/recent check-in from ht_checkins_legacy
    let checkin_row = sqlx::query(
        r#"
        SELECT
            cin_cust_name,
            cin_room_in,
            cin_room_out
        FROM ht_checkins_legacy
        WHERE cin_room_no = $1
        ORDER BY cin_room_in DESC
        LIMIT 1
        "#,
    )
    .bind(room_no)
    .fetch_optional(pool)
    .await?;

    let current_guest = checkin_row.map(|row| CurrentGuest {
        name: row.get::<Option<String>, _>("cin_cust_name"),
        check_in: row
            .get::<Option<NaiveDateTime>, _>("cin_room_in")
            .map(|dt| dt.and_utc()),
        check_out: row
            .get::<Option<NaiveDateTime>, _>("cin_room_out")
            .map(|dt| dt.and_utc()),
    });

    let room = RoomDetail {
        room_no: room_row.get::<String, _>("room_no"),
        room_type: room_row.get::<Option<String>, _>("room_type"),
        room_details: room_row.get::<Option<String>, _>("room_details"),
        room_clean: room_row.get::<Option<String>, _>("room_clean"),
        room_use: room_row.get::<Option<String>, _>("room_use"),
        room_book: room_row.get::<Option<String>, _>("room_book"),
        room_manternace: room_row.get::<Option<String>, _>("room_manternace"),
        room_price_a: room_row.get::<Option<f64>, _>("room_price_a"),
        room_price_b: room_row.get::<Option<f64>, _>("room_price_b"),
        room_price_c: room_row.get::<Option<f64>, _>("room_price_c"),
        room_group: room_row.get::<Option<String>, _>("room_group"),
        room_book_name: room_row.get::<Option<String>, _>("room_book_name"),
        room_book_time: room_row
            .get::<Option<NaiveDateTime>, _>("room_book_time")
            .map(|dt| dt.and_utc()),
        current_guest,
    };

    Ok(room)
}

/// Get rooms checking out today from PostgreSQL
async fn get_checkouts_today_pg(pool: &crate::db::PgPool) -> ApiResult<Vec<String>> {
    let rows = sqlx::query(
        r#"
        SELECT DISTINCT c.cin_room_no AS room_no
        FROM ht_checkins_legacy c
        INNER JOIN ht_rooms_legacy r ON c.cin_room_no = r.room_no
        WHERE c.cin_room_out::date = CURRENT_DATE
          AND r.room_use = 'yes'
          AND c.cin_room_in = (
              SELECT MAX(c2.cin_room_in)
              FROM ht_checkins_legacy c2
              WHERE c2.cin_room_no = c.cin_room_no
          )
        "#,
    )
    .fetch_all(pool)
    .await?;

    let room_numbers: Vec<String> = rows
        .iter()
        .filter_map(|row| row.get::<Option<String>, _>("room_no"))
        .collect();

    Ok(room_numbers)
}

// ─────────────────────────────────────────────────────────────
// SQL Server (tiberius) helpers - original legacy path
// ─────────────────────────────────────────────────────────────

/// List all rooms from SQL Server
async fn list_rooms_sqlserver(pool: &crate::db::DbPool) -> ApiResult<Vec<Room>> {
    let mut conn = pool.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT
                Room_no,
                Room_Type,
                Room_Details,
                Room_Clean,
                Room_Use,
                Room_Book,
                Room_Manternace,
                Room_PriceA,
                Room_PriceB,
                Room_PriceC,
                Room_Group,
                Room_Book_Name
            FROM HT_Rooms
            ORDER BY Room_no
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let rooms: Vec<Room> = rows
        .iter()
        .map(|row: &TiberiusRow| Room {
            room_no: row.get::<&str, _>("Room_no").unwrap_or_default().to_string(),
            room_type: row.get::<&str, _>("Room_Type").map(String::from),
            room_details: row.get::<&str, _>("Room_Details").map(String::from),
            room_clean: row.get::<&str, _>("Room_Clean").map(String::from),
            room_use: row.get::<&str, _>("Room_Use").map(String::from),
            room_book: row.get::<&str, _>("Room_Book").map(String::from),
            room_manternace: row.get::<&str, _>("Room_Manternace").map(String::from),
            room_price_a: row.get::<f64, _>("Room_PriceA"),
            room_price_b: row.get::<f64, _>("Room_PriceB"),
            room_price_c: row.get::<f64, _>("Room_PriceC"),
            room_group: row.get::<&str, _>("Room_Group").map(String::from),
            room_book_name: row.get::<&str, _>("Room_Book_Name").map(String::from),
        })
        .collect();

    Ok(rooms)
}

/// Get a single room detail from SQL Server with current guest info
async fn get_room_sqlserver(
    pool: &crate::db::DbPool,
    room_no: &str,
) -> ApiResult<RoomDetail> {
    let mut conn = pool.get().await?;

    // Get room details
    let room_rows = conn
        .query(
            r#"
            SELECT
                Room_no,
                Room_Type,
                Room_Details,
                Room_Clean,
                Room_Use,
                Room_Book,
                Room_Manternace,
                Room_PriceA,
                Room_PriceB,
                Room_PriceC,
                Room_Group,
                Room_Book_Name,
                Room_Book_Time
            FROM HT_Rooms
            WHERE Room_no = @P1
            "#,
            &[&room_no],
        )
        .await?
        .into_first_result()
        .await?;

    let room_row = room_rows
        .first()
        .ok_or_else(|| ApiError::NotFound("Room not found".to_string()))?;

    // Get current/recent check-in
    let checkin_rows = conn
        .query(
            r#"
            SELECT TOP 1
                Cin_Cust_Name,
                Cin_Room_In,
                Cin_Room_Out
            FROM View_CheckIn_Ds
            WHERE Cin_Room_No = @P1
            ORDER BY Cin_Room_In DESC
            "#,
            &[&room_no],
        )
        .await?
        .into_first_result()
        .await?;

    let current_guest = checkin_rows.first().map(|row: &TiberiusRow| CurrentGuest {
        name: row.get::<&str, _>("Cin_Cust_Name").map(String::from),
        check_in: row
            .try_get::<NaiveDateTime, _>("Cin_Room_In")
            .ok()
            .flatten()
            .map(|dt| dt.and_utc()),
        check_out: row
            .try_get::<NaiveDateTime, _>("Cin_Room_Out")
            .ok()
            .flatten()
            .map(|dt| dt.and_utc()),
    });

    let room = RoomDetail {
        room_no: room_row
            .get::<&str, _>("Room_no")
            .unwrap_or_default()
            .to_string(),
        room_type: room_row.get::<&str, _>("Room_Type").map(String::from),
        room_details: room_row.get::<&str, _>("Room_Details").map(String::from),
        room_clean: room_row.get::<&str, _>("Room_Clean").map(String::from),
        room_use: room_row.get::<&str, _>("Room_Use").map(String::from),
        room_book: room_row.get::<&str, _>("Room_Book").map(String::from),
        room_manternace: room_row
            .get::<&str, _>("Room_Manternace")
            .map(String::from),
        room_price_a: room_row.get::<f64, _>("Room_PriceA"),
        room_price_b: room_row.get::<f64, _>("Room_PriceB"),
        room_price_c: room_row.get::<f64, _>("Room_PriceC"),
        room_group: room_row.get::<&str, _>("Room_Group").map(String::from),
        room_book_name: room_row
            .get::<&str, _>("Room_Book_Name")
            .map(String::from),
        room_book_time: room_row
            .try_get::<NaiveDateTime, _>("Room_Book_Time")
            .ok()
            .flatten()
            .map(|dt| dt.and_utc()),
        current_guest,
    };

    Ok(room)
}

/// Get room status from SQL Server (View_Room_status)
async fn get_room_status_sqlserver(
    pool: &crate::db::DbPool,
    params: &RoomStatusQuery,
) -> ApiResult<Vec<RoomStatus>> {
    let mut conn = pool.get().await?;

    // Build dynamic query
    let mut query = String::from(
        r#"
        SELECT
            room_no,
            room_date,
            room_status,
            room_Details,
            room_CheckIn_No,
            Room_Type
        FROM View_Room_status
        "#,
    );

    let mut conditions: Vec<String> = Vec::new();

    if params.start_date.is_some() {
        conditions.push("room_date >= @P1".to_string());
    }

    if params.end_date.is_some() {
        conditions.push(format!(
            "room_date <= @P{}",
            if params.start_date.is_some() { 2 } else { 1 }
        ));
    }

    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }

    query.push_str(" ORDER BY room_no, room_date");

    // Execute based on parameters
    let rows: Vec<TiberiusRow> = match (&params.start_date, &params.end_date) {
        (Some(start), Some(end)) => {
            conn.query(&query, &[&start.as_str(), &end.as_str()])
                .await?
                .into_first_result()
                .await?
        }
        (Some(start), None) => {
            conn.query(&query, &[&start.as_str()])
                .await?
                .into_first_result()
                .await?
        }
        (None, Some(end)) => {
            conn.query(&query, &[&end.as_str()])
                .await?
                .into_first_result()
                .await?
        }
        (None, None) => {
            conn.simple_query(&query)
                .await?
                .into_first_result()
                .await?
        }
    };

    let statuses: Vec<RoomStatus> = rows
        .iter()
        .map(|row: &TiberiusRow| RoomStatus {
            room_no: row
                .get::<&str, _>("room_no")
                .unwrap_or_default()
                .to_string(),
            room_date: row
                .get::<NaiveDateTime, _>("room_date")
                .map(|dt| dt.and_utc()),
            room_status: row.get::<&str, _>("room_status").map(String::from),
            room_details: row.get::<&str, _>("room_Details").map(String::from),
            room_checkin_no: row.get::<&str, _>("room_CheckIn_No").map(String::from),
            room_type: row.get::<&str, _>("Room_Type").map(String::from),
        })
        .collect();

    Ok(statuses)
}

/// Get rooms checking out today from SQL Server
async fn get_checkouts_today_sqlserver(pool: &crate::db::DbPool) -> ApiResult<Vec<String>> {
    let mut conn = pool.get().await?;

    let rows = conn
        .simple_query(
            r#"
            SELECT DISTINCT c.Cin_Room_no as room_no
            FROM View_CheckIn_Ds c
            INNER JOIN HT_Rooms r ON c.Cin_Room_No = r.Room_no
            WHERE CAST(c.Cin_Room_Out AS DATE) = CAST(GETDATE() AS DATE)
                AND r.Room_Use = 'yes'
                AND c.Cin_Room_In = (
                    SELECT MAX(c2.Cin_Room_In)
                    FROM View_CheckIn_Ds c2
                    WHERE c2.Cin_Room_No = c.Cin_Room_No
                )
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let room_numbers: Vec<String> = rows
        .iter()
        .filter_map(|row: &TiberiusRow| row.get::<&str, _>("room_no").map(String::from))
        .collect();

    Ok(room_numbers)
}

// ─────────────────────────────────────────────────────────────
// Public route handlers (feature-flagged)
// ─────────────────────────────────────────────────────────────

/// GET /api/rooms - List all rooms
pub async fn list_rooms(State(state): State<AppState>) -> ApiResult<Json<RoomsResponse>> {
    let rooms = if use_pg_source() {
        list_rooms_pg(&state.new_pool).await?
    } else {
        list_rooms_sqlserver(&state.legacy_pool).await?
    };

    let total = rooms.len();

    Ok(Json(RoomsResponse {
        success: true,
        data: rooms,
        total,
    }))
}

/// GET /api/rooms/:id - Get room details with current guest
pub async fn get_room(
    State(state): State<AppState>,
    Path(room_no): Path<String>,
) -> ApiResult<Json<RoomDetailResponse>> {
    let room = if use_pg_source() {
        get_room_pg(&state.new_pool, &room_no).await?
    } else {
        get_room_sqlserver(&state.legacy_pool, &room_no).await?
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
}

/// GET /api/rooms/status - Get room status history
///
/// Note: View_Room_status does not exist in PostgreSQL, so this endpoint
/// always falls back to the SQL Server path regardless of LEGACY_READ_SOURCE.
pub async fn get_room_status(
    State(state): State<AppState>,
    Query(params): Query<RoomStatusQuery>,
) -> ApiResult<Json<RoomStatusResponse>> {
    // View_Room_status is a complex SQL Server view with no PG equivalent.
    // Always use the SQL Server path for this endpoint.
    let statuses = get_room_status_sqlserver(&state.legacy_pool, &params).await?;
    let total = statuses.len();

    Ok(Json(RoomStatusResponse {
        success: true,
        data: statuses,
        total,
    }))
}

/// GET /api/rooms/checkouts-today - Get rooms with checkout today
pub async fn get_checkouts_today(
    State(state): State<AppState>,
) -> ApiResult<Json<CheckoutsTodayResponse>> {
    let room_numbers = if use_pg_source() {
        get_checkouts_today_pg(&state.new_pool).await?
    } else {
        get_checkouts_today_sqlserver(&state.legacy_pool).await?
    };

    Ok(Json(CheckoutsTodayResponse {
        success: true,
        data: room_numbers,
    }))
}
