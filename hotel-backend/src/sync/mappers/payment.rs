//! Payment + receipt CT mappers (Phase 5.4).
//!
//! Two mappers ship in this module:
//!
//! * [`PaymentMapper`] — `HT_CheckIn_Pay`. Tender events on a check-in.
//!   Coalesces by `Cin_No` so multiple payment rows for the same stay
//!   trigger a single check-in aggregate sweep (which re-projects the
//!   `cin_paid_amount` total).
//! * [`ReceiptMapper`] — `HT_Receipt_H`. Standalone receipt header (a
//!   receipt may or may not reference a check-in via `Receipt_ref`).
//!   Per-row dispatch — no coalescing key. UPSERTs into `ht_payments`
//!   and emits `PaymentReceived`.
//!
//! ## Why two mappers?
//!
//! `HT_CheckIn_Pay` is the legacy app's *payment ledger*; every row
//! belongs to exactly one `HT_CheckIn_H.Cin_no`. We don't mirror it
//! into a separate PG table (the canonical source of paid totals is
//! `ht_checkins.cin_paid_amount`, kept synchronised by the check-in
//! aggregate's projection of `Total_Price_Pay`). Instead the payment
//! mapper just re-triggers the check-in aggregate sweep so totals stay
//! current.
//!
//! `HT_Receipt_H` is the receipt artefact (a printed/numbered document).
//! Each row maps cleanly to one `ht_payments` row in our schema. The
//! receipt amount is split via [`vat_inclusive_split`] for parity with
//! the writeback recipe (`writeback/recipes/payment.rs`).

use async_trait::async_trait;
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::db::DbPool;
use crate::outbox::event::{DomainEvent, EventSource};
use crate::service::ids::{aggregate_uuid, AggregateKind};
use crate::sync::change_op::ChangeOp;
use crate::sync::mapper::MssqlChangeMapper;
use crate::sync::mappers::checkin::apply_checkin_aggregate;
use crate::sync::parent_loader::load_checkin_aggregate;
use crate::sync::resolve;
use crate::sync::row::MappableRow;
use crate::sync::SyncError;

const HT_CHECKIN_PAY: &str = "HT_CheckIn_Pay";
const HT_RECEIPT_H: &str = "HT_Receipt_H";

// =============================================================================
// HT_CheckIn_Pay — payment ledger mapper. Delegates to the check-in
// aggregate sweep so `cin_paid_amount` stays in sync.
// =============================================================================

/// CT mapper for `HT_CheckIn_Pay`. Always belongs to a check-in;
/// coalesces by `Cin_No` so multi-row payments roll into a single
/// aggregate apply.
pub struct PaymentMapper;

// Track C — T2 CRIT-2 (`docs/coexistence/audit-2026-05-13.md`):
// `Cin_Status` projected so the CT pipeline carries the cancellation
// marker (`'ยกเลิก'`) down to the check-in aggregate sweep. The legacy
// app cascades a folio cancel into `HT_CheckIn_Pay` via
// `update HT_CheckIn_Pay set cin_status='ยกเลิก' where cin_no=…`
// (COMPAT_CHEATSHEET line 531). Without the projection the sync layer
// silently treats the cancelled rows as active and over-counts
// `cin_paid_amount`. Verified column shape per COMPAT_CHEATSHEET
// line 492 (`Cin_Status varchar(50) NOT NULL DEFAULT '1'`).
//
// The two additional tender columns (`Cin_Pay_Free`, `Cin_Pay_web`)
// round out the canonical sum so a future aggregate-by-tender
// projection has every contributor available without another CT
// pipeline change. Order matches the canonical writeback-recipe order
// (Cash + Credit + Free + Tran + web) so downstream code reads
// left-to-right in the same sequence as the legacy invariant
// (COMPAT_CHEATSHEET line 534).
const PAYMENT_SELECT_COLS: &str = "t.id, t.Cin_No, t.Cin_Pay_Cash, t.Cin_Pay_Credit, \
    t.Cin_Pay_Free, t.Cin_Pay_Tran, t.Cin_Pay_web, t.Pay_No, t.Cin_Status";

