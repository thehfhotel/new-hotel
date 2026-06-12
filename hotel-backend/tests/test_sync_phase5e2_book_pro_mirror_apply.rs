//! Phase 5/E2 integration tests for `BookProMirrorMapper`
//! (coexistence audit 2026-06-11 P2 — `HT_Book_Pro` ingestion).
//!
//! Follows `test_sync_phase55c_mirror_apply.rs` conventions exactly
//! (its 2026-06-11 determinism hardening included): a static Mutex
//! serialises the tests that share the fixture PK, every test cleans
//! up before AND after itself, and the fixture PK sits far outside the
//! production id range (HT_Book_Pro has 0 rows at HF Hotel per
//! `schema-baseline.txt`; the IDENTITY would hand out small ids).
//!
//! Covered:
//! * Insert lands the row with `mirror_source='ct'` and faithful
//!   column values.
//! * Delete works on the post-N4-fix materialised D-row shape (PK
//!   populated via the CT-side `pk_<col>` alias, every projected
//!   column NULL — the LEFT JOIN nulls them) and removes the row.
//!   This matters doubly for HT_Book_Pro because iHOTEL's edit flow is
//!   delete-then-reinsert (`COMPAT_CHEATSHEET.md` §3.7 step 5): every
//!   booking edit fires D rows.
//! * Delete is idempotent on an already-gone row (CT can redeliver).

mod common;

use hotel_backend::sync::change_op::ChangeOp;
use hotel_backend::sync::mapper::MssqlChangeMapper;
use hotel_backend::sync::mappers::BookProMirrorMapper;
use hotel_backend::sync::row::test_support::{HashMapRow, MockValue};

const TEST_PK: i32 = 1_999_999_998;

/// All tests share `TEST_PK` and cargo runs them in parallel — one
/// shared lock keeps the shared-fixture mutations deterministic
/// (2026-06-11 pattern, same as `test_sync_phase55c_mirror_apply`).
static BOOK_PRO_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

async fn cleanup(pool: &sqlx::PgPool) {
    // No canonical dual-write for HT_Book_Pro (pure mirror table), so
    // a single sweep keeps reruns against a persistent dev DB
    // idempotent.
    sqlx::query("DELETE FROM legacy_mirror.ht_book_pro WHERE id = $1")
        .bind(TEST_PK)
        .execute(pool)
        .await
        .expect("cleanup");
}

fn insert_row() -> HashMapRow {
    HashMapRow::new("HT_Book_Pro")
        .with("id", MockValue::I32(TEST_PK))
        .with("B_NO", MockValue::Str("R999999".into()))
        .with("B_ROOM", MockValue::Str("301".into()))
        .with("B_NAME", MockValue::Str("น้ำดื่ม".into()))
        .with("B_UNIT", MockValue::Str("ขวด".into()))
        .with("B_NUM", MockValue::F64(2.0))
        .with("B_PRICE", MockValue::F64(20.0))
        .with("B_PRICE_TOTAL", MockValue::F64(40.0))
        .with("B_PRO_ID", MockValue::I32(17))
}

#[tokio::test]
async fn book_pro_mirror_apply_insert_lands_row_with_ct_source() {
    let _guard = BOOK_PRO_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup(&pool).await;

    let row = insert_row();

    let mut tx = pool.begin().await.expect("begin tx");
    let event = BookProMirrorMapper
        .apply(&mut tx, ChangeOp::Insert, Some(&row))
        .await
        .expect("apply Insert");
    assert!(
        event.is_none(),
        "mirror mappers must not emit DomainEvents (opaque pass-through)"
    );
    tx.commit().await.expect("commit");

    type MirrorRow = (
        Option<String>,
        Option<String>,
        Option<String>,
        Option<f64>,
        Option<i32>,
        String,
    );
    let (b_no, b_room, b_name, total, pro_id, mirror_source): MirrorRow = sqlx::query_as(
        "SELECT b_no, b_room, b_name, b_price_total, b_pro_id, mirror_source \
           FROM legacy_mirror.ht_book_pro WHERE id = $1",
    )
    .bind(TEST_PK)
    .fetch_one(&pool)
    .await
    .expect("row should exist post-Insert");

    assert_eq!(b_no.as_deref(), Some("R999999"));
    assert_eq!(b_room.as_deref(), Some("301"));
    assert_eq!(b_name.as_deref(), Some("น้ำดื่ม"));
    assert_eq!(total, Some(40.0));
    assert_eq!(pro_id, Some(17));
    assert_eq!(
        mirror_source, "ct",
        "incremental CT writes must stamp 'ct' source (vs 'reconcile' from --bootstrap)"
    );

    cleanup(&pool).await;
}

