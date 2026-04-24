//! Booking repository — PostgreSQL data access for `ht_bookings` and
//! `ht_booking_rooms`.
//!
//! Mirrors `routes::new_bookings` SQL behavior 1:1. The route keeps its
//! request/response DTOs (`NewBooking`, `NewBookingRoom`, etc.) and translates
//! between repository row shapes and the wire DTOs.
//!
//! Per `docs/architecture.md` §1, §6. Reads take `&PgPool` (single-shot);
//! writes take `&mut Transaction<'_, Postgres>` so the upcoming service layer
//! (Phase 2) can wrap booking-create + outbox-enqueue + event-publish in one
//! atomic commit.

use async_trait::async_trait;
use chrono::{NaiveDate, NaiveDateTime};
use sqlx::{PgPool, Postgres, Row, Transaction};

use crate::routes::new_bookings::NewBookingsQuery;

/// Result of `list_with_count` — one row per booking, with denormalized
/// customer name and room count.
#[derive(Debug, Clone)]
pub struct BookingListRow {
    pub book_id: i32,
    pub book_no: String,
    pub book_cust_id: i32,
    pub customer_name: Option<String>,
    pub book_checkin: Option<NaiveDateTime>,
    pub book_checkout: Option<NaiveDateTime>,
    pub book_nights: Option<i32>,
    pub book_adults: Option<i32>,
    pub book_children: Option<i32>,
    pub book_status: String,
    pub book_source: Option<String>,
    pub book_total_amount: Option<f64>,
    pub book_deposit_amount: Option<f64>,
    pub book_notes: Option<String>,
    pub room_count: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// Result of `get` — single booking detail (without rooms).
#[derive(Debug, Clone)]
pub struct BookingDetailRow {
    pub book_id: i32,
    pub book_no: String,
    pub book_cust_id: i32,
    pub customer_name: Option<String>,
    pub book_checkin: NaiveDate,
    pub book_checkout: NaiveDate,
    pub book_nights: Option<i32>,
    pub book_adults: Option<i32>,
    pub book_children: Option<i32>,
    pub book_status: Option<String>,
    pub book_source: Option<String>,
    pub book_total_amount: Option<f64>,
    pub book_deposit_amount: Option<f64>,
    pub book_notes: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// One assigned room for a booking (joined from `ht_booking_rooms` /
/// `ht_rooms_new` / `ht_room_types`).
#[derive(Debug, Clone)]
pub struct BookingRoomRow {
    pub br_id: i32,
    pub br_room_id: i32,
    pub room_no: String,
    pub type_name: String,
    pub br_price_per_night: Option<f64>,
}

/// Field set used by `insert` / `update`.
#[derive(Debug, Clone)]
pub struct BookingWrite<'a> {
    pub book_no: &'a str,
    pub customer_id: i32,
    pub check_in: NaiveDate,
    pub check_out: NaiveDate,
    pub adults: i32,
    pub children: i32,
    pub status: &'a str,
    pub source: Option<&'a str>,
    pub total_amount: Option<f64>,
    pub deposit_amount: Option<f64>,
    pub notes: Option<&'a str>,
}

/// One assigned room when creating/updating a booking.
#[derive(Debug, Clone, Copy)]
pub struct BookingRoomAssignment {
    pub room_id: i32,
    pub price_per_night: Option<f64>,
}

/// PostgreSQL data operations for the booking aggregate.
#[async_trait]
pub trait BookingRepository: Send + Sync {
    /// List with pagination + filters; returns `(rows, total_count)`.
    async fn list_with_count(
        &self,
        pool: &PgPool,
        params: &NewBookingsQuery,
    ) -> Result<(Vec<BookingListRow>, i32), sqlx::Error>;

    /// Get one booking by id (no rooms — see [`Self::list_rooms`]).
    async fn get(
        &self,
        pool: &PgPool,
        book_id: i32,
    ) -> Result<Option<BookingDetailRow>, sqlx::Error>;

    /// List rooms attached to a booking.
    async fn list_rooms(
        &self,
        pool: &PgPool,
        book_id: i32,
    ) -> Result<Vec<BookingRoomRow>, sqlx::Error>;

