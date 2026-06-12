//! Regression test for the HF Ville prod fix (v2.66.3, 2026-05-14):
//! the `HT_Room_Status` → `ht_room_calendar` mapper was silently
//! dropping every Ville CT event with
//!   `duplicate key value violates unique constraint
//!    "ux_ht_room_calendar_legacy_id"`.
//!
//! ## Cause
//!
//! `ht_room_calendar` carries two unique constraints:
//!
//!   1. `uq_ht_room_calendar_room_date` on `(rcal_room_id, rcal_date)`
//!      — the mapper's ON CONFLICT target.
//!   2. `ux_ht_room_calendar_legacy_id` on `rcal_legacy_id`
//!      WHERE NOT NULL — the secondary partial unique.
//!
//! Legacy iHOTEL allocates `HT_Room_Status.id = MAX(id)+1` on every
//! insert (writeback recipe `allocate.rs`). When the receptionist
//! deletes a calendar row and re-creates it, the new row gets a
//! *different* legacy id but lands on the same `(room, date)` slot,
//! OR — far more commonly on a busy property like HF Ville — an old
//! id gets rebound to a *new* slot. Either way, the existing
//! `(room, date)` UPSERT trips constraint #2: the new row would
//! stamp `rcal_legacy_id = X` onto its tile, but some other tile
//! already holds `rcal_legacy_id = X`.
//!
//! Pre-fix behaviour: every such CT event errored out (R1 captured
//! `sync.mapper_apply_fail`) and the watcher skipped — the Ville
//! calendar slowly drifted out of sync.
//!
//! ## Fix
//!
//! `apply_upsert` now NULLs out `rcal_legacy_id` on any stale row
//! that holds this id at a *different* `(room, date)` pair before
//! the main UPSERT. The cleared row keeps its tile data — only the
//! drift-detection pointer is reset.
//!
//! ## What this test locks
//!
//! Applies the same `HT_Room_Status` event twice (the simplest
//! regression: id reuse on the same slot). Pre-fix the second apply
//! would fire the constraint and error; post-fix it must be a clean
//! UPDATE-style no-op.
//!
//! Then applies a second event with the same `rcal_legacy_id` but at
//! a *different* `(room, date)` — the legacy MAX+1 rebind scenario.
//! Pre-fix this would also error; post-fix it must succeed and the
//! old row's `rcal_legacy_id` must end up NULL.

mod common;

use chrono::NaiveDate;
use hotel_backend::sync::change_op::ChangeOp;
use hotel_backend::sync::mapper::MssqlChangeMapper;
use hotel_backend::sync::mappers::RoomCalendarMapper;
use hotel_backend::sync::row::test_support::{HashMapRow, MockValue};

// `room_no VARCHAR(10)` — keep under 10 chars. Two fixture rooms PER
// TEST so we can exercise the legacy-id rebind scenario.
//
// 2026-06-11: each test gets a DISJOINT (legacy_id, rooms) namespace.
// The tests previously shared one LEGACY_ID + room pair; cargo runs
// them in parallel within this binary, so one test's cleanup raced the
// other's insert and fired `ux_ht_room_calendar_legacy_id` flakily.
const DOUBLE_ROOM_A_NO: &str = "RCALD-A";
const DOUBLE_ROOM_B_NO: &str = "RCALD-B";
const DOUBLE_LEGACY_ID: i32 = 1_999_999_801;
const REBIND_ROOM_A_NO: &str = "RCALR-A";
const REBIND_ROOM_B_NO: &str = "RCALR-B";
const REBIND_LEGACY_ID: i32 = 1_999_999_802;
const FIXTURE_DATE_YMD: (i32, u32, u32) = (2026, 5, 14);

