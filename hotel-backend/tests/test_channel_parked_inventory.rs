//! B8a/B8c — parked (roomless) bookings must consume loyalty-channel inventory,
//! and (since migration 094) consume it from the TYPE they claim.
//!
//! A **parked** booking is a live `ht_bookings` row with ZERO
//! `ht_booking_rooms` rows: an OTA or desk reservation whose room a human
//! assigns later (`service::booking::create`, the `cmd.rooms.is_empty()`
//! branch), and the shape the CT mapper persists for a header-only
//! `HT_Book_H`. `FREE_ROOM_PREDICATE` reaches bookings through
//! `ht_booking_rooms`, so before this change every physical room still looked
//! free while a parked booking already claimed one — the channel could hold
//! the property's last room.
//!
//! These tests exercise `repository::channel` directly (the counter, the
//! picker and the shared snapshot) against the real PostgreSQL schema.
//! `common` reads `DATABASE_URL`; CI provides a service container and runs
//! `--test-threads=1`.
//!
//! ## The two terms under test
//!
//! B8a could only cap property-wide, because a parked booking recorded no room
//! type anywhere in the canonical schema. Migration 094 adds
//! `ht_bookings.book_room_type_id`, so the rule is now
//!
//! ```text
//! available(type) = min( max(free(type) - parked_typed(type), 0), surplus )
//! surplus         = max(free rooms property-wide - ALL parked claims, 0)
//! ```
//!
//! Scenarios 6 and 7 are a matched pair isolating exactly that difference:
//! identical pressure, identical slack, and the only variable is whether the
//! parked claim names a type. A NULL-type claim must still behave exactly as
//! it did under #304 (scenario 6); a typed one must take its own type's last
//! room out of the channel's view and leave every other type alone
//! (scenario 7).
//!
//! ## Why the assertions are relative, not absolute
//!
//! "Free rooms" spans every room in the database, and a developer's machine
//! carries the real mirrored room list while CI carries only what tests seed
//! — so each scenario MEASURES the surplus via
//! `channel_repo::inventory_snapshot` and then drains it to a known value,
//! instead of assuming a room count. The per-type counts ARE absolute: the
//! two fixture types under test are unique to this file.
//!
//! ## Why the fixture seeds SLACK rooms
//!
//! A scenario that wants the PER-TYPE term to be what blocks a sale only
//! means something while `surplus >= 1` — otherwise the property-wide cap
//! zeroes every type on its own and the assertion passes no matter what the
//! per-type term does. A developer's machine has slack for free; CI seeds
//! ZERO rooms beyond this file's own, so the fixture has to supply it. Type C
//! (`ROOMS_SLACK`) exists only to keep the property from being oversubscribed
//! — nothing asserts on it, and no scenario reads its availability. Every
//! scenario that leans on slack states the premise as an explicit
//! `surplus >= 1` assertion, so removing those rooms fails the guard instead
//! of silently making the test unfalsifiable.
//!
//! ## Why one test function
//!
//! Same reason as `tests/test_channel.rs`: the fixtures are shared, separate
//! `#[tokio::test]` fns would race on them under a default (parallel) local
//! `cargo test`, and a single body guarantees cleanup ordering. Each
//! scenario below owns a DISJOINT far-future stay window, so the bookings one
//! scenario seeds are invisible to every other.

mod common;

use chrono::NaiveDate;
use hotel_backend::repository::channel as channel_repo;
use sqlx::{PgPool, Row};

