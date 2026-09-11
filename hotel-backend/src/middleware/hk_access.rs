//! Housekeeping-surface Cloudflare Access gate — the `/hk` maid identity.
//!
//! Employee-login plan Phase 4 (HF-erp `docs/employee-login-plan.md`, ADR 0002
//! there): the maid-facing housekeeping surface at `hotel.thehfhotel.org/hk*`
//! sits behind its OWN Cloudflare Access application with HF ID (LINE) as the
//! only IdP and an `HF ID grant: housekeeping` policy. Every request that
//! reaches the origin therefore carries a `Cf-Access-Jwt-Assertion` signed by
//! the SAME team domain as `middleware::cf_access`, but with the housekeeping
//! application's OWN AUD tag.
//!
//! This module is a THIRD JWKS consumer alongside `cf_access` /
//! `hfid_assertion` and follows their patterns; it reuses `cf_access`'s wire
//! types AND its single-slot team-JWKS cache (`cf_access::team_jwks` — same
//! issuer, same keys, different audience).
//!
//! ## Verification contract (fail closed)
//!
//! * AUD comes from the `CF_ACCESS_HK_AUD` env var. **Unset ⇒ every request is
//!   rejected 401** — the surface ships dark until the orchestrating Access
//!   wiring creates the app and provides the tag. There is deliberately NO
//!   baked-in default (the app does not exist yet).
//! * Signature RS256 against the team JWKS; `iss` = team domain; `exp` + `nbf`
//!   enforced (CF tokens always carry both).
//! * **Badge extraction** (the identity we stamp into `ht_hk_*` rows): HF ID
//!   mints id_tokens with `badge` + `apps` claims, and the CF IdP config
//!   forwards exactly those (`claims: ["apps", "badge"]` in HF-erp
//!   `infra/cloudflare/hf-id.ts`), which Cloudflare surfaces inside the
//!   application token's `custom` claim. We accept, in priority order:
//!   `custom.badge` → top-level `badge` → the local part of an
//!   `@emp.thehfhotel.org` synthetic email (HF ID's identity anchor,
//!   uppercased to match the canonical badge format). No badge ⇒ 401.
//! * **Grant re-check** (defence-in-depth on top of the edge policy): when an
//!   `apps` claim IS present (`custom.apps` or top-level), it must contain
//!   [`HK_GRANT`] (`"housekeeping"`) or [`RECEPTION_GRANT`] (`"reception"`) or
//!   the request is rejected 403. When the claim is absent the edge policy
//!   remains the (sole) authority — CF only lets granted employees through in
//!   the first place.
//!
//! ## Two roles on one surface (reception viewer, 2026-09)
//!
//! The `/hk` room board is also what reception wants at the desk: "which rooms
//! are clean right now". That is a READ of the same data, so rather than
//! duplicating the surface we admit the `reception` grant here as a READ-ONLY
//! viewer and resolve the capability ONCE, into [`HkIdentity::can_report`]:
//!
//! | `apps` claim contains | admitted | `can_report` |
//! |---|---|---|
//! | `housekeeping` (± `reception`) | yes | `true` |
//! | `reception` only | yes | `false` |
//! | neither | **no — 403** | — |
//! | claim absent entirely | yes (edge policy) | `true` (see below) |
//!
//! Downstream (`routes::hk`) never re-parses `apps`: it reads the resolved
//! boolean, which is what keeps the two mutations' refusal and the `/api/hk/me`
//! payload from drifting apart.
//!
//! **The absent-claim row is deliberately permissive, and it is a preserved
//! behavior, not a new one.** Before this change an apps-less token was
//! admitted with the full maid capability; making it `can_report = false` would
//! 403 a real maid the moment Cloudflare stopped forwarding the claim. The HF
//! ID IdP config forwards `apps` (`claims: ["apps", "badge"]`), so this row is
//! the degraded path, and on it the Access policy is — as it has always been —
//! the sole authority over what the token may do.
//!
//! ## Layering
//!
//! [`require_hk_access`] is a tower `from_fn` middleware that verifies the
//! assertion and injects the verified [`HkIdentity`] as a request extension;
//! `routes::hk` handlers read it via `Extension<HkIdentity>`. No DB, no
//! session, no auto-provisioning — the badge itself is the identity (unlike
//! the card-login flow, nothing here creates `ht_users` rows).

