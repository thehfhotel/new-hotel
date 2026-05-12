//! Check-in service — orchestrates `ht_checkins` writes plus outbox + events.
//!
//! Per `docs/architecture.md` §1, §6 and `docs/legacy-spike/findings.md`
//! §3a (walk-in), §3d (linked to existing booking), §3e (check-out),
//! §3f (extend), §3i (cancel-check-in).
//!
//! Each public method opens one PG transaction and:
//!
//! 1. Writes the canonical `ht_checkins` change through the repository.
//! 2. Updates dependent state (`ht_rooms_new.room_status`,
//!    `ht_bookings.book_status`) inside the same TX so the snapshot stays
//!    consistent.
//! 3. Enqueues the matching [`WritebackIntent`] in the outbox.
//! 4. Publishes a [`DomainEvent`] on the bus.
//! 5. Commits — all four effects atomic.

use std::sync::Arc;

use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::checkin::CheckInState;
use crate::domain::shared::{DateRange, Money};
use crate::outbox::event::{CheckInSnapshot, DomainEvent, EventSource};
use crate::outbox::intent::{CreateCheckInPayload, WritebackIntent};
use crate::outbox::{generate_idempotency_key, EventBus, OutboxRepository};
use crate::repository::checkin::{CheckInInsert, CheckInRepository, CheckOutWrite};

use super::error::{ServiceError, ServiceResult};
use super::ids::{aggregate_uuid, AggregateKind};

/// Common fields the writeback recipe needs to mint the legacy
/// `HT_CheckIn_*` rows for both walk-in and linked-to-booking flows.
#[derive(Debug, Clone)]
pub struct CheckInWritebackContext {
    pub legacy_cust_no: Option<String>,
    pub linked_legacy_book_id: Option<String>,
    pub room_no: String,
    pub room_type: String,
    pub stay: DateRange,
    pub price_per_night: Money,
    pub nights: i32,
    pub price_total: Money,
    pub created_by: String,
    pub guest_name_for_registry: String,
    pub guest_country: String,
}

/// Command for [`CheckInService::walk_in`] — no prior booking exists.
/// Maps to spike §3a (walk-in recipe).
#[derive(Debug, Clone)]
pub struct WalkInCommand {
    pub cin_no: String,
    pub customer_id: i32,
    pub room_id: i32,
    pub check_in_time: Option<NaiveDateTime>,
    pub expected_checkout: NaiveDate,
    pub adults: i32,
    pub children: i32,
    pub rate_per_night: Option<f64>,
    pub notes: Option<String>,
    pub writeback_context: CheckInWritebackContext,
    pub source: EventSource,
}

/// Command for [`CheckInService::check_in_to_booking`] — the guest has an
/// advance booking we are now activating. Maps to spike §3d.
#[derive(Debug, Clone)]
pub struct CheckInToBookingCommand {
    pub cin_no: String,
    pub booking_id: i32,
    pub room_id: i32,
    pub check_in_time: Option<NaiveDateTime>,
    pub expected_checkout: NaiveDate,
    pub adults: i32,
    pub children: i32,
    pub rate_per_night: Option<f64>,
    pub notes: Option<String>,
    pub writeback_context: CheckInWritebackContext,
    pub source: EventSource,
}

/// Command for [`CheckInService::cancel`].
///
/// `room_price` + `pay_to_subtract` ride into [`WritebackIntent::CancelCheckIn`]
/// so the recipe can SUBTRACT the right amounts from `HT_CheckIn_H` totals
/// (multi-room safe per spike §3i). Routes look these up from the canonical
/// PG state — defaulting to `Money::ZERO` only when the room being cancelled
/// is missing from the join (degrades gracefully — totals stay unchanged on
/// MSSQL, but the cancel still proceeds).
#[derive(Debug, Clone)]
pub struct CancelCheckInCommand {
    pub check_in_id: i32,
    pub reason: Option<String>,
    /// `Cin_Room_Price` of the cancelled room, in satang. Subtracted from
    /// `HT_CheckIn_H.Total_Price_Room` / `Net` / `Balance`.
    pub room_price: Money,
    /// Pay portion to subtract from `Total_Price_Pay`. Usually `Money::ZERO`.
    pub pay_to_subtract: Money,
    pub source: EventSource,
}

