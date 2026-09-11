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
//!
//! ## Parked (roomless) bookings — B8a, then B8c
//!
//! The predicate above joins bookings through `ht_booking_rooms`, so a
//! **parked** booking — a live `ht_bookings` row with ZERO
//! `ht_booking_rooms` rows, the shape an OTA/desk reservation takes until a
//! human assigns the room (`service::booking::create`, the `cmd.rooms
//! .is_empty()` branch) and the shape the CT mapper persists for a
//! header-only `HT_Book_H` (`sync::mappers::booking`) — is **invisible** to
//! it. Every physical room still looks free while a parked booking already
//! claims one of them, so the channel could hold the property's last room.
//!
//! B8a fixed the oversell with an inventory-pressure term computed from the
//! SAME status set and the SAME half-open overlap rule, but — because
//! canonical then recorded no room type for a parked booking at all — it
//! could only cap property-wide:
//!
//! ```text
//! surplus            = max(free rooms property-wide − parked claims, 0)
//! available(type)    = min(free rooms of the type, surplus)
//! ```
//!
//! That never oversells, but it never blocks the RIGHT type either: a parked
//! claim on a Deluxe let the channel sell the property's last Deluxe as long
//! as some Standard was free.
//!
//! **B8c (migration 094)** supplies the missing fact —
//! `ht_bookings.book_room_type_id`, written by the desk create/edit path and
//! by the CT mapper (from `HT_Book_Ds.Book_Room_Type` on an iHOTEL
//! "ระบุประเภทห้อง" booking, `HT_Book_H.Book_room_type = 1`). Parked claims
//! therefore split in two and the rule becomes:
//!
//! ```text
//! surplus            = max(free rooms property-wide − ALL parked claims, 0)
//! available(type)    = min( max(free(type) − parked_typed(type), 0), surplus )
//! ```
//!
//! Read the two terms as what they are:
//!
//! * the **per-type** term SUBTRACTS — a parked claim that names a type
//!   removes a room of exactly that type, so the last Deluxe is now refused
//!   while the Standard stays sellable;
//! * the **property-wide** term still CAPS, and its arithmetic is
//!   byte-for-byte what B8a shipped (every parked claim is counted, typed or
//!   not). A claim whose type is NULL — unknown legacy code, an untyped room,
//!   a pre-094 row — therefore behaves EXACTLY as it did under #304, which is
//!   the deliberate fallback, not an oversight.
//!
//! The two compose without double-counting because the type term is a
//! `LEAST`, not a second subtraction: a type's own claims can never be
//! deducted twice, and claims on OTHER types still bound it through
//! `surplus`. Capping (rather than subtracting) property-wide is also what
//! keeps false sold-outs out while the property has slack — the failure mode
//! `loyalty-app` ADR-0003 rejected allotments over.
//!
//! Cost: two extra aggregates per availability/pick call, over the free-room
//! CTE those calls already build plus a `ht_bookings` date-range scan served
//! by `ix_ht_bookings_daterange` (the typed grouping is served by the partial
//! `ix_ht_bookings_room_type`).

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

/// Inventory pressure for one stay window (B8a, extended by B8c) — the
/// numbers the counter and the picker are both derived from, exposed so
/// operators (and tests) can see WHY a type reads sold out without
/// re-deriving the SQL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InventorySnapshot {
    /// Physically free rooms property-wide for the window.
    pub free_rooms: i64,
    /// Live bookings overlapping the window that have no room assigned yet —
    /// typed and untyped together. This is the term `surplus` subtracts, and
    /// its meaning is unchanged from B8a.
    pub parked_claims: i64,
    /// The subset of `parked_claims` that names a room type (B8c / migration
    /// 094). These are subtracted from THEIR OWN type's free-room count as
    /// well as counted property-wide.
    pub parked_claims_typed: i64,
    /// The subset of `parked_claims` with `book_room_type_id IS NULL` — type
    /// unknown, so they can only be capped property-wide (the #304 rule).
    /// `parked_claims_typed + parked_claims_untyped == parked_claims`.
    pub parked_claims_untyped: i64,
    /// `max(free_rooms - parked_claims, 0)` — what the channel may still
    /// sell property-wide. `0` ⇒ every free room is already spoken for.
    pub surplus: i64,
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

