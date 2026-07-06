//! NFC staff-card login — central HF-ID pairing client + pending-login store +
//! badge → user provisioning.
//!
//! ## Flow (see `routes::reader` + `routes::auth::card_login`)
//!
//! The physical reader now posts taps to the CENTRAL HF-ID service, NOT to this
//! PMS. This PMS consumes HF-ID's pairing endpoints on behalf of the login
//! screen:
//!
//! ```text
//!   reader device ──tap──▶ HF-ID (central)            (no longer hits the PMS)
//!   login screen ──POST /api/reader/claim {reader_id}──▶ PMS
//!        PMS ──POST {HFID_BASE_URL}/api/private/reader/claim {reader_id,app}──▶ HF-ID
//!        PMS: Set-Cookie reader_claim (maps this browser → central claim_token)
//!   login screen ──GET  /api/reader/wait (reader_claim cookie)──▶ PMS
//!        PMS ──POST {HFID_BASE_URL}/api/private/reader/wait {claim_token}──▶ HF-ID
//!        HF-ID → 200 {assertion} | 403 not_authorized | 204 timeout
//!        PMS (on 200): verify the RS256 assertion (middleware::hfid_assertion)
//!             → sub=badge → find/provision ht_users → stash pending→delivered
//!               login → return {login_token}
//!   login screen ──POST /api/auth/card-login {login_token}──▶ Set-Cookie session
//! ```
//!
//! ## Composition / testability
//!
//! The central pairing is behind an [`HfIdClient`] trait so unit tests inject a
//! mock without the real HF-ID service. Production wires [`HttpHfIdClient`]
//! (blocking `ureq` dispatched via `spawn_blocking`, same policy as
//! `notifications::slack` / `middleware::cf_access` — no `reqwest`). When the
//! PMS↔central secret is unconfigured we wire [`NullHfIdClient`] which always
//! errors, so a misconfigured deploy fails CLOSED (every claim/wait rejected).
//!
//! ## Why dynamic `sqlx::query()`
//!
//! [`find_or_provision_user_by_badge`] uses runtime-checked `sqlx::query()`
//! (not the compile-time `query!` macro) — same rationale as
//! `repository::user`: the `badge` column is new and this keeps the offline
//! `.sqlx/` cache out of the loop, so there is no stale-cache CI failure.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use rand::RngCore;
use sqlx::{PgPool, Row};

use crate::config::ReaderConfig;
use crate::middleware::hfid_assertion::HFID_APP;

/// Password-hash sentinel stored on auto-provisioned card-only accounts.
///
/// NOT a valid Argon2 PHC string (leading `!`), so
/// `AuthService::verify_password` parses it as invalid and returns
/// `Ok(false)` — the account can never succeed a password login. Card login
/// bypasses password verification entirely (it loads the user by id after a
/// resolved tap), so this row stays login-able only via a tap.
pub const CARD_ONLY_PASSWORD_SENTINEL: &str = "!card-only-no-password";

/// TTL for a stashed pending-login. Short: the login screen is already
/// long-polling `/api/reader/wait` when the tap lands, so 30s is ample and
/// keeps a stray login_token from lingering.
const PENDING_TTL: Duration = Duration::from_secs(30);

/// TTL for a reader_claim binding (browser ↔ central claim_token). Generous — a
/// terminal parked on the login screen keeps its pairing across taps.
const CLAIM_TTL: Duration = Duration::from_secs(10 * 60);

// =============================================================================
// Central HF-ID pairing client
// =============================================================================

/// Outcome of a `POST {HFID_BASE_URL}/api/private/reader/wait` call.
#[derive(Debug, Clone)]
pub enum WaitOutcome {
    /// HF-ID delivered a signed RS256 assertion for a tapped, authorized badge
    /// (central **200**). The inner string is the raw assertion (id_token),
    /// still to be verified by `middleware::hfid_assertion`.
    Authorized(String),
    /// A tap landed but the badge is not authorized for this app (central
    /// **403** `{"error":"not_authorized"}`).
    NotAuthorized,
    /// No tap within the central long-poll budget (central **204**). The login
    /// screen re-polls.
    Timeout,
}

