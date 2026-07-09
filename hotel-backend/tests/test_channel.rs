//! Loyalty-channel integration tests (docs/loyalty-channel.md).
//!
//! Exercises the real PostgreSQL plumbing behind `/api/channel/*`:
//! availability math (half-open overlap, maintenance/active predicates,
//! guest-capacity filter), hold create (consumes availability, stamps the
//! 2h deadline, enqueues the CreateBooking writeback), payment-verified
//! (confirm + idempotent replay + refuses released holds), release
//! (idempotent replay + refuses confirmed holds), the expiry sweep, and the
//! match-or-create guest resolution with membership attach.
//!
//! ## Running
//!
//! Requires a running PG with migration 078 applied. `common` reads
//! `DATABASE_URL` (fallback: local-dev DSN). CI runs `--test-threads=1`;
//! every fixture row carries a `TEST_loyalty`-scoped marker unique to THIS
//! file and is deleted by `cleanup` (exact-match, per the `common` rules).

mod common;

use std::sync::Arc;

use chrono::{Duration, NaiveDate, Utc};
use sqlx::{PgPool, Row};

use hotel_backend::outbox::event::EventSource;
use hotel_backend::outbox::{EventBus, OutboxRepository};
use hotel_backend::repository::channel as channel_repo;
use hotel_backend::repository::{
    CustomerRepository, PgBookingRepository, PgCustomerRepository,
};
use hotel_backend::service::{
    BookingService, ChannelService, CreateHoldCommand, CustomerService, PaymentPlan, ServiceError,
};
use uuid::Uuid;

/// Unique-to-this-file fixture markers (see tests/common/mod.rs cleanup rules).
const TYPE_CODE: &str = "TSTLC";
const TYPE_NAME: &str = "TEST_loyalty_type";
const ROOM_A: &str = "TL01";
const ROOM_B: &str = "TL02";
const GUEST_FIRST: &str = "TEST_loyalty_guest";
const BOOK_NO_PREFIX: &str = "TESTLC";

fn source() -> EventSource {
    EventSource::our_app(Uuid::nil(), Uuid::new_v4())
}

fn service_for(pool: &PgPool) -> ChannelService {
    let outbox = Arc::new(OutboxRepository::new());
    let events = Arc::new(EventBus::new());
    let customers_repo: Arc<dyn CustomerRepository> = Arc::new(PgCustomerRepository::new());
    let bookings = Arc::new(BookingService::new(
        Arc::new(PgBookingRepository::new()),
        outbox.clone(),
        events.clone(),
        pool.clone(),
    ));
    let customers = Arc::new(CustomerService::new(
        customers_repo.clone(),
        outbox,
        events,
        pool.clone(),
    ));
    ChannelService::new(pool.clone(), bookings, customers, customers_repo)
}

/// Seed one test room type (sleeps 2) with two active rooms. Returns type_id.
async fn seed_rooms(pool: &PgPool) -> i32 {
    let type_id: i32 = sqlx::query(
        "INSERT INTO ht_room_types (type_code, type_name, type_description, type_base_price, type_max_guests, type_active) \
         VALUES ($1, $2, 'loyalty test type', 1200.00, 2, true) RETURNING type_id",
    )
    .bind(TYPE_CODE)
    .bind(TYPE_NAME)
    .fetch_one(pool)
    .await
    .expect("seed room type")
    .get("type_id");

    for room_no in [ROOM_A, ROOM_B] {
        sqlx::query(
            "INSERT INTO ht_rooms_new (room_no, room_type_id, room_status, room_active, room_maintenance) \
             VALUES ($1, $2, 'available', true, false)",
        )
        .bind(room_no)
        .bind(type_id)
        .execute(pool)
        .await
        .expect("seed room");
    }
    type_id
}

