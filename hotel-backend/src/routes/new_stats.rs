//! New Stats API routes for HotelNew database
//!
//! - GET /api/new/stats - Dashboard statistics

use axum::{extract::State, Json};
use serde::Serialize;

use super::mode::AppState;
use crate::error::ApiResult;

/// Dashboard statistics data
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsData {
    pub total_rooms: i32,
    pub available_rooms: i32,
    pub occupied_rooms: i32,
    pub maintenance_rooms: i32,
    pub cleaning_rooms: i32,
    pub today_check_ins: i32,
    pub today_check_outs: i32,
    pub occupancy_rate: f64,
}

/// Response for stats endpoint
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsResponse {
    pub success: bool,
    pub data: StatsData,
}

/// GET /api/new/stats - Dashboard statistics
pub async fn get_stats(State(state): State<AppState>) -> ApiResult<Json<StatsResponse>> {
    let mut conn = state.new_pool.get().await?;

    // Query room counts by status
    let room_stats_rows = conn
        .simple_query(
            r#"
            SELECT
                COUNT(*) as total_rooms,
                SUM(CASE WHEN Room_Status = 'available' THEN 1 ELSE 0 END) as available_rooms,
                SUM(CASE WHEN Room_Status = 'occupied' THEN 1 ELSE 0 END) as occupied_rooms,
                SUM(CASE WHEN Room_Status = 'maintenance' THEN 1 ELSE 0 END) as maintenance_rooms,
                SUM(CASE WHEN Room_Status = 'cleaning' THEN 1 ELSE 0 END) as cleaning_rooms
            FROM HT_Rooms_New
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let (total_rooms, available_rooms, occupied_rooms, maintenance_rooms, cleaning_rooms) =
        room_stats_rows
            .first()
            .map(|row| {
                (
                    row.get::<i32, _>("total_rooms").unwrap_or(0),
                    row.get::<i32, _>("available_rooms").unwrap_or(0),
                    row.get::<i32, _>("occupied_rooms").unwrap_or(0),
                    row.get::<i32, _>("maintenance_rooms").unwrap_or(0),
                    row.get::<i32, _>("cleaning_rooms").unwrap_or(0),
                )
            })
            .unwrap_or((0, 0, 0, 0, 0));

    // Query today's check-ins (active check-ins that started today)
    let checkin_stats_rows = conn
        .simple_query(
            r#"
            SELECT
                SUM(CASE WHEN CAST(Cin_CheckIn_Time AS DATE) = CAST(GETDATE() AS DATE) THEN 1 ELSE 0 END) as today_check_ins,
                SUM(CASE WHEN CAST(Cin_CheckOut_Time AS DATE) = CAST(GETDATE() AS DATE) THEN 1 ELSE 0 END) as today_check_outs
            FROM HT_CheckIns
            "#,
        )
        .await?
        .into_first_result()
        .await?;

    let (today_check_ins, today_check_outs) = checkin_stats_rows
        .first()
        .map(|row| {
            (
                row.get::<i32, _>("today_check_ins").unwrap_or(0),
                row.get::<i32, _>("today_check_outs").unwrap_or(0),
            )
        })
        .unwrap_or((0, 0));

    // Calculate occupancy rate
    let occupancy_rate = if total_rooms > 0 {
        (occupied_rooms as f64 / total_rooms as f64) * 100.0
    } else {
        0.0
    };

    Ok(Json(StatsResponse {
        success: true,
        data: StatsData {
            total_rooms,
            available_rooms,
            occupied_rooms,
            maintenance_rooms,
            cleaning_rooms,
            today_check_ins,
            today_check_outs,
            occupancy_rate,
        },
    }))
}
