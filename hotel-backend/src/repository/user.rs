//! User repository — PostgreSQL data access for `ht_users` (Phase 4 PR1).
//!
//! Per `docs/architecture.md` §1, §2, §6. Read methods accept `&PgPool`,
//! write methods accept `&mut Transaction<'_, Postgres>` so the upcoming
//! `service::auth::AuthService` can compose insert + session-create + last-
//! login update inside a single atomic transaction.
//!
//! PR1 deliberately uses **dynamic** `sqlx::query()` rather than the
//! compile-time `query!()` macro. That keeps the agent from having to
//! regenerate the `.sqlx/` cache for new tables that don't yet exist on
//! the developer's local PG. PR2 may switch to typed macros once the
//! migrations are landed and CI is happy.

use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{PgPool, Postgres, Row, Transaction};

use crate::domain::user::{Role, User};

/// PostgreSQL data operations for the `User` aggregate.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Look up a user by username. Returns `None` for missing rows so
    /// the caller can map to `AuthError::InvalidCredentials` without
    /// distinguishing "no such user" from "wrong password" (prevents
    /// user enumeration via timing or response shape).
    async fn get_by_username(
        &self,
        pool: &PgPool,
        username: &str,
    ) -> Result<Option<User>, sqlx::Error>;

    /// Look up a user by their Cloudflare-Access email mapping key
    /// (migration 074). Matching is case-insensitive (`lower(email)`),
    /// mirroring the partial unique index `ux_ht_users_email_lower`.
    /// Returns `None` for missing rows AND for users with NULL email —
    /// callers map both to `AuthError::InvalidCredentials` so the
    /// cf-login route can answer 401 without distinguishing the cases.
    async fn get_by_email(
        &self,
        pool: &PgPool,
        email: &str,
    ) -> Result<Option<User>, sqlx::Error>;

    /// Look up a user by id. Used by `validate_session` to re-confirm
    /// the user is still active for every cookie-bound request.
    async fn get_by_id(
        &self,
        pool: &PgPool,
        user_id: i64,
    ) -> Result<Option<User>, sqlx::Error>;

    /// Insert a new user. Returns the freshly assigned `user_id`.
    /// Caller is responsible for hashing `password_hash` first; see
    /// [`crate::service::auth::AuthService::hash_password`].
    async fn insert(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        username: &str,
        password_hash: &str,
        role: Role,
    ) -> Result<i64, sqlx::Error>;

    /// Stamp `last_login_at = NOW()` for `user_id` inside an open
    /// transaction. Used by [`SessionRepository::create_and_touch_login`]
    /// so the bump and the session insert commit atomically.
    /// Returns rows affected so the caller can detect "user vanished
    /// mid-login" (the surrounding TX will see 0 and roll back).
    async fn update_last_login(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
    ) -> Result<u64, sqlx::Error>;

    /// Pool-level variant of [`Self::update_last_login`] used by the
    /// default impl of `SessionRepository::create_and_touch_login` and
    /// by mock repositories that cannot fabricate a `Transaction`.
    /// Default impl opens its own transaction; mocks override.
    async fn touch_last_login_via_pool(
        &self,
        pool: &PgPool,
        user_id: i64,
    ) -> Result<u64, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let n = self.update_last_login(&mut tx, user_id).await?;
        tx.commit().await?;
        Ok(n)
    }

    /// List every user row, ordered by `username` ASC.
    ///
    /// Used by the admin user-management page (Phase 4 PR4). Returns
    /// the full domain `User` (including `password_hash`) — the route
    /// layer is responsible for projecting to a hash-free DTO before
    /// sending to the wire. Read-only, takes the pool directly.
    async fn list_all(&self, pool: &PgPool) -> Result<Vec<User>, sqlx::Error>;

    /// Flip `active` for an existing user. Returns rows affected so the
    /// caller can detect "user vanished" (0) and surface a 404.
    async fn update_active(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        active: bool,
    ) -> Result<u64, sqlx::Error>;

    /// Replace `role` for an existing user. Returns rows affected so
    /// the caller can detect a missing user.
    async fn update_role(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        role: Role,
    ) -> Result<u64, sqlx::Error>;

    /// Replace `password_hash` for an existing user. The caller MUST
    /// have run the plaintext through [`crate::service::auth::AuthService::hash_password`]
    /// — this method does not hash. Returns rows affected.
    async fn update_password_hash(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        password_hash: &str,
    ) -> Result<u64, sqlx::Error>;

    /// High-level façade for admin-side user mutation: opens one
    /// transaction, applies whichever of `active` / `role` / `password_hash`
    /// are `Some`, commits. Returns the total number of UPDATE rows
    /// affected across all sub-statements so the caller can detect
    /// "user not found" (0 from every sub-update).
    ///
    /// Default impl chains the trait methods inside `pool.begin()`. Mock
    /// repos that ignore `Transaction` should override.
    async fn apply_admin_patch(
        &self,
        pool: &PgPool,
        user_id: i64,
        active: Option<bool>,
        role: Option<Role>,
        password_hash: Option<&str>,
    ) -> Result<u64, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let mut affected = 0u64;
        if let Some(value) = active {
            affected = affected.max(self.update_active(&mut tx, user_id, value).await?);
        }
        if let Some(value) = role {
            affected = affected.max(self.update_role(&mut tx, user_id, value).await?);
        }
        if let Some(value) = password_hash {
            affected = affected.max(self.update_password_hash(&mut tx, user_id, value).await?);
        }
        tx.commit().await?;
        Ok(affected)
    }

    /// High-level façade for `INSERT INTO ht_users`: opens one
    /// transaction, inserts, commits. Returns the new `user_id`. Mock
    /// repos that ignore `Transaction` should override.
    async fn insert_via_pool(
        &self,
        pool: &PgPool,
        username: &str,
        password_hash: &str,
        role: Role,
    ) -> Result<i64, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let id = self.insert(&mut tx, username, password_hash, role).await?;
        tx.commit().await?;
        Ok(id)
    }
}

