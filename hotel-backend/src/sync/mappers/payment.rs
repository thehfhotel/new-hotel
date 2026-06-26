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
use crate::sync::row::test_support::HashMapRow;
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
    // Track J7a — mirror this Cin_No's per-line tender ledger into the
    // canonical `ht_payment_ledger` for the round-close reconciliation +
    // shift report. Rides the same MSSQL read `load_checkin_aggregate`
    // already issued (no extra query). `?` so a mirror failure HOLDS the
    // watermark (the silent-drop class — sync_silent_drop_class memory):
    // the whole coalesced apply rolls back and retries next tick rather than
    // advancing past a half-mirrored payment.
    mirror_payment_ledger(tx, cin_no, &aggregate.payments).await?;
    apply_checkin_aggregate(tx, Some(mssql), &aggregate, cin_no).await
}

/// One `HT_CheckIn_Pay` line projected for the canonical mirror. PURE —
/// extracted from [`mirror_payment_ledger`] so the cell-reading / tender
/// defaulting is unit-testable without a PG transaction.
#[derive(Debug, Clone, PartialEq)]
struct PaymentLine {
    legacy_id: i32,
    pay_no: Option<String>,
    cust_no: Option<String>,
    ds_label: Option<String>,
    ds_name: Option<String>,
    ds_id: Option<String>,
    ds_num: Option<f64>,
    cash: f64,
    credit: f64,
    free: f64,
    tran: f64,
    web: f64,
    amount: f64,
    status: String,
    branch: Option<String>,
    pay_by: Option<String>,
    note: Option<String>,
    pay_date: Option<chrono::DateTime<chrono::Utc>>,
}

/// Project a materialised `HT_CheckIn_Pay` row into a [`PaymentLine`].
/// Tenders default to 0 when NULL/missing; `amount` falls back to the
/// tender sum when `Cin_Pay_Ds_Price` is absent (preserving the legacy
/// invariant `Ds_Price = cash+credit+free+tran+web`). `Cin_Status` defaults
/// to `'1'` (active) — the legacy INSERT omits it and relies on that default.
fn project_payment_line(row: &dyn MappableRow) -> Result<PaymentLine, SyncError> {
    let legacy_id = row.try_get_i32("id")?.ok_or_else(|| SyncError::Mapper {
        table: HT_CHECKIN_PAY,
        message: "ledger mirror: HT_CheckIn_Pay.id is NULL (required PK)".into(),
    })?;
    // Review J7a P2: a non-finite legacy tender (NaN/±Inf) must NOT reach the
    // INSERT. ±Inf would error the NUMERIC cast and POISON the shared batch tx
    // (wedging the whole payment tick on every retry); NaN is silently stored
    // by PG `numeric` and then poisons the round-window SUM. A non-finite money
    // value is garbage, so coerce to 0 and WARN (observable, never silent).
    let read_tender = |col: &str| -> Result<f64, SyncError> {
        let v = row.try_get_decimal(col)?.unwrap_or(0.0);
        if v.is_finite() {
            Ok(v)
        } else {
            tracing::warn!(
                event_name = "payment_ledger_nonfinite",
                table = HT_CHECKIN_PAY,
                legacy_id,
                column = col,
                "non-finite tender in HT_CheckIn_Pay — coercing to 0 for ht_payment_ledger"
            );
            Ok(0.0)
        }
    };
    let cash = read_tender("Cin_Pay_Cash")?;
    let credit = read_tender("Cin_Pay_Credit")?;
    let free = read_tender("Cin_Pay_Free")?;
    let tran = read_tender("Cin_Pay_Tran")?;
    let web = read_tender("Cin_Pay_web")?;
    let amount = match row.try_get_decimal("Cin_Pay_Ds_Price")? {
        Some(v) if v.is_finite() => v,
        Some(_) => {
            tracing::warn!(
                event_name = "payment_ledger_nonfinite",
                table = HT_CHECKIN_PAY,
                legacy_id,
                column = "Cin_Pay_Ds_Price",
                "non-finite line total — falling back to the (finite) tender sum"
            );
            cash + credit + free + tran + web
        }
        None => cash + credit + free + tran + web,
    };
    Ok(PaymentLine {
        legacy_id,
        pay_no: row.try_get_str("Pay_No")?.map(str::to_string),
        cust_no: row.try_get_str("Cin_Cust_no")?.map(str::to_string),
        ds_label: row.try_get_str("Cin_Pay_Ds")?.map(str::to_string),
        ds_name: row.try_get_str("Cin_Pay_Ds_Name")?.map(str::to_string),
        ds_id: row.try_get_str("Cin_Pay_Ds_ID")?.map(str::to_string),
        ds_num: row.try_get_decimal("Cin_Pay_Ds_Num")?,
        cash,
        credit,
        free,
        tran,
        web,
        amount,
        status: row.try_get_str("Cin_Status")?.unwrap_or("1").to_string(),
        branch: row.try_get_str("Branch")?.map(str::to_string),
        pay_by: row.try_get_str("Pay_by")?.map(str::to_string),
        note: row.try_get_str("Cin_Pay_Note")?.map(str::to_string),
        pay_date: row.try_get_datetime("Cin_Pay_Date")?.map(naive_thai_to_utc),
    })
}

