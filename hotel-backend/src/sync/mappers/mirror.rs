//! Phase 5.5c — CT mappers for the 6 transactional legacy-only tables
//! that mirror into `legacy_mirror.*`.
//!
//! Per `docs/architecture.md` §11 (opaque pass-through), these tables
//! are owned by the legacy .NET app; we never write to them from the
//! application path. The CT watcher subscribes to row changes and
//! stamps each one into the corresponding `legacy_mirror.<table>` PG
//! table so our UI can display them without round-tripping to MSSQL.
//!
//! Each mapper:
//! * Returns `Ok(None)` from [`apply`] (no `DomainEvent` emission —
//!   nothing in our app subscribes to legacy-only changes via the bus)
//! * UPSERTs the row on I/U using the legacy natural key as PK
//! * DELETEs the mirror row on D
//! * Sets `mirror_source = 'ct'` so dashboards can distinguish
//!   incremental CT writes from `--bootstrap` snapshot rows
//!
//! The tables and their natural keys (matching `migrations/pg/020`):
//!
//! | MSSQL                | Mirror PG table                  | PK         |
//! |----------------------|----------------------------------|------------|
//! | `HT_Cupon`           | `legacy_mirror.ht_cupon`         | `cupon_no` |
//! | `HT_CheckIn_Product` | `legacy_mirror.ht_checkin_product` | `id`     |
//! | `HT_Deposit`         | `legacy_mirror.ht_deposit`       | `id`       |
//! | `HT_Changed_Room`    | `legacy_mirror.ht_changed_room`  | `id`       |
//! | `HT_Bill_Debt_H`     | `legacy_mirror.ht_bill_debt_h`   | `Bill_No`  |
//! | `HT_Bill_Debt_Ds`    | `legacy_mirror.ht_bill_debt_ds`  | `id`       |
//! | `HT_Rooms_Cancel`    | `legacy_mirror.ht_rooms_cancel`  | `id`       |
//! | `HT_Book_Pro`        | `legacy_mirror.ht_book_pro`      | `id`       |
//!
//! Track E1 (audit 2026-05-13 T2 HIGH-5) added `HT_Rooms_Cancel` — CT
//! was enabled back in Phase 5 (migration 020) but no mapper consumed
//! the rows; the mirror table sat empty while CT retention silently
//! accumulated row history forever. The new
//! `RoomsCancelMirrorMapper` closes the dangling subscription.
//!
//! Phase 5/E2 (coexistence audit 2026-06-11 P2) added `HT_Book_Pro`
//! (pre-booked products attached to a booking) — see
//! [`BookProMirrorMapper`] for the iHOTEL semantics and the
//! booking→check-in conversion gap it surfaces.
//!
//! ## Echo-before-stamp adoption (2026-07-28)
//!
//! Two of the mappers here don't just mirror — they also reverse-sync into
//! a CANONICAL table our own app writes (`HT_CheckIn_Product` →
//! `ht_pos_sales`, `HT_Changed_Room` → `ht_room_changes`). Both canonical
//! tables carry a legacy back-link that the writeback worker stamps
//! **after** the MSSQL commit, on a separate PG connection. A CT tick that
//! lands inside that window sees an unstamped canonical row, matches
//! nothing on the back-link, and inserts a phantom duplicate.
//! [`ADOPT_UNSTAMPED_POS_SALE_SQL`] and
//! [`ADOPT_UNSTAMPED_ROOM_CHANGE_SQL`] close that window by claiming the
//! matching unstamped app-originated row first — the same remedy
//! `sync::mappers::guest_registry` already applies to
//! `HT_CheckIn_Other_People`.
//!
//! ## Eager product mirror on a POS-sale FK miss (#263)
//!
//! [`upsert_canonical_pos_sale`] resolves `Cin_Pro_id` through an INNER
//! JOIN on `ht_products` and returns `Err` on a miss, which holds the
//! GLOBAL watermark (see [`FkMissStage`] — that `Err` is the June-3
//! silent-drop guarantee and must never soften into a skip). Before #263
//! the only thing that could clear such a miss was the next
//! `products::poll_products_once` tick inside
//! `scheduler::mirror::reload_mirror_dimensions` — a ≤15-minute stall of
//! that site's entire CT sync. [`resolve_pos_sale_fks_or_eager_mirror`]
//! closes it the way `checkin::resolve_customer_or_eager_mirror` closes
//! the customer equivalent: fetch the one `HT_Products` row inline from
//! MSSQL, UPSERT it through `products::upsert_product` (so the
//! `Pro_Name` char-clamp and the canonical column set stay in ONE
//! place), and retry the resolve exactly once.
//!
//! **The eager path is present but NOT yet armed in production.**
//! [`MssqlChangeMapper::apply`] carries `(tx, op, row)` and no legacy
//! pool, and `CheckinProductMirrorMapper` is dispatched on the flat
//! per-row path (`bin/sync.rs::poll_table`), not the coalesced aggregate
//! path that hands `mssql` to `apply_*_aggregate`. So this module passes
//! `mssql = None` and every miss still takes the (unchanged)
//! watermark-holding `Err`. Arming it is a two-line change in files this
//! module does not own:
//!
//! 1. `sync/mapper.rs` — add a defaulted trait method
//!    `apply_with_legacy(&self, tx, mssql: &DbPool, op, row)` whose
//!    default body is `self.apply(tx, op, row).await`, and override it
//!    for `CheckinProductMirrorMapper`.
//! 2. `bin/sync.rs::poll_table` — swap the single
//!    `mapper.apply(&mut tx, op, Some(row))` call site for
//!    `mapper.apply_with_legacy(&mut tx, mssql, op, Some(row))`. The
//!    pool is already in scope there.
//!
//! That keeps the other 18 mappers, `build_mappers`, and every
//! integration test that calls `.apply(…)` directly untouched.

use async_trait::async_trait;

use crate::db::mssql_timeout::{simple_query_with_timeout_pooled, MssqlOpKind};
use crate::db::DbPool;
use crate::outbox::event::DomainEvent;
use crate::sync::change_op::ChangeOp;
use crate::sync::mapper::MssqlChangeMapper;
use crate::sync::mappers::products;
use crate::sync::row::test_support::HashMapRow;
use crate::sync::row::MappableRow;
use crate::sync::SyncError;

// ─── HT_Cupon ────────────────────────────────────────────────────────

pub struct CuponMirrorMapper;

/// CT JOIN projection for `HT_Cupon`. Held as a module-private const so
/// Track J1's projection-lock test can pin every column against the
/// authoritative HF Hotel schema dump.
const CUPON_SELECT_COLS: &str =
    "t.cupon_no, t.cupon_cin_no, t.cupon_cin_room, t.cupon_date, \
     t.cupon_gen_date, t.cupon_by, t.cupon_print";

#[async_trait]
impl MssqlChangeMapper for CuponMirrorMapper {
    fn table(&self) -> &'static str {
        "HT_Cupon"
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        &["cupon_no"]
    }

    fn select_sql(&self) -> &'static str {
        CUPON_SELECT_COLS
    }

    async fn apply(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        op: ChangeOp,
        row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        let row = row.ok_or_else(|| SyncError::Mapper {
            table: "HT_Cupon",
            message: "row required for both I/U and D".into(),
        })?;
        let cupon_no = row.try_get_i32("cupon_no")?.ok_or_else(|| SyncError::Mapper {
            table: "HT_Cupon",
            message: "cupon_no NULL — should not happen post Phase 5.5b".into(),
        })?;
        match op {
            ChangeOp::Delete => {
                sqlx::query("DELETE FROM legacy_mirror.ht_cupon WHERE cupon_no = $1")
                    .bind(cupon_no)
                    .execute(&mut **tx)
                    .await?;
            }
            ChangeOp::Insert | ChangeOp::Update => {
                sqlx::query(
                    "INSERT INTO legacy_mirror.ht_cupon \
                        (cupon_no, cupon_cin_no, cupon_cin_room, cupon_date, \
                         cupon_gen_date, cupon_by, cupon_print, \
                         mirrored_at, mirror_source) \
                     VALUES ($1, $2, $3, $4, $5, $6, COALESCE($7, 0), now(), 'ct') \
                     ON CONFLICT (cupon_no) DO UPDATE SET \
                        cupon_cin_no   = EXCLUDED.cupon_cin_no, \
                        cupon_cin_room = EXCLUDED.cupon_cin_room, \
                        cupon_date     = EXCLUDED.cupon_date, \
                        cupon_gen_date = EXCLUDED.cupon_gen_date, \
                        cupon_by       = EXCLUDED.cupon_by, \
                        cupon_print    = EXCLUDED.cupon_print, \
                        mirrored_at    = now(), \
                        mirror_source  = 'ct'",
                )
                .bind(cupon_no)
                .bind(row.try_get_str("cupon_cin_no")?)
                .bind(row.try_get_str("cupon_cin_room")?)
                .bind(row.try_get_datetime("cupon_date")?)
                .bind(row.try_get_datetime("cupon_gen_date")?)
                .bind(row.try_get_str("cupon_by")?)
                .bind(row.try_get_i32("cupon_print")?)
                .execute(&mut **tx)
                .await?;
            }
        }
        // Track G5 — dual-write into canonical `ht_coupons` so
        // iHOTEL-issued coupons are visible to the new app's
        // dashboards. Mirrors the Track G4 `ChangedRoomMirrorMapper`
        // dual-write pattern; both writes run in the same TX so a PG
        // failure rolls back the mirror AND the canonical state
        // atomically.
        super::coupon::apply_canonical_cupon_event(tx, op, row).await?;
        Ok(None)
    }
}

// ─── HT_CheckIn_Product ──────────────────────────────────────────────

pub struct CheckinProductMirrorMapper;

/// CT JOIN projection for `HT_CheckIn_Product`. Held as a module-private
/// const so Track J1's projection-lock test can pin every column.
const CHECKIN_PRODUCT_SELECT_COLS: &str =
    "t.id, t.Cin_No, t.Cin_Room_no, t.Cin_Ds_date, t.Cin_Pro_id, \
     t.Cin_Pro_name, t.Cin_Pro_Unit, t.Cin_Pro_num, t.Cin_Pro_price, \
     t.Cin_Pro_priceTotal, t.Cin_Pro_pay, t.Cin_Pro_note";

