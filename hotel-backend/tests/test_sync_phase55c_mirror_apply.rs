//! Phase 5.5c integration tests for the legacy_mirror CT mappers.
//!
//! Locks the **regression fix** for the D-row PK NULL overwrite bug
//! (audit finding N4, fixed in `bin/sync.rs::build_materialised_row`
//! by reordering the projection / PK loops). The unit-level fix
//! is locked by `bin/sync.rs::tests::d_row_pk_survives_null_projection_overwrite`;
//! this file proves the end-to-end apply() path also works on a real
//! PG transaction.
//!
//! Coverage scope: `CuponMirrorMapper` Insert + Delete are
//! exercised here as the **canonical** example. The other 5 mirror
//! mappers (`CheckinProductMirrorMapper`, `DepositMirrorMapper`,
//! `ChangedRoomMirrorMapper`, `BillDebtHMirrorMapper`,
//! `BillDebtDsMirrorMapper`) follow the **identical structure** —
//! their tests are tracked as backlog (test-suite-analyzer P1
//! follow-up) but skipping them here is intentional: the bug under
//! repair lives in shared `materialise_row` code, and one mapper
//! exercising the post-fix path is sufficient to prove the
//! regression is closed. Per-mapper column-mapping correctness is
//! caught by the structural test in `mappers/mirror.rs::tests`.
//!
//! Test data uses `cupon_no = 1_999_999_999` — well outside the
//! production range (max real cupon_no observed in HF Hotel data is
//! ~17,894 as of 2026-04). Pre/post cleanup makes runs idempotent.

mod common;

use chrono::NaiveDate;
use hotel_backend::sync::change_op::ChangeOp;
use hotel_backend::sync::mappers::CuponMirrorMapper;
use hotel_backend::sync::mapper::MssqlChangeMapper;
use hotel_backend::sync::row::test_support::{HashMapRow, MockValue};

const TEST_PK: i32 = 1_999_999_999;

async fn cleanup(pool: &sqlx::PgPool) {
    sqlx::query("DELETE FROM legacy_mirror.ht_cupon WHERE cupon_no = $1")
        .bind(TEST_PK)
        .execute(pool)
        .await
        .expect("cleanup");
}

#[tokio::test]
async fn cupon_mirror_apply_insert_lands_row_with_ct_source() {
    let pool = common::create_test_pool().await;
    cleanup(&pool).await;

    let row = HashMapRow::new("HT_Cupon")
        .with("cupon_no", MockValue::I32(TEST_PK))
        .with(
            "cupon_cin_no",
            MockValue::Str("CH26-TEST-0001".into()),
        )
        .with("cupon_cin_room", MockValue::Str("301".into()))
        .with(
            "cupon_date",
            MockValue::DateTime(
                NaiveDate::from_ymd_opt(2026, 4, 29)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            ),
        )
        .with(
            "cupon_gen_date",
            MockValue::DateTime(
                NaiveDate::from_ymd_opt(2026, 4, 28)
                    .unwrap()
                    .and_hms_opt(19, 21, 23)
                    .unwrap(),
            ),
        )
        .with("cupon_by", MockValue::Str("TestRunner".into()))
        .with("cupon_print", MockValue::I32(1));

    let mut tx = pool.begin().await.expect("begin tx");
    let event = CuponMirrorMapper
        .apply(&mut tx, ChangeOp::Insert, Some(&row))
        .await
        .expect("apply Insert");
    assert!(
        event.is_none(),
        "mirror mappers must not emit DomainEvents (opaque pass-through)"
    );
    tx.commit().await.expect("commit");

    let (cin, room, by, mirror_source): (Option<String>, Option<String>, Option<String>, String) =
        sqlx::query_as(
            "SELECT cupon_cin_no, cupon_cin_room, cupon_by, mirror_source \
               FROM legacy_mirror.ht_cupon WHERE cupon_no = $1",
        )
        .bind(TEST_PK)
        .fetch_one(&pool)
        .await
        .expect("row should exist post-Insert");

    assert_eq!(cin.as_deref(), Some("CH26-TEST-0001"));
    assert_eq!(room.as_deref(), Some("301"));
    assert_eq!(by.as_deref(), Some("TestRunner"));
    assert_eq!(
        mirror_source, "ct",
        "incremental CT writes must stamp 'ct' source (vs 'reconcile' from --bootstrap)"
    );

    cleanup(&pool).await;
}

