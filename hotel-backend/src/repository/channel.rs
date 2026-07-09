//! Loyalty-channel repository — PG-only reads/writes backing the loyalty-app
//! booking channel (`routes::channel` / `service::channel`) and the checkout
//! stay hook (`service::loyalty`). See `docs/loyalty-channel.md`.
//!
//! Free functions rather than a per-aggregate trait: every query here is a
//! cross-aggregate read (room types × rooms × bookings × check-ins) or a
//! channel-specific projection with a single PG implementation — a trait
//! would add indirection with nothing to swap. Mutating statements take
//! `&mut Transaction<'_, Postgres>` (service composes them with events);
//! reads take `&PgPool`. No MSSQL, no HTTP types (architecture.md §2).
//!
//! ## Availability semantics (must stay aligned with `room_is_available`)
//!
//! A room is FREE for `[check_in, check_out)` (half-open — the checkout day
//! is bookable) iff:
//! * `room_active` and not under maintenance (`room_maintenance` flag or
//!   `room_status='maintenance'`) — a stricter predicate than
//!   `routes::new_bookings::room_is_available` (which trusts the picker to
//!   exclude maintenance rooms; the channel has no human picker), and
//! * no overlapping booking with `book_status IN ('confirmed','pending')`,
//!   and
//! * no overlapping non-cancelled check-in (via the deprecated
//!   `cin_room_id` OR the canonical `ht_checkin_rooms` junction) —
//!   byte-for-byte the overlap predicate of `room_is_available` /
//!   `validate_booking` (spike Phase 3), generalized across rooms.

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use sqlx::{PgPool, Postgres, Transaction};

/// One room type's availability for a requested stay window.
#[derive(Debug, Clone)]
pub struct RoomTypeAvailability {
    pub type_id: i32,
    pub name: String,
    pub description: Option<String>,
    /// Quoted nightly price in baht (`ht_room_types.type_base_price`).
    pub nightly_price: f64,
    pub available_count: i64,
}

/// The free room the channel picked for a hold.
#[derive(Debug, Clone)]
pub struct PickedRoom {
    pub room_id: i32,
    pub room_no: String,
    pub type_name: String,
}

/// Channel-relevant projection of one `ht_bookings` row.
#[derive(Debug, Clone)]
pub struct ChannelBookingRow {
    pub book_id: i32,
    pub book_no: String,
    pub status: String,
    pub channel: Option<String>,
    pub customer_id: i32,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub total_amount: f64,
    pub deposit_amount: f64,
    pub hold_expires_at: Option<DateTime<Utc>>,
}

/// Checkout snapshot for the loyalty stay hook (`service::loyalty`).
#[derive(Debug, Clone)]
pub struct StaySnapshot {
    pub cin_status: String,
    pub membership_id: Option<String>,
    pub check_in_time: NaiveDateTime,
    pub check_out_time: Option<NaiveDateTime>,
    /// `ht_checkins.cin_expected_checkout` is NOT NULL in the canonical
    /// schema — the fallback when the actual checkout timestamp is absent.
    pub expected_checkout: NaiveDate,
}

/// Shared free-room predicate. `$1=check_in, $2=check_out` (both DATE) must
/// be bound by every query that interpolates this. Kept as a `const` so the
/// aggregation query and the picker can never drift apart.
const FREE_ROOM_PREDICATE: &str = r#"
      r.room_active = true
      AND COALESCE(r.room_maintenance, false) = false
      AND COALESCE(r.room_status, '') <> 'maintenance'
      AND NOT EXISTS (
            SELECT 1 FROM ht_booking_rooms br
              JOIN ht_bookings b ON b.book_id = br.br_book_id
             WHERE br.br_room_id = r.room_id
               AND b.book_status IN ('confirmed','pending')
               AND b.book_checkin  < $2
               AND b.book_checkout > $1
      )
      AND NOT EXISTS (
            SELECT 1 FROM ht_checkins c
             WHERE c.cin_status <> 'cancelled'
               AND (c.cin_room_id = r.room_id OR EXISTS(
                     SELECT 1 FROM ht_checkin_rooms cr
                      WHERE cr.cr_cin_id = c.cin_id AND cr.cr_room_id = r.room_id))
               AND c.cin_checkin_time::date < $2
               AND COALESCE(c.cin_checkout_time, c.cin_expected_checkout)::date > $1
      )
"#;

