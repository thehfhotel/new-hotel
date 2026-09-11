//! CR-1 — **iHOTEL WINS** the room-clean status the maid sees on `/hk`.
//!
//! ## What this pins
//!
//! The owner's decision (locked 2026-08-15): reception works the iHOTEL board,
//! so a maid must be shown legacy `HT_Rooms.Room_Clean`, with canonical
//! `ht_rooms_new.room_clean` demoted to a mirror. The unit tests in
//! `routes::hk` pin the merge RULES; this suite pins the WIRING — that
//! `GET /api/hk/rooms` and `GET /api/hk/rooms/{id}` actually serve the merged
//! value and the `legacyStatusStale` flag over real HTTP, through the real
//! handler stack, out of a real PostgreSQL row.
//!
//! Three properties, each a distinct way the feature could ship broken:
//!
//! 1. iHOTEL's value REPLACES the canonical one on both endpoints.
//! 2. An unreachable iHOTEL degrades to the canonical value with
//!    `legacyStatusStale: true` — and is **200, never 5xx**. A maid on a
//!    stairwell must never get an error page because a legacy server blinked.
//! 3. Divergence never reaches the maid: the payload carries no second
//!    opinion, only the one value she should act on.
//!
//! ## The DERIVED facts (second half of this file)
//!
//! `occupancy` merges the same way from `HT_Rooms.Room_Use`, but its FALLBACK
//! is derived per fetch from active checkins rather than read from a mirror
//! column — and `expectedArrival` / `expectedDeparture` are derived-only, with
//! no legacy counterpart at all. Those are SQL claims, so the second half of
//! this file seeds real rows (customer, rooms, folios, junction lines,
//! bookings) instead of scripting a reader.
//!
//! ## No MSSQL, ever
//!
//! The legacy read is injected as a scripted [`RoomFlagsSource`], so CI needs
//! no SQL Server and cannot flake on one. The live-MSSQL path is covered by
//! the live verification in `docs/coexistence/PENDING-VERIFICATIONS.md`.
//!
//! ## Running
//! Needs `DATABASE_URL` (the handlers read `ht_rooms_new` before the merge
//! runs); SKIPS cleanly when PG is unreachable — same convention as
//! `tests/test_hk_branch_required.rs`.
//!
//! Every test owns its OWN fixture identifiers ([`claim_room_no`] in the first
//! half, [`Occ`] in the second) and scopes every cleanup to them, so the suite
//! is safe under full `cargo test` parallelism — no `--test-threads=1`, no
//! serial harness.

use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, Mutex, OnceLock};

use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::Extension;
use hotel_backend::legacy_room_status::{LegacyRoomFlags, RoomFlagsOutcome, RoomFlagsSource};
use hotel_backend::middleware::hk_access::HkIdentity;
use hotel_backend::routes::hk::HkPolicy;
use hotel_backend::routes::mode::{AppState, Branch};
use sqlx::{PgPool, Row as _};
use tower::ServiceExt; // for `oneshot`

/// Marker room-number PREFIX for the first half of this file. Distinct from
/// every other suite's markers, and every test appends its own slug to it.
const ROOM_PREFIX: &str = "ZT-CR1";

/// This test's private marker room number: `ZT-CR1` + a one-letter slug.
///
/// `ht_rooms_new.room_no` is `VARCHAR(10) NOT NULL UNIQUE`, so when every test
/// seeded the same marker two concurrent tests raced on that unique index
/// (23505 on the seed) and deleted each other's rows on cleanup. The slug is a
/// literal chosen per test — not a clock reading, not a hash — so a given test
/// seeds the SAME row on every run and a crashed run still self-heals; it
/// simply never shares that row with a sibling on another thread.
fn claim_room_no(slug: &str) -> String {
    let room_no = format!("{ROOM_PREFIX}{slug}");
    assert!(
        room_no.len() <= 10,
        "ht_rooms_new.room_no is VARCHAR(10), got {room_no:?}"
    );
    claim(&room_no);
    room_no
}

/// Fails the run if two tests ever claim the same fixture identity — the
/// copy-paste that would silently reintroduce the collision this suite was
/// fixed for. This is a registry, not a serialisation device: the lock is held
/// for one set insert and no test waits on another.
fn claim(id: &str) {
    static CLAIMED: OnceLock<Mutex<BTreeSet<String>>> = OnceLock::new();
    assert!(
        CLAIMED
            .get_or_init(|| Mutex::new(BTreeSet::new()))
            .lock()
            .expect("fixture registry")
            .insert(id.to_string()),
        "two tests claim the fixture identity {id:?} — they would collide in parallel"
    );
}

/// A scripted iHOTEL answer — the whole reason CI needs no SQL Server.
#[derive(Debug)]
struct ScriptedIhotel(RoomFlagsOutcome);

#[async_trait]
impl RoomFlagsSource for ScriptedIhotel {
    async fn room_flags(&self) -> RoomFlagsOutcome {
        self.0.clone()
    }
}

