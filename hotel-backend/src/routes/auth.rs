//! Authentication HTTP routes — Phase 4 PR2.
//!
//! Three endpoints exposed under `/api/auth/*`:
//!
//! | Method | Path              | Auth | Purpose                                   |
//! |--------|-------------------|------|-------------------------------------------|
//! | POST   | /api/auth/login   | none | Verify credentials, mint a session cookie |
//! | POST   | /api/auth/logout  | none | Drop the session row, clear the cookie    |
//! | GET    | /api/auth/me      | none | Resolve the cookie → current user         |
//!
//! All three are intentionally PUBLIC — they are the bootstrap surface
//! the frontend hits BEFORE it has an authenticated session. The
//! `require_auth` middleware (see `middleware::auth`) only gates the
//! `/api/new/*` subrouter.
//!
//! ## Cookie shape
//!
//! Single cookie named `session`, holding the raw 64-char hex token
//! returned by [`AuthService::login`]. Flags:
//! * `HttpOnly` — JS cannot read it (mitigates XSS)
//! * `SameSite=Lax` — cross-site POSTs from a link don't smuggle the
//!   cookie, but top-level navigations + same-site fetches still carry
//!   it. Strict would break OAuth-style return flows we may add later.
//! * `Path=/` — every backend route sees the cookie
//! * `Max-Age=86400` — matches `service::auth::DEFAULT_SESSION_TTL`
//! * `Secure` — set ONLY when the request arrived over HTTPS. Local
//!   development on `http://localhost:3003` would otherwise fail to
//!   echo the cookie back.
//!
//! ## Error contract
//!
//! Every failure response is `{"error": "<machine-readable-code>"}` with
//! the appropriate status code. Codes used:
//! * `invalid_credentials` (401) — login: unknown user OR wrong password
//!   OR deactivated account (collapsed at the wire to prevent enumeration)
//! * `unauthenticated` (401) — `/api/auth/me` with no/invalid cookie

use std::sync::Arc;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::user::{Role, User};
use crate::repository::session::PgSessionRepository;
use crate::repository::user::PgUserRepository;
use crate::routes::mode::AppState;
use crate::service::auth::{AuthError, AuthService, DEFAULT_SESSION_TTL};

/// Cookie name carrying the session token.
///
/// Public because the middleware reads it and tests assert against it —
/// keeping the literal in one place prevents drift between writer +
/// reader.
pub const SESSION_COOKIE_NAME: &str = "session";

/// Type alias for the production `AuthService` — wraps the PG-backed
/// repositories. Carried by `AppState::auth_service` and used by the
/// route handlers + middleware.
pub type ProdAuthService = AuthService<PgUserRepository, PgSessionRepository>;

// =============================================================================
// Wire-format DTOs
// =============================================================================

/// Public, password-hash-free projection of [`User`].
///
/// Built with `From<User>` so handlers can `user.into()` without ever
/// risking a stray `password_hash` leak — the field simply isn't
/// reachable on the DTO.
#[derive(Debug, Clone, Serialize)]
pub struct UserDto {
    pub user_id: i64,
    pub username: String,
    pub role: Role,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub last_login_at: Option<NaiveDateTime>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            user_id: user.user_id,
            username: user.username,
            role: user.role,
            active: user.active,
            created_at: user.created_at,
            last_login_at: user.last_login_at,
        }
    }
}

/// Request body for `POST /api/auth/login`.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Success body for `POST /api/auth/login`.
///
/// Track G7 added the `permissions` field so the frontend has the
/// user's gate-check set on the very first render after login —
/// otherwise the UI would briefly show non-cashier users a Refund
/// button (or similar) while a follow-up `/api/auth/me` call resolved.
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: UserDto,
    pub permissions: Vec<String>,
}

/// Success body for `GET /api/auth/me`.
///
/// Track G7 (audit T4 HIGH-9) added the `permissions` field so the
/// frontend can hide buttons the current user lacks permission to
/// click. Always sorted alphabetically for stable rendering. Empty
/// when the user has no role assignments (or when auth is disabled —
/// see [`me`] below).
#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub user: UserDto,
    pub permissions: Vec<String>,
}

/// Uniform error body — single `error` string keying a machine-readable
/// code. Mirrors the contract documented at the top of this module.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: &'static str,
}

impl ErrorResponse {
    fn new(code: &'static str) -> Self {
        Self { error: code }
    }
}

