//! Admin user-management HTTP routes — Phase 4 PR4.
//!
//! Three endpoints exposed under `/api/admin/users*`:
//!
//! | Method | Path                       | Auth  | Purpose                          |
//! |--------|----------------------------|-------|----------------------------------|
//! | GET    | /api/admin/users           | admin | List every user (no password_hash) |
//! | POST   | /api/admin/users           | admin | Provision a new user             |
//! | PATCH  | /api/admin/users/{user_id} | admin | Toggle active / change role / reset password |
//!
//! All three require an authenticated admin. Authentication is enforced
//! by the `require_auth` middleware applied to the whole `/api/admin/*`
//! subrouter in `main.rs`; authorization (admin role) is enforced PER
//! HANDLER below via the [`Extension<User>`] injected by that middleware.
//!
//! ## Auth-disabled mode (`AUTH_ENABLED=false`)
//!
//! When the operator has not yet flipped `AUTH_ENABLED=true` the
//! `require_auth` middleware is a no-op pass-through and never inserts
//! a `User` extension. Each handler extracts `Option<Extension<User>>`
//! and returns `401 unauthenticated` when the extension is missing —
//! that gives a clear, debuggable response instead of the cryptic 500
//! Axum would otherwise emit when the bare `Extension<User>` extractor
//! fails.
//!
//! ## Error contract
//!
//! Every failure response is `{"error": "<machine-readable-code>"}`
//! with the appropriate status code:
//!
//! * `unauthenticated` (401) — no `User` in the request extensions
//! * `forbidden` (403) — caller is authenticated but not an admin
//! * `invalid_username` / `invalid_password` (400) — validation failure
//! * `username_taken` (409) — POST collided with an existing user
//! * `user_not_found` (404) — PATCH targeted a missing `user_id`
//! * `internal_error` (500) — repository / hashing failure

use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    routing::{get, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::domain::user::{Role, User};
use crate::routes::auth::UserDto;
use crate::routes::mode::AppState;
use crate::service::auth::AuthError;

// =============================================================================
// Wire-format DTOs
// =============================================================================

/// Success body for `GET /api/admin/users`.
#[derive(Debug, Serialize)]
pub struct ListResponse {
    pub users: Vec<UserDto>,
}

/// Request body for `POST /api/admin/users`.
#[derive(Debug, Deserialize)]
pub struct CreateRequest {
    pub username: String,
    pub password: String,
    pub role: Role,
}

/// Success body for `POST /api/admin/users`.
#[derive(Debug, Serialize)]
pub struct CreateResponse {
    pub user: UserDto,
}

/// Request body for `PATCH /api/admin/users/{user_id}`.
///
/// Every field is optional — admins may toggle one attribute at a time.
/// An empty body is legal and returns the unchanged user.
#[derive(Debug, Deserialize, Default)]
pub struct PatchRequest {
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub role: Option<Role>,
    #[serde(default)]
    pub password: Option<String>,
}

/// Success body for `PATCH /api/admin/users/{user_id}`.
#[derive(Debug, Serialize)]
pub struct PatchResponse {
    pub user: UserDto,
}

/// Uniform error body — single `error` string keying a machine-readable
/// code. Mirrors `routes::auth::ErrorResponse` so the frontend can
/// share an error-mapping helper across both modules.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: &'static str,
}

impl ErrorResponse {
    fn new(code: &'static str) -> Self {
        Self { error: code }
    }
}

// =============================================================================
// Router
// =============================================================================

/// Build the `/api/admin/users*` subrouter.
///
/// Mount with `app.merge(routes::admin_users::router().with_state(state))`
/// from `main.rs`, then wrap with `require_auth` so every handler
/// receives a `User` extension. Role enforcement happens per-handler.
pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/admin/users",
            get(list_users).post(create_user),
        )
        .route("/api/admin/users/{user_id}", patch(patch_user))
}

// =============================================================================
// Handlers
// =============================================================================

