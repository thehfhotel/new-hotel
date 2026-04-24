//! Check-in repository — PostgreSQL data access for `ht_checkins` and the
//! related `ht_guest_registry`.
//!
//! Mirrors `routes::new_checkins` SQL behavior 1:1. The route keeps its
//! request/response DTOs.
//!
//! Per `docs/architecture.md` §1, §6. The check-in flow (walk-in, link to
//! booking, check-out, cancel) is intentionally exposed as data ops here —
//! the multi-step business logic (e.g. "checking-in must update room status
//! AND booking status") moves to the service layer in Phase 2 where the
//! caller composes these methods inside a single transaction.

use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{PgPool, Postgres, Row, Transaction};

use crate::routes::new_checkins::NewCheckInsQuery;

/// Result of `list_with_count` — one check-in with denormalized customer/room/booking joins.
#[derive(Debug, Clone)]
pub struct CheckInListRow {
    pub cin_id: i32,
    pub cin_no: String,
    pub cin_book_id: Option<i32>,
    pub book_no: Option<String>,
    pub cin_cust_id: i32,
    pub customer_name: Option<String>,
    pub cin_room_id: i32,
    pub room_no: Option<String>,
    pub type_name: Option<String>,
    pub cin_checkin_time: Option<NaiveDateTime>,
    pub cin_checkout_time: Option<NaiveDateTime>,
    pub cin_expected_checkout: Option<NaiveDateTime>,
    pub cin_adults: Option<i32>,
    pub cin_children: Option<i32>,
    pub cin_status: String,
    pub cin_rate_per_night: Option<f64>,
    pub cin_total_amount: Option<f64>,
    pub cin_payment_status: Option<String>,
    pub cin_notes: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// Result of `get` — single check-in detail.
#[derive(Debug, Clone)]
pub struct CheckInDetailRow {
    pub cin_id: i32,
    pub cin_no: String,
    pub cin_book_id: Option<i32>,
    pub book_no: String,
    pub cin_cust_id: i32,
    pub customer_name: Option<String>,
    pub cin_room_id: i32,
    pub room_no: String,
    pub type_name: String,
    pub cin_checkin_time: NaiveDateTime,
    pub cin_checkout_time: Option<NaiveDateTime>,
    pub cin_expected_checkout: NaiveDate,
    pub cin_adults: Option<i32>,
    pub cin_children: Option<i32>,
    pub cin_status: Option<String>,
    pub cin_rate_per_night: Option<f64>,
    pub cin_total_amount: Option<f64>,
    pub cin_payment_status: Option<String>,
    pub cin_notes: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// Field set used by `insert` — one row in `ht_checkins`.
#[derive(Debug, Clone)]
pub struct CheckInInsert<'a> {
    pub cin_no: &'a str,
    pub booking_id: Option<i32>,
    pub customer_id: i32,
    pub room_id: i32,
    pub check_in_time: Option<NaiveDateTime>,
    pub expected_checkout: NaiveDate,
    pub adults: i32,
    pub children: i32,
    pub rate_per_night: Option<f64>,
    pub notes: Option<&'a str>,
}

/// Status snapshot returned by `find_status` — used by checkout/guest endpoints
/// to decide whether the operation is permitted.
#[derive(Debug, Clone)]
pub struct CheckInStatus {
    pub cin_id: i32,
    pub cin_no: String,
    pub cin_room_id: i32,
    pub cin_book_id: Option<i32>,
    pub cin_status: Option<String>,
}

/// Mutable inputs for `apply_checkout`.
#[derive(Debug, Clone)]
pub struct CheckOutWrite<'a> {
    pub check_out_time: Option<NaiveDateTime>,
    pub total_amount: Option<f64>,
    pub payment_status: &'a str,
    pub notes: Option<&'a str>,
}

