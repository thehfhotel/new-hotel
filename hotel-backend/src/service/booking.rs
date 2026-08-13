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
use crate::repository::booking::{
    BookingProductAssignment, BookingRepository, BookingRoomAssignment, BookingWrite,
};

use super::error::{ServiceError, ServiceResult};
use super::ids::{aggregate_uuid, AggregateKind};

/// One assigned room within a booking command.
#[derive(Debug, Clone, Copy)]
pub struct BookingRoomCommand {
    pub room_id: i32,
    pub price_per_night: Option<f64>,
}

/// One pre-ordered product line within a create-booking command (task #52).
/// Persisted canonically in `ht_booking_products`; the legacy `HT_Book_Pro`
/// write-back is deferred (shape unverified), so these never enqueue a
/// writeback intent today.
#[derive(Debug, Clone)]
pub struct BookingProductCommand {
    pub product_id: i64,
    pub qty: f64,
    /// `None` ⇒ default from the product's catalog price at INSERT time.
    pub unit_price: Option<f64>,
    pub note: Option<String>,
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

    /// Pre-ordered product lines (task #52). Optional — empty for the common
    /// case. Persisted canonically; no legacy write-back today.
    pub products: Vec<BookingProductCommand>,

    /// Snapshot context used to build the [`CreateBookingPayload`] sent to
    /// the writeback worker. Populated from the request DTO at the route
    /// layer; the service does not query for these.
    pub writeback_context: BookingWritebackContext,

    /// OTA provenance / caller-idempotency natural key (migration 076).
    /// `book_channel` = the source channel; `book_ext_ref` = that channel's
    /// own booking id. Both-or-neither in practice; when both are set,
    /// [`BookingService::create`] dedupes on `(channel, ext_ref)` so a
    /// double-POST of one OTA reservation cannot create two bookings. Both
    /// `None` for every existing (walk-in / manual) caller — unchanged path.
    pub book_channel: Option<String>,
    pub book_ext_ref: Option<String>,

    /// Payment-hold deadline (migration 086 — loyalty-channel TENTATIVE
    /// holds). `Some(_)` ⇒ the row is stamped with `book_hold_expires_at`
    /// in the SAME transaction as the insert, so a hold can never commit
    /// without the deadline the expiry sweep keys on. `None` for every
    /// non-channel caller — unchanged path. PG-canonical only.
    pub hold_expires_at: Option<chrono::DateTime<Utc>>,

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

    /// Write-back context for the promote-to-`CreateBooking` path — a parked
    /// (roomless, never-mirrored) booking getting its FIRST room via the edit
    /// flow, which must produce the real byte-parity iHOTEL booking. Built by
    /// the route (same helper as create) when `rooms` is non-empty; consumed
    /// only when [`modify_writeback_plan`] returns [`ModifyWriteback::Create`].
    /// `None` for a roomless edit — there's nothing to mirror.
    pub promote_context: Option<BookingWritebackContext>,

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
    /// Deposit (`เงินมัดจำ`) the receptionist entered on the booking form.
    /// Maps to legacy `HT_Book_H.Book_Price_Pay`. Defaults to zero — most
    /// bookings have no upfront deposit.
    pub deposit: Money,
    pub created_by: String,
    pub notes: Option<String>,
}

/// Outcome of a successful `create` — the new repository id + event aggregate id.
#[derive(Debug, Clone)]
pub struct BookingOutcome {
    pub book_id: i32,
    pub aggregate_id: Uuid,
    /// The EXISTING booking's number when `create` short-circuited on the OTA
    /// caller-idempotency key (repeat create with the same `(channel,
    /// ext_ref)`). `None` on the normal create path (the caller already holds
    /// the freshly-generated number) and from `modify` / `cancel`.
    pub book_no: Option<String>,
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

