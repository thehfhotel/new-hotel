//! New Report API routes for HotelNew database
//!
//! - GET /api/new/reports/revenue - Revenue report with period grouping
//! - GET /api/new/reports/occupancy - Occupancy statistics
//! - GET /api/new/reports/revenue-by-room-type - Revenue breakdown by room type

use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use super::mode::{AppState, Branch};
use crate::error::{ApiError, ApiResult};

/// Group by period for revenue report
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GroupBy {
    Day,
    Week,
    Month,
}

impl GroupBy {
    pub fn as_sql_format(&self) -> &'static str {
        match self {
            GroupBy::Day => "YYYY-MM-DD",
            GroupBy::Week => r#"IYYY-"W"IW"#,
            GroupBy::Month => "YYYY-MM",
        }
    }
}

/// Query parameters for revenue report
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RevenueQuery {
    pub from: String,
    pub to: String,
    #[serde(default = "default_group_by")]
    pub group_by: GroupBy,
    /// Branch selector: 'hfhotel' (default) | 'hfville'. Site data lives in
    /// separate logical PG databases, so the pool selection is the site filter.
    pub branch: Option<Branch>,
}

fn default_group_by() -> GroupBy {
    GroupBy::Day
}

/// Revenue data point
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevenueDataPoint {
    pub period: String,
    pub revenue: f64,
    pub bookings: i32,
}

/// Response for revenue report
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevenueResponse {
    pub success: bool,
    pub data: Vec<RevenueDataPoint>,
}

/// Query parameters for occupancy report
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OccupancyQuery {
    pub from: String,
    pub to: String,
    /// Branch selector: 'hfhotel' (default) | 'hfville'. Site data lives in
    /// separate logical PG databases, so the pool selection is the site filter.
    pub branch: Option<Branch>,
}

/// Response for occupancy report
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OccupancyResponse {
    pub success: bool,
    pub occupancy_rate: f64,
    pub total_rooms: i32,
    pub occupied_nights: i32,
    pub available_nights: i32,
    pub adr: f64,
    pub revpar: f64,
    pub avg_stay_length: f64,
}

/// Revenue by room type data
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomTypeRevenue {
    pub room_type: String,
    pub revenue: f64,
    pub percentage: f64,
}

/// Response for revenue by room type
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RevenueByRoomTypeResponse {
    pub success: bool,
    pub data: Vec<RoomTypeRevenue>,
}

/// GET /api/new/reports/revenue - Revenue report with period grouping
///
/// Revenue calculation: Sum of (Cin_Rate_Per_Night * nights stayed) for completed check-ins
pub async fn get_revenue(
    State(state): State<AppState>,
    Query(params): Query<RevenueQuery>,
) -> ApiResult<Json<RevenueResponse>> {
    // Branch-aware: HF Ville reads ville_pool. `All` → HF Hotel (the report
    // is single-site; cross-site aggregation is out of scope here).
    let pool = match params.branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool()?,
        Branch::Hfhotel | Branch::All => &state.new_pool,
    };

    // Validate date parameters
    if params.from.is_empty() || params.to.is_empty() {
        return Err(ApiError::BadRequest("Both 'from' and 'to' dates are required".to_string()));
    }

    let date_format = params.group_by.as_sql_format();
    let from_escaped = params.from.replace('\'', "''");
    let to_escaped = params.to.replace('\'', "''");

    // For grouping, we use the check-in date as the revenue attribution date
    // Revenue = Rate_Per_Night * (checkout_date - checkin_date)
    let query = format!(
        r#"
        WITH DateRange AS (
            SELECT
                TO_CHAR(ci.cin_checkin_time, '{}') as period,
                COALESCE(ci.cin_total_amount,
                    ci.cin_rate_per_night * (COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout)::date - ci.cin_checkin_time::date)) as revenue,
                ci.cin_id
            FROM ht_checkins ci
            WHERE ci.cin_status = 'checkedout'
              AND ci.cin_checkin_time::date >= '{}'
              AND ci.cin_checkin_time::date <= '{}'
        )
        SELECT
            period,
            COALESCE(SUM(revenue), 0)::float8 as total_revenue,
            COUNT(*)::int as booking_count
        FROM DateRange
        GROUP BY period
        ORDER BY period ASC
        "#,
        date_format, from_escaped, to_escaped
    );

    let rows = sqlx::query(sqlx::AssertSqlSafe(&*query))
        .fetch_all(pool)
        .await?;

    let data: Vec<RevenueDataPoint> = rows
        .iter()
        .map(|row| RevenueDataPoint {
            period: row.try_get::<String, _>("period").unwrap_or_default(),
            revenue: row.try_get::<f64, _>("total_revenue").unwrap_or(0.0),
            bookings: row.try_get::<i32, _>("booking_count").unwrap_or(0),
        })
        .collect();

    Ok(Json(RevenueResponse {
        success: true,
        data,
    }))
}