// Note: the `/api/auth/login|logout|me` routes are mounted INLINE in main.rs
// (login carries its own rate-limit layer and the group stays outside the auth
// gate); there is no `router()` builder here.

// =============================================================================
// Handlers
// =============================================================================

/// `POST /api/auth/login`
///
/// On success: 200 + `LoginResponse` body + `Set-Cookie: session=<token>`.
/// On any failure (unknown user, wrong password, deactivated account):
/// 401 + `{"error": "invalid_credentials"}`. The deactivated case is
/// collapsed into `invalid_credentials` at the wire to prevent username
/// enumeration — the service layer surfaces a distinct `UserDeactivated`
/// variant for richer admin-UI handling, but THIS endpoint hides it.
pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    headers: HeaderMap,
    Json(body): Json<LoginRequest>,
) -> Result<(CookieJar, Json<LoginResponse>), (StatusCode, Json<ErrorResponse>)> {
    let ip = client_ip(&headers);
    let user_agent = client_user_agent(&headers);

    let result = state
        .auth_service
        .login(
            &state.new_pool,
            &body.username,
            &body.password,
            ip.as_deref(),
            user_agent.as_deref(),
        )
        .await;

    let (user, session) = match result {
        Ok(pair) => pair,
        Err(AuthError::InvalidCredentials) | Err(AuthError::UserDeactivated) => {
            return Err(invalid_credentials());
        }
        Err(AuthError::Db(err)) => {
            tracing::error!(error = %err, "auth/login: database error");
            return Err(internal_error());
        }
        Err(AuthError::Hash(err)) => {
            tracing::error!(error = %err, "auth/login: hash error");
            return Err(internal_error());
        }
        // PR4 added admin-only error variants to `AuthError`
        // (`UserNotFound`, `UsernameTaken`). They cannot arise from the
        // `login()` code path, but the exhaustiveness checker still
        // requires us to spell out the case. If one ever surfaces it
        // points at a service-layer regression — alarm and 500.
        Err(other) => {
            tracing::error!(error = %other, "auth/login: unexpected service error");
            return Err(internal_error());
        }
    };

    let secure = is_https_request(&headers);
    let cookie = build_session_cookie(session.id, secure);

    // Track G7 — ship permissions alongside the user so the UI is
    // gate-aware from the very first render. Lookup failures degrade
    // to an empty list (same rationale as `/api/auth/me`).
    let permissions = crate::middleware::permissions_for_user(
        &state.new_pool,
        user.user_id,
    )
    .await
    .unwrap_or_else(|err| {
        tracing::warn!(
            error = %err,
            user_id = user.user_id,
            "auth/login: permission lookup failed, returning empty list"
        );
        Vec::new()
    });

    Ok((
        jar.add(cookie),
        Json(LoginResponse {
            user: user.into(),
            permissions,
        }),
    ))
}

/// `POST /api/auth/logout`
///
/// Idempotent — no session cookie + no DB row both succeed. Always
/// returns 200 + empty JSON body and clears the cookie via a
/// `Max-Age=0` Set-Cookie header.
pub async fn logout(
    State(state): State<AppState>,
    jar: CookieJar,
    headers: HeaderMap,
) -> (CookieJar, Json<serde_json::Value>) {
    if let Some(token) = jar.get(SESSION_COOKIE_NAME).map(|c| c.value().to_string()) {
        if let Err(err) = state.auth_service.logout(&state.new_pool, &token).await {
            // Failure to delete the row is logged but does not fail the
            // request — the client still wants its cookie cleared, and a
            // dangling DB row will be reaped by the periodic cleanup
            // (Phase 4 follow-up).
            tracing::warn!(error = %err, "auth/logout: failed to delete session row");
        }
    }
    let secure = is_https_request(&headers);
    let cleared = clear_session_cookie(secure);
    (jar.add(cleared), Json(serde_json::json!({})))
}

