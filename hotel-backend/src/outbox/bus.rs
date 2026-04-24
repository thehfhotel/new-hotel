//! `EventBus` — durable domain-event publisher.
//!
//! Per `docs/architecture.md` §3.6c (publication path) and §4d-bis
//! (`event_log` schema).
//!
//! ## Two effects per publish, one transaction
//!
//! Each call to [`EventBus::publish`] performs two SQL operations inside the
//! caller's transaction:
//!
//! 1. `INSERT INTO event_log (...)` — durable history. Subscribers that miss a
//!    `NOTIFY` (e.g. they were reconnecting) replay from this table using
//!    their stored `(aggregate_id, created_at)` cursor.
//! 2. `SELECT pg_notify('domain_events', payload)` — fan-out to every backend
//!    process holding a `LISTEN domain_events` connection. Per PostgreSQL
//!    semantics, the NOTIFY message is **buffered until COMMIT** — so
//!    subscribers never see an event that hasn't been durably persisted, and
//!    a rolled-back transaction emits no notification at all. This is what
//!    makes the publish safe to do in the same TX as the canonical write.
//!
//! ## What's NOT here (Phase 3b scope)
//!
//! - No `LISTEN` side. The browser SSE endpoint, writeback worker, and audit
//!   logger are subscribers; they land in Wave 4 (`bin/writeback.rs`,
//!   `routes/events.rs`).
//! - No retry / backoff. NOTIFY is best-effort by PG design; subscribers are
//!   expected to reconcile via `event_log` on reconnect (see
//!   `architecture.md` §3.6e).

use sqlx::{Postgres, Row, Transaction};
use uuid::Uuid;

use super::event::DomainEvent;

/// Publisher-side handle for the durable domain-event bus.
///
/// Stateless — every method takes the caller's open transaction. Wrap in
/// `Arc` and share across services.
#[derive(Debug, Default, Clone)]
pub struct EventBus;

impl EventBus {
    /// Construct a new event-bus publisher handle.
    pub fn new() -> Self {
        Self
    }

    /// Publish a [`DomainEvent`] inside the caller's transaction.
    ///
    /// Persists one `event_log` row and emits one `pg_notify('domain_events',
    /// ...)` — both buffered until the caller's `tx.commit()`. On rollback,
    /// neither effect is observed by subscribers.
    ///
    /// Returns the UUID of the inserted `event_log` row (also useful for the
    /// caller as a correlation id for downstream audit).
    pub async fn publish(
        tx: &mut Transaction<'_, Postgres>,
        event: &DomainEvent,
    ) -> Result<Uuid, sqlx::Error> {
        let event_type = event.type_name();
        let aggregate_id = event.aggregate_id();
        let source = event.source();
        let source_kind = source.kind_str();
        let (source_user_id, source_request_id) = source.correlation();

        let payload = serde_json::to_value(event).map_err(|e| {
            sqlx::Error::Encode(format!("DomainEvent serialization failed: {e}").into())
        })?;
        // The NOTIFY payload mirrors the JSONB stored in event_log so the SSE
        // broadcaster can forward it to browsers verbatim without an extra
        // roundtrip to event_log on the hot path.
        let notify_payload = serde_json::to_string(event).map_err(|e| {
            sqlx::Error::Encode(format!("DomainEvent serialization failed: {e}").into())
        })?;

        // Step 1: durable persistence.
        // Dynamic `sqlx::query()` (not `query!`) — see queue.rs rationale.
        // Schema owned by migration 012 (`event_log.sql`).
        let row = sqlx::query(
            "INSERT INTO event_log \
                 (id, event_type, aggregate_id, payload, \
                  source_kind, source_user_id, source_request_id, created_at) \
             VALUES (gen_random_uuid(), $1, $2, $3, $4, $5, $6, NOW()) \
             RETURNING id",
        )
        .bind(event_type)
        .bind(aggregate_id)
        .bind(&payload)
        .bind(source_kind)
        .bind(source_user_id)
        .bind(source_request_id)
        .fetch_one(&mut **tx)
        .await?;

        // Step 2: best-effort fan-out (deferred to COMMIT by PG).
        // pg_notify takes (channel, payload) and returns void — we don't need
        // the result, just the side-effect.
        sqlx::query("SELECT pg_notify('domain_events', $1)")
            .bind(&notify_payload)
            .execute(&mut **tx)
            .await?;

        let id: Uuid = row.try_get("id")?;
        Ok(id)
    }
}
