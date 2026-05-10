//! Customer repository — PostgreSQL data access for `ht_customers`.
//!
//! Mirrors the legacy `HT_Customers` table. The repository owns the SQL; the
//! HTTP route (`routes::new_customers`) keeps request/response shaping.
//!
//! Per `docs/architecture.md` §6 (worked example). Method signatures intentionally
//! avoid HTTP types — the route translates between domain rows and `NewCustomer`
//! DTOs.
//!
//! Read methods accept `&PgPool` directly (single-shot queries). Write methods
//! accept `&mut sqlx::Transaction<'_, Postgres>` so the upcoming service layer
//! (Phase 2) can compose them with outbox enqueue + event publish in one
//! atomic unit.

use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{PgPool, Postgres, Row, Transaction};

use crate::routes::new_customers::NewCustomersQuery;

/// Row shape returned by [`CustomerRepository`] reads.
///
/// Mirrors the `ht_customers` columns 1:1 — the route maps it to
/// `routes::new_customers::NewCustomer` for wire serialization.
#[derive(Debug, Clone)]
pub struct CustomerRow {
    pub cust_id: i32,
    pub cust_firstname: String,
    pub cust_lastname: Option<String>,
    pub cust_phone: Option<String>,
    pub cust_email: Option<String>,
    pub cust_idcard: Option<String>,
    pub cust_address: Option<String>,
    pub cust_type: Option<String>,
    pub cust_notes: Option<String>,
    pub cust_active: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// Field set used by `insert` / `update`. Mirrors the legacy column names so
/// the writeback worker (Phase 4b) can serialize them straight onto MSSQL.
#[derive(Debug, Clone)]
pub struct CustomerWrite<'a> {
    pub first_name: &'a str,
    pub last_name: Option<&'a str>,
    pub phone: Option<&'a str>,
    pub email: Option<&'a str>,
    pub id_card: Option<&'a str>,
    pub address: Option<&'a str>,
    pub customer_type: Option<&'a str>,
    pub notes: Option<&'a str>,
}

/// PostgreSQL data operations for the customer aggregate.
///
/// All methods return `Result<_, sqlx::Error>`. Higher layers map this to
/// `ApiError` via the existing `From<sqlx::Error>` impl in `crate::error`.
#[async_trait]
pub trait CustomerRepository: Send + Sync {
    /// List with `(rows, total_count)`. SQL is built dynamically because the
    /// route already supports arbitrary `sort_by` + `LIKE` search.
    async fn list_with_count(
        &self,
        pool: &PgPool,
        params: &NewCustomersQuery,
    ) -> Result<(Vec<CustomerRow>, i32), sqlx::Error>;

    /// Fetch a single customer by id.
    async fn get(
        &self,
        pool: &PgPool,
        cust_id: i32,
    ) -> Result<Option<CustomerRow>, sqlx::Error>;

    /// Insert a new customer; returns its assigned `cust_id`.
    async fn insert(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        write: CustomerWrite<'_>,
    ) -> Result<i32, sqlx::Error>;

    /// Update an existing customer; returns the number of rows affected.
    async fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        cust_id: i32,
        write: CustomerWrite<'_>,
    ) -> Result<u64, sqlx::Error>;

    /// Soft-delete (set `cust_active=false`); returns rows affected.
    async fn soft_delete(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        cust_id: i32,
    ) -> Result<u64, sqlx::Error>;
}

/// Default `CustomerRepository` implementation backed by sqlx + PostgreSQL.
#[derive(Clone, Debug, Default)]
pub struct PgCustomerRepository;

