//! Dashboard statistics API route
//!
//! - GET /api/stats - Get dashboard statistics
//!
//! Supports dual read source via `LEGACY_READ_SOURCE` env var:
//! - "pg" (default): Read from PostgreSQL legacy mirror tables
//! - "sqlserver": Read from SQL Server legacy database

use axum::{extract::{Query, State}, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::db::{DbPool, PgPool};
use crate::error::ApiResult;
use crate::routes::mode::{AppState, Branch};

/// Dashboard statistics
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardStats {
    pub total_rooms: i32,
    pub occupied_rooms: i32,
    pub checkout_rooms: i32,
    pub booked_rooms: i32,
    pub today_check_ins: i32,
    pub today_check_outs: i32,
    pub active_bookings: i32,
    pub total_customers: i32,
}

/// Stats response
#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub success: bool,
    pub data: DashboardStats,
}

/// Check if we should read from SQL Server legacy database
fn use_sqlserver() -> bool {
    std::env::var("LEGACY_READ_SOURCE")
        .map(|v| v.eq_ignore_ascii_case("sqlserver"))
        .unwrap_or(false)
}

/// Query parameters for stats (branch support)
#[derive(Debug, Deserialize)]
pub struct StatsQuery {
    pub branch: Option<Branch>,
}

/// GET /api/stats - Get dashboard statistics
pub async fn get_stats(
    State(state): State<AppState>,
    Query(params): Query<StatsQuery>,
) -> ApiResult<Json<StatsResponse>> {
    let branch = params.branch.unwrap_or_default();

    let stats = match branch {
        Branch::Hfhotel => {
            if use_sqlserver() {
                get_stats_sqlserver(&state.legacy_pool).await?
            } else {
                get_stats_pg(&state.new_pool).await?
            }
        }
        Branch::Hfville => {
            get_stats_pg(state.ville_pool()?).await?
        }
        Branch::All => {
            let hf = if use_sqlserver() {
                get_stats_sqlserver(&state.legacy_pool).await?
            } else {
                get_stats_pg(&state.new_pool).await?
            };
            if let Ok(vp) = state.ville_pool() {
                let ville = get_stats_pg(vp).await?;
                DashboardStats {
                    total_rooms: hf.total_rooms + ville.total_rooms,
                    occupied_rooms: hf.occupied_rooms + ville.occupied_rooms,
                    checkout_rooms: hf.checkout_rooms + ville.checkout_rooms,
                    booked_rooms: hf.booked_rooms + ville.booked_rooms,
                    today_check_ins: hf.today_check_ins + ville.today_check_ins,
                    today_check_outs: hf.today_check_outs + ville.today_check_outs,
                    active_bookings: hf.active_bookings + ville.active_bookings,
                    total_customers: hf.total_customers + ville.total_customers,
                }
            } else {
                hf
            }
        }
    };

    Ok(Json(StatsResponse {
        success: true,
        data: stats,
    }))
}

/// Fetch dashboard stats from PostgreSQL legacy mirror tables
async fn get_stats_pg(pool: &PgPool) -> ApiResult<DashboardStats> {
    // Total rooms count
    let total_rooms: i64 = sqlx::query("SELECT COUNT(*) as count FROM ht_rooms_legacy")
        .fetch_one(pool)
        .await?
        .try_get("count")
        .unwrap_or(0);

    // Occupied rooms count - rooms with guests checked in (excludes checkout rooms after 6 AM)
    let occupied_rooms: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) as count
        FROM ht_rooms_legacy
        WHERE room_use = 'yes'
            AND room_no NOT IN (
                SELECT DISTINCT c.cin_room_no
                FROM ht_checkins_legacy c
                WHERE c.cin_room_out::date = CURRENT_DATE
                    AND EXTRACT(HOUR FROM NOW()) >= 6
                    AND c.cin_room_in = (
                        SELECT MAX(c2.cin_room_in)
                        FROM ht_checkins_legacy c2
                        WHERE c2.cin_room_no = c.cin_room_no
                    )
            )
        "#,
    )
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Checkout rooms count - rooms with checkout today (after 6 AM)
    let checkout_rooms: i64 = sqlx::query(
        r#"
        SELECT COUNT(DISTINCT r.room_no) as count
        FROM ht_rooms_legacy r
        INNER JOIN ht_checkins_legacy c ON r.room_no = c.cin_room_no
        WHERE r.room_use = 'yes'
            AND c.cin_room_out::date = CURRENT_DATE
            AND EXTRACT(HOUR FROM NOW()) >= 6
            AND c.cin_room_in = (
                SELECT MAX(c2.cin_room_in)
                FROM ht_checkins_legacy c2
                WHERE c2.cin_room_no = c.cin_room_no
            )
        "#,
    )
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Booked rooms count - rooms with booking but not checked in
    let booked_rooms: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) as count
        FROM ht_rooms_legacy
        WHERE room_use <> 'yes' AND room_book IS NOT NULL AND room_book <> ''
        "#,
    )
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Today's check-ins count
    let today_check_ins: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) as count
        FROM ht_checkins_legacy
        WHERE cin_room_in::date = CURRENT_DATE
        "#,
    )
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Today's check-outs count
    let today_check_outs: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) as count
        FROM ht_checkins_legacy
        WHERE cin_room_out::date = CURRENT_DATE
        "#,
    )
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Active bookings count
    let active_bookings: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) as count
        FROM ht_bookings_legacy
        WHERE book_status IS NOT NULL
        "#,
    )
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Total customers count
    let total_customers: i64 = sqlx::query("SELECT COUNT(*) as count FROM ht_customers_legacy")
        .fetch_one(pool)
        .await?
        .try_get("count")
        .unwrap_or(0);

    Ok(DashboardStats {
        total_rooms: total_rooms as i32,
        occupied_rooms: occupied_rooms as i32,
        checkout_rooms: checkout_rooms as i32,
        booked_rooms: booked_rooms as i32,
        today_check_ins: today_check_ins as i32,
        today_check_outs: today_check_outs as i32,
        active_bookings: active_bookings as i32,
        total_customers: total_customers as i32,
    })
}

