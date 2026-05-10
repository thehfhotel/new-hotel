//! Inventory repository — PostgreSQL data access for `ht_inventory_*` and
//! `ht_room_inventory`.
//!
//! Mirrors `routes::new_inventory` SQL behavior 1:1.
//!
//! NOTE: inventory is not (yet) modeled in the `domain/` layer. The repo
//! returns the raw row shapes the route needs and the route maps them into
//! its existing wire DTOs. When the inventory aggregate gets a domain model
//! (post-Phase 2) these row types stay; only the DTOs evolve.

use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{PgPool, Postgres, Row, Transaction};

use crate::routes::new_inventory::{InventoryRoomsQuery, ItemsQuery, TransactionsQuery};

/// One row in `ht_inventory_categories`.
#[derive(Debug, Clone)]
pub struct CategoryRow {
    pub cat_id: i32,
    pub cat_name: String,
    pub cat_description: Option<String>,
    pub cat_active: Option<bool>,
    pub cat_created: Option<NaiveDateTime>,
}

/// Field set for `insert_category`.
#[derive(Debug, Clone)]
pub struct CategoryInsert<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub active: bool,
}

/// One row in `ht_inventory_items` joined with its category.
#[derive(Debug, Clone)]
pub struct ItemRow {
    pub item_id: i32,
    pub item_code: String,
    pub item_name: String,
    pub item_category_id: Option<i32>,
    pub cat_name: Option<String>,
    pub item_unit: String,
    pub item_min_stock: Option<i32>,
    pub item_current_stock: Option<i32>,
    pub item_cost: Option<f64>,
    pub item_active: Option<bool>,
    pub item_created: Option<NaiveDateTime>,
    pub item_updated: Option<NaiveDateTime>,
}

/// Field set for `insert_item` / `update_item`.
#[derive(Debug, Clone)]
pub struct ItemWrite<'a> {
    pub code: &'a str,
    pub name: &'a str,
    pub category_id: Option<i32>,
    pub unit: &'a str,
    pub min_stock: i32,
    pub current_stock: i32,
    pub cost: Option<f64>,
    pub active: bool,
}

/// Lightweight room row used by the inventory checklist screen.
#[derive(Debug, Clone)]
pub struct RoomLite {
    pub room_no: String,
    pub type_name: String,
}

/// One row in the room inventory checklist.
#[derive(Debug, Clone)]
pub struct RoomChecklistItemRow {
    pub item_id: i32,
    pub item_code: String,
    pub item_name: String,
    pub category: String,
    pub item_unit: String,
    pub assigned_quantity: i32,
    pub last_checked: Option<NaiveDateTime>,
}

/// Roll-up row used by the inventory rooms list page.
#[derive(Debug, Clone)]
pub struct InventoryRoomRollupRow {
    pub room_id: i32,
    pub room_no: String,
    pub room_type: String,
    pub room_status: String,
    pub inventory_count: i32,
    pub last_checked: Option<NaiveDateTime>,
    pub missing_items: i32,
}

/// One row in `ht_inventory_transactions` joined with the item.
#[derive(Debug, Clone)]
pub struct TransactionRow {
    pub trans_id: i32,
    pub trans_item_id: Option<i32>,
    pub item_code: Option<String>,
    pub item_name: Option<String>,
    pub trans_type: String,
    pub trans_quantity: i32,
    pub trans_room_id: Option<i32>,
    pub trans_notes: Option<String>,
    pub trans_date: Option<NaiveDateTime>,
    pub trans_by: Option<String>,
}

/// Aggregated stats for the dashboard.
#[derive(Debug, Clone)]
pub struct InventoryStatsRow {
    pub total_items: Option<i32>,
    pub total_categories: Option<i32>,
    pub low_stock_count: Option<i32>,
    pub out_of_stock_count: Option<i32>,
    pub total_stock_value: Option<f64>,
}

