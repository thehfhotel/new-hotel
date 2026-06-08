//! Dashboard statistics API route
//!
//! - GET /api/stats - Get dashboard statistics
//!
//! Reads from canonical PostgreSQL tables (`ht_rooms_new`, `ht_checkins`,
//! `ht_bookings`, `ht_customers`). Previously read `ht_*_legacy` mirrors,
//! but those were demoted to hash-only drift detection in Phase 5.5
//! (2026-04-28) and stopped receiving row-level updates — leaving the
//! receptionist dashboard 2+ weeks stale.
//!
//! Timezone discipline: every "today" or "06:00 morning-flip" predicate is
//! evaluated in Bangkok local time. The PG server runs UTC; bare
//! `CURRENT_DATE` / `NOW()` would interpret "today" as UTC and the
//! `today_check_outs` tile would silently miss any checkout done between
//! 00:00 and 07:00 BKK (17:00–24:00 UTC the prior day). Same story for
//! the morning-flip — the 06:00 grace period must trigger at 06:00 BKK,
//! not 06:00 UTC (= 13:00 BKK). Coexistence audit T3 MED-1 + MED-2.

use axum::{extract::{Query, State}, Json};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::db::PgPool;
use crate::error::ApiResult;
use crate::routes::mode::{AppState, Branch};

/// SQL fragment for "today" in Bangkok — replaces bare `CURRENT_DATE` so
/// the dashboard never reads "today" off the UTC clock. Use everywhere a
/// receptionist-facing date is being computed.
pub(crate) const BANGKOK_TODAY_SQL: &str = "(NOW() AT TIME ZONE 'Asia/Bangkok')::date";

