//! Phase 5.2 integration tests for the customer + room CT mappers.
//!
//! Boots a live PG (via `DATABASE_URL`, see `tests/common/mod.rs`),
//! drives the mapper directly with `HashMapRow` fixtures, and asserts
//! that the canonical PG row + `event_log` content match expectations.
//!
//! Skipped silently when `DATABASE_URL` is unreachable — the unit tests
//! in `sync::mappers::*` cover the pure logic; this suite covers the
//! UPSERT + idempotency + event-persistence loop.

mod common;

use hotel_backend::sync::change_op::ChangeOp;
use hotel_backend::sync::mapper::MssqlChangeMapper;
use hotel_backend::sync::mappers::{CustomerMapper, RoomMasterMapper};
use hotel_backend::sync::row::test_support::{HashMapRow, MockValue};

/// Helper — generate a unique-ish Cust_no so re-runs don't clash with
/// each other (the mapper UPSERTs by `legacy_cust_no` so re-using the
/// same value across runs would just re-update the same row).
/// Process-unique residue (2026-06-11): bare `nanos % N` slices collide
/// for parallel tests in the same residue window; atomic counter + pid
/// make this unique per process and de-correlated across runs.
fn unique_residue() -> u32 {
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u32;
    nanos
        .wrapping_add(std::process::id())
        .wrapping_add(seq.wrapping_mul(7919))
}

fn unique_cust_no() -> String {
    // Truncate to 6 digits — Cust_no in legacy fits 'C\d+' anyway.
    format!("CT{:06}", unique_residue() % 1_000_000)
}

fn customer_row_full(cust_no: &str, name: &str, phone: &str) -> HashMapRow {
    // Track E2 (T1 HIGH-2) widened the projection — fill the new
    // columns with Null so HashMapRow returns Ok(None) at probe time
    // (HashMapRow distinguishes missing cells from explicit NULLs).
    HashMapRow::new("HT_Customers")
        .with("Cust_no", MockValue::Str(cust_no.into()))
        .with("Cust_name", MockValue::Str(name.into()))
        .with("Cust_name2", MockValue::Null)
        .with("Cust_perfix", MockValue::Str("นาย".into()))
        .with("Cust_sex", MockValue::Null)
        .with("Cust_IDcard", MockValue::Str("REDACTED-sa-pw90123".into()))
        .with("Cust_Type", MockValue::Null)
        .with(
            "Cust_Type_Main",
            MockValue::Str("บุคคลธรรมดา".into()),
        )
        .with("Cust_Email", MockValue::Str("a@b.co".into()))
        .with("Cust_Add_no", MockValue::Str("123/4".into()))
        .with("Cust_Add_moo", MockValue::Null)
        .with("Cust_Add_soi", MockValue::Null)
        .with("Cust_Add_road", MockValue::Null)
        .with("Cust_Add_tambon", MockValue::Null)
        .with("Cust_Add_ampore", MockValue::Null)
        .with("Cust_Add_province", MockValue::Null)
        .with("Cust_Add_code", MockValue::Null)
        .with("Cust_Add_tel", MockValue::Str(phone.into()))
        .with("Cust_Add_fax", MockValue::Null)
        .with("Cust_Work_Name", MockValue::Null)
        .with("Cust_Work_no", MockValue::Null)
        .with("Cust_Work_moo", MockValue::Null)
        .with("Cust_Work_soi", MockValue::Null)
        .with("Cust_Work_road", MockValue::Null)
        .with("Cust_Work_tambon", MockValue::Null)
        .with("Cust_Work_ampore", MockValue::Null)
        .with("Cust_Work_province", MockValue::Null)
        .with("Cust_Work_code", MockValue::Null)
        .with("Cust_Work_tel", MockValue::Null)
        .with("Cust_Work_fax", MockValue::Null)
        .with("Cust_Work_Tax", MockValue::Null)
        .with("Cust_Last_Change", MockValue::Null)
        .with("Cust_Contry", MockValue::Null)
        .with("Cust_Price_Over", MockValue::Null)
}

