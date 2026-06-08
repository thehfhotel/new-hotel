//! Calendar API route with hybrid database support
//!
//! - GET /api/calendar - Hybrid calendar endpoint fetching from both databases
//!
//! Reads legacy data from PG (`ht_bookings_legacy` + `ht_checkins_legacy` cache,
//! fed by drift-reconcile + CT mappers).

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Row;

use crate::error::{ApiError, ApiResult};
use super::mode::{AppState, Branch, SystemMode};

/// Query parameters for calendar endpoint
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarQuery {
    pub start_date: String,
    pub end_date: String,
    pub branch: Option<Branch>,
}

/// Source database for a calendar entry
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DataSource {
    Legacy,
    New,
}

/// A booking entry for the calendar
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarBooking {
    pub id: String,
    pub booking_no: String,
    pub customer_name: Option<String>,
    pub room_no: Option<String>,
    pub check_in: Option<DateTime<Utc>>,
    pub check_out: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub source: DataSource,
}

/// A check-in entry for the calendar
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarCheckin {
    pub id: String,
    pub checkin_no: String,
    pub customer_name: Option<String>,
    pub room_no: Option<String>,
    pub check_in: Option<DateTime<Utc>>,
    pub check_out: Option<DateTime<Utc>>,
    pub source: DataSource,
}

/// Calendar data combining bookings and check-ins
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarData {
    pub bookings: Vec<CalendarBooking>,
    pub checkins: Vec<CalendarCheckin>,
}

/// Calendar response
#[derive(Debug, Serialize)]
pub struct CalendarResponse {
    pub success: bool,
    pub data: CalendarData,
    pub mode: SystemMode,
}

/// GET /api/calendar - Hybrid calendar endpoint
///
/// Fetches calendar data from both legacy and new databases.
/// In legacy mode, only legacy data is returned.
/// In new mode, data from both databases is returned with source indicators.
pub async fn get_calendar(
    State(state): State<AppState>,
    Query(params): Query<CalendarQuery>,
) -> ApiResult<Json<CalendarResponse>> {
    let mode = state.current_mode();
    let branch = params.branch.unwrap_or_default();

    // Validate date parameters
    if params.start_date.is_empty() || params.end_date.is_empty() {
        return Err(ApiError::BadRequest("startDate and endDate are required".to_string()));
    }

    let mut all_bookings: Vec<CalendarBooking> = Vec::new();
    let mut all_checkins: Vec<CalendarCheckin> = Vec::new();

    // Fetch data based on branch selection
    let fetch_hfhotel = branch == Branch::Hfhotel || branch == Branch::All;
    let fetch_hfville = branch == Branch::Hfville || branch == Branch::All;

    if fetch_hfhotel {
        // Coexistence audit T3 MED-3: in `SystemMode::New` the canonical
        // tables ARE the source of truth — fetching the legacy mirror in
        // addition produces duplicate rows with nondeterministic ordering
        // (the frontend dedup-by-key paper-overs most cases, but where the
        // mirror and canonical disagree on a field the displayed value
        // flipped between page loads). The legacy fetch now only runs in
        // `SystemMode::Legacy`; New mode reads canonical exclusively.
        match mode {
            SystemMode::Legacy => {
                let (legacy_bookings, legacy_checkins) = fetch_legacy_calendar_data_pg(
                    &state.new_pool,
                    &params.start_date,
                    &params.end_date,
                )
                .await?;

                all_bookings.extend(legacy_bookings);
                all_checkins.extend(legacy_checkins);
            }
            SystemMode::New => {
                let (new_bookings, new_checkins) = fetch_new_calendar_data(
                    &state.new_pool,
                    &params.start_date,
                    &params.end_date,
                )
                .await?;

                all_bookings.extend(new_bookings);
                all_checkins.extend(new_checkins);
            }
        }
    }

    if fetch_hfville {
        if let Ok(vp) = state.ville_pool() {
            // HF Ville canonical (`hotelville` PG database — see
            // `canonical_pg_split` memory) has the same schema as
            // `hotelnew`. The pre-rewrite path called
            // `fetch_legacy_calendar_data_pg` against Ville's pool,
            // which read `ht_bookings_legacy` + `ht_checkins_legacy`.
            // Ville's cache is missing ~16% of bookings and ~13% of
            // check-ins that exist in canonical (audit 2026-05-16), so
            // the calendar silently rendered an incomplete view for
            // operators. Use the canonical fetcher instead — same
            // shape, complete data.
            let (ville_bookings, ville_checkins) = fetch_new_calendar_data(
                vp,
                &params.start_date,
                &params.end_date,
            ).await?;

            // Remap IDs to avoid collisions with HF Hotel. The
            // canonical fetcher emits `new-booking-<id>` /
            // `new-checkin-<id>-<room>`; rewrite the `new-` prefix to
            // `ville-` so the frontend's per-site dedup still works.
            let ville_bookings: Vec<CalendarBooking> = ville_bookings.into_iter().map(|mut b| {
                b.id = b.id.replace("new-booking-", "ville-booking-");
                b
            }).collect();
            let ville_checkins: Vec<CalendarCheckin> = ville_checkins.into_iter().map(|mut c| {
                c.id = c.id.replace("new-checkin-", "ville-checkin-");
                c
            }).collect();

            all_bookings.extend(ville_bookings);
            all_checkins.extend(ville_checkins);
        }
    }

    Ok(Json(CalendarResponse {
        success: true,
        data: CalendarData {
            bookings: all_bookings,
            checkins: all_checkins,
        },
        mode,
    }))
}