    /// Look up the latest YYYYMMDD sequence used in `book_no` so the route can
    /// generate the next number. Returns the last `book_no` (or None).
    async fn latest_book_no_today(
        &self,
        pool: &PgPool,
    ) -> Result<Option<String>, sqlx::Error>;

    /// Render today's date as `YYYYMMDD` via the database (preserves the
    /// original behavior — important if the DB is on a different TZ than the
    /// API process).
    async fn today_yyyymmdd(&self, pool: &PgPool) -> Result<String, sqlx::Error>;

    /// Insert a booking; returns the assigned `book_id`.
    async fn insert_booking(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        write: BookingWrite<'_>,
    ) -> Result<i32, sqlx::Error>;

    /// Insert one row into `ht_booking_rooms`.
    async fn insert_booking_room(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        book_id: i32,
        assignment: BookingRoomAssignment,
    ) -> Result<(), sqlx::Error>;

    /// Look up booking_no for an existing booking; used by update flow to
    /// 404 vs 200.
    async fn get_book_no(
        &self,
        pool: &PgPool,
        book_id: i32,
    ) -> Result<Option<String>, sqlx::Error>;

    /// Update an existing booking (does NOT touch its rooms).
    async fn update_booking(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        book_id: i32,
        write: BookingWrite<'_>,
    ) -> Result<u64, sqlx::Error>;

    /// Delete all `ht_booking_rooms` rows for a booking (used before re-inserting).
    async fn delete_booking_rooms(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        book_id: i32,
    ) -> Result<(), sqlx::Error>;