/// GET /api/new/reports/occupancy - Occupancy statistics
///
/// Occupancy calculation: (Occupied room-nights / Total available room-nights) * 100
pub async fn get_occupancy(
    State(state): State<AppState>,
    Query(params): Query<OccupancyQuery>,
) -> ApiResult<Json<OccupancyResponse>> {
    // Branch-aware: HF Ville reads ville_pool. `All` → HF Hotel (single-site).
    let pool = match params.branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool()?,
        Branch::Hfhotel | Branch::All => &state.new_pool,
    };

    // Validate date parameters
    if params.from.is_empty() || params.to.is_empty() {
        return Err(ApiError::BadRequest("Both 'from' and 'to' dates are required".to_string()));
    }

    let from_escaped = params.from.replace('\'', "''");
    let to_escaped = params.to.replace('\'', "''");

    // Get total number of rooms
    let rooms_query = "SELECT COUNT(*)::int as total_rooms FROM ht_rooms_new WHERE room_active = true";
    let rooms_rows = sqlx::query(rooms_query)
        .fetch_all(pool)
        .await?;

    let total_rooms: i32 = rooms_rows
        .first()
        .map(|r| r.try_get::<i32, _>("total_rooms").unwrap_or(0))
        .unwrap_or(0);

    if total_rooms == 0 {
        return Ok(Json(OccupancyResponse {
            success: true,
            occupancy_rate: 0.0,
            total_rooms: 0,
            occupied_nights: 0,
            available_nights: 0,
            adr: 0.0,
            revpar: 0.0,
            avg_stay_length: 0.0,
        }));
    }

    // Calculate the number of days in the period
    let days_query = format!(
        "SELECT ('{}'::date - '{}'::date) + 1 as total_days",
        to_escaped, from_escaped
    );
    let days_rows = sqlx::query(sqlx::AssertSqlSafe(&*days_query))
        .fetch_all(pool)
        .await?;

    let total_days: i32 = days_rows
        .first()
        .map(|r| r.try_get::<i32, _>("total_days").unwrap_or(1))
        .unwrap_or(1);

    // Total available room-nights
    let available_nights = total_rooms * total_days;

    // Calculate occupied room-nights from check-ins that overlap with the date range
    // For each check-in, count the nights that fall within the requested period
    let occupancy_query = format!(
        r#"
        SELECT
            COALESCE(SUM(
                (CASE WHEN COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout)::date > '{}'
                     THEN ('{}'::date + 1)
                     ELSE COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout)::date END)
                -
                (CASE WHEN ci.cin_checkin_time::date < '{}'
                     THEN '{}'::date
                     ELSE ci.cin_checkin_time::date END)
            ), 0) as occupied_nights,
            COALESCE(SUM(COALESCE(ci.cin_total_amount,
                ci.cin_rate_per_night * (COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout)::date - ci.cin_checkin_time::date))), 0)::float8 as total_revenue,
            COUNT(*)::int as checkin_count,
            COALESCE(AVG((COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout)::date - ci.cin_checkin_time::date)::float8), 0) as avg_stay
        FROM ht_checkins ci
        WHERE ci.cin_status IN ('active', 'checkedout')
          AND ci.cin_checkin_time::date <= '{}'
          AND COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout)::date >= '{}'
        "#,
        to_escaped, to_escaped, from_escaped, from_escaped, to_escaped, from_escaped
    );

    let occupancy_rows = sqlx::query(sqlx::AssertSqlSafe(&*occupancy_query))
        .fetch_all(pool)
        .await?;

    let (occupied_nights, total_revenue, _checkin_count, avg_stay) = occupancy_rows
        .first()
        .map(|r| {
            (
                r.try_get::<i32, _>("occupied_nights").unwrap_or(0),
                r.try_get::<f64, _>("total_revenue").unwrap_or(0.0),
                r.try_get::<i32, _>("checkin_count").unwrap_or(0),
                r.try_get::<f64, _>("avg_stay").unwrap_or(0.0),
            )
        })
        .unwrap_or((0, 0.0, 0, 0.0));

    // Calculate metrics
    let occupancy_rate = if available_nights > 0 {
        (occupied_nights as f64 / available_nights as f64) * 100.0
    } else {
        0.0
    };

    // ADR (Average Daily Rate) = Total Revenue / Occupied Room-Nights
    let adr = if occupied_nights > 0 {
        total_revenue / occupied_nights as f64
    } else {
        0.0
    };

    // RevPAR (Revenue Per Available Room) = Total Revenue / Available Room-Nights
    // Or equivalently: ADR * Occupancy Rate
    let revpar = if available_nights > 0 {
        total_revenue / available_nights as f64
    } else {
        0.0
    };

    Ok(Json(OccupancyResponse {
        success: true,
        occupancy_rate: (occupancy_rate * 10.0).round() / 10.0, // Round to 1 decimal
        total_rooms,
        occupied_nights,
        available_nights,
        adr: (adr * 100.0).round() / 100.0, // Round to 2 decimals
        revpar: (revpar * 100.0).round() / 100.0,
        avg_stay_length: (avg_stay * 10.0).round() / 10.0,
    }))
}

