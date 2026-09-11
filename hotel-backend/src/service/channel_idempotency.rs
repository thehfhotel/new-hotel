//! Caller-side request idempotency for the loyalty-app booking channel —
//! the `Idempotency-Key` header contract on `POST /api/channel/bookings`.
//! See `docs/loyalty-channel.md` §Idempotency and migration 093.
//!
//! ## The problem this closes
//!
//! `ChannelService::create_hold` mints a booking, enqueues its byte-parity
//! legacy write-back and publishes a domain event in ONE PG transaction. The
//! write-back's own idempotency key is derived from the fresh `book_id`, so it
//! protects the PG→MSSQL leg only: a retried WORKER never double-writes. It
//! does nothing about a duplicate *create request*. A loyalty-app client whose
//! HTTP call hangs and is retried therefore gets TWO holds → two `ht_bookings`
//! rows → two real iHOTEL `จอง` bookings, one of which nobody releases before
//! its 2h deadline. The loyalty app works around that today with a 20 s Redis
//! lock — a timing heuristic, not a guarantee.
//!
//! ## The contract
//!
//! * The client sends `Idempotency-Key: <client-generated string>`.
//! * The handler calls [`ChannelIdempotency::reserve`]:
//!   - [`Reserved::Fresh`] — this caller owns the side effect. Do the work,
//!     then call [`Reservation::complete`] with the response it will send (or
//!     [`Reservation::abandon`] on failure).
//!   - [`Reserved::Replay`] — a previous request with this key already
//!     finished. Return its stored status + body verbatim, marked with the
//!     replay header.
//!   - [`Reserved::Mismatch`] — the key was already spent on a DIFFERENT
//!     request. Answer 422; never silently replay an unrelated booking.
//! * A request with NO key skips all of this and behaves exactly as before.
//!
//! ## Why a service helper and not middleware
//!
//! The stored response must commit with the work it describes, or a crash
//! between them leaves a cached response pointing at a hold that rolled back
//! (or, worse, a committed hold with no record of the key that made it). That
//! means the handler — which owns the transaction boundary — has to hold the
//! reservation. This is the same shape the loyalty app's own
//! `services/idempotency.rs` settled on, with two deliberate differences:
//!
//! 1. **The reserving transaction stays open across the work.** The loyalty
//!    app reserves and lets the conflict path re-read from the pool, which
//!    cannot see an in-flight winner and so can hand a caller a placeholder.
//!    Here the uncommitted unique-index entry makes a simultaneous duplicate
//!    BLOCK until the winner commits, and the loser then reads a COMPLETE row.
//!    That is what makes "two concurrent identical requests create one hold"
//!    true rather than likely.
//! 2. **The request is fingerprinted.** Re-using a key for a different request
//!    is a client bug; it is answered 422 rather than replaying a response
//!    that describes someone else's stay.
//!
//! ## Cost
//!
//! A keyed request holds one extra pooled connection for the duration of the
//! create (the reservation's transaction) on top of the connection the create
//! itself uses. The channel is a low-rate machine surface — one request per
//! guest booking — so this is bounded by the loyalty app's own concurrency,
//! not by guest traffic.

use sha2::{Digest, Sha256};
use sqlx::{PgPool, Postgres, Transaction};

use crate::repository::channel_idempotency as idem_repo;

use super::error::{ServiceError, ServiceResult};

/// Request header carrying the client-generated key. Lowercase because that is
/// how `http::HeaderMap` normalises names; the wire spelling documented for the
/// loyalty app is `Idempotency-Key` (HTTP header names are case-insensitive).
pub const IDEMPOTENCY_KEY_HEADER: &str = "idempotency-key";

/// Response header stamped on a replayed response so the client can tell a
/// cached answer from a fresh one. Value is always the string `"true"`.
pub const IDEMPOTENCY_REPLAYED_HEADER: &str = "idempotency-replayed";