use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde_json::{json, Value};

use super::cf_access::{self, Jwk, JwkSet, CF_ACCESS_TEAM_DOMAIN};

/// The HF ID grant key gating this surface (Access policy
/// `HF ID grant: housekeeping`; managed per-employee in Employee Management).
///
/// It is the grant that carries the WRITE capability — see [`HkRole`].
pub const HK_GRANT: &str = "housekeeping";

/// The HF ID grant key that opens this surface READ-ONLY (Access policy
/// `HF ID grant: reception`; a NEW grant minted 2026-09 in HF ID's app
/// catalog — granted per employee in Employee Management, like housekeeping).
///
/// It admits the room board and refuses every mutation. Holding it ALONGSIDE
/// [`HK_GRANT`] changes nothing — housekeeping wins, because a capability
/// union of "full" and "read-only" is "full".
pub const RECEPTION_GRANT: &str = "reception";

/// Env var carrying the housekeeping Access application's AUD tag. No baked-in
/// default — unset means the surface is not wired yet and MUST fail closed.
pub const HK_AUD_ENV: &str = "CF_ACCESS_HK_AUD";

/// Domain suffix of HF ID's synthetic employee emails
/// (`<badge>@emp.thehfhotel.org` — the identity anchor Employee Management
/// auto-fills on approval). Used only as the LAST-RESORT badge derivation when
/// the explicit `badge` claim is missing.
const SYNTHETIC_EMAIL_DOMAIN: &str = "@emp.thehfhotel.org";

/// What a verified identity may DO on the `/hk` surface, resolved from the
/// `apps` grant list ONCE by [`role_from_apps`].
///
/// Two variants and no `None`: an identity that resolves to neither never
/// becomes an [`HkIdentity`] at all — it is [`HkAccessError::NotGranted`], and
/// making that a `Result` rather than a third variant is what stops a
/// downstream `match` from accidentally treating "no grant" as a weak role.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HkRole {
    /// Holds [`HK_GRANT`] — the maid surface exactly as it has always been:
    /// reads AND the cleaning / linen-shortage reports.
    Housekeeping,
    /// Holds [`RECEPTION_GRANT`] but NOT [`HK_GRANT`] — the desk's read-only
    /// room board. Every mutation is refused by `routes::hk`.
    ReceptionViewer,
}

impl HkRole {
    /// May this role file a report (`POST …/cleaning`, `POST
    /// …/linen-shortage`)? THE capability question this whole role exists to
    /// answer, and the only one `routes::hk` asks.
    pub fn can_report(self) -> bool {
        matches!(self, HkRole::Housekeeping)
    }
}

/// A verified maid identity extracted from a valid housekeeping-app assertion.
#[derive(Debug, Clone)]
pub struct HkIdentity {
    /// The employee badge (HF ID `sub`) — stamped into `ht_hk_*` rows.
    pub badge: String,
    /// Display name when the claims carry one (they usually don't — CF only
    /// forwards the configured `apps`/`badge` claims); `None` otherwise.
    pub display_name: Option<String>,
    /// The CF-asserted email (synthetic for LINE-only employees). Logged for
    /// audit; not used as an identity key.
    pub email: Option<String>,
    /// May this identity file reports? `true` for [`HK_GRANT`] holders (and
    /// for the apps-less edge-policy path), `false` for a
    /// [`RECEPTION_GRANT`]-only viewer — see [`HkRole::can_report`].
    ///
    /// Resolved HERE, once, from the verified claims. `routes::hk` reads this
    /// boolean and never re-parses `apps`, so there is exactly one place where
    /// a grant becomes a permission.
    pub can_report: bool,
}