/// Error from a central HF-ID claim/wait attempt. Opaque string — the routes
/// map any transport/decoding failure to a single wire code so a browser never
/// learns the central service's failure mode.
#[derive(Debug, Clone)]
pub struct HfIdError(pub String);

impl std::fmt::Display for HfIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hf-id error: {}", self.0)
    }
}

/// Central HF-ID pairing client. Behind a trait so tests inject a mock.
#[async_trait]
pub trait HfIdClient: Send + Sync {
    /// Claim a pairing for `reader_id` (app = `hotel`). Returns the central
    /// `claim_token` this browser will long-poll against.
    async fn claim(&self, reader_id: &str) -> Result<String, HfIdError>;

    /// Long-poll a pairing for the next tap. See [`WaitOutcome`].
    async fn wait(&self, claim_token: &str) -> Result<WaitOutcome, HfIdError>;
}

/// Production client: blocking `ureq` dispatched onto a blocking task so the
/// async runtime is never stalled (same pattern as `middleware::cf_access`).
///
/// The agent timeout is generous (`WAIT_TIMEOUT`) because the central `wait`
/// call long-polls — it holds the connection until a tap lands or its own
/// budget elapses (then returns 204). `claim` returns promptly and is bounded
/// by the same ceiling.
pub struct HttpHfIdClient {
    agent: ureq::Agent,
    base_url: String,
    secret: String,
}

/// Upper bound on a single central call. Must exceed the central `wait`
/// long-poll budget so our read doesn't abort mid-poll (the central side
/// returns 204 well before this).
const WAIT_TIMEOUT: Duration = Duration::from_secs(35);

impl HttpHfIdClient {
    pub fn new(base_url: String, secret: String) -> Self {
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(3))
            .timeout(WAIT_TIMEOUT)
            .build();
        Self {
            agent,
            base_url,
            secret,
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url.trim_end_matches('/'), path)
    }
}

/// `{ "claim_token": "..." }` — the central claim response.
#[derive(serde::Deserialize)]
struct ClaimResponse {
    claim_token: String,
}

/// `{ "assertion": "<jwt>" }` — the central wait **200** response.
#[derive(serde::Deserialize)]
struct WaitAssertion {
    assertion: String,
}

#[async_trait]
impl HfIdClient for HttpHfIdClient {
    async fn claim(&self, reader_id: &str) -> Result<String, HfIdError> {
        let url = self.url("/api/private/reader/claim");
        let agent = self.agent.clone();
        let secret = self.secret.clone();
        let body = serde_json::json!({ "reader_id": reader_id, "app": HFID_APP });

        let joined = tokio::task::spawn_blocking(move || {
            agent
                .post(&url)
                .set("X-Reader-Secret", &secret)
                .send_json(body)
        })
        .await
        .map_err(|err| HfIdError(format!("join: {err}")))?;

        match joined {
            Ok(response) => response
                .into_json::<ClaimResponse>()
                .map(|c| c.claim_token)
                .map_err(|err| HfIdError(format!("decode: {err}"))),
            Err(ureq::Error::Status(code, _)) => {
                Err(HfIdError(format!("central claim status {code}")))
            }
            Err(err) => Err(HfIdError(err.to_string())),
        }
    }

    async fn wait(&self, claim_token: &str) -> Result<WaitOutcome, HfIdError> {
        let url = self.url("/api/private/reader/wait");
        let agent = self.agent.clone();
        let secret = self.secret.clone();
        let body = serde_json::json!({ "claim_token": claim_token });

        let joined = tokio::task::spawn_blocking(move || {
            agent
                .post(&url)
                .set("X-Reader-Secret", &secret)
                .send_json(body)
        })
        .await
        .map_err(|err| HfIdError(format!("join: {err}")))?;

        match joined {
            Ok(response) => {
                // 2xx: 204 = long-poll timeout (no tap), 200 = assertion body.
                if response.status() == 204 {
                    Ok(WaitOutcome::Timeout)
                } else {
                    response
                        .into_json::<WaitAssertion>()
                        .map(|w| WaitOutcome::Authorized(w.assertion))
                        .map_err(|err| HfIdError(format!("decode: {err}")))
                }
            }
            // 403 = a tap landed but the badge is not authorized for `hotel`.
            Err(ureq::Error::Status(403, _)) => Ok(WaitOutcome::NotAuthorized),
            Err(ureq::Error::Status(code, _)) => {
                Err(HfIdError(format!("central wait status {code}")))
            }
            Err(err) => Err(HfIdError(err.to_string())),
        }
    }
}

