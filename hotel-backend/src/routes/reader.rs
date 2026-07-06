//! NFC staff-card login — PUBLIC reader endpoints (central-pairing flow).
//!
//! The physical reader now posts taps to the CENTRAL HF-ID service, not to this
//! PMS. So there is no local `scan` endpoint any more: the login screen pairs
//! to a reader through HF-ID (via this PMS) and long-polls HF-ID for the tap.
//! Both routes here are mounted OUTSIDE `require_auth` (a login screen has no
//! session yet), wired in `main.rs`. The session-minting counterpart lives in
//! `routes::auth::card_login` (also public).
//!
//! | Method | Path               | Auth                    | Purpose                                   |
//! |--------|--------------------|-------------------------|-------------------------------------------|
//! | POST   | /api/reader/claim  | none (mints cookie)     | Pair this browser to a reader via HF-ID   |
//! | GET    | /api/reader/wait   | `reader_claim` cookie   | Long-poll HF-ID → verify → login_token    |
//!
//! ## The claim/wait race it closes
//!
//! `claim` calls HF-ID `/api/private/reader/claim {reader_id, app:"hotel"}`,
//! gets back a central `claim_token`, and binds THIS browser to it via an
//! HttpOnly `reader_claim` cookie (the cookie holds a local handle; the central
//! token stays server-side — see `service::reader::ReaderStore::put_claim`).
//! `wait` resolves the cookie → central claim_token → HF-ID
//! `/api/private/reader/wait {claim_token}`:
//!   * **200** `{assertion}` → verify the RS256 assertion
//!     (`middleware::hfid_assertion`), map `sub`=badge → `ht_users` (auto-
//!     provision), stash a one-time `login_token` (pending → delivered) and
//!     hand it to the paired browser.
//!   * **204** timeout → 204 (client re-polls).
//!   * **403** not authorized → 403.
//! The `login_token` is delivered ONCE and consumed once again at `card-login`,
//! TTL-bounded end to end — a token minted for one browser can't be hijacked by
//! an unrelated client polling `wait` (it never sees the cookie's pairing).

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::{Deserialize, Serialize};

use crate::middleware::hfid_assertion::verify_hfid_assertion;
use crate::routes::mode::AppState;
use crate::service::reader::{find_or_provision_user_by_badge, WaitOutcome};

/// Cookie binding a browser to its HF-ID pairing (set by `claim`, read by
/// `wait`). Holds a LOCAL handle that maps server-side to the central
/// claim_token.
pub const READER_CLAIM_COOKIE: &str = "reader_claim";

/// Reader-claim cookie lifetime (10 min) — matches the store's `CLAIM_TTL` so
/// the browser stops presenting a claim about when the server forgets it.
const READER_CLAIM_MAX_AGE_SECS: i64 = 10 * 60;

// =============================================================================
// Wire DTOs
// =============================================================================

/// `POST /api/reader/claim` body.
#[derive(Debug, Deserialize)]
pub struct ClaimRequest {
    pub reader_id: String,
}

/// `GET /api/reader/wait` success body (shape unchanged — the frontend consumes
/// `{login_token}` exactly as before).
#[derive(Debug, Serialize)]
struct WaitResponse {
    login_token: String,
}

/// Uniform error body — a single machine-readable `error` code (same shape as
/// `routes::auth::ErrorResponse`).
#[derive(Debug, Serialize)]
struct ReaderError {
    error: &'static str,
}

/// Build a `(StatusCode, Json)` error response.
fn err(status: StatusCode, code: &'static str) -> Response {
    (status, Json(ReaderError { error: code })).into_response()
}

// =============================================================================
// Handlers
// =============================================================================

/// `POST /api/reader/claim` — PUBLIC. Claim a pairing for `reader_id` against
/// central HF-ID and Set-Cookie the resulting `reader_claim` handle.
pub async fn claim(
    State(state): State<AppState>,
    jar: CookieJar,
    headers: HeaderMap,
    Json(body): Json<ClaimRequest>,
) -> Result<(CookieJar, StatusCode), Response> {
    let reader_id = body.reader_id.trim();
    if reader_id.is_empty() {
        return Err(err(StatusCode::BAD_REQUEST, "missing_reader_id"));
    }

    // Claim the pairing centrally. Any failure (transport, non-2xx, or the
    // fail-closed Null client when unconfigured) → 502; the browser surfaces a
    // "can't reach the reader" message and retries.
    let central_claim_token = match state.reader.hfid.claim(reader_id).await {
        Ok(token) => token,
        Err(error) => {
            tracing::warn!(error = %error, reader_id = %reader_id, "reader/claim: central claim failed");
            return Err(err(StatusCode::BAD_GATEWAY, "claim_failed"));
        }
    };

    // Map the central token to a fresh LOCAL cookie handle (kept server-side).
    let cookie_token = state.reader.store.put_claim(&central_claim_token);
    let cookie = build_reader_claim_cookie(cookie_token, is_https_request(&headers));
    Ok((jar.add(cookie), StatusCode::OK))
}

