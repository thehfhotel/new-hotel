//! Coupon service — Track G5.
//!
//! Promotes the legacy `HT_Cupon` table (food/breakfast coupon
//! entitlement, per `docs/legacy-app/COMPAT_CHEATSHEET.md` §`HT_Cupon`)
//! to a canonical-owned `ht_coupons` table so receptionists can issue
//! coupons from the new app without falling back to iHOTEL.
//!
//! Two public methods:
//!
//! - [`CouponService::issue_coupon`] — INSERTs a canonical `ht_coupons`
//!   row, enqueues [`WritebackIntent::IssueCoupon`] for the legacy
//!   `HT_Cupon` INSERT (the writeback worker allocates `cupon_no` via
//!   TABLOCKX+HOLDLOCK and back-populates `legacy_cupon_no`).
//! - [`CouponService::redeem_coupon`] — looks up by `coupon_code`,
//!   validates the coupon is still `issued` and not expired, flips
//!   `coupon_status='redeemed'` + stamps `coupon_redeemed_at` /
//!   `coupon_redeemed_cin_id`, enqueues [`WritebackIntent::RedeemCoupon`]
//!   (mirrors legacy `Print_Report.cs:2552`).
//!
//! Both methods follow the project's standard service pattern:
//! one PG `begin()` transaction wraps the canonical write + outbox
//! enqueue (events are emitted via subscribers off the writeback
//! status, not directly here — coupons don't need a dedicated
//! `DomainEvent` variant yet).

use std::sync::Arc;

use chrono::{NaiveDate, Utc};
use sqlx::{PgPool, Postgres, Row, Transaction};
use uuid::Uuid;

use crate::outbox::event::EventSource;
use crate::outbox::intent::WritebackIntent;
use crate::outbox::{generate_idempotency_key, EventBus, OutboxRepository};

use super::error::{ServiceError, ServiceResult};
use super::ids::{aggregate_uuid, AggregateKind};

/// Command for [`CouponService::issue_coupon`].
#[derive(Debug, Clone)]
pub struct IssueCouponCommand {
    /// Canonical `ht_customers.cust_id` the coupon is issued to.
    /// Optional — promo coupons may not be tied to a specific customer.
    pub customer_id: Option<i32>,
    /// Face value in baht. Zero for legacy-style entitlement coupons
    /// (the most common case — `HT_Cupon` itself has no value column).
    pub value_baht: f64,
    /// Optional expiry date (date-only, no time-of-day).
    pub expires_at: Option<NaiveDate>,
    /// Operator username. Empty string falls back to `"Admin"` in the
    /// writeback recipe — mirrors the legacy `cupon_by` default.
    pub issued_by: String,
    /// Legacy `Cin_no` the coupon is attached to. Optional —
    /// standalone promo coupons not yet bound to a stay.
    pub for_cin_no: Option<String>,
    pub source: EventSource,
}

/// Command for [`CouponService::redeem_coupon`].
#[derive(Debug, Clone)]
pub struct RedeemCouponCommand {
    pub coupon_code: String,
    /// Canonical `ht_checkins.cin_id` the redemption is applied
    /// against. Optional — coupons may also be redeemed standalone
    /// (e.g. promo handed at the lobby cafe without a folio link).
    pub redeemed_cin_id: Option<i32>,
    pub source: EventSource,
}

/// Outcome of [`CouponService::issue_coupon`].
#[derive(Debug, Clone)]
pub struct IssueCouponOutcome {
    pub coupon_id: i64,
    pub coupon_code: String,
    pub aggregate_id: Uuid,
}

/// Outcome of [`CouponService::redeem_coupon`].
#[derive(Debug, Clone)]
pub struct RedeemCouponOutcome {
    pub coupon_id: i64,
    pub aggregate_id: Uuid,
}

/// Service handle for the coupon aggregate.
#[derive(Clone)]
pub struct CouponService {
    pub(crate) outbox: Arc<OutboxRepository>,
    #[allow(dead_code)]
    pub(crate) events: Arc<EventBus>,
    pub(crate) pg: PgPool,
}

impl CouponService {
    pub fn new(outbox: Arc<OutboxRepository>, events: Arc<EventBus>, pg: PgPool) -> Self {
        Self { outbox, events, pg }
    }