/// How long a key is honoured. A retry after this behaves like a first-time
/// request — 24 h is far beyond any client retry window and far short of a
/// window in which an operator would be surprised to see a replay.
pub const IDEMPOTENCY_TTL_HOURS: i32 = 24;

/// Upper bound on the key length. 255 is the de-facto ceiling (Stripe's limit)
/// and comfortably fits a UUID, a ULID, or the loyalty app's own booking-intent
/// id.
pub const MAX_IDEMPOTENCY_KEY_LEN: usize = 255;

/// `idem_endpoint` label for `POST /api/channel/bookings`.
pub const ENDPOINT_CREATE_BOOKING: &str = "channel_create_booking";

/// How many expired rows the opportunistic TTL sweep clears per fresh
/// reservation. Bounded so the sweep can never become the slow part of a
/// booking; at channel volume it never has more than a handful to do.
const PURGE_BATCH: i64 = 200;

/// How many times [`ChannelIdempotency::reserve`] re-attempts when a row
/// vanishes between the failed INSERT and the follow-up read (only reachable
/// if the TTL sweep deletes a row in that microsecond gap).
const RESERVE_ATTEMPTS: usize = 3;

/// The response a previous request with this key produced.
#[derive(Debug, Clone)]
pub struct StoredResponse {
    /// HTTP status as it was sent (normally 201).
    pub status: u16,
    /// Response body as it was sent, verbatim.
    pub body: String,
    /// The booking that request created, when it created one.
    pub book_id: Option<i32>,
}

/// Outcome of [`ChannelIdempotency::reserve`].
pub enum Reserved {
    /// First time this key has been seen (or the previous one expired). The
    /// caller owns the side effect and MUST finish with
    /// [`Reservation::complete`] or [`Reservation::abandon`].
    Fresh(Reservation),
    /// A previous request with this key already completed — return this.
    Replay(StoredResponse),
    /// The key was spent on a materially different request. 422.
    Mismatch,
}

/// A held reservation: the open transaction whose uncommitted row is both the
/// record of this request and the lock that blocks a concurrent duplicate.
///
/// Dropping a `Reservation` without calling [`complete`](Self::complete)
/// rolls the transaction back (sqlx's `Transaction` drop behaviour), which
/// releases the key — the correct outcome for a request that failed, since an
/// error response is never cached. Prefer [`abandon`](Self::abandon) so the
/// rollback is awaited rather than deferred to a background task.
pub struct Reservation {
    tx: Transaction<'static, Postgres>,
    idem_id: i64,
}

impl Reservation {
    /// Store the response this request is about to send and commit.
    ///
    /// `body` is stored verbatim so a replay is byte-identical to what the
    /// original caller received.
    pub async fn complete(
        mut self,
        status: u16,
        body: &str,
        book_id: Option<i32>,
    ) -> ServiceResult<()> {
        let rows =
            idem_repo::complete(&mut self.tx, self.idem_id, status as i16, body, book_id).await?;
        if rows == 0 {
            // Unreachable: the row was inserted by this very transaction and
            // nothing else can see it, let alone delete it. Fail loudly rather
            // than commit a reservation with no response to replay.
            return Err(ServiceError::internal(
                "idempotency reservation vanished before its response could be recorded",
            ));
        }
        self.tx.commit().await?;
        Ok(())
    }

    /// Release the reservation without storing a response, so the client may
    /// retry the same key. Errors are logged, not surfaced: the request's own
    /// failure is what the caller is reporting.
    pub async fn abandon(self) {
        if let Err(err) = self.tx.rollback().await {
            tracing::warn!(
                error = %err,
                "failed to roll back a channel idempotency reservation; the key frees itself at TTL"
            );
        }
    }
}

/// Per-site handle (cheap: a pool clone), built by the route from the same
/// branch pool the create writes to — so the key, the booking and the outbox
/// row all live in one database.
#[derive(Clone)]
pub struct ChannelIdempotency {
    pg: PgPool,
}