impl PgCustomerRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CustomerRepository for PgCustomerRepository {
    async fn list_with_count(
        &self,
        pool: &PgPool,
        params: &NewCustomersQuery,
    ) -> Result<(Vec<CustomerRow>, i32), sqlx::Error> {
        let offset = (params.page - 1) * params.limit;
        let sort_order = params
            .sort_order
            .as_ref()
            .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
            .unwrap_or("ASC");

        let order_by_column = match params.sort_by.as_deref() {
            Some("firstName") => "cust_firstname",
            Some("lastName") => "cust_lastname",
            Some("phone") => "cust_phone",
            Some("email") => "cust_email",
            Some("createdAt") => "created_at",
            _ => "cust_id",
        };

        let mut conditions: Vec<String> = Vec::new();

        if params.active_only {
            conditions.push("cust_active = true".to_string());
        }

        // Build a parameterized LIKE pattern. Escape the LIKE-special characters
        // (`\`, `%`, `_`) so user input cannot inject wildcards or break out of
        // the pattern. The actual value is passed via sqlx `.bind($1)` below,
        // which prevents SQL injection at the protocol level.
        let like_pattern: Option<String> = params.search.as_ref().map(|search| {
            format!(
                "%{}%",
                search
                    .replace('\\', "\\\\")
                    .replace('%', "\\%")
                    .replace('_', "\\_")
            )
        });

        if like_pattern.is_some() {
            conditions.push(
                "(cust_firstname LIKE $1 ESCAPE '\\' \
                 OR cust_lastname LIKE $1 ESCAPE '\\' \
                 OR cust_phone LIKE $1 ESCAPE '\\' \
                 OR cust_email LIKE $1 ESCAPE '\\' \
                 OR cust_idcard LIKE $1 ESCAPE '\\')"
                    .to_string(),
            );
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let count_query = format!(
            "SELECT COUNT(*)::int as total FROM ht_customers {}",
            where_clause
        );

        let mut count_q = sqlx::query(&count_query);
        if let Some(ref pattern) = like_pattern {
            count_q = count_q.bind(pattern);
        }
        let count_rows = count_q.fetch_all(pool).await?;

        let total: i32 = count_rows
            .first()
            .map(|r| r.try_get::<i32, _>("total").unwrap_or(0))
            .unwrap_or(0);

        let data_query = format!(
            r#"
            SELECT
                cust_id,
                cust_firstname,
                cust_lastname,
                cust_phone,
                cust_email,
                cust_idcard,
                cust_address,
                cust_type,
                cust_notes,
                cust_active,
                created_at,
                updated_at
            FROM ht_customers
            {}
            ORDER BY {} {}
            LIMIT {} OFFSET {}
            "#,
            where_clause, order_by_column, sort_order, params.limit, offset
        );

        let mut data_q = sqlx::query(&data_query);
        if let Some(ref pattern) = like_pattern {
            data_q = data_q.bind(pattern);
        }
        let rows = data_q.fetch_all(pool).await?;

        let customers: Vec<CustomerRow> = rows
            .iter()
            .map(|row| CustomerRow {
                cust_id: row.try_get::<i32, _>("cust_id").unwrap_or(0),
                cust_firstname: row.try_get::<String, _>("cust_firstname").unwrap_or_default(),
                cust_lastname: row.try_get::<String, _>("cust_lastname").ok(),
                cust_phone: row.try_get::<String, _>("cust_phone").ok(),
                cust_email: row.try_get::<String, _>("cust_email").ok(),
                cust_idcard: row.try_get::<String, _>("cust_idcard").ok(),
                cust_address: row.try_get::<String, _>("cust_address").ok(),
                cust_type: row.try_get::<String, _>("cust_type").ok(),
                cust_notes: row.try_get::<String, _>("cust_notes").ok(),
                cust_active: row.try_get::<bool, _>("cust_active").ok(),
                created_at: row.try_get::<NaiveDateTime, _>("created_at").ok(),
                updated_at: row.try_get::<NaiveDateTime, _>("updated_at").ok(),
            })
            .collect();

        Ok((customers, total))
    }

    async fn get(
        &self,
        pool: &PgPool,
        cust_id: i32,
    ) -> Result<Option<CustomerRow>, sqlx::Error> {
        let rec = sqlx::query!(
            r#"
            SELECT
                cust_id,
                cust_firstname,
                cust_lastname,
                cust_phone,
                cust_email,
                cust_idcard,
                cust_address,
                cust_type,
                cust_notes,
                cust_active,
                created_at,
                updated_at
            FROM ht_customers
            WHERE cust_id = $1
            "#,
            cust_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(rec.map(|r| CustomerRow {
            cust_id: r.cust_id,
            cust_firstname: r.cust_firstname,
            cust_lastname: r.cust_lastname,
            cust_phone: r.cust_phone,
            cust_email: r.cust_email,
            cust_idcard: r.cust_idcard,
            cust_address: r.cust_address,
            cust_type: r.cust_type,
            cust_notes: r.cust_notes,
            cust_active: r.cust_active,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn insert(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        write: CustomerWrite<'_>,
    ) -> Result<i32, sqlx::Error> {
        let rec = sqlx::query!(
            r#"
            INSERT INTO ht_customers (
                cust_firstname,
                cust_lastname,
                cust_phone,
                cust_email,
                cust_idcard,
                cust_address,
                cust_type,
                cust_notes
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING cust_id
            "#,
            write.first_name,
            write.last_name,
            write.phone,
            write.email,
            write.id_card,
            write.address,
            write.customer_type,
            write.notes
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(rec.cust_id)
    }

    async fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        cust_id: i32,
        write: CustomerWrite<'_>,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE ht_customers
            SET cust_firstname = $1,
                cust_lastname = $2,
                cust_phone = $3,
                cust_email = $4,
                cust_idcard = $5,
                cust_address = $6,
                cust_type = $7,
                cust_notes = $8,
                updated_at = NOW()
            WHERE cust_id = $9
            "#,
            write.first_name,
            write.last_name,
            write.phone,
            write.email,
            write.id_card,
            write.address,
            write.customer_type,
            write.notes,
            cust_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(result.rows_affected())
    }

    async fn soft_delete(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        cust_id: i32,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE ht_customers
            SET cust_active = false,
                updated_at = NOW()
            WHERE cust_id = $1
            "#,
            cust_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(result.rows_affected())
    }
}
