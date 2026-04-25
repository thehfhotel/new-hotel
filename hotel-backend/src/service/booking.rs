//! Booking service — orchestrates `ht_bookings` writes plus outbox + events.
//!
//! Per `docs/architecture.md` §1, §6. Each public method opens one PG
//! transaction, performs the canonical `ht_bookings` mutation through
//! [`BookingRepository`](crate::repository::booking::BookingRepository),
//! enqueues the matching legacy MSSQL writeback intent via
//! [`OutboxRepository`](crate::outbox::OutboxRepository), publishes a
//! [`DomainEvent`](crate::outbox::DomainEvent) via
//! [`EventBus`](crate::outbox::EventBus), and commits — all atomic.
//!
//! Routes still call the repository directly today; Wave 4 thins them to
//! delegate through this service. Constructing the service today proves the
//! wiring + makes the call sites greppable for the Wave 4 refactor.

use std::sync::Arc;

use chrono::{NaiveDate, NaiveTime, TimeZone, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::booking::BookingState;
use crate::domain::shared::{DateRange, Money};
use crate::outbox::event::{BookingSnapshot, DomainEvent, EventSource};
use crate::outbox::intent::{BookingChanges, CreateBookingPayload, WritebackIntent};
use crate::outbox::{generate_idempotency_key, EventBus, OutboxRepository};
use crate::repository::booking::{BookingRepository, BookingRoomAssignment, BookingWrite};

use super::error::{ServiceError, ServiceResult};
use super::ids::{aggregate_uuid, AggregateKind};

/// One assigned room within a booking command.
#[derive(Debug, Clone, Copy)]
pub struct BookingRoomCommand {
    pub room_id: i32,
    pub price_per_night: Option<f64>,
}

/// Command for [`BookingService::create`].
///
/// Wraps everything the service needs to mint a `ht_bookings` row, attach its
/// rooms, enqueue the [`WritebackIntent::CreateBooking`] payload, and publish
/// [`DomainEvent::BookingCreated`].
#[derive(Debug, Clone)]
pub struct CreateBookingCommand {
    pub book_no: String,
    pub customer_id: i32,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub adults: i32,
    pub children: i32,
    pub status: String,
    pub source_label: Option<String>,
    pub total_amount: Option<f64>,
    pub deposit_amount: Option<f64>,
    pub notes: Option<String>,
    pub rooms: Vec<BookingRoomCommand>,

    /// Snapshot context used to build the [`CreateBookingPayload`] sent to
    /// the writeback worker. Populated from the request DTO at the route
    /// layer; the service does not query for these.
    pub writeback_context: BookingWritebackContext,

    /// Where this command originated. Routes populate from auth context.
    pub source: EventSource,
}

/// Command for [`BookingService::modify`].
#[derive(Debug, Clone)]
pub struct ModifyBookingCommand {
    pub book_id: i32,
    pub customer_id: i32,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub adults: i32,
    pub children: i32,
    pub status: String,
    pub source_label: Option<String>,
    pub total_amount: Option<f64>,
    pub deposit_amount: Option<f64>,
    pub notes: Option<String>,
    pub rooms: Vec<BookingRoomCommand>,

    /// Field-level diff carried straight through to
    /// [`WritebackIntent::ModifyBooking`].
    pub changes: BookingChanges,

    /// Snapshot context (`before` / `after`) for [`DomainEvent::BookingModified`].
    pub before_snapshot: Option<BookingSnapshotInputs>,
    pub after_snapshot: BookingSnapshotInputs,

    pub source: EventSource,
}

/// Command for [`BookingService::cancel`].
#[derive(Debug, Clone)]
pub struct CancelBookingCommand {
    pub book_id: i32,
    pub reason: Option<String>,
    pub source: EventSource,
}

/// Snapshot inputs the service uses to build a [`BookingSnapshot`] without
/// re-querying. Populated from the request DTO + the freshly-minted aggregate.
#[derive(Debug, Clone)]
pub struct BookingSnapshotInputs {
    pub legacy_book_id: Option<String>,
    pub state: BookingState,
    pub stay_start: chrono::DateTime<Utc>,
    pub stay_end: chrono::DateTime<Utc>,
    pub room_no: Option<String>,
    pub price: Money,
}

/// Free-form context the route fills in for the create-booking writeback
/// payload. Mirrors [`CreateBookingPayload`] minus the fields the service
/// derives itself (booking id / aggregate id / nights count).
#[derive(Debug, Clone)]
pub struct BookingWritebackContext {
    pub customer_aggregate_id: Uuid,
    pub legacy_cust_no: Option<String>,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub stay: DateRange,
    pub room_no: String,
    pub room_type: String,
    pub price: Money,
    pub created_by: String,
    pub notes: Option<String>,
}

/// Outcome of a successful `create` — the new repository id + event aggregate id.
#[derive(Debug, Clone)]
pub struct BookingOutcome {
    pub book_id: i32,
    pub aggregate_id: Uuid,
}

/// Service handle for the booking aggregate.
///
/// `outbox` and `events` Arcs are held for Wave 4 (when their `publish` /
/// `enqueue` become `&self` methods for mockability). Today they're
/// invoked via static calls — see [`super::customer`] for the same note.
#[derive(Clone)]
pub struct BookingService {
    pub(crate) repo: Arc<dyn BookingRepository>,
    #[allow(dead_code)]
    pub(crate) outbox: Arc<OutboxRepository>,
    #[allow(dead_code)]
    pub(crate) events: Arc<EventBus>,
    pub(crate) pg: PgPool,
}

impl BookingService {
    pub fn new(
        repo: Arc<dyn BookingRepository>,
        outbox: Arc<OutboxRepository>,
        events: Arc<EventBus>,
        pg: PgPool,
    ) -> Self {
        Self { repo, outbox, events, pg }
    }

    /// Create a booking + its assigned rooms + outbox writeback + event.
    pub async fn create(&self, cmd: CreateBookingCommand) -> ServiceResult<BookingOutcome> {
        validate_stay_range(cmd.check_in, cmd.check_out)?;
        validate_room_assignments(&cmd.rooms)?;

        let mut tx = self.pg.begin().await?;

        let book_id = self
            .repo
            .insert_booking(
                &mut tx,
                BookingWrite {
                    book_no: &cmd.book_no,
                    customer_id: cmd.customer_id,
                    check_in: cmd.check_in,
                    check_out: cmd.check_out,
                    adults: cmd.adults,
                    children: cmd.children,
                    status: &cmd.status,
                    source: cmd.source_label.as_deref(),
                    total_amount: cmd.total_amount,
                    deposit_amount: cmd.deposit_amount,
                    notes: cmd.notes.as_deref(),
                },
            )
            .await?;

        for assignment in &cmd.rooms {
            self.repo
                .insert_booking_room(
                    &mut tx,
                    book_id,
                    BookingRoomAssignment {
                        room_id: assignment.room_id,
                        price_per_night: assignment.price_per_night,
                    },
                )
                .await?;
        }

        let aggregate_id = aggregate_uuid(AggregateKind::Booking, book_id);
        // Stamp the deterministic UUID onto the row so the writeback worker's
        // resolver can map `writeback_jobs.aggregate_id` → `ht_bookings`
        // (migration 014). Same transaction as the INSERT — if the outbox
        // enqueue fails, the row never becomes visible.
        self.repo.set_aggregate_id(&mut tx, book_id, aggregate_id).await?;
        let nights = nights_between(cmd.check_in, cmd.check_out);
        let payload = CreateBookingPayload {
            customer_id: cmd.writeback_context.customer_aggregate_id,
            legacy_cust_no: cmd.writeback_context.legacy_cust_no,
            customer_name: cmd.writeback_context.customer_name,
            customer_phone: cmd.writeback_context.customer_phone,
            stay: cmd.writeback_context.stay.clone(),
            room_no: cmd.writeback_context.room_no.clone(),
            room_type: cmd.writeback_context.room_type,
            price: cmd.writeback_context.price,
            nights,
            created_by: cmd.writeback_context.created_by,
            notes: cmd.writeback_context.notes,
        };

        let intent = WritebackIntent::CreateBooking {
            booking_id: aggregate_id,
            payload,
        };
        let key = generate_idempotency_key(&intent, aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        let snapshot = BookingSnapshot {
            id: aggregate_id,
            legacy_book_id: None,
            customer_id: aggregate_uuid(AggregateKind::Customer, cmd.customer_id),
            state: parse_booking_state(&cmd.status),
            stay_start: cmd.writeback_context.stay.start,
            stay_end: cmd.writeback_context.stay.end,
            room_no: Some(cmd.writeback_context.room_no),
            price: cmd.writeback_context.price,
        };
        let event = DomainEvent::BookingCreated {
            id: aggregate_id,
            source: cmd.source.clone(),
            snapshot,
        };
        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        tx.commit().await?;

        Ok(BookingOutcome { book_id, aggregate_id })
    }

    /// Modify a booking — replaces its rooms + enqueues the writeback diff.
    ///
    /// Mirrors today's `update_booking` route: deletes the existing
    /// `ht_booking_rooms` rows and re-inserts the supplied ones inside the
    /// same TX as the `ht_bookings` UPDATE.
    pub async fn modify(&self, cmd: ModifyBookingCommand) -> ServiceResult<BookingOutcome> {
        validate_stay_range(cmd.check_in, cmd.check_out)?;
        validate_room_assignments(&cmd.rooms)?;

        let mut tx = self.pg.begin().await?;

        let rows_affected = self
            .repo
            .update_booking(
                &mut tx,
                cmd.book_id,
                BookingWrite {
                    book_no: "", // book_no is immutable post-create; repo.update ignores it.
                    customer_id: cmd.customer_id,
                    check_in: cmd.check_in,
                    check_out: cmd.check_out,
                    adults: cmd.adults,
                    children: cmd.children,
                    status: &cmd.status,
                    source: cmd.source_label.as_deref(),
                    total_amount: cmd.total_amount,
                    deposit_amount: cmd.deposit_amount,
                    notes: cmd.notes.as_deref(),
                },
            )
            .await?;

        if rows_affected == 0 {
            return Err(ServiceError::not_found(format!(
                "booking {} does not exist",
                cmd.book_id
            )));
        }

        self.repo.delete_booking_rooms(&mut tx, cmd.book_id).await?;
        for assignment in &cmd.rooms {
            self.repo
                .insert_booking_room(
                    &mut tx,
                    cmd.book_id,
                    BookingRoomAssignment {
                        room_id: assignment.room_id,
                        price_per_night: assignment.price_per_night,
                    },
                )
                .await?;
        }

        let aggregate_id = aggregate_uuid(AggregateKind::Booking, cmd.book_id);
        let intent = WritebackIntent::ModifyBooking {
            booking_id: aggregate_id,
            changes: cmd.changes,
        };
        let key = generate_idempotency_key(&intent, aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        let after = build_snapshot(aggregate_id, cmd.customer_id, &cmd.after_snapshot);
        let before = cmd
            .before_snapshot
            .as_ref()
            .map(|inputs| build_snapshot(aggregate_id, cmd.customer_id, inputs))
            .unwrap_or_else(|| after.clone());
        let event = DomainEvent::BookingModified {
            id: aggregate_id,
            source: cmd.source.clone(),
            before,
            after,
        };
        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        tx.commit().await?;

        Ok(BookingOutcome {
            book_id: cmd.book_id,
            aggregate_id,
        })
    }

    /// Cancel a booking — repository updates `book_status='cancelled'` only
    /// for non-terminal rows, and we publish + enqueue accordingly.
    ///
    /// Returns `ServiceError::Conflict` when the row is already terminal
    /// (the repository reports `0 rows_affected` because of its `NOT IN
    /// ('completed', 'cancelled')` guard).
    pub async fn cancel(&self, cmd: CancelBookingCommand) -> ServiceResult<BookingOutcome> {
        let mut tx = self.pg.begin().await?;

        let rows_affected = self.repo.cancel(&mut tx, cmd.book_id).await?;
        if rows_affected == 0 {
            return Err(ServiceError::conflict(format!(
                "booking {} is missing or already terminal",
                cmd.book_id
            )));
        }

        let aggregate_id = aggregate_uuid(AggregateKind::Booking, cmd.book_id);
        let intent = WritebackIntent::CancelBooking { booking_id: aggregate_id };
        let key = generate_idempotency_key(&intent, aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        let event = DomainEvent::BookingCancelled {
            id: aggregate_id,
            source: cmd.source.clone(),
            reason: cmd.reason,
        };
        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        tx.commit().await?;

        Ok(BookingOutcome {
            book_id: cmd.book_id,
            aggregate_id,
        })
    }
}

/// Reject empty room lists + non-positive prices. The legacy app permits
/// "no-room bookings" (a placeholder), so we mirror that — empty `rooms` is
/// allowed; only individually invalid rows are rejected.
fn validate_room_assignments(rooms: &[BookingRoomCommand]) -> ServiceResult<()> {
    for room in rooms {
        if let Some(price) = room.price_per_night {
            if price < 0.0 {
                return Err(ServiceError::validation(format!(
                    "price_per_night for room {} must be non-negative, got {}",
                    room.room_id, price
                )));
            }
        }
    }
    Ok(())
}

/// Reject reversed / zero-night stays.
fn validate_stay_range(check_in: NaiveDate, check_out: NaiveDate) -> ServiceResult<()> {
    if check_out <= check_in {
        return Err(ServiceError::validation(format!(
            "check_out ({}) must be after check_in ({})",
            check_out, check_in
        )));
    }
    Ok(())
}

/// Best-effort string → [`BookingState`] conversion. Falls back to `Pending`
/// for unknown / missing values (matching the prior route default).
fn parse_booking_state(raw: &str) -> BookingState {
    match raw.trim().to_lowercase().as_str() {
        "active" | "confirmed" => BookingState::Active,
        "checkedin" | "checked_in" | "checked-in" => BookingState::CheckedIn,
        "completed" => BookingState::Completed,
        "cancelled" | "canceled" => BookingState::Cancelled,
        _ => BookingState::Pending,
    }
}

/// Compute the integer night count between two `NaiveDate` values. Used to
/// populate [`CreateBookingPayload::nights`] for the writeback recipe.
fn nights_between(check_in: NaiveDate, check_out: NaiveDate) -> i32 {
    (check_out - check_in).num_days().max(0) as i32
}

/// Build a [`BookingSnapshot`] from snapshot inputs + the booking aggregate id.
fn build_snapshot(
    booking_aggregate_id: Uuid,
    customer_id: i32,
    inputs: &BookingSnapshotInputs,
) -> BookingSnapshot {
    BookingSnapshot {
        id: booking_aggregate_id,
        legacy_book_id: inputs.legacy_book_id.clone(),
        customer_id: aggregate_uuid(AggregateKind::Customer, customer_id),
        state: inputs.state,
        stay_start: inputs.stay_start,
        stay_end: inputs.stay_end,
        room_no: inputs.room_no.clone(),
        price: inputs.price,
    }
}

/// Convenience: convert a `NaiveDate` (legacy schema) to a `DateTime<Utc>`
/// at midnight. Useful for callers that have a `NaiveDate` but need to fill
/// in a [`DateRange`] for writeback context.
pub fn naive_date_to_utc(date: NaiveDate) -> chrono::DateTime<Utc> {
    let midnight = NaiveTime::from_hms_opt(0, 0, 0).expect("hardcoded midnight is valid");
    Utc.from_utc_datetime(&date.and_time(midnight))
}