/// `GET /api/reader/wait` — PUBLIC (requires the `reader_claim` cookie).
///
/// Resolve the cookie → central claim_token, long-poll HF-ID for the next tap,
/// and on an authorized tap verify the assertion, map/auto-provision the badge,
/// and deliver a one-time `login_token`. 204 on timeout (client re-polls);
/// 403 when a tap was not authorized; 401 when the claim cookie is
/// missing/expired (client re-claims).
pub async fn wait(State(state): State<AppState>, jar: CookieJar) -> Response {
    let cookie_token = match jar.get(READER_CLAIM_COOKIE).map(|c| c.value().to_string()) {
        Some(token) => token,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };
    let central_claim_token = match state.reader.store.resolve_claim(&cookie_token) {
        Some(token) => token,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    // Long-poll HF-ID. This request blocks for the central wait budget.
    let assertion = match state.reader.hfid.wait(&central_claim_token).await {
        Ok(WaitOutcome::Authorized(assertion)) => assertion,
        Ok(WaitOutcome::Timeout) => return StatusCode::NO_CONTENT.into_response(),
        Ok(WaitOutcome::NotAuthorized) => {
            return err(StatusCode::FORBIDDEN, "not_authorized");
        }
        Err(error) => {
            tracing::warn!(error = %error, "reader/wait: central wait failed");
            return err(StatusCode::BAD_GATEWAY, "wait_failed");
        }
    };

    // Verify the RS256 assertion (signature, iss, aud=hotel, exp, apps⊇hotel)
    // and extract the badge. Any verification failure → 403; detail logs only.
    let identity = match verify_hfid_assertion(&assertion, &state.reader.base_url).await {
        Ok(identity) => identity,
        Err(error) => {
            tracing::warn!(error = %error, "reader/wait: HF-ID assertion rejected");
            return err(StatusCode::FORBIDDEN, "assertion_invalid");
        }
    };

    // Map / auto-provision the local user by badge.
    let user_id = match find_or_provision_user_by_badge(
        &state.new_pool,
        &identity.badge,
        identity.display_name.as_deref(),
    )
    .await
    {
        Ok(id) => id,
        Err(error) => {
            tracing::error!(error = %error, badge = %identity.badge, "reader/wait: user provision failed");
            return err(StatusCode::INTERNAL_SERVER_ERROR, "provision_failed");
        }
    };

    // Stash a one-time pending login and immediately move it pending→delivered
    // so `POST /api/auth/card-login` can consume it. Keyed by the central
    // claim_token (unique per pairing); a browser only runs one `wait` at a
    // time, so there is no put/take contention.
    state.reader.store.put_pending(&central_claim_token, user_id);
    match state
        .reader
        .store
        .take_pending_login_token(&central_claim_token)
    {
        Some(login_token) => Json(WaitResponse { login_token }).into_response(),
        None => {
            // Should be unreachable — we just stashed it. Fail safe rather than
            // hand back an empty token.
            tracing::error!("reader/wait: freshly-stashed login token vanished before delivery");
            err(StatusCode::INTERNAL_SERVER_ERROR, "deliver_failed")
        }
    }
}

// =============================================================================
// Helpers
// =============================================================================

/// Build the `reader_claim` cookie. Same flags as the session cookie
/// (`HttpOnly`, `SameSite=Lax`, `Path=/`, `Secure` only over HTTPS) so it
/// behaves identically behind the proxy; shorter `Max-Age` (claim TTL).
fn build_reader_claim_cookie(token: String, secure: bool) -> Cookie<'static> {
    Cookie::build((READER_CLAIM_COOKIE, token))
        .http_only(true)
        .secure(secure)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(cookie::time::Duration::seconds(READER_CLAIM_MAX_AGE_SECS))
        .build()
}

/// True when the request arrived over HTTPS (drives the `Secure` flag). Same
/// `X-Forwarded-Proto` detection as `routes::auth::is_https_request` — kept
/// private here so the module doesn't reach into handler internals.
fn is_https_request(headers: &HeaderMap) -> bool {
    headers
        .get("x-forwarded-proto")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.eq_ignore_ascii_case("https"))
        .unwrap_or(false)
}

// =============================================================================
// Tests (pure pieces — the store + assertion verification live in
// service::reader / middleware::hfid_assertion)
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wait_response_wire_shape() {
        let value = serde_json::to_value(WaitResponse {
            login_token: "abc".into(),
        })
        .unwrap();
        assert_eq!(value.get("login_token").and_then(|v| v.as_str()), Some("abc"));
        assert_eq!(value.as_object().unwrap().len(), 1);
    }

    #[test]
    fn reader_error_uses_single_error_key() {
        let value = serde_json::to_value(ReaderError {
            error: "not_authorized",
        })
        .unwrap();
        assert_eq!(
            value.get("error").and_then(|v| v.as_str()),
            Some("not_authorized")
        );
        assert_eq!(value.as_object().unwrap().len(), 1);
    }

    #[test]
    fn reader_claim_cookie_flags() {
        let c = build_reader_claim_cookie("tok".to_string(), true);
        assert_eq!(c.name(), READER_CLAIM_COOKIE);
        assert_eq!(c.http_only(), Some(true));
        assert_eq!(c.secure(), Some(true));
        assert_eq!(c.same_site(), Some(SameSite::Lax));
        assert_eq!(c.path(), Some("/"));
        assert_eq!(
            c.max_age().map(|d| d.whole_seconds()),
            Some(READER_CLAIM_MAX_AGE_SECS)
        );
        // No TLS → no Secure (local dev over http would otherwise drop the cookie).
        let insecure = build_reader_claim_cookie("tok".to_string(), false);
        assert_eq!(insecure.secure(), Some(false));
    }

    #[test]
    fn is_https_only_true_for_forwarded_https() {
        let mut headers = HeaderMap::new();
        assert!(!is_https_request(&headers));
        headers.insert("x-forwarded-proto", "https".parse().unwrap());
        assert!(is_https_request(&headers));
        headers.insert("x-forwarded-proto", "HTTPS".parse().unwrap());
        assert!(is_https_request(&headers));
        headers.insert("x-forwarded-proto", "http".parse().unwrap());
        assert!(!is_https_request(&headers));
    }
}