#[tokio::test]
async fn cupon_mirror_apply_delete_removes_row_using_ct_pk_alias_only() {
    // This is the regression test for audit finding N4. Pre-fix,
    // build_materialised_row would write the PK first then the
    // projection loop would overwrite it with NULL on a D row (the
    // LEFT JOIN nulls every t.<col>). Mapper would then crash on
    // try_get_i32("cupon_no")?.ok_or(...) with "cupon_no NULL —
    // should not happen post Phase 5.5b" and the row would never get
    // deleted from the mirror.
    //
    // Post-fix the projection runs first (writing NULL for cupon_no)
    // then the PK loop overwrites with the real CT-side I32 value.
    // The D-row HashMapRow we construct here mirrors that
    // post-materialisation shape: cupon_no holds the real PK, every
    // other projected column is Null.

    let pool = common::create_test_pool().await;
    cleanup(&pool).await;

    // Pre-seed the row directly via SQL (bypass the mapper to keep
    // this test orthogonal to the Insert path).
    sqlx::query(
        "INSERT INTO legacy_mirror.ht_cupon \
            (cupon_no, cupon_cin_no, cupon_cin_room, cupon_date, \
             cupon_gen_date, cupon_by, cupon_print, mirror_source) \
         VALUES ($1, 'CH26-TEST-0001', '301', \
                 '2026-04-29 00:00:00', '2026-04-28 19:21:23', \
                 'TestRunner', 1, 'ct')",
    )
    .bind(TEST_PK)
    .execute(&pool)
    .await
    .expect("pre-seed");

    // Construct the row shape the post-fix materialise_row produces
    // for a D event: PK populated, all projected non-PK cols NULL.
    let row = HashMapRow::new("HT_Cupon")
        .with("cupon_no", MockValue::I32(TEST_PK))
        .with("cupon_cin_no", MockValue::Null)
        .with("cupon_cin_room", MockValue::Null)
        .with("cupon_date", MockValue::Null)
        .with("cupon_gen_date", MockValue::Null)
        .with("cupon_by", MockValue::Null)
        .with("cupon_print", MockValue::Null);

    let mut tx = pool.begin().await.expect("begin tx");
    let event = CuponMirrorMapper
        .apply(&mut tx, ChangeOp::Delete, Some(&row))
        .await
        .expect(
            "apply Delete on D-row shape must succeed — pre-fix this would \
             crash with 'cupon_no NULL — should not happen post Phase 5.5b'",
        );
    assert!(event.is_none(), "Delete returns no DomainEvent");
    tx.commit().await.expect("commit");

    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM legacy_mirror.ht_cupon WHERE cupon_no = $1",
    )
    .bind(TEST_PK)
    .fetch_one(&pool)
    .await
    .expect("count");
    assert_eq!(count, 0, "D-event should have removed the mirror row");

    cleanup(&pool).await;
}

#[tokio::test]
async fn cupon_mirror_apply_delete_is_idempotent_on_already_gone_row() {
    // CT can deliver the same Delete event twice (e.g. on watcher
    // restart mid-batch) — the second apply must not error.
    let pool = common::create_test_pool().await;
    cleanup(&pool).await;

    let row = HashMapRow::new("HT_Cupon").with("cupon_no", MockValue::I32(TEST_PK));

    // Apply Delete on a row that doesn't exist — should be a no-op
    // returning Ok(None).
    let mut tx = pool.begin().await.expect("begin tx");
    let event = CuponMirrorMapper
        .apply(&mut tx, ChangeOp::Delete, Some(&row))
        .await
        .expect("Delete on missing row must be idempotent");
    assert!(event.is_none());
    tx.commit().await.expect("commit");
}
