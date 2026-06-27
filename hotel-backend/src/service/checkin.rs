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
use sqlx::{PgPool, Row};
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
    /// Room deposit (`เงินมัดจำ`) collected at check-in. Persisted canonically
    /// to `ht_checkin_rooms.cr_dep_amount` (+ `cr_dep_status`) and threaded
    /// into [`CreateCheckInPayload::deposit`] so the legacy
    /// `HT_CheckIn_Ds.Cin_Room_Dep` mirror stays consistent through the sync
    /// round-trip. `Money::ZERO` ⇒ no deposit (the common walk-in).
    pub deposit: Money,
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

/// Command for [`CheckInService::change_room`].
///
/// Track G4 / T4 HIGH-3 (`docs/coexistence/audit-2026-05-13.md`). A
/// receptionist moves an active guest from `from_room_id` to
/// `to_room_id` mid-stay. The service:
///
/// 1. Validates the check-in is active.
/// 2. Validates `from_room_id` is currently in this folio's junction.
/// 3. Validates `to_room_id` is free (no overlapping active occupancy).
/// 4. Inside one PG transaction: inserts the audit row in
///    `ht_room_changes`, UPDATEs the matching `ht_checkin_rooms` row's
///    `cr_room_id` from `from` → `to`, and emits
///    [`WritebackIntent::RoomChange`] for the legacy mirror.
///
/// `actor` is the operator name captured by the route from the auth
/// session (today: empty string until G7 auth middleware lands).
#[derive(Debug, Clone)]
pub struct ChangeRoomCommand {
    pub check_in_id: i32,
    pub from_room_id: i32,
    pub to_room_id: i32,
    pub reason: Option<String>,
    pub actor: String,
    pub source: EventSource,
}

/// Outcome of a successful `change_room` operation. Carries the newly
/// inserted `ht_room_changes.rc_id` so the route can return the audit
/// row without an additional lookup.
#[derive(Debug, Clone)]
pub struct ChangeRoomOutcome {
    pub check_in_id: i32,
    pub aggregate_id: Uuid,
    pub rc_id: i64,
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

        self.persist_deposit_junction(&mut tx, cin_id, cmd.room_id, &cmd.writeback_context)
            .await?;

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

        // Single-room-only guard (interim): reject a multi-room booking rather
        // than silently check in one room and leave the rest vacant. See
        // `reject_multi_room_checkin`.
        let booking_room_count = self
            .repo
            .count_booking_rooms(&self.pg, cmd.booking_id)
            .await?;
        reject_multi_room_checkin(booking_room_count)?;

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

        self.persist_deposit_junction(&mut tx, cin_id, cmd.room_id, &cmd.writeback_context)
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
            .map_err(ServiceError::from_enqueue_error)?;

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