/// iHOTEL answering the CLEANLINESS fact only — `Room_Use` UNKNOWN, so the
/// pre-existing suites keep testing the cleanliness rules in isolation.
fn ihotel_says(room_no: &str, is_clean: bool) -> RoomFlagsOutcome {
    let mut map = HashMap::new();
    map.insert(
        room_no.to_string(),
        LegacyRoomFlags {
            is_clean: Some(is_clean),
            occupied: None,
        },
    );
    RoomFlagsOutcome::Available(map)
}

/// iHOTEL answering the OCCUPANCY fact only (`Room_Use`), cleanliness UNKNOWN.
fn ihotel_occupancy(room_no: &str, occupied: bool) -> RoomFlagsOutcome {
    let mut map = HashMap::new();
    map.insert(
        room_no.to_string(),
        LegacyRoomFlags {
            is_clean: None,
            occupied: Some(occupied),
        },
    );
    RoomFlagsOutcome::Available(map)
}

async fn try_pool() -> Option<PgPool> {
    let url = std::env::var("DATABASE_URL").ok()?;
    PgPool::connect(&url).await.ok()
}

fn maid() -> HkIdentity {
    HkIdentity {
        badge: "Q1001".to_string(),
        display_name: None,
        email: None,
        can_report: true,
    }
}

/// The handler table with a verified identity injected where the Cloudflare
/// Access layer would have put one, and a scripted iHOTEL reader behind it.
fn app(state: AppState, outcome: RoomFlagsOutcome) -> axum::Router {
    let policy = HkPolicy {
        branches: vec![Branch::Hfhotel],
        ..HkPolicy::default()
    }
    .with_legacy_room_flags(Branch::Hfhotel, Arc::new(ScriptedIhotel(outcome)));
    hotel_backend::routes::hk::routes_inside_access(state, policy).layer(Extension(maid()))
}

async fn get_json(app: axum::Router, uri: &str) -> (StatusCode, serde_json::Value) {
    let req = Request::builder()
        .method("GET")
        .uri(uri)
        .body(Body::empty())
        .expect("request builds");
    let response = app.oneshot(req).await.expect("router responds");
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), 1024 * 1024)
        .await
        .expect("body reads");
    let json = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| panic!("body must be JSON: {}", String::from_utf8_lossy(&bytes)));
    (status, json)
}

/// Seed ONE active room with an explicit canonical `room_clean`, returning its
/// id. `room_clean` is deliberately the OPPOSITE of what the scripted iHOTEL
/// will say in the merge tests, so a merge that silently no-ops is a failure
/// rather than an accidental pass.
async fn seed_room(pool: &PgPool, room_no: &str, canonical_clean: bool) -> i32 {
    let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
        .bind(room_no)
        .execute(pool)
        .await;
    let row = sqlx::query(
        "INSERT INTO ht_rooms_new (room_no, room_clean, room_active) \
         VALUES ($1, $2, true) RETURNING room_id",
    )
    .bind(room_no)
    .bind(canonical_clean)
    .fetch_one(pool)
    .await
    .expect("seed insert must succeed");
    row.try_get("room_id").expect("room_id")
}

/// Scoped to THIS test's marker room, so it can never delete a sibling's row.
async fn cleanup(pool: &PgPool, room_no: &str) {
    let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
        .bind(room_no)
        .execute(pool)
        .await;
}

/// Pluck our marker room out of the list payload. Other tests' marker rooms are
/// in there too under parallelism — we only ever look at our own.
fn seeded<'a>(list: &'a serde_json::Value, room_no: &str) -> &'a serde_json::Value {
    list["data"]
        .as_array()
        .expect("data is an array")
        .iter()
        .find(|r| r["roomNo"] == room_no)
        .expect("the seeded room must be listed")
}

/// Property 1 — canonical says CLEAN, iHOTEL says DIRTY, the maid sees DIRTY.
///
/// This is the live-observable shape of the change: a room checked out in
/// iHOTEL whose CT mirror has not landed yet. Before CR-1 the maid saw
/// "clean" and walked past it.
#[tokio::test]
async fn ihotel_dirty_beats_canonical_clean_on_both_endpoints() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping ihotel_dirty_beats_canonical_clean_on_both_endpoints — PG not reachable");
        return;
    };
    let room_no = claim_room_no("A");
    let room_id = seed_room(&pool, &room_no, true).await;
    let state = AppState::new(pool.clone());

    let (status, list) = get_json(
        app(state.clone(), ihotel_says(&room_no, false)),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        seeded(&list, &room_no)["roomClean"],
        serde_json::json!(false),
        "the list must show iHOTEL's answer, not the canonical mirror: {list}"
    );
    assert_eq!(
        list["legacyStatusStale"],
        serde_json::json!(false),
        "iHOTEL answered — nothing is stale: {list}"
    );

    let (status, detail) = get_json(
        app(state, ihotel_says(&room_no, false)),
        &format!("/api/hk/rooms/{room_id}?branch=hfhotel"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        detail["room"]["roomClean"],
        serde_json::json!(false),
        "detail and list must never tell the maid different stories: {detail}"
    );
    assert_eq!(detail["legacyStatusStale"], serde_json::json!(false));

    cleanup(&pool, &room_no).await;
}

