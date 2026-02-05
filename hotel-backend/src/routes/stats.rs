//! Dashboard statistics API route
//!
//! - GET /api/stats - Get dashboard statistics

use axum::{extract::State, Json};
use serde::Serialize;

use crate::db::DbPool;
use crate::error::ApiResult;

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

/// GET /api/stats - Get dashboard statistics
pub async fn get_stats(State(pool): State<DbPool>) -> ApiResult<Json<StatsResponse>> {
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

    Ok(Json(StatsResponse {
        success: true,
        data: DashboardStats {
            total_rooms,
            occupied_rooms,
            checkout_rooms,
            booked_rooms,
            today_check_ins,
            today_check_outs,
            active_bookings,
            total_customers,
        },
    }))
}
