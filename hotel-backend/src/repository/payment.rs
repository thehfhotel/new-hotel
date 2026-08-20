//! Payment repository — PostgreSQL data access for `ht_payments`.
//!
//! Mirrors `routes::new_payments` SQL behavior 1:1.

use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{PgPool, Postgres, Transaction};

/// One row in `ht_payments`.
#[derive(Debug, Clone)]
pub struct PaymentRow {
    pub pay_id: i32,
    pub pay_cin_id: i32,
    pub pay_amount: Option<f64>,
    pub pay_method: String,
    pub pay_reference: Option<String>,
    pub pay_notes: Option<String>,
    pub pay_date: Option<NaiveDateTime>,
    pub pay_created_by: Option<String>,
    pub pay_voided: Option<bool>,
    pub pay_voided_at: Option<NaiveDateTime>,
    pub pay_voided_by: Option<String>,
    pub created_at: Option<NaiveDateTime>,
}

/// Slice of `ht_checkins` used to derive a payment summary (total, balance).
///
/// Mirrors the columns the route reads via the `list_payments` SELECT.
#[derive(Debug, Clone)]
pub struct CheckInBillingRow {
    /// `ht_checkins.cin_total_amount` — mirrors legacy
    /// `HT_CheckIn_H.Total_Price_Net`, which is **Room + Product by
    /// definition**. NOT a room-only basis; see [`Self::cin_room_amount`].
    pub cin_total_amount: Option<f64>,
    /// `ht_checkins.cin_room_amount` (migration 079) — mirrors legacy
    /// `HT_CheckIn_H.Total_Price_Room`, the ROOM-ONLY leg of the folio.
    /// This is the room basis `routes::new_checkins::folio_breakdown`
    /// uses so `net = room + product` counts each POS line exactly once.
    ///
    /// `None` means the sync has never projected this folio's
    /// `Total_Price_Room` (a row predating migration 079 that has had no
    /// CT event since, or an app-originated check-in before its first
    /// read-back tick) — callers fall back to [`Self::cin_total_amount`].
    /// `Some(0.0)` is a genuine zero room charge and must NOT be treated
    /// as absent.
    pub cin_room_amount: Option<f64>,
    pub cin_rate_per_night: Option<f64>,
    pub nights: Option<i32>,
}

/// Field set for `insert`.
#[derive(Debug, Clone)]
pub struct PaymentInsert<'a> {
    pub cin_id: i32,
    pub amount: f64,
    pub method: &'a str,
    pub reference: Option<&'a str>,
    pub notes: Option<&'a str>,
    pub created_by: Option<&'a str>,
}

/// Snapshot returned by `find_for_void` — used by the void endpoint to detect
/// "already voided".
#[derive(Debug, Clone)]
pub struct PaymentStatus {
    pub pay_id: i32,
    pub pay_voided: Option<bool>,
}

/// Snapshot returned by `find_for_refund` — used by `refund_payment` so it
/// can validate the original payment exists, is not voided, and decide
/// which tender method the refund should debit. Mirrors the shape of
/// `PaymentStatus` but carries the extra fields the refund path needs.
#[derive(Debug, Clone)]
pub struct PaymentForRefund {
    pub pay_id: i32,
    pub pay_cin_id: i32,
    pub pay_amount: Option<f64>,
    pub pay_method: String,
    pub pay_voided: Option<bool>,
    /// Always `None` for an original payment; populated for an already-
    /// recorded refund row so the service layer can refuse to refund a
    /// refund.
    pub refund_of_payment_id: Option<i32>,
}

/// Field set for `insert_refund`.
#[derive(Debug, Clone)]
pub struct RefundInsert<'a> {
    pub cin_id: i32,
    /// POSITIVE refund amount in baht. The repository negates it before
    /// writing to the canonical `pay_amount` column so consumers reading
    /// `ht_payments` see a negative `pay_amount` (matches legacy
    /// `Cin_Pay_Cash/Credit` convention per
    /// `docs/legacy-app/COMPAT_CHEATSHEET.md:550`).
    pub amount: f64,
    pub method: &'a str,
    pub original_payment_id: i32,
    pub reason: Option<&'a str>,
    pub created_by: Option<&'a str>,
}

/// PostgreSQL data operations for the payment aggregate.
#[async_trait]
pub trait PaymentRepository: Send + Sync {
    /// Read the billing slice of a check-in (total amount + rate + nights) so
    /// the route can compute a balance. Returns None if the check-in is missing.
    async fn check_in_billing(
        &self,
        pool: &PgPool,
        cin_id: i32,
    ) -> Result<Option<CheckInBillingRow>, sqlx::Error>;

    /// List all payments for a check-in (newest first).
    async fn list_for_checkin(
        &self,
        pool: &PgPool,
        cin_id: i32,
    ) -> Result<Vec<PaymentRow>, sqlx::Error>;

    /// Insert a payment; returns its assigned `pay_id`.
    async fn insert(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        insert: PaymentInsert<'_>,
    ) -> Result<i32, sqlx::Error>;