#[tokio::test]
async fn customer_insert_upserts_pg_row_and_writes_event_log() {
    let pool = common::create_test_pool().await;
    let mapper = CustomerMapper;
    let cust_no = unique_cust_no();
    let row = customer_row_full(&cust_no, "Alpha Tester", "0801112222");

    let mut tx = pool.begin().await.expect("begin tx");
    let event = mapper
        .apply(&mut tx, ChangeOp::Insert, Some(&row))
        .await
        .expect("Insert must succeed");
    assert!(
        event.is_some(),
        "Insert against a fresh Cust_no must emit a CustomerCreated event"
    );

    // Persist the event into event_log via the same TX so subscribers see it.
    if let Some(ref ev) = event {
        hotel_backend::outbox::bus::EventBus::publish(&mut tx, ev)
            .await
            .expect("publish must succeed");
    }
    tx.commit().await.expect("commit");

    // Verify the canonical row landed.
    let (cust_id, legacy_cust_no, agg_id, name): (
        i32,
        Option<String>,
        Option<uuid::Uuid>,
        String,
    ) = sqlx::query_as(
        "SELECT cust_id, legacy_cust_no, aggregate_id, cust_firstname \
           FROM ht_customers WHERE legacy_cust_no = $1",
    )
    .bind(&cust_no)
    .fetch_one(&pool)
    .await
    .expect("row must exist after Insert");
    assert_eq!(legacy_cust_no.as_deref(), Some(cust_no.as_str()));
    assert!(agg_id.is_some(), "aggregate_id must be populated");
    assert_eq!(name, "Alpha Tester");

    // Verify event_log carries the matching CustomerCreated row.
    let kind: String = sqlx::query_scalar(
        "SELECT event_type FROM event_log \
          WHERE aggregate_id = $1 ORDER BY created_at DESC LIMIT 1",
    )
    .bind(agg_id.unwrap())
    .fetch_one(&pool)
    .await
    .expect("event_log row must exist");
    assert_eq!(kind, "CustomerCreated");

    // Cleanup.
    sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
        .bind(agg_id.unwrap())
        .execute(&pool)
        .await
        .ok();
    sqlx::query("DELETE FROM ht_customers WHERE cust_id = $1")
        .bind(cust_id)
        .execute(&pool)
        .await
        .ok();
}

#[tokio::test]
async fn customer_re_apply_with_identical_row_skips_event() {
    let pool = common::create_test_pool().await;
    let mapper = CustomerMapper;
    let cust_no = unique_cust_no();
    let row = customer_row_full(&cust_no, "Idem Tester", "0809998888");

    // First apply — emits an event.
    let mut tx = pool.begin().await.expect("begin");
    let first = mapper
        .apply(&mut tx, ChangeOp::Insert, Some(&row))
        .await
        .expect("first apply");
    assert!(first.is_some(), "first apply must emit an event");
    tx.commit().await.unwrap();

    // Second apply (Update, same data) — must NOT emit an event.
    let mut tx2 = pool.begin().await.expect("begin 2");
    let second = mapper
        .apply(&mut tx2, ChangeOp::Update, Some(&row))
        .await
        .expect("second apply");
    assert!(
        second.is_none(),
        "re-apply with identical content must skip event publication"
    );
    tx2.commit().await.unwrap();

    // Cleanup.
    let agg: Option<uuid::Uuid> = sqlx::query_scalar(
        "SELECT aggregate_id FROM ht_customers WHERE legacy_cust_no = $1",
    )
    .bind(&cust_no)
    .fetch_optional(&pool)
    .await
    .unwrap();
    if let Some(a) = agg {
        sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
            .bind(a)
            .execute(&pool)
            .await
            .ok();
    }
    sqlx::query("DELETE FROM ht_customers WHERE legacy_cust_no = $1")
        .bind(&cust_no)
        .execute(&pool)
        .await
        .ok();
}

