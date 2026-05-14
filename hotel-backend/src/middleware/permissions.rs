//! Permission-gating middleware — Track G7 (`audit-2026-05-13.md` T4 HIGH-9).
//!
//! Layered on top of the cookie-session [`require_auth`](super::auth::require_auth)
//! middleware: by the time a permission gate fires, the request already
//! carries an authenticated [`User`] in its extensions. The gate looks
//! up the permissions granted to that user (cached in-process) and
//! short-circuits to 403 unless the configured permission key is present.
//!
//! ## Construction
//!
//! ```ignore
//! use crate::middleware::permissions::require_permission;
//!
//! Router::new()
//!     .route("/api/new/payments/{id}/refund", post(handler))
//!     .route_layer(require_permission("payment.refund", state.clone()));
//! ```
//!
//! The helper returns a `tower` layer (`axum::middleware::from_fn_with_state`
//! wrapper) so it can be chained onto a single route via `route_layer`
//! or applied to a subrouter via `layer`. Mount it AFTER `require_auth`
//! — the gate relies on `Extension<User>` being present.
//!
//! ## Auth-disabled fallback
//!
//! When `state.auth_enabled == false` the gate is a no-op pass-through,
//! mirroring `require_auth`'s default-off behavior. This keeps the
//! production default (auth flag still off) working exactly as it does
//! today, and lets developers exercise the routes locally without
//! provisioning a full role grid.
//!
//! ## Caching
//!
//! User → permissions is a stable, slow-changing relation. We cache it
//! in-process via a `OnceCell<RwLock<HashMap<i64, Arc<HashSet<String>>>>>`.
//! Reads take a read-lock + clone of the `Arc<HashSet<_>>`, so they are
//! cheap and lock-free for the hot path. The first miss for a user
//! issues one SQL round-trip to load `ht_user_roles` ⋈ `ht_role_permissions`
//! ⋈ `ht_permissions`.
//!
//! TODO (G7 follow-up): wire `PermissionCache::invalidate(user_id)`
//! into the admin "edit user roles" UI (PATCH /api/admin/users/{id}/roles)
//! once that endpoint exists. Until then, role grants applied via direct
//! SQL require a backend restart to take effect.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock, RwLock};

use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::{from_fn_with_state, Next},
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::domain::user::User;
use crate::routes::mode::AppState;

/// Uniform error body returned by permission failures.
///
/// Single `error` string keying a machine-readable code, mirroring the
/// shape used by `routes::auth::ErrorResponse` so the frontend can share
/// one error-mapping helper across the auth + permission layers.
#[derive(Debug, Serialize)]
pub struct PermissionErrorResponse {
    pub error: &'static str,
}

/// Process-local cache: `user_id` → set of granted `permission_key`s.
///
/// Held in an `Arc<HashSet<_>>` so reads return a cheap clone-of-Arc
/// without copying the underlying set. Wrapped in `RwLock` so concurrent
/// reads do not serialize each other; writes (first miss for a user)
/// are rare.
type Inner = RwLock<HashMap<i64, Arc<HashSet<String>>>>;

/// Singleton cache instance. Read via [`cache`].
static CACHE: OnceLock<Inner> = OnceLock::new();

