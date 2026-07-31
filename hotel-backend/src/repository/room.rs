//! Room repository — PostgreSQL data access for `ht_rooms_new`.
//!
//! Mirrors `routes::new_rooms` SQL behavior 1:1.

use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{postgres::PgRow, PgPool, Postgres, Row, Transaction};

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
    /// Receptionist-arranged board position, mirrored from legacy
    /// `HT_Rooms.Room_X`/`Room_y` by the sync (migration 036). Populated by
    /// the runtime `list_with_count` path only — the compile-time `get`
    /// query leaves them `None` (adding columns there would require a
    /// `.sqlx/` regen; the spatial grid consumes the LIST payload).
    pub room_x: Option<i32>,
    pub room_y: Option<i32>,
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
    ///
    /// The `status` filter/sort here works on the **stored**
    /// `ht_rooms_new.room_status` column. That column is a bypassed
    /// denormalization (issue #200) — callers that need the status a
    /// receptionist actually sees must use [`Self::list_for_derived_status`]
    /// and filter on the derived value in the route layer.
    async fn list_with_count(
        &self,
        pool: &PgPool,
        params: &NewRoomsQuery,
    ) -> Result<(Vec<RoomRow>, i32), sqlx::Error>;

    /// Candidate rows for the derived-status path: the same join/filters as
    /// [`Self::list_with_count`] **minus the stored-`room_status` filter and
    /// minus LIMIT/OFFSET**, so the route layer can overlay the live derived
    /// status (`routes::new_rooms::live_room_flags`) and then filter, sort and
    /// paginate on that derived value.
    ///
    /// Deliberately does NOT re-implement the occupancy derivation in SQL:
    /// duplicating it here is the exact drift class issue #200 comes from.
    /// The room table is ~58 rows per site, so the unbounded scan is free.
    async fn list_for_derived_status(
        &self,
        pool: &PgPool,
        params: &NewRoomsQuery,
    ) -> Result<Vec<RoomRow>, sqlx::Error>;

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

/// SELECT list + join shared by the paginated and the derived-status list
/// queries. Callers append their own `WHERE` / `ORDER BY` / `LIMIT`.
const ROOM_SELECT_SQL: &str = r#"
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
            r.room_x,
            r.room_y,
            r.created_at,
            r.updated_at
        FROM ht_rooms_new r
        LEFT JOIN ht_room_types rt ON r.room_type_id = rt.type_id
"#;

/// WHERE fragments that are NOT the status filter. Both values are `i32`
/// parsed by serde, so inlining them carries no injection vector (unlike
/// `status`, which is bound — see `list_with_count`).
fn non_status_conditions(params: &NewRoomsQuery) -> Vec<String> {
    let mut conditions: Vec<String> = Vec::new();

    if let Some(type_id) = params.room_type_id {
        conditions.push(format!("r.room_type_id = {}", type_id));
    }

    if let Some(floor) = params.floor {
        conditions.push(format!("r.room_floor = {}", floor));
    }

    conditions
}

fn where_clause(conditions: &[String]) -> String {
    if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    }
}

fn sort_direction(params: &NewRoomsQuery) -> &'static str {
    params
        .sort_order
        .as_ref()
        .map(|s| if s.to_lowercase() == "desc" { "DESC" } else { "ASC" })
        .unwrap_or("ASC")
}

/// SQL sort column. `status` maps to the STORED column, which is only
/// meaningful for [`RoomRepository::list_with_count`]; the derived-status path
/// re-sorts in the route layer and asks for `room_no` here instead.
fn sort_column(params: &NewRoomsQuery) -> &'static str {
    match params.sort_by.as_deref() {
        Some("roomNo") => "r.room_no",
        Some("floor") => "r.room_floor",
        Some("status") => "r.room_status",
        Some("roomType") => "rt.type_name",
        _ => "r.room_no",
    }
}

