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
use hotel_backend::outbox::{
    generate_idempotency_key, EventBus, EventSource, OutboxRepository, WritebackIntent,
};
use hotel_backend::repository::PgBookingRepository;
use hotel_backend::service::{
    aggregate_uuid, AggregateKind, BookingRoomCommand, BookingService, BookingSnapshotInputs,
    BookingWritebackContext, CreateBookingCommand, ModifyBookingCommand, RoomTypeEdit,
    ServiceError,
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
    sqlx::query(
        "SELECT intent, idempotency_key FROM writeback_jobs WHERE aggregate_id = $1 ORDER BY id",
    )
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

/// B8c / issue #304 / migration 094 — the desk & OTA create path records the
/// room type a booking claims, and the recorded value can never contradict the
/// recorded room.
///
/// Four shapes, one body (shared fixtures; CI runs `--test-threads=1`):
///   (a) PARKED create + `roomTypeId`  → stored as sent (the case the whole
///       feature exists for: a roomless booking has nowhere else to put it)
///   (b) roomed create, `roomTypeId` omitted → DERIVED from the assigned room
///   (c) roomed create, `roomTypeId` contradicting the room → refused (400),
///       and nothing is committed
///   (d) the promote edit (parked booking gains its first room) re-derives
#[tokio::test]
async fn desk_create_records_the_claimed_room_type() {
    let pool = common::create_test_pool().await;
    let (cust_id, room_id, room_no) = create_fixtures(&pool, "03").await;

    // Two types: the room's real one, and a decoy for the disagreement case.
    let type_id: i32 = sqlx::query(
        "INSERT INTO ht_room_types (type_code, type_name, type_base_price, type_max_guests) \
         VALUES ('TEST-OP3A', 'TEST_ota_promote_type_a', 1200.00, 2) \
         ON CONFLICT (type_code) DO UPDATE SET type_name = EXCLUDED.type_name \
         RETURNING type_id",
    )
    .fetch_one(&pool)
    .await
    .expect("seed type A")
    .try_get("type_id")
    .unwrap();
    let decoy_type_id: i32 = sqlx::query(
        "INSERT INTO ht_room_types (type_code, type_name, type_base_price, type_max_guests) \
         VALUES ('TEST-OP3B', 'TEST_ota_promote_type_b', 900.00, 2) \
         ON CONFLICT (type_code) DO UPDATE SET type_name = EXCLUDED.type_name \
         RETURNING type_id",
    )
    .fetch_one(&pool)
    .await
    .expect("seed type B")
    .try_get("type_id")
    .unwrap();
    sqlx::query("UPDATE ht_rooms_new SET room_type_id = $1 WHERE room_id = $2")
        .bind(type_id)
        .bind(room_id)
        .execute(&pool)
        .await
        .expect("type the fixture room");

    let svc = service(&pool);

    async fn stored_type(pool: &sqlx::PgPool, book_id: i32) -> Option<i32> {
        sqlx::query("SELECT book_room_type_id FROM ht_bookings WHERE book_id = $1")
            .bind(book_id)
            .fetch_one(pool)
            .await
            .expect("read booking")
            .try_get::<Option<i32>, _>("book_room_type_id")
            .unwrap()
    }

    fn create_cmd(
        book_no: &str,
        cust_id: i32,
        rooms: Vec<BookingRoomCommand>,
        room_type_id: Option<i32>,
        room_no: &str,
    ) -> CreateBookingCommand {
        CreateBookingCommand {
            book_no: book_no.to_string(),
            book_channel: None,
            book_ext_ref: None,
            book_ext_ref_fingerprint: None,
            // B8e / L3: these fixtures drive the service directly and never
            // race, so they keep the pre-lock path (the channel's shape).
            inventory_lock: None,
            hold_expires_at: None,
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
            rooms,
            room_type_id,
            products: vec![],
            writeback_context: wb_context(cust_id, room_no),
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
        }
    }

    // --- (a) parked create carrying the type ------------------------------
    //
    // Parked on the DECOY type on purpose: the room assigned in (d) belongs to
    // `type_id`, so the promote has to MOVE the attribution. Parking it on the
    // room's own type would make (d) pass against a service that never
    // re-derived at all.
    let parked = svc
        .create(create_cmd(
            "TEST-OTA-PARK-03A",
            cust_id,
            vec![],
            Some(decoy_type_id),
            "",
        ))
        .await
        .expect("parked create with a declared type");
    assert_eq!(
        stored_type(&pool, parked.book_id).await,
        Some(decoy_type_id),
        "(a) a PARKED booking must record the type it claims — it is the only \
         place the claim exists, and what lets the channel subtract per type"
    );

    // --- (b) roomed create, type omitted → derived ------------------------
    let derived = svc
        .create(create_cmd(
            "TEST-OTA-PARK-03B",
            cust_id,
            vec![BookingRoomCommand {
                room_id,
                price_per_night: Some(1200.0),
            }],
            None,
            &room_no,
        ))
        .await
        .expect("roomed create without a declared type");
    assert_eq!(
        stored_type(&pool, derived.book_id).await,
        Some(type_id),
        "(b) an omitted roomTypeId is DERIVED from the assigned room"
    );

    // --- (c) roomed create, type disagrees → refused ----------------------
    let refused = svc
        .create(create_cmd(
            "TEST-OTA-PARK-03C",
            cust_id,
            vec![BookingRoomCommand {
                room_id,
                price_per_night: Some(1200.0),
            }],
            Some(decoy_type_id),
            &room_no,
        ))
        .await;
    // `ServiceError::Validation` specifically — `routes::new_bookings` maps it
    // to 400. A bare `is_err()` would also pass on a 500 from the FK, which is
    // the failure mode the existence check exists to prevent.
    match refused {
        Err(ServiceError::Validation(msg)) => {
            assert!(
                msg.contains("disagrees"),
                "(c) the 400 must say WHICH facts disagree; got: {msg}"
            );
        }
        other => panic!(
            "(c) a roomTypeId contradicting the assigned room must be a \
             validation error (400), got {other:?}"
        ),
    }
    let orphan: i64 = sqlx::query("SELECT COUNT(*) AS n FROM ht_bookings WHERE book_no = $1")
        .bind("TEST-OTA-PARK-03C")
        .fetch_one(&pool)
        .await
        .expect("count")
        .try_get("n")
        .unwrap();
    assert_eq!(orphan, 0, "(c) the refused create must commit nothing");

    // --- (d) promote: the parked booking gains its first room -------------
    svc.modify(ModifyBookingCommand {
        book_id: parked.book_id,
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
        // Still omitted (`Keep`) — with a room now assigned the service
        // re-derives from that room, which is what MOVES the stored type off
        // the decoy it was parked on.
        room_type_id: RoomTypeEdit::Keep,
        changes: empty_changes(),
        promote_context: Some(wb_context(cust_id, &room_no)),
        before_snapshot: None,
        after_snapshot: snapshot(),
        source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
    })
    .await
    .expect("promote edit should succeed");
    assert_eq!(
        stored_type(&pool, parked.book_id).await,
        Some(type_id),
        "(d) assigning the first room must MOVE the attribution off the decoy \
         it was parked on and onto the room's own type — the room is the \
         authoritative fact once one exists"
    );
    assert_ne!(
        stored_type(&pool, parked.book_id).await,
        Some(decoy_type_id),
        "(d) the decoy must not survive the promote"
    );

    for book_id in [parked.book_id, derived.book_id] {
        cleanup(
            &pool,
            aggregate_uuid(AggregateKind::Booking, book_id),
            book_id,
            room_id,
            cust_id,
        )
        .await;
    }
    sqlx::query("DELETE FROM ht_room_types WHERE type_code IN ('TEST-OP3A', 'TEST-OP3B')")
        .execute(&pool)
        .await
        .ok();
}

/// B8c regression — an ORDINARY edit of a parked booking must not wipe its
/// room-type attribution.
///
/// This is the shape the desk actually sends. Both savers omit `roomTypeId`
/// entirely and post `rooms: []` for a booking that has no room yet, so the
/// first cut of migration 094 — which wrote `resolve_room_type(...)`
/// unconditionally on every modify — silently cleared `book_room_type_id` the
/// moment anyone touched a note or a date. The parked claim then fell back to
/// the property-wide cap, which is the pre-#304 behaviour: no error, no log,
/// just the feature quietly undoing itself on first contact with the UI.
///
/// The fix is the tri-state [`RoomTypeEdit`]: absent (`Keep`) is not `Clear`.
#[tokio::test]
async fn ordinary_edit_of_a_parked_booking_preserves_its_room_type() {
    let pool = common::create_test_pool().await;
    let (cust_id, room_id, room_no) = create_fixtures(&pool, "04").await;

    let type_id: i32 = sqlx::query(
        "INSERT INTO ht_room_types (type_code, type_name, type_base_price, type_max_guests) \
         VALUES ('TEST-OP4A', 'TEST_ota_promote_type_keep', 1100.00, 2) \
         ON CONFLICT (type_code) DO UPDATE SET type_name = EXCLUDED.type_name \
         RETURNING type_id",
    )
    .fetch_one(&pool)
    .await
    .expect("seed type")
    .try_get("type_id")
    .unwrap();

    let svc = service(&pool);

    async fn stored_type(pool: &sqlx::PgPool, book_id: i32) -> Option<i32> {
        sqlx::query("SELECT book_room_type_id FROM ht_bookings WHERE book_id = $1")
            .bind(book_id)
            .fetch_one(pool)
            .await
            .expect("read booking")
            .try_get::<Option<i32>, _>("book_room_type_id")
            .unwrap()
    }

    fn parked_edit(
        book_id: i32,
        cust_id: i32,
        notes: &str,
        room_type_id: RoomTypeEdit,
    ) -> ModifyBookingCommand {
        ModifyBookingCommand {
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
            notes: Some(notes.to_string()),
            // Still parked — the desk has not assigned a room yet.
            rooms: vec![],
            room_type_id,
            changes: empty_changes(),
            promote_context: None,
            before_snapshot: None,
            after_snapshot: snapshot(),
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
        }
    }

    let parked = svc
        .create(CreateBookingCommand {
            book_no: "TEST-OTA-PARK-04A".to_string(),
            book_channel: None,
            book_ext_ref: None,
            book_ext_ref_fingerprint: None,
            // B8e / L3: these fixtures drive the service directly and never
            // race, so they keep the pre-lock path (the channel's shape).
            inventory_lock: None,
            hold_expires_at: None,
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
            rooms: vec![],
            room_type_id: Some(type_id),
            products: vec![],
            writeback_context: wb_context(cust_id, ""),
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
        })
        .await
        .expect("parked create");
    assert_eq!(stored_type(&pool, parked.book_id).await, Some(type_id));

    // --- the edit the UI actually sends: no roomTypeId, rooms: [] ---------
    svc.modify(parked_edit(
        parked.book_id,
        cust_id,
        "guest called about a late arrival",
        RoomTypeEdit::Keep,
    ))
    .await
    .expect("ordinary parked edit");
    assert_eq!(
        stored_type(&pool, parked.book_id).await,
        Some(type_id),
        "an edit that never mentions roomTypeId must LEAVE IT ALONE — this is \
         the shape every desk save of a parked booking takes"
    );

    // A second edit must be just as harmless (the bug would have cleared it on
    // the first, so a single-edit test could pass against a one-shot fix).
    svc.modify(parked_edit(
        parked.book_id,
        cust_id,
        "and again",
        RoomTypeEdit::Keep,
    ))
    .await
    .expect("second parked edit");
    assert_eq!(stored_type(&pool, parked.book_id).await, Some(type_id));

    // --- an EXPLICIT null still clears, which is the point of the tri-state -
    svc.modify(parked_edit(
        parked.book_id,
        cust_id,
        "type withdrawn",
        RoomTypeEdit::Clear,
    ))
    .await
    .expect("explicit clear");
    assert_eq!(
        stored_type(&pool, parked.book_id).await,
        None,
        "`\"roomTypeId\": null` is a deliberate clear and must still work — \
         preserving on absent would be worthless if it also swallowed this"
    );

    // --- and an explicit id sets it again ---------------------------------
    svc.modify(parked_edit(
        parked.book_id,
        cust_id,
        "type restored",
        RoomTypeEdit::Set(type_id),
    ))
    .await
    .expect("explicit set");
    assert_eq!(stored_type(&pool, parked.book_id).await, Some(type_id));

    cleanup(
        &pool,
        aggregate_uuid(AggregateKind::Booking, parked.book_id),
        parked.book_id,
        room_id,
        cust_id,
    )
    .await;
    let _ = room_no;
    sqlx::query("DELETE FROM ht_room_types WHERE type_code = 'TEST-OP4A'")
        .execute(&pool)
        .await
        .ok();
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
            book_ext_ref_fingerprint: None,
            // B8e / L3: these fixtures drive the service directly and never
            // race, so they keep the pre-lock path (the channel's shape).
            inventory_lock: None,
            hold_expires_at: None,
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
            // Parked create with no declared type — the #304 fallback shape
            // (migration 094 leaves book_room_type_id NULL).
            room_type_id: None,
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
        // `Keep` = the field was absent from the request. With rooms assigned
        // the service still DERIVES from the room, so this is the ordinary
        // desk shape.
        room_type_id: RoomTypeEdit::Keep,
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
            book_ext_ref_fingerprint: None,
            // B8e / L3: these fixtures drive the service directly and never
            // race, so they keep the pre-lock path (the channel's shape).
            inventory_lock: None,
            hold_expires_at: None,
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
            // Omitted on purpose: the service DERIVES it from the assigned
            // room (migration 094 agree-or-derive rule).
            room_type_id: None,
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
    assert_eq!(
        jobs[0].0, "create_booking",
        "create-with-room enqueues CreateBooking"
    );

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
        // `Keep` = the field was absent from the request. With rooms assigned
        // the service still DERIVES from the room, so this is the ordinary
        // desk shape.
        room_type_id: RoomTypeEdit::Keep,
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
