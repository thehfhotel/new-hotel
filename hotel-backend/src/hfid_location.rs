//! HF ID badge → employee-location lookup (owner-directed `/hk` location
//! enforcement, wave-4 housekeeping C).
//!
//! ## Why this exists
//!
//! `GET /api/hk/me` used to hand EVERY maid the same global `HK_BRANCHES`
//! allowlist, and `require_branch` checked only that allowlist. So an HF Ville
//! maid was offered "ฮาร์เบอร์ฟร้อนท์" in the picker and could file a cleaning
//! report — and, for `done`, a real `MarkRoomClean` writeback — against the
//! WRONG PROPERTY. The allowlist answers "which properties does this
//! deployment serve", never "which property does THIS employee work at".
//!
//! HF ID already holds the authoritative answer in `Employee.location`. It is
//! in no OIDC claim, and adding a Cloudflare Access claim is impractical (the
//! IdP `claims` config is set at IdP-create time with no update path), so the
//! backend asks HF ID over the LAN instead, server-to-server, with the same
//! `X-Reader-Secret` shared secret the card-login pairing calls already use
//! (`service::reader`). See `~/HF/fingerprint-time-logger` `app/api/reader.py`
//! → `POST /resolve-badge`.
//!
//! ## Placement
//!
//! A crate-root module, next to [`crate::secrets`], NOT under `service/`. It
//! holds no business rule and touches no database or transaction — it is an
//! outbound adapter that turns a badge into an answer. The policy decisions
//! built ON that answer (which branch, which status code, which Thai message)
//! live in `routes::hk`, which is also where the single
//! [`EmployeeLocation`] → `Branch` mapping lives.
//!
//! ## Fail-closed contract
//!
//! [`LocationOutcome`] has exactly three shapes and the caller must never
//! collapse them into "assume HF Hotel":
//!
//! - [`LocationOutcome::Resolved`] — HF ID answered, the employee is active,
//!   and `location` parsed. The ONLY shape that may admit a request.
//! - [`LocationOutcome::NoLocation`] — HF ID answered DEFINITIVELY, but there
//!   is no usable branch: `location` is null, `found` is false, or the
//!   employee is inactive / still `pending` approval. Retrying will not help;
//!   the owner must set the employee's location (or approve them).
//! - [`LocationOutcome::Unavailable`] — no answer was obtained: unconfigured
//!   URL/secret, transport failure, non-2xx, an undecodable body, or a
//!   `location` string this build does not recognise. Retrying MIGHT help.
//!
//! `location` is consumed VERBATIM, null included. Coercing null to a default
//! would silently reintroduce the exact wrong-property bug this module closes,
//! one layer down and much harder to see.
//!
//! ## Caching
//!
//! POSITIVE ONLY, TTL [`LOCATION_CACHE_TTL`], keyed by badge. A resolved
//! location is stable (an employee does not change property mid-shift) so a
//! 60s window removes a per-request LAN round-trip from every room list. A
//! miss, a null location, an inactive employee or ANY error is NEVER cached:
//! the moment the owner fixes a missing location, or the LAN recovers, the
//! next tap must see it. Caching a negative would turn a 5-second fix into a
//! 60-second one and, worse, make an outage look sticky.
//!
//! ## Transport
//!
//! Blocking `ureq` dispatched via `spawn_blocking` — the repo's outbound-HTTP
//! policy (no `reqwest`), same as `service::reader` / `middleware::cf_access` /
//! `notifications::slack`. The timeout is SHORT ([`LOCATION_LOOKUP_TIMEOUT`]):
//! this call sits in the request path of every `/hk` read, so a hung HF ID
//! must become a fast, actionable 503 rather than a page that spins.
//!
//! The endpoint is consumed as a FULL URL from `HFID_LOCATION_URL`
//! ([`crate::config::HfidLocationConfig`]) rather than base-URL + a hardcoded
//! path, so the peer service can settle its final path with zero code change
//! here. It must be a plain LAN address (`http://192.168.100.228:5000/…`) —
//! never routed through Cloudflare.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use serde::Deserialize;

