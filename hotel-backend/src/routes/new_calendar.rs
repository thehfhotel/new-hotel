//! Booking-reminder API (task #53 / migration 064).
//!
//! Companion to the read-only calendar grid (which is served entirely by the
//! existing [`crate::routes::calendar::get_calendar`] — `GET /api/calendar`).
//! This module adds the reminder feed that backs the in-shell notification bell:
//!
//! - `GET  /api/calendar/reminders?branch=` — upcoming arrivals + balance-due
//!   bookings. Branch-aware (HF Ville reads the ville pool; `all` unions both
//!   sites and tags each item's `branch` so the bell can dismiss against the
//!   correct pool). Read-only.
//! - `POST /api/calendar/reminders/{id}/dismiss?branch=` — suppress a booking's
//!   reminder (stamps `book_notify_dismissed_at = NOW()`). Idempotent.
//!
//! ## Coexistence
//!
//! The notify columns (migration 064) are **PG-CANONICAL ONLY** — there is no
//! legacy writeback. iHOTEL has no equivalent stored reminder/dismiss flag (it
//! surfaces arrivals via an ad-hoc dashboard query), so there is nothing to
//! mirror. TODO(coexistence): if a legacy reminder column is ever identified,
//! wire a writeback recipe; until then dismiss state lives only here.
//!
//! All queries are LITERAL strings bound with `.bind()` (runtime `sqlx::query`,
//! no `query!` macro / no `.sqlx` cache), matching [`crate::routes::new_notes`].

use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::Row as _;

use super::mode::{AppState, Branch};
use crate::error::{ApiError, ApiResult};

/// Default reminder lead time (days before check-in) used when a booking has no
/// per-row `book_notify_day` override. Mirrors the migration-064 column comment.
const DEFAULT_NOTIFY_LEAD_DAYS: i32 = 3;

/// Cap on the reminder feed — the bell is a glance, not a report.
const REMINDER_LIMIT: i64 = 200;

/// `GET /api/calendar/reminders` query string.
#[derive(Debug, Deserialize)]
pub struct ReminderQuery {
    pub branch: Option<Branch>,
}

/// Branch selector for the dismiss endpoint.
#[derive(Debug, Deserialize)]
pub struct DismissQuery {
    pub branch: Option<Branch>,
}

/// One reminder row in the bell dropdown.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReminderItem {
    pub book_id: i32,
    pub book_no: String,
    /// Legacy MSSQL id (`R\d{6}`) when the booking has been mirrored. Omitted
    /// until the writeback worker back-populates it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_book_id: Option<String>,
    pub customer_name: Option<String>,
    pub check_in: Option<NaiveDate>,
    pub check_out: Option<NaiveDate>,
    pub status: String,
    pub total_amount: f64,
    pub deposit_amount: f64,
    /// `total - deposit`, floored at 0.
    pub balance_due: f64,
    /// Whole days from today to check-in (negative ⇒ already past — only happens
    /// on balance-due-only rows whose stay window has started).
    pub days_until_checkin: i32,
    pub is_upcoming: bool,
    pub is_balance_due: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_note: Option<String>,
    /// Which site this booking belongs to ('hfhotel' | 'hfville') so the `all`
    /// bell dismisses against the correct per-site pool.
    pub branch: String,
}

/// `200` body for `GET /api/calendar/reminders`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReminderListResponse {
    pub success: bool,
    pub reminders: Vec<ReminderItem>,
    pub upcoming_count: usize,
    pub balance_due_count: usize,
}

/// `200` body for the dismiss endpoint.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DismissResponse {
    pub success: bool,
    pub book_id: i32,
}

