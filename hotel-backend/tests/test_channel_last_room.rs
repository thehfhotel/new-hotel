//! B8e — serialized pick → create (L3) and the property-wide last-room floor
//! (L2), against the real PostgreSQL schema.
//!
//! ## What was broken
//!
//! `pick_free_room` ran outside any transaction and the INSERT that consumed
//! the picked room happened in a LATER one, with a guest match-or-create round
//! trip in between. Two holds for the property's last room — or a hold and a
//! desk booking typed in the same minute — could both pick it and both commit.
//! Nothing in the schema rejects the second write (`uq_ht_br_bookroom` only
//! stops ONE booking listing a room twice), so the double-sell surfaced as a
//! guest turned away at reception.
//!
//! ## What these tests pin
//!
//! * **L3, hold vs hold** — two concurrent holds for the same last room:
//!   exactly one is created, the other is refused, and the room carries
//!   exactly ONE live assignment afterwards.
//! * **L3, hold vs desk** — a desk create that commits inside the hold's
//!   pick→insert window wins the room, and the hold re-evaluates and refuses
//!   instead of taking it too. The ordering is forced by the test itself
//!   holding the same public lock, so this is deterministic rather than a
//!   timing lottery.
//! * **L2, the floor** — refuses at `floor = 1`, allows the identical request
//!   at `floor = 0`, and does not fire while the property has slack (it is a
//!   floor, not an allotment cap).
//! * **L2, the desk is not gated** — the desk books the very room the channel
//!   just declined.
//!
//! ## Running
//!
//! `common` reads `DATABASE_URL` (CI provides a service container and runs
//! `--test-threads=1`). Every fixture row carries a `TEST_lastroom`-scoped
//! marker unique to THIS file and is deleted by `cleanup` (exact-match, per
//! the `common` rules).
//!
//! ## Why the surplus is MEASURED, not assumed
//!
//! "Free rooms" spans every room in the database, and a developer machine
//! carries the real mirrored room list while CI carries only what tests seed.
//! Each floor scenario therefore reads `inventory_snapshot` and DRAINS the
//! surplus to a known value with parked (roomless) bookings, exactly as
//! `test_channel_parked_inventory.rs` does, instead of assuming a room count.
//!
//! ## Why one test function
//!
//! Same reason as `test_channel.rs` and `test_channel_parked_inventory.rs`:
//! the fixtures are shared, separate `#[tokio::test]` fns would race on them
//! under a default (parallel) local `cargo test`, and a single body guarantees
//! cleanup ordering. Each scenario owns a DISJOINT far-future stay window, so
//! the bookings one scenario seeds are invisible to every other.

mod common;

use std::sync::Arc;
use std::time::Duration;