impl CheckinProductMirrorMapper {
    /// Shared body for [`MssqlChangeMapper::apply`] /
    /// [`MssqlChangeMapper::apply_with_legacy`] — identical except for
    /// whether the legacy pool is in reach for the eager product
    /// mirror (#263).
    async fn apply_impl(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        mssql: Option<&DbPool>,
        op: ChangeOp,
        row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        let row = row.ok_or_else(|| SyncError::Mapper {
            table: "HT_CheckIn_Product",
            message: "row required".into(),
        })?;
        let id = row.try_get_i32("id")?.ok_or_else(|| SyncError::Mapper {
            table: "HT_CheckIn_Product",
            message: "id NULL — identity column should never be NULL".into(),
        })?;
        match op {
            ChangeOp::Delete => {
                sqlx::query("DELETE FROM legacy_mirror.ht_checkin_product WHERE id = $1")
                    .bind(id)
                    .execute(&mut **tx)
                    .await?;
                // Track G6 — cascade the canonical row delete keyed
                // on the legacy back-link. Idempotent: legacy-origin
                // races where no canonical row exists yet are no-ops.
                sqlx::query("DELETE FROM ht_pos_sales WHERE sale_legacy_id = $1")
                    .bind(id)
                    .execute(&mut **tx)
                    .await?;
            }
            ChangeOp::Insert | ChangeOp::Update => {
                let cin_no = row.try_get_str("Cin_No")?;
                let room_no = row.try_get_str("Cin_Room_no")?;
                let ds_date = row.try_get_datetime("Cin_Ds_date")?;
                let pro_id = row.try_get_str("Cin_Pro_id")?;
                let pro_name = row.try_get_str("Cin_Pro_name")?;
                let pro_unit = row.try_get_str("Cin_Pro_Unit")?;
                let pro_num = row.try_get_f64("Cin_Pro_num")?;
                let pro_price = row.try_get_f64("Cin_Pro_price")?;
                let pro_pricetotal = row.try_get_f64("Cin_Pro_priceTotal")?;
                let pro_pay = row.try_get_f64("Cin_Pro_pay")?;
                let pro_note = row.try_get_str("Cin_Pro_note")?;

                sqlx::query(
                    "INSERT INTO legacy_mirror.ht_checkin_product \
                        (id, cin_no, cin_room_no, cin_ds_date, cin_pro_id, \
                         cin_pro_name, cin_pro_unit, cin_pro_num, cin_pro_price, \
                         cin_pro_pricetotal, cin_pro_pay, cin_pro_note, \
                         mirrored_at, mirror_source) \
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, now(), 'ct') \
                     ON CONFLICT (id) DO UPDATE SET \
                        cin_no             = EXCLUDED.cin_no, \
                        cin_room_no        = EXCLUDED.cin_room_no, \
                        cin_ds_date        = EXCLUDED.cin_ds_date, \
                        cin_pro_id         = EXCLUDED.cin_pro_id, \
                        cin_pro_name       = EXCLUDED.cin_pro_name, \
                        cin_pro_unit       = EXCLUDED.cin_pro_unit, \
                        cin_pro_num        = EXCLUDED.cin_pro_num, \
                        cin_pro_price      = EXCLUDED.cin_pro_price, \
                        cin_pro_pricetotal = EXCLUDED.cin_pro_pricetotal, \
                        cin_pro_pay        = EXCLUDED.cin_pro_pay, \
                        cin_pro_note       = EXCLUDED.cin_pro_note, \
                        mirrored_at        = now(), \
                        mirror_source      = 'ct'",
                )
                .bind(id)
                .bind(cin_no)
                .bind(room_no)
                .bind(ds_date)
                .bind(pro_id)
                .bind(pro_name)
                .bind(pro_unit)
                .bind(pro_num)
                .bind(pro_price)
                .bind(pro_pricetotal)
                .bind(pro_pay)
                .bind(pro_note)
                .execute(&mut **tx)
                .await?;

                // Track G6 — reverse-sync into the canonical
                // `ht_pos_sales` table so sales rung up via iHOTEL
                // also land canonically. The mapper resolves
                // `Cin_No` → `cin_id` and `Cin_Pro_id` → `prod_id`
                // via FK joins; a resolution miss ERRORS so the
                // watcher holds the watermark (the mirror UPSERT
                // above still commits — the tick TX is not rolled
                // back on a per-row error — and the retry re-applies
                // both writes idempotently). Pre-2026-06-11 misses
                // were silently skipped under the false "next CT tick
                // re-fires" belief — the June-3 silent-drop class.
                //
                // `mssql` flows through from the dispatch path: the
                // watcher's flat per-row loop calls `apply_with_legacy`
                // (pool present → eager product mirror ARMED, #263);
                // direct `apply` callers — tests, any legacy code path —
                // get `None` and byte-identical pre-#263 behaviour.
                upsert_canonical_pos_sale(
                    tx, mssql, id, cin_no, pro_id, pro_num, pro_price, pro_note, ds_date,
                )
                .await?;
            }
        }
        Ok(None)
    }
}

#[async_trait]
impl MssqlChangeMapper for CheckinProductMirrorMapper {
    fn table(&self) -> &'static str {
        "HT_CheckIn_Product"
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        CHECKIN_PRODUCT_SELECT_COLS
    }

    async fn apply(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        op: ChangeOp,
        row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        self.apply_impl(tx, None, op, row).await
    }

    /// Arms the eager product mirror (#263): the watcher's per-row
    /// dispatch hands the legacy pool down so a missing `ht_products`
    /// row is mirrored on the spot instead of holding the watermark for
    /// up to one dimension-reload interval. Legacy is READ here, never
    /// written.
    async fn apply_with_legacy(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        mssql: &DbPool,
        op: ChangeOp,
        row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        self.apply_impl(tx, Some(mssql), op, row).await
    }
}

/// Echo-before-stamp adoption for `ht_pos_sales` (2026-07-28). Same defect
/// and same remedy as `guest_registry::ADOPT_UNSTAMPED_COMPANION_SQL`.
///
/// When OUR app posts a POS line, `service::pos` commits the canonical row
/// with `sale_legacy_id IS NULL` and enqueues
/// `WritebackIntent::RecordPosSale`. The worker INSERTs
/// `HT_CheckIn_Product` and only THEN stamps the captured IDENTITY onto
/// `sale_legacy_id` — **after** the MSSQL commit, on a separate PG
/// connection (`bin/writeback.rs::back_populate_legacy_ids`). A CT tick
/// landing inside that window finds no row carrying this `sale_legacy_id`,
/// so the dedup UPDATE below matches 0 rows and the legacy-origin INSERT
/// creates a phantom canonical duplicate. This statement claims the
/// matching unstamped row first, so the dedup UPDATE hits it and the
/// INSERT never runs. Back-population usually wins the race (making this a
/// no-op); its own `AND sale_legacy_id IS NULL` guard means a stamp that
/// arrives after an adoption is a harmless 0-row UPDATE.
///
/// Natural key = every field `writeback/recipes/pos_sale.rs` puts on the
/// wire, expressed in canonical space:
///
/// | canonical column  | legacy column     | why it round-trips exactly |
/// |-------------------|-------------------|----------------------------|
/// | `sale_cin_id` $2  | `Cin_No`          | resolved via `ht_checkins.legacy_cin_no` |
/// | `sale_product_id` $3 | `Cin_Pro_id`   | resolved via `ht_products.prod_legacy_no` |
/// | `sale_qty` $4     | `Cin_Pro_num`     | recipe formats `{:.3}` → `ROUND(…, 3)` |
/// | `sale_unit_price` $5 | `Cin_Pro_price`| recipe formats `{:.2}` → `ROUND(…, 2)` |
/// | `sale_note` $6    | `Cin_Pro_note`    | written verbatim, same varchar(500) width |
/// | `sale_sold_at` $7 | `Cin_Ds_date`     | `format_legacy_datetime` = Bangkok wall-clock TRUNCATED to the second |
///
/// `sale_legacy_id IS NULL` + `source = 'canonical'` restrict adoption to
/// app-originated rows — this mapper stamps `sale_legacy_id` on every row
/// it creates, so an unstamped row can only be ours. The `NOT EXISTS`
/// guard makes the statement idempotent: once any canonical row carries
/// this legacy id (back-population won, or an earlier tick adopted), no
/// second row can ever be claimed for it, so CT re-delivery and
/// iHOTEL-side U-events fall straight through to the plain dedup path.
/// `ORDER BY sale_id` (oldest first) matches the FIFO order the writeback
/// queue drains in, which is the order MSSQL allocated the IDENTITYs.
const ADOPT_UNSTAMPED_POS_SALE_SQL: &str = "UPDATE ht_pos_sales SET sale_legacy_id = $1 \
     WHERE sale_id = ( \
        SELECT sale_id FROM ht_pos_sales \
        WHERE sale_legacy_id IS NULL \
          AND source = 'canonical' \
          AND sale_cin_id = $2 \
          AND sale_product_id = $3 \
          AND sale_qty = ROUND($4::numeric, 3) \
          AND sale_unit_price = ROUND($5::numeric, 2) \
          AND COALESCE(sale_note, '') = COALESCE($6, '') \
          AND date_trunc('second', sale_sold_at AT TIME ZONE 'Asia/Bangkok') = $7::timestamp \
        ORDER BY sale_id LIMIT 1) \
       AND NOT EXISTS (SELECT 1 FROM ht_pos_sales x WHERE x.sale_legacy_id = $1)";