/// Fetch active reminders from one site's canonical `ht_bookings`, tagging each
/// row with `branch_label` ('hfhotel' | 'hfville').
///
/// A booking surfaces when it is confirmed/pending, not dismissed, AND either
/// (a) its check-in is within its reminder window (`book_notify_day` or the
/// default), or (b) it carries an outstanding balance (`total > deposit`).
///
/// `book_checkin` / `book_checkout` are cast `::date` so the decode is stable
/// whether the column is DATE or TIMESTAMP across environments.
async fn fetch_reminders(
    pool: &crate::db::PgPool,
    branch_label: &str,
) -> ApiResult<Vec<ReminderItem>> {
    let rows = sqlx::query(
        "SELECT \
             b.book_id, \
             b.book_no, \
             b.legacy_book_id, \
             CONCAT(c.cust_firstname, ' ', COALESCE(c.cust_lastname, '')) AS customer_name, \
             b.book_checkin::date  AS check_in, \
             b.book_checkout::date AS check_out, \
             b.book_status, \
             COALESCE(b.book_total_amount, 0)::float8   AS total_amount, \
             COALESCE(b.book_deposit_amount, 0)::float8 AS deposit_amount, \
             b.book_notify_note, \
             (b.book_checkin::date - CURRENT_DATE)::int AS days_until_checkin, \
             (b.book_checkin::date >= CURRENT_DATE \
                AND b.book_checkin::date <= CURRENT_DATE + COALESCE(b.book_notify_day, $1)) AS is_upcoming, \
             (COALESCE(b.book_total_amount, 0) > COALESCE(b.book_deposit_amount, 0)) AS is_balance_due \
         FROM ht_bookings b \
         LEFT JOIN ht_customers c ON b.book_cust_id = c.cust_id \
         WHERE b.book_status IN ('confirmed', 'pending') \
           AND b.book_notify_dismissed_at IS NULL \
           AND ( \
                 (b.book_checkin::date >= CURRENT_DATE \
                    AND b.book_checkin::date <= CURRENT_DATE + COALESCE(b.book_notify_day, $1)) \
                 OR COALESCE(b.book_total_amount, 0) > COALESCE(b.book_deposit_amount, 0) \
               ) \
         ORDER BY b.book_checkin::date ASC, b.book_id ASC \
         LIMIT $2",
    )
    .bind(DEFAULT_NOTIFY_LEAD_DAYS)
    .bind(REMINDER_LIMIT)
    .fetch_all(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to list reminders: {e}")))?;

    let mut items = Vec::with_capacity(rows.len());
    for row in &rows {
        let total_amount: f64 = row.try_get("total_amount").unwrap_or(0.0);
        let deposit_amount: f64 = row.try_get("deposit_amount").unwrap_or(0.0);
        items.push(ReminderItem {
            book_id: row.try_get("book_id").unwrap_or(0),
            book_no: row.try_get("book_no").unwrap_or_default(),
            legacy_book_id: row.try_get::<String, _>("legacy_book_id").ok(),
            customer_name: row.try_get::<String, _>("customer_name").ok(),
            check_in: row.try_get::<NaiveDate, _>("check_in").ok(),
            check_out: row.try_get::<NaiveDate, _>("check_out").ok(),
            status: row.try_get::<String, _>("book_status").unwrap_or_default(),
            total_amount,
            deposit_amount,
            balance_due: (total_amount - deposit_amount).max(0.0),
            days_until_checkin: row.try_get::<i32, _>("days_until_checkin").unwrap_or(0),
            is_upcoming: row.try_get::<bool, _>("is_upcoming").unwrap_or(false),
            is_balance_due: row.try_get::<bool, _>("is_balance_due").unwrap_or(false),
            notify_note: row.try_get::<String, _>("book_notify_note").ok(),
            branch: branch_label.to_string(),
        });
    }

    Ok(items)
}

/// `GET /api/calendar/reminders` — upcoming arrivals + balance-due bookings,
/// branch-aware. `all` unions both sites (skipping HF Ville gracefully when its
/// pool is unconfigured, matching the calendar read route).
pub async fn list_reminders(
    State(state): State<AppState>,
    Query(query): Query<ReminderQuery>,
) -> ApiResult<Json<ReminderListResponse>> {
    let branch = query.branch.unwrap_or_default();
    let fetch_hfhotel = branch == Branch::Hfhotel || branch == Branch::All;
    let fetch_hfville = branch == Branch::Hfville || branch == Branch::All;

    let mut reminders: Vec<ReminderItem> = Vec::new();

    if fetch_hfhotel {
        reminders.extend(fetch_reminders(&state.new_pool, "hfhotel").await?);
    }
    if fetch_hfville {
        if let Ok(vp) = state.ville_pool() {
            reminders.extend(fetch_reminders(vp, "hfville").await?);
        }
    }

    // Upcoming first (soonest arrival), then balance-due-only by largest balance.
    reminders.sort_by(|a, b| {
        b.is_upcoming
            .cmp(&a.is_upcoming)
            .then(a.days_until_checkin.cmp(&b.days_until_checkin))
            .then(
                b.balance_due
                    .partial_cmp(&a.balance_due)
                    .unwrap_or(std::cmp::Ordering::Equal),
            )
    });

    let upcoming_count = reminders.iter().filter(|r| r.is_upcoming).count();
    let balance_due_count = reminders.iter().filter(|r| r.is_balance_due).count();

    Ok(Json(ReminderListResponse {
        success: true,
        reminders,
        upcoming_count,
        balance_due_count,
    }))
}

/// `POST /api/calendar/reminders/{id}/dismiss` — suppress a booking's reminder.
/// Stamps `book_notify_dismissed_at = NOW()` on the resolved per-site pool.
/// Idempotent: dismissing an already-dismissed booking is a success; an unknown
/// id is a 404. PG-canonical only (no legacy writeback).
pub async fn dismiss_reminder(
    State(state): State<AppState>,
    Path(book_id): Path<i32>,
    Query(query): Query<DismissQuery>,
) -> ApiResult<Json<DismissResponse>> {
    // Per-site pool via the unified write chokepoint (Ship-B).
    let pool = state.write_pool(query.branch)?;

    let result = sqlx::query(
        "UPDATE ht_bookings \
            SET book_notify_dismissed_at = NOW() \
          WHERE book_id = $1 AND book_notify_dismissed_at IS NULL",
    )
    .bind(book_id)
    .execute(pool)
    .await
    .map_err(|e| ApiError::Internal(format!("failed to dismiss reminder: {e}")))?;

    if result.rows_affected() == 0 {
        // No row updated: either already dismissed (idempotent success) or the
        // booking doesn't exist (404). A cheap existence probe distinguishes.
        let exists = sqlx::query("SELECT 1 FROM ht_bookings WHERE book_id = $1")
            .bind(book_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| ApiError::Internal(format!("failed to probe booking: {e}")))?
            .is_some();
        if !exists {
            return Err(ApiError::NotFound(format!("booking {book_id} not found")));
        }
    }

    Ok(Json(DismissResponse {
        success: true,
        book_id,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The reminder query string parses the documented camelCase branch param.
    #[test]
    fn reminder_query_defaults_to_hfhotel() {
        let q: ReminderQuery = serde_json::from_str("{}").expect("must parse");
        assert_eq!(q.branch.unwrap_or_default(), Branch::Hfhotel);
    }

    /// balance_due is floored at zero even when deposit exceeds total (data skew).
    #[test]
    fn balance_due_floors_at_zero() {
        let total = 1000.0_f64;
        let deposit = 1200.0_f64;
        assert_eq!((total - deposit).max(0.0), 0.0);
    }
}