/// PostgreSQL data operations for the inventory aggregate.
#[async_trait]
pub trait InventoryRepository: Send + Sync {
    // ----- Categories -----
    async fn list_categories(
        &self,
        pool: &PgPool,
    ) -> Result<Vec<CategoryRow>, sqlx::Error>;

    async fn insert_category(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        insert: CategoryInsert<'_>,
    ) -> Result<i32, sqlx::Error>;

    // ----- Items -----
    async fn list_items_with_count(
        &self,
        pool: &PgPool,
        params: &ItemsQuery,
    ) -> Result<(Vec<ItemRow>, i32), sqlx::Error>;

    async fn find_item_id_by_code(
        &self,
        pool: &PgPool,
        code: &str,
    ) -> Result<Option<i32>, sqlx::Error>;

    async fn find_item_id_by_code_excluding(
        &self,
        pool: &PgPool,
        code: &str,
        excluding: i32,
    ) -> Result<Option<i32>, sqlx::Error>;

    async fn insert_item(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        write: ItemWrite<'_>,
    ) -> Result<i32, sqlx::Error>;

    async fn get_item(
        &self,
        pool: &PgPool,
        item_id: i32,
    ) -> Result<Option<ItemRow>, sqlx::Error>;

    async fn update_item(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        item_id: i32,
        write: ItemWrite<'_>,
    ) -> Result<u64, sqlx::Error>;

    async fn deactivate_item(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        item_id: i32,
    ) -> Result<u64, sqlx::Error>;

    // ----- Stock helpers -----
    /// Look up `item_current_stock` for a single item.
    ///
    /// Returns `Ok(None)` when the row is missing, `Ok(Some(None))` when the
    /// row exists but the column is NULL, and `Ok(Some(Some(stock)))` for a
    /// valid value. Mirrors the route's existing `fetch_optional` semantics
    /// without forcing one of the two distinct error conventions on callers.
    async fn get_item_current_stock(
        &self,
        pool: &PgPool,
        item_id: i32,
    ) -> Result<Option<Option<i32>>, sqlx::Error>;

    async fn set_item_current_stock(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        item_id: i32,
        new_stock: i32,
    ) -> Result<(), sqlx::Error>;

    // ----- Room inventory -----
    async fn get_room_lite(
        &self,
        pool: &PgPool,
        room_id: i32,
    ) -> Result<Option<RoomLite>, sqlx::Error>;

    async fn list_room_checklist(
        &self,
        pool: &PgPool,
        room_id: i32,
    ) -> Result<Vec<RoomChecklistItemRow>, sqlx::Error>;

    async fn list_inventory_rooms(
        &self,
        pool: &PgPool,
        params: &InventoryRoomsQuery,
    ) -> Result<Vec<InventoryRoomRollupRow>, sqlx::Error>;

    async fn touch_room_item_last_checked(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        room_id: i32,
        item_id: i32,
    ) -> Result<(), sqlx::Error>;

    /// Best-effort log entry. Mirrors the route's `.ok()` swallowing of errors
    /// — returns `Result<(), sqlx::Error>` so the caller can ignore it the
    /// same way.
    async fn log_room_check(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        room_id: i32,
        notes: &str,
    ) -> Result<(), sqlx::Error>;

    async fn log_replenish_out(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        item_id: i32,
        quantity: i32,
        room_id: i32,
        notes: &str,
    ) -> Result<(), sqlx::Error>;

    async fn delete_room_inventory(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        room_id: i32,
    ) -> Result<(), sqlx::Error>;

    async fn insert_room_inventory(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        room_id: i32,
        item_id: i32,
        quantity: i32,
    ) -> Result<(), sqlx::Error>;

    // ----- Transactions -----
    async fn list_transactions_with_count(
        &self,
        pool: &PgPool,
        params: &TransactionsQuery,
    ) -> Result<(Vec<TransactionRow>, i32), sqlx::Error>;

    async fn insert_transaction(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        item_id: i32,
        trans_type: &str,
        quantity: i32,
        room_id: Option<i32>,
        notes: Option<&str>,
        created_by: Option<&str>,
    ) -> Result<i32, sqlx::Error>;

