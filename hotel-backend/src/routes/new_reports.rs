//! New Report API routes for HotelNew database
//!
//! - GET /api/reports/revenue - Revenue report with period grouping
//! - GET /api/reports/occupancy - Occupancy statistics
//! - GET /api/reports/revenue-by-room-type - Revenue breakdown by room type
//! - GET /api/reports/vat-summary - Output-tax (VAT) period summary
//! - GET /api/reports/sales-by-customer - Revenue grouped by customer
//!
//! All handlers are branch-aware: HF Ville reads its own logical PG database
//! (`ville_pool`); HF Hotel (and the `all` dashboard view) read `new_pool`.
//! The site-scoping is connection-level, not row-level — see the canonical
//! PG split note in CLAUDE.md.

use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use super::mode::{AppState, Branch};
use crate::error::{ApiError, ApiResult};

/// SQL fragment computing a check-in's attributed revenue: the recorded folio
/// total when present, otherwise `rate × nights` derived from the stay dates.
/// Shared verbatim across every report so the revenue, room-type, VAT and
/// sales-by-customer figures always agree. The fragment references the
/// `ht_checkins` alias `ci`, so every query that interpolates it must alias the
/// table `ci`. It is a compile-time constant (no user input) — the call sites
/// inject it via `format!` and wrap the result in [`sqlx::AssertSqlSafe`].
const CHECKIN_REVENUE_EXPR: &str = "COALESCE(ci.cin_total_amount, \
    ci.cin_rate_per_night * (COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout)::date \
    - ci.cin_checkin_time::date))";

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

fn default_group_by_month() -> GroupBy {
    GroupBy::Month
}

/// Query parameters for the VAT / output-tax summary.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VatSummaryQuery {
    pub from: String,
    pub to: String,
    /// Period grouping for the breakdown table. Defaults to `month` because the
    /// Thai output-tax filing (ภ.พ.30 / PP30) is monthly — the common case for
    /// this report — but `day`/`week` are accepted for ad-hoc views.
    #[serde(default = "default_group_by_month")]
    pub group_by: GroupBy,
    /// Branch selector: 'hfhotel' (default) | 'hfville'. Site data lives in
    /// separate logical PG databases, so the pool selection is the site filter.
    pub branch: Option<Branch>,
}

/// One period row of the VAT summary. `gross` is the VAT-inclusive revenue;
/// `before_vat` is the net (ex-VAT) base; `vat` is the output tax.
#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VatPeriodRow {
    pub period: String,
    pub gross: f64,
    pub before_vat: f64,
    pub vat: f64,
    pub bookings: i32,
}

/// Response for the VAT / output-tax summary.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VatSummaryResponse {
    pub success: bool,
    /// The branch-scoped VAT percent applied (from `ht_settings.vat_percent`).
    /// HF Hotel runs 0; HF Ville may differ.
    pub vat_percent: i32,
    /// Period totals across the whole range.
    pub gross: f64,
    pub before_vat: f64,
    pub vat: f64,
    pub data: Vec<VatPeriodRow>,
}

/// Split each period's gross (VAT-inclusive) revenue into the net base and the
/// output tax, reusing the exact inclusive-split math the receipt + iHOTEL
/// writeback use (`writeback::format::vat_inclusive_split`) so the report ties
/// out to the printed receipts to the satang. Returns the per-period rows plus
/// the `(gross, before_vat, vat)` grand totals. Pure — unit-tested below.
fn aggregate_vat_periods(
    rows: Vec<(String, f64, i32)>,
    vat_percent: i32,
) -> (Vec<VatPeriodRow>, f64, f64, f64) {
    let mut data = Vec::with_capacity(rows.len());
    let (mut total_gross, mut total_before, mut total_vat) = (0.0_f64, 0.0_f64, 0.0_f64);
    for (period, gross, bookings) in rows {
        let (before_vat, vat) = crate::writeback::format::vat_inclusive_split(gross, vat_percent);
        total_gross += gross;
        total_before += before_vat;
        total_vat += vat;
        data.push(VatPeriodRow {
            period,
            gross,
            before_vat,
            vat,
            bookings,
        });
    }
    (data, total_gross, total_before, total_vat)
}

