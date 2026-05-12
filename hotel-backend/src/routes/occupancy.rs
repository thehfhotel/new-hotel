//! Occupancy trends API route
//!
//! - GET /api/occupancy - Get occupancy trends for the last N days
//!
//! Reads from canonical PostgreSQL tables (`ht_checkins`). Previously read
//! the `ht_checkins_legacy` mirror, but that mirror was demoted to hash-only
//! drift detection in Phase 5.5 (2026-04-28) and stopped receiving row-level
//! updates — leaving the occupancy-trend chart silently broken.
//! Coexistence audit T3 HIGH-4 (`docs/coexistence/audit-2026-05-13.md`).

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

/// SQL for the occupancy-trend window query. Exposed as a `const` so unit
/// tests can assert it reads canonical (`ht_checkins`) rather than the demoted
/// `ht_checkins_legacy` mirror.
///
/// Occupancy window: a checkin contributes to date `d` when
/// `cin_checkin_time::date <= d < COALESCE(cin_checkout_time, cin_expected_checkout)::date`.
/// Restricted to non-cancelled rows (`cin_status IN ('active','checkedout')`).
/// Counts DISTINCT `cin_room_id` (canonical FK) — drops the legacy string
/// `cin_room_no` from the previous query.
pub(crate) const OCCUPANCY_TREND_SQL: &str = r#"
    WITH DateRange AS (
        SELECT (CURRENT_DATE - n)::date AS date_val
        FROM generate_series(0, $1 - 1) AS n
    )
    SELECT
        dr.date_val AS date,
        COUNT(DISTINCT c.cin_room_id)::int AS occupied_rooms
    FROM DateRange dr
    LEFT JOIN ht_checkins c
        ON c.cin_checkin_time::date <= dr.date_val
       AND COALESCE(c.cin_checkout_time, c.cin_expected_checkout)::date > dr.date_val
       AND c.cin_status IN ('active', 'checkedout')
    GROUP BY dr.date_val
    ORDER BY dr.date_val ASC
"#;

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

/// Read occupancy data from canonical PostgreSQL (`ht_checkins`).
async fn get_occupancy_pg(pool: &PgPool, days: i32) -> ApiResult<Vec<OccupancyData>> {
    let rows = sqlx::query(OCCUPANCY_TREND_SQL)
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

#[cfg(test)]
mod tests {
    use super::OCCUPANCY_TREND_SQL;

    /// Coexistence audit T3 HIGH-4: the occupancy trend query must read
    /// canonical `ht_checkins`, never the demoted `ht_checkins_legacy`
    /// mirror (drift-detection-only since 2026-04-28).
    #[test]
    fn reads_canonical_ht_checkins_not_legacy_mirror() {
        assert!(
            !OCCUPANCY_TREND_SQL.contains("ht_checkins_legacy"),
            "occupancy SQL must not read the demoted legacy mirror"
        );
        assert!(
            OCCUPANCY_TREND_SQL.contains("ht_checkins"),
            "occupancy SQL must read canonical ht_checkins"
        );
    }

    /// Counts distinct rooms by the canonical FK column (`cin_room_id`), not
    /// the legacy string `cin_room_no`. The legacy column doesn't exist on
    /// `ht_checkins` so a wrong reference would also break at runtime; this
    /// guards against a future regression copying the wrong column back in.
    #[test]
    fn counts_distinct_canonical_room_id_not_legacy_room_no() {
        assert!(OCCUPANCY_TREND_SQL.contains("COUNT(DISTINCT c.cin_room_id)"));
        assert!(!OCCUPANCY_TREND_SQL.contains("cin_room_no"));
    }

    /// Occupancy window must use canonical column names: `cin_checkin_time`
    /// for the start and `COALESCE(cin_checkout_time, cin_expected_checkout)`
    /// for the end. The legacy mirror used `cin_room_in` / `cin_room_out`.
    #[test]
    fn uses_canonical_time_columns_with_expected_checkout_fallback() {
        assert!(OCCUPANCY_TREND_SQL.contains("c.cin_checkin_time::date"));
        assert!(OCCUPANCY_TREND_SQL
            .contains("COALESCE(c.cin_checkout_time, c.cin_expected_checkout)::date"));
        assert!(!OCCUPANCY_TREND_SQL.contains("cin_room_in"));
        assert!(!OCCUPANCY_TREND_SQL.contains("cin_room_out"));
    }

    /// Cancelled rows must not appear in the trend.
    #[test]
    fn filters_out_cancelled_rows() {
        assert!(OCCUPANCY_TREND_SQL.contains("c.cin_status IN ('active', 'checkedout')"));
    }
}

