//! Outbox layer — durable command queue + domain event bus.
//!
//! Per `docs/architecture.md` §3.6, §4c, §4d-bis.
//!
//! - **Phase 3a** (already on master): type-only contracts in [`event`] /
//!   [`intent`] plus the `writeback_jobs` / `event_log` migrations.
//! - **Phase 3b** (this layer): runtime publishers — [`queue::OutboxRepository`]
//!   for the writeback queue, [`bus::EventBus`] for the domain-event bus, and
//!   [`idempotency`] helpers for deterministic dedup keys.
//!
//! Subscribers (writeback worker LISTEN'er, SSE broadcaster, audit logger)
//! land in Wave 4 and live outside this module.

pub mod bus;
pub mod event;
pub mod idempotency;
pub mod intent;
pub mod queue;

pub use bus::EventBus;
pub use event::{DomainEvent, EventSource};
pub use idempotency::{generate_idempotency_key, WRITEBACK_NAMESPACE};
pub use intent::WritebackIntent;
pub use queue::OutboxRepository;