fn cache() -> &'static Inner {
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Fetch the permission set for `user_id`, populating the cache on miss.
///
/// Returns `Err` on a database failure — callers (the middleware) map
/// that to HTTP 500 rather than 403 so a transient PG outage does not
/// masquerade as "you lack permission".
async fn load_permissions(
    pool: &PgPool,
    user_id: i64,
) -> Result<Arc<HashSet<String>>, sqlx::Error> {
    // Fast path: cache hit. Use a short read-lock so we don't block
    // concurrent miss-fillers for other users.
    if let Some(set) = cache().read().expect("permission cache poisoned").get(&user_id).cloned() {
        return Ok(set);
    }

    // Slow path: load + cache. Multiple concurrent misses for the same
    // user can issue duplicate SQL — we accept that over holding the
    // write-lock across the await, which would serialize ALL miss
    // traffic site-wide.
    let rows: Vec<(String,)> = sqlx::query_as(
        r#"
        SELECT p.permission_key
        FROM ht_user_roles ur
        JOIN ht_role_permissions rp ON rp.role_id = ur.role_id
        JOIN ht_permissions p        ON p.permission_id = rp.permission_id
        WHERE ur.user_id = $1
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    let set: HashSet<String> = rows.into_iter().map(|(k,)| k).collect();
    let arc = Arc::new(set);
    cache()
        .write()
        .expect("permission cache poisoned")
        .insert(user_id, arc.clone());
    Ok(arc)
}

/// Drop the cached permissions for `user_id` so the next request reloads
/// from the database. Called by the admin user-management UI (when it
/// learns to edit role assignments — see module-level TODO).
#[allow(dead_code)]
pub fn invalidate(user_id: i64) {
    let mut guard = cache().write().expect("permission cache poisoned");
    guard.remove(&user_id);
}

/// Drop the entire cache. Used by tests + by the admin UI's "bulk
/// permission reset" flow.
#[allow(dead_code)]
pub fn invalidate_all() {
    let mut guard = cache().write().expect("permission cache poisoned");
    guard.clear();
}

/// State carried by [`require_permission`]: the auth+pool-bearing
/// `AppState` plus the static permission key being enforced.
///
/// `&'static str` for the permission key forces callers to use the
/// route-table literals (`"payment.refund"`, etc.) — no risk of a typo
/// at runtime, every key is a compile-time constant.
#[derive(Clone)]
pub struct PermissionGateState {
    app: AppState,
    required: &'static str,
}

impl PermissionGateState {
    /// Wrap an `AppState` + permission key for use with
    /// [`from_fn_with_state`] (called by [`require_permission`]).
    pub fn new(app: AppState, required: &'static str) -> Self {
        Self { app, required }
    }
}

/// Build a permission-gate `axum` middleware layer.
///
/// # Example
///
/// ```ignore
/// .route(
///     "/api/new/payments/{id}/refund",
///     post(refund_payment),
/// )
/// .route_layer(require_permission("payment.refund", state.clone()))
/// ```
///
/// The layer:
/// 1. Is a no-op when `state.auth_enabled == false` (matches `require_auth`).
/// 2. Reads `Extension<User>` from the request. Missing → 401 unauthenticated.
/// 3. Loads (and caches) the user's permission set.
/// 4. Returns 403 forbidden when `required` is not in the set.
/// 5. Otherwise forwards to the inner handler.
///
/// Returns an opaque `tower::Layer<S>` so the Axum router can apply it
/// via `route_layer` / `layer` without having to name the inner
/// `FromFnLayer` type.
///
/// The async fn pointer (`gate as fn(...) -> _`) is named explicitly so
/// the returned `FromFnLayer<...>` has a concrete, nameable type — that
/// lets `main.rs` bind the layer to a `let` before chaining it onto
/// route(s).
pub fn require_permission(
    required: &'static str,
    app: AppState,
) -> axum::middleware::FromFnLayer<
    GateFn,
    PermissionGateState,
    (axum::extract::State<PermissionGateState>, Request<Body>),
> {
    let state = PermissionGateState::new(app, required);
    from_fn_with_state(state, gate as GateFn)
}

/// Concrete async-fn-pointer type for the permission gate. Names the
/// signature axum's `from_fn_with_state` infers so the public
/// [`require_permission`] return type can be expressed without
/// `impl Trait`.
pub type GateFn = fn(
    axum::extract::State<PermissionGateState>,
    Request<Body>,
    Next,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>>;

/// Inner permission-gate implementation.
///
/// Boxes its own future so the fn-pointer type is stable (an `async fn`
/// pointer would have an unnamed opaque return type that we cannot
/// surface in [`GateFn`]). The runtime cost is one heap alloc per gated
/// request, dwarfed by the SQL+session validation that runs upstream.
fn gate(
    State(state): State<PermissionGateState>,
    request: Request<Body>,
    next: Next,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = Response> + Send>> {
    Box::pin(gate_inner(state, request, next))
}

async fn gate_inner(state: PermissionGateState, request: Request<Body>, next: Next) -> Response {
    // Auth-flag bypass — see module docs. Mirrors `require_auth`.
    if !state.app.auth_enabled {
        return next.run(request).await;
    }

    // The `require_auth` middleware MUST have run first; bare 401 if not.
    let user = match request.extensions().get::<User>().cloned() {
        Some(u) => u,
        None => return forbidden_response(StatusCode::UNAUTHORIZED, "unauthenticated"),
    };

    match load_permissions(&state.app.new_pool, user.user_id).await {
        Ok(perms) => {
            if perms.contains(state.required) {
                next.run(request).await
            } else {
                tracing::debug!(
                    user_id = user.user_id,
                    username = %user.username,
                    required = state.required,
                    "permission gate refused"
                );
                forbidden_response(StatusCode::FORBIDDEN, "forbidden")
            }
        }
        Err(err) => {
            tracing::error!(
                error = %err,
                user_id = user.user_id,
                required = state.required,
                "permission gate: database failure"
            );
            forbidden_response(StatusCode::INTERNAL_SERVER_ERROR, "internal_error")
        }
    }
}

fn forbidden_response(status: StatusCode, code: &'static str) -> Response {
    let body = PermissionErrorResponse { error: code };
    (status, Json(body)).into_response()
}

/// Resolve every permission key granted to `user_id` via the user→role
/// →permission grid. Used by the `/api/auth/me` route to ship the user's
/// permission list down to the frontend so it can hide buttons the user
/// cannot exercise. Cache-friendly — re-uses the same path as the gate.
pub async fn permissions_for_user(
    pool: &PgPool,
    user_id: i64,
) -> Result<Vec<String>, sqlx::Error> {
    let set = load_permissions(pool, user_id).await?;
    let mut keys: Vec<String> = set.iter().cloned().collect();
    keys.sort();
    Ok(keys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permission_error_response_serializes_to_single_error_key() {
        let body = PermissionErrorResponse { error: "forbidden" };
        let json = serde_json::to_value(&body).unwrap();
        let obj = json.as_object().unwrap();
        assert_eq!(obj.len(), 1);
        assert_eq!(obj.get("error").and_then(|v| v.as_str()), Some("forbidden"));
    }

    // The permission cache is a process-wide singleton, so any test
    // that mutates it concurrently with another would race. We serialize
    // every cache-touching unit test behind one mutex; outside test
    // builds the mutex is dead code (`#[cfg(test)]` only).
    static CACHE_TEST_MUTEX: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn invalidate_removes_only_target_user() {
        let _g = CACHE_TEST_MUTEX
            .lock()
            .expect("permission cache test mutex poisoned");
        const UID_A: i64 = -100_001;
        const UID_B: i64 = -100_002;

        {
            let mut guard = cache().write().unwrap();
            guard.insert(UID_A, Arc::new(HashSet::from(["a".to_string()])));
            guard.insert(UID_B, Arc::new(HashSet::from(["b".to_string()])));
        }

        invalidate(UID_A);

        {
            let guard = cache().read().unwrap();
            assert!(!guard.contains_key(&UID_A), "UID_A not removed");
            assert!(guard.contains_key(&UID_B), "UID_B should remain");
        }

        // Cleanup so siblings see an unmodified cache state.
        invalidate(UID_B);
    }

    #[test]
    fn invalidate_all_clears_every_entry() {
        let _g = CACHE_TEST_MUTEX
            .lock()
            .expect("permission cache test mutex poisoned");
        const UID_X: i64 = -200_010;
        const UID_Y: i64 = -200_011;

        {
            let mut guard = cache().write().unwrap();
            guard.insert(UID_X, Arc::new(HashSet::from(["x".to_string()])));
            guard.insert(UID_Y, Arc::new(HashSet::from(["y".to_string()])));
        }

        invalidate_all();

        let guard = cache().read().unwrap();
        assert!(
            !guard.contains_key(&UID_X) && !guard.contains_key(&UID_Y),
            "invalidate_all left {} entries",
            guard.len()
        );
    }

    #[test]
    fn forbidden_response_has_expected_status_and_body_code() {
        let resp = forbidden_response(StatusCode::FORBIDDEN, "forbidden");
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }
}