#[async_trait]
impl MssqlChangeMapper for PaymentMapper {
    fn table(&self) -> &'static str {
        HT_CHECKIN_PAY
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        // `id` is IDENTITY on `HT_CheckIn_Pay` (cheatsheet §3.4 — the
        // only checkin sub-table whose `id` is auto-allocated).
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        PAYMENT_SELECT_COLS
    }

    async fn apply(
        &self,
        _tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        _op: ChangeOp,
        _row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        // Watcher coalesces this row into the check-in aggregate sweep
        // via [`apply_payment_aggregate`]. See CheckInHeaderMapper::apply
        // for the full rationale.
        Ok(None)
    }

    fn coalesce_key(&self, row: &dyn MappableRow) -> Option<String> {
        // I/U rows: the joined `Cin_No` is present.
        // D rows: NULL → return None and rely on a sibling check-in
        //         header CT row to drive the aggregate sweep.
        row.try_get_str("Cin_No").ok().flatten().map(str::to_string)
    }
}

/// Re-sync the check-in aggregate that this payment row belongs to.
/// Thin wrapper over [`apply_checkin_aggregate`]; the watcher calls
/// this from the coalesced dispatch path when the coalesce key came
/// from a `HT_CheckIn_Pay` row.
///
/// Returns whatever the check-in aggregate sweep returns — typically
/// `Ok(None)` (idempotent skip on `cin_paid_amount` change) or a
/// `CheckInCreated` re-emission with the updated paid amount.
pub async fn apply_payment_aggregate(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    mssql: &DbPool,
    cin_no: &str,
) -> Result<Option<DomainEvent>, SyncError> {
    let aggregate = load_checkin_aggregate(mssql, cin_no).await?;
    apply_checkin_aggregate(tx, Some(mssql), &aggregate, cin_no).await
}

// =============================================================================
// HT_Receipt_H — receipt header mapper. UPSERTs into ht_payments and
// emits PaymentReceived.
// =============================================================================

/// CT mapper for `HT_Receipt_H`. Receipts are append-only in the legacy
/// app (cheatsheet §3.9 "Receipts are never deleted on check-out") so
/// the I path dominates; U is the cancel path (status_name='ยกเลิก').
pub struct ReceiptMapper;

const RECEIPT_SELECT_COLS: &str =
    "t.id, t.Receipt_no, t.Receipt_Date, t.Receipt_Total, t.Receipt_ref, \
     t.Receipt_c_no, t.status_name";

#[async_trait]
impl MssqlChangeMapper for ReceiptMapper {
    fn table(&self) -> &'static str {
        HT_RECEIPT_H
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        // `id` is INT NOT NULL (allocated via TABLOCKX MAX+1 — NOT
        // IDENTITY per cheatsheet §3.6 / writeback/recipes/payment.rs).
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        RECEIPT_SELECT_COLS
    }

    async fn apply(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        op: ChangeOp,
        row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        match op {
            ChangeOp::Insert | ChangeOp::Update => {
                let row = row.ok_or_else(|| SyncError::Mapper {
                    table: HT_RECEIPT_H,
                    message: "I/U operation requires joined row".into(),
                })?;
                apply_receipt_upsert(tx, row).await
            }
            ChangeOp::Delete => {
                // Receipts are never hard-deleted in the legacy app
                // (cheatsheet §3.9). If a D arrives we log + skip.
                tracing::warn!(
                    table = HT_RECEIPT_H,
                    "HT_Receipt_H D event received — ignored \
                     (legacy app uses status_name='ยกเลิก' for cancel)"
                );
                Ok(None)
            }
        }
    }

    /// Receipts are NOT aggregated — each row maps 1:1 to an
    /// `ht_payments` row. Return `None` so the watcher routes through
    /// the per-row dispatch path.
    fn coalesce_key(&self, _row: &dyn MappableRow) -> Option<String> {
        None
    }
}

