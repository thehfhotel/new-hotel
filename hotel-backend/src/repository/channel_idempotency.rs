//! Loyalty-channel request-idempotency repository — PG-only access to
//! `ht_channel_idempotency` (migration 093). See `service::channel_idempotency`
//! for the policy this plumbing serves and `docs/loyalty-channel.md` for the
//! wire contract.
//!
//! Free functions rather than a trait, for `repository::channel`'s reason:
//! there is a single PG implementation and nothing to swap. The reserving
//! statement takes `&mut Transaction<'_, Postgres>` because its transaction is
//! the concurrency serializer — it must stay open across the create the caller
//! is about to perform (see [`try_reserve`]). No MSSQL, no HTTP types
//! (architecture.md §2).
//!
//! ## Runtime queries, not `sqlx::query!`
//!
//! `ht_channel_idempotency` postdates the committed `.sqlx` offline cache, so
//! the compile-time-checked macros cannot describe it without a live database
//! at build time. Same call as `repository::booking`'s `book_ext_ref`
//! statements (migration 076) — runtime `sqlx::query` with explicit binds.

use sqlx::{PgPool, Postgres, Row, Transaction};

/// One `ht_channel_idempotency` row as the service reads it back.
///
/// `status` / `body` are `None` only on a row that is still in flight, which
/// no other transaction can see: the reserving transaction sets them at the
/// same moment it commits. A row read by anyone else is therefore complete —
/// the `Option`s exist because the columns are nullable, not because a
/// committed half-state is reachable.
#[derive(Debug, Clone)]
pub struct IdempotencyRecord {
    pub idem_id: i64,
    pub fingerprint: String,
    pub status: Option<i16>,
    pub body: Option<String>,
    pub book_id: Option<i32>,
}

/// Reserve `(caller, key)` for a request that is about to run.
///
/// Returns the new row's `idem_id` when THIS caller won the reservation, and
/// `None` when a live row already exists (→ the caller must [`load`] it and
/// replay or reject).
///
/// **The transaction is the point.** `tx` must stay open until the work is
/// done and [`complete`] has run, because the uncommitted unique-index entry
/// this INSERT creates is what makes a simultaneous duplicate request BLOCK
/// (PostgreSQL speculative insertion) instead of racing us into a second hold.
/// Roll `tx` back and the reservation disappears, which is exactly right for a
/// failed request: an error is never cached, so the client may retry the key.
///
/// `ON CONFLICT … DO UPDATE … WHERE idem_expires_at <= NOW()` makes an EXPIRED
/// row transparently reusable — the key is taken over and the request is fresh
/// — while a live row updates nothing and returns no id. Written as DO UPDATE
/// rather than DO NOTHING so that "expired" and "in flight" are decided by one
/// statement under one lock rather than by a read-then-write race.
pub async fn try_reserve(
    tx: &mut Transaction<'_, Postgres>,
    caller: &str,
    key: &str,
    endpoint: &str,
    fingerprint: &str,
    ttl_hours: i32,
) -> Result<Option<i64>, sqlx::Error> {
    let row = sqlx::query(
        "INSERT INTO ht_channel_idempotency \
           (idem_caller, idem_key, idem_endpoint, idem_fingerprint, idem_expires_at) \
         VALUES ($1, $2, $3, $4, NOW() + make_interval(hours => $5::int)) \
         ON CONFLICT ON CONSTRAINT ux_ht_channel_idempotency_caller_key DO UPDATE \
           SET idem_endpoint = EXCLUDED.idem_endpoint, \
               idem_fingerprint = EXCLUDED.idem_fingerprint, \
               idem_status = NULL, \
               idem_body = NULL, \
               idem_book_id = NULL, \
               idem_created_at = NOW(), \
               idem_completed_at = NULL, \
               idem_expires_at = EXCLUDED.idem_expires_at \
           WHERE ht_channel_idempotency.idem_expires_at <= NOW() \
         RETURNING idem_id",
    )
    .bind(caller)
    .bind(key)
    .bind(endpoint)
    .bind(fingerprint)
    .bind(ttl_hours)
    .fetch_optional(&mut **tx)
    .await?;

    Ok(row.map(|r| r.get::<i64, _>("idem_id")))
}

/// Read the live row for `(caller, key)`, or `None` when there is none.
///
/// Expired rows read as absent (`idem_expires_at > NOW()`), so a key replayed
/// past its TTL behaves like a first-time request rather than resurrecting a
/// day-old response.
///
/// Takes an executor so the service can read through the SAME transaction that
/// just lost the reservation race: under READ COMMITTED each statement takes a
/// fresh snapshot, so the winner's row — committed while our INSERT waited on
/// it — is visible without a second connection.
pub async fn load<'e, E>(
    executor: E,
    caller: &str,
    key: &str,
) -> Result<Option<IdempotencyRecord>, sqlx::Error>
where
    E: sqlx::PgExecutor<'e>,
{
    let row = sqlx::query(
        "SELECT idem_id, idem_fingerprint, idem_status, idem_body, idem_book_id \
         FROM ht_channel_idempotency \
         WHERE idem_caller = $1 AND idem_key = $2 AND idem_expires_at > NOW()",
    )
    .bind(caller)
    .bind(key)
    .fetch_optional(executor)
    .await?;

    Ok(row.map(|r| IdempotencyRecord {
        idem_id: r.get("idem_id"),
        fingerprint: r.get("idem_fingerprint"),
        status: r.get("idem_status"),
        body: r.get("idem_body"),
        book_id: r.get("idem_book_id"),
    }))
}

/// Record the response a reserved request produced. Runs inside the reserving
/// transaction, so the stored response and the work it describes commit
/// together — a rollback loses both, never one without the other.
pub async fn complete(
    tx: &mut Transaction<'_, Postgres>,
    idem_id: i64,
    status: i16,
    body: &str,
    book_id: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE ht_channel_idempotency \
         SET idem_status = $2, idem_body = $3, idem_book_id = $4, idem_completed_at = NOW() \
         WHERE idem_id = $1",
    )
    .bind(idem_id)
    .bind(status)
    .bind(body)
    .bind(book_id)
    .execute(&mut **tx)
    .await?;

    Ok(result.rows_affected())
}

/// Delete expired rows, at most `limit` per call. Bounded so the TTL sweep can
/// ride the request path (before a fresh reservation, on its own statement)
/// without ever becoming the slow part of a booking.
pub async fn purge_expired(pool: &PgPool, limit: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM ht_channel_idempotency \
         WHERE idem_id IN ( \
             SELECT idem_id FROM ht_channel_idempotency \
             WHERE idem_expires_at <= NOW() \
             ORDER BY idem_expires_at \
             LIMIT $1 \
         )",
    )
    .bind(limit)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
