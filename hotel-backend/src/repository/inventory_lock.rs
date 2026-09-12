//! Booking-inventory serialisation lock (B8e / L3) — PG advisory lock.
//!
//! ## The race this closes
//!
//! `repository::channel::pick_free_room` and the desk form's
//! `room_is_available` are plain SELECTs, decoupled in time from the INSERT
//! that consumes what they found. Between the two there is a guest
//! match-or-create round trip, so the window is tens to hundreds of
//! milliseconds — long enough for a second writer to pick the SAME last room
//! and commit it. Nothing in the canonical schema rejects the second write:
//! `uq_ht_br_bookroom` stops one booking listing a room twice and says
//! nothing about two bookings claiming one room-night, and there is no
//! `EXCLUDE USING gist` on `ht_booking_rooms` (channel rows carry
//! `room_id = NULL` on the loyalty side by design, so the loyalty app's own
//! range constraint cannot cover them either).
//!
//! ## EXACTLY which paths take this lock
//!
//! Three, and only three. Do not read this module as "inventory is now
//! serialised"; it is not, and the list below is the whole of it:
//!
//! 1. **`service::channel::create_hold`** — holds it across last-room-floor
//!    check → `pick_free_room` → `BookingService::create`.
//! 2. **`service::booking::create`**, whenever
//!    `CreateBookingCommand::inventory_lock` is `Some(property)` — which
//!    `routes::new_bookings::create_booking` (the desk form and the OTA
//!    bridge) always sets, roomless creates included.
//! 3. **`service::booking::modify`** (B8g), whenever
//!    `ModifyBookingCommand::inventory_lock` is `Some(property)` — which
//!    `routes::new_bookings::update_booking` always sets — **AND** the edit
//!    changes the booking's room set (`service::booking::room_set_changed`).
//!    That covers the parked promote (first room assigned), a room swap, an
//!    added room and a room released back to the waitlist. An edit that leaves
//!    the rooms alone (notes, price, guest counts, status) takes NOTHING: it
//!    moves no inventory, and one property-wide lock on every desk save would
//!    serialise edits against creates for no benefit.
//!
//!    **Unlike 1 and 2, this one takes the lock INSIDE the caller's
//!    transaction** (B8h): `modify` locks the booking row first, reads the
//!    committed room set from behind that lock, and only then decides whether
//!    to acquire this lock — so the predicate and the legacy promote decision
//!    can no longer be made on two disagreeing snapshots. The lock is still
//!    taken before any WRITE, which is all this lock's exclusion needs. Why
//!    that inversion does not deadlock against 1 and 2 (and the one future
//!    change that would break it) is written up in the "Lock order" section of
//!    `service::checkin`'s module doc.
//!
//! **Everything else that moves inventory still runs unlocked**, by design and
//! for now:
//!
//! | path | why it is out of scope |
//! |---|---|
//! | walk-in check-in (`service::checkin::create`) | consumes a room directly; a desk-vs-desk race, unchanged from today |
//! | room change (`service::checkin::change_room`) | moves an occupied stay between rooms |
//! | stay extension (`service::checkin::extend_stay`) | lengthens an existing claim |
//! | booking edit that only RE-DATES an unchanged room set (`service::booking::modify`) | shifts which room-nights are consumed without touching the room set, so B8g's predicate does not fire; same class as the two rows above, and widening to dates is its own decision |
//! | CT sync mappers (`bin/sync.rs`) | replay iHOTEL's own writes; iHOTEL is the writer there and cannot be asked to take our lock |
//!
//! What protects the channel against those is **not** this lock — it is the
//! **L2 last-room floor**, which keeps a buffer of sellable rooms the channel
//! will not touch, so an unlocked desk-side write landing a beat later still
//! finds a room. The lock removes the hold-vs-desk-create and hold-vs-hold
//! races outright; the floor absorbs the rest. Widening the lock to the rows
//! above is a separate, larger change (they take real row locks and sit inside
//! their own transactions) and needs its own decision record.
//!
//! ## Why the key is per-PROPERTY, not per (type, date)
//!
//! The obvious key — `(property, room type, check-in date)` — is unsound
//! twice over:
//!
//! * **Dates.** Two stays that overlap need not share a check-in date
//!   (Nov 1-5 vs Nov 2-3). Keyed on the check-in date those two writers take
//!   DIFFERENT locks and can still land on one room.
//! * **Types.** Since B8a/B8c the channel's per-type availability is coupled
//!   property-wide through `inventory_surplus` — a parked (roomless) claim on
//!   ANY type caps what EVERY type may sell — and B8e's last-room floor is
//!   property-wide by definition. A type-scoped lock cannot serialise either
//!   quantity.
//!
//! So the lock is one per property: hold-create and desk-create are
//! human-paced (single digits per hour at both properties), the critical
//! section is a handful of statements, and a correct coarse lock beats a
//! fine-grained one that does not actually exclude.
//!
//! Properties are ALSO separated by the database itself — HF Hotel and HF
//! Ville have their own PostgreSQL databases and advisory locks are
//! database-scoped — so the property term in the key is belt and braces: it
//! keeps the lock correct if the two sites ever share a database.
//!
//! ## Why the wait polls instead of blocking
//!
//! `pg_advisory_xact_lock` blocks server-side, which would pin the waiting
//! task's pooled connection for the whole wait. Every waiter that pins a
//! connection while the holder needs a SECOND connection (the guard's, plus
//! the one `BookingService::create` begins) is a pool-exhaustion deadlock
//! waiting for a burst — `NEW_DB_POOL_MAX` defaults to 10. So we try, and on
//! failure DROP the transaction (returning the connection to the pool) before
//! sleeping. No connection is held while waiting.
//!
//! ## ⚠️ The guard depends on its transaction staying open
//!
//! The lock is `pg_advisory_xact_lock` inside a transaction this guard owns
//! and then leaves IDLE while the caller does its work on OTHER connections.
//! **A server-side `idle_in_transaction_session_timeout` would therefore kill
//! that backend mid-critical-section and silently release the lock**, with no
//! error anywhere near the caller — the pick and the insert would simply stop
//! excluding, and the double-sell would come back looking like a heisenbug.
//!
//! PostgreSQL ships that setting as `0` (disabled) and this repo never sets
//! it, on the server or per role. If it is ever turned on:
//!
//! * it must be larger than the whole create span (hundreds of ms), and
//! * this module should move to the structural design instead — take
//!   `pg_advisory_xact_lock` as the FIRST statement of the SAME transaction
//!   that does the INSERT, threading `&mut Transaction` through
//!   `pick_free_room` / `inventory_snapshot`.
//!
//! The structural version has no idle transaction at all and is strictly
//! better; it is not what shipped because `BookingService::create` owns its
//! transaction and giving every caller (desk form, OTA bridge, channel,
//! scheduler) a way to pass one in is a refactor with a far wider blast radius
//! than this fix. Recorded in `docs/adr/0009-booking-inventory-lock.md`.