/// Property 1, the other direction — canonical says DIRTY, iHOTEL says CLEAN.
/// Pinned separately so a merge that just forced everything to `false` (or
/// inverted the polarity) cannot pass.
#[tokio::test]
async fn ihotel_clean_beats_canonical_dirty() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping ihotel_clean_beats_canonical_dirty — PG not reachable");
        return;
    };
    let room_no = claim_room_no("B");
    seed_room(&pool, &room_no, false).await;
    let state = AppState::new(pool.clone());

    let (status, list) = get_json(
        app(state, ihotel_says(&room_no, true)),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        seeded(&list, &room_no)["roomClean"],
        serde_json::json!(true),
        "{list}"
    );
    assert_eq!(list["legacyStatusStale"], serde_json::json!(false));

    cleanup(&pool, &room_no).await;
}

/// Property 2 — the fallback the owner locked. An unreachable iHOTEL must
/// produce a USABLE list: 200, canonical values untouched, and the flag that
/// makes the client render its Thai note. Stale-but-shown beats dead screen.
#[tokio::test]
async fn unreachable_ihotel_serves_the_canonical_mirror_with_the_stale_flag() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping unreachable_ihotel_serves_the_canonical_mirror_with_the_stale_flag — PG not reachable");
        return;
    };
    let room_no = claim_room_no("C");
    let room_id = seed_room(&pool, &room_no, false).await;
    let state = AppState::new(pool.clone());

    let (status, list) = get_json(
        app(state.clone(), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(
        status,
        StatusCode::OK,
        "a legacy outage must NEVER become an error page: {list}"
    );
    assert_eq!(list["success"], serde_json::json!(true));
    assert_eq!(
        seeded(&list, &room_no)["roomClean"],
        serde_json::json!(false),
        "the canonical value must be served verbatim: {list}"
    );
    assert_eq!(
        list["legacyStatusStale"],
        serde_json::json!(true),
        "the maid must be TOLD she is looking at the mirror: {list}"
    );

    let (status, detail) = get_json(
        app(state, RoomFlagsOutcome::Unavailable),
        &format!("/api/hk/rooms/{room_id}?branch=hfhotel"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(detail["legacyStatusStale"], serde_json::json!(true), "{detail}");

    cleanup(&pool, &room_no).await;
}

/// A branch with NO reader attached behaves exactly like an unreachable one.
/// This is the state the surface SHIPS in for any branch `main.rs` could not
/// wire (no legacy pool, no Ville password) — it must be a degraded display,
/// never a 500.
#[tokio::test]
async fn a_branch_without_a_reader_is_the_same_degraded_display() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping a_branch_without_a_reader_is_the_same_degraded_display — PG not reachable");
        return;
    };
    let room_no = claim_room_no("D");
    seed_room(&pool, &room_no, true).await;

    // No `with_legacy_room_flags` at all.
    let policy = HkPolicy {
        branches: vec![Branch::Hfhotel],
        ..HkPolicy::default()
    };
    let router = hotel_backend::routes::hk::routes_inside_access(AppState::new(pool.clone()), policy)
        .layer(Extension(maid()));

    let (status, list) = get_json(router, "/api/hk/rooms?branch=hfhotel").await;
    assert_eq!(status, StatusCode::OK, "{list}");
    assert_eq!(list["legacyStatusStale"], serde_json::json!(true), "{list}");
    assert_eq!(
        seeded(&list, &room_no)["roomClean"],
        serde_json::json!(true),
        "{list}"
    );

    cleanup(&pool, &room_no).await;
}

/// Property 3 — divergence is an operator signal and must not reach the wire.
/// The maid gets ONE value to act on; a second opinion on her screen is a
/// question she cannot answer.
#[tokio::test]
async fn the_maid_never_receives_the_canonical_second_opinion() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping the_maid_never_receives_the_canonical_second_opinion — PG not reachable");
        return;
    };
    let room_no = claim_room_no("E");
    seed_room(&pool, &room_no, true).await;
    let state = AppState::new(pool.clone());

    let (_, list) = get_json(
        app(state, ihotel_says(&room_no, false)),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    let body = list.to_string().to_lowercase();
    for leaked in ["diverg", "pmsclean", "canonicalclean", "ihotelclean"] {
        assert!(
            !body.contains(leaked),
            "the payload must carry no second opinion ({leaked}): {body}"
        );
    }

    cleanup(&pool, &room_no).await;
}

// ============================================================================
// Occupancy + arrival/departure — the DERIVED canonical facts, through real PG
// ============================================================================
//
// `occupancy` falls back to a value DERIVED from active checkins, and the two
// planning tags are derived-only with no legacy counterpart at all. None of
// that can be proven against a scripted reader: it is SQL, so it needs rows.
//
// Every test here seeds its OWN fixture namespace (an [`Occ`]) and tears it down
// at both ends, so a crashed run self-heals on the next one AND two tests can
// run on different threads without meeting.

/// Marker STEM stamped on every note column this section writes. Each test
/// appends its own slug, so a cleanup is exact-match on that test's rows and can
/// touch neither a sibling test's nor another suite's.
const OCC_MARKER: &str = "ZT-CR1-OCC";

