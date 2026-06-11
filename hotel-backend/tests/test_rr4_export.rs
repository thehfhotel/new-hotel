//! Track G8 — integration tests for RR.4 Thai immigration export.
//!
//! These tests seed real rows into `ht_customers` + `ht_checkins` +
//! `ht_guest_registry` + `ht_rooms_new`, run the service against a
//! live PG pool, and verify the foreign-guest filter + column-order
//! contract.
//!
//! Cleanup uses an EXACT-match marker (`TEST_rr4_g8_*`) per
//! `tests/common/mod.rs` discipline. The tests do NOT touch any row
//! they did not author.

mod common;

use chrono::NaiveDate;
use hotel_backend::service::reports::{
    render_csv, Rr4ExportFormat, Rr4ExportRequest, Rr4Row, Rr4Service, RR4_COLUMN_HEADERS,
};
use sqlx::PgPool;

/// `cleanup_g8` sweeps EVERY row carrying the shared TEST marker, and
/// each test calls it on entry — so two tests running in parallel race:
/// one's cleanup deletes the room a sibling just seeded (observed
/// 2026-06-11 as an FK violation on `fk_ht_checkins_room`). Serialize
/// the suite with one lock (same pattern as test_sync_phase55_bootstrap).
static G8_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

const ROOM_MARKER: &str = "TEST_rr4_g8_room";
const CUST_MARKER: &str = "TEST_rr4_g8_cust";
const CIN_MARKER: &str = "TEST_rr4_g8_cin";

