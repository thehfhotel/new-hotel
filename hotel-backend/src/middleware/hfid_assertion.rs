//! HF-ID card-tap assertion verification — central-pairing card login.
//!
//! The physical NFC reader now posts taps to the CENTRAL HF-ID service (not to
//! this PMS). The login screen pairs to a reader via HF-ID's claim/wait pair
//! (`service::reader::HfIdClient`); when a tap lands, HF-ID hands back a signed
//! RS256 **assertion** (an HF-ID id_token). This module verifies that assertion
//! so `GET /api/reader/wait` (routes::reader::wait) can trust the tapped
//! identity and stash a one-time login for `POST /api/auth/card-login`.
//!
//! This is a SECOND JWKS source alongside `middleware::cf_access` and follows
//! that module's pattern verbatim — the only differences are the issuer
//! (`https://id.thehfhotel.org/oidc`), the audience/app (`hotel`), the JWKS
//! endpoint (`{HFID_BASE_URL}/oidc/jwks`), and the identity we extract (`sub` =
//! badge, plus the `apps` grant list). The wire JWK types are reused from
//! `cf_access` (identical JWKS shape) to avoid a duplicate struct.
//!
//! ## Verification contract
//!
//! * Signature: RS256 against the JWKS fetched from
//!   `{HFID_BASE_URL}/oidc/jwks`, cached in-process for [`JWKS_CACHE_TTL`].
//! * `iss` MUST equal [`HFID_ISSUER`].
//! * `aud` MUST equal [`HFID_APP`] (`"hotel"`).
//! * `exp` enforced (HF-ID assertions are `iat + 300s`; default 60s leeway).
//! * `sub` (badge) REQUIRED and non-empty — it is the identity we resolve /
//!   auto-provision via `service::reader::find_or_provision_user_by_badge`.
//! * `apps` MUST contain [`HFID_APP`] — defence-in-depth on top of the `aud`
//!   check (HF-ID gates authorization centrally and only ever mints an
//!   assertion for an authorized tap, but we re-check the grant here so a
//!   misissued token can never log a badge into the PMS).
//!
//! ## Network + testing
//!
//! JWKS fetch uses `ureq` dispatched through `tokio::task::spawn_blocking`
//! (the no-reqwest policy, same as `cf_access` / `notifications::slack`). The
//! token check is the PURE function [`verify_assertion`], unit-tested against a
//! static in-repo JWKS + RSA key so tests never touch the network.

use std::sync::Mutex;
use std::time::{Duration, Instant};

use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::Deserialize;

// Reuse the JWK wire types from the CF Access verifier — the HF-ID JWKS has the
// identical `{ "keys": [ { kid, kty, n, e } ] }` shape, so there is no reason
// to duplicate the structs.
use super::cf_access::{Jwk, JwkSet};

/// REQUIRED `iss` of every accepted HF-ID assertion.
pub const HFID_ISSUER: &str = "https://id.thehfhotel.org/oidc";

/// The app identifier this PMS is: the REQUIRED `aud`, the value that must be
/// present in the assertion's `apps` grant list, and the `app` sent on the
/// central claim call. HF Hotel front-desk card login only.
pub const HFID_APP: &str = "hotel";

/// How long a fetched JWKS is trusted before re-fetching. HF-ID rotates signing
/// keys rarely and keeps predecessors in the set during rollover, so
/// minutes-level staleness is safe; a short TTL bounds the window where a
/// brand-new key would fail-verify. Mirrors `cf_access::JWKS_CACHE_TTL`.
const JWKS_CACHE_TTL: Duration = Duration::from_secs(10 * 60);

/// Errors from the HF-ID assertion verification path. Coarse on purpose — the
/// `wait` route collapses every variant to a single wire code; the detail
/// lands in logs only.
#[derive(Debug, thiserror::Error)]
pub enum HfIdAssertionError {
    /// JWKS endpoint unreachable / non-JSON / empty (or no RSA) key set.
    #[error("JWKS fetch failed: {0}")]
    Jwks(String),

    /// Token failed signature / iss / aud / exp validation, or carried no
    /// usable `sub` (badge).
    #[error("assertion verification failed: {0}")]
    Invalid(String),

    /// Token verified but its `apps` grant list does not contain `"hotel"` —
    /// the badge is not authorized for this app.
    #[error("assertion does not grant the '{HFID_APP}' app")]
    NotAuthorized,
}