        // Caller idempotency (migration 076 — OTA Desk Phase 0). When this
        // create carries an OTA natural key (channel + external ref) and a
        // booking with that pair already exists, a prior create already
        // inserted the canonical row AND enqueued its byte-parity legacy
        // write-back. Return that booking unchanged — do NOT insert, enqueue,
        // or publish again — so a double-POST of one OTA reservation cannot
        // mint a second ht_bookings row → a second real iHOTEL booking. Only
        // both-present keys dedupe; every existing (walk-in / manual) caller
        // passes neither and is unaffected.
        if let (Some(channel), Some(ext_ref)) =
            (cmd.book_channel.as_deref(), cmd.book_ext_ref.as_deref())
        {
            if let Some((existing_id, existing_no)) = self
                .repo
                .find_by_channel_ext_ref(&self.pg, channel, ext_ref)
                .await?
            {
                return Ok(BookingOutcome {
                    book_id: existing_id,
                    aggregate_id: aggregate_uuid(AggregateKind::Booking, existing_id),
                    book_no: Some(existing_no),
                });
            }
        }

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

        // Stamp the OTA provenance + enforce the (channel, ext_ref) natural key
        // (migration 076). The pre-check above handles the common sequential
        // double-POST; this UPDATE plus the partial UNIQUE index are the
        // serializer-of-last-resort for a tight concurrent race that slips two
        // creates past the SELECT. On the losing side we roll back this
        // half-built row and return the winner's booking (idempotent — no
        // duplicate committed either way).
        if let (Some(channel), None) = (cmd.book_channel.as_deref(), cmd.book_ext_ref.as_deref()) {
            // Channel-only provenance (loyalty holds — no caller-side booking
            // id exists, so there is no natural key to dedupe on; the channel
            // label alone drives the expiry sweep + channel-API guards).
            self.repo.set_booking_channel(&mut tx, book_id, channel).await?;
        }

        if let (Some(channel), Some(ext_ref)) =
            (cmd.book_channel.as_deref(), cmd.book_ext_ref.as_deref())
        {
            match self
                .repo
                .set_booking_provenance(&mut tx, book_id, channel, ext_ref)
                .await
            {
                Ok(()) => {}
                Err(err) if is_unique_violation(&err) => {
                    // Concurrent create won the race; our tx is poisoned.
                    // Dropping it rolls back this row, then return the row the
                    // winner committed.
                    drop(tx);
                    let (existing_id, existing_no) = self
                        .repo
                        .find_by_channel_ext_ref(&self.pg, channel, ext_ref)
                        .await?
                        .ok_or_else(|| {
                            ServiceError::internal(
                                "unique violation on (book_channel, book_ext_ref) but no \
                                 matching booking found on re-select",
                            )
                        })?;
                    return Ok(BookingOutcome {
                        book_id: existing_id,
                        aggregate_id: aggregate_uuid(AggregateKind::Booking, existing_id),
                        book_no: Some(existing_no),
                    });
                }
                Err(err) => return Err(ServiceError::from(err)),
            }
        }

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