/// Fail-closed client used when `READER_RESOLVE_SECRET` is unconfigured. Always
/// errors so every claim/wait is rejected rather than talking to central with a
/// blank secret.
pub struct NullHfIdClient;

#[async_trait]
impl HfIdClient for NullHfIdClient {
    async fn claim(&self, _reader_id: &str) -> Result<String, HfIdError> {
        Err(HfIdError(
            "HF-ID pairing not configured (set HFID_BASE_URL + READER_RESOLVE_SECRET)".into(),
        ))
    }

    async fn wait(&self, _claim_token: &str) -> Result<WaitOutcome, HfIdError> {
        Err(HfIdError(
            "HF-ID pairing not configured (set HFID_BASE_URL + READER_RESOLVE_SECRET)".into(),
        ))
    }
}

// =============================================================================
// In-memory pending-login + claim store
// =============================================================================

struct PendingLogin {
    user_id: i64,
    login_token: String,
    expires_at: Instant,
}

/// A login_token that `wait` has handed to a paired browser but `card-login`
/// has not yet consumed. Split out from `pending` so the two consume points —
/// `wait` (keyed by the pairing key) and `card-login` (keyed by login_token) —
/// never contend for the same map entry: `wait` MOVES the entry
/// `pending → delivered`.
struct DeliveredLogin {
    user_id: i64,
    expires_at: Instant,
}

/// A browser ↔ central-pairing binding. The `reader_claim` cookie holds a local
/// opaque token that maps HERE to the `central_claim_token` the PMS long-polls
/// against — so the central token never leaves the server.
struct Claim {
    central_claim_token: String,
    expires_at: Instant,
}

#[derive(Default)]
struct ReaderStoreInner {
    /// pairing key → the one pending login for that pairing (overwritten on
    /// each new tap). Moved to `delivered` when `wait` hands out its token.
    pending: HashMap<String, PendingLogin>,
    /// login_token → a delivered-but-unconsumed login. `card-login` consumes
    /// from here (one-time). Keyed by token because that is all card-login has.
    delivered: HashMap<String, DeliveredLogin>,
    /// local cookie claim_token → the central claim_token a browser paired to.
    claims: HashMap<String, Claim>,
}

/// Cloneable handle to the process-local reader state. Wrapped in
/// `Arc<RwLock<_>>` so a single instance shared via `AppState` is seen by
/// every concurrent request (same style as `middleware::rate_limit`).
///
/// Single-process only — fine while the backend runs as one container. The
/// paired login screen always hits the same process across its claim/wait/
/// card-login sequence.
#[derive(Clone, Default)]
pub struct ReaderStore {
    inner: Arc<RwLock<ReaderStoreInner>>,
}