/// Per-type availability for `[check_in, check_out)`, optionally filtered to
/// types that sleep at least `guests`. Types with zero free rooms are
/// included (`available_count = 0`) so the loyalty app can show sold-out
/// states. Runtime `sqlx::query` — the SQL interpolates the shared
/// [`FREE_ROOM_PREDICATE`] const (static, no user input).
pub async fn availability_by_type(
    pool: &PgPool,
    check_in: NaiveDate,
    check_out: NaiveDate,
    guests: i32,
) -> Result<Vec<RoomTypeAvailability>, sqlx::Error> {
    use sqlx::Row;

    let sql = format!(
        r#"
        WITH free_rooms AS (
            SELECT r.room_type_id, r.room_id
              FROM ht_rooms_new r
             WHERE {FREE_ROOM_PREDICATE}
        )
        SELECT rt.type_id,
               rt.type_name,
               rt.type_description,
               COALESCE(rt.type_base_price, 0)::float8 AS nightly_price,
               COUNT(f.room_id)::int8 AS available_count
          FROM ht_room_types rt
          LEFT JOIN free_rooms f ON f.room_type_id = rt.type_id
         WHERE COALESCE(rt.type_active, true) = true
           AND ($3::int IS NULL OR rt.type_max_guests IS NULL OR rt.type_max_guests >= $3)
         GROUP BY rt.type_id, rt.type_name, rt.type_description,
                  rt.type_base_price, rt.type_sort_order
         ORDER BY rt.type_sort_order NULLS LAST, rt.type_id
        "#
    );

    let rows = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(check_in)
        .bind(check_out)
        .bind(guests)
        .fetch_all(pool)
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| RoomTypeAvailability {
            type_id: row.try_get("type_id").unwrap_or(0),
            name: row.try_get("type_name").unwrap_or_default(),
            description: row.try_get("type_description").ok(),
            nightly_price: row.try_get("nightly_price").unwrap_or(0.0),
            available_count: row.try_get("available_count").unwrap_or(0),
        })
        .collect())
}

/// Pick one free room of `type_id` for the stay window (lowest room number
/// first — deterministic, matches the desk habit of filling low rooms first).
/// `None` ⇒ the type is sold out for the window.
///
/// NOTE: pick → create is not serialized against a concurrent pick of the
/// same room (same race window the walk-in / booking form has today; the
/// existing create path accepts it and the shadow validator observes it).
pub async fn pick_free_room(
    pool: &PgPool,
    type_id: i32,
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> Result<Option<PickedRoom>, sqlx::Error> {
    use sqlx::Row;

    let sql = format!(
        r#"
        SELECT r.room_id, r.room_no, rt.type_name
          FROM ht_rooms_new r
          JOIN ht_room_types rt ON rt.type_id = r.room_type_id
         WHERE r.room_type_id = $3
           AND {FREE_ROOM_PREDICATE}
         ORDER BY r.room_no
         LIMIT 1
        "#
    );

    let row = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(check_in)
        .bind(check_out)
        .bind(type_id)
        .fetch_optional(pool)
        .await?;

    Ok(row.map(|row| PickedRoom {
        room_id: row.try_get("room_id").unwrap_or(0),
        room_no: row.try_get("room_no").unwrap_or_default(),
        type_name: row.try_get("type_name").unwrap_or_default(),
    }))
}

/// Quoted nightly price for a room type (baht). `None` ⇒ unknown type.
pub async fn type_nightly_price(
    pool: &PgPool,
    type_id: i32,
) -> Result<Option<(String, f64)>, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        SELECT type_name,
               COALESCE(type_base_price, 0)::float8 AS "nightly_price!"
          FROM ht_room_types
         WHERE type_id = $1 AND COALESCE(type_active, true) = true
        "#,
        type_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(rec.map(|r| (r.type_name, r.nightly_price)))
}

/// Load the channel projection of a booking. Plain read (no lock).
pub async fn get_channel_booking(
    pool: &PgPool,
    book_id: i32,
) -> Result<Option<ChannelBookingRow>, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        SELECT book_id,
               book_no,
               COALESCE(book_status, 'pending') AS "status!",
               book_channel,
               book_cust_id,
               book_checkin,
               book_checkout,
               COALESCE(book_total_amount, 0)::float8 AS "total_amount!",
               COALESCE(book_deposit_amount, 0)::float8 AS "deposit_amount!",
               book_hold_expires_at
          FROM ht_bookings
         WHERE book_id = $1
        "#,
        book_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(rec.map(|r| ChannelBookingRow {
        book_id: r.book_id,
        book_no: r.book_no,
        status: r.status,
        channel: r.book_channel,
        customer_id: r.book_cust_id,
        check_in: r.book_checkin,
        check_out: r.book_checkout,
        total_amount: r.total_amount,
        deposit_amount: r.deposit_amount,
        hold_expires_at: r.book_hold_expires_at,
    }))
}

