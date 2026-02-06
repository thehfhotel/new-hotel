//! New Stats API routes for HotelNew database
//!
//! - GET /api/new/stats - Dashboard statistics

use axum::{extract::State, Json};
use serde::Serialize;
use sqlx::Row;

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
    let pool = &state.new_pool;

    // Query room counts by status
    let room_stats_sql =
        r#"
            SELECT
                COUNT(*)::int as total_rooms,
                SUM(CASE WHEN room_status = 'available' THEN 1 ELSE 0 END)::int as available_rooms,
                SUM(CASE WHEN room_status = 'occupied' THEN 1 ELSE 0 END)::int as occupied_rooms,
                SUM(CASE WHEN room_status = 'maintenance' THEN 1 ELSE 0 END)::int as maintenance_rooms,
                SUM(CASE WHEN room_status = 'cleaning' THEN 1 ELSE 0 END)::int as cleaning_rooms
            FROM ht_rooms_new
            "#;

    let room_stats_rows = sqlx::query(room_stats_sql).fetch_all(pool).await?;

    let (total_rooms, available_rooms, occupied_rooms, maintenance_rooms, cleaning_rooms) =
        room_stats_rows
            .first()
            .map(|row| {
                (
                    row.try_get::<i32, _>("total_rooms").unwrap_or(0),
                    row.try_get::<i32, _>("available_rooms").unwrap_or(0),
                    row.try_get::<i32, _>("occupied_rooms").unwrap_or(0),
                    row.try_get::<i32, _>("maintenance_rooms").unwrap_or(0),
                    row.try_get::<i32, _>("cleaning_rooms").unwrap_or(0),
                )
            })
            .unwrap_or((0, 0, 0, 0, 0));

    // Query today's check-ins (active check-ins that started today)
    let checkin_stats_sql =
        r#"
            SELECT
                SUM(CASE WHEN cin_checkin_time::date = NOW()::date THEN 1 ELSE 0 END)::int as today_check_ins,
                SUM(CASE WHEN cin_checkout_time::date = NOW()::date THEN 1 ELSE 0 END)::int as today_check_outs
            FROM ht_checkins
            "#;

    let checkin_stats_rows = sqlx::query(checkin_stats_sql).fetch_all(pool).await?;

    let (today_check_ins, today_check_outs) = checkin_stats_rows
        .first()
        .map(|row| {
            (
                row.try_get::<i32, _>("today_check_ins").unwrap_or(0),
                row.try_get::<i32, _>("today_check_outs").unwrap_or(0),
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