/// A verified HF-ID identity extracted from a valid assertion.
#[derive(Debug, Clone)]
pub struct HfIdIdentity {
    /// The `sub` claim — the badge, keyed into `ht_users.badge`.
    pub badge: String,
    /// Optional `name` claim, stored only on first-sight auto-provision. HF-ID
    /// assertions may omit it; `None` then leaves the display name unset.
    pub display_name: Option<String>,
}

/// Claims read off a verified HF-ID assertion. `iss` / `aud` / `exp` are
/// enforced by `jsonwebtoken`'s [`Validation`] and are not materialized here.
#[derive(Debug, Deserialize)]
struct HfIdClaims {
    /// Subject — the badge. Non-`Option` so a token missing `sub` fails to
    /// deserialize (→ [`HfIdAssertionError::Invalid`]).
    sub: String,
    #[serde(default)]
    apps: Vec<String>,
    #[serde(default)]
    name: Option<String>,
}

/// PURE assertion verification against an already-fetched JWKS.
///
/// Returns the verified [`HfIdIdentity`] on success. No network, no DB — this
/// is the unit-testable core. `issuer` and `audience` are parameters (not read
/// from env/consts) so the tests can drive them directly, mirroring
/// `cf_access::verify_token`.
pub fn verify_assertion(
    token: &str,
    jwks: &JwkSet,
    issuer: &str,
    audience: &str,
) -> Result<HfIdIdentity, HfIdAssertionError> {
    let header =
        decode_header(token).map_err(|err| HfIdAssertionError::Invalid(err.to_string()))?;
    if header.alg != Algorithm::RS256 {
        return Err(HfIdAssertionError::Invalid(format!(
            "unexpected alg {:?} (only RS256 accepted)",
            header.alg
        )));
    }

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[issuer]);
    validation.set_audience(&[audience]);
    // Defence-in-depth vs GHSA-h395-gr6q-cpjc (jsonwebtoken type confusion) —
    // same rationale as `cf_access`: requiring the claims keeps the bypass
    // closed even if the crate regresses. HF-ID assertions always carry
    // exp/iss/aud/sub. `nbf` is NOT required (HF-ID assertions omit it).
    validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);

    // Prefer the key whose kid matches the token header; fall back to trying
    // every RSA key in the set (rotation within the cache TTL surfaces as a kid
    // miss — the signature itself is the proof). Same policy as `cf_access`.
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
        return Err(HfIdAssertionError::Jwks(
            "JWKS contains no RSA keys".to_string(),
        ));
    }

    let mut last_err = HfIdAssertionError::Invalid("no key verified the assertion".to_string());
    for key in candidates {
        let (Some(n), Some(e)) = (key.n.as_deref(), key.e.as_deref()) else {
            continue;
        };
        let decoding_key = match DecodingKey::from_rsa_components(n, e) {
            Ok(k) => k,
            Err(err) => {
                last_err = HfIdAssertionError::Jwks(format!("bad JWK components: {err}"));
                continue;
            }
        };
        match decode::<HfIdClaims>(token, &decoding_key, &validation) {
            Ok(data) => {
                let claims = data.claims;
                // Grant re-check on top of the enforced `aud`.
                if !claims.apps.iter().any(|app| app == audience) {
                    return Err(HfIdAssertionError::NotAuthorized);
                }
                let badge = claims.sub.trim();
                if badge.is_empty() {
                    return Err(HfIdAssertionError::Invalid("empty sub (badge)".to_string()));
                }
                return Ok(HfIdIdentity {
                    badge: badge.to_string(),
                    display_name: claims
                        .name
                        .map(|n| n.trim().to_string())
                        .filter(|n| !n.is_empty()),
                });
            }
            Err(err) => {
                last_err = HfIdAssertionError::Invalid(err.to_string());
            }
        }
    }
    Err(last_err)
}

// =============================================================================
// JWKS fetch + in-process cache
// =============================================================================

/// Process-wide JWKS cache: `(fetched_at, keys)`. Single-slot (like
/// `cf_access`) — `HFID_BASE_URL` is fixed per process, so one slot suffices.
static JWKS_CACHE: Mutex<Option<(Instant, std::sync::Arc<JwkSet>)>> = Mutex::new(None);