use crate::config::HfidLocationConfig;

/// How long a POSITIVE badge → location answer is reused. See the module
/// note on why nothing else is cached.
pub const LOCATION_CACHE_TTL: Duration = Duration::from_secs(60);

/// Upper bound on one HF ID lookup. Deliberately short — this call is in the
/// request path of every `/hk` read.
pub const LOCATION_LOOKUP_TIMEOUT: Duration = Duration::from_secs(3);

/// Connect budget inside [`LOCATION_LOOKUP_TIMEOUT`]. A host that is simply
/// down (LAN address, nothing listening) fails here almost immediately.
const LOCATION_CONNECT_TIMEOUT: Duration = Duration::from_secs(2);

/// The shared-secret header HF ID's app↔central surface expects. Same header
/// and same secret VALUE as the card-login pairing calls in `service::reader`
/// — only the new-hotel env var name differs (`HFID_RESOLVE_SECRET`, see
/// `CLAUDE.md`).
pub const RESOLVE_SECRET_HEADER: &str = "X-Reader-Secret";

// ============================================================================
// The answer
// ============================================================================

/// An employee's branch as HF ID spells it. The wire values are HF ID's
/// `Employee.location` verbatim; the mapping onto our `Branch` lives in
/// `routes::hk` (one place, next to the `?branch=` spellings).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmployeeLocation {
    /// `"HF"` — The Harbour Front Hotel.
    Hf,
    /// `"HF_VILLE"` — HF Ville.
    HfVille,
}

impl EmployeeLocation {
    /// Parse HF ID's `location` string. PURE — unit-tested below.
    ///
    /// EXACT, uppercase, whitespace-trimmed only. Deliberately not
    /// case-folded and deliberately not prefix-matched: an unrecognised value
    /// must surface as `None` (⇒ [`LocationOutcome::Unavailable`]) rather than
    /// be fuzzily coerced onto a property, because a wrong branch here is the
    /// exact bug this module exists to prevent.
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim() {
            "HF" => Some(Self::Hf),
            "HF_VILLE" => Some(Self::HfVille),
            _ => None,
        }
    }

    /// The HF ID wire spelling — for logs and round-trip tests.
    pub fn as_hfid_str(self) -> &'static str {
        match self {
            Self::Hf => "HF",
            Self::HfVille => "HF_VILLE",
        }
    }
}

/// The three shapes a lookup can produce. See the module-level fail-closed
/// contract — the caller must keep all three distinct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationOutcome {
    /// HF ID answered: employee found, active, `location` recognised.
    Resolved(EmployeeLocation),
    /// HF ID answered definitively, but there is no usable branch (null
    /// `location`, `found=false`, or an inactive employee). Not retryable.
    NoLocation,
    /// No answer was obtained (unconfigured, transport, non-2xx, undecodable,
    /// or an unrecognised `location` value). Possibly retryable.
    Unavailable,
}

// ============================================================================
// Transport
// ============================================================================

/// Badge → location lookup. Behind a trait so the enforcement tests drive
/// every matrix row without an HF ID service (the same composition idiom as
/// `service::reader::HfIdClient`).
#[async_trait]
pub trait LocationLookup: Send + Sync {
    /// Never returns an `Err`: every failure mode is already a
    /// [`LocationOutcome`] the caller must handle, and collapsing them into a
    /// `Result` invites a `?` that would drop the NoLocation/Unavailable
    /// distinction on the floor.
    async fn lookup(&self, badge: &str) -> LocationOutcome;
}

