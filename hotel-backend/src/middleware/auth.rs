//! Cookie-session auth middleware — Phase 4 PR2.
//!
//! Wraps the `/api/new/*` subrouter. Two operating modes, switched by
//! the `auth_enabled` flag on [`AppState`]:
//!
//! * **Disabled** (`AUTH_ENABLED=false`, the default): the middleware
//!   becomes a no-op pass-through. Production stays exactly as it was
//!   before PR2 until an operator provisions an admin user via the
//!   `create_user` CLI and flips the flag.
//! * **Enabled** (`AUTH_ENABLED=true`): every request must carry the
//!   `session` cookie. Missing or invalid → `401` with
//!   `{"error":"unauthenticated"}`. Valid → the resolved [`User`] is
//!   attached to `request.extensions_mut()` so downstream handlers can
//!   pull it out via `Extension<User>` (PR4 will use this for admin-
//!   only checks).
//!
//! The `/api/auth/*` endpoints (login / logout / me) are deliberately
//! NOT wrapped by this middleware — that would create a chicken-and-egg
//! login flow. `main.rs` mounts those on a separate router.

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::cookie::CookieJar;

use crate::routes::auth::{ErrorResponse, SESSION_COOKIE_NAME};
use crate::routes::mode::AppState;

/// Reject the request with a 401 + `{"error":"unauthenticated"}` body.
///
/// Centralised so both the missing-cookie and the invalid-cookie paths
/// emit the exact same wire shape — the frontend in PR3 will pattern
/// match against this string to decide whether to redirect to /login.
fn unauthenticated_response() -> Response {
    let body = ErrorResponse { error: "unauthenticated" };
    (StatusCode::UNAUTHORIZED, Json(body)).into_response()
}

/// Reject the request with a 500 — used only when the database itself
/// fails during session validation. We do NOT collapse this into 401:
/// a transient PG outage shouldn't masquerade as "your cookie is bad"
/// and force every signed-in user to reauthenticate.
fn internal_error_response() -> Response {
    let body = ErrorResponse { error: "internal_error" };
    (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
}

/// Auth middleware for protected `/api/new/*` routes.
///
/// Mounted in `main.rs` via:
/// ```ignore
/// .route_layer(axum::middleware::from_fn_with_state(state.clone(), require_auth))
/// ```
///
/// Returns the inner handler's response on success; otherwise a 401 or
/// 500 JSON response. On success, the resolved [`crate::domain::user::User`]
/// is inserted into the request's extension map.
pub async fn require_auth(
    State(state): State<AppState>,
    jar: CookieJar,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    // Auth-flag bypass: when disabled, the middleware is invisible.
    // This is the production default until operators explicitly opt
    // in, so the existing routes keep their pre-PR2 behavior.
    if !state.auth_enabled {
        return next.run(request).await;
    }

    let token = match jar.get(SESSION_COOKIE_NAME) {
        Some(cookie) => cookie.value().to_string(),
        None => return unauthenticated_response(),
    };

    match state
        .auth_service
        .validate_session(&state.new_pool, &token)
        .await
    {
        Ok(Some((user, _session))) => {
            // Attach the user to the request so downstream handlers can
            // pull it out via `axum::Extension<User>` (PR4 will use this
            // for the admin-vs-receptionist split). We attach the full
            // domain `User` rather than the `UserDto` because the latter
            // is purely a wire-format concern — the handler may want
            // fields the DTO drops (none today, but room to grow).
            request.extensions_mut().insert(user);
            next.run(request).await
        }
        Ok(None) => unauthenticated_response(),
        Err(err) => {
            tracing::error!(
                error = %err,
                "auth middleware: validate_session failed"
            );
            internal_error_response()
        }
    }
}