/// Same projection, `FOR UPDATE` inside the caller's transaction — serializes
/// payment-verified against a concurrent release/sweep of the same hold.
pub async fn lock_channel_booking(
    tx: &mut Transaction<'_, Postgres>,
    book_id: i32,
) -> Result<Option<ChannelBookingRow>, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        SELECT book_id,
               book_no,
               COALESCE(book_status, 'pending') AS "status!",
               book_channel,
               book_cust_id,
               book_checkin,
               book_checkout,
               COALESCE(book_total_amount, 0)::float8 AS "total_amount!",
               COALESCE(book_deposit_amount, 0)::float8 AS "deposit_amount!",
               book_hold_expires_at
          FROM ht_bookings
         WHERE book_id = $1
           FOR UPDATE
        "#,
        book_id
    )
    .fetch_optional(&mut **tx)
    .await?;

    Ok(rec.map(|r| ChannelBookingRow {
        book_id: r.book_id,
        book_no: r.book_no,
        status: r.status,
        channel: r.book_channel,
        customer_id: r.book_cust_id,
        check_in: r.book_checkin,
        check_out: r.book_checkout,
        total_amount: r.total_amount,
        deposit_amount: r.deposit_amount,
        hold_expires_at: r.book_hold_expires_at,
    }))
}

/// Flip a `pending` hold to `confirmed`, recording the received deposit.
/// Guarded on `book_status='pending'` so a raced release/sweep loses cleanly
/// (0 rows). Returns rows affected.
pub async fn confirm_booking_payment(
    tx: &mut Transaction<'_, Postgres>,
    book_id: i32,
    deposit_baht: f64,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE ht_bookings
           SET book_status = 'confirmed',
               book_deposit_amount = $2::float8,
               book_deposit_date = NOW(),
               updated_at = NOW()
         WHERE book_id = $1
           AND book_status = 'pending'
        "#,
        book_id,
        deposit_baht
    )
    .execute(&mut **tx)
    .await?;

    Ok(result.rows_affected())
}

/// Cancel a hold — like `BookingRepository::cancel` but guarded on
/// `book_status='pending'` (NOT merely non-terminal) so a release replay or
/// an expiry sweep can never cancel a hold that a racing payment-verified
/// just confirmed. Also stamps the cancel metadata columns. Returns rows
/// affected (0 ⇒ the guard lost — caller re-reads and maps the outcome).
pub async fn release_hold(
    tx: &mut Transaction<'_, Postgres>,
    book_id: i32,
    reason: &str,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        UPDATE ht_bookings
           SET book_status = 'cancelled',
               book_cancelled_at = NOW(),
               book_cancel_reason = $2,
               updated_at = NOW()
         WHERE book_id = $1
           AND book_status = 'pending'
        "#,
        book_id,
        reason
    )
    .execute(&mut **tx)
    .await?;

    Ok(result.rows_affected())
}

/// Loyalty-channel holds whose payment window has lapsed (sweep input).
/// Bounded batch so one tick can't stall on a pathological backlog.
pub async fn expired_hold_ids(pool: &PgPool) -> Result<Vec<i32>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT book_id
          FROM ht_bookings
         WHERE book_channel = 'loyalty'
           AND book_status = 'pending'
           AND book_hold_expires_at IS NOT NULL
           AND book_hold_expires_at < NOW()
         ORDER BY book_id
         LIMIT 100
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows.into_iter().map(|r| r.book_id).collect())
}

/// Checkout snapshot for the loyalty stay hook: stay status + dates + the
/// guest's membership link. `None` ⇒ unknown check-in.
pub async fn stay_snapshot_for_loyalty(
    pool: &PgPool,
    cin_id: i32,
) -> Result<Option<StaySnapshot>, sqlx::Error> {
    let rec = sqlx::query!(
        r#"
        SELECT COALESCE(c.cin_status, '') AS "cin_status!",
               cu.cust_membership_id,
               c.cin_checkin_time,
               c.cin_checkout_time,
               c.cin_expected_checkout
          FROM ht_checkins c
          JOIN ht_customers cu ON cu.cust_id = c.cin_cust_id
         WHERE c.cin_id = $1
        "#,
        cin_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(rec.map(|r| StaySnapshot {
        cin_status: r.cin_status,
        membership_id: r.cust_membership_id,
        check_in_time: r.cin_checkin_time,
        check_out_time: r.cin_checkout_time,
        expected_checkout: r.cin_expected_checkout,
    }))
}

/// Attach a membership id to a guest profile if it differs (loyalty-channel
/// booking with `membership_id` present). Last-write-wins by design — the
/// member booked while signed in, which is at least as strong a signal as a
/// desk entry. Standalone statement (own connection, not the booking tx):
/// losing the link on a crash is acceptable; blocking the hold is not.
pub async fn attach_membership(
    pool: &PgPool,
    cust_id: i32,
    membership_id: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE ht_customers
           SET cust_membership_id = $1,
               updated_at = NOW()
         WHERE cust_id = $2
           AND (cust_membership_id IS DISTINCT FROM $1)
        "#,
        membership_id,
        cust_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
