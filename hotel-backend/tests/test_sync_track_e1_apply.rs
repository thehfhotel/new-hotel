//! Track E1 (audit 2026-05-13) integration tests for the
//! `HT_CheckIn_Other_People` + `HT_Rooms_Cancel` CT mappers and the
//! `HT_CheckIn_Ds` D-event orphan recovery path.
//!
//! Exercises the post-apply state of each mapper against a real PG
//! transaction. Test data uses a `legacy_id = 1_999_999_xxx` range so
//! it never collides with the production identity-column sequence.

mod common;

use chrono::NaiveDate;
use hotel_backend::sync::change_op::ChangeOp;
use hotel_backend::sync::mapper::MssqlChangeMapper;
use hotel_backend::sync::mappers::{GuestRegistryMapper, RoomsCancelMirrorMapper};
use hotel_backend::sync::row::test_support::{HashMapRow, MockValue};

// =============================================================================
// HT_CheckIn_Other_People → ht_guest_registry  (T2 HIGH-3)
// =============================================================================

const GUEST_LEGACY_ID: i32 = 1_999_999_701;

async fn cleanup_guest(pool: &sqlx::PgPool) {
    sqlx::query("DELETE FROM ht_guest_registry WHERE guest_legacy_id = $1")
        .bind(GUEST_LEGACY_ID)
        .execute(pool)
        .await
        .expect("cleanup guest");
}

/// Test-scoped marker for the room + customer the seed routine
/// creates if the CI database is fresh. Tests that need a brand-new
/// room set this up themselves; the lookup is exact-match so it can't
/// race with other tests.
// `room_no VARCHAR(10)` — keep under 10 chars.
const FIXTURE_ROOM_NO: &str = "E1FX-RM";
const FIXTURE_CUST_FIRSTNAME: &str = "TRACK-E1-CUST";

/// Ensure at least one room and customer row exist for the test
/// fixtures (CI's fresh DB has neither). Idempotent — re-running just
/// finds the existing row. Returns `(cust_id, room_id)`.
async fn ensure_fixture_room_and_customer(pool: &sqlx::PgPool) -> (i32, i32) {
    let cust_id: i32 = sqlx::query_scalar(
        "INSERT INTO ht_customers (cust_firstname) VALUES ($1) \
         ON CONFLICT DO NOTHING \
         RETURNING cust_id",
    )
    .bind(FIXTURE_CUST_FIRSTNAME)
    .fetch_optional(pool)
    .await
    .expect("insert fixture cust")
    .unwrap_or_else(|| {
        // Already existed — re-fetch.
        0
    });
    let cust_id = if cust_id > 0 {
        cust_id
    } else {
        sqlx::query_scalar(
            "SELECT cust_id FROM ht_customers WHERE cust_firstname = $1 LIMIT 1",
        )
        .bind(FIXTURE_CUST_FIRSTNAME)
        .fetch_one(pool)
        .await
        .expect("fixture cust must exist post-insert")
    };

    let room_id: i32 = sqlx::query_scalar(
        "INSERT INTO ht_rooms_new (room_no) VALUES ($1) \
         ON CONFLICT (room_no) DO NOTHING \
         RETURNING room_id",
    )
    .bind(FIXTURE_ROOM_NO)
    .fetch_optional(pool)
    .await
    .expect("insert fixture room")
    .unwrap_or(0);
    let room_id = if room_id > 0 {
        room_id
    } else {
        sqlx::query_scalar("SELECT room_id FROM ht_rooms_new WHERE room_no = $1 LIMIT 1")
            .bind(FIXTURE_ROOM_NO)
            .fetch_one(pool)
            .await
            .expect("fixture room must exist post-insert")
    };

    (cust_id, room_id)
}

/// Seed a parent check-in row so the mapper's FK lookup hits. Returns
/// the canonical `cin_id` for assertion. Idempotent across runs (uses
/// a fixed `legacy_cin_no` test marker).
async fn seed_parent_checkin(pool: &sqlx::PgPool, legacy_cin_no: &str) -> i32 {
    sqlx::query(
        "DELETE FROM ht_guest_registry \
            WHERE guest_cin_id IN \
                (SELECT cin_id FROM ht_checkins WHERE legacy_cin_no = $1)",
    )
    .bind(legacy_cin_no)
    .execute(pool)
    .await
    .ok();
    sqlx::query("DELETE FROM ht_checkins WHERE legacy_cin_no = $1")
        .bind(legacy_cin_no)
        .execute(pool)
        .await
        .ok();
    let (cust_id, room_id) = ensure_fixture_room_and_customer(pool).await;
    let cin_id: i32 = sqlx::query_scalar(
        "INSERT INTO ht_checkins \
            (cin_no, cin_cust_id, cin_room_id, cin_checkin_time, cin_expected_checkout, \
             cin_status, legacy_cin_no) \
         VALUES ($1, $2, $3, NOW(), CURRENT_DATE + 1, 'active', $1) \
         RETURNING cin_id",
    )
    .bind(legacy_cin_no)
    .bind(cust_id)
    .bind(room_id)
    .fetch_one(pool)
    .await
    .expect("seed parent checkin");
    cin_id
}