/// Command for [`CheckInService::extend`].
#[derive(Debug, Clone)]
pub struct ExtendStayCommand {
    pub check_in_id: i32,
    /// New (later) departure boundary. Must be strictly after the current
    /// expected checkout — the service enforces this via the repository's
    /// `apply_checkout` semantics is NOT used here; this is a separate flow
    /// the repository will own in a later wave. Today we only emit the
    /// writeback intent + event so the worker can apply the legacy diff.
    pub new_end: DateTime<Utc>,
    /// Original `Cin_Room_In` from the canonical PG state. The recipe enumerates
    /// calendar nights in `[stay_start, new_end)` so the legacy `HT_Room_Status`
    /// rows cover the full range (not just `[today, new_end)`).
    pub stay_start: DateTime<Utc>,
    /// Customer name to display in `HT_Room_Status.room_Details` (the .NET
    /// app's calendar grid label).
    pub guest_label: String,
    /// Recomputed totals to push into `HT_CheckIn_H`. All in satang.
    pub new_room_price_total: Money,
    pub new_net_total: Money,
    pub new_pay_total: Money,
    pub new_balance_total: Money,
    pub source: EventSource,
}

/// Command for [`CheckInService::check_out`].
///
/// The revenue/nights/balance fields ride into
/// [`WritebackIntent::CheckOut`] so the legacy `HT_CheckIn_Ds` +
/// `HT_CheckIn_H` UPDATEs receive real values, not zeros (audit H1). All
/// `*_total` figures are baht (legacy column type is MONEY/decimal in
/// baht, not satang).
#[derive(Debug, Clone)]
pub struct CheckOutCommand {
    pub check_in_id: i32,
    pub check_out_time: Option<NaiveDateTime>,
    pub total_amount: Option<f64>,
    pub payment_status: String,
    pub notes: Option<String>,
    pub source: EventSource,
    /// Nights actually stayed (>=1).
    pub nights: f64,
    /// Total room revenue in baht (rate per night × nights).
    pub room_price_total: f64,
    /// Total minibar / extras revenue in baht (0 today — no
    /// `HT_CheckIn_Product` plumbing yet).
    pub product_total: f64,
    /// Net (room + product) revenue in baht.
    pub net_total: f64,
    /// Total payments tendered in baht.
    pub pay_total: f64,
    /// Outstanding balance in baht (net − pay).
    pub balance: f64,
}

/// Outcome of a successful check-in mutation.
#[derive(Debug, Clone)]
pub struct CheckInOutcome {
    pub check_in_id: i32,
    pub aggregate_id: Uuid,
}

/// Service handle for the check-in aggregate.
///
/// `outbox` Arc is held for Wave 4 — see [`super::customer`] note.
#[derive(Clone)]
pub struct CheckInService {
    pub(crate) repo: Arc<dyn CheckInRepository>,
    #[allow(dead_code)]
    pub(crate) outbox: Arc<OutboxRepository>,
    pub(crate) events: Arc<EventBus>,
    pub(crate) pg: PgPool,
}

impl CheckInService {
    pub fn new(
        repo: Arc<dyn CheckInRepository>,
        outbox: Arc<OutboxRepository>,
        events: Arc<EventBus>,
        pg: PgPool,
    ) -> Self {
        Self { repo, outbox, events, pg }
    }