/// The JWKS endpoint for the configured HF-ID base URL.
fn certs_url(base_url: &str) -> String {
    format!("{}/oidc/jwks", base_url.trim_end_matches('/'))
}

/// Blocking JWKS fetch — call via `spawn_blocking` only.
fn fetch_jwks_blocking(url: &str) -> Result<JwkSet, HfIdAssertionError> {
    let response = ureq::get(url)
        .timeout(Duration::from_secs(10))
        .call()
        .map_err(|err| HfIdAssertionError::Jwks(err.to_string()))?;
    let set: JwkSet = response
        .into_json()
        .map_err(|err| HfIdAssertionError::Jwks(format!("JWKS parse: {err}")))?;
    if set.keys.is_empty() {
        return Err(HfIdAssertionError::Jwks("JWKS key set is empty".to_string()));
    }
    Ok(set)
}

/// Return the cached JWKS, re-fetching when older than [`JWKS_CACHE_TTL`].
/// Stale-if-error (serve expired keys when a refresh fails but a cache exists)
/// mirrors `cf_access::cached_jwks` — fail-open on freshness, never on the
/// verification itself.
async fn cached_jwks(base_url: &str) -> Result<std::sync::Arc<JwkSet>, HfIdAssertionError> {
    if let Some((fetched_at, keys)) = JWKS_CACHE.lock().unwrap().as_ref() {
        if fetched_at.elapsed() < JWKS_CACHE_TTL {
            return Ok(keys.clone());
        }
    }

    let url = certs_url(base_url);
    let fetched = tokio::task::spawn_blocking(move || fetch_jwks_blocking(&url))
        .await
        .map_err(|err| HfIdAssertionError::Jwks(format!("join error: {err}")))?;

    match fetched {
        Ok(set) => {
            let arc = std::sync::Arc::new(set);
            *JWKS_CACHE.lock().unwrap() = Some((Instant::now(), arc.clone()));
            Ok(arc)
        }
        Err(err) => {
            if let Some((_, keys)) = JWKS_CACHE.lock().unwrap().as_ref() {
                tracing::warn!(error = %err, "hfid_assertion: JWKS refresh failed, serving stale cache");
                return Ok(keys.clone());
            }
            Err(err)
        }
    }
}

/// Full production verification: fetch (cached) JWKS from `{base_url}/oidc/jwks`,
/// verify `token` against the fixed HF-ID issuer + `hotel` audience, return the
/// asserted identity.
pub async fn verify_hfid_assertion(
    token: &str,
    base_url: &str,
) -> Result<HfIdIdentity, HfIdAssertionError> {
    let jwks = cached_jwks(base_url).await?;
    verify_assertion(token, &jwks, HFID_ISSUER, HFID_APP)
}