/// One row in `ht_guest_registry`.
#[derive(Debug, Clone)]
pub struct GuestRow {
    pub guest_id: i32,
    pub guest_cin_id: i32,
    pub guest_cust_id: Option<i32>,
    pub guest_firstname: String,
    pub guest_lastname: Option<String>,
    pub guest_idcard: Option<String>,
    pub guest_passport: Option<String>,
    pub guest_nationality: Option<String>,
    pub guest_is_primary: Option<bool>,
    pub guest_created_at: Option<NaiveDateTime>,
}

/// Field set used by `insert_guest`.
#[derive(Debug, Clone)]
pub struct GuestInsert<'a> {
    pub cin_id: i32,
    pub cust_id: Option<i32>,
    pub first_name: &'a str,
    pub last_name: Option<&'a str>,
    pub id_card: Option<&'a str>,
    pub passport: Option<&'a str>,
    pub nationality: Option<&'a str>,
    pub is_primary: bool,
}

/// PostgreSQL data operations for the check-in aggregate.
#[async_trait]
pub trait CheckInRepository: Send + Sync {
    /// List with pagination + filters; returns `(rows, total_count)`.
    async fn list_with_count(
        &self,
        pool: &PgPool,
        params: &NewCheckInsQuery,
    ) -> Result<(Vec<CheckInListRow>, i32), sqlx::Error>;

    /// Get one check-in detail by id.
    async fn get(
        &self,
        pool: &PgPool,
        cin_id: i32,
    ) -> Result<Option<CheckInDetailRow>, sqlx::Error>;

    /// Look up the customer attached to a booking — used when promoting a
    /// booking to a check-in.
    async fn get_booking_customer_id(
        &self,
        pool: &PgPool,
        booking_id: i32,
    ) -> Result<Option<i32>, sqlx::Error>;

    /// Count active check-ins for a room — used to detect double-booking.
    async fn count_active_for_room(
        &self,
        pool: &PgPool,
        room_id: i32,
    ) -> Result<i32, sqlx::Error>;

    /// Latest `cin_no` for today (for sequence generation).
    async fn latest_cin_no_today(
        &self,
        pool: &PgPool,
    ) -> Result<Option<String>, sqlx::Error>;

    /// Render today as `YYYYMMDD` via the database (matches the existing route).
    async fn today_yyyymmdd(&self, pool: &PgPool) -> Result<String, sqlx::Error>;

    /// Insert a check-in; returns the assigned `cin_id`.
    async fn insert(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        write: CheckInInsert<'_>,
    ) -> Result<i32, sqlx::Error>;

    /// Mark a room as occupied. Used after a successful insert.
    async fn mark_room_occupied(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        room_id: i32,
    ) -> Result<(), sqlx::Error>;

    /// Mark a room available + dirty. Used after check-out.
    async fn mark_room_available_dirty(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        room_id: i32,
    ) -> Result<(), sqlx::Error>;

    /// Set a booking's `book_status = 'checkedin'` (linked check-in).
    async fn set_booking_checkedin(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        booking_id: i32,
    ) -> Result<(), sqlx::Error>;

    /// Set a booking's `book_status = 'completed'` (post check-out).
    async fn set_booking_completed(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        booking_id: i32,
    ) -> Result<(), sqlx::Error>;

    /// Look up the small status snapshot used to gate checkout / guest changes.
    async fn find_status(
        &self,
        pool: &PgPool,
        cin_id: i32,
    ) -> Result<Option<CheckInStatus>, sqlx::Error>;

    /// Apply checkout fields to an active check-in.
    async fn apply_checkout(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        cin_id: i32,
        write: CheckOutWrite<'_>,
    ) -> Result<(), sqlx::Error>;

    // ------- Guest registry -------

    /// Confirm a check-in id exists (without loading anything else).
    async fn exists(&self, pool: &PgPool, cin_id: i32) -> Result<bool, sqlx::Error>;

    /// List guests for a check-in (ordered: primary first, then by id).
    async fn list_guests(
        &self,
        pool: &PgPool,
        cin_id: i32,
    ) -> Result<Vec<GuestRow>, sqlx::Error>;