impl ReaderStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Stash a one-time pending login under `key`, returning the fresh
    /// `login_token`. Overwrites any existing pending login for that key.
    ///
    /// In the central-pairing flow `GET /api/reader/wait` calls this with the
    /// pairing's central claim_token as the key, then immediately
    /// [`take_pending_login_token`](Self::take_pending_login_token) to move it
    /// into `delivered` — the two-step keeps the `card-login` consume point
    /// reading a separate map.
    pub fn put_pending(&self, key: &str, user_id: i64) -> String {
        let login_token = random_token();
        let mut guard = self.inner.write().expect("reader store poisoned");
        prune_pending(&mut guard.pending);
        guard.pending.insert(
            key.to_string(),
            PendingLogin {
                user_id,
                login_token: login_token.clone(),
                expires_at: Instant::now() + PENDING_TTL,
            },
        );
        login_token
    }

    /// Deliver-once from `pending`: remove the pending login for `key` (if
    /// present and unexpired), MOVE it into `delivered` keyed by its
    /// login_token, and return that token. Used by `GET /api/reader/wait`.
    ///
    /// Moving (rather than plain-removing) is what lets `card-login` later
    /// resolve the same token to a user via [`take_user_for_login_token`](Self::take_user_for_login_token)
    /// — the two consume points read different maps, so `wait` and `card-login`
    /// never race for one entry. The delivered copy carries a fresh
    /// `PENDING_TTL` so the browser's immediate follow-up `card-login` always
    /// finds it.
    pub fn take_pending_login_token(&self, key: &str) -> Option<String> {
        let mut guard = self.inner.write().expect("reader store poisoned");
        let now = Instant::now();
        let pending = match guard.pending.remove(key) {
            Some(p) if p.expires_at > now => p,
            _ => return None,
        };
        prune_delivered(&mut guard.delivered);
        guard.delivered.insert(
            pending.login_token.clone(),
            DeliveredLogin {
                user_id: pending.user_id,
                expires_at: now + PENDING_TTL,
            },
        );
        Some(pending.login_token)
    }

    /// Consume-once: remove and return the `user_id` behind a *delivered*
    /// `login_token` (if present and unexpired). Used by
    /// `POST /api/auth/card-login`. Only tokens already handed out by `wait`
    /// live in `delivered`, so a pending entry cannot be redeemed without
    /// first being delivered — this is the single-session guarantee.
    pub fn take_user_for_login_token(&self, login_token: &str) -> Option<i64> {
        let mut guard = self.inner.write().expect("reader store poisoned");
        match guard.delivered.remove(login_token) {
            Some(d) if d.expires_at > Instant::now() => Some(d.user_id),
            _ => None,
        }
    }

    /// Bind a browser to a `central_claim_token`, returning the fresh LOCAL
    /// claim_token to set as the `reader_claim` cookie. Used by
    /// `POST /api/reader/claim`. Keeping the central token server-side (the
    /// browser only holds the opaque local handle) is a small defence-in-depth
    /// layer — a stolen cookie can't be replayed straight against HF-ID.
    pub fn put_claim(&self, central_claim_token: &str) -> String {
        let claim_token = random_token();
        let mut guard = self.inner.write().expect("reader store poisoned");
        prune_claims(&mut guard.claims);
        guard.claims.insert(
            claim_token.clone(),
            Claim {
                central_claim_token: central_claim_token.to_string(),
                expires_at: Instant::now() + CLAIM_TTL,
            },
        );
        claim_token
    }

    /// Resolve a `reader_claim` cookie value back to its central claim_token (if
    /// the claim exists and has not expired). Read-only — the claim persists for
    /// its whole TTL so the terminal can wait across multiple taps.
    pub fn resolve_claim(&self, claim_token: &str) -> Option<String> {
        let guard = self.inner.read().expect("reader store poisoned");
        match guard.claims.get(claim_token) {
            Some(c) if c.expires_at > Instant::now() => Some(c.central_claim_token.clone()),
            _ => None,
        }
    }
}

fn prune_pending(pending: &mut HashMap<String, PendingLogin>) {
    let now = Instant::now();
    pending.retain(|_, p| p.expires_at > now);
}

fn prune_delivered(delivered: &mut HashMap<String, DeliveredLogin>) {
    let now = Instant::now();
    delivered.retain(|_, d| d.expires_at > now);
}

fn prune_claims(claims: &mut HashMap<String, Claim>) {
    let now = Instant::now();
    claims.retain(|_, c| c.expires_at > now);
}

/// 32 random bytes from the OS RNG, hex-encoded → 64 ASCII chars. Same shape
/// and entropy as `service::auth::generate_session_id`.
fn random_token() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

// =============================================================================
// Reader state carried by AppState
// =============================================================================

/// Cheap-to-clone bundle of the reader feature's runtime state: the pending /
/// claim store, the central HF-ID pairing client, and the HF-ID base URL (used
/// to fetch the JWKS when verifying an assertion).
#[derive(Clone)]
pub struct ReaderState {
    pub store: ReaderStore,
    pub hfid: Arc<dyn HfIdClient>,
    /// Central HF-ID base URL (`HFID_BASE_URL`). Drives both the claim/wait
    /// endpoints (inside `hfid`) and the JWKS endpoint the `wait` route passes
    /// to `middleware::hfid_assertion::verify_hfid_assertion`.
    pub base_url: String,
}