/// `GET /api/admin/users` — return every user as a `UserDto` list,
/// sorted by username. 401 when no session, 403 when caller is not an
/// admin.
pub async fn list_users(
    State(state): State<AppState>,
    actor: Option<Extension<User>>,
) -> Result<Json<ListResponse>, (StatusCode, Json<ErrorResponse>)> {
    require_admin(actor.as_deref())?;

    match state.auth_service.list_users(&state.new_pool).await {
        Ok(users) => Ok(Json(ListResponse {
            users: users.into_iter().map(UserDto::from).collect(),
        })),
        Err(err) => {
            tracing::error!(error = %err, "admin/users: list failed");
            Err(internal_error())
        }
    }
}

/// `POST /api/admin/users` — provision a user. 201 on success.
///
/// Validation runs BEFORE the service call so we can map each rule to
/// a distinct error code. Returning 400 keeps invalid-input errors
/// distinguishable from 409 (collision) and 500 (db failure).
pub async fn create_user(
    State(state): State<AppState>,
    actor: Option<Extension<User>>,
    Json(body): Json<CreateRequest>,
) -> Result<(StatusCode, Json<CreateResponse>), (StatusCode, Json<ErrorResponse>)> {
    require_admin(actor.as_deref())?;

    validate_username(&body.username)?;
    validate_password(&body.password)?;

    match state
        .auth_service
        .create_user(&state.new_pool, &body.username, &body.password, body.role)
        .await
    {
        Ok(user) => Ok((
            StatusCode::CREATED,
            Json(CreateResponse {
                user: UserDto::from(user),
            }),
        )),
        Err(AuthError::UsernameTaken) => Err(error_response(StatusCode::CONFLICT, "username_taken")),
        Err(err) => {
            tracing::error!(error = %err, "admin/users: create failed");
            Err(internal_error())
        }
    }
}

/// `PATCH /api/admin/users/{user_id}` — apply zero-or-more field updates.
pub async fn patch_user(
    State(state): State<AppState>,
    actor: Option<Extension<User>>,
    Path(user_id): Path<i64>,
    Json(body): Json<PatchRequest>,
) -> Result<Json<PatchResponse>, (StatusCode, Json<ErrorResponse>)> {
    require_admin(actor.as_deref())?;

    if let Some(ref password) = body.password {
        validate_password(password)?;
    }

    match state
        .auth_service
        .update_user(
            &state.new_pool,
            user_id,
            body.active,
            body.role,
            body.password.as_deref(),
        )
        .await
    {
        Ok(user) => Ok(Json(PatchResponse {
            user: UserDto::from(user),
        })),
        Err(AuthError::UserNotFound) => Err(error_response(StatusCode::NOT_FOUND, "user_not_found")),
        Err(err) => {
            tracing::error!(error = %err, "admin/users: patch failed");
            Err(internal_error())
        }
    }
}

// =============================================================================
// Helpers
// =============================================================================

/// Reject the request unless the caller is an authenticated admin.
///
/// Returns 401 when the `User` extension is missing (auth disabled OR
/// middleware short-circuited) and 403 when the user is authenticated
/// but their role is not `Admin`. This split keeps the frontend able
/// to distinguish "log in first" from "you don't have permission".
fn require_admin(actor: Option<&User>) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    let user = actor.ok_or_else(|| error_response(StatusCode::UNAUTHORIZED, "unauthenticated"))?;
    if user.role != Role::Admin {
        return Err(error_response(StatusCode::FORBIDDEN, "forbidden"));
    }
    Ok(())
}

/// Username validation: 3-64 chars, alphanumeric + `.` + `_` only.
///
/// Manual char loop instead of pulling in `regex` for a one-shot rule.
/// Mirrors the `--username` validation in `bin/create_user` (length
/// only) but tightens the character set so admins can't sneak in
/// shell metacharacters that would surprise downstream tooling.
fn validate_username(username: &str) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    let trimmed = username.trim();
    if trimmed.is_empty() {
        return Err(error_response(StatusCode::BAD_REQUEST, "invalid_username"));
    }
    if trimmed.len() < 3 || trimmed.len() > 64 {
        return Err(error_response(StatusCode::BAD_REQUEST, "invalid_username"));
    }
    let allowed = trimmed
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.');
    if !allowed {
        return Err(error_response(StatusCode::BAD_REQUEST, "invalid_username"));
    }
    Ok(())
}

