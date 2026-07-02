//! Cloudflare Access JWT verification — auto-login support.
//!
//! The deployment sits behind Cloudflare Access: every request that
//! reaches the origin already passed the CF identity check, and CF
//! attaches a signed RS256 JWT asserting WHO passed, either as the
//! `Cf-Access-Jwt-Assertion` request header or as the
//! `CF_Authorization` cookie. This module verifies that assertion so
//! `POST /api/auth/cf-login` (routes::auth::cf_login) can mint a normal
//! `ht_sessions` cookie session for the mapped user — no second login.
//!
//! ## Verification contract
//!
//! * Signature: RS256 against the team JWKS fetched from
//!   `<team domain>/cdn-cgi/access/certs`, cached in-process for
//!   [`JWKS_CACHE_TTL`] (CF rotates keys; a stale-cache miss surfaces as
//!   a verification failure and the next request after TTL re-fetches).
//! * `iss` MUST equal the team domain ([`CF_ACCESS_TEAM_DOMAIN`]).
//! * `aud` MUST contain the Access application AUD tag — env override
//!   `CF_ACCESS_AUD`, defaulting to [`DEFAULT_CF_ACCESS_AUD`] (the AUD
//!   tag is a public application identifier, not a secret).
//! * `exp` / `nbf` enforced (default 60s leeway from `jsonwebtoken`).
//! * The `email` claim is REQUIRED — it is the identity we map to
//!   `ht_users.email`. Service-token JWTs (no email) are rejected.
//!
//! ## Layering
//!
//! This is a leaf verification helper (no DB, no session logic). The
//! route layer owns the "map email → user → mint session" flow via
//! `AuthService::login_via_email`, keeping business logic in the
//! service layer per docs/architecture.md.
//!
//! ## Network + testing
//!
//! JWKS fetch uses `ureq` (the crate's only HTTP client, per the
//! no-reqwest policy) dispatched through `tokio::task::spawn_blocking`
//! — the same pattern as `notifications::slack`. The actual token check
//! is the PURE function [`verify_token`], unit-tested against a static
//! in-repo JWKS + RSA key so tests never touch the network.

use std::sync::Mutex;
use std::time::{Duration, Instant};

use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::Deserialize;

/// The Cloudflare team domain — the REQUIRED `iss` of every accepted
/// JWT, and the base URL the JWKS is fetched from.
pub const CF_ACCESS_TEAM_DOMAIN: &str = "https://laikaexpress.cloudflareaccess.com";

/// Default Access-application AUD tag (public identifier, not a
/// secret). Override with the `CF_ACCESS_AUD` env var if the Access
/// application is ever recreated.
pub const DEFAULT_CF_ACCESS_AUD: &str =
    "832861a2b62e6ce2b0e100d1bd40c84789ddee561aa5973c896f4aced4d821cf";

/// Request header CF attaches the assertion under.
pub const CF_ACCESS_JWT_HEADER: &str = "cf-access-jwt-assertion";

/// Cookie fallback (browser requests carry this instead of / alongside
/// the header depending on how the request reached the origin).
pub const CF_ACCESS_COOKIE_NAME: &str = "CF_Authorization";

/// How long a fetched JWKS is trusted before re-fetching. Cloudflare
/// rotates signing keys on the order of weeks and keeps predecessors in
/// the set during rollover, so minutes-level staleness is safe; a short
/// TTL bounds the window where a brand-new key would fail-verify.
const JWKS_CACHE_TTL: Duration = Duration::from_secs(10 * 60);

/// Resolve the expected AUD tag: `CF_ACCESS_AUD` env override, else the
/// baked-in default. Trimmed so a stray newline in an env file doesn't
/// silently break every verification.
pub fn expected_aud() -> String {
    match std::env::var("CF_ACCESS_AUD") {
        Ok(value) if !value.trim().is_empty() => value.trim().to_string(),
        _ => DEFAULT_CF_ACCESS_AUD.to_string(),
    }
}

