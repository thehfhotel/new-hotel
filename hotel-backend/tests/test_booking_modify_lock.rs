//! B8g — the booking EDIT path takes the per-property inventory lock, but only
//! when the edit moves inventory.
//!
//! ## What was broken
//!
//! B8e (#311) put every room-CONSUMING write behind one property-wide advisory
//! lock — the channel's `create_hold` and `BookingService::create`. It left
//! `BookingService::modify` out, and `modify` replaces a booking's whole room
//! set: the desk assigning the first room to a parked booking, or swapping one
//! room for another, consumes exactly the room-nights a concurrent loyalty hold
//! is choosing between its `pick_free_room` SELECT and its INSERT. So the edit
//! flow was the one desk path that could still hand a hold's freshly-picked
//! last room to a walk-in.
//!
//! ## What these tests pin
//!
//! * **It blocks.** A modify that ASSIGNS the type's last room waits while the
//!   property lock is held, and completes once it is released. Ordering is
//!   forced by the test holding the same public lock, so this is deterministic
//!   rather than a timing lottery.
//! * **It does not over-block.** A notes-only edit — same room set — takes
//!   NOTHING and completes while the lock is held. Without that half, the
//!   correct fix would be "lock every booking save", which serialises every
//!   desk edit at a property behind every create for a race it cannot lose.
//!
//! ## Running
//!
//! `common` reads `DATABASE_URL` (CI provides a service container and runs
//! `--test-threads=1`). Every fixture row carries a `TEST_b8glock`-scoped
//! marker unique to THIS file and is deleted by `cleanup` (exact-match, per the
//! `common` rules). One test function, one disjoint far-future stay window —
//! same reasoning as `test_channel_last_room.rs`.

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

/// Unique-to-this-file fixture markers (see tests/common/mod.rs cleanup rules).
const TYPE_CODE: &str = "TSTB8G";
const TYPE_NAME: &str = "TEST_b8glock_type";
const ROOM_NO: &str = "TB8G01";
const GUEST_FIRST: &str = "TEST_b8glock_guest";
const BOOK_NO: &str = "TESTB8G000001";

/// The property every scenario locks on — the same literal
/// `routes::channel::parse_property` and `routes::new_bookings::branch_property`
/// yield for `"hf"`.
const PROPERTY: &str = "hf";

/// How long the BLOCKED-side assertion waits before concluding the task really
/// is parked on the lock. Comfortably longer than an uncontended modify (a
/// handful of statements, low single-digit ms) and far inside the lock's 5 s
/// acquire deadline. This assertion fails SAFE: a slow runner makes it more
/// likely to still be unfinished, not less.
const SETTLE: Duration = Duration::from_millis(400);

/// Deadline for the UNBLOCKED-side assertion, which does NOT fail safe — a
/// fixed sleep would flake on a loaded CI runner that simply had not scheduled
/// the task yet. Polled to this ceiling instead, so the test only fails when
/// the edit genuinely did not finish while the lock was held.
const UNBLOCKED_DEADLINE: Duration = Duration::from_secs(2);

/// Poll `task.is_finished()` until it flips or `deadline` passes.
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

fn source() -> EventSource {
    EventSource::our_app(Uuid::nil(), Uuid::new_v4())
}

fn booking_service(pool: &PgPool) -> Arc<BookingService> {
    Arc::new(BookingService::new(
        Arc::new(PgBookingRepository::new()),
        Arc::new(OutboxRepository::new()),
        Arc::new(EventBus::new()),
        pool.clone(),
    ))
}

fn wb_context(cust_id: i32, check_in: NaiveDate, check_out: NaiveDate) -> BookingWritebackContext {
    BookingWritebackContext {
        customer_aggregate_id: aggregate_uuid(AggregateKind::Customer, cust_id),
        legacy_cust_no: None,
        customer_name: "TEST b8glock guest".to_string(),
        customer_phone: None,
        stay: DateRange::new(utc(check_in), utc(check_out)),
        room_no: ROOM_NO.to_string(),
        room_type: TYPE_NAME.to_string(),
        price: Money::from_baht(1000),
        deposit: Money::ZERO,
        created_by: String::new(),
        notes: None,
    }
}

