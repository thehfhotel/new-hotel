//! Customer API routes
//!
//! - GET /api/customers - List customers (search/sort/pagination)
//! - GET /api/customers/:id/bookings - Get customer's booking history
//! - GET /api/customers/:id/stats - Get customer statistics
//!
//! Reads from canonical PostgreSQL tables (`ht_customers`, `ht_bookings`,
//! `ht_checkins`). Previously read `ht_*_legacy` mirrors, but those were
//! demoted to hash-only drift detection in Phase 5.5 (2026-04-28) and
//! stopped receiving row-level updates. Coexistence audit T3 HIGH-5
//! (`docs/coexistence/audit-2026-05-13.md`).
//!
//! Path-id resolution: `Path<String>` accepts either the canonical integer
//! `cust_id` (rendered as a decimal string) or the legacy `cust_no`
//! (`C\\d+` shape stored in `ht_customers.legacy_cust_no`). The two
//! co-existence states keep older saved links/bookmarks working.

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

/// Build the data SELECT for the customer list. Exposed for unit tests so
/// they can assert the query reads canonical (`ht_customers`) and not the
/// demoted `ht_customers_legacy` mirror.
///
/// `c.cust_name` is a derived expression on canonical:
/// `TRIM(BOTH FROM c.cust_firstname || ' ' || COALESCE(c.cust_lastname, ''))`.
/// It's exposed as an alias so ORDER BY / WHERE clauses can keep referring to
/// `c.cust_name` without rewriting every call site.
pub(crate) fn build_list_customers_sql(
    has_search: bool,
    include_last_visit: bool,
    order_by_clause: &str,
) -> String {
    let last_visit_select = if include_last_visit {
        ",\n        lv.last_visit"
    } else {
        ""
    };
    // Canonical: derive `last_visit` from non-cancelled bookings only.
    // `book_cust_id` is INTEGER (FK to ht_customers.cust_id), not the legacy
    // string `cust_no`.
    let last_visit_join = if include_last_visit {
        r#"
      LEFT JOIN (
        SELECT book_cust_id, MAX(book_checkout) AS last_visit
        FROM ht_bookings
        WHERE book_status <> 'cancelled'
        GROUP BY book_cust_id
      ) lv ON c.cust_id = lv.book_cust_id"#
    } else {
        ""
    };

    // Search predicate — applied to canonical columns. `c.cust_name` is the
    // derived alias defined in the SELECT list. `cust_code` and
    // `legacy_cust_no` cover both the canonical customer-code shape and the
    // legacy `C\\d+` identifier the receptionist still types into search.
    let where_clause = if has_search {
        " WHERE c.cust_name ILIKE $1\n             OR c.cust_phone ILIKE $1\n             \
         OR c.cust_idcard ILIKE $1\n             OR c.cust_code ILIKE $1\n             \
         OR c.legacy_cust_no ILIKE $1\n             OR c.cust_id::text = TRIM(BOTH '%' FROM $1)"
    } else {
        ""
    };

    let (limit_clause, _) = if has_search {
        ("LIMIT $2 OFFSET $3", "ws")
    } else {
        ("LIMIT $1 OFFSET $2", "ns")
    };

    format!(
        r#"
        SELECT
            c.cust_id,
            c.legacy_cust_no,
            TRIM(BOTH FROM c.cust_firstname || ' ' || COALESCE(c.cust_lastname, '')) AS cust_name,
            c.cust_type,
            c.cust_phone,
            c.cust_idcard,
            c.cust_address{last_visit_select}
        FROM ht_customers c{last_visit_join}
        {where_clause}
        {soft_delete}
        ORDER BY {order_by_clause}
        {limit_clause}
        "#,
        soft_delete = if has_search {
            "AND c.cust_deleted_at IS NULL"
        } else {
            "WHERE c.cust_deleted_at IS NULL"
        },
    )
}

