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
//!
//! ## Echo-before-stamp adoption (issue #266, 2026-07-28)
//!
//! The second half of this file covers the `ht_pos_sales` /
//! `ht_room_changes` **echo-before-stamp adoption** arms added in
//! `7b57edc`. Those shipped with SQL *shape* pins only
//! (`mappers/mirror.rs::tests` asserts `.contains(...)` on the constants);
//! nothing executed them against a real PG transaction. These tests close
//! that gap by driving the same `apply()` entry point the CT watcher uses.
//!
//! Fixtures are built to the EXACT wire format
//! `writeback/recipes/pos_sale.rs` / `room_change.rs` emit, because the
//! adoption predicate IS that wire format expressed in canonical space:
//! `{:.3}` qty / `{:.2}` money → `ROUND(…, 3)` / `ROUND(…, 2)`, and
//! `format_legacy_datetime` → Bangkok wall-clock truncated to the second
//! → `date_trunc('second', … AT TIME ZONE 'Asia/Bangkok')`. The canonical
//! rows are deliberately seeded with sub-second precision so the
//! truncation is load-bearing rather than incidental.
//!
//! Every adoption test owns a disjoint fixture slot (its own customer,
//! rooms, check-in, product and legacy id) so no lock is needed — the
//! `tests/common/mod.rs` "exact-match markers, never wildcards" rule.

mod common;

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use hotel_backend::sync::change_op::ChangeOp;
use hotel_backend::sync::mapper::MssqlChangeMapper;
use hotel_backend::sync::mappers::{
    ChangedRoomMirrorMapper, CheckinProductMirrorMapper, CuponMirrorMapper,
};
use hotel_backend::sync::row::test_support::{HashMapRow, MockValue};

const TEST_PK: i32 = 1_999_999_999;

/// All tests share `TEST_PK` and cargo runs them in parallel — one
/// shared lock keeps the shared-fixture mutations deterministic
/// (2026-06-11, same pattern as `test_sync_phase55_bootstrap`).
static CUPON_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

async fn cleanup(pool: &sqlx::PgPool) {
    sqlx::query("DELETE FROM legacy_mirror.ht_cupon WHERE cupon_no = $1")
        .bind(TEST_PK)
        .execute(pool)
        .await
        .expect("cleanup");
    // The G5 dual-write also lands a canonical `ht_coupons` row, and
    // the Delete arm deliberately ORPHANS it (clears legacy_cupon_no,
    // keeps the row + its synthetic code). Sweep both shapes so reruns
    // against a persistent dev DB stay idempotent (2026-06-11: a
    // leftover orphan made the insert test fail deterministically on
    // ht_coupons_coupon_code_key).
    sqlx::query("DELETE FROM ht_coupons WHERE legacy_cupon_no = $1 OR coupon_code = $2")
        .bind(TEST_PK)
        .bind(format!("LEGACY-{TEST_PK}"))
        .execute(pool)
        .await
        .expect("cleanup canonical");
}

