//! Phase 3b integration tests — outbox + event bus.
//!
//! These exercise the real PostgreSQL plumbing: `writeback_jobs` row insert,
//! `event_log` row insert, and the `pg_notify('domain_events', ...)` channel.
//!
//! ## Running
//!
//! Requires a running PG with migrations 011 and 012 applied. The `common`
//! helper module reads `DATABASE_URL` from the environment, falling back to
//! the local-dev connection string in `tests/common/mod.rs`.
//!
//! When PG is unavailable (CI without service container, fresh checkout),
//! `cargo test --no-run` still verifies the crate compiles. The runtime
//! `assert!` calls only fire when the runtime can establish a connection.

mod common;

use std::time::Duration;

use chrono::{TimeZone, Utc};
use sqlx::postgres::PgListener;
use sqlx::Row;
use uuid::Uuid;

use hotel_backend::domain::shared::{DateRange, Money};
use hotel_backend::outbox::event::{BookingSnapshot, DomainEvent, EventSource};
use hotel_backend::outbox::idempotency::generate_idempotency_key;
use hotel_backend::outbox::intent::{CreateBookingPayload, WritebackIntent};
use hotel_backend::outbox::queue::OutboxRepository;
use hotel_backend::outbox::bus::EventBus;
use hotel_backend::domain::booking::BookingState;

/// Build a deterministic CreateBooking intent for fixture purposes.
fn sample_create_booking(booking_id: Uuid, customer_id: Uuid) -> WritebackIntent {
    WritebackIntent::CreateBooking {
        booking_id,
        payload: CreateBookingPayload {
            customer_id,
            legacy_cust_no: None,
            customer_name: "TEST_Outbox Fixture".into(),
            customer_phone: None,
            stay: DateRange {
                start: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
                end: Utc.with_ymd_and_hms(2026, 1, 2, 0, 0, 0).unwrap(),
            },
            room_no: "T01".into(),
            room_type: "TST".into(),
            price: Money::from_satang(123_45),
            nights: 1,
            // deposit was added in v2.23.0 (CreateBookingPayload.deposit).
            // Existing fixture predates that change — defaulting to zero
            // preserves the prior test semantics.
            deposit: Money::ZERO,
            created_by: "outbox_test".into(),
            notes: Some("TEST_outbox row — safe to delete".into()),
        },
    }
}

fn sample_booking_event(booking_id: Uuid, customer_id: Uuid) -> DomainEvent {
    DomainEvent::BookingCreated {
        id: booking_id,
        source: EventSource::our_app(
            Uuid::parse_str("00000000-0000-0000-0000-00000000beef").unwrap(),
            Uuid::parse_str("00000000-0000-0000-0000-00000000cafe").unwrap(),
        ),
        snapshot: BookingSnapshot {
            id: booking_id,
            legacy_book_id: None,
            customer_id,
            state: BookingState::Pending,
            stay_start: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
            stay_end: Utc.with_ymd_and_hms(2026, 1, 2, 0, 0, 0).unwrap(),
            room_no: Some("T01".into()),
            price: Money::from_satang(123_45),
        },
    }
}