    /// Cancel a booking unless it's already terminal; returns rows affected.
    async fn cancel(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        book_id: i32,
    ) -> Result<u64, sqlx::Error>;
}

/// Default `BookingRepository` impl backed by sqlx + PostgreSQL.
#[derive(Clone, Debug, Default)]
pub struct PgBookingRepository;

impl PgBookingRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl BookingRepository for PgBookingRepository {
    async fn list_with_count(
        &self,
        pool: &PgPool,
        params: &NewBookingsQuery,
    ) -> Result<(Vec<BookingListRow>, i32), sqlx::Error> {
        let offset = (params.page - 1) * params.limit;
        let sort_order = params
            .sort_order
            .as_ref()
            .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
            .unwrap_or("DESC");

        let order_by_column = match params.sort_by.as_deref() {
            Some("bookNo") => "b.book_no",
            Some("customer") => "c.cust_firstname",
            Some("checkIn") => "b.book_checkin",
            Some("checkOut") => "b.book_checkout",
            Some("status") => "b.book_status",
            Some("totalAmount") => "b.book_total_amount",
            _ => "b.created_at",
        };

        let mut conditions: Vec<String> = Vec::new();

        if let Some(ref search) = params.search {
            let escaped = search.replace('\'', "''");
            conditions.push(format!(
                "(b.book_no LIKE '%{}%' OR c.cust_firstname LIKE '%{}%' OR c.cust_lastname LIKE '%{}%' OR c.cust_phone LIKE '%{}%')",
                escaped, escaped, escaped, escaped
            ));
        }

        if let Some(ref status) = params.status {
            let escaped = status.replace('\'', "''");
            conditions.push(format!("b.book_status = '{}'", escaped));
        }

        if let Some(ref start_date) = params.start_date {
            conditions.push(format!(
                "b.book_checkout::date >= '{}'",
                start_date.replace('\'', "''")
            ));
        }

        if let Some(ref end_date) = params.end_date {
            conditions.push(format!(
                "b.book_checkin::date <= '{}'",
                end_date.replace('\'', "''")
            ));
        }

        if let Some(cust_id) = params.customer_id {
            conditions.push(format!("b.book_cust_id = {}", cust_id));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let count_query = format!(
            r#"
        SELECT COUNT(DISTINCT b.book_id)::int as total
        FROM ht_bookings b
        LEFT JOIN ht_customers c ON b.book_cust_id = c.cust_id
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
            b.book_id,
            b.book_no,
            b.book_cust_id,
            CONCAT(c.cust_firstname, ' ', COALESCE(c.cust_lastname, '')) as customer_name,
            b.book_checkin,
            b.book_checkout,
            b.book_nights,
            b.book_adults,
            b.book_children,
            b.book_status,
            b.book_source,
            b.book_total_amount::float8 as book_total_amount,
            b.book_deposit_amount::float8 as book_deposit_amount,
            b.book_notes,
            b.created_at,
            b.updated_at,
            (SELECT COUNT(*)::int FROM ht_booking_rooms br WHERE br.br_book_id = b.book_id) as room_count
        FROM ht_bookings b
        LEFT JOIN ht_customers c ON b.book_cust_id = c.cust_id
        {}
        ORDER BY {} {}
        LIMIT {} OFFSET {}
        "#,
            where_clause, order_by_column, sort_order, params.limit, offset
        );

        let rows = sqlx::query(&data_query).fetch_all(pool).await?;

        let bookings: Vec<BookingListRow> = rows
            .iter()
            .map(|row| BookingListRow {
                book_id: row.try_get::<i32, _>("book_id").unwrap_or(0),
                book_no: row.try_get::<String, _>("book_no").unwrap_or_default(),
                book_cust_id: row.try_get::<i32, _>("book_cust_id").unwrap_or(0),
                customer_name: row.try_get::<String, _>("customer_name").ok(),
                book_checkin: row.try_get::<NaiveDateTime, _>("book_checkin").ok(),
                book_checkout: row.try_get::<NaiveDateTime, _>("book_checkout").ok(),
                book_nights: row.try_get::<i32, _>("book_nights").ok(),
                book_adults: row.try_get::<i32, _>("book_adults").ok(),
                book_children: row.try_get::<i32, _>("book_children").ok(),
                book_status: row
                    .try_get::<String, _>("book_status")
                    .unwrap_or_else(|_| "pending".to_string()),
                book_source: row.try_get::<String, _>("book_source").ok(),
                book_total_amount: row.try_get::<f64, _>("book_total_amount").ok(),
                book_deposit_amount: row.try_get::<f64, _>("book_deposit_amount").ok(),
                book_notes: row.try_get::<String, _>("book_notes").ok(),
                room_count: row.try_get::<i32, _>("room_count").unwrap_or(0),
                created_at: row.try_get::<NaiveDateTime, _>("created_at").ok(),
                updated_at: row.try_get::<NaiveDateTime, _>("updated_at").ok(),
            })
            .collect();

        Ok((bookings, total))
    }

    async fn get(
        &self,
        pool: &PgPool,
        book_id: i32,
    ) -> Result<Option<BookingDetailRow>, sqlx::Error> {
        let rec = sqlx::query!(
            r#"SELECT
            b.book_id,
            b.book_no,
            b.book_cust_id,
            CONCAT(c.cust_firstname, ' ', COALESCE(c.cust_lastname, '')) as customer_name,
            b.book_checkin,
            b.book_checkout,
            b.book_nights,
            b.book_adults,
            b.book_children,
            b.book_status,
            b.book_source,
            b.book_total_amount::float8 as book_total_amount,
            b.book_deposit_amount::float8 as book_deposit_amount,
            b.book_notes,
            b.created_at,
            b.updated_at
        FROM ht_bookings b
        LEFT JOIN ht_customers c ON b.book_cust_id = c.cust_id
        WHERE b.book_id = $1"#,
            book_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(rec.map(|r| BookingDetailRow {
            book_id: r.book_id,
            book_no: r.book_no,
            book_cust_id: r.book_cust_id,
            customer_name: r.customer_name,
            book_checkin: r.book_checkin,
            book_checkout: r.book_checkout,
            book_nights: r.book_nights,
            book_adults: r.book_adults,
            book_children: r.book_children,
            book_status: r.book_status,
            book_source: r.book_source,
            book_total_amount: r.book_total_amount,
            book_deposit_amount: r.book_deposit_amount,
            book_notes: r.book_notes,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn list_rooms(
        &self,
        pool: &PgPool,
        book_id: i32,
    ) -> Result<Vec<BookingRoomRow>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"SELECT
            br.br_id,
            br.br_room_id,
            r.room_no,
            rt.type_name,
            br.br_price_per_night::float8 as br_price_per_night
        FROM ht_booking_rooms br
        LEFT JOIN ht_rooms_new r ON br.br_room_id = r.room_id
        LEFT JOIN ht_room_types rt ON r.room_type_id = rt.type_id
        WHERE br.br_book_id = $1"#,
            book_id
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| BookingRoomRow {
                br_id: r.br_id,
                br_room_id: r.br_room_id,
                room_no: r.room_no,
                type_name: r.type_name,
                br_price_per_night: r.br_price_per_night,
            })
            .collect())
    }

    async fn latest_book_no_today(
        &self,
        pool: &PgPool,
    ) -> Result<Option<String>, sqlx::Error> {
        let rec = sqlx::query!(
            r#"SELECT book_no FROM ht_bookings
        WHERE book_no LIKE TO_CHAR(NOW(), 'YYYYMMDD') || '-%'
        ORDER BY book_no DESC LIMIT 1"#
        )
        .fetch_optional(pool)
        .await?;

        Ok(rec.map(|r| r.book_no))
    }

    async fn today_yyyymmdd(&self, pool: &PgPool) -> Result<String, sqlx::Error> {
        let rec = sqlx::query!("SELECT TO_CHAR(NOW(), 'YYYYMMDD') as today")
            .fetch_one(pool)
            .await?;
        Ok(rec.today.unwrap_or_else(|| "00000000".to_string()))
    }

    async fn insert_booking(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        write: BookingWrite<'_>,
    ) -> Result<i32, sqlx::Error> {
        let rec = sqlx::query!(
            r#"INSERT INTO ht_bookings (book_no, book_cust_id, book_checkin, book_checkout, book_adults, book_children, book_status, book_source, book_total_amount, book_deposit_amount, book_notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9::float8, $10::float8, $11)
        RETURNING book_id"#,
            write.book_no,
            write.customer_id,
            write.check_in,
            write.check_out,
            write.adults,
            write.children,
            write.status,
            write.source,
            write.total_amount,
            write.deposit_amount,
            write.notes
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(rec.book_id)
    }

    async fn insert_booking_room(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        book_id: i32,
        assignment: BookingRoomAssignment,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO ht_booking_rooms (br_book_id, br_room_id, br_price_per_night) VALUES ($1, $2, $3::float8)"#,
            book_id,
            assignment.room_id,
            assignment.price_per_night
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn get_book_no(
        &self,
        pool: &PgPool,
        book_id: i32,
    ) -> Result<Option<String>, sqlx::Error> {
        let rec = sqlx::query!(
            "SELECT book_no FROM ht_bookings WHERE book_id = $1",
            book_id
        )
        .fetch_optional(pool)
        .await?;
        Ok(rec.map(|r| r.book_no))
    }

    async fn update_booking(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        book_id: i32,
        write: BookingWrite<'_>,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"UPDATE ht_bookings SET book_cust_id = $1, book_checkin = $2, book_checkout = $3, book_adults = $4, book_children = $5, book_status = $6, book_source = $7, book_total_amount = $8::float8, book_deposit_amount = $9::float8, book_notes = $10, updated_at = NOW()
        WHERE book_id = $11"#,
            write.customer_id,
            write.check_in,
            write.check_out,
            write.adults,
            write.children,
            write.status,
            write.source,
            write.total_amount,
            write.deposit_amount,
            write.notes,
            book_id
        )
        .execute(&mut **tx)
        .await?;

        Ok(result.rows_affected())
    }

    async fn delete_booking_rooms(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        book_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM ht_booking_rooms WHERE br_book_id = $1",
            book_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn cancel(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        book_id: i32,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"UPDATE ht_bookings SET book_status = 'cancelled', updated_at = NOW()
        WHERE book_id = $1 AND book_status NOT IN ('completed', 'cancelled')"#,
            book_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(result.rows_affected())
    }
}