use std::time::{Duration, Instant};

use sqlx::{PgPool, Postgres, Transaction};
use thiserror::Error;

/// First key slot of every booking-inventory advisory lock — a namespace, so
/// this lock can never collide with a future advisory-lock user. `BKIV`
/// ("booking inventory") in ASCII, which is what a `pg_locks` row shows.
pub const INVENTORY_LOCK_CLASS: i32 = i32::from_be_bytes(*b"BKIV");

/// How long [`InventoryLock::acquire`] keeps trying before giving up. Any
/// real wait here is milliseconds; five seconds means something is wedged and
/// the caller should be told so rather than hang.
const ACQUIRE_TIMEOUT: Duration = Duration::from_secs(5);
const FIRST_BACKOFF: Duration = Duration::from_millis(5);
const MAX_BACKOFF: Duration = Duration::from_millis(50);

/// Why a lock could not be taken.
#[derive(Debug, Error)]
pub enum InventoryLockError {
    /// A real database failure — not retryable, surfaces as a 500.
    #[error(transparent)]
    Db(sqlx::Error),

    /// Transient contention: either nobody released the lock inside
    /// [`ACQUIRE_TIMEOUT`], or the connection pool had nothing to lend us
    /// (`PoolTimedOut`) — indistinguishable to the caller and identical in
    /// remedy, so they share one variant. Callers surface this as `503` +
    /// `Retry-After`, never as a successful booking.
    ///
    /// This `Display` is the INTERNAL, logged form; the caller-facing message
    /// is built in `From<InventoryLockError> for ServiceError` and names
    /// neither the property nor this subsystem.
    #[error(
        "booking-inventory lock for property '{property}' unavailable after {waited:?} ({cause})"
    )]
    Busy {
        property: String,
        waited: Duration,
        cause: BusyCause,
    },
}

/// What stopped us from taking the lock — carried for the log line only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusyCause {
    /// Another writer held the lock for the whole acquire window.
    LockHeld,
    /// The PG connection pool could not lend a connection to even try.
    PoolExhausted,
}

impl std::fmt::Display for BusyCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BusyCause::LockHeld => f.write_str("another booking write held it"),
            BusyCause::PoolExhausted => f.write_str("no pooled connection available"),
        }
    }
}

/// The `(classid, objid)` pair `pg_advisory_xact_lock` is called with for
/// `property`. Exposed so tests (and a psql session reading `pg_locks`) can
/// name the same lock without re-deriving the hash.
pub fn lock_key(property: &str) -> (i32, i32) {
    (INVENTORY_LOCK_CLASS, fnv1a32(property.as_bytes()) as i32)
}

/// FNV-1a/32 — a stable, dependency-free string→int32 fold. The value only
/// has to be deterministic across processes and unlikely to collide across
/// the two property labels we have; it is not a security digest.
const fn fnv1a32(bytes: &[u8]) -> u32 {
    let mut hash: u32 = 0x811c_9dc5;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u32;
        hash = hash.wrapping_mul(0x0100_0193);
        i += 1;
    }
    hash
}