/// Errors from the housekeeping Access verification path. Coarse on purpose —
/// the middleware collapses everything to 401 (403 for a missing grant); the
/// detail lands in logs only.
#[derive(Debug, thiserror::Error)]
pub enum HkAccessError {
    /// `CF_ACCESS_HK_AUD` is unset/blank — the surface is not wired yet.
    #[error("housekeeping Access application not configured (set {HK_AUD_ENV})")]
    NotConfigured,

    /// Neither the `Cf-Access-Jwt-Assertion` header nor the `CF_Authorization`
    /// cookie carried a token.
    #[error("no Cloudflare Access token on the request")]
    MissingToken,

    /// JWKS endpoint unreachable / non-JSON / empty (or no RSA) key set.
    #[error("JWKS fetch failed: {0}")]
    Jwks(String),

    /// Token failed signature / iss / aud / exp / nbf validation.
    #[error("token verification failed: {0}")]
    Invalid(String),

    /// Token verified but no badge could be derived from its claims.
    #[error("verified token carries no badge claim")]
    NoBadge,

    /// Token verified and carries an `apps` grant list, but the list contains
    /// NEITHER the `housekeeping` grant nor the `reception` one.
    #[error("verified token grants neither '{HK_GRANT}' nor '{RECEPTION_GRANT}'")]
    NotGranted,
}

impl HkAccessError {
    /// Wire mapping: missing grant is 403 (identity proven, permission
    /// absent); everything else is 401 (no usable identity). Bodies use the
    /// repo-wide `{"success": false, "error": ...}` envelope with a coarse
    /// message — verification detail stays in logs.
    fn status(&self) -> StatusCode {
        match self {
            HkAccessError::NotGranted => StatusCode::FORBIDDEN,
            _ => StatusCode::UNAUTHORIZED,
        }
    }
}

impl IntoResponse for HkAccessError {
    fn into_response(self) -> Response {
        let message = match self {
            // Deliberately UNCHANGED by the reception-viewer work. The body is
            // coarse by design (detail stays in logs, where the `Display` impl
            // now names both grants), and a no-grant identity's wire answer is
            // one of the behaviors this surface promises to keep byte-stable.
            HkAccessError::NotGranted => "housekeeping grant required",
            _ => "unauthenticated",
        };
        (
            self.status(),
            Json(json!({ "success": false, "error": message })),
        )
            .into_response()
    }
}

/// Resolve the expected housekeeping AUD from the env. `None` when unset or
/// blank (⇒ fail closed).
pub fn hk_aud() -> Option<String> {
    match std::env::var(HK_AUD_ENV) {
        Ok(value) if !value.trim().is_empty() => Some(value.trim().to_string()),
        _ => None,
    }
}

/// Read a claim that may live at the top level or nested inside CF's `custom`
/// claim object (Cloudflare surfaces forwarded IdP claims under `custom`;
/// checking both keeps us robust to either placement).
fn claim<'a>(claims: &'a Value, key: &str) -> Option<&'a Value> {
    claims
        .get("custom")
        .and_then(|custom| custom.get(key))
        .or_else(|| claims.get(key))
}

