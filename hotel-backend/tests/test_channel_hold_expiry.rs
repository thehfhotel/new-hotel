//! Direct-booking program B13 — the TYPED hold-expiry marker.
//!
//! B13 is "measure the expired-hold rate, then take an explicit `HOLD_TTL`
//! decision". Migration 096 added `ht_bookings.book_hold_auto_released_at` to
//! make that measurable, because until then a swept hold landed in exactly the
//! terminal shape every other cancellation lands in and could only be
//! identified by matching free-text `book_cancel_reason` — a match that is
//! WRONG today. TWO paths write a hold cancellation: the sweeper's
//! `loyalty hold expired (auto-release)` and the channel release endpoint's
//! `loyalty payment window lapsed (channel release)`. Only the sweeper path is
//! a TTL expiry; the endpoint is a guest abandonment and must never be counted
//! as one.
//!
//! So the assertions here are about PROVENANCE, not about cancellation:
//!
//! 1. the expiry sweep stamps the marker, and stamps it EXACTLY ONCE (a second
//!    sweep must not re-stamp — the value has to stay the instant the hold
//!    actually died, or a rate bucketed by it drifts);
//! 2. a release somebody ASKED for — the loyalty app's own endpoint path, and
//!    a plain desk cancellation — leaves it NULL;
//! 3. an unexpired hold and a confirmed booking are untouched;
//! 4. the D3 channel rollup counts it, and still reports it as `holdsExpired`.
//!
//! Kept in its own file rather than appended to `test_channel.rs` so the
//! B13 fixtures cannot collide with that suite's shared markers.
//!
//! Isolation follows `tests/common/mod.rs` discipline: EXACT-match fixture
//! markers (never `LIKE`), cleaned on entry AND exit. The rollup assertion
//! additionally uses a far-future window (`2098-*`) that no other suite
//! touches, because that report aggregates every booking in its range.

mod common;

use std::sync::Arc;

use chrono::{DateTime, Duration, NaiveDate, Utc};
use sqlx::{PgPool, Row};

use hotel_backend::outbox::event::EventSource;
use hotel_backend::outbox::{EventBus, OutboxRepository};
use hotel_backend::repository::{CustomerRepository, PgBookingRepository, PgCustomerRepository};
use hotel_backend::service::reports::channel_rollup::{load_channel_rollup, rollup, ChannelTotals};
use hotel_backend::service::{
    BookingService, CancelBookingCommand, ChannelService, CustomerService, ReleaseCause,
};
use uuid::Uuid;

/// These tests share fixture markers and the rollup window, so they must not
/// interleave.
static B13_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

const CUST_MARKER: &str = "TEST_b13_expiry_cust";
const BOOK_MARKER: &str = "TEST_b13_expiry_book";

/// Far-future window nothing else in the suite touches. The rollup attributes
/// by `book_checkin`, so every fixture below arrives inside it.
const WINDOW_FROM: (i32, u32, u32) = (2098, 5, 1);
const WINDOW_TO: (i32, u32, u32) = (2098, 5, 31);
const ARRIVAL: (i32, u32, u32) = (2098, 5, 10);

fn date(d: (i32, u32, u32)) -> NaiveDate {
    NaiveDate::from_ymd_opt(d.0, d.1, d.2).expect("valid fixture date")
}

/// The channel service plus the `BookingService` behind it, so a test can
/// exercise the DESK cancel path (`BookingService::cancel`) as well as the
/// channel's own release — they are different repository writes, and only one
/// of them is allowed to stamp the marker.
fn service_for(pool: &PgPool) -> (ChannelService, Arc<BookingService>) {
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
    // Floor 0 — these scenarios are about the marker, not the last-room guard.
    (
        ChannelService::new(pool.clone(), bookings.clone(), customers, customers_repo, 0),
        bookings,
    )
}