    /// Issue a new coupon. Mirrors iHOTEL's `Module1.GEN_Cupon`.
    pub async fn issue_coupon(
        &self,
        cmd: IssueCouponCommand,
    ) -> ServiceResult<IssueCouponOutcome> {
        if !cmd.value_baht.is_finite() || cmd.value_baht < 0.0 {
            return Err(ServiceError::validation(format!(
                "coupon value must be finite and non-negative (got {})",
                cmd.value_baht
            )));
        }

        let coupon_code = generate_coupon_code();
        let aggregate_id = generate_aggregate_id();
        let issued_by = if cmd.issued_by.trim().is_empty() {
            "Admin".to_string()
        } else {
            cmd.issued_by.clone()
        };

        let mut tx = self.pg.begin().await?;

        let coupon_id = insert_coupon(
            &mut tx,
            &coupon_code,
            cmd.value_baht,
            cmd.expires_at,
            &issued_by,
            cmd.for_cin_no.as_deref(),
            cmd.customer_id,
            aggregate_id,
        )
        .await?;

        let intent = WritebackIntent::IssueCoupon {
            coupon_aggregate_id: aggregate_id,
            coupon_id,
        };
        let key = generate_idempotency_key(&intent, aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(ServiceError::from_enqueue_error)?;

        // Touch the held EventBus / outbox handles so the field-usage
        // lints stay green (we don't publish a DomainEvent variant for
        // coupons yet — outbox subscribers cover the SSE side once a
        // future track adds CouponIssued).
        let _ = (&self.outbox, &cmd.source);

        tx.commit().await?;

        Ok(IssueCouponOutcome {
            coupon_id,
            coupon_code,
            aggregate_id,
        })
    }

    /// Redeem (mark printed) an existing coupon. Mirrors legacy
    /// `Print_Report.cs:2552` flipping `cupon_print=1`.
    pub async fn redeem_coupon(
        &self,
        cmd: RedeemCouponCommand,
    ) -> ServiceResult<RedeemCouponOutcome> {
        if cmd.coupon_code.trim().is_empty() {
            return Err(ServiceError::validation("coupon_code is required"));
        }

        let mut tx = self.pg.begin().await?;

        let row = sqlx::query(
            "SELECT coupon_id, coupon_status, coupon_expires_at, aggregate_id \
             FROM ht_coupons \
             WHERE coupon_code = $1 \
             FOR UPDATE",
        )
        .bind(&cmd.coupon_code)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| {
            ServiceError::not_found(format!(
                "coupon code '{code}' not found",
                code = cmd.coupon_code
            ))
        })?;

        let coupon_id: i64 = row.try_get("coupon_id").map_err(|e| {
            ServiceError::internal(format!("coupon_id column missing: {e}"))
        })?;
        let status: String = row.try_get("coupon_status").map_err(|e| {
            ServiceError::internal(format!("coupon_status column missing: {e}"))
        })?;
        let expires_at: Option<NaiveDate> = row.try_get("coupon_expires_at").ok();
        let aggregate_id: Uuid = row.try_get("aggregate_id").map_err(|e| {
            ServiceError::internal(format!("aggregate_id column missing: {e}"))
        })?;

        if status != "issued" {
            return Err(ServiceError::conflict(format!(
                "coupon '{code}' is in status '{status}' — only 'issued' coupons can be redeemed",
                code = cmd.coupon_code
            )));
        }

        if let Some(expires) = expires_at {
            let today = Utc::now().date_naive();
            if expires < today {
                return Err(ServiceError::conflict(format!(
                    "coupon '{code}' expired on {expires}",
                    code = cmd.coupon_code
                )));
            }
        }

        let now = Utc::now();
        sqlx::query(
            "UPDATE ht_coupons SET \
                coupon_status = 'redeemed', \
                coupon_redeemed_at = $2, \
                coupon_redeemed_cin_id = $3, \
                updated_at = NOW() \
             WHERE coupon_id = $1",
        )
        .bind(coupon_id)
        .bind(now)
        .bind(cmd.redeemed_cin_id)
        .execute(&mut *tx)
        .await?;

        let intent = WritebackIntent::RedeemCoupon {
            coupon_aggregate_id: aggregate_id,
            coupon_id,
        };
        let key = generate_idempotency_key(&intent, aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(ServiceError::from_enqueue_error)?;

        let _ = (&self.outbox, &cmd.source);

        tx.commit().await?;

        Ok(RedeemCouponOutcome {
            coupon_id,
            aggregate_id,
        })
    }
}

/// Generate a human-readable `COUP-XXXXXXXX` code from a v4 UUID's
/// upper 32 bits (8 hex chars, uppercase). The probability of
/// collision over the realistic operational lifetime of the table
/// (~100k coupons over 10 years) is dwarfed by the canonical
/// `UNIQUE(coupon_code)` constraint — a collision causes the INSERT
/// to fail, the route returns 500, and the operator retries (which
/// re-rolls the code).
fn generate_coupon_code() -> String {
    let uuid = Uuid::new_v4();
    let bytes = uuid.as_bytes();
    let high: u32 = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    format!("COUP-{high:08X}")
}

/// Generate a fresh v4 aggregate id for a coupon. Each canonical row
/// gets a distinct id — unlike booking/check-in/payment where the
/// SERIAL `i32` PK is hashed into a deterministic v5 UUID, coupons
/// don't have a useful pre-row id to hash from (the BIGSERIAL is
/// minted by the INSERT itself).
fn generate_aggregate_id() -> Uuid {
    Uuid::new_v4()
}

/// Helper — INSERT the canonical row inside the caller's transaction
/// and return the BIGSERIAL coupon_id.
#[allow(clippy::too_many_arguments)]
async fn insert_coupon(
    tx: &mut Transaction<'_, Postgres>,
    coupon_code: &str,
    value_baht: f64,
    expires_at: Option<NaiveDate>,
    issued_by: &str,
    for_cin_no: Option<&str>,
    customer_id: Option<i32>,
    aggregate_id: Uuid,
) -> ServiceResult<i64> {
    let row = sqlx::query(
        "INSERT INTO ht_coupons ( \
            coupon_code, coupon_value, coupon_status, \
            coupon_issued_at, coupon_expires_at, coupon_issued_by, \
            coupon_for_cin_no, coupon_for_cust_id, aggregate_id, source \
         ) VALUES ( \
            $1, $2::float8, 'issued', NOW(), $3, $4, $5, $6, $7, 'canonical' \
         ) RETURNING coupon_id",
    )
    .bind(coupon_code)
    .bind(value_baht)
    .bind(expires_at)
    .bind(issued_by)
    .bind(for_cin_no)
    .bind(customer_id.map(|v| v as i64))
    .bind(aggregate_id)
    .fetch_one(&mut **tx)
    .await?;

    let coupon_id: i64 = row.try_get("coupon_id").map_err(|e| {
        ServiceError::internal(format!("RETURNING coupon_id missing: {e}"))
    })?;
    Ok(coupon_id)
}

/// Translate the canonical `coupon_status` value to the legacy
/// `cupon_print` integer (0 = unprinted, 1 = printed). Used by the
/// sync mapper when reverse-hydrating coupons from CT events.
///
/// Symmetric with [`pg_status_to_legacy_print`].
pub fn legacy_print_to_pg_status(cupon_print: i32) -> &'static str {
    match cupon_print {
        0 => "issued",
        1 => "redeemed",
        // Defensive — the legacy column is NOT NULL DEFAULT 0 with
        // observed values 0/1 only. Any other value (e.g. operator
        // hand-edited row) gets a sentinel status the dashboard can
        // surface for reconciliation.
        _ => "cancelled",
    }
}