/// `GET /api/auth/me`
///
/// Cookie present + valid → 200 + `MeResponse`. Cookie missing OR
/// invalid OR pointing at a deactivated user → 401 +
/// `{"error": "unauthenticated"}`. Never 500s on a missing cookie —
/// that's a normal "not logged in yet" state, not a server fault.
pub async fn me(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<Json<MeResponse>, (StatusCode, Json<ErrorResponse>)> {
    let token = jar
        .get(SESSION_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or_else(unauthenticated)?;

    match state
        .auth_service
        .validate_session(&state.new_pool, &token)
        .await
    {
        Ok(Some((user, _session))) => {
            // Track G7 — surface the permission set so the frontend can
            // pre-hide gated UI. We swallow lookup failures here (returning
            // an empty list rather than a 500) because the session itself
            // is valid: a transient permission-table outage should not
            // log the user out, it should just temporarily under-grant.
            let permissions = crate::middleware::permissions_for_user(
                &state.new_pool,
                user.user_id,
            )
            .await
            .unwrap_or_else(|err| {
                tracing::warn!(
                    error = %err,
                    user_id = user.user_id,
                    "auth/me: permission lookup failed, returning empty list"
                );
                Vec::new()
            });

            Ok(Json(MeResponse {
                user: user.into(),
                permissions,
            }))
        }
        Ok(None) => Err(unauthenticated()),
        Err(err) => {
            tracing::error!(error = %err, "auth/me: validate_session failed");
            Err(internal_error())
        }
    }
}

/// `POST /api/auth/cf-login` — Cloudflare Access auto-login.
///
/// The deployment sits behind Cloudflare Access, so every request that
/// reaches us already passed the CF identity check and carries a signed
/// RS256 assertion (`Cf-Access-Jwt-Assertion` header, `CF_Authorization`
/// cookie fallback). This endpoint verifies that assertion
/// (`middleware::cf_access`), maps the verified `email` claim to a
/// `ht_users` row (`AuthService::login_via_email` → migration 074's
/// `email` column), and mints a session through the EXACT same
/// `create_and_touch_login` + `build_session_cookie` path as password
/// [`login`] — the resulting session is indistinguishable.
///
/// STRICTLY ADDITIVE: password login is untouched and remains the
/// fallback for users without an email mapping.
///
/// Responses:
/// * 200 + [`LoginResponse`] + `Set-Cookie: session=…` — auto-login OK
///   (or an already-valid session existed; idempotent, no new session).
/// * 401 `{"error":"cf_token_missing"}` — no CF assertion on the request.
/// * 401 `{"error":"cf_token_invalid"}` — assertion failed verification.
/// * 401 `{"error":"cf_email_not_mapped"}` — verified email matches no
///   active local user (frontend silently falls back to the password
///   form). Deactivated users collapse into this code too.
/// * 404 `{"error":"cf_login_disabled"}` — kill-switch `CF_AUTO_LOGIN`
///   is off (`false`/`0`; default ON).
pub async fn cf_login(
    State(state): State<AppState>,
    jar: CookieJar,
    headers: HeaderMap,
) -> Result<(CookieJar, Json<LoginResponse>), (StatusCode, Json<ErrorResponse>)> {
    if !cf_auto_login_enabled() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse::new("cf_login_disabled")),
        ));
    }

    // Idempotency: a valid app session already exists → return it as-is
    // (no fresh session row, no Set-Cookie). Keeps a re-fired cf-login
    // from stacking ht_sessions rows on every page load.
    if let Some(token) = jar.get(SESSION_COOKIE_NAME).map(|c| c.value().to_string()) {
        if let Ok(Some((user, _session))) = state
            .auth_service
            .validate_session(&state.new_pool, &token)
            .await
        {
            let permissions = permissions_or_empty(&state, user.user_id, "auth/cf-login").await;
            return Ok((
                jar,
                Json(LoginResponse {
                    user: user.into(),
                    permissions,
                }),
            ));
        }
        // Invalid/expired cookie — fall through and mint a fresh session
        // from the CF assertion (same as having no cookie at all).
    }

    let cf_token = crate::middleware::cf_access::extract_token(&headers, &jar).ok_or((
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse::new("cf_token_missing")),
    ))?;

    let email = match crate::middleware::cf_access::verify_cf_access_token(&cf_token).await {
        Ok(email) => email,
        Err(err) => {
            // Verification detail goes to logs only — the wire gets a
            // stable machine code the frontend treats as "fall back to
            // the password form".
            tracing::warn!(error = %err, "auth/cf-login: CF Access token rejected");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse::new("cf_token_invalid")),
            ));
        }
    };

    let ip = client_ip(&headers);
    let user_agent = client_user_agent(&headers);

    let result = state
        .auth_service
        .login_via_email(&state.new_pool, &email, ip.as_deref(), user_agent.as_deref())
        .await;

    let (user, session) = match result {
        Ok(pair) => pair,
        // Unknown email AND deactivated account share one wire code —
        // both mean "no auto-login for you, use the password form".
        Err(AuthError::InvalidCredentials) | Err(AuthError::UserDeactivated) => {
            tracing::info!("auth/cf-login: verified CF email has no active local mapping");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse::new("cf_email_not_mapped")),
            ));
        }
        Err(AuthError::Db(err)) => {
            tracing::error!(error = %err, "auth/cf-login: database error");
            return Err(internal_error());
        }
        Err(other) => {
            tracing::error!(error = %other, "auth/cf-login: unexpected service error");
            return Err(internal_error());
        }
    };

    let secure = is_https_request(&headers);
    let cookie = build_session_cookie(session.id, secure);
    let permissions = permissions_or_empty(&state, user.user_id, "auth/cf-login").await;

    Ok((
        jar.add(cookie),
        Json(LoginResponse {
            user: user.into(),
            permissions,
        }),
    ))
}

