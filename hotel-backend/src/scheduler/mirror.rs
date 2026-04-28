//! Phase 5.5a — full-table reload of slow-changing legacy dimension
//! tables into the `legacy_mirror.*` schema.
//!
//! Four tables are eligible for reconcile-only reload (no Change
//! Tracking enablement on the legacy DB required, no schema changes):
//! * `HT_ContinueTime`  — hourly extension price master
//! * `HT_Rooms_Price`   — per-customer-type room price overrides
//! * `HT_Order_Up`      — per-customer-type pricing tier (price-up)
//! * `HT_Order_Down`    — per-customer-type pricing tier (price-down)
//!
//! Strategy: full DELETE + INSERT in one PG transaction per table.
//! Tables are tiny (max ~32 rows at HF Hotel as of 2026-04 snapshot),
//! so prune-on-reload is trivially correct and avoids the diff/upsert
//! complexity that `scheduler::sync` carries for the canonical tables.
//!
//! The 6 transactional legacy-only tables (`HT_Cupon`,
//! `HT_CheckIn_Product`, `HT_Deposit`, `HT_Changed_Room`,
//! `HT_Bill_Debt_*`, `HT_Rooms_Cancel`) need incremental propagation
//! and are handled by the CT watcher (Phase 5.5c) — not here.

use crate::db::DbPool;
use sqlx::PgPool;
use std::time::Instant;

type AnyError = Box<dyn std::error::Error + Send + Sync>;

/// Reload all 4 dimension tables. Errors on any single table are
/// logged and observable but do NOT abort the others — each table is
/// independent and the failure modes are typically table-local
/// (e.g. legacy DB temporarily blocked one for an index rebuild).
pub async fn reload_mirror_dimensions(legacy_pool: &DbPool, pg_pool: &PgPool) {
    let start = Instant::now();
    tracing::info!("[Mirror] Reloading legacy_mirror dimension tables...");

    if let Err(e) = reload_continuetime(legacy_pool, pg_pool).await {
        tracing::error!(error = %e, "[Mirror] reload HT_ContinueTime failed");
    }
    if let Err(e) = reload_rooms_price(legacy_pool, pg_pool).await {
        tracing::error!(error = %e, "[Mirror] reload HT_Rooms_Price failed");
    }
    if let Err(e) = reload_order_up(legacy_pool, pg_pool).await {
        tracing::error!(error = %e, "[Mirror] reload HT_Order_Up failed");
    }
    if let Err(e) = reload_order_down(legacy_pool, pg_pool).await {
        tracing::error!(error = %e, "[Mirror] reload HT_Order_Down failed");
    }

    tracing::info!(
        duration_ms = start.elapsed().as_millis(),
        "[Mirror] Dimension reload cycle complete"
    );
}

async fn reload_continuetime(legacy_pool: &DbPool, pg_pool: &PgPool) -> Result<(), AnyError> {
    let mut conn = legacy_pool.get().await?;
    let rows = conn
        .simple_query(
            "SELECT id, Con_Name, Con_Minute, Con_Price, Con_Type FROM HT_ContinueTime",
        )
        .await?
        .into_first_result()
        .await?;

    let mut tx = pg_pool.begin().await?;
    sqlx::query("DELETE FROM legacy_mirror.ht_continuetime")
        .execute(&mut *tx)
        .await?;

    let (mut inserted, mut skipped) = (0i64, 0i64);
    for r in &rows {
        let Some(id): Option<i32> = r.get(0) else {
            skipped += 1;
            continue;
        };
        let con_name: Option<&str> = r.get(1);
        let con_minute: Option<i32> = r.get(2);
        let con_price: Option<f64> = r.get(3);
        let con_type: Option<&str> = r.get(4);
        sqlx::query(
            "INSERT INTO legacy_mirror.ht_continuetime \
                (id, con_name, con_minute, con_price, con_type, mirror_source) \
             VALUES ($1, $2, $3, $4, $5, 'reconcile')",
        )
        .bind(id)
        .bind(con_name)
        .bind(con_minute)
        .bind(con_price)
        .bind(con_type)
        .execute(&mut *tx)
        .await?;
        inserted += 1;
    }
    tx.commit().await?;
    tracing::info!(
        table = "HT_ContinueTime",
        inserted,
        skipped_null_pk = skipped,
        "[Mirror] reloaded"
    );
    Ok(())
}

