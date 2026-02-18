//! Check-in API routes
//!
//! - GET /api/checkins - List check-ins (paginated with filters)
//!
//! Reads from PostgreSQL (ht_checkins_legacy) by default.
//! Set LEGACY_READ_SOURCE=sqlserver to read from SQL Server (View_CheckIn_Ds).

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::Row;

use crate::db::{DbPool, PgPool};
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

/// Determine whether to read from PostgreSQL or SQL Server
fn use_pg_source() -> bool {
    match std::env::var("LEGACY_READ_SOURCE") {
        Ok(val) => val.to_lowercase() != "sqlserver",
        Err(_) => true, // default to PG
    }
}

/// GET /api/checkins - List check-ins (paginated)
pub async fn list_checkins(
    State(state): State<AppState>,
    Query(params): Query<CheckInsQuery>,
) -> ApiResult<Json<CheckInsResponse>> {
    let branch = params.branch.unwrap_or_default();

    match branch {
        Branch::Hfhotel => {
            if use_pg_source() {
                list_checkins_pg(&state.new_pool, params).await
            } else {
                list_checkins_sqlserver(&state.legacy_pool, params).await
            }
        }
        Branch::Hfville => {
            list_checkins_pg(state.ville_pool()?, params).await
        }
        Branch::All => {
            let hf_result = if use_pg_source() {
                list_checkins_pg(&state.new_pool, params).await?
            } else {
                list_checkins_sqlserver(&state.legacy_pool, params).await?
            };
            // Note: for All branch with pagination, we just return the primary branch
            // as merging paginated results is complex. Frontend handles by switching branches.
            Ok(hf_result)
        }
    }
}

/// Read check-ins from PostgreSQL (ht_checkins_legacy)
async fn list_checkins_pg(
    pool: &PgPool,
    params: CheckInsQuery,
) -> ApiResult<Json<CheckInsResponse>> {
    let offset = (params.page - 1) * params.limit;

    // Build WHERE conditions with direct value interpolation
    let mut conditions: Vec<String> = Vec::new();

    if let Some(ref status) = params.status {
        conditions.push(format!("cin_status = '{}'", status.replace('\'', "''")));
    }

    // Date range filter: find check-ins that OVERLAP the given range
    // A check-in overlaps if it starts before the end AND ends after the start
    if let Some(ref start_date) = params.start_date {
        conditions.push(format!("cin_room_out::date >= '{}'", start_date.replace('\'', "''")));
    }

    if let Some(ref end_date) = params.end_date {
        conditions.push(format!("cin_room_in::date <= '{}'", end_date.replace('\'', "''")));
    }

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

    let count_rows = sqlx::query(&count_query)
        .fetch_all(pool)
        .await?;

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

    let rows = sqlx::query(&data_query)
        .fetch_all(pool)
        .await?;

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

/// Read check-ins from SQL Server (View_CheckIn_Ds) - legacy fallback
async fn list_checkins_sqlserver(
    pool: &DbPool,
    params: CheckInsQuery,
) -> ApiResult<Json<CheckInsResponse>> {
    let mut conn = pool.get().await?;

    let offset = (params.page - 1) * params.limit;

    // Build WHERE conditions with direct value interpolation
    let mut conditions: Vec<String> = Vec::new();

    if let Some(ref status) = params.status {
        conditions.push(format!("Cin_status = '{}'", status.replace('\'', "''")));
    }

    // Date range filter: find check-ins that OVERLAP the given range
    // A check-in overlaps if it starts before the end AND ends after the start
    // Use CAST to compare dates without time component
    if let Some(ref start_date) = params.start_date {
        conditions.push(format!("CAST(Cin_Room_Out AS DATE) >= '{}'", start_date.replace('\'', "''")));
    }

    if let Some(ref end_date) = params.end_date {
        conditions.push(format!("CAST(Cin_Room_In AS DATE) <= '{}'", end_date.replace('\'', "''")));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Get total count
    let count_query = format!(
        "SELECT COUNT(*) as total FROM View_CheckIn_Ds {}",
        where_clause
    );

    let count_rows = conn
        .simple_query(&count_query)
        .await?
        .into_first_result()
        .await?;

    let total: i32 = count_rows
        .first()
        .and_then(|r| r.get::<i32, _>("total"))
        .unwrap_or(0);

    // Get paginated data
    let data_query = format!(
        r#"
        SELECT
            Cin_no,
            Cin_Room_No,
            Cin_Room_In,
            Cin_Room_Out,
            Cin_cust_name,
            Cin_status
        FROM View_CheckIn_Ds
        {}
        ORDER BY Cin_Room_In DESC
        OFFSET {} ROWS FETCH NEXT {} ROWS ONLY
        "#,
        where_clause, offset, params.limit
    );

    let rows = conn
        .simple_query(&data_query)
        .await?
        .into_first_result()
        .await?;

    let checkins: Vec<CheckIn> = rows
        .iter()
        .map(|row| CheckIn {
            cin_no: row.get::<&str, _>("Cin_no").map(String::from),
            cin_room_no: row.get::<&str, _>("Cin_Room_No").map(String::from),
            cin_room_in: row.get::<NaiveDateTime, _>("Cin_Room_In").map(|dt| dt.and_utc()),
            cin_room_out: row.get::<NaiveDateTime, _>("Cin_Room_Out").map(|dt| dt.and_utc()),
            cin_cust_name: row.get::<&str, _>("Cin_cust_name").map(String::from),
            cin_status: row.get::<&str, _>("Cin_status").map(String::from),
        })
        .collect();

    Ok(Json(CheckInsResponse {
        success: true,
        data: checkins,
        pagination: Pagination::new(params.page, params.limit, total),
    }))
}
