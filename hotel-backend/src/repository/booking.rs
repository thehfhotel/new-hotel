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
    /// Legacy MSSQL identifier (`R\d{6}`) — populated by the writeback worker
    /// after the booking is mirrored to the .NET app's DB. `None` until then.
    /// Surfaced to the UI so receptionists can cross-reference between our
    /// app and the legacy app on the same booking.
    pub legacy_book_id: Option<String>,
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
    /// See `BookingListRow::legacy_book_id`.
    pub legacy_book_id: Option<String>,
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

/// One pre-ordered product line attached to a booking (task #52 / migration
/// 061 — canonical `ht_booking_products`). `unit_price` is optional: when
/// `None` the INSERT defaults it from the product's catalog price
/// (`ht_products.prod_price`) so the form can omit the price for a product it
/// hasn't looked up. The legacy `HT_Book_Pro` write-back is deferred (shape
/// unverified), so these rows are canonical-only for now.
#[derive(Debug, Clone)]
pub struct BookingProductAssignment {
    pub product_id: i64,
    pub qty: f64,
    pub unit_price: Option<f64>,
    pub note: Option<String>,
    pub aggregate_id: uuid::Uuid,
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

    /// Insert one pre-ordered product line into `ht_booking_products` (task #52
    /// / migration 061). Uses a runtime `sqlx::query` (no `query!` macro, so no
    /// `.sqlx` cache entry) — the table is newer than the offline schema
    /// snapshot. `unit_price` defaults from `ht_products.prod_price` when the
    /// caller passes `None`.
    async fn insert_booking_product(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        book_id: i32,
        product: BookingProductAssignment,
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

    /// Stamp the deterministic aggregate UUID onto a freshly-inserted booking.
    /// The service layer computes this via `service::ids::aggregate_uuid` once
    /// `insert_booking` returns the SERIAL `book_id`. Without it, the writeback
    /// worker's resolver cannot map the UUID in `writeback_jobs.aggregate_id`
    /// back to a row in `ht_bookings`.
    async fn set_aggregate_id(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        book_id: i32,
        aggregate_id: uuid::Uuid,
    ) -> Result<(), sqlx::Error>;
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

        // Build a parameterized WHERE clause. All user-supplied filter values
        // flow through `.bind()` (see below), never via string interpolation.
        // For the LIKE search we additionally escape `\`, `%`, `_` so the user
        // cannot inject wildcards or break out of the pattern; the literal
        // value still rides on a parameter slot — `ESCAPE '\\'` only changes
        // how PostgreSQL interprets the bound text.
        let search_pattern: Option<String> = params.search.as_ref().map(|search| {
            format!(
                "%{}%",
                search
                    .replace('\\', "\\\\")
                    .replace('%', "\\%")
                    .replace('_', "\\_")
            )
        });

        let mut conditions: Vec<String> = Vec::new();
        let mut next_idx: usize = 0;

        if search_pattern.is_some() {
            next_idx += 1;
            let i = next_idx;
            conditions.push(format!(
                "(b.book_no LIKE ${i} ESCAPE '\\' \
                 OR c.cust_firstname LIKE ${i} ESCAPE '\\' \
                 OR c.cust_lastname LIKE ${i} ESCAPE '\\' \
                 OR c.cust_phone LIKE ${i} ESCAPE '\\')",
                i = i
            ));
        }

        if params.status.is_some() {
            next_idx += 1;
            conditions.push(format!("b.book_status = ${}", next_idx));
        }

        if params.start_date.is_some() {
            next_idx += 1;
            conditions.push(format!("b.book_checkout::date >= ${}::date", next_idx));
        }

        if params.end_date.is_some() {
            next_idx += 1;
            conditions.push(format!("b.book_checkin::date <= ${}::date", next_idx));
        }

        if params.customer_id.is_some() {
            next_idx += 1;
            conditions.push(format!("b.book_cust_id = ${}", next_idx));
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

        // Bind in the SAME order the conditions were appended above: search,
        // status, start_date, end_date, customer_id. sqlx binds positionally
        // by call order, not by `$N` literal — so the order must match the
        // `next_idx` increments above.
        let mut count_q = sqlx::query(sqlx::AssertSqlSafe(&*count_query));
        if let Some(ref pattern) = search_pattern {
            count_q = count_q.bind(pattern);
        }
        if let Some(ref status) = params.status {
            count_q = count_q.bind(status);
        }
        if let Some(ref start_date) = params.start_date {
            count_q = count_q.bind(start_date);
        }
        if let Some(ref end_date) = params.end_date {
            count_q = count_q.bind(end_date);
        }
        if let Some(cust_id) = params.customer_id {
            count_q = count_q.bind(cust_id);
        }
        let count_rows = count_q.fetch_all(pool).await?;

        let total: i32 = count_rows
            .first()
            .map(|r| r.try_get::<i32, _>("total").unwrap_or(0))
            .unwrap_or(0);

        let data_query = format!(
            r#"
        SELECT
            b.book_id,
            b.book_no,
            b.legacy_book_id,
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

        let mut data_q = sqlx::query(sqlx::AssertSqlSafe(&*data_query));
        if let Some(ref pattern) = search_pattern {
            data_q = data_q.bind(pattern);
        }
        if let Some(ref status) = params.status {
            data_q = data_q.bind(status);
        }
        if let Some(ref start_date) = params.start_date {
            data_q = data_q.bind(start_date);
        }
        if let Some(ref end_date) = params.end_date {
            data_q = data_q.bind(end_date);
        }
        if let Some(cust_id) = params.customer_id {
            data_q = data_q.bind(cust_id);
        }
        let rows = data_q.fetch_all(pool).await?;

        let bookings: Vec<BookingListRow> = rows
            .iter()
            .map(|row| BookingListRow {
                book_id: row.try_get::<i32, _>("book_id").unwrap_or(0),
                book_no: row.try_get::<String, _>("book_no").unwrap_or_default(),
                legacy_book_id: row.try_get::<String, _>("legacy_book_id").ok(),
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
        // Runtime `query()` (not `query!`) so we don't have to regenerate the
        // sqlx offline cache every time we add a column to the SELECT.
        let row = sqlx::query(
            r#"SELECT
            b.book_id,
            b.book_no,
            b.legacy_book_id,
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
        )
        .bind(book_id)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| BookingDetailRow {
            book_id: r.try_get("book_id").unwrap_or(0),
            book_no: r.try_get::<String, _>("book_no").unwrap_or_default(),
            legacy_book_id: r.try_get::<String, _>("legacy_book_id").ok(),
            book_cust_id: r.try_get("book_cust_id").unwrap_or(0),
            customer_name: r.try_get("customer_name").ok(),
            book_checkin: r.try_get("book_checkin").expect("book_checkin NOT NULL"),
            book_checkout: r.try_get("book_checkout").expect("book_checkout NOT NULL"),
            book_nights: r.try_get("book_nights").ok(),
            book_adults: r.try_get("book_adults").ok(),
            book_children: r.try_get("book_children").ok(),
            book_status: r.try_get("book_status").ok(),
            book_source: r.try_get("book_source").ok(),
            book_total_amount: r.try_get("book_total_amount").ok(),
            book_deposit_amount: r.try_get("book_deposit_amount").ok(),
            book_notes: r.try_get("book_notes").ok(),
            created_at: r.try_get("created_at").ok(),
            updated_at: r.try_get("updated_at").ok(),
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

    async fn insert_booking_product(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        book_id: i32,
        product: BookingProductAssignment,
    ) -> Result<(), sqlx::Error> {
        // Runtime query (not `query!`): `ht_booking_products` postdates the
        // committed `.sqlx` offline snapshot, so a compile-time-checked macro
        // would fail the offline build. `COALESCE($4, prod_price)` defaults the
        // unit price from the catalog when the form omits it.
        sqlx::query(
            r#"
            INSERT INTO ht_booking_products
                (bp_book_id, bp_product_id, bp_qty, bp_unit_price, bp_note, source, aggregate_id)
            VALUES (
                $1, $2, $3,
                COALESCE($4, (SELECT prod_price FROM ht_products WHERE prod_id = $2)),
                $5, 'canonical', $6
            )
            "#,
        )
        .bind(book_id)
        .bind(product.product_id)
        .bind(product.qty)
        .bind(product.unit_price)
        .bind(product.note)
        .bind(product.aggregate_id)
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

    async fn set_aggregate_id(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        book_id: i32,
        aggregate_id: uuid::Uuid,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE ht_bookings SET aggregate_id = $2 WHERE book_id = $1",
        )
        .bind(book_id)
        .bind(aggregate_id)
        .execute(&mut **tx)
        .await?;
        Ok(())
    }
}