impl ReaderState {
    /// Build from a [`ReaderConfig`]. Wires [`HttpHfIdClient`] only when the
    /// PMS↔central secret is present; otherwise [`NullHfIdClient`] (fail
    /// closed). The base URL always has a value (config default).
    pub fn from_config(cfg: ReaderConfig) -> Self {
        let hfid: Arc<dyn HfIdClient> = match cfg.resolve_secret {
            Some(secret) => Arc::new(HttpHfIdClient::new(cfg.base_url.clone(), secret)),
            None => {
                tracing::info!(
                    "reader: HF-ID pairing not configured — card taps will be rejected \
                     (set READER_RESOLVE_SECRET, and HFID_BASE_URL if not the default, to enable)"
                );
                Arc::new(NullHfIdClient)
            }
        };
        Self {
            store: ReaderStore::new(),
            hfid,
            base_url: cfg.base_url,
        }
    }

    pub fn from_env() -> Self {
        Self::from_config(ReaderConfig::from_env())
    }
}

// =============================================================================
// Badge → user resolution / auto-provisioning
// =============================================================================

/// Resolve `badge` to a `ht_users.user_id`, auto-provisioning a card-only
/// receptionist account (+ the `ht_user_roles` junction row so permissions
/// resolve) on first sight of an authorized badge.
///
/// Idempotent + race-safe: a concurrent first tap of the same new badge that
/// loses the `ux_ht_users_badge` unique race re-selects the winner's row
/// instead of erroring. `display_name` is stored only on INSERT (an existing
/// row keeps its current name).
pub async fn find_or_provision_user_by_badge(
    pool: &PgPool,
    badge: &str,
    display_name: Option<&str>,
) -> Result<i64, sqlx::Error> {
    if let Some(id) = select_user_id_by_badge(pool, badge).await? {
        return Ok(id);
    }

    // Provision: user row + receptionist junction row in ONE transaction so a
    // half-provisioned account (user without a role) is impossible.
    let mut tx = pool.begin().await?;
    let username = format!("card-{badge}");
    let insert = sqlx::query(
        r#"
        INSERT INTO ht_users (username, password_hash, role, active, display_name, badge)
        VALUES ($1, $2, 'receptionist', TRUE, $3, $4)
        RETURNING user_id
        "#,
    )
    .bind(&username)
    .bind(CARD_ONLY_PASSWORD_SENTINEL)
    .bind(display_name)
    .bind(badge)
    .fetch_one(&mut *tx)
    .await;

    let user_id: i64 = match insert {
        Ok(row) => row.try_get("user_id")?,
        Err(err) => {
            // Lost the race (badge OR username unique violation): another tap
            // provisioned this badge first. Roll back and hand back the winner.
            drop(tx);
            if is_unique_violation(&err) {
                if let Some(id) = select_user_id_by_badge(pool, badge).await? {
                    return Ok(id);
                }
            }
            return Err(err);
        }
    };

    sqlx::query(
        r#"
        INSERT INTO ht_user_roles (user_id, role_id)
        SELECT $1, role_id FROM ht_roles WHERE role_key = 'receptionist'
        ON CONFLICT (user_id, role_id) DO NOTHING
        "#,
    )
    .bind(user_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(user_id)
}

async fn select_user_id_by_badge(pool: &PgPool, badge: &str) -> Result<Option<i64>, sqlx::Error> {
    let row = sqlx::query("SELECT user_id FROM ht_users WHERE badge = $1")
        .bind(badge)
        .fetch_optional(pool)
        .await?;
    row.map(|r| r.try_get::<i64, _>("user_id")).transpose()
}

/// PG unique-violation SQLSTATE detector (mirrors `bin/create_user`).
fn is_unique_violation(err: &sqlx::Error) -> bool {
    matches!(err, sqlx::Error::Database(db) if db.code().as_deref() == Some("23505"))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_token_is_64_hex_chars_and_unique() {
        let a = random_token();
        let b = random_token();
        assert_eq!(a.len(), 64);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
        assert_ne!(a, b, "tokens must not collide");
    }

    #[test]
    fn pending_login_delivers_once_by_key() {
        let store = ReaderStore::new();
        let token = store.put_pending("pairing-1", 42);
        assert_eq!(token.len(), 64);
        // First take delivers, second is empty (deliver-once).
        assert_eq!(store.take_pending_login_token("pairing-1"), Some(token));
        assert_eq!(store.take_pending_login_token("pairing-1"), None);
    }

    #[test]
    fn pending_login_overwrites_previous_for_same_key() {
        let store = ReaderStore::new();
        let first = store.put_pending("pairing-1", 1);
        let second = store.put_pending("pairing-1", 2);
        assert_ne!(first, second);
        // The overwritten first token was never delivered → not redeemable.
        assert_eq!(store.take_user_for_login_token(&first), None);
        // Deliver the surviving pending, then redeem it → user 2.
        assert_eq!(
            store.take_pending_login_token("pairing-1").as_deref(),
            Some(second.as_str())
        );
        assert_eq!(store.take_user_for_login_token(&second), Some(2));
    }

    #[test]
    fn take_user_for_login_token_is_one_time() {
        let store = ReaderStore::new();
        let token = store.put_pending("pairing-9", 7);
        // wait delivers the token (pending → delivered); card-login consumes it.
        assert_eq!(
            store.take_pending_login_token("pairing-9").as_deref(),
            Some(token.as_str())
        );
        assert_eq!(store.take_user_for_login_token(&token), Some(7));
        // Second use rejected — the token was consumed.
        assert_eq!(store.take_user_for_login_token(&token), None);
        // And the pairing-key path is also empty now.
        assert_eq!(store.take_pending_login_token("pairing-9"), None);
    }

    #[test]
    fn wait_card_login_round_trip() {
        // The real central-pairing sequence: `wait` stashes a pending login and
        // moves it to delivered, `card-login` redeems it. Regression guard —
        // wait and card-login must NOT contend for the same entry.
        let store = ReaderStore::new();
        store.put_pending("central-tok", 99); // wait: stash after verifying the assertion
        let token = store
            .take_pending_login_token("central-tok") // wait: deliver to the paired browser
            .expect("wait must deliver the freshly-stashed login token");
        assert_eq!(
            store.take_user_for_login_token(&token), // POST /api/auth/card-login
            Some(99),
            "card-login must resolve the delivered token to the tapped user"
        );
        // One-time: a replayed login_token is rejected.
        assert_eq!(store.take_user_for_login_token(&token), None);
    }

    #[test]
    fn unknown_login_token_resolves_to_none() {
        let store = ReaderStore::new();
        assert_eq!(store.take_user_for_login_token("no-such-token"), None);
    }

    #[test]
    fn claim_round_trips_central_claim_token() {
        let store = ReaderStore::new();
        // put_claim stores the CENTRAL claim_token and returns a LOCAL cookie
        // token; resolve_claim maps the cookie back to the central token.
        let cookie = store.put_claim("central-claim-abc");
        assert_eq!(cookie.len(), 64);
        assert_ne!(cookie, "central-claim-abc", "cookie must be a local handle");
        assert_eq!(
            store.resolve_claim(&cookie).as_deref(),
            Some("central-claim-abc")
        );
        // Unknown cookie resolves to None.
        assert_eq!(store.resolve_claim("bogus"), None);
    }

    #[test]
    fn null_hfid_client_always_errors() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let client = NullHfIdClient;
        assert!(rt.block_on(client.claim("reader-1")).is_err());
        assert!(rt.block_on(client.wait("claim-1")).is_err());
    }

    #[test]
    fn reader_state_without_secret_uses_null_client_and_fails_closed() {
        // No secret → NullHfIdClient → every claim/wait errors (fail closed),
        // even though base_url carries the config default.
        let state = ReaderState::from_config(ReaderConfig {
            base_url: "http://192.168.1.250".into(),
            resolve_secret: None,
        });
        assert_eq!(state.base_url, "http://192.168.1.250");
        let rt = tokio::runtime::Runtime::new().unwrap();
        assert!(rt.block_on(state.hfid.claim("r")).is_err());
        assert!(rt.block_on(state.hfid.wait("c")).is_err());
    }
}