/// Build the `(count, data)` SQL for the paginated list. The `$1` status
/// placeholder — present only when `params.status.is_some()` — is bound by the
/// caller.
fn list_with_count_sql(params: &NewRoomsQuery) -> (String, String) {
    // Clamped via `NewRoomsQuery` so this path and the route's in-memory
    // `filter_sort_paginate` agree bit-for-bit. The raw values used to give
    // `LIMIT 50 OFFSET -50` for `?page=0` (a PG error) where the derived path
    // silently served page 1.
    let limit = params.clamped_limit();
    let offset = params.clamped_offset();

    let mut conditions: Vec<String> = Vec::new();

    // Parameterize the status equality filter to remove the SQL string-concat
    // injection vector. The actual value is passed via sqlx `.bind($1)` by the
    // caller. It is the only parameterized condition in this query, so the `$1`
    // placeholder index is fixed — keep it pushed FIRST.
    if params.status.is_some() {
        conditions.push("r.room_status = $1".to_string());
    }

    conditions.extend(non_status_conditions(params));

    let where_clause = where_clause(&conditions);

    let count_sql = format!(
        "SELECT COUNT(*)::int as total FROM ht_rooms_new r {}",
        where_clause
    );

    let data_sql = format!(
        "{}{}\n        ORDER BY {} {}\n        LIMIT {} OFFSET {}\n        ",
        ROOM_SELECT_SQL,
        where_clause,
        sort_column(params),
        sort_direction(params),
        limit,
        offset
    );

    (count_sql, data_sql)
}

/// Build the candidate-scan SQL for the derived-status path: non-status
/// filters only, no LIMIT/OFFSET, no bound parameters.
fn derived_status_list_sql(params: &NewRoomsQuery) -> String {
    let where_clause = where_clause(&non_status_conditions(params));

    // `sortBy=status` cannot be honoured here (the derived value isn't a
    // column); fall back to `room_no` so the pre-sort order is stable and let
    // the route re-sort. Every other sort is honoured in SQL and survives the
    // in-memory filter, which preserves order.
    let status_sort = params.sort_by.as_deref() == Some("status");
    let order_by_column = if status_sort {
        "r.room_no"
    } else {
        sort_column(params)
    };
    let sort_order = if status_sort {
        "ASC"
    } else {
        sort_direction(params)
    };

    format!(
        "{}{}\n        ORDER BY {} {}\n        ",
        ROOM_SELECT_SQL, where_clause, order_by_column, sort_order
    )
}

fn map_room_rows(rows: &[PgRow]) -> Vec<RoomRow> {
    rows.iter()
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
            room_x: row.try_get::<i32, _>("room_x").ok(),
            room_y: row.try_get::<i32, _>("room_y").ok(),
            created_at: row.try_get::<NaiveDateTime, _>("created_at").ok(),
            updated_at: row.try_get::<NaiveDateTime, _>("updated_at").ok(),
        })
        .collect()
}

#[async_trait]
impl RoomRepository for PgRoomRepository {
    async fn list_with_count(
        &self,
        pool: &PgPool,
        params: &NewRoomsQuery,
    ) -> Result<(Vec<RoomRow>, i32), sqlx::Error> {
        let (count_query, data_query) = list_with_count_sql(params);

        let count_q = sqlx::query(sqlx::AssertSqlSafe(&*count_query));
        let count_q = match &params.status {
            Some(v) => count_q.bind(v),
            None => count_q,
        };
        let count_rows = count_q.fetch_all(pool).await?;

        let total: i32 = count_rows
            .first()
            .map(|r| r.try_get::<i32, _>("total").unwrap_or(0))
            .unwrap_or(0);

        let data_q = sqlx::query(sqlx::AssertSqlSafe(&*data_query));
        let data_q = match &params.status {
            Some(v) => data_q.bind(v),
            None => data_q,
        };
        let rows = data_q.fetch_all(pool).await?;

        Ok((map_room_rows(&rows), total))
    }