#[tokio::test]
async fn guest_registry_apply_insert_upserts_canonical_row() {
    let pool = common::create_test_pool().await;
    let legacy_cin_no = "CT26-E1-OP-INS";
    let expected_cin_id = seed_parent_checkin(&pool, legacy_cin_no).await;
    cleanup_guest(&pool).await;

    let row = HashMapRow::new("HT_CheckIn_Other_People")
        .with("id", MockValue::I32(GUEST_LEGACY_ID))
        .with("Cin_no", MockValue::Str(legacy_cin_no.into()))
        .with("Cin_name", MockValue::Str("John Companion".into()))
        .with("Cin_contry", MockValue::Str("US".into()));

    let mut tx = pool.begin().await.expect("begin tx");
    let event = GuestRegistryMapper
        .apply(&mut tx, ChangeOp::Insert, Some(&row))
        .await
        .expect("apply Insert");
    assert!(
        event.is_none(),
        "guest registry mapper does not emit DomainEvents (sync-only)"
    );
    tx.commit().await.expect("commit");

    let (cin_id, firstname, nationality): (i32, String, Option<String>) = sqlx::query_as(
        "SELECT guest_cin_id, guest_firstname, guest_nationality \
           FROM ht_guest_registry WHERE guest_legacy_id = $1",
    )
    .bind(GUEST_LEGACY_ID)
    .fetch_one(&pool)
    .await
    .expect("row should exist post-Insert");

    assert_eq!(cin_id, expected_cin_id);
    assert_eq!(firstname, "John Companion");
    assert_eq!(nationality.as_deref(), Some("US"));

    cleanup_guest(&pool).await;
    sqlx::query("DELETE FROM ht_checkins WHERE legacy_cin_no = $1")
        .bind(legacy_cin_no)
        .execute(&pool)
        .await
        .ok();
}

#[tokio::test]
async fn guest_registry_apply_delete_removes_canonical_row() {
    let pool = common::create_test_pool().await;
    let legacy_cin_no = "CT26-E1-OP-DEL";
    seed_parent_checkin(&pool, legacy_cin_no).await;
    cleanup_guest(&pool).await;

    // Pre-seed the row via the Insert path (orthogonal to the Delete
    // path under test).
    let insert_row = HashMapRow::new("HT_CheckIn_Other_People")
        .with("id", MockValue::I32(GUEST_LEGACY_ID))
        .with("Cin_no", MockValue::Str(legacy_cin_no.into()))
        .with("Cin_name", MockValue::Str("Ms. Anonymous".into()))
        .with("Cin_contry", MockValue::Str("TH".into()));
    let mut tx = pool.begin().await.expect("begin tx");
    GuestRegistryMapper
        .apply(&mut tx, ChangeOp::Insert, Some(&insert_row))
        .await
        .expect("pre-seed via Insert");
    tx.commit().await.expect("commit pre-seed");

    // D-row shape: PK populated, all projected non-PK cols NULL
    // (mirrors what `build_materialised_row` produces — see
    // `bin/sync.rs::materialise_row` docstring).
    let delete_row = HashMapRow::new("HT_CheckIn_Other_People")
        .with("id", MockValue::I32(GUEST_LEGACY_ID))
        .with("Cin_no", MockValue::Null)
        .with("Cin_name", MockValue::Null)
        .with("Cin_contry", MockValue::Null);

    let mut tx = pool.begin().await.expect("begin tx");
    let event = GuestRegistryMapper
        .apply(&mut tx, ChangeOp::Delete, Some(&delete_row))
        .await
        .expect("apply Delete on D-row shape must succeed");
    assert!(event.is_none());
    tx.commit().await.expect("commit");

    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM ht_guest_registry WHERE guest_legacy_id = $1",
    )
    .bind(GUEST_LEGACY_ID)
    .fetch_one(&pool)
    .await
    .expect("count");
    assert_eq!(count, 0, "D-event should have removed the canonical row");

    cleanup_guest(&pool).await;
    sqlx::query("DELETE FROM ht_checkins WHERE legacy_cin_no = $1")
        .bind(legacy_cin_no)
        .execute(&pool)
        .await
        .ok();
}

