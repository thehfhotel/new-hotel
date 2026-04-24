//! `OutboxRepository` — durable command queue (writeback to legacy MSSQL).
//!
//! Per `docs/architecture.md` §3.6c (publication path) and §4c (table schema).
//!
//! ## Atomicity contract
//!
//! [`OutboxRepository::enqueue`] takes the **caller's** `&mut Transaction`
//! rather than opening its own. This is deliberate: the canonical write
//! (`INSERT INTO public.ht_bookings ...`) and the outbox enqueue must commit
//! or roll back together. If the canonical write succeeds but the enqueue
//! fails (or vice versa), we'd either lose a writeback or queue a writeback
//! for a booking that doesn't exist. Using one transaction makes both effects
//! atomic.
//!
//! ## Subscriber-side
//!
//! No subscriber here — that's `bin/writeback.rs` (Phase 4b). This module is
//! producer-side only, per the Phase 3b scope in `architecture.md` §8.

use sqlx::{Postgres, Row, Transaction};
use uuid::Uuid;

use super::intent::WritebackIntent;

/// Producer-side handle for the `writeback_jobs` outbox.
///
/// Stateless (no pool field) — every method takes the caller's open
/// transaction. Wrap in `Arc` and share across the request/service layer.
#[derive(Debug, Default, Clone)]
pub struct OutboxRepository;

impl OutboxRepository {
    /// Construct a new outbox repository handle.
    pub fn new() -> Self {
        Self
    }

    /// Enqueue a [`WritebackIntent`] for the writeback worker.
    ///
    /// Inserts a single row into `writeback_jobs` with `status = 'pending'`,
    /// the JSON-serialized intent payload, the aggregate id (extracted from
    /// the intent), and the supplied idempotency key.
    ///
    /// Returns the BIGSERIAL id of the inserted row.
    ///
    /// ## Errors
    ///
    /// * `sqlx::Error::Database` with constraint name
    ///   `writeback_jobs_idempotency_key_key` if a job with the same key was
    ///   already enqueued. The caller should treat this as "already queued"
    ///   and roll back its transaction (which is the correct outcome — the
    ///   canonical write should not commit twice either).
    /// * Any other `sqlx::Error` is a real I/O / serialization failure and
    ///   bubbles up so the calling transaction can roll back.
    pub async fn enqueue(
        tx: &mut Transaction<'_, Postgres>,
        intent: &WritebackIntent,
        idempotency_key: Uuid,
    ) -> Result<i64, sqlx::Error> {
        let intent_name = intent.intent_name();
        let aggregate_id = intent.aggregate_id();
        let payload = serde_json::to_value(intent).map_err(|e| {
            sqlx::Error::Encode(format!("WritebackIntent serialization failed: {e}").into())
        })?;

        // Dynamic `sqlx::query()` (not the `query!` macro) — keeps the crate
        // buildable without a live PG at compile time. The DDL is owned by
        // migration 011 (`writeback_jobs.sql`); column names / types here
        // mirror that file 1:1.
        let row = sqlx::query(
            "INSERT INTO writeback_jobs \
                 (intent, payload, aggregate_id, idempotency_key, status) \
             VALUES ($1, $2, $3, $4, 'pending') \
             RETURNING id",
        )
        .bind(intent_name)
        .bind(&payload)
        .bind(aggregate_id)
        .bind(idempotency_key)
        .fetch_one(&mut **tx)
        .await?;

        let id: i64 = row.try_get("id")?;
        Ok(id)
    }
}