/// What we read out of HF ID's `/resolve-badge` answer.
///
/// The live payload is `{found, badge, display_name, apps, active, pending,
/// location}` (fingerprint-time-logger `5b45a235`). We consume only the four
/// fields that bear on admission; the rest are ignored by serde, so the peer
/// can extend its payload freely.
///
/// Every field defaults to the FAIL-CLOSED value: a body missing
/// `found`/`active` reads as not-found / inactive, never as an admitted
/// employee. `pending` is the one inversion — its fail-closed default is
/// `false`, because HF ID omitting it must not lock out every employee; a
/// TRUE value is what refuses.
#[derive(Debug, Deserialize)]
struct ResolveBadgeResponse {
    #[serde(default)]
    found: bool,
    #[serde(default)]
    active: bool,
    /// `pending_approval` — an onboarded-but-unapproved employee. Refused on
    /// the same footing as inactive: HF ID has not yet vouched for them, so
    /// their `location` is not an authority to file cleaning reports against a
    /// property.
    #[serde(default)]
    pending: bool,
    #[serde(default)]
    location: Option<String>,
}

impl ResolveBadgeResponse {
    /// Interpret a decoded body. PURE — unit-tested below, which is where the
    /// null/inactive/pending/miss ⇒ `NoLocation` and unknown-value ⇒
    /// `Unavailable` rules are actually pinned.
    fn outcome(&self) -> LocationOutcome {
        if !self.found || !self.active || self.pending {
            return LocationOutcome::NoLocation;
        }
        let Some(raw) = self.location.as_deref() else {
            return LocationOutcome::NoLocation;
        };
        match EmployeeLocation::parse(raw) {
            Some(location) => LocationOutcome::Resolved(location),
            None => {
                // HF ID knows a property this build does not. That is a
                // deploy-skew fault, not an unset field, so it must NOT read
                // as "the owner forgot to set a location" — and it must never
                // be guessed onto a branch.
                tracing::error!(
                    location = raw,
                    "HF ID returned an unrecognised employee location — \
                     /hk cannot map it to a branch (expected HF | HF_VILLE)"
                );
                LocationOutcome::Unavailable
            }
        }
    }
}

/// Production lookup: `POST {HFID_LOCATION_URL}` with `{"badge": "…"}` and the
/// shared secret header, mirroring HF ID's `/resolve-badge` contract.
pub struct HttpLocationLookup {
    agent: ureq::Agent,
    url: String,
    secret: String,
}

impl HttpLocationLookup {
    pub fn new(url: String, secret: String) -> Self {
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(LOCATION_CONNECT_TIMEOUT)
            .timeout(LOCATION_LOOKUP_TIMEOUT)
            .build();
        Self { agent, url, secret }
    }
}

#[async_trait]
impl LocationLookup for HttpLocationLookup {
    async fn lookup(&self, badge: &str) -> LocationOutcome {
        let url = self.url.clone();
        let agent = self.agent.clone();
        let secret = self.secret.clone();
        let body = serde_json::json!({ "badge": badge });

        let joined = tokio::task::spawn_blocking(move || {
            agent
                .post(&url)
                .set(RESOLVE_SECRET_HEADER, &secret)
                .send_json(body)
        })
        .await;

        let sent = match joined {
            Ok(sent) => sent,
            Err(err) => {
                tracing::warn!(error = %err, "HF ID location lookup task failed to join");
                return LocationOutcome::Unavailable;
            }
        };

        match sent {
            Ok(response) => match response.into_json::<ResolveBadgeResponse>() {
                Ok(decoded) => decoded.outcome(),
                Err(err) => {
                    tracing::warn!(error = %err, "HF ID location lookup returned an undecodable body");
                    LocationOutcome::Unavailable
                }
            },
            // 404 = HF ID's own dark-until-configured posture (its
            // READER_RESOLVE_SECRET is unset); 401 = our secret disagrees with
            // its. Both are "no answer", never "no location".
            Err(ureq::Error::Status(code, _)) => {
                tracing::warn!(status = code, "HF ID location lookup returned a non-2xx status");
                LocationOutcome::Unavailable
            }
            Err(err) => {
                tracing::warn!(error = %err, "HF ID location lookup transport error");
                LocationOutcome::Unavailable
            }
        }
    }
}

