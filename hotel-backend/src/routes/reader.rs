//! NFC staff-card login — PUBLIC reader endpoints.
//!
//! A reader device has no session cookie, so all three routes here are
//! mounted OUTSIDE `require_auth` (like `/api/auth/*` and `/health`). They
//! are wired in `main.rs`. The session-minting counterpart lives in
//! `routes::auth::card_login` (also public) so it can reuse the exact
//! `mint_session_for` + `build_session_cookie` path as password / CF login.
//!
//! | Method | Path               | Auth                    | Purpose                              |
//! |--------|--------------------|-------------------------|--------------------------------------|
//! | POST   | /api/reader/scan   | `X-Reader-Secret` header| Resolve a tapped UID → stash login   |
//! | POST   | /api/reader/claim  | none (mints cookie)     | Pair this browser to a reader_id     |
//! | GET    | /api/reader/wait   | `reader_claim` cookie   | Long-poll for a delivered login_token|
//!
//! ## Reader firmware contract
//!
//! `scan` answers **200** on an authorized tap (reader single-beeps) and any
//! **non-2xx** on unknown UID / not authorized / resolve or secret failure
//! (the firmware treats non-2xx as "warn" → double-beep). The body is
//! informational only.
//!
//! ## The claim/wait race it closes
//!
//! `claim` binds a browser to a `reader_id` via an HttpOnly `reader_claim`
//! cookie BEFORE it starts waiting. `wait` only ever hands the pending
//! `login_token` to a browser that proved (via the cookie) it owns that
//! reader_id — so a `login_token` minted by a tap can't be hijacked by an
//! unrelated client polling `wait`. The `login_token` is itself delivered
//! ONCE and consumed once again at `card-login`, TTL-bounded end to end.

use std::time::Duration;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use serde::{Deserialize, Serialize};

use crate::routes::mode::AppState;
use crate::service::reader::{authorize, ct_eq, find_or_provision_user_by_badge};

/// Cookie binding a browser to a `reader_id` (set by `claim`, read by `wait`).
pub const READER_CLAIM_COOKIE: &str = "reader_claim";

/// Reader-claim cookie lifetime (10 min) — matches the store's `CLAIM_TTL` so
/// the browser stops presenting a claim about when the server forgets it.
const READER_CLAIM_MAX_AGE_SECS: i64 = 10 * 60;

/// `wait` long-poll budget: 50 ticks × 500ms ≈ 25s, then 204 (client re-polls).
const WAIT_TICKS: u32 = 50;
const WAIT_TICK: Duration = Duration::from_millis(500);

// =============================================================================
// Wire DTOs
// =============================================================================

/// `POST /api/reader/scan` body. `ts` is the reader's optional millis clock —
/// accepted for future replay-window checks, unused today.
#[derive(Debug, Deserialize)]
pub struct ScanRequest {
    pub uid: String,
    pub reader: String,
    #[serde(default)]
    pub ts: Option<i64>,
}

/// 200 body — reader single-beeps.
#[derive(Debug, Serialize)]
pub struct ScanOk {
    pub ok: bool,
    pub display_name: Option<String>,
}

/// non-2xx body — reader double-beeps. `reason` is a stable machine code.
#[derive(Debug, Serialize)]
pub struct ScanErr {
    pub ok: bool,
    pub reason: &'static str,
}

impl ScanErr {
    fn of(reason: &'static str) -> Json<Self> {
        Json(Self { ok: false, reason })
    }
}

/// `POST /api/reader/claim` body.
#[derive(Debug, Deserialize)]
pub struct ClaimRequest {
    pub reader_id: String,
}

/// `GET /api/reader/wait` success body.
#[derive(Debug, Serialize)]
struct WaitResponse {
    login_token: String,
}

// =============================================================================
// Handlers
// =============================================================================

