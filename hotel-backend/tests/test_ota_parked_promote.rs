//! OTA "parked booking" promote-to-CreateBooking integration tests (Part 3 of
//! the New-OTA-bookings workflow).
//!
//! Proves the load-bearing coexistence change: a booking created ROOMLESS
//! (canonical-only, no legacy write) that is later assigned its FIRST room via
//! the edit path must enqueue a byte-parity `CreateBooking` (so the front desk
//! assigning the room in the PMS produces the real iHOTEL booking), while an
//! already-mirrored booking keeps taking the normal `ModifyBooking` path.
//!
//! These exercise the REAL `BookingService` against PostgreSQL. Like the rest
//! of the suite they read `DATABASE_URL` (CI provides a service container); the
//! runtime assertions only fire once a connection is established. `cargo test
//! --no-run` still compiles them without a DB.
//!
//! Coverage (per the workstream spec):
//!   (a) roomless create           → NO writeback job
//!   (b) later first-room add       → exactly ONE `CreateBooking` job
//!   (c) that job's key is the DETERMINISTIC create key (ledger-idempotent on
//!       retry), and a duplicate enqueue of it is rejected by the DB
//!   (d) an already-mirrored booking (legacy_book_id set) → `ModifyBooking`

mod common;

use std::sync::Arc;

use chrono::{NaiveDate, TimeZone, Utc};
use sqlx::Row;
use uuid::Uuid;

use hotel_backend::domain::booking::BookingState;
use hotel_backend::domain::shared::{DateRange, Money};
use hotel_backend::outbox::intent::BookingChanges;
use hotel_backend::outbox::{generate_idempotency_key, EventBus, EventSource, OutboxRepository, WritebackIntent};
use hotel_backend::repository::PgBookingRepository;
use hotel_backend::service::{
    aggregate_uuid, AggregateKind, BookingRoomCommand, BookingService, BookingSnapshotInputs,
    BookingWritebackContext, CreateBookingCommand, ModifyBookingCommand,
};

const CI: NaiveDate = date(2026, 8, 10);
const CO: NaiveDate = date(2026, 8, 12);

const fn date(y: i32, m: u32, d: u32) -> NaiveDate {
    match NaiveDate::from_ymd_opt(y, m, d) {
        Some(d) => d,
        None => panic!("bad test date"),
    }
}

fn utc(d: NaiveDate) -> chrono::DateTime<Utc> {
    Utc.from_utc_datetime(&d.and_hms_opt(0, 0, 0).unwrap())
}

fn service(pool: &sqlx::PgPool) -> BookingService {
    BookingService::new(
        Arc::new(PgBookingRepository::new()),
        Arc::new(OutboxRepository::new()),
        Arc::new(EventBus::new()),
        pool.clone(),
    )
}

/// The write-back context the route builds for the recipe (customer + first
/// room). `room_no` is empty for the roomless create; populated for a promote.
fn wb_context(cust_id: i32, room_no: &str) -> BookingWritebackContext {
    BookingWritebackContext {
        customer_aggregate_id: aggregate_uuid(AggregateKind::Customer, cust_id),
        legacy_cust_no: None,
        customer_name: "OTA Parked Guest".to_string(),
        customer_phone: None,
        stay: DateRange::new(utc(CI), utc(CO)),
        room_no: room_no.to_string(),
        room_type: "TEST-PARK".to_string(),
        price: Money::from_baht(1200),
        deposit: Money::ZERO,
        created_by: "ota-desk".to_string(),
        notes: None,
    }
}

fn empty_changes() -> BookingChanges {
    BookingChanges {
        new_stay: None,
        new_room_no: None,
        new_room_type: None,
        new_price: None,
        new_state: None,
        new_notes: None,
        new_customer_phone: None,
        new_customer_name: None,
        customer_resave: None,
    }
}

fn snapshot() -> BookingSnapshotInputs {
    BookingSnapshotInputs {
        legacy_book_id: None,
        state: BookingState::Pending,
        stay_start: utc(CI),
        stay_end: utc(CO),
        room_no: None,
        price: Money::from_baht(1200),
    }
}