/// The five scenario ROLES. These are deliberately no longer room numbers: the
/// NUMBER for a role is per test, so two tests can both seed "the departing
/// room" concurrently without racing the `room_no` unique index.
const R_STAY: &str = "STAY";
const R_OUT: &str = "OUT";
const R_NOCR: &str = "NOCR";
const R_ARR: &str = "ARR";
const R_DEP: &str = "DEP";
const OCC_ROLES: [&str; 5] = [R_STAY, R_OUT, R_NOCR, R_ARR, R_DEP];

/// One test's private fixture namespace.
///
/// Every identifier this section writes — the five room numbers, the notes
/// marker every cleanup DELETE matches on, and the UNIQUE `cin_no` / `book_no`
/// keys — is derived from a two-character slug that is a literal at the test's
/// own call site. Deterministic (a given test seeds the same rows on every run,
/// so a crashed run still self-heals) and disjoint (no two tests can meet).
struct Occ {
    slug: &'static str,
    /// The `cust_notes` / `room_notes` / `cin_notes` / `book_notes` value that
    /// scopes this test's cleanup.
    marker: String,
}

impl Occ {
    fn new(slug: &'static str) -> Self {
        claim(&format!("occ:{slug}"));
        Self {
            slug,
            marker: format!("{OCC_MARKER}-{slug}"),
        }
    }

    /// `ht_rooms_new.room_no` is `VARCHAR(10) NOT NULL UNIQUE`:
    /// `ZO` + slug(2) + `-` + role(≤4) = 9 characters at most.
    fn room_no(&self, role: &str) -> String {
        let room_no = format!("ZO{}-{role}", self.slug);
        assert!(
            room_no.len() <= 10,
            "ht_rooms_new.room_no is VARCHAR(10), got {room_no:?}"
        );
        room_no
    }

    /// `ht_checkins.cin_no` is `VARCHAR(20) NOT NULL UNIQUE`.
    fn cin_no(&self, tag: &str) -> String {
        format!("ZC{}-{tag}", self.slug)
    }

    /// `ht_bookings.book_no` is `VARCHAR(20) NOT NULL UNIQUE`.
    fn book_no(&self, tag: &str) -> String {
        format!("ZB{}-{tag}", self.slug)
    }

    /// Pluck one of THIS test's marker rooms out of a list payload. Sibling
    /// tests' rooms are in the same payload under parallelism; we never look at
    /// them.
    fn room_in<'a>(&self, list: &'a serde_json::Value, role: &str) -> &'a serde_json::Value {
        let room_no = self.room_no(role);
        list["data"]
            .as_array()
            .expect("data is an array")
            .iter()
            .find(|r| r["roomNo"] == room_no)
            .unwrap_or_else(|| panic!("room {room_no} must be listed: {list}"))
    }

    /// FK order: checkins (cascades `ht_checkin_rooms`) → bookings (cascades
    /// `ht_booking_rooms`) → rooms → customer.
    async fn cleanup(&self, pool: &PgPool) {
        for sql in [
            "DELETE FROM ht_checkins WHERE cin_notes = $1",
            "DELETE FROM ht_bookings WHERE book_notes = $1",
            "DELETE FROM ht_rooms_new WHERE room_notes = $1",
            "DELETE FROM ht_customers WHERE cust_notes = $1",
        ] {
            let _ = sqlx::query(sql).bind(&self.marker).execute(pool).await;
        }
    }

    /// Customer + the five marker rooms, all active and canonically CLEAN.
    /// Returns `(cust_id, role → room_id)`.
    async fn seed(&self, pool: &PgPool) -> (i32, HashMap<&'static str, i32>) {
        self.cleanup(pool).await;
        let cust_id: i32 = sqlx::query_scalar(
            "INSERT INTO ht_customers (cust_firstname, cust_notes) \
             VALUES ($1, $2) RETURNING cust_id",
        )
        .bind(format!("ZT_CR1_OCC_{}", self.slug))
        .bind(&self.marker)
        .fetch_one(pool)
        .await
        .expect("seed customer");

        let mut rooms = HashMap::new();
        for role in OCC_ROLES {
            let room_id: i32 = sqlx::query_scalar(
                "INSERT INTO ht_rooms_new (room_no, room_clean, room_active, room_notes) \
                 VALUES ($1, true, true, $2) RETURNING room_id",
            )
            .bind(self.room_no(role))
            .bind(&self.marker)
            .fetch_one(pool)
            .await
            .expect("seed room");
            rooms.insert(role, room_id);
        }
        (cust_id, rooms)
    }

    /// An ACTIVE folio (`cin_checkout_time IS NULL`) whose header room is
    /// `room_id`, due out on `expected_checkout`.
    async fn checkin(
        &self,
        pool: &PgPool,
        cust_id: i32,
        room_id: i32,
        tag: &str,
        expected_checkout: chrono::NaiveDate,
    ) -> i32 {
        sqlx::query_scalar(
            "INSERT INTO ht_checkins \
               (cin_no, cin_cust_id, cin_room_id, cin_checkin_time, cin_expected_checkout, \
                cin_status, cin_notes) \
             VALUES ($1, $2, $3, NOW() - INTERVAL '1 day', $4, 'active', $5) \
             RETURNING cin_id",
        )
        .bind(self.cin_no(tag))
        .bind(cust_id)
        .bind(room_id)
        .bind(expected_checkout)
        .bind(&self.marker)
        .fetch_one(pool)
        .await
        .expect("seed checkin")
    }

    /// A booking whose stay starts on `checkin` and runs one night.
    async fn booking(
        &self,
        pool: &PgPool,
        cust_id: i32,
        tag: &str,
        checkin: chrono::NaiveDate,
        status: &str,
    ) -> i32 {
        sqlx::query_scalar(
            "INSERT INTO ht_bookings \
               (book_no, book_cust_id, book_checkin, book_checkout, book_status, book_notes) \
             VALUES ($1, $2, $3, $3 + 1, $4, $5) RETURNING book_id",
        )
        .bind(self.book_no(tag))
        .bind(cust_id)
        .bind(checkin)
        .bind(status)
        .bind(&self.marker)
        .fetch_one(pool)
        .await
        .expect("seed booking")
    }
}