/// Symmetric to [`legacy_print_to_pg_status`] — translate the
/// canonical status back to the legacy `cupon_print` integer. Used
/// when the canonical writeback recomputes the legacy flag (e.g.
/// reconcile sweep).
pub fn pg_status_to_legacy_print(status: &str) -> i32 {
    match status {
        "issued" => 0,
        "redeemed" => 1,
        // `expired` and `cancelled` are canonical-only — the legacy
        // table has no concept of either, so we map both to
        // `cupon_print=1` (so iHOTEL won't try to print them again).
        _ => 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Generated codes must match the `COUP-XXXXXXXX` pattern (8 hex
    /// chars). Catches any refactor that accidentally changes the
    /// width or prefix — receptionists print these codes on tickets,
    /// so the format is a contract.
    #[test]
    fn generate_coupon_code_emits_documented_format() {
        for _ in 0..32 {
            let code = generate_coupon_code();
            assert!(code.starts_with("COUP-"), "missing prefix: {code}");
            let hex = &code[5..];
            assert_eq!(hex.len(), 8, "expected 8 hex chars: {code}");
            assert!(
                hex.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_lowercase()),
                "expected uppercase hex chars: {code}"
            );
        }
    }

    /// Two consecutive calls overwhelmingly produce distinct codes —
    /// guards against a refactor that accidentally fixes the random
    /// seed.
    #[test]
    fn generate_coupon_code_is_random() {
        let a = generate_coupon_code();
        let b = generate_coupon_code();
        assert_ne!(a, b, "codes are too predictable: {a} == {b}");
    }

    /// Aggregate ids are v4 random (distinct per call). Guards
    /// against an accidental switch back to v5(NIL, …) which would
    /// alias every coupon to the same row.
    #[test]
    fn generate_aggregate_id_is_random() {
        let a = generate_aggregate_id();
        let b = generate_aggregate_id();
        assert_ne!(a, b);
        // v4 has version=4 in the 7th byte's high nibble.
        let bytes = a.as_bytes();
        assert_eq!(bytes[6] >> 4, 4, "aggregate id must be v4 UUID");
    }

    /// Round-trip the legacy `cupon_print` ↔ canonical `coupon_status`
    /// mapping. The sync mapper writes a `legacy` row using the
    /// `_to_pg` direction; future reconcile sweeps re-derive
    /// `cupon_print` via the reverse function. Both must agree on
    /// the two-state core (issued ⇔ 0, redeemed ⇔ 1).
    #[test]
    fn legacy_print_status_roundtrip() {
        assert_eq!(legacy_print_to_pg_status(0), "issued");
        assert_eq!(legacy_print_to_pg_status(1), "redeemed");
        // Defensive sentinel for unexpected legacy values.
        assert_eq!(legacy_print_to_pg_status(42), "cancelled");

        assert_eq!(pg_status_to_legacy_print("issued"), 0);
        assert_eq!(pg_status_to_legacy_print("redeemed"), 1);
        // Canonical-only statuses degrade to printed=1 so iHOTEL won't
        // re-print expired/cancelled coupons.
        assert_eq!(pg_status_to_legacy_print("expired"), 1);
        assert_eq!(pg_status_to_legacy_print("cancelled"), 1);
    }
}
