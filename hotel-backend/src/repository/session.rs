//! Session repository — PostgreSQL data access for `ht_sessions` (Phase 4 PR1).
//!
//! Mirrors the pattern in [`super::user`]: read methods take `&PgPool`,
//! write methods take `&mut Transaction<'_, Postgres>` so the caller
//! (`service::auth::AuthService`) can group create-session +
//! update-last-login into one atomic transaction.
//!
//! Uses dynamic `sqlx::query()` rather than the `query!()` macro for the
//! same reason as [`super::user`] — PR1 should not require regenerating
//! the `.sqlx/` cache for tables that don't yet exist on a developer
//! laptop.

use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{PgPool, Postgres, Row, Transaction};

use crate::domain::session::Session;
use crate::repository::user::UserRepository;

/// PostgreSQL data operations for the `Session` aggregate.
#[async_trait]
pub trait SessionRepository: Send + Sync {
    /// Insert a new session row. The `session_id` is the cookie token
    /// the caller already generated; the repository does not mint it.
    async fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        session_id: &str,
        user_id: i64,
        expires_at: NaiveDateTime,
        ip: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(), sqlx::Error>;

    /// Look up a session by its token, filtering out expired rows
    /// (`expires_at <= now`). The caller passes `now` so tests can
    /// pin the clock; production passes `Utc::now().naive_utc()`.
    async fn get_active(
        &self,
        pool: &PgPool,
        session_id: &str,
        now: NaiveDateTime,
    ) -> Result<Option<Session>, sqlx::Error>;

    /// Delete a single session (logout). Returns rows affected so the
    /// caller can detect a no-op delete (already logged out / expired
    /// reaper got there first).
    async fn delete(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        session_id: &str,
    ) -> Result<u64, sqlx::Error>;

    /// Periodic-cleanup hook: drop every session row whose
    /// `expires_at <= now`. Returns the number of rows reaped.
    async fn delete_expired(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        now: NaiveDateTime,
    ) -> Result<u64, sqlx::Error>;

    /// High-level façade used by `AuthService::login`: in ONE
    /// transaction, INSERT the session row and bump
    /// `ht_users.last_login_at` for the same `user_id`. Production
    /// implementation opens a `pool.begin()` internally; mock impls
    /// can simply call the same trait methods on themselves without
    /// touching a real transaction.
    ///
    /// Default impl chains [`Self::create`] + [`UserRepository::update_last_login`]
    /// inside `pool.begin()` — works for the production [`PgSessionRepository`].
    /// Mock repos that ignore `Transaction` should override this to
    /// bypass the `pool.begin()` call.
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
        let mut tx = pool.begin().await?;
        self.create(&mut tx, session_id, user_id, expires_at, ip, user_agent)
            .await?;
        users.update_last_login(&mut tx, user_id).await?;
        tx.commit().await?;
        Ok(())
    }

    /// Logout helper — DELETE the session row by id, opening its own
    /// transaction. Mirrors the `create_and_touch_login` contract:
    /// production opens `pool.begin()`; mocks can override to avoid
    /// that call.
    ///
    /// Returns rows-affected.
    async fn delete_by_id(
        &self,
        pool: &PgPool,
        session_id: &str,
    ) -> Result<u64, sqlx::Error> {
        let mut tx = pool.begin().await?;
        let n = self.delete(&mut tx, session_id).await?;
        tx.commit().await?;
        Ok(n)
    }
}

/// Default `SessionRepository` impl backed by sqlx + PostgreSQL.
#[derive(Clone, Debug, Default)]
pub struct PgSessionRepository;

impl PgSessionRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl SessionRepository for PgSessionRepository {
    async fn create(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        session_id: &str,
        user_id: i64,
        expires_at: NaiveDateTime,
        ip: Option<&str>,
        user_agent: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        // `ip::inet` is cast at query time so we can bind a plain &str —
        // `sqlx` doesn't ship an INET binding without an extra feature
        // and the application logic doesn't need to validate the IP
        // here (the caller pulls it from request headers).
        sqlx::query(
            r#"
            INSERT INTO ht_sessions
                (session_id, user_id, expires_at, ip, user_agent)
            VALUES ($1, $2, $3, $4::inet, $5)
            "#,
        )
        .bind(session_id)
        .bind(user_id)
        .bind(expires_at)
        .bind(ip)
        .bind(user_agent)
        .execute(&mut **tx)
        .await?;

        Ok(())
    }

    async fn get_active(
        &self,
        pool: &PgPool,
        session_id: &str,
        now: NaiveDateTime,
    ) -> Result<Option<Session>, sqlx::Error> {
        let row = sqlx::query(
            r#"
            SELECT session_id, user_id, created_at, expires_at,
                   ip::text AS ip_text, user_agent
            FROM ht_sessions
            WHERE session_id = $1 AND expires_at > $2
            "#,
        )
        .bind(session_id)
        .bind(now)
        .fetch_optional(pool)
        .await?;

        row.map(map_session_row).transpose()
    }

    async fn delete(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        session_id: &str,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM ht_sessions WHERE session_id = $1")
            .bind(session_id)
            .execute(&mut **tx)
            .await?;
        Ok(result.rows_affected())
    }

    async fn delete_expired(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        now: NaiveDateTime,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM ht_sessions WHERE expires_at <= $1")
            .bind(now)
            .execute(&mut **tx)
            .await?;
        Ok(result.rows_affected())
    }
}

fn map_session_row(row: sqlx::postgres::PgRow) -> Result<Session, sqlx::Error> {
    Ok(Session {
        id: row.try_get("session_id")?,
        user_id: row.try_get("user_id")?,
        created_at: row.try_get::<NaiveDateTime, _>("created_at")?,
        expires_at: row.try_get::<NaiveDateTime, _>("expires_at")?,
        ip: row.try_get::<Option<String>, _>("ip_text")?,
        user_agent: row.try_get::<Option<String>, _>("user_agent")?,
    })
}