// =============================================================================
// Helpers
// =============================================================================

/// Kill-switch for the Cloudflare Access auto-login route. DEFAULT ON —
/// explicitly set `CF_AUTO_LOGIN=false` (or `0`) to make
/// `POST /api/auth/cf-login` answer 404 without touching anything else.
/// (Opposite default polarity from the ship-dark writeback flags on
/// purpose: this endpoint only READS the shared world — it never writes
/// legacy — so on-by-default is safe and is the entire point of the
/// feature.)
fn cf_auto_login_enabled() -> bool {
    match std::env::var("CF_AUTO_LOGIN") {
        Ok(value) => {
            let normalized = value.trim().to_ascii_lowercase();
            !(normalized == "false" || normalized == "0")
        }
        Err(_) => true,
    }
}

/// Permission lookup that degrades to an empty list (Track G7 pattern
/// shared by [`login`] / [`me`] / [`cf_login`]) — a transient
/// permission-table failure should under-grant, not 500.
async fn permissions_or_empty(state: &AppState, user_id: i64, ctx: &'static str) -> Vec<String> {
    crate::middleware::permissions_for_user(&state.new_pool, user_id)
        .await
        .unwrap_or_else(|err| {
            tracing::warn!(
                error = %err,
                user_id,
                "{ctx}: permission lookup failed, returning empty list"
            );
            Vec::new()
        })
}

/// Build the session cookie for a freshly minted session.
///
/// Lifetime mirrors [`DEFAULT_SESSION_TTL`] so the browser stops sending
/// the cookie at roughly the same moment the server stops accepting it.
///
/// The `Max-Age` value uses `cookie::time::Duration` (the same `time`
/// crate the `cookie` crate depends on internally) — that's the type
/// `CookieBuilder::max_age` requires. `chrono::Duration::num_seconds`
/// bridges the two.
fn build_session_cookie(token: String, secure: bool) -> Cookie<'static> {
    let max_age = cookie::time::Duration::seconds(DEFAULT_SESSION_TTL.num_seconds());
    Cookie::build((SESSION_COOKIE_NAME, token))
        .http_only(true)
        .secure(secure)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(max_age)
        .build()
}

/// Build a "delete me" Set-Cookie header. `Max-Age=0` is the standard
/// signal for the browser to drop the cookie immediately.
fn clear_session_cookie(secure: bool) -> Cookie<'static> {
    Cookie::build((SESSION_COOKIE_NAME, ""))
        .http_only(true)
        .secure(secure)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(cookie::time::Duration::seconds(0))
        .build()
}