/// A held booking-inventory lock. Alive for exactly as long as the guard.
///
/// The lock is **transaction-scoped** (`pg_advisory_xact_lock`) on a
/// transaction this guard owns and never writes through. That is what makes
/// it leak-proof without an async `Drop`: if the guard is dropped on an error
/// path instead of [`released`](Self::release), sqlx queues a `ROLLBACK` for
/// the connection and flushes it when the connection returns to the pool, and
/// the rollback is what frees the lock. A session-scoped
/// (`pg_advisory_lock`) variant would have needed an explicit unlock that a
/// `?` could skip — and a skipped unlock wedges every later booking write.
///
/// Prefer [`release`](Self::release) anyway: it frees the lock at a
/// deterministic point (right after the caller's own commit) rather than
/// whenever the pool gets round to the connection.
///
/// A guard with `tx == None` is a **bypass** — the kill switch
/// (`BOOKING_INVENTORY_LOCK_ENABLED=false`) is off, so nothing is held and
/// every method is a no-op. The type is still handed back so callers keep one
/// code path.
pub struct InventoryLock {
    /// `None` after [`release`](Self::release), or when the lock is bypassed.
    tx: Option<Transaction<'static, Postgres>>,
    property: String,
    bypassed: bool,
}

impl std::fmt::Debug for InventoryLock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (class, obj) = lock_key(&self.property);
        f.debug_struct("InventoryLock")
            .field("property", &self.property)
            .field("key", &(class, obj))
            .field("held", &self.tx.is_some())
            .field("bypassed", &self.bypassed)
            .finish()
    }
}

impl InventoryLock {
    /// Take the booking-inventory lock for `property`, waiting up to
    /// [`ACQUIRE_TIMEOUT`].
    ///
    /// Returns a no-op bypass guard when `BOOKING_INVENTORY_LOCK_ENABLED` is
    /// off — the kill switch exists so an operator can un-serialise booking
    /// writes (channel holds, desk creates, room-moving edits) without a
    /// rollback deploy if this lock ever becomes the thing that is wrong. It
    /// re-opens the B8 §2.1/§2.3 double-sell window, so it is an incident tool,
    /// not a tuning knob.
    pub async fn acquire(pool: &PgPool, property: &str) -> Result<Self, InventoryLockError> {
        if !crate::config::booking_inventory_lock_enabled() {
            tracing::warn!(
                property,
                "BOOKING_INVENTORY_LOCK_ENABLED=false — booking writes are NOT serialised; \
                 concurrent writers can take the same last room"
            );
            return Ok(Self {
                tx: None,
                property: property.to_string(),
                bypassed: true,
            });
        }

        let (class, obj) = lock_key(property);
        let started = Instant::now();
        let mut backoff = FIRST_BACKOFF;

        let busy = |waited: Duration, cause: BusyCause| InventoryLockError::Busy {
            property: property.to_string(),
            waited,
            cause,
        };

        loop {
            // Deadline checked around the ACQUIRE as a whole, not only around
            // the sleeps: `pool.begin()` below has its own multi-second
            // acquire timeout, so without this a slow pool could stretch the
            // effective wait well past ACQUIRE_TIMEOUT and hold the caller's
            // request open for it.
            if started.elapsed() >= ACQUIRE_TIMEOUT {
                return Err(busy(started.elapsed(), BusyCause::LockHeld));
            }

            let mut tx = match pool.begin().await {
                Ok(tx) => tx,
                // A pool timeout is contention, not a fault: every connection
                // is busy right now. Surfacing it as `Db` would render a raw
                // 500 for a condition that clears by itself in milliseconds.
                Err(sqlx::Error::PoolTimedOut) => {
                    return Err(busy(started.elapsed(), BusyCause::PoolExhausted))
                }
                Err(err) => return Err(InventoryLockError::Db(err)),
            };

            let acquired: bool = sqlx::query_scalar("SELECT pg_try_advisory_xact_lock($1, $2)")
                .bind(class)
                .bind(obj)
                .fetch_one(&mut *tx)
                .await
                .map_err(InventoryLockError::Db)?;

            if acquired {
                return Ok(Self {
                    tx: Some(tx),
                    property: property.to_string(),
                    bypassed: false,
                });
            }

            // Give the connection back BEFORE sleeping — see the module doc on
            // why a waiter must not pin one.
            drop(tx);

            let waited = started.elapsed();
            if waited >= ACQUIRE_TIMEOUT {
                return Err(busy(waited, BusyCause::LockHeld));
            }
            tokio::time::sleep(backoff).await;
            backoff = std::cmp::min(backoff * 2, MAX_BACKOFF);
        }
    }

    /// The property this lock is scoped to.
    pub fn property(&self) -> &str {
        &self.property
    }

    /// `true` when the kill switch is off and this guard holds nothing.
    pub fn is_bypassed(&self) -> bool {
        self.bypassed
    }

    /// Release the lock now. Rolls back the guard's own transaction (which
    /// wrote nothing) — that is what drops the advisory lock. A no-op on a
    /// bypass guard.
    pub async fn release(mut self) -> Result<(), sqlx::Error> {
        match self.tx.take() {
            Some(tx) => tx.rollback().await,
            None => Ok(()),
        }
    }
}
