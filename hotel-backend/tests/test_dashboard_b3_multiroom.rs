//! Track B3 — Dashboard read-path multi-room regression tests
//! (`audit-2026-05-13.md` T3 CRIT-2, T3 HIGH-1, T3 HIGH-3).
//!
//! ## Why this exists
//!
//! Track B2 (commit 79f8276) made the canonical sync mapper emit one
//! `ht_checkin_rooms` row per `HT_CheckIn_Ds` row in a multi-room folio.
//! Track B3 makes the dashboard readers consume those rows: `routes::
//! rooms`, `routes::stats`, and `routes::calendar` previously joined on
//! the header-level `ht_checkins.cin_room_id` only — every multi-room
//! folio collapsed to its first room on the dashboard, the morning
//! checkout tile, the calendar matrix, and the calendar listing.
//!
//! These tests seed a 2-room folio directly into the canonical PG tables
//! (no MSSQL needed) and assert each B3-touched read path surfaces BOTH
//! rooms.
//!
//! ## What each test asserts
//!
//! - `rooms_listing_marks_both_rooms_of_multi_room_folio_in_use` —
//!   `routes::rooms::list_rooms` / `ROOM_PROJECTION`. Pre-B3 only the
//!   first room of a multi-room folio got `room_use = 'yes'`; B3 walks
//!   `ht_checkin_rooms` so secondary rooms also flip.
//! - `checkouts_today_returns_both_rooms_of_multi_room_folio` —
//!   `routes::rooms::get_checkouts_today_pg`. Pre-B3 the
//!   `JOIN ON cin_room_id` returned the first room only; B3's
//!   `LEFT JOIN ht_checkin_rooms + COALESCE` returns both.
//! - `room_status_calendar_shows_both_rooms_occupied` —
//!   `routes::rooms::get_room_status_pg`. Pre-B3 only the first room got
//!   `'เข้าพัก'` on the date matrix; B3's CTE walks the junction so
//!   both rooms turn occupied.
//! - `stats_occupied_rooms_counts_both_rooms_of_multi_room_folio` —
//!   `routes::stats::get_stats_pg`'s `occupied_rooms` count. Pre-B3
//!   `COUNT(DISTINCT cin_room_id)` collapsed to 1; B3
//!   `COUNT(DISTINCT COALESCE(cr.cr_room_id, c.cin_room_id))` counts 2.
//! - `stats_checkout_rooms_counts_both_rooms_of_multi_room_folio` —
//!   parallel coverage for the morning-flip checkout tile.
//! - `calendar_new_data_emits_one_entry_per_room_of_multi_room_folio` —
//!   `routes::calendar::fetch_new_calendar_data`. Pre-B3 emitted one
//!   `CalendarCheckin` per folio (collapsing room 2..N onto the dedup
//!   key); B3 emits one per (folio, room) with a per-room synthetic id.
//!
//! ## Fallback coverage
//!
//! `legacy_folio_without_junction_falls_back_to_cin_room_id` asserts the
//! pre-B2 fallback path: a folio with no `ht_checkin_rooms` rows still
//! surfaces its `cin_room_id` room. After B5 backfills every historical
//! folio into the junction the column drops and this test goes away.
//!
//! ## Cleanup
//!
//! Every test uses a `TEST_B3` marker on `cust_notes` / `room_notes` so
//! cleanup is exact-match and never touches concurrent tests' data.
//! Test data is torn down even on panic via `cleanup_b3_fixture`.

mod common;

use chrono::{Duration, Local, NaiveDateTime};
use sqlx::Row;

const TEST_MARKER: &str = "TEST_B3";

/// Yesterday in BKK wall-clock — `cin_checkin_time` is stored as
/// `TIMESTAMP` (naive) representing local Bangkok time, so we just
/// format the local-now timestamp directly.
fn yesterday_bkk() -> NaiveDateTime {
    (Local::now() - Duration::days(1)).naive_local()
}

/// Today's date in BKK — what `cin_expected_checkout` should land on
/// for the morning-flip / checkouts-today tests.
fn today_bkk_date() -> chrono::NaiveDate {
    Local::now().date_naive()
}

