//! Daily guest roster API routes — task #43 (desk paperwork).
//!
//! Front-desk "who's in the building today" lists, returned in FULL (no silent
//! `limit` cap — the v2 dashboard glances cap at 100 and therefore under-count
//! a busy day). All read-only, all branch-aware exactly like
//! [`crate::routes::new_shifts`]: `?branch=hfhotel|hfville|all`. Site data lives
//! in separate logical PG databases, so the pool selection *is* the site filter;
//! `all` merges both sites (HF Hotel + HF Ville), tagging every row with its
//! `branch` like `new_shifts::round_summary`.
//!
//! - `GET /api/rosters/summary?branch=&date=`     — counts (the daily summary)
//! - `GET /api/rosters/in-house?branch=&date=`    — guests currently staying
//! - `GET /api/rosters/arrivals?branch=&date=`    — expected arrivals (bookings)
//! - `GET /api/rosters/departures?branch=&date=`  — guests departing on `date`
//! - `GET /api/rosters/continuing?branch=&date=`  — stayovers (arrived earlier,
//!                                                   not leaving on `date`)
//!
//! `date` is `YYYY-MM-DD`, defaulting to today in Bangkok
//! (`(NOW() AT TIME ZONE 'Asia/Bangkok')::date`) so the roster matches the
//! Thai-local naive timestamps stored in `ht_checkins` / `ht_bookings`.
//!
//! The roster definitions mirror the v2 dashboard (`app/v2/page.tsx`) so the two
//! surfaces agree: in-house = active/checked-in check-ins covering the night of
//! `date`; arrivals = `pending`/`confirmed` bookings with `book_checkin = date`
//! (i.e. expected, not yet checked in); departures = in-house with
//! `cin_expected_checkout = date`; continuing = in-house arrived before `date`
//! and not leaving on `date`.

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use super::mode::{AppState, Branch};
use crate::db::PgPool;
use crate::error::{ApiError, ApiResult};

// =============================================================================
// Request / response shapes
// =============================================================================

/// Common query string for every roster endpoint.
#[derive(Debug, Deserialize)]
pub struct RosterQuery {
    /// `hfhotel` (default) | `hfville` | `all`.
    pub branch: Option<Branch>,
    /// As-of date (`YYYY-MM-DD`). Defaults to today in Bangkok.
    pub date: Option<NaiveDate>,
}

/// One check-in-anchored roster row (in-house / departures / continuing).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StayRosterRow {
    /// Site this row belongs to ("hfhotel" | "hfville") — set on every row so
    /// the merged `all` view can label + group by site.
    pub branch: String,
    pub cin_id: i32,
    pub cin_no: String,
    pub booking_no: Option<String>,
    pub customer_name: Option<String>,
    pub phone: Option<String>,
    pub room_no: Option<String>,
    pub room_type_name: Option<String>,
    pub check_in_time: Option<NaiveDateTime>,
    pub check_out_time: Option<NaiveDateTime>,
    pub expected_checkout: Option<NaiveDate>,
    pub adults: Option<i32>,
    pub children: Option<i32>,
    /// `cin_expected_checkout - cin_checkin_time::date` (whole nights booked).
    pub nights: Option<i32>,
    pub rate_per_night: Option<f64>,
    pub total_amount: Option<f64>,
    pub notes: Option<String>,
}

/// One expected-arrival roster row (booking-anchored).
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrivalRosterRow {
    pub branch: String,
    pub book_id: i32,
    pub book_no: String,
    pub customer_name: Option<String>,
    pub phone: Option<String>,
    pub check_in: Option<NaiveDate>,
    pub check_out: Option<NaiveDate>,
    pub adults: Option<i32>,
    pub children: Option<i32>,
    pub nights: Option<i32>,
    /// Rooms assigned to the booking (0 ⇒ waitlist / unassigned).
    pub room_count: i32,
    /// Comma-joined room numbers, or `None` when unassigned.
    pub room_nos: Option<String>,
    pub status: String,
}

/// Envelope for the stay-based roster endpoints.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StayRosterResponse {
    pub success: bool,
    pub date: NaiveDate,
    pub count: i32,
    pub data: Vec<StayRosterRow>,
}

