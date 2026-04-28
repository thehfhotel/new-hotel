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

use async_trait::async_trait;

use crate::outbox::event::DomainEvent;
use crate::sync::change_op::ChangeOp;
use crate::sync::mapper::MssqlChangeMapper;
use crate::sync::row::MappableRow;
use crate::sync::SyncError;

// ─── HT_Cupon ────────────────────────────────────────────────────────

pub struct CuponMirrorMapper;

#[async_trait]
impl MssqlChangeMapper for CuponMirrorMapper {
    fn table(&self) -> &'static str {
        "HT_Cupon"
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        &["cupon_no"]
    }

    fn select_sql(&self) -> &'static str {
        "t.cupon_no, t.cupon_cin_no, t.cupon_cin_room, t.cupon_date, \
         t.cupon_gen_date, t.cupon_by, t.cupon_print"
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
        Ok(None)
    }
}

// ─── HT_CheckIn_Product ──────────────────────────────────────────────

pub struct CheckinProductMirrorMapper;

#[async_trait]
impl MssqlChangeMapper for CheckinProductMirrorMapper {
    fn table(&self) -> &'static str {
        "HT_CheckIn_Product"
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        "t.id, t.Cin_No, t.Cin_Room_no, t.Cin_Ds_date, t.Cin_Pro_id, \
         t.Cin_Pro_name, t.Cin_Pro_Unit, t.Cin_Pro_num, t.Cin_Pro_price, \
         t.Cin_Pro_priceTotal, t.Cin_Pro_pay, t.Cin_Pro_note"
    }

    async fn apply(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
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
            }
            ChangeOp::Insert | ChangeOp::Update => {
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
                .bind(row.try_get_str("Cin_No")?)
                .bind(row.try_get_str("Cin_Room_no")?)
                .bind(row.try_get_datetime("Cin_Ds_date")?)
                .bind(row.try_get_str("Cin_Pro_id")?)
                .bind(row.try_get_str("Cin_Pro_name")?)
                .bind(row.try_get_str("Cin_Pro_Unit")?)
                .bind(row.try_get_f64("Cin_Pro_num")?)
                .bind(row.try_get_f64("Cin_Pro_price")?)
                .bind(row.try_get_f64("Cin_Pro_priceTotal")?)
                .bind(row.try_get_f64("Cin_Pro_pay")?)
                .bind(row.try_get_str("Cin_Pro_note")?)
                .execute(&mut **tx)
                .await?;
            }
        }
        Ok(None)
    }
}

// ─── HT_Deposit ──────────────────────────────────────────────────────

pub struct DepositMirrorMapper;

#[async_trait]
impl MssqlChangeMapper for DepositMirrorMapper {
    fn table(&self) -> &'static str {
        "HT_Deposit"
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        "t.id, t.Dep_no, t.Dep_Date, t.Dep_Room, t.Dep_Name, \
         t.Dep_Price, t.Dep_Status, t.Dep_ref"
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

#[async_trait]
impl MssqlChangeMapper for ChangedRoomMirrorMapper {
    fn table(&self) -> &'static str {
        "HT_Changed_Room"
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        "t.id, t.cin_no, t.room_before, t.room_after, t.change_date, \
         t.room_before_price, t.Note, t.ToPrice"
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
            }
            ChangeOp::Insert | ChangeOp::Update => {
                let cin_no = row
                    .try_get_str("cin_no")?
                    .ok_or_else(|| SyncError::Mapper {
                        table: "HT_Changed_Room",
                        message: "cin_no NULL — schema declares NOT NULL".into(),
                    })?;
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
                .bind(row.try_get_str("room_before")?)
                .bind(row.try_get_str("room_after")?)
                .bind(row.try_get_datetime("change_date")?)
                .bind(row.try_get_f64("room_before_price")?)
                .bind(row.try_get_str("Note")?)
                .bind(row.try_get_str("ToPrice")?)
                .execute(&mut **tx)
                .await?;
            }
        }
        Ok(None)
    }
}

// ─── HT_Bill_Debt_H ──────────────────────────────────────────────────

pub struct BillDebtHMirrorMapper;

#[async_trait]
impl MssqlChangeMapper for BillDebtHMirrorMapper {
    fn table(&self) -> &'static str {
        "HT_Bill_Debt_H"
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        &["Bill_No"]
    }

    fn select_sql(&self) -> &'static str {
        "t.Bill_No, t.Bill_Cust_ID, t.Bill_Cust_Name, t.Bill_Cust_Address, \
         t.Bill_Cust_Tel, t.Bill_Cust_Fax, t.Bill_Date, t.Bill_Ref, \
         t.Bill_Price_Type, t.Bill_Type, t.Bill_Total, t.Bill_Pay, \
         t.Bill_Debt, t.Bill_Pay_CASH, t.Bill_Pay_CREDIT, t.Bill_Status, \
         t.Bill_by, t.Bill_Note"
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

#[async_trait]
impl MssqlChangeMapper for BillDebtDsMirrorMapper {
    fn table(&self) -> &'static str {
        "HT_Bill_Debt_Ds"
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        "t.id, t.Bill_No, t.DS_ID, t.DS_NO, t.DS_NAME, \
         t.DS_UNIT, t.DS_NUM, t.DS_PRICE, t.DS_PRICE_TOTAL"
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

    fn dummy_row() -> crate::sync::row::test_support::HashMapRow {
        crate::sync::row::test_support::HashMapRow::new("HT_Cupon")
    }
}