#[tokio::test]
async fn test_enqueue_inserts_row() {
    let pool = common::create_test_pool().await;

    let booking_id = Uuid::new_v4();
    let customer_id = Uuid::new_v4();
    let intent = sample_create_booking(booking_id, customer_id);
    let key = generate_idempotency_key(&intent, booking_id);

    let mut tx = pool.begin().await.expect("begin");
    let job_id = OutboxRepository::enqueue(&mut tx, &intent, key)
        .await
        .expect("enqueue should succeed");
    tx.commit().await.expect("commit");

    let row = sqlx::query(
        "SELECT intent, status, aggregate_id, idempotency_key, payload \
         FROM writeback_jobs WHERE id = $1",
    )
    .bind(job_id)
    .fetch_one(&pool)
    .await
    .expect("select inserted job");

    let stored_intent: String = row.try_get("intent").unwrap();
    let stored_status: String = row.try_get("status").unwrap();
    let stored_aggregate: Uuid = row.try_get("aggregate_id").unwrap();
    let stored_key: Uuid = row.try_get("idempotency_key").unwrap();
    let stored_payload: serde_json::Value = row.try_get("payload").unwrap();

    assert_eq!(stored_intent, "create_booking");
    assert_eq!(stored_status, "pending");
    assert_eq!(stored_aggregate, booking_id);
    assert_eq!(stored_key, key);
    assert_eq!(stored_payload["intent"], "create_booking");
    // serde(tag="intent", content="payload") wraps the variant fields under
    // "payload"; the CreateBooking variant's struct field is also named
    // `payload`, so the inner CreateBookingPayload sits at payload.payload.
    assert_eq!(stored_payload["payload"]["booking_id"], booking_id.to_string());
    assert_eq!(stored_payload["payload"]["payload"]["customer_id"], customer_id.to_string());

    // Cleanup so reruns don't accumulate test rows.
    sqlx::query("DELETE FROM writeback_jobs WHERE id = $1")
        .bind(job_id)
        .execute(&pool)
        .await
        .ok();
}

#[tokio::test]
async fn test_idempotency_key_dedup() {
    let pool = common::create_test_pool().await;

    let booking_id = Uuid::new_v4();
    let customer_id = Uuid::new_v4();
    let intent = sample_create_booking(booking_id, customer_id);
    let key = generate_idempotency_key(&intent, booking_id);

    // First enqueue commits.
    let mut tx = pool.begin().await.expect("begin");
    let first_id = OutboxRepository::enqueue(&mut tx, &intent, key)
        .await
        .expect("first enqueue");
    tx.commit().await.expect("commit first");

    // Second enqueue with the same key must hit the unique constraint.
    let mut tx2 = pool.begin().await.expect("begin");
    let dup_result = OutboxRepository::enqueue(&mut tx2, &intent, key).await;
    tx2.rollback().await.ok();

    let err = dup_result.expect_err("duplicate idempotency_key should fail");
    let err_msg = format!("{err}");
    assert!(
        err_msg.contains("idempotency_key") || err_msg.contains("unique"),
        "expected unique-violation message, got: {err_msg}"
    );

    // Cleanup the surviving first row.
    sqlx::query("DELETE FROM writeback_jobs WHERE id = $1")
        .bind(first_id)
        .execute(&pool)
        .await
        .ok();
}

#[tokio::test]
async fn test_publish_inserts_event_log_and_notifies() {
    let pool = common::create_test_pool().await;

    // Subscribe BEFORE publishing so the NOTIFY isn't lost.
    // PgListener owns its own connection — independent of the publish TX.
    let mut listener = PgListener::connect_with(&pool)
        .await
        .expect("listener connect");
    listener
        .listen("domain_events")
        .await
        .expect("LISTEN domain_events");

    let booking_id = Uuid::new_v4();
    let customer_id = Uuid::new_v4();
    let event = sample_booking_event(booking_id, customer_id);

    let mut tx = pool.begin().await.expect("begin");
    let event_id = EventBus::publish(&mut tx, &event)
        .await
        .expect("publish should succeed");
    tx.commit().await.expect("commit");

    // 1) Row is in event_log.
    let row = sqlx::query(
        "SELECT event_type, aggregate_id, source_kind, source_user_id, source_request_id, payload \
         FROM event_log WHERE id = $1",
    )
    .bind(event_id)
    .fetch_one(&pool)
    .await
    .expect("select event_log row");

    let event_type: String = row.try_get("event_type").unwrap();
    let aggregate_id: Uuid = row.try_get("aggregate_id").unwrap();
    let source_kind: String = row.try_get("source_kind").unwrap();
    let source_user_id: Uuid = row.try_get("source_user_id").unwrap();
    let source_request_id: Uuid = row.try_get("source_request_id").unwrap();
    let payload: serde_json::Value = row.try_get("payload").unwrap();

    assert_eq!(event_type, "BookingCreated");
    assert_eq!(aggregate_id, booking_id);
    assert_eq!(source_kind, "our_app");
    assert_eq!(source_user_id.to_string(), "00000000-0000-0000-0000-00000000beef");
    assert_eq!(source_request_id.to_string(), "00000000-0000-0000-0000-00000000cafe");
    assert_eq!(payload["type"], "BookingCreated");
    assert_eq!(payload["data"]["id"], booking_id.to_string());

    // 2) NOTIFY was delivered. PG buffers NOTIFY until COMMIT. Tests run in
    //    parallel and share the `domain_events` channel, so drain notifications
    //    until we see one for OUR booking_id (or time out).
    let notify_payload = recv_for_booking(&mut listener, booking_id, Duration::from_secs(2))
        .await
        .expect("NOTIFY for this booking_id should arrive within 2s of commit");
    assert_eq!(notify_payload["type"], "BookingCreated");
    assert_eq!(notify_payload["data"]["id"], booking_id.to_string());

    // Cleanup.
    sqlx::query("DELETE FROM event_log WHERE id = $1")
        .bind(event_id)
        .execute(&pool)
        .await
        .ok();
}