/// TODAY as the SQL under test computes it.
///
/// Deliberately NOT `chrono::Utc::now().date_naive()` (nor `Local`): between
/// 17:00 and 24:00 UTC the Bangkok civil day is already tomorrow, and a test
/// that built its fixture off the wrong one would go red every evening — or,
/// worse, go green against a `CURRENT_DATE` implementation that is wrong for
/// those seven hours. Asking PG the same question the predicate asks IS the
/// midnight-boundary assertion.
async fn bkk_today(pool: &PgPool) -> chrono::NaiveDate {
    sqlx::query_scalar("SELECT (NOW() AT TIME ZONE 'Asia/Bangkok')::date")
        .fetch_one(pool)
        .await
        .expect("PG answers its own date expression")
}

/// One `ht_checkin_rooms` line. `status` is the literal under test — the point
/// of the NOT-IN rule is that it mixes CT-mirrored legacy spellings with ours.
async fn occ_cr(pool: &PgPool, cin_id: i32, room_id: i32, status: &str) {
    sqlx::query(
        "INSERT INTO ht_checkin_rooms \
           (cr_cin_id, cr_room_id, cr_room_status, cr_rate_per_night, cr_nights, cr_room_total) \
         VALUES ($1, $2, $3, 0, 1, 0)",
    )
    .bind(cin_id)
    .bind(room_id)
    .bind(status)
    .execute(pool)
    .await
    .expect("seed checkin-room line");
}

async fn occ_booking_room(pool: &PgPool, book_id: i32, room_id: i32) {
    sqlx::query("INSERT INTO ht_booking_rooms (br_book_id, br_room_id) VALUES ($1, $2)")
        .bind(book_id)
        .bind(room_id)
        .execute(pool)
        .await
        .expect("seed booking-room assignment");
}

/// The DERIVED occupancy fallback, all three shapes at once, with iHOTEL
/// UNREACHABLE so the canonical value reaches the wire untouched.
///
/// This is also delta (e): a `legacyStatusStale: true` response must still
/// carry the derived occupancy, not a blank or a default. The fallback is the
/// state this surface ships in for any branch without a reader.
#[tokio::test]
async fn stale_legacy_leaves_occupancy_on_the_derived_value() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping stale_legacy_leaves_occupancy_on_the_derived_value — PG not reachable");
        return;
    };
    let occ = Occ::new("S1");
    let (cust_id, rooms) = occ.seed(&pool).await;
    let today = bkk_today(&pool).await;

    // 1. Active folio, this room's line is the legacy in-house literal.
    let cin_stay = occ.checkin(&pool, cust_id, rooms[R_STAY], "STAY", today).await;
    occ_cr(&pool, cin_stay, rooms[R_STAY], "เข้าพัก").await;

    // 2. Active folio (someone else is still in it), but THIS room's line has
    //    checked out. The room is free even though the folio is not closed —
    //    the case `LIVE_ROOM_FLAGS_SQL` gets wrong.
    let cin_out = occ.checkin(&pool, cust_id, rooms[R_OUT], "OUT", today).await;
    occ_cr(&pool, cin_out, rooms[R_OUT], "Check-Out").await;
    occ_cr(&pool, cin_out, rooms[R_STAY], "เข้าพัก").await;

    // 3. Pre-B5 shape: an active folio with NO junction rows at all, so
    //    `cin_room_id` is the only pointer to the occupied room.
    occ.checkin(&pool, cust_id, rooms[R_NOCR], "NOCR", today).await;

    let (status, list) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{list}");
    assert_eq!(
        list["legacyStatusStale"],
        serde_json::json!(true),
        "iHOTEL is unreachable in this test: {list}"
    );

    assert_eq!(
        occ.room_in(&list, R_STAY)["occupancy"],
        serde_json::json!("occupied"),
        "an active folio line in 'เข้าพัก' is OCCUPIED: {list}"
    );
    assert_eq!(
        occ.room_in(&list, R_OUT)["occupancy"],
        serde_json::json!("vacant"),
        "a 'Check-Out' line is VACANT even under a still-active folio: {list}"
    );
    assert_eq!(
        occ.room_in(&list, R_NOCR)["occupancy"],
        serde_json::json!("occupied"),
        "a folio with no junction rows still occupies its cin_room_id room: {list}"
    );

    occ.cleanup(&pool).await;
}