/// Delete every row this file created, children first. Exact-match markers.
async fn cleanup(pool: &PgPool) {
    // Outbox / event rows for our bookings + customers.
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
    // Customer events: `CustomerService::create` publishes with the
    // deterministic aggregate uuid but does not necessarily stamp it onto the
    // row — recompute it from the SERIAL ids instead of trusting the column.
    if let Ok(rows) = sqlx::query("SELECT cust_id FROM ht_customers WHERE cust_firstname = $1")
        .bind(GUEST_FIRST)
        .fetch_all(pool)
        .await
    {
        for row in rows {
            let cust_id: i32 = row.get("cust_id");
            let agg = hotel_backend::service::aggregate_uuid(
                hotel_backend::service::AggregateKind::Customer,
                cust_id,
            );
            sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
                .bind(agg)
                .execute(pool)
                .await
                .ok();
        }
    }

    // Check-ins seeded as occupancy conflicts (marker in cin_notes).
    sqlx::query("DELETE FROM ht_checkins WHERE cin_notes = 'TEST_loyalty_channel'")
        .execute(pool)
        .await
        .ok();
    // Bookings (ht_booking_rooms rows cascade on br_book_id).
    sqlx::query("DELETE FROM ht_bookings WHERE book_no LIKE $1")
        .bind(format!("{BOOK_NO_PREFIX}%"))
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_customers WHERE cust_firstname = $1")
        .bind(GUEST_FIRST)
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

fn d(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
}

/// Find our test type's row in an availability result.
fn count_for(
    rows: &[channel_repo::RoomTypeAvailability],
    type_id: i32,
) -> Option<&channel_repo::RoomTypeAvailability> {
    rows.iter().find(|r| r.type_id == type_id)
}

fn hold_cmd(book_no: &str, type_id: i32, check_in: &str, check_out: &str) -> CreateHoldCommand {
    CreateHoldCommand {
        book_no: book_no.to_string(),
        room_type_id: type_id,
        check_in: d(check_in),
        check_out: d(check_out),
        guests: 2,
        guest_name: format!("{GUEST_FIRST} Somsri"),
        guest_phone: "0899990001".to_string(),
        membership_id: Some("TEST-LOYAL-M1".to_string()),
        payment: PaymentPlan::Deposit50,
        source: source(),
    }
}

/// One mega-test instead of many small ones: the fixtures are shared and the
/// scenarios build on each other (hold → confirm → release → sweep), and a
/// single body guarantees cleanup ordering without cross-test interference
/// (see tests/common/mod.rs on marker scoping).
#[tokio::test]
async fn loyalty_channel_end_to_end() {
    let pool = common::create_test_pool().await;
    // Pre-clean in case a previous run aborted mid-test.
    cleanup(&pool).await;

    let type_id = seed_rooms(&pool).await;
    let svc = service_for(&pool);

    // Windows far in the future so live data can never overlap the fixtures.
    let w1 = ("2126-08-01", "2126-08-03"); // availability + hold lifecycle
    let w2 = ("2126-09-01", "2126-09-02"); // second guest-match hold
    let w3 = ("2126-10-01", "2126-10-02"); // sweep hold

    // ---------- availability math ----------

    // Both rooms free.
    let rows = svc.availability(d(w1.0), d(w1.1), 2).await.expect("availability");
    let t = count_for(&rows, type_id).expect("test type present");
    assert_eq!(t.available_count, 2, "both rooms free");
    assert_eq!(t.nightly_price, 1200.0, "quoted from type_base_price");
    assert_eq!(t.name, TYPE_NAME);

    // Guest-capacity filter: type sleeps 2, ask for 3 → type excluded.
    let rows = svc.availability(d(w1.0), d(w1.1), 3).await.expect("availability");
    assert!(
        count_for(&rows, type_id).is_none(),
        "type_max_guests=2 must exclude the type for guests=3"
    );

    // Confirmed booking on ROOM_A overlapping w1 blocks it.
    let cust_id: i32 = sqlx::query(
        "INSERT INTO ht_customers (cust_firstname) VALUES ($1) RETURNING cust_id",
    )
    .bind(GUEST_FIRST)
    .fetch_one(&pool)
    .await
    .expect("seed conflict customer")
    .get("cust_id");
    let room_a_id: i32 = sqlx::query("SELECT room_id FROM ht_rooms_new WHERE room_no = $1")
        .bind(ROOM_A)
        .fetch_one(&pool)
        .await
        .expect("room A id")
        .get("room_id");
    let conflict_book_id: i32 = sqlx::query(
        "INSERT INTO ht_bookings (book_no, book_cust_id, book_checkin, book_checkout, book_status) \
         VALUES ($1, $2, $3, $4, 'confirmed') RETURNING book_id",
    )
    .bind(format!("{BOOK_NO_PREFIX}-C1"))
    .bind(cust_id)
    .bind(d(w1.0))
    .bind(d(w1.1))
    .fetch_one(&pool)
    .await
    .expect("seed conflict booking")
    .get("book_id");
    sqlx::query("INSERT INTO ht_booking_rooms (br_book_id, br_room_id) VALUES ($1, $2)")
        .bind(conflict_book_id)
        .bind(room_a_id)
        .execute(&pool)
        .await
        .expect("seed conflict booking room");

    let rows = svc.availability(d(w1.0), d(w1.1), 2).await.expect("availability");
    assert_eq!(
        count_for(&rows, type_id).unwrap().available_count,
        1,
        "overlapping confirmed booking consumes one room"
    );

    // Half-open interval: a booking ENDING on the window start does not block.
    let rows = svc
        .availability(d(w1.1), d("2126-08-05"), 2)
        .await
        .expect("availability");
    assert_eq!(
        count_for(&rows, type_id).unwrap().available_count,
        2,
        "checkout day is bookable (half-open [ci, co))"
    );

    // Active check-in on ROOM_B overlapping w1 blocks the second room.
    sqlx::query(
        "INSERT INTO ht_checkins \
           (cin_no, cin_cust_id, cin_room_id, cin_checkin_time, cin_expected_checkout, cin_status, cin_notes) \
         VALUES ('TESTLC-CIN1', $1, $2, $3::date::timestamp, $4, 'active', 'TEST_loyalty_channel')",
    )
    .bind(cust_id)
    .bind(
        sqlx::query("SELECT room_id FROM ht_rooms_new WHERE room_no = $1")
            .bind(ROOM_B)
            .fetch_one(&pool)
            .await
            .expect("room B id")
            .get::<i32, _>("room_id"),
    )
    .bind(d(w1.0))
    .bind(d(w1.1))
    .execute(&pool)
    .await
    .expect("seed occupancy conflict");

    let rows = svc.availability(d(w1.0), d(w1.1), 2).await.expect("availability");
    assert_eq!(
        count_for(&rows, type_id).unwrap().available_count,
        0,
        "active check-in consumes the other room"
    );

    // Sold out ⇒ hold create must refuse with Conflict.
    match svc.create_hold(hold_cmd(&format!("{BOOK_NO_PREFIX}-X1"), type_id, w1.0, w1.1)).await {
        Err(ServiceError::Conflict(_)) => {}
        other => panic!("expected Conflict for sold-out window, got {other:?}"),
    }

    // Party larger than the type sleeps ⇒ refuse even with rooms free
    // elsewhere (a direct create must not trust the availability filter).
    let mut oversized = hold_cmd(&format!("{BOOK_NO_PREFIX}-X2"), type_id, w2.0, w2.1);
    oversized.guests = 3;
    match svc.create_hold(oversized).await {
        Err(ServiceError::Conflict(_)) => {}
        other => panic!("expected Conflict for oversized party, got {other:?}"),
    }

    // Free the seeded conflicts for the rest of the scenario.
    sqlx::query("DELETE FROM ht_checkins WHERE cin_notes = 'TEST_loyalty_channel'")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM ht_bookings WHERE book_id = $1")
        .bind(conflict_book_id)
        .execute(&pool)
        .await
        .unwrap();

    // ---------- hold create ----------

    let before = Utc::now();
    let hold = svc
        .create_hold(hold_cmd(&format!("{BOOK_NO_PREFIX}-H1"), type_id, w1.0, w1.1))
        .await
        .expect("create hold");

    // Money: 2 nights × 1200 = 2400 total; deposit50 → 1200 due now.
    assert_eq!(hold.total_baht, 2400.0);
    assert_eq!(hold.amount_due_baht, 1200.0);
    // Deadline: now + 2h (generous tolerance for slow CI).
    let ttl = hold.hold_expires_at - before;
    assert!(
        ttl > Duration::minutes(115) && ttl <= Duration::minutes(125),
        "hold_expires_at must be ~2h out, got {ttl}"
    );

    // Canonical row: pending / loyalty / deadline stamped / room assigned.
    let row = sqlx::query(
        "SELECT b.book_status, b.book_channel, b.book_hold_expires_at, b.book_source, \
                b.book_total_amount::float8 AS total, \
                (SELECT COUNT(*) FROM ht_booking_rooms br WHERE br.br_book_id = b.book_id) AS rooms \
           FROM ht_bookings b WHERE b.book_id = $1",
    )
    .bind(hold.book_id)
    .fetch_one(&pool)
    .await
    .expect("hold row");
    assert_eq!(row.get::<String, _>("book_status"), "pending");
    assert_eq!(row.get::<String, _>("book_channel"), "loyalty");
    assert_eq!(row.get::<String, _>("book_source"), "loyalty");
    assert!(row.get::<Option<chrono::DateTime<Utc>>, _>("book_hold_expires_at").is_some());
    assert_eq!(row.get::<f64, _>("total"), 2400.0);
    assert_eq!(row.get::<i64, _>("rooms"), 1, "hold consumes exactly one room");

    // The hold consumed availability.
    let rows = svc.availability(d(w1.0), d(w1.1), 2).await.expect("availability");
    assert_eq!(
        count_for(&rows, type_id).unwrap().available_count,
        1,
        "pending hold must consume availability"
    );

    // Dual-write invariant: the hold enqueued its CreateBooking writeback.
    let jobs: i64 = sqlx::query(
        "SELECT COUNT(*) AS n FROM writeback_jobs w \
          WHERE w.intent = 'create_booking' \
            AND w.aggregate_id = (SELECT aggregate_id FROM ht_bookings WHERE book_id = $1)",
    )
    .bind(hold.book_id)
    .fetch_one(&pool)
    .await
    .expect("writeback count")
    .get("n");
    assert_eq!(jobs, 1, "roomed pending hold rides the normal create writeback");

    // Guest was created + membership attached.
    let guest = sqlx::query(
        "SELECT c.cust_id, c.cust_membership_id, c.cust_phone \
           FROM ht_customers c JOIN ht_bookings b ON b.book_cust_id = c.cust_id \
          WHERE b.book_id = $1",
    )
    .bind(hold.book_id)
    .fetch_one(&pool)
    .await
    .expect("guest row");
    assert_eq!(guest.get::<String, _>("cust_membership_id"), "TEST-LOYAL-M1");
    let first_guest_id: i32 = guest.get("cust_id");

    // ---------- match-or-create: same phone+name reuses the profile ----------

    let hold2 = svc
        .create_hold(hold_cmd(&format!("{BOOK_NO_PREFIX}-H2"), type_id, w2.0, w2.1))
        .await
        .expect("second hold");
    let second_guest_id: i32 = sqlx::query(
        "SELECT book_cust_id FROM ht_bookings WHERE book_id = $1",
    )
    .bind(hold2.book_id)
    .fetch_one(&pool)
    .await
    .expect("second hold row")
    .get("book_cust_id");
    assert_eq!(
        second_guest_id, first_guest_id,
        "same phone+name must match the existing profile, not mint a duplicate"
    );

    // ---------- payment-verified: confirm + idempotent replay ----------

    let confirm = svc
        .confirm_payment(hold.book_id, hold.amount_due_baht)
        .await
        .expect("confirm");
    assert!(!confirm.already_confirmed);
    assert_eq!(confirm.deposit_baht, 1200.0);
    assert_eq!(confirm.balance_due_baht, 1200.0);

    let status: String =
        sqlx::query("SELECT book_status FROM ht_bookings WHERE book_id = $1")
            .bind(hold.book_id)
            .fetch_one(&pool)
            .await
            .unwrap()
            .get("book_status");
    assert_eq!(status, "confirmed");

    // Replay tolerates + reports the stored numbers, writes nothing.
    let replay = svc
        .confirm_payment(hold.book_id, hold.amount_due_baht)
        .await
        .expect("confirm replay");
    assert!(replay.already_confirmed);
    assert_eq!(replay.deposit_baht, 1200.0);

    // A confirmed booking refuses release (guard on 'pending').
    match svc.release(hold.book_id, "test release").await {
        Err(ServiceError::Conflict(_)) => {}
        other => panic!("release of confirmed booking must conflict, got {other:?}"),
    }

    // ---------- release: cancel + idempotent replay ----------

    let release = svc.release(hold2.book_id, "test release").await.expect("release");
    assert!(!release.already_released);
    let status: String =
        sqlx::query("SELECT book_status FROM ht_bookings WHERE book_id = $1")
            .bind(hold2.book_id)
            .fetch_one(&pool)
            .await
            .unwrap()
            .get("book_status");
    assert_eq!(status, "cancelled");

    // Cancel writeback enqueued so iHOTEL frees the room.
    let cancel_jobs: i64 = sqlx::query(
        "SELECT COUNT(*) AS n FROM writeback_jobs w \
          WHERE w.intent = 'cancel_booking' \
            AND w.aggregate_id = (SELECT aggregate_id FROM ht_bookings WHERE book_id = $1)",
    )
    .bind(hold2.book_id)
    .fetch_one(&pool)
    .await
    .unwrap()
    .get("n");
    assert_eq!(cancel_jobs, 1, "release rides the normal cancel writeback");

    let replay = svc.release(hold2.book_id, "test release").await.expect("release replay");
    assert!(replay.already_released, "release must tolerate replays");

    // Confirming a released hold refuses.
    match svc.confirm_payment(hold2.book_id, 600.0).await {
        Err(ServiceError::Conflict(_)) => {}
        other => panic!("confirm of released hold must conflict, got {other:?}"),
    }

    // ---------- expiry sweep ----------

    let hold3 = svc
        .create_hold(hold_cmd(&format!("{BOOK_NO_PREFIX}-H3"), type_id, w3.0, w3.1))
        .await
        .expect("third hold");
    // Not expired yet → sweep must NOT touch it.
    let released = svc.sweep_expired_holds("test").await;
    let status: String =
        sqlx::query("SELECT book_status FROM ht_bookings WHERE book_id = $1")
            .bind(hold3.book_id)
            .fetch_one(&pool)
            .await
            .unwrap()
            .get("book_status");
    assert_eq!(status, "pending", "unexpired hold must survive the sweep (released={released})");

    // Force the deadline into the past → sweep releases it.
    sqlx::query(
        "UPDATE ht_bookings SET book_hold_expires_at = NOW() - INTERVAL '1 minute' WHERE book_id = $1",
    )
    .bind(hold3.book_id)
    .execute(&pool)
    .await
    .unwrap();
    let released = svc.sweep_expired_holds("test").await;
    assert!(released >= 1, "sweep must release the expired hold");
    let status: String =
        sqlx::query("SELECT book_status FROM ht_bookings WHERE book_id = $1")
            .bind(hold3.book_id)
            .fetch_one(&pool)
            .await
            .unwrap()
            .get("book_status");
    assert_eq!(status, "cancelled", "expired hold must be auto-released");

    // The confirmed booking (hold 1) is never touched by the sweep.
    let status: String =
        sqlx::query("SELECT book_status FROM ht_bookings WHERE book_id = $1")
            .bind(hold.book_id)
            .fetch_one(&pool)
            .await
            .unwrap()
            .get("book_status");
    assert_eq!(status, "confirmed");

    cleanup(&pool).await;
}

/// Membership set/clear repository behavior (backs
/// `PUT /api/customers/{id}/membership` and the channel attach).
#[tokio::test]
async fn membership_set_and_clear() {
    let pool = common::create_test_pool().await;
    // Own marker so the mega-test's cleanup can't race this one.
    sqlx::query("DELETE FROM ht_customers WHERE cust_firstname = 'TEST_loyalty_membership'")
        .execute(&pool)
        .await
        .ok();

    let cust_id: i32 = sqlx::query(
        "INSERT INTO ht_customers (cust_firstname, cust_phone) \
         VALUES ('TEST_loyalty_membership', '0812340000') RETURNING cust_id",
    )
    .fetch_one(&pool)
    .await
    .expect("seed customer")
    .get("cust_id");

    let repo = PgCustomerRepository::new();

    // Set.
    let mut tx = pool.begin().await.unwrap();
    let n = repo
        .set_membership(&mut tx, cust_id, Some("TEST-LOYAL-M2"))
        .await
        .expect("set membership");
    tx.commit().await.unwrap();
    assert_eq!(n, 1);
    let row = repo.get(&pool, cust_id).await.unwrap().unwrap();
    assert_eq!(row.cust_membership_id.as_deref(), Some("TEST-LOYAL-M2"));

    // Clear.
    let mut tx = pool.begin().await.unwrap();
    let n = repo.set_membership(&mut tx, cust_id, None).await.expect("clear membership");
    tx.commit().await.unwrap();
    assert_eq!(n, 1);
    let row = repo.get(&pool, cust_id).await.unwrap().unwrap();
    assert_eq!(row.cust_membership_id, None);

    // Unknown customer → 0 rows (route maps to 404).
    let mut tx = pool.begin().await.unwrap();
    let n = repo.set_membership(&mut tx, -1, Some("X")).await.expect("no-op");
    tx.commit().await.unwrap();
    assert_eq!(n, 0);

    // event_log rows are only written by the service layer (not exercised
    // here), so cleanup is just the fixture row.
    sqlx::query("DELETE FROM ht_customers WHERE cust_id = $1")
        .bind(cust_id)
        .execute(&pool)
        .await
        .ok();
}