/// Resolve the client IP for session bookkeeping.
///
/// Preference order:
/// 1. `X-Forwarded-For` (first comma-separated entry) — the deployment
///    sits behind nginx/cloudflared, so the only IP worth recording is
///    the proxy's `X-Forwarded-For` value.
/// 2. `X-Real-IP` — secondary header some proxies set instead.
///
/// Returns `None` when neither header is present (e.g. local dev hitting
/// the backend directly). We deliberately do NOT fall back to the socket
/// peer address — wiring `ConnectInfo<SocketAddr>` would force every
/// caller of `axum::serve` to switch to `into_make_service_with_connect_info`,
/// which is more invasive than the value of recording `127.0.0.1` in
/// `ht_sessions.ip` for local development.
fn client_ip(headers: &HeaderMap) -> Option<String> {
    if let Some(value) = headers.get("x-forwarded-for").and_then(|v| v.to_str().ok()) {
        if let Some(first) = value.split(',').next() {
            let trimmed = first.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    if let Some(value) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
        let trimmed = value.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }
    None
}

/// Capture the User-Agent header (if any) for session bookkeeping.
fn client_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

/// True when the request arrived over HTTPS — drives the `Secure`
/// cookie flag. Detection relies on the standard reverse-proxy header
/// `X-Forwarded-Proto`; absent that, we conservatively assume HTTP so
/// local dev (no TLS) keeps working. Production deployments terminate
/// TLS at cloudflared/nginx and forward the header.
fn is_https_request(headers: &HeaderMap) -> bool {
    headers
        .get("x-forwarded-proto")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.eq_ignore_ascii_case("https"))
        .unwrap_or(false)
}

fn invalid_credentials() -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse::new("invalid_credentials")),
    )
}

fn unauthenticated() -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(ErrorResponse::new("unauthenticated")),
    )
}

fn internal_error() -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse::new("internal_error")),
    )
}

/// Build the production `AuthService` instance.
///
/// Public so `main.rs` (and the future test harness) can mint one
/// without re-deriving the generic type. The service is cheap to clone
/// (Arcs all the way down) so a single instance is shared across the
/// whole router via `AppState`.
pub fn build_auth_service() -> Arc<ProdAuthService> {
    Arc::new(AuthService::new(
        Arc::new(PgUserRepository::new()),
        Arc::new(PgSessionRepository::new()),
    ))
}