#[tokio::test]
async fn cupon_mirror_apply_insert_lands_row_with_ct_source() {
    let _guard = CUPON_LOCK.lock().await;
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

    let _guard = CUPON_LOCK.lock().await;
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
    let _guard = CUPON_LOCK.lock().await;
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

/// 2026-06-11 — legacy-id-reuse poison pill (same class as the v2.66.3
/// `ht_room_calendar` rebind fix). iHOTEL allocates `cupon_no = MAX+1`,
/// so after a coupon DELETE the next issue REUSES the id. The canonical
/// Delete arm orphans the old row (clears `legacy_cupon_no`, keeps the
/// synthetic `coupon_code`), so pre-fix the re-insert missed the
/// ON CONFLICT (legacy_cupon_no) arbiter and errored on
/// `ht_coupons_coupon_code_key` — every retry, holding the watermark.
/// Post-fix the orphan is re-attached and the apply succeeds.
#[tokio::test]
async fn cupon_canonical_insert_after_delete_reuses_legacy_id_without_error() {
    let _guard = CUPON_LOCK.lock().await;
    let pool = common::create_test_pool().await;
    cleanup(&pool).await;

    let row = HashMapRow::new("HT_Cupon")
        .with("cupon_no", MockValue::I32(TEST_PK))
        .with("cupon_cin_no", MockValue::Str("CH26-TEST-0002".into()))
        .with("cupon_cin_room", MockValue::Str("302".into()))
        .with(
            "cupon_date",
            MockValue::DateTime(
                NaiveDate::from_ymd_opt(2026, 6, 11)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
            ),
        )
        .with(
            "cupon_gen_date",
            MockValue::DateTime(
                NaiveDate::from_ymd_opt(2026, 6, 11)
                    .unwrap()
                    .and_hms_opt(9, 0, 0)
                    .unwrap(),
            ),
        )
        .with("cupon_by", MockValue::Str("TestRunner".into()))
        .with("cupon_print", MockValue::I32(0));

    // Issue → canonical row lands with the back-pointer.
    let mut tx = pool.begin().await.expect("begin tx 1");
    CuponMirrorMapper
        .apply(&mut tx, ChangeOp::Insert, Some(&row))
        .await
        .expect("first insert");
    tx.commit().await.expect("commit 1");

    // iHOTEL deletes the coupon → canonical row orphaned (pointer NULL,
    // code kept for the audit trail). Row shape mirrors the post-N4-fix
    // materialised D row: PK populated, projected columns NULL.
    let del = HashMapRow::new("HT_Cupon")
        .with("cupon_no", MockValue::I32(TEST_PK))
        .with("cupon_cin_no", MockValue::Null)
        .with("cupon_cin_room", MockValue::Null)
        .with("cupon_date", MockValue::Null)
        .with("cupon_gen_date", MockValue::Null)
        .with("cupon_by", MockValue::Null)
        .with("cupon_print", MockValue::Null);
    let mut tx = pool.begin().await.expect("begin tx 2");
    CuponMirrorMapper
        .apply(&mut tx, ChangeOp::Delete, Some(&del))
        .await
        .expect("delete");
    tx.commit().await.expect("commit 2");

    // MAX+1 reuses the id → the re-insert must re-attach the orphan,
    // not error on the coupon_code unique.
    let mut tx = pool.begin().await.expect("begin tx 3");
    CuponMirrorMapper
        .apply(&mut tx, ChangeOp::Insert, Some(&row))
        .await
        .expect("re-insert with reused legacy id must succeed (pre-fix: duplicate key on ht_coupons_coupon_code_key)");
    tx.commit().await.expect("commit 3");

    // Exactly one canonical row, re-attached.
    let (count, attached): (i64, i64) = sqlx::query_as(
        "SELECT COUNT(*)::bigint, COUNT(legacy_cupon_no)::bigint \
           FROM ht_coupons WHERE coupon_code = $1",
    )
    .bind(format!("LEGACY-{TEST_PK}"))
    .fetch_one(&pool)
    .await
    .expect("count");
    assert_eq!(count, 1, "reuse must converge on ONE canonical row");
    assert_eq!(attached, 1, "the orphan must be re-attached to the reused legacy id");

    cleanup(&pool).await;
}

// ═══════════════════════════════════════════════════════════════════
// Echo-before-stamp adoption — issue #266
// ═══════════════════════════════════════════════════════════════════
//
// The race these tests reproduce (see
// `mappers/mirror.rs::ADOPT_UNSTAMPED_POS_SALE_SQL` for the full
// narrative):
//
//   t0  service commits the canonical row, `*_legacy_id IS NULL`,
//       and enqueues the writeback intent
//   t1  writeback worker INSERTs the legacy row, MSSQL commits
//   t2  ← CT tick lands HERE: the echo of our own write arrives
//   t3  worker back-populates `*_legacy_id` on a SEPARATE PG connection
//
// A CT tick at t2 sees an unstamped canonical row, matches nothing on
// the back-link, and (pre-fix) INSERTs a phantom duplicate. The adoption
// UPDATE claims the matching unstamped row first so the dedup UPDATE
// that follows hits it and the INSERT never runs.
//
// Every test below simulates t2 by seeding the unstamped canonical row
// and then applying the CT event WITHOUT ever running the t3
// back-population.

/// Legacy ids for the adoption fixtures. Both `HT_CheckIn_Product.id`
/// and `HT_Changed_Room.id` are `int IDENTITY`; production values are in
/// the tens of thousands, so 1.99e9 can never collide.
const ADOPT_LEGACY_ID_BASE: i32 = 1_999_990_000;

/// Wire values a POS sale carries, matching `recipes/pos_sale.rs`:
/// `Cin_Pro_num` = `{:.3}` of qty, `Cin_Pro_price` = `{:.2}` of unit price.
const POS_QTY: f64 = 1.5;
const POS_UNIT_PRICE: f64 = 25.0;
const POS_NOTE: &str = "minibar";
const POS_PRODUCT_NAME: &str = "Coca-Cola 330ml";
const POS_PRODUCT_UNIT: &str = "ขวด";

/// Wire values a room change carries, matching `recipes/room_change.rs`:
/// `room_before_price` = `{:.2}`, `Note` / `ToPrice` verbatim.
const RC_BEFORE_PRICE: f64 = 890.0;
const RC_REASON: &str = "guest requested a quieter room";
const RC_TO_PRICE: &str = "950";

fn cust_marker(slot: &str) -> String {
    format!("ZTECHO-{slot}")
}
fn cin_marker(slot: &str) -> String {
    format!("ZTECHO-{slot}")
}
fn legacy_cin_marker(slot: &str) -> String {
    format!("CHZT-{slot}")
}
fn prod_marker(slot: &str) -> String {
    format!("ZPRO-{slot}")
}
fn room_marker(slot: &str, tag: char) -> String {
    format!("Z{slot}{tag}")
}

/// The Bangkok wall-clock second the writeback recipe puts on the wire.
/// `format_legacy_datetime` renders `M/D/YYYY h:mm:ss tt` — no sub-second
/// component survives, which is exactly why the adoption predicate
/// compares `date_trunc('second', …)`.
fn pos_wire_second(offset_secs: i64) -> NaiveDateTime {
    NaiveDate::from_ymd_opt(2026, 7, 28)
        .unwrap()
        .and_hms_opt(14, 23, 45)
        .unwrap()
        + chrono::Duration::seconds(offset_secs)
}

/// The canonical `sale_sold_at` instant our app stored — the same
/// Bangkok second as [`pos_wire_second`] but WITH microseconds, as a real
/// `NOW()`-defaulted row has. 14:23:45.123456 +07 == 07:23:45.123456 UTC.
fn pos_sold_at_utc(offset_secs: i64) -> DateTime<Utc> {
    (NaiveDate::from_ymd_opt(2026, 7, 28)
        .unwrap()
        .and_hms_micro_opt(7, 23, 45, 123_456)
        .unwrap()
        + chrono::Duration::seconds(offset_secs))
    .and_utc()
}

fn rc_wire_second() -> NaiveDateTime {
    NaiveDate::from_ymd_opt(2026, 7, 28)
        .unwrap()
        .and_hms_opt(16, 5, 7)
        .unwrap()
}

fn rc_changed_at_utc() -> DateTime<Utc> {
    NaiveDate::from_ymd_opt(2026, 7, 28)
        .unwrap()
        .and_hms_micro_opt(9, 5, 7, 987_654)
        .unwrap()
        .and_utc()
}

/// The instant the mapper's dedup UPDATE converges a row onto: the wire
/// second re-interpreted as Bangkok wall-clock, i.e. the seeded instant
/// with its microseconds dropped.
fn converged_utc(wire: NaiveDateTime) -> DateTime<Utc> {
    (wire - chrono::Duration::hours(7)).and_utc()
}

/// One disjoint fixture slot: a customer, three rooms, an active
/// check-in carrying `legacy_cin_no`, and a product carrying
/// `prod_legacy_no` — everything the two `upsert_canonical_*` FK
/// resolvers need to succeed.
struct EchoFixture {
    slot: &'static str,
    legacy_id: i32,
    cin_id: i32,
    prod_id: i64,
    room_a: i32,
    room_b: i32,
    room_c: i32,
}

async fn cleanup_echo_fixture(pool: &sqlx::PgPool, slot: &str, legacy_id: i32) {
    // Children first. The `OR *_legacy_id = $1` arm sweeps rows left by an
    // aborted run whose parent check-in is already gone — a stale row
    // carrying the test legacy id would defeat the `NOT EXISTS` guard and
    // make the adoption tests fail for the wrong reason.
    sqlx::query(
        "DELETE FROM ht_pos_sales WHERE sale_legacy_id = $1 \
            OR sale_cin_id IN (SELECT cin_id FROM ht_checkins WHERE cin_no = $2)",
    )
    .bind(legacy_id)
    .bind(cin_marker(slot))
    .execute(pool)
    .await
    .expect("cleanup ht_pos_sales");

    sqlx::query(
        "DELETE FROM ht_room_changes WHERE rc_legacy_id = $1 \
            OR rc_cin_id IN (SELECT cin_id FROM ht_checkins WHERE cin_no = $2)",
    )
    .bind(legacy_id)
    .bind(cin_marker(slot))
    .execute(pool)
    .await
    .expect("cleanup ht_room_changes");

    sqlx::query("DELETE FROM legacy_mirror.ht_checkin_product WHERE id = $1")
        .bind(legacy_id)
        .execute(pool)
        .await
        .expect("cleanup mirror checkin_product");

    sqlx::query("DELETE FROM legacy_mirror.ht_changed_room WHERE id = $1")
        .bind(legacy_id)
        .execute(pool)
        .await
        .expect("cleanup mirror changed_room");

    sqlx::query("DELETE FROM ht_checkins WHERE cin_no = $1")
        .bind(cin_marker(slot))
        .execute(pool)
        .await
        .expect("cleanup ht_checkins");

    sqlx::query("DELETE FROM ht_products WHERE prod_legacy_no = $1")
        .bind(prod_marker(slot))
        .execute(pool)
        .await
        .expect("cleanup ht_products");

    sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = ANY($1)")
        .bind(vec![
            room_marker(slot, 'A'),
            room_marker(slot, 'B'),
            room_marker(slot, 'C'),
        ])
        .execute(pool)
        .await
        .expect("cleanup ht_rooms_new");

    sqlx::query("DELETE FROM ht_customers WHERE cust_firstname = $1")
        .bind(cust_marker(slot))
        .execute(pool)
        .await
        .expect("cleanup ht_customers");
}

async fn seed_echo_fixture(
    pool: &sqlx::PgPool,
    slot: &'static str,
    legacy_id: i32,
) -> EchoFixture {
    cleanup_echo_fixture(pool, slot, legacy_id).await;

    let cust_id: i32 =
        sqlx::query_scalar("INSERT INTO ht_customers (cust_firstname) VALUES ($1) RETURNING cust_id")
            .bind(cust_marker(slot))
            .fetch_one(pool)
            .await
            .expect("seed customer");

    let mut rooms = Vec::with_capacity(3);
    for tag in ['A', 'B', 'C'] {
        let room_id: i32 = sqlx::query_scalar(
            "INSERT INTO ht_rooms_new (room_no, room_status) \
             VALUES ($1, 'available') RETURNING room_id",
        )
        .bind(room_marker(slot, tag))
        .fetch_one(pool)
        .await
        .expect("seed room");
        rooms.push(room_id);
    }

    let cin_id: i32 = sqlx::query_scalar(
        "INSERT INTO ht_checkins \
            (cin_no, cin_cust_id, cin_room_id, cin_checkin_time, \
             cin_expected_checkout, cin_status, legacy_cin_no) \
         VALUES ($1, $2, $3, NOW(), CURRENT_DATE + 1, 'active', $4) \
         RETURNING cin_id",
    )
    .bind(cin_marker(slot))
    .bind(cust_id)
    .bind(rooms[0])
    .bind(legacy_cin_marker(slot))
    .fetch_one(pool)
    .await
    .expect("seed checkin");

    let prod_id: i64 = sqlx::query_scalar(
        "INSERT INTO ht_products (prod_legacy_no, prod_name, prod_unit, prod_price) \
         VALUES ($1, $2, $3, $4::numeric) RETURNING prod_id",
    )
    .bind(prod_marker(slot))
    .bind(POS_PRODUCT_NAME)
    .bind(POS_PRODUCT_UNIT)
    .bind(POS_UNIT_PRICE)
    .fetch_one(pool)
    .await
    .expect("seed product");

    EchoFixture {
        slot,
        legacy_id,
        cin_id,
        prod_id,
        room_a: rooms[0],
        room_b: rooms[1],
        room_c: rooms[2],
    }
}

/// Seed the unstamped, app-originated `ht_pos_sales` row that exists in
/// the t0→t3 window. Mirrors what `service::pos` commits: `source =
/// 'canonical'`, `sale_legacy_id IS NULL`, sub-second `sale_sold_at`.
async fn seed_unstamped_pos_sale(
    pool: &sqlx::PgPool,
    f: &EchoFixture,
    qty: f64,
    sold_at: DateTime<Utc>,
) -> i64 {
    sqlx::query_scalar(
        "INSERT INTO ht_pos_sales \
            (sale_cin_id, sale_product_id, sale_qty, sale_unit_price, \
             sale_sold_at, sale_note, source, aggregate_id) \
         VALUES ($1, $2, $3::numeric, $4::numeric, $5, $6, 'canonical', gen_random_uuid()) \
         RETURNING sale_id",
    )
    .bind(f.cin_id)
    .bind(f.prod_id)
    .bind(qty)
    .bind(POS_UNIT_PRICE)
    .bind(sold_at)
    .bind(POS_NOTE)
    .fetch_one(pool)
    .await
    .expect("seed unstamped pos sale")
}

/// The CT projection row for the legacy twin of that sale — every column
/// `CHECKIN_PRODUCT_SELECT_COLS` projects, carrying the values
/// `recipes/pos_sale.rs::build_insert_statement` put on the wire.
fn pos_ct_row(f: &EchoFixture, wire_second: NaiveDateTime, qty: f64) -> HashMapRow {
    HashMapRow::new("HT_CheckIn_Product")
        .with("id", MockValue::I32(f.legacy_id))
        .with("Cin_No", MockValue::Str(legacy_cin_marker(f.slot)))
        .with("Cin_Room_no", MockValue::Str(room_marker(f.slot, 'A')))
        .with("Cin_Ds_date", MockValue::DateTime(wire_second))
        .with("Cin_Pro_id", MockValue::Str(prod_marker(f.slot)))
        .with("Cin_Pro_name", MockValue::Str(POS_PRODUCT_NAME.into()))
        .with("Cin_Pro_Unit", MockValue::Str(POS_PRODUCT_UNIT.into()))
        .with("Cin_Pro_num", MockValue::F64(qty))
        .with("Cin_Pro_price", MockValue::F64(POS_UNIT_PRICE))
        .with("Cin_Pro_priceTotal", MockValue::F64(qty * POS_UNIT_PRICE))
        // Posted-to-folio lines are unpaid until round-bill — the recipe
        // pegs Cin_Pro_pay at 0.00.
        .with("Cin_Pro_pay", MockValue::F64(0.0))
        .with("Cin_Pro_note", MockValue::Str(POS_NOTE.into()))
}

async fn apply_pos_ct_event(pool: &sqlx::PgPool, row: &HashMapRow) {
    let mut tx = pool.begin().await.expect("begin tx");
    let event = CheckinProductMirrorMapper
        .apply(&mut tx, ChangeOp::Insert, Some(row))
        .await
        .expect("apply HT_CheckIn_Product Insert");
    assert!(event.is_none(), "mirror mappers emit no DomainEvent");
    tx.commit().await.expect("commit");
}

/// Seed the unstamped, app-originated `ht_room_changes` row.
/// `ht_room_changes` has no `source` column — `rc_legacy_id IS NULL` is
/// the whole ownership guard, because this mapper stamps every row it
/// inserts.
async fn seed_unstamped_room_change(
    pool: &sqlx::PgPool,
    f: &EchoFixture,
    to_room_id: i32,
) -> i64 {
    sqlx::query_scalar(
        "INSERT INTO ht_room_changes \
            (rc_cin_id, rc_from_room_id, rc_to_room_id, rc_reason, \
             rc_changed_at, rc_room_before_price, rc_to_price) \
         VALUES ($1, $2, $3, $4, $5, $6::numeric, $7) \
         RETURNING rc_id",
    )
    .bind(f.cin_id)
    .bind(f.room_a)
    .bind(to_room_id)
    .bind(RC_REASON)
    .bind(rc_changed_at_utc())
    .bind(RC_BEFORE_PRICE)
    .bind(RC_TO_PRICE)
    .fetch_one(pool)
    .await
    .expect("seed unstamped room change")
}

fn rc_ct_row(f: &EchoFixture, to_tag: char) -> HashMapRow {
    HashMapRow::new("HT_Changed_Room")
        .with("id", MockValue::I32(f.legacy_id))
        .with("cin_no", MockValue::Str(legacy_cin_marker(f.slot)))
        .with("room_before", MockValue::Str(room_marker(f.slot, 'A')))
        .with("room_after", MockValue::Str(room_marker(f.slot, to_tag)))
        .with("change_date", MockValue::DateTime(rc_wire_second()))
        .with("room_before_price", MockValue::F64(RC_BEFORE_PRICE))
        .with("Note", MockValue::Str(RC_REASON.into()))
        .with("ToPrice", MockValue::Str(RC_TO_PRICE.into()))
}

async fn apply_rc_ct_event(pool: &sqlx::PgPool, row: &HashMapRow) {
    let mut tx = pool.begin().await.expect("begin tx");
    let event = ChangedRoomMirrorMapper
        .apply(&mut tx, ChangeOp::Insert, Some(row))
        .await
        .expect("apply HT_Changed_Room Insert");
    assert!(event.is_none(), "mirror mappers emit no DomainEvent");
    tx.commit().await.expect("commit");
}

async fn pos_rows(pool: &sqlx::PgPool, cin_id: i32) -> Vec<(i64, Option<i32>, String, f64, DateTime<Utc>)> {
    sqlx::query_as(
        "SELECT sale_id, sale_legacy_id, source, sale_qty::float8, sale_sold_at \
           FROM ht_pos_sales WHERE sale_cin_id = $1 ORDER BY sale_id",
    )
    .bind(cin_id)
    .fetch_all(pool)
    .await
    .expect("read back ht_pos_sales")
}

async fn rc_rows(pool: &sqlx::PgPool, cin_id: i32) -> Vec<(i64, Option<i32>, i32, f64, DateTime<Utc>)> {
    sqlx::query_as(
        "SELECT rc_id, rc_legacy_id, rc_to_room_id, rc_room_before_price::float8, rc_changed_at \
           FROM ht_room_changes WHERE rc_cin_id = $1 ORDER BY rc_id",
    )
    .bind(cin_id)
    .fetch_all(pool)
    .await
    .expect("read back ht_room_changes")
}

// ─── 1. The race ─────────────────────────────────────────────────────

/// **The race, POS side.** An unstamped app-originated sale exists; the
/// CT echo of its own legacy twin arrives before back-population. Post-fix
/// the mapper ADOPTS that row: one row, stamped, converged.
///
/// Pre-fix (adoption UPDATE removed) this test fails with 2 rows — see the
/// red-capability note on issue #266.
#[tokio::test]
async fn pos_sale_echo_before_stamp_adopts_unstamped_row_instead_of_duplicating() {
    let pool = common::create_test_pool().await;
    let f = seed_echo_fixture(&pool, "p1", ADOPT_LEGACY_ID_BASE + 1).await;

    let seeded_id = seed_unstamped_pos_sale(&pool, &f, POS_QTY, pos_sold_at_utc(0)).await;
    apply_pos_ct_event(&pool, &pos_ct_row(&f, pos_wire_second(0), POS_QTY)).await;

    let rows = pos_rows(&pool, f.cin_id).await;
    assert_eq!(
        rows.len(),
        1,
        "the CT echo must ADOPT the unstamped row, not insert a phantom \
         duplicate — got {rows:?}"
    );
    let (sale_id, legacy_id, source, qty, sold_at) = rows[0].clone();
    assert_eq!(
        sale_id, seeded_id,
        "adoption must claim the EXISTING row (same sale_id), not replace it"
    );
    assert_eq!(legacy_id, Some(f.legacy_id), "row must end up stamped");
    assert_eq!(
        source, "legacy",
        "after adoption the dedup UPDATE converges the row to the same \
         state the non-racing order (back-populate first) produces"
    );
    assert!((qty - POS_QTY).abs() < 1e-9, "qty must survive adoption");
    assert_eq!(
        sold_at,
        converged_utc(pos_wire_second(0)),
        "content converges on the legacy wire second (sub-second dropped)"
    );

    cleanup_echo_fixture(&pool, f.slot, f.legacy_id).await;
}

/// **The race, room-change side.**
#[tokio::test]
async fn room_change_echo_before_stamp_adopts_unstamped_row_instead_of_duplicating() {
    let pool = common::create_test_pool().await;
    let f = seed_echo_fixture(&pool, "r1", ADOPT_LEGACY_ID_BASE + 11).await;

    let seeded_id = seed_unstamped_room_change(&pool, &f, f.room_b).await;
    apply_rc_ct_event(&pool, &rc_ct_row(&f, 'B')).await;

    let rows = rc_rows(&pool, f.cin_id).await;
    assert_eq!(
        rows.len(),
        1,
        "the CT echo must ADOPT the unstamped room change, not insert a \
         phantom duplicate — got {rows:?}"
    );
    let (rc_id, legacy_id, to_room_id, price, changed_at) = rows[0].clone();
    assert_eq!(rc_id, seeded_id, "adoption must claim the EXISTING row");
    assert_eq!(legacy_id, Some(f.legacy_id), "row must end up stamped");
    assert_eq!(to_room_id, f.room_b);
    assert!((price - RC_BEFORE_PRICE).abs() < 1e-9);
    assert_eq!(
        changed_at,
        converged_utc(rc_wire_second()),
        "content converges on the legacy wire second (sub-second dropped)"
    );

    cleanup_echo_fixture(&pool, f.slot, f.legacy_id).await;
}

// ─── 2. No hijack ────────────────────────────────────────────────────

/// **No hijack, POS side.** The unstamped row differs from the CT event by
/// exactly ONE second — the subtlest near-miss the natural key has to
/// reject, and the one a `sale_sold_at = $7` (no `date_trunc`)
/// implementation would get wrong in both directions. Adoption must NOT
/// fire; the legacy-origin INSERT runs and we end with two rows.
///
/// qty / unit_price / note / product / folio are identical here, so a
/// failure isolates the timestamp arm of the key. The other arms are
/// pinned by the SQL-shape unit tests in `mappers/mirror.rs::tests`.
#[tokio::test]
async fn pos_sale_adoption_never_hijacks_a_row_one_second_off() {
    let pool = common::create_test_pool().await;
    let f = seed_echo_fixture(&pool, "p2", ADOPT_LEGACY_ID_BASE + 2).await;

    // Our app's row is one second LATER than the sale the echo describes.
    let decoy_sold_at = pos_sold_at_utc(1);
    let decoy_id = seed_unstamped_pos_sale(&pool, &f, POS_QTY, decoy_sold_at).await;
    apply_pos_ct_event(&pool, &pos_ct_row(&f, pos_wire_second(0), POS_QTY)).await;

    let rows = pos_rows(&pool, f.cin_id).await;
    assert_eq!(
        rows.len(),
        2,
        "a one-second-off unstamped row is a DIFFERENT sale — the echo must \
         insert its own legacy-origin row, got {rows:?}"
    );

    let decoy = rows
        .iter()
        .find(|r| r.0 == decoy_id)
        .expect("the decoy row must still exist");
    assert_eq!(
        decoy.1, None,
        "the non-matching unstamped row must NOT be hijacked"
    );
    assert_eq!(
        decoy.2, "canonical",
        "the decoy is still awaiting its own back-population"
    );
    assert_eq!(
        decoy.4, decoy_sold_at,
        "the decoy's sub-second timestamp must be left byte-identical"
    );

    let adopted = rows
        .iter()
        .find(|r| r.0 != decoy_id)
        .expect("a legacy-origin row must have been inserted");
    assert_eq!(adopted.1, Some(f.legacy_id));
    assert_eq!(adopted.2, "legacy");

    cleanup_echo_fixture(&pool, f.slot, f.legacy_id).await;
}

/// **No hijack, room-change side.** Same folio, same second, same price /
/// reason / to-price — but the guest moved to a DIFFERENT destination
/// room. The room-pair is the structural half of the natural key.
#[tokio::test]
async fn room_change_adoption_never_hijacks_a_different_room_pair() {
    let pool = common::create_test_pool().await;
    let f = seed_echo_fixture(&pool, "r2", ADOPT_LEGACY_ID_BASE + 12).await;

    // Our app recorded A→C; the echo describes A→B.
    let decoy_id = seed_unstamped_room_change(&pool, &f, f.room_c).await;
    apply_rc_ct_event(&pool, &rc_ct_row(&f, 'B')).await;

    let rows = rc_rows(&pool, f.cin_id).await;
    assert_eq!(
        rows.len(),
        2,
        "a different room-pair is a DIFFERENT move — the echo must insert \
         its own legacy-origin row, got {rows:?}"
    );

    let decoy = rows
        .iter()
        .find(|r| r.0 == decoy_id)
        .expect("the decoy row must still exist");
    assert_eq!(
        decoy.1, None,
        "the non-matching unstamped row must NOT be hijacked"
    );
    assert_eq!(decoy.2, f.room_c, "the decoy's destination room must not move");
    assert_eq!(
        decoy.4,
        rc_changed_at_utc(),
        "the decoy's sub-second timestamp must be left byte-identical"
    );

    let adopted = rows
        .iter()
        .find(|r| r.0 != decoy_id)
        .expect("a legacy-origin row must have been inserted");
    assert_eq!(adopted.1, Some(f.legacy_id));
    assert_eq!(adopted.2, f.room_b);

    cleanup_echo_fixture(&pool, f.slot, f.legacy_id).await;
}

// ─── 3. Once per legacy id ───────────────────────────────────────────

/// **CT re-delivery, POS side.** CT can hand the same row over twice
/// (watcher restart mid-batch, or an iHOTEL-side U-event on the same row).
/// After adoption the second apply must fall straight through to the plain
/// dedup UPDATE — no second claim, no new row.
#[tokio::test]
async fn pos_sale_adoption_is_once_per_legacy_id_on_ct_redelivery() {
    let pool = common::create_test_pool().await;
    let f = seed_echo_fixture(&pool, "p3", ADOPT_LEGACY_ID_BASE + 3).await;

    let seeded_id = seed_unstamped_pos_sale(&pool, &f, POS_QTY, pos_sold_at_utc(0)).await;
    let row = pos_ct_row(&f, pos_wire_second(0), POS_QTY);
    apply_pos_ct_event(&pool, &row).await;
    apply_pos_ct_event(&pool, &row).await;

    let rows = pos_rows(&pool, f.cin_id).await;
    assert_eq!(
        rows.len(),
        1,
        "re-delivery must be a no-op on the dedup path, got {rows:?}"
    );
    assert_eq!(rows[0].0, seeded_id);
    assert_eq!(rows[0].1, Some(f.legacy_id));
    assert_eq!(rows[0].2, "legacy");

    cleanup_echo_fixture(&pool, f.slot, f.legacy_id).await;
}

/// **CT re-delivery, room-change side.**
#[tokio::test]
async fn room_change_adoption_is_once_per_legacy_id_on_ct_redelivery() {
    let pool = common::create_test_pool().await;
    let f = seed_echo_fixture(&pool, "r3", ADOPT_LEGACY_ID_BASE + 13).await;

    let seeded_id = seed_unstamped_room_change(&pool, &f, f.room_b).await;
    let row = rc_ct_row(&f, 'B');
    apply_rc_ct_event(&pool, &row).await;
    apply_rc_ct_event(&pool, &row).await;

    let rows = rc_rows(&pool, f.cin_id).await;
    assert_eq!(
        rows.len(),
        1,
        "re-delivery must be a no-op on the dedup path, got {rows:?}"
    );
    assert_eq!(rows[0].0, seeded_id);
    assert_eq!(rows[0].1, Some(f.legacy_id));

    cleanup_echo_fixture(&pool, f.slot, f.legacy_id).await;
}

/// **The `NOT EXISTS` guard, POS side.** Once a legacy id is carried by
/// any canonical row, no SECOND row may ever be claimed for it — even one
/// whose natural key matches perfectly (two identical minibar lines rung
/// up in the same second have two distinct legacy ids; each must adopt its
/// own row). Without the guard, a re-delivered event would stamp a
/// duplicate legacy id and trip `ht_pos_sales_legacy_id_uq`.
#[tokio::test]
async fn pos_sale_adoption_guard_leaves_a_second_identical_unstamped_row_alone() {
    let pool = common::create_test_pool().await;
    let f = seed_echo_fixture(&pool, "p4", ADOPT_LEGACY_ID_BASE + 4).await;

    let first_id = seed_unstamped_pos_sale(&pool, &f, POS_QTY, pos_sold_at_utc(0)).await;
    let row = pos_ct_row(&f, pos_wire_second(0), POS_QTY);
    apply_pos_ct_event(&pool, &row).await;

    // A second, byte-identical unstamped row shows up (its own legacy twin
    // is still in flight) and CT re-delivers the FIRST event.
    let second_id = seed_unstamped_pos_sale(&pool, &f, POS_QTY, pos_sold_at_utc(0)).await;
    apply_pos_ct_event(&pool, &row).await;

    let rows = pos_rows(&pool, f.cin_id).await;
    assert_eq!(rows.len(), 2, "no third row may appear, got {rows:?}");

    let first = rows.iter().find(|r| r.0 == first_id).expect("first row");
    assert_eq!(first.1, Some(f.legacy_id), "the original adoption stands");

    let second = rows.iter().find(|r| r.0 == second_id).expect("second row");
    assert_eq!(
        second.1, None,
        "NOT EXISTS must stop a second claim on the same legacy id"
    );
    assert_eq!(second.2, "canonical");

    cleanup_echo_fixture(&pool, f.slot, f.legacy_id).await;
}

/// **The `NOT EXISTS` guard, room-change side.**
#[tokio::test]
async fn room_change_adoption_guard_leaves_a_second_identical_unstamped_row_alone() {
    let pool = common::create_test_pool().await;
    let f = seed_echo_fixture(&pool, "r4", ADOPT_LEGACY_ID_BASE + 14).await;

    let first_id = seed_unstamped_room_change(&pool, &f, f.room_b).await;
    let row = rc_ct_row(&f, 'B');
    apply_rc_ct_event(&pool, &row).await;

    let second_id = seed_unstamped_room_change(&pool, &f, f.room_b).await;
    apply_rc_ct_event(&pool, &row).await;

    let rows = rc_rows(&pool, f.cin_id).await;
    assert_eq!(rows.len(), 2, "no third row may appear, got {rows:?}");

    let first = rows.iter().find(|r| r.0 == first_id).expect("first row");
    assert_eq!(first.1, Some(f.legacy_id), "the original adoption stands");

    let second = rows.iter().find(|r| r.0 == second_id).expect("second row");
    assert_eq!(
        second.1, None,
        "NOT EXISTS must stop a second claim on the same legacy id"
    );

    cleanup_echo_fixture(&pool, f.slot, f.legacy_id).await;
}