/// Idempotent fixture seed. Returns `(room_a_id, room_b_id)`.
async fn ensure_fixture_rooms(pool: &sqlx::PgPool, room_a: &str, room_b: &str) -> (i32, i32) {
    let mut ids = [0_i32; 2];
    for (idx, no) in [room_a, room_b].into_iter().enumerate() {
        let inserted: Option<i32> = sqlx::query_scalar(
            "INSERT INTO ht_rooms_new (room_no) VALUES ($1) \
             ON CONFLICT (room_no) DO NOTHING \
             RETURNING room_id",
        )
        .bind(no)
        .fetch_optional(pool)
        .await
        .expect("insert fixture room");
        ids[idx] = match inserted {
            Some(id) => id,
            None => sqlx::query_scalar(
                "SELECT room_id FROM ht_rooms_new WHERE room_no = $1 LIMIT 1",
            )
            .bind(no)
            .fetch_one(pool)
            .await
            .expect("fixture room must exist post-insert"),
        };
    }
    (ids[0], ids[1])
}

async fn cleanup_calendar_fixture(
    pool: &sqlx::PgPool,
    legacy_id: i32,
    room_a: &str,
    room_b: &str,
) {
    sqlx::query("DELETE FROM ht_room_calendar WHERE rcal_legacy_id = $1")
        .bind(legacy_id)
        .execute(pool)
        .await
        .expect("cleanup by legacy_id");
    sqlx::query(
        "DELETE FROM ht_room_calendar WHERE rcal_room_id IN \
            (SELECT room_id FROM ht_rooms_new WHERE room_no IN ($1, $2))",
    )
    .bind(room_a)
    .bind(room_b)
    .execute(pool)
    .await
    .expect("cleanup by fixture room");
}

fn make_row(legacy_id: i32, room_no: &str, status: &str) -> HashMapRow {
    let (y, m, d) = FIXTURE_DATE_YMD;
    HashMapRow::new("HT_Room_Status")
        .with("id", MockValue::I32(legacy_id))
        .with("room_no", MockValue::Str(room_no.into()))
        .with(
            "room_date",
            MockValue::DateTime(
                NaiveDate::from_ymd_opt(y, m, d)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            ),
        )
        .with("room_status", MockValue::Str(status.into()))
        .with("room_Book_No", MockValue::Null)
        .with("room_CheckIn_No", MockValue::Null)
        .with("room_Details", MockValue::Null)
}

/// Baseline idempotency: applying the same CT event twice must not
/// trip any unique constraint. Pre-fix this passed already (the
/// existing ON CONFLICT (rcal_room_id, rcal_date) caught it), but we
/// keep the assertion to lock the baseline behavior alongside the
/// new rebind scenario.
#[tokio::test]
async fn room_calendar_double_apply_same_event_is_idempotent() {
    let pool = common::create_test_pool().await;
    let (_room_a_id, _room_b_id) =
        ensure_fixture_rooms(&pool, DOUBLE_ROOM_A_NO, DOUBLE_ROOM_B_NO).await;
    cleanup_calendar_fixture(&pool, DOUBLE_LEGACY_ID, DOUBLE_ROOM_A_NO, DOUBLE_ROOM_B_NO).await;

    let row = make_row(DOUBLE_LEGACY_ID, DOUBLE_ROOM_A_NO, "เข้าพัก");

    let mut tx = pool.begin().await.expect("begin tx 1");
    RoomCalendarMapper
        .apply(&mut tx, ChangeOp::Insert, Some(&row))
        .await
        .expect("first apply must succeed");
    tx.commit().await.expect("commit tx 1");

    let mut tx = pool.begin().await.expect("begin tx 2");
    RoomCalendarMapper
        .apply(&mut tx, ChangeOp::Update, Some(&row))
        .await
        .expect("second apply must succeed (idempotent UPSERT)");
    tx.commit().await.expect("commit tx 2");

    let row_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM ht_room_calendar WHERE rcal_legacy_id = $1",
    )
    .bind(DOUBLE_LEGACY_ID)
    .fetch_one(&pool)
    .await
    .expect("count");
    assert_eq!(
        row_count, 1,
        "double-apply must yield exactly one canonical row"
    );

    cleanup_calendar_fixture(&pool, DOUBLE_LEGACY_ID, DOUBLE_ROOM_A_NO, DOUBLE_ROOM_B_NO).await;
}

