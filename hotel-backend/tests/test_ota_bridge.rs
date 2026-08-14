//! OTA booking-bridge integration tests (`docs/ota-bridge.md`).
//!
//! Drives the SHIPPED router — `routes::ota::router_with_config`, the very
//! function `main.rs` mounts — so the gate order and the reconcile query under
//! test cannot drift from production wiring.
//!
//! ## What each test pins
//!
//! 1. `reconcile_dates_are_plain_yyyy_mm_dd` — **the tripwire.** The client
//!    compares `checkIn` to its own `YYYY-MM-DD` strings; a `NaiveDate`/
//!    `NaiveDateTime` serde field would emit `"2031-03-01T00:00:00"` and take
//!    the reconciler to zero matches SILENTLY (rows just stay `unmatched`).
//! 2. `null_status_is_returned_cancelled_is_not` — `coalesce(status,'') <>
//!    'cancelled'` includes NULL. A status whitelist would drop live bookings.
//! 3. `guest_name_is_empty_string_never_null` — the field is a non-optional
//!    `String`; a nameless guest yields `""`, never `null`.
//! 4. `keyset_pagination_walks_every_row_once` — no repeat, no skip, and
//!    `nextCursor` is null only on the short final page.
//! 5. `ville_booking_write_is_refused_by_the_layered_guard` — **403, not 503
//!    and not 200**: proves `ville_write_guard` really is layered under the OTA
//!    gate, i.e. `/api/ota/*` did not punch a HF Ville write hole around ADR
//!    0002.
//! 6. `disabled_surface_answers_503_even_with_a_valid_bearer` — ships DARK.
//!
//! The seven-row mode matrix itself is unit-tested at its decision function in
//! `src/middleware/ota_token.rs` (it needs no router).
//!
//! ## Running
//!
//! `DATABASE_URL` → `hotelnew` (fallback: local-dev DSN); tests SKIP when PG is
//! unreachable, matching `test_hk_ville_guard.rs`. Every fixture row carries a
//! `TEST_ota`-scoped marker unique to THIS file and is deleted by [`cleanup`]
//! with EXACT-match predicates (never `LIKE 'TEST_%'`, per `tests/common`).
//! Fixtures live in a 2031-03 check-in window used by no other test file, so
//! the reconcile page assertions can be exact.

use axum::body::Body;
use axum::http::{Request, StatusCode};
use hotel_backend::config::OtaBridgeConfig;
use hotel_backend::routes::mode::AppState;
use serde_json::Value;
use sqlx::PgPool;
use tower::ServiceExt; // for `oneshot`

const TOKEN: &str = "TEST_ota_bridge_token";

/// Fixture markers — unique to this file (see the module docs).
const NAMED_GUEST_FIRST: &str = "TEST_ota_guest";
const NAMED_GUEST_LAST: &str = "Jaidee";
const BLANK_GUEST_CODE: &str = "TEST_ota_blank_guest";
const BOOK_NO_A: &str = "TESTOTA-A";
const BOOK_NO_B: &str = "TESTOTA-B";
const BOOK_NO_CANCELLED: &str = "TESTOTA-C";
const BOOK_NO_D: &str = "TESTOTA-D";
const ALL_BOOK_NOS: [&str; 4] = [BOOK_NO_A, BOOK_NO_B, BOOK_NO_CANCELLED, BOOK_NO_D];

/// The window every fixture check-in falls inside. Used by no other test file.
const WINDOW_FROM: &str = "2031-03-01";
const WINDOW_TO: &str = "2031-03-31";

async fn new_pool() -> Option<PgPool> {
    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgresql://postgres:REDACTED-pg-2026@localhost:5439/hotelnew".to_string()
    });
    PgPool::connect(&url).await.ok()
}

fn enabled_config() -> OtaBridgeConfig {
    OtaBridgeConfig {
        enabled: true,
        enforce: true,
        token: Some(TOKEN.to_string()),
        previous_token: None,
    }
}

/// Send an authenticated request through the REAL `/api/ota/*` router.
async fn call(
    pool: PgPool,
    config: OtaBridgeConfig,
    hfville_writes: bool,
    method: &str,
    uri: &str,
    body: &str,
) -> (StatusCode, Value) {
    let state = AppState::new(pool).with_hfville_writes(hfville_writes);
    let app = hotel_backend::routes::ota::router_with_config(state, config);
    let request = Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .header("authorization", format!("Bearer {TOKEN}"))
        .body(Body::from(body.to_string()))
        .expect("request builds");

    let response = app.oneshot(request).await.expect("router responds");
    let status = response.status();
    let bytes = axum::body::to_bytes(response.into_body(), 4 * 1024 * 1024)
        .await
        .expect("body reads");
    let json = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
    (status, json)
}