    /// Walk-in flow — no prior booking, single-step room occupancy.
    pub async fn walk_in(&self, cmd: WalkInCommand) -> ServiceResult<CheckInOutcome> {
        validate_party_size(cmd.adults, cmd.children)?;

        let mut tx = self.pg.begin().await?;

        let active_count = self
            .repo
            .count_active_for_room(&self.pg, cmd.room_id)
            .await?;
        if active_count > 0 {
            return Err(ServiceError::conflict(format!(
                "room {} already has an active check-in",
                cmd.room_id
            )));
        }

        let cin_id = self
            .repo
            .insert(
                &mut tx,
                CheckInInsert {
                    cin_no: &cmd.cin_no,
                    booking_id: None,
                    customer_id: cmd.customer_id,
                    room_id: cmd.room_id,
                    check_in_time: cmd.check_in_time,
                    expected_checkout: cmd.expected_checkout,
                    adults: cmd.adults,
                    children: cmd.children,
                    rate_per_night: cmd.rate_per_night,
                    notes: cmd.notes.as_deref(),
                },
            )
            .await?;

        self.repo.mark_room_occupied(&mut tx, cmd.room_id).await?;

        self.enqueue_create_check_in(
            &mut tx,
            cin_id,
            None,
            cmd.customer_id,
            &cmd.writeback_context,
        )
        .await?;

        let aggregate_id = aggregate_uuid(AggregateKind::CheckIn, cin_id);
        // Stamp UUID on the row for the writeback resolver (migration 014).
        self.repo.set_aggregate_id(&mut tx, cin_id, aggregate_id).await?;
        let snapshot = build_check_in_snapshot(
            aggregate_id,
            None,
            cmd.customer_id,
            CheckInState::Active,
            &cmd.writeback_context,
        );
        let event = DomainEvent::CheckInCreated {
            id: aggregate_id,
            source: cmd.source.clone(),
            snapshot,
        };
        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        tx.commit().await?;

        Ok(CheckInOutcome { check_in_id: cin_id, aggregate_id })
    }

    /// Activate an existing booking — links the new `ht_checkins` row to the
    /// booking and marks the booking as `checkedin` in the same TX.
    pub async fn check_in_to_booking(
        &self,
        cmd: CheckInToBookingCommand,
    ) -> ServiceResult<CheckInOutcome> {
        validate_party_size(cmd.adults, cmd.children)?;

        let customer_id = self
            .repo
            .get_booking_customer_id(&self.pg, cmd.booking_id)
            .await?
            .ok_or_else(|| {
                ServiceError::not_found(format!("booking {} does not exist", cmd.booking_id))
            })?;

        let active_count = self
            .repo
            .count_active_for_room(&self.pg, cmd.room_id)
            .await?;
        if active_count > 0 {
            return Err(ServiceError::conflict(format!(
                "room {} already has an active check-in",
                cmd.room_id
            )));
        }

        let mut tx = self.pg.begin().await?;

        let cin_id = self
            .repo
            .insert(
                &mut tx,
                CheckInInsert {
                    cin_no: &cmd.cin_no,
                    booking_id: Some(cmd.booking_id),
                    customer_id,
                    room_id: cmd.room_id,
                    check_in_time: cmd.check_in_time,
                    expected_checkout: cmd.expected_checkout,
                    adults: cmd.adults,
                    children: cmd.children,
                    rate_per_night: cmd.rate_per_night,
                    notes: cmd.notes.as_deref(),
                },
            )
            .await?;

        self.repo.mark_room_occupied(&mut tx, cmd.room_id).await?;
        self.repo
            .set_booking_checkedin(&mut tx, cmd.booking_id)
            .await?;

        self.enqueue_create_check_in(
            &mut tx,
            cin_id,
            Some(cmd.booking_id),
            customer_id,
            &cmd.writeback_context,
        )
        .await?;

        let aggregate_id = aggregate_uuid(AggregateKind::CheckIn, cin_id);
        // Stamp UUID on the row for the writeback resolver (migration 014).
        self.repo.set_aggregate_id(&mut tx, cin_id, aggregate_id).await?;
        let snapshot = build_check_in_snapshot(
            aggregate_id,
            Some(cmd.booking_id),
            customer_id,
            CheckInState::Active,
            &cmd.writeback_context,
        );
        let event = DomainEvent::CheckInCreated {
            id: aggregate_id,
            source: cmd.source.clone(),
            snapshot,
        };
        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        tx.commit().await?;

        Ok(CheckInOutcome { check_in_id: cin_id, aggregate_id })
    }