/// Resolve a VERIFIED email through the alias spec: `from1=to1,from2=to2`,
/// case-insensitive on the `from` side. Returns `None` when no alias
/// matches (caller keeps the original email). Malformed entries (no `=`,
/// empty side, blank segment) are silently skipped — a typo in one alias
/// must not break the others or the passthrough path.
///
/// Applied AFTER JWT verification and BEFORE the `ht_users.email`
/// lookup, so it only ever REMAPS an identity Cloudflare already proved
/// — it can never mint a session for an unverified address. Exists
/// because a person can legitimately hold several Google identities at
/// the CF Access edge (live verification 2026-07-02: the owner signs in
/// alternately as nut.winut@gmail.com and winut.hf@gmail.com) while
/// `ht_users.email` stores exactly one canonical mailbox per user.
pub fn resolve_email_alias(email: &str, spec: &str) -> Option<String> {
    for entry in spec.split(',') {
        let Some((from, to)) = entry.split_once('=') else {
            continue; // malformed: no '=' — skip
        };
        let (from, to) = (from.trim(), to.trim());
        if from.is_empty() || to.is_empty() {
            continue; // malformed: empty side — skip
        }
        if from.eq_ignore_ascii_case(email) {
            return Some(to.to_string());
        }
    }
    None
}

/// Env-configured alias application: reads `CF_EMAIL_ALIASES`
/// (`from1=to1,from2=to2`) and remaps `email` when an alias matches,
/// else returns it unchanged. Unset/empty env = pure passthrough.
pub fn apply_email_aliases(email: &str) -> String {
    match std::env::var("CF_EMAIL_ALIASES") {
        Ok(spec) => match resolve_email_alias(email, &spec) {
            Some(mapped) => {
                tracing::info!(
                    from = %email,
                    to = %mapped,
                    "cf_access: applied CF_EMAIL_ALIASES mapping"
                );
                mapped
            }
            None => email.to_string(),
        },
        Err(_) => email.to_string(),
    }
}

/// Errors from the CF Access verification path. Coarse on purpose — the
/// route maps everything except `NoEmail`/`Invalid` details to a single
/// 401 wire code; the detail lands in logs only.
#[derive(Debug, thiserror::Error)]
pub enum CfAccessError {
    /// Neither the `Cf-Access-Jwt-Assertion` header nor the
    /// `CF_Authorization` cookie carried a token.
    #[error("no Cloudflare Access token on the request")]
    MissingToken,

    /// JWKS endpoint unreachable / non-JSON / empty key set.
    #[error("JWKS fetch failed: {0}")]
    Jwks(String),

    /// Token failed signature / iss / aud / exp / nbf validation.
    #[error("token verification failed: {0}")]
    Invalid(String),

    /// Token verified but carries no `email` claim (e.g. a service
    /// token) — nothing to map to `ht_users.email`.
    #[error("verified token has no email claim")]
    NoEmail,
}

/// Minimal JWK projection — only what RS256 verification needs.
#[derive(Debug, Clone, Deserialize)]
pub struct Jwk {
    #[serde(default)]
    pub kid: Option<String>,
    pub kty: String,
    #[serde(default)]
    pub n: Option<String>,
    #[serde(default)]
    pub e: Option<String>,
}

/// The `/cdn-cgi/access/certs` response shape (we ignore the legacy
/// `public_cert(s)` PEM fields — the `keys` JWKS array is canonical).
#[derive(Debug, Clone, Deserialize)]
pub struct JwkSet {
    pub keys: Vec<Jwk>,
}

/// Claims we read off a verified CF Access token. `aud`/`iss`/`exp`/
/// `nbf` are enforced by `jsonwebtoken`'s `Validation` and don't need
/// to be materialized here beyond what serde requires.
#[derive(Debug, Deserialize)]
struct CfAccessClaims {
    #[serde(default)]
    email: Option<String>,
}