/// `GET` the reconcile endpoint over the whole fixture window.
async fn reconcile(pool: &PgPool, extra: &str) -> Value {
    let uri =
        format!("/api/ota/reconcile/bookings?branch=hfhotel&checkin_from={WINDOW_FROM}&checkin_to={WINDOW_TO}{extra}");
    let (status, body) = call(pool.clone(), enabled_config(), false, "GET", &uri, "").await;
    assert_eq!(status, StatusCode::OK, "reconcile failed: {body}");
    body
}

async fn cleanup(pool: &PgPool) {
    for book_no in ALL_BOOK_NOS {
        let _ = sqlx::query("DELETE FROM ht_bookings WHERE book_no = $1")
            .bind(book_no)
            .execute(pool)
            .await;
    }
    let _ = sqlx::query("DELETE FROM ht_customers WHERE cust_firstname = $1")
        .bind(NAMED_GUEST_FIRST)
        .execute(pool)
        .await;
    let _ = sqlx::query("DELETE FROM ht_customers WHERE cust_code = $1")
        .bind(BLANK_GUEST_CODE)
        .execute(pool)
        .await;
}

/// Seed four bookings in the 2031-03 window:
///
/// | book_no | check-in | status | guest | in the reconcile page? |
/// |---|---|---|---|---|
/// | `TESTOTA-A` | 2031-03-01 | **NULL** | named | yes |
/// | `TESTOTA-B` | 2031-03-02 | confirmed | **nameless** | yes |
/// | `TESTOTA-C` | 2031-03-03 | cancelled | named | **no** |
/// | `TESTOTA-D` | 2031-03-04 | confirmed | named | yes |
async fn seed(pool: &PgPool) {
    cleanup(pool).await;

    let named: i32 = sqlx::query_scalar(
        "INSERT INTO ht_customers (cust_firstname, cust_lastname) VALUES ($1, $2) \
         RETURNING cust_id",
    )
    .bind(NAMED_GUEST_FIRST)
    .bind(NAMED_GUEST_LAST)
    .fetch_one(pool)
    .await
    .expect("seed the named guest");

    // `cust_firstname` is NOT NULL, and `book_cust_id` carries a FK, so a
    // literally orphaned booking row is unreachable by construction. The
    // observable invariant is the same either way: BOTH name parts empty must
    // produce `""` out of `trim(coalesce(..) || ' ' || coalesce(..))`, never a
    // null the client would blow up substring-searching.
    let nameless: i32 = sqlx::query_scalar(
        "INSERT INTO ht_customers (cust_code, cust_firstname, cust_lastname) \
         VALUES ($1, '', NULL) RETURNING cust_id",
    )
    .bind(BLANK_GUEST_CODE)
    .fetch_one(pool)
    .await
    .expect("seed the nameless guest");

    let rows: [(&str, &str, &str, Option<&str>, Option<&str>, Option<&str>, i32); 4] = [
        // book_no, checkin, checkout, status, notes, special_requests, cust
        (
            BOOK_NO_A,
            "2031-03-01",
            "2031-03-03",
            None,
            Some("จ่ายแล้ว Agoda"),
            None,
            named,
        ),
        (
            BOOK_NO_B,
            "2031-03-02",
            "2031-03-04",
            Some("confirmed"),
            None,
            None,
            nameless,
        ),
        (
            BOOK_NO_CANCELLED,
            "2031-03-03",
            "2031-03-05",
            Some("cancelled"),
            None,
            None,
            named,
        ),
        (
            BOOK_NO_D,
            "2031-03-04",
            "2031-03-06",
            Some("confirmed"),
            None,
            Some("late checkout"),
            named,
        ),
    ];

    for (book_no, checkin, checkout, status, notes, special, cust_id) in rows {
        sqlx::query(
            "INSERT INTO ht_bookings \
               (book_no, book_cust_id, book_checkin, book_checkout, book_status, \
                book_notes, book_special_requests) \
             VALUES ($1, $2, $3::date, $4::date, $5, $6, $7)",
        )
        .bind(book_no)
        .bind(cust_id)
        .bind(checkin)
        .bind(checkout)
        .bind(status)
        .bind(notes)
        .bind(special)
        .execute(pool)
        .await
        .unwrap_or_else(|e| panic!("seed {book_no}: {e}"));
    }
}