async fn create_fixtures(pool: &sqlx::PgPool, suffix: &str) -> (i32, i32, String) {
    let marker = format!("TEST_ota_promote_{}", suffix);
    let row = sqlx::query(
        "INSERT INTO ht_customers (cust_firstname, cust_lastname, cust_notes) \
         VALUES ($1, 'Guest', $2) RETURNING cust_id",
    )
    .bind(format!("OtaPark{}", suffix))
    .bind(&marker)
    .fetch_one(pool)
    .await
    .expect("INSERT customer fixture");
    let cust_id: i32 = row.try_get("cust_id").unwrap();

    let room_no = format!("OP{}", suffix);
    let row = sqlx::query(
        "INSERT INTO ht_rooms_new (room_no, room_floor, room_status, room_notes) \
         VALUES ($1, 2, 'available', $2) \
         ON CONFLICT (room_no) DO UPDATE SET room_notes = EXCLUDED.room_notes \
         RETURNING room_id",
    )
    .bind(&room_no)
    .bind(&marker)
    .fetch_one(pool)
    .await
    .expect("INSERT room fixture");
    let room_id: i32 = row.try_get("room_id").unwrap();

    (cust_id, room_id, room_no)
}

async fn cleanup(pool: &sqlx::PgPool, agg: Uuid, book_id: i32, room_id: i32, cust_id: i32) {
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
    sqlx::query("DELETE FROM ht_booking_rooms WHERE br_book_id = $1")
        .bind(book_id)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_bookings WHERE book_id = $1")
        .bind(book_id)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
        .bind(room_id)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_customers WHERE cust_id = $1")
        .bind(cust_id)
        .execute(pool)
        .await
        .ok();
}

async fn writeback_intents(pool: &sqlx::PgPool, agg: Uuid) -> Vec<(String, Uuid)> {
    sqlx::query("SELECT intent, idempotency_key FROM writeback_jobs WHERE aggregate_id = $1 ORDER BY id")
        .bind(agg)
        .fetch_all(pool)
        .await
        .expect("query writeback_jobs")
        .into_iter()
        .map(|r| {
            (
                r.try_get::<String, _>("intent").unwrap(),
                r.try_get::<Uuid, _>("idempotency_key").unwrap(),
            )
        })
        .collect()
}

/// (a) roomless create → no legacy write; (b) later first-room add → exactly one
/// byte-parity CreateBooking; (c) with the deterministic (ledger-idempotent) key.
#[tokio::test]
async fn parked_roomless_booking_promotes_to_create_on_room_assign() {
    let pool = common::create_test_pool().await;
    let (cust_id, room_id, room_no) = create_fixtures(&pool, "01").await;
    let svc = service(&pool);

    // --- (a) create ROOMLESS ---
    let outcome = svc
        .create(CreateBookingCommand {
            book_no: "TEST-OTA-PARK-01".to_string(),
            book_channel: None,
            book_ext_ref: None,
            customer_id: cust_id,
            check_in: CI,
            check_out: CO,
            adults: 2,
            children: 0,
            status: "pending".to_string(),
            source_label: Some("ota".to_string()),
            total_amount: Some(2400.0),
            deposit_amount: None,
            notes: None,
            rooms: vec![], // parked — no room yet
            products: vec![],
            writeback_context: wb_context(cust_id, ""),
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
        })
        .await
        .expect("roomless create should succeed");

    let book_id = outcome.book_id;
    let agg = aggregate_uuid(AggregateKind::Booking, book_id);

    let jobs = writeback_intents(&pool, agg).await;
    assert!(
        jobs.is_empty(),
        "(a) a roomless create must enqueue NO legacy write-back; got {jobs:?}"
    );

    // --- (b) assign the first room via the edit path → promote to CreateBooking ---
    svc.modify(ModifyBookingCommand {
        book_id,
        customer_id: cust_id,
        check_in: CI,
        check_out: CO,
        adults: 2,
        children: 0,
        status: "pending".to_string(),
        source_label: Some("ota".to_string()),
        total_amount: Some(2400.0),
        deposit_amount: None,
        notes: None,
        rooms: vec![BookingRoomCommand {
            room_id,
            price_per_night: Some(1200.0),
        }],
        changes: empty_changes(),
        promote_context: Some(wb_context(cust_id, &room_no)),
        before_snapshot: None,
        after_snapshot: snapshot(),
        source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
    })
    .await
    .expect("room-assign modify should succeed");

    let jobs = writeback_intents(&pool, agg).await;
    assert_eq!(
        jobs.len(),
        1,
        "(b) assigning the first room must enqueue exactly ONE write-back; got {jobs:?}"
    );
    assert_eq!(
        jobs[0].0, "create_booking",
        "(b) the promoted job must be a byte-parity CreateBooking, not a ModifyBooking"
    );

    // --- (c) that job's key is the DETERMINISTIC create key (ledger-idempotent) ---
    let expected_key = generate_idempotency_key(
        &WritebackIntent::CreateBooking {
            booking_id: agg,
            // key derivation is payload-independent, so any payload with this
            // aggregate reproduces the key.
            payload: hotel_backend::outbox::intent::CreateBookingPayload {
                customer_id: aggregate_uuid(AggregateKind::Customer, cust_id),
                legacy_cust_no: None,
                customer_name: String::new(),
                customer_phone: None,
                stay: DateRange::new(utc(CI), utc(CO)),
                room_no: room_no.clone(),
                room_type: "TEST-PARK".to_string(),
                price: Money::from_baht(1200),
                nights: 2,
                deposit: Money::ZERO,
                created_by: "ota-desk".to_string(),
                notes: None,
            },
        },
        agg,
    );
    assert_eq!(
        jobs[0].1, expected_key,
        "(c) the promoted CreateBooking must use the deterministic create key so a \
         worker retry maps to the same ledger row"
    );

    // A duplicate enqueue of that key is rejected by the DB unique constraint
    // (the retry backstop that prevents a double legacy write).
    let dup = sqlx::query(
        "INSERT INTO writeback_jobs (intent, payload, aggregate_id, idempotency_key, status) \
         VALUES ('CreateBooking', '{}'::jsonb, $1, $2, 'pending')",
    )
    .bind(agg)
    .bind(expected_key)
    .execute(&pool)
    .await;
    assert!(
        dup.is_err(),
        "(c) re-enqueuing the same idempotency_key must be rejected (unique violation)"
    );

    cleanup(&pool, agg, book_id, room_id, cust_id).await;
}

