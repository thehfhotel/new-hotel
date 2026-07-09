//! Loyalty-channel service — orchestrates the loyalty app's booking flows
//! (availability quote → tentative HOLD → payment-verified confirm → release
//! / expiry sweep). See `docs/loyalty-channel.md` and `routes::channel`.
//!
//! ## Where this sits
//!
//! A hold is a normal `ht_bookings` row (`book_status='pending'`,
//! `book_channel='loyalty'`, one assigned room, `book_hold_expires_at` set)
//! created through [`BookingService::create`] — the SAME path the booking
//! form uses — so every dual-write invariant holds unchanged:
//!
//! * A roomed `pending` booking writes back to iHOTEL as `จอง` (the repo's
//!   existing rule — `create` gates the legacy mirror on room presence, not
//!   status). That is deliberate: iHOTEL receptionists must SEE the hold,
//!   otherwise they'd double-book the room during the payment window. Legacy
//!   has no tentative/confirmed distinction (both are `จอง`), so...
//! * ...payment-verified is a PG-only flip (`pending` → `confirmed` +
//!   deposit recorded). No legacy write: the validated `booking_modify`
//!   recipe has no deposit (`Book_Price_Pay`) leg, and inventing one would
//!   violate the byte-parity rule. Known, documented divergence: iHOTEL
//!   shows the booking with deposit 0; folio truth lands at checkout.
//! * Release / expiry rides the normal cancel writeback
//!   (`WritebackIntent::CancelBooking`) so iHOTEL sees the room free again.
//!
//! Constructed per request by `routes::channel` bound to the branch pool +
//! the branch's `WiredServices` (same shape as `resolve_write_services`),
//! and by the scheduler sweep bound to each site's pool.

use std::sync::Arc;

use chrono::{DateTime, Duration, NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::booking::BookingState;
use crate::domain::shared::{DateRange, Money};
use crate::outbox::event::{BookingSnapshot, DomainEvent, EventSource};
use crate::outbox::intent::WritebackIntent;
use crate::outbox::{generate_idempotency_key, EventBus, OutboxRepository};
use crate::repository::channel as channel_repo;
use crate::repository::channel::RoomTypeAvailability;
use crate::repository::CustomerRepository;

use super::booking::naive_date_to_utc;
use super::error::{ServiceError, ServiceResult};
use super::ids::{aggregate_uuid, AggregateKind};
use super::{
    BookingRoomCommand, BookingService, BookingWritebackContext, CreateBookingCommand,
    CreateCustomerCommand, CustomerService,
};

/// How long a channel hold reserves the room while the loyalty app collects
/// payment. Locked by the interface contract: `hold_expires_at = now + 2h`.
pub const HOLD_TTL: Duration = Duration::hours(2);

/// `book_channel` marker every loyalty-channel booking carries. Doubles as
/// the caller-idempotency channel label (migration 076 machinery) and the
/// expiry sweep's filter.
pub const LOYALTY_CHANNEL: &str = "loyalty";

/// Payment plan the guest chose in the loyalty app.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentPlan {
    /// 50% of the total due now (rounded per [`amount_due_satang`]).
    Deposit50,
    /// Full amount due now.
    Full,
}

/// Command for [`ChannelService::create_hold`].
#[derive(Debug, Clone)]
pub struct CreateHoldCommand {
    /// Route-generated `YYYYMMDD-NNNN` (same allocator as the booking form).
    pub book_no: String,
    pub room_type_id: i32,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub guests: i32,
    pub guest_name: String,
    pub guest_phone: String,
    pub membership_id: Option<String>,
    pub payment: PaymentPlan,
    pub source: EventSource,
}

/// Outcome of a successful hold create.
#[derive(Debug, Clone)]
pub struct HoldOutcome {
    pub book_id: i32,
    pub book_no: String,
    pub total_baht: f64,
    pub amount_due_baht: f64,
    pub hold_expires_at: DateTime<Utc>,
}

/// Outcome of `confirm_payment` (payment-verified). `already_confirmed` is
/// the idempotent-replay marker — the contract requires replays to succeed.
#[derive(Debug, Clone)]
pub struct ConfirmOutcome {
    pub book_id: i32,
    pub deposit_baht: f64,
    pub balance_due_baht: f64,
    pub already_confirmed: bool,
}