impl ChannelIdempotency {
    pub fn new(pg: PgPool) -> Self {
        Self { pg }
    }

    /// Claim `key` for `caller`, or report what to do instead.
    ///
    /// `fingerprint` must be a stable hash of the CANONICALISED request — see
    /// [`fingerprint_of`]. It is compared, not stored-and-forgotten: the same
    /// key with a different request is [`Reserved::Mismatch`].
    pub async fn reserve(
        &self,
        caller: &str,
        key: &str,
        endpoint: &str,
        fingerprint: &str,
    ) -> ServiceResult<Reserved> {
        // Opportunistic TTL sweep, outside the reservation so its row locks are
        // never held across the create. Best-effort: a failed sweep must not
        // fail a booking — the rows it missed simply read as absent until the
        // next request clears them.
        match idem_repo::purge_expired(&self.pg, PURGE_BATCH).await {
            Ok(n) if n > 0 => tracing::debug!(purged = n, "swept expired channel idempotency keys"),
            Ok(_) => {}
            Err(err) => tracing::warn!(
                error = %err,
                "channel idempotency TTL sweep failed; continuing (expired rows read as absent)"
            ),
        }

        for _ in 0..RESERVE_ATTEMPTS {
            let mut tx = self.pg.begin().await?;

            // Winning this INSERT is the reservation. The transaction stays
            // open — and its uncommitted unique-index entry is what a
            // simultaneous duplicate blocks on — until complete()/abandon().
            if let Some(idem_id) = idem_repo::try_reserve(
                &mut tx,
                caller,
                key,
                endpoint,
                fingerprint,
                IDEMPOTENCY_TTL_HOURS,
            )
            .await?
            {
                return Ok(Reserved::Fresh(Reservation { tx, idem_id }));
            }

            // Lost (or arrived after) a live reservation. READ COMMITTED gives
            // this statement a fresh snapshot, so the winner's row — committed
            // while the INSERT above waited on it — is visible here.
            let existing = idem_repo::load(&mut *tx, caller, key).await?;
            // Nothing in this transaction is worth keeping either way.
            let _ = tx.rollback().await;

            let Some(record) = existing else {
                // The row expired and was swept between the INSERT and the
                // read. Re-attempt: the next INSERT takes the key cleanly.
                continue;
            };

            if record.fingerprint != fingerprint {
                return Ok(Reserved::Mismatch);
            }

            let (Some(status), Some(body)) = (record.status, record.body) else {
                // A committed row always carries its response (it is written in
                // the same transaction). Reaching here means someone wrote this
                // table by hand; refuse rather than guess.
                return Err(ServiceError::internal(
                    "channel idempotency row is committed without a stored response",
                ));
            };

            return Ok(Reserved::Replay(StoredResponse {
                status: status.max(0) as u16,
                body,
                book_id: record.book_id,
            }));
        }

        Err(ServiceError::internal(
            "could not settle the idempotency key after repeated attempts; retry the request",
        ))
    }
}

/// Validate and normalise a client-supplied key.
///
/// Trimmed; must be 1..=[`MAX_IDEMPOTENCY_KEY_LEN`] printable ASCII characters.
/// A present-but-blank or unprintable header is a client bug and is REFUSED
/// (400) rather than quietly treated as "no key" — silently dropping
/// idempotency protection is the failure mode this whole module exists to
/// remove.
pub fn normalize_key(raw: &str) -> ServiceResult<String> {
    let key = raw.trim();
    if key.is_empty() {
        return Err(ServiceError::validation(
            "Idempotency-Key must not be empty (omit the header entirely to opt out)",
        ));
    }
    if key.len() > MAX_IDEMPOTENCY_KEY_LEN {
        return Err(ServiceError::validation(format!(
            "Idempotency-Key must be at most {MAX_IDEMPOTENCY_KEY_LEN} characters"
        )));
    }
    if !key.chars().all(|c| c.is_ascii_graphic()) {
        return Err(ServiceError::validation(
            "Idempotency-Key must be printable ASCII without spaces (a UUID is ideal)",
        ));
    }
    Ok(key.to_string())
}

