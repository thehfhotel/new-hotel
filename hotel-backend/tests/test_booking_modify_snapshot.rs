//! B8h — `BookingService::modify` decides the inventory lock AND the legacy
//! write-back leg from ONE snapshot, taken behind the booking row lock.
//!
//! ## What was broken (#325 review finding F6)
//!
//! B8g read the booking's prior room set twice, from two different places:
//!
//! * on the POOL, before the transaction, to decide whether to take the
//!   per-property inventory lock (the lock has to be held before the write
//!   starts, so the read had to come first), and
//! * inside the transaction — but before anything had locked the booking row —
//!   to decide whether the edit PROMOTES a parked booking to a byte-parity
//!   legacy `CreateBooking` or takes the ordinary `ModifyBooking` leg.
//!
//! A concurrent edit of the SAME booking could land between the two. The
//! disagreeing case includes the headline one: a rival edit that clears the
//! rooms makes the first read see "room set unchanged" (skip the lock) and the
//! second read see 0 prior rooms (promote) — so the room-consuming
//! `CreateBooking`, the exact write the inventory lock exists to serialise, was
//! emitted with no lock held.
//!
//! ## What these tests pin
//!
//! * **`the_loser_of_a_concurrent_edit_promotes_behind_the_inventory_lock`** —
//!   the interleaving above, forced deterministically. A rival transaction
//!   holds the booking row and clears its rooms; the edit must not decide
//!   anything until that rival commits, must then SEE the cleared room set, and
//!   must therefore block on the property lock instead of promoting through it.
//!   The outbox proves which leg it took.
//! * **`two_concurrent_room_assigns_promote_exactly_once`** — two real
//!   concurrent `modify` calls on one parked booking. Exactly one
//!   `create_booking` may be enqueued; the loser must see the winner's
//!   committed room and take `modify_booking`. (On the two-snapshot code both
//!   could read 0 prior rooms and both take the Create leg, colliding on the
//!   deterministic idempotency key and losing the second desk save.)
//!
//! Ordering is forced with the public `InventoryLock` and with a real row lock
//! held by the test, never with sleeps standing in for happens-before — the
//! only sleeps here are the "still blocked?" observations, which fail safe.
//!
//! ## Running
//!
//! `common` reads `DATABASE_URL` (CI provides a service container and runs
//! `--test-threads=1`). Every fixture row carries a `TEST_b8hsnap`-scoped
//! marker unique to THIS file, per-test, and is deleted by `cleanup`
//! (exact-match, per the `common` rules).

mod common;

use std::sync::Arc;
use std::time::Duration;

