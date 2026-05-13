//! Persisted Slack-notification watermark per (site, notification type).
//!
//! Backs the scheduler polling jobs in `scheduler::jobs` so that a
//! backend-container redeploy does NOT cause the post-restart poll to
//! re-page ~45 historical events (the symptom verified in production
//! on 2026-05-13).
//!
//! ## Root cause the watermark fixes
//!
//! 1. `SchedulerState` previously held `last_<event>_timestamp` in
//!    process memory only. A container restart cleared it.
//! 2. The first-poll branch seeded the watermark with
//!    `chrono::Utc::now().naive_utc()`. But the legacy MSSQL columns
//!    we filter on (`Cin_Room_In`, `Cin_Room_Out`, `Book_Date`) store
//!    **Thai local time** (GMT+7) as `NaiveDateTime` — see
//!    CLAUDE.md "Timezone Handling".
//! 3. Comparing a UTC-seeded watermark to a Thai-local column made
//!    every event from the previous 7 hours look "new" on the first
//!    post-deploy poll. At ~6/hour that's ~45 messages.
//!
//! ## Design
//!
//! Backed by `scheduler_notification_state(site_id, notification_type,
//! last_event_at, updated_at)` — migration 037. Three helpers:
//!
//! * [`now_thai_local`] — initial watermark on first-ever poll, in the
//!   same wall-clock zone the legacy columns use.
//! * [`load_watermark`] — read the persisted watermark on startup.
//! * [`save_watermark`] — write after each successful poll that
//!   advanced the in-memory watermark.
//!
//! The pure-function split lets the watermark math be unit-tested
//! without a live DB; the I/O helpers are exercised by the polling
//! integration tests.

use chrono::NaiveDateTime;
use sqlx::PgPool;

/// Notification types used as the composite-key second column.
///
/// Free-form text in the DB column so a future polling job can land
/// without a schema change — but the three current callers ALL go
/// through these constants so a typo at the call site fails the
/// `match` arm in `as_str()`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationType {
    Checkin,
    Checkout,
    Booking,
}

impl NotificationType {
    pub const fn as_str(self) -> &'static str {
        match self {
            NotificationType::Checkin => "checkin",
            NotificationType::Checkout => "checkout",
            NotificationType::Booking => "booking",
        }
    }
}

/// Wall-clock "now" in Thai local time (GMT+7), expressed as a
/// `NaiveDateTime` so it slots into the same comparison the polling
/// SQL does against `Cin_Room_In` / `Cin_Room_Out` / `Book_Date`.
///
/// Used as the seed watermark the very first time a poll runs for a
/// given (site, notification type) — i.e. the row is absent from
/// `scheduler_notification_state`. After that first persist, the
/// watermark is loaded from PG and this function is not consulted.
pub fn now_thai_local() -> NaiveDateTime {
    // FixedOffset is infallible at +7h; the cast cannot panic in any
    // production tz database. The whole expression is one tick — no
    // observable cost vs `Utc::now`.
    chrono::Utc::now()
        .with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).expect("GMT+7 is in range"))
        .naive_local()
}

/// Load the persisted watermark for (site, notification type).
///
/// Returns `Ok(None)` if no row exists yet — caller should seed with
/// [`now_thai_local`] and persist before doing any "WHERE event_ts > $1"
/// query. Returns `Err` only on DB connectivity / schema issues, which
/// the caller should LOG (not propagate) so the polling loop continues
/// — falling back to the previous in-memory behavior would just
/// re-introduce the replay bug.
pub async fn load_watermark(
    pg: &PgPool,
    site_id: &str,
    notification_type: NotificationType,
) -> Result<Option<NaiveDateTime>, sqlx::Error> {
    let row: Option<(NaiveDateTime,)> = sqlx::query_as(
        r#"
        SELECT last_event_at
          FROM scheduler_notification_state
         WHERE site_id = $1 AND notification_type = $2
        "#,
    )
    .bind(site_id)
    .bind(notification_type.as_str())
    .fetch_optional(pg)
    .await?;
    Ok(row.map(|(ts,)| ts))
}

/// Persist the watermark for (site, notification type). UPSERT.
///
/// The `updated_at` column is bumped by the BEFORE-UPDATE trigger so
/// callers don't need to set it. The PK is `(site_id, notification_type)`
/// — concurrent writes for the same key are serialized by PG.
pub async fn save_watermark(
    pg: &PgPool,
    site_id: &str,
    notification_type: NotificationType,
    last_event_at: NaiveDateTime,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO scheduler_notification_state
            (site_id, notification_type, last_event_at)
        VALUES ($1, $2, $3)
        ON CONFLICT (site_id, notification_type)
        DO UPDATE SET last_event_at = EXCLUDED.last_event_at
        "#,
    )
    .bind(site_id)
    .bind(notification_type.as_str())
    .bind(last_event_at)
    .execute(pg)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `now_thai_local` must produce a wall-clock value that lands in
    /// the same zone the legacy MSSQL columns use. Smoke-test: it must
    /// be roughly 7 hours ahead of UTC. We allow a generous +/- one
    /// second to absorb scheduling jitter between the two `now()` reads.
    #[test]
    fn now_thai_local_is_approximately_seven_hours_ahead_of_utc() {
        let thai = now_thai_local();
        let utc = chrono::Utc::now().naive_utc();
        let delta = thai.signed_duration_since(utc);
        let expected = chrono::Duration::hours(7);
        let drift = (delta - expected).num_milliseconds().abs();
        assert!(
            drift < 1_000,
            "Thai-local watermark drifted from UTC+7 by {drift}ms; \
             must be within 1s — otherwise the seeded watermark won't \
             match the wall-clock of MSSQL's Cin_Room_Out column"
        );
    }

    /// Notification-type strings are part of the DB schema (composite
    /// PK). Renaming them would orphan existing rows. Pin them.
    #[test]
    fn notification_type_str_is_stable_db_key() {
        assert_eq!(NotificationType::Checkin.as_str(), "checkin");
        assert_eq!(NotificationType::Checkout.as_str(), "checkout");
        assert_eq!(NotificationType::Booking.as_str(), "booking");
    }

    /// The three variants must round-trip to distinct DB keys — a
    /// collision would silently merge two watermarks under one row.
    #[test]
    fn notification_type_str_values_are_all_distinct() {
        let keys = [
            NotificationType::Checkin.as_str(),
            NotificationType::Checkout.as_str(),
            NotificationType::Booking.as_str(),
        ];
        let mut sorted = keys.to_vec();
        sorted.sort();
        sorted.dedup();
        assert_eq!(
            sorted.len(),
            keys.len(),
            "duplicate DB keys would merge watermarks"
        );
    }
}