/// Outcome of `release`. `already_released` marks an idempotent replay.
#[derive(Debug, Clone)]
pub struct ReleaseOutcome {
    pub book_id: i32,
    pub already_released: bool,
}

/// Per-request/per-site service handle (cheap: Arc clones + pool handle).
#[derive(Clone)]
pub struct ChannelService {
    pg: PgPool,
    bookings: Arc<BookingService>,
    customers_service: Arc<CustomerService>,
    customers_repo: Arc<dyn CustomerRepository>,
}

impl ChannelService {
    pub fn new(
        pg: PgPool,
        bookings: Arc<BookingService>,
        customers_service: Arc<CustomerService>,
        customers_repo: Arc<dyn CustomerRepository>,
    ) -> Self {
        Self {
            pg,
            bookings,
            customers_service,
            customers_repo,
        }
    }

    /// Per-room-type availability + nightly quote for the stay window.
    pub async fn availability(
        &self,
        check_in: NaiveDate,
        check_out: NaiveDate,
        guests: i32,
    ) -> ServiceResult<Vec<RoomTypeAvailability>> {
        validate_stay(check_in, check_out)?;
        if guests < 1 {
            return Err(ServiceError::validation("guests must be >= 1"));
        }
        Ok(channel_repo::availability_by_type(&self.pg, check_in, check_out, guests).await?)
    }

    /// Create a TENTATIVE hold: match-or-create the guest, pick a free room
    /// of the requested type, and drive [`BookingService::create`] with
    /// `status='pending'` + the hold deadline. Consumes availability
    /// immediately (the room is assigned) and mirrors to iHOTEL as `จอง`
    /// through the normal create writeback.
    pub async fn create_hold(&self, cmd: CreateHoldCommand) -> ServiceResult<HoldOutcome> {
        validate_stay(cmd.check_in, cmd.check_out)?;
        if cmd.guests < 1 {
            return Err(ServiceError::validation("guests must be >= 1"));
        }
        let (first_name, last_name) = split_guest_name(&cmd.guest_name)?;
        let phone = cmd.guest_phone.trim();
        if phone.is_empty() {
            return Err(ServiceError::validation("guest.phone must not be empty"));
        }

        // Room type + quote. The nightly price the guest saw in availability
        // is the price the hold is written with (type_base_price).
        let (type_name, nightly_baht) =
            channel_repo::type_nightly_price(&self.pg, cmd.room_type_id)
                .await?
                .ok_or_else(|| {
                    ServiceError::not_found(format!(
                        "room type {} does not exist or is inactive",
                        cmd.room_type_id
                    ))
                })?;

        let room = channel_repo::pick_free_room(&self.pg, cmd.room_type_id, cmd.check_in, cmd.check_out)
            .await?
            .ok_or_else(|| {
                ServiceError::conflict(format!(
                    "no {type_name} room available for {} to {}",
                    cmd.check_in, cmd.check_out
                ))
            })?;

        // Guest: match (exact phone + case-insensitive name) or create.
        let customer_id = match self
            .customers_repo
            .find_by_phone_name(&self.pg, phone, first_name, last_name)
            .await?
        {
            Some(id) => id,
            None => {
                self.customers_service
                    .create(CreateCustomerCommand {
                        first_name: first_name.to_string(),
                        last_name: last_name.map(str::to_string),
                        phone: Some(phone.to_string()),
                        email: None,
                        id_card: None,
                        address: None,
                        customer_type: None,
                        notes: None,
                        enrichment: Default::default(),
                        source: cmd.source.clone(),
                    })
                    .await?
                    .customer_id
            }
        };

        // Membership link (PG-only; last-write-wins — see attach_membership).
        if let Some(membership) = cmd.membership_id.as_deref().map(str::trim) {
            if !membership.is_empty() {
                channel_repo::attach_membership(&self.pg, customer_id, membership).await?;
            }
        }

        let nights = (cmd.check_out - cmd.check_in).num_days().max(1);
        let nightly = Money::from_satang((nightly_baht * 100.0).round() as i64);
        let total = Money::from_satang(nightly.as_satang() * nights);
        let due = Money::from_satang(amount_due_satang(total.as_satang(), cmd.payment));
        let hold_expires_at = Utc::now() + HOLD_TTL;

        let customer_name = match last_name {
            Some(last) => format!("{first_name} {last}"),
            None => first_name.to_string(),
        };

        let writeback_context = BookingWritebackContext {
            customer_aggregate_id: aggregate_uuid(AggregateKind::Customer, customer_id),
            legacy_cust_no: None,
            customer_name,
            customer_phone: Some(phone.to_string()),
            stay: DateRange::new(
                naive_date_to_utc(cmd.check_in),
                naive_date_to_utc(cmd.check_out),
            ),
            room_no: room.room_no.clone(),
            room_type: room.type_name.clone(),
            price: nightly,
            // No money has changed hands at hold time — the legacy
            // `Book_Price_Pay` starts at 0 like any undeposited booking.
            deposit: Money::ZERO,
            // Matches the existing create path (routes::new_bookings passes
            // an empty created_by) for byte-parity of the legacy INSERT.
            created_by: String::new(),
            notes: Some(HOLD_NOTES.to_string()),
        };

        let outcome = self
            .bookings
            .create(CreateBookingCommand {
                book_no: cmd.book_no.clone(),
                customer_id,
                check_in: cmd.check_in,
                check_out: cmd.check_out,
                adults: cmd.guests,
                children: 0,
                status: "pending".to_string(),
                source_label: Some(LOYALTY_CHANNEL.to_string()),
                total_amount: Some(total.as_satang() as f64 / 100.0),
                deposit_amount: None,
                notes: Some(HOLD_NOTES.to_string()),
                rooms: vec![BookingRoomCommand {
                    room_id: room.room_id,
                    price_per_night: Some(nightly.as_satang() as f64 / 100.0),
                }],
                products: Vec::new(),
                writeback_context,
                book_channel: Some(LOYALTY_CHANNEL.to_string()),
                book_ext_ref: None,
                hold_expires_at: Some(hold_expires_at),
                source: cmd.source,
            })
            .await?;

        Ok(HoldOutcome {
            book_id: outcome.book_id,
            book_no: outcome.book_no.unwrap_or(cmd.book_no),
            total_baht: total.as_satang() as f64 / 100.0,
            amount_due_baht: due.as_satang() as f64 / 100.0,
            hold_expires_at,
        })
    }