/// The booking statuses that HOLD inventory ("live"). Expanded into both
/// [`FREE_ROOM_PREDICATE`] and [`PARKED_CLAIM_PREDICATE`] at compile time
/// (via `concat!`, which a `const` + `format!` could not do) so the two can
/// never disagree about what "live" means.
macro_rules! live_booking_statuses {
    () => {
        "'confirmed','pending'"
    };
}

/// Shared free-room predicate. `$1=check_in, $2=check_out` (both DATE) must
/// be bound by every query that interpolates this. Kept as a `const` so the
/// aggregation query and the picker can never drift apart.
const FREE_ROOM_PREDICATE: &str = concat!(
    r#"
      r.room_active = true
      AND COALESCE(r.room_maintenance, false) = false
      AND COALESCE(r.room_status, '') <> 'maintenance'
      AND NOT EXISTS (
            SELECT 1 FROM ht_booking_rooms br
              JOIN ht_bookings b ON b.book_id = br.br_book_id
             WHERE br.br_room_id = r.room_id
               AND b.book_status IN ("#,
    live_booking_statuses!(),
    r#")
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
"#
);

/// Shared parked-claim predicate (B8a) — a LIVE booking that has not been
/// given a room yet. Same `$1=check_in, $2=check_out` binding, the same
/// `live_booking_statuses!()` set and the same half-open overlap rule as
/// [`FREE_ROOM_PREDICATE`]; the only difference is that it counts BOOKINGS
/// (which `ht_booking_rooms` cannot reach) instead of rooms.
///
/// A cancelled/checked-out/no-show parked booking is excluded by the status
/// set, exactly as it is for a roomed one.
///
/// B8c deliberately does NOT add a type term here: this predicate defines
/// "which bookings are parked claims", and every parked claim counts toward
/// the property-wide cap whether or not its type is known. The typed split is
/// an extra aggregate over the SAME predicate (see [`inventory_ctes`]), so
/// the two can never disagree about the population.
const PARKED_CLAIM_PREDICATE: &str = concat!(
    r#"
      b.book_status IN ("#,
    live_booking_statuses!(),
    r#")
      AND b.book_checkin  < $2
      AND b.book_checkout > $1
      AND NOT EXISTS (
            SELECT 1 FROM ht_booking_rooms br WHERE br.br_book_id = b.book_id
      )
"#
);

/// The common table expressions every inventory read is built on — the
/// single definition of "what is sellable for `[check_in, check_out)`",
/// shared verbatim by the counter ([`availability_by_type`]), the picker
/// ([`pick_free_room`]) and the snapshot ([`inventory_snapshot`]) so the
/// three cannot drift:
///
/// * `free_rooms` — one row per physically free room (id, number, type);
/// * `parked_claims` — how many live roomless bookings overlap the window,
///   typed and untyped together (the property-wide pressure term);
/// * `parked_claims_typed` — the same population grouped by
///   `book_room_type_id`, NULL types excluded (B8c / migration 094);
/// * `inventory_surplus` — free rooms that are NOT already spoken for by a
///   parked booking, floored at 0 — arithmetically identical to what B8a
///   shipped, which is what preserves the #304 behaviour for a NULL-type
///   claim;
/// * `type_availability` — the ONE expression that decides what a type may
///   sell: `min( max(free(type) − parked_typed(type), 0), surplus )`, one row
///   per `ht_room_types` row (including inactive ones; the counter filters).
///   Both the counter and the picker read it, so a type the counter shows as
///   `0` is a type the picker refuses, by construction rather than by two
///   parallel edits.
///
/// A function rather than a `const` only because it interpolates two other
/// consts. Callers prefix it with `WITH `. Binds `$1=check_in, $2=check_out`.
fn inventory_ctes() -> String {
    format!(
        r#"
        free_rooms AS (
            SELECT r.room_id, r.room_no, r.room_type_id
              FROM ht_rooms_new r
             WHERE {FREE_ROOM_PREDICATE}
        ),
        parked_claims AS (
            SELECT COUNT(*)::int8 AS n
              FROM ht_bookings b
             WHERE {PARKED_CLAIM_PREDICATE}
        ),
        parked_claims_typed AS (
            SELECT b.book_room_type_id AS type_id, COUNT(*)::int8 AS n
              FROM ht_bookings b
             WHERE {PARKED_CLAIM_PREDICATE}
               AND b.book_room_type_id IS NOT NULL
             GROUP BY b.book_room_type_id
        ),
        free_rooms_by_type AS (
            SELECT room_type_id AS type_id, COUNT(*)::int8 AS n
              FROM free_rooms
             WHERE room_type_id IS NOT NULL
             GROUP BY room_type_id
        ),
        inventory_surplus AS (
            SELECT GREATEST(
                       (SELECT COUNT(*)::int8 FROM free_rooms)
                       - (SELECT n FROM parked_claims),
                       0::int8
                   ) AS n
        ),
        type_availability AS (
            SELECT rt.type_id,
                   LEAST(
                       GREATEST(
                           COALESCE(f.n, 0::int8) - COALESCE(p.n, 0::int8),
                           0::int8
                       ),
                       (SELECT n FROM inventory_surplus)
                   ) AS n
              FROM ht_room_types rt
              LEFT JOIN free_rooms_by_type   f ON f.type_id = rt.type_id
              LEFT JOIN parked_claims_typed  p ON p.type_id = rt.type_id
        )
        "#
    )
}

/// Per-type availability for `[check_in, check_out)`, optionally filtered to
/// types that sleep at least `guests`. Types with zero free rooms are
/// included (`available_count = 0`) so the loyalty app can show sold-out
/// states. Runtime `sqlx::query` — the SQL interpolates the shared
/// [`inventory_ctes`] (static, no user input).
///
/// B8a/B8c: the count comes straight from the shared `type_availability`
/// CTE — a parked (roomless) live booking removes a room from the channel's
/// view even though no specific room looks taken, from its OWN type when it
/// names one and property-wide otherwise. See the module docs for the two
/// terms and why the property-wide one still only caps.
pub async fn availability_by_type(
    pool: &PgPool,
    check_in: NaiveDate,
    check_out: NaiveDate,
    guests: i32,
) -> Result<Vec<RoomTypeAvailability>, sqlx::Error> {
    use sqlx::Row;

    let sql = format!(
        r#"
        WITH {ctes}
        SELECT rt.type_id,
               rt.type_name,
               rt.type_description,
               COALESCE(rt.type_base_price, 0)::float8 AS nightly_price,
               COALESCE(ta.n, 0::int8) AS available_count
          FROM ht_room_types rt
          LEFT JOIN type_availability ta ON ta.type_id = rt.type_id
         WHERE COALESCE(rt.type_active, true) = true
           AND ($3::int IS NULL OR rt.type_max_guests IS NULL OR rt.type_max_guests >= $3)
         ORDER BY rt.type_sort_order NULLS LAST, rt.type_id
        "#,
        ctes = inventory_ctes()
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
/// `None` ⇒ the type is sold out for the window, or cannot sleep `guests`
/// (the availability endpoint filters capacity, but a direct create must not
/// trust the caller to have gone through it).
///
/// NOTE: pick → create is not serialized against a concurrent pick of the
/// same room (same race window the walk-in / booking form has today; the
/// existing create path accepts it and the shadow validator observes it).
///
/// B8a/B8c: gated on the same `type_availability` row the counter reports, so
/// the picker REFUSES while this TYPE is spoken for by parked (roomless)
/// bookings — or while the property as a whole is oversubscribed — even
/// though this specific room looks free. Counter and picker read one shared
/// definition ([`inventory_ctes`]); they cannot drift.
pub async fn pick_free_room(
    pool: &PgPool,
    type_id: i32,
    check_in: NaiveDate,
    check_out: NaiveDate,
    guests: i32,
) -> Result<Option<PickedRoom>, sqlx::Error> {
    use sqlx::Row;

    let sql = format!(
        r#"
        WITH {ctes}
        SELECT f.room_id, f.room_no, rt.type_name
          FROM free_rooms f
          JOIN ht_room_types rt ON rt.type_id = f.room_type_id
         WHERE f.room_type_id = $3
           AND (rt.type_max_guests IS NULL OR rt.type_max_guests >= $4)
           AND COALESCE(
                 (SELECT ta.n FROM type_availability ta WHERE ta.type_id = $3),
                 0::int8
               ) > 0
         ORDER BY f.room_no
         LIMIT 1
        "#,
        ctes = inventory_ctes()
    );

    let row = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(check_in)
        .bind(check_out)
        .bind(type_id)
        .bind(guests)
        .fetch_optional(pool)
        .await?;

    Ok(row.map(|row| PickedRoom {
        room_id: row.try_get("room_id").unwrap_or(0),
        room_no: row.try_get("room_no").unwrap_or_default(),
        type_name: row.try_get("type_name").unwrap_or_default(),
    }))
}

/// Inventory pressure for `[check_in, check_out)` (B8a/B8c) — free rooms,
/// parked claims (split into typed and untyped) and the resulting
/// property-wide surplus, read through the SAME [`inventory_ctes`] the
/// counter and the picker use. Diagnostic / test-facing: it answers "is this
/// type sold out because its own rooms are taken, because a parked booking
/// claimed a room OF THAT TYPE, or because an untyped parked booking claimed
/// the property's last room?".
pub async fn inventory_snapshot(
    pool: &PgPool,
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> Result<InventorySnapshot, sqlx::Error> {
    use sqlx::Row;

    let sql = format!(
        r#"
        WITH {ctes}
        SELECT (SELECT COUNT(*)::int8 FROM free_rooms) AS free_rooms,
               (SELECT n FROM parked_claims)           AS parked_claims,
               COALESCE((SELECT SUM(n) FROM parked_claims_typed), 0::int8)
                                                       AS parked_claims_typed,
               (SELECT n FROM parked_claims)
                 - COALESCE((SELECT SUM(n) FROM parked_claims_typed), 0::int8)
                                                       AS parked_claims_untyped,
               (SELECT n FROM inventory_surplus)       AS surplus
        "#,
        ctes = inventory_ctes()
    );

    let row = sqlx::query(sqlx::AssertSqlSafe(&*sql))
        .bind(check_in)
        .bind(check_out)
        .fetch_one(pool)
        .await?;

    Ok(InventorySnapshot {
        free_rooms: row.try_get("free_rooms").unwrap_or(0),
        parked_claims: row.try_get("parked_claims").unwrap_or(0),
        parked_claims_typed: row.try_get("parked_claims_typed").unwrap_or(0),
        parked_claims_untyped: row.try_get("parked_claims_untyped").unwrap_or(0),
        surplus: row.try_get("surplus").unwrap_or(0),
    })
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

/// The channel projection of the booking a `(book_channel, book_ext_ref)`
/// natural key names — migration 076's partial UNIQUE index guarantees at
/// most one (B8d / issue #305).
///
/// This is the READ half of making a hold-create replay-safe ACROSS a crash.
/// `ht_channel_idempotency` (migration 093) records the key in a transaction
/// SEPARATE from the booking's, so a crash between the two commits leaves the
/// hold committed and the key gone — and the retry, entering as a fresh
/// reservation, used to mint a SECOND hold. Stamping the key onto the booking
/// itself as `book_ext_ref` moves the dedupe INSIDE the booking transaction;
/// this lookup is how the service recognises the survivor and replays it
/// instead of creating another.
///
/// Returns the same shape as [`get_channel_booking`] so the replay payload is
/// rendered from the STORED hold (its own total, its own 2 h deadline), never
/// re-quoted from the retry's request.
pub async fn channel_booking_by_ext_ref(
    pool: &PgPool,
    channel: &str,
    ext_ref: &str,
) -> Result<Option<ChannelBookingRow>, sqlx::Error> {
    // Runtime query (not `query!`): `book_ext_ref` postdates the committed
    // `.sqlx` offline snapshot — same rationale as
    // `repository::booking::find_by_channel_ext_ref`.
    let row: Option<(
        i32,
        String,
        Option<String>,
        Option<String>,
        i32,
        NaiveDate,
        NaiveDate,
        Option<f64>,
        Option<f64>,
        Option<DateTime<Utc>>,
    )> = sqlx::query_as(
        "SELECT book_id, book_no, book_status, book_channel, book_cust_id, \
                book_checkin, book_checkout, \
                book_total_amount::float8, book_deposit_amount::float8, \
                book_hold_expires_at \
           FROM ht_bookings \
          WHERE book_channel = $1 AND book_ext_ref = $2 \
          ORDER BY book_id LIMIT 1",
    )
    .bind(channel)
    .bind(ext_ref)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(
        |(
            book_id,
            book_no,
            status,
            channel,
            customer_id,
            check_in,
            check_out,
            total_amount,
            deposit_amount,
            hold_expires_at,
        )| ChannelBookingRow {
            book_id,
            book_no,
            status: status.unwrap_or_else(|| "pending".to_string()),
            channel,
            customer_id,
            check_in,
            check_out,
            total_amount: total_amount.unwrap_or(0.0),
            deposit_amount: deposit_amount.unwrap_or(0.0),
            hold_expires_at,
        },
    ))
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