fn book_nos(body: &Value) -> Vec<String> {
    body["data"]
        .as_array()
        .expect("data is an array")
        .iter()
        .map(|row| row["bookNo"].as_str().expect("bookNo is a string").to_string())
        .collect()
}

/// **THE TRIPWIRE.** `checkIn`/`checkOut` must be bare `YYYY-MM-DD`. Serializing
/// them as a datetime takes ota-desk's reconciler to zero matches without a
/// single error anywhere — rows simply stay `unmatched` forever.
#[tokio::test]
async fn reconcile_dates_are_plain_yyyy_mm_dd() {
    let Some(pool) = new_pool().await else {
        eprintln!("skipping reconcile_dates_are_plain_yyyy_mm_dd — PG not reachable");
        return;
    };
    seed(&pool).await;

    let body = reconcile(&pool, "").await;
    let rows = body["data"].as_array().expect("data is an array");
    assert!(!rows.is_empty(), "fixtures must produce rows");

    let is_date = |v: &Value| -> bool {
        let s = match v.as_str() {
            Some(s) => s,
            None => return false,
        };
        s.len() == 10
            && s.as_bytes()[4] == b'-'
            && s.as_bytes()[7] == b'-'
            && s.bytes()
                .enumerate()
                .all(|(i, b)| if i == 4 || i == 7 { b == b'-' } else { b.is_ascii_digit() })
    };

    for row in rows {
        assert!(
            is_date(&row["checkIn"]),
            "checkIn must match ^\\d{{4}}-\\d{{2}}-\\d{{2}}$, got {} — the ::text cast \
             was dropped or the field became a NaiveDate/NaiveDateTime, which \
             silently zeroes ota-desk's date matching",
            row["checkIn"]
        );
        assert!(
            is_date(&row["checkOut"]),
            "checkOut must match ^\\d{{4}}-\\d{{2}}-\\d{{2}}$, got {}",
            row["checkOut"]
        );
    }

    // And the exact fixture values, so a timezone shift would fail too.
    let a = rows
        .iter()
        .find(|r| r["bookNo"] == Value::String(BOOK_NO_A.to_string()))
        .expect("TESTOTA-A is in the page");
    assert_eq!(a["checkIn"], Value::String("2031-03-01".to_string()));
    assert_eq!(a["checkOut"], Value::String("2031-03-03".to_string()));

    cleanup(&pool).await;
}

/// `coalesce(book_status,'') <> 'cancelled'` — a NULL status is a LIVE booking
/// and must be returned; only the literal `'cancelled'` is filtered out.
#[tokio::test]
async fn null_status_is_returned_cancelled_is_not() {
    let Some(pool) = new_pool().await else {
        eprintln!("skipping null_status_is_returned_cancelled_is_not — PG not reachable");
        return;
    };
    seed(&pool).await;

    let nos = book_nos(&reconcile(&pool, "").await);
    assert!(
        nos.contains(&BOOK_NO_A.to_string()),
        "a booking with book_status IS NULL must be RETURNED; rewriting the \
         predicate as a status whitelist would silently hide live bookings. got {nos:?}"
    );
    assert!(
        !nos.contains(&BOOK_NO_CANCELLED.to_string()),
        "a 'cancelled' booking must be filtered out. got {nos:?}"
    );

    // The notes composition, including its interior AND trailing space —
    // the client substring-searches this exact value.
    let body = reconcile(&pool, "").await;
    let a = body["data"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["bookNo"] == Value::String(BOOK_NO_A.to_string()))
        .expect("TESTOTA-A is in the page");
    assert_eq!(
        a["notes"],
        Value::String("จ่ายแล้ว Agoda ".to_string()),
        "notes = book_notes || ' ' || book_special_requests, trailing space preserved"
    );

    cleanup(&pool).await;
}

