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
use super::shifts::ShiftService;

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
    /// Optional customer phone — threaded through to
    /// [`CreateCheckInPayload::customer_phone`] so the booking-linked
    /// check-in's `UPDATE HT_Customers SET [Cust_Add_tel]=…` preserves
    /// the booking-time phone instead of wiping it (Wave 5a item 1).
    pub customer_phone: Option<String>,
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
    /// Optional shift gate — Track G9 / T4 HIGH-8. When wired,
    /// `check_out` (round-bill) refuses to fold the folio unless
    /// `shifts.current_open_shift()` returns `Some`. Same builder
    /// pattern as `PaymentService::with_shifts` from F2 so the two
    /// gates stay symmetric. Stays `None` in unit tests not exercising
    /// the shift flow; production construction in
    /// `AppState::wire_services` always wires it on.
    pub(crate) shifts: Option<Arc<ShiftService>>,
}

impl CheckInService {
    pub fn new(
        repo: Arc<dyn CheckInRepository>,
        outbox: Arc<OutboxRepository>,
        events: Arc<EventBus>,
        pg: PgPool,
    ) -> Self {
        Self { repo, outbox, events, pg, shifts: None }
    }

    /// Builder-style attach for the round-bill shift gate.
    ///
    /// Wired from `AppState::wire_services` so production deployments
    /// refuse to fold a round-bill unless an `ht_shifts` row is open
    /// for the site. Mirrors F2's `PaymentService::with_shifts`
    /// pattern — same `ShiftService` instance is shared by both
    /// services so the "one open shift per site" invariant is
    /// observed identically across both gates.
    pub fn with_shifts(mut self, shifts: Arc<ShiftService>) -> Self {
        self.shifts = Some(shifts);
        self
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
    ///
    /// Track G9 / T4 HIGH-8 (`docs/coexistence/audit-2026-05-13.md`):
    /// when a shift gate is wired (production), the call refuses to
    /// fold the folio unless `shifts.current_open_shift()` returns
    /// `Some`. The gate uses the EXACT same helper F2 introduced for
    /// `PaymentService::record_payment` — see `service/payment.rs:203`.
    /// The resolved `shift_id` is stamped onto the
    /// `ht_checkins.cin_round_bill_shift_id` column (migration 048)
    /// so per-shift revenue attribution is possible from canonical PG.
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

        // Track G9 / T4 HIGH-8: refuse the round-bill when a shift gate
        // is wired and no shift is open. Mirrors the F2 gate at
        // `payment.rs:203`. The Thai-localized message is the one the
        // receptionist sees on the cashier UI; the English prefix
        // matches F2's wording so log scraping treats both gates
        // uniformly. The shift id is captured here under the same read
        // path the payment gate uses; if multiple open shifts somehow
        // exist (the partial UNIQUE index `ht_shifts_one_open_per_site`
        // prevents this — defense in depth), `current_open_shift`
        // returns the single row that survived the index.
        let round_bill_shift_id = if let Some(shifts) = &self.shifts {
            match shifts.current_open_shift().await? {
                Some(shift) => Some(shift.shift_id),
                None => {
                    return Err(ServiceError::validation(
                        "no open shift — please open a cashier shift before folding the round-bill \
                         (ไม่พบรอบเงินที่เปิดอยู่ — กรุณาเปิดรอบเงินก่อนปิดบิล)",
                    ));
                }
            }
        } else {
            None
        };

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
                    round_bill_shift_id,
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
            // Wave 5a item 1 — preserve booking-time phone instead of
            // wiping `HT_Customers.Cust_Add_tel` on every check-in.
            customer_phone: ctx.customer_phone.clone(),
            // photo plumbing arrives in a follow-up — see audit HIGH-3.
            photo_tmp_no: None,
            // Track B4 — multi-room slice. Empty for now; populated by
            // the multi-room walk-in route (T4 HIGH-1) when it lands.
            // Today the create-check-in flow is single-room so the
            // recipe falls back to the legacy `room_no` / `room_type`
            // top-level fields (the synthetic single-line slice in
            // `effective_room_lines`).
            room_lines: Vec::new(),
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

// -----------------------------------------------------------------------------
// tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    //! Track G9 / T4 HIGH-8 — exercise the round-bill shift gate on
    //! `check_out`. Mirrors the F2 payment-gate test layout in
    //! `service::payment::tests`: each test uses a unique `site_id`
    //! marker so the partial UNIQUE index `ht_shifts_one_open_per_site`
    //! doesn't cause cross-test interference, and every test skips
    //! cleanly when PG is not reachable so `cargo test` without a DB
    //! still passes the rest of the suite.
    use super::*;
    use crate::outbox::event::EventSource;
    use crate::outbox::{EventBus, OutboxRepository};
    use crate::repository::checkin::PgCheckInRepository;
    use crate::service::shifts::{CloseShiftCommand, OpenShiftCommand, ShiftService};

    async fn try_pool() -> Option<PgPool> {
        let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://postgres:REDACTED-pg-2026@localhost:5439/hotelnew".to_string()
        });
        PgPool::connect(&url).await.ok()
    }

    /// Build a `CheckInService` wired with a real `ShiftService` so the
    /// round-bill gate is exercised end-to-end. Mirrors the
    /// `build_service_with_gate` helper in `service::payment::tests`.
    fn build_service_with_gate(pool: PgPool, site_id: &str) -> CheckInService {
        let repo: Arc<dyn CheckInRepository> = Arc::new(PgCheckInRepository::new());
        let outbox = Arc::new(OutboxRepository::new());
        let events = Arc::new(EventBus::new());
        let shifts = Arc::new(ShiftService::new(pool.clone(), site_id));
        CheckInService::new(repo, outbox, events, pool).with_shifts(shifts)
    }

    /// Reset the `ht_shifts` table for a marker site so reruns of the
    /// same test don't accumulate closed-but-undeleted rows that would
    /// hide a regression.
    async fn clear_shifts(pool: &PgPool, site: &str) {
        let _ = sqlx::query("DELETE FROM ht_shifts WHERE shift_site_id = $1")
            .bind(site)
            .execute(pool)
            .await;
    }

    /// Insert a temporary `ht_checkins` row in `active` status so
    /// `check_out` can target it. Returns `None` if the underlying
    /// `ht_customers` / `ht_rooms_new` tables are empty (fresh dev DB
    /// without seed data — the test is then skipped via the `?` early
    /// return at the call site).
    async fn seed_active_checkin(pool: &PgPool, marker: &str) -> Option<i32> {
        let cust_id: Option<i32> =
            sqlx::query_scalar("SELECT cust_id FROM ht_customers ORDER BY cust_id LIMIT 1")
                .fetch_optional(pool)
                .await
                .ok()
                .flatten();
        let room_id: Option<i32> =
            sqlx::query_scalar("SELECT room_id FROM ht_rooms_new ORDER BY room_id LIMIT 1")
                .fetch_optional(pool)
                .await
                .ok()
                .flatten();
        let (cust_id, room_id) = (cust_id?, room_id?);

        // `cin_no` is UNIQUE NOT NULL — use the marker so two concurrent
        // tests can't collide. Truncated to 20 chars (the column width).
        let cin_no: String = format!("G9-{marker}").chars().take(20).collect();
        let row: Option<(i32,)> = sqlx::query_as(
            "INSERT INTO ht_checkins (cin_no, cin_cust_id, cin_room_id, cin_checkin_time, \
             cin_expected_checkout, cin_total_amount, cin_status, cin_rate_per_night, \
             cin_adults, cin_children) \
             VALUES ($1, $2, $3, NOW(), (NOW() + INTERVAL '1 day')::date, 100, 'active', 100, 1, 0) \
             RETURNING cin_id",
        )
        .bind(&cin_no)
        .bind(cust_id)
        .bind(room_id)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten();
        row.map(|(id,)| id)
    }

    async fn cleanup_checkin(pool: &PgPool, cin_id: i32) {
        let _ = sqlx::query("DELETE FROM ht_checkins WHERE cin_id = $1")
            .bind(cin_id)
            .execute(pool)
            .await;
    }

    fn sample_checkout(cin_id: i32) -> CheckOutCommand {
        CheckOutCommand {
            check_in_id: cin_id,
            check_out_time: None,
            total_amount: Some(100.0),
            payment_status: "paid".to_string(),
            notes: None,
            source: EventSource::System {
                reason: "G9_gate_test".into(),
            },
            nights: 1.0,
            room_price_total: 100.0,
            product_total: 0.0,
            net_total: 100.0,
            pay_total: 100.0,
            balance: 0.0,
        }
    }

    /// Track G9 / T4 HIGH-8 — without an open shift, `check_out`
    /// must reject with a `Validation` error whose message carries the
    /// "no open shift" English token (matches F2's payment-gate
    /// wording so the cashier UI can render either gate uniformly)
    /// AND the Thai instruction so the receptionist sees actionable
    /// guidance.
    #[tokio::test]
    async fn round_bill_rejected_when_no_open_shift() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping round_bill_rejected_when_no_open_shift — PG not reachable");
            return;
        };
        let site = "TEST_G9_no_shift";
        clear_shifts(&pool, site).await;

        let Some(cin_id) = seed_active_checkin(&pool, site).await else {
            eprintln!("skipping round_bill_rejected_when_no_open_shift — seed failed");
            return;
        };
        let svc = build_service_with_gate(pool.clone(), site);

        let err = svc
            .check_out(sample_checkout(cin_id))
            .await
            .expect_err("must reject when no shift is open");

        cleanup_checkin(&pool, cin_id).await;
        clear_shifts(&pool, site).await;

        match err {
            ServiceError::Validation(msg) => {
                assert!(
                    msg.contains("no open shift"),
                    "expected English gate message, got: {msg}"
                );
                assert!(
                    msg.contains("เปิดรอบเงิน"),
                    "expected Thai-localized instruction, got: {msg}"
                );
            }
            other => panic!("expected Validation, got {other:?}"),
        }
    }

    /// Track G9 / T4 HIGH-8 — once a shift is opened the round-bill
    /// must succeed AND stamp `cin_round_bill_shift_id` on the
    /// canonical row (per-shift attribution per migration 048).
    #[tokio::test]
    async fn round_bill_succeeds_when_shift_open_and_stamps_shift_id() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping round_bill_succeeds_when_shift_open — PG not reachable");
            return;
        };
        let site = "TEST_G9_open_shift";
        clear_shifts(&pool, site).await;

        let Some(cin_id) = seed_active_checkin(&pool, site).await else {
            eprintln!("skipping round_bill_succeeds_when_shift_open — seed failed");
            return;
        };

        let shifts = Arc::new(ShiftService::new(pool.clone(), site));
        let open_outcome = match shifts
            .open_shift(OpenShiftCommand {
                opened_by: "alice".into(),
                opening_float: 500.0,
                notes: None,
            })
            .await
        {
            Ok(o) => o,
            Err(err) => {
                cleanup_checkin(&pool, cin_id).await;
                panic!("open_shift failed: {err:?}");
            }
        };

        let repo: Arc<dyn CheckInRepository> = Arc::new(PgCheckInRepository::new());
        let outbox = Arc::new(OutboxRepository::new());
        let events = Arc::new(EventBus::new());
        let svc = CheckInService::new(repo, outbox, events, pool.clone())
            .with_shifts(shifts.clone());

        let result = svc.check_out(sample_checkout(cin_id)).await;

        // Read back the stamped shift_id before cleaning up so the
        // assertion has access to the canonical state we wrote.
        let stamped: Option<i64> = sqlx::query_scalar(
            "SELECT cin_round_bill_shift_id FROM ht_checkins WHERE cin_id = $1",
        )
        .bind(cin_id)
        .fetch_one(&pool)
        .await
        .ok()
        .flatten();

        cleanup_checkin(&pool, cin_id).await;
        clear_shifts(&pool, site).await;

        let outcome = result.expect("round-bill must succeed with open shift");
        assert_eq!(outcome.check_in_id, cin_id);
        assert_eq!(
            stamped,
            Some(open_outcome.shift_id),
            "cin_round_bill_shift_id must be stamped with the open shift's id"
        );
    }

    /// Track G9 / T4 HIGH-8 — closing the shift before the round-bill
    /// returns the same `Validation` error as the no-shift-ever case
    /// (`current_open_shift()` returns `None` for both, the gate can't
    /// distinguish). Test pins the contract so a refactor that
    /// accidentally exempts "previously open" shifts is caught.
    #[tokio::test]
    async fn round_bill_rejected_when_shift_already_closed() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping round_bill_rejected_when_shift_already_closed — PG not reachable");
            return;
        };
        let site = "TEST_G9_closed_shift";
        clear_shifts(&pool, site).await;

        let Some(cin_id) = seed_active_checkin(&pool, site).await else {
            eprintln!("skipping round_bill_rejected_when_shift_already_closed — seed failed");
            return;
        };

        let shifts = Arc::new(ShiftService::new(pool.clone(), site));
        if let Err(err) = shifts
            .open_shift(OpenShiftCommand {
                opened_by: "alice".into(),
                opening_float: 0.0,
                notes: None,
            })
            .await
        {
            cleanup_checkin(&pool, cin_id).await;
            panic!("open_shift failed: {err:?}");
        }
        if let Err(err) = shifts
            .close_shift(CloseShiftCommand {
                closed_by: "alice".into(),
                notes: None,
            })
            .await
        {
            cleanup_checkin(&pool, cin_id).await;
            clear_shifts(&pool, site).await;
            panic!("close_shift failed: {err:?}");
        }

        let repo: Arc<dyn CheckInRepository> = Arc::new(PgCheckInRepository::new());
        let outbox = Arc::new(OutboxRepository::new());
        let events = Arc::new(EventBus::new());
        let svc = CheckInService::new(repo, outbox, events, pool.clone())
            .with_shifts(shifts.clone());

        let err = svc
            .check_out(sample_checkout(cin_id))
            .await
            .expect_err("closed shift must be treated as no-open-shift");

        cleanup_checkin(&pool, cin_id).await;
        clear_shifts(&pool, site).await;

        match err {
            ServiceError::Validation(msg) => {
                assert!(
                    msg.contains("no open shift"),
                    "expected gate message after close, got: {msg}"
                );
            }
            other => panic!("expected Validation, got {other:?}"),
        }
    }

    /// Track G9 / T4 HIGH-8 — concurrency: two parallel round-bills
    /// against the same checkin must result in exactly one success.
    /// The second sees `cin_status='checkedout'` already (set by the
    /// first's `apply_checkout`) and fails the precheck. Mirrors F2's
    /// UPDLOCK pattern adapted for PG (`find_status` + status guard
    /// before opening the TX; the actual UPDATE then targets a row
    /// whose `cin_status` has already been demoted).
    #[tokio::test]
    async fn concurrent_round_bills_against_same_checkin_one_wins() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping concurrent_round_bills — PG not reachable");
            return;
        };
        let site = "TEST_G9_concurrent";
        clear_shifts(&pool, site).await;

        let Some(cin_id) = seed_active_checkin(&pool, site).await else {
            eprintln!("skipping concurrent_round_bills — seed failed");
            return;
        };

        let shifts = Arc::new(ShiftService::new(pool.clone(), site));
        if let Err(err) = shifts
            .open_shift(OpenShiftCommand {
                opened_by: "alice".into(),
                opening_float: 0.0,
                notes: None,
            })
            .await
        {
            cleanup_checkin(&pool, cin_id).await;
            panic!("open_shift failed: {err:?}");
        }

        let repo: Arc<dyn CheckInRepository> = Arc::new(PgCheckInRepository::new());
        let outbox = Arc::new(OutboxRepository::new());
        let events = Arc::new(EventBus::new());
        let svc = Arc::new(
            CheckInService::new(repo, outbox, events, pool.clone())
                .with_shifts(shifts.clone()),
        );

        let svc_a = svc.clone();
        let svc_b = svc.clone();
        let cmd_a = sample_checkout(cin_id);
        let cmd_b = sample_checkout(cin_id);

        let (ra, rb) = tokio::join!(
            tokio::spawn(async move { svc_a.check_out(cmd_a).await }),
            tokio::spawn(async move { svc_b.check_out(cmd_b).await }),
        );

        cleanup_checkin(&pool, cin_id).await;
        clear_shifts(&pool, site).await;

        let ra = ra.expect("task A must not panic");
        let rb = rb.expect("task B must not panic");

        let successes = [ra.is_ok(), rb.is_ok()].iter().filter(|x| **x).count();
        assert_eq!(
            successes, 1,
            "exactly one of two concurrent round-bills must succeed; got {successes} (ra={ra:?}, rb={rb:?})"
        );
        // The losing call must be a `Conflict` (status != 'active'),
        // not a `Validation` — that distinguishes the concurrency
        // collision from a gate rejection.
        let loser = match (ra, rb) {
            (Err(err), Ok(_)) | (Ok(_), Err(err)) => err,
            (Err(_), Err(_)) => panic!("both round-bills failed; exactly one should win"),
            (Ok(_), Ok(_)) => panic!("both round-bills succeeded; exactly one should win"),
        };
        assert!(
            matches!(loser, ServiceError::Conflict(_)),
            "loser must be a Conflict (folio already folded), got: {loser:?}"
        );
    }
}