/// Both alternative checkout spellings free the room. `'Check Out'` (no
/// hyphen) is FrmCheckOut.cs:6246's known iHOTEL inconsistency; `'ยกเลิก'` is
/// the cancel literal. Missing either would show a maid an occupied room she
/// could have cleaned.
#[tokio::test]
async fn every_checkout_spelling_frees_the_room() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping every_checkout_spelling_frees_the_room — PG not reachable");
        return;
    };
    // One namespace for the whole loop: the iterations are serial and each one
    // re-seeds after tearing the previous down, so they cannot collide.
    let occ = Occ::new("S2");
    for literal in ["Check-Out", "Check Out", "ยกเลิก"] {
        let (cust_id, rooms) = occ.seed(&pool).await;
        let today = bkk_today(&pool).await;
        let cin = occ.checkin(&pool, cust_id, rooms[R_OUT], "SPELL", today).await;
        occ_cr(&pool, cin, rooms[R_OUT], literal).await;

        let (_, list) = get_json(
            app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
            "/api/hk/rooms?branch=hfhotel",
        )
        .await;
        assert_eq!(
            occ.room_in(&list, R_OUT)["occupancy"],
            serde_json::json!("vacant"),
            "cr_room_status {literal:?} must free the room: {list}"
        );
        occ.cleanup(&pool).await;
    }
}

/// A `cr_room_status` we do not recognise, under an ACTIVE folio, biases
/// OCCUPIED. A maid walking in on a guest is the failure to avoid; a room shown
/// occupied that is actually empty costs one door knock.
#[tokio::test]
async fn an_unrecognised_room_status_under_an_active_folio_reads_occupied() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping an_unrecognised_room_status_under_an_active_folio_reads_occupied — PG not reachable");
        return;
    };
    let occ = Occ::new("S3");
    let (cust_id, rooms) = occ.seed(&pool).await;
    let today = bkk_today(&pool).await;
    let cin = occ.checkin(&pool, cust_id, rooms[R_STAY], "JUNK", today).await;
    occ_cr(&pool, cin, rooms[R_STAY], "active").await;

    let (_, list) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(
        occ.room_in(&list, R_STAY)["occupancy"],
        serde_json::json!("occupied"),
        "our own 'active' literal — and anything else — must not free the room: {list}"
    );

    occ.cleanup(&pool).await;
}

/// CR-1 rule 1 for the SECOND fact, end to end: iHOTEL says `Room_Use='no'`
/// for a room PG derived as OCCUPIED, and the maid sees `vacant` on BOTH
/// endpoints.
///
/// This is a live shape, not a hypothetical: HF Ville room 106 on 2026-08-19
/// had exactly this disagreement (active folio in PG, `Room_Use='no'` in
/// iHOTEL). Reception works the iHOTEL board, so iHOTEL wins.
#[tokio::test]
async fn ihotel_room_use_beats_the_derived_occupancy_on_both_endpoints() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping ihotel_room_use_beats_the_derived_occupancy_on_both_endpoints — PG not reachable");
        return;
    };
    let occ = Occ::new("S4");
    let (cust_id, rooms) = occ.seed(&pool).await;
    let today = bkk_today(&pool).await;
    let cin = occ.checkin(&pool, cust_id, rooms[R_STAY], "WINS", today).await;
    occ_cr(&pool, cin, rooms[R_STAY], "เข้าพัก").await;
    let room_id = rooms[R_STAY];
    let stay_no = occ.room_no(R_STAY);
    let state = AppState::new(pool.clone());

    let (status, list) = get_json(
        app(state.clone(), ihotel_occupancy(&stay_no, false)),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        list["legacyStatusStale"],
        serde_json::json!(false),
        "iHOTEL answered — nothing is stale: {list}"
    );
    assert_eq!(
        occ.room_in(&list, R_STAY)["occupancy"],
        serde_json::json!("vacant"),
        "iHOTEL Room_Use='no' must beat the canonical derived OCCUPIED: {list}"
    );

    let (status, detail) = get_json(
        app(state, ihotel_occupancy(&stay_no, false)),
        &format!("/api/hk/rooms/{room_id}?branch=hfhotel"),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        detail["room"]["occupancy"],
        serde_json::json!("vacant"),
        "detail and list must never tell the maid different stories: {detail}"
    );

    occ.cleanup(&pool).await;
}