    /// Variant used by `create_stock_adjustment`: matches the route's existing
    /// SQL which always passes `room_id = NULL`.
    async fn insert_stock_adjustment_transaction(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        item_id: i32,
        trans_type: &str,
        quantity: i32,
        notes: Option<&str>,
        created_by: Option<&str>,
    ) -> Result<i32, sqlx::Error>;

    // ----- Dashboard -----
    async fn get_stats(&self, pool: &PgPool) -> Result<InventoryStatsRow, sqlx::Error>;

    async fn list_low_stock(&self, pool: &PgPool) -> Result<Vec<ItemRow>, sqlx::Error>;
}

/// Default `InventoryRepository` impl backed by sqlx + PostgreSQL.
#[derive(Clone, Debug, Default)]
pub struct PgInventoryRepository;

impl PgInventoryRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl InventoryRepository for PgInventoryRepository {
    async fn list_categories(
        &self,
        pool: &PgPool,
    ) -> Result<Vec<CategoryRow>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"SELECT cat_id, cat_name, cat_description, cat_active, cat_created
        FROM ht_inventory_categories ORDER BY cat_name ASC"#
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| CategoryRow {
                cat_id: r.cat_id,
                cat_name: r.cat_name,
                cat_description: r.cat_description,
                cat_active: r.cat_active,
                cat_created: r.cat_created,
            })
            .collect())
    }

    async fn insert_category(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        insert: CategoryInsert<'_>,
    ) -> Result<i32, sqlx::Error> {
        let rec = sqlx::query!(
            r#"INSERT INTO ht_inventory_categories (cat_name, cat_description, cat_active)
        VALUES ($1, $2, $3) RETURNING cat_id"#,
            insert.name,
            insert.description,
            insert.active
        )
        .fetch_one(&mut **tx)
        .await?;
        Ok(rec.cat_id)
    }

    async fn list_items_with_count(
        &self,
        pool: &PgPool,
        params: &ItemsQuery,
    ) -> Result<(Vec<ItemRow>, i32), sqlx::Error> {
        let offset = (params.page - 1) * params.limit;

        // Bind values are pushed in the same order conditions reference $N.
        // Both count_query and data_query bind in this exact order below.
        let mut conditions: Vec<String> = Vec::new();
        let mut bind_index: i32 = 0;

        if let Some(cat_id) = params.category_id {
            // i32 is safe to inline (no string content), kept inline to keep
            // the `.bind()` chain focused on user-supplied strings.
            conditions.push(format!("i.item_category_id = {}", cat_id));
        }

        if params.low_stock == Some(true) {
            conditions.push("i.item_current_stock <= i.item_min_stock".to_string());
        }

        // Build a parameterized LIKE pattern. Escape the LIKE-special
        // characters (`\`, `%`, `_`) so user input cannot inject wildcards or
        // break out of the pattern. The actual value is passed via sqlx
        // `.bind()` below, which prevents SQL injection at the protocol level.
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
            bind_index += 1;
            let n = bind_index;
            conditions.push(format!(
                "(i.item_code LIKE ${n} ESCAPE '\\' OR i.item_name LIKE ${n} ESCAPE '\\')"
            ));
        }

        if let Some(active) = params.active {
            // Boolean literal — safe to inline.
            conditions.push(format!(
                "i.item_active = {}",
                if active { "true" } else { "false" }
            ));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let count_query = format!(
            "SELECT COUNT(*)::int as total FROM ht_inventory_items i {}",
            where_clause
        );

        let count_q = sqlx::query(&count_query);
        let count_q = match &like_pattern {
            Some(v) => count_q.bind(v),
            None => count_q,
        };
        let count_rows = count_q.fetch_all(pool).await?;

        let total: i32 = count_rows
            .first()
            .and_then(|r| r.try_get::<i32, _>("total").ok())
            .unwrap_or(0);

        let data_query = format!(
            r#"
        SELECT
            i.item_id,
            i.item_code,
            i.item_name,
            i.item_category_id,
            c.cat_name,
            i.item_unit,
            i.item_min_stock,
            i.item_current_stock,
            i.item_cost::float8 as item_cost,
            i.item_active,
            i.item_created,
            i.item_updated
        FROM ht_inventory_items i
        LEFT JOIN ht_inventory_categories c ON i.item_category_id = c.cat_id
        {}
        ORDER BY i.item_name ASC
        LIMIT {} OFFSET {}
        "#,
            where_clause, params.limit, offset
        );

        let data_q = sqlx::query(&data_query);
        let data_q = match &like_pattern {
            Some(v) => data_q.bind(v),
            None => data_q,
        };
        let rows = data_q.fetch_all(pool).await?;

        let items: Vec<ItemRow> = rows
            .iter()
            .map(|row| ItemRow {
                item_id: row.try_get::<i32, _>("item_id").unwrap_or(0),
                item_code: row.try_get::<String, _>("item_code").unwrap_or_default(),
                item_name: row.try_get::<String, _>("item_name").unwrap_or_default(),
                item_category_id: row.try_get::<i32, _>("item_category_id").ok(),
                cat_name: row.try_get::<String, _>("cat_name").ok(),
                item_unit: row.try_get::<String, _>("item_unit").unwrap_or_default(),
                item_min_stock: row.try_get::<i32, _>("item_min_stock").ok(),
                item_current_stock: row.try_get::<i32, _>("item_current_stock").ok(),
                item_cost: row.try_get::<f64, _>("item_cost").ok(),
                item_active: row.try_get::<bool, _>("item_active").ok(),
                item_created: row.try_get::<NaiveDateTime, _>("item_created").ok(),
                item_updated: row.try_get::<NaiveDateTime, _>("item_updated").ok(),
            })
            .collect();

        Ok((items, total))
    }

    async fn find_item_id_by_code(
        &self,
        pool: &PgPool,
        code: &str,
    ) -> Result<Option<i32>, sqlx::Error> {
        let rec = sqlx::query!(
            "SELECT item_id FROM ht_inventory_items WHERE item_code = $1",
            code
        )
        .fetch_optional(pool)
        .await?;
        Ok(rec.map(|r| r.item_id))
    }

    async fn find_item_id_by_code_excluding(
        &self,
        pool: &PgPool,
        code: &str,
        excluding: i32,
    ) -> Result<Option<i32>, sqlx::Error> {
        let rec = sqlx::query!(
            "SELECT item_id FROM ht_inventory_items WHERE item_code = $1 AND item_id != $2",
            code,
            excluding
        )
        .fetch_optional(pool)
        .await?;
        Ok(rec.map(|r| r.item_id))
    }

    async fn insert_item(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        write: ItemWrite<'_>,
    ) -> Result<i32, sqlx::Error> {
        let rec = sqlx::query!(
            r#"INSERT INTO ht_inventory_items (item_code, item_name, item_category_id, item_unit, item_min_stock, item_current_stock, item_cost, item_active)
        VALUES ($1, $2, $3, $4, $5, $6, $7::float8, $8) RETURNING item_id"#,
            write.code,
            write.name,
            write.category_id,
            write.unit,
            write.min_stock,
            write.current_stock,
            write.cost,
            write.active
        )
        .fetch_one(&mut **tx)
        .await?;
        Ok(rec.item_id)
    }

    async fn get_item(
        &self,
        pool: &PgPool,
        item_id: i32,
    ) -> Result<Option<ItemRow>, sqlx::Error> {
        let rec = sqlx::query!(
            r#"SELECT i.item_id, i.item_code, i.item_name, i.item_category_id, c.cat_name,
            i.item_unit, i.item_min_stock, i.item_current_stock,
            i.item_cost::float8 as item_cost, i.item_active, i.item_created, i.item_updated
        FROM ht_inventory_items i
        LEFT JOIN ht_inventory_categories c ON i.item_category_id = c.cat_id
        WHERE i.item_id = $1"#,
            item_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(rec.map(|r| ItemRow {
            item_id: r.item_id,
            item_code: r.item_code,
            item_name: r.item_name,
            item_category_id: r.item_category_id,
            cat_name: Some(r.cat_name),
            item_unit: r.item_unit,
            item_min_stock: r.item_min_stock,
            item_current_stock: r.item_current_stock,
            item_cost: r.item_cost,
            item_active: r.item_active,
            item_created: r.item_created,
            item_updated: r.item_updated,
        }))
    }

    async fn update_item(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        item_id: i32,
        write: ItemWrite<'_>,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"UPDATE ht_inventory_items SET item_code = $1, item_name = $2, item_category_id = $3, item_unit = $4,
        item_min_stock = $5, item_current_stock = $6, item_cost = $7::float8, item_active = $8, item_updated = NOW()
        WHERE item_id = $9"#,
            write.code,
            write.name,
            write.category_id,
            write.unit,
            write.min_stock,
            write.current_stock,
            write.cost,
            write.active,
            item_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(result.rows_affected())
    }

    async fn deactivate_item(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        item_id: i32,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE ht_inventory_items SET item_active = false, item_updated = NOW() WHERE item_id = $1",
            item_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(result.rows_affected())
    }

    async fn get_item_current_stock(
        &self,
        pool: &PgPool,
        item_id: i32,
    ) -> Result<Option<Option<i32>>, sqlx::Error> {
        let rec = sqlx::query!(
            "SELECT item_current_stock FROM ht_inventory_items WHERE item_id = $1",
            item_id
        )
        .fetch_optional(pool)
        .await?;
        Ok(rec.map(|r| r.item_current_stock))
    }

    async fn set_item_current_stock(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        item_id: i32,
        new_stock: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE ht_inventory_items SET item_current_stock = $1, item_updated = NOW() WHERE item_id = $2",
            new_stock,
            item_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn get_room_lite(
        &self,
        pool: &PgPool,
        room_id: i32,
    ) -> Result<Option<RoomLite>, sqlx::Error> {
        let rec = sqlx::query!(
            r#"SELECT r.room_no, COALESCE(rt.type_name, '') as "type_name!"
        FROM ht_rooms_new r
        LEFT JOIN ht_room_types rt ON r.room_type_id = rt.type_id
        WHERE r.room_id = $1"#,
            room_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(rec.map(|r| RoomLite {
            room_no: r.room_no,
            type_name: r.type_name,
        }))
    }

    async fn list_room_checklist(
        &self,
        pool: &PgPool,
        room_id: i32,
    ) -> Result<Vec<RoomChecklistItemRow>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"SELECT
            ri.ri_item_id,
            COALESCE(i.item_code, '') as "item_code!",
            COALESCE(i.item_name, '') as "item_name!",
            COALESCE(c.cat_name, 'Equipment') as "category!",
            COALESCE(i.item_unit, '') as "item_unit!",
            ri.ri_quantity,
            ri.ri_last_checked
        FROM ht_room_inventory ri
        LEFT JOIN ht_inventory_items i ON ri.ri_item_id = i.item_id
        LEFT JOIN ht_inventory_categories c ON i.item_category_id = c.cat_id
        WHERE ri.ri_room_id = $1
        ORDER BY i.item_name ASC"#,
            room_id
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| RoomChecklistItemRow {
                item_id: r.ri_item_id.unwrap_or(0),
                item_code: r.item_code,
                item_name: r.item_name,
                category: r.category,
                item_unit: r.item_unit,
                assigned_quantity: r.ri_quantity.unwrap_or(0),
                last_checked: r.ri_last_checked,
            })
            .collect())
    }

    async fn list_inventory_rooms(
        &self,
        pool: &PgPool,
        params: &InventoryRoomsQuery,
    ) -> Result<Vec<InventoryRoomRollupRow>, sqlx::Error> {
        let mut conditions: Vec<String> = vec!["r.room_active = true".to_string()];

        // Build a parameterized LIKE pattern. Escape the LIKE-special
        // characters (`\`, `%`, `_`) so user input cannot inject wildcards or
        // break out of the pattern. The actual value is bound via sqlx
        // `.bind()` below.
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
            conditions.push("r.room_no LIKE $1 ESCAPE '\\'".to_string());
        }

        let where_clause = format!("WHERE {}", conditions.join(" AND "));

        let data_query = format!(
            r#"
        SELECT
            r.room_id,
            r.room_no,
            COALESCE(rt.type_name, '') AS room_type,
            COALESCE(r.room_status, 'available') AS room_status,
            COALESCE((SELECT COUNT(*)::int FROM ht_room_inventory ri WHERE ri.ri_room_id = r.room_id), 0) AS inventory_count,
            (SELECT MAX(ri.ri_last_checked) FROM ht_room_inventory ri WHERE ri.ri_room_id = r.room_id) AS last_checked,
            0::int AS missing_items
        FROM ht_rooms_new r
        LEFT JOIN ht_room_types rt ON r.room_type_id = rt.type_id
        {}
        ORDER BY r.room_no ASC
        "#,
            where_clause
        );

        let data_q = sqlx::query(&data_query);
        let data_q = match &like_pattern {
            Some(v) => data_q.bind(v),
            None => data_q,
        };
        let rows = data_q.fetch_all(pool).await?;

        Ok(rows
            .iter()
            .map(|row| InventoryRoomRollupRow {
                room_id: row.try_get::<i32, _>("room_id").unwrap_or(0),
                room_no: row.try_get::<String, _>("room_no").unwrap_or_default(),
                room_type: row.try_get::<String, _>("room_type").unwrap_or_default(),
                room_status: row
                    .try_get::<String, _>("room_status")
                    .unwrap_or_else(|_| "available".to_string()),
                inventory_count: row.try_get::<i32, _>("inventory_count").unwrap_or(0),
                last_checked: row.try_get::<NaiveDateTime, _>("last_checked").ok(),
                missing_items: row.try_get::<i32, _>("missing_items").unwrap_or(0),
            })
            .collect())
    }

    async fn touch_room_item_last_checked(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        room_id: i32,
        item_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"UPDATE ht_room_inventory SET ri_last_checked = NOW()
            WHERE ri_room_id = $1 AND ri_item_id = $2"#,
            room_id,
            item_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn log_room_check(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        room_id: i32,
        notes: &str,
    ) -> Result<(), sqlx::Error> {
        // SQL text + indentation match the original route verbatim so the
        // existing `.sqlx/` cache key stays valid (sqlx hashes the literal).
        sqlx::query!(
            r#"INSERT INTO ht_inventory_transactions (trans_item_id, trans_type, trans_quantity, trans_room_id, trans_notes)
        VALUES (NULL, 'CHECK', 0, $1, $2)"#,
            room_id,
            notes
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn log_replenish_out(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        item_id: i32,
        quantity: i32,
        room_id: i32,
        notes: &str,
    ) -> Result<(), sqlx::Error> {
        // SQL text + indentation match the original route verbatim so the
        // existing `.sqlx/` cache key stays valid.
        sqlx::query!(
            r#"INSERT INTO ht_inventory_transactions (trans_item_id, trans_type, trans_quantity, trans_room_id, trans_notes)
            VALUES ($1, 'OUT', $2, $3, $4)"#,
            item_id,
            quantity,
            room_id,
            notes
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn delete_room_inventory(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        room_id: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM ht_room_inventory WHERE ri_room_id = $1",
            room_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn insert_room_inventory(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        room_id: i32,
        item_id: i32,
        quantity: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO ht_room_inventory (ri_room_id, ri_item_id, ri_quantity, ri_last_checked) VALUES ($1, $2, $3, NOW())"#,
            room_id,
            item_id,
            quantity
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    async fn list_transactions_with_count(
        &self,
        pool: &PgPool,
        params: &TransactionsQuery,
    ) -> Result<(Vec<TransactionRow>, i32), sqlx::Error> {
        let offset = (params.page - 1) * params.limit;

        // Bind values are pushed in the same order conditions reference $N.
        // Both count_query and data_query bind these in the exact same order.
        let mut conditions: Vec<String> = Vec::new();
        let mut bind_index: i32 = 0;

        if let Some(item_id) = params.item_id {
            // i32 — safe to inline.
            conditions.push(format!("t.trans_item_id = {}", item_id));
        }

        // Equality on trans_type (route preserves the original .to_uppercase()
        // semantics by normalizing the bind value, not the SQL).
        let trans_type_bind: Option<String> = params
            .trans_type
            .as_ref()
            .map(|t| t.to_uppercase());
        if trans_type_bind.is_some() {
            bind_index += 1;
            conditions.push(format!("t.trans_type = ${}", bind_index));
        }

        // Date range filters bind the raw string and cast to ::date in SQL,
        // matching the original SQL semantics exactly (no parsing in Rust).
        let from_bind: Option<String> = params.from.clone();
        if from_bind.is_some() {
            bind_index += 1;
            conditions.push(format!("t.trans_date >= ${}::date", bind_index));
        }

        let to_bind: Option<String> = params.to.clone();
        if to_bind.is_some() {
            bind_index += 1;
            conditions.push(format!("t.trans_date <= ${}::date", bind_index));
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let count_query = format!(
            "SELECT COUNT(*)::int as total FROM ht_inventory_transactions t {}",
            where_clause
        );

        let count_q = sqlx::query(&count_query);
        let count_q = match &trans_type_bind {
            Some(v) => count_q.bind(v),
            None => count_q,
        };
        let count_q = match &from_bind {
            Some(v) => count_q.bind(v),
            None => count_q,
        };
        let count_q = match &to_bind {
            Some(v) => count_q.bind(v),
            None => count_q,
        };
        let count_rows = count_q.fetch_all(pool).await?;

        let total: i32 = count_rows
            .first()
            .and_then(|r| r.try_get::<i32, _>("total").ok())
            .unwrap_or(0);

        let data_query = format!(
            r#"
        SELECT
            t.trans_id,
            t.trans_item_id,
            i.item_code,
            i.item_name,
            t.trans_type,
            t.trans_quantity,
            t.trans_room_id,
            t.trans_notes,
            t.trans_date,
            t.trans_by
        FROM ht_inventory_transactions t
        LEFT JOIN ht_inventory_items i ON t.trans_item_id = i.item_id
        {}
        ORDER BY t.trans_date DESC
        LIMIT {} OFFSET {}
        "#,
            where_clause, params.limit, offset
        );

        let data_q = sqlx::query(&data_query);
        let data_q = match &trans_type_bind {
            Some(v) => data_q.bind(v),
            None => data_q,
        };
        let data_q = match &from_bind {
            Some(v) => data_q.bind(v),
            None => data_q,
        };
        let data_q = match &to_bind {
            Some(v) => data_q.bind(v),
            None => data_q,
        };
        let rows = data_q.fetch_all(pool).await?;

        let transactions: Vec<TransactionRow> = rows
            .iter()
            .map(|row| TransactionRow {
                trans_id: row.try_get::<i32, _>("trans_id").unwrap_or(0),
                trans_item_id: row.try_get::<i32, _>("trans_item_id").ok(),
                item_code: row.try_get::<String, _>("item_code").ok(),
                item_name: row.try_get::<String, _>("item_name").ok(),
                trans_type: row.try_get::<String, _>("trans_type").unwrap_or_default(),
                trans_quantity: row.try_get::<i32, _>("trans_quantity").unwrap_or(0),
                trans_room_id: row.try_get::<i32, _>("trans_room_id").ok(),
                trans_notes: row.try_get::<String, _>("trans_notes").ok(),
                trans_date: row.try_get::<NaiveDateTime, _>("trans_date").ok(),
                trans_by: row.try_get::<String, _>("trans_by").ok(),
            })
            .collect();

        Ok((transactions, total))
    }

    async fn insert_transaction(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        item_id: i32,
        trans_type: &str,
        quantity: i32,
        room_id: Option<i32>,
        notes: Option<&str>,
        created_by: Option<&str>,
    ) -> Result<i32, sqlx::Error> {
        let rec = sqlx::query!(
            r#"INSERT INTO ht_inventory_transactions (trans_item_id, trans_type, trans_quantity, trans_room_id, trans_notes, trans_by)
        VALUES ($1, $2, $3, $4, $5, $6) RETURNING trans_id"#,
            item_id,
            trans_type,
            quantity,
            room_id,
            notes,
            created_by
        )
        .fetch_one(&mut **tx)
        .await?;
        Ok(rec.trans_id)
    }

    async fn insert_stock_adjustment_transaction(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        item_id: i32,
        trans_type: &str,
        quantity: i32,
        notes: Option<&str>,
        created_by: Option<&str>,
    ) -> Result<i32, sqlx::Error> {
        let rec = sqlx::query!(
            r#"INSERT INTO ht_inventory_transactions (trans_item_id, trans_type, trans_quantity, trans_room_id, trans_notes, trans_by)
        VALUES ($1, $2, $3, NULL, $4, $5) RETURNING trans_id"#,
            item_id,
            trans_type,
            quantity,
            notes,
            created_by
        )
        .fetch_one(&mut **tx)
        .await?;
        Ok(rec.trans_id)
    }

    async fn get_stats(&self, pool: &PgPool) -> Result<InventoryStatsRow, sqlx::Error> {
        let rec = sqlx::query!(
            r#"SELECT
            (SELECT COUNT(*)::int FROM ht_inventory_items WHERE item_active = true) as total_items,
            (SELECT COUNT(*)::int FROM ht_inventory_categories WHERE cat_active = true) as total_categories,
            (SELECT COUNT(*)::int FROM ht_inventory_items WHERE item_active = true AND item_current_stock <= item_min_stock AND item_current_stock > 0) as low_stock_count,
            (SELECT COUNT(*)::int FROM ht_inventory_items WHERE item_active = true AND item_current_stock = 0) as out_of_stock_count,
            (SELECT COALESCE(SUM(item_current_stock * COALESCE(item_cost, 0)), 0)::float8 FROM ht_inventory_items WHERE item_active = true) as total_stock_value"#
        )
        .fetch_one(pool)
        .await?;

        Ok(InventoryStatsRow {
            total_items: rec.total_items,
            total_categories: rec.total_categories,
            low_stock_count: rec.low_stock_count,
            out_of_stock_count: rec.out_of_stock_count,
            total_stock_value: rec.total_stock_value,
        })
    }

    async fn list_low_stock(&self, pool: &PgPool) -> Result<Vec<ItemRow>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"SELECT i.item_id, i.item_code, i.item_name, i.item_category_id, c.cat_name,
            i.item_unit, i.item_min_stock, i.item_current_stock,
            i.item_cost::float8 as item_cost, i.item_active, i.item_created, i.item_updated
        FROM ht_inventory_items i
        LEFT JOIN ht_inventory_categories c ON i.item_category_id = c.cat_id
        WHERE i.item_active = true AND i.item_current_stock <= i.item_min_stock
        ORDER BY (i.item_current_stock - i.item_min_stock) ASC, i.item_name ASC"#
        )
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| ItemRow {
                item_id: r.item_id,
                item_code: r.item_code,
                item_name: r.item_name,
                item_category_id: r.item_category_id,
                cat_name: Some(r.cat_name),
                item_unit: r.item_unit,
                item_min_stock: r.item_min_stock,
                item_current_stock: r.item_current_stock,
                item_cost: r.item_cost,
                item_active: r.item_active,
                item_created: r.item_created,
                item_updated: r.item_updated,
            })
            .collect())
    }
}