    /// Payment-verified: flip a `pending` hold to `confirmed`, recording the
    /// received deposit. Idempotent — a replay against an already-confirmed
    /// booking succeeds without writing. PG-only (see module doc for why the
    /// deposit is not mirrored to iHOTEL).
    pub async fn confirm_payment(
        &self,
        book_id: i32,
        amount_baht: f64,
    ) -> ServiceResult<ConfirmOutcome> {
        if !(amount_baht.is_finite() && amount_baht >= 0.0) {
            return Err(ServiceError::validation("amount must be a non-negative number"));
        }

        let mut tx = self.pg.begin().await?;

        // FOR UPDATE: serializes against a racing release / expiry sweep.
        let row = channel_repo::lock_channel_booking(&mut tx, book_id)
            .await?
            .filter(|r| r.channel.as_deref() == Some(LOYALTY_CHANNEL))
            .ok_or_else(|| {
                ServiceError::not_found(format!("loyalty-channel booking {book_id} not found"))
            })?;

        match row.status.as_str() {
            "pending" => {}
            // Idempotent replay — report the stored numbers, write nothing.
            "confirmed" | "checkedin" | "completed" => {
                if (row.deposit_amount - amount_baht).abs() > 0.01 {
                    tracing::warn!(
                        book_id,
                        stored_deposit = row.deposit_amount,
                        replay_amount = amount_baht,
                        "loyalty payment-verified replay carries a different amount; keeping stored value"
                    );
                }
                return Ok(ConfirmOutcome {
                    book_id,
                    deposit_baht: row.deposit_amount,
                    balance_due_baht: (row.total_amount - row.deposit_amount).max(0.0),
                    already_confirmed: true,
                });
            }
            "cancelled" => {
                return Err(ServiceError::conflict(format!(
                    "hold {book_id} was already released/expired; create a new booking"
                )));
            }
            other => {
                return Err(ServiceError::conflict(format!(
                    "booking {book_id} is in state '{other}' and cannot be confirmed"
                )));
            }
        }

        let rows = channel_repo::confirm_booking_payment(&mut tx, book_id, amount_baht).await?;
        if rows == 0 {
            // The FOR UPDATE row said 'pending'; a zero here is unreachable
            // short of a concurrent writer bypassing the lock. Fail loudly.
            return Err(ServiceError::conflict(format!(
                "hold {book_id} changed state during confirmation; retry"
            )));
        }

        let aggregate_id = aggregate_uuid(AggregateKind::Booking, book_id);
        let stay = DateRange::new(
            naive_date_to_utc(row.check_in),
            naive_date_to_utc(row.check_out),
        );
        let snapshot = |state: BookingState| BookingSnapshot {
            id: aggregate_id,
            legacy_book_id: None,
            customer_id: aggregate_uuid(AggregateKind::Customer, row.customer_id),
            state,
            stay_start: stay.start,
            stay_end: stay.end,
            room_no: None,
            price: Money::from_satang((row.total_amount * 100.0).round() as i64),
        };
        let event = DomainEvent::BookingModified {
            id: aggregate_id,
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
            before: snapshot(BookingState::Pending),
            after: snapshot(BookingState::Active),
        };
        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        tx.commit().await?;

        Ok(ConfirmOutcome {
            book_id,
            deposit_baht: amount_baht,
            balance_due_baht: (row.total_amount - amount_baht).max(0.0),
            already_confirmed: false,
        })
    }