/// Track G6 — UPSERT a canonical `ht_pos_sales` row from the legacy
/// `HT_CheckIn_Product` projection. Resolves `Cin_No` →
/// `ht_checkins.cin_id`, `Cin_Pro_id` → `ht_products.prod_id`.
///
/// A NULL `Cin_No` / `Cin_Pro_id` on the legacy row is a deliberate
/// skip (the row genuinely references nothing — retrying can't change
/// its shape). A RESOLUTION miss, however, ERRORS (2026-06-11): the
/// parent rows simply haven't been mirrored yet, and nothing ever
/// re-fires a consumed CT row — the pre-fix silent skip permanently
/// dropped the canonical sale (June-3 class). The error holds the
/// watermark; the mirror-table UPSERT above still commits (per-row
/// errors don't roll back the tick TX) so `legacy_mirror` readers see
/// the row immediately, and the retried apply is idempotent.
///
/// `mssql` (#263) is the optional legacy pool used to eager-mirror a
/// not-yet-known product before giving up — see
/// [`resolve_pos_sale_fks_or_eager_mirror`]. `None` reproduces the
/// pre-#263 behaviour exactly (miss → `Err`), which is what the trait
/// dispatch path passes today.
#[allow(clippy::too_many_arguments)]
async fn upsert_canonical_pos_sale(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    mssql: Option<&DbPool>,
    legacy_id: i32,
    cin_no: Option<&str>,
    pro_id: Option<&str>,
    pro_num: Option<f64>,
    pro_price: Option<f64>,
    pro_note: Option<&str>,
    ds_date: Option<chrono::NaiveDateTime>,
) -> Result<(), SyncError> {
    let Some(cin_no) = cin_no else { return Ok(()) };
    let Some(pro_id) = pro_id else { return Ok(()) };
    // "-1" is iHOTEL's "no product" sentinel for a folio line whose product id
    // resolved to nothing (FrmCheckIn's `value3 = "-1"` fallback; the new
    // booking→check-in product transfer emits it too). It can NEVER satisfy the
    // `prod_legacy_no` FK join below, so treat it exactly like a NULL pro_id and
    // SKIP — a folio line referencing no real product has no canonical pos-sale
    // to mirror. Without this, a "-1" row falls through to the watermark-holding
    // Err and PERMANENTLY jams all HT_CheckIn_Product CT sync (both apps). Genuine
    // not-yet-mirrored products (a real Pro_no) still hit the Err below and hold
    // the watermark for retry, preserving the eager-mirror-or-Err guarantee.
    if pro_id == "-1" {
        return Ok(());
    }

    // Resolve cin_id + prod_id in one round trip; on a miss, try the
    // eager product mirror once (#263) and otherwise error so the
    // watermark holds and the retry runs against mirrored parents.
    let (cin_id, prod_id) =
        resolve_pos_sale_fks_or_eager_mirror(tx, mssql, legacy_id, cin_no, pro_id).await?;

    // Two-step UPSERT keyed on `sale_legacy_id`. The partial UNIQUE
    // index `WHERE sale_legacy_id IS NOT NULL` means standard
    // `ON CONFLICT (sale_legacy_id)` needs the predicate inlined,
    // which sqlx-dynamic can't bind cleanly. Manual UPDATE-then-
    // INSERT is just as idempotent under the serialized CT-tick
    // driver and reads cleanly.
    //
    // Rows our app originated carry `sale_legacy_id IS NULL` until
    // the writeback worker back-populates it. This mapper only
    // touches rows whose `sale_legacy_id` is already set (legacy-
    // origin) OR creates a fresh legacy-origin row when no
    // canonical row exists yet for this legacy id.
    let qty = pro_num.unwrap_or(0.0);
    let unit_price = pro_price.unwrap_or(0.0);

    // Echo-before-stamp adoption — see ADOPT_UNSTAMPED_POS_SALE_SQL. Runs
    // BEFORE the dedup UPDATE so an adopted row is already stamped by the
    // time that UPDATE runs; the whole apply then converges to exactly the
    // state the non-racing order (back-populate first, echo second)
    // produces, including `source = 'legacy'`.
    let adopted = sqlx::query(ADOPT_UNSTAMPED_POS_SALE_SQL)
        .bind(legacy_id)
        .bind(cin_id)
        .bind(prod_id)
        .bind(qty)
        .bind(unit_price)
        .bind(pro_note)
        .bind(ds_date)
        .execute(&mut **tx)
        .await?
        .rows_affected();
    if adopted > 0 {
        tracing::info!(
            legacy_id,
            cin_id,
            prod_id,
            "adopted unstamped app-originated ht_pos_sales row (CT echo beat \
             the writeback back-population) — no phantom duplicate inserted"
        );
    }

    let updated = sqlx::query(
        "UPDATE ht_pos_sales SET \
             sale_cin_id     = $1, \
             sale_product_id = $2, \
             sale_qty        = $3::numeric, \
             sale_unit_price = $4::numeric, \
             sale_note       = $5, \
             sale_sold_at    = COALESCE($6::timestamp AT TIME ZONE 'Asia/Bangkok', sale_sold_at), \
             source          = 'legacy', \
             updated_at      = NOW() \
           WHERE sale_legacy_id = $7",
    )
    .bind(cin_id)
    .bind(prod_id)
    .bind(qty)
    .bind(unit_price)
    .bind(pro_note)
    .bind(ds_date)
    .bind(legacy_id)
    .execute(&mut **tx)
    .await?
    .rows_affected();

    if updated == 0 {
        // Legacy-origin INSERT (no canonical row yet for this id).
        // Aggregate id derived via UUID v5 over the legacy id under
        // a stable namespace so retries from a duplicate CT row
        // converge to the same UUID — matching the
        // `product_aggregate_fallback` shape from F3.
        let namespace = uuid::Uuid::new_v5(
            &uuid::Uuid::NAMESPACE_OID,
            b"new-hotel.aggregate.pos_sale.legacy_id",
        );
        let agg = uuid::Uuid::new_v5(&namespace, &legacy_id.to_be_bytes());
        sqlx::query(
            "INSERT INTO ht_pos_sales ( \
                 sale_cin_id, sale_product_id, sale_qty, sale_unit_price, \
                 sale_note, sale_sold_at, source, aggregate_id, sale_legacy_id \
             ) VALUES ( \
                 $1, $2, $3::numeric, $4::numeric, $5, \
                 COALESCE($6::timestamp AT TIME ZONE 'Asia/Bangkok', NOW()), \
                 'legacy', $7, $8 \
             ) \
             ON CONFLICT (aggregate_id) DO NOTHING",
        )
        .bind(cin_id)
        .bind(prod_id)
        .bind(qty)
        .bind(unit_price)
        .bind(pro_note)
        .bind(ds_date)
        .bind(agg)
        .bind(legacy_id)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

// ─── POS-sale FK resolution with eager product mirror (#263) ─────────

/// The combined FK resolve for one `HT_CheckIn_Product` line. Hoisted to
/// a const so the shape test below can pin that it stays an INNER JOIN —
/// softening it to a LEFT JOIN would resolve `cin_id` with a NULL
/// `prod_id` and turn the watermark-holding `Err` into a silent skip.
/// `&'static str` also keeps it on sqlx's `SqlSafeStr` fast path.
const POS_SALE_FK_RESOLVE_SQL: &str = "SELECT c.cin_id, p.prod_id \
       FROM ht_checkins c \
       JOIN ht_products p ON p.prod_legacy_no = $2 \
      WHERE c.legacy_cin_no = $1 \
      LIMIT 1";

/// Which side of [`POS_SALE_FK_RESOLVE_SQL`] missed. Answering that
/// costs one indexed lookup and decides whether an MSSQL round trip can
/// possibly help — the product is the only side with an eager path
/// (a missing check-in is driven by its own CT table).
const CANONICAL_PRODUCT_PROBE_SQL: &str =
    "SELECT 1 FROM ht_products WHERE prod_legacy_no = $1 LIMIT 1";

/// Strategy for sourcing a `HT_Products` row when the canonical
/// `prod_legacy_no` lookup misses. The production strategy hits MSSQL
/// via tiberius; tests inject a closure that returns a synthesised
/// `HashMapRow` so the eager-mirror path is exercised without a live
/// legacy connection.
///
/// Deliberately the same shape as `checkin::CustomerSource` — that is
/// the established eager-mirror idiom in this crate and the two should
/// stay recognisably identical. Kept `#[doc(hidden)] pub` for the same
/// reason: the integration suite is a separate crate compiled without
/// the lib's `test` cfg, so a `#[cfg(test)]` gate would put the stub out
/// of its reach.
#[doc(hidden)]
pub enum ProductSource<'a> {
    /// Production: borrow a live MSSQL pool and `SELECT` one row by
    /// `Pro_no`.
    Mssql(&'a DbPool),
    /// Test injection: deterministic stub returning the row a real MSSQL
    /// fetch would have returned.
    Stub(ProductRowSupplier<'a>),
}

/// Injected row supplier behind [`ProductSource::Stub`]. Aliased rather
/// than written inline so the variant stays under clippy's
/// `type_complexity` threshold.
#[doc(hidden)]
pub type ProductRowSupplier<'a> = Box<dyn Fn(&str) -> Option<HashMapRow> + Send + Sync + 'a>;

/// Why a POS-sale FK resolution gave up. EVERY variant ends in the
/// watermark-holding `Err` — the enum exists to make the reason
/// greppable in production logs, never to introduce a skip. Adding a
/// variant that maps to `Ok(())` would re-open the June-3 silent-drop
/// class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FkMissStage {
    /// No legacy pool reached this mapper, so no eager fetch was even
    /// attempted. The pre-#263 behaviour, and what the trait dispatch
    /// path still produces today.
    NoEagerAttempt,
    /// The product IS canonical — the parent `ht_checkins` row is what's
    /// missing. No eager path exists for it here (`HT_CheckIn_H` drives
    /// its own aggregate mapper), so hold and retry.
    CheckinNotMirrored,
    /// The eager fetch ran and `HT_Products` has no usable row for this
    /// `Cin_Pro_id` at all.
    ProductAbsentInLegacy,
    /// The product was eagerly mirrored, but the retried join still
    /// misses — i.e. the check-in side is missing too.
    StillUnresolvedAfterEagerMirror,
}

impl FkMissStage {
    /// Distinct, greppable tail for the log/error line.
    fn reason(self) -> &'static str {
        match self {
            Self::NoEagerAttempt => {
                "product not yet mirrored and no legacy pool wired into this \
                 mapper (eager product mirror unarmed — see module docstring)"
            }
            Self::CheckinNotMirrored => {
                "product resolves; the parent check-in is not yet mirrored"
            }
            Self::ProductAbsentInLegacy => {
                "eager product mirror found no usable HT_Products row for this \
                 Cin_Pro_id either"
            }
            Self::StillUnresolvedAfterEagerMirror => {
                "product eagerly mirrored from MSSQL but the join still misses \
                 — the parent check-in is not yet mirrored"
            }
        }
    }
}

/// Build the watermark-holding error for a FK miss. Keeps the
/// pre-#263 `canonical pos-sale FK unresolvable for legacy id=…` prefix
/// (operators grep for it) and appends the stage-specific reason.
fn pos_sale_fk_error(
    stage: FkMissStage,
    legacy_id: i32,
    cin_no: &str,
    pro_id: &str,
) -> SyncError {
    SyncError::Mapper {
        table: "HT_CheckIn_Product",
        message: format!(
            "canonical pos-sale FK unresolvable for legacy id={legacy_id} \
             cin_no={cin_no} pro_id={pro_id} — {}; holding watermark for \
             loud retry",
            stage.reason()
        ),
    }
}

/// What to do after the combined resolve missed. PURE branch table so
/// the recovery's control flow is testable without PG or MSSQL.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FkMissAction {
    /// Give up now with this stage — no MSSQL round trip can help.
    HoldWatermark(FkMissStage),
    /// The product side is the missing one and a pool is available.
    EagerMirrorProduct,
}

/// PURE. See [`FkMissAction`].
fn classify_fk_miss(product_in_canonical: bool, have_mssql: bool) -> FkMissAction {
    if product_in_canonical {
        // The join can only have missed on the check-in side.
        FkMissAction::HoldWatermark(FkMissStage::CheckinNotMirrored)
    } else if have_mssql {
        FkMissAction::EagerMirrorProduct
    } else {
        FkMissAction::HoldWatermark(FkMissStage::NoEagerAttempt)
    }
}

/// PURE. Outcome of one eager attempt: `None` means the sale resolved
/// (the only non-error exit), `Some(stage)` is the reason to hold the
/// watermark. A legacy row that exists but fails to upsert never reaches
/// here — `products::upsert_product`'s error propagates out of
/// [`eager_mirror_product`] as a `SyncError`, which is also a hold.
fn classify_eager_outcome(mirrored: bool, resolved: bool) -> Option<FkMissStage> {
    match (mirrored, resolved) {
        (_, true) => None,
        (false, false) => Some(FkMissStage::ProductAbsentInLegacy),
        (true, false) => Some(FkMissStage::StillUnresolvedAfterEagerMirror),
    }
}

/// Resolve `(cin_id, prod_id)` for one folio line; on a product miss,
/// eagerly mirror the referenced `HT_Products` row from MSSQL into
/// `ht_products` and retry the resolve ONCE.
///
/// Returns the resolved pair or a `SyncError` — deliberately NOT an
/// `Option`, so no future edit can turn a miss into a skip (June-3
/// silent-drop class; `sync::resolve` module doc).
///
/// ## Why eager-mirror instead of waiting for the poll
///
/// `products::poll_products_once` (wired first in
/// `scheduler::mirror::reload_mirror_dimensions`, `7b57edc`) bounds the
/// stall at the 15-minute reload cadence, but the CT watcher's GLOBAL
/// watermark gates on `!errored` — so for those minutes the miss freezes
/// EVERY CT table for that site, not just POS lines. A product named by
/// a `HT_CheckIn_Product` row must exist in `HT_Products` (iHOTEL
/// inserts the SKU before it can be rung up), so a synchronous
/// single-row fetch is always safe — the same argument
/// `checkin::resolve_customer_or_eager_mirror` makes for `HT_Customers`.
///
/// The eager UPSERT runs in the CT tick's own transaction, so the
/// product and the sale it unblocks commit atomically.
async fn resolve_pos_sale_fks_or_eager_mirror(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    mssql: Option<&DbPool>,
    legacy_id: i32,
    cin_no: &str,
    pro_id: &str,
) -> Result<(i32, i64), SyncError> {
    // Fast path — both parents already canonical.
    if let Some(hit) = resolve_pos_sale_fks(tx, cin_no, pro_id).await? {
        return Ok(hit);
    }

    let product_in_canonical = canonical_product_exists(tx, pro_id).await?;
    let pool = match classify_fk_miss(product_in_canonical, mssql.is_some()) {
        FkMissAction::HoldWatermark(stage) => {
            return Err(pos_sale_fk_error(stage, legacy_id, cin_no, pro_id));
        }
        FkMissAction::EagerMirrorProduct => match mssql {
            Some(pool) => pool,
            // Unreachable by construction — `classify_fk_miss` only
            // returns `EagerMirrorProduct` when `have_mssql`. Kept as a
            // hold rather than a panic so a future refactor of the
            // branch table can't take down the watcher.
            None => {
                return Err(pos_sale_fk_error(
                    FkMissStage::NoEagerAttempt,
                    legacy_id,
                    cin_no,
                    pro_id,
                ));
            }
        },
    };

    // A failure INSIDE the eager mirror (MSSQL read error, malformed
    // legacy row, PG upsert rejection) propagates as `Err` — the legacy
    // row existed, so this must hold the watermark, never skip.
    let mirrored = eager_mirror_product(tx, ProductSource::Mssql(pool), pro_id)
        .await?
        .is_some();
    let resolved = if mirrored {
        resolve_pos_sale_fks(tx, cin_no, pro_id).await?
    } else {
        None
    };

    match (classify_eager_outcome(mirrored, resolved.is_some()), resolved) {
        (None, Some((cin_id, prod_id))) => {
            tracing::info!(
                legacy_id,
                cin_no,
                pro_id,
                cin_id,
                prod_id,
                "pos-sale eager product mirror: resolved on retry — closed the \
                 ≤15-min products-poll race without holding the watermark"
            );
            Ok((cin_id, prod_id))
        }
        (stage, _) => Err(pos_sale_fk_error(
            stage.unwrap_or(FkMissStage::StillUnresolvedAfterEagerMirror),
            legacy_id,
            cin_no,
            pro_id,
        )),
    }
}

/// The plain combined resolve — one round trip, no recovery.
async fn resolve_pos_sale_fks(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    cin_no: &str,
    pro_id: &str,
) -> Result<Option<(i32, i64)>, SyncError> {
    let resolved: Option<(i32, i64)> = sqlx::query_as(POS_SALE_FK_RESOLVE_SQL)
        .bind(cin_no)
        .bind(pro_id)
        .fetch_optional(&mut **tx)
        .await?;
    Ok(resolved)
}