        // Pre-ordered products (task #52) — canonical-only. Each line gets a
        // stable v4 aggregate id so a future legacy write-back / event path can
        // correlate without a back-link round-trip.
        for product in &cmd.products {
            self.repo
                .insert_booking_product(
                    &mut tx,
                    book_id,
                    BookingProductAssignment {
                        product_id: product.product_id,
                        qty: product.qty,
                        unit_price: product.unit_price,
                        note: product.note.clone(),
                        aggregate_id: Uuid::new_v4(),
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

        // Loyalty-channel hold deadline (migration 086) — same-transaction
        // stamp so a hold can never commit without its expiry.
        if let Some(expires_at) = cmd.hold_expires_at {
            self.repo.set_hold_expiry(&mut tx, book_id, expires_at).await?;
        }

        let nights = nights_between(cmd.check_in, cmd.check_out);

        // Waitlist / unassigned booking (task #52): a zero-room booking has no
        // room number to mirror, and the `CreateBooking` recipe keys every
        // `HT_Book_Ds` / `HT_Book_Date` / `HT_Room_Status` row on the room
        // number — emitting them with an empty room number would write
        // malformed rows into the SHARED legacy DB. The legacy shape of a
        // roomless booking is unverified, so we skip the legacy mirror for
        // these and keep the booking canonical-only. The domain event below
        // still fires (drives the dashboard/SSE), and once a room is assigned
        // via a later edit the normal write path takes over.
        //
        // TODO(task#52 follow-up): if iHOTEL's no-room placeholder shape is
        // ever captured in `docs/legacy-spike/findings.md`, enqueue a tailored
        // intent here so waitlist bookings also surface in the .NET app.
        if !cmd.rooms.is_empty() {
            let payload = CreateBookingPayload {
                customer_id: cmd.writeback_context.customer_aggregate_id,
                legacy_cust_no: cmd.writeback_context.legacy_cust_no,
                customer_name: cmd.writeback_context.customer_name.clone(),
                customer_phone: cmd.writeback_context.customer_phone.clone(),
                stay: cmd.writeback_context.stay.clone(),
                room_no: cmd.writeback_context.room_no.clone(),
                room_type: cmd.writeback_context.room_type.clone(),
                price: cmd.writeback_context.price,
                nights,
                deposit: cmd.writeback_context.deposit,
                created_by: cmd.writeback_context.created_by.clone(),
                notes: cmd.writeback_context.notes.clone(),
            };

            let intent = WritebackIntent::CreateBooking {
                booking_id: aggregate_id,
                payload,
            };
            let key = generate_idempotency_key(&intent, aggregate_id);
            OutboxRepository::enqueue(&mut tx, &intent, key)
                .await
                .map_err(ServiceError::from_enqueue_error)?;
        }

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

        Ok(BookingOutcome {
            book_id,
            aggregate_id,
            book_no: None,
        })
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

        // Capture the pre-modify write-back state BEFORE we touch the rooms:
        // whether the booking is already mirrored to iHOTEL (`legacy_book_id`)
        // and how many rooms it has right now. A parked (roomless, never
        // mirrored) booking that gains its FIRST room here must produce the real
        // byte-parity `CreateBooking`, not a `ModifyBooking` that has no legacy
        // row to target. Missing row ⇒ `(None, 0)`; `update_booking` below
        // still returns the not-found error.
        let (prior_legacy_book_id, prior_room_count) = self
            .repo
            .writeback_state(&mut tx, cmd.book_id)
            .await?
            .unwrap_or((None, 0));

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

        // Choose the legacy write-back leg. `legacy_book_id` is treated as
        // "mirrored" only when non-empty (matches the dispatcher's `nonempty`
        // gate). See [`modify_writeback_plan`] for the full matrix.
        let legacy = prior_legacy_book_id.as_deref().filter(|s| !s.is_empty());
        match modify_writeback_plan(legacy, prior_room_count, cmd.rooms.len()) {
            ModifyWriteback::Create => {
                // Promote: a parked roomless booking got its first room → emit
                // the SAME byte-parity CreateBooking an at-create-time room
                // would have. The route builds `promote_context` (customer +
                // first room) exactly like the create path's
                // `build_writeback_context`, so the recipe output is identical.
                let ctx = cmd.promote_context.as_ref().ok_or_else(|| {
                    ServiceError::internal(
                        "room-assign promote requires a write-back context but none was supplied",
                    )
                })?;
                let nights = nights_between(cmd.check_in, cmd.check_out);
                let payload = CreateBookingPayload {
                    customer_id: ctx.customer_aggregate_id,
                    legacy_cust_no: ctx.legacy_cust_no.clone(),
                    customer_name: ctx.customer_name.clone(),
                    customer_phone: ctx.customer_phone.clone(),
                    stay: ctx.stay.clone(),
                    room_no: ctx.room_no.clone(),
                    room_type: ctx.room_type.clone(),
                    price: ctx.price,
                    nights,
                    deposit: ctx.deposit,
                    created_by: ctx.created_by.clone(),
                    notes: ctx.notes.clone(),
                };
                let intent = WritebackIntent::CreateBooking {
                    booking_id: aggregate_id,
                    payload,
                };
                // Deterministic (intent, aggregate) key — IDENTICAL to what an
                // at-create-time CreateBooking for this book_id would use, so a
                // crash-after-commit retry or a duplicate promote maps to the
                // same `dbo.ht_writeback_ledger` row (no double legacy write),
                // and the worker back-populates `legacy_book_id` onto this row.
                // The row's `aggregate_id` was stamped at create time.
                let key = generate_idempotency_key(&intent, aggregate_id);
                OutboxRepository::enqueue(&mut tx, &intent, key)
                    .await
                    .map_err(ServiceError::from_enqueue_error)?;
            }
            ModifyWriteback::Modify => {
                let intent = WritebackIntent::ModifyBooking {
                    booking_id: aggregate_id,
                    changes: cmd.changes,
                };
                // Repeatable-per-aggregate intent: the SECOND occurrence for the
                // same aggregate would collide on the permanently-retained
                // `writeback_jobs.idempotency_key` UNIQUE if we used the
                // deterministic (intent, aggregate) key — completed jobs stay as
                // status='done' rows. Per-event v4 discriminator instead (same
                // precedent as payment/customer-update; see outbox/idempotency.rs
                // "caller adds a discriminator"). 2026-06-12 audit follow-up.
                let key = generate_idempotency_key(&intent, uuid::Uuid::new_v4());
                OutboxRepository::enqueue(&mut tx, &intent, key)
                    .await
                    .map_err(ServiceError::from_enqueue_error)?;
            }
            ModifyWriteback::Skip => {
                // Roomless AND never mirrored → canonical-only, no legacy write
                // (matches create's roomless behavior). Previously this enqueued
                // a ModifyBooking that could never resolve its legacy id; now it
                // is a clean no-op. The domain event below still fires.
            }
        }

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
            book_no: None,
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
            .map_err(ServiceError::from_enqueue_error)?;

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
            book_no: None,
        })
    }
}