/// Unique-to-this-file fixture markers (see tests/common/mod.rs cleanup rules).
const TYPE_CODE_A: &str = "TSTPKA";
const TYPE_NAME_A: &str = "TEST_parked_type_a";
const TYPE_CODE_B: &str = "TSTPKB";
const TYPE_NAME_B: &str = "TEST_parked_type_b";
const ROOM_A1: &str = "TPK01";
const ROOM_A2: &str = "TPK02";
const ROOM_B1: &str = "TPK03";
/// Type C is never asserted on. Its rooms exist so the property keeps slack
/// (`surplus >= 1`) in the scenarios whose whole point is the PER-TYPE term —
/// on CI, where the only rooms in the database are the ones this file seeds.
const TYPE_CODE_C: &str = "TSTPKC";
const TYPE_NAME_C: &str = "TEST_parked_type_c";
const ROOMS_SLACK: [&str; 3] = ["TPK04", "TPK05", "TPK06"];
const GUEST_FIRST: &str = "TEST_parked_guest";
const BOOK_NO_PREFIX: &str = "TESTPK";

/// Every seeded type sleeps 2, so every query below asks for 2 guests.
const GUESTS: i32 = 2;

fn d(s: &str) -> NaiveDate {
    NaiveDate::parse_from_str(s, "%Y-%m-%d").expect("test date")
}

// ── fixtures ────────────────────────────────────────────────────────────────

async fn seed_type(pool: &PgPool, code: &str, name: &str) -> i32 {
    sqlx::query(
        "INSERT INTO ht_room_types (type_code, type_name, type_description, type_base_price, type_max_guests, type_active) \
         VALUES ($1, $2, 'parked-inventory test type', 1000.00, 2, true) RETURNING type_id",
    )
    .bind(code)
    .bind(name)
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

/// Insert `count` PARKED bookings — `ht_bookings` rows with NO
/// `ht_booking_rooms` child, exactly the shape `service::booking::create`
/// persists for a roomless OTA/desk reservation. One statement regardless of
/// `count` so draining a developer machine's full room list stays cheap.
async fn seed_parked(
    pool: &PgPool,
    cust_id: i32,
    check_in: NaiveDate,
    check_out: NaiveDate,
    status: &str,
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
         SELECT $1 || lpad(g::text, 6, '0'), $2, $3::date, $4::date, $5, 'bookingcom' \
           FROM generate_series($6::bigint, $7::bigint) AS g",
    )
    .bind(BOOK_NO_PREFIX)
    .bind(cust_id)
    .bind(check_in)
    .bind(check_out)
    .bind(status)
    .bind(from)
    .bind(to)
    .execute(pool)
    .await
    .expect("seed parked bookings");
}

/// Insert `count` PARKED bookings that DO name a room type (B8c / migration
/// 094) — `book_room_type_id` set, still zero `ht_booking_rooms` rows. This is
/// what a roomless OTA/desk reservation with a declared `roomTypeId` looks
/// like, and what the CT mapper persists for an iHOTEL "ระบุประเภทห้อง"
/// booking whose `HT_Book_Ds.Book_Room_Type` code resolved.
#[allow(clippy::too_many_arguments)]
async fn seed_parked_typed(
    pool: &PgPool,
    cust_id: i32,
    type_id: i32,
    check_in: NaiveDate,
    check_out: NaiveDate,
    status: &str,
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
        "INSERT INTO ht_bookings (book_no, book_cust_id, book_checkin, book_checkout, book_status, book_channel, book_room_type_id) \
         SELECT $1 || lpad(g::text, 6, '0'), $2, $3::date, $4::date, $5, 'bookingcom', $8 \
           FROM generate_series($6::bigint, $7::bigint) AS g",
    )
    .bind(BOOK_NO_PREFIX)
    .bind(cust_id)
    .bind(check_in)
    .bind(check_out)
    .bind(status)
    .bind(from)
    .bind(to)
    .bind(type_id)
    .execute(pool)
    .await
    .expect("seed typed parked bookings");
}

/// Insert one ASSIGNED live booking occupying `room_id` — the shape the
/// existing `FREE_ROOM_PREDICATE` already sees.
async fn seed_assigned(
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
    .expect("seed assigned booking")
    .get("book_id");

    sqlx::query("INSERT INTO ht_booking_rooms (br_book_id, br_room_id) VALUES ($1, $2)")
        .bind(book_id)
        .bind(room_id)
        .execute(pool)
        .await
        .expect("assign room");
}

