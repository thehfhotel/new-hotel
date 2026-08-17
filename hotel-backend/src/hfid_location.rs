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
//! ## The `housekeeping_admin` grant — the ONE way to be more than one place
//!
//! Some employees genuinely work both properties, and the owner needs a way to
//! say so. That way is an HF ID GRANT ([`HK_ADMIN_GRANT`]), read from the same
//! `/resolve-badge` answer's `apps` list — the full grant list the badge holds,
//! which HF ID already returns and this module previously ignored.
//!
//! A badge holding it resolves to [`LocationOutcome::AnyLocation`]: not bound
//! to one property. That is a FOURTH shape, deliberately not a "null location
//! means everywhere" rule — the distinction is the entire point:
//!
//! - a null `location` is an ABSENCE. Nobody decided anything; it is the owner
//!   not having filled a field in yet. It stays [`LocationOutcome::NoLocation`]
//!   and still refuses, exactly as before.
//! - the grant is a DECISION. Someone ticked a box against a named employee in
//!   HF ID's Employee Management, and that tick is auditable there.
//!
//! So the grant overrides `location` ENTIRELY — including a null one, and
//! including one this build does not recognise (a grant-holder must not be
//! locked out by deploy skew). It is checked BEFORE `location` is even looked
//! at, which is why holding it makes the field irrelevant rather than optional.
//!
//! What it does NOT override is `found` / `active` / `pending`: those are HF ID
//! vouching for the PERSON, not placing them at a property. A grant on an
//! inactive or unapproved employee still refuses, and the check order below is
//! what enforces that. Widening a location scope and admitting an employee HF
//! ID has disowned are different decisions, and only the first one was made.
//!
//! The branch half of the rule lives in `routes::hk` (as always): "any
//! location" ∩ `HK_BRANCHES` = the whole allowlist, which keeps the DEPLOYMENT
//! allowlist an outer bound the grant cannot cross.
//!
//! **This grant WIDENS; it never ADMITS.** The grant that opens the surface at
//! all is `middleware::hk_access::HK_GRANT` (`"housekeeping"`), checked at the
//! Cloudflare edge policy and re-checked behind it. `housekeeping_admin` on
//! its own reaches nothing — that badge is refused 403 before any lookup is
//! issued. So the pairing is: `housekeeping` lets you in, `housekeeping_admin`
//! decides how many properties you see once you are. Note the two keys share a
//! PREFIX; both comparisons are exact equality, and both are test-pinned
//! (`hk_access::admin_grant_alone_does_not_open_the_surface`).
//!
//! ## Caching
//!
//! POSITIVE ONLY, TTL [`LOCATION_CACHE_TTL`], keyed by badge. A resolved
//! answer is stable (an employee does not change property mid-shift) so a
//! 60s window removes a per-request LAN round-trip from every room list. A
//! miss, a null location, an inactive employee or ANY error is NEVER cached:
//! the moment the owner fixes a missing location, or the LAN recovers, the
//! next tap must see it. Caching a negative would turn a 5-second fix into a
//! 60-second one and, worse, make an outage look sticky.
//!
//! What is cached is the whole positive answer — [`LocationAdmission`], which
//! carries the grant fact ALONGSIDE the location rather than the location
//! alone. Both halves therefore expire on the same clock, so granting or
//! revoking `housekeeping_admin` propagates within one TTL exactly like moving
//! an employee between properties does. Caching only the location would have
//! made a revoked grant survive until the entry aged out for an unrelated
//! reason — a stale WIDENING, which is the one direction that must not linger.
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

/// The HF ID grant that frees an employee from single-location binding — an
/// entry in `/resolve-badge`'s `apps` list, managed per-employee in HF ID's
/// Employee Management (the same catalog as
/// [`crate::middleware::hk_access::HK_GRANT`], a DIFFERENT key and a different
/// transport: that one arrives in the Access token and opens the surface at
/// all, this one arrives in the lookup body and widens which branches the
/// surface offers).
///
/// Matched EXACTLY, like every other identifier crossing this boundary. No
/// prefix match and no case folding: an `apps` entry is a catalog key, not
/// prose, and a fuzzy match here would hand someone both properties.
pub const HK_ADMIN_GRANT: &str = "housekeeping_admin";

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