    /// Stamp `ht_payments.aggregate_id` for a freshly-inserted row. Required
    /// for the writeback worker's `back_populate_legacy_ids` step to target
    /// the payment from `WritebackIntent::RecordPayment.payment_aggregate_id`
    /// (Wave 5a item 3).
    async fn stamp_aggregate_id(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        pay_id: i32,
        aggregate_id: uuid::Uuid,
    ) -> Result<(), sqlx::Error>;

    /// Look up just the void-related fields of a payment.
    async fn find_for_void(
        &self,
        pool: &PgPool,
        pay_id: i32,
    ) -> Result<Option<PaymentStatus>, sqlx::Error>;

    /// Soft-void a payment (sets `pay_voided=true`, `pay_voided_at=NOW()`).
    async fn void(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        pay_id: i32,
    ) -> Result<(), sqlx::Error>;

    /// Read the slice of a payment the refund service needs to validate
    /// the request: existence, current voided flag, method, amount, the
    /// owning check-in, and (if this row is already a refund) the FK
    /// back to its original. Track G2 / T4 CRIT-1.
    async fn find_for_refund(
        &self,
        pool: &PgPool,
        pay_id: i32,
    ) -> Result<Option<PaymentForRefund>, sqlx::Error>;

    /// Sum the absolute value of every previously-recorded refund
    /// against an original payment (`pay_voided = false` only). Returns
    /// `0.0` when no refunds exist. Used by `refund_payment` to enforce
    /// the "sum of refunds ≤ original amount" invariant on partial
    /// refunds. Track G2 / T4 CRIT-1.
    async fn sum_refunded_against(
        &self,
        pool: &PgPool,
        original_pay_id: i32,
    ) -> Result<f64, sqlx::Error>;

    /// Insert a refund row (negative `pay_amount`) and return the new
    /// `pay_id`. Track G2 / T4 CRIT-1. The repository negates the
    /// caller-supplied positive `amount` before writing so canonical
    /// rows have a negative `pay_amount` consistent with the legacy
    /// `Cin_Pay_Cash/Credit` negation convention.
    async fn insert_refund(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        insert: RefundInsert<'_>,
    ) -> Result<i32, sqlx::Error>;
}

/// Default `PaymentRepository` impl backed by sqlx + PostgreSQL.
#[derive(Clone, Debug, Default)]
pub struct PgPaymentRepository;

impl PgPaymentRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl PaymentRepository for PgPaymentRepository {
    async fn check_in_billing(
        &self,
        pool: &PgPool,
        cin_id: i32,
    ) -> Result<Option<CheckInBillingRow>, sqlx::Error> {
        // Dynamic `query_as` rather than the `query!` macro so adding
        // `cin_room_amount` (migration 079) needs no `.sqlx/` regeneration.
        let rec = sqlx::query_as::<_, (Option<f64>, Option<f64>, Option<f64>, Option<i32>)>(
            "SELECT cin_total_amount::float8, cin_room_amount::float8, \
                    cin_rate_per_night::float8, \
                    (COALESCE(cin_checkout_time, cin_expected_checkout)::date \
                     - cin_checkin_time::date) AS nights \
               FROM ht_checkins WHERE cin_id = $1",
        )
        .bind(cin_id)
        .fetch_optional(pool)
        .await?;

        Ok(
            rec.map(|(total, room, rate, nights)| CheckInBillingRow {
                cin_total_amount: total,
                cin_room_amount: room,
                cin_rate_per_night: rate,
                nights,
            }),
        )
    }