/// `guestName` is a non-optional `String`. A guest with no name at all yields
/// `""` out of `trim(' ')` — never `null`, which would break the client's
/// `name + ' ' + notes` substring search.
#[tokio::test]
async fn guest_name_is_empty_string_never_null() {
    let Some(pool) = new_pool().await else {
        eprintln!("skipping guest_name_is_empty_string_never_null — PG not reachable");
        return;
    };
    seed(&pool).await;

    let body = reconcile(&pool, "").await;
    let rows = body["data"].as_array().unwrap();

    let nameless = rows
        .iter()
        .find(|r| r["bookNo"] == Value::String(BOOK_NO_B.to_string()))
        .expect("TESTOTA-B is in the page");
    assert!(
        !nameless["guestName"].is_null(),
        "guestName must never be null"
    );
    assert_eq!(
        nameless["guestName"],
        Value::String(String::new()),
        "a nameless guest must yield the empty string"
    );

    // And the composed name for a normal guest.
    let named = rows
        .iter()
        .find(|r| r["bookNo"] == Value::String(BOOK_NO_A.to_string()))
        .expect("TESTOTA-A is in the page");
    assert_eq!(
        named["guestName"],
        Value::String(format!("{NAMED_GUEST_FIRST} {NAMED_GUEST_LAST}")),
        "guestName is composed in SQL as first || ' ' || last"
    );

    cleanup(&pool).await;
}

/// Keyset pagination over the three live fixtures with `limit=1`: every row
/// exactly once, in `book_no` order, and `nextCursor` null only once the page
/// comes back short.
#[tokio::test]
async fn keyset_pagination_walks_every_row_once() {
    let Some(pool) = new_pool().await else {
        eprintln!("skipping keyset_pagination_walks_every_row_once — PG not reachable");
        return;
    };
    seed(&pool).await;

    let expected = vec![
        BOOK_NO_A.to_string(),
        BOOK_NO_B.to_string(),
        BOOK_NO_D.to_string(),
    ];
    assert_eq!(
        book_nos(&reconcile(&pool, "").await),
        expected,
        "the unpaginated page is the baseline: 3 live rows, book_no ASC"
    );

    let mut walked: Vec<String> = Vec::new();
    let mut cursor: Option<String> = None;
    for page in 0..6 {
        let extra = match &cursor {
            Some(c) => format!("&limit=1&cursor={c}"),
            None => "&limit=1".to_string(),
        };
        let body = reconcile(&pool, &extra).await;
        let nos = book_nos(&body);

        if nos.is_empty() {
            assert!(
                body["nextCursor"].is_null(),
                "page {page}: a short (empty) final page must report nextCursor null"
            );
            break;
        }

        assert_eq!(nos.len(), 1, "page {page}: limit=1 must return one row");
        assert_eq!(
            body["nextCursor"],
            Value::String(nos[0].clone()),
            "page {page}: a FULL page's nextCursor is its last bookNo"
        );
        walked.extend(nos);
        cursor = body["nextCursor"].as_str().map(str::to_string);
    }

    assert_eq!(
        walked, expected,
        "keyset paging must walk every live row exactly once, in order, with no repeat or skip"
    );

    cleanup(&pool).await;
}

/// **The layering proof.** With Ville writes disabled, a `branch=hfville`
/// booking POST must be refused by `ville_write_guard` with **403**.
///
/// The three outcomes are distinguishable and that is the whole point:
/// - **403** ⇒ the Ville guard is layered under the OTA gate. Correct.
/// - **503** ⇒ the OTA gate rejected it — the test isn't proving anything.
/// - anything else ⇒ the request reached the handler and `/api/ota/*` punched a
///   HF Ville write hole around ADR 0002's admission gate.
#[tokio::test]
async fn ville_booking_write_is_refused_by_the_layered_guard() {
    let Some(pool) = new_pool().await else {
        eprintln!("skipping ville_booking_write_is_refused_by_the_layered_guard — PG not reachable");
        return;
    };

    // The guard inspects method + path + query ONLY, never the body — so an
    // empty body is enough, and deliberately cannot create anything at HF Hotel
    // in the control probe below.
    let (status, _) = call(
        pool.clone(),
        enabled_config(),
        false, // HFVILLE_WRITES_ENABLED=false
        "POST",
        "/api/ota/bookings?branch=hfville",
        "{}",
    )
    .await;
    assert_eq!(
        status,
        StatusCode::FORBIDDEN,
        "a branch=hfville booking POST must be refused by ville_write_guard (403). \
         503 would mean the OTA gate answered first (test proves nothing); any \
         other status means /api/ota/* bypassed the ADR 0002 admission gate"
    );

    // The same request at HF Hotel is NOT refused by the guard — it gets past
    // both layers and is answered by the route itself (a body rejection), so
    // the 403 above is the guard's doing and not a blanket refusal of the
    // route. The empty body guarantees nothing is ever created.
    let (hf_status, _) = call(
        pool,
        enabled_config(),
        false,
        "POST",
        "/api/ota/bookings?branch=hfhotel",
        "{}",
    )
    .await;
    assert_ne!(
        hf_status,
        StatusCode::FORBIDDEN,
        "HF Hotel must not be gated by the Ville guard"
    );
    assert_ne!(
        hf_status,
        StatusCode::SERVICE_UNAVAILABLE,
        "a valid bearer on an enabled surface must clear the OTA gate"
    );
}

