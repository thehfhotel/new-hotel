//! Event log repository — durable bus for `DomainEvent`s.
//!
//! Per `docs/architecture.md` §3.6c, §4d-bis. **Stub trait + scaffolded impl**.
//! Agent D fills in the live `publish` body in parallel; until then the trait
//! is the stable boundary the service layer (Phase 2) will depend on.
//!
//! The implementation appends to `event_log` and fires
//! `pg_notify('domain_events', ...)` so SSE subscribers wake up.

use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::outbox::event::DomainEvent;

/// Durable, replay-capable bus for `DomainEvent`s.
///
/// Like `OutboxRepository`, every publish takes a `&mut Transaction` so that
/// the canonical PG write + outbox enqueue + event publish all commit
/// atomically.
#[async_trait]
pub trait EventLogRepository: Send + Sync {
    /// Persist a `DomainEvent` and notify subscribers.
    ///
    /// Implementation contract (see Agent D / `architecture.md` §3.6c):
    /// 1. INSERT into `event_log` (event_type, aggregate_id, payload,
    ///    source_kind, source_user_id, source_request_id).
    /// 2. `SELECT pg_notify('domain_events', <serialized event>)`.
    async fn publish(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        event: &DomainEvent,
    ) -> Result<(), sqlx::Error>;
}

/// Stub `EventLogRepository` impl — Agent D replaces the body.
#[derive(Clone, Debug, Default)]
pub struct PgEventLogRepository;

impl PgEventLogRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventLogRepository for PgEventLogRepository {
    async fn publish(
        &self,
        _tx: &mut Transaction<'_, Postgres>,
        _event: &DomainEvent,
    ) -> Result<(), sqlx::Error> {
        // Agent D fills in: INSERT event_log + pg_notify('domain_events', ...)
        unimplemented!(
            "PgEventLogRepository::publish is implemented in Agent D's branch (Phase 1b parallel)"
        )
    }
}