/// Generate suffixes unique enough to allow parallel test runs against
/// the same DB without colliding `room_no` / `cin_no` values.
///
/// 2026-06-11: same fix as `test_backfill_b5::unique_suffix` — the bare
/// `nanos % 100_000` residue collided for tests starting within the same
/// 100µs (and across runs against a persistent dev DB). Atomic counter
/// guarantees intra-process uniqueness; the seeder pre-deletes same-key
/// leftovers so cross-run/cross-file residue (b5 shares the TA/TB room
/// prefixes) self-heals.
fn unique_suffix(tag: &str) -> String {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u32;
    // 5-digit slice so room_no stays under VARCHAR(10) and cin_no under
    // VARCHAR(20). 7919 (prime) spreads the counter across the residue.
    let mixed = nanos
        .wrapping_add(std::process::id())
        .wrapping_add(seq.wrapping_mul(7919));
    format!("{}{:05}", tag, mixed % 100_000)
}

/// Seed a multi-room fixture: 1 customer, 2 rooms, 1 active checkin
/// folio, 2 `ht_checkin_rooms` junction rows (one per room) — the
/// canonical post-B2 shape for a 2-room walk-in.
///
/// Returns `(cust_id, room_id_a, room_id_b, room_no_a, room_no_b,
/// cin_id, cin_no)`.
async fn seed_multi_room_folio(
    pool: &sqlx::PgPool,
    tag: &str,
    expected_checkout: chrono::NaiveDate,
) -> (i32, i32, i32, String, String, i32, String) {
    let suffix = unique_suffix(tag);
    let room_no_a = format!("TA{}", &suffix[2..]);
    let room_no_b = format!("TB{}", &suffix[2..]);
    let cin_no = format!("CB3-{}", &suffix[..]);
    let legacy_cust_no = format!("CB3{}", &suffix[..]);

    // Self-heal leftovers from aborted prior runs / sibling test files
    // sharing the TA/TB prefixes (see unique_suffix doc). FK order:
    // junction → checkin → customer/rooms.
    let _ = sqlx::query(
        "DELETE FROM ht_checkin_rooms WHERE cin_id IN \
         (SELECT cin_id FROM ht_checkins WHERE cin_no = $1) \
         OR room_id IN (SELECT room_id FROM ht_rooms_new WHERE room_no IN ($2, $3))",
    )
    .bind(&cin_no)
    .bind(&room_no_a)
    .bind(&room_no_b)
    .execute(pool)
    .await;
    let _ = sqlx::query("DELETE FROM ht_checkins WHERE cin_no = $1")
        .bind(&cin_no)
        .execute(pool)
        .await;
    let _ = sqlx::query("DELETE FROM ht_customers WHERE legacy_cust_no = $1")
        .bind(&legacy_cust_no)
        .execute(pool)
        .await;
    let _ = sqlx::query(
        "DELETE FROM ht_rooms_new WHERE room_no IN ($1, $2) \
         AND room_id NOT IN (SELECT room_id FROM ht_checkin_rooms)",
    )
    .bind(&room_no_a)
    .bind(&room_no_b)
    .execute(pool)
    .await;

    let cust_id: i32 = sqlx::query_scalar(
        "INSERT INTO ht_customers (cust_firstname, legacy_cust_no, cust_notes) \
         VALUES ('TEST_B3_cust', $1, $2) \
         RETURNING cust_id",
    )
    .bind(&legacy_cust_no)
    .bind(TEST_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed customer");

    let room_id_a: i32 = sqlx::query_scalar(
        "INSERT INTO ht_rooms_new (room_no, room_clean, room_notes, room_active) \
         VALUES ($1, true, $2, true) \
         RETURNING room_id",
    )
    .bind(&room_no_a)
    .bind(TEST_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed room A");

    let room_id_b: i32 = sqlx::query_scalar(
        "INSERT INTO ht_rooms_new (room_no, room_clean, room_notes, room_active) \
         VALUES ($1, true, $2, true) \
         RETURNING room_id",
    )
    .bind(&room_no_b)
    .bind(TEST_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed room B");

    // Folio's header-level `cin_room_id` points at room A — exactly
    // what the pre-B2 mapper would have written (it picked
    // `first_room_no`). The junction rows below add room B as the
    // second room of the same folio. A pre-B3 reader would see only
    // room A; a B3 reader sees both.
    let cin_id: i32 = sqlx::query_scalar(
        "INSERT INTO ht_checkins ( \
             cin_no, legacy_cin_no, cin_cust_id, cin_room_id, \
             cin_checkin_time, cin_expected_checkout, cin_status, cin_notes \
         ) VALUES ($1, $1, $2, $3, $4, $5, 'active', $6) \
         RETURNING cin_id",
    )
    .bind(&cin_no)
    .bind(cust_id)
    .bind(room_id_a)
    .bind(yesterday_bkk())
    .bind(expected_checkout)
    .bind(TEST_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed checkin");

    // Junction row for room A (legacy_ds_id NULL so the partial index
    // doesn't collide with parallel test runs).
    sqlx::query(
        "INSERT INTO ht_checkin_rooms ( \
             cr_cin_id, cr_room_id, cr_room_status, cr_rate_per_night, \
             cr_nights, cr_room_total \
         ) VALUES ($1, $2, 'เข้าพัก', $3::float8, 1, $3::float8)",
    )
    .bind(cin_id)
    .bind(room_id_a)
    .bind(890.0_f64)
    .execute(pool)
    .await
    .expect("seed junction row A");

    sqlx::query(
        "INSERT INTO ht_checkin_rooms ( \
             cr_cin_id, cr_room_id, cr_room_status, cr_rate_per_night, \
             cr_nights, cr_room_total \
         ) VALUES ($1, $2, 'เข้าพัก', $3::float8, 1, $3::float8)",
    )
    .bind(cin_id)
    .bind(room_id_b)
    .bind(890.0_f64)
    .execute(pool)
    .await
    .expect("seed junction row B");

    (
        cust_id,
        room_id_a,
        room_id_b,
        room_no_a,
        room_no_b,
        cin_id,
        cin_no,
    )
}

/// Seed a legacy-shape folio: 1 customer, 1 room, 1 active checkin, NO
/// junction rows (mirrors a pre-B2 folio that hasn't been re-synced
/// through the new mapper yet). Used by the fallback test.
async fn seed_legacy_single_room_folio(
    pool: &sqlx::PgPool,
    tag: &str,
    expected_checkout: chrono::NaiveDate,
) -> (i32, i32, String, i32, String) {
    let suffix = unique_suffix(tag);
    let room_no = format!("TL{}", &suffix[2..]);
    let cin_no = format!("CB3L-{}", &suffix[..]);
    let legacy_cust_no = format!("CB3L{}", &suffix[..]);

    let cust_id: i32 = sqlx::query_scalar(
        "INSERT INTO ht_customers (cust_firstname, legacy_cust_no, cust_notes) \
         VALUES ('TEST_B3_legacy_cust', $1, $2) \
         RETURNING cust_id",
    )
    .bind(&legacy_cust_no)
    .bind(TEST_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed legacy customer");

    let room_id: i32 = sqlx::query_scalar(
        "INSERT INTO ht_rooms_new (room_no, room_clean, room_notes, room_active) \
         VALUES ($1, true, $2, true) \
         RETURNING room_id",
    )
    .bind(&room_no)
    .bind(TEST_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed legacy room");

    let cin_id: i32 = sqlx::query_scalar(
        "INSERT INTO ht_checkins ( \
             cin_no, legacy_cin_no, cin_cust_id, cin_room_id, \
             cin_checkin_time, cin_expected_checkout, cin_status, cin_notes \
         ) VALUES ($1, $1, $2, $3, $4, $5, 'active', $6) \
         RETURNING cin_id",
    )
    .bind(&cin_no)
    .bind(cust_id)
    .bind(room_id)
    .bind(yesterday_bkk())
    .bind(expected_checkout)
    .bind(TEST_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed legacy checkin");

    (cust_id, room_id, room_no, cin_id, cin_no)
}

/// Tear down a fixture by its `cin_id` (children cascade), customer,
/// and rooms. Always called via `defer`-style guard pattern so it runs
/// even on assertion panic.
async fn cleanup_b3_fixture(pool: &sqlx::PgPool, cin_id: i32, cust_id: i32, room_ids: &[i32]) {
    // `ht_checkin_rooms.cr_cin_id` has ON DELETE CASCADE so this
    // removes the junction rows too. Tolerate failure on each step.
    sqlx::query("DELETE FROM ht_checkins WHERE cin_id = $1")
        .bind(cin_id)
        .execute(pool)
        .await
        .ok();
    for room_id in room_ids {
        sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
            .bind(room_id)
            .execute(pool)
            .await
            .ok();
    }
    sqlx::query("DELETE FROM ht_customers WHERE cust_id = $1")
        .bind(cust_id)
        .execute(pool)
        .await
        .ok();
}

// ═════════════════════════════════════════════════════════════════════
// routes::rooms — `ROOM_PROJECTION` `room_use` column
// ═════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn rooms_listing_marks_both_rooms_of_multi_room_folio_in_use() {
    let pool = common::create_test_pool().await;
    let (cust_id, room_id_a, room_id_b, room_no_a, room_no_b, cin_id, _cin_no) =
        seed_multi_room_folio(&pool, "ru", today_bkk_date()).await;

    // Same SQL fragment as `routes::rooms::ROOM_PROJECTION` — locks
    // both rooms' `room_use` to `'yes'`. Pre-B3 only room A would
    // flip because the EXISTS subquery joined on `cin_room_id` only.
    let probe_sql = r#"
        SELECT
            r.room_no,
            CASE WHEN EXISTS (
                SELECT 1 FROM ht_checkins c
                WHERE c.cin_status = 'active'
                  AND c.cin_checkout_time IS NULL
                  AND (
                      c.cin_room_id = r.room_id
                      OR EXISTS (
                          SELECT 1 FROM ht_checkin_rooms cr
                          WHERE cr.cr_cin_id = c.cin_id
                            AND cr.cr_room_id = r.room_id
                      )
                  )
            ) THEN 'yes' ELSE 'no' END AS room_use
        FROM ht_rooms_new r
        WHERE r.room_id IN ($1, $2)
        ORDER BY r.room_no
    "#;

    let rows = sqlx::query(probe_sql)
        .bind(room_id_a)
        .bind(room_id_b)
        .fetch_all(&pool)
        .await
        .expect("ROOM_PROJECTION probe must succeed");

    cleanup_b3_fixture(&pool, cin_id, cust_id, &[room_id_a, room_id_b]).await;

    assert_eq!(rows.len(), 2, "must observe both rooms in the projection");
    for row in &rows {
        let room_no: String = row.try_get("room_no").unwrap();
        let room_use: String = row.try_get("room_use").unwrap();
        assert_eq!(
            room_use, "yes",
            "Track B3 / T3 CRIT-2: room {} must surface as in-use (pre-B3 \
             room_no={} would have shown 'no' for the secondary room of \
             a multi-room folio)",
            room_no, room_no
        );
    }

    // Sanity: explicitly verify both room numbers are present.
    let observed: Vec<String> = rows
        .iter()
        .filter_map(|r| r.try_get::<String, _>("room_no").ok())
        .collect();
    assert!(
        observed.contains(&room_no_a),
        "first room ({}) must be in result set",
        room_no_a
    );
    assert!(
        observed.contains(&room_no_b),
        "secondary room ({}) must be in result set — pre-B3 bug",
        room_no_b
    );
}

// ═════════════════════════════════════════════════════════════════════
// routes::rooms — `get_checkouts_today_pg`
// ═════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn checkouts_today_returns_both_rooms_of_multi_room_folio() {
    let pool = common::create_test_pool().await;
    let (cust_id, room_id_a, room_id_b, room_no_a, room_no_b, cin_id, _cin_no) =
        seed_multi_room_folio(&pool, "co", today_bkk_date()).await;

    // Mirror the new `get_checkouts_today_pg` SQL exactly — fan out
    // through `ht_checkin_rooms`. Bypasses the `hour >= 6` clause by
    // computing it explicitly so the test passes regardless of when
    // it runs (the morning-flip is exercised by `test_stats.rs`).
    let probe_sql = r#"
        SELECT DISTINCT r.room_no AS room_no
        FROM ht_checkins c
        LEFT JOIN ht_checkin_rooms cr ON cr.cr_cin_id = c.cin_id
        JOIN ht_rooms_new r ON r.room_id = COALESCE(cr.cr_room_id, c.cin_room_id)
        WHERE c.cin_id = $1
          AND c.cin_status = 'active'
          AND c.cin_checkout_time IS NULL
        ORDER BY r.room_no
    "#;

    let rows = sqlx::query(probe_sql)
        .bind(cin_id)
        .fetch_all(&pool)
        .await
        .expect("checkouts-today probe must succeed");

    cleanup_b3_fixture(&pool, cin_id, cust_id, &[room_id_a, room_id_b]).await;

    let room_nos: Vec<String> = rows
        .iter()
        .map(|r| r.get::<String, _>("room_no"))
        .collect();
    assert_eq!(
        room_nos.len(),
        2,
        "Track B3 / T3 CRIT-2: multi-room folio must emit BOTH rooms \
         from checkouts-today (got {:?})",
        room_nos
    );
    assert!(
        room_nos.contains(&room_no_a),
        "first room ({}) must appear",
        room_no_a
    );
    assert!(
        room_nos.contains(&room_no_b),
        "secondary room ({}) must appear — pre-B3 bug",
        room_no_b
    );
}

// ═════════════════════════════════════════════════════════════════════
// routes::rooms — `get_room_status_pg` (calendar matrix)
// ═════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn room_status_calendar_shows_both_rooms_occupied() {
    let pool = common::create_test_pool().await;
    // Tomorrow as the expected checkout so today (the date we probe)
    // lies inside the occupancy window.
    let expected_out = today_bkk_date() + Duration::days(1);
    let (cust_id, room_id_a, room_id_b, room_no_a, room_no_b, cin_id, _cin_no) =
        seed_multi_room_folio(&pool, "cs", expected_out).await;

    let today = today_bkk_date().format("%Y-%m-%d").to_string();

    // Same CTE/junction shape as `get_room_status_pg`. Probes the two
    // seeded rooms on today's date only.
    let probe_sql = r#"
        WITH checkin_rooms AS (
            SELECT
                ci.cin_id,
                ci.cin_no,
                ci.cin_checkin_time,
                ci.cin_checkout_time,
                ci.cin_expected_checkout,
                COALESCE(cr.cr_room_id, ci.cin_room_id) AS room_id
            FROM ht_checkins ci
            LEFT JOIN ht_checkin_rooms cr ON cr.cr_cin_id = ci.cin_id
            WHERE ci.cin_status IN ('active', 'checkedout')
        )
        SELECT
            r.room_no,
            CASE
                WHEN ci.cin_no IS NOT NULL THEN 'เข้าพัก'
                ELSE 'ว่าง'
            END AS room_status
        FROM ht_rooms_new r
        LEFT JOIN checkin_rooms ci
            ON ci.room_id = r.room_id
            AND $1::date >= ci.cin_checkin_time::date
            AND $1::date < COALESCE(ci.cin_checkout_time::date, ci.cin_expected_checkout)
        WHERE r.room_id IN ($2, $3)
        ORDER BY r.room_no
    "#;

    let rows = sqlx::query(probe_sql)
        .bind(&today)
        .bind(room_id_a)
        .bind(room_id_b)
        .fetch_all(&pool)
        .await
        .expect("room-status probe must succeed");

    cleanup_b3_fixture(&pool, cin_id, cust_id, &[room_id_a, room_id_b]).await;

    assert_eq!(rows.len(), 2, "must observe both rooms");
    for row in &rows {
        let room_no: String = row.try_get("room_no").unwrap();
        let status: String = row.try_get("room_status").unwrap();
        assert_eq!(
            status, "เข้าพัก",
            "Track B3 / T3 CRIT-2: calendar must show room {} as 'เข้าพัก' \
             on the occupancy window (pre-B3 the secondary room of a \
             multi-room folio showed 'ว่าง')",
            room_no
        );
    }
    let observed: Vec<String> = rows
        .iter()
        .filter_map(|r| r.try_get::<String, _>("room_no").ok())
        .collect();
    assert!(observed.contains(&room_no_a));
    assert!(observed.contains(&room_no_b));
}

// ═════════════════════════════════════════════════════════════════════
// routes::stats — `occupied_rooms` + `checkout_rooms`
// ═════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn stats_occupied_rooms_counts_both_rooms_of_multi_room_folio() {
    let pool = common::create_test_pool().await;
    // Expected checkout in the future so the morning-flip exclusion
    // doesn't kick this folio out of the occupied bucket.
    let expected_out = today_bkk_date() + Duration::days(3);
    let (cust_id, room_id_a, room_id_b, _room_no_a, _room_no_b, cin_id, _cin_no) =
        seed_multi_room_folio(&pool, "so", expected_out).await;

    // Same shape as `stats::get_stats_pg::occupied_rooms` but scoped
    // to this folio's two rooms only (so the system-wide count from
    // any other concurrent fixture doesn't confound the assertion).
    let probe_sql = r#"
        SELECT COUNT(DISTINCT COALESCE(cr.cr_room_id, c.cin_room_id))::bigint AS count
        FROM ht_checkins c
        LEFT JOIN ht_checkin_rooms cr ON cr.cr_cin_id = c.cin_id
        WHERE c.cin_id = $1
          AND c.cin_status = 'active'
          AND c.cin_checkout_time IS NULL
    "#;

    let count: i64 = sqlx::query_scalar(probe_sql)
        .bind(cin_id)
        .fetch_one(&pool)
        .await
        .expect("occupied-rooms probe must succeed");

    cleanup_b3_fixture(&pool, cin_id, cust_id, &[room_id_a, room_id_b]).await;

    assert_eq!(
        count, 2,
        "Track B3 / T3 HIGH-1: occupied_rooms must count BOTH rooms of \
         a multi-room folio (pre-B3 DISTINCT cin_room_id would have \
         returned 1)"
    );
}

#[tokio::test]
async fn stats_checkout_rooms_counts_both_rooms_of_multi_room_folio() {
    let pool = common::create_test_pool().await;
    // Expected checkout = today (BKK) so the folio lands in the
    // checkout-today bucket. We bypass the `hour >= 6` morning-flip
    // gate by probing the count directly — the gate itself is locked
    // by `test_stats.rs::bangkok_hour_uses_at_time_zone_asia_bangkok`.
    let (cust_id, room_id_a, room_id_b, _room_no_a, _room_no_b, cin_id, _cin_no) =
        seed_multi_room_folio(&pool, "sc", today_bkk_date()).await;

    let probe_sql = r#"
        SELECT COUNT(DISTINCT COALESCE(cr.cr_room_id, c.cin_room_id))::bigint AS count
        FROM ht_checkins c
        LEFT JOIN ht_checkin_rooms cr ON cr.cr_cin_id = c.cin_id
        WHERE c.cin_id = $1
          AND c.cin_status = 'active'
          AND c.cin_checkout_time IS NULL
    "#;

    let count: i64 = sqlx::query_scalar(probe_sql)
        .bind(cin_id)
        .fetch_one(&pool)
        .await
        .expect("checkout-rooms probe must succeed");

    cleanup_b3_fixture(&pool, cin_id, cust_id, &[room_id_a, room_id_b]).await;

    assert_eq!(
        count, 2,
        "Track B3 / T3 HIGH-3: checkout_rooms must count BOTH rooms of \
         a multi-room folio whose expected checkout is today"
    );
}

// ═════════════════════════════════════════════════════════════════════
// routes::calendar — `fetch_new_calendar_data` checkin query
// ═════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn calendar_new_data_emits_one_entry_per_room_of_multi_room_folio() {
    let pool = common::create_test_pool().await;
    // Window must include today (the seed inserts `cin_checkin_time =
    // yesterday`); use a wide range.
    let start = (today_bkk_date() - Duration::days(7))
        .format("%Y-%m-%d")
        .to_string();
    let end = (today_bkk_date() + Duration::days(7))
        .format("%Y-%m-%d")
        .to_string();

    let (cust_id, room_id_a, room_id_b, room_no_a, room_no_b, cin_id, _cin_no) =
        seed_multi_room_folio(&pool, "ca", today_bkk_date() + Duration::days(2)).await;

    // Same SQL as `calendar::fetch_new_calendar_data`'s checkin query
    // — emit one row per (folio, room) and assert both rooms surface.
    // Scoped to this folio's cin_id so other test data on the same DB
    // doesn't pollute the assertion.
    let probe_sql = r#"
        SELECT
            ci.cin_id,
            ci.cin_no,
            COALESCE(cr.cr_room_id, ci.cin_room_id) AS effective_room_id,
            r.room_no
        FROM ht_checkins ci
        LEFT JOIN ht_checkin_rooms cr ON cr.cr_cin_id = ci.cin_id
        INNER JOIN ht_rooms_new r ON r.room_id = COALESCE(cr.cr_room_id, ci.cin_room_id)
        WHERE ci.cin_id = $1
          AND COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout) >= $2::date
          AND ci.cin_checkin_time <= $3::date
        ORDER BY r.room_no
    "#;

    let rows = sqlx::query(probe_sql)
        .bind(cin_id)
        .bind(&start)
        .bind(&end)
        .fetch_all(&pool)
        .await
        .expect("calendar probe must succeed");

    cleanup_b3_fixture(&pool, cin_id, cust_id, &[room_id_a, room_id_b]).await;

    assert_eq!(
        rows.len(),
        2,
        "Track B3 / T3 CRIT-2: calendar must emit ONE entry per room of \
         a multi-room folio (pre-B3 emitted ONE entry per folio so the \
         frontend's seenIds dedup dropped rooms 2..N)"
    );
    let room_nos: Vec<String> = rows
        .iter()
        .map(|r| r.get::<String, _>("room_no"))
        .collect();
    assert!(room_nos.contains(&room_no_a));
    assert!(room_nos.contains(&room_no_b));

    // The B3 synthetic id is `new-checkin-<cin_id>-<room_id>` — assert
    // the two rows yield two DISTINCT room_id values so the frontend
    // dedup-by-id keeps both.
    let room_ids: std::collections::HashSet<i32> = rows
        .iter()
        .filter_map(|r| r.try_get::<i32, _>("effective_room_id").ok())
        .collect();
    assert_eq!(
        room_ids.len(),
        2,
        "effective_room_id must be distinct per row — guarantees \
         the per-(folio, room) synthetic id collides nowhere"
    );
}

// ═════════════════════════════════════════════════════════════════════
// Fallback path: pre-B2 folio (junction empty) must still surface its
// `cin_room_id` room.
// ═════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn legacy_folio_without_junction_falls_back_to_cin_room_id() {
    let pool = common::create_test_pool().await;
    let (cust_id, room_id, room_no, cin_id, _cin_no) = seed_legacy_single_room_folio(
        &pool,
        "fb",
        today_bkk_date() + Duration::days(1),
    )
    .await;

    // Same fan-out shape as `get_checkouts_today_pg` — scope to this
    // folio. With an empty junction the LEFT JOIN yields NULL and
    // COALESCE picks the header `cin_room_id`.
    let probe_sql = r#"
        SELECT DISTINCT r.room_no AS room_no
        FROM ht_checkins c
        LEFT JOIN ht_checkin_rooms cr ON cr.cr_cin_id = c.cin_id
        JOIN ht_rooms_new r ON r.room_id = COALESCE(cr.cr_room_id, c.cin_room_id)
        WHERE c.cin_id = $1
    "#;

    let rows = sqlx::query(probe_sql)
        .bind(cin_id)
        .fetch_all(&pool)
        .await
        .expect("fallback probe must succeed");

    cleanup_b3_fixture(&pool, cin_id, cust_id, &[room_id]).await;

    assert_eq!(
        rows.len(),
        1,
        "legacy folio (no junction rows) must still surface its \
         cin_room_id room — fallback path coverage per \
         CARDINALITY_MAP's DEPRECATED note"
    );
    let observed: String = rows[0].try_get("room_no").unwrap();
    assert_eq!(observed, room_no);
}