// =============================================================================
// Tests — static JWKS + RSA key, NO network. Mirrors `cf_access::tests`.
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{encode, EncodingKey, Header};
    use serde::Serialize;

    /// Test-only 2048-bit RSA key (shared shape with `cf_access::tests`; only
    /// used under `cfg(test)`).
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

    const TEST_ISS: &str = HFID_ISSUER;
    const TEST_AUD: &str = HFID_APP; // "hotel"

    fn test_jwks() -> JwkSet {
        JwkSet {
            keys: vec![Jwk {
                kid: Some("hfid-key-1".to_string()),
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
        iat: i64,
        sub: String,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        apps: Vec<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        name: Option<String>,
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

    /// A valid HF-ID assertion: `sub` = badge, `aud`/`apps` = "hotel",
    /// `exp` = iat + 300s.
    fn valid_claims() -> TestClaims {
        TestClaims {
            iss: TEST_ISS.to_string(),
            aud: TEST_AUD.to_string(),
            iat: now(),
            exp: now() + 300,
            sub: "B123".to_string(),
            apps: vec!["hotel".to_string()],
            name: Some("Nok".to_string()),
        }
    }

    #[test]
    fn valid_assertion_yields_badge_and_name() {
        let token = sign(&valid_claims(), Some("hfid-key-1"));
        let id = verify_assertion(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect("valid assertion must verify");
        assert_eq!(id.badge, "B123");
        assert_eq!(id.display_name.as_deref(), Some("Nok"));
    }

    #[test]
    fn valid_assertion_without_name_leaves_display_none() {
        let mut claims = valid_claims();
        claims.name = None;
        let token = sign(&claims, Some("hfid-key-1"));
        let id = verify_assertion(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect("name is optional");
        assert_eq!(id.badge, "B123");
        assert!(id.display_name.is_none());
    }

    #[test]
    fn assertion_without_kid_still_verifies_via_fallback() {
        // Key rotation inside the cache TTL surfaces as a kid miss — the
        // fallback tries every RSA key, and the signature is the proof.
        let token = sign(&valid_claims(), None);
        let id = verify_assertion(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect("kid-less assertion must verify via try-all fallback");
        assert_eq!(id.badge, "B123");
    }

    #[test]
    fn expired_assertion_is_rejected() {
        let mut claims = valid_claims();
        claims.exp = now() - 3600; // well past the 60s default leeway
        let token = sign(&claims, Some("hfid-key-1"));
        let err = verify_assertion(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect_err("expired assertion must be rejected");
        assert!(matches!(err, HfIdAssertionError::Invalid(_)), "got: {err:?}");
    }

    #[test]
    fn wrong_audience_is_rejected() {
        let mut claims = valid_claims();
        claims.aud = "payroll".to_string();
        let token = sign(&claims, Some("hfid-key-1"));
        let err = verify_assertion(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect_err("wrong aud must be rejected");
        assert!(matches!(err, HfIdAssertionError::Invalid(_)), "got: {err:?}");
    }

    #[test]
    fn wrong_issuer_is_rejected() {
        let mut claims = valid_claims();
        claims.iss = "https://evil.example.org/oidc".to_string();
        let token = sign(&claims, Some("hfid-key-1"));
        let err = verify_assertion(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect_err("wrong iss must be rejected");
        assert!(matches!(err, HfIdAssertionError::Invalid(_)), "got: {err:?}");
    }

    #[test]
    fn assertion_without_hotel_grant_is_not_authorized() {
        // aud passes (a token could carry aud=hotel) but the apps grant list
        // lacks "hotel" — the defence-in-depth grant re-check must reject it.
        // Force the mismatch by verifying against a DIFFERENT expected app so
        // the enforced `aud` still matches the token while `apps` does not.
        let mut claims = valid_claims();
        claims.aud = "hotel".to_string();
        claims.apps = vec!["payroll".to_string()];
        let token = sign(&claims, Some("hfid-key-1"));
        let err = verify_assertion(&token, &test_jwks(), TEST_ISS, "hotel")
            .expect_err("missing hotel grant must be rejected");
        assert!(
            matches!(err, HfIdAssertionError::NotAuthorized),
            "got: {err:?}"
        );
    }

    #[test]
    fn garbage_assertion_is_rejected() {
        for garbage in ["", "not-a-jwt", "a.b.c", "!!!.???.###"] {
            let err = verify_assertion(garbage, &test_jwks(), TEST_ISS, TEST_AUD)
                .expect_err("garbage must be rejected");
            assert!(matches!(err, HfIdAssertionError::Invalid(_)), "got: {err:?}");
        }
    }

    #[test]
    fn assertion_with_empty_sub_is_rejected() {
        let mut claims = valid_claims();
        claims.sub = "   ".to_string();
        let token = sign(&claims, Some("hfid-key-1"));
        let err = verify_assertion(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect_err("empty sub must be rejected");
        assert!(matches!(err, HfIdAssertionError::Invalid(_)), "got: {err:?}");
    }

    #[test]
    fn jwks_without_rsa_keys_is_rejected() {
        let token = sign(&valid_claims(), Some("hfid-key-1"));
        let empty = JwkSet {
            keys: vec![Jwk {
                kid: Some("ec-key".to_string()),
                kty: "EC".to_string(),
                n: None,
                e: None,
            }],
        };
        let err = verify_assertion(&token, &empty, TEST_ISS, TEST_AUD)
            .expect_err("no-RSA JWKS must fail closed");
        assert!(matches!(err, HfIdAssertionError::Jwks(_)), "got: {err:?}");
    }

    #[test]
    fn certs_url_appends_oidc_jwks_and_trims_slash() {
        assert_eq!(
            certs_url("http://192.168.1.250"),
            "http://192.168.1.250/oidc/jwks"
        );
        assert_eq!(
            certs_url("http://192.168.1.250/"),
            "http://192.168.1.250/oidc/jwks"
        );
    }
}