/// True when `err` is a PostgreSQL unique-constraint violation (SQLSTATE
/// 23505). Mirrors the detection idiom in `service::shifts` and the
/// `create_user` / `set_user_card` bins.
fn is_unique_violation(err: &sqlx::Error) -> bool {
    if let sqlx::Error::Database(db_err) = err {
        db_err.code().as_deref() == Some("23505")
    } else {
        false
    }
}

/// Which legacy write-back leg a [`BookingService::modify`] should take.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModifyWriteback {
    /// Promote to a byte-parity `CreateBooking` — a parked (roomless, never
    /// mirrored) booking is getting its FIRST room, so the real iHOTEL booking
    /// must be created now (there is nothing to modify yet).
    Create,
    /// Normal targeted `ModifyBooking` against an existing legacy booking (or a
    /// create still in flight whose `legacy_book_id` hasn't back-populated yet).
    Modify,
    /// No legacy write — the booking is roomless AND never mirrored, so there is
    /// nothing in iHOTEL to create or modify (matches create's roomless rule).
    Skip,
}

/// Decide the modify write-back leg from the pre-modify state. Pure — no I/O —
/// so the full matrix is unit-tested without a database.
///
/// * `legacy_book_id` — the booking's mirrored legacy id, already normalised to
///   `None` when NULL/empty (an empty string is "not mirrored", matching the
///   dispatcher's `nonempty` gate).
/// * `prior_room_count` — rooms the booking had BEFORE this modify replaced them.
/// * `new_room_count` — rooms this modify assigns.
///
/// | legacy | prior rooms | new rooms | leg | why |
/// |---|---|---|---|---|
/// | none | 0 | ≥1 | **Create** | parked booking gets its first room → real iHOTEL create |
/// | none | 0 | 0 | **Skip** | still roomless, nothing to mirror |
/// | none | ≥1 | any | **Modify** | create write-back in flight; ModifyBooking resolves once it lands |
/// | some | any | any | **Modify** | already mirrored → targeted modify |
fn modify_writeback_plan(
    legacy_book_id: Option<&str>,
    prior_room_count: i64,
    new_room_count: usize,
) -> ModifyWriteback {
    match legacy_book_id {
        Some(_) => ModifyWriteback::Modify,
        None => {
            if prior_room_count == 0 && new_room_count > 0 {
                ModifyWriteback::Create
            } else if prior_room_count == 0 {
                ModifyWriteback::Skip
            } else {
                ModifyWriteback::Modify
            }
        }
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

#[cfg(test)]
mod modify_writeback_plan_tests {
    use super::*;

    // (b) A parked (roomless, never-mirrored) booking that gains its FIRST room
    // via the edit flow must PROMOTE to the byte-parity CreateBooking, so the
    // front desk assigning the room in the PMS produces the real iHOTEL booking.
    #[test]
    fn parked_roomless_booking_gaining_first_room_promotes_to_create() {
        assert_eq!(modify_writeback_plan(None, 0, 1), ModifyWriteback::Create);
        assert_eq!(modify_writeback_plan(None, 0, 5), ModifyWriteback::Create);
    }

    // A still-roomless, never-mirrored booking has nothing to mirror — no doomed
    // ModifyBooking (the previous behavior), just a clean skip.
    #[test]
    fn roomless_staying_roomless_and_unmirrored_skips_legacy() {
        assert_eq!(modify_writeback_plan(None, 0, 0), ModifyWriteback::Skip);
    }

    // Had rooms at create (CreateBooking already queued) but legacy id not
    // back-populated yet: keep ModifyBooking — it resolves once the in-flight
    // create lands. Must NOT re-enqueue a second CreateBooking.
    #[test]
    fn unmirrored_with_rooms_still_modifies_create_in_flight() {
        assert_eq!(modify_writeback_plan(None, 1, 1), ModifyWriteback::Modify);
        assert_eq!(modify_writeback_plan(None, 2, 0), ModifyWriteback::Modify);
        assert_eq!(modify_writeback_plan(None, 1, 2), ModifyWriteback::Modify);
    }

    // (d) An already-mirrored booking (resolved legacy_book_id) always takes the
    // normal targeted ModifyBooking — regardless of the room delta, including a
    // mirrored booking that is currently roomless re-gaining a room (the legacy
    // HT_Book_H already exists; don't re-create it).
    #[test]
    fn already_mirrored_booking_always_modifies() {
        assert_eq!(modify_writeback_plan(Some("R012345"), 1, 1), ModifyWriteback::Modify);
        assert_eq!(modify_writeback_plan(Some("R012345"), 0, 1), ModifyWriteback::Modify);
        assert_eq!(modify_writeback_plan(Some("R012345"), 1, 0), ModifyWriteback::Modify);
        assert_eq!(modify_writeback_plan(Some("R012345"), 0, 0), ModifyWriteback::Modify);
    }

    // (c) Retry idempotency: the promoted CreateBooking's key depends ONLY on
    // (intent variant, aggregate_id) — never the payload — so a crash-after-commit
    // retry or a duplicate promote maps to the SAME writeback_jobs.idempotency_key
    // / ledger row as a normal at-create-time CreateBooking → no double legacy write.
    #[test]
    fn promoted_create_key_is_deterministic_and_payload_independent() {
        let agg = aggregate_uuid(AggregateKind::Booking, 4242);
        let mk = |name: &str| WritebackIntent::CreateBooking {
            booking_id: agg,
            payload: CreateBookingPayload {
                customer_id: Uuid::nil(),
                legacy_cust_no: None,
                customer_name: "T".into(),
                customer_phone: None,
                stay: DateRange::new(
                    Utc.with_ymd_and_hms(2026, 7, 20, 0, 0, 0).unwrap(),
                    Utc.with_ymd_and_hms(2026, 7, 22, 0, 0, 0).unwrap(),
                ),
                room_no: "402".into(),
                room_type: "DLX".into(),
                price: Money::from_baht(1200),
                nights: 2,
                deposit: Money::ZERO,
                created_by: name.into(),
                notes: None,
            },
        };
        assert_eq!(
            generate_idempotency_key(&mk("a"), agg),
            generate_idempotency_key(&mk("b"), agg),
            "same (CreateBooking, aggregate) must yield the same key regardless of payload"
        );
    }
}