/// Is this `Cin_Pro_id` already a canonical product? Discriminates the
/// two sides of the combined-resolve miss.
async fn canonical_product_exists(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    pro_id: &str,
) -> Result<bool, SyncError> {
    let hit: Option<i32> = sqlx::query_scalar(CANONICAL_PRODUCT_PROBE_SQL)
        .bind(pro_id)
        .fetch_optional(&mut **tx)
        .await?;
    Ok(hit.is_some())
}

/// Fetch one `HT_Products` row through [`ProductSource`] and UPSERT it
/// into canonical `ht_products`, returning the new `prod_id`.
///
/// `Ok(None)` means "no usable legacy row" (genuinely absent, or a blank
/// business key that could never satisfy the join) — the caller turns
/// that into the watermark-holding `Err`. It is NOT a skip.
///
/// The write goes through `products::upsert_product` on purpose: that is
/// where the `Pro_Name` → `prod_name` char-clamp lives
/// (`PROD_NAME_MAX_CHARS`; legacy `varchar(500)` counts TIS-620 BYTES,
/// canonical `VARCHAR(250)` counts CHARACTERS, so a long Thai name
/// really does overflow) and where `prod_active` is deliberately left
/// alone. A hand-rolled INSERT here would silently drift from the poll.
async fn eager_mirror_product(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    source: ProductSource<'_>,
    pro_id: &str,
) -> Result<Option<i64>, SyncError> {
    if pro_id.trim().is_empty() {
        // Can never satisfy `p.prod_legacy_no = $2`; don't spend an
        // MSSQL round trip per tick on it.
        tracing::warn!(
            pro_id,
            "pos-sale eager product mirror: blank Cin_Pro_id — no eager fetch \
             possible; caller will error to hold the watermark"
        );
        return Ok(None);
    }

    let projected = match &source {
        ProductSource::Mssql(pool) => fetch_product_projection_from_mssql(pool, pro_id).await?,
        ProductSource::Stub(f) => match f(pro_id) {
            // Same projection function the poll uses, so the eager path
            // can never disagree with it about column → field mapping.
            Some(row) => Some(products::project(&row)?),
            None => None,
        },
    };

    let Some(projection) = projected else {
        tracing::warn!(
            pro_id,
            "pos-sale eager product mirror: HT_Products has no row for this \
             Cin_Pro_id — caller will error to hold the watermark"
        );
        return Ok(None);
    };

    if !products::is_acceptable_projection(&projection) {
        tracing::warn!(
            pro_id,
            "pos-sale eager product mirror: legacy row has a blank Pro_no — \
             unusable as an FK target; caller will error to hold the watermark"
        );
        return Ok(None);
    }

    let prod_id = products::upsert_product(tx, &projection).await?;
    tracing::info!(
        pro_id,
        prod_id,
        "pos-sale eager product mirror: eagerly mirrored referenced product \
         from MSSQL"
    );
    Ok(Some(prod_id))
}

/// Pull one `HT_Products` row by `Pro_no` and project it. Mirrors
/// `checkin::fetch_customer_row_from_mssql` — a single-table `SELECT`
/// via `simple_query` with an inline-quoted WHERE value, wrapped in the
/// per-op read timeout so one stuck row-lock can't serialize-block the
/// watcher tick (R2, 2026-05-14).
///
/// Returns `Ok(None)` when the row genuinely doesn't exist in MSSQL.
async fn fetch_product_projection_from_mssql(
    pool: &DbPool,
    pro_no: &str,
) -> Result<Option<products::ProductProjection>, SyncError> {
    let mut conn = pool.get().await?;
    let sql = product_eager_fetch_sql(pro_no);
    let raw_rows = simple_query_with_timeout_pooled(&mut conn, &sql, MssqlOpKind::Read).await?;
    let Some(raw) = raw_rows.first() else {
        return Ok(None);
    };
    // `tiberius::Row` implements `MappableRow`, so the poll and this
    // eager fetch share ONE projection path.
    Ok(Some(products::project(raw)?))
}

/// Compose the single-row eager fetch from the products poll's own
/// SELECT, so the column list stays in lockstep with `POLL_COLUMNS`
/// (and inherits its Track-J1 projection-lock tests). PURE — no I/O.
///
/// Plain `'…'` literal, never `N'…'`: the legacy DB stores Thai as
/// TIS-620 and an `N` prefix corrupts it (findings.md).
fn product_eager_fetch_sql(pro_no: &str) -> String {
    format!(
        "{} WHERE Pro_no = {}",
        products::poll_select_sql(),
        sql_quote_inline(pro_no)
    )
}

/// SQL-quote a value for inline interpolation. Same semantics as
/// `checkin::sql_quote_inline` / `parent_loader::sql_quote_inline` —
/// duplicated (as those two already are of each other) so this module
/// stays self-contained for the eager-fetch path.
fn sql_quote_inline(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('\'');
    for ch in value.chars() {
        if ch == '\'' {
            out.push_str("''");
        } else {
            out.push(ch);
        }
    }
    out.push('\'');
    out
}

/// Test seam for the integration suite: exercises the eager-mirror path
/// without a live MSSQL connection by injecting a stub row-supplier.
/// Mirrors `checkin::resolve_customer_via_eager_mirror_for_test`.
///
/// Returns the mirrored `prod_id`, or `Ok(None)` when the stub supplies
/// no row (product truly missing in MSSQL) — which the production
/// caller converts into the watermark-holding `Err`.
#[doc(hidden)]
pub async fn eager_mirror_product_for_test<F>(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    pro_id: &str,
    supplier: F,
) -> Result<Option<i64>, SyncError>
where
    F: Fn(&str) -> Option<HashMapRow> + Send + Sync + 'static,
{
    eager_mirror_product(tx, ProductSource::Stub(Box::new(supplier)), pro_id).await
}

// ─── HT_Deposit ──────────────────────────────────────────────────────

pub struct DepositMirrorMapper;

/// CT JOIN projection for `HT_Deposit`. Held as a module-private const
/// so Track J1's projection-lock test can pin every column.
const DEPOSIT_SELECT_COLS: &str =
    "t.id, t.Dep_no, t.Dep_Date, t.Dep_Room, t.Dep_Name, \
     t.Dep_Price, t.Dep_Status, t.Dep_ref";

#[async_trait]
impl MssqlChangeMapper for DepositMirrorMapper {
    fn table(&self) -> &'static str {
        "HT_Deposit"
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        DEPOSIT_SELECT_COLS
    }

    async fn apply(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        op: ChangeOp,
        row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        let row = row.ok_or_else(|| SyncError::Mapper {
            table: "HT_Deposit",
            message: "row required".into(),
        })?;
        let id = row.try_get_i32("id")?.ok_or_else(|| SyncError::Mapper {
            table: "HT_Deposit",
            message: "id NULL — should not happen post Phase 5.5b".into(),
        })?;
        match op {
            ChangeOp::Delete => {
                sqlx::query("DELETE FROM legacy_mirror.ht_deposit WHERE id = $1")
                    .bind(id)
                    .execute(&mut **tx)
                    .await?;
            }
            ChangeOp::Insert | ChangeOp::Update => {
                sqlx::query(
                    "INSERT INTO legacy_mirror.ht_deposit \
                        (id, dep_no, dep_date, dep_room, dep_name, \
                         dep_price, dep_status, dep_ref, mirrored_at, mirror_source) \
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, now(), 'ct') \
                     ON CONFLICT (id) DO UPDATE SET \
                        dep_no        = EXCLUDED.dep_no, \
                        dep_date      = EXCLUDED.dep_date, \
                        dep_room      = EXCLUDED.dep_room, \
                        dep_name      = EXCLUDED.dep_name, \
                        dep_price     = EXCLUDED.dep_price, \
                        dep_status    = EXCLUDED.dep_status, \
                        dep_ref       = EXCLUDED.dep_ref, \
                        mirrored_at   = now(), \
                        mirror_source = 'ct'",
                )
                .bind(id)
                .bind(row.try_get_str("Dep_no")?)
                .bind(row.try_get_datetime("Dep_Date")?)
                .bind(row.try_get_str("Dep_Room")?)
                .bind(row.try_get_str("Dep_Name")?)
                .bind(row.try_get_f64("Dep_Price")?)
                .bind(row.try_get_str("Dep_Status")?)
                .bind(row.try_get_str("Dep_ref")?)
                .execute(&mut **tx)
                .await?;
            }
        }
        Ok(None)
    }
}

// ─── HT_Changed_Room ─────────────────────────────────────────────────

pub struct ChangedRoomMirrorMapper;

/// CT JOIN projection for `HT_Changed_Room`. Held as a module-private
/// const so Track J1's projection-lock test can pin every column.
const CHANGED_ROOM_SELECT_COLS: &str =
    "t.id, t.cin_no, t.room_before, t.room_after, t.change_date, \
     t.room_before_price, t.Note, t.ToPrice";

#[async_trait]
impl MssqlChangeMapper for ChangedRoomMirrorMapper {
    fn table(&self) -> &'static str {
        "HT_Changed_Room"
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        CHANGED_ROOM_SELECT_COLS
    }

    async fn apply(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        op: ChangeOp,
        row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        let row = row.ok_or_else(|| SyncError::Mapper {
            table: "HT_Changed_Room",
            message: "row required".into(),
        })?;
        let id = row.try_get_i32("id")?.ok_or_else(|| SyncError::Mapper {
            table: "HT_Changed_Room",
            message: "id NULL — identity column should never be NULL".into(),
        })?;
        match op {
            ChangeOp::Delete => {
                sqlx::query("DELETE FROM legacy_mirror.ht_changed_room WHERE id = $1")
                    .bind(id)
                    .execute(&mut **tx)
                    .await?;
                // Track G4 — also cascade the canonical row delete keyed
                // on the back-link. Idempotent: if no canonical row
                // exists yet (legacy-origin race) this is a no-op.
                sqlx::query("DELETE FROM ht_room_changes WHERE rc_legacy_id = $1")
                    .bind(id)
                    .execute(&mut **tx)
                    .await?;
            }
            ChangeOp::Insert | ChangeOp::Update => {
                let cin_no = row
                    .try_get_str("cin_no")?
                    .ok_or_else(|| SyncError::Mapper {
                        table: "HT_Changed_Room",
                        message: "cin_no NULL — schema declares NOT NULL".into(),
                    })?;
                let room_before = row.try_get_str("room_before")?;
                let room_after = row.try_get_str("room_after")?;
                let change_date = row.try_get_datetime("change_date")?;
                let price = row.try_get_f64("room_before_price")?;
                let note = row.try_get_str("Note")?;
                let toprice = row.try_get_str("ToPrice")?;

                sqlx::query(
                    "INSERT INTO legacy_mirror.ht_changed_room \
                        (id, cin_no, room_before, room_after, change_date, \
                         room_before_price, note, toprice, mirrored_at, mirror_source) \
                     VALUES ($1, $2, $3, $4, $5, COALESCE($6, 0), $7, $8, now(), 'ct') \
                     ON CONFLICT (id) DO UPDATE SET \
                        cin_no            = EXCLUDED.cin_no, \
                        room_before       = EXCLUDED.room_before, \
                        room_after        = EXCLUDED.room_after, \
                        change_date       = EXCLUDED.change_date, \
                        room_before_price = EXCLUDED.room_before_price, \
                        note              = EXCLUDED.note, \
                        toprice           = EXCLUDED.toprice, \
                        mirrored_at       = now(), \
                        mirror_source     = 'ct'",
                )
                .bind(id)
                .bind(cin_no)
                .bind(room_before)
                .bind(room_after)
                .bind(change_date)
                .bind(price)
                .bind(note)
                .bind(toprice)
                .execute(&mut **tx)
                .await?;

                // Track G4 / T4 HIGH-3 — also reverse-sync into the
                // canonical `ht_room_changes` table so a room change
                // performed via iHOTEL lands canonically. The legacy
                // row carries `cin_no` + room_no strings; we resolve
                // them to canonical ids via the existing junctions.
                // A resolution miss ERRORS so the watcher holds the
                // watermark (2026-06-11; the pre-fix silent skip never
                // retried — nothing re-fires a consumed CT row — so
                // the canonical room change was dropped forever).
                upsert_canonical_room_change(
                    tx, id, cin_no, room_before, room_after, change_date, price, note, toprice,
                )
                .await?;
            }
        }
        Ok(None)
    }
}