#[tokio::test]
async fn customer_update_with_changed_phone_emits_modified_event() {
    let pool = common::create_test_pool().await;
    let mapper = CustomerMapper;
    let cust_no = unique_cust_no();
    let row1 = customer_row_full(&cust_no, "Phone Mod", "0801111111");
    let row2 = customer_row_full(&cust_no, "Phone Mod", "0809999999");

    let mut tx = pool.begin().await.unwrap();
    mapper
        .apply(&mut tx, ChangeOp::Insert, Some(&row1))
        .await
        .unwrap();
    tx.commit().await.unwrap();

    let mut tx2 = pool.begin().await.unwrap();
    let event = mapper
        .apply(&mut tx2, ChangeOp::Update, Some(&row2))
        .await
        .expect("update must succeed");
    tx2.commit().await.unwrap();

    let event = event.expect("phone change must produce an event");
    assert_eq!(event.type_name(), "CustomerModified");

    // Cleanup.
    let agg: Option<uuid::Uuid> = sqlx::query_scalar(
        "SELECT aggregate_id FROM ht_customers WHERE legacy_cust_no = $1",
    )
    .bind(&cust_no)
    .fetch_optional(&pool)
    .await
    .unwrap();
    if let Some(a) = agg {
        sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
            .bind(a)
            .execute(&pool)
            .await
            .ok();
    }
    sqlx::query("DELETE FROM ht_customers WHERE legacy_cust_no = $1")
        .bind(&cust_no)
        .execute(&pool)
        .await
        .ok();
}

