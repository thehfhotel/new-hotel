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
    pub cin_total_amount: Option<f64>,
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
        let rec = sqlx::query!(
            r#"SELECT cin_id, cin_total_amount::float8 as cin_total_amount, cin_rate_per_night::float8 as cin_rate_per_night,
            (COALESCE(cin_checkout_time, cin_expected_checkout)::date - cin_checkin_time::date) as nights
        FROM ht_checkins WHERE cin_id = $1"#,
            cin_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(rec.map(|r| CheckInBillingRow {
            cin_total_amount: r.cin_total_amount,
            cin_rate_per_night: r.cin_rate_per_night,
            nights: r.nights,
        }))
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
}