/// Fetch legacy calendar data from PostgreSQL mirror tables
async fn fetch_legacy_calendar_data_pg(
    pool: &crate::db::PgPool,
    start_date: &str,
    end_date: &str,
) -> ApiResult<(Vec<CalendarBooking>, Vec<CalendarCheckin>)> {
    // Fetch bookings from PG mirror
    let booking_rows = sqlx::query(
        r#"
        SELECT book_no, book_cust_name, book_room_no, book_date_in, book_date_out, book_status
        FROM ht_bookings_legacy
        WHERE book_date_out::date >= $1::date
          AND book_date_in::date <= $2::date
        ORDER BY book_date_in
        "#,
    )
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await?;

    let bookings: Vec<CalendarBooking> = booking_rows
        .iter()
        .map(|row| {
            let book_no: String = row.try_get("book_no").unwrap_or_default();
            CalendarBooking {
                id: format!("legacy-booking-{}", book_no),
                booking_no: book_no,
                customer_name: row.try_get("book_cust_name").ok(),
                room_no: row.try_get("book_room_no").ok(),
                check_in: row.try_get::<NaiveDateTime, _>("book_date_in").ok().map(|dt| dt.and_utc()),
                check_out: row.try_get::<NaiveDateTime, _>("book_date_out").ok().map(|dt| dt.and_utc()),
                status: row.try_get::<i32, _>("book_status").ok().map(|s| map_legacy_status(s)),
                source: DataSource::Legacy,
            }
        })
        .collect();

    // Fetch check-ins from PG mirror
    let checkin_rows = sqlx::query(
        r#"
        SELECT cin_no, cin_cust_name, cin_room_no, cin_room_in, cin_room_out
        FROM ht_checkins_legacy
        WHERE cin_room_out::date >= $1::date
          AND cin_room_in::date <= $2::date
        ORDER BY cin_room_in
        "#,
    )
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await?;

    let checkins: Vec<CalendarCheckin> = checkin_rows
        .iter()
        .map(|row| {
            let cin_no: String = row.try_get("cin_no").unwrap_or_default();
            CalendarCheckin {
                id: format!("legacy-checkin-{}", cin_no),
                checkin_no: cin_no,
                customer_name: row.try_get("cin_cust_name").ok(),
                room_no: row.try_get("cin_room_no").ok(),
                check_in: row.try_get::<NaiveDateTime, _>("cin_room_in").ok().map(|dt| dt.and_utc()),
                check_out: row.try_get::<NaiveDateTime, _>("cin_room_out").ok().map(|dt| dt.and_utc()),
                source: DataSource::Legacy,
            }
        })
        .collect();

    Ok((bookings, checkins))
}