/// The arrival tag: a booking on this room whose stay starts TODAY and is
/// still `'confirmed'` / `'pending'`.
///
/// Every negative is a distinct way the tag could have been built wrong —
/// wrong status set, wrong date comparison, or reading `ht_bookings` without
/// the room assignment (which would tag every room on the floor).
#[tokio::test]
async fn the_arrival_tag_is_todays_unconsumed_booking_for_this_room() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping the_arrival_tag_is_todays_unconsumed_booking_for_this_room — PG not reachable");
        return;
    };
    let occ = Occ::new("S5");
    let (cust_id, rooms) = occ.seed(&pool).await;
    let today = bkk_today(&pool).await;
    let yesterday = today - chrono::Duration::days(1);
    let tomorrow = today + chrono::Duration::days(1);

    // The one that must tag.
    let b = occ.booking(&pool, cust_id, "CONF", today, "confirmed").await;
    occ_booking_room(&pool, b, rooms[R_ARR]).await;

    // Every non-tagging shape, all pointed at R_DEP so a single assertion
    // covers them: already checked in, cancelled, tomorrow, yesterday.
    for (tag, checkin, status) in [
        ("CIN", today, "checked_in"),
        ("CANC", today, "cancelled"),
        ("TOM", tomorrow, "confirmed"),
        ("YEST", yesterday, "confirmed"),
    ] {
        let id = occ.booking(&pool, cust_id, tag, checkin, status).await;
        occ_booking_room(&pool, id, rooms[R_DEP]).await;
    }

    // A today-confirmed booking with NO room assignment must tag NOTHING.
    occ.booking(&pool, cust_id, "NOROOM", today, "confirmed").await;

    let (_, list) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(
        occ.room_in(&list, R_ARR)["expectedArrival"],
        serde_json::json!(true),
        "a confirmed booking starting today tags an arrival: {list}"
    );
    assert_eq!(
        occ.room_in(&list, R_DEP)["expectedArrival"],
        serde_json::json!(false),
        "checked_in / cancelled / tomorrow / yesterday must NOT tag: {list}"
    );
    assert_eq!(
        occ.room_in(&list, R_STAY)["expectedArrival"],
        serde_json::json!(false),
        "an unassigned booking must not tag every room: {list}"
    );

    // A 'pending' hold is an arrival too — same tag, second admitted status.
    let p = occ.booking(&pool, cust_id, "PEND", today, "pending").await;
    occ_booking_room(&pool, p, rooms[R_NOCR]).await;
    let (_, list) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(
        occ.room_in(&list, R_NOCR)["expectedArrival"],
        serde_json::json!(true),
        "'pending' is an admitted arrival status: {list}"
    );

    occ.cleanup(&pool).await;
}

/// The departure tag: an ACTIVE checkin on this room due out today or EARLIER.
///
/// `<=`, not `=`: a guest past their date is still due to leave, and that is
/// the maid's planning question. Production carries zero folios more than 7
/// days overdue at either site (checked 2026-08-19), so the widened rule adds
/// no noise.
#[tokio::test]
async fn the_departure_tag_covers_today_and_overstay_but_not_tomorrow() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping the_departure_tag_covers_today_and_overstay_but_not_tomorrow — PG not reachable");
        return;
    };
    let occ = Occ::new("S6");
    let (cust_id, rooms) = occ.seed(&pool).await;
    let today = bkk_today(&pool).await;
    let yesterday = today - chrono::Duration::days(1);
    let tomorrow = today + chrono::Duration::days(1);

    // Due out TODAY.
    let c_today = occ.checkin(&pool, cust_id, rooms[R_DEP], "DTODAY", today).await;
    occ_cr(&pool, c_today, rooms[R_DEP], "เข้าพัก").await;
    // OVERSTAY — due out yesterday, still in house.
    let c_over = occ
        .checkin(&pool, cust_id, rooms[R_STAY], "DOVER", yesterday)
        .await;
    occ_cr(&pool, c_over, rooms[R_STAY], "เข้าพัก").await;
    // Staying on.
    let c_tom = occ.checkin(&pool, cust_id, rooms[R_ARR], "DTOM", tomorrow).await;
    occ_cr(&pool, c_tom, rooms[R_ARR], "เข้าพัก").await;

    let (_, list) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(
        occ.room_in(&list, R_DEP)["expectedDeparture"],
        serde_json::json!(true),
        "due out today: {list}"
    );
    assert_eq!(
        occ.room_in(&list, R_STAY)["expectedDeparture"],
        serde_json::json!(true),
        "an overstay is still due to leave: {list}"
    );
    assert_eq!(
        occ.room_in(&list, R_ARR)["expectedDeparture"],
        serde_json::json!(false),
        "a guest staying another night is not departing: {list}"
    );

    // A REAL checkout clears both the tag and the occupancy — no separate
    // "departed" state to maintain, which is the whole reason this is derived.
    sqlx::query("UPDATE ht_checkins SET cin_checkout_time = NOW(), cin_status = 'completed' WHERE cin_id = $1")
        .bind(c_today)
        .execute(&pool)
        .await
        .expect("close the folio");
    let (_, list) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    assert_eq!(
        occ.room_in(&list, R_DEP)["expectedDeparture"],
        serde_json::json!(false),
        "after a real checkout the tag is gone: {list}"
    );
    assert_eq!(
        occ.room_in(&list, R_DEP)["occupancy"],
        serde_json::json!("vacant"),
        "…and so is the occupancy: {list}"
    );

    occ.cleanup(&pool).await;
}