#[tokio::test]
async fn book_pro_mirror_apply_delete_removes_row_using_ct_pk_alias_only() {
    // iHOTEL's FrmAddBook2.SAVE_EDIT runs `delete from HT_Book_Pro
    // where [B_NO]='<id>'` then re-inserts (cheatsheet §3.7 step 5),
    // so D rows are routine. Post-N4-fix materialise_row hands the
    // mapper a row whose PK carries the real CT-side value while every
    // projected column is NULL (the LEFT JOIN nulls the deleted row).
    let _guard = BOOK_PRO_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup(&pool).await;

    // Pre-seed directly via SQL (bypass the mapper to keep this test
    // orthogonal to the Insert path).
    sqlx::query(
        "INSERT INTO legacy_mirror.ht_book_pro \
            (id, b_no, b_room, b_name, b_unit, b_num, \
             b_price, b_price_total, b_pro_id, mirror_source) \
         VALUES ($1, 'R999999', '301', 'น้ำดื่ม', 'ขวด', 2, 20, 40, 17, 'ct')",
    )
    .bind(TEST_PK)
    .execute(&pool)
    .await
    .expect("pre-seed");

    // D-row shape: PK populated, all projected non-PK columns NULL.
    let row = HashMapRow::new("HT_Book_Pro")
        .with("id", MockValue::I32(TEST_PK))
        .with("B_NO", MockValue::Null)
        .with("B_ROOM", MockValue::Null)
        .with("B_NAME", MockValue::Null)
        .with("B_UNIT", MockValue::Null)
        .with("B_NUM", MockValue::Null)
        .with("B_PRICE", MockValue::Null)
        .with("B_PRICE_TOTAL", MockValue::Null)
        .with("B_PRO_ID", MockValue::Null);

    let mut tx = pool.begin().await.expect("begin tx");
    let event = BookProMirrorMapper
        .apply(&mut tx, ChangeOp::Delete, Some(&row))
        .await
        .expect("apply Delete on D-row shape must succeed");
    assert!(event.is_none(), "Delete returns no DomainEvent");
    tx.commit().await.expect("commit");

    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM legacy_mirror.ht_book_pro WHERE id = $1",
    )
    .bind(TEST_PK)
    .fetch_one(&pool)
    .await
    .expect("count");
    assert_eq!(count, 0, "D-event should have removed the mirror row");

    cleanup(&pool).await;
}

#[tokio::test]
async fn book_pro_mirror_apply_delete_is_idempotent_on_already_gone_row() {
    // CT can deliver the same Delete event twice (e.g. on watcher
    // restart mid-batch) — the second apply must not error.
    let _guard = BOOK_PRO_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup(&pool).await;

    let row = HashMapRow::new("HT_Book_Pro").with("id", MockValue::I32(TEST_PK));

    let mut tx = pool.begin().await.expect("begin tx");
    let event = BookProMirrorMapper
        .apply(&mut tx, ChangeOp::Delete, Some(&row))
        .await
        .expect("Delete on missing row must be idempotent");
    assert!(event.is_none());
    tx.commit().await.expect("commit");
}

#[tokio::test]
async fn book_pro_mirror_apply_update_upserts_over_existing_row() {
    // iHOTEL never UPDATEs HT_Book_Pro in place (edit = delete +
    // reinsert), but CT coalescing can surface an I-then-U pair as a
    // single U — the mapper must upsert either way.
    let _guard = BOOK_PRO_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup(&pool).await;

    let mut tx = pool.begin().await.expect("begin tx 1");
    BookProMirrorMapper
        .apply(&mut tx, ChangeOp::Insert, Some(&insert_row()))
        .await
        .expect("insert");
    tx.commit().await.expect("commit 1");

    let updated = insert_row()
        .with("B_NUM", MockValue::F64(3.0))
        .with("B_PRICE_TOTAL", MockValue::F64(60.0));
    let mut tx = pool.begin().await.expect("begin tx 2");
    BookProMirrorMapper
        .apply(&mut tx, ChangeOp::Update, Some(&updated))
        .await
        .expect("update");
    tx.commit().await.expect("commit 2");

    let (count, total): (i64, Option<f64>) = sqlx::query_as(
        "SELECT COUNT(*)::bigint, MAX(b_price_total) \
           FROM legacy_mirror.ht_book_pro WHERE id = $1",
    )
    .bind(TEST_PK)
    .fetch_one(&pool)
    .await
    .expect("count");
    assert_eq!(count, 1, "upsert must converge on one row");
    assert_eq!(total, Some(60.0), "U must overwrite the projected columns");

    cleanup(&pool).await;
}