#[tokio::test]
async fn test_rollback_emits_no_event_and_no_notify() {
    // Defensive — if a caller's TX rolls back, neither the event_log row nor
    // the NOTIFY may escape. This is the property that makes EventBus::publish
    // safe to call alongside the canonical write.
    let pool = common::create_test_pool().await;

    let mut listener = PgListener::connect_with(&pool)
        .await
        .expect("listener connect");
    listener
        .listen("domain_events")
        .await
        .expect("LISTEN domain_events");

    let booking_id = Uuid::new_v4();
    let customer_id = Uuid::new_v4();
    let event = sample_booking_event(booking_id, customer_id);

    let mut tx = pool.begin().await.expect("begin");
    let event_id = EventBus::publish(&mut tx, &event)
        .await
        .expect("publish into uncommitted tx");
    tx.rollback().await.expect("rollback");

    // event_log row must NOT exist.
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM event_log WHERE id = $1")
        .bind(event_id)
        .fetch_one(&pool)
        .await
        .expect("count");
    assert_eq!(count, 0, "rollback must not leak an event_log row");

    // No NOTIFY for OUR booking_id should arrive — but the channel is shared
    // across the parallel `test_publish_inserts_event_log_and_notifies`, so we
    // drain anything that lands and only fail on a match.
    let received = recv_for_booking(&mut listener, booking_id, Duration::from_millis(500)).await;
    assert!(
        received.is_none(),
        "rollback must not emit a NOTIFY for booking_id={booking_id}; got: {received:?}"
    );
}

/// Drain notifications until either `deadline` expires or a `domain_events`
/// notification whose `data.id` equals `booking_id` arrives. Returns the
/// matching JSON payload, or `None` on timeout.
///
/// Tests run in parallel against a shared PG, and the bus uses a single
/// `domain_events` channel, so this filter is required to avoid cross-test
/// pollution.
async fn recv_for_booking(
    listener: &mut PgListener,
    booking_id: Uuid,
    timeout: Duration,
) -> Option<serde_json::Value> {
    let deadline = tokio::time::Instant::now() + timeout;
    loop {
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        if remaining.is_zero() {
            return None;
        }
        match tokio::time::timeout(remaining, listener.recv()).await {
            Err(_) => return None,
            Ok(Err(e)) => panic!("listener error: {e}"),
            Ok(Ok(notif)) => {
                let payload: serde_json::Value = match serde_json::from_str(notif.payload()) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                if payload["data"]["id"] == booking_id.to_string() {
                    return Some(payload);
                }
                // Different booking — keep listening.
            }
        }
    }
}