/// Echo-before-stamp adoption for `ht_room_changes` (2026-07-28) — the
/// `HT_Changed_Room` twin of [`ADOPT_UNSTAMPED_POS_SALE_SQL`]; read that
/// doc first for the race narrative.
///
/// `service::checkin::change_room` commits the canonical row with
/// `rc_legacy_id IS NULL` and enqueues `WritebackIntent::RoomChange`; the
/// worker INSERTs `HT_Changed_Room` and stamps `rc_legacy_id` only after
/// the MSSQL commit, on a separate PG connection. A CT tick inside that
/// window used to INSERT a phantom canonical duplicate.
///
/// Natural key = every field `writeback/recipes/room_change.rs` puts on
/// the wire (its INSERT populates all 7 columns), in canonical space:
///
/// | canonical column       | legacy column       | note |
/// |------------------------|---------------------|------|
/// | `rc_cin_id` $2         | `cin_no`            | resolved via `ht_checkins.legacy_cin_no` |
/// | `rc_from_room_id` $3   | `room_before`       | resolved via `ht_rooms_new.room_no` |
/// | `rc_to_room_id` $4     | `room_after`        | resolved via `ht_rooms_new.room_no` |
/// | `rc_room_before_price` $5 | `room_before_price` | recipe formats `{:.2}` → `ROUND(…, 2)` |
/// | `rc_reason` $6         | `Note`              | written verbatim |
/// | `rc_to_price` $7       | `ToPrice`           | written verbatim, varchar(20) both sides |
/// | `rc_changed_at` $8     | `change_date`       | Bangkok wall-clock truncated to the second |
///
/// The structural half of that key — one folio moving from room X to room
/// Y at one wall-clock second — is already unique by construction: a
/// second move of the same folio between the same two rooms in the same
/// second is not a thing a receptionist can produce. The content half is
/// belt-and-braces, matching the exact-content style of the guest-registry
/// adoption. `rc_legacy_id IS NULL` is the ownership guard (this mapper
/// stamps `rc_legacy_id` on every row it inserts, so an unstamped row is
/// necessarily app-originated — `ht_room_changes` has no `source` column),
/// and `NOT EXISTS` keeps re-delivered CT rows on the plain dedup path.
const ADOPT_UNSTAMPED_ROOM_CHANGE_SQL: &str = "UPDATE ht_room_changes SET rc_legacy_id = $1 \
     WHERE rc_id = ( \
        SELECT rc_id FROM ht_room_changes \
        WHERE rc_legacy_id IS NULL \
          AND rc_cin_id = $2 \
          AND rc_from_room_id = $3 \
          AND rc_to_room_id = $4 \
          AND rc_room_before_price = ROUND(COALESCE($5::numeric, 0), 2) \
          AND COALESCE(rc_reason, '') = COALESCE($6, '') \
          AND COALESCE(rc_to_price, '') = COALESCE($7, '') \
          AND date_trunc('second', rc_changed_at AT TIME ZONE 'Asia/Bangkok') = $8::timestamp \
        ORDER BY rc_id LIMIT 1) \
       AND NOT EXISTS (SELECT 1 FROM ht_room_changes x WHERE x.rc_legacy_id = $1)";

/// Track G4 / T4 HIGH-3 — UPSERT a canonical `ht_room_changes` row from
/// the legacy `HT_Changed_Room` projection. Resolves `cin_no` →
/// `ht_checkins.cin_id`, `room_before` / `room_after` →
/// `ht_rooms_new.room_id`.
///
/// NULL `room_before` / `room_after` on the legacy row is a deliberate
/// skip (the row genuinely lacks the data; retrying can't change it).
/// A RESOLUTION miss ERRORS (2026-06-11) so the watcher holds the
/// watermark — nothing ever re-fires a consumed CT row, and the
/// pre-fix silent skip dropped the canonical room change forever
/// (June-3 class). The mirror-table UPSERT above still commits
/// (per-row errors don't roll back the tick TX) so `legacy_mirror`
/// readers see the row immediately, and the retried apply is
/// idempotent.
#[allow(clippy::too_many_arguments)]
async fn upsert_canonical_room_change(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    legacy_id: i32,
    cin_no: &str,
    room_before: Option<&str>,
    room_after: Option<&str>,
    change_date: Option<chrono::NaiveDateTime>,
    price: Option<f64>,
    note: Option<&str>,
    toprice: Option<&str>,
) -> Result<(), SyncError> {
    let Some(room_before) = room_before else { return Ok(()) };
    let Some(room_after) = room_after else { return Ok(()) };

    // Resolve cin_id + both room_ids in a single round-trip. Any NULL
    // means the parent rows aren't yet mirrored — error so the
    // watermark holds and the retry runs against mirrored parents.
    let resolved: Option<(i32, i32, i32)> = sqlx::query_as(
        "SELECT c.cin_id, rf.room_id, rt.room_id \
           FROM ht_checkins  c \
           JOIN ht_rooms_new rf ON rf.room_no = $2 \
           JOIN ht_rooms_new rt ON rt.room_no = $3 \
          WHERE c.legacy_cin_no = $1 \
          LIMIT 1",
    )
    .bind(cin_no)
    .bind(room_before)
    .bind(room_after)
    .fetch_optional(&mut **tx)
    .await?;

    let Some((cin_id, from_room_id, to_room_id)) = resolved else {
        return Err(SyncError::Mapper {
            table: "HT_Changed_Room",
            message: format!(
                "canonical room-change FK unresolvable for legacy id={legacy_id} \
                 cin_no={cin_no} rooms {room_before}->{room_after} — parent \
                 checkin/rooms not yet mirrored; holding watermark for loud retry"
            ),
        });
    };

    // Two-step UPSERT keyed on the legacy back-link. The partial
    // UNIQUE index on `rc_legacy_id WHERE rc_legacy_id IS NOT NULL`
    // means standard `ON CONFLICT (rc_legacy_id) DO UPDATE` would
    // need the inference predicate spelled out, which sqlx can't bind
    // dynamically — manual UPDATE-then-INSERT is clearer and
    // equally idempotent under our serialized CT-tick driver.
    //
    // The canonical row our app originated has `rc_legacy_id IS NULL`
    // until the writeback worker's `mark_done` back-populates it;
    // this mapper only touches rows whose `rc_legacy_id` is already
    // set (legacy-origin) OR creates a fresh legacy-origin row.
    //
    // Echo-before-stamp adoption first — see
    // ADOPT_UNSTAMPED_ROOM_CHANGE_SQL. Stamping ahead of the dedup UPDATE
    // means an adopted row converges to exactly the state the non-racing
    // order would have produced.
    let adopted = sqlx::query(ADOPT_UNSTAMPED_ROOM_CHANGE_SQL)
        .bind(legacy_id)
        .bind(cin_id)
        .bind(from_room_id)
        .bind(to_room_id)
        .bind(price)
        .bind(note)
        .bind(toprice)
        .bind(change_date)
        .execute(&mut **tx)
        .await?
        .rows_affected();
    if adopted > 0 {
        tracing::info!(
            legacy_id,
            cin_id,
            from_room_id,
            to_room_id,
            "adopted unstamped app-originated ht_room_changes row (CT echo beat \
             the writeback back-population) — no phantom duplicate inserted"
        );
    }

    let updated = sqlx::query(
        "UPDATE ht_room_changes SET \
             rc_cin_id            = $1, \
             rc_from_room_id      = $2, \
             rc_to_room_id        = $3, \
             rc_reason            = $4, \
             rc_changed_at        = COALESCE($5::timestamp AT TIME ZONE 'Asia/Bangkok', rc_changed_at), \
             rc_room_before_price = COALESCE($6::numeric, rc_room_before_price), \
             rc_to_price          = $7, \
             rc_updated_at        = NOW() \
           WHERE rc_legacy_id = $8",
    )
    .bind(cin_id)
    .bind(from_room_id)
    .bind(to_room_id)
    .bind(note)
    .bind(change_date)
    .bind(price)
    .bind(toprice)
    .bind(legacy_id)
    .execute(&mut **tx)
    .await?
    .rows_affected();

    if updated == 0 {
        // Legacy-origin INSERT (no canonical row yet for this id).
        sqlx::query(
            "INSERT INTO ht_room_changes ( \
                 rc_cin_id, rc_from_room_id, rc_to_room_id, rc_reason, \
                 rc_changed_at, rc_room_before_price, rc_to_price, rc_legacy_id \
             ) VALUES ( \
                 $1, $2, $3, $4, \
                 COALESCE($5::timestamp AT TIME ZONE 'Asia/Bangkok', NOW()), \
                 COALESCE($6::numeric, 0), $7, $8 \
             )",
        )
        .bind(cin_id)
        .bind(from_room_id)
        .bind(to_room_id)
        .bind(note)
        .bind(change_date)
        .bind(price)
        .bind(toprice)
        .bind(legacy_id)
        .execute(&mut **tx)
        .await?;
    }
    Ok(())
}

// ─── HT_Bill_Debt_H ──────────────────────────────────────────────────

pub struct BillDebtHMirrorMapper;

/// CT JOIN projection for `HT_Bill_Debt_H`. Held as a module-private
/// const so Track J1's projection-lock test can pin every column.
const BILL_DEBT_H_SELECT_COLS: &str =
    "t.Bill_No, t.Bill_Cust_ID, t.Bill_Cust_Name, t.Bill_Cust_Address, \
     t.Bill_Cust_Tel, t.Bill_Cust_Fax, t.Bill_Date, t.Bill_Ref, \
     t.Bill_Price_Type, t.Bill_Type, t.Bill_Total, t.Bill_Pay, \
     t.Bill_Debt, t.Bill_Pay_CASH, t.Bill_Pay_CREDIT, t.Bill_Status, \
     t.Bill_by, t.Bill_Note";

#[async_trait]
impl MssqlChangeMapper for BillDebtHMirrorMapper {
    fn table(&self) -> &'static str {
        "HT_Bill_Debt_H"
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        &["Bill_No"]
    }

    fn select_sql(&self) -> &'static str {
        BILL_DEBT_H_SELECT_COLS
    }

    async fn apply(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        op: ChangeOp,
        row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        let row = row.ok_or_else(|| SyncError::Mapper {
            table: "HT_Bill_Debt_H",
            message: "row required".into(),
        })?;
        let bill_no = row
            .try_get_str("Bill_No")?
            .ok_or_else(|| SyncError::Mapper {
                table: "HT_Bill_Debt_H",
                message: "Bill_No NULL — should not happen post Phase 5.5b".into(),
            })?
            .to_string();
        match op {
            ChangeOp::Delete => {
                sqlx::query("DELETE FROM legacy_mirror.ht_bill_debt_h WHERE bill_no = $1")
                    .bind(&bill_no)
                    .execute(&mut **tx)
                    .await?;
            }
            ChangeOp::Insert | ChangeOp::Update => {
                sqlx::query(
                    "INSERT INTO legacy_mirror.ht_bill_debt_h \
                        (bill_no, bill_cust_id, bill_cust_name, bill_cust_address, \
                         bill_cust_tel, bill_cust_fax, bill_date, bill_ref, \
                         bill_price_type, bill_type, bill_total, bill_pay, \
                         bill_debt, bill_pay_cash, bill_pay_credit, bill_status, \
                         bill_by, bill_note, mirrored_at, mirror_source) \
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, \
                             $13, $14, $15, $16, $17, $18, now(), 'ct') \
                     ON CONFLICT (bill_no) DO UPDATE SET \
                        bill_cust_id      = EXCLUDED.bill_cust_id, \
                        bill_cust_name    = EXCLUDED.bill_cust_name, \
                        bill_cust_address = EXCLUDED.bill_cust_address, \
                        bill_cust_tel     = EXCLUDED.bill_cust_tel, \
                        bill_cust_fax     = EXCLUDED.bill_cust_fax, \
                        bill_date         = EXCLUDED.bill_date, \
                        bill_ref          = EXCLUDED.bill_ref, \
                        bill_price_type   = EXCLUDED.bill_price_type, \
                        bill_type         = EXCLUDED.bill_type, \
                        bill_total        = EXCLUDED.bill_total, \
                        bill_pay          = EXCLUDED.bill_pay, \
                        bill_debt         = EXCLUDED.bill_debt, \
                        bill_pay_cash     = EXCLUDED.bill_pay_cash, \
                        bill_pay_credit   = EXCLUDED.bill_pay_credit, \
                        bill_status       = EXCLUDED.bill_status, \
                        bill_by           = EXCLUDED.bill_by, \
                        bill_note         = EXCLUDED.bill_note, \
                        mirrored_at       = now(), \
                        mirror_source     = 'ct'",
                )
                .bind(&bill_no)
                .bind(row.try_get_str("Bill_Cust_ID")?)
                .bind(row.try_get_str("Bill_Cust_Name")?)
                .bind(row.try_get_str("Bill_Cust_Address")?)
                .bind(row.try_get_str("Bill_Cust_Tel")?)
                .bind(row.try_get_str("Bill_Cust_Fax")?)
                .bind(row.try_get_datetime("Bill_Date")?)
                .bind(row.try_get_str("Bill_Ref")?)
                .bind(row.try_get_str("Bill_Price_Type")?)
                .bind(row.try_get_str("Bill_Type")?)
                .bind(row.try_get_f64("Bill_Total")?)
                .bind(row.try_get_f64("Bill_Pay")?)
                .bind(row.try_get_f64("Bill_Debt")?)
                .bind(row.try_get_f64("Bill_Pay_CASH")?)
                .bind(row.try_get_f64("Bill_Pay_CREDIT")?)
                .bind(row.try_get_str("Bill_Status")?)
                .bind(row.try_get_str("Bill_by")?)
                .bind(row.try_get_str("Bill_Note")?)
                .execute(&mut **tx)
                .await?;
            }
        }
        Ok(None)
    }
}