/// PURE token verification against an already-fetched JWKS.
///
/// Returns the verified `email` claim on success. No network, no DB —
/// this is the unit-testable core. `iss` and `aud` are parameters (not
/// read from env) for the same reason.
pub fn verify_token(
    token: &str,
    jwks: &JwkSet,
    issuer: &str,
    audience: &str,
) -> Result<String, CfAccessError> {
    let header =
        decode_header(token).map_err(|err| CfAccessError::Invalid(err.to_string()))?;
    if header.alg != Algorithm::RS256 {
        return Err(CfAccessError::Invalid(format!(
            "unexpected alg {:?} (only RS256 accepted)",
            header.alg
        )));
    }

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[issuer]);
    validation.set_audience(&[audience]);
    validation.validate_nbf = true;

    // Prefer the key whose kid matches the token header; fall back to
    // trying every RSA key in the set (CF keeps the outgoing key in the
    // JWKS during rotation, so a kid miss usually means "rotated within
    // our cache TTL" — any key that verifies the signature is fine, the
    // signature itself is the proof).
    let candidates: Vec<&Jwk> = {
        let matching: Vec<&Jwk> = jwks
            .keys
            .iter()
            .filter(|k| k.kty == "RSA" && k.kid.as_deref() == header.kid.as_deref())
            .collect();
        if matching.is_empty() {
            jwks.keys.iter().filter(|k| k.kty == "RSA").collect()
        } else {
            matching
        }
    };

    if candidates.is_empty() {
        return Err(CfAccessError::Jwks("JWKS contains no RSA keys".to_string()));
    }

    let mut last_err = CfAccessError::Invalid("no key verified the token".to_string());
    for key in candidates {
        let (Some(n), Some(e)) = (key.n.as_deref(), key.e.as_deref()) else {
            continue;
        };
        let decoding_key = match DecodingKey::from_rsa_components(n, e) {
            Ok(k) => k,
            Err(err) => {
                last_err = CfAccessError::Jwks(format!("bad JWK components: {err}"));
                continue;
            }
        };
        match decode::<CfAccessClaims>(token, &decoding_key, &validation) {
            Ok(data) => {
                return match data.claims.email {
                    Some(email) if !email.trim().is_empty() => {
                        Ok(email.trim().to_string())
                    }
                    _ => Err(CfAccessError::NoEmail),
                };
            }
            Err(err) => {
                last_err = CfAccessError::Invalid(err.to_string());
            }
        }
    }
    Err(last_err)
}

// =============================================================================
// JWKS fetch + in-process cache
// =============================================================================

/// Process-wide JWKS cache: `(fetched_at, keys)`. A `Mutex<Option<..>>`
/// (not `RwLock`) is fine — the lock is held only to clone out the Arc
/// or swap in a fresh fetch, both sub-microsecond.
static JWKS_CACHE: Mutex<Option<(Instant, std::sync::Arc<JwkSet>)>> = Mutex::new(None);

/// The JWKS endpoint for the team domain.
fn certs_url() -> String {
    format!("{CF_ACCESS_TEAM_DOMAIN}/cdn-cgi/access/certs")
}

/// Blocking JWKS fetch — call via `spawn_blocking` only.
fn fetch_jwks_blocking(url: &str) -> Result<JwkSet, CfAccessError> {
    let response = ureq::get(url)
        .timeout(std::time::Duration::from_secs(10))
        .call()
        .map_err(|err| CfAccessError::Jwks(err.to_string()))?;
    let set: JwkSet = response
        .into_json()
        .map_err(|err| CfAccessError::Jwks(format!("JWKS parse: {err}")))?;
    if set.keys.is_empty() {
        return Err(CfAccessError::Jwks("JWKS key set is empty".to_string()));
    }
    Ok(set)
}

/// Return the cached JWKS, re-fetching when older than [`JWKS_CACHE_TTL`].
///
/// On fetch failure with a stale-but-present cache we KEEP serving the
/// stale keys (fail-open on freshness, never on verification): a
/// transient certs-endpoint blip shouldn't break auto-login while the
/// previous keys are still likely valid. With no cache at all the error
/// propagates and the caller answers 401.
async fn cached_jwks() -> Result<std::sync::Arc<JwkSet>, CfAccessError> {
    if let Some((fetched_at, keys)) = JWKS_CACHE.lock().unwrap().as_ref() {
        if fetched_at.elapsed() < JWKS_CACHE_TTL {
            return Ok(keys.clone());
        }
    }

    let url = certs_url();
    let fetched = tokio::task::spawn_blocking(move || fetch_jwks_blocking(&url))
        .await
        .map_err(|err| CfAccessError::Jwks(format!("join error: {err}")))?;

    match fetched {
        Ok(set) => {
            let arc = std::sync::Arc::new(set);
            *JWKS_CACHE.lock().unwrap() = Some((Instant::now(), arc.clone()));
            Ok(arc)
        }
        Err(err) => {
            // Stale-if-error: serve the expired cache when we have one.
            if let Some((_, keys)) = JWKS_CACHE.lock().unwrap().as_ref() {
                tracing::warn!(error = %err, "cf_access: JWKS refresh failed, serving stale cache");
                return Ok(keys.clone());
            }
            Err(err)
        }
    }
}