    /// Find a guest scoped to its check-in (used for delete authorization).
    async fn find_guest_in_checkin(
        &self,
        pool: &PgPool,
        cin_id: i32,
        guest_id: i32,
    ) -> Result<Option<i32>, sqlx::Error>;

    /// Clear `guest_is_primary` for every guest of a check-in (used before
    /// inserting a new primary).
    async fn unset_primary_guests(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        cin_id: i32,
    ) -> Result<(), sqlx::Error>;

    /// Insert a guest; returns the assigned `guest_id`.
    async fn insert_guest(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        insert: GuestInsert<'_>,
    ) -> Result<i32, sqlx::Error>;

    /// Delete a guest scoped to its check-in.
    async fn delete_guest(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        cin_id: i32,
        guest_id: i32,
    ) -> Result<(), sqlx::Error>;
}

/// Default `CheckInRepository` impl backed by sqlx + PostgreSQL.
#[derive(Clone, Debug, Default)]
pub struct PgCheckInRepository;

impl PgCheckInRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CheckInRepository for PgCheckInRepository {
    async fn list_with_count(
        &self,
        pool: &PgPool,
        params: &NewCheckInsQuery,
    ) -> Result<(Vec<CheckInListRow>, i32), sqlx::Error> {
        let offset = (params.page - 1) * params.limit;
        let sort_order = params
            .sort_order
            .as_ref()
            .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
            .unwrap_or("DESC");

        let order_by_column = match params.sort_by.as_deref() {
            Some("cinNo") => "ci.cin_no",
            Some("customer") => "c.cust_firstname",
            Some("room") => "r.room_no",
            Some("checkInTime") => "ci.cin_checkin_time",
            Some("checkOutTime") => "ci.cin_checkout_time",
            Some("status") => "ci.cin_status",
            _ => "ci.cin_checkin_time",
        };

        let mut conditions: Vec<String> = Vec::new();

        if let Some(ref status) = params.status {
            let escaped = status.replace('\'', "''");
            conditions.push(format!("ci.cin_status = '{}'", escaped));
        }

        if let Some(ref start_date) = params.start_date {
            conditions.push(format!(
                "COALESCE(ci.cin_checkout_time, ci.cin_expected_checkout)::date >= '{}'",
                start_date.replace('\'', "''")
            ));
        }

        if let Some(ref end_date) = params.end_date {
            conditions.push(format!(
                "ci.cin_checkin_time::date <= '{}'",
                end_date.replace('\'', "''")
            ));
        }

        if let Some(room_id) = params.room_id {
            conditions.push(format!("ci.cin_room_id = {}", room_id));
        }

        if let Some(cust_id) = params.customer_id {
            conditions.push(format!("ci.cin_cust_id = {}", cust_id));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let count_query = format!(
            r#"
        SELECT COUNT(*)::int as total
        FROM ht_checkins ci
        {}
        "#,
            where_clause
        );

        let count_rows = sqlx::query(&count_query).fetch_all(pool).await?;

        let total: i32 = count_rows
            .first()
            .map(|r| r.try_get::<i32, _>("total").unwrap_or(0))
            .unwrap_or(0);

        let data_query = format!(
            r#"
        SELECT
            ci.cin_id,
            ci.cin_no,
            ci.cin_book_id,
            b.book_no,
            ci.cin_cust_id,
            CONCAT(c.cust_firstname, ' ', COALESCE(c.cust_lastname, '')) as customer_name,
            ci.cin_room_id,
            r.room_no,
            rt.type_name,
            ci.cin_checkin_time,
            ci.cin_checkout_time,
            ci.cin_expected_checkout,
            ci.cin_adults,
            ci.cin_children,
            ci.cin_status,
            ci.cin_rate_per_night,
            ci.cin_total_amount,
            ci.cin_payment_status,
            ci.cin_notes,
            ci.created_at,
            ci.updated_at
        FROM ht_checkins ci
        LEFT JOIN ht_customers c ON ci.cin_cust_id = c.cust_id
        LEFT JOIN ht_rooms_new r ON ci.cin_room_id = r.room_id
        LEFT JOIN ht_room_types rt ON r.room_type_id = rt.type_id
        LEFT JOIN ht_bookings b ON ci.cin_book_id = b.book_id
        {}
        ORDER BY {} {}
        LIMIT {} OFFSET {}
        "#,
            where_clause, order_by_column, sort_order, params.limit, offset
        );

        let rows = sqlx::query(&data_query).fetch_all(pool).await?;

        let checkins: Vec<CheckInListRow> = rows
            .iter()
            .map(|row| CheckInListRow {
                cin_id: row.try_get::<i32, _>("cin_id").unwrap_or(0),
                cin_no: row.try_get::<String, _>("cin_no").unwrap_or_default(),
                cin_book_id: row.try_get::<i32, _>("cin_book_id").ok(),
                book_no: row.try_get::<String, _>("book_no").ok(),
                cin_cust_id: row.try_get::<i32, _>("cin_cust_id").unwrap_or(0),
                customer_name: row.try_get::<String, _>("customer_name").ok(),
                cin_room_id: row.try_get::<i32, _>("cin_room_id").unwrap_or(0),
                room_no: row.try_get::<String, _>("room_no").ok(),
                type_name: row.try_get::<String, _>("type_name").ok(),
                cin_checkin_time: row.try_get::<NaiveDateTime, _>("cin_checkin_time").ok(),
                cin_checkout_time: row.try_get::<NaiveDateTime, _>("cin_checkout_time").ok(),
                cin_expected_checkout: row
                    .try_get::<NaiveDateTime, _>("cin_expected_checkout")
                    .ok(),
                cin_adults: row.try_get::<i32, _>("cin_adults").ok(),
                cin_children: row.try_get::<i32, _>("cin_children").ok(),
                cin_status: row
                    .try_get::<String, _>("cin_status")
                    .unwrap_or_else(|_| "active".to_string()),
                cin_rate_per_night: row.try_get::<f64, _>("cin_rate_per_night").ok(),
                cin_total_amount: row.try_get::<f64, _>("cin_total_amount").ok(),
                cin_payment_status: row.try_get::<String, _>("cin_payment_status").ok(),
                cin_notes: row.try_get::<String, _>("cin_notes").ok(),
                created_at: row.try_get::<NaiveDateTime, _>("created_at").ok(),
                updated_at: row.try_get::<NaiveDateTime, _>("updated_at").ok(),
            })
            .collect();

        Ok((checkins, total))
    }

    async fn get(
        &self,
        pool: &PgPool,
        cin_id: i32,
    ) -> Result<Option<CheckInDetailRow>, sqlx::Error> {
        let rec = sqlx::query!(
            r#"
        SELECT
            ci.cin_id,
            ci.cin_no,
            ci.cin_book_id,
            b.book_no,
            ci.cin_cust_id,
            CONCAT(c.cust_firstname, ' ', COALESCE(c.cust_lastname, '')) as customer_name,
            ci.cin_room_id,
            r.room_no,
            rt.type_name,
            ci.cin_checkin_time,
            ci.cin_checkout_time,
            ci.cin_expected_checkout,
            ci.cin_adults,
            ci.cin_children,
            ci.cin_status,
            ci.cin_rate_per_night::float8 as cin_rate_per_night,
            ci.cin_total_amount::float8 as cin_total_amount,
            ci.cin_payment_status,
            ci.cin_notes,
            ci.created_at,
            ci.updated_at
        FROM ht_checkins ci
        LEFT JOIN ht_customers c ON ci.cin_cust_id = c.cust_id
        LEFT JOIN ht_rooms_new r ON ci.cin_room_id = r.room_id
        LEFT JOIN ht_room_types rt ON r.room_type_id = rt.type_id
        LEFT JOIN ht_bookings b ON ci.cin_book_id = b.book_id
        WHERE ci.cin_id = $1
        "#,
            cin_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(rec.map(|r| CheckInDetailRow {
            cin_id: r.cin_id,
            cin_no: r.cin_no,
            cin_book_id: r.cin_book_id,
            book_no: r.book_no,
            cin_cust_id: r.cin_cust_id,
            customer_name: r.customer_name,
            cin_room_id: r.cin_room_id,
            room_no: r.room_no,
            type_name: r.type_name,
            cin_checkin_time: r.cin_checkin_time,
            cin_checkout_time: r.cin_checkout_time,
            cin_expected_checkout: r.cin_expected_checkout,
            cin_adults: r.cin_adults,
            cin_children: r.cin_children,
            cin_status: r.cin_status,
            cin_rate_per_night: r.cin_rate_per_night,
            cin_total_amount: r.cin_total_amount,
            cin_payment_status: r.cin_payment_status,
            cin_notes: r.cin_notes,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn get_booking_customer_id(
        &self,
        pool: &PgPool,
        booking_id: i32,
    ) -> Result<Option<i32>, sqlx::Error> {
        let rec = sqlx::query!(
            "SELECT book_cust_id FROM ht_bookings WHERE book_id = $1",
            booking_id
        )
        .fetch_optional(pool)
        .await?;
        Ok(rec.map(|r| r.book_cust_id))
    }

    async fn count_active_for_room(
        &self,
        pool: &PgPool,
        room_id: i32,
    ) -> Result<i32, sqlx::Error> {
        let rec = sqlx::query!(
            r#"SELECT COUNT(*)::int as active_count FROM ht_checkins
        WHERE cin_room_id = $1 AND cin_status = 'active'"#,
            room_id
        )
        .fetch_one(pool)
        .await?;
        Ok(rec.active_count.unwrap_or(0))
    }

    async fn latest_cin_no_today(
        &self,
        pool: &PgPool,
    ) -> Result<Option<String>, sqlx::Error> {
        let rec = sqlx::query!(
            r#"SELECT cin_no FROM ht_checkins
        WHERE cin_no LIKE 'CIN-' || TO_CHAR(NOW(), 'YYYYMMDD') || '-%'
        ORDER BY cin_no DESC LIMIT 1"#
        )
        .fetch_optional(pool)
        .await?;
        Ok(rec.map(|r| r.cin_no))
    }

    async fn today_yyyymmdd(&self, pool: &PgPool) -> Result<String, sqlx::Error> {
        let rec = sqlx::query!("SELECT TO_CHAR(NOW(), 'YYYYMMDD') as today")
            .fetch_one(pool)
            .await?;
        Ok(rec.today.unwrap_or_else(|| "00000000".to_string()))
    }

    async fn insert(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        write: CheckInInsert<'_>,
    ) -> Result<i32, sqlx::Error> {
        let rec = sqlx::query!(
            r#"INSERT INTO ht_checkins (cin_no, cin_book_id, cin_cust_id, cin_room_id, cin_checkin_time, cin_expected_checkout, cin_adults, cin_children, cin_status, cin_rate_per_night, cin_notes)
        VALUES ($1, $2, $3, $4, COALESCE($5, NOW()::timestamp), $6, $7, $8, 'active', $9::float8, $10)
        RETURNING cin_id"#,
            write.cin_no,
            write.booking_id,
            write.customer_id,
            write.room_id,
            write.check_in_time,
            write.expected_checkout,
            write.adults,
            write.children,
            write.rate_per_night,
            write.notes
        )
        .fetch_one(&mut **tx)
        .await?;
        Ok(rec.cin_id)
    }

    async fn mark_room_occupied(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        room_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE ht_rooms_new SET room_status = 'occupied', updated_at = NOW() WHERE room_id = $1"#,
            room_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn mark_room_available_dirty(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        room_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE ht_rooms_new SET room_status = 'available', room_clean = false, updated_at = NOW() WHERE room_id = $1"#,
            room_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn set_booking_checkedin(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        booking_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE ht_bookings SET book_status = 'checkedin', updated_at = NOW() WHERE book_id = $1"#,
            booking_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn set_booking_completed(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        booking_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE ht_bookings SET book_status = 'completed', updated_at = NOW() WHERE book_id = $1"#,
            booking_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn find_status(
        &self,
        pool: &PgPool,
        cin_id: i32,
    ) -> Result<Option<CheckInStatus>, sqlx::Error> {
        let rec = sqlx::query!(
            r#"SELECT cin_no, cin_room_id, cin_book_id, cin_status FROM ht_checkins WHERE cin_id = $1"#,
            cin_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(rec.map(|r| CheckInStatus {
            cin_id,
            cin_no: r.cin_no,
            cin_room_id: r.cin_room_id,
            cin_book_id: r.cin_book_id,
            cin_status: r.cin_status,
        }))
    }

    async fn apply_checkout(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        cin_id: i32,
        write: CheckOutWrite<'_>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE ht_checkins SET cin_checkout_time = COALESCE($1, NOW()::timestamp), cin_status = 'checkedout',
        cin_total_amount = COALESCE($2::float8, cin_total_amount), cin_payment_status = $3,
        cin_notes = COALESCE($4, cin_notes), updated_at = NOW() WHERE cin_id = $5"#,
            write.check_out_time,
            write.total_amount,
            write.payment_status,
            write.notes,
            cin_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn exists(&self, pool: &PgPool, cin_id: i32) -> Result<bool, sqlx::Error> {
        let rec = sqlx::query!("SELECT cin_id FROM ht_checkins WHERE cin_id = $1", cin_id)
            .fetch_optional(pool)
            .await?;
        Ok(rec.is_some())
    }

    async fn list_guests(
        &self,
        pool: &PgPool,
        cin_id: i32,
    ) -> Result<Vec<GuestRow>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"SELECT guest_id, guest_cin_id, guest_cust_id, guest_firstname, guest_lastname,
            guest_idcard, guest_passport, guest_nationality, guest_is_primary, guest_created_at
        FROM ht_guest_registry WHERE guest_cin_id = $1
        ORDER BY guest_is_primary DESC, guest_id ASC"#,
            cin_id
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| GuestRow {
                guest_id: r.guest_id,
                guest_cin_id: r.guest_cin_id,
                guest_cust_id: r.guest_cust_id,
                guest_firstname: r.guest_firstname,
                guest_lastname: r.guest_lastname,
                guest_idcard: r.guest_idcard,
                guest_passport: r.guest_passport,
                guest_nationality: r.guest_nationality,
                guest_is_primary: r.guest_is_primary,
                guest_created_at: r.guest_created_at,
            })
            .collect())
    }

    async fn find_guest_in_checkin(
        &self,
        pool: &PgPool,
        cin_id: i32,
        guest_id: i32,
    ) -> Result<Option<i32>, sqlx::Error> {
        let rec = sqlx::query!(
            "SELECT guest_id FROM ht_guest_registry WHERE guest_id = $1 AND guest_cin_id = $2",
            guest_id,
            cin_id
        )
        .fetch_optional(pool)
        .await?;
        Ok(rec.map(|r| r.guest_id))
    }

    async fn unset_primary_guests(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        cin_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE ht_guest_registry SET guest_is_primary = false WHERE guest_cin_id = $1",
            cin_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn insert_guest(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        insert: GuestInsert<'_>,
    ) -> Result<i32, sqlx::Error> {
        let rec = sqlx::query!(
            r#"INSERT INTO ht_guest_registry (guest_cin_id, guest_cust_id, guest_firstname, guest_lastname, guest_idcard, guest_passport, guest_nationality, guest_is_primary)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING guest_id"#,
            insert.cin_id,
            insert.cust_id,
            insert.first_name,
            insert.last_name,
            insert.id_card,
            insert.passport,
            insert.nationality,
            insert.is_primary
        )
        .fetch_one(&mut **tx)
        .await?;
        Ok(rec.guest_id)
    }

    async fn delete_guest(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        cin_id: i32,
        guest_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM ht_guest_registry WHERE guest_id = $1 AND guest_cin_id = $2",
            guest_id,
            cin_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }
}
