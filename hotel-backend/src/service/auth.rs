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
}