#[derive(Debug, Clone, PartialEq)]
struct ReceiptProjection {
    legacy_id: i32,
    receipt_no: String,
    receipt_date: Option<NaiveDateTime>,
    /// Gross amount captured in `Receipt_Total`. Stored verbatim into
    /// `ht_payments.pay_amount` (the legacy app's gross is our gross —
    /// the VAT split is reporting metadata, not an alternate total).
    receipt_total: f64,
    /// `Receipt_ref` carries the originating `Cin_no` when the receipt
    /// is for a check-in payment. Empty string for sales without a
    /// check-in (rare — the spike captured one such row).
    legacy_cin_no: Option<String>,
    /// `Receipt_c_no` carries the customer number (`C\d{5}`).
    legacy_cust_no: Option<String>,
    /// Cancellation marker. `'ปกติ'` is normal; `'ยกเลิก'` is the
    /// legacy cancel literal.
    status_name: Option<String>,
}

fn project_receipt(row: &dyn MappableRow) -> Result<ReceiptProjection, SyncError> {
    let legacy_id = row.try_get_i32("id")?.ok_or_else(|| SyncError::Mapper {
        table: HT_RECEIPT_H,
        message: "id is NULL — required PK".into(),
    })?;
    let receipt_no = row
        .try_get_str("Receipt_no")?
        .ok_or_else(|| SyncError::Mapper {
            table: HT_RECEIPT_H,
            message: "Receipt_no is NULL — required business key".into(),
        })?
        .to_string();
    let receipt_total = row
        .try_get_decimal("Receipt_Total")?
        .ok_or_else(|| SyncError::Mapper {
            table: HT_RECEIPT_H,
            message: "Receipt_Total is NULL".into(),
        })?;
    let legacy_cin_no = row
        .try_get_str("Receipt_ref")?
        .map(str::to_string)
        .filter(|s| !s.is_empty());
    let legacy_cust_no = row
        .try_get_str("Receipt_c_no")?
        .map(str::to_string)
        .filter(|s| !s.is_empty());
    let receipt_date = row.try_get_datetime("Receipt_Date")?;
    let status_name = row.try_get_str("status_name")?.map(str::to_string);

    Ok(ReceiptProjection {
        legacy_id,
        receipt_no,
        receipt_date,
        receipt_total,
        legacy_cin_no,
        legacy_cust_no,
        status_name,
    })
}