/// The four shapes a lookup can produce. See the module-level fail-closed
/// contract — the caller must keep all four distinct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationOutcome {
    /// HF ID answered: employee found, active, `location` recognised.
    Resolved(EmployeeLocation),
    /// HF ID answered: employee found, active, and holding [`HK_ADMIN_GRANT`]
    /// — NOT bound to a single property. `location` is irrelevant here (it may
    /// be set, null, or a value this build cannot map) because the grant is an
    /// explicit, auditable decision and the field is not.
    ///
    /// This is a widening, so it is deliberately the only variant that admits
    /// more than one branch — and it still cannot cross `HK_BRANCHES`, which
    /// `routes::hk` intersects it against.
    AnyLocation,
    /// HF ID answered definitively, but there is no usable branch (null
    /// `location`, `found=false`, or an inactive / pending-approval employee).
    /// Not retryable.
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
/// location}` (fingerprint-time-logger `5b45a235`). We consume only the five
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
    /// Every grant this badge holds, verbatim. Typed as a raw [`Value`], not
    /// `Vec<String>`, on purpose: this field is read on EVERY `/hk` request
    /// while enforcement is on, and a `Vec<String>` makes any surprise shape
    /// (`null`, an object, a list with one number in it) fail the WHOLE body
    /// decode — which is [`LocationOutcome::Unavailable`], i.e. a 503 for every
    /// maid at both properties because a peer changed a field we only read to
    /// widen someone. Leniency here is fail-SAFE precisely because the only
    /// thing this field can do is grant: an unreadable shape yields no grants
    /// ([`Self::holds_admin_grant`]), never an accidental one.
    #[serde(default)]
    apps: Option<serde_json::Value>,
}

impl ResolveBadgeResponse {
    /// Whether `apps` contains [`HK_ADMIN_GRANT`] exactly. PURE.
    ///
    /// Everything that is not an array of strings containing that exact key —
    /// absent, `null`, an object, a string, a list of numbers — is NO grant.
    /// Mirrors `middleware::hk_access::extract_apps`, which reads the same list
    /// off the Access token.
    fn holds_admin_grant(&self) -> bool {
        self.apps
            .as_ref()
            .and_then(|apps| apps.as_array())
            .is_some_and(|entries| {
                entries
                    .iter()
                    .any(|entry| entry.as_str() == Some(HK_ADMIN_GRANT))
            })
    }