/// A claim value as a non-empty trimmed string. Numbers are stringified so a
/// numeric badge (e.g. `1188`) survives JSON number encoding.
fn value_as_string(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => {
            let trimmed = s.trim();
            (!trimmed.is_empty()).then(|| trimmed.to_string())
        }
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

/// Derive the badge from verified claims: explicit `badge` claim first
/// (custom/top-level), then the synthetic-email local part.
fn extract_badge(claims: &Value) -> Option<String> {
    if let Some(badge) = claim(claims, "badge").and_then(value_as_string) {
        return Some(badge);
    }
    // Last resort: HF ID's synthetic email is `<badge>@emp.thehfhotel.org`.
    // Uppercase the local part so a lowercased mailbox (`q1001@…`) still
    // matches the canonical badge format (`Q1001`); numeric badges are
    // unaffected. Non-synthetic emails (managers' Gmail) derive nothing.
    let email = claims.get("email").and_then(value_as_string)?;
    let lowered = email.to_lowercase();
    let local = lowered.strip_suffix(SYNTHETIC_EMAIL_DOMAIN)?;
    let local = local.trim();
    (!local.is_empty()).then(|| local.to_uppercase())
}

/// The `apps` grant list when present (custom/top-level). `None` when the
/// claim is absent entirely — the caller then trusts the edge policy.
fn extract_apps(claims: &Value) -> Option<Vec<String>> {
    claim(claims, "apps")?.as_array().map(|entries| {
        entries
            .iter()
            .filter_map(value_as_string)
            .collect::<Vec<_>>()
    })
}

/// PURE assertion verification against an already-fetched JWKS — the
/// unit-testable core (no network, no DB, no env), mirroring
/// `cf_access::verify_token` / `hfid_assertion::verify_assertion`.
pub fn verify_hk_token(
    token: &str,
    jwks: &JwkSet,
    issuer: &str,
    audience: &str,
) -> Result<HkIdentity, HkAccessError> {
    let header = decode_header(token).map_err(|err| HkAccessError::Invalid(err.to_string()))?;
    if header.alg != Algorithm::RS256 {
        return Err(HkAccessError::Invalid(format!(
            "unexpected alg {:?} (only RS256 accepted)",
            header.alg
        )));
    }

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[issuer]);
    validation.set_audience(&[audience]);
    validation.validate_nbf = true;
    // Same GHSA-h395-gr6q-cpjc defence-in-depth as `cf_access`: CF Access
    // tokens always carry exp + nbf, so requiring them costs nothing.
    validation.set_required_spec_claims(&["exp", "nbf", "iss", "aud"]);

    // kid-preferred key selection with try-all-RSA fallback — same rotation
    // policy as `cf_access` / `hfid_assertion`.
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
        return Err(HkAccessError::Jwks("JWKS contains no RSA keys".to_string()));
    }

    let mut last_err = HkAccessError::Invalid("no key verified the token".to_string());
    for key in candidates {
        let (Some(n), Some(e)) = (key.n.as_deref(), key.e.as_deref()) else {
            continue;
        };
        let decoding_key = match DecodingKey::from_rsa_components(n, e) {
            Ok(k) => k,
            Err(err) => {
                last_err = HkAccessError::Jwks(format!("bad JWK components: {err}"));
                continue;
            }
        };
        match decode::<Value>(token, &decoding_key, &validation) {
            Ok(data) => return identity_from_claims(&data.claims),
            Err(err) => {
                last_err = HkAccessError::Invalid(err.to_string());
            }
        }
    }
    Err(last_err)
}

/// THE GRANT → ROLE MAPPING. PURE, total, and table-tested below.
///
/// `None` means "neither grant" ⇒ [`HkAccessError::NotGranted`] (403).
///
/// Both comparisons are EXACT EQUALITY, never `starts_with` / `contains`, and
/// that is load-bearing twice over: `"housekeeping_admin"` has `"housekeeping"`
/// as a prefix (it widens branches, it is not a way in — see the test below),
/// and `"receptionist"` has `"reception"` as one. A substring "fix" here would
/// hand the surface to every badge whose grant merely looks similar.
///
/// Housekeeping wins over reception when both are present: the union of "full"
/// and "read-only" is "full", so an employee who covers both jobs keeps the
/// maid capability rather than being demoted by holding an extra grant.
fn role_from_apps(apps: &[String]) -> Option<HkRole> {
    if apps.iter().any(|app| app == HK_GRANT) {
        return Some(HkRole::Housekeeping);
    }
    if apps.iter().any(|app| app == RECEPTION_GRANT) {
        return Some(HkRole::ReceptionViewer);
    }
    None
}

/// Turn VERIFIED claims into an [`HkIdentity`]: badge required, grant
/// re-checked (and resolved to a capability) when the `apps` claim is present.
fn identity_from_claims(claims: &Value) -> Result<HkIdentity, HkAccessError> {
    // Absent `apps` ⇒ the edge policy is the sole authority, and the
    // capability is the pre-reception-viewer one (full). See the module doc's
    // table: making this row `false` would 403 a real maid the day Cloudflare
    // stopped forwarding the claim.
    let can_report = match extract_apps(claims) {
        Some(apps) => role_from_apps(&apps)
            .ok_or(HkAccessError::NotGranted)?
            .can_report(),
        None => true,
    };
    let badge = extract_badge(claims).ok_or(HkAccessError::NoBadge)?;
    Ok(HkIdentity {
        badge,
        display_name: claim(claims, "name").and_then(value_as_string),
        email: claims.get("email").and_then(value_as_string),
        can_report,
    })
}