/// Drop all rows authored by these tests. Guards every test against
/// leftover state from a previously aborted run.
async fn cleanup_g8(pool: &PgPool) {
    // Order matters: drop FK children before parents.
    sqlx::query(
        "DELETE FROM ht_rr4_exports WHERE site LIKE 'TEST_rr4_%' OR exported_by = 'rr4-g8-test'",
    )
    .execute(pool)
    .await
    .ok();
    sqlx::query(
        "DELETE FROM ht_guest_registry WHERE guest_cin_id IN \
         (SELECT cin_id FROM ht_checkins WHERE cin_notes = $1)",
    )
    .bind(CIN_MARKER)
    .execute(pool)
    .await
    .ok();
    sqlx::query("DELETE FROM ht_checkins WHERE cin_notes = $1")
        .bind(CIN_MARKER)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_customers WHERE cust_notes = $1")
        .bind(CUST_MARKER)
        .execute(pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_notes = $1")
        .bind(ROOM_MARKER)
        .execute(pool)
        .await
        .ok();
}

/// Seed a unique room. Returns `room_id`. Each invocation generates a
/// distinct `room_no` to avoid the UNIQUE constraint.
async fn seed_room(pool: &PgPool, room_no: &str) -> i32 {
    sqlx::query_scalar::<_, i32>(
        "INSERT INTO ht_rooms_new (room_no, room_notes) VALUES ($1, $2) RETURNING room_id",
    )
    .bind(room_no)
    .bind(ROOM_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed_room failed")
}

/// Seed a customer. Returns `cust_id`.
#[allow(clippy::too_many_arguments)]
async fn seed_customer(
    pool: &PgPool,
    firstname: &str,
    lastname: &str,
    nationality: &str,
    passport: &str,
    address: &str,
) -> i32 {
    sqlx::query_scalar::<_, i32>(
        "INSERT INTO ht_customers \
            (cust_firstname, cust_lastname, cust_nationality, cust_passport, \
             cust_address, cust_notes) \
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING cust_id",
    )
    .bind(firstname)
    .bind(lastname)
    .bind(nationality)
    .bind(passport)
    .bind(address)
    .bind(CUST_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed_customer failed")
}

/// Seed a check-in. `cin_no` must be unique. Returns `cin_id`.
async fn seed_checkin(
    pool: &PgPool,
    cin_no: &str,
    cust_id: i32,
    room_id: i32,
    checkin_date: NaiveDate,
) -> i32 {
    let checkin_ts = checkin_date.and_hms_opt(14, 0, 0).unwrap();
    let expected_out = checkin_date.succ_opt().unwrap();
    sqlx::query_scalar::<_, i32>(
        "INSERT INTO ht_checkins \
            (cin_no, cin_cust_id, cin_room_id, cin_checkin_time, \
             cin_expected_checkout, cin_status, cin_notes) \
         VALUES ($1, $2, $3, $4, $5, 'active', $6) RETURNING cin_id",
    )
    .bind(cin_no)
    .bind(cust_id)
    .bind(room_id)
    .bind(checkin_ts)
    .bind(expected_out)
    .bind(CIN_MARKER)
    .fetch_one(pool)
    .await
    .expect("seed_checkin failed")
}

/// Foreign-guest filter: 3 Thai + 2 foreign in window → result row
/// count = 2 (Thai filtered out). This is the spec's pivot test.
#[tokio::test]
async fn rr4_filters_thai_nationals_out_of_export() {
    let _guard = G8_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup_g8(&pool).await;

    // Distinct rooms keep room_no unique.
    let room_1 = seed_room(&pool, "T9G8-101").await;
    let room_2 = seed_room(&pool, "T9G8-102").await;
    let room_3 = seed_room(&pool, "T9G8-103").await;
    let room_4 = seed_room(&pool, "T9G8-104").await;
    let room_5 = seed_room(&pool, "T9G8-105").await;

    // 3 Thai (should be filtered out — none of these have a foreign nationality)
    let thai_1 = seed_customer(&pool, "สมชาย", "ใจดี", "TH", "", "BKK 1").await;
    let thai_2 = seed_customer(&pool, "อาภา", "ดีงาม", "Thai", "", "BKK 2").await;
    let thai_3 = seed_customer(&pool, "ไพศาล", "สมบูรณ์", "Thailand", "", "BKK 3").await;
    // 2 foreign (should be INCLUDED)
    let usa = seed_customer(&pool, "John", "Smith", "USA", "P12345", "NYC 100").await;
    let jp = seed_customer(&pool, "Yamada", "Taro", "JP", "TZ9999", "Tokyo 1-2-3").await;

    let date = NaiveDate::from_ymd_opt(2026, 5, 7).unwrap();
    seed_checkin(&pool, "TEST_G8_C1", thai_1, room_1, date).await;
    seed_checkin(&pool, "TEST_G8_C2", thai_2, room_2, date).await;
    seed_checkin(&pool, "TEST_G8_C3", thai_3, room_3, date).await;
    seed_checkin(&pool, "TEST_G8_C4", usa, room_4, date).await;
    seed_checkin(&pool, "TEST_G8_C5", jp, room_5, date).await;

    let svc = Rr4Service::new(pool.clone());
    let req = Rr4ExportRequest {
        site: "TEST_rr4_filter".into(),
        from: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        to: NaiveDate::from_ymd_opt(2026, 5, 31).unwrap(),
        format: Rr4ExportFormat::Csv,
        exported_by: "rr4-g8-test".into(),
    };
    let rows = svc
        .fetch_rows(&req)
        .await
        .expect("fetch_rows must succeed");

    // Filter the result to only rows our seeded customers authored —
    // the test database may have other foreign check-ins from earlier
    // dev work that fall in the window. We assert on the marker shape.
    let mine: Vec<&Rr4Row> = rows
        .iter()
        .filter(|r| {
            (r.full_name == "John Smith" && r.nationality == "USA")
                || (r.full_name == "Yamada Taro" && r.nationality == "JP")
        })
        .collect();
    assert_eq!(
        mine.len(),
        2,
        "expected exactly the 2 foreign rows we seeded, got: {rows:#?}"
    );
    // And none of the Thai seeded customers leaked through.
    for r in &rows {
        assert!(
            !["สมชาย ใจดี", "อาภา ดีงาม", "ไพศาล สมบูรณ์"].contains(&r.full_name.as_str()),
            "Thai-national row leaked into export: {r:?}"
        );
    }

    cleanup_g8(&pool).await;
}

/// Column-order contract: the serialised CSV header line must equal
/// `RR4_COLUMN_HEADERS` joined by commas. Anything else means the
/// regulator's expected layout has drifted.
#[tokio::test]
async fn rr4_csv_header_row_pins_column_order() {
    let bytes = render_csv(&[]).expect("empty csv render must succeed");
    let text = std::str::from_utf8(&bytes).expect("csv must be utf-8");
    let first_line = text.lines().next().expect("csv must have header");

    // CSV crate quotes fields containing commas / quotes; our headers
    // contain neither so the output is `h1,h2,h3,...` verbatim.
    let expected = RR4_COLUMN_HEADERS.join(",");
    assert_eq!(
        first_line, expected,
        "CSV header line must match RR4_COLUMN_HEADERS verbatim"
    );
}

/// End-to-end export: round-trip through `Rr4Service::export()` writes
/// an audit row with the same SHA-256 the caller receives.
#[tokio::test]
async fn rr4_export_writes_audit_row_with_matching_hash() {
    let _guard = G8_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup_g8(&pool).await;

    let svc = Rr4Service::new(pool.clone());
    let req = Rr4ExportRequest {
        site: "TEST_rr4_audit".into(),
        from: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        to: NaiveDate::from_ymd_opt(2026, 5, 31).unwrap(),
        format: Rr4ExportFormat::Csv,
        exported_by: "rr4-g8-test".into(),
    };
    let result = svc.export(req).await.expect("export must succeed");

    // Audit row landed?
    let audit_hash: Option<String> = sqlx::query_scalar(
        "SELECT file_hash FROM ht_rr4_exports \
         WHERE site = 'TEST_rr4_audit' AND exported_by = 'rr4-g8-test' \
         ORDER BY id DESC LIMIT 1",
    )
    .fetch_optional(&pool)
    .await
    .expect("audit fetch failed");

    assert_eq!(
        audit_hash.as_deref(),
        Some(result.file_hash.as_str()),
        "audit row file_hash must equal the returned hash"
    );

    cleanup_g8(&pool).await;
}

/// Row-shape contract: when the export emits a row for a foreign
/// guest, all eight columns are populated from the right tables.
#[tokio::test]
async fn rr4_row_assembles_columns_from_canonical_join() {
    let _guard = G8_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup_g8(&pool).await;

    let room = seed_room(&pool, "T9G8-901").await;
    let cust = seed_customer(
        &pool,
        "Akira",
        "Nakamura",
        "JP",
        "TZ-AUDIT-1",
        "1-2-3 Shibuya, Tokyo",
    )
    .await;
    let date = NaiveDate::from_ymd_opt(2026, 5, 10).unwrap();
    seed_checkin(&pool, "TEST_G8_AUDIT", cust, room, date).await;

    let svc = Rr4Service::new(pool.clone());
    let req = Rr4ExportRequest {
        site: "TEST_rr4_shape".into(),
        from: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        to: NaiveDate::from_ymd_opt(2026, 5, 31).unwrap(),
        format: Rr4ExportFormat::Csv,
        exported_by: "rr4-g8-test".into(),
    };
    let rows = svc.fetch_rows(&req).await.expect("fetch_rows must succeed");
    let mine = rows
        .iter()
        .find(|r| r.passport_no == "TZ-AUDIT-1")
        .expect("seeded row must appear in result");
    assert_eq!(mine.full_name, "Akira Nakamura");
    assert_eq!(mine.nationality, "JP");
    assert_eq!(mine.passport_no, "TZ-AUDIT-1");
    assert_eq!(mine.checkin_date, "2026-05-10");
    assert_eq!(mine.room_no, "T9G8-901");
    assert_eq!(mine.address, "1-2-3 Shibuya, Tokyo");
    assert!(mine.seq >= 1, "seq must be 1-indexed");

    cleanup_g8(&pool).await;
}

/// Cancelled check-ins are excluded — the regulator only wants
/// arrivals that actually happened.
#[tokio::test]
async fn rr4_excludes_cancelled_checkins() {
    let _guard = G8_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup_g8(&pool).await;

    let room = seed_room(&pool, "T9G8-801").await;
    let cust = seed_customer(
        &pool,
        "Pierre",
        "Dupont",
        "FR",
        "FR-CXL-1",
        "Paris 75001",
    )
    .await;
    let date = NaiveDate::from_ymd_opt(2026, 5, 5).unwrap();
    let cin_id = seed_checkin(&pool, "TEST_G8_CXL", cust, room, date).await;

    // Mark cancelled.
    sqlx::query("UPDATE ht_checkins SET cin_status = 'cancelled' WHERE cin_id = $1")
        .bind(cin_id)
        .execute(&pool)
        .await
        .expect("status update failed");

    let svc = Rr4Service::new(pool.clone());
    let req = Rr4ExportRequest {
        site: "TEST_rr4_cxl".into(),
        from: NaiveDate::from_ymd_opt(2026, 5, 1).unwrap(),
        to: NaiveDate::from_ymd_opt(2026, 5, 31).unwrap(),
        format: Rr4ExportFormat::Csv,
        exported_by: "rr4-g8-test".into(),
    };
    let rows = svc.fetch_rows(&req).await.expect("fetch_rows must succeed");
    let leaked = rows.iter().any(|r| r.passport_no == "FR-CXL-1");
    assert!(
        !leaked,
        "cancelled foreign check-in must NOT appear in RR.4 export"
    );

    cleanup_g8(&pool).await;
}