    /// Interpret a decoded body. PURE — unit-tested below, which is where the
    /// null/inactive/pending/miss ⇒ `NoLocation`, unknown-value ⇒
    /// `Unavailable` and grant ⇒ `AnyLocation` rules are actually pinned.
    ///
    /// THE ORDER IS THE POLICY, and it is two decisions, not one:
    ///  1. `found`/`active`/`pending` FIRST. HF ID vouching for the person is
    ///     the outer bound; a grant on someone it has disowned is not a
    ///     location decision and must not read as one.
    ///  2. the grant SECOND, before `location` is even inspected — which is
    ///     what makes the field irrelevant to a holder rather than merely
    ///     optional, null and unmappable values included.
    fn outcome(&self) -> LocationOutcome {
        if !self.found || !self.active || self.pending {
            return LocationOutcome::NoLocation;
        }
        if self.holds_admin_grant() {
            return LocationOutcome::AnyLocation;
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

/// Interpret a `/resolve-badge` response body exactly as the production lookup
/// does, including its treatment of an undecodable one. PURE.
///
/// `pub` for the enforcement matrix (`tests/test_hk_location_enforcement.rs`),
/// which drives its rows from REAL payload shapes — an admin WITH a location,
/// an admin with a NULL one, a live non-holder — rather than from hand-written
/// [`LocationOutcome`]s, which would assume the very collapse under test.
pub fn outcome_for_resolve_badge_body(body: &str) -> LocationOutcome {
    match serde_json::from_str::<ResolveBadgeResponse>(body) {
        Ok(decoded) => decoded.outcome(),
        // Same answer the HTTP path gives an undecodable body: no answer.
        Err(_) => LocationOutcome::Unavailable,
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
                tracing::warn!(
                    status = code,
                    "HF ID location lookup returned a non-2xx status"
                );
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

/// A POSITIVE lookup answer — the only thing the cache is able to hold.
///
/// This type is the positive-only policy expressed as a TYPE rather than a
/// comment: [`LocationCache::put`] takes one of these, and there is no way to
/// build one from [`LocationOutcome::NoLocation`] or
/// [`LocationOutcome::Unavailable`] ([`Self::from_outcome`] is the single,
/// total place that decides). A future edit that wanted to cache an outage
/// would have to delete this type to do it.
///
/// It carries BOTH halves of the answer — where the employee is bound, or that
/// they are not bound at all — so a grant change and a location change expire
/// on the same clock. See the module's caching note.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocationAdmission {
    /// Bound to exactly one property.
    At(EmployeeLocation),
    /// Not location-bound — the badge holds [`HK_ADMIN_GRANT`].
    AnyLocation,
}

impl LocationAdmission {
    /// The positive half of an outcome, or `None` for the two negatives. The
    /// ONE place positivity is decided, and total over the enum so a new
    /// outcome variant cannot silently default into the cache.
    pub fn from_outcome(outcome: LocationOutcome) -> Option<Self> {
        match outcome {
            LocationOutcome::Resolved(location) => Some(Self::At(location)),
            LocationOutcome::AnyLocation => Some(Self::AnyLocation),
            LocationOutcome::NoLocation | LocationOutcome::Unavailable => None,
        }
    }

    /// The outcome this admission stands for — what a cache HIT replays. Must
    /// round-trip [`Self::from_outcome`] exactly, or a cached answer would
    /// differ from a fresh one (pinned by a test below).
    pub fn into_outcome(self) -> LocationOutcome {
        match self {
            Self::At(location) => LocationOutcome::Resolved(location),
            Self::AnyLocation => LocationOutcome::AnyLocation,
        }
    }
}

struct CacheEntry {
    admission: LocationAdmission,
    expires_at: Instant,
}

/// Process-local badge → admission cache. POSITIVE ENTRIES ONLY — see the
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

    /// The cached admission for `badge`, or `None` when absent or expired.
    pub fn get(&self, badge: &str) -> Option<LocationAdmission> {
        let guard = self.inner.read().expect("location cache poisoned");
        match guard.get(badge) {
            Some(entry) if entry.expires_at > Instant::now() => Some(entry.admission),
            _ => None,
        }
    }

    /// Cache a POSITIVE answer. There is deliberately no `put` for the other
    /// two outcomes — [`LocationAdmission`] cannot represent them, so the type
    /// system, not a comment, is what keeps a miss or an outage out of the
    /// cache.
    pub fn put(&self, badge: &str, admission: LocationAdmission) {
        let now = Instant::now();
        let mut guard = self.inner.write().expect("location cache poisoned");
        guard.retain(|_, entry| entry.expires_at > now);
        guard.insert(
            badge.to_string(),
            CacheEntry {
                admission,
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
    /// The cache is written only for an outcome [`LocationAdmission`] can
    /// represent — that single `if let` is the whole positive-only policy, and
    /// the negatives are unrepresentable rather than merely unwritten.
    pub async fn resolve(&self, badge: &str) -> LocationOutcome {
        if let Some(admission) = self.cache.get(badge) {
            return admission.into_outcome();
        }
        let outcome = self.lookup.lookup(badge).await;
        if let Some(admission) = LocationAdmission::from_outcome(outcome) {
            self.cache.put(badge, admission);
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
        for bad in [
            "",
            "  ",
            "hf",
            "hf_ville",
            "HFVILLE",
            "HF-VILLE",
            "HF_VILLE_2",
            "ALL",
        ] {
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
        assert_eq!(
            client.resolve("421").await,
            LocationOutcome::Resolved(EmployeeLocation::Hf)
        );
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
        assert_eq!(
            client.cache().get("421"),
            Some(LocationAdmission::At(EmployeeLocation::HfVille))
        );
        assert_eq!(
            client.cache().get("Q1001"),
            Some(LocationAdmission::At(EmployeeLocation::Hf))
        );
        assert_eq!(client.cache().len(), 2);
    }

    // ---- the `housekeeping_admin` grant --------------------------------

    /// The catalog key, pinned as a literal. It is a contract with HF ID's
    /// Employee Management (the peer session creates the entry) — a rename on
    /// either side silently stops widening anyone, so the string is asserted
    /// here rather than only referenced.
    #[test]
    fn the_admin_grant_key_is_the_exact_hfid_catalog_entry() {
        assert_eq!(HK_ADMIN_GRANT, "housekeeping_admin");
    }

    /// The grant overrides `location` ENTIRELY — set, null, or unmappable.
    /// The null row is the load-bearing one: it is the case a "null means
    /// everywhere" shortcut would have handled identically and WRONGLY, since
    /// a null location without the grant still refuses (asserted alongside).
    #[test]
    fn the_admin_grant_overrides_every_location_shape() {
        for location in [r#""HF""#, r#""HF_VILLE""#, "null", r#""HF_THIRD_PROPERTY""#] {
            assert_eq!(
                decode(&format!(
                    r#"{{"found":true,"active":true,
                        "apps":["hotel","housekeeping","housekeeping_admin"],
                        "location":{location}}}"#
                )),
                LocationOutcome::AnyLocation,
                "location={location} must be irrelevant to a grant-holder"
            );
        }
        // …and WITHOUT the grant those same bodies behave exactly as before:
        // this is the byte-unchanged half, and it is what makes the feature
        // dark while nobody holds the grant.
        assert_eq!(
            decode(
                r#"{"found":true,"active":true,"apps":["hotel","housekeeping"],"location":null}"#
            ),
            LocationOutcome::NoLocation,
            "a null location is STILL an absence, not a widening"
        );
        assert_eq!(
            decode(r#"{"found":true,"active":true,"apps":["housekeeping"],"location":"HF"}"#),
            LocationOutcome::Resolved(EmployeeLocation::Hf)
        );
        assert_eq!(
            decode(
                r#"{"found":true,"active":true,"apps":["housekeeping"],
                    "location":"HF_THIRD_PROPERTY"}"#
            ),
            LocationOutcome::Unavailable
        );
    }

    /// What the grant does NOT override: HF ID vouching for the person. A
    /// grant on an unknown, inactive or unapproved employee is still refused —
    /// widening someone's location scope and admitting someone HF ID has
    /// disowned are different decisions, and only the first was made.
    #[test]
    fn the_admin_grant_does_not_override_found_active_or_pending() {
        for body in [
            r#"{"found":false,"active":true,"apps":["housekeeping_admin"],"location":"HF"}"#,
            r#"{"found":true,"active":false,"apps":["housekeeping_admin"],"location":"HF"}"#,
            r#"{"found":true,"active":true,"pending":true,
                "apps":["housekeeping_admin"],"location":"HF"}"#,
        ] {
            assert_eq!(
                decode(body),
                LocationOutcome::NoLocation,
                "the grant must not admit an employee HF ID has not vouched for: {body}"
            );
        }
    }

    /// `apps` is matched EXACTLY. Near-misses, case variants and substrings
    /// grant nothing — an `apps` entry is a catalog key, and a fuzzy match
    /// would hand someone both properties.
    #[test]
    fn near_miss_grant_names_do_not_widen_anyone() {
        for apps in [
            r#"["housekeeping"]"#,
            r#"["HOUSEKEEPING_ADMIN"]"#,
            r#"["Housekeeping_Admin"]"#,
            r#"["housekeeping-admin"]"#,
            r#"["housekeeping_admin_readonly"]"#,
            r#"["admin"]"#,
            r#"[]"#,
        ] {
            assert_eq!(
                decode(&format!(
                    r#"{{"found":true,"active":true,"apps":{apps},"location":"HF"}}"#
                )),
                LocationOutcome::Resolved(EmployeeLocation::Hf),
                "apps={apps} must not grant"
            );
        }
    }

    /// A surprise `apps` SHAPE must not take the surface down. This field is
    /// read on every `/hk` request while enforcement is on, so a strict
    /// `Vec<String>` would turn a peer-side change into a body-decode failure
    /// ⇒ `Unavailable` ⇒ 503 for every maid at both properties. Leniency is
    /// safe here because the only thing the field can do is GRANT, and none of
    /// these shapes does.
    #[test]
    fn a_surprise_apps_shape_degrades_to_no_grant_not_to_an_outage() {
        for apps in [
            "null",
            r#""housekeeping_admin""#, // a bare string, not a list
            r#"{"housekeeping_admin":true}"#,
            "[1,2,3]",
            r#"[null,{"k":"v"},["housekeeping_admin"]]"#, // nested, not an entry
        ] {
            assert_eq!(
                decode(&format!(
                    r#"{{"found":true,"active":true,"apps":{apps},"location":"HF_VILLE"}}"#
                )),
                LocationOutcome::Resolved(EmployeeLocation::HfVille),
                "apps={apps} must read as no-grant, never as an undecodable body"
            );
        }
        // An ABSENT `apps` is the pre-grant payload, and must keep behaving
        // exactly as it did before this field was consumed at all.
        assert_eq!(
            decode(r#"{"found":true,"active":true,"location":"HF_VILLE"}"#),
            LocationOutcome::Resolved(EmployeeLocation::HfVille)
        );
    }

    /// This layer WIDENS; it does not ADMIT. A body carrying
    /// `housekeeping_admin` but NOT `housekeeping` still answers `AnyLocation`
    /// here — and that is correct, because such a badge never reaches this
    /// code: `middleware::hk_access` (and the Cloudflare Access policy in
    /// front of it) refuses it 403 before any lookup is issued, which
    /// `hk_access::admin_grant_alone_does_not_open_the_surface` pins.
    ///
    /// Recorded as a test so the layering is not "fixed" by adding a
    /// `housekeeping` requirement here too. That would be worse than
    /// redundant: this list and the Access token's `apps` claim are two
    /// SNAPSHOTS of the same catalog taken at different moments over different
    /// transports, so a duplicated check could refuse a real admin whose token
    /// is fine — a lockout bought for no security, since the surface is
    /// already closed upstream.
    #[test]
    fn the_location_layer_widens_but_never_admits() {
        assert_eq!(
            decode(r#"{"found":true,"active":true,"apps":["housekeeping_admin"],"location":null}"#),
            LocationOutcome::AnyLocation,
            "admission is hk_access's job, not this module's"
        );
    }

    /// The cache carries the GRANT alongside the location, so a grant-holder's
    /// cached answer replays as `AnyLocation` rather than decaying into a
    /// location (or into nothing).
    #[tokio::test]
    async fn the_admin_grant_is_cached_like_a_location() {
        let lookup = ScriptedLookup::new(LocationOutcome::AnyLocation);
        let client = HfidLocationClient::with_lookup(lookup.clone());

        for _ in 0..5 {
            assert_eq!(client.resolve("ADMIN").await, LocationOutcome::AnyLocation);
        }
        assert_eq!(lookup.calls(), 1, "one LAN round-trip, then the cache");
        assert_eq!(
            client.cache().get("ADMIN"),
            Some(LocationAdmission::AnyLocation),
            "the cached entry must carry the grant, not just a location"
        );
    }

    /// Every positive outcome round-trips through the cache unchanged — a
    /// cache HIT must be indistinguishable from a fresh lookup, or the 60s
    /// window would quietly change someone's admission.
    #[test]
    fn admissions_round_trip_every_positive_outcome() {
        for outcome in [
            LocationOutcome::Resolved(EmployeeLocation::Hf),
            LocationOutcome::Resolved(EmployeeLocation::HfVille),
            LocationOutcome::AnyLocation,
        ] {
            let admission = LocationAdmission::from_outcome(outcome)
                .unwrap_or_else(|| panic!("{outcome:?} is positive and must be cacheable"));
            assert_eq!(admission.into_outcome(), outcome);
        }
        // …and the negatives remain unrepresentable.
        for outcome in [LocationOutcome::NoLocation, LocationOutcome::Unavailable] {
            assert!(
                LocationAdmission::from_outcome(outcome).is_none(),
                "{outcome:?} must never become a cache entry"
            );
        }
    }

    /// A grant CHANGE propagates within one TTL, exactly like a location
    /// change — including the direction that matters most, revocation. The
    /// widening must not outlive the grant.
    #[tokio::test]
    async fn revoking_the_grant_takes_effect_within_one_ttl() {
        /// Answers `AnyLocation` once, then the employee's plain location —
        /// the owner un-ticking the box between the two calls.
        struct Revoking(AtomicUsize);
        #[async_trait]
        impl LocationLookup for Revoking {
            async fn lookup(&self, _badge: &str) -> LocationOutcome {
                match self.0.fetch_add(1, Ordering::SeqCst) {
                    0 => LocationOutcome::AnyLocation,
                    _ => LocationOutcome::Resolved(EmployeeLocation::HfVille),
                }
            }
        }

        let client = HfidLocationClient::with_lookup_and_ttl(
            Arc::new(Revoking(AtomicUsize::new(0))),
            Duration::from_millis(40),
        );
        assert_eq!(client.resolve("ADMIN").await, LocationOutcome::AnyLocation);
        assert_eq!(
            client.resolve("ADMIN").await,
            LocationOutcome::AnyLocation,
            "still cached inside the window"
        );

        tokio::time::sleep(Duration::from_millis(60)).await;
        assert_eq!(
            client.resolve("ADMIN").await,
            LocationOutcome::Resolved(EmployeeLocation::HfVille),
            "a revoked grant must narrow again within one TTL"
        );
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