/// Extract the CF Access token from a request: header first, cookie
/// fallback. Returns `None` when neither is present.
pub fn extract_token(
    headers: &axum::http::HeaderMap,
    jar: &axum_extra::extract::cookie::CookieJar,
) -> Option<String> {
    if let Some(value) = headers
        .get(CF_ACCESS_JWT_HEADER)
        .and_then(|v| v.to_str().ok())
    {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }
    jar.get(CF_ACCESS_COOKIE_NAME)
        .map(|c| c.value().trim().to_string())
        .filter(|v| !v.is_empty())
}

/// Full production verification: fetch (cached) JWKS, verify `token`,
/// return the asserted email. Env-configurable AUD, fixed team-domain
/// issuer.
pub async fn verify_cf_access_token(token: &str) -> Result<String, CfAccessError> {
    let jwks = cached_jwks().await?;
    verify_token(token, &jwks, CF_ACCESS_TEAM_DOMAIN, &expected_aud())
}

// =============================================================================
// Tests — static JWKS + RSA key, NO network
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde::Serialize;

    /// Test-only 2048-bit RSA key (generated for this test module; not
    /// used anywhere outside `cfg(test)`).
    const TEST_RSA_PRIVATE_PEM: &str = "-----BEGIN PRIVATE KEY-----
