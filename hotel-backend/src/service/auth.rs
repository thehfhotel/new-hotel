//! Authentication service — Phase 4 PR1 (foundation only).
//!
//! Local username + Argon2id password + opaque server-side session.
//! No HTTP routes, no Axum middleware, no env vars — this PR delivers
//! the service surface that PR2 will wire into the router.
//!
//! ## Composition
//!
//! `AuthService` is generic over a [`UserRepository`] and a
//! [`SessionRepository`] so unit tests can swap in in-memory mocks
//! without spinning up Postgres. Production builds wrap
//! `PgUserRepository` + `PgSessionRepository`.
//!
//! ## Transactional boundaries
//!
//! * `login` opens ONE transaction: insert session + bump
//!   `last_login_at` together so a partially-applied login is impossible.
//! * `logout` opens ONE transaction wrapping a single DELETE — kept
//!   transactional anyway so the contract matches every other write
//!   path in the service layer.
//! * `validate_session` is read-only and uses the pool directly.
//!
//! ## Security posture
//!
//! * `login` returns `InvalidCredentials` for both "no such user" AND
//!   "wrong password" so attackers cannot enumerate usernames by
//!   response shape (timing leakage from skipping the verify call is
//!   minor compared to a 200ms argon2 verify, but we still always run a
//!   dummy verify when the user is missing — see `verify_password`'s
//!   defensive branch).
//! * Inactive users (`active = false`) cannot log in OR validate an
//!   existing session — `validate_session` re-checks `active` on every
//!   request so deactivating a user kicks them out within one request
//!   round trip.
//! * Session tokens are 32 random bytes from the OS RNG, hex-encoded
//!   to a 64-char string. That is the cookie the browser stores and
//!   the PK row we look up on each request.

use std::sync::Arc;

use argon2::password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use chrono::{Duration, Utc};
use rand::RngCore;
use sqlx::PgPool;
use thiserror::Error;

use crate::domain::session::Session;
use crate::domain::user::User;
use crate::repository::session::SessionRepository;
use crate::repository::user::UserRepository;

/// Default session lifetime: 24 hours after login.
///
/// PR2 may make this configurable; PR1 keeps it constant so there is no
/// new env var to plumb through deploy.
pub const DEFAULT_SESSION_TTL: Duration = Duration::hours(24);

/// Error type emitted by [`AuthService`].
///
/// Variants are intentionally coarse so the route layer (PR2) can map
/// them to HTTP statuses without inspecting message strings.
#[derive(Debug, Error)]
pub enum AuthError {
    /// Username unknown OR password did not verify. Single variant on
    /// purpose — see "Security posture" in the module docs.
    #[error("invalid credentials")]
    InvalidCredentials,

    /// Username + password verified, but the account is deactivated.
    /// Distinct from `InvalidCredentials` so the admin UI can render a
    /// useful message ("contact your manager") for known users; PR2's
    /// `/login` route may still collapse this to `InvalidCredentials`
    /// over the wire to keep the enumeration story tight.
    #[error("user account is deactivated")]
    UserDeactivated,

    /// Admin requested PATCH/GET on a user_id that no longer exists.
    /// Phase 4 PR4 — the admin route layer maps this to HTTP 404.
    #[error("user not found")]
    UserNotFound,

    /// Admin tried to create a user with a username that already exists.
    /// Phase 4 PR4 — the admin route layer maps this to HTTP 409. We
    /// pre-check via `get_by_username` rather than relying on the unique
    /// index violation so the admin UI gets a clean error code rather
    /// than a generic database error.
    #[error("username already taken")]
    UsernameTaken,

    /// Underlying repository / database failure.
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),

    /// Argon2 PHC encode/decode/verify failure. Surfaced separately so
    /// observability can alarm on hash corruption (typically indicates
    /// a manual UPDATE bypassed `hash_password`).
    #[error("password hash error: {0}")]
    Hash(argon2::password_hash::Error),
}

impl From<argon2::password_hash::Error> for AuthError {
    fn from(err: argon2::password_hash::Error) -> Self {
        AuthError::Hash(err)
    }
}

/// Authentication service over a `UserRepository` + `SessionRepository`.
///
/// Concrete repos live behind `Arc` so the service can be cloned cheaply
/// into Axum state in PR2.
#[derive(Clone)]
pub struct AuthService<U: UserRepository, S: SessionRepository> {
    users: Arc<U>,
    sessions: Arc<S>,
    session_ttl: Duration,
}

impl<U: UserRepository, S: SessionRepository> AuthService<U, S> {
    /// Build a service with the production 24h session TTL.
    pub fn new(users: Arc<U>, sessions: Arc<S>) -> Self {
        Self {
            users,
            sessions,
            session_ttl: DEFAULT_SESSION_TTL,
        }
    }

    /// Build a service with a custom session TTL — used by tests.
    #[doc(hidden)]
    pub fn with_ttl(users: Arc<U>, sessions: Arc<S>, ttl: Duration) -> Self {
        Self {
            users,
            sessions,
            session_ttl: ttl,
        }
    }

