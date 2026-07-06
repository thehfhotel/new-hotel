//! NFC staff-card login — central resolve client + pending-login store +
//! badge → user provisioning.
//!
//! ## Flow (see `routes::reader` + `routes::auth::card_login`)
//!
//! ```text
//!   reader device ──POST /api/reader/scan (X-Reader-Secret)──▶ PMS
//!        PMS ──POST {HFID_RESOLVE_URL}/api/private/reader/resolve──▶ HF-ID
//!        PMS: authorize → find/provision ht_users by badge → stash pending
//!             login keyed by reader_id (one-time login_token, ~30s TTL)
//!   login screen ──POST /api/reader/claim {reader_id}──▶ Set-Cookie reader_claim
//!   login screen ──GET  /api/reader/wait (reader_claim cookie)──▶ {login_token}
//!   login screen ──POST /api/auth/card-login {login_token}──▶ Set-Cookie session
//! ```
//!
//! ## Composition / testability
//!
//! The central resolve is behind a [`ResolveClient`] trait so unit tests
//! inject a mock without the real HF-ID service. Production wires
//! [`HttpResolveClient`] (blocking `ureq` dispatched via
//! `spawn_blocking`, same policy as `notifications::slack` /
//! `middleware::cf_access` — no `reqwest`). When the central URL / secret
//! is unconfigured we wire [`NullResolveClient`] which always errors, so a
//! misconfigured deploy fails CLOSED (every scan rejected).
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
use serde::Deserialize;
use sqlx::{PgPool, Row};

use crate::config::ReaderConfig;

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

/// TTL for a reader_claim binding (browser ↔ reader_id). Generous — a
/// terminal parked on the login screen keeps its pairing across taps.
const CLAIM_TTL: Duration = Duration::from_secs(10 * 60);

// =============================================================================
// Central resolve client
// =============================================================================

/// Parsed response from `POST {HFID_RESOLVE_URL}/api/private/reader/resolve`.
///
/// `#[serde(default)]` on the non-`found` fields so a terse `{"found":false}`
/// negative response deserializes without the caller having to send every key.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ResolveResponse {
    pub found: bool,
    #[serde(default)]
    pub badge: Option<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub apps: Vec<String>,
    #[serde(default)]
    pub active: bool,
    #[serde(default)]
    pub pending: bool,
}

/// Error from a central resolve attempt. Opaque string — the scan route maps
/// any resolve failure to a single "not authorized" wire response so the
/// reader double-beeps without leaking the central service's failure mode.
#[derive(Debug, Clone)]
pub struct ResolveError(pub String);

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "resolve error: {}", self.0)
    }
}

/// Central card→employee resolver. Behind a trait so tests inject a mock.
#[async_trait]
pub trait ResolveClient: Send + Sync {
    async fn resolve(&self, uid: &str) -> Result<ResolveResponse, ResolveError>;
}

/// Production resolver: blocking `ureq` POST dispatched onto a blocking task
/// so the async runtime is never stalled (same pattern as
/// `notifications::slack`).
pub struct HttpResolveClient {
    agent: ureq::Agent,
    base_url: String,
    secret: String,
}

impl HttpResolveClient {
    pub fn new(base_url: String, secret: String) -> Self {
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(3))
            .timeout(Duration::from_secs(5))
            .build();
        Self {
            agent,
            base_url,
            secret,
        }
    }
}

#[async_trait]
impl ResolveClient for HttpResolveClient {
    async fn resolve(&self, uid: &str) -> Result<ResolveResponse, ResolveError> {
        let url = format!(
            "{}/api/private/reader/resolve",
            self.base_url.trim_end_matches('/')
        );
        let agent = self.agent.clone();
        let secret = self.secret.clone();
        let body = serde_json::json!({ "uid": uid });

        let joined = tokio::task::spawn_blocking(move || {
            agent
                .post(&url)
                .set("X-Reader-Secret", &secret)
                .send_json(body)
        })
        .await
        .map_err(|err| ResolveError(format!("join: {err}")))?;

        match joined {
            Ok(response) => response
                .into_json::<ResolveResponse>()
                .map_err(|err| ResolveError(format!("decode: {err}"))),
            Err(ureq::Error::Status(code, _)) => {
                Err(ResolveError(format!("central returned status {code}")))
            }
            Err(err) => Err(ResolveError(err.to_string())),
        }
    }
}

