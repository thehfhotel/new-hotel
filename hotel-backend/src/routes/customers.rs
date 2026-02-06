//! Customer API routes
//!
//! - GET /api/customers - List customers (search/sort/pagination)
//! - GET /api/customers/:id/bookings - Get customer's booking history
//! - GET /api/customers/:id/stats - Get customer statistics

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::db::DbPool;
use crate::error::ApiResult;
use crate::models::{
    Customer, CustomerBooking, CustomerBookingsResponse, CustomerStats, CustomerStatsResponse,
    CustomersResponse,
};

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
}

fn default_page() -> i32 { 1 }
fn default_limit() -> i32 { 20 }

/// GET /api/customers - List customers with search, sort, and pagination
pub async fn list_customers(
    State(pool): State<DbPool>,
    Query(params): Query<CustomersQuery>,
) -> ApiResult<Json<CustomersResponse>> {
    let mut conn = pool.get().await?;

    let offset = (params.page - 1) * params.limit;
    let sort_order = params
        .sort_order
        .as_ref()
        .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
        .unwrap_or("ASC");

    // Map frontend column names to SQL columns
    let order_by_column = match params.sort_by.as_deref() {
        Some("name") => "c.Cust_name",
        Some("type") => "c.Cust_Type",
        Some("phone") => "c.Cust_Add_tel",
        Some("lastVisit") if params.include_last_visit => "lv.lastVisit",
        _ => "CAST(SUBSTRING(c.Cust_no, 2, LEN(c.Cust_no)-1) AS INT)", // id default
    };

    // Build ORDER BY clause with nulls last for lastVisit
    let order_by_clause = if params.sort_by.as_deref() == Some("lastVisit") && params.include_last_visit {
        format!(
            "CASE WHEN lv.lastVisit IS NULL THEN 1 ELSE 0 END, {} {}",
            order_by_column, sort_order
        )
    } else {
        format!("{} {}", order_by_column, sort_order)
    };

    // Build WHERE clause
    let where_clause = if params.search.is_some() {
        r#" WHERE c.Cust_name LIKE @P1
           OR c.Cust_Add_tel LIKE @P1
           OR c.Cust_IDcard LIKE @P1
           OR CAST(c.Cust_no AS NVARCHAR) LIKE @P1"#
    } else {
        ""
    };

    // Count query
    let count_query = format!(
        "SELECT COUNT(*) as total FROM View_Customers c {}",
        where_clause
    );

    let count_rows = if let Some(ref search) = params.search {
        let search_pattern = format!("%{}%", search);
        conn.query(&count_query, &[&search_pattern.as_str()])
            .await?
            .into_first_result()
            .await?
    } else {
        conn.simple_query(&count_query)
            .await?
            .into_first_result()
            .await?
    };

    let total: i32 = count_rows
        .first()
        .and_then(|r| r.get::<i32, _>("total"))
        .unwrap_or(0);

    // Build data query with optional lastVisit JOIN
    let last_visit_select = if params.include_last_visit {
        ",\n        lv.lastVisit"
    } else {
        ""
    };

    let last_visit_join = if params.include_last_visit {
        r#"
      LEFT JOIN (
        SELECT Book_Cust_ID, MAX(Book_Date_out) as lastVisit
        FROM View_Booking_Ds
        GROUP BY Book_Cust_ID
      ) lv ON c.Cust_no = lv.Book_Cust_ID"#
    } else {
        ""
    };

    let data_query = format!(
        r#"
        SELECT
            c.Cust_no,
            c.Cust_name,
            c.Cust_Type,
            c.Cust_Add_tel,
            c.Cust_IDcard,
            c.C_Address{}
        FROM View_Customers c{}
        {}
        ORDER BY {}
        OFFSET {} ROWS FETCH NEXT {} ROWS ONLY
        "#,
        last_visit_select,
        last_visit_join,
        where_clause,
        order_by_clause,
        offset,
        params.limit
    );

    let rows = if let Some(ref search) = params.search {
        let search_pattern = format!("%{}%", search);
        conn.query(&data_query, &[&search_pattern.as_str()])
            .await?
            .into_first_result()
            .await?
    } else {
        conn.simple_query(&data_query)
            .await?
            .into_first_result()
            .await?
    };

    let customers: Vec<Customer> = rows
        .iter()
        .map(|row| Customer {
            id: row.get::<&str, _>("Cust_no").unwrap_or_default().to_string(),
            name: row.get::<&str, _>("Cust_name").map(String::from),
            customer_type: row.get::<&str, _>("Cust_Type").map(String::from),
            phone: row.get::<&str, _>("Cust_Add_tel").map(String::from),
            id_card: row.get::<&str, _>("Cust_IDcard").map(String::from),
            address: row.get::<&str, _>("C_Address").map(String::from),
            last_visit: if params.include_last_visit {
                row.get::<NaiveDateTime, _>("lastVisit").map(|dt| dt.and_utc())
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

/// GET /api/customers/:id/bookings - Get customer's booking history
pub async fn get_customer_bookings(
    State(pool): State<DbPool>,
    Path(cust_id): Path<String>,
) -> ApiResult<Json<CustomerBookingsResponse>> {
    let mut conn = pool.get().await?;

    let rows = conn
        .query(
            r#"
            SELECT
                Book_No,
                Book_Room_Type,
                Book_Date_in,
                Book_Date_out,
                Book_Status
            FROM View_Booking_Ds
            WHERE Book_Cust_ID = @P1
            ORDER BY Book_Date_in DESC
            "#,
            &[&cust_id],
        )
        .await?
        .into_first_result()
        .await?;

    let bookings: Vec<CustomerBooking> = rows
        .iter()
        .map(|row| {
            let status_code = row.get::<i32, _>("Book_Status");
            let status = match status_code {
                Some(1) => "confirmed",
                Some(2) => "completed",
                _ => "pending",
            };

            CustomerBooking {
                id: row.get::<&str, _>("Book_No").unwrap_or_default().to_string(),
                room_number: "-".to_string(),
                room_type: row
                    .get::<&str, _>("Book_Room_Type")
                    .unwrap_or("-")
                    .to_string(),
                check_in_date: row.try_get::<NaiveDateTime, _>("Book_Date_in").ok().flatten().map(|dt| dt.and_utc()),
                check_out_date: row.try_get::<NaiveDateTime, _>("Book_Date_out").ok().flatten().map(|dt| dt.and_utc()),
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

/// GET /api/customers/:id/stats - Get customer statistics
pub async fn get_customer_stats(
    State(pool): State<DbPool>,
    Path(cust_id): Path<String>,
) -> ApiResult<Json<CustomerStatsResponse>> {
    let mut conn = pool.get().await?;

    // Query 1: Total bookings and booking stats
    let booking_stats_rows = conn
        .query(
            r#"
            SELECT
                COUNT(*) as totalBookings,
                MIN(Book_Date_in) as firstVisit,
                MAX(Book_Date_out) as lastVisit,
                AVG(CAST(DATEDIFF(day, Book_Date_in, Book_Date_out) AS FLOAT)) as avgStayDays
            FROM View_Booking_Ds
            WHERE Book_Cust_ID = @P1
            "#,
            &[&cust_id],
        )
        .await?
        .into_first_result()
        .await?;

    // Query 2: Total stays (check-ins)
    let stays_rows = conn
        .query(
            r#"
            SELECT COUNT(*) as totalStays
            FROM View_CheckIn_Ds
            WHERE Cin_cust_no = @P1
            "#,
            &[&cust_id],
        )
        .await?
        .into_first_result()
        .await?;

    // Query 3: Favorite room type (most booked)
    let favorite_room_rows = conn
        .query(
            r#"
            SELECT TOP 1 Book_Room_Type, COUNT(*) as cnt
            FROM View_Booking_Ds
            WHERE Book_Cust_ID = @P1 AND Book_Room_Type IS NOT NULL
            GROUP BY Book_Room_Type
            ORDER BY cnt DESC
            "#,
            &[&cust_id],
        )
        .await?
        .into_first_result()
        .await?;

    let booking_stats = booking_stats_rows.first();
    let stays_stats = stays_rows.first();
    let favorite_room = favorite_room_rows.first();

    let avg_stay_days = booking_stats
        .and_then(|r| r.get::<f64, _>("avgStayDays"))
        .map(|v| (v * 10.0).round() / 10.0);

    let stats = CustomerStats {
        total_bookings: booking_stats
            .and_then(|r| r.get::<i32, _>("totalBookings"))
            .unwrap_or(0),
        total_stays: stays_stats
            .and_then(|r| r.get::<i32, _>("totalStays"))
            .unwrap_or(0),
        first_visit: booking_stats.and_then(|r| r.get::<NaiveDateTime, _>("firstVisit")).map(|dt| dt.and_utc()),
        last_visit: booking_stats.and_then(|r| r.get::<NaiveDateTime, _>("lastVisit")).map(|dt| dt.and_utc()),
        favorite_room_type: favorite_room
            .and_then(|r| r.get::<&str, _>("Book_Room_Type").map(String::from)),
        avg_stay_days,
    };

    Ok(Json(CustomerStatsResponse {
        success: true,
        stats,
    }))
}