use chrono::{NaiveDate, TimeZone, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use hotel_backend::domain::shared::{DateRange, Money};
use hotel_backend::outbox::event::EventSource;
use hotel_backend::outbox::{EventBus, OutboxRepository};
use hotel_backend::repository::channel as channel_repo;
use hotel_backend::repository::inventory_lock::{lock_key, InventoryLock};
use hotel_backend::repository::{CustomerRepository, PgBookingRepository, PgCustomerRepository};
use hotel_backend::service::{
    aggregate_uuid, AggregateKind, BookingRoomCommand, BookingService, BookingWritebackContext,
    ChannelService, CreateBookingCommand, CreateHoldCommand, CustomerService, HoldCreateOutcome,
    PaymentPlan,
};

/// Unique-to-this-file fixture markers (see tests/common/mod.rs cleanup rules).
const TYPE_CODE: &str = "TSTLRM";
const TYPE_NAME: &str = "TEST_lastroom_type";
const ROOM_A: &str = "TLR01";
const ROOM_B: &str = "TLR02";
const GUEST_FIRST: &str = "TEST_lastroom_guest";
const BOOK_NO_PREFIX: &str = "TESTLRM";

/// The property every scenario locks on — the same literal
/// `routes::channel::parse_property` yields for `"hf"`.
const PROPERTY: &str = "hf";

/// Every seeded room sleeps 2, so every request asks for 2 guests.
const GUESTS: i32 = 2;

fn d(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").expect("test date")
}

fn utc(day: NaiveDate) -> chrono::DateTime<Utc> {
    Utc.from_utc_datetime(&day.and_hms_opt(0, 0, 0).unwrap())
}

fn source() -> EventSource {
    EventSource::our_app(Uuid::nil(), Uuid::new_v4())
}

// ── services under test ─────────────────────────────────────────────────────

fn booking_service(pool: &PgPool) -> Arc<BookingService> {
    Arc::new(BookingService::new(
        Arc::new(PgBookingRepository::new()),
        Arc::new(OutboxRepository::new()),
        Arc::new(EventBus::new()),
        pool.clone(),
    ))
}

/// A `ChannelService` with the B8e/L2 floor pinned to `last_room_floor`.
///
/// Pinned per service rather than read from the process environment: these
/// scenarios assert both sides of the guard in one test body, and an env var
/// cannot be two values at once (nor mutated safely from a multi-threaded
/// runtime).
fn channel_service(pool: &PgPool, last_room_floor: i64) -> ChannelService {
    let outbox = Arc::new(OutboxRepository::new());
    let events = Arc::new(EventBus::new());
    let customers_repo: Arc<dyn CustomerRepository> = Arc::new(PgCustomerRepository::new());
    let customers = Arc::new(CustomerService::new(
        customers_repo.clone(),
        outbox,
        events,
        pool.clone(),
    ));
    ChannelService::new(
        pool.clone(),
        booking_service(pool),
        customers,
        customers_repo,
        last_room_floor,
    )
}

fn hold_cmd(
    book_no: &str,
    type_id: i32,
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> CreateHoldCommand {
    CreateHoldCommand {
        book_no: book_no.to_string(),
        property: PROPERTY.to_string(),
        room_type_id: type_id,
        check_in,
        check_out,
        guests: GUESTS,
        guest_name: format!("{GUEST_FIRST} Somchai"),
        guest_phone: "0800000999".to_string(),
        membership_id: None,
        payment: PaymentPlan::Deposit50,
        // Unkeyed on purpose in the race scenarios: with an `ext_ref` the
        // (book_channel, book_ext_ref) index would dedupe the two writers and
        // the lock would never be the thing under test.
        ext_ref: None,
        ext_ref_fingerprint: None,
        source: source(),
    }
}

/// The DESK create, as `routes::new_bookings::create_booking` builds it —
/// a specific room, and `inventory_lock: Some(property)` so the service takes
/// the same lock the channel does.
fn desk_cmd(
    book_no: &str,
    cust_id: i32,
    room_id: i32,
    room_no: &str,
    check_in: NaiveDate,
    check_out: NaiveDate,
    inventory_lock: Option<String>,
) -> CreateBookingCommand {
    CreateBookingCommand {
        book_no: book_no.to_string(),
        customer_id: cust_id,
        check_in,
        check_out,
        adults: GUESTS,
        children: 0,
        status: "confirmed".to_string(),
        source_label: Some("walkin".to_string()),
        total_amount: Some(2000.0),
        deposit_amount: None,
        notes: None,
        rooms: vec![BookingRoomCommand {
            room_id,
            price_per_night: Some(1000.0),
        }],
        room_type_id: None,
        products: vec![],
        writeback_context: BookingWritebackContext {
            customer_aggregate_id: aggregate_uuid(AggregateKind::Customer, cust_id),
            legacy_cust_no: None,
            customer_name: "TEST lastroom desk".to_string(),
            customer_phone: None,
            stay: DateRange::new(utc(check_in), utc(check_out)),
            room_no: room_no.to_string(),
            room_type: TYPE_NAME.to_string(),
            price: Money::from_baht(1000),
            deposit: Money::ZERO,
            created_by: String::new(),
            notes: None,
        },
        book_channel: None,
        book_ext_ref: None,
        book_ext_ref_fingerprint: None,
        inventory_lock,
        hold_expires_at: None,
        source: source(),
    }
}

// ── fixtures ────────────────────────────────────────────────────────────────

async fn seed_type(pool: &PgPool) -> i32 {
    sqlx::query(
        "INSERT INTO ht_room_types (type_code, type_name, type_description, type_base_price, type_max_guests, type_active) \
         VALUES ($1, $2, 'last-room test type', 1000.00, 2, true) RETURNING type_id",
    )
    .bind(TYPE_CODE)
    .bind(TYPE_NAME)
    .fetch_one(pool)
    .await
    .expect("seed room type")
    .get("type_id")
}

async fn seed_room(pool: &PgPool, room_no: &str, type_id: i32) -> i32 {
    sqlx::query(
        "INSERT INTO ht_rooms_new (room_no, room_type_id, room_status, room_active, room_maintenance) \
         VALUES ($1, $2, 'available', true, false) RETURNING room_id",
    )
    .bind(room_no)
    .bind(type_id)
    .fetch_one(pool)
    .await
    .expect("seed room")
    .get("room_id")
}

async fn seed_customer(pool: &PgPool) -> i32 {
    sqlx::query("INSERT INTO ht_customers (cust_firstname) VALUES ($1) RETURNING cust_id")
        .bind(GUEST_FIRST)
        .fetch_one(pool)
        .await
        .expect("seed customer")
        .get("cust_id")
}

/// Occupy `room_id` for the window with a live assigned booking — the shape
/// `FREE_ROOM_PREDICATE` already sees, used to reduce a type to its LAST room.
async fn occupy(
    pool: &PgPool,
    cust_id: i32,
    room_id: i32,
    check_in: NaiveDate,
    check_out: NaiveDate,
    seq: &mut i64,
) {
    let book_no = format!("{BOOK_NO_PREFIX}{:06}", *seq);
    *seq += 1;
    let book_id: i32 = sqlx::query(
        "INSERT INTO ht_bookings (book_no, book_cust_id, book_checkin, book_checkout, book_status) \
         VALUES ($1, $2, $3, $4, 'confirmed') RETURNING book_id",
    )
    .bind(&book_no)
    .bind(cust_id)
    .bind(check_in)
    .bind(check_out)
    .fetch_one(pool)
    .await
    .expect("seed occupying booking")
    .get("book_id");

    sqlx::query("INSERT INTO ht_booking_rooms (br_book_id, br_room_id) VALUES ($1, $2)")
        .bind(book_id)
        .bind(room_id)
        .execute(pool)
        .await
        .expect("assign occupying room");
}

/// Insert `count` PARKED bookings (live `ht_bookings` rows with no
/// `ht_booking_rooms` child) — the cheapest way to push the property-wide
/// surplus down to a known number without inventing rooms.
async fn seed_parked(
    pool: &PgPool,
    cust_id: i32,
    check_in: NaiveDate,
    check_out: NaiveDate,
    seq: &mut i64,
    count: i64,
) {
    if count <= 0 {
        return;
    }
    let from = *seq;
    let to = *seq + count - 1;
    *seq = to + 1;

    sqlx::query(
        "INSERT INTO ht_bookings (book_no, book_cust_id, book_checkin, book_checkout, book_status, book_channel) \
         SELECT $1 || lpad(g::text, 6, '0'), $2, $3::date, $4::date, 'confirmed', 'bookingcom' \
           FROM generate_series($5::bigint, $6::bigint) AS g",
    )
    .bind(BOOK_NO_PREFIX)
    .bind(cust_id)
    .bind(check_in)
    .bind(check_out)
    .bind(from)
    .bind(to)
    .execute(pool)
    .await
    .expect("seed parked bookings");
}

/// Drain the property-wide surplus for the window down to exactly `target`.
async fn drain_surplus_to(
    pool: &PgPool,
    cust_id: i32,
    check_in: NaiveDate,
    check_out: NaiveDate,
    seq: &mut i64,
    target: i64,
) {
    let before = surplus(pool, check_in, check_out).await;
    assert!(
        before >= target,
        "fixture cannot ADD inventory: surplus {before} is already below the target {target}"
    );
    seed_parked(pool, cust_id, check_in, check_out, seq, before - target).await;
    let after = surplus(pool, check_in, check_out).await;
    assert_eq!(after, target, "drain must land the surplus exactly");
}

async fn surplus(pool: &PgPool, check_in: NaiveDate, check_out: NaiveDate) -> i64 {
    channel_repo::inventory_snapshot(pool, check_in, check_out)
        .await
        .expect("inventory snapshot")
        .surplus
}

/// How many LIVE bookings hold `room_id` over the window. The double-sell this
/// whole feature exists to prevent reads as `2` here.
async fn live_claims_on(
    pool: &PgPool,
    room_id: i32,
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> i64 {
    sqlx::query(
        "SELECT COUNT(*)::int8 AS n \
           FROM ht_booking_rooms br \
           JOIN ht_bookings b ON b.book_id = br.br_book_id \
          WHERE br.br_room_id = $1 \
            AND b.book_status IN ('confirmed','pending') \
            AND b.book_checkin  < $3::date \
            AND b.book_checkout > $2::date",
    )
    .bind(room_id)
    .bind(check_in)
    .bind(check_out)
    .fetch_one(pool)
    .await
    .expect("count live claims")
    .get("n")
}

/// Delete every row this file created, children first. Exact-match markers.
async fn cleanup(pool: &PgPool) {
    sqlx::query(
        "DELETE FROM writeback_jobs WHERE aggregate_id IN \
         (SELECT aggregate_id FROM ht_bookings WHERE book_no LIKE $1 AND aggregate_id IS NOT NULL)",
    )
    .bind(format!("{BOOK_NO_PREFIX}%"))
    .execute(pool)
    .await
    .ok();
    sqlx::query(
        "DELETE FROM event_log WHERE aggregate_id IN \
         (SELECT aggregate_id FROM ht_bookings WHERE book_no LIKE $1 AND aggregate_id IS NOT NULL)",
    )
    .bind(format!("{BOOK_NO_PREFIX}%"))
    .execute(pool)
    .await
    .ok();
    // Customer events: `CustomerService::create` publishes under the
    // deterministic aggregate uuid without necessarily stamping the row, so
    // recompute it from the SERIAL id rather than trusting a column.
    if let Ok(rows) = sqlx::query("SELECT cust_id FROM ht_customers WHERE cust_firstname = $1")
        .bind(GUEST_FIRST)
        .fetch_all(pool)
        .await
    {
        for row in rows {
            let cust_id: i32 = row.get("cust_id");
            sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
                .bind(aggregate_uuid(AggregateKind::Customer, cust_id))
                .execute(pool)
                .await
                .ok();
        }
    }
    // Bookings (ht_booking_rooms rows cascade on br_book_id).
    sqlx::query("DELETE FROM ht_bookings WHERE book_no LIKE $1")
        .bind(format!("{BOOK_NO_PREFIX}%"))
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_customers WHERE cust_firstname LIKE $1")
        .bind(format!("{GUEST_FIRST}%"))
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_no IN ($1, $2)")
        .bind(ROOM_A)
        .bind(ROOM_B)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_room_types WHERE type_code = $1")
        .bind(TYPE_CODE)
        .execute(pool)
        .await
        .ok();
}

// ── the lock key (no database needed) ───────────────────────────────────────

/// The two properties must not share a lock, and a property's key must not
/// move between releases.
///
/// The literals are PINNED, not recomputed from `lock_key`, and that is the
/// whole point: a self-referential assertion would pass through any change to
/// the hash. A key that drifts is a lock that silently stops excluding — two
/// backend versions mid-deploy would take different keys for one property and
/// the double-sell would come back for the length of the rollout, with every
/// test still green.
#[test]
fn lock_keys_are_stable_and_per_property() {
    // classid 1112230230 == i32::from_be_bytes(*b"BKIV"); objid == FNV-1a/32
    // of the property label, as i32.
    assert_eq!(lock_key("hf"), (1_112_230_230, 1_530_585_635));
    assert_eq!(lock_key("hfville"), (1_112_230_230, -2_099_596_955));
    assert_ne!(
        lock_key("hf"),
        lock_key("hfville"),
        "each property needs its own lock"
    );
}

// ── the tests ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn last_room_is_serialized_and_floored() {
    let pool = common::create_test_pool().await;
    // Pre-clean in case a previous run aborted mid-test.
    cleanup(&pool).await;

    let type_id = seed_type(&pool).await;
    let room_a = seed_room(&pool, ROOM_A, type_id).await;
    let room_b = seed_room(&pool, ROOM_B, type_id).await;
    let cust = seed_customer(&pool).await;

    // Monotonic book_no suffix — book_no is UNIQUE across the whole table.
    let mut seq: i64 = 1;

    // ---------------------------------------------------------------------
    // 1. L3 — two concurrent holds for the LAST room of a type.
    //
    // Floor 0 so the only thing that can refuse the loser is the serialized
    // pick itself.
    // ---------------------------------------------------------------------
    let (w1_in, w1_out) = (d("2027-03-01"), d("2027-03-03"));
    occupy(&pool, cust, room_b, w1_in, w1_out, &mut seq).await;

    let svc = channel_service(&pool, 0);
    let a = hold_cmd(&format!("{BOOK_NO_PREFIX}-0101"), type_id, w1_in, w1_out);
    let b = hold_cmd(&format!("{BOOK_NO_PREFIX}-0102"), type_id, w1_in, w1_out);
    let (sa, sb) = (svc.clone(), svc.clone());
    let ta = tokio::spawn(async move { sa.create_hold(a).await });
    let tb = tokio::spawn(async move { sb.create_hold(b).await });
    let ra = ta.await.expect("task a");
    let rb = tb.await.expect("task b");

    // The loser must be SOLD OUT specifically — it re-picked after the winner
    // committed and found nothing. Asserting a looser "it failed somehow"
    // would also accept a lock TIMEOUT (503) or any repository error, which
    // would mean the serialization never actually ran and the test was
    // certifying a failure mode instead of the fix.
    let created = [&ra, &rb]
        .iter()
        .filter(|r| matches!(r, Ok(HoldCreateOutcome::Created(_))))
        .count();
    let refused = [&ra, &rb]
        .iter()
        .filter(|r| matches!(r, Ok(HoldCreateOutcome::SoldOut { .. })))
        .count();
    assert_eq!(
        (created, refused),
        (1, 1),
        "exactly one of two concurrent holds may take the last room, and the loser must be \
         SoldOut (not a lock timeout); got {ra:?} / {rb:?}"
    );
    assert_eq!(
        live_claims_on(&pool, room_a, w1_in, w1_out).await,
        1,
        "the last room must carry exactly ONE live claim after the race"
    );

    // ---------------------------------------------------------------------
    // 2. L3 — a hold racing a DESK booking for the last room.
    //
    // Ordering is forced rather than left to the scheduler: the test takes the
    // property's lock first, so the spawned hold parks on it; the desk create
    // commits inside that window; then the lock is released and the hold has
    // to re-evaluate. Without the lock in `create_hold` the spawned hold does
    // not wait at all, picks the room it saw free, and the room ends the
    // scenario with TWO live claims — which is what this asserts against.
    // ---------------------------------------------------------------------
    let (w2_in, w2_out) = (d("2027-03-11"), d("2027-03-13"));
    occupy(&pool, cust, room_b, w2_in, w2_out, &mut seq).await;

    let gate = InventoryLock::acquire(&pool, PROPERTY)
        .await
        .expect("the test takes the property lock first");

    let svc2 = channel_service(&pool, 0);
    let held = hold_cmd(&format!("{BOOK_NO_PREFIX}-0201"), type_id, w2_in, w2_out);
    let hold_task = tokio::spawn(async move { svc2.create_hold(held).await });
    // Long enough for the hold to reach the lock and start waiting on it.
    tokio::time::sleep(Duration::from_millis(200)).await;

    // The desk writes while the channel is parked on the lock. It passes
    // `inventory_lock: None` because the gate above already stands in for the
    // lock its route would take — re-acquiring here would deadlock the test
    // against its own guard.
    booking_service(&pool)
        .create(desk_cmd(
            &format!("{BOOK_NO_PREFIX}-0202"),
            cust,
            room_a,
            ROOM_A,
            w2_in,
            w2_out,
            None,
        ))
        .await
        .expect("the desk always wins its own write");

    gate.release().await.expect("release the gate");

    let hold_result = hold_task.await.expect("hold task");
    // Again SoldOut specifically: the hold waited out the gate, re-picked, and
    // found the desk's room gone. A 503 here would mean it gave up on the lock
    // without ever re-evaluating — a different (and much worse) outcome that
    // an `is_err()` assertion would have happily accepted.
    assert!(
        matches!(hold_result, Ok(HoldCreateOutcome::SoldOut { .. })),
        "the hold must re-evaluate after the desk commits and report SoldOut; got {hold_result:?}"
    );
    assert_eq!(
        live_claims_on(&pool, room_a, w2_in, w2_out).await,
        1,
        "the desk booking must be the only live claim on the last room"
    );

    // ---------------------------------------------------------------------
    // 2b. L3 — the DESK create takes the lock too.
    //
    // Scenario 2 proved the channel WAITS. This one proves the other half of
    // the mutual exclusion, which nothing else covers: that
    // `routes::new_bookings`' shape of the command (`inventory_lock:
    // Some(property)`) actually blocks. Without it a reviewer could delete the
    // field from the desk route and every remaining test would still pass.
    //
    // Shape: hold the gate, start the desk create, prove it has NOT finished
    // while the gate is held, release, prove it then completes.
    // ---------------------------------------------------------------------
    let (w2b_in, w2b_out) = (d("2027-03-16"), d("2027-03-18"));

    let gate = InventoryLock::acquire(&pool, PROPERTY)
        .await
        .expect("take the property lock");
    assert!(
        !gate.is_bypassed(),
        "BOOKING_INVENTORY_LOCK_ENABLED must be on for this suite to mean anything"
    );

    let svc_desk = booking_service(&pool);
    let desk_cmd_locked = desk_cmd(
        &format!("{BOOK_NO_PREFIX}-0211"),
        cust,
        room_a,
        ROOM_A,
        w2b_in,
        w2b_out,
        Some(PROPERTY.to_string()),
    );
    let desk_task = tokio::spawn(async move { svc_desk.create(desk_cmd_locked).await });

    // Comfortably longer than an uncontended desk create (a handful of
    // statements, low single-digit ms) and far inside the lock's 5 s acquire
    // deadline, so a finished task here can only mean it never waited.
    tokio::time::sleep(Duration::from_millis(400)).await;
    assert!(
        !desk_task.is_finished(),
        "the desk create must BLOCK on the property's inventory lock while it is held"
    );

    gate.release().await.expect("release the gate");

    let desk_done = desk_task
        .await
        .expect("desk task")
        .expect("the desk create must proceed once the lock is free");
    assert!(desk_done.book_id > 0);
    assert_eq!(
        live_claims_on(&pool, room_a, w2b_in, w2b_out).await,
        1,
        "the desk booking landed exactly once"
    );

    // ---------------------------------------------------------------------
    // 3. L2 — the floor refuses at N=1 and allows the SAME request at N=0.
    // ---------------------------------------------------------------------
    let (w3_in, w3_out) = (d("2027-03-21"), d("2027-03-23"));
    drain_surplus_to(&pool, cust, w3_in, w3_out, &mut seq, 1).await;

    let refusal = channel_service(&pool, 1)
        .create_hold(hold_cmd(
            &format!("{BOOK_NO_PREFIX}-0301"),
            type_id,
            w3_in,
            w3_out,
        ))
        .await
        .expect("the floor is a refusal outcome, not an error");
    match refusal {
        HoldCreateOutcome::LastRoomHeldForDesk { free_rooms, floor } => {
            assert_eq!(
                (free_rooms, floor),
                (1, 1),
                "the 409 body must name both numbers"
            );
        }
        other => panic!("expected the last-room refusal at floor 1, got {other:?}"),
    }

    // Same fixture, same request, floor 0 — the guard is the ONLY difference.
    let allowed = channel_service(&pool, 0)
        .create_hold(hold_cmd(
            &format!("{BOOK_NO_PREFIX}-0302"),
            type_id,
            w3_in,
            w3_out,
        ))
        .await
        .expect("floor 0 disables the guard");
    assert!(
        matches!(allowed, HoldCreateOutcome::Created(_)),
        "with the floor off the last room is sellable through the channel; got {allowed:?}"
    );

    // ---------------------------------------------------------------------
    // 4. L2 — a FLOOR, not a cap: with slack the channel sells as before, and
    //    the guard engages only on the tail. Drain to 2 with floor 1, create
    //    one hold (surplus -> 1), then watch the next hold get refused.
    // ---------------------------------------------------------------------
    let (w4_in, w4_out) = (d("2027-03-31"), d("2027-04-02"));
    drain_surplus_to(&pool, cust, w4_in, w4_out, &mut seq, 2).await;

    let svc4 = channel_service(&pool, 1);
    let first = svc4
        .create_hold(hold_cmd(
            &format!("{BOOK_NO_PREFIX}-0401"),
            type_id,
            w4_in,
            w4_out,
        ))
        .await
        .expect("a property with slack is not gated");
    assert!(
        matches!(first, HoldCreateOutcome::Created(_)),
        "surplus 2 > floor 1 must sell; got {first:?}"
    );
    assert_eq!(
        surplus(&pool, w4_in, w4_out).await,
        1,
        "the hold consumed one sellable room"
    );

    let second = svc4
        .create_hold(hold_cmd(
            &format!("{BOOK_NO_PREFIX}-0402"),
            type_id,
            w4_in,
            w4_out,
        ))
        .await
        .expect("the floor is a refusal outcome, not an error");
    assert!(
        matches!(
            second,
            HoldCreateOutcome::LastRoomHeldForDesk {
                free_rooms: 1,
                floor: 1
            }
        ),
        "the guard must engage as soon as the tail is reached; got {second:?}"
    );

    // ---------------------------------------------------------------------
    // 5. L2 — reception is NOT gated. The desk books the very room the channel
    //    just declined, through the real desk path (lock acquired by the
    //    service, exactly as `routes::new_bookings` asks for).
    // ---------------------------------------------------------------------
    let (w5_in, w5_out) = (d("2027-04-10"), d("2027-04-12"));
    occupy(&pool, cust, room_b, w5_in, w5_out, &mut seq).await;
    drain_surplus_to(&pool, cust, w5_in, w5_out, &mut seq, 1).await;

    let declined = channel_service(&pool, 1)
        .create_hold(hold_cmd(
            &format!("{BOOK_NO_PREFIX}-0501"),
            type_id,
            w5_in,
            w5_out,
        ))
        .await
        .expect("the floor is a refusal outcome, not an error");
    assert!(
        matches!(declined, HoldCreateOutcome::LastRoomHeldForDesk { .. }),
        "the channel stands down at the floor; got {declined:?}"
    );

    let desk = booking_service(&pool)
        .create(desk_cmd(
            &format!("{BOOK_NO_PREFIX}-0502"),
            cust,
            room_a,
            ROOM_A,
            w5_in,
            w5_out,
            Some(PROPERTY.to_string()),
        ))
        .await
        .expect("the desk books the room the channel refused");
    assert!(desk.book_id > 0);
    assert_eq!(
        live_claims_on(&pool, room_a, w5_in, w5_out).await,
        1,
        "the desk booking is the room's only live claim"
    );

    cleanup(&pool).await;
}