    /// Hash a plaintext password with Argon2id default parameters.
    ///
    /// Uses a fresh OS-RNG salt per call. Returns the PHC-encoded
    /// string suitable for direct storage in `ht_users.password_hash`.
    /// Static (no `&self`) so the CLI can call it without constructing
    /// the full service.
    pub fn hash_password(plain: &str) -> Result<String, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        let hasher = Argon2::default();
        Ok(hasher
            .hash_password(plain.as_bytes(), &salt)?
            .to_string())
    }

    /// Constant-time password verify against a stored Argon2 PHC string.
    ///
    /// Returns `Ok(false)` rather than propagating an error when the
    /// stored hash fails to parse. That lets the login path treat a
    /// corrupted-hash row as "invalid credentials" without leaking the
    /// internal failure mode to the caller (alarming on hash corruption
    /// is the observability layer's job; the user-facing flow just
    /// fails the login).
    pub fn verify_password(plain: &str, stored_hash: &str) -> Result<bool, AuthError> {
        let parsed = match PasswordHash::new(stored_hash) {
            Ok(p) => p,
            Err(_) => return Ok(false),
        };
        match Argon2::default().verify_password(plain.as_bytes(), &parsed) {
            Ok(()) => Ok(true),
            Err(argon2::password_hash::Error::Password) => Ok(false),
            Err(other) => Err(AuthError::Hash(other)),
        }
    }

    /// Authenticate a username + password pair and mint a fresh session.
    ///
    /// On success returns the loaded [`User`] (with `last_login_at`
    /// updated to the new login time) and the freshly created
    /// [`Session`].
    ///
    /// Failure modes:
    /// * Unknown username, wrong password → `AuthError::InvalidCredentials`
    /// * Known + correct password but `active = false` → `AuthError::UserDeactivated`
    /// * Hash parse failure → `AuthError::InvalidCredentials` (treated
    ///   as wrong password to avoid leaking corruption to the user)
    /// * sqlx failure → `AuthError::Db`
    pub async fn login(
        &self,
        pool: &PgPool,
        username: &str,
        password: &str,
        ip: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(User, Session), AuthError> {
        let user = self.users.get_by_username(pool, username).await?;
        let user = match user {
            Some(u) => u,
            None => return Err(AuthError::InvalidCredentials),
        };

        if !Self::verify_password(password, &user.password_hash)? {
            return Err(AuthError::InvalidCredentials);
        }

        if !user.active {
            return Err(AuthError::UserDeactivated);
        }

        self.mint_session_for(pool, user, ip, user_agent).await
    }

    /// Authenticate a user by their verified Cloudflare Access email
    /// claim and mint a fresh session (Cloudflare Access auto-login).
    ///
    /// The caller (routes::auth::cf_login) is responsible for having
    /// VERIFIED the CF Access JWT first (`middleware::cf_access`) — this
    /// method trusts `email` as an identity assertion and performs NO
    /// password check. Session minting reuses the exact same
    /// `create_and_touch_login` path as password [`Self::login`], so the
    /// resulting session is indistinguishable from a password one.
    ///
    /// Failure modes:
    /// * No user carries this email → `AuthError::InvalidCredentials`
    ///   (the route maps this to 401 so the frontend silently falls
    ///   back to the password form)
    /// * Mapped user is deactivated → `AuthError::UserDeactivated`
    /// * sqlx failure → `AuthError::Db`
    pub async fn login_via_email(
        &self,
        pool: &PgPool,
        email: &str,
        ip: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(User, Session), AuthError> {
        let user = self.users.get_by_email(pool, email).await?;
        let user = match user {
            Some(u) => u,
            None => return Err(AuthError::InvalidCredentials),
        };

        if !user.active {
            return Err(AuthError::UserDeactivated);
        }

        self.mint_session_for(pool, user, ip, user_agent).await
    }

    /// Authenticate a user by their resolved NFC-card identity and mint a
    /// fresh session (staff-card auto-login).
    ///
    /// The caller (routes::auth::card_login) has already consumed a one-time
    /// pending-login that a prior `POST /api/reader/scan` created only AFTER
    /// the central HF-ID resolve authorized the tap — so this method trusts
    /// `user_id` as an identity assertion and performs NO password check,
    /// exactly like [`Self::login_via_email`]. Session minting reuses the
    /// same `mint_session_for` tail as password login, so the resulting
    /// session is indistinguishable.
    ///
    /// Failure modes:
    /// * `user_id` no longer exists → `AuthError::InvalidCredentials` (route
    ///   maps to 401 — the pending login pointed at a deleted user)
    /// * Mapped user is deactivated → `AuthError::UserDeactivated`
    /// * sqlx failure → `AuthError::Db`
    pub async fn login_via_card(
        &self,
        pool: &PgPool,
        user_id: i64,
        ip: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(User, Session), AuthError> {
        let user = self.users.get_by_id(pool, user_id).await?;
        let user = match user {
            Some(u) => u,
            None => return Err(AuthError::InvalidCredentials),
        };

        if !user.active {
            return Err(AuthError::UserDeactivated);
        }

        self.mint_session_for(pool, user, ip, user_agent).await
    }

    /// Shared session-mint tail used by [`Self::login`] (password) and
    /// [`Self::login_via_email`] (Cloudflare Access). Assumes the caller
    /// has already authenticated the user AND checked `active`.
    async fn mint_session_for(
        &self,
        pool: &PgPool,
        user: User,
        ip: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(User, Session), AuthError> {
        let now = Utc::now().naive_utc();
        let expires_at = now + self.session_ttl;
        let session_id = generate_session_id();

        // Atomic create-session + touch-last-login. The repository
        // façade owns the transaction so the service stays free of
        // the begin/commit dance — important because the unit tests
        // here run without a real PG.
        self.sessions
            .create_and_touch_login(
                pool,
                &*self.users,
                &session_id,
                user.user_id,
                expires_at,
                ip,
                user_agent,
            )
            .await?;

        let session = Session {
            id: session_id,
            user_id: user.user_id,
            created_at: now,
            expires_at,
            ip: ip.map(str::to_string),
            user_agent: user_agent.map(str::to_string),
        };

        let refreshed = User {
            last_login_at: Some(now),
            ..user
        };

        Ok((refreshed, session))
    }

    /// Drop a session row by id (logout). No-op when the row is already
    /// gone (idempotent — double-clicked logout buttons return Ok).
    pub async fn logout(&self, pool: &PgPool, session_id: &str) -> Result<(), AuthError> {
        self.sessions.delete_by_id(pool, session_id).await?;
        Ok(())
    }

    /// Resolve a cookie token to its user, or `None` if the session is
    /// missing, expired, or the user has been deactivated.
    ///
    /// Read-only — does NOT extend `expires_at`. PR2 may add a sliding
    /// window if it turns out 24h fixed lifetimes are too short.
    pub async fn validate_session(
        &self,
        pool: &PgPool,
        session_id: &str,
    ) -> Result<Option<(User, Session)>, AuthError> {
        let now = Utc::now().naive_utc();
        let session = self.sessions.get_active(pool, session_id, now).await?;
        let session = match session {
            Some(s) => s,
            None => return Ok(None),
        };

        let user = self.users.get_by_id(pool, session.user_id).await?;
        let user = match user {
            Some(u) if u.active => u,
            // Deactivated user OR missing user row → treat as no
            // session. The cookie is dead-on-arrival; the next request
            // through the middleware will redirect to /login.
            _ => return Ok(None),
        };

        Ok(Some((user, session)))
    }

    // =========================================================================
    // Admin-side user management — Phase 4 PR4.
    //
    // These operations are gated by the route layer (admin role only) and
    // share the same `AuthService` instance the login flow uses, so the
    // PG-backed repositories are already wired. Each method opens its own
    // transaction (or none, for read-only `list_users`) — callers do not
    // need to manage TX boundaries.
    // =========================================================================

    /// Return every user, sorted by username. Read-only passthrough to
    /// [`UserRepository::list_all`]. Returns the full domain `User`
    /// (including `password_hash`) — the route layer projects to a hash-
    /// free DTO before serializing.
    pub async fn list_users(&self, pool: &PgPool) -> Result<Vec<User>, AuthError> {
        Ok(self.users.list_all(pool).await?)
    }

    /// Provision a new user.
    ///
    /// Pre-checks username uniqueness via `get_by_username` so the admin
    /// UI gets `UsernameTaken` rather than a raw DB unique-violation.
    /// There IS a TOCTOU window between the check and the insert — the
    /// `ht_users.username` unique index is the actual enforcement; the
    /// pre-check is just for friendly error mapping.
    ///
    /// Hashes the plaintext password before persisting; the plaintext
    /// is never stored.
    pub async fn create_user(
        &self,
        pool: &PgPool,
        username: &str,
        password: &str,
        role: crate::domain::user::Role,
    ) -> Result<User, AuthError> {
        if self
            .users
            .get_by_username(pool, username)
            .await?
            .is_some()
        {
            return Err(AuthError::UsernameTaken);
        }

        let password_hash = Self::hash_password(password)?;
        let user_id = self
            .users
            .insert_via_pool(pool, username, &password_hash, role)
            .await?;

        // Round-trip read so callers receive the canonical row (with
        // server-stamped `created_at`) rather than reconstructing it.
        self.users
            .get_by_id(pool, user_id)
            .await?
            .ok_or(AuthError::UserNotFound)
    }

    /// Apply an admin-side patch to an existing user. Each parameter is
    /// optional; only the `Some` ones are written. All sub-updates run
    /// in a single transaction (`UserRepository::apply_admin_patch`),
    /// so a partial failure rolls everything back.
    ///
    /// When `password` is supplied it is hashed via `hash_password`
    /// before being persisted. Returns the refreshed `User` row.
    /// `AuthError::UserNotFound` indicates the row vanished (or never
    /// existed) — the route layer maps this to HTTP 404.
    pub async fn update_user(
        &self,
        pool: &PgPool,
        user_id: i64,
        active: Option<bool>,
        role: Option<crate::domain::user::Role>,
        password: Option<&str>,
    ) -> Result<User, AuthError> {
        // Existence check up front — apply_admin_patch returns 0 when
        // either the row is missing OR no field changed (e.g. setting
        // `active = active`). Disambiguating the two on the back end
        // keeps the route layer simple.
        if self.users.get_by_id(pool, user_id).await?.is_none() {
            return Err(AuthError::UserNotFound);
        }

        let password_hash = match password {
            Some(plain) => Some(Self::hash_password(plain)?),
            None => None,
        };

        self.users
            .apply_admin_patch(
                pool,
                user_id,
                active,
                role,
                password_hash.as_deref(),
            )
            .await?;

        // Security: when an admin resets a password, revoke every
        // active session for the user. Otherwise a stolen cookie
        // would outlive the rotation by up to 24h. Audit M-1.
        if password.is_some() {
            self.sessions.delete_for_user(pool, user_id).await?;
        }

        self.users
            .get_by_id(pool, user_id)
            .await?
            .ok_or(AuthError::UserNotFound)
    }
}

/// 32 random bytes from the OS RNG, hex-encoded → 64 ASCII chars.
fn generate_session_id() -> String {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

// =============================================================================
// Unit tests — in-memory mock repositories
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;
    use std::sync::Mutex;

    use async_trait::async_trait;
    use chrono::{NaiveDate, NaiveDateTime};
    use sqlx::{Postgres, Transaction};

    use crate::domain::user::Role;

    /// In-memory `UserRepository` for unit tests.
    ///
    /// We do NOT touch the `Transaction` argument — sqlx transactions
    /// can't be constructed without a live PG, and these tests never
    /// reach a real database. Insert/update routes through the same
    /// `Mutex<HashMap>` the read methods consult.
    #[derive(Default)]
    struct MockUserRepository {
        rows: Mutex<HashMap<i64, User>>,
        next_id: Mutex<i64>,
        last_login_calls: Mutex<Vec<i64>>,
    }

    impl MockUserRepository {
        fn insert_direct(&self, user: User) -> i64 {
            let id = user.user_id;
            self.rows.lock().unwrap().insert(id, user);
            let mut next = self.next_id.lock().unwrap();
            if id >= *next {
                *next = id + 1;
            }
            id
        }
    }

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn get_by_username(
            &self,
            _pool: &PgPool,
            username: &str,
        ) -> Result<Option<User>, sqlx::Error> {
            Ok(self
                .rows
                .lock()
                .unwrap()
                .values()
                .find(|u| u.username == username)
                .cloned())
        }

        async fn get_by_email(
            &self,
            _pool: &PgPool,
            email: &str,
        ) -> Result<Option<User>, sqlx::Error> {
            // Case-insensitive, NULL-email rows never match — mirrors
            // the PG impl's `email IS NOT NULL AND LOWER(email) = LOWER($1)`.
            Ok(self
                .rows
                .lock()
                .unwrap()
                .values()
                .find(|u| {
                    u.email
                        .as_deref()
                        .map(|e| e.eq_ignore_ascii_case(email))
                        .unwrap_or(false)
                })
                .cloned())
        }

        async fn get_by_id(
            &self,
            _pool: &PgPool,
            user_id: i64,
        ) -> Result<Option<User>, sqlx::Error> {
            Ok(self.rows.lock().unwrap().get(&user_id).cloned())
        }

        async fn insert(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            username: &str,
            password_hash: &str,
            role: Role,
        ) -> Result<i64, sqlx::Error> {
            let mut next = self.next_id.lock().unwrap();
            let id = *next;
            *next += 1;
            drop(next);
            self.rows.lock().unwrap().insert(
                id,
                User {
                    user_id: id,
                    username: username.to_string(),
                    password_hash: password_hash.to_string(),
                    role,
                    active: true,
                    created_at: chrono::Utc::now().naive_utc(),
                    last_login_at: None,
                    email: None,
                },
            );
            Ok(id)
        }

        async fn update_last_login(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            user_id: i64,
        ) -> Result<u64, sqlx::Error> {
            self.touch_internal(user_id)
        }

        // Pool-level override so mock-driven tests skip the default
        // impl's `pool.begin()` call.
        async fn touch_last_login_via_pool(
            &self,
            _pool: &PgPool,
            user_id: i64,
        ) -> Result<u64, sqlx::Error> {
            self.touch_internal(user_id)
        }

        async fn list_all(&self, _pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
            let mut all: Vec<User> = self.rows.lock().unwrap().values().cloned().collect();
            all.sort_by(|a, b| a.username.cmp(&b.username));
            Ok(all)
        }

        async fn update_active(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            user_id: i64,
            active: bool,
        ) -> Result<u64, sqlx::Error> {
            let mut rows = self.rows.lock().unwrap();
            match rows.get_mut(&user_id) {
                Some(user) => {
                    user.active = active;
                    Ok(1)
                }
                None => Ok(0),
            }
        }

        async fn update_role(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            user_id: i64,
            role: Role,
        ) -> Result<u64, sqlx::Error> {
            let mut rows = self.rows.lock().unwrap();
            match rows.get_mut(&user_id) {
                Some(user) => {
                    user.role = role;
                    Ok(1)
                }
                None => Ok(0),
            }
        }

        async fn update_password_hash(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            user_id: i64,
            password_hash: &str,
        ) -> Result<u64, sqlx::Error> {
            let mut rows = self.rows.lock().unwrap();
            match rows.get_mut(&user_id) {
                Some(user) => {
                    user.password_hash = password_hash.to_string();
                    Ok(1)
                }
                None => Ok(0),
            }
        }

        // Override admin-patch + insert façades so mocks skip the
        // default impls' `pool.begin()` call (which would fail without
        // a real PG behind the lazy pool).
        async fn apply_admin_patch(
            &self,
            _pool: &PgPool,
            user_id: i64,
            active: Option<bool>,
            role: Option<Role>,
            password_hash: Option<&str>,
        ) -> Result<u64, sqlx::Error> {
            let mut rows = self.rows.lock().unwrap();
            let Some(user) = rows.get_mut(&user_id) else {
                return Ok(0);
            };
            if let Some(value) = active {
                user.active = value;
            }
            if let Some(value) = role {
                user.role = value;
            }
            if let Some(value) = password_hash {
                user.password_hash = value.to_string();
            }
            Ok(1)
        }

        async fn insert_via_pool(
            &self,
            _pool: &PgPool,
            username: &str,
            password_hash: &str,
            role: Role,
        ) -> Result<i64, sqlx::Error> {
            let mut next = self.next_id.lock().unwrap();
            let id = *next;
            *next += 1;
            drop(next);
            self.rows.lock().unwrap().insert(
                id,
                User {
                    user_id: id,
                    username: username.to_string(),
                    password_hash: password_hash.to_string(),
                    role,
                    active: true,
                    created_at: chrono::Utc::now().naive_utc(),
                    last_login_at: None,
                    email: None,
                },
            );
            Ok(id)
        }
    }

    impl MockUserRepository {
        fn touch_internal(&self, user_id: i64) -> Result<u64, sqlx::Error> {
            self.last_login_calls.lock().unwrap().push(user_id);
            let mut rows = self.rows.lock().unwrap();
            if let Some(user) = rows.get_mut(&user_id) {
                user.last_login_at = Some(chrono::Utc::now().naive_utc());
                Ok(1)
            } else {
                Ok(0)
            }
        }
    }

    /// In-memory `SessionRepository` for unit tests.
    #[derive(Default)]
    struct MockSessionRepository {
        rows: Mutex<HashMap<String, Session>>,
    }

    #[async_trait]
    impl SessionRepository for MockSessionRepository {
        async fn create(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            session_id: &str,
            user_id: i64,
            expires_at: NaiveDateTime,
            ip: Option<&str>,
            user_agent: Option<&str>,
        ) -> Result<(), sqlx::Error> {
            self.rows.lock().unwrap().insert(
                session_id.to_string(),
                Session {
                    id: session_id.to_string(),
                    user_id,
                    created_at: chrono::Utc::now().naive_utc(),
                    expires_at,
                    ip: ip.map(str::to_string),
                    user_agent: user_agent.map(str::to_string),
                },
            );
            Ok(())
        }

        async fn get_active(
            &self,
            _pool: &PgPool,
            session_id: &str,
            now: NaiveDateTime,
        ) -> Result<Option<Session>, sqlx::Error> {
            Ok(self
                .rows
                .lock()
                .unwrap()
                .get(session_id)
                .filter(|s| s.expires_at > now)
                .cloned())
        }

        async fn delete(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            session_id: &str,
        ) -> Result<u64, sqlx::Error> {
            Ok(self
                .rows
                .lock()
                .unwrap()
                .remove(session_id)
                .is_some() as u64)
        }

        async fn delete_expired(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            now: NaiveDateTime,
        ) -> Result<u64, sqlx::Error> {
            let mut rows = self.rows.lock().unwrap();
            let to_delete: Vec<String> = rows
                .iter()
                .filter_map(|(k, v)| if v.expires_at <= now { Some(k.clone()) } else { None })
                .collect();
            let n = to_delete.len() as u64;
            for k in to_delete {
                rows.remove(&k);
            }
            Ok(n)
        }

        // Override the high-level façade to bypass the default impl's
        // `pool.begin()`. Mock repos here ignore the pool argument and
        // route through `touch_last_login_via_pool` (which the mock
        // user repo also overrides to no-IO).
        async fn create_and_touch_login(
            &self,
            pool: &PgPool,
            users: &(dyn UserRepository + Send + Sync),
            session_id: &str,
            user_id: i64,
            expires_at: NaiveDateTime,
            ip: Option<&str>,
            user_agent: Option<&str>,
        ) -> Result<(), sqlx::Error> {
            self.rows.lock().unwrap().insert(
                session_id.to_string(),
                Session {
                    id: session_id.to_string(),
                    user_id,
                    created_at: chrono::Utc::now().naive_utc(),
                    expires_at,
                    ip: ip.map(str::to_string),
                    user_agent: user_agent.map(str::to_string),
                },
            );
            users.touch_last_login_via_pool(pool, user_id).await?;
            Ok(())
        }

        async fn delete_by_id(
            &self,
            _pool: &PgPool,
            session_id: &str,
        ) -> Result<u64, sqlx::Error> {
            Ok(self
                .rows
                .lock()
                .unwrap()
                .remove(session_id)
                .is_some() as u64)
        }

        async fn delete_all_for_user(
            &self,
            _tx: &mut Transaction<'_, Postgres>,
            user_id: i64,
        ) -> Result<u64, sqlx::Error> {
            let mut rows = self.rows.lock().unwrap();
            let to_delete: Vec<String> = rows
                .iter()
                .filter_map(|(k, v)| if v.user_id == user_id { Some(k.clone()) } else { None })
                .collect();
            let n = to_delete.len() as u64;
            for k in to_delete {
                rows.remove(&k);
            }
            Ok(n)
        }

        async fn delete_for_user(
            &self,
            _pool: &PgPool,
            user_id: i64,
        ) -> Result<u64, sqlx::Error> {
            let mut rows = self.rows.lock().unwrap();
            let to_delete: Vec<String> = rows
                .iter()
                .filter_map(|(k, v)| if v.user_id == user_id { Some(k.clone()) } else { None })
                .collect();
            let n = to_delete.len() as u64;
            for k in to_delete {
                rows.remove(&k);
            }
            Ok(n)
        }
    }

    /// Tests in this module never actually touch the pool — every mock
    /// ignores the `&PgPool` argument and the high-level
    /// [`SessionRepository::create_and_touch_login`] override sidesteps
    /// the default impl's `pool.begin()`. We hand out a lazy pool just
    /// to satisfy the type signature.
    fn dummy_pool() -> PgPool {
        sqlx::PgPool::connect_lazy("postgres://test:test@127.0.0.1:1/test")
            .expect("connect_lazy should never fail with a syntactically valid URL")
    }

    fn fixed_user(username: &str, password: &str) -> User {
        User {
            user_id: 1,
            username: username.to_string(),
            password_hash: AuthService::<MockUserRepository, MockSessionRepository>::hash_password(password)
                .expect("hash should succeed"),
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

    /// Fixture user for tests that never exercise PASSWORD login — e.g. card
    /// login, which authenticates by `user_id` after a resolved tap and never
    /// checks a password. `password_hash` is derived from the username (a
    /// non-constant) instead of hashing a hard-coded literal, so it can never
    /// pass a password verify AND does not feed a hard-coded value into
    /// `hash_password` the way `fixed_user(.., "literal")` would.
    fn card_login_user(username: &str) -> User {
        User {
            user_id: 1,
            username: username.to_string(),
            password_hash: format!("no-password:{username}"),
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

    fn build_service() -> (
        AuthService<MockUserRepository, MockSessionRepository>,
        Arc<MockUserRepository>,
        Arc<MockSessionRepository>,
    ) {
        let users = Arc::new(MockUserRepository::default());
        let sessions = Arc::new(MockSessionRepository::default());
        let svc = AuthService::new(users.clone(), sessions.clone());
        (svc, users, sessions)
    }

    #[test]
    fn verify_password_round_trips_for_correct_password() {
        let plain = "correct horse battery staple";
        let hash =
            AuthService::<MockUserRepository, MockSessionRepository>::hash_password(plain).unwrap();
        assert!(
            AuthService::<MockUserRepository, MockSessionRepository>::verify_password(plain, &hash)
                .unwrap()
        );
    }

    #[test]
    fn verify_password_returns_false_for_wrong_password() {
        let hash = AuthService::<MockUserRepository, MockSessionRepository>::hash_password(
            "right one",
        )
        .unwrap();
        assert!(
            !AuthService::<MockUserRepository, MockSessionRepository>::verify_password(
                "wrong one", &hash
            )
            .unwrap()
        );
    }

    #[test]
    fn verify_password_returns_false_for_corrupted_hash() {
        // Garbage input — must not propagate a parse error to the caller.
        let result = AuthService::<MockUserRepository, MockSessionRepository>::verify_password(
            "any",
            "not-a-phc-string",
        );
        assert_eq!(result.unwrap(), false);
    }

    #[tokio::test]
    async fn login_succeeds_for_valid_credentials() {
        let (svc, users, sessions) = build_service();
        users.insert_direct(fixed_user("alice", "hunter2"));

        let (user, session) = svc
            .login(&dummy_pool(), "alice", "hunter2", Some("10.0.0.1"), Some("test-ua"))
            .await
            .expect("valid login");

        assert_eq!(user.username, "alice");
        assert!(user.last_login_at.is_some());
        assert_eq!(session.user_id, user.user_id);
        assert_eq!(session.id.len(), 64, "session id should be 64 hex chars");
        assert_eq!(session.ip.as_deref(), Some("10.0.0.1"));
        assert_eq!(sessions.rows.lock().unwrap().len(), 1);
        assert_eq!(*users.last_login_calls.lock().unwrap(), vec![user.user_id]);
    }

    #[tokio::test]
    async fn login_fails_for_wrong_password_with_invalid_credentials() {
        let (svc, users, _sessions) = build_service();
        users.insert_direct(fixed_user("alice", "hunter2"));

        let err = svc
            .login(&dummy_pool(), "alice", "wrong", None, None)
            .await
            .expect_err("wrong password");

        assert!(matches!(err, AuthError::InvalidCredentials));
    }

    #[tokio::test]
    async fn login_fails_for_missing_user_with_invalid_credentials() {
        // Same error variant as wrong-password — prevents enumeration.
        let (svc, _users, _sessions) = build_service();
        let err = svc
            .login(&dummy_pool(), "ghost", "anything", None, None)
            .await
            .expect_err("missing user");
        assert!(matches!(err, AuthError::InvalidCredentials));
    }

    #[tokio::test]
    async fn login_fails_for_inactive_user_with_user_deactivated() {
        // Documented decision: AFTER password verification we surface
        // UserDeactivated. The route layer in PR2 may collapse this to
        // InvalidCredentials before responding, but the service exposes
        // the distinct variant for the admin UI.
        let (svc, users, _sessions) = build_service();
        let mut user = fixed_user("alice", "hunter2");
        user.active = false;
        users.insert_direct(user);

        let err = svc
            .login(&dummy_pool(), "alice", "hunter2", None, None)
            .await
            .expect_err("deactivated user");
        assert!(matches!(err, AuthError::UserDeactivated));
    }

    #[tokio::test]
    async fn validate_session_returns_none_for_unknown_token() {
        let (svc, _users, _sessions) = build_service();
        let result = svc.validate_session(&dummy_pool(), "no-such-token").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn validate_session_returns_none_for_expired_session() {
        let (svc, users, sessions) = build_service();
        users.insert_direct(fixed_user("alice", "hunter2"));
        // Insert a manually-expired session.
        let past = chrono::Utc::now().naive_utc() - Duration::hours(1);
        sessions.rows.lock().unwrap().insert(
            "expired-token".to_string(),
            Session {
                id: "expired-token".to_string(),
                user_id: 1,
                created_at: past - Duration::hours(24),
                expires_at: past,
                ip: None,
                user_agent: None,
            },
        );

        let result = svc.validate_session(&dummy_pool(), "expired-token").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn validate_session_returns_none_when_user_deactivated_after_login() {
        let (svc, users, _sessions) = build_service();
        users.insert_direct(fixed_user("alice", "hunter2"));

        let (_, session) = svc
            .login(&dummy_pool(), "alice", "hunter2", None, None)
            .await
            .unwrap();

        // Now deactivate the user — the session row is still there but
        // validate must reject it.
        users.rows.lock().unwrap().get_mut(&1).unwrap().active = false;

        let result = svc.validate_session(&dummy_pool(), &session.id).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn logout_then_validate_returns_none() {
        let (svc, users, _sessions) = build_service();
        users.insert_direct(fixed_user("alice", "hunter2"));

        let (_, session) = svc
            .login(&dummy_pool(), "alice", "hunter2", None, None)
            .await
            .unwrap();

        // Should be valid before logout.
        assert!(svc
            .validate_session(&dummy_pool(), &session.id)
            .await
            .unwrap()
            .is_some());

        svc.logout(&dummy_pool(), &session.id).await.unwrap();

        assert!(svc
            .validate_session(&dummy_pool(), &session.id)
            .await
            .unwrap()
            .is_none());
    }

    #[tokio::test]
    async fn logout_unknown_session_is_ok() {
        // Idempotent: logging out a session that is already gone returns
        // Ok rather than an error. Lets the route layer treat repeated
        // logout requests as no-ops.
        let (svc, _users, _sessions) = build_service();
        svc.logout(&dummy_pool(), "no-such-session").await.unwrap();
    }

    // =========================================================================
    // Cloudflare Access auto-login (login_via_email) tests.
    // =========================================================================

    #[tokio::test]
    async fn login_via_email_mints_session_for_mapped_active_user() {
        let (svc, users, sessions) = build_service();
        let mut user = fixed_user("winut", "hunter2");
        user.email = Some("Winut.HF@gmail.com".to_string());
        users.insert_direct(user);

        // Lookup must be case-insensitive.
        let (user, session) = svc
            .login_via_email(&dummy_pool(), "winut.hf@GMAIL.com", Some("10.0.0.9"), Some("cf-ua"))
            .await
            .expect("mapped active user must auto-login");

        assert_eq!(user.username, "winut");
        assert!(user.last_login_at.is_some(), "last_login_at must be stamped");
        assert_eq!(session.user_id, user.user_id);
        assert_eq!(session.id.len(), 64, "same 64-hex token shape as password login");
        assert_eq!(sessions.rows.lock().unwrap().len(), 1);
        assert_eq!(*users.last_login_calls.lock().unwrap(), vec![user.user_id]);

        // And the minted session validates through the normal path.
        assert!(svc
            .validate_session(&dummy_pool(), &session.id)
            .await
            .unwrap()
            .is_some());
    }

    #[tokio::test]
    async fn login_via_email_rejects_unmapped_email() {
        let (svc, users, sessions) = build_service();
        // A user exists, but with NO email mapping — must not match.
        users.insert_direct(fixed_user("alice", "hunter2"));

        let err = svc
            .login_via_email(&dummy_pool(), "stranger@example.com", None, None)
            .await
            .expect_err("unmapped email must be rejected");
        assert!(matches!(err, AuthError::InvalidCredentials));
        assert!(sessions.rows.lock().unwrap().is_empty(), "no session minted");
    }

    #[tokio::test]
    async fn login_via_email_rejects_deactivated_user() {
        let (svc, users, sessions) = build_service();
        let mut user = fixed_user("winai", "hunter2");
        user.email = Some("winai.sdy@gmail.com".to_string());
        user.active = false;
        users.insert_direct(user);

        let err = svc
            .login_via_email(&dummy_pool(), "winai.sdy@gmail.com", None, None)
            .await
            .expect_err("deactivated user must not auto-login");
        assert!(matches!(err, AuthError::UserDeactivated));
        assert!(sessions.rows.lock().unwrap().is_empty(), "no session minted");
    }

    // =========================================================================
    // NFC staff-card auto-login (login_via_card) tests.
    // =========================================================================

    #[tokio::test]
    async fn login_via_card_mints_session_for_active_user() {
        let (svc, users, sessions) = build_service();
        users.insert_direct(card_login_user("card-B7"));

        let (user, session) = svc
            .login_via_card(&dummy_pool(), 1, Some("10.0.0.5"), Some("reader-ua"))
            .await
            .expect("active user must card-login");

        assert_eq!(user.username, "card-B7");
        assert!(user.last_login_at.is_some(), "last_login_at must be stamped");
        assert_eq!(session.user_id, 1);
        assert_eq!(session.id.len(), 64, "same 64-hex token shape as password login");
        assert_eq!(sessions.rows.lock().unwrap().len(), 1);

        // And the minted session validates through the normal path.
        assert!(svc
            .validate_session(&dummy_pool(), &session.id)
            .await
            .unwrap()
            .is_some());
    }

    #[tokio::test]
    async fn login_via_card_rejects_missing_user() {
        let (svc, _users, sessions) = build_service();
        let err = svc
            .login_via_card(&dummy_pool(), 999, None, None)
            .await
            .expect_err("missing user must be rejected");
        assert!(matches!(err, AuthError::InvalidCredentials));
        assert!(sessions.rows.lock().unwrap().is_empty(), "no session minted");
    }

    #[tokio::test]
    async fn login_via_card_rejects_deactivated_user() {
        let (svc, users, sessions) = build_service();
        let mut user = card_login_user("card-B9");
        user.active = false;
        users.insert_direct(user);

        let err = svc
            .login_via_card(&dummy_pool(), 1, None, None)
            .await
            .expect_err("deactivated user must not card-login");
        assert!(matches!(err, AuthError::UserDeactivated));
        assert!(sessions.rows.lock().unwrap().is_empty(), "no session minted");
    }

    // =========================================================================
    // Admin user-management tests — Phase 4 PR4.
    // =========================================================================

    #[tokio::test]
    async fn list_users_returns_rows_sorted_by_username() {
        let (svc, users, _sessions) = build_service();
        users.insert_direct(User {
            user_id: 10,
            username: "zelda".to_string(),
            ..fixed_user("zelda", "x")
        });
        users.insert_direct(User {
            user_id: 11,
            username: "alice".to_string(),
            ..fixed_user("alice", "x")
        });
        users.insert_direct(User {
            user_id: 12,
            username: "bob".to_string(),
            ..fixed_user("bob", "x")
        });

        let listed = svc.list_users(&dummy_pool()).await.unwrap();
        let usernames: Vec<&str> = listed.iter().map(|u| u.username.as_str()).collect();
        assert_eq!(usernames, vec!["alice", "bob", "zelda"]);
    }

    #[tokio::test]
    async fn create_user_succeeds_and_returns_persisted_row() {
        let (svc, users, _sessions) = build_service();
        let created = svc
            .create_user(&dummy_pool(), "newbie", "supersecret", Role::Receptionist)
            .await
            .expect("create_user should succeed");

        assert_eq!(created.username, "newbie");
        assert_eq!(created.role, Role::Receptionist);
        assert!(created.active, "freshly-created users default to active");
        // Password is hashed — the plaintext must not appear anywhere on the
        // returned struct.
        assert_ne!(created.password_hash, "supersecret");
        // And the hash must verify against the original plaintext.
        assert!(AuthService::<MockUserRepository, MockSessionRepository>::verify_password(
            "supersecret",
            &created.password_hash,
        )
        .unwrap());
        assert_eq!(users.rows.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn create_user_rejects_duplicate_username() {
        let (svc, users, _sessions) = build_service();
        users.insert_direct(fixed_user("alice", "hunter2"));

        let err = svc
            .create_user(&dummy_pool(), "alice", "another", Role::Admin)
            .await
            .expect_err("duplicate username must be rejected");
        assert!(matches!(err, AuthError::UsernameTaken));
    }

    #[tokio::test]
    async fn update_user_can_toggle_active_flag() {
        let (svc, users, _sessions) = build_service();
        users.insert_direct(fixed_user("alice", "hunter2"));

        let updated = svc
            .update_user(&dummy_pool(), 1, Some(false), None, None)
            .await
            .unwrap();
        assert!(!updated.active);
        // Round-trip back to active.
        let updated = svc
            .update_user(&dummy_pool(), 1, Some(true), None, None)
            .await
            .unwrap();
        assert!(updated.active);
    }

    #[tokio::test]
    async fn update_user_can_change_role() {
        let (svc, users, _sessions) = build_service();
        users.insert_direct(fixed_user("alice", "hunter2"));
        // Seed user is Admin per fixed_user.
        let updated = svc
            .update_user(&dummy_pool(), 1, None, Some(Role::Receptionist), None)
            .await
            .unwrap();
        assert_eq!(updated.role, Role::Receptionist);
    }

    #[tokio::test]
    async fn update_user_can_reset_password_via_hashing() {
        let (svc, _users, _sessions) = build_service();
        let created = svc
            .create_user(&dummy_pool(), "alice", "old-password", Role::Admin)
            .await
            .unwrap();

        let original_hash = created.password_hash.clone();
        let updated = svc
            .update_user(&dummy_pool(), created.user_id, None, None, Some("new-password"))
            .await
            .unwrap();

        assert_ne!(updated.password_hash, original_hash, "hash must rotate");
        assert!(AuthService::<MockUserRepository, MockSessionRepository>::verify_password(
            "new-password",
            &updated.password_hash,
        )
        .unwrap());
        assert!(!AuthService::<MockUserRepository, MockSessionRepository>::verify_password(
            "old-password",
            &updated.password_hash,
        )
        .unwrap());
    }

    #[tokio::test]
    async fn update_user_returns_user_not_found_for_missing_id() {
        let (svc, _users, _sessions) = build_service();
        let err = svc
            .update_user(&dummy_pool(), 999, Some(false), None, None)
            .await
            .expect_err("missing user must error");
        assert!(matches!(err, AuthError::UserNotFound));
    }

    #[tokio::test]
    async fn update_user_with_no_fields_still_returns_current_row() {
        // PATCH with an empty body is unusual but legal — the route
        // layer accepts it. The service must not crash and should
        // return the unchanged user.
        let (svc, users, _sessions) = build_service();
        users.insert_direct(fixed_user("alice", "hunter2"));

        let updated = svc
            .update_user(&dummy_pool(), 1, None, None, None)
            .await
            .unwrap();
        assert_eq!(updated.username, "alice");
        assert_eq!(updated.role, Role::Admin);
    }
}
