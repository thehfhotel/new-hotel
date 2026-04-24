//! `DomainEvent` — the type-only contract for the durable event bus.
//!
//! Per `docs/architecture.md` §3.6b. **Type definitions only** — the
//! `EventBus::publish` impl, `pg_notify` plumbing, and `event_log` persistence
//! all land in Phase 3b (Wave 2). This file is the wire contract; nothing here
//! does I/O.
//!
//! Wire format: `#[serde(tag = "type", content = "data")]` produces JSON of the
//! shape `{"type": "BookingCreated", "data": { ... }}` — friendly for the
//! browser-side `EventSource` listener and trivially auditable in `event_log`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    booking::BookingState, checkin::CheckInState, customer::CustomerType, payment::PaymentMethod,
    shared::Money,
};

/// Every state-mutating action in the system emits exactly one `DomainEvent`.
///
/// Subscribers (SSE broadcaster, writeback worker, audit log, notifications)
/// match on the variant to decide what to do. The variant set must cover every
/// observable change — adding a new domain action means adding a variant here.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum DomainEvent {
    BookingCreated {
        id: Uuid,
        source: EventSource,
        snapshot: BookingSnapshot,
    },
    BookingModified {
        id: Uuid,
        source: EventSource,
        before: BookingSnapshot,
        after: BookingSnapshot,
    },
    BookingCancelled {
        id: Uuid,
        source: EventSource,
        reason: Option<String>,
    },

    CheckInCreated {
        id: Uuid,
        source: EventSource,
        snapshot: CheckInSnapshot,
    },
    CheckOutCompleted {
        id: Uuid,
        source: EventSource,
    },
    CheckInCancelled {
        id: Uuid,
        source: EventSource,
        reason: Option<String>,
    },

    CustomerCreated {
        id: Uuid,
        source: EventSource,
        snapshot: CustomerSnapshot,
    },
    CustomerModified {
        id: Uuid,
        source: EventSource,
        changed_fields: Vec<String>,
    },

    PaymentReceived {
        check_in_id: Uuid,
        amount: Money,
        method: PaymentMethod,
        source: EventSource,
    },

    RoomMarkedClean {
        room_id: Uuid,
        by: String,
        source: EventSource,
    },
    RoomMarkedDirty {
        room_id: Uuid,
        source: EventSource,
    },
}

impl DomainEvent {
    /// Stable string identifier for this variant — matches the `type` discriminant
    /// produced by `serde(tag = "type")` so that subscribers can filter on either.
    pub fn type_name(&self) -> &'static str {
        match self {
            DomainEvent::BookingCreated { .. } => "BookingCreated",
            DomainEvent::BookingModified { .. } => "BookingModified",
            DomainEvent::BookingCancelled { .. } => "BookingCancelled",
            DomainEvent::CheckInCreated { .. } => "CheckInCreated",
            DomainEvent::CheckOutCompleted { .. } => "CheckOutCompleted",
            DomainEvent::CheckInCancelled { .. } => "CheckInCancelled",
            DomainEvent::CustomerCreated { .. } => "CustomerCreated",
            DomainEvent::CustomerModified { .. } => "CustomerModified",
            DomainEvent::PaymentReceived { .. } => "PaymentReceived",
            DomainEvent::RoomMarkedClean { .. } => "RoomMarkedClean",
            DomainEvent::RoomMarkedDirty { .. } => "RoomMarkedDirty",
        }
    }

    /// The aggregate root id this event mutates.
    ///
    /// Used by the `event_log` index `(aggregate_id, created_at DESC)` so that
    /// the UI can reconstruct an entity's history in one query.
    pub fn aggregate_id(&self) -> Uuid {
        match self {
            DomainEvent::BookingCreated { id, .. }
            | DomainEvent::BookingModified { id, .. }
            | DomainEvent::BookingCancelled { id, .. }
            | DomainEvent::CheckInCreated { id, .. }
            | DomainEvent::CheckOutCompleted { id, .. }
            | DomainEvent::CheckInCancelled { id, .. }
            | DomainEvent::CustomerCreated { id, .. }
            | DomainEvent::CustomerModified { id, .. } => *id,
            DomainEvent::PaymentReceived { check_in_id, .. } => *check_in_id,
            DomainEvent::RoomMarkedClean { room_id, .. }
            | DomainEvent::RoomMarkedDirty { room_id, .. } => *room_id,
        }
    }
}

/// Where a domain event originated.
///
/// Subscribers use this to avoid feedback loops — e.g. the writeback worker
/// must skip events with `source = LegacyApp` (those changes are already in
/// MSSQL by definition).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EventSource {
    /// Came through our HTTP routes — carries the actor + correlation id.
    OurApp { user_id: Uuid, request_id: Uuid },
    /// Detected via SQL Server Change Tracking on the legacy DB.
    LegacyApp { detected_at: DateTime<Utc> },
    /// Internal scheduled job (reconcile, retention sweep, etc.).
    System { reason: String },
}

/// Minimal snapshot of a booking carried in events.
///
/// Subscribers re-fetch the full aggregate via the repository if they need
/// more — this keeps event payloads under the 8 KB budget noted in
/// `architecture.md` §10.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingSnapshot {
    pub id: Uuid,
    pub legacy_book_id: Option<String>,
    pub customer_id: Uuid,
    pub state: BookingState,
    pub stay_start: DateTime<Utc>,
    pub stay_end: DateTime<Utc>,
    pub room_no: Option<String>,
    pub price: Money,
}

/// Minimal snapshot of a check-in carried in events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckInSnapshot {
    pub id: Uuid,
    pub legacy_cin_no: Option<String>,
    pub booking_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub status: CheckInState,
    pub room_no: String,
    pub stay_start: DateTime<Utc>,
    pub stay_end: DateTime<Utc>,
    pub total_price_net: Money,
}

/// Minimal snapshot of a customer carried in events.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerSnapshot {
    pub id: Uuid,
    pub legacy_cust_no: Option<String>,
    pub name: String,
    pub customer_type: CustomerType,
    pub phone: Option<String>,
}