#[tokio::test]
async fn guest_registry_apply_defers_when_parent_checkin_missing() {
    let pool = common::create_test_pool().await;
    cleanup_guest(&pool).await;

    let row = HashMapRow::new("HT_CheckIn_Other_People")
        .with("id", MockValue::I32(GUEST_LEGACY_ID))
        .with(
            "Cin_no",
            MockValue::Str("CT26-DOES-NOT-EXIST".into()),
        )
        .with("Cin_name", MockValue::Str("Orphan".into()))
        .with("Cin_contry", MockValue::Str("XX".into()));

    let mut tx = pool.begin().await.expect("begin tx");
    let event = GuestRegistryMapper
        .apply(&mut tx, ChangeOp::Insert, Some(&row))
        .await
        .expect("apply must not error on missing parent — defers via Ok(None)");
    assert!(event.is_none(), "defer returns Ok(None)");
    tx.commit().await.expect("commit (no-op)");

    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM ht_guest_registry WHERE guest_legacy_id = $1",
    )
    .bind(GUEST_LEGACY_ID)
    .fetch_one(&pool)
    .await
    .expect("count");
    assert_eq!(
        count, 0,
        "deferred apply must not insert a canonical row pointing nowhere"
    );

    cleanup_guest(&pool).await;
}

// =============================================================================
// HT_Rooms_Cancel → legacy_mirror.ht_rooms_cancel  (T2 HIGH-5)
// =============================================================================

const ROOMS_CANCEL_TEST_PK: i32 = 1_999_999_801;

async fn cleanup_rooms_cancel(pool: &sqlx::PgPool) {
    sqlx::query("DELETE FROM legacy_mirror.ht_rooms_cancel WHERE id = $1")
        .bind(ROOMS_CANCEL_TEST_PK)
        .execute(pool)
        .await
        .expect("cleanup rooms_cancel");
}

#[tokio::test]
async fn rooms_cancel_mirror_apply_insert_lands_row_with_ct_source() {
    let pool = common::create_test_pool().await;
    cleanup_rooms_cancel(&pool).await;

    let row = HashMapRow::new("HT_Rooms_Cancel")
        .with("id", MockValue::I32(ROOMS_CANCEL_TEST_PK))
        .with("room_no", MockValue::Str("301".into()))
        .with("cin_no", MockValue::Str("CH26-TEST-ROOM-CANCEL".into()))
        .with(
            "cancel_date",
            MockValue::DateTime(
                NaiveDate::from_ymd_opt(2026, 5, 13)
                    .unwrap()
                    .and_hms_opt(9, 15, 0)
                    .unwrap(),
            ),
        )
        .with("cancel_by", MockValue::Str("TestRunner".into()))
        .with(
            "cancel_note",
            MockValue::Str("E1 mapper test seed".into()),
        );

    let mut tx = pool.begin().await.expect("begin tx");
    let event = RoomsCancelMirrorMapper
        .apply(&mut tx, ChangeOp::Insert, Some(&row))
        .await
        .expect("apply Insert");
    assert!(
        event.is_none(),
        "mirror mappers must not emit DomainEvents (opaque pass-through)"
    );
    tx.commit().await.expect("commit");

    let (room, cin, by, note, mirror_source): (
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        String,
    ) = sqlx::query_as(
        "SELECT room_no, cin_no, cancel_by, cancel_note, mirror_source \
           FROM legacy_mirror.ht_rooms_cancel WHERE id = $1",
    )
    .bind(ROOMS_CANCEL_TEST_PK)
    .fetch_one(&pool)
    .await
    .expect("row should exist post-Insert");

    assert_eq!(room.as_deref(), Some("301"));
    assert_eq!(cin.as_deref(), Some("CH26-TEST-ROOM-CANCEL"));
    assert_eq!(by.as_deref(), Some("TestRunner"));
    assert_eq!(note.as_deref(), Some("E1 mapper test seed"));
    assert_eq!(
        mirror_source, "ct",
        "incremental CT writes must stamp 'ct' source (vs 'reconcile' from --bootstrap)"
    );

    cleanup_rooms_cancel(&pool).await;
}

