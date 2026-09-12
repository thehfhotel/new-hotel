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
use crate::repository::inventory_lock::InventoryLock;
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
    /// Contract property id (`hf` / `hfville`) — the scope of the
    /// booking-inventory lock this hold takes across pick → create (B8e / L3,
    /// `repository::inventory_lock`). The route already parsed it out of the
    /// request body, so this is the SAME string the desk create path locks on
    /// for the same property; the two must not drift or the lock stops
    /// excluding.
    pub property: String,
    pub room_type_id: i32,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub guests: i32,
    pub guest_name: String,
    pub guest_phone: String,
    pub membership_id: Option<String>,
    pub payment: PaymentPlan,
    /// Caller-idempotency key for the hold, stamped onto the booking itself as
    /// `ht_bookings.book_ext_ref` so migration 076's partial UNIQUE index
    /// `(book_channel, book_ext_ref)` dedupes INSIDE the booking transaction
    /// (B8d / issue #305). Built by the route from the caller identity + the
    /// `Idempotency-Key` header — see [`hold_ext_ref`].
    ///
    /// `None` = an unkeyed request, which behaves exactly as it always has:
    /// every call mints a new hold.
    pub ext_ref: Option<String>,
    /// SHA-256 of the canonicalised request (migration 095 / issue #305), the
    /// SAME value `ht_channel_idempotency.idem_fingerprint` carries. Stored
    /// with the hold so a key reused for a materially DIFFERENT request is
    /// refused (422) rather than replaying an unrelated stay, even once the
    /// 093 row is gone — after a crash, or after its 24 h TTL.
    ///
    /// `None` only on an unkeyed request, where there is no key to bind.
    pub ext_ref_fingerprint: Option<String>,
    pub source: EventSource,
}

/// The `ht_bookings.book_ext_ref` a keyed loyalty hold carries (B8d).
///
/// `caller` is `routes::channel::caller_identity` — the SHA-256 of the
/// presented channel bearer, the same value `ht_channel_idempotency.idem_caller`
/// stores, and safe to persist for exactly that reason (it is a digest, never
/// the token). Including it matters because `book_ext_ref` is unique only
/// within `book_channel`, which is the constant `'loyalty'` for every hold:
/// without the caller scope two clients — or the same client either side of a
/// token rotation, which deliberately starts a fresh key space — could collide
/// on a key as ordinary as `"1"` and the second would REPLAY the first's
/// booking. The `idem:` prefix keeps the value self-describing in a psql
/// session and keeps it out of the namespace a genuine OTA `channel_booking_id`
/// would occupy.
pub fn hold_ext_ref(caller: &str, key: &str) -> String {
    format!("idem:{caller}:{key}")
}

/// Outcome of a successful hold create.
#[derive(Debug, Clone)]
pub struct HoldOutcome {
    pub book_id: i32,
    pub book_no: String,
    pub total_baht: f64,
    pub amount_due_baht: f64,
    pub hold_expires_at: DateTime<Utc>,
    /// `true` when this call returned an EXISTING hold that the same
    /// `ext_ref` already created, rather than minting one (B8d / issue #305).
    /// The route stamps `Idempotency-Replayed: true` on the response.
    ///
    /// Distinct from the `ht_channel_idempotency` replay: that one short-circuits
    /// before the service is entered at all. This one fires when the KEY record
    /// is gone but the BOOKING survived — a crash between the two transactions —
    /// which is precisely the case the key table could not cover.
    pub replayed: bool,
}