// ============================================================================
// Positive-only cache
// ============================================================================

struct CacheEntry {
    location: EmployeeLocation,
    expires_at: Instant,
}

/// Process-local badge → location cache. POSITIVE ENTRIES ONLY — see the
/// module note. `Arc<RwLock<_>>` so one instance shared through `HkPolicy` is
/// seen by every concurrent request (same style as
/// `service::reader::ReaderStore`).
#[derive(Clone)]
pub struct LocationCache {
    ttl: Duration,
    inner: Arc<RwLock<HashMap<String, CacheEntry>>>,
}

impl Default for LocationCache {
    fn default() -> Self {
        Self::with_ttl(LOCATION_CACHE_TTL)
    }
}

impl LocationCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// A cache with an explicit TTL. `pub` for the tests, which cannot wait
    /// 60 real seconds to prove expiry.
    pub fn with_ttl(ttl: Duration) -> Self {
        Self {
            ttl,
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// The cached location for `badge`, or `None` when absent or expired.
    pub fn get(&self, badge: &str) -> Option<EmployeeLocation> {
        let guard = self.inner.read().expect("location cache poisoned");
        match guard.get(badge) {
            Some(entry) if entry.expires_at > Instant::now() => Some(entry.location),
            _ => None,
        }
    }

    /// Cache a RESOLVED location. There is deliberately no `put` for the
    /// other two outcomes — the type system, not a comment, is what keeps a
    /// miss or an outage out of the cache.
    pub fn put(&self, badge: &str, location: EmployeeLocation) {
        let now = Instant::now();
        let mut guard = self.inner.write().expect("location cache poisoned");
        guard.retain(|_, entry| entry.expires_at > now);
        guard.insert(
            badge.to_string(),
            CacheEntry {
                location,
                expires_at: now + self.ttl,
            },
        );
    }

    /// Live entry count (post-prune). Test/observability helper.
    pub fn len(&self) -> usize {
        let now = Instant::now();
        let guard = self.inner.read().expect("location cache poisoned");
        guard.values().filter(|e| e.expires_at > now).count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// ============================================================================
// The client `routes::hk` holds
// ============================================================================

/// Cheap-to-clone HF ID location client: a [`LocationLookup`] plus its
/// positive-only [`LocationCache`].
///
/// Held by `routes::hk::HkPolicy` as an `Option` — `None` means
/// `HFID_LOCATION_URL` / `HFID_RESOLVE_SECRET` are unset, which with
/// enforcement ON is `lookup_unavailable`, never a fallback to the allowlist.
#[derive(Clone)]
pub struct HfidLocationClient {
    lookup: Arc<dyn LocationLookup>,
    cache: LocationCache,
}

impl std::fmt::Debug for HfidLocationClient {
    /// Hand-written: `HkPolicy` derives `Debug` and is logged at startup, and
    /// neither the shared secret nor the URL may ride along into a log line.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HfidLocationClient")
            .field("cached_badges", &self.cache.len())
            .finish_non_exhaustive()
    }
}

impl HfidLocationClient {
    /// Wire the production HTTP lookup. Returns `None` when either half of
    /// the configuration is missing — there is no partially-configured client,
    /// because a URL with no secret would just collect 401s and a secret with
    /// no URL has nowhere to go.
    pub fn from_config(cfg: HfidLocationConfig) -> Option<Self> {
        let (Some(url), Some(secret)) = (cfg.url, cfg.resolve_secret) else {
            return None;
        };
        Some(Self::with_lookup(Arc::new(HttpLocationLookup::new(
            url, secret,
        ))))
    }

    /// Build around an arbitrary lookup — the seam the enforcement tests use.
    pub fn with_lookup(lookup: Arc<dyn LocationLookup>) -> Self {
        Self {
            lookup,
            cache: LocationCache::new(),
        }
    }

    /// [`with_lookup`](Self::with_lookup) with an explicit cache TTL, so the
    /// expiry test does not have to sleep for a minute.
    pub fn with_lookup_and_ttl(lookup: Arc<dyn LocationLookup>, ttl: Duration) -> Self {
        Self {
            lookup,
            cache: LocationCache::with_ttl(ttl),
        }
    }

    /// Read-only handle on the cache (tests / observability).
    pub fn cache(&self) -> &LocationCache {
        &self.cache
    }

    /// Resolve a badge, consulting then populating the positive-only cache.
    ///
    /// The cache is written on [`LocationOutcome::Resolved`] and ONLY then —
    /// that single `if let` is the whole positive-only policy.
    pub async fn resolve(&self, badge: &str) -> LocationOutcome {
        if let Some(location) = self.cache.get(badge) {
            return LocationOutcome::Resolved(location);
        }
        let outcome = self.lookup.lookup(badge).await;
        if let LocationOutcome::Resolved(location) = outcome {
            self.cache.put(badge, location);
        }
        outcome
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// A scripted lookup that counts calls — the cache proofs need both.
    struct ScriptedLookup {
        outcome: LocationOutcome,
        calls: AtomicUsize,
    }

    impl ScriptedLookup {
        fn new(outcome: LocationOutcome) -> Arc<Self> {
            Arc::new(Self {
                outcome,
                calls: AtomicUsize::new(0),
            })
        }
        fn calls(&self) -> usize {
            self.calls.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl LocationLookup for ScriptedLookup {
        async fn lookup(&self, _badge: &str) -> LocationOutcome {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.outcome
        }
    }

    // ---- location parsing ----------------------------------------------

    #[test]
    fn location_parses_only_the_two_exact_hfid_spellings() {
        assert_eq!(EmployeeLocation::parse("HF"), Some(EmployeeLocation::Hf));
        assert_eq!(
            EmployeeLocation::parse("HF_VILLE"),
            Some(EmployeeLocation::HfVille)
        );
        // Surrounding whitespace is forgiven (it is never part of the value).
        assert_eq!(
            EmployeeLocation::parse(" HF_VILLE "),
            Some(EmployeeLocation::HfVille)
        );
        // Everything else is None — case variants included. Folding case here
        // would be a guess, and a guessed branch is the bug this closes.
        for bad in ["", "  ", "hf", "hf_ville", "HFVILLE", "HF-VILLE", "HF_VILLE_2", "ALL"] {
            assert_eq!(EmployeeLocation::parse(bad), None, "{bad:?} must not parse");
        }
    }

    #[test]
    fn location_round_trips_its_wire_spelling() {
        for location in [EmployeeLocation::Hf, EmployeeLocation::HfVille] {
            assert_eq!(
                EmployeeLocation::parse(location.as_hfid_str()),
                Some(location)
            );
        }
    }

    // ---- response interpretation ---------------------------------------

    fn decode(json: &str) -> LocationOutcome {
        serde_json::from_str::<ResolveBadgeResponse>(json)
            .expect("body decodes")
            .outcome()
    }

    #[test]
    fn resolve_badge_body_maps_to_the_three_outcomes() {
        // The happy pair.
        assert_eq!(
            decode(r#"{"found":true,"active":true,"location":"HF"}"#),
            LocationOutcome::Resolved(EmployeeLocation::Hf)
        );
        assert_eq!(
            decode(r#"{"found":true,"active":true,"location":"HF_VILLE"}"#),
            LocationOutcome::Resolved(EmployeeLocation::HfVille)
        );

        // Definite "no branch for you" — all three sub-cases. NEVER a default
        // branch, and (per the cache contract) never cached.
        assert_eq!(
            decode(r#"{"found":true,"active":true,"location":null}"#),
            LocationOutcome::NoLocation,
            "a null location is UNKNOWN, never a default"
        );
        assert_eq!(
            decode(r#"{"found":false,"active":false,"location":null}"#),
            LocationOutcome::NoLocation,
            "a badge HF ID does not know"
        );
        assert_eq!(
            decode(r#"{"found":true,"active":false,"location":"HF"}"#),
            LocationOutcome::NoLocation,
            "an INACTIVE employee is refused even with a location on file"
        );
        assert_eq!(
            decode(r#"{"found":true,"active":true,"pending":true,"location":"HF"}"#),
            LocationOutcome::NoLocation,
            "a PENDING-approval employee is refused even with a location on file"
        );

        // A location this build cannot map is deploy skew, not an unset field.
        assert_eq!(
            decode(r#"{"found":true,"active":true,"location":"HF_THIRD_PROPERTY"}"#),
            LocationOutcome::Unavailable
        );
    }

    /// Extra peer fields must not break decoding, and MISSING fields must read
    /// fail-closed rather than as an admitted employee.
    #[test]
    fn resolve_badge_body_is_forward_compatible_and_fails_closed() {
        // The VERBATIM live payload shape (fingerprint-time-logger 5b45a235).
        assert_eq!(
            decode(
                r#"{"found":true,"badge":"421","display_name":"นก","apps":["hotel"],
                    "active":true,"pending":false,"location":"HF"}"#
            ),
            LocationOutcome::Resolved(EmployeeLocation::Hf),
            "the live payload's extra fields are ignored, not fatal"
        );
        assert_eq!(
            decode(r#"{"location":"HF"}"#),
            LocationOutcome::NoLocation,
            "absent found/active default to the fail-closed value"
        );
        assert_eq!(decode("{}"), LocationOutcome::NoLocation);
        // …but an omitted `pending` must NOT read as pending, or a peer that
        // stops sending the field would lock out every employee at once.
        assert_eq!(
            decode(r#"{"found":true,"active":true,"location":"HF_VILLE"}"#),
            LocationOutcome::Resolved(EmployeeLocation::HfVille),
            "an omitted `pending` is not a refusal"
        );
    }

    // ---- cache ---------------------------------------------------------

    #[tokio::test]
    async fn resolved_lookups_are_cached_and_not_re_asked() {
        let lookup = ScriptedLookup::new(LocationOutcome::Resolved(EmployeeLocation::HfVille));
        let client = HfidLocationClient::with_lookup(lookup.clone());

        for _ in 0..5 {
            assert_eq!(
                client.resolve("421").await,
                LocationOutcome::Resolved(EmployeeLocation::HfVille)
            );
        }
        assert_eq!(lookup.calls(), 1, "one LAN round-trip, then the cache");
        assert_eq!(client.cache().len(), 1);
    }

    /// The load-bearing half: a miss, a null location, an inactive employee or
    /// an outage must NEVER be cached. Caching a negative would turn the
    /// owner's five-second fix (set badge 421's location) into a 60-second one
    /// and make a transient LAN blip look sticky.
    #[tokio::test]
    async fn negative_and_error_outcomes_are_never_cached() {
        for outcome in [LocationOutcome::NoLocation, LocationOutcome::Unavailable] {
            let lookup = ScriptedLookup::new(outcome);
            let client = HfidLocationClient::with_lookup(lookup.clone());

            for _ in 0..3 {
                assert_eq!(client.resolve("421").await, outcome);
            }
            assert_eq!(
                lookup.calls(),
                3,
                "{outcome:?} must be re-asked every time, not cached"
            );
            assert!(
                client.cache().is_empty(),
                "{outcome:?} must leave the cache empty"
            );
        }
    }

    /// A cached positive expires: after the TTL the next call re-asks HF ID,
    /// so a genuine location change is picked up within one TTL window.
    #[tokio::test]
    async fn cached_locations_expire_after_the_ttl() {
        let lookup = ScriptedLookup::new(LocationOutcome::Resolved(EmployeeLocation::Hf));
        let client =
            HfidLocationClient::with_lookup_and_ttl(lookup.clone(), Duration::from_millis(40));

        assert_eq!(
            client.resolve("421").await,
            LocationOutcome::Resolved(EmployeeLocation::Hf)
        );
        assert_eq!(client.resolve("421").await, LocationOutcome::Resolved(EmployeeLocation::Hf));
        assert_eq!(lookup.calls(), 1, "second call served from cache");

        tokio::time::sleep(Duration::from_millis(60)).await;
        assert!(client.cache().get("421").is_none(), "entry has expired");
        assert_eq!(
            client.resolve("421").await,
            LocationOutcome::Resolved(EmployeeLocation::Hf)
        );
        assert_eq!(lookup.calls(), 2, "post-expiry call re-asks HF ID");
    }

    /// Two employees at different properties must not share a cache slot —
    /// the key is the badge, and a collision here would be the wrong-property
    /// bug wearing a cache hat.
    #[tokio::test]
    async fn cache_is_keyed_per_badge() {
        struct PerBadge;
        #[async_trait]
        impl LocationLookup for PerBadge {
            async fn lookup(&self, badge: &str) -> LocationOutcome {
                match badge {
                    "421" => LocationOutcome::Resolved(EmployeeLocation::HfVille),
                    _ => LocationOutcome::Resolved(EmployeeLocation::Hf),
                }
            }
        }
        let client = HfidLocationClient::with_lookup(Arc::new(PerBadge));
        assert_eq!(
            client.resolve("421").await,
            LocationOutcome::Resolved(EmployeeLocation::HfVille)
        );
        assert_eq!(
            client.resolve("Q1001").await,
            LocationOutcome::Resolved(EmployeeLocation::Hf)
        );
        // …and the cached reads stay distinct.
        assert_eq!(client.cache().get("421"), Some(EmployeeLocation::HfVille));
        assert_eq!(client.cache().get("Q1001"), Some(EmployeeLocation::Hf));
        assert_eq!(client.cache().len(), 2);
    }

    // ---- configuration -------------------------------------------------

    /// No partially-configured client: BOTH the URL and the secret are needed,
    /// so a half-set deploy is `None` ⇒ `lookup_unavailable`, never a silent
    /// fallback to the allowlist.
    #[test]
    fn client_requires_both_url_and_secret() {
        assert!(HfidLocationClient::from_config(HfidLocationConfig {
            url: None,
            resolve_secret: None,
        })
        .is_none());
        assert!(HfidLocationClient::from_config(HfidLocationConfig {
            url: Some("http://192.168.100.228:5000/api/private/reader/resolve-badge".into()),
            resolve_secret: None,
        })
        .is_none());
        assert!(HfidLocationClient::from_config(HfidLocationConfig {
            url: None,
            resolve_secret: Some("s".into()),
        })
        .is_none());
        assert!(HfidLocationClient::from_config(HfidLocationConfig {
            url: Some("http://192.168.100.228:5000/api/private/reader/resolve-badge".into()),
            resolve_secret: Some("s".into()),
        })
        .is_some());
    }

    /// `HkPolicy` derives `Debug` and is logged at startup — the client's
    /// `Debug` must never carry the secret or the URL into a log line.
    #[test]
    fn debug_leaks_neither_secret_nor_url() {
        let client = HfidLocationClient::from_config(HfidLocationConfig {
            url: Some("http://192.168.100.228:5000/api/private/reader/resolve-badge".into()),
            resolve_secret: Some("super-secret-value".into()),
        })
        .expect("configured");
        let rendered = format!("{client:?}");
        assert!(!rendered.contains("super-secret-value"), "{rendered}");
        assert!(!rendered.contains("192.168.100.228"), "{rendered}");
    }
}