use chrono::{NaiveDate, TimeZone, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use hotel_backend::domain::booking::BookingState;
use hotel_backend::domain::shared::{DateRange, Money};
use hotel_backend::outbox::event::EventSource;
use hotel_backend::outbox::intent::BookingChanges;
use hotel_backend::outbox::{EventBus, OutboxRepository};
use hotel_backend::repository::inventory_lock::InventoryLock;
use hotel_backend::repository::PgBookingRepository;
use hotel_backend::service::{
    aggregate_uuid, AggregateKind, BookingRoomCommand, BookingService, BookingSnapshotInputs,
    BookingWritebackContext, ModifyBookingCommand, RoomTypeEdit,
};

/// The property every scenario locks on — the same literal
/// `routes::new_bookings::branch_property` yields for `"hf"`.
const PROPERTY: &str = "hf";

/// How long a BLOCKED-side assertion waits before concluding the task really is
/// parked. Comfortably longer than an uncontended modify (a handful of
/// statements, low single-digit ms), far inside the inventory lock's 5 s
/// acquire deadline AND inside the booking-row guard's 3 s `lock_timeout`.
/// These assertions fail SAFE: a slow runner makes it MORE likely the task is
/// still unfinished, not less.
const SETTLE: Duration = Duration::from_millis(400);

/// Deadline for an assertion that a task DID finish, which does not fail safe.
/// Polled rather than slept so a loaded runner that simply had not scheduled
/// the task yet does not flake.
const FINISH_DEADLINE: Duration = Duration::from_secs(5);

async fn finished_within(
    task: &tokio::task::JoinHandle<impl Send + 'static>,
    deadline: Duration,
) -> bool {
    let started = std::time::Instant::now();
    while started.elapsed() < deadline {
        if task.is_finished() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    task.is_finished()
}

fn d(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").expect("test date")
}

fn utc(day: NaiveDate) -> chrono::DateTime<Utc> {
    Utc.from_utc_datetime(&day.and_hms_opt(0, 0, 0).unwrap())
}

fn booking_service(pool: &PgPool) -> Arc<BookingService> {
    Arc::new(BookingService::new(
        Arc::new(PgBookingRepository::new()),
        Arc::new(OutboxRepository::new()),
        Arc::new(EventBus::new()),
        pool.clone(),
    ))
}

/// Per-test fixture markers, so the two tests never share a row even if the
/// `--test-threads=1` guarantee is ever relaxed.
struct Marks {
    type_code: String,
    type_name: String,
    room_no: String,
    room_no_alt: String,
    guest_first: String,
    book_no: String,
}

impl Marks {
    fn new(suffix: &str) -> Self {
        Self {
            type_code: format!("TSTB8H{suffix}"),
            type_name: format!("TEST_b8hsnap_type_{suffix}"),
            room_no: format!("TB8H{suffix}A"),
            room_no_alt: format!("TB8H{suffix}B"),
            guest_first: format!("TEST_b8hsnap_guest_{suffix}"),
            book_no: format!("TESTB8H{suffix}0001"),
        }
    }
}

fn wb_context(
    m: &Marks,
    cust_id: i32,
    room_no: &str,
    ci: NaiveDate,
    co: NaiveDate,
) -> BookingWritebackContext {
    BookingWritebackContext {
        customer_aggregate_id: aggregate_uuid(AggregateKind::Customer, cust_id),
        legacy_cust_no: None,
        customer_name: "TEST b8hsnap guest".to_string(),
        customer_phone: None,
        stay: DateRange::new(utc(ci), utc(co)),
        room_no: room_no.to_string(),
        room_type: m.type_name.clone(),
        price: Money::from_baht(1000),
        deposit: Money::ZERO,
        created_by: String::new(),
        notes: None,
    }
}

/// The DESK edit, exactly as `routes::new_bookings::update_booking` builds it —
/// `inventory_lock: Some(property)` on EVERY save, room-moving or not. The
/// service is what decides whether the lock is actually taken, which is the
/// decision under test.
fn desk_edit(
    m: &Marks,
    book_id: i32,
    cust_id: i32,
    rooms: Vec<BookingRoomCommand>,
    room_no: &str,
    ci: NaiveDate,
    co: NaiveDate,
) -> ModifyBookingCommand {
    let promote_context = (!rooms.is_empty()).then(|| wb_context(m, cust_id, room_no, ci, co));
    ModifyBookingCommand {
        book_id,
        customer_id: cust_id,
        check_in: ci,
        check_out: co,
        adults: 2,
        children: 0,
        status: "confirmed".to_string(),
        source_label: Some("walkin".to_string()),
        total_amount: Some(2000.0),
        deposit_amount: None,
        notes: None,
        rooms,
        room_type_id: RoomTypeEdit::Keep,
        changes: BookingChanges {
            new_stay: Some(DateRange::new(utc(ci), utc(co))),
            new_room_no: None,
            new_room_type: None,
            new_price: None,
            new_state: None,
            new_notes: None,
            new_customer_phone: None,
            new_customer_name: None,
            customer_resave: None,
        },
        promote_context,
        inventory_lock: Some(PROPERTY.to_string()),
        before_snapshot: None,
        after_snapshot: BookingSnapshotInputs {
            legacy_book_id: None,
            state: BookingState::Pending,
            stay_start: utc(ci),
            stay_end: utc(co),
            room_no: None,
            price: Money::from_baht(1000),
        },
        source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
    }
}

// ── fixtures ────────────────────────────────────────────────────────────────

async fn seed_type(pool: &PgPool, m: &Marks) -> i32 {
    sqlx::query(
        "INSERT INTO ht_room_types (type_code, type_name, type_description, type_base_price, type_max_guests, type_active) \
         VALUES ($1, $2, 'modify-snapshot test type', 1000.00, 2, true) RETURNING type_id",
    )
    .bind(&m.type_code)
    .bind(&m.type_name)
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

async fn seed_customer(pool: &PgPool, m: &Marks) -> i32 {
    sqlx::query("INSERT INTO ht_customers (cust_firstname) VALUES ($1) RETURNING cust_id")
        .bind(&m.guest_first)
        .fetch_one(pool)
        .await
        .expect("seed customer")
        .get("cust_id")
}

/// A booking seeded by SQL rather than through `BookingService::create`, so the
/// create path's own lock plays no part in what is measured. `legacy_book_id`
/// stays NULL — the booking is NOT yet mirrored to iHOTEL, which is the state
/// in which an edit can still promote.
async fn seed_booking(pool: &PgPool, m: &Marks, cust_id: i32, ci: NaiveDate, co: NaiveDate) -> i32 {
    sqlx::query(
        "INSERT INTO ht_bookings (book_no, book_cust_id, book_checkin, book_checkout, book_status) \
         VALUES ($1, $2, $3, $4, 'confirmed') RETURNING book_id",
    )
    .bind(&m.book_no)
    .bind(cust_id)
    .bind(ci)
    .bind(co)
    .fetch_one(pool)
    .await
    .expect("seed booking")
    .get("book_id")
}

async fn assign_room_directly(pool: &PgPool, book_id: i32, room_id: i32) {
    sqlx::query(
        "INSERT INTO ht_booking_rooms (br_book_id, br_room_id, br_price_per_night) \
         VALUES ($1, $2, 1000.00)",
    )
    .bind(book_id)
    .bind(room_id)
    .execute(pool)
    .await
    .expect("seed assigned room");
}

async fn assigned_rooms(pool: &PgPool, book_id: i32) -> Vec<i32> {
    sqlx::query_scalar::<_, i32>(
        "SELECT br_room_id FROM ht_booking_rooms WHERE br_book_id = $1 ORDER BY br_room_id",
    )
    .bind(book_id)
    .fetch_all(pool)
    .await
    .expect("read assigned rooms")
}

/// Every write-back job the booking aggregate enqueued, oldest first. Same
/// shape as `test_ota_parked_promote::writeback_intents` — the intent LABEL is
/// what says which legacy leg was chosen (`create_booking` = the byte-parity
/// promote, `modify_booking` = the diff).
async fn writeback_intents(pool: &PgPool, agg: Uuid) -> Vec<String> {
    sqlx::query("SELECT intent FROM writeback_jobs WHERE aggregate_id = $1 ORDER BY id")
        .bind(agg)
        .fetch_all(pool)
        .await
        .expect("query writeback_jobs")
        .into_iter()
        .map(|row| row.get::<String, _>("intent"))
        .collect()
}

/// Delete every row this test created, children first. Exact-match markers.
async fn cleanup(pool: &PgPool, m: &Marks) {
    if let Ok(rows) = sqlx::query("SELECT book_id FROM ht_bookings WHERE book_no = $1")
        .bind(&m.book_no)
        .fetch_all(pool)
        .await
    {
        for row in rows {
            let book_id: i32 = row.get("book_id");
            let agg = aggregate_uuid(AggregateKind::Booking, book_id);
            sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
                .bind(agg)
                .execute(pool)
                .await
                .ok();
            sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
                .bind(agg)
                .execute(pool)
                .await
                .ok();
        }
    }
    // ht_booking_rooms cascades on br_book_id.
    sqlx::query("DELETE FROM ht_bookings WHERE book_no = $1")
        .bind(&m.book_no)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_customers WHERE cust_firstname = $1")
        .bind(&m.guest_first)
        .execute(pool)
        .await
        .ok();
    for room_no in [&m.room_no, &m.room_no_alt] {
        sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
            .bind(room_no)
            .execute(pool)
            .await
            .ok();
    }
    sqlx::query("DELETE FROM ht_room_types WHERE type_code = $1")
        .bind(&m.type_code)
        .execute(pool)
        .await
        .ok();
}

// ── the tests ───────────────────────────────────────────────────────────────

/// The F6 interleaving, forced. A rival transaction clears the booking's rooms
/// while an edit that re-sends the SAME room set is in flight.
///
/// With the two-snapshot code the edit read "rooms unchanged" on the pool
/// (before the rival committed), skipped the inventory lock, and then wrote
/// with the rival's outcome in effect — a room-consuming legacy write, decided
/// on a room set that no longer existed, with nothing serialising it.
///
/// With one snapshot the edit cannot read anything until it holds the booking
/// row, so it sees the rival's committed (empty) room set, correctly classifies
/// itself as a room-MOVING edit, and blocks on the property lock the test is
/// holding. Only then may it promote.
#[tokio::test]
async fn the_loser_of_a_concurrent_edit_promotes_behind_the_inventory_lock() {
    let pool = common::create_test_pool().await;
    let m = Marks::new("01");
    cleanup(&pool, &m).await; // in case a previous run aborted mid-test

    let (ci, co) = (d("2027-05-01"), d("2027-05-03"));
    let type_id = seed_type(&pool, &m).await;
    let room_id = seed_room(&pool, &m.room_no, type_id).await;
    let cust = seed_customer(&pool, &m).await;
    let book_id = seed_booking(&pool, &m, cust, ci, co).await;
    let agg = aggregate_uuid(AggregateKind::Booking, book_id);

    // The booking starts WITH the room assigned and NOT mirrored to iHOTEL —
    // the state in which re-sending the same room set looks like a no-op edit.
    assign_room_directly(&pool, book_id, room_id).await;

    // The test holds the property lock for the whole race. Any edit that
    // correctly classifies itself as room-moving must park on it.
    let gate = InventoryLock::acquire(&pool, PROPERTY)
        .await
        .expect("the test takes the property lock first");
    assert!(
        !gate.is_bypassed(),
        "BOOKING_INVENTORY_LOCK_ENABLED must be on for this suite to mean anything"
    );

    // The RIVAL edit: holds the booking row and clears the room set, exactly as
    // a concurrent `modify` that releases the room back to the waitlist would,
    // and does not commit yet. Raw SQL rather than a second `modify` call
    // because the whole point is to pin WHEN the rival becomes visible; two
    // service calls would serialise in an order the test cannot choose.
    let mut rival = pool.begin().await.expect("rival tx");
    sqlx::query("SELECT book_id FROM ht_bookings WHERE book_id = $1 FOR NO KEY UPDATE")
        .bind(book_id)
        .fetch_one(&mut *rival)
        .await
        .expect("rival takes the booking row");
    sqlx::query("DELETE FROM ht_booking_rooms WHERE br_book_id = $1")
        .bind(book_id)
        .execute(&mut *rival)
        .await
        .expect("rival clears the room set");

    // The edit under test re-sends the room set the booking shows to anyone
    // reading OUTSIDE the rival's transaction — i.e. "unchanged", the shape
    // that skipped the lock.
    let svc = booking_service(&pool);
    let edit = desk_edit(
        &m,
        book_id,
        cust,
        vec![BookingRoomCommand {
            room_id,
            price_per_night: Some(1000.0),
        }],
        &m.room_no,
        ci,
        co,
    );
    let edit_task = {
        let svc = svc.clone();
        tokio::spawn(async move { svc.modify(edit).await })
    };

    // 1. While the rival holds the booking row, the edit must not have decided
    //    anything — it is parked on the row lock, before either read.
    tokio::time::sleep(SETTLE).await;
    let parked_on_the_row = !edit_task.is_finished();

    rival
        .commit()
        .await
        .expect("rival commits the cleared rooms");

    // 2. THE ASSERTION THIS FILE EXISTS FOR. The edit now holds the booking row
    //    and reads the rival's committed state: 0 rooms, none mirrored. That
    //    makes it a room-MOVING edit and a PROMOTE, so it must be waiting on
    //    the property lock the test still holds. On the two-snapshot code it
    //    had already decided "unchanged / no lock" from the pool read and would
    //    have committed straight through this window.
    tokio::time::sleep(SETTLE).await;
    let parked_on_the_inventory_lock = !edit_task.is_finished();

    gate.release().await.expect("release the gate");

    let finished = finished_within(&edit_task, FINISH_DEADLINE).await;
    let result = edit_task.await.expect("edit task");
    let rooms_after = assigned_rooms(&pool, book_id).await;
    let intents = writeback_intents(&pool, agg).await;

    cleanup(&pool, &m).await;

    // Assertions AFTER cleanup so a failure never leaves fixtures behind.
    assert!(
        parked_on_the_row,
        "the edit must take the booking ROW LOCK before reading anything — with the reads \
         in front of the lock it decides on a snapshot a rival can still invalidate"
    );
    assert!(
        parked_on_the_inventory_lock,
        "B8h: having read the rival's cleared room set from behind the row lock, the edit is \
         a room-MOVING promote and MUST block on the property inventory lock. Finishing here \
         means it promoted UNLOCKED — the #325 F6 defect: a byte-parity legacy CreateBooking \
         emitted inside a live hold's pick window"
    );
    assert!(
        finished,
        "the edit must proceed once the property lock is free"
    );
    assert!(
        result.is_ok(),
        "the edit must succeed once it holds both locks; got {result:?}"
    );
    assert_eq!(
        rooms_after,
        vec![room_id],
        "the edit re-assigns exactly the room it asked for"
    );
    assert_eq!(
        intents,
        vec!["create_booking".to_string()],
        "prior rooms were 0 at the moment this edit held the row, and the booking was never \
         mirrored — so the legacy leg is the byte-parity CreateBooking promote, not a \
         ModifyBooking with no legacy row to target"
    );
}

/// Two REAL concurrent `modify` calls on one parked booking, each assigning a
/// different room. Whoever gets the booking row first promotes; the other must
/// then see a booking that already has a room and take the diff leg.
///
/// Exactly one `create_booking` may exist. Two would mean two transactions both
/// read "0 prior rooms" — which on the two-snapshot code they could, because
/// that read happened before either had locked the row. Both would then derive
/// the SAME deterministic `(CreateBooking, aggregate)` idempotency key, and the
/// loser's enqueue would collide on `writeback_jobs.idempotency_key` and fail
/// the desk's save outright.
#[tokio::test]
async fn two_concurrent_room_assigns_promote_exactly_once() {
    let pool = common::create_test_pool().await;
    let m = Marks::new("02");
    cleanup(&pool, &m).await;

    let (ci, co) = (d("2027-06-01"), d("2027-06-03"));
    let type_id = seed_type(&pool, &m).await;
    let room_a = seed_room(&pool, &m.room_no, type_id).await;
    let room_b = seed_room(&pool, &m.room_no_alt, type_id).await;
    let cust = seed_customer(&pool, &m).await;
    // PARKED: roomless and unmirrored, so either edit on its own would promote.
    let book_id = seed_booking(&pool, &m, cust, ci, co).await;
    let agg = aggregate_uuid(AggregateKind::Booking, book_id);

    let svc = booking_service(&pool);
    let first = {
        let svc = svc.clone();
        let cmd = desk_edit(
            &m,
            book_id,
            cust,
            vec![BookingRoomCommand {
                room_id: room_a,
                price_per_night: Some(1000.0),
            }],
            &m.room_no,
            ci,
            co,
        );
        tokio::spawn(async move { svc.modify(cmd).await })
    };
    let second = {
        let svc = svc.clone();
        let cmd = desk_edit(
            &m,
            book_id,
            cust,
            vec![BookingRoomCommand {
                room_id: room_b,
                price_per_night: Some(1000.0),
            }],
            &m.room_no_alt,
            ci,
            co,
        );
        tokio::spawn(async move { svc.modify(cmd).await })
    };

    let first_result = first.await.expect("first edit task");
    let second_result = second.await.expect("second edit task");
    let rooms_after = assigned_rooms(&pool, book_id).await;
    let mut intents = writeback_intents(&pool, agg).await;

    cleanup(&pool, &m).await;

    assert!(
        first_result.is_ok() && second_result.is_ok(),
        "both concurrent edits must succeed — the loser takes the diff leg, it does not \
         collide on the promote's deterministic idempotency key; got {first_result:?} / \
         {second_result:?}"
    );
    assert_eq!(
        rooms_after.len(),
        1,
        "modify REPLACES the room set, so whichever edit committed last owns the only room"
    );
    assert!(
        rooms_after == vec![room_a] || rooms_after == vec![room_b],
        "the surviving room must be one of the two that were asked for; got {rooms_after:?}"
    );

    let creates = intents.iter().filter(|i| *i == "create_booking").count();
    let modifies = intents.iter().filter(|i| *i == "modify_booking").count();
    intents.sort();
    assert_eq!(
        creates, 1,
        "exactly ONE edit may promote: the second must read the winner's COMMITTED room from \
         behind the booking row lock and take modify_booking. Two promotes means both read 0 \
         prior rooms — the #325 F6 two-snapshot defect. Got {intents:?}"
    );
    assert_eq!(
        modifies, 1,
        "the loser must still enqueue its diff (the booking has a room and a create in \
         flight, so modify_booking is the right leg). Got {intents:?}"
    );
}
