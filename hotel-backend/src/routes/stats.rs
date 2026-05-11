//! Dashboard statistics API route
//!
//! - GET /api/stats - Get dashboard statistics
//!
//! Reads from canonical PostgreSQL tables (`ht_rooms_new`, `ht_checkins`,
//! `ht_bookings`, `ht_customers`). Previously read `ht_*_legacy` mirrors,
//! but those were demoted to hash-only drift detection in Phase 5.5
//! (2026-04-28) and stopped receiving row-level updates — leaving the
//! receptionist dashboard 2+ weeks stale.

use axum::{extract::{Query, State}, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::db::PgPool;
use crate::error::ApiResult;
use crate::routes::mode::{AppState, Branch};

/// Dashboard statistics
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardStats {
    pub total_rooms: i32,
    pub occupied_rooms: i32,
    pub checkout_rooms: i32,
    pub booked_rooms: i32,
    pub today_check_ins: i32,
    pub today_check_outs: i32,
    pub active_bookings: i32,
    pub total_customers: i32,
}

/// Stats response
#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub success: bool,
    pub data: DashboardStats,
}

/// Query parameters for stats (branch support)
#[derive(Debug, Deserialize)]
pub struct StatsQuery {
    pub branch: Option<Branch>,
}

/// GET /api/stats - Get dashboard statistics
pub async fn get_stats(
    State(state): State<AppState>,
    Query(params): Query<StatsQuery>,
) -> ApiResult<Json<StatsResponse>> {
    let branch = params.branch.unwrap_or_default();

    let stats = match branch {
        Branch::Hfhotel => get_stats_pg(&state.new_pool).await?,
        Branch::Hfville => get_stats_pg(state.ville_pool()?).await?,
        Branch::All => {
            let hf = get_stats_pg(&state.new_pool).await?;
            if let Ok(vp) = state.ville_pool() {
                let ville = get_stats_pg(vp).await?;
                DashboardStats {
                    total_rooms: hf.total_rooms + ville.total_rooms,
                    occupied_rooms: hf.occupied_rooms + ville.occupied_rooms,
                    checkout_rooms: hf.checkout_rooms + ville.checkout_rooms,
                    booked_rooms: hf.booked_rooms + ville.booked_rooms,
                    today_check_ins: hf.today_check_ins + ville.today_check_ins,
                    today_check_outs: hf.today_check_outs + ville.today_check_outs,
                    active_bookings: hf.active_bookings + ville.active_bookings,
                    total_customers: hf.total_customers + ville.total_customers,
                }
            } else {
                hf
            }
        }
    };

    Ok(Json(StatsResponse {
        success: true,
        data: stats,
    }))
}

/// Fetch dashboard stats from canonical PostgreSQL tables.
///
/// "Active checkin" predicate (reused across queries):
///     cin_status = 'active' AND cin_checkout_time IS NULL
///
/// Checkout-day flip rule (preserves prior behaviour from legacy mirror):
/// a room whose active checkin has `cin_expected_checkout = CURRENT_DATE`
/// flips from "occupied" to "checkout" once the clock passes 06:00 local —
/// giving the morning crew time to process the departure.
async fn get_stats_pg(pool: &PgPool) -> ApiResult<DashboardStats> {
    // Total active rooms in inventory
    let total_rooms: i64 = sqlx::query(
        r#"SELECT COUNT(*) AS count FROM ht_rooms_new WHERE room_active = true"#,
    )
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Occupied rooms: distinct rooms with an active checkin, EXCLUDING
    // those that have already flipped to "checkout today" (handled below).
    let occupied_rooms: i64 = sqlx::query(
        r#"
        SELECT COUNT(DISTINCT c.cin_room_id) AS count
        FROM ht_checkins c
        WHERE c.cin_status = 'active'
          AND c.cin_checkout_time IS NULL
          AND NOT (
              c.cin_expected_checkout = CURRENT_DATE
              AND EXTRACT(HOUR FROM NOW()) >= 6
          )
        "#,
    )
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Checkout rooms: active checkin whose expected checkout is today,
    // and the morning grace period (06:00) has passed.
    let checkout_rooms: i64 = sqlx::query(
        r#"
        SELECT COUNT(DISTINCT c.cin_room_id) AS count
        FROM ht_checkins c
        WHERE c.cin_status = 'active'
          AND c.cin_checkout_time IS NULL
          AND c.cin_expected_checkout = CURRENT_DATE
          AND EXTRACT(HOUR FROM NOW()) >= 6
        "#,
    )
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Booked rooms: distinct rooms with a confirmed/pending booking
    // straddling today, with no active checkin in that room yet.
    let booked_rooms: i64 = sqlx::query(
        r#"
        SELECT COUNT(DISTINCT br.br_room_id) AS count
        FROM ht_booking_rooms br
        JOIN ht_bookings b ON b.book_id = br.br_book_id
        WHERE b.book_status IN ('confirmed', 'pending')
          AND b.book_checkin <= CURRENT_DATE
          AND b.book_checkout > CURRENT_DATE
          AND NOT EXISTS (
              SELECT 1 FROM ht_checkins c
              WHERE c.cin_room_id = br.br_room_id
                AND c.cin_status = 'active'
                AND c.cin_checkout_time IS NULL
          )
        "#,
    )
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Today's check-ins (any checkin row whose checkin_time landed today)
    let today_check_ins: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) AS count
        FROM ht_checkins
        WHERE cin_checkin_time::date = CURRENT_DATE
        "#,
    )
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Today's check-outs (rows that were actually checked out today)
    let today_check_outs: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) AS count
        FROM ht_checkins
        WHERE cin_checkout_time::date = CURRENT_DATE
        "#,
    )
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Active bookings: anything not cancelled/checked-out is "active" on
    // the dashboard. Matches prior legacy-mirror semantics (book_status
    // IS NOT NULL) more precisely now that we have the canonical enum.
    let active_bookings: i64 = sqlx::query(
        r#"
        SELECT COUNT(*) AS count
        FROM ht_bookings
        WHERE book_status IN ('confirmed', 'pending', 'checkedin')
        "#,
    )
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Total customers (soft-delete aware)
    let total_customers: i64 = sqlx::query(
        r#"SELECT COUNT(*) AS count FROM ht_customers WHERE cust_deleted_at IS NULL"#,
    )
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    Ok(DashboardStats {
        total_rooms: total_rooms as i32,
        occupied_rooms: occupied_rooms as i32,
        checkout_rooms: checkout_rooms as i32,
        booked_rooms: booked_rooms as i32,
        today_check_ins: today_check_ins as i32,
        today_check_outs: today_check_outs as i32,
        active_bookings: active_bookings as i32,
        total_customers: total_customers as i32,
    })
}
