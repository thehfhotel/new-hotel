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
    booking::BookingState, checkin::CheckInState, customer::CustomerType, hk_signal::RoomSignal,
    payment::PaymentMethod, shared::Money,
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

    /// Track G2 — refund / negative payment recorded against an existing
    /// `ht_payments` row. `amount` is the POSITIVE magnitude of the
    /// refund; canonical PG stores a negative `pay_amount` on the new
    /// refund row. `original_payment_id` carries the aggregate id of
    /// the payment being refunded so subscribers (SSE, audit log) can
    /// resolve both ends of the relationship.
    PaymentRefunded {
        check_in_id: Uuid,
        original_payment_id: Uuid,
        refund_payment_id: Uuid,
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
    /// A จัดผัง layout-edit drop moved one or two tiles on the SHARED room
    /// board (#236) — `room_x`/`room_y` changed, nothing else about the room
    /// did.
    ///
    /// Carries no coordinates on purpose: the board is a whole-list render
    /// (`GET /api/rooms`), so every consumer refetches the list rather than
    /// patching a tile. A swap emits ONE event (aggregate = the first moved
    /// room) because the drop is one intent.
    RoomLayoutChanged {
        room_id: Uuid,
        source: EventSource,
    },

    /// A maid tapped เริ่มทำความสะอาด on the `/hk` surface — the room is now
    /// being cleaned.
    ///
    /// PG-ONLY BY DESIGN: unlike [`RoomMarkedClean`](Self::RoomMarkedClean) /
    /// [`RoomMarkedDirty`](Self::RoomMarkedDirty) this event has NO writeback
    /// twin. iHOTEL's in-progress field `Room_Clean_Time` feeds its room-power
    /// countdown, so mirroring "started" is parity risk for no operational gain
    /// (housekeeping-ops plan, decision #3; `routes::hk` module header).
    ///
    /// Its whole job is reception visibility: the แผนกแม่บ้าน board subscribes
    /// to this name over SSE (`routes::events`) and re-renders the middle
    /// "กำลังทำความสะอาด" column live, without the maid touching iHOTEL.
    /// `by` is the maid label (verified HF ID display name, badge fallback) —
    /// the same value `MarkRoomClean` carries.
    RoomCleaningStarted {
        room_id: Uuid,
        by: String,
        source: EventSource,
    },

    /// A room signal (ADR 0008) was raised — by the desk on the v2 surface, by
    /// a maid on `/hk`, or as a `problems` room-check answer's child signal.
    ///
    /// PG-ONLY, like [`RoomCleaningStarted`](Self::RoomCleaningStarted) and
    /// more absolutely: `ht_hk_room_signals` has no legacy counterpart at all
    /// (migration 089), so these four variants have no writeback twin and never
    /// will. They exist for ONE job — UI event plumbing. Reception's board
    /// subscribes to the variant names over `routes::events`, and the maid's
    /// `/hk` page subscribes to `GET /api/hk/events`, which re-frames exactly
    /// these four under the single `hk_signal` event name carrying `signal` as
    /// its data.
    ///
    /// The whole [`RoomSignal`] DTO rides the event rather than an id: the
    /// boards render the signal directly from the frame (who raised it, which
    /// room, which canned type), so carrying only an id would make every live
    /// update a refetch — the opposite of what the SSE path is for. The DTO is
    /// ~300 bytes, comfortably inside the 8 KB payload budget
    /// (`architecture.md` §10).
    RoomSignalRaised {
        room_id: Uuid,
        signal: RoomSignal,
        source: EventSource,
    },
    /// Somebody took a room signal — the ack is what answers "who's on it".
    RoomSignalAcked {
        room_id: Uuid,
        signal: RoomSignal,
        source: EventSource,
    },
    /// A room signal reached `done`. `signal.done_source` says HOW: a tap, a
    /// maid's เสร็จแล้ว report auto-completing it, or a room-check answer (in
    /// which case `signal.outcome` carries เคลียร์ / problems).
    RoomSignalCompleted {
        room_id: Uuid,
        signal: RoomSignal,
        source: EventSource,
    },
    /// The creator's side withdrew a still-open room signal.
    RoomSignalCancelled {
        room_id: Uuid,
        signal: RoomSignal,
        source: EventSource,
    },
}

