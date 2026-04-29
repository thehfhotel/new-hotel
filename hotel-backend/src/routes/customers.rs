//! Customer API routes
//!
//! - GET /api/customers - List customers (search/sort/pagination)
//! - GET /api/customers/:id/bookings - Get customer's booking history
//! - GET /api/customers/:id/stats - Get customer statistics
//!
//! Reads from PG (`ht_customers_legacy` / `ht_bookings_legacy` / `ht_checkins_legacy`
//! cache, fed by drift-reconcile + CT mappers).

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::Row;

use crate::db::PgPool;
use crate::error::ApiResult;
use crate::models::{
    Customer, CustomerBooking, CustomerBookingsResponse, CustomerStats, CustomerStatsResponse,
    CustomersResponse,
};
use crate::routes::mode::{AppState, Branch};

/// Query parameters for customers list
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomersQuery {
    pub search: Option<String>,
    #[serde(default = "default_page")]
    pub page: i32,
    #[serde(default = "default_limit")]
    pub limit: i32,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    #[serde(default)]
    pub include_last_visit: bool,
    pub branch: Option<Branch>,
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 20 }

// ---------------------------------------------------------------------------
// GET /api/customers
// ---------------------------------------------------------------------------

/// GET /api/customers - List customers with search, sort, and pagination
///
/// Reads from PG (`ht_customers_legacy` cache, fed by drift-reconcile + CT mappers).
pub async fn list_customers(
    State(state): State<AppState>,
    Query(params): Query<CustomersQuery>,
) -> ApiResult<Json<CustomersResponse>> {
    let branch = params.branch.unwrap_or_default();

    match branch {
        Branch::Hfhotel | Branch::All => list_customers_pg(&state.new_pool, &params).await,
        Branch::Hfville => list_customers_pg(state.ville_pool()?, &params).await,
    }
}