/// Build the COUNT SQL for the customer list. Exposed for unit tests.
pub(crate) fn build_count_customers_sql(has_search: bool) -> String {
    // `c.cust_name` here references the same derived expression — DRY by
    // inlining the COALESCE form for the WHERE clause so we don't pay for an
    // extra subquery.
    let where_clause = if has_search {
        " WHERE (TRIM(BOTH FROM c.cust_firstname || ' ' || COALESCE(c.cust_lastname, ''))\
                ILIKE $1\n             OR c.cust_phone ILIKE $1\n             \
         OR c.cust_idcard ILIKE $1\n             OR c.cust_code ILIKE $1\n             \
         OR c.legacy_cust_no ILIKE $1\n             OR c.cust_id::text = TRIM(BOTH '%' FROM $1))"
    } else {
        ""
    };
    format!(
        "SELECT COUNT(*)::int AS total FROM ht_customers c {where_clause} {soft_delete}",
        soft_delete = if has_search {
            "AND c.cust_deleted_at IS NULL"
        } else {
            "WHERE c.cust_deleted_at IS NULL"
        },
    )
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

    // Map frontend column names to canonical PG columns. `name` orders by the
    // derived alias from build_list_customers_sql; `lastVisit` requires the
    // JOIN to be present.
    let order_by_column = match params.sort_by.as_deref() {
        Some("name") => "cust_name",
        Some("type") => "c.cust_type",
        Some("phone") => "c.cust_phone",
        Some("lastVisit") if params.include_last_visit => "lv.last_visit",
        _ => "c.cust_id",
    };

    let order_by_clause = if params.sort_by.as_deref() == Some("lastVisit")
        && params.include_last_visit
    {
        format!(
            "CASE WHEN lv.last_visit IS NULL THEN 1 ELSE 0 END, {} {}",
            order_by_column, sort_order
        )
    } else {
        format!("{} {}", order_by_column, sort_order)
    };

    let count_query = build_count_customers_sql(params.search.is_some());

    let total: i32 = if let Some(ref search) = params.search {
        let search_pattern = format!("%{}%", search);
        sqlx::query(&count_query)
            .bind(&search_pattern)
            .fetch_one(pool)
            .await?
            .get("total")
    } else {
        sqlx::query(&count_query).fetch_one(pool).await?.get("total")
    };

    let data_query = build_list_customers_sql(
        params.search.is_some(),
        params.include_last_visit,
        &order_by_clause,
    );

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

    // Map canonical rows to the public Customer shape. `id` carries the
    // canonical `cust_id` as a decimal string so the path-id endpoints
    // round-trip; the legacy `cust_no` (when present) lives in `customer.id`
    // for routes that pass `C\\d+`-shaped values — both forms resolve via
    // `resolve_customer_id_int` below.
    let customers: Vec<Customer> = rows
        .iter()
        .map(|row| {
            let canonical_id: i32 = row.get("cust_id");
            let legacy_no: Option<String> = row.get("legacy_cust_no");
            // Prefer the legacy id when present so existing frontend callers
            // that pass `C\\d+` continue to receive the same identifier shape
            // they post back to the detail endpoints.
            let id = legacy_no.unwrap_or_else(|| canonical_id.to_string());
            Customer {
                id,
                name: row.get::<Option<String>, _>("cust_name"),
                customer_type: row.get::<Option<String>, _>("cust_type"),
                phone: row.get::<Option<String>, _>("cust_phone"),
                id_card: row.get::<Option<String>, _>("cust_idcard"),
                address: row.get::<Option<String>, _>("cust_address"),
                last_visit: if params.include_last_visit {
                    row.get::<Option<NaiveDateTime>, _>("last_visit")
                        .map(|dt| dt.and_utc())
                } else {
                    None
                },
            }
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

/// Resolve a path-id (which may be canonical `cust_id` decimal or legacy
/// `cust_no` like `C21636`) to the canonical integer `cust_id`. Returns
/// `Ok(None)` when no canonical row matches — the caller renders an empty
/// payload (preserves the prior legacy-mirror behaviour where a missing
/// customer produced an empty bookings/stats response, not a 404).
async fn resolve_customer_id_int(
    pool: &PgPool,
    path_id: &str,
) -> ApiResult<Option<i32>> {
    // Numeric-only path — try the canonical PK first.
    if let Ok(parsed) = path_id.parse::<i32>() {
        let row = sqlx::query(
            "SELECT cust_id FROM ht_customers WHERE cust_id = $1 AND cust_deleted_at IS NULL",
        )
        .bind(parsed)
        .fetch_optional(pool)
        .await?;
        if let Some(r) = row {
            return Ok(Some(r.get("cust_id")));
        }
    }
    // Fall through to legacy lookup (handles `C21636`-shaped ids that
    // pre-date the canonical migration).
    let row = sqlx::query(
        "SELECT cust_id FROM ht_customers \
         WHERE legacy_cust_no = $1 AND cust_deleted_at IS NULL",
    )
    .bind(path_id)
    .fetch_optional(pool)
    .await?;
    Ok(row.map(|r| r.get("cust_id")))
}

// ---------------------------------------------------------------------------
// GET /api/customers/:id/bookings
// ---------------------------------------------------------------------------

/// GET /api/customers/:id/bookings - Get customer's booking history
///
/// Reads canonical `ht_bookings` joined to `ht_rooms_new` (via
/// `ht_booking_rooms`) and `ht_room_types`. The legacy mirror that this
/// previously read (`ht_bookings_legacy`) was demoted to drift-detection
/// only in Phase 5.5 (2026-04-28). Coexistence audit T3 HIGH-5.
pub async fn get_customer_bookings(
    State(state): State<AppState>,
    Path(cust_id): Path<String>,
) -> ApiResult<Json<CustomerBookingsResponse>> {
    get_customer_bookings_pg(&state.new_pool, &cust_id).await
}

/// Canonical SELECT for a customer's booking history. Exposed so unit tests
/// can assert it reads `ht_bookings` (canonical) not `ht_bookings_legacy`.
///
/// `room_number` falls back to `'-'` for bookings with no assigned room yet
/// — preserves the prior legacy-mirror display shape. `total_amount` carries
/// `book_total_amount` from canonical (the legacy mirror always emitted 0.0
/// because the column was missing).
pub(crate) const CUSTOMER_BOOKINGS_SQL: &str = r#"
    SELECT
        b.book_no,
        rt.type_name AS book_room_type,
        r.room_no AS book_room_no,
        b.book_checkin AS book_date_in,
        b.book_checkout AS book_date_out,
        b.book_status,
        b.book_total_amount::float8 AS total_amount
    FROM ht_bookings b
    LEFT JOIN ht_booking_rooms br ON br.br_book_id = b.book_id
    LEFT JOIN ht_rooms_new r ON r.room_id = br.br_room_id
    LEFT JOIN ht_room_types rt ON rt.type_id = COALESCE(br.br_room_type_id, r.room_type_id)
    WHERE b.book_cust_id = $1
    ORDER BY b.book_checkin DESC, b.book_id DESC
"#;

/// PostgreSQL implementation of get_customer_bookings
async fn get_customer_bookings_pg(
    pool: &PgPool,
    path_id: &str,
) -> ApiResult<Json<CustomerBookingsResponse>> {
    let cust_id_int = match resolve_customer_id_int(pool, path_id).await? {
        Some(id) => id,
        None => {
            return Ok(Json(CustomerBookingsResponse {
                success: true,
                bookings: vec![],
            }));
        }
    };

    let rows = sqlx::query(CUSTOMER_BOOKINGS_SQL)
        .bind(cust_id_int)
        .fetch_all(pool)
        .await?;

    let bookings: Vec<CustomerBooking> = rows
        .iter()
        .map(|row| {
            // Canonical book_status is a text enum
            // ('confirmed' | 'pending' | 'cancelled' | 'checkedin' | 'checkedout').
            // Surface verbatim — the prior i32-keyed map was a no-op on canonical.
            let status: String = row
                .get::<Option<String>, _>("book_status")
                .unwrap_or_else(|| "pending".to_string());

            CustomerBooking {
                id: row.get::<String, _>("book_no"),
                room_number: row
                    .get::<Option<String>, _>("book_room_no")
                    .unwrap_or_else(|| "-".to_string()),
                room_type: row
                    .get::<Option<String>, _>("book_room_type")
                    .unwrap_or_else(|| "-".to_string()),
                check_in_date: row
                    .get::<Option<chrono::NaiveDate>, _>("book_date_in")
                    .and_then(|d| d.and_hms_opt(0, 0, 0))
                    .map(|dt| dt.and_utc()),
                check_out_date: row
                    .get::<Option<chrono::NaiveDate>, _>("book_date_out")
                    .and_then(|d| d.and_hms_opt(0, 0, 0))
                    .map(|dt| dt.and_utc()),
                status,
                total_amount: row.get::<Option<f64>, _>("total_amount").unwrap_or(0.0),
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
/// Reads canonical `ht_bookings` + `ht_checkins` + `ht_room_types`. The
/// legacy mirrors this previously read (`ht_bookings_legacy`,
/// `ht_checkins_legacy`) were demoted to drift-detection only in Phase 5.5
/// (2026-04-28). Coexistence audit T3 HIGH-5.
pub async fn get_customer_stats(
    State(state): State<AppState>,
    Path(cust_id): Path<String>,
) -> ApiResult<Json<CustomerStatsResponse>> {
    get_customer_stats_pg(&state.new_pool, &cust_id).await
}

/// Canonical SQL for the per-customer booking aggregates. Cancelled bookings
/// are excluded so "first/last visit" and "average stay" reflect realised
/// stays only. Exposed for unit tests.
pub(crate) const CUSTOMER_STATS_BOOKINGS_SQL: &str = r#"
    SELECT
        COUNT(*)::int AS total_bookings,
        MIN(book_checkin) AS first_visit,
        MAX(book_checkout) AS last_visit,
        AVG((book_checkout - book_checkin)::float8) AS avg_stay_days
    FROM ht_bookings
    WHERE book_cust_id = $1
      AND book_status <> 'cancelled'
"#;

/// Canonical SQL for the per-customer checkin count. Counts only realised
/// stays (`cin_status IN ('active','checkedout')`).
pub(crate) const CUSTOMER_STATS_STAYS_SQL: &str = r#"
    SELECT COUNT(*)::int AS total_stays
    FROM ht_checkins
    WHERE cin_cust_id = $1
      AND cin_status IN ('active', 'checkedout')
"#;

/// Canonical SQL for the customer's most-frequent room type. Joins
/// `ht_booking_rooms` for the type code, then `ht_room_types` for the
/// display name.
pub(crate) const CUSTOMER_STATS_FAVORITE_ROOM_SQL: &str = r#"
    SELECT rt.type_name AS book_room_type, COUNT(*) AS cnt
    FROM ht_bookings b
    JOIN ht_booking_rooms br ON br.br_book_id = b.book_id
    LEFT JOIN ht_rooms_new r ON r.room_id = br.br_room_id
    JOIN ht_room_types rt ON rt.type_id = COALESCE(br.br_room_type_id, r.room_type_id)
    WHERE b.book_cust_id = $1
      AND b.book_status <> 'cancelled'
    GROUP BY rt.type_name
    ORDER BY cnt DESC
    LIMIT 1
"#;

/// PostgreSQL implementation of get_customer_stats
async fn get_customer_stats_pg(
    pool: &PgPool,
    path_id: &str,
) -> ApiResult<Json<CustomerStatsResponse>> {
    let cust_id_int = match resolve_customer_id_int(pool, path_id).await? {
        Some(id) => id,
        None => {
            return Ok(Json(CustomerStatsResponse {
                success: true,
                stats: CustomerStats {
                    total_bookings: 0,
                    total_stays: 0,
                    first_visit: None,
                    last_visit: None,
                    favorite_room_type: None,
                    avg_stay_days: None,
                },
            }));
        }
    };

    let booking_stats_row = sqlx::query(CUSTOMER_STATS_BOOKINGS_SQL)
        .bind(cust_id_int)
        .fetch_one(pool)
        .await?;

    let stays_row = sqlx::query(CUSTOMER_STATS_STAYS_SQL)
        .bind(cust_id_int)
        .fetch_one(pool)
        .await?;

    let favorite_room_row = sqlx::query(CUSTOMER_STATS_FAVORITE_ROOM_SQL)
        .bind(cust_id_int)
        .fetch_optional(pool)
        .await?;

    let total_bookings: i32 = booking_stats_row.get("total_bookings");
    // Canonical `book_checkin` / `book_checkout` are DATE; promote to a
    // midnight datetime for the existing `Option<DateTime<Utc>>` shape.
    let first_visit: Option<NaiveDateTime> = booking_stats_row
        .get::<Option<chrono::NaiveDate>, _>("first_visit")
        .and_then(|d| d.and_hms_opt(0, 0, 0));
    let last_visit: Option<NaiveDateTime> = booking_stats_row
        .get::<Option<chrono::NaiveDate>, _>("last_visit")
        .and_then(|d| d.and_hms_opt(0, 0, 0));
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

    Ok(Json(CustomerStatsResponse { success: true, stats }))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------------------------------------------------------------------
    // T3 HIGH-5: list / bookings / stats must read canonical tables, never
    // the demoted `ht_*_legacy` mirrors.
    // ---------------------------------------------------------------------

    #[test]
    fn list_reads_ht_customers_not_legacy() {
        let sql = build_list_customers_sql(false, false, "c.cust_id ASC");
        assert!(!sql.contains("ht_customers_legacy"), "must not read demoted mirror");
        assert!(sql.contains("FROM ht_customers c"));
    }

    #[test]
    fn list_count_reads_ht_customers_not_legacy() {
        let count_sql = build_count_customers_sql(true);
        assert!(!count_sql.contains("ht_customers_legacy"));
        assert!(count_sql.contains("FROM ht_customers c"));
    }

    #[test]
    fn list_projects_canonical_name_columns_not_legacy_cust_name() {
        // Canonical schema has cust_firstname + cust_lastname; the demoted
        // mirror had a single `cust_name`. The SELECT must derive the alias.
        let sql = build_list_customers_sql(false, false, "c.cust_id ASC");
        assert!(sql.contains("c.cust_firstname"));
        assert!(sql.contains("c.cust_lastname"));
    }

    #[test]
    fn list_with_last_visit_reads_ht_bookings_canonical() {
        let sql = build_list_customers_sql(false, true, "c.cust_id ASC");
        assert!(sql.contains("FROM ht_bookings"));
        assert!(!sql.contains("ht_bookings_legacy"));
        // Aggregates non-cancelled bookings only
        assert!(sql.contains("book_status <> 'cancelled'"));
        // Joins on canonical FK `book_cust_id = c.cust_id` (integer), not the
        // legacy string `book_cust_id = c.cust_no` shape.
        assert!(sql.contains("c.cust_id = lv.book_cust_id"));
    }

    #[test]
    fn list_filters_out_soft_deleted_customers() {
        let sql = build_list_customers_sql(false, false, "c.cust_id ASC");
        assert!(sql.contains("c.cust_deleted_at IS NULL"));
    }

    #[test]
    fn bookings_reads_canonical_ht_bookings() {
        assert!(!CUSTOMER_BOOKINGS_SQL.contains("ht_bookings_legacy"));
        assert!(CUSTOMER_BOOKINGS_SQL.contains("FROM ht_bookings b"));
        // Canonical column names; legacy ones must not appear.
        assert!(CUSTOMER_BOOKINGS_SQL.contains("b.book_checkin"));
        assert!(CUSTOMER_BOOKINGS_SQL.contains("b.book_checkout"));
        assert!(!CUSTOMER_BOOKINGS_SQL.contains("book_date_in "));
        assert!(!CUSTOMER_BOOKINGS_SQL.contains("book_date_out "));
    }

    #[test]
    fn stats_reads_canonical_tables_not_legacy_mirrors() {
        // Bookings aggregate
        assert!(!CUSTOMER_STATS_BOOKINGS_SQL.contains("ht_bookings_legacy"));
        assert!(CUSTOMER_STATS_BOOKINGS_SQL.contains("FROM ht_bookings"));
        // Stays count
        assert!(!CUSTOMER_STATS_STAYS_SQL.contains("ht_checkins_legacy"));
        assert!(CUSTOMER_STATS_STAYS_SQL.contains("FROM ht_checkins"));
        // Favorite room
        assert!(!CUSTOMER_STATS_FAVORITE_ROOM_SQL.contains("ht_bookings_legacy"));
        assert!(CUSTOMER_STATS_FAVORITE_ROOM_SQL.contains("FROM ht_bookings"));
    }

    #[test]
    fn stats_excludes_cancelled_bookings_and_checkins() {
        assert!(CUSTOMER_STATS_BOOKINGS_SQL.contains("book_status <> 'cancelled'"));
        assert!(CUSTOMER_STATS_STAYS_SQL.contains("cin_status IN ('active', 'checkedout')"));
        assert!(CUSTOMER_STATS_FAVORITE_ROOM_SQL.contains("book_status <> 'cancelled'"));
    }

    #[test]
    fn stats_stays_filters_by_canonical_int_cust_id() {
        // Canonical `cin_cust_id` is INTEGER (FK), not the legacy
        // `cin_cust_no` VARCHAR.
        assert!(CUSTOMER_STATS_STAYS_SQL.contains("cin_cust_id = $1"));
        assert!(!CUSTOMER_STATS_STAYS_SQL.contains("cin_cust_no"));
    }
}