/// Fail-closed resolver used when `HFID_RESOLVE_URL` / `READER_RESOLVE_SECRET`
/// is unconfigured. Always errors so every scan is rejected rather than
/// silently authorizing against a blank secret.
pub struct NullResolveClient;

#[async_trait]
impl ResolveClient for NullResolveClient {
    async fn resolve(&self, _uid: &str) -> Result<ResolveResponse, ResolveError> {
        Err(ResolveError(
            "HF-ID resolve not configured (set HFID_RESOLVE_URL + READER_RESOLVE_SECRET)".into(),
        ))
    }
}

/// Authorization decision for a resolved card. Pure so it is unit-testable
/// without any I/O.
///
/// Authorized iff the badge was found, the employee is active, NOT pending
/// (staged-rollout gate), and the required app grant is present.
pub fn authorize(resp: &ResolveResponse, required_app: &str) -> bool {
    resp.found
        && resp.active
        && !resp.pending
        && resp.apps.iter().any(|app| app == required_app)
}

// =============================================================================
// Constant-time secret compare
// =============================================================================

/// Constant-time byte-slice equality for the reader secret compare. Length is
/// checked up front (leaking only the length of a shared secret, which is
/// acceptable); the byte loop then runs in time independent of WHERE the
/// mismatch is, so a timing side-channel can't be walked to recover the
/// secret one byte at a time.
pub fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
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
/// `wait` (keyed by reader_id) and `card-login` (keyed by login_token) — never
/// contend for the same map entry: `wait` MOVES the entry `pending → delivered`.
struct DeliveredLogin {
    user_id: i64,
    expires_at: Instant,
}

struct Claim {
    reader_id: String,
    expires_at: Instant,
}

#[derive(Default)]
struct ReaderStoreInner {
    /// reader_id → the one pending login for that reader (overwritten on each
    /// new tap). Moved to `delivered` when `wait` hands out its token.
    pending: HashMap<String, PendingLogin>,
    /// login_token → a delivered-but-unconsumed login. `card-login` consumes
    /// from here (one-time). Keyed by token because that is all card-login has.
    delivered: HashMap<String, DeliveredLogin>,
    /// claim_token → the reader_id a browser paired to.
    claims: HashMap<String, Claim>,
}

/// Cloneable handle to the process-local reader state. Wrapped in
/// `Arc<RwLock<_>>` so a single instance shared via `AppState` is seen by
/// every concurrent request (same style as `middleware::rate_limit`).
///
/// Single-process only — fine while the backend runs as one container. Taps
/// and the paired login screen always hit the same process.
#[derive(Clone, Default)]
pub struct ReaderStore {
    inner: Arc<RwLock<ReaderStoreInner>>,
}

