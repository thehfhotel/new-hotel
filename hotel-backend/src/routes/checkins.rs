//! Check-in API routes
//!
//! - GET /api/checkins - List check-ins (paginated with filters)
//!
//! Reads from PG (`ht_checkins_legacy` cache, fed by drift-reconcile + CT mappers).

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::Row;

use crate::db::PgPool;
use crate::error::ApiResult;
use crate::models::{CheckIn, CheckInsResponse, Pagination};
use crate::routes::mode::{AppState, Branch};

/// Query parameters for check-ins list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckInsQuery {
    pub status: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
    pub branch: Option<Branch>,
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 20 }

/// GET /api/checkins - List check-ins (paginated)
pub async fn list_checkins(
    State(state): State<AppState>,
    Query(params): Query<CheckInsQuery>,
) -> ApiResult<Json<CheckInsResponse>> {
    let branch = params.branch.unwrap_or_default();

    match branch {
        // Note: for the All branch with pagination, we just return the primary branch
        // as merging paginated results is complex. Frontend handles by switching branches.
        Branch::Hfhotel | Branch::All => list_checkins_pg(&state.new_pool, params).await,
        Branch::Hfville => list_checkins_pg(state.ville_pool()?, params).await,
    }
}

/// Read check-ins from PostgreSQL (ht_checkins_legacy)
async fn list_checkins_pg(
    pool: &PgPool,
    params: CheckInsQuery,
) -> ApiResult<Json<CheckInsResponse>> {
    let offset = (params.page - 1) * params.limit;

    // Build WHERE conditions with parameterized placeholders. Each filter that
    // contributes a bind value reserves the next `$N` slot; the values are
    // bound below in the same order via sqlx, eliminating SQL injection risk.
    let mut conditions: Vec<String> = Vec::new();
    let mut next_param_index: i32 = 1;

    if params.status.is_some() {
        conditions.push(format!("cin_status = ${}", next_param_index));
        next_param_index += 1;
    }

    // Date range filter: find check-ins that OVERLAP the given range.
    // A check-in overlaps if it starts before the end AND ends after the start.
    if params.start_date.is_some() {
        conditions.push(format!("cin_room_out::date >= ${}", next_param_index));
        next_param_index += 1;
    }

    if params.end_date.is_some() {
        conditions.push(format!("cin_room_in::date <= ${}", next_param_index));
        next_param_index += 1;
    }

    // Discard the final increment to keep the unused-assignment lint quiet
    // when `end_date` is the last filter; the bind chain below still relies
    // on the placeholders generated above.
    let _ = next_param_index;

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Get total count
    let count_query = format!(
        "SELECT COUNT(*)::int as total FROM ht_checkins_legacy {}",
        where_clause
    );

    let count_q = sqlx::query(&count_query);
    let count_q = match &params.status { Some(s) => count_q.bind(s), None => count_q };
    let count_q = match &params.start_date { Some(s) => count_q.bind(s), None => count_q };
    let count_q = match &params.end_date { Some(s) => count_q.bind(s), None => count_q };
    let count_rows = count_q.fetch_all(pool).await?;

    let total: i32 = count_rows
        .first()
        .and_then(|r| r.try_get::<i32, _>("total").ok())
        .unwrap_or(0);

    // Get paginated data
    let data_query = format!(
        r#"
        SELECT
            cin_no,
            cin_room_no,
            cin_room_in,
            cin_room_out,
            cin_cust_name,
            cin_status
        FROM ht_checkins_legacy
        {}
        ORDER BY cin_room_in DESC
        OFFSET {} LIMIT {}
        "#,
        where_clause, offset, params.limit
    );

    let data_q = sqlx::query(&data_query);
    let data_q = match &params.status { Some(s) => data_q.bind(s), None => data_q };
    let data_q = match &params.start_date { Some(s) => data_q.bind(s), None => data_q };
    let data_q = match &params.end_date { Some(s) => data_q.bind(s), None => data_q };
    let rows = data_q.fetch_all(pool).await?;

    let checkins: Vec<CheckIn> = rows
        .iter()
        .map(|row| CheckIn {
            cin_no: row.try_get::<String, _>("cin_no").ok(),
            cin_room_no: row.try_get::<String, _>("cin_room_no").ok(),
            cin_room_in: row.try_get::<NaiveDateTime, _>("cin_room_in").ok().map(|dt| dt.and_utc()),
            cin_room_out: row.try_get::<NaiveDateTime, _>("cin_room_out").ok().map(|dt| dt.and_utc()),
            cin_cust_name: row.try_get::<String, _>("cin_cust_name").ok(),
            cin_status: row.try_get::<String, _>("cin_status").ok(),
        })
        .collect();

    Ok(Json(CheckInsResponse {
        success: true,
        data: checkins,
        pagination: Pagination::new(params.page, params.limit, total),
    }))
}