/// Default `UserRepository` implementation backed by sqlx + PostgreSQL.
#[derive(Clone, Debug, Default)]
pub struct PgUserRepository;

impl PgUserRepository {
    pub fn new() -> Self {
        Self
    }
}

/// Convert a raw DB string into a `Role`, mapping unknown values to
/// `sqlx::Error::Decode` so callers see a typed error rather than a
/// silent default.
fn role_from_db(raw: &str) -> Result<Role, sqlx::Error> {
    Role::try_from(raw).map_err(|err| sqlx::Error::Decode(Box::new(err)))
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn get_by_username(
        &self,
        pool: &PgPool,
        username: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT user_id, username, password_hash, role, active,
                   created_at, last_login_at, email
            FROM ht_users
            WHERE username = $1
            "#,
        )
        .bind(username)
        .fetch_optional(pool)
        .await?;

        row.map(map_user_row).transpose()
    }

    async fn get_by_email(
        &self,
        pool: &PgPool,
        email: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT user_id, username, password_hash, role, active,
                   created_at, last_login_at, email
            FROM ht_users
            WHERE email IS NOT NULL AND LOWER(email) = LOWER($1)
            "#,
        )
        .bind(email)
        .fetch_optional(pool)
        .await?;

        row.map(map_user_row).transpose()
    }

    async fn get_by_id(
        &self,
        pool: &PgPool,
        user_id: i64,
    ) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT user_id, username, password_hash, role, active,
                   created_at, last_login_at, email
            FROM ht_users
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?;

        row.map(map_user_row).transpose()
    }

    async fn insert(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        username: &str,
        password_hash: &str,
        role: Role,
    ) -> Result<i64, sqlx::Error> {
        let row = sqlx::query(
            r#"
            INSERT INTO ht_users (username, password_hash, role)
            VALUES ($1, $2, $3)
            RETURNING user_id
            "#,
        )
        .bind(username)
        .bind(password_hash)
        .bind(role.as_str())
        .fetch_one(&mut **tx)
        .await?;

        row.try_get::<i64, _>("user_id")
    }

    async fn update_last_login(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            UPDATE ht_users
            SET last_login_at = NOW()
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .execute(&mut **tx)
        .await?;

        Ok(result.rows_affected())
    }

    async fn list_all(&self, pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        let rows = sqlx::query(
            r#"
            SELECT user_id, username, password_hash, role, active,
                   created_at, last_login_at, email
            FROM ht_users
            ORDER BY username ASC
            "#,
        )
        .fetch_all(pool)
        .await?;

        rows.into_iter().map(map_user_row).collect()
    }

    async fn update_active(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        active: bool,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            UPDATE ht_users
            SET active = $2
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .bind(active)
        .execute(&mut **tx)
        .await?;

        Ok(result.rows_affected())
    }

    async fn update_role(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        role: Role,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            UPDATE ht_users
            SET role = $2
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .bind(role.as_str())
        .execute(&mut **tx)
        .await?;

        Ok(result.rows_affected())
    }

    async fn update_password_hash(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        user_id: i64,
        password_hash: &str,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            UPDATE ht_users
            SET password_hash = $2
            WHERE user_id = $1
            "#,
        )
        .bind(user_id)
        .bind(password_hash)
        .execute(&mut **tx)
        .await?;

        Ok(result.rows_affected())
    }
}

fn map_user_row(row: sqlx::postgres::PgRow) -> Result<User, sqlx::Error> {
    let role_str: String = row.try_get("role")?;
    Ok(User {
        user_id: row.try_get("user_id")?,
        username: row.try_get("username")?,
        password_hash: row.try_get("password_hash")?,
        role: role_from_db(&role_str)?,
        active: row.try_get("active")?,
        created_at: row.try_get::<NaiveDateTime, _>("created_at")?,
        last_login_at: row.try_get::<Option<NaiveDateTime>, _>("last_login_at")?,
        email: row.try_get::<Option<String>, _>("email")?,
    })
}