    /// Mid-stay room change — Track G4 / T4 HIGH-3.
    ///
    /// Validates the move, then in one PG transaction:
    /// - INSERTs the audit row into `ht_room_changes`.
    /// - UPDATEs the matching `ht_checkin_rooms.cr_room_id` from `from`
    ///   to `to` so subsequent reads see the new occupancy.
    /// - Marks the old room available-dirty and the new room occupied
    ///   on `ht_rooms_new` so the dashboard's room-grid status flips
    ///   immediately (matches the create-flow's `mark_room_occupied`
    ///   convention).
    /// - Enqueues [`WritebackIntent::RoomChange`] for the legacy mirror.
    ///
    /// Returns [`ChangeRoomOutcome::rc_id`] so the route can return the
    /// freshly-inserted audit row.
    pub async fn change_room(
        &self,
        cmd: ChangeRoomCommand,
    ) -> ServiceResult<ChangeRoomOutcome> {
        // 1. Source check-in must exist + be active. We reject any
        //    non-active state explicitly so receptionists get a clear
        //    409 wording instead of a silent no-op.
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
        if cmd.from_room_id == cmd.to_room_id {
            return Err(ServiceError::validation(
                "from_room_id and to_room_id must differ — nothing to change",
            ));
        }

        let mut tx = self.pg.begin().await?;

        // 2. The `from_room_id` MUST currently sit in this folio's
        //    junction (otherwise we'd be moving a room the guest
        //    never actually occupied). COALESCE handles pre-B5 folios
        //    where the junction may not yet be populated — for those
        //    we fall back to `ht_checkins.cin_room_id`.
        let from_in_folio: bool = sqlx::query(
            "SELECT EXISTS(\
               SELECT 1 FROM ht_checkin_rooms \
                WHERE cr_cin_id = $1 AND cr_room_id = $2\
             ) OR EXISTS(\
               SELECT 1 FROM ht_checkins \
                WHERE cin_id = $1 AND cin_room_id = $2\
             ) AS in_folio",
        )
        .bind(cmd.check_in_id)
        .bind(cmd.from_room_id)
        .fetch_one(&mut *tx)
        .await?
        .try_get("in_folio")
        .unwrap_or(false);
        if !from_in_folio {
            return Err(ServiceError::validation(format!(
                "room {} is not currently assigned to check-in {}",
                cmd.from_room_id, cmd.check_in_id
            )));
        }

        // 3. The `to_room_id` MUST be free. We accept the target room as
        //    "free" when no OTHER active check-in occupies it. The
        //    junction is the source of truth for B2+ folios; legacy
        //    `cin_room_id` is the COALESCE fallback for pre-B2 ones.
        let target_occupied: bool = sqlx::query(
            "SELECT EXISTS(\
               SELECT 1 FROM ht_checkin_rooms cr \
                 JOIN ht_checkins c ON c.cin_id = cr.cr_cin_id \
                WHERE cr.cr_room_id = $2 \
                  AND c.cin_status = 'active' \
                  AND cr.cr_cin_id <> $1\
             ) OR EXISTS(\
               SELECT 1 FROM ht_checkins c \
                WHERE c.cin_room_id = $2 \
                  AND c.cin_status = 'active' \
                  AND c.cin_id <> $1\
             ) AS occupied",
        )
        .bind(cmd.check_in_id)
        .bind(cmd.to_room_id)
        .fetch_one(&mut *tx)
        .await?
        .try_get("occupied")
        .unwrap_or(false);
        if target_occupied {
            return Err(ServiceError::conflict(format!(
                "room {} is already occupied by another active check-in",
                cmd.to_room_id
            )));
        }

        // 4. Pull the `cr_rate_per_night` of the from-row so the audit
        //    row records the pre-move price. Pre-B5 folios that lack
        //    a junction row fall back to `ht_checkins.cin_rate_per_night`.
        let room_before_price: f64 = sqlx::query(
            "SELECT COALESCE(\
               (SELECT cr_rate_per_night::float8 FROM ht_checkin_rooms \
                  WHERE cr_cin_id = $1 AND cr_room_id = $2), \
               (SELECT cin_rate_per_night FROM ht_checkins WHERE cin_id = $1), \
               0.0 \
             ) AS price",
        )
        .bind(cmd.check_in_id)
        .bind(cmd.from_room_id)
        .fetch_one(&mut *tx)
        .await?
        .try_get("price")
        .unwrap_or(0.0);

        // 5. INSERT the canonical audit row.
        let reason_trimmed = cmd
            .reason
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let rc_id: i64 = sqlx::query(
            "INSERT INTO ht_room_changes ( \
                rc_cin_id, rc_from_room_id, rc_to_room_id, rc_reason, \
                rc_changed_at, rc_changed_by, rc_room_before_price \
             ) VALUES ($1, $2, $3, $4, NOW(), $5, $6::numeric) \
             RETURNING rc_id",
        )
        .bind(cmd.check_in_id)
        .bind(cmd.from_room_id)
        .bind(cmd.to_room_id)
        .bind(reason_trimmed)
        .bind(if cmd.actor.is_empty() { None } else { Some(cmd.actor.as_str()) })
        .bind(room_before_price)
        .fetch_one(&mut *tx)
        .await?
        .try_get("rc_id")?;

        // 6. UPDATE the junction row from from-room → to-room. Pre-B5
        //    folios that don't yet have a junction row get one created
        //    so the move still lands canonically (matches the B2 mapper
        //    UPSERT shape).
        let junction_updated = sqlx::query(
            "UPDATE ht_checkin_rooms \
                SET cr_room_id = $3, cr_updated_at = NOW() \
              WHERE cr_cin_id = $1 AND cr_room_id = $2",
        )
        .bind(cmd.check_in_id)
        .bind(cmd.from_room_id)
        .bind(cmd.to_room_id)
        .execute(&mut *tx)
        .await?
        .rows_affected();
        if junction_updated == 0 {
            // Pre-B5 fallback: insert a junction row keyed on the
            // to-room so the dashboard's B3 reader sees the new
            // occupancy. cr_room_status mirrors the active-stay
            // literal the B2 mapper writes.
            sqlx::query(
                "INSERT INTO ht_checkin_rooms ( \
                    cr_cin_id, cr_room_id, cr_room_status, cr_rate_per_night \
                 ) VALUES ($1, $2, 'active', $3::numeric) \
                 ON CONFLICT (cr_cin_id, cr_room_id) DO NOTHING",
            )
            .bind(cmd.check_in_id)
            .bind(cmd.to_room_id)
            .bind(room_before_price)
            .execute(&mut *tx)
            .await?;
        }

        // 7. Flip room-grid status so the dashboard tile colours update
        //    on the next refresh. The RoomChange writeback recipe emits
        //    the matching legacy-side statements (`HT_CheckIn_Ds.Cin_Room_No`
        //    move + `HT_Rooms.Room_Use` flips for both rooms, cheatsheet
        //    §3.17) in its own MSSQL transaction — this PG write keeps the
        //    local view consistent in the meantime. (An earlier comment
        //    claimed the "sync mappers" would converge MSSQL; they run
        //    legacy→PG, the wrong direction — corrected 2026-06-11.)
        self.repo
            .mark_room_available_dirty(&mut tx, cmd.from_room_id)
            .await?;
        self.repo.mark_room_occupied(&mut tx, cmd.to_room_id).await?;

        // ht_checkins still carries the deprecated single-room column.
        // Update it as a fallback for legacy-reader code paths so the
        // existing dashboard rendering doesn't surface stale data
        // between now and Track B5's column drop.
        sqlx::query("UPDATE ht_checkins SET cin_room_id = $2, updated_at = NOW() WHERE cin_id = $1")
            .bind(cmd.check_in_id)
            .bind(cmd.to_room_id)
            .execute(&mut *tx)
            .await?;

        // 8. Enqueue the writeback intent + commit. The aggregate id is
        //    the check-in's so the writeback worker's resolver picks
        //    up the same `legacy_cin_no` self-heal path.
        let aggregate_id = aggregate_uuid(AggregateKind::CheckIn, cmd.check_in_id);
        let intent = WritebackIntent::RoomChange {
            check_in_id: aggregate_id,
            rc_id,
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

        // No dedicated DomainEvent variant for room-change today (mirrors
        // the extend-stay pattern — subscribers learn via writeback row
        // + UI poll). When a `CheckInRoomChanged` event lands the publish
        // goes here.
        let _ = &self.events;

        tx.commit().await?;

        Ok(ChangeRoomOutcome {
            check_in_id: cmd.check_in_id,
            aggregate_id,
            rc_id,
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
            .map_err(ServiceError::from_enqueue_error)?;

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

    /// Persist a collected room deposit onto the canonical
    /// `ht_checkin_rooms` junction so the folio/cashier reads
    /// (`folio_breakdown`, `new_shifts`) and the room-grid see it
    /// immediately — before the legacy→PG sync round-trip would refresh it.
    ///
    /// The create flow is single-room, so a deposit applies to the one
    /// room being checked in. When no deposit is taken (`Money::ZERO`) we
    /// write nothing: non-deposit walk-ins keep their pre-junction shape
    /// (dashboard readers COALESCE-fall back to `ht_checkins.cin_room_id`),
    /// so this only adds a junction row in the deposit case.
    ///
    /// `cr_room_status` (`'เข้าพัก'`) and `cr_dep_status`
    /// (`'ยังไม่คืนค่ามัดจำ'`) are the exact Thai literals the B2 sync mapper
    /// writes from legacy, so the later `ON CONFLICT … DO UPDATE` round-trip
    /// is a no-op rather than flapping the row. `ON CONFLICT DO UPDATE`
    /// guards against a pre-existing junction row (defensive; none exists at
    /// create time today).
    async fn persist_deposit_junction(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        cin_id: i32,
        room_id: i32,
        ctx: &CheckInWritebackContext,
    ) -> ServiceResult<()> {
        if ctx.deposit.as_satang() <= 0 {
            return Ok(());
        }
        let deposit_baht = ctx.deposit.as_satang() as f64 / 100.0;
        let rate_baht = ctx.price_per_night.as_satang() as f64 / 100.0;
        let total_baht = ctx.price_total.as_satang() as f64 / 100.0;
        sqlx::query(
            "INSERT INTO ht_checkin_rooms ( \
                 cr_cin_id, cr_room_id, cr_room_in, cr_room_out, cr_room_status, \
                 cr_rate_per_night, cr_nights, cr_room_total, cr_dep_amount, cr_dep_status \
             ) VALUES ($1, $2, $3, $4, 'เข้าพัก', $5::numeric, $6, $7::numeric, \
                       $8::numeric, 'ยังไม่คืนค่ามัดจำ') \
             ON CONFLICT (cr_cin_id, cr_room_id) DO UPDATE SET \
                 cr_dep_amount = EXCLUDED.cr_dep_amount, \
                 cr_dep_status = EXCLUDED.cr_dep_status, \
                 cr_updated_at = NOW()",
        )
        .bind(cin_id)
        .bind(room_id)
        .bind(ctx.stay.start)
        .bind(ctx.stay.end)
        .bind(rate_baht)
        .bind(ctx.nights)
        .bind(total_baht)
        .bind(deposit_baht)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

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
            // Deposit capture — the legacy `HT_CheckIn_Ds.Cin_Room_Dep` mirror
            // (+ `Cin_dep_status`) is written by the walkin/checkin_to_booking
            // recipes; `Money::ZERO` keeps the no-deposit byte-shape.
            deposit: ctx.deposit,
            // photo plumbing arrives in a follow-up — see audit HIGH-3.
            // TODO(out of scope): guest photo capture/persistence
            // (`photo_tmp_no`, `Tb_Save_Image`) — see docs/architecture.md:1385.
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
            .map_err(ServiceError::from_enqueue_error)?;
        Ok(())
    }
}

/// Guard: the new-app check-in flow is SINGLE-ROOM (one `room_id` per POST —
/// routes/new_checkins.rs + the single-room UI). A booking spanning multiple
/// rooms (`ht_booking_rooms`) cannot be checked in correctly here yet: checking
/// in one room would leave the rest showing vacant (overbooking footgun), and
/// the full multi-room writeback (one `HT_CheckIn_H` folio + N `HT_CheckIn_Ds`
/// — the `room_lines` fan-out in checkin_to_booking, built+tested but unfed) is
/// not wired into the service/route/UI. Reject loudly until that lands;
/// multi-room bookings are checked in via iHOTEL (it owns the one-folio model).
/// `0`/`1` pass: 1 = the normal single-room flow; 0 = a booking with no
/// `ht_booking_rooms` rows yet (data gap) — neither is the unsupported case.
fn reject_multi_room_checkin(booking_room_count: i64) -> ServiceResult<()> {
    if booking_room_count > 1 {
        return Err(ServiceError::validation(format!(
            "this booking spans {booking_room_count} rooms; multi-room check-in \
             is not yet supported in the new app — check it in via iHOTEL"
        )));
    }
    Ok(())
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

    // Pure guard tests (no DB) — the single-room-only interim guard.
    #[test]
    fn reject_multi_room_checkin_blocks_multi_room_bookings() {
        assert!(reject_multi_room_checkin(2).is_err());
        assert!(reject_multi_room_checkin(45).is_err()); // group bookings exist in prod
    }

    #[test]
    fn reject_multi_room_checkin_allows_single_or_zero_room_bookings() {
        // 1 = the normal single-room flow; 0 = a booking with no ht_booking_rooms
        // rows yet (data gap). Only >1 is the unsupported multi-room case.
        assert!(reject_multi_room_checkin(1).is_ok());
        assert!(reject_multi_room_checkin(0).is_ok());
    }

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
        // Track J6 — the open/close tests run with round_writeback ON, which
        // enqueues open_round/close_round jobs keyed on (intent, site, legacy
        // id). Clear them so a rerun (next_no resets to 1) doesn't collide on
        // the idempotency unique key.
        let _ = sqlx::query(
            "DELETE FROM writeback_jobs \
             WHERE intent IN ('open_round','close_round') \
               AND payload->'payload'->>'site_id' = $1",
        )
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

        let shifts = Arc::new(ShiftService::new(pool.clone(), site).with_round_writeback(true));
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

        let shifts = Arc::new(ShiftService::new(pool.clone(), site).with_round_writeback(true));
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
                cash_count: None,
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

        let shifts = Arc::new(ShiftService::new(pool.clone(), site).with_round_writeback(true));
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

#[cfg(test)]
mod change_room_tests {
    //! Track G4 / T4 HIGH-3 — tests for `CheckInService::change_room`.
    //! Integration-style: each test connects to the PG test database and
    //! seeds its own check-in / room rows. Skipped at runtime when PG
    //! is unreachable so the suite still passes in environments where
    //! the DB isn't available.

    use super::*;
    use crate::outbox::event::EventSource;
    use crate::outbox::{EventBus, OutboxRepository};
    use crate::repository::checkin::PgCheckInRepository;

    async fn try_pool() -> Option<PgPool> {
        let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://postgres:REDACTED-pg-2026@localhost:5439/hotelnew".to_string()
        });
        PgPool::connect(&url).await.ok()
    }

    fn build_service(pool: PgPool) -> CheckInService {
        let repo: Arc<dyn crate::repository::checkin::CheckInRepository> =
            Arc::new(PgCheckInRepository::new());
        let outbox = Arc::new(OutboxRepository::new());
        let events = Arc::new(EventBus::new());
        CheckInService::new(repo, outbox, events, pool)
    }

    /// Pure validation: from == to must be rejected before any DB work.
    /// This guard fires AFTER `find_status` so the test still needs PG
    /// (the find_status check runs first to surface NotFound on bad ids)
    /// — but we seed a minimal active check-in to exercise the equality
    /// guard in isolation.
    #[tokio::test]
    async fn change_room_rejects_same_room_id() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping change_room_rejects_same_room_id — PG not reachable");
            return;
        };
        let (cin_id, _from, _to) = match seed_active_checkin(&pool, "G4_same").await {
            Some(ids) => ids,
            None => return,
        };
        let svc = build_service(pool.clone());
        let err = svc
            .change_room(ChangeRoomCommand {
                check_in_id: cin_id,
                from_room_id: 999_001,
                to_room_id: 999_001,
                reason: None,
                actor: "tester".into(),
                source: EventSource::System {
                    reason: "G4_same".into(),
                },
            })
            .await
            .expect_err("same-room change must reject");
        match err {
            ServiceError::Validation(msg) => {
                assert!(msg.contains("must differ"), "got: {msg}");
            }
            other => panic!("expected Validation, got {other:?}"),
        }
        cleanup(&pool, cin_id).await;
    }

    /// Not-found check-in surfaces as NotFound, not as a silent no-op.
    #[tokio::test]
    async fn change_room_rejects_missing_checkin() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping change_room_rejects_missing_checkin — PG not reachable");
            return;
        };
        let svc = build_service(pool);
        let err = svc
            .change_room(ChangeRoomCommand {
                check_in_id: -1,
                from_room_id: 1,
                to_room_id: 2,
                reason: None,
                actor: String::new(),
                source: EventSource::System {
                    reason: "G4_missing".into(),
                },
            })
            .await
            .expect_err("missing check-in must reject");
        assert!(matches!(err, ServiceError::NotFound(_)));
    }

    /// Happy path: seed an active check-in occupying room A; move to B.
    /// Verifies (1) the audit row landed in `ht_room_changes`,
    /// (2) the junction `cr_room_id` flipped from A→B,
    /// (3) a `room_change` outbox row was enqueued with the right
    ///     aggregate id.
    #[tokio::test]
    async fn change_room_writes_audit_and_emits_intent() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping change_room_writes_audit_and_emits_intent — PG not reachable");
            return;
        };
        let (cin_id, from_room_id, to_room_id) =
            match seed_active_checkin(&pool, "G4_happy").await {
                Some(ids) => ids,
                None => return,
            };

        let svc = build_service(pool.clone());
        let outcome = svc
            .change_room(ChangeRoomCommand {
                check_in_id: cin_id,
                from_room_id,
                to_room_id,
                reason: Some("noise complaint".into()),
                actor: "alice".into(),
                source: EventSource::System {
                    reason: "G4_happy".into(),
                },
            })
            .await
            .expect("happy-path change_room must succeed");

        assert_eq!(outcome.check_in_id, cin_id);
        assert!(outcome.rc_id > 0);

        // 1. Audit row present with the right values.
        let audit: (i32, i32, i32, Option<String>, Option<String>) = sqlx::query_as(
            "SELECT rc_cin_id, rc_from_room_id, rc_to_room_id, rc_reason, rc_changed_by \
               FROM ht_room_changes WHERE rc_id = $1",
        )
        .bind(outcome.rc_id)
        .fetch_one(&pool)
        .await
        .expect("audit row must be present");
        assert_eq!(audit.0, cin_id);
        assert_eq!(audit.1, from_room_id);
        assert_eq!(audit.2, to_room_id);
        assert_eq!(audit.3.as_deref(), Some("noise complaint"));
        assert_eq!(audit.4.as_deref(), Some("alice"));

        // 2. Junction flipped.
        let junction: (i32,) = sqlx::query_as(
            "SELECT cr_room_id FROM ht_checkin_rooms WHERE cr_cin_id = $1",
        )
        .bind(cin_id)
        .fetch_one(&pool)
        .await
        .expect("junction row must exist");
        assert_eq!(
            junction.0, to_room_id,
            "junction must point to to_room_id after move"
        );

        // 3. Outbox row enqueued with intent='room_change' on the
        //    aggregate id derived from cin_id.
        let aggregate_id = aggregate_uuid(AggregateKind::CheckIn, cin_id);
        let outbox: (i64, String) = sqlx::query_as(
            "SELECT id, intent FROM writeback_jobs \
              WHERE aggregate_id = $1 \
                AND intent = 'room_change' \
              ORDER BY id DESC LIMIT 1",
        )
        .bind(aggregate_id)
        .fetch_one(&pool)
        .await
        .expect("room_change outbox row must be enqueued");
        assert_eq!(outbox.1, "room_change");

        cleanup(&pool, cin_id).await;
    }

    /// Source room not in folio → 400/Validation. Receptionist sees an
    /// explicit error rather than a silent no-op that would corrupt the
    /// audit ledger.
    #[tokio::test]
    async fn change_room_rejects_from_room_not_in_folio() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping change_room_rejects_from_room_not_in_folio — PG not reachable");
            return;
        };
        let (cin_id, _from, to_room_id) = match seed_active_checkin(&pool, "G4_notin").await {
            Some(ids) => ids,
            None => return,
        };
        let svc = build_service(pool.clone());
        // Pick a definitely-unrelated room id.
        let err = svc
            .change_room(ChangeRoomCommand {
                check_in_id: cin_id,
                from_room_id: 999_999, // not in this folio
                to_room_id,
                reason: None,
                actor: String::new(),
                source: EventSource::System {
                    reason: "G4_notin".into(),
                },
            })
            .await
            .expect_err("from-room not in folio must reject");
        match err {
            ServiceError::Validation(msg) => {
                assert!(msg.contains("not currently assigned"), "got: {msg}");
            }
            other => panic!("expected Validation, got {other:?}"),
        }
        cleanup(&pool, cin_id).await;
    }

    // -------------------- test helpers --------------------

    /// Seed a minimal `(customer, two rooms, active check-in, junction row)`
    /// graph for the change-room tests. Returns `(cin_id, from_room_id,
    /// to_room_id)` or `None` if any seed step fails (e.g. unique
    /// constraint on rooms.room_no). All rows are tagged with the
    /// `marker` so `cleanup` can remove them deterministically.
    async fn seed_active_checkin(pool: &PgPool, marker: &str) -> Option<(i32, i32, i32)> {
        // Customer
        let cust_id: i32 = sqlx::query_scalar(
            "INSERT INTO ht_customers (cust_firstname, cust_lastname) \
             VALUES ($1, 'TestG4') RETURNING cust_id",
        )
        .bind(format!("Guest_{marker}"))
        .fetch_one(pool)
        .await
        .ok()?;

        // Two rooms. We pick room_no values unlikely to collide with
        // seed data — the marker is included so reruns don't conflict
        // (cleanup wipes them but a previous crashed run may have
        // leftovers).
        let room_no_from = format!("G4F{marker}");
        let room_no_to = format!("G4T{marker}");
        let from_room_id: i32 = sqlx::query_scalar(
            "INSERT INTO ht_rooms_new (room_no, room_status) \
             VALUES ($1, 'available') RETURNING room_id",
        )
        .bind(&room_no_from)
        .fetch_one(pool)
        .await
        .ok()?;
        let to_room_id: i32 = sqlx::query_scalar(
            "INSERT INTO ht_rooms_new (room_no, room_status) \
             VALUES ($1, 'available') RETURNING room_id",
        )
        .bind(&room_no_to)
        .fetch_one(pool)
        .await
        .ok()?;

        // Active check-in
        let cin_id: i32 = sqlx::query_scalar(
            "INSERT INTO ht_checkins (\
                cin_no, cin_cust_id, cin_room_id, cin_checkin_time, \
                cin_expected_checkout, cin_status, cin_rate_per_night\
             ) VALUES ($1, $2, $3, NOW(), CURRENT_DATE + 1, 'active', 1200) \
             RETURNING cin_id",
        )
        .bind(format!("CIN-{marker}"))
        .bind(cust_id)
        .bind(from_room_id)
        .fetch_one(pool)
        .await
        .ok()?;

        // Junction row (B2 mapper would have populated this; we seed it
        // explicitly so we exercise the UPDATE path).
        sqlx::query(
            "INSERT INTO ht_checkin_rooms (\
                cr_cin_id, cr_room_id, cr_room_status, cr_rate_per_night\
             ) VALUES ($1, $2, 'active', 1200)",
        )
        .bind(cin_id)
        .bind(from_room_id)
        .execute(pool)
        .await
        .ok()?;

        Some((cin_id, from_room_id, to_room_id))
    }

    /// Tear down the rows seeded by `seed_active_checkin`. Best-effort:
    /// missing rows (already-cleaned, FK cascade) are silently
    /// tolerated.
    async fn cleanup(pool: &PgPool, cin_id: i32) {
        let _ = sqlx::query("DELETE FROM writeback_jobs WHERE aggregate_id = $1")
            .bind(aggregate_uuid(AggregateKind::CheckIn, cin_id))
            .execute(pool)
            .await;
        let _ = sqlx::query("DELETE FROM ht_room_changes WHERE rc_cin_id = $1")
            .bind(cin_id)
            .execute(pool)
            .await;
        let _ = sqlx::query("DELETE FROM ht_checkin_rooms WHERE cr_cin_id = $1")
            .bind(cin_id)
            .execute(pool)
            .await;
        let _ = sqlx::query(
            "DELETE FROM ht_rooms_new WHERE room_id IN \
              (SELECT cin_room_id FROM ht_checkins WHERE cin_id = $1)",
        )
        .bind(cin_id)
        .execute(pool)
        .await;
        let _ = sqlx::query(
            "DELETE FROM ht_customers WHERE cust_id IN \
              (SELECT cin_cust_id FROM ht_checkins WHERE cin_id = $1)",
        )
        .bind(cin_id)
        .execute(pool)
        .await;
        let _ = sqlx::query("DELETE FROM ht_checkins WHERE cin_id = $1")
            .bind(cin_id)
            .execute(pool)
            .await;
        let _ = sqlx::query("DELETE FROM ht_rooms_new WHERE room_no LIKE 'G4%'")
            .execute(pool)
            .await;
    }
}