/// The DESK edit, as `routes::new_bookings::update_booking` builds it —
/// `inventory_lock: Some(property)` on every save, room-moving or not; the
/// SERVICE is what decides whether the lock is actually taken.
fn desk_edit(
    book_id: i32,
    cust_id: i32,
    rooms: Vec<BookingRoomCommand>,
    notes: Option<String>,
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> ModifyBookingCommand {
    let promote_context = (!rooms.is_empty()).then(|| wb_context(cust_id, check_in, check_out));
    ModifyBookingCommand {
        book_id,
        customer_id: cust_id,
        check_in,
        check_out,
        adults: 2,
        children: 0,
        status: "confirmed".to_string(),
        source_label: Some("walkin".to_string()),
        total_amount: Some(2000.0),
        deposit_amount: None,
        notes,
        rooms,
        room_type_id: RoomTypeEdit::Keep,
        changes: BookingChanges {
            new_stay: Some(DateRange::new(utc(check_in), utc(check_out))),
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
            stay_start: utc(check_in),
            stay_end: utc(check_out),
            room_no: None,
            price: Money::from_baht(1000),
        },
        source: source(),
    }
}

// ── fixtures ────────────────────────────────────────────────────────────────

async fn seed_type(pool: &PgPool) -> i32 {
    sqlx::query(
        "INSERT INTO ht_room_types (type_code, type_name, type_description, type_base_price, type_max_guests, type_active) \
         VALUES ($1, $2, 'modify-lock test type', 1000.00, 2, true) RETURNING type_id",
    )
    .bind(TYPE_CODE)
    .bind(TYPE_NAME)
    .fetch_one(pool)
    .await
    .expect("seed room type")
    .get("type_id")
}

async fn seed_room(pool: &PgPool, type_id: i32) -> i32 {
    sqlx::query(
        "INSERT INTO ht_rooms_new (room_no, room_type_id, room_status, room_active, room_maintenance) \
         VALUES ($1, $2, 'available', true, false) RETURNING room_id",
    )
    .bind(ROOM_NO)
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

/// A PARKED (roomless) booking — the shape the desk edit promotes by assigning
/// its first room. Seeded by SQL rather than through `BookingService::create`
/// so the create path's own lock plays no part in what is measured here.
async fn seed_parked_booking(
    pool: &PgPool,
    cust_id: i32,
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> i32 {
    sqlx::query(
        "INSERT INTO ht_bookings (book_no, book_cust_id, book_checkin, book_checkout, book_status) \
         VALUES ($1, $2, $3, $4, 'confirmed') RETURNING book_id",
    )
    .bind(BOOK_NO)
    .bind(cust_id)
    .bind(check_in)
    .bind(check_out)
    .fetch_one(pool)
    .await
    .expect("seed parked booking")
    .get("book_id")
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

/// Delete every row this file created, children first. Exact-match markers.
async fn cleanup(pool: &PgPool) {
    sqlx::query(
        "DELETE FROM writeback_jobs WHERE aggregate_id IN \
         (SELECT aggregate_id FROM ht_bookings WHERE book_no = $1 AND aggregate_id IS NOT NULL)",
    )
    .bind(BOOK_NO)
    .execute(pool)
    .await
    .ok();
    // `modify` publishes under the DETERMINISTIC booking aggregate uuid whether
    // or not the row carries it, so clear by both routes.
    if let Ok(rows) = sqlx::query("SELECT book_id FROM ht_bookings WHERE book_no = $1")
        .bind(BOOK_NO)
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
        .bind(BOOK_NO)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_customers WHERE cust_firstname = $1")
        .bind(GUEST_FIRST)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
        .bind(ROOM_NO)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_room_types WHERE type_code = $1")
        .bind(TYPE_CODE)
        .execute(pool)
        .await
        .ok();
}

// ── the test ────────────────────────────────────────────────────────────────

#[tokio::test]
async fn a_room_moving_edit_waits_for_the_property_lock_and_a_notes_only_edit_does_not() {
    let pool = common::create_test_pool().await;
    // Pre-clean in case a previous run aborted mid-test.
    cleanup(&pool).await;

    let (check_in, check_out) = (d("2027-04-01"), d("2027-04-03"));
    let type_id = seed_type(&pool).await;
    let room_id = seed_room(&pool, type_id).await;
    let cust = seed_customer(&pool).await;
    let book_id = seed_parked_booking(&pool, cust, check_in, check_out).await;

    // ---------------------------------------------------------------------
    // 1. The edit that MOVES inventory blocks while the lock is held.
    //
    // The seeded room is the only one of its type, so this edit is the desk
    // taking the last room of that type — the write a concurrent loyalty hold
    // must not be able to interleave with.
    // ---------------------------------------------------------------------
    let gate = InventoryLock::acquire(&pool, PROPERTY)
        .await
        .expect("the test takes the property lock first");
    assert!(
        !gate.is_bypassed(),
        "BOOKING_INVENTORY_LOCK_ENABLED must be on for this suite to mean anything"
    );

    let svc = booking_service(&pool);
    let assign = desk_edit(
        book_id,
        cust,
        vec![BookingRoomCommand {
            room_id,
            price_per_night: Some(1000.0),
        }],
        None,
        check_in,
        check_out,
    );
    let assign_task = {
        let svc = svc.clone();
        tokio::spawn(async move { svc.modify(assign).await })
    };

    tokio::time::sleep(SETTLE).await;
    let blocked = !assign_task.is_finished();

    gate.release().await.expect("release the gate");

    let assign_result = assign_task.await.expect("assign task");
    let rooms_after_assign = assigned_rooms(&pool, book_id).await;

    // ---------------------------------------------------------------------
    // 2. The notes-only edit takes NOTHING and runs straight through.
    //
    // Same room set as the booking now carries — which is exactly what both
    // desk savers re-send on an ordinary save. If this one waited, every edit
    // at the property would queue behind every create.
    // ---------------------------------------------------------------------
    let gate = InventoryLock::acquire(&pool, PROPERTY)
        .await
        .expect("take the property lock again");

    let notes_only = desk_edit(
        book_id,
        cust,
        vec![BookingRoomCommand {
            room_id,
            price_per_night: Some(1000.0),
        }],
        Some("TEST_b8glock note".to_string()),
        check_in,
        check_out,
    );
    let notes_task = {
        let svc = svc.clone();
        tokio::spawn(async move { svc.modify(notes_only).await })
    };

    let ran_unlocked = finished_within(&notes_task, UNBLOCKED_DEADLINE).await;

    gate.release().await.expect("release the second gate");

    let notes_result = notes_task.await.expect("notes task");
    let rooms_after_notes = assigned_rooms(&pool, book_id).await;

    cleanup(&pool).await;

    // Assertions AFTER cleanup so a failure never leaves fixtures behind for
    // the next run to trip over.
    assert!(
        blocked,
        "a room-assigning edit must BLOCK on the property's inventory lock while it is held \
         — this is the B8g gap: without the lock it commits inside a live hold's pick window"
    );
    assert!(
        assign_result.is_ok(),
        "the edit must proceed once the lock is free; got {assign_result:?}"
    );
    assert_eq!(
        rooms_after_assign,
        vec![room_id],
        "the promoted booking must carry exactly the assigned room"
    );

    assert!(
        ran_unlocked,
        "a notes-only edit moves no inventory and must NOT wait on the property lock"
    );
    assert!(
        notes_result.is_ok(),
        "the notes-only edit must succeed; got {notes_result:?}"
    );
    assert_eq!(
        rooms_after_notes,
        vec![room_id],
        "a notes-only edit must leave the room set alone"
    );
}