    /// Release a hold (loyalty-side payment window lapsed, or the expiry
    /// sweep). Idempotent — releasing an already-cancelled hold succeeds.
    /// Guarded on `book_status='pending'` (NOT the generic cancel guard) so
    /// a release can never cancel a hold that payment-verified just
    /// confirmed. The legacy mirror rides the normal `CancelBooking`
    /// writeback so iHOTEL sees the room free again.
    pub async fn release(&self, book_id: i32, reason: &str) -> ServiceResult<ReleaseOutcome> {
        let mut tx = self.pg.begin().await?;

        let row = channel_repo::lock_channel_booking(&mut tx, book_id)
            .await?
            .filter(|r| r.channel.as_deref() == Some(LOYALTY_CHANNEL))
            .ok_or_else(|| {
                ServiceError::not_found(format!("loyalty-channel booking {book_id} not found"))
            })?;

        match row.status.as_str() {
            "cancelled" => {
                // Idempotent replay — nothing to write (tx read-only, drop it).
                return Ok(ReleaseOutcome {
                    book_id,
                    already_released: true,
                });
            }
            "pending" => {}
            other => {
                return Err(ServiceError::conflict(format!(
                    "booking {book_id} is '{other}' (payment already verified?); refusing to release"
                )));
            }
        }

        let rows = channel_repo::release_hold(&mut tx, book_id, reason).await?;
        if rows == 0 {
            return Err(ServiceError::conflict(format!(
                "hold {book_id} changed state during release; retry"
            )));
        }

        let aggregate_id = aggregate_uuid(AggregateKind::Booking, book_id);
        // Same intent + deterministic key BookingService::cancel would use,
        // so a later manual cancel of the same booking maps to the same
        // ledger row (no double legacy write).
        let intent = WritebackIntent::CancelBooking {
            booking_id: aggregate_id,
        };
        let key = generate_idempotency_key(&intent, aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(ServiceError::from_enqueue_error)?;

        let event = DomainEvent::BookingCancelled {
            id: aggregate_id,
            source: EventSource::our_app(Uuid::nil(), Uuid::new_v4()),
            reason: Some(reason.to_string()),
        };
        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        tx.commit().await?;

        Ok(ReleaseOutcome {
            book_id,
            already_released: false,
        })
    }

    /// Auto-release every hold whose payment window has lapsed. The
    /// scheduler's belt-and-braces behind the loyalty app's own `release`
    /// call. Per-hold failures are logged and skipped — one bad row must not
    /// wedge the sweep. Returns the number of holds released.
    pub async fn sweep_expired_holds(&self, site_id: &str) -> usize {
        let ids = match channel_repo::expired_hold_ids(&self.pg).await {
            Ok(ids) => ids,
            Err(err) => {
                tracing::error!(site = %site_id, error = %err, "loyalty hold sweep: query failed");
                return 0;
            }
        };

        let mut released = 0usize;
        for book_id in ids {
            match self.release(book_id, "loyalty hold expired (auto-release)").await {
                Ok(outcome) if !outcome.already_released => {
                    tracing::info!(site = %site_id, book_id, "loyalty hold sweep: released expired hold");
                    released += 1;
                }
                Ok(_) => {}
                Err(err) => {
                    // Conflict = raced with payment-verified — expected, fine.
                    tracing::warn!(
                        site = %site_id,
                        book_id,
                        error = %err,
                        "loyalty hold sweep: skipping hold (raced or errored)"
                    );
                }
            }
        }
        released
    }
}

/// Notes stamped on every channel hold (canonical `book_notes` AND the
/// legacy `Book_Details` via the create recipe). ASCII on purpose.
const HOLD_NOTES: &str = "Loyalty app booking";

/// 50% deposit (round half-up to the satang) or the full amount. Pure —
/// unit-tested below; keep all rounding here so the route, the DB write and
/// the response can never disagree.
pub fn amount_due_satang(total_satang: i64, plan: PaymentPlan) -> i64 {
    match plan {
        PaymentPlan::Full => total_satang,
        PaymentPlan::Deposit50 => (total_satang + 1) / 2,
    }
}

/// Split a free-form guest name into (first, rest-as-last). The loyalty app
/// sends one `name` string; `ht_customers` stores first/last separately.
fn split_guest_name(name: &str) -> ServiceResult<(&str, Option<&str>)> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return Err(ServiceError::validation("guest.name must not be empty"));
    }
    match trimmed.split_once(char::is_whitespace) {
        Some((first, rest)) => Ok((first, Some(rest.trim()).filter(|s| !s.is_empty()))),
        None => Ok((trimmed, None)),
    }
}