/// Envelope for the arrivals roster endpoint.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArrivalRosterResponse {
    pub success: bool,
    pub date: NaiveDate,
    pub count: i32,
    pub data: Vec<ArrivalRosterRow>,
}

/// Daily summary counts (the "paperwork at a glance").
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RosterSummaryResponse {
    pub success: bool,
    pub date: NaiveDate,
    /// Expected arrivals (bookings checking in on `date`, not yet checked in).
    pub arrivals: i32,
    /// Guests currently staying (active/checked-in, covering the night of `date`).
    pub in_house: i32,
    /// Guests departing on `date`.
    pub departures: i32,
    /// Stayovers: arrived before `date`, not leaving on `date`.
    pub continuing: i32,
    /// Headcount (adults + children) of the in-house set.
    pub in_house_guests: i32,
}

// =============================================================================
// SQL building blocks
// =============================================================================

/// Shared SELECT/JOIN prefix for the check-in-anchored rosters. The per-roster
/// WHERE fragment + the ORDER suffix are appended at runtime; all fragments are
/// compile-time literals (no user value interpolated — the as-of date is bound
/// as `$1`), so the assembled string is wrapped in [`sqlx::AssertSqlSafe`].
const STAY_SELECT: &str = "SELECT \
    ci.cin_id, COALESCE(NULLIF(ci.legacy_cin_no, ''), ci.cin_no) AS cin_no, b.book_no, \
    CONCAT(c.cust_firstname, ' ', COALESCE(c.cust_lastname, '')) AS customer_name, \
    c.cust_phone, r.room_no, rt.type_name, \
    ci.cin_checkin_time, ci.cin_checkout_time, ci.cin_expected_checkout, \
    ci.cin_adults, ci.cin_children, ci.cin_status, \
    ci.cin_rate_per_night::float8 AS cin_rate_per_night, \
    ci.cin_total_amount::float8  AS cin_total_amount, ci.cin_notes, \
    (ci.cin_expected_checkout - ci.cin_checkin_time::date) AS nights \
    FROM ht_checkins ci \
    LEFT JOIN ht_customers  c  ON ci.cin_cust_id  = c.cust_id \
    LEFT JOIN ht_rooms_new  r  ON ci.cin_room_id  = r.room_id \
    LEFT JOIN ht_room_types rt ON r.room_type_id  = rt.type_id \
    LEFT JOIN ht_bookings   b  ON ci.cin_book_id  = b.book_id ";

const STAY_ORDER: &str = " ORDER BY r.room_no ASC NULLS LAST, ci.cin_id ASC";

/// Which check-in roster to fetch — selects the WHERE fragment.
#[derive(Debug, Clone, Copy)]
enum StayKind {
    InHouse,
    Departures,
    Continuing,
}

impl StayKind {
    /// Literal WHERE fragment. The as-of date is `$1` (bound, not interpolated).
    /// `'checkedin'` is included defensively alongside `'active'` to match the
    /// dashboard filter (`status === 'active' || 'checkedin'`).
    fn where_clause(self) -> &'static str {
        match self {
            // Covering the night of `date`: arrived on/before it, leaving on/after.
            StayKind::InHouse => {
                "WHERE ci.cin_status IN ('active','checkedin') \
                   AND ci.cin_checkin_time::date <= $1 \
                   AND ci.cin_expected_checkout   >= $1"
            }
            // Leaving on `date`.
            StayKind::Departures => {
                "WHERE ci.cin_status IN ('active','checkedin') \
                   AND ci.cin_expected_checkout = $1"
            }
            // Arrived before `date`, staying past it (a stayover, not a departure).
            StayKind::Continuing => {
                "WHERE ci.cin_status IN ('active','checkedin') \
                   AND ci.cin_checkin_time::date < $1 \
                   AND ci.cin_expected_checkout  > $1"
            }
        }
    }
}