/// The four [`DomainEvent`] variant names that carry a [`RoomSignal`].
///
/// The maid stream (`GET /api/hk/events`) filters the shared fan-out on this
/// list before it parses anything, so a non-signal event costs one `contains`
/// and never a JSON parse. Kept next to the variants themselves — and pinned
/// by a test below — because a fifth signal variant added without an entry
/// here would silently never reach the maid's page.
pub const ROOM_SIGNAL_EVENT_NAMES: [&str; 4] = [
    "RoomSignalRaised",
    "RoomSignalAcked",
    "RoomSignalCompleted",
    "RoomSignalCancelled",
];

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
            DomainEvent::PaymentRefunded { .. } => "PaymentRefunded",
            DomainEvent::RoomMarkedClean { .. } => "RoomMarkedClean",
            DomainEvent::RoomMarkedDirty { .. } => "RoomMarkedDirty",
            DomainEvent::RoomLayoutChanged { .. } => "RoomLayoutChanged",
            DomainEvent::RoomCleaningStarted { .. } => "RoomCleaningStarted",
            DomainEvent::RoomSignalRaised { .. } => "RoomSignalRaised",
            DomainEvent::RoomSignalAcked { .. } => "RoomSignalAcked",
            DomainEvent::RoomSignalCompleted { .. } => "RoomSignalCompleted",
            DomainEvent::RoomSignalCancelled { .. } => "RoomSignalCancelled",
        }
    }

    /// The [`RoomSignal`] this event carries, or `None` for every other
    /// variant. The maid stream's projection — it turns a `domain_events`
    /// payload into the DTO the `/hk` page renders, with no second wire shape
    /// to keep in step.
    pub fn room_signal(&self) -> Option<&RoomSignal> {
        match self {
            DomainEvent::RoomSignalRaised { signal, .. }
            | DomainEvent::RoomSignalAcked { signal, .. }
            | DomainEvent::RoomSignalCompleted { signal, .. }
            | DomainEvent::RoomSignalCancelled { signal, .. } => Some(signal),
            _ => None,
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
            DomainEvent::PaymentReceived { check_in_id, .. }
            | DomainEvent::PaymentRefunded { check_in_id, .. } => *check_in_id,
            DomainEvent::RoomMarkedClean { room_id, .. }
            | DomainEvent::RoomMarkedDirty { room_id, .. }
            | DomainEvent::RoomLayoutChanged { room_id, .. }
            | DomainEvent::RoomCleaningStarted { room_id, .. }
            | DomainEvent::RoomSignalRaised { room_id, .. }
            | DomainEvent::RoomSignalAcked { room_id, .. }
            | DomainEvent::RoomSignalCompleted { room_id, .. }
            | DomainEvent::RoomSignalCancelled { room_id, .. } => *room_id,
        }
    }

    /// Borrow the originating [`EventSource`].
    ///
    /// Subscribers (e.g. the writeback worker) use this to filter out events
    /// they themselves caused — preventing feedback loops between the legacy
    /// detector and our writeback adapter.
    pub fn source(&self) -> &EventSource {
        match self {
            DomainEvent::BookingCreated { source, .. }
            | DomainEvent::BookingModified { source, .. }
            | DomainEvent::BookingCancelled { source, .. }
            | DomainEvent::CheckInCreated { source, .. }
            | DomainEvent::CheckOutCompleted { source, .. }
            | DomainEvent::CheckInCancelled { source, .. }
            | DomainEvent::CustomerCreated { source, .. }
            | DomainEvent::CustomerModified { source, .. }
            | DomainEvent::PaymentReceived { source, .. }
            | DomainEvent::PaymentRefunded { source, .. }
            | DomainEvent::RoomMarkedClean { source, .. }
            | DomainEvent::RoomMarkedDirty { source, .. }
            | DomainEvent::RoomLayoutChanged { source, .. }
            | DomainEvent::RoomCleaningStarted { source, .. }
            | DomainEvent::RoomSignalRaised { source, .. }
            | DomainEvent::RoomSignalAcked { source, .. }
            | DomainEvent::RoomSignalCompleted { source, .. }
            | DomainEvent::RoomSignalCancelled { source, .. } => source,
        }
    }
}

#[cfg(test)]
mod room_signal_event_tests {
    use super::*;
    use crate::domain::hk_signal::{
        RoomSignal, SignalActor, SignalDirection, SignalStatus, ROOM_CHECK,
    };