/// Ships DARK: with `OTA_BRIDGE_ENABLED=false` the whole surface answers 503,
/// even to a perfectly valid bearer — the flag, not the credential, is the
/// master switch.
#[tokio::test]
async fn disabled_surface_answers_503_even_with_a_valid_bearer() {
    let Some(pool) = new_pool().await else {
        eprintln!("skipping disabled_surface_answers_503_even_with_a_valid_bearer — PG not reachable");
        return;
    };

    let dark = OtaBridgeConfig {
        enabled: false,
        enforce: false,
        token: Some(TOKEN.to_string()),
        previous_token: None,
    };

    for (method, uri) in [
        ("GET", "/api/ota/rooms?branch=hfhotel&limit=1"),
        ("GET", "/api/ota/customers/search?branch=hfhotel&search=x"),
        (
            "GET",
            "/api/ota/reconcile/bookings?branch=hfhotel&checkin_from=2031-03-01&checkin_to=2031-03-31",
        ),
        ("POST", "/api/ota/bookings?branch=hfhotel"),
    ] {
        let (status, body) = call(pool.clone(), dark.clone(), true, method, uri, "{}").await;
        assert_eq!(
            status,
            StatusCode::SERVICE_UNAVAILABLE,
            "{method} {uri} must answer 503 while OTA_BRIDGE_ENABLED is off"
        );
        assert_eq!(body["success"], Value::Bool(false));
        assert_eq!(
            body["error"],
            Value::String("ota bridge is disabled".to_string())
        );
    }

    // The default config (everything unset) is the dark one.
    let (status, _) = call(
        pool,
        OtaBridgeConfig::default(),
        true,
        "GET",
        "/api/ota/rooms?branch=hfhotel&limit=1",
        "",
    )
    .await;
    assert_eq!(
        status,
        StatusCode::SERVICE_UNAVAILABLE,
        "an unconfigured deployment must serve nothing on /api/ota/*"
    );
}

/// Param validation is the client's contract too: `branch=all`, a missing
/// date, a malformed date and an out-of-range limit are all 400 — never a
/// silently different result set.
#[tokio::test]
async fn reconcile_rejects_bad_params_with_400() {
    let Some(pool) = new_pool().await else {
        eprintln!("skipping reconcile_rejects_bad_params_with_400 — PG not reachable");
        return;
    };

    for uri in [
        "/api/ota/reconcile/bookings?checkin_from=2031-03-01&checkin_to=2031-03-31",
        "/api/ota/reconcile/bookings?branch=all&checkin_from=2031-03-01&checkin_to=2031-03-31",
        "/api/ota/reconcile/bookings?branch=hfhotel&checkin_to=2031-03-31",
        "/api/ota/reconcile/bookings?branch=hfhotel&checkin_from=01/03/2031&checkin_to=2031-03-31",
        // checkin_to before checkin_from
        "/api/ota/reconcile/bookings?branch=hfhotel&checkin_from=2031-03-31&checkin_to=2031-03-01",
        "/api/ota/reconcile/bookings?branch=hfhotel&checkin_from=2031-03-01&checkin_to=2031-03-31&limit=0",
        "/api/ota/reconcile/bookings?branch=hfhotel&checkin_from=2031-03-01&checkin_to=2031-03-31&limit=1001",
    ] {
        let (status, body) = call(pool.clone(), enabled_config(), false, "GET", uri, "").await;
        assert_eq!(
            status,
            StatusCode::BAD_REQUEST,
            "{uri} must be a 400, not a silently different result set"
        );
        assert_eq!(body["success"], Value::Bool(false), "{uri}");
    }

    // A well-formed but empty window is a 200 with an empty page and a null
    // cursor — NOT an error. The reconciler polls windows that may hold
    // nothing.
    let (status, body) = call(
        pool,
        enabled_config(),
        false,
        "GET",
        "/api/ota/reconcile/bookings?branch=hfhotel&checkin_from=2031-09-01&checkin_to=2031-09-02",
        "",
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"], Value::Array(vec![]));
    assert!(body["nextCursor"].is_null());
}