async fn cleanup(pool: &PgPool) {
    sqlx::query(
        "DELETE FROM ht_booking_rooms WHERE br_book_id IN \
         (SELECT book_id FROM ht_bookings WHERE book_notes = $1)",
    )
    .bind(BOOK_MARKER)
    .execute(pool)
    .await
    .ok();
    sqlx::query("DELETE FROM ht_bookings WHERE book_notes = $1")
        .bind(BOOK_MARKER)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_customers WHERE cust_notes = $1")
        .bind(CUST_MARKER)
        .execute(pool)
        .await
        .ok();
}

async fn seed_customer(pool: &PgPool) -> i32 {
    sqlx::query_scalar::<_, i32>(
        "INSERT INTO ht_customers (cust_firstname, cust_lastname, cust_notes) \
         VALUES ('B13', 'Expiry', $1) RETURNING cust_id",
    )
    .bind(CUST_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed_customer failed")
}

/// Seed a booking directly. `hold_expires_in` drives `book_hold_expires_at`:
/// a negative duration makes the hold already overdue, which is what the sweep
/// looks for.
#[allow(clippy::too_many_arguments)]
async fn seed_booking(
    pool: &PgPool,
    book_no: &str,
    cust_id: i32,
    channel: Option<&str>,
    status: &str,
    hold_expires_in: Option<Duration>,
) -> i32 {
    let checkin = date(ARRIVAL);
    sqlx::query_scalar::<_, i32>(
        "INSERT INTO ht_bookings \
            (book_no, book_cust_id, book_checkin, book_checkout, book_channel, \
             book_source, book_status, book_total_amount, book_hold_expires_at, book_notes) \
         VALUES ($1, $2, $3, $4, $5, 'loyalty', $6, 1000::float8, $7, $8) RETURNING book_id",
    )
    .bind(book_no)
    .bind(cust_id)
    .bind(checkin)
    .bind(checkin + Duration::days(2))
    .bind(channel)
    .bind(status)
    .bind(hold_expires_in.map(|d| Utc::now() + d))
    .bind(BOOK_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed_booking failed")
}

async fn marker_of(pool: &PgPool, book_id: i32) -> Option<DateTime<Utc>> {
    sqlx::query("SELECT book_hold_auto_released_at FROM ht_bookings WHERE book_id = $1")
        .bind(book_id)
        .fetch_one(pool)
        .await
        .expect("read book_hold_auto_released_at")
        .get::<Option<DateTime<Utc>>, _>("book_hold_auto_released_at")
}

async fn status_of(pool: &PgPool, book_id: i32) -> String {
    sqlx::query("SELECT book_status FROM ht_bookings WHERE book_id = $1")
        .bind(book_id)
        .fetch_one(pool)
        .await
        .expect("read book_status")
        .get::<String, _>("book_status")
}

// ---------------------------------------------------------------------------
// 1. The sweep stamps the marker — exactly once, and only on a real expiry
// ---------------------------------------------------------------------------

#[tokio::test]
async fn sweep_stamps_the_marker_once_and_only_on_expiry() {
    let _guard = B13_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup(&pool).await;
    let (svc, _bookings) = service_for(&pool);
    let cust = seed_customer(&pool).await;

    // An overdue loyalty hold, and one whose window is still open.
    let overdue = seed_booking(
        &pool,
        "TESTB13-EXP",
        cust,
        Some("loyalty"),
        "pending",
        Some(Duration::minutes(-1)),
    )
    .await;
    let live = seed_booking(
        &pool,
        "TESTB13-LIVE",
        cust,
        Some("loyalty"),
        "pending",
        Some(Duration::hours(2)),
    )
    .await;
    // A PAID booking that kept its (now past) deadline — `confirm_booking_payment`
    // deliberately does not clear `book_hold_expires_at`, so this is the row
    // that would be mislabelled by reading the deadline instead of the marker.
    let confirmed = seed_booking(
        &pool,
        "TESTB13-PAID",
        cust,
        Some("loyalty"),
        "confirmed",
        Some(Duration::minutes(-1)),
    )
    .await;

    assert_eq!(marker_of(&pool, overdue).await, None, "marker starts NULL");

    svc.sweep_expired_holds("test").await;

    let first = marker_of(&pool, overdue)
        .await
        .expect("the sweep must stamp book_hold_auto_released_at on an expired hold");
    assert_eq!(
        status_of(&pool, overdue).await,
        "cancelled",
        "the marker rides the cancellation, not a separate write"
    );

    // Only the expired hold. A live hold and a paid booking are untouched —
    // the paid one proves the marker is not derived from the stale deadline.
    assert_eq!(marker_of(&pool, live).await, None, "live hold not stamped");
    assert_eq!(status_of(&pool, live).await, "pending");
    assert_eq!(
        marker_of(&pool, confirmed).await,
        None,
        "a PAID booking past its old deadline must never be marked expired"
    );
    assert_eq!(status_of(&pool, confirmed).await, "confirmed");

    // Exactly once: the sweep is guarded on `book_status='pending'`, so a
    // second tick writes nothing and the instant stays put. If this ever
    // re-stamped, a rate bucketed on the marker would drift every tick.
    svc.sweep_expired_holds("test").await;
    assert_eq!(
        marker_of(&pool, overdue).await,
        Some(first),
        "a second sweep must not re-stamp the marker"
    );

    cleanup(&pool).await;
}

// ---------------------------------------------------------------------------
// 2. A release somebody ASKED for leaves the marker NULL
// ---------------------------------------------------------------------------

#[tokio::test]
async fn requested_release_and_manual_cancel_leave_the_marker_null() {
    let _guard = B13_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup(&pool).await;
    let (svc, bookings) = service_for(&pool);
    let cust = seed_customer(&pool).await;

    // The path `routes::channel::release` takes. Note the hold is ALSO overdue:
    // the marker must follow the CAUSE, not the clock, or this test passes for
    // the wrong reason.
    let asked = seed_booking(
        &pool,
        "TESTB13-REL",
        cust,
        Some("loyalty"),
        "pending",
        Some(Duration::minutes(-1)),
    )
    .await;
    svc.release(asked, "loyalty payment window lapsed (channel release)")
        .await
        .expect("requested release must succeed");

    assert_eq!(
        status_of(&pool, asked).await,
        "cancelled",
        "a requested release still cancels"
    );
    assert_eq!(
        marker_of(&pool, asked).await,
        None,
        "a release the app asked for is NOT a measured expiry"
    );

    // The same guarantee stated against the enum directly, so a future caller
    // that reaches for `release_with_cause` cannot quietly opt into the marker.
    let asked2 = seed_booking(
        &pool,
        "TESTB13-REL2",
        cust,
        Some("loyalty"),
        "pending",
        Some(Duration::minutes(-1)),
    )
    .await;
    svc.release_with_cause(asked2, "guest abandoned checkout", ReleaseCause::Requested)
        .await
        .expect("explicit Requested release must succeed");
    assert_eq!(marker_of(&pool, asked2).await, None);
    assert!(!ReleaseCause::Requested.is_auto_release());
    assert!(ReleaseCause::PaymentWindowExpired.is_auto_release());

    // A genuine DESK cancellation — `BookingService::cancel`, a different
    // repository write that never mentions the marker column at all. Asserted
    // rather than assumed: if someone later folds the stamp into the generic
    // cancel path, every desk cancellation of a lapsed hold would start
    // counting as a TTL expiry and this is what catches it.
    let desk = seed_booking(
        &pool,
        "TESTB13-DESK",
        cust,
        Some("loyalty"),
        "pending",
        Some(Duration::minutes(-1)),
    )
    .await;
    bookings
        .cancel(CancelBookingCommand {
            book_id: desk,
            reason: Some("desk cancelled the hold".to_string()),
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
        })
        .await
        .expect("desk cancel must succeed");
    assert_eq!(
        status_of(&pool, desk).await,
        "cancelled",
        "the desk cancel path still cancels"
    );
    assert_eq!(
        marker_of(&pool, desk).await,
        None,
        "a desk cancellation is not a TTL expiry"
    );

    cleanup(&pool).await;
}

// ---------------------------------------------------------------------------
// 3. The metric keeps its business name on the wire
// ---------------------------------------------------------------------------

/// The column and the Rust field are named for the MECHANISM
/// (`..._auto_released`) so they cannot be typo-confused with
/// `book_hold_expires_at`; the wire field keeps the BUSINESS name
/// (`holdsExpired`) because that is the question the number answers and what
/// loyalty-app's friction card reads. An explicit `serde(rename)` pins them
/// together — this test is what stops a future rename from silently breaking
/// that consumer, since nothing else would fail to compile.
#[test]
fn rollup_metric_is_holds_expired_on_the_wire() {
    let json = serde_json::to_string(&ChannelTotals::default()).expect("serialize totals");
    assert!(
        json.contains("\"holdsExpired\""),
        "the wire name must stay holdsExpired: {json}"
    );
    assert!(
        !json.contains("holdsAutoReleased"),
        "the mechanism name must not leak onto the wire: {json}"
    );
}

// ---------------------------------------------------------------------------
// 4. The D3 rollup counts it
// ---------------------------------------------------------------------------

#[tokio::test]
async fn channel_rollup_counts_holds_expired_as_a_subset_of_cancelled() {
    let _guard = B13_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup(&pool).await;
    let (svc, _bookings) = service_for(&pool);
    let cust = seed_customer(&pool).await;

    // Two overdue holds that the sweep will kill, one hold the app releases
    // itself, and one confirmed booking that stays sold.
    for no in ["TESTB13-R1", "TESTB13-R2"] {
        seed_booking(
            &pool,
            no,
            cust,
            Some("loyalty"),
            "pending",
            Some(Duration::minutes(-1)),
        )
        .await;
    }
    let released = seed_booking(
        &pool,
        "TESTB13-R3",
        cust,
        Some("loyalty"),
        "pending",
        Some(Duration::minutes(-1)),
    )
    .await;
    seed_booking(
        &pool,
        "TESTB13-R4",
        cust,
        Some("loyalty"),
        "confirmed",
        None,
    )
    .await;

    // A hold that will be swept and then REVIVED from the legacy side.
    let revived = seed_booking(
        &pool,
        "TESTB13-R5",
        cust,
        Some("loyalty"),
        "pending",
        Some(Duration::minutes(-1)),
    )
    .await;

    svc.release(released, "guest abandoned checkout")
        .await
        .expect("requested release");
    svc.sweep_expired_holds("test").await;

    // iHOTEL reinstates the lapsed hold: `sync::mappers::booking` writes
    // book_status unconditionally and legacy `จอง` maps back to 'confirmed',
    // so the row keeps its marker but is no longer cancelled. The room is sold
    // again — it must NOT be counted as a hold we lost to the clock, and it
    // must not be able to push holdsExpired above cancelled.
    sqlx::query("UPDATE ht_bookings SET book_status = 'confirmed' WHERE book_id = $1")
        .bind(revived)
        .execute(&pool)
        .await
        .expect("simulate a CT-driven revive");
    assert!(
        marker_of(&pool, revived).await.is_some(),
        "the revive leaves the marker behind — that is exactly the hazard"
    );

    let rows = load_channel_rollup(&pool, date(WINDOW_FROM), date(WINDOW_TO))
        .await
        .expect("load_channel_rollup failed");
    let out = rollup(&rows);
    let app = out
        .buckets
        .iter()
        .find(|b| b.key == "app")
        .expect("the loyalty bucket must be present");

    assert_eq!(app.totals.bookings, 5, "all five arrive in the window");
    assert_eq!(
        app.totals.cancelled, 3,
        "two swept + one requested release; the revived one is confirmed again"
    );
    assert_eq!(
        app.totals.holds_auto_released, 2,
        "only the two the clock killed AND left cancelled count as expired holds \
         — the stamped-then-revived row is excluded"
    );
    assert!(
        app.totals.holds_auto_released <= app.totals.cancelled,
        "holdsExpired must stay a subset of cancelled even after a legacy revive"
    );
    assert_eq!(
        out.totals.holds_auto_released, 2,
        "the window total sums the buckets"
    );

    cleanup(&pool).await;
}