// ── read helpers (the two surfaces under test) ──────────────────────────────

async fn snapshot(
    pool: &PgPool,
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> channel_repo::InventorySnapshot {
    channel_repo::inventory_snapshot(pool, check_in, check_out)
        .await
        .expect("inventory snapshot")
}

async fn counted(pool: &PgPool, type_id: i32, check_in: NaiveDate, check_out: NaiveDate) -> i64 {
    channel_repo::availability_by_type(pool, check_in, check_out, GUESTS)
        .await
        .expect("availability_by_type")
        .iter()
        .find(|r| r.type_id == type_id)
        .unwrap_or_else(|| panic!("type {type_id} missing from availability"))
        .available_count
}

async fn picked(
    pool: &PgPool,
    type_id: i32,
    check_in: NaiveDate,
    check_out: NaiveDate,
) -> Option<channel_repo::PickedRoom> {
    channel_repo::pick_free_room(pool, type_id, check_in, check_out, GUESTS)
        .await
        .expect("pick_free_room")
}

/// The invariant the whole change rests on: the counter and the picker read
/// one shared definition, so they must never disagree about sold-out.
async fn assert_counter_and_picker_agree(
    pool: &PgPool,
    type_id: i32,
    check_in: NaiveDate,
    check_out: NaiveDate,
    label: &str,
) -> i64 {
    let count = counted(pool, type_id, check_in, check_out).await;
    let pick = picked(pool, type_id, check_in, check_out).await;
    assert_eq!(
        count > 0,
        pick.is_some(),
        "{label}: counter says {count} but picker returned {pick:?}"
    );
    count
}

/// Seed parked bookings until the window's surplus is exactly `target`.
async fn drain_surplus_to(
    pool: &PgPool,
    cust_id: i32,
    check_in: NaiveDate,
    check_out: NaiveDate,
    seq: &mut i64,
    target: i64,
) {
    let before = snapshot(pool, check_in, check_out).await;
    assert!(
        before.surplus >= target,
        "cannot drain surplus {} down to {target}",
        before.surplus
    );
    seed_parked(
        pool,
        cust_id,
        check_in,
        check_out,
        "confirmed",
        seq,
        before.surplus - target,
    )
    .await;
    let after = snapshot(pool, check_in, check_out).await;
    assert_eq!(after.surplus, target, "drain landed wrong: {after:?}");
}

// ── cleanup ─────────────────────────────────────────────────────────────────

/// Delete every row this file created, children first. Exact-match markers
/// (wildcards were the `customer_search_by_name` flake — see tests/common).
/// `ht_booking_rooms` cascades on `br_book_id`.
async fn cleanup(pool: &PgPool) {
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
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_no IN ($1, $2, $3, $4, $5, $6)")
        .bind(ROOM_A1)
        .bind(ROOM_A2)
        .bind(ROOM_B1)
        .bind(ROOMS_SLACK[0])
        .bind(ROOMS_SLACK[1])
        .bind(ROOMS_SLACK[2])
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_room_types WHERE type_code IN ($1, $2, $3)")
        .bind(TYPE_CODE_A)
        .bind(TYPE_CODE_B)
        .bind(TYPE_CODE_C)
        .execute(pool)
        .await
        .ok();
}

// ── the test ────────────────────────────────────────────────────────────────

#[tokio::test]
async fn parked_bookings_consume_channel_inventory() {
    let pool = common::create_test_pool().await;
    // Pre-clean in case a previous run aborted mid-test.
    cleanup(&pool).await;

    let type_a = seed_type(&pool, TYPE_CODE_A, TYPE_NAME_A).await;
    let type_b = seed_type(&pool, TYPE_CODE_B, TYPE_NAME_B).await;
    let room_a1 = seed_room(&pool, ROOM_A1, type_a).await;
    let room_a2 = seed_room(&pool, ROOM_A2, type_a).await;
    let _room_b1 = seed_room(&pool, ROOM_B1, type_b).await;
    // Slack, not subject matter: three free rooms of a type no assertion
    // names, so the property-wide cap is never what zeroes type A or type B.
    let type_c = seed_type(&pool, TYPE_CODE_C, TYPE_NAME_C).await;
    for room_no in ROOMS_SLACK {
        seed_room(&pool, room_no, type_c).await;
    }
    let cust = seed_customer(&pool).await;

    // Monotonic book_no suffix — book_no is UNIQUE across the whole table.
    let mut seq: i64 = 1;

    // Stay windows far enough out that no live/mirrored data can overlap, and
    // DISJOINT from each other so scenarios never see each other's bookings.
    // (test_channel.rs owns 2126-*; this file owns 2127-*.)
    let w1 = (d("2127-03-01"), d("2127-03-05")); // same window blocks last room
    let w2 = (d("2127-05-01"), d("2127-05-05")); // non-overlapping parked
    let w2_after = (d("2127-05-05"), d("2127-05-07")); // half-open boundary
    let w2_far = (d("2127-06-01"), d("2127-06-05")); // plainly disjoint
    let w3 = (d("2127-07-01"), d("2127-07-05")); // cancelled parked
    let w4 = (d("2127-09-01"), d("2127-09-05")); // surplus absorbs the claim
    let w5 = (d("2127-11-01"), d("2127-11-05")); // counter/picker agreement
    let w6 = (d("2128-01-01"), d("2128-01-05")); // B8c: NULL-type claim, slack
    let w7 = (d("2128-03-01"), d("2128-03-05")); // B8c: typed claim, same slack
    let w8 = (d("2128-05-01"), d("2128-05-05")); // B8c: two types + one untyped

    // ── baseline: nothing parked, both types fully available ────────────────

    let base = snapshot(&pool, w1.0, w1.1).await;
    assert_eq!(base.parked_claims, 0, "clean window has no parked claims");
    assert_eq!(
        base.surplus, base.free_rooms,
        "no parked claims ⇒ surplus == free rooms"
    );
    assert_eq!(counted(&pool, type_a, w1.0, w1.1).await, 2, "A1 + A2 free");
    assert_eq!(counted(&pool, type_b, w1.0, w1.1).await, 1, "B1 free");

    // ── 1. a parked booking on OVERLAPPING dates blocks the last room ───────
    //
    // Occupy A2 so type A has exactly ONE free room, then bring the property
    // surplus down to exactly 1 — the channel may still sell that last room.
    // ONE more parked booking, and it may not.

    seed_assigned(&pool, cust, room_a2, w1.0, w1.1, &mut seq).await;
    assert_eq!(
        counted(&pool, type_a, w1.0, w1.1).await,
        1,
        "assigned booking on A2 leaves one free room of type A"
    );

    drain_surplus_to(&pool, cust, w1.0, w1.1, &mut seq, 1).await;
    assert_eq!(
        counted(&pool, type_a, w1.0, w1.1).await,
        1,
        "surplus 1: the last room of type A is still sellable"
    );
    let pick = picked(&pool, type_a, w1.0, w1.1).await.expect("picks A1");
    assert_eq!(pick.room_no, ROOM_A1);

    seed_parked(&pool, cust, w1.0, w1.1, "confirmed", &mut seq, 1).await;

    let after = snapshot(&pool, w1.0, w1.1).await;
    assert_eq!(
        after.surplus, 0,
        "one more parked claim exhausts the property"
    );
    assert_eq!(
        counted(&pool, type_a, w1.0, w1.1).await,
        0,
        "parked booking on overlapping dates blocks the last room of the type"
    );
    assert!(
        picked(&pool, type_a, w1.0, w1.1).await.is_none(),
        "picker must refuse A1 even though no booking names it"
    );

    // ── 2. a parked booking on NON-OVERLAPPING dates does not block ─────────
    //
    // Same half-open `[check_in, check_out)` rule the free-room predicate
    // uses: a parked stay that starts on our checkout day does not overlap.

    seed_assigned(&pool, cust, room_a2, w2.0, w2.1, &mut seq).await;
    drain_surplus_to(&pool, cust, w2.0, w2.1, &mut seq, 1).await;
    let before = snapshot(&pool, w2.0, w2.1).await;

    seed_parked(&pool, cust, w2_far.0, w2_far.1, "confirmed", &mut seq, 5).await;
    seed_parked(
        &pool,
        cust,
        w2_after.0,
        w2_after.1,
        "confirmed",
        &mut seq,
        5,
    )
    .await;

    assert_eq!(
        snapshot(&pool, w2.0, w2.1).await,
        before,
        "parked bookings outside [check_in, check_out) must not move inventory \
         (including one starting exactly on the checkout day)"
    );
    assert_eq!(
        counted(&pool, type_a, w2.0, w2.1).await,
        1,
        "non-overlapping parked bookings leave the last room sellable"
    );
    assert!(picked(&pool, type_a, w2.0, w2.1).await.is_some());

    // ── 3. a CANCELLED parked booking does not block ────────────────────────
    //
    // Same status set the free-room predicate uses for "live"
    // (`live_booking_statuses!`), so a cancelled roomless booking is as
    // invisible as a cancelled roomed one.

    seed_assigned(&pool, cust, room_a2, w3.0, w3.1, &mut seq).await;
    drain_surplus_to(&pool, cust, w3.0, w3.1, &mut seq, 1).await;
    let before = snapshot(&pool, w3.0, w3.1).await;

    seed_parked(&pool, cust, w3.0, w3.1, "cancelled", &mut seq, 5).await;

    assert_eq!(
        snapshot(&pool, w3.0, w3.1).await,
        before,
        "cancelled parked bookings hold no inventory"
    );
    assert_eq!(counted(&pool, type_a, w3.0, w3.1).await, 1);
    assert!(picked(&pool, type_a, w3.0, w3.1).await.is_some());

    // ── 4. a parked claim the property's slack absorbs blocks nothing ───────
    //
    // The "parked booking of a different type" case, as far as the schema can
    // express it: a parked booking names no type, so what decides whether it
    // blocks type A is whether SOME other free room can absorb it. Here B1
    // (and every real free room) can — so no type loses a sale. This is the
    // false-sold-out failure mode `loyalty-app` ADR-0003 rejected allotments
    // over, and it must not reappear here.

    let before = snapshot(&pool, w4.0, w4.1).await;
    assert!(
        before.free_rooms >= 3,
        "fixture guarantees at least A1+A2+B1 free"
    );

    seed_parked(&pool, cust, w4.0, w4.1, "confirmed", &mut seq, 1).await;

    let after = snapshot(&pool, w4.0, w4.1).await;
    assert_eq!(after.parked_claims, 1);
    assert_eq!(after.surplus, before.free_rooms - 1, "one room spoken for");
    assert_eq!(
        counted(&pool, type_a, w4.0, w4.1).await,
        2,
        "both type-A rooms stay sellable while the property has slack"
    );
    assert_eq!(counted(&pool, type_b, w4.0, w4.1).await, 1);
    assert!(picked(&pool, type_a, w4.0, w4.1).await.is_some());
    assert!(picked(&pool, type_b, w4.0, w4.1).await.is_some());

    // ── 5. counter and picker agree, on both roads to zero ──────────────────
    //
    // A type reads sold out for two different reasons — its own rooms are
    // taken (LEAST's left arm) or the property is oversubscribed by parked
    // bookings (LEAST's right arm). Both must move the counter and the picker
    // together.

    // (a) type A's own rooms are gone; the property still has slack.
    seed_assigned(&pool, cust, room_a1, w5.0, w5.1, &mut seq).await;
    seed_assigned(&pool, cust, room_a2, w5.0, w5.1, &mut seq).await;
    assert!(snapshot(&pool, w5.0, w5.1).await.surplus > 0);
    assert_eq!(
        assert_counter_and_picker_agree(&pool, type_a, w5.0, w5.1, "A, own rooms taken").await,
        0
    );
    assert_eq!(
        assert_counter_and_picker_agree(&pool, type_b, w5.0, w5.1, "B, slack remains").await,
        1
    );

    // (b) the property is oversubscribed; type B loses its free room too.
    drain_surplus_to(&pool, cust, w5.0, w5.1, &mut seq, 0).await;
    assert_eq!(
        assert_counter_and_picker_agree(&pool, type_a, w5.0, w5.1, "A, surplus 0").await,
        0
    );
    assert_eq!(
        assert_counter_and_picker_agree(&pool, type_b, w5.0, w5.1, "B, surplus 0").await,
        0
    );

    // ── 6. B8c — a NULL-type parked claim still only CAPS property-wide ─────
    //
    // The control for scenario 7. Identical setup, identical pressure: type A
    // is down to its last room (A1) and ONE parked booking overlaps — but the
    // booking names no type, so nothing tells us it wants an A. While the
    // property still has slack somewhere, A's last room stays sellable. That
    // is the #304 rule, and migration 094 must leave it exactly as it was.

    seed_assigned(&pool, cust, room_a2, w6.0, w6.1, &mut seq).await;
    assert_eq!(
        counted(&pool, type_a, w6.0, w6.1).await,
        1,
        "A2 assigned ⇒ exactly one free room of type A"
    );

    seed_parked(&pool, cust, w6.0, w6.1, "confirmed", &mut seq, 1).await;

    let untyped = snapshot(&pool, w6.0, w6.1).await;
    assert_eq!(untyped.parked_claims, 1);
    assert_eq!(untyped.parked_claims_typed, 0, "the claim names no type");
    assert_eq!(untyped.parked_claims_untyped, 1);
    assert!(
        untyped.surplus >= 1,
        "the scenario only means something while the property has slack: {untyped:?}"
    );
    assert_eq!(
        counted(&pool, type_a, w6.0, w6.1).await,
        1,
        "a NULL-type claim must not single out a type — it can only cap the \
         property, which still has room"
    );
    assert!(picked(&pool, type_a, w6.0, w6.1).await.is_some());
    assert_eq!(counted(&pool, type_b, w6.0, w6.1).await, 1);

    // ── 7. B8c — a TYPED parked claim blocks the last room of THAT type ─────
    //
    // Same pressure as scenario 6, one fact added: the parked booking now
    // records `book_room_type_id = type_a`. The last type-A room must go
    // unsellable — and type B must NOT, which is the whole point (before B8c
    // both stayed sellable and the channel could hand out the A that the
    // parked booking was waiting for).

    seed_assigned(&pool, cust, room_a2, w7.0, w7.1, &mut seq).await;
    assert_eq!(counted(&pool, type_a, w7.0, w7.1).await, 1);
    assert_eq!(counted(&pool, type_b, w7.0, w7.1).await, 1);

    seed_parked_typed(&pool, cust, type_a, w7.0, w7.1, "confirmed", &mut seq, 1).await;

    let typed = snapshot(&pool, w7.0, w7.1).await;
    assert_eq!(typed.parked_claims, 1);
    assert_eq!(typed.parked_claims_typed, 1, "the claim names type A");
    assert_eq!(typed.parked_claims_untyped, 0);
    assert!(
        typed.surplus >= 1,
        "the property must still have slack, or the property-wide cap — not \
         the per-type term — would be what blocks A: {typed:?}"
    );

    assert_eq!(
        assert_counter_and_picker_agree(&pool, type_a, w7.0, w7.1, "A, typed claim").await,
        0,
        "a parked claim ON TYPE A must take A's last room out of the channel's \
         view even though the property has slack elsewhere"
    );
    assert_eq!(
        assert_counter_and_picker_agree(&pool, type_b, w7.0, w7.1, "B, other type").await,
        1,
        "and it must NOT touch another type — per-type subtraction, not a \
         wider sold-out"
    );

    // A cancelled TYPED claim is as invisible as a cancelled untyped one: the
    // type split is an extra aggregate over the SAME parked-claim predicate.
    seed_parked_typed(&pool, cust, type_b, w7.0, w7.1, "cancelled", &mut seq, 3).await;
    assert_eq!(
        snapshot(&pool, w7.0, w7.1).await,
        typed,
        "cancelled typed parked bookings hold no inventory"
    );
    assert_eq!(counted(&pool, type_b, w7.0, w7.1).await, 1);

    // ── 8. B8c — claims of DIFFERENT types are attributed independently ─────
    //
    // The scenario a single-type test cannot distinguish: one live parked
    // claim on A and one on B, in the same window, with the property still
    // holding slack. Each type must lose exactly its OWN claim.
    //
    // THE SLACK IS THE WHOLE TEST. Three live claims stand against this
    // window, so without the type-C rooms the property would be
    // oversubscribed (`surplus == 0`) and BOTH zeros below would be handed
    // down by the property-wide cap — true under #304, true with the per-type
    // term deleted, true under any implementation at all. The
    // `mixed.surplus >= 1` assertion states that premise so the scenario
    // cannot quietly decay back into a tautology; with it holding, the only
    // thing that can zero A and B is the per-type subtraction. (The
    // complementary mutation — summing ALL typed claims against EVERY type —
    // is what scenario 7 catches, where B must stay at 1.)
    //
    // The bookkeeping assertions below are meaningful here for the same
    // reason: the two columns are fed by rows that are NOT all the same kind,
    // so a mis-bucketed claim actually moves them.

    seed_assigned(&pool, cust, room_a2, w8.0, w8.1, &mut seq).await;
    assert_eq!(counted(&pool, type_a, w8.0, w8.1).await, 1, "A1 free");
    assert_eq!(counted(&pool, type_b, w8.0, w8.1).await, 1, "B1 free");

    seed_parked_typed(&pool, cust, type_a, w8.0, w8.1, "confirmed", &mut seq, 1).await;
    seed_parked_typed(&pool, cust, type_b, w8.0, w8.1, "confirmed", &mut seq, 1).await;
    // ...plus one claim that names no type at all, so the two columns disagree
    // and the split has something real to get wrong.
    seed_parked(&pool, cust, w8.0, w8.1, "confirmed", &mut seq, 1).await;

    let mixed = snapshot(&pool, w8.0, w8.1).await;
    assert_eq!(
        mixed.parked_claims, 3,
        "three live parked claims: {mixed:?}"
    );
    assert_eq!(
        mixed.parked_claims_typed, 2,
        "exactly the two that name a type: {mixed:?}"
    );
    assert_eq!(
        mixed.parked_claims_untyped, 1,
        "exactly the one that does not: {mixed:?}"
    );
    assert!(
        mixed.surplus >= 1,
        "the property must still have slack, or the property-wide cap — not \
         the per-type term — is what zeroes A and B, and neither assertion \
         below could ever go red: {mixed:?}"
    );

    assert_eq!(
        assert_counter_and_picker_agree(&pool, type_a, w8.0, w8.1, "A, own claim").await,
        0,
        "A loses its last room to the claim that names A, not to the \
         property-wide cap — surplus is still positive"
    );
    assert_eq!(
        assert_counter_and_picker_agree(&pool, type_b, w8.0, w8.1, "B, own claim").await,
        0,
        "and B loses its last room to the claim that names B — independently, \
         not because A's claim spilled over"
    );

    cleanup(&pool).await;
}