/// Password validation: at least 8 characters. Admins can use anything
/// they want above that — we don't enforce complexity (a long passphrase
/// of dictionary words beats a short `P@ssw0rd!`).
fn validate_password(password: &str) -> Result<(), (StatusCode, Json<ErrorResponse>)> {
    if password.len() < 8 {
        return Err(error_response(StatusCode::BAD_REQUEST, "invalid_password"));
    }
    Ok(())
}

fn error_response(status: StatusCode, code: &'static str) -> (StatusCode, Json<ErrorResponse>) {
    (status, Json(ErrorResponse::new(code)))
}

fn internal_error() -> (StatusCode, Json<ErrorResponse>) {
    error_response(StatusCode::INTERNAL_SERVER_ERROR, "internal_error")
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn admin_user() -> User {
        User {
            user_id: 1,
            username: "admin".to_string(),
            password_hash: "hash".to_string(),
            role: Role::Admin,
            active: true,
            created_at: NaiveDate::from_ymd_opt(2026, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            last_login_at: None,
            email: None,
        }
    }

    fn receptionist_user() -> User {
        User {
            role: Role::Receptionist,
            ..admin_user()
        }
    }

    #[test]
    fn require_admin_accepts_admin() {
        let user = admin_user();
        assert!(require_admin(Some(&user)).is_ok());
    }

    #[test]
    fn require_admin_rejects_receptionist_with_403() {
        let user = receptionist_user();
        let (status, Json(body)) = require_admin(Some(&user)).unwrap_err();
        assert_eq!(status, StatusCode::FORBIDDEN);
        assert_eq!(body.error, "forbidden");
    }

    #[test]
    fn require_admin_rejects_missing_user_with_401() {
        let (status, Json(body)) = require_admin(None).unwrap_err();
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        assert_eq!(body.error, "unauthenticated");
    }

    #[test]
    fn validate_username_accepts_safe_inputs() {
        assert!(validate_username("alice").is_ok());
        assert!(validate_username("alice_smith").is_ok());
        assert!(validate_username("alice.smith").is_ok());
        assert!(validate_username("user01").is_ok());
        // Boundary: exactly 3 and 64 chars.
        assert!(validate_username("abc").is_ok());
        assert!(validate_username(&"a".repeat(64)).is_ok());
    }

    #[test]
    fn validate_username_rejects_too_short() {
        let (status, Json(body)) = validate_username("ab").unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(body.error, "invalid_username");
    }

    #[test]
    fn validate_username_rejects_too_long() {
        let (status, _) = validate_username(&"a".repeat(65)).unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[test]
    fn validate_username_rejects_disallowed_chars() {
        for bad in ["alice bob", "alice@host", "alice/admin", "alice;DROP", "中文"] {
            let result = validate_username(bad);
            assert!(result.is_err(), "{bad} should be rejected");
        }
    }

    #[test]
    fn validate_username_rejects_empty_or_whitespace() {
        assert!(validate_username("").is_err());
        assert!(validate_username("   ").is_err());
    }

    #[test]
    fn validate_password_accepts_eight_or_more_chars() {
        assert!(validate_password("abcdefgh").is_ok());
        assert!(validate_password("a long passphrase with spaces").is_ok());
    }

    #[test]
    fn validate_password_rejects_short_input() {
        let (status, Json(body)) = validate_password("short").unwrap_err();
        assert_eq!(status, StatusCode::BAD_REQUEST);
        assert_eq!(body.error, "invalid_password");
    }

    #[test]
    fn list_response_serializes_with_users_key() {
        let value = serde_json::to_value(ListResponse {
            users: vec![UserDto::from(admin_user())],
        })
        .unwrap();
        assert!(value.get("users").is_some(), "shape: {value}");
        let users_arr = value.get("users").unwrap().as_array().unwrap();
        assert_eq!(users_arr.len(), 1);
        // password_hash MUST NOT leak through the list response either.
        let serialized = serde_json::to_string(&value).unwrap();
        assert!(!serialized.contains("password_hash"));
    }

    #[test]
    fn error_response_uses_single_error_key() {
        let value = serde_json::to_value(ErrorResponse::new("forbidden")).unwrap();
        assert_eq!(
            value.get("error").and_then(|v| v.as_str()),
            Some("forbidden"),
        );
        assert_eq!(value.as_object().unwrap().len(), 1);
    }
}
