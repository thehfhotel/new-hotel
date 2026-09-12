//! Shared discipline for the `ht_bookings` **row** guards.
//!
//! Two paths take `SELECT … FOR NO KEY UPDATE` on a booking row as the first
//! lock of their transaction:
//!
//! * `checkin::PgCheckInRepository::lock_booking_for_check_in` (B7b — the
//!   booking-level double-check-in guard), and
//! * `booking::PgBookingRepository::lock_booking_for_modify` (B8h — the single
//!   snapshot the edit path's inventory-lock and legacy-promote decisions are
//!   both made on).
//!
//! They must agree on the bound, so the bound lives here rather than being
//! copied. Nothing else belongs in this module: it is a shared constant and
//! the one statement that applies it, not a lock abstraction.
//!
//! **`FOR NO KEY UPDATE`, never `FOR UPDATE`** — that rule is not encoded here
//! (each guard writes its own SELECT) but it is the same rule in both places,
//! and `checkin`'s trait doc carries the full argument: `ht_bookings` is the
//! parent of five FK children whose inserts take `FOR KEY SHARE` on the parent
//! row; `FOR KEY SHARE` conflicts with `FOR UPDATE` and is compatible with
//! `FOR NO KEY UPDATE`. Both guards are also followed, in the same
//! transaction, by an `UPDATE` of that same row — which takes `FOR NO KEY
//! UPDATE` itself — so neither guard introduces a lock strength its
//! transaction was not already going to take.

use sqlx::{Postgres, Transaction};

/// How long a booking-row guard waits before answering "busy".
///
/// Without a bound, any holder of the row — realistically a CT sync
/// table-tick, which keeps ONE PG transaction open across N MSSQL round trips
/// (`bin/sync.rs`) — parks the desk's request behind itself for as long as it
/// runs, and the receptionist watches a spinner until her HTTP client gives up
/// and she retries, queueing a SECOND waiter behind the same holder.
///
/// Three seconds: comfortably longer than any legitimate contender, short
/// enough that the desk gets a retryable answer instead of a hang. Deliberately
/// under the inventory lock's 5 s `ACQUIRE_TIMEOUT`
/// (`repository::inventory_lock`), because a row lock behind a bulk tick is the
/// less recoverable of the two waits.
///
/// **It bounds the WAIT, not the HOLD.** This caps how long a guard waits to
/// TAKE a booking row; once taken, the row stays locked until the caller's
/// transaction ends, and nothing here caps that. In the B8h edit path the hold
/// is the longer half: `service::booking::modify` holds this row while it
/// acquires the property inventory lock, so the worst-case HOLD is ~10 s —
/// `InventoryLock::acquire`'s 5 s `ACQUIRE_TIMEOUT`, plus up to another
/// `db::pg_pool::PG_ACQUIRE_TIMEOUT` (5 s) if its final `pool.begin()` starts
/// just under that deadline on a saturated pool — after which it answers Busy
/// and rolls back. What this constant does give is that every contender queued
/// behind that row gets its own 3 s cap and a retryable 503, so a long hold
/// does not cascade into a queue of hung desk requests.
///
/// Surfaces as SQLSTATE `55P03` (`lock_not_available`), which
/// `service::checkin::map_booking_lock_error` turns into `ServiceError::Busy`
/// → `503` + `Retry-After`. Never a 500: nothing was written and the identical
/// request will normally succeed.
pub(crate) const BOOKING_LOCK_TIMEOUT_MS: i32 = 3_000;

/// `SET LOCAL lock_timeout` via `set_config`, which — unlike `SET` — accepts a
/// bind parameter. `is_local = true` ties the setting to the caller's
/// transaction, so it can never leak onto the pooled connection.
///
/// Callers read the inherited value with `current_setting('lock_timeout')`
/// first and restore it right after the guarded SELECT, so a server- or
/// role-configured timeout is put back rather than assumed to be the shipped
/// default of `0` — silently REMOVING a configured `lock_timeout` for the rest
/// of the transaction would be worse than the hang being fixed.
pub(crate) async fn set_local_lock_timeout(
    tx: &mut Transaction<'_, Postgres>,
    value: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query_scalar::<_, String>("SELECT set_config('lock_timeout', $1, true)")
        .bind(value)
        .fetch_one(&mut **tx)
        .await?;
    Ok(())
}