/// Arrivals: `pending`/`confirmed` bookings checking in on the as-of date (i.e.
/// expected, not yet promoted to a check-in). Room numbers are aggregated from
/// `ht_booking_rooms`. Full literal — bound `$1` is the as-of date.
const ARRIVALS_SQL: &str = "SELECT \
    b.book_id, b.book_no, \
    CONCAT(c.cust_firstname, ' ', COALESCE(c.cust_lastname, '')) AS customer_name, \
    c.cust_phone, b.book_checkin, b.book_checkout, \
    b.book_adults, b.book_children, b.book_status, b.book_nights, \
    (SELECT COUNT(*)::int FROM ht_booking_rooms br WHERE br.br_book_id = b.book_id) AS room_count, \
    (SELECT STRING_AGG(r.room_no, ', ' ORDER BY r.room_no) \
       FROM ht_booking_rooms br JOIN ht_rooms_new r ON br.br_room_id = r.room_id \
      WHERE br.br_book_id = b.book_id) AS room_nos \
    FROM ht_bookings b \
    LEFT JOIN ht_customers c ON b.book_cust_id = c.cust_id \
    WHERE b.book_checkin = $1 AND b.book_status IN ('pending','confirmed') \
    ORDER BY customer_name ASC, b.book_id ASC";

/// One-shot summary counts for a single site. Full literal — `$1` is as-of.
const SUMMARY_SQL: &str = "SELECT \
    (SELECT COUNT(*)::int FROM ht_bookings \
       WHERE book_checkin = $1 AND book_status IN ('pending','confirmed')) AS arrivals, \
    (SELECT COUNT(*)::int FROM ht_checkins \
       WHERE cin_status IN ('active','checkedin') \
         AND cin_checkin_time::date <= $1 AND cin_expected_checkout >= $1) AS in_house, \
    (SELECT COALESCE(SUM(cin_adults + cin_children),0)::int FROM ht_checkins \
       WHERE cin_status IN ('active','checkedin') \
         AND cin_checkin_time::date <= $1 AND cin_expected_checkout >= $1) AS in_house_guests, \
    (SELECT COUNT(*)::int FROM ht_checkins \
       WHERE cin_status IN ('active','checkedin') AND cin_expected_checkout = $1) AS departures, \
    (SELECT COUNT(*)::int FROM ht_checkins \
       WHERE cin_status IN ('active','checkedin') \
         AND cin_checkin_time::date < $1 AND cin_expected_checkout > $1) AS continuing";

// =============================================================================
// Pool / date resolution helpers (mirror new_shifts::round_summary)
// =============================================================================

/// Resolve which site pool(s) to query. `all` merges both sites; HF Ville is
/// included only when its DB is wired up (graceful degrade for `all`, hard error
/// for an explicit `hfville` request) — identical to `round_summary`.
fn resolve_pools<'a>(
    state: &'a AppState,
    branch: Branch,
) -> ApiResult<Vec<(&'static str, &'a PgPool)>> {
    let mut pools: Vec<(&'static str, &PgPool)> = Vec::new();
    if branch == Branch::Hfhotel || branch == Branch::All {
        pools.push(("hfhotel", &state.new_pool));
    }
    if branch == Branch::Hfville || branch == Branch::All {
        match state.ville_pool() {
            Ok(vp) => pools.push(("hfville", vp)),
            Err(e) if branch == Branch::Hfville => return Err(e),
            Err(_) => {} // `all` + no Ville DB → HF Hotel only.
        }
    }
    Ok(pools)
}

/// Resolve the as-of date: the request value, or today in Bangkok (computed in
/// the DB so it matches the Thai-local naive timestamps the rosters compare).
async fn resolve_as_of(state: &AppState, date: Option<NaiveDate>) -> ApiResult<NaiveDate> {
    if let Some(d) = date {
        return Ok(d);
    }
    sqlx::query_scalar::<_, NaiveDate>("SELECT (NOW() AT TIME ZONE 'Asia/Bangkok')::date")
        .fetch_one(&state.new_pool)
        .await
        .map_err(|e| ApiError::Internal(format!("failed to resolve today's date: {e}")))
}

/// Decode failure on a roster read ⇒ 500 (schema drift, not a client mistake).
fn map_row_err(e: sqlx::Error) -> ApiError {
    ApiError::Internal(format!("failed to map roster row: {e}"))
}

// =============================================================================
// Fetchers
// =============================================================================

async fn fetch_stay(
    pool: &PgPool,
    branch_label: &str,
    as_of: NaiveDate,
    kind: StayKind,
) -> ApiResult<Vec<StayRosterRow>> {
    let sql = format!("{STAY_SELECT}{}{STAY_ORDER}", kind.where_clause());
    let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(as_of)
        .fetch_all(pool)
        .await
        .map_err(|e| ApiError::Internal(format!("failed to load roster: {e}")))?;

    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        out.push(StayRosterRow {
            branch: branch_label.to_string(),
            cin_id: row.try_get("cin_id").map_err(map_row_err)?,
            cin_no: row.try_get("cin_no").map_err(map_row_err)?,
            booking_no: row.try_get("book_no").ok(),
            customer_name: row.try_get("customer_name").ok(),
            phone: row.try_get("cust_phone").ok(),
            room_no: row.try_get("room_no").ok(),
            room_type_name: row.try_get("type_name").ok(),
            check_in_time: row.try_get("cin_checkin_time").ok(),
            check_out_time: row.try_get("cin_checkout_time").ok(),
            expected_checkout: row.try_get("cin_expected_checkout").ok(),
            adults: row.try_get("cin_adults").ok(),
            children: row.try_get("cin_children").ok(),
            nights: row.try_get("nights").ok(),
            rate_per_night: row.try_get("cin_rate_per_night").ok(),
            total_amount: row.try_get("cin_total_amount").ok(),
            notes: row.try_get("cin_notes").ok(),
        });
    }
    Ok(out)
}