/// GET /api/reports/vat-summary - Output-tax (VAT) period summary.
///
/// VAT is treated as inclusive in the headline revenue: `before = gross / (1 +
/// vat%/100)`, `vat = gross - before` (per period, then rounded). When
/// `vat_percent = 0` (HF Hotel today) the split is a no-op: `before == gross`
/// and `vat == 0`. Branch-aware — the rate is read from `ht_settings` on the
/// SELECTED site's pool, so HF Ville can run a different percent than HF Hotel.
pub async fn get_vat_summary(
    State(state): State<AppState>,
    Query(params): Query<VatSummaryQuery>,
) -> ApiResult<Json<VatSummaryResponse>> {
    let pool = match params.branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool()?,
        Branch::Hfhotel | Branch::All => &state.new_pool,
    };

    if params.from.is_empty() || params.to.is_empty() {
        return Err(ApiError::BadRequest(
            "Both 'from' and 'to' dates are required".to_string(),
        ));
    }

    // Branch-scoped VAT rate from `ht_settings` on the resolved pool.
    let vat_percent = crate::repository::settings::get_vat_percent(pool).await;

    let date_format = params.group_by.as_sql_format();
    // Controlled fragments only (the GroupBy enum's fixed format string + the
    // CHECKIN_REVENUE_EXPR constant); the date range is bound, not interpolated.
    let query = format!(
        r#"
        SELECT
            TO_CHAR(ci.cin_checkin_time, '{fmt}') AS period,
            COALESCE(SUM({rev}), 0)::float8 AS gross,
            COUNT(*)::int AS bookings
        FROM ht_checkins ci
        WHERE ci.cin_status = 'checkedout'
          AND ci.cin_checkin_time::date >= $1::date
          AND ci.cin_checkin_time::date <= $2::date
        GROUP BY period
        ORDER BY period ASC
        "#,
        fmt = date_format,
        rev = CHECKIN_REVENUE_EXPR,
    );

    let rows = sqlx::query(sqlx::AssertSqlSafe(&*query))
        .bind(&params.from)
        .bind(&params.to)
        .fetch_all(pool)
        .await?;

    let period_rows: Vec<(String, f64, i32)> = rows
        .iter()
        .map(|row| {
            (
                row.try_get::<String, _>("period").unwrap_or_default(),
                row.try_get::<f64, _>("gross").unwrap_or(0.0),
                row.try_get::<i32, _>("bookings").unwrap_or(0),
            )
        })
        .collect();

    let (data, gross, before_vat, vat) = aggregate_vat_periods(period_rows, vat_percent);

    Ok(Json(VatSummaryResponse {
        success: true,
        vat_percent,
        gross,
        before_vat,
        vat,
        data,
    }))
}

/// Query parameters for the sales-by-customer report.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SalesByCustomerQuery {
    pub from: String,
    pub to: String,
    /// Cap on the number of customers returned (top spenders first). Defaults
    /// to 50 and is clamped to 1..=1000 so the report can't be turned into an
    /// unbounded scan via the query string.
    pub limit: Option<i64>,
    /// Branch selector: 'hfhotel' (default) | 'hfville'.
    pub branch: Option<Branch>,
}

/// One customer's spend over the requested range.
#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSales {
    pub customer_id: i32,
    pub customer_name: String,
    pub phone: Option<String>,
    pub checkins: i32,
    pub revenue: f64,
}

/// Response for the sales-by-customer report.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SalesByCustomerResponse {
    pub success: bool,
    /// Sum of `revenue` across the returned rows (the displayed total).
    pub total_revenue: f64,
    pub data: Vec<CustomerSales>,
}