// =============================================================================
// Tests
// =============================================================================
//
// Existing route modules in this crate (e.g. `routes::health`) only
// exercise pure handler logic — none spin up a `Router` against a real
// `AppState`, because building that state requires both a SQL Server
// pool AND a PostgreSQL pool. We follow the same pattern: assert the
// pieces we CAN test in isolation (DTO mapping, cookie shape, error
// contracts), and lean on the service-layer tests in
// `service::auth::tests` (12 cases from PR1) for end-to-end correctness.

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn fixed_user() -> User {
        User {
            user_id: 7,
            username: "alice".to_string(),
            password_hash: "argon2id$v=19$...redacted...".to_string(),
            role: Role::Admin,
            active: true,
            created_at: NaiveDate::from_ymd_opt(2026, 5, 10)
                .unwrap()
                .and_hms_opt(19, 40, 0)
                .unwrap(),
            last_login_at: Some(
                NaiveDate::from_ymd_opt(2026, 5, 10)
                    .unwrap()
                    .and_hms_opt(19, 45, 0)
                    .unwrap(),
            ),
            email: None,
        }
    }

    #[test]
    fn user_dto_drops_password_hash() {
        let dto: UserDto = fixed_user().into();
        let json = serde_json::to_string(&dto).unwrap();
        assert!(
            !json.contains("password_hash"),
            "UserDto must never serialize password_hash; got: {json}"
        );
        assert!(json.contains("\"username\":\"alice\""));
        assert!(json.contains("\"role\":\"admin\""));
    }

    #[test]
    fn user_dto_preserves_all_safe_fields() {
        let user = fixed_user();
        let dto: UserDto = user.clone().into();
        assert_eq!(dto.user_id, user.user_id);
        assert_eq!(dto.username, user.username);
        assert_eq!(dto.role, user.role);
        assert_eq!(dto.active, user.active);
        assert_eq!(dto.created_at, user.created_at);
        assert_eq!(dto.last_login_at, user.last_login_at);
    }

    #[test]
    fn user_dto_serializes_last_login_null_when_never_logged_in() {
        let mut user = fixed_user();
        user.last_login_at = None;
        let dto: UserDto = user.into();
        let value = serde_json::to_value(&dto).unwrap();
        assert!(value.get("last_login_at").unwrap().is_null());
    }

    #[test]
    fn login_response_carries_user_and_permissions_fields() {
        // Track G7 widened the contract — frontend relies on
        // `permissions` being present so it doesn't have to make a
        // follow-up /me round-trip just to learn the role grid.
        let resp = LoginResponse {
            user: fixed_user().into(),
            permissions: vec!["payment.refund".to_string()],
        };
        let value = serde_json::to_value(&resp).unwrap();
        let obj = value.as_object().unwrap();
        assert_eq!(obj.len(), 2);
        assert!(obj.contains_key("user"));
        assert!(obj.contains_key("permissions"));
        assert_eq!(
            obj.get("permissions").unwrap().as_array().unwrap().len(),
            1
        );
    }

    #[test]
    fn me_response_carries_user_and_permissions_fields() {
        let resp = MeResponse {
            user: fixed_user().into(),
            permissions: vec![
                "admin.users".to_string(),
                "payment.refund".to_string(),
            ],
        };
        let value = serde_json::to_value(&resp).unwrap();
        let obj = value.as_object().unwrap();
        assert!(obj.contains_key("user"));
        assert!(obj.contains_key("permissions"));
        assert_eq!(
            obj.get("permissions").unwrap().as_array().unwrap().len(),
            2
        );
    }

    #[test]
    fn error_response_uses_single_error_key() {
        let value = serde_json::to_value(ErrorResponse::new("invalid_credentials")).unwrap();
        assert_eq!(
            value.get("error").and_then(|v| v.as_str()),
            Some("invalid_credentials")
        );
        assert_eq!(value.as_object().unwrap().len(), 1);
    }

    #[test]
    fn build_session_cookie_sets_security_flags() {
        let cookie = build_session_cookie("a".repeat(64), true);
        assert_eq!(cookie.name(), SESSION_COOKIE_NAME);
        assert_eq!(cookie.value().len(), 64);
        assert_eq!(cookie.http_only(), Some(true));
        assert_eq!(cookie.secure(), Some(true));
        assert_eq!(cookie.same_site(), Some(SameSite::Lax));
        assert_eq!(cookie.path(), Some("/"));
        assert_eq!(
            cookie.max_age().map(|d| d.whole_seconds()),
            Some(DEFAULT_SESSION_TTL.num_seconds())
        );
    }

    #[test]
    fn build_session_cookie_omits_secure_for_http() {
        // Local dev (no TLS, no x-forwarded-proto) MUST NOT set Secure —
        // browsers refuse to echo Secure cookies back over plain HTTP,
        // which would silently break login on `http://localhost:3003`.
        let cookie = build_session_cookie("token".to_string(), false);
        assert_eq!(cookie.secure(), Some(false));
    }

    #[test]
    fn clear_session_cookie_uses_zero_max_age() {
        let cookie = clear_session_cookie(false);
        assert_eq!(cookie.value(), "");
        assert_eq!(cookie.max_age().map(|d| d.whole_seconds()), Some(0));
        assert_eq!(cookie.path(), Some("/"));
    }

    #[test]
    fn invalid_credentials_returns_401_with_machine_code() {
        let (status, Json(body)) = invalid_credentials();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body.error, "invalid_credentials");
    }

    #[test]
    fn unauthenticated_returns_401_with_machine_code() {
        let (status, Json(body)) = unauthenticated();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body.error, "unauthenticated");
    }

    #[test]
    fn client_ip_prefers_x_forwarded_for_first_entry() {
        let mut headers = HeaderMap::new();
        headers.insert("x-forwarded-for", "203.0.113.4, 10.0.0.1".parse().unwrap());
        assert_eq!(client_ip(&headers).as_deref(), Some("203.0.113.4"));
    }

    #[test]
    fn client_ip_falls_back_to_x_real_ip() {
        let mut headers = HeaderMap::new();
        headers.insert("x-real-ip", "198.51.100.7".parse().unwrap());
        assert_eq!(client_ip(&headers).as_deref(), Some("198.51.100.7"));
    }

    #[test]
    fn client_ip_returns_none_without_proxy_headers() {
        // No fall-through to socket peer — we deliberately don't wire
        // ConnectInfo. Local dev lookups return None and the auth
        // service stores NULL in `ht_sessions.ip`.
        let headers = HeaderMap::new();
        assert!(client_ip(&headers).is_none());
    }

    #[test]
    fn is_https_request_only_true_for_forwarded_https() {
        let mut headers = HeaderMap::new();
        assert!(!is_https_request(&headers));
        headers.insert("x-forwarded-proto", "http".parse().unwrap());
        assert!(!is_https_request(&headers));
        headers.insert("x-forwarded-proto", "https".parse().unwrap());
        assert!(is_https_request(&headers));
        // Case-insensitive — some proxies upcase the value.
        headers.insert("x-forwarded-proto", "HTTPS".parse().unwrap());
        assert!(is_https_request(&headers));
    }
}