/// What [`ChannelService::create_hold`] did — three genuinely different
/// answers that `Result<HoldOutcome, _>` used to flatten into two.
#[derive(Debug, Clone)]
pub enum HoldCreateOutcome {
    /// A new hold was minted. **201**.
    Created(HoldOutcome),
    /// This key already made this hold and it is still live; the payload is
    /// rendered from the STORED row. **201** + `Idempotency-Replayed: true`.
    Replayed(HoldOutcome),
    /// The key is bound to a materially DIFFERENT request (the fingerprint
    /// stored with the surviving booking does not match this one). The route
    /// renders the SAME 422 `ht_channel_idempotency` renders for the same
    /// mistake, so a client cannot tell — and need not care — which of the two
    /// records caught it.
    KeyReusedForDifferentRequest,
    /// **B8e / L2 — the last-room floor.** The property has `free_rooms`
    /// sellable rooms left for the requested nights and the configured floor
    /// is `floor`; `free_rooms <= floor`, so the channel stands down and the
    /// remaining rooms stay for the FRONT DESK. **409** with a stable machine
    /// reason, not a generic sold-out: the loyalty app shows call-the-desk
    /// copy for this, and a guest who is told "sold out" while reception can
    /// still sell the room has been told something false.
    ///
    /// A FLOOR, never a cap (the B8 analysis rejects allotments — loyalty-app
    /// ADR-0003): it only bites on the last `floor` rooms, so a property with
    /// slack sells through the channel exactly as it did before.
    LastRoomHeldForDesk { free_rooms: i64, floor: i64 },
    /// No room of the requested type is sellable for the window. **409**.
    ///
    /// An outcome rather than a `ServiceError::Conflict` so the route can give
    /// it a stable `reason` of its own: `sold_out` and
    /// `last_room_held_for_desk` share a status code and mean genuinely
    /// different things to the guest ("try other dates" vs "call the desk"),
    /// and a client cannot tell them apart by matching on English prose. It is
    /// also what keeps a LOSER of the L3 race distinguishable from a lock
    /// TIMEOUT, which is now a 503 — the two used to collapse onto one
    /// `Conflict` and a test could not tell which it had caught.
    SoldOut { room_type: String },
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

/// The guest fields of a hold, validated BEFORE the booking-inventory lock is
/// taken and handed to the locked half as owned strings.
///
/// A struct rather than three positional `String`s so a future reorder cannot
/// silently swap the phone and the surname.
struct HoldGuest {
    first_name: String,
    last_name: Option<String>,
    phone: String,
}

/// Per-request/per-site service handle (cheap: Arc clones + pool handle).
#[derive(Clone)]
pub struct ChannelService {
    pg: PgPool,
    bookings: Arc<BookingService>,
    customers_service: Arc<CustomerService>,
    customers_repo: Arc<dyn CustomerRepository>,
    /// B8e / L2 — how many sellable rooms the property keeps for the desk.
    /// Injected rather than read from the environment inside `create_hold` so
    /// a test can pin it without mutating process env (and so the number that
    /// refused a hold is visible in the service that refused it).
    /// `0` disables the guard.
    last_room_floor: i64,
}

/// Decide what a surviving keyed booking means for THIS request (B8d).
///
/// Three outcomes, and the order matters: identity is checked before
/// liveness, because "you reused someone else's key" is a different mistake
/// from "the hold this key made is gone" and must not be reported as the
/// latter.
///
/// 1. **Fingerprint mismatch** ⇒ [`HoldCreateOutcome::KeyReusedForDifferentRequest`]
///    (422). `None` on either side is "no opinion" and does not mismatch — a
///    pre-095 booking, or an unkeyed path, has nothing to compare.
/// 2. **Not a live hold** ⇒ [`ServiceError::Conflict`] (409). The 201 contract
///    has no status field, so returning a cancelled, swept or already-paid
///    booking as a fresh hold would hand the client a `hold_expires_at` in the
///    past and no way to notice. The error names the stored status and the
///    booking id instead, which is actionable: look it up, or mint a new key.
/// 3. **Live hold** ⇒ [`HoldCreateOutcome::Replayed`] rendered from the STORED
///    row.
///
/// On the amount: `amount_due` is recomputed with [`amount_due_satang`]
/// rather than read back, because it is not a stored column (a pending hold's
/// `book_deposit_amount` is 0 — no money has moved). That is sound precisely
/// BECAUSE gate 1 ran first: the payment plan is part of the request
/// fingerprint, so by the time we get here the plan is proven identical to the
/// one the original attempt quoted. A replay can therefore never quote a
/// different figure than the original — the property the reviewer asked for,
/// enforced by the fingerprint rather than by a redundant column.
fn replay_keyed_hold(
    found: channel_repo::KeyedHold,
    requested_fingerprint: Option<&str>,
    payment: PaymentPlan,
    now: DateTime<Utc>,
) -> ServiceResult<HoldCreateOutcome> {
    let stored = found.booking;

    // 1. identity
    if let (Some(stored_fp), Some(requested_fp)) =
        (found.ext_ref_fingerprint.as_deref(), requested_fingerprint)
    {
        if stored_fp != requested_fp {
            tracing::warn!(
                book_id = stored.book_id,
                "loyalty hold create reused an Idempotency-Key for a different \
                 request; refusing rather than replaying an unrelated booking"
            );
            return Ok(HoldCreateOutcome::KeyReusedForDifferentRequest);
        }
    }

    // 2. liveness
    let expired = stored
        .hold_expires_at
        .is_none_or(|deadline| deadline <= now);
    if stored.status != "pending" || expired {
        return Err(ServiceError::conflict(format!(
            "this Idempotency-Key already created booking {} (status '{}', hold \
             deadline {}); it is no longer a live hold, so it cannot be replayed \
             as one — look the booking up, or retry with a NEW key",
            stored.book_no,
            stored.status,
            stored
                .hold_expires_at
                .map(|d| d.to_rfc3339())
                .unwrap_or_else(|| "none".to_string()),
        )));
    }

    // 3. replay from the stored row
    let total = Money::from_satang((stored.total_amount * 100.0).round() as i64);
    let due = Money::from_satang(amount_due_satang(total.as_satang(), payment));
    Ok(HoldCreateOutcome::Replayed(HoldOutcome {
        book_id: stored.book_id,
        book_no: stored.book_no,
        total_baht: total.as_satang() as f64 / 100.0,
        amount_due_baht: due.as_satang() as f64 / 100.0,
        // The ORIGINAL deadline, never `now + TTL`: a retry must not silently
        // extend a 2 h hold.
        hold_expires_at: stored
            .hold_expires_at
            .expect("liveness gate above rejects a hold with no deadline"),
        replayed: true,
    }))
}

impl ChannelService {
    /// `last_room_floor` is the B8e/L2 floor — the number of sellable rooms
    /// the property holds back for the front desk (`config::
    /// loyalty_last_room_floor`, default 1). `0` turns the guard off.
    pub fn new(
        pg: PgPool,
        bookings: Arc<BookingService>,
        customers_service: Arc<CustomerService>,
        customers_repo: Arc<dyn CustomerRepository>,
        last_room_floor: i64,
    ) -> Self {
        Self {
            pg,
            bookings,
            customers_service,
            customers_repo,
            last_room_floor,
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
    ///
    /// ## Crash-safe idempotency (B8d / issue #305)
    ///
    /// When `cmd.ext_ref` is set, the hold carries it as
    /// `ht_bookings.book_ext_ref` alongside `book_channel = 'loyalty'`, so
    /// migration 076's partial UNIQUE index dedupes the create INSIDE the
    /// booking's own transaction. That closes the window
    /// `ht_channel_idempotency` (migration 093) structurally cannot: the key
    /// row and the booking commit in DIFFERENT transactions, so a crash
    /// between them leaves the hold committed and the key gone, and the retry
    /// — entering as a FRESH reservation — used to mint a second hold against
    /// a second room that nobody would release before its 2 h deadline.
    ///
    /// Two arms reach the same answer, and both return `replayed: true`:
    ///
    /// * the pre-check below, which also matters because it runs BEFORE the
    ///   guest match-or-create — a retry must not leave a duplicate
    ///   `ht_customers` row behind on its way to discovering the booking
    ///   already exists;
    /// * `BookingService::create`'s own unique-violation arm, for two retries
    ///   racing each other (it rolls its half-built row back and re-selects
    ///   the winner).
    ///
    /// The replay payload is rendered from the STORED hold — its total, its
    /// original deadline — never re-quoted from the retry's request, so a
    /// price change between attempts cannot alter what the guest was told.
    ///
    /// ## Serialized pick → create, and the last-room floor (B8e / L3 + L2)
    ///
    /// The pick and the insert it feeds run under the property's
    /// booking-inventory advisory lock (`repository::inventory_lock`), which
    /// the DESK create path takes too. Two concurrent holds for the last room
    /// therefore no longer both succeed: the second one waits, re-runs the
    /// picker against the first one's committed booking, and refuses.
    ///
    /// Inside that lock the hold is also checked against the property-wide
    /// **last-room floor**: with `free_rooms <= last_room_floor` the channel
    /// stands down ([`HoldCreateOutcome::LastRoomHeldForDesk`]) so the desk —
    /// which is not gated — can still sell the room to the guest on the
    /// phone. A floor, not an allotment: with slack the channel is unchanged.
    pub async fn create_hold(&self, cmd: CreateHoldCommand) -> ServiceResult<HoldCreateOutcome> {
        validate_stay(cmd.check_in, cmd.check_out)?;
        if cmd.guests < 1 {
            return Err(ServiceError::validation("guests must be >= 1"));
        }

        // Crash-recovery replay: a prior attempt committed the hold but never
        // recorded its key. Answer from that hold and touch nothing else —
        // note this runs BEFORE the guest match-or-create below, which is what
        // keeps a replay from leaving a duplicate `ht_customers` row behind.
        if let Some(ext_ref) = cmd.ext_ref.as_deref() {
            if let Some(found) =
                channel_repo::channel_booking_by_ext_ref(&self.pg, LOYALTY_CHANNEL, ext_ref).await?
            {
                tracing::info!(
                    book_id = found.booking.book_id,
                    ext_ref,
                    "loyalty hold create matched an existing booking by its own ext_ref \
                     (the idempotency record did not survive the first attempt)"
                );
                return replay_keyed_hold(
                    found,
                    cmd.ext_ref_fingerprint.as_deref(),
                    cmd.payment,
                    Utc::now(),
                );
            }
        }
        // Everything that can REFUSE this request without reading inventory
        // runs before the lock is taken: a malformed guest name, a blank
        // phone, an unknown room type. Holding the property's lock while
        // rejecting a typo would make a real booking wait on a request that
        // was never going to consume a room.
        let (first_name, last_name) = split_guest_name(&cmd.guest_name)?;
        let (first_name, last_name) = (
            first_name.to_string(),
            last_name.map(|last| last.to_string()),
        );
        let phone = cmd.guest_phone.trim().to_string();
        if phone.is_empty() {
            return Err(ServiceError::validation("guest.phone must not be empty"));
        }

        // Room type + quote. The nightly price the guest saw in availability
        // is the price the hold is written with (type_base_price). A plain
        // read of a near-static table — no inventory, so it stays outside the
        // lock too.
        let (type_name, nightly_baht) =
            channel_repo::type_nightly_price(&self.pg, cmd.room_type_id)
                .await?
                .ok_or_else(|| {
                    ServiceError::not_found(format!(
                        "room type {} does not exist or is inactive",
                        cmd.room_type_id
                    ))
                })?;

        // B8e / L3 — everything past this point CONSUMES inventory: the
        // last-room floor reads it, the picker claims a room from it, and
        // `BookingService::create` commits that claim. Hold the property's
        // booking-inventory lock across all three, so a second hold — or a
        // desk create for the same night — re-evaluates against our COMMITTED
        // booking instead of the availability it read before we started. The
        // guard is released right after the create commits; dropping it on an
        // error path frees the lock too (see `repository::inventory_lock`).
        let lock = InventoryLock::acquire(&self.pg, &cmd.property).await?;
        let result = self
            .create_hold_locked(
                &cmd,
                HoldGuest {
                    first_name,
                    last_name,
                    phone,
                },
                &type_name,
                nightly_baht,
            )
            .await;
        if let Err(err) = lock.release().await {
            tracing::warn!(
                error = %err,
                "releasing the booking-inventory lock failed; it frees on connection return"
            );
        }
        result
    }

    /// The inventory-consuming half of [`ChannelService::create_hold`], run
    /// with the property's booking-inventory lock held (B8e / L3).
    ///
    /// Split out purely so the lock guard lives in a scope that cannot
    /// accidentally skip its release: every `?` in here returns into
    /// `create_hold`, which releases and only then propagates.
    async fn create_hold_locked(
        &self,
        cmd: &CreateHoldCommand,
        guest: HoldGuest,
        type_name: &str,
        nightly_baht: f64,
    ) -> ServiceResult<HoldCreateOutcome> {
        let HoldGuest {
            first_name,
            last_name,
            phone,
        } = guest;
        let (first_name, last_name, phone) =
            (first_name.as_str(), last_name.as_deref(), phone.as_str());

        // B8e / L2 — property-wide last-room floor. `surplus` is the shared
        // inventory CTE's own answer to "what may the channel still sell
        // property-wide for this window" (free rooms minus parked claims,
        // floored at 0 — B8a/B8c), so this guard cannot drift from the
        // counter or the picker: it reads the same number they do.
        //
        // Checked BEFORE the pick, and for the whole stay window rather than
        // per night, because `free_rooms` already requires a room to be free
        // for EVERY night of `[check_in, check_out)`.
        //
        // Type-independent on purpose: when the property is down to its last
        // rooms the channel stands down entirely, whatever type was asked
        // for. Answering "no Deluxe available" while reception can still sell
        // the Deluxe would be a false sold-out; "the desk is holding the last
        // rooms" is the true statement, and the loyalty app has copy for it.
        if self.last_room_floor > 0 {
            let snapshot =
                channel_repo::inventory_snapshot(&self.pg, cmd.check_in, cmd.check_out).await?;
            if snapshot.surplus <= self.last_room_floor {
                tracing::info!(
                    property = %cmd.property,
                    check_in = %cmd.check_in,
                    check_out = %cmd.check_out,
                    free_rooms = snapshot.surplus,
                    floor = self.last_room_floor,
                    "loyalty hold refused: the property is at its last-room floor; \
                     the remaining rooms stay for the desk"
                );
                return Ok(HoldCreateOutcome::LastRoomHeldForDesk {
                    free_rooms: snapshot.surplus,
                    floor: self.last_room_floor,
                });
            }
        }

        let room = channel_repo::pick_free_room(
            &self.pg,
            cmd.room_type_id,
            cmd.check_in,
            cmd.check_out,
            cmd.guests,
        )
        .await?;
        // Sold out for this type/window. An OUTCOME, not an error, so the
        // route can stamp `reason: "sold_out"` — see `HoldCreateOutcome
        // ::SoldOut`. This is also the arm a LOSER of the serialized race
        // lands on: it re-picked after the winner committed and found nothing.
        let Some(room) = room else {
            tracing::info!(
                property = %cmd.property,
                room_type = %type_name,
                check_in = %cmd.check_in,
                check_out = %cmd.check_out,
                "loyalty hold refused: no room of the requested type is sellable"
            );
            return Ok(HoldCreateOutcome::SoldOut {
                room_type: type_name.to_string(),
            });
        };

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
                // B8c / migration 094. `pick_free_room` already filtered on
                // `room_type_id = $3`, so this AGREES with the picked room by
                // construction — passing it explicitly turns that into a
                // checked invariant (`service::booking::resolve_room_type`
                // rejects a disagreement) rather than an assumption.
                room_type_id: Some(cmd.room_type_id),
                products: Vec::new(),
                writeback_context,
                book_channel: Some(LOYALTY_CHANNEL.to_string()),
                // B8e / L3: we already hold the property's booking-inventory
                // lock (across the pick above and this create). Re-acquiring
                // it inside `BookingService::create`, on a different pooled
                // connection, would deadlock against our own guard.
                inventory_lock: None,
                // B8d: the caller-idempotency key IS the natural key here, so
                // the (book_channel, book_ext_ref) index dedupes the hold in
                // the same transaction that creates it.
                book_ext_ref: cmd.ext_ref.clone(),
                // Migration 095 — bound to the key in the same statement, so
                // a surviving booking can refuse a key reused for a DIFFERENT
                // request instead of replaying an unrelated stay.
                book_ext_ref_fingerprint: cmd.ext_ref_fingerprint.clone(),
                hold_expires_at: Some(hold_expires_at),
                source: cmd.source.clone(),
            })
            .await?;

        // Lost the concurrent race inside `BookingService::create` — our row
        // was rolled back and the winner's booking came back instead. Re-select
        // BY THE KEY (not by id) so this arm gets the winner's fingerprint too
        // and runs the identical identity + liveness gates as the pre-check.
        if outcome.deduped {
            let ext_ref = cmd.ext_ref.as_deref().ok_or_else(|| {
                ServiceError::internal(
                    "BookingService::create reported a (book_channel, book_ext_ref) \
                     dedupe for a hold that carries no ext_ref",
                )
            })?;
            let found =
                channel_repo::channel_booking_by_ext_ref(&self.pg, LOYALTY_CHANNEL, ext_ref)
                    .await?
                    .ok_or_else(|| {
                        // Never fall through to a fresh-looking 201 here: our own
                        // row was rolled back, so "no winner found" means the
                        // caller would be told a hold exists that does not.
                        ServiceError::internal(format!(
                            "lost the (book_channel, book_ext_ref) race for '{ext_ref}' but the \
                         winning booking could not be re-selected"
                        ))
                    })?;
            tracing::info!(
                book_id = found.booking.book_id,
                ext_ref,
                "loyalty hold create lost the (book_channel, book_ext_ref) race; \
                 replaying the winning hold"
            );
            return replay_keyed_hold(
                found,
                cmd.ext_ref_fingerprint.as_deref(),
                cmd.payment,
                Utc::now(),
            );
        }

        Ok(HoldCreateOutcome::Created(HoldOutcome {
            book_id: outcome.book_id,
            book_no: outcome.book_no.unwrap_or_else(|| cmd.book_no.clone()),
            total_baht: total.as_satang() as f64 / 100.0,
            amount_due_baht: due.as_satang() as f64 / 100.0,
            hold_expires_at,
            replayed: false,
        }))
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
            return Err(ServiceError::validation(
                "amount must be a non-negative number",
            ));
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
            s if is_settled_status(s) => {
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
    ///
    /// This is the REQUESTED release — somebody (the loyalty app, via
    /// `routes::channel::release`) asked for it. It leaves
    /// `book_hold_auto_released_at` NULL. Only [`Self::sweep_expired_holds`]
    /// records an auto-release, through [`Self::release_with_cause`]; see
    /// [`ReleaseCause`].
    pub async fn release(&self, book_id: i32, reason: &str) -> ServiceResult<ReleaseOutcome> {
        self.release_with_cause(book_id, reason, ReleaseCause::Requested)
            .await
    }

    /// [`Self::release`] plus the typed reason the hold is dying, which is
    /// what decides whether `book_hold_auto_released_at` is stamped
    /// (migration 096, B13).
    ///
    /// Kept as a separate entry point rather than a fourth parameter on
    /// `release` so that the DEFAULT is the safe one: every existing caller
    /// keeps its signature and keeps writing NULL, and a path can only be
    /// counted as an expiry by naming [`ReleaseCause::PaymentWindowExpired`]
    /// out loud.
    pub async fn release_with_cause(
        &self,
        book_id: i32,
        reason: &str,
        cause: ReleaseCause,
    ) -> ServiceResult<ReleaseOutcome> {
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

        let rows =
            channel_repo::release_hold(&mut tx, book_id, reason, cause.is_auto_release()).await?;
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
            match self
                .release_with_cause(
                    book_id,
                    "loyalty hold expired (auto-release)",
                    ReleaseCause::PaymentWindowExpired,
                )
                .await
            {
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

/// Why a hold is being cancelled — the discriminator behind
/// `ht_bookings.book_hold_auto_released_at` (migration 096, B13).
///
/// It exists because both release paths end in the SAME
/// `repository::channel::release_hold` write, and until B13 the only thing
/// telling them apart was the free-text `book_cancel_reason`. That text cannot
/// carry the distinction: the sweep writes *"loyalty hold expired
/// (auto-release)"* and the channel's own endpoint writes *"loyalty payment
/// window lapsed (channel release)"* — both sentences say the payment window
/// ran out, yet only one of them is an expiry whose TTL we control. Counting
/// them together would inflate the expired-hold rate B13 exists to read, and
/// would make a `HOLD_TTL` change look effective for reasons that have nothing
/// to do with `HOLD_TTL`.
///
/// So the cause travels as a type instead of being re-derived from prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseCause {
    /// Somebody asked for the release — today that is the loyalty app calling
    /// `POST /api/channel/bookings/{id}/release`, for whatever reason of its
    /// own (guest abandoned the checkout, its own client-side timer, a retry).
    /// Leaves `book_hold_auto_released_at` NULL: this is a guest abandonment,
    /// not a TTL expiry, and it must never be counted as one.
    Requested,
    /// The scheduler's expiry sweep observed `book_hold_expires_at` in the
    /// past while the hold was still `pending`. THIS is the event B13 counts,
    /// and the only one that stamps `book_hold_auto_released_at`.
    PaymentWindowExpired,
}

impl ReleaseCause {
    /// Whether this release is the sweep's auto-release — i.e. whether
    /// `book_hold_auto_released_at` gets stamped. Named for the act rather
    /// than the verdict, matching the column. Sole reader:
    /// `ChannelService::release_with_cause`.
    pub fn is_auto_release(self) -> bool {
        matches!(self, Self::PaymentWindowExpired)
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

/// `book_status` values that mean "this hold is already past the payment step",
/// i.e. a `payment-verified` call for it is an idempotent REPLAY rather than a
/// state change.
///
/// **Both spellings of each state are accepted, deliberately.** One canonical
/// booking row can be written by two engines that spell the same state
/// differently, and neither spelling is wrong:
///
/// * our own app writes `'checkedin'` (`repository::checkin`, one word);
/// * the CT sync mapper writes `'checked_in'` when iHOTEL's `เข้าพัก` arrives
///   (`sync::mappers::booking::legacy_status_to_pg`), and that is the STEADY
///   state — a hold the desk later checks in through iHOTEL converges on the
///   underscored spelling.
///
/// Matching only `'checkedin'` meant a payment-verified retry landing after the
/// guest had been checked in through iHOTEL fell through to the catch-all and
/// answered **409 "booking is in state 'checked_in' and cannot be confirmed"** —
/// a hard error for what is simply a late replay. `'completed'` (legacy
/// `ออกแล้ว`) was already here; `'checkedout'`/`'checked_out'` join it for the
/// same reason, and the `-` spellings mirror the tolerant list
/// `service::booking::parse_booking_state` already keeps.
///
/// This changes only what we ACCEPT. It does not change what any writer
/// produces — the mapper's literals are untouched.
fn is_settled_status(status: &str) -> bool {
    matches!(
        status,
        "confirmed"
            | "checkedin"
            | "checked_in"
            | "checked-in"
            | "checkedout"
            | "checked_out"
            | "checked-out"
            | "completed"
    )
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

    // ----- is_settled_status: both spellings of every post-payment state -----

    #[test]
    fn settled_set_accepts_both_spellings_of_checked_in() {
        // `checkedin` is what THIS app writes; `checked_in` is what the CT sync
        // mapper writes for iHOTEL's `เข้าพัก`, and it is the steady state.
        for status in [
            "confirmed",
            "checkedin",
            "checked_in",
            "checked-in",
            "checkedout",
            "checked_out",
            "checked-out",
            "completed",
        ] {
            assert!(
                is_settled_status(status),
                "{status} must replay, not 409 — a payment-verified retry for a settled hold \
                 is not an error"
            );
        }
    }

    #[test]
    fn settled_set_excludes_states_that_are_not_a_replay() {
        // `pending` is the real work; `cancelled` is its own 409 with a
        // different, actionable message; unknown states must not be swallowed.
        for status in [
            "pending",
            "cancelled",
            "",
            "no_show",
            "confirmed_",
            "CHECKEDIN",
        ] {
            assert!(
                !is_settled_status(status),
                "{status} must NOT be treated as an already-confirmed replay"
            );
        }
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
        assert_eq!(
            split_guest_name("Somchai Jaidee").unwrap(),
            ("Somchai", Some("Jaidee"))
        );
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
