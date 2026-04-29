//! Occupancy trends API route
//!
//! - GET /api/occupancy - Get occupancy trends for the last N days
//!
//! Reads from PG (`ht_checkins_legacy` cache, fed by drift-reconcile + CT mappers).

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::db::PgPool;
use crate::error::ApiResult;
use crate::routes::mode::{AppState, Branch};

/// Query parameters for occupancy
#[derive(Debug, Deserialize)]
pub struct OccupancyQuery {
    #[serde(default = "default_days")]
    pub days: i32,
    pub branch: Option<Branch>,
}

fn default_days() -> i32 { 7 }

/// Occupancy data point
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OccupancyData {
    pub date: NaiveDate,
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
    State(state): State<AppState>,
    Query(params): Query<OccupancyQuery>,
) -> ApiResult<Json<OccupancyResponse>> {
    let branch = params.branch.unwrap_or_default();

    let data = match branch {
        Branch::Hfhotel | Branch::All => get_occupancy_pg(&state.new_pool, params.days).await?,
        Branch::Hfville => get_occupancy_pg(state.ville_pool()?, params.days).await?,
    };

    Ok(Json(OccupancyResponse {
        success: true,
        data,
    }))
}

/// Read occupancy data from PostgreSQL (ht_checkins_legacy)
async fn get_occupancy_pg(pool: &PgPool, days: i32) -> ApiResult<Vec<OccupancyData>> {
    let rows = sqlx::query(
        r#"
        WITH DateRange AS (
            SELECT (CURRENT_DATE - n)::date as date_val
            FROM generate_series(0, $1 - 1) AS n
        )
        SELECT
            dr.date_val as date,
            COUNT(DISTINCT c.cin_room_no)::int as occupied_rooms
        FROM DateRange dr
        LEFT JOIN ht_checkins_legacy c ON
            c.cin_room_in::date <= dr.date_val
            AND c.cin_room_out::date > dr.date_val
        GROUP BY dr.date_val
        ORDER BY dr.date_val ASC
        "#,
    )
    .bind(days)
    .fetch_all(pool)
    .await?;

    let data: Vec<OccupancyData> = rows
        .iter()
        .map(|row| OccupancyData {
            date: row.get("date"),
            occupied_rooms: row.get::<Option<i32>, _>("occupied_rooms").unwrap_or(0),
        })
        .collect();

    Ok(data)
}