    fn dto() -> RoomSignal {
        RoomSignal {
            signal_id: 1,
            room_id: 42,
            room_no: "104".to_string(),
            direction: SignalDirection::DeskToMaid,
            signal_type: ROOM_CHECK.to_string(),
            status: SignalStatus::Open,
            outcome: None,
            parent_id: None,
            created_by: SignalActor {
                badge: "Q1".to_string(),
                name: None,
            },
            created_at: "2026-09-01T03:00:00Z".to_string(),
            acked_by: None,
            acked_at: None,
            done_by: None,
            done_at: None,
            done_source: None,
        }
    }

    fn variants() -> Vec<DomainEvent> {
        let room_id = Uuid::nil();
        let source = EventSource::our_app(Uuid::nil(), Uuid::nil());
        vec![
            DomainEvent::RoomSignalRaised {
                room_id,
                signal: dto(),
                source: source.clone(),
            },
            DomainEvent::RoomSignalAcked {
                room_id,
                signal: dto(),
                source: source.clone(),
            },
            DomainEvent::RoomSignalCompleted {
                room_id,
                signal: dto(),
                source: source.clone(),
            },
            DomainEvent::RoomSignalCancelled {
                room_id,
                signal: dto(),
                source,
            },
        ]
    }

    /// The maid stream filters on [`ROOM_SIGNAL_EVENT_NAMES`] before parsing.
    /// A fifth signal variant added without an entry there would silently never
    /// reach `/hk`, so the list is pinned against the variants themselves.
    #[test]
    fn every_signal_variant_is_listed_and_exposes_its_dto() {
        let built = variants();
        assert_eq!(built.len(), ROOM_SIGNAL_EVENT_NAMES.len());
        for event in &built {
            assert!(
                ROOM_SIGNAL_EVENT_NAMES.contains(&event.type_name()),
                "{} is missing from ROOM_SIGNAL_EVENT_NAMES",
                event.type_name()
            );
            assert!(
                event.room_signal().is_some(),
                "{} must expose its DTO to the maid stream",
                event.type_name()
            );
        }
    }

    /// A non-signal event must not be projected as one — the filter is a
    /// whitelist, and `room_signal()` is the second, independent gate.
    #[test]
    fn non_signal_events_carry_no_dto() {
        let event = DomainEvent::RoomMarkedDirty {
            room_id: Uuid::nil(),
            source: EventSource::our_app(Uuid::nil(), Uuid::nil()),
        };
        assert!(event.room_signal().is_none());
        assert!(!ROOM_SIGNAL_EVENT_NAMES.contains(&event.type_name()));
    }

    /// The wire round-trip the SSE relay depends on: publish → `pg_notify`
    /// payload → `serde_json::from_str::<DomainEvent>` → the same DTO.
    #[test]
    fn a_signal_event_round_trips_through_the_notify_payload() {
        for event in variants() {
            let payload = serde_json::to_string(&event).expect("serializes");
            let decoded: DomainEvent = serde_json::from_str(&payload).expect("deserializes");
            assert_eq!(decoded.type_name(), event.type_name());
            assert_eq!(decoded.room_signal(), event.room_signal());
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

impl EventSource {
    /// Convenience constructor for service-layer callers — most events come
    /// from an HTTP request and carry both the authenticated user id and the
    /// request correlation id.
    pub fn our_app(user_id: Uuid, request_id: Uuid) -> Self {
        EventSource::OurApp { user_id, request_id }
    }

    /// Stable string for the `event_log.source_kind` column. Matches the
    /// `serde(tag = "kind")` discriminant.
    pub fn kind_str(&self) -> &'static str {
        match self {
            EventSource::OurApp { .. } => "our_app",
            EventSource::LegacyApp { .. } => "legacy_app",
            EventSource::System { .. } => "system",
        }
    }

    /// Return the `(user_id, request_id)` pair for `OurApp` sources, otherwise
    /// `(None, None)` — used to populate `event_log.source_user_id` /
    /// `source_request_id` (both nullable for non-`OurApp` events).
    pub fn correlation(&self) -> (Option<Uuid>, Option<Uuid>) {
        match self {
            EventSource::OurApp { user_id, request_id } => (Some(*user_id), Some(*request_id)),
            _ => (None, None),
        }
    }
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