/// SQL fragment for "current hour" in Bangkok — gates the morning-flip
/// (06:00 BKK) that moves an expected-checkout-today from "occupied" to
/// "checkout pending" on the dashboard.
pub(crate) const BANGKOK_HOUR_SQL: &str =
    "EXTRACT(HOUR FROM NOW() AT TIME ZONE 'Asia/Bangkok')";

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
    //
    // ## Track B3 / T3 HIGH-1 — count rooms through the junction
    //
    // Pre-B3 this counted `DISTINCT c.cin_room_id`, which collapsed every
    // multi-room folio to its first room. Now walks `ht_checkin_rooms`
    // (one row per actual room) with a `COALESCE` fallback to header-level
    // `cin_room_id` for pre-B2 folios that haven't been re-synced through
    // the B2 mapper. After B5 backfill the fallback is dead code and the
    // column drops.
    let occupied_rooms: i64 = sqlx::query(sqlx::AssertSqlSafe(format!(
        r#"
        SELECT COUNT(DISTINCT COALESCE(cr.cr_room_id, c.cin_room_id)) AS count
        FROM ht_checkins c
        LEFT JOIN ht_checkin_rooms cr ON cr.cr_cin_id = c.cin_id
        WHERE c.cin_status = 'active'
          AND c.cin_checkout_time IS NULL
          AND NOT (
              c.cin_expected_checkout = {today}
              AND {hour} >= 6
          )
        "#,
        today = BANGKOK_TODAY_SQL,
        hour = BANGKOK_HOUR_SQL,
    )))
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Checkout rooms: active checkin whose expected checkout is today,
    // and the morning grace period (06:00 BKK) has passed.
    //
    // Track B3 / T3 HIGH-3: same junction-walk pattern as `occupied_rooms`
    // — the morning checkout tile must enumerate every room of a
    // multi-room folio, not just the first.
    let checkout_rooms: i64 = sqlx::query(sqlx::AssertSqlSafe(format!(
        r#"
        SELECT COUNT(DISTINCT COALESCE(cr.cr_room_id, c.cin_room_id)) AS count
        FROM ht_checkins c
        LEFT JOIN ht_checkin_rooms cr ON cr.cr_cin_id = c.cin_id
        WHERE c.cin_status = 'active'
          AND c.cin_checkout_time IS NULL
          AND c.cin_expected_checkout = {today}
          AND {hour} >= 6
        "#,
        today = BANGKOK_TODAY_SQL,
        hour = BANGKOK_HOUR_SQL,
    )))
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Booked rooms: distinct rooms with a confirmed/pending booking
    // straddling today (BKK), with no active checkin in that room yet.
    //
    // Track B3 / T3 HIGH-1: the "no active checkin in that room" guard
    // now also walks `ht_checkin_rooms` so a 2-room folio doesn't
    // inadvertently leak rooms 2..N back into the "booked" bucket while
    // the folio is checked in.
    let booked_rooms: i64 = sqlx::query(sqlx::AssertSqlSafe(format!(
        r#"
        SELECT COUNT(DISTINCT br.br_room_id) AS count
        FROM ht_booking_rooms br
        JOIN ht_bookings b ON b.book_id = br.br_book_id
        WHERE b.book_status IN ('confirmed', 'pending')
          AND b.book_checkin <= {today}
          AND b.book_checkout > {today}
          AND NOT EXISTS (
              SELECT 1 FROM ht_checkins c
              WHERE c.cin_status = 'active'
                AND c.cin_checkout_time IS NULL
                AND (
                    c.cin_room_id = br.br_room_id
                    OR EXISTS (
                        SELECT 1 FROM ht_checkin_rooms cr
                        WHERE cr.cr_cin_id = c.cin_id
                          AND cr.cr_room_id = br.br_room_id
                    )
                )
          )
        "#,
        today = BANGKOK_TODAY_SQL,
    )))
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Today's check-ins (any checkin row whose checkin_time landed today
    // in Bangkok local time). Per CLAUDE.md timezone discipline,
    // `cin_checkin_time` is a naive TIMESTAMP storing Bangkok wall-clock —
    // compare its `::date` directly against `BANGKOK_TODAY_SQL`. Do NOT
    // also `AT TIME ZONE 'Asia/Bangkok'` the value (that would treat it
    // as UTC and shift it forward by 7h, giving the next-day date).
    let today_check_ins: i64 = sqlx::query(sqlx::AssertSqlSafe(format!(
        r#"
        SELECT COUNT(*) AS count
        FROM ht_checkins
        WHERE cin_checkin_time::date = {today}
        "#,
        today = BANGKOK_TODAY_SQL,
    )))
    .fetch_one(pool)
    .await?
    .try_get("count")
    .unwrap_or(0);

    // Today's check-outs (rows that were actually checked out today in
    // Bangkok local time). Same reasoning: `cin_checkout_time` is naive
    // BKK wall-clock, compare directly.
    let today_check_outs: i64 = sqlx::query(sqlx::AssertSqlSafe(format!(
        r#"
        SELECT COUNT(*) AS count
        FROM ht_checkins
        WHERE cin_checkout_time::date = {today}
        "#,
        today = BANGKOK_TODAY_SQL,
    )))
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

#[cfg(test)]
mod tests {
    use super::{BANGKOK_HOUR_SQL, BANGKOK_TODAY_SQL};

    /// Coexistence audit T3 MED-1 + MED-2: every "today" predicate must be
    /// evaluated against the Bangkok wall-clock, not UTC. The constants
    /// carry the exact SQL fragments callers splice into queries; a
    /// regression to bare `CURRENT_DATE` would surface immediately here.
    #[test]
    fn bangkok_today_uses_at_time_zone_asia_bangkok() {
        assert_eq!(BANGKOK_TODAY_SQL, "(NOW() AT TIME ZONE 'Asia/Bangkok')::date");
    }

    #[test]
    fn bangkok_hour_uses_at_time_zone_asia_bangkok() {
        assert_eq!(
            BANGKOK_HOUR_SQL,
            "EXTRACT(HOUR FROM NOW() AT TIME ZONE 'Asia/Bangkok')"
        );
    }

    #[test]
    fn bangkok_today_is_not_bare_current_date() {
        assert!(!BANGKOK_TODAY_SQL.contains("CURRENT_DATE"));
        assert!(BANGKOK_TODAY_SQL.contains("AT TIME ZONE 'Asia/Bangkok'"));
    }

    #[test]
    fn bangkok_hour_is_not_bare_now() {
        assert!(BANGKOK_HOUR_SQL.contains("AT TIME ZONE 'Asia/Bangkok'"));
    }
}