fn validate_stay(check_in: NaiveDate, check_out: NaiveDate) -> ServiceResult<()> {
    if check_out <= check_in {
        return Err(ServiceError::validation(format!(
            "check_out ({check_out}) must be after check_in ({check_in})"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // ----- amount_due_satang: the contract's "50% rounded per repo money
    // conventions" (integer satang, round half-up) -----

    #[test]
    fn deposit50_halves_even_totals_exactly() {
        // 2 nights × 1,200.00 THB = 240,000 satang → 120,000 satang due.
        assert_eq!(amount_due_satang(240_000, PaymentPlan::Deposit50), 120_000);
    }

    #[test]
    fn deposit50_rounds_odd_satang_half_up() {
        // 99,999 satang (999.99 THB) → 50,000 satang (500.00 THB), never
        // 49,999.5 — Money is integer satang.
        assert_eq!(amount_due_satang(99_999, PaymentPlan::Deposit50), 50_000);
        assert_eq!(amount_due_satang(1, PaymentPlan::Deposit50), 1);
        assert_eq!(amount_due_satang(0, PaymentPlan::Deposit50), 0);
    }

    #[test]
    fn full_plan_charges_everything_now() {
        assert_eq!(amount_due_satang(240_000, PaymentPlan::Full), 240_000);
    }

    #[test]
    fn deposit_plus_balance_never_exceeds_total() {
        for total in [0i64, 1, 99_999, 100_000, 123_457] {
            let due = amount_due_satang(total, PaymentPlan::Deposit50);
            let balance = total - due;
            assert!(due >= balance, "deposit must cover at least half");
            assert!(due + balance == total, "no satang minted or lost");
        }
    }

    // ----- guest-name splitting -----

    #[test]
    fn split_name_first_last() {
        assert_eq!(split_guest_name("Somchai Jaidee").unwrap(), ("Somchai", Some("Jaidee")));
    }

    #[test]
    fn split_name_single_token_has_no_lastname() {
        assert_eq!(split_guest_name("Madonna").unwrap(), ("Madonna", None));
    }

    #[test]
    fn split_name_multi_part_lastname_stays_joined() {
        assert_eq!(
            split_guest_name("  Anna Maria  van der Berg ").unwrap(),
            ("Anna", Some("Maria  van der Berg"))
        );
    }

    #[test]
    fn split_name_rejects_blank() {
        assert!(split_guest_name("   ").is_err());
    }

    // ----- stay validation -----

    #[test]
    fn stay_must_be_at_least_one_night() {
        let d = |s: &str| NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap();
        assert!(validate_stay(d("2026-08-01"), d("2026-08-02")).is_ok());
        assert!(validate_stay(d("2026-08-01"), d("2026-08-01")).is_err());
        assert!(validate_stay(d("2026-08-02"), d("2026-08-01")).is_err());
    }
}