    async fn list_for_derived_status(
        &self,
        pool: &PgPool,
        params: &NewRoomsQuery,
    ) -> Result<Vec<RoomRow>, sqlx::Error> {
        // No `r.room_status = $1` condition and no LIMIT/OFFSET — the route
        // layer owns both once the live status has been overlaid. The
        // non-status filters stay in SQL exactly as on the paginated path.
        let data_query = derived_status_list_sql(params);

        let rows = sqlx::query(sqlx::AssertSqlSafe(&*data_query))
            .fetch_all(pool)
            .await?;

        Ok(map_room_rows(&rows))
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
            // Board coordinates ride the runtime LIST path only (see the
            // RoomRow field comment) — not selected by this query! macro.
            room_x: None,
            room_y: None,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn params() -> NewRoomsQuery {
        NewRoomsQuery {
            status: None,
            room_type_id: None,
            floor: None,
            page: 1,
            limit: 50,
            sort_by: None,
            sort_order: None,
            branch: None,
        }
    }

    /// The half of the paginated path production still reaches: SQL
    /// LIMIT/OFFSET on the requested sort column, with no status filter.
    /// `routes::new_rooms` takes this path for every request that does NOT
    /// filter or sort by status (dashboard, housekeeping, v2 grid).
    #[test]
    fn paginated_sql_keeps_sql_pagination_and_sort() {
        let mut p = params();
        p.page = 3;
        p.limit = 20;
        p.sort_by = Some("floor".to_string());
        p.sort_order = Some("desc".to_string());
        let (count_sql, data_sql) = list_with_count_sql(&p);

        assert!(!count_sql.contains("WHERE"), "no filters requested: {count_sql}");
        assert!(data_sql.contains("ORDER BY r.room_floor DESC"), "{data_sql}");
        assert!(data_sql.contains("LIMIT 20 OFFSET 40"), "{data_sql}");
    }

    /// **Unreachable from the route, retained deliberately.**
    /// `routes::new_rooms::needs_derived_status_pass` diverts every
    /// `status.is_some()` request to [`RoomRepository::list_for_derived_status`],
    /// so production can no longer produce a `list_with_count` call carrying a
    /// status filter — asserting the stored-column filter "still works" would be
    /// false assurance about live behavior. The branch stays for direct callers
    /// of the public trait, and this test guards the one property that would
    /// still matter to them: the value is BOUND as `$1`, never concatenated into
    /// the SQL. It pins the injection fix, not the (bypassed) filter semantics.
    #[test]
    fn paginated_sql_binds_status_rather_than_interpolating_it() {
        let mut p = params();
        p.status = Some("available'; DROP TABLE ht_rooms_new; --".to_string());
        let (count_sql, data_sql) = list_with_count_sql(&p);

        assert!(count_sql.contains("WHERE r.room_status = $1"), "{count_sql}");
        assert!(data_sql.contains("WHERE r.room_status = $1"), "{data_sql}");
        assert!(!data_sql.contains("DROP TABLE"), "value must be bound: {data_sql}");
    }

    /// Both list paths clamp identically (review finding 3). `?page=0` used to
    /// build `OFFSET -20` here — a hard PG error — while the derived path
    /// silently served page 1; a negative `?limit=` built `LIMIT -20`.
    #[test]
    fn paginated_sql_clamps_out_of_range_pagination() {
        let mut p = params();
        p.page = 0;
        p.limit = 20;
        let (_, data_sql) = list_with_count_sql(&p);
        assert!(data_sql.contains("LIMIT 20 OFFSET 0"), "{data_sql}");

        let mut p = params();
        p.page = -3;
        p.limit = -20;
        let (_, data_sql) = list_with_count_sql(&p);
        assert!(data_sql.contains("LIMIT 0 OFFSET 0"), "{data_sql}");
    }

    /// The derived-status scan drops BOTH the stored-status filter and the
    /// pagination — the route layer owns them once the live status is overlaid.
    #[test]
    fn derived_sql_has_no_status_filter_and_no_pagination() {
        let mut p = params();
        p.status = Some("available".to_string());
        p.limit = 20;
        let sql = derived_status_list_sql(&p);

        // The column is still SELECTed (the route needs it for the stored
        // `maintenance` flag) — what must be gone is the FILTER on it.
        assert!(!sql.contains("r.room_status ="), "{sql}");
        assert!(!sql.contains("WHERE"), "{sql}");
        assert!(!sql.contains("$1"), "no bound parameters remain: {sql}");
        assert!(!sql.contains("LIMIT"), "{sql}");
        assert!(!sql.contains("OFFSET"), "{sql}");
        assert!(sql.contains("FROM ht_rooms_new r"), "{sql}");
        assert!(sql.contains("ORDER BY r.room_no ASC"), "{sql}");
    }

    /// Non-status filters and non-status sorts still ride SQL on both paths.
    #[test]
    fn derived_sql_keeps_non_status_filters_and_sorts() {
        let mut p = params();
        p.status = Some("available".to_string());
        p.room_type_id = Some(7);
        p.floor = Some(2);
        p.sort_by = Some("floor".to_string());
        p.sort_order = Some("desc".to_string());
        let sql = derived_status_list_sql(&p);

        assert!(sql.contains("WHERE r.room_type_id = 7 AND r.room_floor = 2"), "{sql}");
        assert!(sql.contains("ORDER BY r.room_floor DESC"), "{sql}");
    }

    /// `sortBy=status` cannot be honoured in SQL (the derived value is not a
    /// column) — the scan falls back to a stable `room_no` order and the route
    /// re-sorts on the derived value.
    #[test]
    fn derived_sql_falls_back_to_room_no_for_status_sort() {
        let mut p = params();
        p.sort_by = Some("status".to_string());
        p.sort_order = Some("desc".to_string());
        let sql = derived_status_list_sql(&p);

        assert!(sql.contains("ORDER BY r.room_no ASC"), "{sql}");
        assert!(!sql.contains("ORDER BY r.room_status"), "{sql}");
    }
}