// ─── HT_Bill_Debt_Ds ─────────────────────────────────────────────────

pub struct BillDebtDsMirrorMapper;

/// CT JOIN projection for `HT_Bill_Debt_Ds`. Held as a module-private
/// const so Track J1's projection-lock test can pin every column.
const BILL_DEBT_DS_SELECT_COLS: &str =
    "t.id, t.Bill_No, t.DS_ID, t.DS_NO, t.DS_NAME, \
     t.DS_UNIT, t.DS_NUM, t.DS_PRICE, t.DS_PRICE_TOTAL";

#[async_trait]
impl MssqlChangeMapper for BillDebtDsMirrorMapper {
    fn table(&self) -> &'static str {
        "HT_Bill_Debt_Ds"
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        BILL_DEBT_DS_SELECT_COLS
    }

    async fn apply(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        op: ChangeOp,
        row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        let row = row.ok_or_else(|| SyncError::Mapper {
            table: "HT_Bill_Debt_Ds",
            message: "row required".into(),
        })?;
        let id = row.try_get_i32("id")?.ok_or_else(|| SyncError::Mapper {
            table: "HT_Bill_Debt_Ds",
            message: "id NULL — identity column should never be NULL".into(),
        })?;
        match op {
            ChangeOp::Delete => {
                sqlx::query("DELETE FROM legacy_mirror.ht_bill_debt_ds WHERE id = $1")
                    .bind(id)
                    .execute(&mut **tx)
                    .await?;
            }
            ChangeOp::Insert | ChangeOp::Update => {
                sqlx::query(
                    "INSERT INTO legacy_mirror.ht_bill_debt_ds \
                        (id, bill_no, ds_id, ds_no, ds_name, \
                         ds_unit, ds_num, ds_price, ds_price_total, \
                         mirrored_at, mirror_source) \
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, now(), 'ct') \
                     ON CONFLICT (id) DO UPDATE SET \
                        bill_no        = EXCLUDED.bill_no, \
                        ds_id          = EXCLUDED.ds_id, \
                        ds_no          = EXCLUDED.ds_no, \
                        ds_name        = EXCLUDED.ds_name, \
                        ds_unit        = EXCLUDED.ds_unit, \
                        ds_num         = EXCLUDED.ds_num, \
                        ds_price       = EXCLUDED.ds_price, \
                        ds_price_total = EXCLUDED.ds_price_total, \
                        mirrored_at    = now(), \
                        mirror_source  = 'ct'",
                )
                .bind(id)
                .bind(row.try_get_str("Bill_No")?)
                .bind(row.try_get_i32("DS_ID")?)
                .bind(row.try_get_str("DS_NO")?)
                .bind(row.try_get_str("DS_NAME")?)
                .bind(row.try_get_str("DS_UNIT")?)
                .bind(row.try_get_f64("DS_NUM")?)
                .bind(row.try_get_f64("DS_PRICE")?)
                .bind(row.try_get_f64("DS_PRICE_TOTAL")?)
                .execute(&mut **tx)
                .await?;
            }
        }
        Ok(None)
    }
}

// ─── HT_Rooms_Cancel ─────────────────────────────────────────────────
// Track E1 / T2 HIGH-5 — CT was enabled in Phase 5 (migration 020) but
// no mapper existed. The dangling subscription kept CT retention
// growing forever without a consumer. This mapper closes the gap by
// mirroring each cancelled-room row into `legacy_mirror.ht_rooms_cancel`
// where the dashboard can surface the cancelled-room ledger alongside
// the other mirror tables.

pub struct RoomsCancelMirrorMapper;

/// CT JOIN projection for `HT_Rooms_Cancel`. Held as a module-private
/// const so Track J1's projection-lock test can pin every column.
const ROOMS_CANCEL_SELECT_COLS: &str =
    "t.id, t.room_no, t.cin_no, t.cancel_date, t.cancel_by, t.cancel_note";

#[async_trait]
impl MssqlChangeMapper for RoomsCancelMirrorMapper {
    fn table(&self) -> &'static str {
        "HT_Rooms_Cancel"
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        // Migration 020 (Phase 5) tightened `id INT NOT NULL` and
        // added PK_HT_Rooms_Cancel on it. All-lowercase columns per
        // the SCHEMA.sql dump.
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        ROOMS_CANCEL_SELECT_COLS
    }

    async fn apply(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        op: ChangeOp,
        row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        let row = row.ok_or_else(|| SyncError::Mapper {
            table: "HT_Rooms_Cancel",
            message: "row required for both I/U and D".into(),
        })?;
        let id = row.try_get_i32("id")?.ok_or_else(|| SyncError::Mapper {
            table: "HT_Rooms_Cancel",
            message: "id NULL — should not happen post Phase 5 (migration 020)".into(),
        })?;
        match op {
            ChangeOp::Delete => {
                sqlx::query("DELETE FROM legacy_mirror.ht_rooms_cancel WHERE id = $1")
                    .bind(id)
                    .execute(&mut **tx)
                    .await?;
            }
            ChangeOp::Insert | ChangeOp::Update => {
                sqlx::query(
                    "INSERT INTO legacy_mirror.ht_rooms_cancel \
                        (id, room_no, cin_no, cancel_date, cancel_by, cancel_note, \
                         mirrored_at, mirror_source) \
                     VALUES ($1, $2, $3, $4, $5, $6, now(), 'ct') \
                     ON CONFLICT (id) DO UPDATE SET \
                        room_no       = EXCLUDED.room_no, \
                        cin_no        = EXCLUDED.cin_no, \
                        cancel_date   = EXCLUDED.cancel_date, \
                        cancel_by     = EXCLUDED.cancel_by, \
                        cancel_note   = EXCLUDED.cancel_note, \
                        mirrored_at   = now(), \
                        mirror_source = 'ct'",
                )
                .bind(id)
                .bind(row.try_get_str("room_no")?)
                .bind(row.try_get_str("cin_no")?)
                .bind(row.try_get_datetime("cancel_date")?)
                .bind(row.try_get_str("cancel_by")?)
                .bind(row.try_get_str("cancel_note")?)
                .execute(&mut **tx)
                .await?;
            }
        }
        Ok(None)
    }
}

// ─── HT_Book_Pro ─────────────────────────────────────────────────────
// Phase 5/E2 — coexistence audit 2026-06-11 P2 gap closure.
//
// ## What `HT_Book_Pro` rows mean in iHOTEL
//
// Pre-booked products (food / drinks pre-ordered) attached to a
// booking. `COMPAT_CHEATSHEET.md` lines 725-730 ("Table: `HT_Book_Pro`
// (A) — pre-booked products"):
//
//   * PK `id int IDENTITY` — inserts omit `[id]` (FrmAddBook2.cs:3638).
//   * 9 columns: `id, B_NO (Book_ID), B_ROOM, B_NAME, B_UNIT, B_NUM,
//     B_PRICE, B_PRICE_TOTAL, B_PRO_ID`.
//   * Insert in a loop on FrmAddBook2 save (§3.4 step 3.5, cheatsheet
//     line 1222: `INSERT HT_Book_Pro (B_NO=Book_ID, B_ROOM, B_NAME,
//     B_UNIT, B_NUM, B_PRICE, B_PRICE_TOTAL, B_PRO_ID)`).
//   * Delete-on-edit `delete from HT_Book_Pro where [B_NO]='<id>'` —
//     part of FrmAddBook2.SAVE_EDIT's delete-then-reinsert rewrite
//     (§3.7 step 5, cheatsheet line 1260; same pattern as HT_Book_Date
//     / HT_Book_Ds per line 650). Expect D-rows followed by fresh
//     I-rows with NEW ids on every booking edit.
//   * `B_NO` joins to `HT_Book_H.Book_ID` (manual cascade, cheatsheet
//     line 1536). Read by FrmCheckIn (booking→check-in conversion) and
//     FormBookingInvoice (FEATURE_MAP lines 247, 507, 543).
//
// ## TODO — booking→check-in conversion writeback gap (do NOT fix here)
//
// iHOTEL's FrmCheckIn, when converting a booking ("เปลี่ยนเป็น
// Check-In", FEATURE_MAP §J3 lines 676-682), READS the booking's
// `HT_Book_Pro` lines into its product grid (FEATURE_MAP line 543:
// FrmCheckIn touches `HT_Book_Pro(R)` + `HT_CheckIn_Product(RW)`) and
// on save runs §3.1 Step 3 (cheatsheet lines 1124-1127) for each
// product row:
//
//   1. `INSERT HT_CheckIn_Product (Cin_No, Cin_Room_no, Cin_Pro_id,
//      …)` — omitting `[id]` (IDENTITY, cheatsheet lines 539-547);
//   2. the MANDATORY stock pairing `UPDATE HT_Products SET
//      Pro_Amt=Pro_Amt-<num> WHERE Pro_no='<p>'` (cheatsheet lines
//      560-566, "The new app MUST replicate this pairing");
//   3. `Insert_Pay(...)` when a payment amount accompanies the line.
//
// Our `writeback/recipes/checkin_to_booking.rs` emits NO
// `HT_CheckIn_Product` statements (it is owned by another track — see
// its `build_statements`), so a booking carrying `HT_Book_Pro` lines
// that is checked in via the NEW app still drops those charges on the
// iHOTEL side: the folio shows no product lines and stock is never
// decremented. What `checkin_to_booking.rs` would need to add, in the
// same recipe transaction:
//
//   * Load the booking's product lines (now available canonically from
//     `legacy_mirror.ht_book_pro WHERE b_no = <Book_ID>` — this
//     mapper's output — or directly from `HT_Book_Pro` inside the
//     recipe's MSSQL TX);
//   * Per line, emit the `HT_CheckIn_Product` INSERT (cheatsheet §3.1
//     Step 3 shape) + the paired `HT_Products.Pro_Amt` decrement;
//   * VERIFY FIRST against the FrmCheckIn decompile / a fresh capture
//     how `B_PRO_ID` (int) maps onto `HT_CheckIn_Product.Cin_Pro_id`
//     (varchar(250)) and `HT_Products.Pro_no` (varchar(50)) — the
//     cheatsheet does not pin that conversion, and guessing it would
//     corrupt the stock pairing key.
//
// Until that lands, this mapper at least makes iHOTEL-entered booking
// products VISIBLE to the new app so the conversion UI can warn.

pub struct BookProMirrorMapper;

/// CT JOIN projection for `HT_Book_Pro`. Held as a module-private
/// const so Track J1's projection-lock test can pin every column
/// against the authoritative HF Hotel schema dump
/// (`schema-baseline.txt` lines 179-187).
const BOOK_PRO_SELECT_COLS: &str =
    "t.id, t.B_NO, t.B_ROOM, t.B_NAME, t.B_UNIT, t.B_NUM, \
     t.B_PRICE, t.B_PRICE_TOTAL, t.B_PRO_ID";

