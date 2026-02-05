//! Occupancy trends API route
//!
//! - GET /api/occupancy - Get occupancy trends for the last N days

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::db::DbPool;
use crate::error::ApiResult;

/// Query parameters for occupancy
#[derive(Debug, Deserialize)]
pub struct OccupancyQuery {
    #[serde(default = "default_days")]
    pub days: i32,
}

fn default_days() -> i32 { 7 }

/// Occupancy data point
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OccupancyData {
    pub date: NaiveDateTime,
    pub occupied_rooms: i32,
}

/// Occupancy response
#[derive(Debug, Serialize)]
pub struct OccupancyResponse {
    pub success: bool,
    pub data: Vec<OccupancyData>,
}

/// GET /api/occupancy - Get occupancy trends
pub async fn get_occupancy(
    State(pool): State<DbPool>,
    Query(params): Query<OccupancyQuery>,
) -> ApiResult<Json<OccupancyResponse>> {
    let mut conn = pool.get().await?;

    // Get occupancy for the last N days
    // Count rooms where check-in date <= day AND check-out date > day
    let query = format!(
        r#"
        WITH DateRange AS (
            SELECT CAST(DATEADD(day, -n, CAST(GETDATE() AS DATE)) AS DATE) as date_val
            FROM (
                SELECT 0 as n UNION ALL SELECT 1 UNION ALL SELECT 2 UNION ALL SELECT 3
                UNION ALL SELECT 4 UNION ALL SELECT 5 UNION ALL SELECT 6
                UNION ALL SELECT 7 UNION ALL SELECT 8 UNION ALL SELECT 9
                UNION ALL SELECT 10 UNION ALL SELECT 11 UNION ALL SELECT 12 UNION ALL SELECT 13
            ) numbers
            WHERE n < {}
        )
        SELECT
            dr.date_val as date,
            COUNT(DISTINCT c.Cin_Room_No) as occupiedRooms
        FROM DateRange dr
        LEFT JOIN View_CheckIn_Ds c ON
            CAST(c.Cin_Room_In AS DATE) <= dr.date_val
            AND CAST(c.Cin_Room_Out AS DATE) > dr.date_val
        GROUP BY dr.date_val
        ORDER BY dr.date_val ASC
        "#,
        params.days
    );

    let rows = conn
        .simple_query(&query)
        .await?
        .into_first_result()
        .await?;

    let data: Vec<OccupancyData> = rows
        .iter()
        .filter_map(|row| {
            row.get::<NaiveDateTime, _>("date").map(|date| OccupancyData {
                date,
                occupied_rooms: row.get::<i32, _>("occupiedRooms").unwrap_or(0),
            })
        })
        .collect();

    Ok(Json(OccupancyResponse {
        success: true,
        data,
    }))
}