/// `POST /api/reader/scan` — PUBLIC (device-authenticated by header secret).
///
/// Gate on `X-Reader-Secret` (constant-time; fail closed when unconfigured),
/// resolve the UID centrally, authorize, map/auto-provision the `ht_users`
/// row by badge, then stash a one-time pending login keyed by `reader`.
pub async fn scan(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<ScanRequest>,
) -> Result<(StatusCode, Json<ScanOk>), (StatusCode, Json<ScanErr>)> {
    // 1. Device secret gate. A missing config OR a mismatch rejects.
    let presented = headers
        .get("x-reader-secret")
        .and_then(|v| v.to_str().ok());
    if !secret_ok(state.reader.reader_secret.as_deref(), presented) {
        return Err((StatusCode::UNAUTHORIZED, ScanErr::of("bad_reader_secret")));
    }

    // 2. Central resolve. Any failure → 403 (reader double-beeps); detail logs only.
    let resolved = match state.reader.resolve.resolve(&body.uid).await {
        Ok(r) => r,
        Err(err) => {
            tracing::warn!(error = %err, reader = %body.reader, "reader/scan: central resolve failed");
            return Err((StatusCode::FORBIDDEN, ScanErr::of("resolve_failed")));
        }
    };

    // 3. Authorize: found & active & !pending & carries the required app grant.
    if !authorize(&resolved, &state.reader.required_app) {
        return Err((StatusCode::FORBIDDEN, ScanErr::of("not_authorized")));
    }
    let badge = match resolved.badge.as_deref() {
        Some(b) if !b.trim().is_empty() => b,
        _ => {
            tracing::warn!(reader = %body.reader, "reader/scan: authorized but resolve carried no badge");
            return Err((StatusCode::FORBIDDEN, ScanErr::of("no_badge")));
        }
    };

    // 4. Map / auto-provision the local user by badge.
    let user_id = match find_or_provision_user_by_badge(
        &state.new_pool,
        badge,
        resolved.display_name.as_deref(),
    )
    .await
    {
        Ok(id) => id,
        Err(err) => {
            tracing::error!(error = %err, badge = %badge, "reader/scan: user provision failed");
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                ScanErr::of("provision_failed"),
            ));
        }
    };

    // 5. Stash a one-time pending login for this reader (overwrites any prior).
    state.reader.store.put_pending(&body.reader, user_id);

    Ok((
        StatusCode::OK,
        Json(ScanOk {
            ok: true,
            display_name: resolved.display_name,
        }),
    ))
}

/// `POST /api/reader/claim` — PUBLIC. Bind this browser to `reader_id` and
/// Set-Cookie the resulting `reader_claim` token.
pub async fn claim(
    State(state): State<AppState>,
    jar: CookieJar,
    headers: HeaderMap,
    Json(body): Json<ClaimRequest>,
) -> Result<(CookieJar, StatusCode), (StatusCode, Json<ScanErr>)> {
    let reader_id = body.reader_id.trim();
    if reader_id.is_empty() {
        return Err((StatusCode::BAD_REQUEST, ScanErr::of("missing_reader_id")));
    }
    let claim_token = state.reader.store.put_claim(reader_id);
    let cookie = build_reader_claim_cookie(claim_token, is_https_request(&headers));
    Ok((jar.add(cookie), StatusCode::OK))
}

/// `GET /api/reader/wait` — PUBLIC (requires the `reader_claim` cookie).
///
/// Resolve the cookie → reader_id, then long-poll the pending store for ~25s.
/// Delivers the `login_token` ONCE (removed on read) then returns 200; on
/// timeout 204 (client re-polls); missing/invalid cookie 401.
pub async fn wait(State(state): State<AppState>, jar: CookieJar) -> Response {
    let claim_token = match jar.get(READER_CLAIM_COOKIE).map(|c| c.value().to_string()) {
        Some(token) => token,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };
    let reader_id = match state.reader.store.resolve_claim(&claim_token) {
        Some(id) => id,
        None => return StatusCode::UNAUTHORIZED.into_response(),
    };

    for _ in 0..WAIT_TICKS {
        if let Some(login_token) = state.reader.store.take_pending_login_token(&reader_id) {
            return Json(WaitResponse { login_token }).into_response();
        }
        tokio::time::sleep(WAIT_TICK).await;
    }
    StatusCode::NO_CONTENT.into_response()
}

// =============================================================================
// Helpers
// =============================================================================

/// Reader secret gate: true only when a secret is configured AND the presented
/// header matches it constant-time. Unconfigured (`None`) or absent header ⇒
/// false (fail closed).
fn secret_ok(configured: Option<&str>, presented: Option<&str>) -> bool {
    match (configured, presented) {
        (Some(cfg), Some(pres)) => ct_eq(cfg.as_bytes(), pres.as_bytes()),
        _ => false,
    }
}

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
// Tests (pure pieces — the store + authorize decision live in service::reader)
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secret_ok_requires_configured_and_matching() {
        assert!(secret_ok(Some("s3cret"), Some("s3cret")));
        assert!(!secret_ok(Some("s3cret"), Some("wrong")));
        // Fail closed: no configured secret rejects even a present header.
        assert!(!secret_ok(None, Some("anything")));
        // Missing header rejects.
        assert!(!secret_ok(Some("s3cret"), None));
        assert!(!secret_ok(None, None));
    }

    #[test]
    fn scan_ok_and_err_wire_shape() {
        let ok = serde_json::to_value(ScanOk {
            ok: true,
            display_name: Some("Nok".into()),
        })
        .unwrap();
        assert_eq!(ok.get("ok").and_then(|v| v.as_bool()), Some(true));
        assert_eq!(ok.get("display_name").and_then(|v| v.as_str()), Some("Nok"));

        let err = serde_json::to_value(ScanErr {
            ok: false,
            reason: "not_authorized",
        })
        .unwrap();
        assert_eq!(err.get("ok").and_then(|v| v.as_bool()), Some(false));
        assert_eq!(
            err.get("reason").and_then(|v| v.as_str()),
            Some("not_authorized")
        );
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