async fn fetch_arrivals(
    pool: &PgPool,
    branch_label: &str,
    as_of: NaiveDate,
) -> ApiResult<Vec<ArrivalRosterRow>> {
    let rows = sqlx::query(ARRIVALS_SQL)
        .bind(as_of)
        .fetch_all(pool)
        .await
        .map_err(|e| ApiError::Internal(format!("failed to load arrivals roster: {e}")))?;

    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        out.push(ArrivalRosterRow {
            branch: branch_label.to_string(),
            book_id: row.try_get("book_id").map_err(map_row_err)?,
            book_no: row.try_get("book_no").map_err(map_row_err)?,
            customer_name: row.try_get("customer_name").ok(),
            phone: row.try_get("cust_phone").ok(),
            check_in: row.try_get("book_checkin").ok(),
            check_out: row.try_get("book_checkout").ok(),
            adults: row.try_get("book_adults").ok(),
            children: row.try_get("book_children").ok(),
            nights: row.try_get("book_nights").ok(),
            room_count: row.try_get("room_count").unwrap_or(0),
            room_nos: row.try_get("room_nos").ok(),
            status: row.try_get("book_status").map_err(map_row_err)?,
        });
    }
    Ok(out)
}

// =============================================================================
// Handlers
// =============================================================================

/// Shared body for the three check-in-anchored rosters.
async fn stay_roster(
    state: AppState,
    query: RosterQuery,
    kind: StayKind,
) -> ApiResult<Json<StayRosterResponse>> {
    let as_of = resolve_as_of(&state, query.date).await?;
    let pools = resolve_pools(&state, query.branch.unwrap_or_default())?;

    let mut data: Vec<StayRosterRow> = Vec::new();
    for (label, pool) in pools {
        data.extend(fetch_stay(pool, label, as_of, kind).await?);
    }

    let count = i32::try_from(data.len()).unwrap_or(i32::MAX);
    Ok(Json(StayRosterResponse {
        success: true,
        date: as_of,
        count,
        data,
    }))
}

/// `GET /api/rosters/in-house` — guests currently staying (full list).
pub async fn in_house_roster(
    State(state): State<AppState>,
    Query(query): Query<RosterQuery>,
) -> ApiResult<Json<StayRosterResponse>> {
    stay_roster(state, query, StayKind::InHouse).await
}

/// `GET /api/rosters/departures` — guests departing on the as-of date (full list).
pub async fn departures_roster(
    State(state): State<AppState>,
    Query(query): Query<RosterQuery>,
) -> ApiResult<Json<StayRosterResponse>> {
    stay_roster(state, query, StayKind::Departures).await
}