/// The Ville prod fix: legacy MAX+1 allocator can rebind an old
/// `HT_Room_Status.id` to a *new* `(room, date)` slot. Pre-fix the
/// second apply triggered the `ux_ht_room_calendar_legacy_id` partial
/// unique index. Post-fix the old row's `rcal_legacy_id` is cleared
/// first so the rebind succeeds.
#[tokio::test]
async fn room_calendar_legacy_id_rebind_to_new_slot_does_not_error() {
    let pool = common::create_test_pool().await;
    let (_room_a_id, _room_b_id) =
        ensure_fixture_rooms(&pool, REBIND_ROOM_A_NO, REBIND_ROOM_B_NO).await;
    cleanup_calendar_fixture(&pool, REBIND_LEGACY_ID, REBIND_ROOM_A_NO, REBIND_ROOM_B_NO).await;

    // First apply: legacy_id stamps Room A.
    let row_a = make_row(REBIND_LEGACY_ID, REBIND_ROOM_A_NO, "เข้าพัก");
    let mut tx = pool.begin().await.expect("begin tx A");
    RoomCalendarMapper
        .apply(&mut tx, ChangeOp::Insert, Some(&row_a))
        .await
        .expect("first apply (Room A) must succeed");
    tx.commit().await.expect("commit tx A");

    // Second apply: SAME legacy_id, DIFFERENT room. Pre-fix this is
    // the exact scenario that fires `ux_ht_room_calendar_legacy_id`
    // and gets captured by R1 as `sync.mapper_apply_fail`.
    let row_b = make_row(REBIND_LEGACY_ID, REBIND_ROOM_B_NO, "จอง");
    let mut tx = pool.begin().await.expect("begin tx B");
    RoomCalendarMapper
        .apply(&mut tx, ChangeOp::Update, Some(&row_b))
        .await
        .expect(
            "rebind apply (Room B) must succeed — pre-fix this errored \
             with duplicate key on ux_ht_room_calendar_legacy_id",
        );
    tx.commit().await.expect("commit tx B");

    // Post-fix invariant: legacy_id now points at Room B; Room A's
    // tile is preserved but its drift-pointer is NULL.
    let (winning_room_no,): (String,) = sqlx::query_as(
        "SELECT r.room_no \
           FROM ht_room_calendar c \
           JOIN ht_rooms_new r ON r.room_id = c.rcal_room_id \
          WHERE c.rcal_legacy_id = $1",
    )
    .bind(REBIND_LEGACY_ID)
    .fetch_one(&pool)
    .await
    .expect("exactly one row must hold the legacy_id post-rebind");
    assert_eq!(
        winning_room_no, REBIND_ROOM_B_NO,
        "rebind must move the legacy_id pointer to the new slot"
    );

    let stale_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM ht_room_calendar c \
           JOIN ht_rooms_new r ON r.room_id = c.rcal_room_id \
          WHERE r.room_no = $1 AND c.rcal_legacy_id IS NOT NULL",
    )
    .bind(REBIND_ROOM_A_NO)
    .fetch_one(&pool)
    .await
    .expect("stale count");
    assert_eq!(
        stale_count, 0,
        "Room A's tile must keep its data but lose its drift-pointer"
    );

    cleanup_calendar_fixture(&pool, REBIND_LEGACY_ID, REBIND_ROOM_A_NO, REBIND_ROOM_B_NO).await;
}

/// 2026-06-12 (audit follow-up) — a calendar tile referencing a room PG
/// doesn't know must ERROR (watermark hold + retry) instead of the old
/// warn-skip that dropped the tile permanently. The room master mapper
/// auto-creates unknown rooms from their own `HT_Rooms` CT row, which
/// polls earlier in the same tick — so this only fires on genuine
/// corruption.
#[tokio::test]
async fn room_calendar_unknown_room_errors_instead_of_skipping() {
    let pool = common::create_test_pool().await;
    let missing_room = "RCAL-GONE";
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
        .bind(missing_room)
        .execute(&pool)
        .await
        .ok();

    let row = make_row(1_999_999_803, missing_room, "เข้าพัก");
    let mut tx = pool.begin().await.expect("begin");
    let result = RoomCalendarMapper
        .apply(&mut tx, ChangeOp::Insert, Some(&row))
        .await;
    tx.rollback().await.ok();
    assert!(
        result.is_err(),
        "unknown room must hold the watermark, got {result:?}"
    );
}