/// GET /api/new/reports/revenue-by-room-type - Revenue breakdown by room type
pub async fn get_revenue_by_room_type(
    State(state): State<AppState>,
    Query(params): Query<OccupancyQuery>,
) -> ApiResult<Json<RevenueByRoomTypeResponse>> {
    // Branch-aware: HF Ville reads ville_pool. `All` → HF Hotel (single-site).
    let pool = match params.branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool()?,
        Branch::Hfhotel | Branch::All => &state.new_pool,
    };

    // Validate date parameters
    if params.from.is_empty() || params.to.is_empty() {
        return Err(ApiError::BadRequest("Both 'from' and 'to' dates are required".to_string()));
    }

    let from_escaped = params.from.replace('\'', "''");
    let to_escaped = params.to.replace('\'', "''");

    // Get revenue grouped by room type
    let query = format!(
        r#"
        SELECT
            COALESCE(rt.type_name, 'Unknown') as room_type,
            COALESCE(SUM(COALESCE(ci.cin_total_amount,
                ci.cin_rate_per_night * (COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout)::date - ci.cin_checkin_time::date))), 0)::float8 as revenue
        FROM ht_checkins ci
        LEFT JOIN ht_rooms_new r ON ci.cin_room_id = r.room_id
        LEFT JOIN ht_room_types rt ON r.room_type_id = rt.type_id
        WHERE ci.cin_status = 'checkedout'
          AND ci.cin_checkin_time::date >= '{}'
          AND ci.cin_checkin_time::date <= '{}'
        GROUP BY rt.type_name
        ORDER BY revenue DESC
        "#,
        from_escaped, to_escaped
    );

    let rows = sqlx::query(sqlx::AssertSqlSafe(&*query))
        .fetch_all(pool)
        .await?;

    let mut data: Vec<RoomTypeRevenue> = rows
        .iter()
        .map(|row| RoomTypeRevenue {
            room_type: row.try_get::<String, _>("room_type").unwrap_or_else(|_| "Unknown".to_string()),
            revenue: row.try_get::<f64, _>("revenue").unwrap_or(0.0),
            percentage: 0.0, // Will calculate after
        })
        .collect();

    // Calculate total revenue for percentage calculation
    let total_revenue: f64 = data.iter().map(|d| d.revenue).sum();

    // Calculate percentages
    for item in &mut data {
        item.percentage = if total_revenue > 0.0 {
            ((item.revenue / total_revenue) * 1000.0).round() / 10.0 // Round to 1 decimal
        } else {
            0.0
        };
    }

    Ok(Json(RevenueByRoomTypeResponse {
        success: true,
        data,
    }))
}