/// (d) An already-mirrored booking (resolved legacy_book_id) takes the normal
/// ModifyBooking path, not a second CreateBooking.
#[tokio::test]
async fn already_mirrored_booking_takes_modify_path() {
    let pool = common::create_test_pool().await;
    let (cust_id, room_id, room_no) = create_fixtures(&pool, "02").await;
    let svc = service(&pool);

    // Create WITH a room (enqueues CreateBooking), then simulate the worker
    // back-populating the legacy id so the booking is "mirrored".
    let outcome = svc
        .create(CreateBookingCommand {
            book_no: "TEST-OTA-PARK-02".to_string(),
            book_channel: None,
            book_ext_ref: None,
            customer_id: cust_id,
            check_in: CI,
            check_out: CO,
            adults: 1,
            children: 0,
            status: "pending".to_string(),
            source_label: Some("ota".to_string()),
            total_amount: Some(1200.0),
            deposit_amount: None,
            notes: None,
            rooms: vec![BookingRoomCommand {
                room_id,
                price_per_night: Some(1200.0),
            }],
            products: vec![],
            writeback_context: wb_context(cust_id, &room_no),
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
        })
        .await
        .expect("create-with-room should succeed");

    let book_id = outcome.book_id;
    let agg = aggregate_uuid(AggregateKind::Booking, book_id);

    let jobs = writeback_intents(&pool, agg).await;
    assert_eq!(jobs.len(), 1);
    assert_eq!(jobs[0].0, "create_booking", "create-with-room enqueues CreateBooking");

    // Simulate worker back-population of the legacy id → booking is now mirrored.
    sqlx::query("UPDATE ht_bookings SET legacy_book_id = 'R999002' WHERE book_id = $1")
        .bind(book_id)
        .execute(&pool)
        .await
        .expect("stamp legacy_book_id");

    // Edit the mirrored booking (keep the room) → must be a ModifyBooking.
    svc.modify(ModifyBookingCommand {
        book_id,
        customer_id: cust_id,
        check_in: CI,
        check_out: CO,
        adults: 1,
        children: 0,
        status: "confirmed".to_string(),
        source_label: Some("ota".to_string()),
        total_amount: Some(1200.0),
        deposit_amount: None,
        notes: Some("edited".to_string()),
        rooms: vec![BookingRoomCommand {
            room_id,
            price_per_night: Some(1200.0),
        }],
        changes: empty_changes(),
        promote_context: Some(wb_context(cust_id, &room_no)),
        before_snapshot: None,
        after_snapshot: snapshot(),
        source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
    })
    .await
    .expect("modify of mirrored booking should succeed");

    let intents: Vec<String> = writeback_intents(&pool, agg)
        .await
        .into_iter()
        .map(|(i, _)| i)
        .collect();
    assert!(
        intents.iter().any(|i| i == "modify_booking"),
        "(d) editing an already-mirrored booking must enqueue a ModifyBooking; got {intents:?}"
    );
    assert_eq!(
        intents.iter().filter(|i| *i == "create_booking").count(),
        1,
        "(d) must NOT enqueue a second CreateBooking; got {intents:?}"
    );

    cleanup(&pool, agg, book_id, room_id, cust_id).await;
}