/// Full production verification: resolve the AUD from the env (fail closed
/// when unset), fetch the cached team JWKS, verify, extract the identity.
pub async fn verify_hk_assertion(token: &str) -> Result<HkIdentity, HkAccessError> {
    let audience = hk_aud().ok_or(HkAccessError::NotConfigured)?;
    let jwks = cf_access::team_jwks()
        .await
        .map_err(|err| HkAccessError::Jwks(err.to_string()))?;
    verify_hk_token(token, &jwks, CF_ACCESS_TEAM_DOMAIN, &audience)
}

/// Tower middleware for the `/api/hk/*` router: verify the CF Access
/// assertion and inject the [`HkIdentity`] extension, or answer 401/403.
pub async fn require_hk_access(mut req: Request, next: Next) -> Response {
    let jar = CookieJar::from_headers(req.headers());
    let Some(token) = cf_access::extract_token(req.headers(), &jar) else {
        tracing::debug!("hk_access: request without CF Access token rejected");
        return HkAccessError::MissingToken.into_response();
    };

    match verify_hk_assertion(&token).await {
        Ok(identity) => {
            tracing::debug!(
                badge = %identity.badge,
                can_report = identity.can_report,
                "hk_access: verified /hk identity"
            );
            req.extensions_mut().insert(identity);
            next.run(req).await
        }
        Err(err) => {
            tracing::warn!(error = %err, "hk_access: rejected request");
            err.into_response()
        }
    }
}

