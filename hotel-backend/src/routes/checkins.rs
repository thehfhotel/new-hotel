//! Check-in API routes
//!
//! - GET /api/checkins - List check-ins (paginated with filters)
//!
//! Reads canonical PG tables (`ht_checkins` JOIN `ht_rooms_new` + `ht_customers`).
//! The legacy `ht_checkins_legacy` mirror was demoted to hash-only drift detection
//! by the Phase 5.5 cutover (2026-04-28), so its data columns are NULL for all
//! rows inserted after that date — reading it left "Recent Activity" showing
//! "ไม่ระบุ / ห้อง / 1 ม.ค. - 1 ม.ค." for every check-in.

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

/// Read check-ins from canonical PG (`ht_checkins` JOIN `ht_rooms_new` + `ht_customers`).
///
/// The output uses legacy field names (`Cin_Room_No`, `Cin_cust_name`, etc.)
/// via the `CheckIn` model's serde renames so the frontend contract is
/// preserved. `Cin_Room_Out` falls back to `cin_expected_checkout` when the
/// guest hasn't actually checked out yet — otherwise active stays would render
/// as epoch (Jan 1) in the UI.
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
        conditions.push(format!("c.cin_status = ${}", next_param_index));
        next_param_index += 1;
    }

    // Date range filter: find check-ins that OVERLAP the given range.
    // For active stays without an actual checkout, fall back to the planned
    // checkout date so the row still participates in overlap checks.
    if params.start_date.is_some() {
        conditions.push(format!(
            "COALESCE(c.cin_checkout_time::date, c.cin_expected_checkout) >= ${}",
            next_param_index
        ));
        next_param_index += 1;
    }

    if params.end_date.is_some() {
        conditions.push(format!(
            "c.cin_checkin_time::date <= ${}",
            next_param_index
        ));
        next_param_index += 1;
    }

    let _ = next_param_index;

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Total count — same WHERE clause, no JOIN needed (status + dates are on
    // ht_checkins directly).
    let count_query = format!(
        "SELECT COUNT(*)::int as total FROM ht_checkins c {}",
        where_clause
    );

    let count_q = sqlx::query(sqlx::AssertSqlSafe(&*count_query));
    let count_q = match &params.status { Some(s) => count_q.bind(s), None => count_q };
    let count_q = match &params.start_date { Some(s) => count_q.bind(s), None => count_q };
    let count_q = match &params.end_date { Some(s) => count_q.bind(s), None => count_q };
    let count_rows = count_q.fetch_all(pool).await?;

    let total: i32 = count_rows
        .first()
        .and_then(|r| r.try_get::<i32, _>("total").ok())
        .unwrap_or(0);

    // Paginated data. Customer name is built from first + last, trimmed,
    // because cust_lastname is nullable. NULLIF/TRIM keeps the result NULL
    // when both halves are empty so the frontend's "ไม่ระบุ" fallback fires.
    let data_query = format!(
        r#"
        SELECT
            c.cin_no                                AS cin_no,
            COALESCE(r.room_no, c.legacy_room_no)   AS cin_room_no,
            c.cin_checkin_time                      AS cin_room_in,
            COALESCE(c.cin_checkout_time, c.cin_expected_checkout::timestamp) AS cin_room_out,
            NULLIF(TRIM(BOTH ' ' FROM
                COALESCE(cu.cust_firstname, '') || ' ' || COALESCE(cu.cust_lastname, '')
            ), '')                                  AS cin_cust_name,
            c.cin_status                            AS cin_status
        FROM ht_checkins c
        LEFT JOIN ht_rooms_new r ON r.room_id = c.cin_room_id
        LEFT JOIN ht_customers cu ON cu.cust_id = c.cin_cust_id
        {}
        ORDER BY c.cin_checkin_time DESC
        OFFSET {} LIMIT {}
        "#,
        where_clause, offset, params.limit
    );

    let data_q = sqlx::query(sqlx::AssertSqlSafe(&*data_query));
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