#[async_trait]
impl MssqlChangeMapper for BookProMirrorMapper {
    fn table(&self) -> &'static str {
        "HT_Book_Pro"
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        // Migration legacy-mssql/023 added PK_HT_Book_Pro on the
        // IDENTITY `id` (already NOT NULL per the live baseline).
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        BOOK_PRO_SELECT_COLS
    }

    async fn apply(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        op: ChangeOp,
        row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        let row = row.ok_or_else(|| SyncError::Mapper {
            table: "HT_Book_Pro",
            message: "row required for both I/U and D".into(),
        })?;
        let id = row.try_get_i32("id")?.ok_or_else(|| SyncError::Mapper {
            table: "HT_Book_Pro",
            message: "id NULL — identity column should never be NULL".into(),
        })?;
        match op {
            ChangeOp::Delete => {
                // Fired on every FrmAddBook2 edit (delete-then-reinsert,
                // §3.7 step 5) as well as genuine removals — plain
                // mirror delete, idempotent on already-gone rows.
                sqlx::query("DELETE FROM legacy_mirror.ht_book_pro WHERE id = $1")
                    .bind(id)
                    .execute(&mut **tx)
                    .await?;
            }
            ChangeOp::Insert | ChangeOp::Update => {
                sqlx::query(
                    "INSERT INTO legacy_mirror.ht_book_pro \
                        (id, b_no, b_room, b_name, b_unit, b_num, \
                         b_price, b_price_total, b_pro_id, \
                         mirrored_at, mirror_source) \
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, now(), 'ct') \
                     ON CONFLICT (id) DO UPDATE SET \
                        b_no          = EXCLUDED.b_no, \
                        b_room        = EXCLUDED.b_room, \
                        b_name        = EXCLUDED.b_name, \
                        b_unit        = EXCLUDED.b_unit, \
                        b_num         = EXCLUDED.b_num, \
                        b_price       = EXCLUDED.b_price, \
                        b_price_total = EXCLUDED.b_price_total, \
                        b_pro_id      = EXCLUDED.b_pro_id, \
                        mirrored_at   = now(), \
                        mirror_source = 'ct'",
                )
                .bind(id)
                .bind(row.try_get_str("B_NO")?)
                .bind(row.try_get_str("B_ROOM")?)
                .bind(row.try_get_str("B_NAME")?)
                .bind(row.try_get_str("B_UNIT")?)
                .bind(row.try_get_f64("B_NUM")?)
                .bind(row.try_get_f64("B_PRICE")?)
                .bind(row.try_get_f64("B_PRICE_TOTAL")?)
                .bind(row.try_get_i32("B_PRO_ID")?)
                .execute(&mut **tx)
                .await?;
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Lock the trait surface for every mirror mapper — table name, PK
    /// column list, and that `select_sql` produces a non-empty
    /// projection (so the watcher actually JOINs to the underlying
    /// table instead of taking the no-op short-circuit at
    /// `bin/sync.rs::poll_table` step 2).
    #[test]
    fn mirror_mappers_declare_correct_table_pk_and_nonempty_projection() {
        let cases: &[(&dyn MssqlChangeMapper, &str, &[&str])] = &[
            (&CuponMirrorMapper, "HT_Cupon", &["cupon_no"]),
            (&CheckinProductMirrorMapper, "HT_CheckIn_Product", &["id"]),
            (&DepositMirrorMapper, "HT_Deposit", &["id"]),
            (&ChangedRoomMirrorMapper, "HT_Changed_Room", &["id"]),
            (&BillDebtHMirrorMapper, "HT_Bill_Debt_H", &["Bill_No"]),
            (&BillDebtDsMirrorMapper, "HT_Bill_Debt_Ds", &["id"]),
            (&RoomsCancelMirrorMapper, "HT_Rooms_Cancel", &["id"]),
            (&BookProMirrorMapper, "HT_Book_Pro", &["id"]),
        ];
        for (mapper, table, pk) in cases {
            assert_eq!(mapper.table(), *table, "table() mismatch");
            assert_eq!(mapper.primary_key_cols(), *pk, "PK mismatch on {table}");
            assert!(
                !mapper.select_sql().is_empty(),
                "{table} must have a non-empty SELECT projection"
            );
            // No coalesce_key — mirrors are flat-table per-row dispatch.
            assert!(mapper.coalesce_key(&dummy_row()).is_none());
        }
    }

    /// Track G4 / T4 HIGH-3 — `HT_Changed_Room` mapper must surface
    /// every column the canonical `ht_room_changes` projection needs.
    /// Refactor protection — dropping `Note` / `ToPrice` from the
    /// projection would silently de-populate the audit detail.
    #[test]
    fn changed_room_mapper_projects_columns_for_canonical_upsert() {
        let select = ChangedRoomMirrorMapper.select_sql();
        for col in &[
            "id",
            "cin_no",
            "room_before",
            "room_after",
            "change_date",
            "room_before_price",
            "Note",
            "ToPrice",
        ] {
            assert!(
                select.contains(col),
                "select_sql must project {col}; got: {select}"
            );
        }
    }

    /// Track E1 / T2 HIGH-5 — lock the projection columns for
    /// `HT_Rooms_Cancel` so a refactor can't silently drop one. The
    /// mirror PG table has 6 source columns + 2 bookkeeping columns.
    #[test]
    fn rooms_cancel_mapper_projects_six_source_columns() {
        let select = RoomsCancelMirrorMapper.select_sql();
        for col in &["id", "room_no", "cin_no", "cancel_date", "cancel_by", "cancel_note"] {
            assert!(
                select.contains(col),
                "select_sql must project {col}; got: {select}"
            );
        }
    }

    fn dummy_row() -> crate::sync::row::test_support::HashMapRow {
        crate::sync::row::test_support::HashMapRow::new("HT_Cupon")
    }

    // -------------------------------------------------------------------
    // Track J1 — projection-lock guards for every mirror mapper.
    //
    // Each test pins the per-mapper SELECT projection const against the
    // baseline schema dump for the underlying legacy table. A typo'd
    // column name fails the test at CI time, never reaching the watcher.
    // -------------------------------------------------------------------

    #[test]
    fn cupon_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(CUPON_SELECT_COLS, "HT_Cupon");
    }

    #[test]
    fn checkin_product_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(
            CHECKIN_PRODUCT_SELECT_COLS,
            "HT_CheckIn_Product"
        );
    }

    #[test]
    fn deposit_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(DEPOSIT_SELECT_COLS, "HT_Deposit");
    }

    #[test]
    fn changed_room_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(CHANGED_ROOM_SELECT_COLS, "HT_Changed_Room");
    }

    #[test]
    fn bill_debt_h_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(BILL_DEBT_H_SELECT_COLS, "HT_Bill_Debt_H");
    }

    #[test]
    fn bill_debt_ds_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(BILL_DEBT_DS_SELECT_COLS, "HT_Bill_Debt_Ds");
    }

    #[test]
    fn rooms_cancel_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(ROOMS_CANCEL_SELECT_COLS, "HT_Rooms_Cancel");
    }

    #[test]
    fn book_pro_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(BOOK_PRO_SELECT_COLS, "HT_Book_Pro");
    }

    // -------------------------------------------------------------------
    // Echo-before-stamp adoption (2026-07-28).
    //
    // The four behaviours below are properties of the adoption SQL's WHERE
    // clauses, so they're pinned the same way
    // `guest_registry::adoption_sql_targets_unstamped_non_primary_with_concat_name_match`
    // pins the identical fix: assert the clause that ENFORCES each
    // behaviour. `src/` unit tests in this crate are pure by construction
    // (no pool is reachable from a mapper unit test — see
    // `sync::resolve::tests`); runtime coverage of the apply() path lives
    // in `tests/test_sync_phase55c_mirror_apply.rs`.
    // -------------------------------------------------------------------

    /// Behaviour 1 — adoption CLAIMS an unstamped app-originated row
    /// instead of letting the legacy-origin INSERT create a duplicate.
    /// The statement stamps the legacy id ($1) onto a row selected by
    /// `sale_legacy_id IS NULL` + `source = 'canonical'` (the two markers
    /// of "our app wrote this, the writeback hasn't back-populated yet").
    #[test]
    fn pos_sale_adoption_claims_unstamped_app_originated_row() {
        let sql = ADOPT_UNSTAMPED_POS_SALE_SQL;
        assert!(sql.starts_with("UPDATE ht_pos_sales SET sale_legacy_id = $1"), "{sql}");
        assert!(sql.contains("sale_legacy_id IS NULL"), "{sql}");
        assert!(sql.contains("source = 'canonical'"), "{sql}");
        // Deterministic single-row claim, oldest first — FIFO writeback
        // drain order == MSSQL IDENTITY allocation order.
        assert!(sql.contains("ORDER BY sale_id LIMIT 1"), "{sql}");
    }

    /// Behaviour 1, room-change twin. `ht_room_changes` has no `source`
    /// column — `rc_legacy_id IS NULL` alone is the ownership marker,
    /// because this mapper stamps `rc_legacy_id` on every row it inserts.
    #[test]
    fn room_change_adoption_claims_unstamped_app_originated_row() {
        let sql = ADOPT_UNSTAMPED_ROOM_CHANGE_SQL;
        assert!(sql.starts_with("UPDATE ht_room_changes SET rc_legacy_id = $1"), "{sql}");
        assert!(sql.contains("rc_legacy_id IS NULL"), "{sql}");
        assert!(sql.contains("ORDER BY rc_id LIMIT 1"), "{sql}");
    }

    /// Behaviour 2 — a genuinely new legacy row still INSERTs. Both
    /// adoption statements are scoped UPDATEs whose target is a single
    /// subselect row, so a legacy row that matches no unstamped canonical
    /// row affects 0 rows and control falls through to the existing
    /// legacy-origin INSERT. Guard: neither statement may grow an INSERT
    /// or an unbounded (subselect-less) WHERE.
    #[test]
    fn adoption_statements_are_bounded_updates_never_inserts() {
        for sql in [ADOPT_UNSTAMPED_POS_SALE_SQL, ADOPT_UNSTAMPED_ROOM_CHANGE_SQL] {
            assert!(sql.starts_with("UPDATE "), "adoption must never INSERT: {sql}");
            assert!(!sql.contains("INSERT"), "adoption must never INSERT: {sql}");
            // The UPDATE is pinned to one row id chosen by a LIMIT 1
            // subselect — no match ⇒ rows_affected() == 0 ⇒ INSERT path.
            assert!(sql.contains("LIMIT 1)"), "{sql}");
        }
    }

    /// Behaviour 3 — tightness. Adoption must not hijack a row belonging
    /// to a DIFFERENT sale. Every field
    /// `writeback/recipes/pos_sale.rs::build_insert_statement` puts on the
    /// wire has to appear in the predicate, or a same-folio sale of a
    /// different product / qty / price / note / minute could be claimed.
    /// The two money columns are compared through `ROUND` at the recipe's
    /// own precision (`{:.3}` qty, `{:.2}` price) so the float round-trip
    /// through MSSQL can't produce a false miss.
    #[test]
    fn pos_sale_adoption_key_cannot_hijack_a_different_sale() {
        let sql = ADOPT_UNSTAMPED_POS_SALE_SQL;
        assert!(sql.contains("sale_cin_id = $2"), "{sql}");
        assert!(sql.contains("sale_product_id = $3"), "{sql}");
        assert!(sql.contains("sale_qty = ROUND($4::numeric, 3)"), "{sql}");
        assert!(sql.contains("sale_unit_price = ROUND($5::numeric, 2)"), "{sql}");
        assert!(sql.contains("COALESCE(sale_note, '') = COALESCE($6, '')"), "{sql}");
        // `format_legacy_datetime` renders Bangkok wall-clock truncated to
        // the second, so the canonical timestamptz must be compared in the
        // same space — NOT as a raw equality (which never matches, the
        // canonical column carries sub-second precision) and NOT in UTC.
        assert!(
            sql.contains(
                "date_trunc('second', sale_sold_at AT TIME ZONE 'Asia/Bangkok') = $7::timestamp"
            ),
            "{sql}"
        );
    }

    /// Behaviour 3, room-change twin. The structural triple (folio, from
    /// room, to room) plus the wall-clock second is already unique by
    /// construction; price / reason / to_price are pinned too so the key
    /// stays exact-content like the guest-registry model.
    #[test]
    fn room_change_adoption_key_cannot_hijack_a_different_move() {
        let sql = ADOPT_UNSTAMPED_ROOM_CHANGE_SQL;
        assert!(sql.contains("rc_cin_id = $2"), "{sql}");
        assert!(sql.contains("rc_from_room_id = $3"), "{sql}");
        assert!(sql.contains("rc_to_room_id = $4"), "{sql}");
        assert!(
            sql.contains("rc_room_before_price = ROUND(COALESCE($5::numeric, 0), 2)"),
            "{sql}"
        );
        assert!(sql.contains("COALESCE(rc_reason, '') = COALESCE($6, '')"), "{sql}");
        assert!(sql.contains("COALESCE(rc_to_price, '') = COALESCE($7, '')"), "{sql}");
        assert!(
            sql.contains(
                "date_trunc('second', rc_changed_at AT TIME ZONE 'Asia/Bangkok') = $8::timestamp"
            ),
            "{sql}"
        );
    }

    /// Behaviour 4 — an already-stamped legacy id takes the plain dedup
    /// path. The `NOT EXISTS` guard makes adoption a strict once-per-legacy-id
    /// operation: CT re-delivery, iHOTEL-side U-events and post-crash
    /// retries all find a canonical row already carrying the id and adopt
    /// nothing, so a second unstamped row can never be swept up.
    #[test]
    fn adoption_never_reclaims_an_already_stamped_legacy_id() {
        assert!(
            ADOPT_UNSTAMPED_POS_SALE_SQL
                .contains("NOT EXISTS (SELECT 1 FROM ht_pos_sales x WHERE x.sale_legacy_id = $1)"),
            "{ADOPT_UNSTAMPED_POS_SALE_SQL}"
        );
        assert!(
            ADOPT_UNSTAMPED_ROOM_CHANGE_SQL.contains(
                "NOT EXISTS (SELECT 1 FROM ht_room_changes x WHERE x.rc_legacy_id = $1)"
            ),
            "{ADOPT_UNSTAMPED_ROOM_CHANGE_SQL}"
        );
    }

    /// The FK-miss error path in `upsert_canonical_pos_sale` is the
    /// watermark-holding guarantee (June-3 silent-drop class) and adoption
    /// must stay orthogonal to it: adoption is expressed purely in
    /// canonical id space (`sale_cin_id` / `sale_product_id`), so it can
    /// only run AFTER the `ht_checkins`/`ht_products` join resolved. No
    /// adoption clause may reference a legacy business key, which would be
    /// a way to sneak past the unresolved-FK `Err`.
    #[test]
    fn adoption_never_bypasses_the_fk_resolution_error_path() {
        for sql in [ADOPT_UNSTAMPED_POS_SALE_SQL, ADOPT_UNSTAMPED_ROOM_CHANGE_SQL] {
            for legacy_key in &["legacy_cin_no", "prod_legacy_no", "room_no", "ht_products"] {
                assert!(
                    !sql.contains(legacy_key),
                    "adoption must key on resolved canonical ids only, found {legacy_key}: {sql}"
                );
            }
        }
    }

    // ─── Eager product mirror (#263) ─────────────────────────────────

    /// The FK resolve must stay an INNER JOIN. A LEFT JOIN would return
    /// `cin_id` with a NULL `prod_id`, which decodes as a resolved row
    /// and turns the watermark-holding `Err` into a silent skip — the
    /// exact June-3 class the eager mirror is layered ON TOP of, not a
    /// replacement for.
    #[test]
    fn pos_sale_fk_resolve_stays_an_inner_join_on_the_legacy_business_key() {
        let sql = POS_SALE_FK_RESOLVE_SQL;
        assert!(sql.contains("JOIN ht_products p ON p.prod_legacy_no = $2"), "{sql}");
        assert!(!sql.contains("LEFT JOIN"), "{sql}");
        assert!(sql.contains("WHERE c.legacy_cin_no = $1"), "{sql}");
        // The probe that discriminates the two sides of a miss keys on
        // the same legacy business key.
        assert!(
            CANONICAL_PRODUCT_PROBE_SQL.contains("prod_legacy_no = $1"),
            "{CANONICAL_PRODUCT_PROBE_SQL}"
        );
    }

    /// Branch table for a combined-resolve miss. Only a product miss
    /// with a live pool may spend an MSSQL round trip; a check-in miss
    /// has no eager path here and an unarmed mapper must behave exactly
    /// as it did pre-#263.
    #[test]
    fn fk_miss_only_takes_the_eager_path_for_a_missing_product_with_a_pool() {
        assert_eq!(
            classify_fk_miss(false, true),
            FkMissAction::EagerMirrorProduct
        );
        assert_eq!(
            classify_fk_miss(false, false),
            FkMissAction::HoldWatermark(FkMissStage::NoEagerAttempt)
        );
        // Product already canonical → the check-in is the missing side;
        // an MSSQL product fetch cannot help, so don't make one.
        assert_eq!(
            classify_fk_miss(true, true),
            FkMissAction::HoldWatermark(FkMissStage::CheckinNotMirrored)
        );
        assert_eq!(
            classify_fk_miss(true, false),
            FkMissAction::HoldWatermark(FkMissStage::CheckinNotMirrored)
        );
    }

    /// Eager hit: the product was mirrored and the retried resolve
    /// landed → no error, the sale applies in this same tick instead of
    /// waiting up to 15 minutes for `poll_products_once`.
    #[test]
    fn eager_hit_resolves_the_sale_without_an_error() {
        assert_eq!(classify_eager_outcome(true, true), None);
    }

    /// Genuine absence still errs. `HT_Products` has no row for this
    /// `Cin_Pro_id`, so nothing can resolve it — hold the watermark for
    /// a loud retry rather than dropping the canonical sale.
    #[test]
    fn genuine_product_absence_still_holds_the_watermark() {
        assert_eq!(
            classify_eager_outcome(false, false),
            Some(FkMissStage::ProductAbsentInLegacy)
        );
        // Mirrored, but the join still misses → the check-in side is
        // missing too. Also a hold, with its own reason.
        assert_eq!(
            classify_eager_outcome(true, false),
            Some(FkMissStage::StillUnresolvedAfterEagerMirror)
        );
    }

    /// Every give-up stage must produce a `SyncError::Mapper` on
    /// `HT_CheckIn_Product` (which is what makes the watcher set
    /// `errored=true` and hold the watermark), keep the pre-#263
    /// greppable prefix, and carry a REASON distinct from every other
    /// stage so a production log line identifies the branch taken.
    #[test]
    fn every_fk_miss_stage_errors_with_a_distinct_greppable_reason() {
        let stages = [
            FkMissStage::NoEagerAttempt,
            FkMissStage::CheckinNotMirrored,
            FkMissStage::ProductAbsentInLegacy,
            FkMissStage::StillUnresolvedAfterEagerMirror,
        ];
        let mut reasons: Vec<&str> = Vec::new();
        for stage in stages {
            let err = pos_sale_fk_error(stage, 4242, "CH26-001061", "B-001");
            match &err {
                SyncError::Mapper { table, message } => {
                    assert_eq!(*table, "HT_CheckIn_Product");
                    assert!(
                        message.starts_with(
                            "canonical pos-sale FK unresolvable for legacy id=4242"
                        ),
                        "{message}"
                    );
                    assert!(message.contains("cin_no=CH26-001061"), "{message}");
                    assert!(message.contains("pro_id=B-001"), "{message}");
                    assert!(
                        message.contains("holding watermark for loud retry"),
                        "{message}"
                    );
                }
                other => panic!("expected SyncError::Mapper, got {other:?}"),
            }
            reasons.push(stage.reason());
        }
        let unique: std::collections::HashSet<&str> = reasons.iter().copied().collect();
        assert_eq!(
            unique.len(),
            reasons.len(),
            "each stage needs its own reason text: {reasons:?}"
        );
    }

    /// The eager fetch is composed from the products poll's own SELECT,
    /// so adding a column to `products::POLL_COLUMNS` reaches both paths
    /// at once. Plain `'…'` literal — `N'…'` corrupts TIS-620 in the
    /// legacy DB.
    #[test]
    fn eager_product_fetch_sql_reuses_the_products_poll_projection() {
        let sql = product_eager_fetch_sql("B-001");
        assert!(
            sql.starts_with(&crate::sync::mappers::products::poll_select_sql()),
            "{sql}"
        );
        assert!(sql.ends_with(" WHERE Pro_no = 'B-001'"), "{sql}");
        for col in crate::sync::mappers::products::POLL_COLUMNS {
            assert!(sql.contains(col), "eager fetch must select `{col}`: {sql}");
        }
        assert!(!sql.contains("N'"), "N'…' corrupts TIS-620: {sql}");
    }

    /// Inline-quoted WHERE value — an embedded apostrophe must double,
    /// not terminate the literal.
    #[test]
    fn eager_product_fetch_sql_doubles_embedded_quotes() {
        assert_eq!(sql_quote_inline("O'Brien"), "'O''Brien'");
        let sql = product_eager_fetch_sql("O'Brien");
        assert!(sql.ends_with(" WHERE Pro_no = 'O''Brien'"), "{sql}");
    }

    /// The `Pro_Name` clamp is load-bearing on the eager path too:
    /// legacy `varchar(500)` counts TIS-620 BYTES while canonical
    /// `VARCHAR(250)` counts CHARACTERS, so a long Thai name overflows
    /// and would abort the tick transaction. The eager path gets the
    /// clamp for free by projecting with `products::project` and writing
    /// with `products::upsert_product` — this pins both halves: the
    /// projection carries the over-long name through unchanged, and the
    /// module never hand-rolls an `ht_products` INSERT that would bypass
    /// `PROD_NAME_MAX_CHARS`.
    #[test]
    fn eager_path_applies_the_pro_name_char_clamp_via_products_upsert() {
        use crate::sync::mappers::products::{
            clamp_chars, project, PROD_NAME_MAX_CHARS, TABLE as HT_PRODUCTS,
        };
        use crate::sync::row::test_support::MockValue;

        // Exactly the row a stubbed eager fetch supplies.
        let row = crate::sync::row::test_support::HashMapRow::new(HT_PRODUCTS)
            .with("Pro_no", MockValue::Str("B-001".into()))
            .with("Pro_Name", MockValue::Str("ก".repeat(400)))
            .with("Pro_Unit", MockValue::Str("ขวด".into()))
            .with("Pro_PriceA", MockValue::Decimal(25.0))
            .with("Pro_Amt", MockValue::Decimal(12.0))
            .with("Pro_Type", MockValue::Str("B".into()));

        let p = project(&row).expect("eager row must project");
        assert_eq!(p.prod_legacy_no, "B-001");
        assert_eq!(p.prod_name.chars().count(), 400);
        // …and the write path clamps it on a char boundary.
        let clamped = clamp_chars(&p.prod_name, PROD_NAME_MAX_CHARS);
        assert_eq!(clamped.chars().count(), PROD_NAME_MAX_CHARS);
        assert!(clamped.chars().all(|c| c == 'ก'));

        // Source pin: the canonical product write must stay delegated to
        // `products::upsert_product`, which owns the clamp AND leaves
        // operator-owned `prod_active` alone.
        // Slice off the test module. The marker is the full `mod tests`
        // header, not a bare `#[cfg(test)]` — the docs above legitimately
        // name that attribute in prose.
        let src = include_str!("mirror.rs");
        let marker = src
            .find("\n#[cfg(test)]\nmod tests {")
            .expect("test module marker");
        let code = &src[..marker];
        assert!(
            code.contains("products::upsert_product(tx, &projection)"),
            "the eager mirror must write through products::upsert_product"
        );
        assert!(
            !code.contains("INSERT INTO ht_products"),
            "a hand-rolled ht_products INSERT here would bypass the \
             Pro_Name char-clamp and drift from the poll's column set"
        );
    }

    /// Phase 5/E2 — lock the projection columns for `HT_Book_Pro` so a
    /// refactor can't silently drop one. The mirror PG table has 9
    /// source columns + 2 bookkeeping columns (migration pg/056).
    #[test]
    fn book_pro_mapper_projects_nine_source_columns() {
        let select = BookProMirrorMapper.select_sql();
        for col in &[
            "id",
            "B_NO",
            "B_ROOM",
            "B_NAME",
            "B_UNIT",
            "B_NUM",
            "B_PRICE",
            "B_PRICE_TOTAL",
            "B_PRO_ID",
        ] {
            assert!(
                select.contains(col),
                "select_sql must project {col}; got: {select}"
            );
        }
    }
}
