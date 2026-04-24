//! Outbox repository — durable writeback queue for legacy MSSQL.
//!
//! Per `docs/architecture.md` §3.6c, §4c. **Stub trait + scaffolded impl** —
//! Agent D fills in the live `enqueue` body in parallel during Phase 1b. The
//! trait is the stable boundary that the upcoming service layer (Phase 2)
//! depends on, so this file ships the contract without forcing the
//! orchestrator to wait on a single agent.
//!
//! When Agent D's branch lands, the `unimplemented!()` body is replaced by an
//! INSERT into `writeback_jobs` + `pg_notify('writeback_channel', ...)`.

use async_trait::async_trait;
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

use crate::outbox::intent::WritebackIntent;

/// Append-only queue of intents the writeback worker will dispatch onto MSSQL.
///
/// All methods take `&mut Transaction<'_, Postgres>` because the service layer
/// MUST enqueue in the SAME transaction as the canonical write — otherwise we
/// can lose writebacks (PG committed, app crashes, queue row never written).
#[async_trait]
pub trait OutboxRepository: Send + Sync {
    /// Enqueue a `WritebackIntent`. Returns the assigned outbox id.
    ///
    /// Implementation contract (see Agent D / `architecture.md` §3.6c):
    /// 1. INSERT into `writeback_jobs` (intent, payload JSONB, aggregate_id,
    ///    idempotency_key, status='pending').
    /// 2. `SELECT pg_notify('writeback_channel', <id>)` so the worker wakes up.
    /// 3. Return the BIGSERIAL id.
    async fn enqueue(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        aggregate_id: Uuid,
        idempotency_key: Uuid,
        intent: &WritebackIntent,
    ) -> Result<i64, sqlx::Error>;
}

/// Stub `OutboxRepository` impl — Agent D replaces the body in their parallel
/// branch. Until then, the type compiles and can be wired into `AppState`,
/// allowing the service layer (Phase 2) to be written against the trait
/// without blocking on outbox machinery.
#[derive(Clone, Debug, Default)]
pub struct PgOutboxRepository;

impl PgOutboxRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl OutboxRepository for PgOutboxRepository {
    async fn enqueue(
        &self,
        _tx: &mut Transaction<'_, Postgres>,
        _aggregate_id: Uuid,
        _idempotency_key: Uuid,
        _intent: &WritebackIntent,
    ) -> Result<i64, sqlx::Error> {
        // Agent D fills in: INSERT writeback_jobs + pg_notify.
        unimplemented!(
            "PgOutboxRepository::enqueue is implemented in Agent D's branch (Phase 1b parallel)"
        )
    }
}