/// GET /api/reports/sales-by-customer - Revenue grouped by customer.
///
/// Sums the same attributed revenue used elsewhere (`CHECKIN_REVENUE_EXPR`) per
/// customer over the date range, ordered by spend descending. Branch-aware.
pub async fn get_sales_by_customer(
    State(state): State<AppState>,
    Query(params): Query<SalesByCustomerQuery>,
) -> ApiResult<Json<SalesByCustomerResponse>> {
    let pool = match params.branch.unwrap_or_default() {
        Branch::Hfville => state.ville_pool()?,
        Branch::Hfhotel | Branch::All => &state.new_pool,
    };

    if params.from.is_empty() || params.to.is_empty() {
        return Err(ApiError::BadRequest(
            "Both 'from' and 'to' dates are required".to_string(),
        ));
    }

    let limit = params.limit.unwrap_or(50).clamp(1, 1000);

    // Only CHECKIN_REVENUE_EXPR (a constant) is interpolated; the date range and
    // limit are bound. Wrapped in AssertSqlSafe per the audited-fragment rule.
    let query = format!(
        r#"
        SELECT
            c.cust_id AS customer_id,
            COALESCE(
                NULLIF(BTRIM(CONCAT_WS(' ', c.cust_firstname, c.cust_lastname)), ''),
                'ไม่ระบุชื่อ'
            ) AS customer_name,
            c.cust_phone AS phone,
            COUNT(*)::int AS checkins,
            COALESCE(SUM({rev}), 0)::float8 AS revenue
        FROM ht_checkins ci
        LEFT JOIN ht_customers c ON ci.cin_cust_id = c.cust_id
        WHERE ci.cin_status = 'checkedout'
          AND ci.cin_checkin_time::date >= $1::date
          AND ci.cin_checkin_time::date <= $2::date
        GROUP BY c.cust_id, c.cust_firstname, c.cust_lastname, c.cust_phone
        ORDER BY revenue DESC, checkins DESC
        LIMIT $3
        "#,
        rev = CHECKIN_REVENUE_EXPR,
    );

    let rows = sqlx::query(sqlx::AssertSqlSafe(&*query))
        .bind(&params.from)
        .bind(&params.to)
        .bind(limit)
        .fetch_all(pool)
        .await?;

    let data: Vec<CustomerSales> = rows
        .iter()
        .map(|row| CustomerSales {
            customer_id: row.try_get::<i32, _>("customer_id").unwrap_or(0),
            customer_name: row
                .try_get::<String, _>("customer_name")
                .unwrap_or_else(|_| "ไม่ระบุชื่อ".to_string()),
            phone: row.try_get::<Option<String>, _>("phone").unwrap_or(None),
            checkins: row.try_get::<i32, _>("checkins").unwrap_or(0),
            revenue: row.try_get::<f64, _>("revenue").unwrap_or(0.0),
        })
        .collect();

    let total_revenue: f64 = data.iter().map(|d| d.revenue).sum();

    Ok(Json(SalesByCustomerResponse {
        success: true,
        total_revenue,
        data,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// At 7% the inclusive split must tie out to the captured legacy receipt
    /// math (Total 801.00 → BeforeVat 748.60, Vat 52.40) and the per-period
    /// totals must accumulate.
    #[test]
    fn aggregate_vat_periods_splits_inclusive_at_7pct() {
        let rows = vec![
            ("2026-05".to_string(), 801.00, 3),
            ("2026-06".to_string(), 107.00, 1),
        ];
        let (data, gross, before, vat) = aggregate_vat_periods(rows, 7);

        assert_eq!(data.len(), 2);
        assert_eq!(data[0].before_vat, 748.60);
        assert_eq!(data[0].vat, 52.40);
        // 107.00 @ 7% inclusive → 100.00 + 7.00
        assert_eq!(data[1].before_vat, 100.00);
        assert_eq!(data[1].vat, 7.00);

        assert_eq!(gross, 908.00);
        assert_eq!(before, 848.60);
        assert_eq!(vat, 59.40);
    }

    /// At 0% (HF Hotel today) the split is a no-op: net == gross, output tax 0.
    #[test]
    fn aggregate_vat_periods_zero_percent_is_noop() {
        let rows = vec![("2026-06".to_string(), 1000.0, 2)];
        let (data, gross, before, vat) = aggregate_vat_periods(rows, 0);

        assert_eq!(data[0].gross, 1000.0);
        assert_eq!(data[0].before_vat, 1000.0);
        assert_eq!(data[0].vat, 0.0);
        assert_eq!((gross, before, vat), (1000.0, 1000.0, 0.0));
    }

    /// Empty input yields zero totals and no rows (clean empty-range render).
    #[test]
    fn aggregate_vat_periods_empty() {
        let (data, gross, before, vat) = aggregate_vat_periods(vec![], 7);
        assert!(data.is_empty());
        assert_eq!((gross, before, vat), (0.0, 0.0, 0.0));
    }
}