#[tokio::test]
async fn customer_delete_sets_soft_delete_timestamp() {
    let pool = common::create_test_pool().await;
    let mapper = CustomerMapper;
    let cust_no = unique_cust_no();
    let row = customer_row_full(&cust_no, "Del Tester", "0807776666");

    // Create.
    let mut tx = pool.begin().await.unwrap();
    mapper
        .apply(&mut tx, ChangeOp::Insert, Some(&row))
        .await
        .unwrap();
    tx.commit().await.unwrap();

    // Delete — pass a row that carries the PK column `id` (we don't
    // actually have it, but Cust_no is what the soft-delete branch
    // uses).
    let del_row = HashMapRow::new("HT_Customers")
        .with("id", MockValue::I32(1))
        .with("Cust_no", MockValue::Str(cust_no.clone()));
    let mut tx2 = pool.begin().await.unwrap();
    let event = mapper
        .apply(&mut tx2, ChangeOp::Delete, Some(&del_row))
        .await
        .expect("Delete must succeed");
    tx2.commit().await.unwrap();
    assert!(event.is_none(), "5.2 D events emit no DomainEvent");

    let deleted_at: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar(
        "SELECT cust_deleted_at FROM ht_customers WHERE legacy_cust_no = $1",
    )
    .bind(&cust_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(
        deleted_at.is_some(),
        "Delete branch must set cust_deleted_at to a non-null timestamp"
    );

    // Cleanup.
    let agg: Option<uuid::Uuid> = sqlx::query_scalar(
        "SELECT aggregate_id FROM ht_customers WHERE legacy_cust_no = $1",
    )
    .bind(&cust_no)
    .fetch_optional(&pool)
    .await
    .unwrap();
    if let Some(a) = agg {
        sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
            .bind(a)
            .execute(&pool)
            .await
            .ok();
    }
    sqlx::query("DELETE FROM ht_customers WHERE legacy_cust_no = $1")
        .bind(&cust_no)
        .execute(&pool)
        .await
        .ok();
}

#[tokio::test]
async fn room_master_clean_flip_emits_marked_clean_event() {
    let pool = common::create_test_pool().await;
    let mapper = RoomMasterMapper;

    // Create a real room (with a unique room_no) so the mapper has
    // something to UPSERT against.
    let room_no = format!("Z{:03}", (rand::random::<u8>() as u16) % 999 + 1);
    let room_id: i32 = sqlx::query_scalar(
        "INSERT INTO ht_rooms_new (room_no, room_clean, room_notes) \
         VALUES ($1, false, 'TEST_phase52_room') \
         ON CONFLICT (room_no) DO UPDATE SET room_clean = EXCLUDED.room_clean \
         RETURNING room_id",
    )
    .bind(&room_no)
    .fetch_one(&pool)
    .await
    .unwrap();

    // Seed a unique legacy_room_id_int so the mapper resolves to this row.
    let legacy_id: i32 = (rand::random::<u32>() % 1_000_000) as i32 + 100_000;
    sqlx::query(
        "UPDATE ht_rooms_new SET legacy_room_id_int = $1, room_clean = false \
         WHERE room_id = $2",
    )
    .bind(legacy_id)
    .bind(room_id)
    .execute(&pool)
    .await
    .unwrap();

    let row = HashMapRow::new("HT_Rooms")
        .with("id", MockValue::I32(legacy_id))
        .with("Room_no", MockValue::Str(room_no.clone()))
        .with("Room_Type", MockValue::Str("Standard".into()))
        .with("Room_Clean", MockValue::Str("yes".into()))
        .with("Room_Use", MockValue::Str("no".into()))
        // `Room_Manternace` (legacy typo) became required after v2.63.0
        // extended the room mapper to project the maintenance flag.
        .with("Room_Manternace", MockValue::Str("no".into()))
        .with("Room_Details", MockValue::Null)
        // Track E2 (T1 HIGH-3) — HashMapRow probes by cell presence;
        // every column the projection reads must be set to either a
        // value or an explicit Null.
        .with("Room_Use_Count", MockValue::Null)
        .with("Room_X", MockValue::Null)
        .with("Room_Y", MockValue::Null)
        .with("Room_Group", MockValue::Null)
        .with("Room_Power_OPEN", MockValue::Null)
        .with("Room_Power_CLOSE", MockValue::Null)
        .with("Room_Power_STATUS", MockValue::Null)
        .with("Room_Polity", MockValue::Null);

    let mut tx = pool.begin().await.unwrap();
    let event = mapper
        .apply(&mut tx, ChangeOp::Update, Some(&row))
        .await
        .expect("upsert must succeed");
    tx.commit().await.unwrap();

    let event = event.expect("clean false→true must emit RoomMarkedClean");
    assert_eq!(event.type_name(), "RoomMarkedClean");

    let new_clean: bool = sqlx::query_scalar(
        "SELECT room_clean FROM ht_rooms_new WHERE room_id = $1",
    )
    .bind(room_id)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(new_clean, "room_clean must be true after the apply");

    // Cleanup.
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
        .bind(room_id)
        .execute(&pool)
        .await
        .ok();
}

#[tokio::test]
async fn room_master_unchanged_clean_skips_event() {
    let pool = common::create_test_pool().await;
    let mapper = RoomMasterMapper;

    let room_no = format!("Y{:03}", (rand::random::<u8>() as u16) % 999 + 1);
    let room_id: i32 = sqlx::query_scalar(
        "INSERT INTO ht_rooms_new (room_no, room_clean, room_notes) \
         VALUES ($1, true, 'TEST_phase52_room_idem') \
         ON CONFLICT (room_no) DO UPDATE SET room_clean = EXCLUDED.room_clean \
         RETURNING room_id",
    )
    .bind(&room_no)
    .fetch_one(&pool)
    .await
    .unwrap();

    let legacy_id: i32 = (rand::random::<u32>() % 1_000_000) as i32 + 200_000;
    sqlx::query(
        "UPDATE ht_rooms_new SET legacy_room_id_int = $1, room_clean = true \
         WHERE room_id = $2",
    )
    .bind(legacy_id)
    .bind(room_id)
    .execute(&pool)
    .await
    .unwrap();

    let row = HashMapRow::new("HT_Rooms")
        .with("id", MockValue::I32(legacy_id))
        .with("Room_no", MockValue::Str(room_no.clone()))
        .with("Room_Type", MockValue::Str("Standard".into()))
        .with("Room_Clean", MockValue::Str("yes".into())) // already true
        .with("Room_Use", MockValue::Str("no".into()))
        // `Room_Manternace` (legacy typo) became required after v2.63.0
        // extended the room mapper to project the maintenance flag.
        .with("Room_Manternace", MockValue::Str("no".into()))
        .with("Room_Details", MockValue::Null)
        // Track E2 (T1 HIGH-3) — projection requires every column to
        // be present in the row.
        .with("Room_Use_Count", MockValue::Null)
        .with("Room_X", MockValue::Null)
        .with("Room_Y", MockValue::Null)
        .with("Room_Group", MockValue::Null)
        .with("Room_Power_OPEN", MockValue::Null)
        .with("Room_Power_CLOSE", MockValue::Null)
        .with("Room_Power_STATUS", MockValue::Null)
        .with("Room_Polity", MockValue::Null);

    let mut tx = pool.begin().await.unwrap();
    let event = mapper
        .apply(&mut tx, ChangeOp::Update, Some(&row))
        .await
        .expect("upsert must succeed");
    tx.commit().await.unwrap();
    assert!(
        event.is_none(),
        "no-clean-flip must skip event (was true, stays true)"
    );

    sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
        .bind(room_id)
        .execute(&pool)
        .await
        .ok();
}

// =============================================================================
// Customer hard-delete resolution by legacy_id (migration 055, audit
// 2026-06-11 P1 #6).
//
// Real CT D-rows for HT_Customers carry ONLY the legacy SERIAL `id` —
// the LEFT JOIN against the deleted row nulls Cust_no. Pre-055 the
// soft-delete branch resolved exclusively by Cust_no, so every iHOTEL
// customer delete (FrmManageCustomersNew) was a silent no-op.
// =============================================================================

#[tokio::test]
async fn customer_delete_resolves_by_legacy_id_alone() {
    let pool = common::create_test_pool().await;
    let mapper = CustomerMapper;
    let cust_no = unique_cust_no();
    // Unique-ish legacy id for this run (avoid cross-run collisions on
    // the shared test DB).
    let legacy_id: i32 = 9_000_000 + (unique_residue() % 900_000) as i32;

    // Create — the I-row carries the `id` PK alias the watcher always
    // materialises in; the UPSERT must persist it as legacy_id.
    let row = customer_row_full(&cust_no, "Del Tester 055", "0807776666")
        .with("id", MockValue::I32(legacy_id));
    let mut tx = pool.begin().await.unwrap();
    mapper
        .apply(&mut tx, ChangeOp::Insert, Some(&row))
        .await
        .unwrap();
    tx.commit().await.unwrap();

    let stored: Option<i32> = sqlx::query_scalar(
        "SELECT legacy_id FROM ht_customers WHERE legacy_cust_no = $1",
    )
    .bind(&cust_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(stored, Some(legacy_id), "I/U apply must persist legacy_id");

    // Delete — faithful CT D-row shape: ONLY the pk `id`; the joined
    // Cust_no is NULL because the MSSQL row is gone.
    let del_row = HashMapRow::new("HT_Customers")
        .with("id", MockValue::I32(legacy_id))
        .with("Cust_no", MockValue::Null);
    let mut tx2 = pool.begin().await.unwrap();
    let event = mapper
        .apply(&mut tx2, ChangeOp::Delete, Some(&del_row))
        .await
        .expect("Delete must succeed");
    tx2.commit().await.unwrap();
    assert!(event.is_none(), "D events emit no DomainEvent");

    let deleted_at: Option<chrono::DateTime<chrono::Utc>> = sqlx::query_scalar(
        "SELECT cust_deleted_at FROM ht_customers WHERE legacy_cust_no = $1",
    )
    .bind(&cust_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert!(
        deleted_at.is_some(),
        "pre-055 this was a silent no-op: a Cust_no-less D-row must now \
         resolve via legacy_id and set cust_deleted_at"
    );

    // Cleanup.
    let agg: Option<uuid::Uuid> = sqlx::query_scalar(
        "SELECT aggregate_id FROM ht_customers WHERE legacy_cust_no = $1",
    )
    .bind(&cust_no)
    .fetch_optional(&pool)
    .await
    .unwrap();
    if let Some(a) = agg {
        sqlx::query("DELETE FROM event_log WHERE aggregate_id = $1")
            .bind(a)
            .execute(&pool)
            .await
            .ok();
    }
    sqlx::query("DELETE FROM ht_customers WHERE legacy_cust_no = $1")
        .bind(&cust_no)
        .execute(&pool)
        .await
        .ok();
}

/// 2026-06-12 (audit follow-up) — a `HT_Rooms` CT row for a room PG has
/// never seen must AUTO-CREATE the canonical row (pre-fix: info-skip,
/// which silently dropped every downstream calendar tile / booking line
/// for a room created in iHOTEL and sold before `backfill_rooms` ran).
#[tokio::test]
async fn room_master_auto_creates_unknown_room() {
    let pool = common::create_test_pool().await;
    let mapper = RoomMasterMapper;

    let room_no = format!("Q{:03}", unique_residue() % 999 + 1);
    // Make sure the room genuinely doesn't exist.
    sqlx::query("DELETE FROM ht_rooms_new WHERE room_no = $1")
        .bind(&room_no)
        .execute(&pool)
        .await
        .ok();
    let legacy_id: i32 = 9_000_000 + (unique_residue() % 900_000) as i32;

    let row = HashMapRow::new("HT_Rooms")
        .with("id", MockValue::I32(legacy_id))
        .with("Room_no", MockValue::Str(room_no.clone()))
        .with("Room_Type", MockValue::Str("Standard".into()))
        .with("Room_Clean", MockValue::Str("yes".into()))
        .with("Room_Use", MockValue::Str("no".into()))
        .with("Room_Manternace", MockValue::Str("no".into()))
        .with("Room_Details", MockValue::Str("TEST_autocreate".into()))
        .with("Room_Use_Count", MockValue::Null)
        .with("Room_X", MockValue::Null)
        .with("Room_Y", MockValue::Null)
        .with("Room_Group", MockValue::Null)
        .with("Room_Power_OPEN", MockValue::Null)
        .with("Room_Power_CLOSE", MockValue::Null)
        .with("Room_Power_STATUS", MockValue::Null)
        .with("Room_Polity", MockValue::Null);

    let mut tx = pool.begin().await.unwrap();
    mapper
        .apply(&mut tx, ChangeOp::Insert, Some(&row))
        .await
        .expect("unknown room must auto-create, not skip/error");
    tx.commit().await.unwrap();

    let (room_id, clean, notes): (i32, Option<bool>, Option<String>) = sqlx::query_as(
        "SELECT room_id, room_clean, room_notes \
           FROM ht_rooms_new WHERE room_no = $1",
    )
    .bind(&room_no)
    .fetch_one(&pool)
    .await
    .expect("auto-created room row must exist");
    assert_eq!(clean, Some(true));
    assert_eq!(notes.as_deref(), Some("TEST_autocreate"));

    let (legacy_no, legacy_int, agg_id): (Option<String>, Option<i32>, Option<uuid::Uuid>) =
        sqlx::query_as(
            "SELECT legacy_room_no, legacy_room_id_int, aggregate_id \
               FROM ht_rooms_new WHERE room_id = $1",
        )
        .bind(room_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(legacy_no.as_deref(), Some(room_no.as_str()));
    assert_eq!(legacy_int, Some(legacy_id));
    assert!(agg_id.is_some(), "aggregate_id must be stamped post-INSERT");

    // Idempotent re-apply: must route through the UPDATE arm now.
    let mut tx = pool.begin().await.unwrap();
    mapper
        .apply(&mut tx, ChangeOp::Update, Some(&row))
        .await
        .expect("re-apply must hit the existing-row arm");
    tx.commit().await.unwrap();
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM ht_rooms_new WHERE room_no = $1",
    )
    .bind(&room_no)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 1, "re-apply must not duplicate the room");

    sqlx::query("DELETE FROM ht_rooms_new WHERE room_id = $1")
        .bind(room_id)
        .execute(&pool)
        .await
        .ok();
}
