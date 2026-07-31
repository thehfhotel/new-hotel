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
//!
//! Two **canonical** (not `legacy_mirror.*`) master tables piggy-back
//! the same cadence, because their legacy sources are likewise not
//! CT-enabled and change on the order of weeks:
//! * `HT_Products`    → `ht_products`    (`sync/mappers/products.rs`)
//! * `HT_Rooms_Price` → `ht_rate_tiers`  (`sync/mappers/rate_tiers.rs`)
//!
//! Both are UPSERT-only rather than DELETE+INSERT: canonical rows carry
//! foreign keys from live transactional data, so the prune-on-reload
//! shortcut used above is not available to them. See
//! `sync/mappers/products.rs`'s module docstring for the full argument.

use crate::db::mssql_timeout::{simple_query_with_timeout_pooled, MssqlOpKind};
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

    // Track F3 (T1 CRIT-3): canonical `ht_products` UPSERT keyed on
    // `prod_legacy_no = Pro_no`. Same rationale as ht_rate_tiers below —
    // `HT_Products` is not CT-enabled, so it rides this reconcile cadence.
    //
    // Runs FIRST because it is the only dimension the CT watcher has a hard
    // FK dependency on: `sync/mappers/mirror.rs::upsert_canonical_pos_sale`
    // INNER JOINs `ht_products` to resolve an `HT_CheckIn_Product` folio line
    // and returns Err when the product is missing, holding the watermark —
    // and the GLOBAL watermark gates on `!errored`, so an unmirrored product
    // freezes every CT table for that site, not just POS. Loading products
    // before the slower `legacy_mirror.*` DELETE+INSERT reloads keeps that
    // window as short as this cadence allows. NOTE this bounds the race, it
    // does not eliminate it: the CT tick is a different task (and, at HF
    // Hotel, a different process) running sub-second against this 15-minute
    // reload. A product created AND sold inside one interval still errors —
    // correctly, since holding the watermark preserves the event — and the
    // retry after this reload succeeds, well inside the 2-day CT retention.
    if let Err(e) = crate::sync::mappers::products::poll_products_once(legacy_pool, pg_pool).await {
        tracing::error!(error = %e, "[Mirror] reload ht_products failed");
    }
    if let Err(e) = reload_continuetime(legacy_pool, pg_pool).await {
        tracing::error!(error = %e, "[Mirror] reload HT_ContinueTime failed");
    }
    if let Err(e) = reload_rooms_price(legacy_pool, pg_pool).await {
        tracing::error!(error = %e, "[Mirror] reload HT_Rooms_Price failed");
    }
    // Track F4 (T1 CRIT-4): canonical `ht_rate_tiers` UPSERT keyed on
    // (Room_Type, Cust_Type). Shares the 15-min reconcile cadence
    // because `HT_Rooms_Price` is not CT-enabled and changes on the
    // order of weeks, not seconds. Independent TX from the
    // legacy_mirror reload above — failures are surface-level (logged,
    // not propagated) so one bad row does not stall the dimension
    // reload cycle.
    if let Err(e) = crate::sync::mappers::rate_tiers::reload_rate_tiers(legacy_pool, pg_pool).await {
        tracing::error!(error = %e, "[Mirror] reload ht_rate_tiers failed");
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

/// The 7 transactional legacy_mirror tables the bootstrap snapshot covers,
/// as `(legacy MSSQL table, canonical PG mirror table)` pairs. Single source
/// of truth for read-only consumers that need the set without running the
/// snapshot — e.g. the `sync --bootstrap --dry-run` preview, which reports
/// source-vs-mirror row counts per table. The actual snapshot
/// (`snapshot_mirror_transactional_tables`) still calls one bespoke function
/// per table (each carries table-specific projection/skip logic); a test pins
/// this list's length to that helper count so the two can't drift.
pub const MIRROR_TRANSACTIONAL_TABLES: &[(&str, &str)] = &[
    ("HT_Cupon", "legacy_mirror.ht_cupon"),
    ("HT_CheckIn_Product", "legacy_mirror.ht_checkin_product"),
    ("HT_Deposit", "legacy_mirror.ht_deposit"),
    ("HT_Changed_Room", "legacy_mirror.ht_changed_room"),
    ("HT_Bill_Debt_H", "legacy_mirror.ht_bill_debt_h"),
    ("HT_Bill_Debt_Ds", "legacy_mirror.ht_bill_debt_ds"),
    ("HT_Book_Pro", "legacy_mirror.ht_book_pro"),
];

/// **Bootstrap-only** snapshot of the 7 transactional legacy_mirror
/// tables (HT_Cupon, HT_CheckIn_Product, HT_Deposit, HT_Changed_Room,
/// HT_Bill_Debt_H, HT_Bill_Debt_Ds, HT_Book_Pro).
///
/// Why bootstrap-only: these tables are CT-tracked (Phase 5.5b) and
/// the CT mappers (Phase 5.5c) handle every change that lands AFTER
/// CT was enabled. But CT history starts from `MIN_VALID_VERSION`,
/// which is the version when CT was enabled — pre-existing rows
/// never appear in CT-driven updates. This function fills that gap
/// by full-table DELETE + INSERT once during `--bootstrap`. After
/// bootstrap, CT mappers maintain steady state.
///
/// `mirror_source = 'reconcile'` distinguishes these snapshot rows
/// from `'ct'` rows the mappers stamp in incrementally.
///
/// Errors per-table are logged but don't abort the others — partial
/// progress is preferable to all-or-nothing in a one-shot bootstrap.
pub async fn snapshot_mirror_transactional_tables(legacy_pool: &DbPool, pg_pool: &PgPool) {
    let start = Instant::now();
    tracing::info!("[Mirror] Bootstrap snapshot of legacy_mirror transactional tables...");

    if let Err(e) = snapshot_cupon(legacy_pool, pg_pool).await {
        tracing::error!(error = %e, "[Mirror] snapshot HT_Cupon failed");
    }
    if let Err(e) = snapshot_checkin_product(legacy_pool, pg_pool).await {
        tracing::error!(error = %e, "[Mirror] snapshot HT_CheckIn_Product failed");
    }
    if let Err(e) = snapshot_deposit(legacy_pool, pg_pool).await {
        tracing::error!(error = %e, "[Mirror] snapshot HT_Deposit failed");
    }
    if let Err(e) = snapshot_changed_room(legacy_pool, pg_pool).await {
        tracing::error!(error = %e, "[Mirror] snapshot HT_Changed_Room failed");
    }
    if let Err(e) = snapshot_bill_debt_h(legacy_pool, pg_pool).await {
        tracing::error!(error = %e, "[Mirror] snapshot HT_Bill_Debt_H failed");
    }
    if let Err(e) = snapshot_bill_debt_ds(legacy_pool, pg_pool).await {
        tracing::error!(error = %e, "[Mirror] snapshot HT_Bill_Debt_Ds failed");
    }
    if let Err(e) = snapshot_book_pro(legacy_pool, pg_pool).await {
        tracing::error!(error = %e, "[Mirror] snapshot HT_Book_Pro failed");
    }

    tracing::info!(
        duration_ms = start.elapsed().as_millis(),
        "[Mirror] Transactional snapshot complete"
    );
}

async fn reload_continuetime(legacy_pool: &DbPool, pg_pool: &PgPool) -> Result<(), AnyError> {
    let mut conn = legacy_pool.get().await?;
    let rows = simple_query_with_timeout_pooled(
        &mut conn,
        "SELECT id, Con_Name, Con_Minute, Con_Price, Con_Type FROM HT_ContinueTime",
        MssqlOpKind::Read,
    )
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
        // Audit finding N5 — defense-in-depth: `try_get` returns
        // `Result<Option<T>>` so a type mismatch becomes `None` instead
        // of crashing the whole reload. The schema-fingerprint guard
        // upstream catches most drift; this is the belt to that
        // suspenders.
        let con_name: Option<&str> = r.try_get(1).ok().flatten();
        let con_minute: Option<i32> = r.try_get(2).ok().flatten();
        let con_price: Option<f64> = r.try_get(3).ok().flatten();
        let con_type: Option<&str> = r.try_get(4).ok().flatten();
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
    let rows = simple_query_with_timeout_pooled(
        &mut conn,
        "SELECT id, Room_Type, Room_CustType, Room_Price, Room_Price_H, Room_Price_M \
         FROM HT_Rooms_Price",
        MssqlOpKind::Read,
    )
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
        // Audit finding N5 — see snapshot_cupon comment on try_get.
        let room_type: Option<&str> = r.try_get(1).ok().flatten();
        let room_custtype: Option<&str> = r.try_get(2).ok().flatten();
        let room_price: Option<f64> = r.try_get(3).ok().flatten();
        let room_price_h: Option<f64> = r.try_get(4).ok().flatten();
        let room_price_m: Option<f64> = r.try_get(5).ok().flatten();
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
    let rows = simple_query_with_timeout_pooled(&mut conn, &select_sql, MssqlOpKind::Read).await?;

    let mut tx = pg_pool.begin().await?;
    let delete_sql = format!("DELETE FROM {pg_table}");
    sqlx::query(sqlx::AssertSqlSafe(&*delete_sql)).execute(&mut *tx).await?;

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
        // Audit finding N5 — see snapshot_cupon comment on try_get.
        let cust_type: Option<&str> = r.try_get(1).ok().flatten();
        let cust_month: Option<i32> = r.try_get(2).ok().flatten();
        let cast_type: Option<&str> = r.try_get(3).ok().flatten();
        sqlx::query(sqlx::AssertSqlSafe(&*insert_sql))
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

// ============================================================================
// Phase 5.5c-b — Bootstrap-only snapshots of the 6 transactional mirror
// tables. Same DELETE+INSERT pattern as the dimension reloads above, just
// bigger volumes (HT_Cupon ~17k, HT_Changed_Room ~3.9k at HF Hotel as of
// 2026-04). Other 4 tables empty at HF Hotel; snapshots no-op cleanly.
// ============================================================================

async fn snapshot_cupon(legacy_pool: &DbPool, pg_pool: &PgPool) -> Result<(), AnyError> {
    let mut conn = legacy_pool.get().await?;
    let rows = simple_query_with_timeout_pooled(
        &mut conn,
        "SELECT cupon_no, cupon_cin_no, cupon_cin_room, cupon_date, \
                cupon_gen_date, cupon_by, cupon_print FROM HT_Cupon",
        MssqlOpKind::Read,
    )
    .await?;

    let mut tx = pg_pool.begin().await?;
    sqlx::query("DELETE FROM legacy_mirror.ht_cupon")
        .execute(&mut *tx)
        .await?;

    let (mut inserted, mut skipped) = (0i64, 0i64);
    for r in &rows {
        let Some(cupon_no): Option<i32> = r.get(0) else {
            skipped += 1;
            continue;
        };
        sqlx::query(
            "INSERT INTO legacy_mirror.ht_cupon \
                (cupon_no, cupon_cin_no, cupon_cin_room, cupon_date, \
                 cupon_gen_date, cupon_by, cupon_print, mirror_source) \
             VALUES ($1, $2, $3, $4, $5, $6, COALESCE($7, 0), 'reconcile')",
        )
        .bind(cupon_no)
        .bind(r.try_get::<&str, _>(1).ok().flatten())
        .bind(r.try_get::<&str, _>(2).ok().flatten())
        .bind(r.try_get::<chrono::NaiveDateTime, _>(3).ok().flatten())
        .bind(r.try_get::<chrono::NaiveDateTime, _>(4).ok().flatten())
        .bind(r.try_get::<&str, _>(5).ok().flatten())
        .bind(r.try_get::<i32, _>(6).ok().flatten())
        .execute(&mut *tx)
        .await?;
        inserted += 1;
    }
    tx.commit().await?;
    tracing::info!(
        table = "HT_Cupon",
        inserted,
        skipped_null_pk = skipped,
        "[Mirror] snapshot complete"
    );
    Ok(())
}

async fn snapshot_checkin_product(legacy_pool: &DbPool, pg_pool: &PgPool) -> Result<(), AnyError> {
    let mut conn = legacy_pool.get().await?;
    let rows = simple_query_with_timeout_pooled(
        &mut conn,
        "SELECT id, Cin_No, Cin_Room_no, Cin_Ds_date, Cin_Pro_id, Cin_Pro_name, \
                Cin_Pro_Unit, Cin_Pro_num, Cin_Pro_price, Cin_Pro_priceTotal, \
                Cin_Pro_pay, Cin_Pro_note FROM HT_CheckIn_Product",
        MssqlOpKind::Read,
    )
    .await?;

    let mut tx = pg_pool.begin().await?;
    sqlx::query("DELETE FROM legacy_mirror.ht_checkin_product")
        .execute(&mut *tx)
        .await?;

    let (mut inserted, mut skipped) = (0i64, 0i64);
    for r in &rows {
        let Some(id): Option<i32> = r.get(0) else {
            skipped += 1;
            continue;
        };
        sqlx::query(
            "INSERT INTO legacy_mirror.ht_checkin_product \
                (id, cin_no, cin_room_no, cin_ds_date, cin_pro_id, cin_pro_name, \
                 cin_pro_unit, cin_pro_num, cin_pro_price, cin_pro_pricetotal, \
                 cin_pro_pay, cin_pro_note, mirror_source) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, 'reconcile')",
        )
        .bind(id)
        .bind(r.try_get::<&str, _>(1).ok().flatten())
        .bind(r.try_get::<&str, _>(2).ok().flatten())
        .bind(r.try_get::<chrono::NaiveDateTime, _>(3).ok().flatten())
        .bind(r.try_get::<&str, _>(4).ok().flatten())
        .bind(r.try_get::<&str, _>(5).ok().flatten())
        .bind(r.try_get::<&str, _>(6).ok().flatten())
        .bind(r.try_get::<f64, _>(7).ok().flatten())
        .bind(r.try_get::<f64, _>(8).ok().flatten())
        .bind(r.try_get::<f64, _>(9).ok().flatten())
        .bind(r.try_get::<f64, _>(10).ok().flatten())
        .bind(r.try_get::<&str, _>(11).ok().flatten())
        .execute(&mut *tx)
        .await?;
        inserted += 1;
    }
    tx.commit().await?;
    tracing::info!(
        table = "HT_CheckIn_Product",
        inserted,
        skipped_null_pk = skipped,
        "[Mirror] snapshot complete"
    );
    Ok(())
}

async fn snapshot_deposit(legacy_pool: &DbPool, pg_pool: &PgPool) -> Result<(), AnyError> {
    let mut conn = legacy_pool.get().await?;
    let rows = simple_query_with_timeout_pooled(
        &mut conn,
        "SELECT id, Dep_no, Dep_Date, Dep_Room, Dep_Name, \
                Dep_Price, Dep_Status, Dep_ref FROM HT_Deposit",
        MssqlOpKind::Read,
    )
    .await?;

    let mut tx = pg_pool.begin().await?;
    sqlx::query("DELETE FROM legacy_mirror.ht_deposit")
        .execute(&mut *tx)
        .await?;

    let (mut inserted, mut skipped) = (0i64, 0i64);
    for r in &rows {
        let Some(id): Option<i32> = r.get(0) else {
            skipped += 1;
            continue;
        };
        sqlx::query(
            "INSERT INTO legacy_mirror.ht_deposit \
                (id, dep_no, dep_date, dep_room, dep_name, \
                 dep_price, dep_status, dep_ref, mirror_source) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'reconcile')",
        )
        .bind(id)
        .bind(r.try_get::<&str, _>(1).ok().flatten())
        .bind(r.try_get::<chrono::NaiveDateTime, _>(2).ok().flatten())
        .bind(r.try_get::<&str, _>(3).ok().flatten())
        .bind(r.try_get::<&str, _>(4).ok().flatten())
        .bind(r.try_get::<f64, _>(5).ok().flatten())
        .bind(r.try_get::<&str, _>(6).ok().flatten())
        .bind(r.try_get::<&str, _>(7).ok().flatten())
        .execute(&mut *tx)
        .await?;
        inserted += 1;
    }
    tx.commit().await?;
    tracing::info!(
        table = "HT_Deposit",
        inserted,
        skipped_null_pk = skipped,
        "[Mirror] snapshot complete"
    );
    Ok(())
}

async fn snapshot_changed_room(legacy_pool: &DbPool, pg_pool: &PgPool) -> Result<(), AnyError> {
    let mut conn = legacy_pool.get().await?;
    let rows = simple_query_with_timeout_pooled(
        &mut conn,
        "SELECT id, cin_no, room_before, room_after, change_date, \
                room_before_price, Note, ToPrice FROM HT_Changed_Room",
        MssqlOpKind::Read,
    )
    .await?;

    let mut tx = pg_pool.begin().await?;
    sqlx::query("DELETE FROM legacy_mirror.ht_changed_room")
        .execute(&mut *tx)
        .await?;

    let (mut inserted, mut skipped_null_pk, mut skipped_null_cin) = (0i64, 0i64, 0i64);
    for r in &rows {
        let Some(id): Option<i32> = r.get(0) else {
            skipped_null_pk += 1;
            continue;
        };
        let Some(cin_no): Option<&str> = r.get(1) else {
            // ht_changed_room.cin_no is NOT NULL in the mirror schema
            // (matches the legacy column constraint). A NULL cin_no in
            // legacy data means the row is malformed — skip and log.
            skipped_null_cin += 1;
            continue;
        };
        sqlx::query(
            "INSERT INTO legacy_mirror.ht_changed_room \
                (id, cin_no, room_before, room_after, change_date, \
                 room_before_price, note, toprice, mirror_source) \
             VALUES ($1, $2, $3, $4, $5, COALESCE($6, 0), $7, $8, 'reconcile')",
        )
        .bind(id)
        .bind(cin_no)
        .bind(r.try_get::<&str, _>(2).ok().flatten())
        .bind(r.try_get::<&str, _>(3).ok().flatten())
        .bind(r.try_get::<chrono::NaiveDateTime, _>(4).ok().flatten())
        .bind(r.try_get::<f64, _>(5).ok().flatten())
        .bind(r.try_get::<&str, _>(6).ok().flatten())
        .bind(r.try_get::<&str, _>(7).ok().flatten())
        .execute(&mut *tx)
        .await?;
        inserted += 1;
    }
    tx.commit().await?;
    tracing::info!(
        table = "HT_Changed_Room",
        inserted,
        skipped_null_pk,
        skipped_null_cin,
        "[Mirror] snapshot complete"
    );
    Ok(())
}

async fn snapshot_bill_debt_h(legacy_pool: &DbPool, pg_pool: &PgPool) -> Result<(), AnyError> {
    let mut conn = legacy_pool.get().await?;
    let rows = simple_query_with_timeout_pooled(
        &mut conn,
        "SELECT Bill_No, Bill_Cust_ID, Bill_Cust_Name, Bill_Cust_Address, \
                Bill_Cust_Tel, Bill_Cust_Fax, Bill_Date, Bill_Ref, \
                Bill_Price_Type, Bill_Type, Bill_Total, Bill_Pay, \
                Bill_Debt, Bill_Pay_CASH, Bill_Pay_CREDIT, Bill_Status, \
                Bill_by, Bill_Note FROM HT_Bill_Debt_H",
        MssqlOpKind::Read,
    )
    .await?;

    let mut tx = pg_pool.begin().await?;
    sqlx::query("DELETE FROM legacy_mirror.ht_bill_debt_h")
        .execute(&mut *tx)
        .await?;

    let (mut inserted, mut skipped) = (0i64, 0i64);
    for r in &rows {
        let Some(bill_no): Option<&str> = r.get(0) else {
            skipped += 1;
            continue;
        };
        sqlx::query(
            "INSERT INTO legacy_mirror.ht_bill_debt_h \
                (bill_no, bill_cust_id, bill_cust_name, bill_cust_address, \
                 bill_cust_tel, bill_cust_fax, bill_date, bill_ref, \
                 bill_price_type, bill_type, bill_total, bill_pay, \
                 bill_debt, bill_pay_cash, bill_pay_credit, bill_status, \
                 bill_by, bill_note, mirror_source) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, \
                     $13, $14, $15, $16, $17, $18, 'reconcile')",
        )
        .bind(bill_no)
        .bind(r.try_get::<&str, _>(1).ok().flatten())
        .bind(r.try_get::<&str, _>(2).ok().flatten())
        .bind(r.try_get::<&str, _>(3).ok().flatten())
        .bind(r.try_get::<&str, _>(4).ok().flatten())
        .bind(r.try_get::<&str, _>(5).ok().flatten())
        .bind(r.try_get::<chrono::NaiveDateTime, _>(6).ok().flatten())
        .bind(r.try_get::<&str, _>(7).ok().flatten())
        .bind(r.try_get::<&str, _>(8).ok().flatten())
        .bind(r.try_get::<&str, _>(9).ok().flatten())
        .bind(r.try_get::<f64, _>(10).ok().flatten())
        .bind(r.try_get::<f64, _>(11).ok().flatten())
        .bind(r.try_get::<f64, _>(12).ok().flatten())
        .bind(r.try_get::<f64, _>(13).ok().flatten())
        .bind(r.try_get::<f64, _>(14).ok().flatten())
        .bind(r.try_get::<&str, _>(15).ok().flatten())
        .bind(r.try_get::<&str, _>(16).ok().flatten())
        .bind(r.try_get::<&str, _>(17).ok().flatten())
        .execute(&mut *tx)
        .await?;
        inserted += 1;
    }
    tx.commit().await?;
    tracing::info!(
        table = "HT_Bill_Debt_H",
        inserted,
        skipped_null_pk = skipped,
        "[Mirror] snapshot complete"
    );
    Ok(())
}

async fn snapshot_bill_debt_ds(legacy_pool: &DbPool, pg_pool: &PgPool) -> Result<(), AnyError> {
    let mut conn = legacy_pool.get().await?;
    let rows = simple_query_with_timeout_pooled(
        &mut conn,
        "SELECT id, Bill_No, DS_ID, DS_NO, DS_NAME, \
                DS_UNIT, DS_NUM, DS_PRICE, DS_PRICE_TOTAL FROM HT_Bill_Debt_Ds",
        MssqlOpKind::Read,
    )
    .await?;

    let mut tx = pg_pool.begin().await?;
    sqlx::query("DELETE FROM legacy_mirror.ht_bill_debt_ds")
        .execute(&mut *tx)
        .await?;

    let (mut inserted, mut skipped_null_pk, mut skipped_null_bill_no) = (0i64, 0i64, 0i64);
    for r in &rows {
        let Some(id): Option<i32> = r.get(0) else {
            skipped_null_pk += 1;
            continue;
        };
        // Audit finding N7 (Phase 5.5 codebase audit, 2026-04-29) —
        // skip rows with NULL bill_no. `bill_no` is the FK to
        // HT_Bill_Debt_H (the actual PK is `id` identity). Downstream
        // queries always filter by bill_no, so a row with NULL bill_no
        // is invisible to consumers — inserting it is wasted storage
        // and a foot-gun. Empty table at HF Hotel today; no migration
        // needed.
        let Some(bill_no): Option<&str> = r.try_get(1).ok().flatten() else {
            skipped_null_bill_no += 1;
            continue;
        };
        sqlx::query(
            "INSERT INTO legacy_mirror.ht_bill_debt_ds \
                (id, bill_no, ds_id, ds_no, ds_name, ds_unit, ds_num, ds_price, \
                 ds_price_total, mirror_source) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'reconcile')",
        )
        .bind(id)
        .bind(bill_no)
        .bind(r.try_get::<i32, _>(2).ok().flatten())
        .bind(r.try_get::<&str, _>(3).ok().flatten())
        .bind(r.try_get::<&str, _>(4).ok().flatten())
        .bind(r.try_get::<&str, _>(5).ok().flatten())
        .bind(r.try_get::<f64, _>(6).ok().flatten())
        .bind(r.try_get::<f64, _>(7).ok().flatten())
        .bind(r.try_get::<f64, _>(8).ok().flatten())
        .execute(&mut *tx)
        .await?;
        inserted += 1;
    }
    tx.commit().await?;
    tracing::info!(
        table = "HT_Bill_Debt_Ds",
        inserted,
        skipped_null_pk,
        skipped_null_bill_no,
        "[Mirror] snapshot complete"
    );
    Ok(())
}

// Track P2 (coexistence audit 2026-06) — HT_Book_Pro joined the CT set in
// Phase 5/E2 (`BookProMirrorMapper`, migrations legacy-mssql/023 + pg/056) but
// was never added to the bootstrap snapshot set above, so rows that predate CT
// enablement (~39 at HF Ville) are invisible to the watcher (CT history starts
// at MIN_VALID_VERSION) AND never snapshotted — they simply never reach
// `legacy_mirror.ht_book_pro`. This closes that gap: same DELETE+INSERT
// `mirror_source='reconcile'` pattern as the other 6, column-for-column aligned
// with the CT mapper's INSERT so 'reconcile' and 'ct' rows are interchangeable.
// The 9-column projection matches `BOOK_PRO_SELECT_COLS` in sync/mappers/mirror.rs.
async fn snapshot_book_pro(legacy_pool: &DbPool, pg_pool: &PgPool) -> Result<(), AnyError> {
    let mut conn = legacy_pool.get().await?;
    let rows = simple_query_with_timeout_pooled(
        &mut conn,
        "SELECT id, B_NO, B_ROOM, B_NAME, B_UNIT, B_NUM, \
                B_PRICE, B_PRICE_TOTAL, B_PRO_ID FROM HT_Book_Pro",
        MssqlOpKind::Read,
    )
    .await?;

    let mut tx = pg_pool.begin().await?;
    sqlx::query("DELETE FROM legacy_mirror.ht_book_pro")
        .execute(&mut *tx)
        .await?;

    let (mut inserted, mut skipped) = (0i64, 0i64);
    for r in &rows {
        // `id` is the IDENTITY PK (NOT NULL on the live baseline); a NULL here
        // would be a schema violation, so skip + count rather than insert junk.
        let Some(id): Option<i32> = r.get(0) else {
            skipped += 1;
            continue;
        };
        sqlx::query(
            "INSERT INTO legacy_mirror.ht_book_pro \
                (id, b_no, b_room, b_name, b_unit, b_num, \
                 b_price, b_price_total, b_pro_id, mirror_source) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'reconcile')",
        )
        .bind(id)
        .bind(r.try_get::<&str, _>(1).ok().flatten())
        .bind(r.try_get::<&str, _>(2).ok().flatten())
        .bind(r.try_get::<&str, _>(3).ok().flatten())
        .bind(r.try_get::<&str, _>(4).ok().flatten())
        .bind(r.try_get::<f64, _>(5).ok().flatten())
        .bind(r.try_get::<f64, _>(6).ok().flatten())
        .bind(r.try_get::<f64, _>(7).ok().flatten())
        .bind(r.try_get::<i32, _>(8).ok().flatten())
        .execute(&mut *tx)
        .await?;
        inserted += 1;
    }
    tx.commit().await?;
    tracing::info!(
        table = "HT_Book_Pro",
        inserted,
        skipped_null_pk = skipped,
        "[Mirror] snapshot complete"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    /// Track F3 regression pin — the same "helper exists, nothing calls it"
    /// class as the HT_Book_Pro guard below, but with a worse blast radius.
    /// `sync/mappers/products.rs` shipped with a module docstring describing
    /// a periodic poll and no poll: `ht_products` was populated only by an
    /// operator hand-typing `prod_legacy_no` into `POST /api/new/products`.
    /// The first product created and sold in iHOTEL therefore made
    /// `sync/mappers/mirror.rs::upsert_canonical_pos_sale` return Err, which
    /// marks the tick errored, which (since 42dc2c0 made the global watermark
    /// gate on `!errored`) pinned the watermark for ALL CT tables on that
    /// site until a human intervened — against a 2-day CT retention window.
    #[test]
    fn products_poll_is_wired_into_the_dimension_reload() {
        let full = include_str!("mirror.rs");
        let src = &full[..full
            .find("#[cfg(test)]")
            .expect("test module marker must exist")];

        let start = src
            .find("pub async fn reload_mirror_dimensions(")
            .expect("dimension-reload dispatcher must exist");
        let rest = &src[start..];
        let body = &rest[..rest.find("\n}").map(|i| i + 2).unwrap_or(rest.len())];

        assert!(
            body.contains("products::poll_products_once(legacy_pool, pg_pool)"),
            "`poll_products_once` is not called by reload_mirror_dimensions — \
             ht_products would have no automatic writer, and one product sold \
             in iHOTEL stalls the whole site's CT watermark"
        );
    }

    /// Products must be loaded BEFORE the sibling reloads, so a slow or
    /// failing `legacy_mirror.*` DELETE+INSERT can never delay the one
    /// dimension the CT watcher hard-depends on.
    #[test]
    fn products_poll_runs_before_the_legacy_mirror_reloads() {
        let full = include_str!("mirror.rs");
        let src = &full[..full
            .find("#[cfg(test)]")
            .expect("test module marker must exist")];
        let start = src
            .find("pub async fn reload_mirror_dimensions(")
            .expect("dimension-reload dispatcher must exist");
        let rest = &src[start..];
        let body = &rest[..rest.find("\n}").map(|i| i + 2).unwrap_or(rest.len())];

        let products_at = body
            .find("poll_products_once(legacy_pool")
            .expect("products poll must be called");
        let first_sibling = body
            .find("reload_continuetime(legacy_pool")
            .expect("reload_continuetime must be called");
        assert!(
            products_at < first_sibling,
            "the ht_products poll must be the FIRST reload in the cycle — it \
             is the only dimension a CT mapper blocks on"
        );
    }

    /// Drift guard for the class that left HT_Book_Pro out of the bootstrap for
    /// a full release (a `snapshot_*` helper existed but was never wired into
    /// `snapshot_mirror_transactional_tables`, so its rows never reached PG).
    /// Every `async fn snapshot_<table>` defined in this file MUST be invoked by
    /// the bootstrap dispatcher. The snapshots are a one-shot path with no live
    /// integration coverage (they need both MSSQL + PG), so this textual guard
    /// is the only mechanical signal that a new snapshot is actually reachable.
    #[test]
    fn every_snapshot_helper_is_wired_into_the_bootstrap_dispatcher() {
        // Scan only the code BEFORE this test module — otherwise the
        // `include_str!` self-reference matches the `"async fn snapshot_"`
        // literals in this very test and pollutes the helper list.
        let full = include_str!("mirror.rs");
        let src = &full[..full
            .find("#[cfg(test)]")
            .expect("test module marker must exist")];

        // Body of snapshot_mirror_transactional_tables (up to its first `\n}`).
        let disp_start = src
            .find("pub async fn snapshot_mirror_transactional_tables(")
            .expect("dispatcher fn must exist");
        let disp_rest = &src[disp_start..];
        let dispatcher = &disp_rest[..disp_rest
            .find("\n}")
            .map(|i| i + 2)
            .unwrap_or(disp_rest.len())];

        // Collect every `async fn snapshot_<name>(` helper definition.
        let mut helpers: Vec<&str> = Vec::new();
        for (i, _) in src.match_indices("async fn snapshot_") {
            let after = &src[i + "async fn ".len()..];
            let name_end = after.find('(').expect("fn def has `(`");
            let name = &after[..name_end];
            // Exclude the dispatcher itself (snapshot_mirror_transactional_tables).
            if name != "snapshot_mirror_transactional_tables" {
                helpers.push(name);
            }
        }
        assert!(
            helpers.len() >= 7,
            "expected the 7 transactional snapshots, found {}: {helpers:?}",
            helpers.len()
        );
        assert!(
            helpers.contains(&"snapshot_book_pro"),
            "snapshot_book_pro must be defined"
        );

        for name in &helpers {
            assert!(
                dispatcher.contains(&format!("{name}(legacy_pool")),
                "snapshot helper `{name}` is defined but NOT called by \
                 snapshot_mirror_transactional_tables — its rows would never \
                 reach legacy_mirror during --bootstrap"
            );
        }

        // MIRROR_TRANSACTIONAL_TABLES (used by the dry-run preview) must list
        // exactly one (mssql, pg) pair per snapshot helper — else the preview
        // silently omits a table the real snapshot writes (the book_pro class).
        assert_eq!(
            super::MIRROR_TRANSACTIONAL_TABLES.len(),
            helpers.len(),
            "MIRROR_TRANSACTIONAL_TABLES ({}) must have one entry per snapshot_* helper ({})",
            super::MIRROR_TRANSACTIONAL_TABLES.len(),
            helpers.len()
        );
    }
}