async fn apply_receipt_upsert(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row: &dyn MappableRow,
) -> Result<Option<DomainEvent>, SyncError> {
    let p = project_receipt(row)?;

    // No `Receipt_ref` → a sale without a check-in. Out of scope today
    // (`ht_payments.pay_cin_id` is NOT NULL, there's nowhere to land
    // it) — a deliberate, logged skip, NOT an FK-defer: no amount of
    // retrying changes the receipt's shape.
    let Some(legacy_cin_no) = p.legacy_cin_no.as_deref() else {
        tracing::info!(
            receipt_no = %p.receipt_no,
            "ht_payments apply skipped: receipt carries no Receipt_ref \
             (sale without check-in — out of scope, deliberate)"
        );
        return Ok(None);
    };

    // Resolve the parent check-in. A miss MUST error — receipts have NO
    // re-fire source: `HT_Receipt_H` rows are written once and never
    // touched again (append-only per cheatsheet §3.9, except the cancel
    // status flip), so once this CT row is consumed with the watermark
    // advancing, the payment is silently lost forever. Erroring makes
    // the watcher hold the watermark and retry loudly until the parent
    // check-in lands (its own table poll usually catches up within a
    // tick). Pre-2026-06-11 this path returned `Ok(None)` — the
    // June-3 silent-drop class.
    let (cin_id, cin_aggregate_id) =
        match resolve::resolve_checkin_id(tx, Some(legacy_cin_no)).await? {
            Some((cid, agg)) => (cid, agg),
            None => {
                return Err(SyncError::Mapper {
                    table: HT_RECEIPT_H,
                    message: format!(
                        "parent check-in FK unresolvable for receipt_no={} \
                         legacy_cin_no={legacy_cin_no} — holding watermark for \
                         loud retry (receipts have no re-fire source)",
                        p.receipt_no
                    ),
                });
            }
        };

    // Cancel path — UPSERT with pay_voided=true and emit no event.
    let is_cancelled = p.status_name.as_deref() == Some("ยกเลิก");

    // UPSERT on (pay_cin_id, pay_reference) — pay_reference carries the
    // legacy Receipt_no and is unique per legacy receipt sequence.
    let now = chrono::Utc::now().naive_utc();
    let pay_date = p.receipt_date.unwrap_or(now);

    // Look up by (cin_id, receipt_no) — the receipt_no is unique
    // within the legacy app. We probe pay_reference first.
    let existing_pay_id: Option<i32> = sqlx::query_scalar(
        "SELECT pay_id FROM ht_payments \
          WHERE pay_cin_id = $1 AND pay_reference = $2 \
          LIMIT 1",
    )
    .bind(cin_id)
    .bind(&p.receipt_no)
    .fetch_optional(&mut **tx)
    .await?;

    let pay_id = match existing_pay_id {
        Some(id) => {
            sqlx::query(
                "UPDATE ht_payments \
                    SET pay_amount    = $1::float8, \
                        pay_date      = $2, \
                        pay_voided    = $3, \
                        pay_voided_at = CASE WHEN $3 THEN NOW() ELSE NULL END \
                  WHERE pay_id = $4",
            )
            .bind(p.receipt_total)
            .bind(pay_date)
            .bind(is_cancelled)
            .bind(id)
            .execute(&mut **tx)
            .await?;
            id
        }
        None => {
            // We do not know the actual tender method from `HT_Receipt_H`
            // alone — that's recorded in the matching `HT_CheckIn_Pay`
            // row. Default to 'cash' to keep the column NOT NULL; the
            // payment mapper's aggregate sweep keeps `cin_paid_amount`
            // accurate independently.
            let row: (i32,) = sqlx::query_as(
                "INSERT INTO ht_payments \
                     (pay_cin_id, pay_amount, pay_method, pay_reference, pay_date, \
                      pay_voided, pay_voided_at, pay_created_by) \
                 VALUES ($1, $2::float8, 'cash', $3, $4, $5, \
                         CASE WHEN $5 THEN NOW() ELSE NULL END, 'legacy_app') \
                 RETURNING pay_id",
            )
            .bind(cin_id)
            .bind(p.receipt_total)
            .bind(&p.receipt_no)
            .bind(pay_date)
            .bind(is_cancelled)
            .fetch_one(&mut **tx)
            .await?;
            row.0
        }
    };

    if is_cancelled {
        // No event for cancellation — the check-in aggregate sweep
        // (triggered by the matching HT_CheckIn_Pay row's status flip)
        // surfaces the change as a `CheckInCreated` re-emission with
        // the updated `cin_paid_amount`.
        return Ok(None);
    }

    // Resolve the check-in's aggregate UUID for the event payload. The
    // `PaymentReceived` event carries `check_in_id` — see
    // `outbox/event.rs::PaymentReceived`.
    let check_in_id = cin_aggregate_id
        .unwrap_or_else(|| aggregate_uuid(AggregateKind::CheckIn, cin_id));

    let _ = p.legacy_cust_no; // surfaced for future audit/event enrichment.
    let _ = p.legacy_id;
    let _ = pay_id;

    Ok(Some(build_payment_event(check_in_id, p.receipt_total)))
}

