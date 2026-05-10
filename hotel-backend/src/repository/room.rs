//! Room repository — PostgreSQL data access for `ht_rooms_new`.
//!
//! Mirrors `routes::new_rooms` SQL behavior 1:1.

use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{PgPool, Postgres, Row, Transaction};

use crate::routes::new_rooms::NewRoomsQuery;

/// Result of `list_with_count` and `get` — a room joined with its room type.
#[derive(Debug, Clone)]
pub struct RoomRow {
    pub room_id: i32,
    pub room_no: String,
    pub room_type_id: Option<i32>,
    pub type_name: Option<String>,
    pub room_floor: Option<i32>,
    pub room_status: Option<String>,
    pub room_clean: Option<bool>,
    pub room_maintenance: Option<bool>,
    pub room_price_weekday: Option<f64>,
    pub room_price_weekend: Option<f64>,
    pub room_price_special: Option<f64>,
    pub room_notes: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// Field set for `insert` / `update`.
#[derive(Debug, Clone)]
pub struct RoomWrite<'a> {
    pub room_no: &'a str,
    pub room_type_id: Option<i32>,
    pub floor: Option<i32>,
    pub status: &'a str,
    pub is_clean: bool,
    pub is_maintenance: bool,
    pub price_weekday: Option<f64>,
    pub price_weekend: Option<f64>,
    pub price_special: Option<f64>,
    pub notes: Option<&'a str>,
}

/// PostgreSQL data operations for the room aggregate.
#[async_trait]
pub trait RoomRepository: Send + Sync {
    /// List with pagination + filters; returns `(rows, total_count)`.
    async fn list_with_count(
        &self,
        pool: &PgPool,
        params: &NewRoomsQuery,
    ) -> Result<(Vec<RoomRow>, i32), sqlx::Error>;

    /// Get one room by id (joined with its room type).
    async fn get(
        &self,
        pool: &PgPool,
        room_id: i32,
    ) -> Result<Option<RoomRow>, sqlx::Error>;

    /// Find a room with the given `room_no` (used for duplicate-detection on
    /// create). Returns its id if present.
    async fn find_by_room_no(
        &self,
        pool: &PgPool,
        room_no: &str,
    ) -> Result<Option<i32>, sqlx::Error>;

    /// Find a room with the given `room_no`, ignoring a specific id (used for
    /// duplicate-detection on update). Returns its id if present.
    async fn find_by_room_no_excluding(
        &self,
        pool: &PgPool,
        room_no: &str,
        excluding: i32,
    ) -> Result<Option<i32>, sqlx::Error>;

    /// Insert a new room; returns its assigned `room_id`.
    async fn insert(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        write: RoomWrite<'_>,
    ) -> Result<i32, sqlx::Error>;

    /// Update a room; returns rows affected.
    async fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        room_id: i32,
        write: RoomWrite<'_>,
    ) -> Result<u64, sqlx::Error>;

    /// Update only the room_status; returns rows affected.
    async fn update_status(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        room_id: i32,
        status: &str,
    ) -> Result<u64, sqlx::Error>;
}

/// Default `RoomRepository` impl backed by sqlx + PostgreSQL.
#[derive(Clone, Debug, Default)]
pub struct PgRoomRepository;