#[tokio::test]
async fn rooms_cancel_mirror_apply_delete_removes_row_on_d_row_shape() {
    let pool = common::create_test_pool().await;
    cleanup_rooms_cancel(&pool).await;

    // Pre-seed directly via SQL (orthogonal to the Insert path).
    sqlx::query(
        "INSERT INTO legacy_mirror.ht_rooms_cancel \
            (id, room_no, cin_no, cancel_date, cancel_by, cancel_note, mirror_source) \
         VALUES ($1, '301', 'CH26-TEST-ROOM-CANCEL', \
                 '2026-05-13 09:15:00', 'TestRunner', 'preseed', 'ct')",
    )
    .bind(ROOMS_CANCEL_TEST_PK)
    .execute(&pool)
    .await
    .expect("pre-seed");

    // D-row shape: PK populated, projection cols NULL.
    let row = HashMapRow::new("HT_Rooms_Cancel")
        .with("id", MockValue::I32(ROOMS_CANCEL_TEST_PK))
        .with("room_no", MockValue::Null)
        .with("cin_no", MockValue::Null)
        .with("cancel_date", MockValue::Null)
        .with("cancel_by", MockValue::Null)
        .with("cancel_note", MockValue::Null);

    let mut tx = pool.begin().await.expect("begin tx");
    let event = RoomsCancelMirrorMapper
        .apply(&mut tx, ChangeOp::Delete, Some(&row))
        .await
        .expect("apply Delete on D-row shape must succeed");
    assert!(event.is_none(), "Delete returns no DomainEvent");
    tx.commit().await.expect("commit");

    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM legacy_mirror.ht_rooms_cancel WHERE id = $1",
    )
    .bind(ROOMS_CANCEL_TEST_PK)
    .fetch_one(&pool)
    .await
    .expect("count");
    assert_eq!(count, 0, "D-event should have removed the mirror row");

    cleanup_rooms_cancel(&pool).await;
}

// =============================================================================
// HT_CheckIn_Ds D-event orphan recovery  (T2 HIGH-6)
//
// The dispatcher-level recovery lives in `bin/sync.rs`; this test
// proves the back-query SQL works against a real PG schema. Inserts
// a canonical `ht_checkins` row with a known `legacy_checkin_ds_id`
// then asserts the SELECT used by the recovery branch hits.
// =============================================================================

#[tokio::test]
async fn high_6_back_query_resolves_cin_no_from_legacy_checkin_ds_id() {
    let pool = common::create_test_pool().await;

    // legacy_cin_no is VARCHAR(20) — must be ≤20 chars.
    let test_cin_no = "CT26-E1-H6-REC";
    let test_ds_id: i32 = 1_999_999_901;

    // Cleanup any prior run.
    sqlx::query("DELETE FROM ht_checkins WHERE legacy_cin_no = $1")
        .bind(test_cin_no)
        .execute(&pool)
        .await
        .ok();

    let (cust_id, room_id) = ensure_fixture_room_and_customer(&pool).await;
    sqlx::query(
        "INSERT INTO ht_checkins \
            (cin_no, cin_cust_id, cin_room_id, cin_checkin_time, cin_expected_checkout, \
             cin_status, legacy_cin_no, legacy_checkin_ds_id) \
         VALUES ($1, $2, $3, NOW(), CURRENT_DATE + 1, 'active', $1, $4)",
    )
    .bind(test_cin_no)
    .bind(cust_id)
    .bind(room_id)
    .bind(test_ds_id)
    .execute(&pool)
    .await
    .expect("seed canonical checkin with legacy_checkin_ds_id");

    // This is the exact SQL the dispatcher uses in `bin/sync.rs`'s
    // orphan-recovery branch — keep them in sync.
    let recovered: Option<Option<String>> = sqlx::query_scalar(
        "SELECT legacy_cin_no FROM ht_checkins \
           WHERE legacy_checkin_ds_id = $1 LIMIT 1",
    )
    .bind(test_ds_id)
    .fetch_optional(&pool)
    .await
    .expect("back-query");
    assert_eq!(
        recovered.flatten().as_deref(),
        Some(test_cin_no),
        "the back-query MUST resolve the parent Cin_no for a known legacy_checkin_ds_id"
    );

    sqlx::query("DELETE FROM ht_checkins WHERE legacy_cin_no = $1")
        .bind(test_cin_no)
        .execute(&pool)
        .await
        .ok();
}

#[tokio::test]
async fn high_6_back_query_returns_none_when_no_match() {
    let pool = common::create_test_pool().await;
    // Use a ds_id far outside any realistic production range.
    let phantom_ds_id: i32 = 1_999_999_999;

    let recovered: Option<Option<String>> = sqlx::query_scalar(
        "SELECT legacy_cin_no FROM ht_checkins \
           WHERE legacy_checkin_ds_id = $1 LIMIT 1",
    )
    .bind(phantom_ds_id)
    .fetch_optional(&pool)
    .await
    .expect("back-query against missing ds_id");
    assert!(
        recovered.is_none(),
        "missing legacy_checkin_ds_id must yield None so the dispatcher logs WARN"
    );
}