/// Convert a legacy `HT_CheckIn_Pay.Cin_Pay_Date` (Thai-local, tz-naive) to a
/// true UTC instant for the canonical `TIMESTAMPTZ`. Same `+07:00` convention
/// as `sync::mappers::checkin::naive_dt_to_utc` and `ht_shifts`, so round-window
/// comparisons against `shift_opened_at..shift_closed_at` line up.
fn naive_thai_to_utc(dt: NaiveDateTime) -> chrono::DateTime<chrono::Utc> {
    use chrono::TimeZone;
    chrono::FixedOffset::east_opt(7 * 3600)
        .expect("+07:00 is a valid offset")
        .from_local_datetime(&dt)
        .single()
        .map(|d| d.with_timezone(&chrono::Utc))
        // FixedOffset never yields an ambiguous/none local time; the fallback
        // is purely to avoid an unwrap (treat the naive value as UTC).
        .unwrap_or_else(|| chrono::DateTime::from_naive_utc_and_offset(dt, chrono::Utc))
}

/// Replace this `Cin_No`'s lines in `ht_payment_ledger` with the current
/// legacy set. The loader returns the full as-of-now line set for the
/// `Cin_No`, so a delete-then-insert keeps the mirror exact through line
/// adds, edits, status flips (`Cin_Status='ยกเลิก'`), AND the rare hard
/// delete (legacy normally cancels via the status flip, which we carry). The
/// `ON CONFLICT (ledger_legacy_id)` arm additionally absorbs the theoretical
/// `Cin_No`-reassignment edge (a row whose old `Cin_No` wasn't in this
/// delete's scope) without tripping the UNIQUE constraint.
/// `pub` so the one-shot `backfill_payment_ledger` bin can reuse the exact
/// projection + upsert (single source of truth) — it loads each `Cin_No`'s
/// lines via `load_checkin_aggregate` and calls this, identical to the live
/// coalesced path.
pub async fn mirror_payment_ledger(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    cin_no: &str,
    payments: &[HashMapRow],
) -> Result<(), SyncError> {
    let map_err = |what: &str, e: sqlx::Error| SyncError::Mapper {
        table: HT_CHECKIN_PAY,
        message: format!("ht_payment_ledger {what}: {e}"),
    };

    sqlx::query("DELETE FROM ht_payment_ledger WHERE ledger_cin_no = $1")
        .bind(cin_no)
        .execute(&mut **tx)
        .await
        .map_err(|e| map_err("delete", e))?;

    for row in payments {
        let line = project_payment_line(row)?;
        sqlx::query(
            "INSERT INTO ht_payment_ledger ( \
                 ledger_legacy_id, ledger_pay_no, ledger_cin_no, ledger_cust_no, \
                 ledger_ds_label, ledger_ds_name, ledger_ds_id, ledger_ds_num, \
                 ledger_cash, ledger_credit, ledger_free, ledger_tran, ledger_web, \
                 ledger_amount, ledger_status, ledger_branch, ledger_pay_by, \
                 ledger_note, ledger_pay_date \
             ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8::numeric,$9::numeric,$10::numeric, \
                 $11::numeric,$12::numeric,$13::numeric,$14::numeric,$15,$16,$17,$18,$19) \
             ON CONFLICT (ledger_legacy_id) DO UPDATE SET \
                 ledger_pay_no = EXCLUDED.ledger_pay_no, ledger_cin_no = EXCLUDED.ledger_cin_no, \
                 ledger_cust_no = EXCLUDED.ledger_cust_no, ledger_ds_label = EXCLUDED.ledger_ds_label, \
                 ledger_ds_name = EXCLUDED.ledger_ds_name, ledger_ds_id = EXCLUDED.ledger_ds_id, \
                 ledger_ds_num = EXCLUDED.ledger_ds_num, ledger_cash = EXCLUDED.ledger_cash, \
                 ledger_credit = EXCLUDED.ledger_credit, ledger_free = EXCLUDED.ledger_free, \
                 ledger_tran = EXCLUDED.ledger_tran, ledger_web = EXCLUDED.ledger_web, \
                 ledger_amount = EXCLUDED.ledger_amount, ledger_status = EXCLUDED.ledger_status, \
                 ledger_branch = EXCLUDED.ledger_branch, ledger_pay_by = EXCLUDED.ledger_pay_by, \
                 ledger_note = EXCLUDED.ledger_note, ledger_pay_date = EXCLUDED.ledger_pay_date, \
                 ledger_synced_at = NOW()",
        )
        .bind(line.legacy_id)
        .bind(&line.pay_no)
        .bind(cin_no)
        .bind(&line.cust_no)
        .bind(&line.ds_label)
        .bind(&line.ds_name)
        .bind(&line.ds_id)
        .bind(line.ds_num)
        .bind(line.cash)
        .bind(line.credit)
        .bind(line.free)
        .bind(line.tran)
        .bind(line.web)
        .bind(line.amount)
        .bind(&line.status)
        .bind(&line.branch)
        .bind(&line.pay_by)
        .bind(&line.note)
        .bind(line.pay_date)
        .execute(&mut **tx)
        .await
        .map_err(|e| map_err(&format!("upsert id={}", line.legacy_id), e))?;
    }
    Ok(())
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
    //
    // Fallback timestamp must be Bangkok WALL-CLOCK, not UTC: every other
    // value in this column comes from legacy `Receipt_Date`, which is Thai
    // local time without timezone (CLAUDE.md "Timezone Handling"). A
    // `naive_utc()` fallback here landed 7h early (2026-06-11 audit).
    let now = chrono::Utc::now()
        .with_timezone(&chrono::FixedOffset::east_opt(7 * 3600).expect("+07:00 is valid"))
        .naive_local();
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

    // ----- Track J7a — ht_payment_ledger line projection -----------------

    /// A materialised `HT_CheckIn_Pay` row carries EVERY projected column
    /// (the loader inserts `MockValue::Null` for SQL NULLs, never omits) —
    /// so tests start from a complete NULL row and override specific cells,
    /// matching production shape. A *missing* column would (correctly) error.
    fn pay_line_row(id: i32) -> HashMapRow {
        let mut r = HashMapRow::new(HT_CHECKIN_PAY).with("id", MockValue::I32(id));
        for col in [
            "Cin_No", "Pay_No", "Cin_Cust_no", "Cin_Pay_Ds", "Cin_Pay_Ds_Name",
            "Cin_Pay_Ds_ID", "Cin_Pay_Ds_Num", "Cin_Pay_Cash", "Cin_Pay_Credit",
            "Cin_Pay_Free", "Cin_Pay_Tran", "Cin_Pay_web", "Cin_Pay_Ds_Price",
            "Cin_Status", "Branch", "Pay_by", "Cin_Pay_Note", "Cin_Pay_Date",
        ] {
            r = r.with(col, MockValue::Null);
        }
        r
    }

    #[test]
    fn project_payment_line_defaults_tenders_and_amount_and_status() {
        // NULL free/web/Ds_Price/Cin_Status: tenders default 0, amount falls
        // back to the tender sum, status defaults to active '1'.
        let row = pay_line_row(50123)
            .with("Cin_No", MockValue::Str("CH26-005228".into()))
            .with("Pay_No", MockValue::Str("R2606-0042".into()))
            .with("Cin_Pay_Cash", MockValue::Decimal(500.0))
            .with("Cin_Pay_Tran", MockValue::Decimal(300.0))
            .with("Cin_Pay_Credit", MockValue::Decimal(0.0))
            .with("Cin_Pay_Ds_ID", MockValue::Str("P001".into()))
            .with("Cin_Pay_Ds_Name", MockValue::Str("ค่าห้อง".into()));
        let line = project_payment_line(&row).unwrap();
        assert_eq!(line.legacy_id, 50123);
        assert_eq!(line.cash, 500.0);
        assert_eq!(line.tran, 300.0);
        assert_eq!(line.free, 0.0);
        assert_eq!(line.web, 0.0);
        assert_eq!(line.amount, 800.0, "amount = tender sum when Ds_Price NULL");
        assert_eq!(line.status, "1", "Cin_Status defaults to active '1'");
        assert_eq!(line.ds_id.as_deref(), Some("P001"));
        assert_eq!(line.pay_no.as_deref(), Some("R2606-0042"));
    }

    #[test]
    fn project_payment_line_honours_explicit_ds_price_and_cancel_status() {
        let row = pay_line_row(7)
            .with("Cin_Pay_Cash", MockValue::Decimal(100.0))
            .with("Cin_Pay_Ds_Price", MockValue::Decimal(100.0))
            .with("Cin_Status", MockValue::Str("ยกเลิก".into()));
        let line = project_payment_line(&row).unwrap();
        assert_eq!(line.amount, 100.0);
        assert_eq!(line.status, "ยกเลิก", "cancelled lines carry the cancel marker");
    }

    #[test]
    fn project_payment_line_carries_negative_refund_tenders() {
        // Refunds use tender negation (COMPAT_CHEATSHEET line 513).
        let row = pay_line_row(9)
            .with("Cin_Pay_Cash", MockValue::Decimal(-450.0))
            .with("Cin_Pay_Ds_Price", MockValue::Decimal(-450.0));
        let line = project_payment_line(&row).unwrap();
        assert_eq!(line.cash, -450.0);
        assert_eq!(line.amount, -450.0);
    }

    #[test]
    fn project_payment_line_errors_on_null_id() {
        let row = pay_line_row(0).with("id", MockValue::Null);
        let err = project_payment_line(&row).expect_err("NULL id must error (required PK)");
        assert!(err.to_string().contains("id"));
    }

    /// Review J7a P2: a non-finite tender (NaN/±Inf) is coerced to 0 (never
    /// reaches the NUMERIC INSERT to poison the tx / the round SUM). The line
    /// still projects — it is NOT dropped or errored (no wedge).
    #[test]
    fn project_payment_line_coerces_nonfinite_tenders_to_zero() {
        let row = pay_line_row(11)
            .with("Cin_Pay_Cash", MockValue::Decimal(f64::NAN))
            .with("Cin_Pay_Tran", MockValue::Decimal(f64::INFINITY))
            .with("Cin_Pay_Credit", MockValue::Decimal(250.0))
            // Non-finite line total falls back to the finite tender sum.
            .with("Cin_Pay_Ds_Price", MockValue::Decimal(f64::NAN));
        let line = project_payment_line(&row).expect("non-finite must not error");
        assert_eq!(line.cash, 0.0, "NaN cash coerced to 0");
        assert_eq!(line.tran, 0.0, "Inf transfer coerced to 0");
        assert_eq!(line.credit, 250.0);
        assert!(line.amount.is_finite(), "amount must be finite");
        assert_eq!(line.amount, 250.0, "amount = finite tender sum");
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