/// Fetch dashboard stats from SQL Server legacy database
async fn get_stats_sqlserver(pool: &DbPool) -> ApiResult<DashboardStats> {
    let mut conn = pool.get().await?;

    // Total rooms count
    let total_rooms_rows = conn
        .simple_query("SELECT COUNT(*) as count FROM HT_Rooms")
        .await?
        .into_first_result()
        .await?;
    let total_rooms: i32 = total_rooms_rows
        .first()
        .and_then(|r| r.get::<i32, _>("count"))
        .unwrap_or(0);

    // Occupied rooms count - rooms with guests checked in (excludes checkout rooms after 6 AM)
    let occupied_rooms_rows = conn
        .simple_query(
            r#"
            SELECT COUNT(*) as count
            FROM HT_Rooms
            WHERE Room_Use = 'yes'
                AND Room_no NOT IN (
                    SELECT DISTINCT c.Cin_Room_No
                    FROM View_CheckIn_Ds c
                    WHERE CAST(c.Cin_Room_Out AS DATE) = CAST(GETDATE() AS DATE)
                        AND DATEPART(HOUR, GETDATE()) >= 6
                        AND c.Cin_Room_In = (
                            SELECT MAX(c2.Cin_Room_In)
                            FROM View_CheckIn_Ds c2
                            WHERE c2.Cin_Room_No = c.Cin_Room_No
                        )
                )
            "#,
        )
        .await?
        .into_first_result()
        .await?;
    let occupied_rooms: i32 = occupied_rooms_rows
        .first()
        .and_then(|r| r.get::<i32, _>("count"))
        .unwrap_or(0);

    // Checkout rooms count - rooms with checkout today (after 6 AM)
    let checkout_rooms_rows = conn
        .simple_query(
            r#"
            SELECT COUNT(DISTINCT r.Room_no) as count
            FROM HT_Rooms r
            INNER JOIN View_CheckIn_Ds c ON r.Room_no = c.Cin_Room_No
            WHERE r.Room_Use = 'yes'
                AND CAST(c.Cin_Room_Out AS DATE) = CAST(GETDATE() AS DATE)
                AND DATEPART(HOUR, GETDATE()) >= 6
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
    let checkout_rooms: i32 = checkout_rooms_rows
        .first()
        .and_then(|r| r.get::<i32, _>("count"))
        .unwrap_or(0);

    // Booked rooms count - rooms with booking but not checked in
    let booked_rooms_rows = conn
        .simple_query(
            r#"
            SELECT COUNT(*) as count
            FROM HT_Rooms
            WHERE Room_Use <> 'yes' AND Room_Book IS NOT NULL AND Room_Book <> ''
            "#,
        )
        .await?
        .into_first_result()
        .await?;
    let booked_rooms: i32 = booked_rooms_rows
        .first()
        .and_then(|r| r.get::<i32, _>("count"))
        .unwrap_or(0);

    // Today's check-ins count
    let today_checkins_rows = conn
        .simple_query(
            r#"
            SELECT COUNT(*) as count
            FROM View_CheckIn_Ds
            WHERE CAST(Cin_Room_In AS DATE) = CAST(GETDATE() AS DATE)
            "#,
        )
        .await?
        .into_first_result()
        .await?;
    let today_check_ins: i32 = today_checkins_rows
        .first()
        .and_then(|r| r.get::<i32, _>("count"))
        .unwrap_or(0);

    // Today's check-outs count
    let today_checkouts_rows = conn
        .simple_query(
            r#"
            SELECT COUNT(*) as count
            FROM View_CheckIn_Ds
            WHERE CAST(Cin_Room_Out AS DATE) = CAST(GETDATE() AS DATE)
            "#,
        )
        .await?
        .into_first_result()
        .await?;
    let today_check_outs: i32 = today_checkouts_rows
        .first()
        .and_then(|r| r.get::<i32, _>("count"))
        .unwrap_or(0);

    // Active bookings count
    let active_bookings_rows = conn
        .simple_query(
            r#"
            SELECT COUNT(*) as count
            FROM View_Booking_Ds
            WHERE Book_Status IS NOT NULL
            "#,
        )
        .await?
        .into_first_result()
        .await?;
    let active_bookings: i32 = active_bookings_rows
        .first()
        .and_then(|r| r.get::<i32, _>("count"))
        .unwrap_or(0);

    // Total customers count
    let total_customers_rows = conn
        .simple_query("SELECT COUNT(*) as count FROM View_Customers")
        .await?
        .into_first_result()
        .await?;
    let total_customers: i32 = total_customers_rows
        .first()
        .and_then(|r| r.get::<i32, _>("count"))
        .unwrap_or(0);

    Ok(DashboardStats {
        total_rooms,
        occupied_rooms,
        checkout_rooms,
        booked_rooms,
        today_check_ins,
        today_check_outs,
        active_bookings,
        total_customers,
    })
}