async fn reload_rooms_price(legacy_pool: &DbPool, pg_pool: &PgPool) -> Result<(), AnyError> {
    let mut conn = legacy_pool.get().await?;
    let rows = conn
        .simple_query(
            "SELECT id, Room_Type, Room_CustType, Room_Price, Room_Price_H, Room_Price_M \
             FROM HT_Rooms_Price",
        )
        .await?
        .into_first_result()
        .await?;

    let mut tx = pg_pool.begin().await?;
    sqlx::query("DELETE FROM legacy_mirror.ht_rooms_price")
        .execute(&mut *tx)
        .await?;

    let (mut inserted, mut skipped) = (0i64, 0i64);
    for r in &rows {
        let Some(id): Option<i32> = r.get(0) else {
            skipped += 1;
            continue;
        };
        let room_type: Option<&str> = r.get(1);
        let room_custtype: Option<&str> = r.get(2);
        let room_price: Option<f64> = r.get(3);
        let room_price_h: Option<f64> = r.get(4);
        let room_price_m: Option<f64> = r.get(5);
        sqlx::query(
            "INSERT INTO legacy_mirror.ht_rooms_price \
                (id, room_type, room_custtype, room_price, room_price_h, room_price_m, mirror_source) \
             VALUES ($1, $2, $3, $4, $5, $6, 'reconcile')",
        )
        .bind(id)
        .bind(room_type)
        .bind(room_custtype)
        .bind(room_price)
        .bind(room_price_h)
        .bind(room_price_m)
        .execute(&mut *tx)
        .await?;
        inserted += 1;
    }
    tx.commit().await?;
    tracing::info!(
        table = "HT_Rooms_Price",
        inserted,
        skipped_null_pk = skipped,
        "[Mirror] reloaded"
    );
    Ok(())
}

async fn reload_order_up(legacy_pool: &DbPool, pg_pool: &PgPool) -> Result<(), AnyError> {
    reload_order_table(legacy_pool, pg_pool, "HT_Order_Up", "legacy_mirror.ht_order_up").await
}

async fn reload_order_down(legacy_pool: &DbPool, pg_pool: &PgPool) -> Result<(), AnyError> {
    reload_order_table(
        legacy_pool,
        pg_pool,
        "HT_Order_Down",
        "legacy_mirror.ht_order_down",
    )
    .await
}

/// HT_Order_Up and HT_Order_Down share an identical 4-column shape:
/// `(id INT, Cust_Type VARCHAR, Cust_Month INT, Cast_Type VARCHAR)`.
/// Inline factor of the reload to avoid duplicating identical SQL twice.
/// Note: `id` is NULLABLE in the legacy schema and NOT identity — the
/// app assigns it. NULL-PK rows are skipped (logged in the count).
async fn reload_order_table(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
    legacy_table: &str,
    pg_table: &str,
) -> Result<(), AnyError> {
    let mut conn = legacy_pool.get().await?;
    let select_sql = format!("SELECT id, Cust_Type, Cust_Month, Cast_Type FROM {legacy_table}");
    let rows = conn
        .simple_query(&select_sql)
        .await?
        .into_first_result()
        .await?;

    let mut tx = pg_pool.begin().await?;
    let delete_sql = format!("DELETE FROM {pg_table}");
    sqlx::query(&delete_sql).execute(&mut *tx).await?;

    let insert_sql = format!(
        "INSERT INTO {pg_table} (id, cust_type, cust_month, cast_type, mirror_source) \
         VALUES ($1, $2, $3, $4, 'reconcile')"
    );
    let (mut inserted, mut skipped) = (0i64, 0i64);
    for r in &rows {
        let Some(id): Option<i32> = r.get(0) else {
            skipped += 1;
            continue;
        };
        let cust_type: Option<&str> = r.get(1);
        let cust_month: Option<i32> = r.get(2);
        let cast_type: Option<&str> = r.get(3);
        sqlx::query(&insert_sql)
            .bind(id)
            .bind(cust_type)
            .bind(cust_month)
            .bind(cast_type)
            .execute(&mut *tx)
            .await?;
        inserted += 1;
    }
    tx.commit().await?;
    tracing::info!(
        table = legacy_table,
        inserted,
        skipped_null_pk = skipped,
        "[Mirror] reloaded"
    );
    Ok(())
}