/// `GET /api/rosters/continuing` — stayovers: arrived earlier, not leaving today.
pub async fn continuing_roster(
    State(state): State<AppState>,
    Query(query): Query<RosterQuery>,
) -> ApiResult<Json<StayRosterResponse>> {
    stay_roster(state, query, StayKind::Continuing).await
}

/// `GET /api/rosters/arrivals` — expected arrivals (bookings) for the as-of date.
pub async fn arrivals_roster(
    State(state): State<AppState>,
    Query(query): Query<RosterQuery>,
) -> ApiResult<Json<ArrivalRosterResponse>> {
    let as_of = resolve_as_of(&state, query.date).await?;
    let pools = resolve_pools(&state, query.branch.unwrap_or_default())?;

    let mut data: Vec<ArrivalRosterRow> = Vec::new();
    for (label, pool) in pools {
        data.extend(fetch_arrivals(pool, label, as_of).await?);
    }

    let count = i32::try_from(data.len()).unwrap_or(i32::MAX);
    Ok(Json(ArrivalRosterResponse {
        success: true,
        date: as_of,
        count,
        data,
    }))
}

/// `GET /api/rosters/summary` — the daily counts. For `all`, the per-site counts
/// are summed.
pub async fn roster_summary(
    State(state): State<AppState>,
    Query(query): Query<RosterQuery>,
) -> ApiResult<Json<RosterSummaryResponse>> {
    let as_of = resolve_as_of(&state, query.date).await?;
    let pools = resolve_pools(&state, query.branch.unwrap_or_default())?;

    let (mut arrivals, mut in_house, mut in_house_guests, mut departures, mut continuing) =
        (0, 0, 0, 0, 0);
    for (_, pool) in pools {
        let row = sqlx::query(SUMMARY_SQL)
            .bind(as_of)
            .fetch_one(pool)
            .await
            .map_err(|e| ApiError::Internal(format!("failed to load roster summary: {e}")))?;
        arrivals += row.try_get::<i32, _>("arrivals").map_err(map_row_err)?;
        in_house += row.try_get::<i32, _>("in_house").map_err(map_row_err)?;
        in_house_guests += row.try_get::<i32, _>("in_house_guests").map_err(map_row_err)?;
        departures += row.try_get::<i32, _>("departures").map_err(map_row_err)?;
        continuing += row.try_get::<i32, _>("continuing").map_err(map_row_err)?;
    }

    Ok(Json(RosterSummaryResponse {
        success: true,
        date: as_of,
        arrivals,
        in_house,
        departures,
        continuing,
        in_house_guests,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Lock the per-roster WHERE fragments so the in-house / departures /
    /// continuing definitions don't silently drift apart (they share a SELECT).
    #[test]
    fn where_clauses_are_distinct_and_bind_only_dollar_one() {
        for k in [StayKind::InHouse, StayKind::Departures, StayKind::Continuing] {
            let w = k.where_clause();
            assert!(w.starts_with("WHERE ci.cin_status IN ('active','checkedin')"));
            assert!(w.contains("$1"));
            // No other bind placeholder may sneak in (the date is the only param).
            assert!(!w.contains("$2"));
        }
        assert_ne!(
            StayKind::InHouse.where_clause(),
            StayKind::Departures.where_clause()
        );
        assert_ne!(
            StayKind::Departures.where_clause(),
            StayKind::Continuing.where_clause()
        );
    }

    /// The summary serializes camelCase (the frontend reads `inHouse`,
    /// `inHouseGuests`, …); a regression to snake_case would blank the cards.
    #[test]
    fn summary_serializes_camel_case() {
        let resp = RosterSummaryResponse {
            success: true,
            date: NaiveDate::from_ymd_opt(2026, 6, 27).unwrap(),
            arrivals: 1,
            in_house: 2,
            departures: 3,
            continuing: 4,
            in_house_guests: 5,
        };
        let v = serde_json::to_value(&resp).unwrap();
        assert_eq!(v.get("inHouse").and_then(|x| x.as_i64()), Some(2));
        assert_eq!(v.get("inHouseGuests").and_then(|x| x.as_i64()), Some(5));
        assert!(v.get("in_house").is_none(), "snake_case must not be emitted");
    }
}
