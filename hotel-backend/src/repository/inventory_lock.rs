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
//! This module is the serializer: every writer that CONSUMES inventory takes
//! the same advisory lock for the whole pick → insert span, so the second
//! writer re-evaluates availability against the first writer's COMMITTED
//! state instead of against the snapshot it read before the race started.
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
    #[error(transparent)]
    Db(#[from] sqlx::Error),

    /// Nobody released the lock inside [`ACQUIRE_TIMEOUT`]. Callers surface
    /// this as a retryable conflict, never as a successful booking.
    #[error(
        "timed out after {waited:?} waiting for the booking-inventory lock of property \
         '{property}'; another booking write is still in progress"
    )]
    Timeout { property: String, waited: Duration },
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
pub struct InventoryLock {
    /// `None` only after [`release`](Self::release) has consumed it.
    tx: Option<Transaction<'static, Postgres>>,
    property: String,
}

impl std::fmt::Debug for InventoryLock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (class, obj) = lock_key(&self.property);
        f.debug_struct("InventoryLock")
            .field("property", &self.property)
            .field("key", &(class, obj))
            .field("held", &self.tx.is_some())
            .finish()
    }
}

impl InventoryLock {
    /// Take the booking-inventory lock for `property`, waiting up to
    /// [`ACQUIRE_TIMEOUT`].
    pub async fn acquire(pool: &PgPool, property: &str) -> Result<Self, InventoryLockError> {
        let (class, obj) = lock_key(property);
        let started = Instant::now();
        let mut backoff = FIRST_BACKOFF;

        loop {
            let mut tx = pool.begin().await?;
            let acquired: bool = sqlx::query_scalar("SELECT pg_try_advisory_xact_lock($1, $2)")
                .bind(class)
                .bind(obj)
                .fetch_one(&mut *tx)
                .await?;

            if acquired {
                return Ok(Self {
                    tx: Some(tx),
                    property: property.to_string(),
                });
            }

            // Give the connection back BEFORE sleeping — see the module doc on
            // why a waiter must not pin one.
            drop(tx);

            let waited = started.elapsed();
            if waited >= ACQUIRE_TIMEOUT {
                return Err(InventoryLockError::Timeout {
                    property: property.to_string(),
                    waited,
                });
            }
            tokio::time::sleep(backoff).await;
            backoff = std::cmp::min(backoff * 2, MAX_BACKOFF);
        }
    }

    /// The property this lock is scoped to.
    pub fn property(&self) -> &str {
        &self.property
    }

    /// Release the lock now. Rolls back the guard's own transaction (which
    /// wrote nothing) — that is what drops the advisory lock.
    pub async fn release(mut self) -> Result<(), sqlx::Error> {
        match self.tx.take() {
            Some(tx) => tx.rollback().await,
            None => Ok(()),
        }
    }
}