MIIEvAIBADANBgkqhkiG9w0BAQEFAASCBKYwggSiAgEAAoIBAQDFw+M9zWA4oTIu
X0r+Okvpsaxj40B56w8GHQE8iG+n7xlgU801zTq/mK9SSu44lJ8SPR8P1/62ta/n
tojdtEbu0FpgIKIgEnmluvGR0itquDR92lK7k35uGMsWHDMU3UmLcwpSj/q9LVYB
8N0CRqTmoVMix4cR5oqUuDS2uqHNxXXM5pRheFaNQZpFEMTjh/ldKwCQnXLxWIzj
AMRUNq7yW8WPLwWVpypQlfUd3CCSN2t9ePfa8dPahzIifqy4qgSnpDdw6xLXT5/s
LEJNZr12MQh0TSNpG7C1QPoTtv6to7BilDPzwkR9WShj8aHW6hvDuaS7/DlyHz3K
pDEqFOqbAgMBAAECggEACQoMk3EVMlFv3T2+zEL24E4eLoyfEFOFosZlnZIw5FCh
7My3xvtD8aj1boH9RHnKkYdYqZ06R7ijOyiVNej9CwJb9yPWtTeS9tfGHd+o215j
C9OUT32M3FRmx/JvBUeeCnEuKhrpn4b4dOtA9s8qz13VUnQjQNa0Q9rtkeKS7bhB
t/WTxYdM/3gC5iNvP7Xt0ozpIp7pcf/53ihcB3bqKNe5/wNymEMZvNU8JlTaou92
EwZnuT3R6a1kiFk2uKCiM1D8u7plElDf9zh9b4HdwLBHaxW79utv2PYzrwgdqsf+
5boOz5JTl+ed1qxkyABl1J93IIf4fh3JhYAtWR5hbQKBgQDnTvtjpi7N6lNY3sVm
iEHeTlJYKOizUV9jnbPyAjtKNyf3+gXJ4NO1BZafYlQRZtXfGosBL0m0SZzMsfds
CXfAVDIopNIrxub+e+s1sknTGOIWJ8upHpJ/2OnlAMRz8j8T0aODYgcD5x7rNIId
4oxYho3MZO7WWseLZt3UDlqKnQKBgQDa4EJ4Ngvp6U61yQ82msGuI5fylUDFh37K
p0MmAIq+nWLSF3O0JX2G0Xk0WqkXxr3jhAFBsXbiYDgbBlJCtM52aSMAa9tNTQFp
aQlQoi4zAImFbrhW+34O2w3UUSNH3f2zXYn2NZeZyfTdviLyH5gwoMBbGLMjPnXE
e6T2DyVIlwKBgGqfy94ZgsXE2HrE5fXnpYVWUTr2UJ4oSnJfBc3vHWmOl0wF4pk2
nCB73PzHlL0YzEm1sJHxPGZw8GijOMyCaMMtjJmTsJYhb+WrNbdg4gr/E2jnG0hw
IVPxp4+6lNRlvJHkNx2fGGDCL0x4veoMvmkoTUEE3dvNqOInnuXbX/05AoGAemGS
SOzPVIbjP7mgDAQT725vc3AIu2m7d0x2uzTqXxJZQudiBoQ/37YYczGOAoFZg3E3
0qeLtZ/fPx0Vub6nAoZez8l+4YYBGBNm5fMIqfPO8RCredc88MmCvghwFasGQ9g1
X7kvfwnxJFs/5unLisUXSNhSsY4nAymvXvWw/xkCgYBO4Y69iQMcF7ZqUGqUt4oN
vcdnJWXHLHTWSdcxL7lTZRWDoJNrwd2g5rJ18eERCAsv46obamuEUIdM4Tv/NFq/
3+zPhdP3HduBY1bRQ6C4SqSUttF3ZLLytgYa37kqCLfIUMh9/36asvzsJMvs6osX
qHreiO5t7e6J+dsQVkE3dg==
-----END PRIVATE KEY-----";

    /// base64url modulus of the public half of `TEST_RSA_PRIVATE_PEM`.
    const TEST_RSA_N: &str = "xcPjPc1gOKEyLl9K_jpL6bGsY-NAeesPBh0BPIhvp-8ZYFPNNc06v5ivUkruOJSfEj0fD9f-trWv57aI3bRG7tBaYCCiIBJ5pbrxkdIrarg0fdpSu5N-bhjLFhwzFN1Ji3MKUo_6vS1WAfDdAkak5qFTIseHEeaKlLg0trqhzcV1zOaUYXhWjUGaRRDE44f5XSsAkJ1y8ViM4wDEVDau8lvFjy8FlacqUJX1HdwgkjdrfXj32vHT2ocyIn6suKoEp6Q3cOsS10-f7CxCTWa9djEIdE0jaRuwtUD6E7b-raOwYpQz88JEfVkoY_Gh1uobw7mku_w5ch89yqQxKhTqmw";
    const TEST_RSA_E: &str = "AQAB";

    const TEST_ISS: &str = "https://laikaexpress.cloudflareaccess.com";
    const TEST_AUD: &str = "test-aud-tag";

    fn test_jwks() -> JwkSet {
        JwkSet {
            keys: vec![Jwk {
                kid: Some("test-key-1".to_string()),
                kty: "RSA".to_string(),
                n: Some(TEST_RSA_N.to_string()),
                e: Some(TEST_RSA_E.to_string()),
            }],
        }
    }

    #[derive(Serialize)]
    struct TestClaims {
        iss: String,
        aud: String,
        exp: i64,
        nbf: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        email: Option<String>,
    }

    fn now() -> i64 {
        chrono::Utc::now().timestamp()
    }

    fn sign(claims: &TestClaims, kid: Option<&str>) -> String {
        let mut header = Header::new(Algorithm::RS256);
        header.kid = kid.map(str::to_string);
        let key = EncodingKey::from_rsa_pem(TEST_RSA_PRIVATE_PEM.as_bytes())
            .expect("test key must parse");
        encode(&header, claims, &key).expect("signing must succeed")
    }

    fn valid_claims() -> TestClaims {
        TestClaims {
            iss: TEST_ISS.to_string(),
            aud: TEST_AUD.to_string(),
            exp: now() + 3600,
            nbf: now() - 60,
            email: Some("winut.hf@gmail.com".to_string()),
        }
    }

    #[test]
    fn valid_token_yields_email() {
        let token = sign(&valid_claims(), Some("test-key-1"));
        let email = verify_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect("valid token must verify");
        assert_eq!(email, "winut.hf@gmail.com");
    }

    #[test]
    fn valid_token_without_kid_still_verifies_via_fallback() {
        // CF key rotation inside our cache TTL surfaces as a kid miss —
        // the fallback tries every RSA key, and the signature is the
        // actual proof.
        let token = sign(&valid_claims(), None);
        let email = verify_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect("kid-less token must verify via try-all fallback");
        assert_eq!(email, "winut.hf@gmail.com");
    }

    #[test]
    fn expired_token_is_rejected() {
        let mut claims = valid_claims();
        claims.exp = now() - 3600; // well past the 60s default leeway
        let token = sign(&claims, Some("test-key-1"));
        let err = verify_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect_err("expired token must be rejected");
        assert!(matches!(err, CfAccessError::Invalid(_)), "got: {err:?}");
    }

    #[test]
    fn not_yet_valid_token_is_rejected() {
        let mut claims = valid_claims();
        claims.nbf = now() + 3600;
        let token = sign(&claims, Some("test-key-1"));
        let err = verify_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect_err("nbf-in-the-future token must be rejected");
        assert!(matches!(err, CfAccessError::Invalid(_)), "got: {err:?}");
    }

    #[test]
    fn wrong_audience_is_rejected() {
        let mut claims = valid_claims();
        claims.aud = "some-other-application-aud".to_string();
        let token = sign(&claims, Some("test-key-1"));
        let err = verify_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect_err("wrong aud must be rejected");
        assert!(matches!(err, CfAccessError::Invalid(_)), "got: {err:?}");
    }

    #[test]
    fn wrong_issuer_is_rejected() {
        let mut claims = valid_claims();
        claims.iss = "https://evil.cloudflareaccess.com".to_string();
        let token = sign(&claims, Some("test-key-1"));
        let err = verify_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect_err("wrong iss must be rejected");
        assert!(matches!(err, CfAccessError::Invalid(_)), "got: {err:?}");
    }

    #[test]
    fn garbage_token_is_rejected() {
        for garbage in ["", "not-a-jwt", "a.b.c", "!!!.???.###"] {
            let err = verify_token(garbage, &test_jwks(), TEST_ISS, TEST_AUD)
                .expect_err("garbage must be rejected");
            assert!(matches!(err, CfAccessError::Invalid(_)), "got: {err:?}");
        }
    }

    #[test]
    fn token_without_email_claim_is_rejected() {
        let mut claims = valid_claims();
        claims.email = None;
        let token = sign(&claims, Some("test-key-1"));
        let err = verify_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect_err("email-less (service) token must be rejected");
        assert!(matches!(err, CfAccessError::NoEmail), "got: {err:?}");
    }

    #[test]
    fn jwks_without_rsa_keys_is_rejected() {
        let token = sign(&valid_claims(), Some("test-key-1"));
        let empty = JwkSet {
            keys: vec![Jwk {
                kid: Some("ec-key".to_string()),
                kty: "EC".to_string(),
                n: None,
                e: None,
            }],
        };
        let err = verify_token(&token, &empty, TEST_ISS, TEST_AUD)
            .expect_err("no-RSA JWKS must fail closed");
        assert!(matches!(err, CfAccessError::Jwks(_)), "got: {err:?}");
    }

    #[test]
    fn email_alias_hit_is_case_insensitive_on_from_side() {
        let spec = "nut.winut@gmail.com=winut.hf@gmail.com,other@x.com=y@x.com";
        assert_eq!(
            resolve_email_alias("Nut.Winut@GMAIL.com", spec).as_deref(),
            Some("winut.hf@gmail.com"),
        );
        // Second entry resolves too.
        assert_eq!(
            resolve_email_alias("other@x.com", spec).as_deref(),
            Some("y@x.com"),
        );
    }

    #[test]
    fn email_alias_miss_is_passthrough() {
        let spec = "nut.winut@gmail.com=winut.hf@gmail.com";
        assert_eq!(resolve_email_alias("winai.sdy@gmail.com", spec), None);
        // Empty spec never matches.
        assert_eq!(resolve_email_alias("anyone@x.com", ""), None);
        // The TO side must not be treated as a FROM key (no reverse map).
        assert_eq!(resolve_email_alias("winut.hf@gmail.com", spec), None);
    }

    #[test]
    fn email_alias_ignores_malformed_entries() {
        // Garbage segments (no '=', empty sides, blank) are skipped
        // without breaking the valid entry around them.
        let spec = "garbage, =nowhere,orphan=, ,nut.winut@gmail.com = winut.hf@gmail.com ,";
        assert_eq!(
            resolve_email_alias("nut.winut@gmail.com", spec).as_deref(),
            Some("winut.hf@gmail.com"),
        );
        assert_eq!(resolve_email_alias("garbage", spec), None);
        assert_eq!(resolve_email_alias("orphan", spec), None);
    }

    #[test]
    fn expected_aud_defaults_to_baked_in_constant() {
        // NB: relies on CF_ACCESS_AUD not being set in the test env.
        if std::env::var("CF_ACCESS_AUD").is_err() {
            assert_eq!(expected_aud(), DEFAULT_CF_ACCESS_AUD);
        }
    }
}