    /// Cancel an active check-in — emits the writeback + event but does NOT
    /// (yet) mutate `ht_checkins` itself. The repository does not currently
    /// expose a cancel SQL; Wave 4 will add it. Today the cancel flow is the
    /// outbox + event publication so subscribers (audit, SSE) react.
    pub async fn cancel(&self, cmd: CancelCheckInCommand) -> ServiceResult<CheckInOutcome> {
        let status = self
            .repo
            .find_status(&self.pg, cmd.check_in_id)
            .await?
            .ok_or_else(|| {
                ServiceError::not_found(format!(
                    "check-in {} does not exist",
                    cmd.check_in_id
                ))
            })?;

        if matches!(status.cin_status.as_deref(), Some("cancelled") | Some("checkedout")) {
            return Err(ServiceError::conflict(format!(
                "check-in {} is already terminal ({:?})",
                cmd.check_in_id, status.cin_status
            )));
        }

        let mut tx = self.pg.begin().await?;
        let aggregate_id = aggregate_uuid(AggregateKind::CheckIn, cmd.check_in_id);

        let intent = WritebackIntent::CancelCheckIn {
            check_in_id: aggregate_id,
            reason: cmd.reason.clone(),
            room_price: cmd.room_price,
            pay_to_subtract: cmd.pay_to_subtract,
        };
        let key = generate_idempotency_key(&intent, aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        let event = DomainEvent::CheckInCancelled {
            id: aggregate_id,
            source: cmd.source.clone(),
            reason: cmd.reason,
        };
        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        tx.commit().await?;

        Ok(CheckInOutcome {
            check_in_id: cmd.check_in_id,
            aggregate_id,
        })
    }

    /// Extend an active stay — emits the writeback intent + a synthetic
    /// `CheckInCreated`-shaped event is NOT used. Today this flow only
    /// publishes the writeback so the worker can re-write `HT_Book_Date` /
    /// `HT_CheckIn_Ds` per spike §3f. UI refreshes via React Query
    /// invalidation on the next bookings/check-ins poll.
    pub async fn extend(&self, cmd: ExtendStayCommand) -> ServiceResult<CheckInOutcome> {
        let status = self
            .repo
            .find_status(&self.pg, cmd.check_in_id)
            .await?
            .ok_or_else(|| {
                ServiceError::not_found(format!(
                    "check-in {} does not exist",
                    cmd.check_in_id
                ))
            })?;

        if !matches!(status.cin_status.as_deref(), Some("active")) {
            return Err(ServiceError::conflict(format!(
                "check-in {} is not active ({:?})",
                cmd.check_in_id, status.cin_status
            )));
        }

        let mut tx = self.pg.begin().await?;
        let aggregate_id = aggregate_uuid(AggregateKind::CheckIn, cmd.check_in_id);

        let intent = WritebackIntent::ExtendStay {
            check_in_id: aggregate_id,
            new_end: cmd.new_end,
            stay_start: cmd.stay_start,
            guest_label: cmd.guest_label.clone(),
            new_room_price_total: cmd.new_room_price_total,
            new_net_total: cmd.new_net_total,
            new_pay_total: cmd.new_pay_total,
            new_balance_total: cmd.new_balance_total,
        };
        let key = generate_idempotency_key(&intent, aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        // Extend has no dedicated DomainEvent variant today (architecture.md
        // §3.6b lists only Created / Cancelled / CheckOutCompleted for
        // check-ins). Worker subscribers learn about the change via the
        // outbox row; UI subscribers re-fetch on the next interval. When a
        // CheckInExtended variant is added, this becomes a publish here.
        let _ = (&self.events,); // mark field used pre-event-variant.

        tx.commit().await?;

        Ok(CheckInOutcome {
            check_in_id: cmd.check_in_id,
            aggregate_id,
        })
    }

    /// Check-out flow — applies repository checkout, frees the room,
    /// completes the booking (if linked), enqueues writeback + event.
    pub async fn check_out(&self, cmd: CheckOutCommand) -> ServiceResult<CheckInOutcome> {
        let status = self
            .repo
            .find_status(&self.pg, cmd.check_in_id)
            .await?
            .ok_or_else(|| {
                ServiceError::not_found(format!(
                    "check-in {} does not exist",
                    cmd.check_in_id
                ))
            })?;

        if !matches!(status.cin_status.as_deref(), Some("active")) {
            return Err(ServiceError::conflict(format!(
                "check-in {} is not active ({:?})",
                cmd.check_in_id, status.cin_status
            )));
        }

        let mut tx = self.pg.begin().await?;

        self.repo
            .apply_checkout(
                &mut tx,
                cmd.check_in_id,
                CheckOutWrite {
                    check_out_time: cmd.check_out_time,
                    total_amount: cmd.total_amount,
                    payment_status: &cmd.payment_status,
                    notes: cmd.notes.as_deref(),
                },
            )
            .await?;

        self.repo
            .mark_room_available_dirty(&mut tx, status.cin_room_id)
            .await?;

        if let Some(booking_id) = status.cin_book_id {
            self.repo.set_booking_completed(&mut tx, booking_id).await?;
        }

        let aggregate_id = aggregate_uuid(AggregateKind::CheckIn, cmd.check_in_id);
        // Audit H1: thread real revenue + nights + balance into the intent
        // so the recipe stops writing zeros to MSSQL on every checkout.
        let intent = WritebackIntent::CheckOut {
            check_in_id: aggregate_id,
            nights: Some(cmd.nights),
            room_price_total: Some(cmd.room_price_total),
            product_total: Some(cmd.product_total),
            net_total: Some(cmd.net_total),
            pay_total: Some(cmd.pay_total),
            balance: Some(cmd.balance),
        };
        let key = generate_idempotency_key(&intent, aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        let event = DomainEvent::CheckOutCompleted {
            id: aggregate_id,
            source: cmd.source.clone(),
        };
        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        tx.commit().await?;

        Ok(CheckInOutcome {
            check_in_id: cmd.check_in_id,
            aggregate_id,
        })
    }

    // ---------- private helpers ----------

    async fn enqueue_create_check_in(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        cin_id: i32,
        linked_booking_id: Option<i32>,
        customer_id: i32,
        ctx: &CheckInWritebackContext,
    ) -> ServiceResult<()> {
        let aggregate_id = aggregate_uuid(AggregateKind::CheckIn, cin_id);
        let payload = CreateCheckInPayload {
            customer_id: aggregate_uuid(AggregateKind::Customer, customer_id),
            legacy_cust_no: ctx.legacy_cust_no.clone(),
            linked_booking_id: linked_booking_id
                .map(|id| aggregate_uuid(AggregateKind::Booking, id)),
            linked_legacy_book_id: ctx.linked_legacy_book_id.clone(),
            room_no: ctx.room_no.clone(),
            room_type: ctx.room_type.clone(),
            stay: ctx.stay.clone(),
            price_per_night: ctx.price_per_night,
            nights: ctx.nights,
            price_total: ctx.price_total,
            created_by: ctx.created_by.clone(),
            guest_name_for_registry: ctx.guest_name_for_registry.clone(),
            guest_country: ctx.guest_country.clone(),
            // photo plumbing arrives in a follow-up — see audit HIGH-3.
            photo_tmp_no: None,
        };
        let intent = WritebackIntent::CreateCheckIn {
            check_in_id: aggregate_id,
            payload,
        };
        let key = generate_idempotency_key(&intent, aggregate_id);
        OutboxRepository::enqueue(tx, &intent, key)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;
        Ok(())
    }
}

fn validate_party_size(adults: i32, children: i32) -> ServiceResult<()> {
    if adults < 0 || children < 0 {
        return Err(ServiceError::validation(format!(
            "adults ({adults}) / children ({children}) must be non-negative"
        )));
    }
    if adults == 0 && children == 0 {
        return Err(ServiceError::validation(
            "check-in must have at least one guest (adults + children > 0)",
        ));
    }
    Ok(())
}

fn build_check_in_snapshot(
    aggregate_id: Uuid,
    linked_booking_id: Option<i32>,
    customer_id: i32,
    status: CheckInState,
    ctx: &CheckInWritebackContext,
) -> CheckInSnapshot {
    CheckInSnapshot {
        id: aggregate_id,
        legacy_cin_no: None,
        booking_id: linked_booking_id.map(|id| aggregate_uuid(AggregateKind::Booking, id)),
        customer_id: aggregate_uuid(AggregateKind::Customer, customer_id),
        status,
        room_no: ctx.room_no.clone(),
        stay_start: ctx.stay.start,
        stay_end: ctx.stay.end,
        total_price_net: ctx.price_total,
    }
}