/// Derive the stored caller identity from the presented bearer.
///
/// The SHA-256 digest, never the token: the column has to be safe to read in a
/// psql session, a dump or a support ticket. Scoping keys per caller means two
/// clients cannot collide on a key like `"1"`, and rotating
/// `LOYALTY_CHANNEL_TOKEN` deliberately starts a fresh key space — a rotated
/// token is a different client.
///
/// `None` cannot occur behind `middleware::channel_token` (it rejects every
/// request without a valid bearer); the fallback label keeps the function total
/// so a future unauthenticated caller cannot be silently keyed as "everyone".
pub fn caller_identity(bearer: Option<&str>) -> String {
    match bearer {
        Some(token) => format!("sha256:{:x}", Sha256::digest(token.as_bytes())),
        None => "anonymous".to_string(),
    }
}

/// Hash a CANONICALISED request into the fingerprint stored with the key.
///
/// Callers pass already-normalised field values in a FIXED order, one per
/// element — not raw request bytes — so that a retry which re-serialises its
/// JSON with different whitespace or key order still matches. Parts are joined
/// with `\u{1f}` (ASCII unit separator), which cannot appear in a validated
/// field, so no part boundary can be forged by field content.
pub fn fingerprint_of(parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            hasher.update([0x1f]);
        }
        hasher.update(part.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_are_trimmed_and_validated() {
        assert_eq!(normalize_key(" abc-123 ").unwrap(), "abc-123");
        assert_eq!(
            normalize_key("6f1b0c3e-6b1a-4a5e-9a1e-1b2c3d4e5f60").unwrap(),
            "6f1b0c3e-6b1a-4a5e-9a1e-1b2c3d4e5f60"
        );
        // Blank is a client bug, not "no key".
        assert!(normalize_key("").is_err());
        assert!(normalize_key("   ").is_err());
        // Spaces / control chars / non-ASCII are refused.
        assert!(normalize_key("has space").is_err());
        assert!(normalize_key("tab\there").is_err());
        assert!(normalize_key("คีย์").is_err());
        // Length ceiling.
        assert!(normalize_key(&"a".repeat(MAX_IDEMPOTENCY_KEY_LEN)).is_ok());
        assert!(normalize_key(&"a".repeat(MAX_IDEMPOTENCY_KEY_LEN + 1)).is_err());
    }

    #[test]
    fn caller_identity_hashes_the_bearer() {
        let id = caller_identity(Some("super-secret-token"));
        assert!(id.starts_with("sha256:"), "{id}");
        assert_eq!(id.len(), "sha256:".len() + 64);
        assert!(
            !id.contains("super-secret-token"),
            "the raw token must never be stored"
        );
        // Different tokens ⇒ different key spaces; same token ⇒ stable.
        assert_ne!(id, caller_identity(Some("another-token")));
        assert_eq!(id, caller_identity(Some("super-secret-token")));
        assert_eq!(caller_identity(None), "anonymous");
    }

    #[test]
    fn fingerprint_is_stable_and_field_boundaries_cannot_be_forged() {
        let a = fingerprint_of(&["hf", "3", "2026-09-20", "2"]);
        assert_eq!(a, fingerprint_of(&["hf", "3", "2026-09-20", "2"]));
        assert_ne!(a, fingerprint_of(&["hf", "3", "2026-09-21", "2"]));
        // Concatenation ambiguity: ["ab","c"] must not hash like ["a","bc"].
        assert_ne!(fingerprint_of(&["ab", "c"]), fingerprint_of(&["a", "bc"]));
        // 64 hex characters, lowercase.
        assert_eq!(a.len(), 64);
        assert!(a
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()));
    }
}