fn build_payment_event(check_in_id: Uuid, amount_baht: f64) -> DomainEvent {
    use crate::domain::payment::PaymentMethod;
    use crate::domain::shared::Money;

    DomainEvent::PaymentReceived {
        check_in_id,
        amount: Money::from_satang((amount_baht * 100.0).round() as i64),
        // Default to Cash — see comment in apply_receipt_upsert about
        // method resolution requiring the matching HT_CheckIn_Pay row
        // (which the watcher processes through a separate code path).
        method: PaymentMethod::Cash,
        source: EventSource::LegacyApp {
            detected_at: chrono::Utc::now(),
        },
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::row::test_support::{HashMapRow, MockValue};

    fn payment_row(cin_no: &str, cash: f64) -> HashMapRow {
        HashMapRow::new(HT_CHECKIN_PAY)
            .with("id", MockValue::I32(50001))
            .with("Cin_No", MockValue::Str(cin_no.into()))
            .with("Cin_Pay_Cash", MockValue::Decimal(cash))
            .with("Cin_Pay_Credit", MockValue::Decimal(0.0))
            .with("Cin_Pay_Tran", MockValue::Decimal(0.0))
            .with("Pay_No", MockValue::Str("R2604-0250".into()))
    }

    fn receipt_row(id: i32, receipt_no: &str, cin_no: &str, total: f64) -> HashMapRow {
        HashMapRow::new(HT_RECEIPT_H)
            .with("id", MockValue::I32(id))
            .with("Receipt_no", MockValue::Str(receipt_no.into()))
            .with(
                "Receipt_Date",
                MockValue::DateTime(
                    chrono::NaiveDate::from_ymd_opt(2026, 4, 26)
                        .unwrap()
                        .and_hms_opt(15, 0, 0)
                        .unwrap(),
                ),
            )
            .with("Receipt_Total", MockValue::Decimal(total))
            .with("Receipt_ref", MockValue::Str(cin_no.into()))
            .with("Receipt_c_no", MockValue::Str("C21607".into()))
            .with("status_name", MockValue::Str("ปกติ".into()))
    }

    // ----- mapper metadata -----------------------------------------------

    #[test]
    fn payment_mapper_metadata_is_correct() {
        let m = PaymentMapper;
        assert_eq!(m.table(), "HT_CheckIn_Pay");
        assert_eq!(m.primary_key_cols(), &["id"]);
        // CRITICAL: capital N. Locks against accidental rename.
        assert!(m.select_sql().contains("t.Cin_No"));
        assert!(m.select_sql().contains("Cin_Pay_Cash"));
    }

    /// Track C — T2 CRIT-2 (`docs/coexistence/audit-2026-05-13.md`):
    /// the payment mapper's SELECT projection must include `Cin_Status`
    /// so the CT pipeline carries the cancellation marker
    /// (`'ยกเลิก'`) into the check-in aggregate sweep. Without it the
    /// cascade `update HT_CheckIn_Pay set cin_status='ยกเลิก' where
    /// cin_no=…` (COMPAT_CHEATSHEET line 531) is silently dropped at
    /// the sync layer and `ht_checkins.cin_paid_amount` over-counts.
    #[test]
    fn projects_cin_pay_status() {
        let m = PaymentMapper;
        let select = m.select_sql();
        assert!(
            select.contains("t.Cin_Status"),
            "PAYMENT_SELECT_COLS must project Cin_Status; got: {select}"
        );
    }

    /// Track C — T2 CRIT-2: the projection rounds out every tender
    /// column so an aggregate-by-tender computation can be derived
    /// from CT-delivered rows without another pipeline change. The
    /// canonical writeback recipe sums Cash + Credit + Free + Tran +
    /// web (COMPAT_CHEATSHEET line 534).
    #[test]
    fn projects_every_tender_column() {
        let m = PaymentMapper;
        let select = m.select_sql();
        for col in [
            "Cin_Pay_Cash",
            "Cin_Pay_Credit",
            "Cin_Pay_Free",
            "Cin_Pay_Tran",
            "Cin_Pay_web",
        ] {
            assert!(
                select.contains(col),
                "PAYMENT_SELECT_COLS must project {col}; got: {select}"
            );
        }
    }

    #[test]
    fn receipt_mapper_metadata_is_correct() {
        let m = ReceiptMapper;
        assert_eq!(m.table(), "HT_Receipt_H");
        assert_eq!(m.primary_key_cols(), &["id"]);
        assert!(m.select_sql().contains("Receipt_no"));
        assert!(m.select_sql().contains("Receipt_Total"));
    }

    // ----- coalesce_key --------------------------------------------------

    #[test]
    fn payment_mapper_coalesces_on_cin_no_when_present() {
        let m = PaymentMapper;
        let row = payment_row("CH26-005228", 890.0);
        assert_eq!(m.coalesce_key(&row).as_deref(), Some("CH26-005228"));
    }

    #[test]
    fn payment_mapper_coalesce_returns_none_when_cin_no_null() {
        let m = PaymentMapper;
        let row = HashMapRow::new(HT_CHECKIN_PAY)
            .with("id", MockValue::I32(50001))
            .with("Cin_No", MockValue::Null);
        assert!(m.coalesce_key(&row).is_none());
    }

    #[test]
    fn receipt_mapper_uses_per_row_dispatch_no_coalesce() {
        let m = ReceiptMapper;
        let row = receipt_row(20663, "B2604-0265", "CH26-005228", 890.0);
        assert!(
            m.coalesce_key(&row).is_none(),
            "receipts process row-by-row; no coalesce key"
        );
    }

    // ----- project_receipt -----------------------------------------------

    #[test]
    fn project_receipt_extracts_required_fields() {
        let row = receipt_row(20663, "B2604-0265", "CH26-005228", 890.0);
        let p = project_receipt(&row).unwrap();
        assert_eq!(p.legacy_id, 20663);
        assert_eq!(p.receipt_no, "B2604-0265");
        assert_eq!(p.receipt_total, 890.0);
        assert_eq!(p.legacy_cin_no.as_deref(), Some("CH26-005228"));
        assert_eq!(p.legacy_cust_no.as_deref(), Some("C21607"));
        assert_eq!(p.status_name.as_deref(), Some("ปกติ"));
    }

    #[test]
    fn project_receipt_treats_empty_receipt_ref_as_none() {
        let mut row = receipt_row(20663, "B2604-0265", "", 890.0);
        row.cells
            .insert("Receipt_ref".into(), MockValue::Str(String::new()));
        let p = project_receipt(&row).unwrap();
        assert!(p.legacy_cin_no.is_none());
    }

    #[test]
    fn project_receipt_errors_when_id_missing() {
        let mut row = receipt_row(20663, "B2604-0265", "CH26-005228", 890.0);
        row.cells.insert("id".into(), MockValue::Null);
        let err = project_receipt(&row).expect_err("NULL id must error");
        assert!(err.to_string().contains("id"));
    }

    #[test]
    fn project_receipt_errors_when_receipt_total_null() {
        let mut row = receipt_row(20663, "B2604-0265", "CH26-005228", 890.0);
        row.cells.insert("Receipt_Total".into(), MockValue::Null);
        let err = project_receipt(&row).expect_err("NULL Receipt_Total must error");
        assert!(err.to_string().contains("Receipt_Total"));
    }

    // ----- build_payment_event -------------------------------------------

    #[test]
    fn build_payment_event_emits_payment_received_with_satang_amount() {
        let agg = aggregate_uuid(AggregateKind::CheckIn, 7);
        let ev = build_payment_event(agg, 890.0);
        assert_eq!(ev.type_name(), "PaymentReceived");
        let json = serde_json::to_value(&ev).unwrap();
        assert_eq!(
            json["data"]["check_in_id"],
            serde_json::Value::String(agg.to_string())
        );
        // Money is serialised as satang per the Money::Serialize impl.
        assert_eq!(json["data"]["amount"], serde_json::Value::from(89000));
    }

    /// Cancelled receipt (`status_name='ยกเลิก'`) projects with the
    /// cancel marker; the apply path translates this into pay_voided=true
    /// and skips the event publish (covered by the integration test).
    #[test]
    fn project_receipt_carries_cancelled_status_through() {
        let mut row = receipt_row(20663, "B2604-0265", "CH26-005228", 890.0);
        row.cells.insert(
            "status_name".into(),
            MockValue::Str("ยกเลิก".into()),
        );
        let p = project_receipt(&row).unwrap();
        assert_eq!(p.status_name.as_deref(), Some("ยกเลิก"));
    }

    // -------------------------------------------------------------------
    // Track J1 — projection-lock guards.
    // -------------------------------------------------------------------

    #[test]
    fn payment_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(PAYMENT_SELECT_COLS, "HT_CheckIn_Pay");
    }

    #[test]
    fn receipt_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(RECEIPT_SELECT_COLS, "HT_Receipt_H");
    }
}
