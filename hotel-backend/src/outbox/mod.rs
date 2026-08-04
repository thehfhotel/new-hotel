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

//! - **`legacy_stale`** sits slightly apart from the rest of this module: it is
//!   an *adapter-level* hint (published by `bin/writeback.rs` AFTER the legacy
//!   MSSQL commit), not a durable domain fact, and it deliberately uses a plain
//!   `pg_notify` with no `event_log` row. See that module's doc for why.

pub mod bus;
pub mod event;
pub mod idempotency;
pub mod intent;
pub mod legacy_stale;
pub mod queue;

pub use bus::EventBus;
pub use event::{DomainEvent, EventSource};
pub use idempotency::{generate_idempotency_key, WRITEBACK_NAMESPACE};
pub use intent::{CompanionEntry, NoteTargetKind, WritebackIntent};
pub use legacy_stale::{LegacyStaleSignal, StaleNote, LEGACY_STALE_CHANNEL, LEGACY_STALE_EVENT};
pub use queue::OutboxRepository;