    async fn list_for_checkin(
        &self,
        pool: &PgPool,
        cin_id: i32,
    ) -> Result<Vec<PaymentRow>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"SELECT pay_id, pay_cin_id, pay_amount::float8 as pay_amount, pay_method, pay_reference,
            pay_notes, pay_date, pay_created_by, pay_voided, pay_voided_at, pay_voided_by, created_at
        FROM ht_payments WHERE pay_cin_id = $1
        ORDER BY pay_date DESC, pay_id DESC"#,
            cin_id
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| PaymentRow {
                pay_id: r.pay_id,
                pay_cin_id: r.pay_cin_id,
                pay_amount: r.pay_amount,
                pay_method: r.pay_method,
                pay_reference: r.pay_reference,
                pay_notes: r.pay_notes,
                pay_date: r.pay_date,
                pay_created_by: r.pay_created_by,
                pay_voided: r.pay_voided,
                pay_voided_at: r.pay_voided_at,
                pay_voided_by: r.pay_voided_by,
                created_at: r.created_at,
            })
            .collect())
    }

    async fn insert(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        insert: PaymentInsert<'_>,
    ) -> Result<i32, sqlx::Error> {
        let rec = sqlx::query!(
            r#"INSERT INTO ht_payments (pay_cin_id, pay_amount, pay_method, pay_reference, pay_notes, pay_created_by)
        VALUES ($1, $2::float8, $3, $4, $5, $6) RETURNING pay_id"#,
            insert.cin_id,
            insert.amount,
            insert.method,
            insert.reference,
            insert.notes,
            insert.created_by
        )
        .fetch_one(&mut **tx)
        .await?;
        Ok(rec.pay_id)
    }

    async fn stamp_aggregate_id(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        pay_id: i32,
        aggregate_id: uuid::Uuid,
    ) -> Result<(), sqlx::Error> {
        // Dynamic `sqlx::query` (not `query!`) so this doesn't require a
        // `.sqlx/` cache regeneration. The column was added by migration 030
        // — `aggregate_id UUID` keyed off the SERIAL `pay_id`.
        sqlx::query("UPDATE ht_payments SET aggregate_id = $2 WHERE pay_id = $1")
            .bind(pay_id)
            .bind(aggregate_id)
            .execute(&mut **tx)
            .await?;
        Ok(())
    }

    async fn find_for_void(
        &self,
        pool: &PgPool,
        pay_id: i32,
    ) -> Result<Option<PaymentStatus>, sqlx::Error> {
        let rec = sqlx::query!(
            "SELECT pay_id, pay_voided FROM ht_payments WHERE pay_id = $1",
            pay_id
        )
        .fetch_optional(pool)
        .await?;
        Ok(rec.map(|r| PaymentStatus {
            pay_id: r.pay_id,
            pay_voided: r.pay_voided,
        }))
    }

    async fn void(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        pay_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE ht_payments SET pay_voided = true, pay_voided_at = NOW() WHERE pay_id = $1"#,
            pay_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn find_for_refund(
        &self,
        pool: &PgPool,
        pay_id: i32,
    ) -> Result<Option<PaymentForRefund>, sqlx::Error> {
        // Dynamic `sqlx::query` so we don't require `.sqlx/` cache
        // regeneration before the migration runs (the new
        // `refund_of_payment_id` column is brand-new in migration 044
        // — runtime cache validation would otherwise fail in CI on the
        // first push before migrate.sh applies it).
        let row = sqlx::query(
            "SELECT pay_id, pay_cin_id, pay_amount::float8 AS pay_amount, \
             pay_method, pay_voided, refund_of_payment_id \
             FROM ht_payments WHERE pay_id = $1",
        )
        .bind(pay_id)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| {
            use sqlx::Row;
            PaymentForRefund {
                pay_id: r.try_get("pay_id").unwrap_or(pay_id),
                pay_cin_id: r.try_get("pay_cin_id").unwrap_or(0),
                pay_amount: r.try_get("pay_amount").ok(),
                pay_method: r.try_get("pay_method").unwrap_or_default(),
                pay_voided: r.try_get("pay_voided").ok(),
                refund_of_payment_id: r.try_get("refund_of_payment_id").ok(),
            }
        }))
    }

    async fn sum_refunded_against(
        &self,
        pool: &PgPool,
        original_pay_id: i32,
    ) -> Result<f64, sqlx::Error> {
        // COALESCE the SUM so an empty result set returns 0.0 instead of
        // NULL. The ABS() guards against the legacy convention of
        // negative `pay_amount` — the canonical refund row stores a
        // negative number, but the invariant we enforce is "total
        // refunded MAGNITUDE ≤ original payment magnitude".
        let row = sqlx::query(
            "SELECT COALESCE(SUM(ABS(pay_amount))::float8, 0.0) AS refunded \
             FROM ht_payments \
             WHERE refund_of_payment_id = $1 AND pay_voided = false",
        )
        .bind(original_pay_id)
        .fetch_one(pool)
        .await?;
        use sqlx::Row;
        Ok(row.try_get::<f64, _>("refunded").unwrap_or(0.0))
    }

    async fn insert_refund(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        insert: RefundInsert<'_>,
    ) -> Result<i32, sqlx::Error> {
        // Negate the caller-supplied positive `amount` so canonical
        // `pay_amount` carries a negative value (matches legacy
        // `Cin_Pay_Cash/Credit` negation convention per
        // COMPAT_CHEATSHEET.md:550). The check that `amount` is
        // positive is the service-layer's responsibility — the
        // repository writes whatever it's given.
        let neg_amount = -insert.amount.abs();
        let row = sqlx::query(
            "INSERT INTO ht_payments (\
               pay_cin_id, pay_amount, pay_method, pay_notes, \
               pay_created_by, refund_of_payment_id, refund_reason) \
             VALUES ($1, $2::float8, $3, $4, $5, $6, $7) \
             RETURNING pay_id",
        )
        .bind(insert.cin_id)
        .bind(neg_amount)
        .bind(insert.method)
        .bind(insert.reason)
        .bind(insert.created_by)
        .bind(insert.original_payment_id)
        .bind(insert.reason)
        .fetch_one(&mut **tx)
        .await?;
        use sqlx::Row;
        Ok(row.try_get::<i32, _>("pay_id").unwrap_or(0))
    }
}