/// A two-room folio checking out ONE room: the checked-out room drops both its
/// occupancy and its departure tag while the sibling keeps both. Occupancy and
/// departure resolve the room through the same predicate, so this is the one
/// fixture that would catch them drifting apart.
#[tokio::test]
async fn a_per_room_checkout_only_clears_that_room_of_the_folio() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping a_per_room_checkout_only_clears_that_room_of_the_folio — PG not reachable");
        return;
    };
    let occ = Occ::new("S7");
    let (cust_id, rooms) = occ.seed(&pool).await;
    let today = bkk_today(&pool).await;

    let cin = occ.checkin(&pool, cust_id, rooms[R_OUT], "PAIR", today).await;
    occ_cr(&pool, cin, rooms[R_OUT], "Check-Out").await;
    occ_cr(&pool, cin, rooms[R_STAY], "เข้าพัก").await;

    let (_, list) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    for (role, occupancy, departing) in [(R_OUT, "vacant", false), (R_STAY, "occupied", true)] {
        assert_eq!(
            occ.room_in(&list, role)["occupancy"],
            serde_json::json!(occupancy),
            "{role} occupancy: {list}"
        );
        assert_eq!(
            occ.room_in(&list, role)["expectedDeparture"],
            serde_json::json!(departing),
            "{role} departure tag: {list}"
        );
    }
    // The header `cin_room_id` points at the CHECKED-OUT room, so the
    // assertion above is only meaningful if that is actually the case: the
    // `cin_room_id` fallback must NOT resurrect it, because the folio has
    // `cr` rows and those are authoritative when present.
    let header_room: i32 =
        sqlx::query_scalar("SELECT cin_room_id FROM ht_checkins WHERE cin_id = $1")
            .bind(cin)
            .fetch_one(&pool)
            .await
            .expect("read header room");
    assert_eq!(
        header_room, rooms[R_OUT],
        "fixture precondition: the folio header points at the checked-out room"
    );

    occ.cleanup(&pool).await;
}

/// Both tags on ONE row: a guest leaving today and a booking arriving into the
/// same room. This is the turnover the maid's list exists to prioritise, and
/// the tags are independent — neither may suppress the other.
#[tokio::test]
async fn one_room_can_be_both_departing_and_arriving_today() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping one_room_can_be_both_departing_and_arriving_today — PG not reachable");
        return;
    };
    let occ = Occ::new("S8");
    let (cust_id, rooms) = occ.seed(&pool).await;
    let today = bkk_today(&pool).await;

    let cin = occ.checkin(&pool, cust_id, rooms[R_DEP], "TURN", today).await;
    occ_cr(&pool, cin, rooms[R_DEP], "เข้าพัก").await;
    let b = occ.booking(&pool, cust_id, "TURN", today, "confirmed").await;
    occ_booking_room(&pool, b, rooms[R_DEP]).await;

    // iHOTEL UNREACHABLE: the two tags are canonical-only, so they must be
    // live and truthful in exactly the response that flags everything else
    // stale.
    let (_, list) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    let room = occ.room_in(&list, R_DEP);
    assert_eq!(list["legacyStatusStale"], serde_json::json!(true), "{list}");
    assert_eq!(room["expectedDeparture"], serde_json::json!(true), "{list}");
    assert_eq!(room["expectedArrival"], serde_json::json!(true), "{list}");
    assert_eq!(room["occupancy"], serde_json::json!("occupied"), "{list}");

    // Same story on the detail endpoint.
    let room_id = rooms[R_DEP];
    let (_, detail) = get_json(
        app(AppState::new(pool.clone()), RoomFlagsOutcome::Unavailable),
        &format!("/api/hk/rooms/{room_id}?branch=hfhotel"),
    )
    .await;
    assert_eq!(
        detail["room"]["expectedDeparture"],
        serde_json::json!(true),
        "{detail}"
    );
    assert_eq!(
        detail["room"]["expectedArrival"],
        serde_json::json!(true),
        "{detail}"
    );

    occ.cleanup(&pool).await;
}

/// An `Available` iHOTEL answer must not disturb the two canonical tags. The
/// merge touches cleanliness and occupancy ONLY; a merge that rebuilt the row
/// would silently blank the maid's planning tags.
#[tokio::test]
async fn an_ihotel_answer_leaves_the_arrival_and_departure_tags_alone() {
    let Some(pool) = try_pool().await else {
        eprintln!("skipping an_ihotel_answer_leaves_the_arrival_and_departure_tags_alone — PG not reachable");
        return;
    };
    let occ = Occ::new("S9");
    let (cust_id, rooms) = occ.seed(&pool).await;
    let today = bkk_today(&pool).await;

    let cin = occ.checkin(&pool, cust_id, rooms[R_DEP], "KEEP", today).await;
    occ_cr(&pool, cin, rooms[R_DEP], "เข้าพัก").await;
    let b = occ.booking(&pool, cust_id, "KEEP", today, "confirmed").await;
    occ_booking_room(&pool, b, rooms[R_DEP]).await;

    // iHOTEL contradicts BOTH merged facts…
    let (_, list) = get_json(
        app(
            AppState::new(pool.clone()),
            ihotel_occupancy(&occ.room_no(R_DEP), false),
        ),
        "/api/hk/rooms?branch=hfhotel",
    )
    .await;
    let room = occ.room_in(&list, R_DEP);
    assert_eq!(room["occupancy"], serde_json::json!("vacant"), "{list}");
    // …and the canonical-only tags are untouched.
    assert_eq!(room["expectedDeparture"], serde_json::json!(true), "{list}");
    assert_eq!(room["expectedArrival"], serde_json::json!(true), "{list}");

    occ.cleanup(&pool).await;
}