impl ReaderStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Stash a one-time pending login for `reader_id`, returning the fresh
    /// `login_token`. Overwrites any existing pending login for that reader.
    pub fn put_pending(&self, reader_id: &str, user_id: i64) -> String {
        let login_token = random_token();
        let mut guard = self.inner.write().expect("reader store poisoned");
        prune_pending(&mut guard.pending);
        guard.pending.insert(
            reader_id.to_string(),
            PendingLogin {
                user_id,
                login_token: login_token.clone(),
                expires_at: Instant::now() + PENDING_TTL,
            },
        );
        login_token
    }

    /// Deliver-once from `pending`: remove the pending login for `reader_id`
    /// (if present and unexpired), MOVE it into `delivered` keyed by its
    /// login_token, and return that token. Used by `GET /api/reader/wait`.
    ///
    /// Moving (rather than plain-removing) is what lets `card-login` later
    /// resolve the same token to a user via [`take_user_for_login_token`] — the
    /// two consume points read different maps, so `wait` and `card-login` never
    /// race for one entry. The delivered copy carries a fresh `PENDING_TTL` so
    /// the browser's immediate follow-up `card-login` always finds it.
    pub fn take_pending_login_token(&self, reader_id: &str) -> Option<String> {
        let mut guard = self.inner.write().expect("reader store poisoned");
        let now = Instant::now();
        let pending = match guard.pending.remove(reader_id) {
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

    /// Bind a browser to `reader_id`, returning the fresh `claim_token` to set
    /// as the `reader_claim` cookie. Used by `POST /api/reader/claim`.
    pub fn put_claim(&self, reader_id: &str) -> String {
        let claim_token = random_token();
        let mut guard = self.inner.write().expect("reader store poisoned");
        prune_claims(&mut guard.claims);
        guard.claims.insert(
            claim_token.clone(),
            Claim {
                reader_id: reader_id.to_string(),
                expires_at: Instant::now() + CLAIM_TTL,
            },
        );
        claim_token
    }

    /// Resolve a `reader_claim` cookie value back to its reader_id (if the
    /// claim exists and has not expired). Read-only — the claim persists for
    /// its whole TTL so the terminal can wait across multiple taps.
    pub fn resolve_claim(&self, claim_token: &str) -> Option<String> {
        let guard = self.inner.read().expect("reader store poisoned");
        match guard.claims.get(claim_token) {
            Some(c) if c.expires_at > Instant::now() => Some(c.reader_id.clone()),
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

/// Cheap-to-clone bundle of the reader feature's runtime state: the pending
/// store, the resolve client, and the two config knobs the scan route needs.
#[derive(Clone)]
pub struct ReaderState {
    pub store: ReaderStore,
    pub resolve: Arc<dyn ResolveClient>,
    /// Device ↔ PMS secret (`READER_SECRET`). `None` ⇒ scan fails closed.
    pub reader_secret: Option<String>,
    /// App grant a badge must carry to be authorized (`READER_REQUIRED_APP`).
    pub required_app: String,
}

impl ReaderState {
    /// Build from a [`ReaderConfig`]. Wires [`HttpResolveClient`] only when
    /// BOTH the central URL and its secret are present; otherwise
    /// [`NullResolveClient`] (fail closed).
    pub fn from_config(cfg: ReaderConfig) -> Self {
        let resolve: Arc<dyn ResolveClient> = match (cfg.resolve_url, cfg.resolve_secret) {
            (Some(url), Some(secret)) => Arc::new(HttpResolveClient::new(url, secret)),
            _ => {
                tracing::info!(
                    "reader: HF-ID resolve not configured — card scans will be rejected \
                     (set HFID_RESOLVE_URL + READER_RESOLVE_SECRET to enable)"
                );
                Arc::new(NullResolveClient)
            }
        };
        Self {
            store: ReaderStore::new(),
            resolve,
            reader_secret: cfg.reader_secret,
            required_app: cfg.required_app,
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

    fn resp(found: bool, active: bool, pending: bool, apps: &[&str]) -> ResolveResponse {
        ResolveResponse {
            found,
            badge: Some("B123".into()),
            display_name: Some("Nok".into()),
            apps: apps.iter().map(|s| s.to_string()).collect(),
            active,
            pending,
        }
    }

    #[test]
    fn authorize_true_only_for_found_active_nonpending_with_app() {
        assert!(authorize(&resp(true, true, false, &["hotel"]), "hotel"));
    }

    #[test]
    fn authorize_rejects_missing_app_grant() {
        assert!(!authorize(&resp(true, true, false, &["payroll"]), "hotel"));
        assert!(!authorize(&resp(true, true, false, &[]), "hotel"));
    }

    #[test]
    fn authorize_rejects_not_found_inactive_or_pending() {
        assert!(!authorize(&resp(false, true, false, &["hotel"]), "hotel"));
        assert!(!authorize(&resp(true, false, false, &["hotel"]), "hotel"));
        assert!(!authorize(&resp(true, true, true, &["hotel"]), "hotel"));
    }

    #[test]
    fn resolve_response_deserializes_terse_negative() {
        // A `{"found":false}` body must decode without the other keys.
        let r: ResolveResponse = serde_json::from_str(r#"{"found":false}"#).unwrap();
        assert!(!r.found);
        assert!(!r.active);
        assert!(!r.pending);
        assert!(r.apps.is_empty());
        assert!(r.badge.is_none());
    }

    #[test]
    fn resolve_response_deserializes_full_positive() {
        let r: ResolveResponse = serde_json::from_str(
            r#"{"found":true,"badge":"B7","display_name":"Nok","apps":["hotel","payroll"],"active":true,"pending":false}"#,
        )
        .unwrap();
        assert!(authorize(&r, "hotel"));
        assert_eq!(r.badge.as_deref(), Some("B7"));
        assert_eq!(r.display_name.as_deref(), Some("Nok"));
    }

    #[test]
    fn ct_eq_matches_only_identical_slices() {
        assert!(ct_eq(b"super-secret", b"super-secret"));
        assert!(!ct_eq(b"super-secret", b"super-secreT"));
        assert!(!ct_eq(b"short", b"longer-value"));
        assert!(ct_eq(b"", b""));
    }

    #[test]
    fn random_token_is_64_hex_chars_and_unique() {
        let a = random_token();
        let b = random_token();
        assert_eq!(a.len(), 64);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
        assert_ne!(a, b, "tokens must not collide");
    }

    #[test]
    fn pending_login_delivers_once_by_reader_id() {
        let store = ReaderStore::new();
        let token = store.put_pending("reader-1", 42);
        assert_eq!(token.len(), 64);
        // First take delivers, second is empty (deliver-once).
        assert_eq!(store.take_pending_login_token("reader-1"), Some(token));
        assert_eq!(store.take_pending_login_token("reader-1"), None);
    }

    #[test]
    fn pending_login_overwrites_previous_for_same_reader() {
        let store = ReaderStore::new();
        let first = store.put_pending("reader-1", 1);
        let second = store.put_pending("reader-1", 2);
        assert_ne!(first, second);
        // The overwritten first token was never delivered → not redeemable.
        assert_eq!(store.take_user_for_login_token(&first), None);
        // Deliver the surviving pending, then redeem it → user 2.
        assert_eq!(
            store.take_pending_login_token("reader-1").as_deref(),
            Some(second.as_str())
        );
        assert_eq!(store.take_user_for_login_token(&second), Some(2));
    }

    #[test]
    fn take_user_for_login_token_is_one_time() {
        let store = ReaderStore::new();
        let token = store.put_pending("reader-9", 7);
        // wait delivers the token (pending → delivered); card-login consumes it.
        assert_eq!(
            store.take_pending_login_token("reader-9").as_deref(),
            Some(token.as_str())
        );
        assert_eq!(store.take_user_for_login_token(&token), Some(7));
        // Second use rejected — the token was consumed.
        assert_eq!(store.take_user_for_login_token(&token), None);
        // And the reader-id path is also empty now.
        assert_eq!(store.take_pending_login_token("reader-9"), None);
    }

    #[test]
    fn scan_wait_card_login_round_trip() {
        // The real end-to-end sequence, which the earlier single-map store
        // silently broke: scan stashes a pending login, wait delivers the token
        // to the paired browser, card-login redeems it. Regression guard —
        // wait and card-login must NOT contend for the same entry.
        let store = ReaderStore::new();
        store.put_pending("frontdesk-1", 99); // POST /api/reader/scan
        let token = store
            .take_pending_login_token("frontdesk-1") // GET /api/reader/wait
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
    fn claim_round_trips_reader_id() {
        let store = ReaderStore::new();
        let claim = store.put_claim("reader-42");
        assert_eq!(store.resolve_claim(&claim).as_deref(), Some("reader-42"));
        // Unknown claim resolves to None.
        assert_eq!(store.resolve_claim("bogus"), None);
    }

    #[test]
    fn null_resolve_client_always_errors() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let client = NullResolveClient;
        let result = rt.block_on(client.resolve("uid-123"));
        assert!(result.is_err());
    }

    #[test]
    fn reader_state_without_config_uses_null_resolver_and_fails_closed() {
        // No URL/secret → NullResolveClient → every resolve errors → the scan
        // route rejects (fail closed). Also reader_secret stays None → the
        // secret gate rejects too.
        let state = ReaderState::from_config(ReaderConfig {
            resolve_url: None,
            resolve_secret: None,
            reader_secret: None,
            required_app: "hotel".into(),
        });
        assert!(state.reader_secret.is_none());
        let rt = tokio::runtime::Runtime::new().unwrap();
        assert!(rt.block_on(state.resolve.resolve("x")).is_err());
    }
}