// =============================================================================
// Tests — static JWKS + RSA key, NO network. Mirrors `cf_access::tests`.
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{encode, EncodingKey, Header};

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

    const TEST_ISS: &str = CF_ACCESS_TEAM_DOMAIN;
    const TEST_AUD: &str = "hk-test-aud-tag";

    fn test_jwks() -> JwkSet {
        JwkSet {
            keys: vec![Jwk {
                kid: Some("hk-key-1".to_string()),
                kty: "RSA".to_string(),
                n: Some(TEST_RSA_N.to_string()),
                e: Some(TEST_RSA_E.to_string()),
            }],
        }
    }

    fn now() -> i64 {
        chrono::Utc::now().timestamp()
    }

    fn sign(claims: &Value, kid: Option<&str>) -> String {
        let mut header = Header::new(Algorithm::RS256);
        header.kid = kid.map(str::to_string);
        let key = EncodingKey::from_rsa_pem(TEST_RSA_PRIVATE_PEM.as_bytes())
            .expect("test key must parse");
        encode(&header, claims, &key).expect("signing must succeed")
    }

    /// The shape CF Access emits for an HF ID login with the IdP configured
    /// to forward `["apps", "badge"]`: standard claims + `custom` object.
    fn valid_claims() -> Value {
        json!({
            "iss": TEST_ISS,
            "aud": TEST_AUD,
            "exp": now() + 3600,
            "nbf": now() - 60,
            "iat": now(),
            "email": "q1001@emp.thehfhotel.org",
            "custom": { "badge": "Q1001", "apps": ["housekeeping", "hotel"] },
        })
    }

    #[test]
    fn valid_token_with_custom_badge_yields_identity() {
        let token = sign(&valid_claims(), Some("hk-key-1"));
        let id = verify_hk_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect("valid token must verify");
        assert_eq!(id.badge, "Q1001");
        assert_eq!(id.email.as_deref(), Some("q1001@emp.thehfhotel.org"));
        assert!(id.display_name.is_none());
        assert!(
            id.can_report,
            "a `housekeeping` holder keeps the full maid capability"
        );
    }

    // ---- the grant → role mapping ---------------------------------------

    /// The whole mapping as one table. PURE — no JWT, no JWKS, no env.
    ///
    /// The near-miss rows are the point: `housekeeping_admin` and
    /// `receptionist` each have a real grant as a PREFIX, and a `starts_with` /
    /// `contains` implementation would silently admit both.
    #[test]
    fn role_from_apps_maps_every_grant_combination() {
        fn apps(list: &[&str]) -> Vec<String> {
            list.iter().map(|s| (*s).to_string()).collect()
        }

        for (list, expected, why) in [
            (&["housekeeping"][..], Some(HkRole::Housekeeping), "the maid"),
            (
                &["reception"][..],
                Some(HkRole::ReceptionViewer),
                "the desk's read-only board",
            ),
            (
                &["reception", "housekeeping"][..],
                Some(HkRole::Housekeeping),
                "both grants ⇒ full: the union of full and read-only is full",
            ),
            (
                &["housekeeping", "reception"][..],
                Some(HkRole::Housekeeping),
                "…and the list ORDER must not decide it",
            ),
            (
                &["hotel", "reception", "payroll"][..],
                Some(HkRole::ReceptionViewer),
                "unrelated grants alongside are ignored",
            ),
            (&[][..], None, "an empty grant list opens nothing"),
            (
                &["hotel", "payroll"][..],
                None,
                "grants for other apps open nothing",
            ),
            (
                &["housekeeping_admin"][..],
                None,
                "PREFIX TRAP: admin widens branches, it is not a way in",
            ),
            (
                &["receptionist"][..],
                None,
                "PREFIX TRAP: `reception` must match exactly",
            ),
            (
                &["Reception"][..],
                None,
                "grant keys are case-sensitive, like every other HF ID grant",
            ),
            (
                &["reception "][..],
                None,
                "no trimming: the claim list is emitted by HF ID, not typed",
            ),
        ] {
            assert_eq!(role_from_apps(&apps(list)), expected, "{list:?}: {why}");
        }

        assert!(HkRole::Housekeeping.can_report());
        assert!(
            !HkRole::ReceptionViewer.can_report(),
            "the viewer is READ-ONLY — this is the one bit `routes::hk` gates on"
        );
    }

    /// A `reception`-only badge is ADMITTED (200-able) but carries
    /// `can_report = false`. Both halves matter: refusing it would leave the
    /// desk without a board, and admitting it with `can_report = true` would
    /// hand reception the maid's writeback.
    #[test]
    fn reception_only_grant_is_admitted_read_only() {
        let mut claims = valid_claims();
        claims["custom"]["apps"] = json!(["hotel", "reception"]);
        let token = sign(&claims, Some("hk-key-1"));
        let id = verify_hk_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect("a reception badge must reach the room board");
        assert_eq!(id.badge, "Q1001");
        assert!(
            !id.can_report,
            "reception is a VIEWER — `routes::hk` refuses its mutations"
        );
    }

    /// Someone who works both jobs is a maid, not a demoted one.
    #[test]
    fn holding_both_grants_keeps_the_full_capability() {
        let mut claims = valid_claims();
        claims["custom"]["apps"] = json!(["reception", "housekeeping"]);
        let token = sign(&claims, Some("hk-key-1"));
        let id = verify_hk_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect("both grants must verify");
        assert!(id.can_report, "housekeeping wins over reception");
    }

    #[test]
    fn top_level_badge_and_apps_also_accepted() {
        let mut claims = valid_claims();
        claims.as_object_mut().unwrap().remove("custom");
        claims["badge"] = json!("1188");
        claims["apps"] = json!(["housekeeping"]);
        let token = sign(&claims, Some("hk-key-1"));
        let id = verify_hk_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect("top-level claim placement must verify");
        assert_eq!(id.badge, "1188");
    }

    #[test]
    fn numeric_badge_claim_is_stringified() {
        let mut claims = valid_claims();
        claims["custom"]["badge"] = json!(1188);
        let token = sign(&claims, Some("hk-key-1"));
        let id = verify_hk_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect("numeric badge must verify");
        assert_eq!(id.badge, "1188");
    }

    #[test]
    fn synthetic_email_fallback_derives_uppercased_badge() {
        // No badge claim anywhere — the synthetic-email local part is the
        // last-resort derivation, uppercased to the canonical badge format.
        let mut claims = valid_claims();
        claims.as_object_mut().unwrap().remove("custom");
        let token = sign(&claims, Some("hk-key-1"));
        let id = verify_hk_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect("synthetic email fallback must verify");
        assert_eq!(id.badge, "Q1001");
    }

    #[test]
    fn non_synthetic_email_without_badge_is_rejected() {
        let mut claims = valid_claims();
        claims.as_object_mut().unwrap().remove("custom");
        claims["email"] = json!("winut.hf@gmail.com");
        let token = sign(&claims, Some("hk-key-1"));
        let err = verify_hk_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect_err("badge-less manager token must be rejected");
        assert!(matches!(err, HkAccessError::NoBadge), "got: {err:?}");
    }

    #[test]
    fn apps_claim_without_housekeeping_grant_is_rejected() {
        let mut claims = valid_claims();
        claims["custom"]["apps"] = json!(["hotel", "payroll"]);
        let token = sign(&claims, Some("hk-key-1"));
        let err = verify_hk_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect_err("missing housekeeping grant must be rejected");
        assert!(matches!(err, HkAccessError::NotGranted), "got: {err:?}");
    }

    /// **`housekeeping_admin` is not a way in.** It WIDENS which branches an
    /// employee may work (`hfid_location::HK_ADMIN_GRANT` → every branch in
    /// `HK_BRANCHES`); it never substitutes for the grant that opens the
    /// surface. Someone holding admin WITHOUT `housekeeping` must be refused
    /// here — before any branch logic runs — and this is the layer that does
    /// it, both at the Cloudflare edge policy (`HF ID grant: housekeeping`)
    /// and in this re-check behind it.
    ///
    /// The trap this pins is textual: `"housekeeping_admin"` has
    /// `"housekeeping"` as a PREFIX. The comparison above is exact equality,
    /// so it refuses — but a future `starts_with` / `contains` "fix" would
    /// silently hand the whole maid surface to every admin-only badge. That is
    /// the regression this test exists to catch.
    #[test]
    fn admin_grant_alone_does_not_open_the_surface() {
        for apps in [
            json!(["housekeeping_admin"]),
            json!(["hotel", "housekeeping_admin"]),
        ] {
            let mut claims = valid_claims();
            claims["custom"]["apps"] = apps.clone();
            let token = sign(&claims, Some("hk-key-1"));
            let err = verify_hk_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
                .expect_err("admin without the housekeeping grant must be refused");
            assert!(matches!(err, HkAccessError::NotGranted), "{apps}: {err:?}");
            assert_eq!(
                err.status(),
                StatusCode::FORBIDDEN,
                "{apps}: identity is proven, the permission is simply absent"
            );
        }

        // …and holding BOTH is the ordinary admin: admitted here, then widened
        // downstream by `routes::hk`. This is the pairing the owner ticks.
        let mut claims = valid_claims();
        claims["custom"]["apps"] = json!(["hotel", "housekeeping", "housekeeping_admin"]);
        let token = sign(&claims, Some("hk-key-1"));
        let id = verify_hk_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect("housekeeping + admin must verify");
        assert_eq!(id.badge, "Q1001");
        assert!(id.can_report);

        // The reception-viewer era adds one row to the same rule: admin PLUS
        // reception is admitted — but by the RECEPTION grant, read-only. The
        // branch-widening admin grant must not smuggle in a write capability
        // it never carried.
        let mut claims = valid_claims();
        claims["custom"]["apps"] = json!(["housekeeping_admin", "reception"]);
        let token = sign(&claims, Some("hk-key-1"));
        let id = verify_hk_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect("reception opens the board even next to an admin grant");
        assert!(
            !id.can_report,
            "`housekeeping_admin` widens branches; it never grants reporting"
        );
    }

    #[test]
    fn absent_apps_claim_defers_to_edge_policy() {
        // CF may not forward the apps claim at all; the Access policy
        // (`HF ID grant: housekeeping`) is then the sole authority and the
        // token still verifies.
        let mut claims = valid_claims();
        claims["custom"].as_object_mut().unwrap().remove("apps");
        let token = sign(&claims, Some("hk-key-1"));
        let id = verify_hk_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect("apps-less token must verify (edge policy authoritative)");
        assert_eq!(id.badge, "Q1001");
        // PRESERVED, not new: this is the capability an apps-less token has
        // always had. Flipping it to `false` would 403 a real maid the day
        // Cloudflare stopped forwarding `apps`.
        assert!(
            id.can_report,
            "the apps-less path keeps the pre-reception-viewer capability"
        );
    }

    /// The reception grant works from the TOP LEVEL too, not only inside
    /// `custom` — same either-placement robustness the badge claim has.
    #[test]
    fn top_level_reception_grant_is_admitted_read_only() {
        let mut claims = valid_claims();
        claims.as_object_mut().unwrap().remove("custom");
        claims["badge"] = json!("1188");
        claims["apps"] = json!(["reception"]);
        let token = sign(&claims, Some("hk-key-1"));
        let id = verify_hk_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect("top-level reception grant must verify");
        assert_eq!(id.badge, "1188");
        assert!(!id.can_report);
    }

    #[test]
    fn expired_token_is_rejected() {
        let mut claims = valid_claims();
        claims["exp"] = json!(now() - 3600);
        let token = sign(&claims, Some("hk-key-1"));
        let err = verify_hk_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect_err("expired token must be rejected");
        assert!(matches!(err, HkAccessError::Invalid(_)), "got: {err:?}");
    }

    #[test]
    fn wrong_audience_is_rejected() {
        let mut claims = valid_claims();
        claims["aud"] = json!("some-other-app-aud");
        let token = sign(&claims, Some("hk-key-1"));
        let err = verify_hk_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect_err("wrong aud must be rejected");
        assert!(matches!(err, HkAccessError::Invalid(_)), "got: {err:?}");
    }

    #[test]
    fn wrong_issuer_is_rejected() {
        let mut claims = valid_claims();
        claims["iss"] = json!("https://evil.cloudflareaccess.com");
        let token = sign(&claims, Some("hk-key-1"));
        let err = verify_hk_token(&token, &test_jwks(), TEST_ISS, TEST_AUD)
            .expect_err("wrong iss must be rejected");
        assert!(matches!(err, HkAccessError::Invalid(_)), "got: {err:?}");
    }

    #[test]
    fn garbage_token_is_rejected() {
        for garbage in ["", "not-a-jwt", "a.b.c", "!!!.???.###"] {
            let err = verify_hk_token(garbage, &test_jwks(), TEST_ISS, TEST_AUD)
                .expect_err("garbage must be rejected");
            assert!(matches!(err, HkAccessError::Invalid(_)), "got: {err:?}");
        }
    }

    #[test]
    fn jwks_without_rsa_keys_is_rejected() {
        let token = sign(&valid_claims(), Some("hk-key-1"));
        let empty = JwkSet {
            keys: vec![Jwk {
                kid: Some("ec-key".to_string()),
                kty: "EC".to_string(),
                n: None,
                e: None,
            }],
        };
        let err = verify_hk_token(&token, &empty, TEST_ISS, TEST_AUD)
            .expect_err("no-RSA JWKS must fail closed");
        assert!(matches!(err, HkAccessError::Jwks(_)), "got: {err:?}");
    }

    #[test]
    fn not_granted_maps_to_403_everything_else_401() {
        assert_eq!(HkAccessError::NotGranted.status(), StatusCode::FORBIDDEN);
        for err in [
            HkAccessError::NotConfigured,
            HkAccessError::MissingToken,
            HkAccessError::Jwks("x".into()),
            HkAccessError::Invalid("x".into()),
            HkAccessError::NoBadge,
        ] {
            assert_eq!(err.status(), StatusCode::UNAUTHORIZED);
        }
    }
}