/// PostgreSQL implementation of list_customers
async fn list_customers_pg(
    pool: &PgPool,
    params: &CustomersQuery,
) -> ApiResult<Json<CustomersResponse>> {
    let offset = (params.page - 1) * params.limit;
    let sort_order = params
        .sort_order
        .as_ref()
        .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
        .unwrap_or("ASC");

    // Map frontend column names to PG columns
    let order_by_column = match params.sort_by.as_deref() {
        Some("name") => "c.cust_name",
        Some("type") => "c.cust_type",
        Some("phone") => "c.cust_phone",
        Some("lastVisit") if params.include_last_visit => "lv.last_visit",
        _ => "c.cust_no",
    };

    // Build ORDER BY clause with nulls last for lastVisit
    let order_by_clause = if params.sort_by.as_deref() == Some("lastVisit") && params.include_last_visit {
        format!(
            "CASE WHEN lv.last_visit IS NULL THEN 1 ELSE 0 END, {} {}",
            order_by_column, sort_order
        )
    } else {
        format!("{} {}", order_by_column, sort_order)
    };

    // Build WHERE clause
    let where_clause = if params.search.is_some() {
        r#" WHERE c.cust_name ILIKE $1
           OR c.cust_phone ILIKE $1
           OR c.cust_idcard ILIKE $1
           OR c.cust_no ILIKE $1"#
    } else {
        ""
    };

    // Count query
    let count_query = format!(
        "SELECT COUNT(*)::int AS total FROM ht_customers_legacy c {}",
        where_clause
    );

    let total: i32 = if let Some(ref search) = params.search {
        let search_pattern = format!("%{}%", search);
        sqlx::query(&count_query)
            .bind(&search_pattern)
            .fetch_one(pool)
            .await?
            .get("total")
    } else {
        sqlx::query(&count_query)
            .fetch_one(pool)
            .await?
            .get("total")
    };

    // Build data query with optional lastVisit JOIN
    let last_visit_select = if params.include_last_visit {
        ",\n        lv.last_visit"
    } else {
        ""
    };

    let last_visit_join = if params.include_last_visit {
        r#"
      LEFT JOIN (
        SELECT book_cust_id, MAX(book_date_out) AS last_visit
        FROM ht_bookings_legacy
        GROUP BY book_cust_id
      ) lv ON c.cust_no = lv.book_cust_id"#
    } else {
        ""
    };

    // For the data query, bind index depends on whether search is used
    // If search is present: $1 = search pattern, $2 = limit, $3 = offset
    // If no search:         $1 = limit, $2 = offset
    let data_query = if params.search.is_some() {
        format!(
            r#"
            SELECT
                c.cust_no,
                c.cust_name,
                c.cust_type,
                c.cust_phone,
                c.cust_idcard,
                c.cust_address{}
            FROM ht_customers_legacy c{}
            {}
            ORDER BY {}
            LIMIT $2 OFFSET $3
            "#,
            last_visit_select,
            last_visit_join,
            where_clause,
            order_by_clause,
        )
    } else {
        format!(
            r#"
            SELECT
                c.cust_no,
                c.cust_name,
                c.cust_type,
                c.cust_phone,
                c.cust_idcard,
                c.cust_address{}
            FROM ht_customers_legacy c{}
            ORDER BY {}
            LIMIT $1 OFFSET $2
            "#,
            last_visit_select,
            last_visit_join,
            order_by_clause,
        )
    };

    let rows = if let Some(ref search) = params.search {
        let search_pattern = format!("%{}%", search);
        sqlx::query(&data_query)
            .bind(&search_pattern)
            .bind(params.limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
    } else {
        sqlx::query(&data_query)
            .bind(params.limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
    };

    let customers: Vec<Customer> = rows
        .iter()
        .map(|row| Customer {
            id: row.get::<String, _>("cust_no"),
            name: row.get::<Option<String>, _>("cust_name"),
            customer_type: row.get::<Option<String>, _>("cust_type"),
            phone: row.get::<Option<String>, _>("cust_phone"),
            id_card: row.get::<Option<String>, _>("cust_idcard"),
            address: row.get::<Option<String>, _>("cust_address"),
            last_visit: if params.include_last_visit {
                row.get::<Option<NaiveDateTime>, _>("last_visit").map(|dt| dt.and_utc())
            } else {
                None
            },
        })
        .collect();

    let total_pages = (total as f64 / params.limit as f64).ceil() as i32;

    Ok(Json(CustomersResponse {
        success: true,
        customers,
        total,
        page: params.page,
        limit: params.limit,
        total_pages,
    }))
}

// ---------------------------------------------------------------------------
// GET /api/customers/:id/bookings
// ---------------------------------------------------------------------------

/// GET /api/customers/:id/bookings - Get customer's booking history
///
/// Reads from PG (`ht_bookings_legacy` cache, fed by drift-reconcile + CT mappers).
pub async fn get_customer_bookings(
    State(state): State<AppState>,
    Path(cust_id): Path<String>,
) -> ApiResult<Json<CustomerBookingsResponse>> {
    get_customer_bookings_pg(&state.new_pool, &cust_id).await
}

/// PostgreSQL implementation of get_customer_bookings
async fn get_customer_bookings_pg(
    pool: &PgPool,
    cust_id: &str,
) -> ApiResult<Json<CustomerBookingsResponse>> {
    let rows = sqlx::query(
        r#"
        SELECT
            book_no,
            book_room_type,
            book_room_no,
            book_date_in,
            book_date_out,
            book_status
        FROM ht_bookings_legacy
        WHERE book_cust_id = $1
        ORDER BY book_date_in DESC
        "#,
    )
    .bind(cust_id)
    .fetch_all(pool)
    .await?;

    let bookings: Vec<CustomerBooking> = rows
        .iter()
        .map(|row| {
            let status_code: Option<i32> = row.get("book_status");
            let status = match status_code {
                Some(1) => "confirmed",
                Some(2) => "completed",
                _ => "pending",
            };

            CustomerBooking {
                id: row.get::<String, _>("book_no"),
                room_number: row
                    .get::<Option<String>, _>("book_room_no")
                    .unwrap_or_else(|| "-".to_string()),
                room_type: row
                    .get::<Option<String>, _>("book_room_type")
                    .unwrap_or_else(|| "-".to_string()),
                check_in_date: row.get::<Option<NaiveDateTime>, _>("book_date_in").map(|dt| dt.and_utc()),
                check_out_date: row.get::<Option<NaiveDateTime>, _>("book_date_out").map(|dt| dt.and_utc()),
                status: status.to_string(),
                total_amount: 0.0,
            }
        })
        .collect();

    Ok(Json(CustomerBookingsResponse {
        success: true,
        bookings,
    }))
}

// ---------------------------------------------------------------------------
// GET /api/customers/:id/stats
// ---------------------------------------------------------------------------

/// GET /api/customers/:id/stats - Get customer statistics
///
/// Reads from PG (`ht_bookings_legacy` + `ht_checkins_legacy` cache, fed by
/// drift-reconcile + CT mappers).
pub async fn get_customer_stats(
    State(state): State<AppState>,
    Path(cust_id): Path<String>,
) -> ApiResult<Json<CustomerStatsResponse>> {
    get_customer_stats_pg(&state.new_pool, &cust_id).await
}

/// PostgreSQL implementation of get_customer_stats
async fn get_customer_stats_pg(
    pool: &PgPool,
    cust_id: &str,
) -> ApiResult<Json<CustomerStatsResponse>> {
    // Query 1: Total bookings and booking stats
    let booking_stats_row = sqlx::query(
        r#"
        SELECT
            COUNT(*)::int AS total_bookings,
            MIN(book_date_in) AS first_visit,
            MAX(book_date_out) AS last_visit,
            AVG((book_date_out::date - book_date_in::date)::float8) AS avg_stay_days
        FROM ht_bookings_legacy
        WHERE book_cust_id = $1
        "#,
    )
    .bind(cust_id)
    .fetch_one(pool)
    .await?;

    // Query 2: Total stays (check-ins)
    let stays_row = sqlx::query(
        r#"
        SELECT COUNT(*)::int AS total_stays
        FROM ht_checkins_legacy
        WHERE cin_cust_no = $1
        "#,
    )
    .bind(cust_id)
    .fetch_one(pool)
    .await?;

    // Query 3: Favorite room type (most booked)
    let favorite_room_row = sqlx::query(
        r#"
        SELECT book_room_type, COUNT(*) AS cnt
        FROM ht_bookings_legacy
        WHERE book_cust_id = $1 AND book_room_type IS NOT NULL
        GROUP BY book_room_type
        ORDER BY cnt DESC
        LIMIT 1
        "#,
    )
    .bind(cust_id)
    .fetch_optional(pool)
    .await?;

    let total_bookings: i32 = booking_stats_row.get("total_bookings");
    let first_visit: Option<NaiveDateTime> = booking_stats_row.get("first_visit");
    let last_visit: Option<NaiveDateTime> = booking_stats_row.get("last_visit");
    let avg_stay_days: Option<f64> = booking_stats_row.get("avg_stay_days");
    let total_stays: i32 = stays_row.get("total_stays");

    let favorite_room_type = favorite_room_row
        .as_ref()
        .map(|r| r.get::<String, _>("book_room_type"));

    let stats = CustomerStats {
        total_bookings,
        total_stays,
        first_visit: first_visit.map(|dt| dt.and_utc()),
        last_visit: last_visit.map(|dt| dt.and_utc()),
        favorite_room_type,
        avg_stay_days: avg_stay_days.map(|v| (v * 10.0).round() / 10.0),
    };

    Ok(Json(CustomerStatsResponse {
        success: true,
        stats,
    }))
}

