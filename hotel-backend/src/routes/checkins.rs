//! Check-in API routes
//!
//! - GET /api/checkins - List check-ins (paginated with filters)

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::db::DbPool;
use crate::error::ApiResult;
use crate::models::{CheckIn, CheckInsResponse, Pagination};

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
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 20 }

/// GET /api/checkins - List check-ins (paginated)
pub async fn list_checkins(
    State(pool): State<DbPool>,
    Query(params): Query<CheckInsQuery>,
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
    if let Some(ref start_date) = params.start_date {
        conditions.push(format!("Cin_Room_Out >= '{}'", start_date.replace('\'', "''")));
    }

    if let Some(ref end_date) = params.end_date {
        conditions.push(format!("Cin_Room_In <= '{}'", end_date.replace('\'', "''")));
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
            cin_room_in: row.get::<NaiveDateTime, _>("Cin_Room_In"),
            cin_room_out: row.get::<NaiveDateTime, _>("Cin_Room_Out"),
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