impl PgRoomRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl RoomRepository for PgRoomRepository {
    async fn list_with_count(
        &self,
        pool: &PgPool,
        params: &NewRoomsQuery,
    ) -> Result<(Vec<RoomRow>, i32), sqlx::Error> {
        let offset = (params.page - 1) * params.limit;
        let sort_order = params
            .sort_order
            .as_ref()
            .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
            .unwrap_or("ASC");

        let order_by_column = match params.sort_by.as_deref() {
            Some("roomNo") => "r.room_no",
            Some("floor") => "r.room_floor",
            Some("status") => "r.room_status",
            Some("roomType") => "rt.type_name",
            _ => "r.room_no",
        };

        let mut conditions: Vec<String> = Vec::new();

        // Parameterize the status equality filter to remove the SQL string-concat
        // injection vector. The actual value is passed via sqlx `.bind($1)` below.
        // It is the only parameterized condition in this query, so the `$1`
        // placeholder index is fixed.
        if params.status.is_some() {
            conditions.push("r.room_status = $1".to_string());
        }

        if let Some(type_id) = params.room_type_id {
            conditions.push(format!("r.room_type_id = {}", type_id));
        }

        if let Some(floor) = params.floor {
            conditions.push(format!("r.room_floor = {}", floor));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let count_query = format!(
            "SELECT COUNT(*)::int as total FROM ht_rooms_new r {}",
            where_clause
        );

        let count_q = sqlx::query(&count_query);
        let count_q = match &params.status {
            Some(v) => count_q.bind(v),
            None => count_q,
        };
        let count_rows = count_q.fetch_all(pool).await?;

        let total: i32 = count_rows
            .first()
            .map(|r| r.try_get::<i32, _>("total").unwrap_or(0))
            .unwrap_or(0);

        let data_query = format!(
            r#"
        SELECT
            r.room_id,
            r.room_no,
            r.room_type_id,
            rt.type_name,
            r.room_floor,
            r.room_status,
            r.room_clean,
            r.room_maintenance,
            r.room_price_weekday::float8 as room_price_weekday,
            r.room_price_weekend::float8 as room_price_weekend,
            r.room_price_special::float8 as room_price_special,
            r.room_notes,
            r.created_at,
            r.updated_at
        FROM ht_rooms_new r
        LEFT JOIN ht_room_types rt ON r.room_type_id = rt.type_id
        {}
        ORDER BY {} {}
        LIMIT {} OFFSET {}
        "#,
            where_clause, order_by_column, sort_order, params.limit, offset
        );

        let data_q = sqlx::query(&data_query);
        let data_q = match &params.status {
            Some(v) => data_q.bind(v),
            None => data_q,
        };
        let rows = data_q.fetch_all(pool).await?;

        let rooms: Vec<RoomRow> = rows
            .iter()
            .map(|row| RoomRow {
                room_id: row.try_get::<i32, _>("room_id").unwrap_or(0),
                room_no: row.try_get::<String, _>("room_no").unwrap_or_default(),
                room_type_id: row.try_get::<i32, _>("room_type_id").ok(),
                type_name: row.try_get::<String, _>("type_name").ok(),
                room_floor: row.try_get::<i32, _>("room_floor").ok(),
                room_status: row.try_get::<String, _>("room_status").ok(),
                room_clean: row.try_get::<bool, _>("room_clean").ok(),
                room_maintenance: row.try_get::<bool, _>("room_maintenance").ok(),
                room_price_weekday: row.try_get::<f64, _>("room_price_weekday").ok(),
                room_price_weekend: row.try_get::<f64, _>("room_price_weekend").ok(),
                room_price_special: row.try_get::<f64, _>("room_price_special").ok(),
                room_notes: row.try_get::<String, _>("room_notes").ok(),
                created_at: row.try_get::<NaiveDateTime, _>("created_at").ok(),
                updated_at: row.try_get::<NaiveDateTime, _>("updated_at").ok(),
            })
            .collect();

        Ok((rooms, total))
    }

    async fn get(
        &self,
        pool: &PgPool,
        room_id: i32,
    ) -> Result<Option<RoomRow>, sqlx::Error> {
        let rec = sqlx::query!(
            r#"SELECT
            r.room_id, r.room_no, r.room_type_id, rt.type_name as "type_name?",
            r.room_floor, r.room_status, r.room_clean, r.room_maintenance,
            r.room_price_weekday::float8 as "room_price_weekday?",
            r.room_price_weekend::float8 as "room_price_weekend?",
            r.room_price_special::float8 as "room_price_special?",
            r.room_notes, r.created_at, r.updated_at
        FROM ht_rooms_new r
        LEFT JOIN ht_room_types rt ON r.room_type_id = rt.type_id
        WHERE r.room_id = $1"#,
            room_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(rec.map(|r| RoomRow {
            room_id: r.room_id,
            room_no: r.room_no,
            room_type_id: r.room_type_id,
            type_name: r.type_name,
            room_floor: r.room_floor,
            room_status: r.room_status,
            room_clean: r.room_clean,
            room_maintenance: r.room_maintenance,
            room_price_weekday: r.room_price_weekday,
            room_price_weekend: r.room_price_weekend,
            room_price_special: r.room_price_special,
            room_notes: r.room_notes,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    async fn find_by_room_no(
        &self,
        pool: &PgPool,
        room_no: &str,
    ) -> Result<Option<i32>, sqlx::Error> {
        let rec = sqlx::query!(
            "SELECT room_id FROM ht_rooms_new WHERE room_no = $1",
            room_no
        )
        .fetch_optional(pool)
        .await?;
        Ok(rec.map(|r| r.room_id))
    }

    async fn find_by_room_no_excluding(
        &self,
        pool: &PgPool,
        room_no: &str,
        excluding: i32,
    ) -> Result<Option<i32>, sqlx::Error> {
        let rec = sqlx::query!(
            "SELECT room_id FROM ht_rooms_new WHERE room_no = $1 AND room_id != $2",
            room_no,
            excluding
        )
        .fetch_optional(pool)
        .await?;
        Ok(rec.map(|r| r.room_id))
    }

    async fn insert(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        write: RoomWrite<'_>,
    ) -> Result<i32, sqlx::Error> {
        let rec = sqlx::query!(
            r#"INSERT INTO ht_rooms_new (room_no, room_type_id, room_floor, room_status, room_clean, room_maintenance, room_price_weekday, room_price_weekend, room_price_special, room_notes)
        VALUES ($1, $2, $3, $4, $5, $6, $7::float8, $8::float8, $9::float8, $10)
        RETURNING room_id"#,
            write.room_no,
            write.room_type_id,
            write.floor,
            write.status,
            write.is_clean,
            write.is_maintenance,
            write.price_weekday,
            write.price_weekend,
            write.price_special,
            write.notes
        )
        .fetch_one(&mut **tx)
        .await?;
        Ok(rec.room_id)
    }

    async fn update(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        room_id: i32,
        write: RoomWrite<'_>,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"UPDATE ht_rooms_new SET room_no = $1, room_type_id = $2, room_floor = $3, room_status = $4, room_clean = $5, room_maintenance = $6, room_price_weekday = $7::float8, room_price_weekend = $8::float8, room_price_special = $9::float8, room_notes = $10, updated_at = NOW()
        WHERE room_id = $11"#,
            write.room_no,
            write.room_type_id,
            write.floor,
            write.status,
            write.is_clean,
            write.is_maintenance,
            write.price_weekday,
            write.price_weekend,
            write.price_special,
            write.notes,
            room_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(result.rows_affected())
    }

    async fn update_status(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        room_id: i32,
        status: &str,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"UPDATE ht_rooms_new SET room_status = $1, updated_at = NOW() WHERE room_id = $2"#,
            status,
            room_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(result.rows_affected())
    }
}
