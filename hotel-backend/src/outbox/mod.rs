//! Outbox layer — durable command queue + domain event bus contracts.
//!
//! Per `docs/architecture.md` §3.6, §4c, §4d-bis. **This file is type
//! scaffolding only** (Phase 3a). The runtime pieces — `EventBus::publish`,
//! `OutboxRepository::enqueue`, `pg_notify` plumbing — land in Phase 3b
//! (Wave 2). See `architecture.md` §8 roadmap.

pub mod event;
pub mod intent;