/// Fetch calendar data from new_hotel database (HotelNew)
///
/// Uses the schema defined in migrations/002_create_new_hotel_database.sql
async fn fetch_new_calendar_data(
    pool: &crate::db::PgPool,
    start_date: &str,
    end_date: &str,
) -> ApiResult<(Vec<CalendarBooking>, Vec<CalendarCheckin>)> {
    // Fetch bookings from HotelNew database (ht_bookings + ht_customers + ht_booking_rooms + ht_rooms_new)
    //
    // Calendar accuracy fix (2026-05-16): tightened the status filter
    // from `!= 'cancelled'` to `IN ('confirmed', 'pending')`. Pre-fix:
    //   * `'checked_in'` bookings (11k+ rows on HF Hotel) shipped to
    //     the frontend even though their guest is already in
    //     `ht_checkins`. The page-side name+date dedup paper-overed
    //     most cases but was fragile — duplicate-name customers (e.g.
    //     two "John Smith"s with overlapping booking windows) flipped
    //     each other off the chart.
    //   * `'completed'` bookings would never reach the chart because
    //     they're historical (booking lifecycle ended), but the old
    //     `!= 'cancelled'` filter kept them in the payload anyway.
    // The "future booking" bar specifically wants UPCOMING reservations,
    // which are exactly `confirmed` / `pending` status. The check-in
    // bar covers the past via `ht_checkins`.
    let booking_query = format!(
        r#"
        SELECT
            b.book_id,
            b.book_no,
            CONCAT(c.cust_firstname, ' ', COALESCE(c.cust_lastname, '')) AS cust_name,
            r.room_no,
            b.book_checkin,
            b.book_checkout,
            b.book_status
        FROM ht_bookings b
        INNER JOIN ht_customers c ON b.book_cust_id = c.cust_id
        LEFT JOIN ht_booking_rooms br ON b.book_id = br.br_book_id
        LEFT JOIN ht_rooms_new r ON br.br_room_id = r.room_id
        WHERE b.book_checkout >= '{}'
          AND b.book_checkin <= '{}'
          AND b.book_status IN ('confirmed', 'pending')
        ORDER BY b.book_checkin
        "#,
        start_date.replace('\'', "''"),
        end_date.replace('\'', "''")
    );

    let booking_rows = sqlx::query(sqlx::AssertSqlSafe(&*booking_query))
        .fetch_all(pool)
        .await?;

    let bookings: Vec<CalendarBooking> = booking_rows
        .iter()
        .map(|row| {
            let id = row.try_get::<i32, _>("book_id").unwrap_or_default();
            let booking_no = row.try_get::<String, _>("book_no").unwrap_or_default();
            CalendarBooking {
                id: format!("new-booking-{}", id),
                booking_no,
                customer_name: row.try_get::<String, _>("cust_name").ok(),
                room_no: row.try_get::<String, _>("room_no").ok(),
                check_in: row.try_get::<NaiveDateTime, _>("book_checkin").ok().map(|dt| dt.and_utc()),
                check_out: row.try_get::<NaiveDateTime, _>("book_checkout").ok().map(|dt| dt.and_utc()),
                status: row.try_get::<String, _>("book_status").ok(),
                source: DataSource::New,
            }
        })
        .collect();

    // Fetch check-ins from HotelNew database (ht_checkins + ht_customers
    // + ht_checkin_rooms junction + ht_rooms_new).
    //
    // ## Track B3 / T3 CRIT-2 — multi-room calendar entries
    //
    // Pre-B3 this query did `INNER JOIN ht_rooms_new ON cin_room_id`,
    // which (a) emitted exactly one row per folio regardless of how many
    // rooms were under it, and (b) combined with the frontend's
    // `seenIds` dedup, dropped rooms 2..N entirely (the dedup key was
    // the cin_id, which is identical across all rooms of a multi-room
    // folio). Joining through `ht_checkin_rooms` now emits one calendar
    // row per actual room AND yields a per-room synthetic key
    // (`new-checkin-<cin_id>-<room_id>`) so the frontend dedup keeps
    // every room.
    //
    // `LEFT JOIN ht_checkin_rooms + COALESCE` gives us:
    //   - 2 rows for a 2-room folio whose junction is populated (B2+);
    //   - 1 row for a legacy folio whose junction is empty (pre-B2),
    //     using header-level `cin_room_id` as the fallback per
    //     CARDINALITY_MAP's deprecation note.
    //
    // The fallback path is dead code after B5 backfill; the column
    // drops then.
    // Calendar accuracy fix (2026-05-16): exclude cancelled check-ins
    // from the occupancy count. `derive_room_state` writes
    // `cin_status='cancelled'` when the legacy header reads `'ยกเลิก'`
    // (per `legacy_status_to_pg`). Pre-fix those rows were rendered as
    // occupied days on the chart even though the room was never
    // actually used.
    let checkin_query = format!(
        r#"
        SELECT
            ci.cin_id,
            ci.cin_no,
            COALESCE(cr.cr_room_id, ci.cin_room_id) AS effective_room_id,
            CONCAT(c.cust_firstname, ' ', COALESCE(c.cust_lastname, '')) AS cust_name,
            r.room_no,
            ci.cin_checkin_time,
            COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout) AS cin_checkout
        FROM ht_checkins ci
        INNER JOIN ht_customers c ON ci.cin_cust_id = c.cust_id
        LEFT JOIN ht_checkin_rooms cr ON cr.cr_cin_id = ci.cin_id
        INNER JOIN ht_rooms_new r ON r.room_id = COALESCE(cr.cr_room_id, ci.cin_room_id)
        WHERE COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout) >= '{}'
          AND ci.cin_checkin_time <= '{}'
          AND ci.cin_status != 'cancelled'
        ORDER BY ci.cin_checkin_time, r.room_no
        "#,
        start_date.replace('\'', "''"),
        end_date.replace('\'', "''")
    );

    let checkin_rows = sqlx::query(sqlx::AssertSqlSafe(&*checkin_query))
        .fetch_all(pool)
        .await?;

    let checkins: Vec<CalendarCheckin> = checkin_rows
        .iter()
        .map(|row| {
            let id = row.try_get::<i32, _>("cin_id").unwrap_or_default();
            let effective_room_id = row.try_get::<i32, _>("effective_room_id").unwrap_or_default();
            let cin_no = row.try_get::<String, _>("cin_no").unwrap_or_default();
            CalendarCheckin {
                // Per-(folio, room) synthetic id so the frontend
                // `seenIds` dedup does not collapse rooms 2..N of a
                // multi-room folio to room 1 (T3 CRIT-2).
                id: format!("new-checkin-{}-{}", id, effective_room_id),
                checkin_no: cin_no,
                customer_name: row.try_get::<String, _>("cust_name").ok(),
                room_no: row.try_get::<String, _>("room_no").ok(),
                check_in: row.try_get::<NaiveDateTime, _>("cin_checkin_time").ok().map(|dt| dt.and_utc()),
                check_out: row.try_get::<NaiveDateTime, _>("cin_checkout").ok().map(|dt| dt.and_utc()),
                source: DataSource::New,
            }
        })
        .collect();

    Ok((bookings, checkins))
}

/// Map legacy booking status code to string
fn map_legacy_status(code: i32) -> String {
    match code {
        0 => "pending".to_string(),
        1 => "confirmed".to_string(),
        2 => "cancelled".to_string(),
        3 => "checked_in".to_string(),
        4 => "completed".to_string(),
        _ => "unknown".to_string(),
    }
}
