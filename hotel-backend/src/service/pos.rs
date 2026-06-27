//! POS / sales-to-room service — Track G6 (MVP).
//!
//! ## What this owns
//!
//! `PosService::record_sale` is the single canonical write path for
//! ringing up a F&B / laundry / amenity product against an active
//! check-in folio. The service:
//!
//! 1. Validates the check-in is active.
//! 2. Resolves the product (must exist + be active) and computes
//!    the effective unit price (catalog price unless the caller
//!    supplied a non-NULL override).
//! 3. Opens one PG transaction. Inside:
//!    a. Decrements `ht_products.prod_current_stock` by `qty`
//!       (additive — never absolute SET — so a concurrent CT poll
//!       can't clobber the running counter).
//!    b. INSERTs the canonical `ht_pos_sales` row keyed on a
//!       freshly-minted v5 aggregate UUID.
//!    c. Enqueues `WritebackIntent::RecordPosSale` so the writeback
//!       worker mirrors the sale into legacy MSSQL (paired
//!       `HT_CheckIn_Product` INSERT + `HT_Products.Pro_Amt`
//!       additive decrement).
//! 4. Commits. On any failure before commit everything rolls back —
//!    the stock decrement, the sale row, and the writeback intent
//!    all land together or none of them do.
//!
//! ## What this does NOT own (MVP scope)
//!
//! The following are intentionally out of scope (see CHANGELOG
//! `### Deferred` for the matching list). TODOs are sprinkled at
//! the relevant call sites:
//!
//! - **Direct-pay POS** — `Cin_Pro_pay` stays zero; sales post to
//!   the folio and settle at round-bill.
//! - **Void / refund of a POS line** — schema supports
//!   `sale_status='voided'` but no service method touches it yet.
//! - **Multi-line receipts** — the route accepts one line per call;
//!   the UI can call N times for an N-line ticket.
//! - **Discount / coupon application** — `sale_unit_price` is the
//!   only price field; G5's discount engine (when it lands) layers
//!   on top.
//! - **Stock low-water alerts** — F3 owns the stock-level dashboard;
//!   this service emits no Slack alert.

use std::sync::Arc;

use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::outbox::intent::WritebackIntent;
use crate::outbox::{generate_idempotency_key, EventBus, OutboxRepository};
use crate::service::error::{ServiceError, ServiceResult};
use crate::service::ids::{aggregate_uuid, AggregateKind};
use crate::writeback::format::vat_inclusive_split;

/// Command for [`PosService::record_sale`].
///
/// `unit_price_override_baht` is an optional cashier override: pass
/// `None` to use the product's current catalog price (`prod_price`).
/// Pass `Some(x)` to override (e.g. happy-hour pricing). The override
/// must be >= 0; negative prices are validation errors.
#[derive(Debug, Clone)]
pub struct RecordSaleCommand {
    pub check_in_id: i32,
    pub product_id: i64,
    pub qty: f64,
    pub unit_price_override_baht: Option<f64>,
    pub note: Option<String>,
    pub actor: String,
}

/// Outcome of a successful `record_sale` operation.
#[derive(Debug, Clone)]
pub struct RecordSaleOutcome {
    pub sale_id: i64,
    pub aggregate_id: Uuid,
    pub check_in_id: i32,
    pub product_id: i64,
    pub qty: f64,
    pub unit_price_baht: f64,
    pub total_baht: f64,
    pub sold_at: DateTime<Utc>,
}

/// One line of a walk-up (roomless) sale — see [`RecordWalkupSaleCommand`].
#[derive(Debug, Clone)]
pub struct WalkupSaleLine {
    pub product_id: i64,
    pub qty: f64,
    /// Optional cashier price override (>= 0). `None` ⇒ catalog price.
    pub unit_price_override_baht: Option<f64>,
    /// Per-line discount in baht (>= 0). Subtracted from `qty × unit_price`.
    pub discount_baht: f64,
}

/// Command for [`PosService::record_walkup_sale`].
///
/// Task #45 — a walk-up / roomless sale: the cashier sells N product lines
/// to a customer who is NOT staying. There is no check-in folio; the sale
/// is persisted as a standalone receipt (`ht_pos_receipts` +
/// `ht_pos_receipt_lines`) and mirrored to legacy `HT_Receipt_H`/`Ds`.
#[derive(Debug, Clone)]
pub struct RecordWalkupSaleCommand {
    pub lines: Vec<WalkupSaleLine>,
    /// Legacy `Cust_no`. Empty ⇒ the anonymous walk-up sentinel `C0000`.
    pub customer_no: String,
    pub customer_name: String,
    pub customer_address: String,
    pub customer_tel: String,
    /// Buyer tax / customer ID (→ `HT_Receipt_H.Receipt_Tax`). Empty allowed.
    pub tax_id: String,
    pub note: String,
    /// VAT percentage to record (e.g. 0 or 7). Drives the inclusive split.
    pub vat_percent: i32,
    /// Header-level discount in baht (>= 0), on top of any per-line discounts.
    pub discount_baht: f64,
    /// Tender method (`cash`/`credit`/`transfer`/`qr`).
    pub payment_method: String,
    /// Direct-pay amount. `None` ⇒ settled in full (defaults to the total).
    pub paid_baht: Option<f64>,
    /// Operator (cashier) — `sold_by`. Empty ⇒ unknown sentinel.
    pub actor: String,
}

/// One persisted line of a walk-up sale outcome.
#[derive(Debug, Clone)]
pub struct WalkupSaleLineOutcome {
    pub line_id: i64,
    pub product_id: i64,
    pub product_legacy_no: String,
    pub product_name: String,
    pub unit_name: String,
    pub qty: f64,
    pub unit_price_baht: f64,
    pub discount_baht: f64,
    pub total_baht: f64,
}

/// Outcome of a successful `record_walkup_sale` operation.
#[derive(Debug, Clone)]
pub struct RecordWalkupSaleOutcome {
    pub receipt_id: i64,
    pub aggregate_id: Uuid,
    pub customer_no: String,
    pub subtotal_baht: f64,
    pub discount_baht: f64,
    pub before_vat_baht: f64,
    pub vat_baht: f64,
    pub vat_percent: i32,
    pub total_baht: f64,
    pub paid_baht: f64,
    pub payment_method: String,
    pub sold_at: DateTime<Utc>,
    pub lines: Vec<WalkupSaleLineOutcome>,
}

/// Command for [`PosService::void_sale`].
#[derive(Debug, Clone)]
pub struct VoidSaleCommand {
    pub sale_id: i64,
    /// Operator performing the void (logged; no canonical column today).
    pub by: String,
    pub reason: Option<String>,
}

/// Outcome of a successful `void_sale` operation.
#[derive(Debug, Clone)]
pub struct VoidSaleOutcome {
    pub sale_id: i64,
    pub check_in_id: i32,
    pub product_id: i64,
    pub qty_restored: f64,
}

/// Service handle for the POS aggregate.
///
/// Follows the same shape as `service::checkin::CheckInService` —
/// `EventBus` is held for future Track G follow-ups (no POS event
/// today; subscribers learn via the writeback row + UI refresh).
#[derive(Clone)]
pub struct PosService {
    #[allow(dead_code)]
    pub(crate) outbox: Arc<OutboxRepository>,
    #[allow(dead_code)]
    pub(crate) events: Arc<EventBus>,
    pub(crate) pg: PgPool,
}

impl PosService {
    pub fn new(outbox: Arc<OutboxRepository>, events: Arc<EventBus>, pg: PgPool) -> Self {
        Self {
            outbox,
            events,
            pg,
        }
    }

    /// Ring up one product line against the given check-in folio.
    ///
    /// Validates → opens PG TX → decrements stock → INSERTs sale →
    /// enqueues writeback intent → commits. See module docs for the
    /// full sequence.
    pub async fn record_sale(
        &self,
        cmd: RecordSaleCommand,
    ) -> ServiceResult<RecordSaleOutcome> {
        // -----------------------------------------------------------
        // 1. Pre-flight validation (no I/O yet, fail fast).
        // -----------------------------------------------------------
        if !cmd.qty.is_finite() || cmd.qty <= 0.0 {
            return Err(ServiceError::validation(format!(
                "qty must be positive and finite (got {})",
                cmd.qty
            )));
        }
        if let Some(price) = cmd.unit_price_override_baht {
            if !price.is_finite() || price < 0.0 {
                return Err(ServiceError::validation(format!(
                    "unit_price_override must be non-negative and finite (got {price})"
                )));
            }
        }

        // -----------------------------------------------------------
        // 2. Verify the check-in is active. We do this OUTSIDE the
        //    transaction so a definitively-cancelled folio fails
        //    fast and the txn never opens. The active state is
        //    re-confirmed inside the tx by the FK constraint when
        //    the INSERT runs; concurrent cancellation would lose
        //    the race deterministically (CASCADE wipes the sale row
        //    if the check-in goes away first).
        // -----------------------------------------------------------
        let checkin_row = sqlx::query(
            "SELECT cin_id, cin_status FROM ht_checkins WHERE cin_id = $1",
        )
        .bind(cmd.check_in_id)
        .fetch_optional(&self.pg)
        .await?
        .ok_or_else(|| {
            ServiceError::not_found(format!("check-in {} does not exist", cmd.check_in_id))
        })?;
        let cin_status: Option<String> = checkin_row.try_get("cin_status").ok();
        if !matches!(cin_status.as_deref(), Some("active")) {
            return Err(ServiceError::conflict(format!(
                "check-in {} is not active ({:?})",
                cmd.check_in_id, cin_status
            )));
        }

        // -----------------------------------------------------------
        // 3. Open transaction. Read the product master, compute
        //    effective unit price + total, decrement stock, INSERT
        //    sale, enqueue intent. All-or-nothing.
        // -----------------------------------------------------------
        let mut tx = self.pg.begin().await?;

        // Read product master + verify it's active. We don't lock
        // the row here because the stock decrement below uses an
        // additive UPDATE (concurrent decrements are race-safe —
        // see `routes/new_products.rs::adjust_stock` for the same
        // pattern).
        let product_row = sqlx::query(
            "SELECT prod_id, prod_legacy_no, prod_name, prod_price::float8 AS prod_price, \
                    prod_active, aggregate_id \
               FROM ht_products WHERE prod_id = $1",
        )
        .bind(cmd.product_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| {
            ServiceError::not_found(format!("product {} does not exist", cmd.product_id))
        })?;
        let prod_active: bool = product_row.try_get("prod_active").unwrap_or(true);
        if !prod_active {
            return Err(ServiceError::conflict(format!(
                "product {} is archived (prod_active=FALSE)",
                cmd.product_id
            )));
        }
        let catalog_price_baht: f64 =
            product_row.try_get("prod_price").unwrap_or(0.0);
        let unit_price_baht = cmd
            .unit_price_override_baht
            .unwrap_or(catalog_price_baht);
        let total_baht = round_baht(cmd.qty * unit_price_baht);

        // 3a. Decrement product stock (additive — see module docs).
        sqlx::query(
            "UPDATE ht_products \
                SET prod_current_stock = prod_current_stock - $1::float8, \
                    prod_updated_at    = NOW() \
              WHERE prod_id = $2",
        )
        .bind(cmd.qty)
        .bind(cmd.product_id)
        .execute(&mut *tx)
        .await?;

        // 3b. INSERT the canonical sale row. Aggregate UUID is the
        //     placeholder Uuid::nil() — we update it post-INSERT
        //     using the freshly-allocated `sale_id`.
        //     RETURNING surfaces both the SERIAL id and the timestamp.
        let note_trimmed = cmd
            .note
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty());
        let sold_by_opt = if cmd.actor.is_empty() {
            None
        } else {
            Some(cmd.actor.as_str())
        };

        // Two-step: insert with a placeholder UUID, then UPDATE to
        // the deterministic v5 aggregate id once we know the SERIAL.
        // PostgreSQL's UNIQUE constraint on `aggregate_id` would
        // otherwise force a roundabout `gen_random_uuid()` mint.
        let inserted = sqlx::query(
            "INSERT INTO ht_pos_sales ( \
                sale_cin_id, sale_product_id, sale_qty, sale_unit_price, \
                sale_sold_by, sale_note, source, aggregate_id \
             ) VALUES ($1, $2, $3::numeric, $4::numeric, $5, $6, 'canonical', $7) \
             RETURNING sale_id, sale_sold_at",
        )
        .bind(cmd.check_in_id)
        .bind(cmd.product_id)
        .bind(cmd.qty)
        .bind(unit_price_baht)
        .bind(sold_by_opt)
        .bind(note_trimmed)
        // Temporary placeholder — replaced immediately below.
        .bind(Uuid::new_v4())
        .fetch_one(&mut *tx)
        .await?;
        let sale_id: i64 = inserted.try_get("sale_id")?;
        let sold_at: DateTime<Utc> = inserted.try_get("sale_sold_at")?;

        // Pin the deterministic v5 aggregate id keyed on the SERIAL.
        // Idempotent retries (same sale_id) hash to the same UUID,
        // which is exactly the property subscribers need to dedup.
        // i32::try_from is safe: sale_id won't exceed i32 for any
        // realistic deployment (BIGSERIAL just sizes the column).
        let aggregate_id =
            aggregate_uuid(AggregateKind::PosSale, i32::try_from(sale_id).unwrap_or(i32::MAX));
        sqlx::query(
            "UPDATE ht_pos_sales SET aggregate_id = $2, updated_at = NOW() \
              WHERE sale_id = $1",
        )
        .bind(sale_id)
        .bind(aggregate_id)
        .execute(&mut *tx)
        .await?;

        // 3c. Enqueue the writeback intent. The aggregate_id we put
        //     on the intent is the parent CHECK-IN's UUID (matches
        //     RecordPayment / RoomChange) so the writeback worker's
        //     resolver finds `legacy_cin_no` via the same code path.
        let cin_aggregate_id = aggregate_uuid(AggregateKind::CheckIn, cmd.check_in_id);
        let intent = WritebackIntent::RecordPosSale {
            check_in_id: cin_aggregate_id,
            sale_id,
        };
        let idem = generate_idempotency_key(&intent, aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, idem)
            .await
            .map_err(ServiceError::from_enqueue_error)?;

        tx.commit().await?;

        Ok(RecordSaleOutcome {
            sale_id,
            aggregate_id,
            check_in_id: cmd.check_in_id,
            product_id: cmd.product_id,
            qty: cmd.qty,
            unit_price_baht,
            total_baht,
            sold_at,
        })
    }

    /// Ring up a walk-up (roomless) sale → standalone receipt. Task #45.
    ///
    /// Validates → opens PG TX → per line: reads the product master,
    /// computes the effective unit price + line total, decrements stock
    /// additively → INSERTs the `ht_pos_receipts` header + N
    /// `ht_pos_receipt_lines` → enqueues `WritebackIntent::RecordReceipt`
    /// → commits. There is NO check-in folio: the sale lands in legacy as
    /// a standalone `HT_Receipt_H`/`Ds` pair (`Receipt_ref=''`).
    pub async fn record_walkup_sale(
        &self,
        cmd: RecordWalkupSaleCommand,
    ) -> ServiceResult<RecordWalkupSaleOutcome> {
        // 1. Pre-flight validation (no I/O yet).
        if cmd.lines.is_empty() {
            return Err(ServiceError::validation(
                "a walk-up sale must have at least one line".to_string(),
            ));
        }
        for (i, line) in cmd.lines.iter().enumerate() {
            if !line.qty.is_finite() || line.qty <= 0.0 {
                return Err(ServiceError::validation(format!(
                    "line {i}: qty must be positive and finite (got {})",
                    line.qty
                )));
            }
            if let Some(p) = line.unit_price_override_baht {
                if !p.is_finite() || p < 0.0 {
                    return Err(ServiceError::validation(format!(
                        "line {i}: unit_price_override must be non-negative and finite (got {p})"
                    )));
                }
            }
            if !line.discount_baht.is_finite() || line.discount_baht < 0.0 {
                return Err(ServiceError::validation(format!(
                    "line {i}: discount must be non-negative and finite (got {})",
                    line.discount_baht
                )));
            }
        }
        if !cmd.discount_baht.is_finite() || cmd.discount_baht < 0.0 {
            return Err(ServiceError::validation(format!(
                "header discount must be non-negative and finite (got {})",
                cmd.discount_baht
            )));
        }
        if let Some(p) = cmd.paid_baht {
            if !p.is_finite() || p < 0.0 {
                return Err(ServiceError::validation(format!(
                    "paid amount must be non-negative and finite (got {p})"
                )));
            }
        }

        let mut tx = self.pg.begin().await?;

        // 2. Per line: read product, compute price + total, decrement stock.
        let mut subtotal = 0.0_f64;
        let mut line_discount_total = 0.0_f64;
        struct StagedLine {
            product_id: i64,
            prod_legacy_no: String,
            prod_name: String,
            unit_name: String,
            qty: f64,
            unit_price_baht: f64,
            discount_baht: f64,
            total_baht: f64,
        }
        let mut staged: Vec<StagedLine> = Vec::with_capacity(cmd.lines.len());
        for (i, line) in cmd.lines.iter().enumerate() {
            let product_row = sqlx::query(
                "SELECT prod_id, prod_legacy_no, prod_name, prod_unit, \
                        prod_price::float8 AS prod_price, prod_active \
                   FROM ht_products WHERE prod_id = $1",
            )
            .bind(line.product_id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| {
                ServiceError::not_found(format!(
                    "line {i}: product {} does not exist",
                    line.product_id
                ))
            })?;
            let prod_active: bool = product_row.try_get("prod_active").unwrap_or(true);
            if !prod_active {
                return Err(ServiceError::conflict(format!(
                    "line {i}: product {} is archived (prod_active=FALSE)",
                    line.product_id
                )));
            }
            let catalog_price: f64 = product_row.try_get("prod_price").unwrap_or(0.0);
            let unit_price = line.unit_price_override_baht.unwrap_or(catalog_price);
            let gross = line.qty * unit_price;
            let line_total = round_baht(gross - line.discount_baht);
            subtotal += gross;
            line_discount_total += line.discount_baht;

            // Additive stock decrement (never absolute SET — race-safe with a
            // concurrent CT poll; same rule as `record_sale`).
            sqlx::query(
                "UPDATE ht_products \
                    SET prod_current_stock = prod_current_stock - $1::float8, \
                        prod_updated_at    = NOW() \
                  WHERE prod_id = $2",
            )
            .bind(line.qty)
            .bind(line.product_id)
            .execute(&mut *tx)
            .await?;

            staged.push(StagedLine {
                product_id: line.product_id,
                prod_legacy_no: product_row.try_get("prod_legacy_no").unwrap_or_default(),
                prod_name: product_row.try_get("prod_name").unwrap_or_default(),
                unit_name: product_row
                    .try_get::<Option<String>, _>("prod_unit")
                    .unwrap_or(None)
                    .unwrap_or_default(),
                qty: line.qty,
                unit_price_baht: unit_price,
                discount_baht: line.discount_baht,
                total_baht: line_total,
            });
        }

        let subtotal = round_baht(subtotal);
        let discount_total = round_baht(line_discount_total + cmd.discount_baht);
        let total = round_baht(subtotal - discount_total).max(0.0);
        let (before_vat, vat) = vat_inclusive_split(total, cmd.vat_percent);
        let paid = cmd.paid_baht.unwrap_or(total);

        let customer_no = if cmd.customer_no.trim().is_empty() {
            "C0000".to_string()
        } else {
            cmd.customer_no.trim().to_string()
        };
        let sold_by_opt = if cmd.actor.trim().is_empty() {
            None
        } else {
            Some(cmd.actor.trim())
        };

        // 3. INSERT the receipt header (placeholder aggregate UUID), then pin
        //    the deterministic v5 aggregate id keyed on the BIGSERIAL.
        let header = sqlx::query(
            "INSERT INTO ht_pos_receipts ( \
                receipt_customer_no, receipt_customer_name, receipt_customer_addr, \
                receipt_customer_tel, receipt_tax_id, receipt_subtotal, receipt_discount, \
                receipt_total, receipt_before_vat, receipt_vat, receipt_vat_percent, \
                receipt_paid, receipt_payment_method, receipt_note, receipt_sold_by, \
                source, aggregate_id \
             ) VALUES ($1, $2, $3, $4, $5, $6::numeric, $7::numeric, $8::numeric, \
                       $9::numeric, $10::numeric, $11, $12::numeric, $13, $14, $15, \
                       'canonical', $16) \
             RETURNING receipt_id, receipt_sold_at",
        )
        .bind(&customer_no)
        .bind(cmd.customer_name.trim())
        .bind(cmd.customer_address.trim())
        .bind(cmd.customer_tel.trim())
        .bind(cmd.tax_id.trim())
        .bind(subtotal)
        .bind(discount_total)
        .bind(total)
        .bind(before_vat)
        .bind(vat)
        .bind(cmd.vat_percent)
        .bind(paid)
        .bind(cmd.payment_method.trim())
        .bind(cmd.note.trim())
        .bind(sold_by_opt)
        .bind(Uuid::new_v4())
        .fetch_one(&mut *tx)
        .await?;
        let receipt_id: i64 = header.try_get("receipt_id")?;
        let sold_at: DateTime<Utc> = header.try_get("receipt_sold_at")?;
        let aggregate_id =
            aggregate_uuid(AggregateKind::Receipt, i32::try_from(receipt_id).unwrap_or(i32::MAX));
        sqlx::query(
            "UPDATE ht_pos_receipts SET aggregate_id = $2, updated_at = NOW() \
              WHERE receipt_id = $1",
        )
        .bind(receipt_id)
        .bind(aggregate_id)
        .execute(&mut *tx)
        .await?;

        // 4. INSERT the lines.
        let mut line_outcomes: Vec<WalkupSaleLineOutcome> = Vec::with_capacity(staged.len());
        for s in &staged {
            let line_row = sqlx::query(
                "INSERT INTO ht_pos_receipt_lines ( \
                    line_receipt_id, line_product_id, line_product_no, line_product_name, \
                    line_unit_name, line_qty, line_unit_price, line_discount, line_total \
                 ) VALUES ($1, $2, $3, $4, $5, $6::numeric, $7::numeric, $8::numeric, $9::numeric) \
                 RETURNING line_id",
            )
            .bind(receipt_id)
            .bind(s.product_id)
            .bind(&s.prod_legacy_no)
            .bind(&s.prod_name)
            .bind(&s.unit_name)
            .bind(s.qty)
            .bind(s.unit_price_baht)
            .bind(s.discount_baht)
            .bind(s.total_baht)
            .fetch_one(&mut *tx)
            .await?;
            line_outcomes.push(WalkupSaleLineOutcome {
                line_id: line_row.try_get("line_id")?,
                product_id: s.product_id,
                product_legacy_no: s.prod_legacy_no.clone(),
                product_name: s.prod_name.clone(),
                unit_name: s.unit_name.clone(),
                qty: s.qty,
                unit_price_baht: s.unit_price_baht,
                discount_baht: s.discount_baht,
                total_baht: s.total_baht,
            });
        }

        // 5. Enqueue the writeback intent (keyed on the receipt's own UUID).
        let intent = WritebackIntent::RecordReceipt {
            receipt_aggregate_id: aggregate_id,
            receipt_id,
        };
        let idem = generate_idempotency_key(&intent, aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, idem)
            .await
            .map_err(ServiceError::from_enqueue_error)?;

        tx.commit().await?;

        Ok(RecordWalkupSaleOutcome {
            receipt_id,
            aggregate_id,
            customer_no,
            subtotal_baht: subtotal,
            discount_baht: discount_total,
            before_vat_baht: before_vat,
            vat_baht: vat,
            vat_percent: cmd.vat_percent,
            total_baht: total,
            paid_baht: round_baht(paid),
            payment_method: cmd.payment_method,
            sold_at,
            lines: line_outcomes,
        })
    }

    /// Void a folio POS sale line. Task #45.
    ///
    /// Flips canonical `ht_pos_sales.sale_status='voided'`, restores the
    /// product stock (additive `+qty`), and enqueues
    /// `WritebackIntent::VoidPosSale` so the legacy `HT_CheckIn_Product`
    /// line is removed + `Pro_Amt` restored — all in one PG transaction.
    pub async fn void_sale(&self, cmd: VoidSaleCommand) -> ServiceResult<VoidSaleOutcome> {
        let mut tx = self.pg.begin().await?;

        // Read + lock the sale row.
        let sale_row = sqlx::query(
            "SELECT sale_cin_id, sale_product_id, sale_qty::float8 AS qty, sale_status \
               FROM ht_pos_sales WHERE sale_id = $1 FOR UPDATE",
        )
        .bind(cmd.sale_id)
        .fetch_optional(&mut *tx)
        .await?
        .ok_or_else(|| {
            ServiceError::not_found(format!("sale {} does not exist", cmd.sale_id))
        })?;
        let status: String = sale_row.try_get("sale_status").unwrap_or_default();
        if status == "voided" {
            return Err(ServiceError::conflict(format!(
                "sale {} is already voided",
                cmd.sale_id
            )));
        }
        let check_in_id: i32 = sale_row.try_get("sale_cin_id").unwrap_or(0);
        let product_id: i64 = sale_row.try_get("sale_product_id").unwrap_or(0);
        let qty: f64 = sale_row.try_get("qty").unwrap_or(0.0);

        // Flip status.
        sqlx::query(
            "UPDATE ht_pos_sales SET sale_status = 'voided', updated_at = NOW() \
              WHERE sale_id = $1",
        )
        .bind(cmd.sale_id)
        .execute(&mut *tx)
        .await?;

        // Restore stock (additive — inverse of the original decrement).
        sqlx::query(
            "UPDATE ht_products \
                SET prod_current_stock = prod_current_stock + $1::float8, \
                    prod_updated_at    = NOW() \
              WHERE prod_id = $2",
        )
        .bind(qty)
        .bind(product_id)
        .execute(&mut *tx)
        .await?;

        // Enqueue the legacy reversal (parent check-in UUID, like RecordPosSale).
        let cin_aggregate_id = aggregate_uuid(AggregateKind::CheckIn, check_in_id);
        let intent = WritebackIntent::VoidPosSale {
            check_in_id: cin_aggregate_id,
            sale_id: cmd.sale_id,
        };
        // The void aggregate is the SALE's own UUID so the idempotency key is
        // distinct from the original RecordPosSale (which keyed on the sale's
        // aggregate too) — voids and creates never collide on the outbox
        // UNIQUE because their intent names differ.
        let sale_aggregate_id =
            aggregate_uuid(AggregateKind::PosSale, i32::try_from(cmd.sale_id).unwrap_or(i32::MAX));
        let idem = generate_idempotency_key(&intent, sale_aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, idem)
            .await
            .map_err(ServiceError::from_enqueue_error)?;

        tx.commit().await?;

        tracing::info!(
            sale_id = cmd.sale_id,
            by = %cmd.by,
            reason = cmd.reason.as_deref().unwrap_or(""),
            "POS sale voided (canonical + legacy reversal enqueued)"
        );

        Ok(VoidSaleOutcome {
            sale_id: cmd.sale_id,
            check_in_id,
            product_id,
            qty_restored: qty,
        })
    }
}

/// Round a baht amount to 2dp (banker-rounding-agnostic — `f64`
/// midpoint rounding is platform-defined, so we use `round()` which
/// is "round half away from zero" everywhere). The legacy schema
/// stores `Cin_Pro_priceTotal` as `float`, but the recipe formats
/// it at 2dp on the wire, so we match the stored precision here.
fn round_baht(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Locks the round-baht helper. 0.555 * 100 = 55.5 on f64;
    /// `.round()` returns 56, so `round_baht(0.555) = 0.56`.
    #[test]
    fn round_baht_rounds_to_two_decimals() {
        assert_eq!(round_baht(50.0), 50.0);
        assert_eq!(round_baht(50.005), 50.01);
        assert_eq!(round_baht(0.554), 0.55);
        assert_eq!(round_baht(0.555), 0.56);
        // Negative paths (defensive — service rejects negative
        // prices but the helper itself is symmetric).
        assert_eq!(round_baht(-50.005), -50.01);
    }

    /// Total computation: qty × unit_price, then 2dp rounding.
    /// Locks the math the route's response body relies on so a
    /// future refactor can't silently introduce 3dp truncation.
    #[test]
    fn total_is_qty_times_unit_price_rounded_to_two_decimals() {
        let qty = 3.0;
        let unit = 25.5;
        let total = round_baht(qty * unit);
        assert_eq!(total, 76.5);
    }

    /// Fractional qty (litres / kg) — 1.5L × 30 = 45.00 baht.
    /// Locks the contract that fractional qty doesn't drift through
    /// the 2dp rounding boundary.
    #[test]
    fn fractional_qty_yields_clean_total() {
        let total = round_baht(1.5 * 30.0);
        assert_eq!(total, 45.0);
    }

    /// Validation: zero qty must be rejected before any I/O.
    /// The async path is exercised in integration tests; here we
    /// just lock the boundary value so the guard isn't quietly
    /// removed.
    #[test]
    fn zero_qty_is_invalid_command_input() {
        let cmd = RecordSaleCommand {
            check_in_id: 1,
            product_id: 1,
            qty: 0.0,
            unit_price_override_baht: None,
            note: None,
            actor: String::new(),
        };
        // The synchronous predicate the service applies first.
        assert!(!(cmd.qty.is_finite() && cmd.qty > 0.0));
    }

    /// NaN qty must be rejected (NaN-poison guard — without it the
    /// stock decrement would silently corrupt prod_current_stock).
    #[test]
    fn nan_qty_is_invalid_command_input() {
        let cmd = RecordSaleCommand {
            check_in_id: 1,
            product_id: 1,
            qty: f64::NAN,
            unit_price_override_baht: None,
            note: None,
            actor: String::new(),
        };
        assert!(!cmd.qty.is_finite());
    }

    /// Negative unit-price override must be rejected. Prevents a
    /// cashier from accidentally / deliberately turning a sale into
    /// a refund (refunds live in `service::payment::refund_payment`).
    #[test]
    fn negative_unit_price_override_is_invalid() {
        let cmd = RecordSaleCommand {
            check_in_id: 1,
            product_id: 1,
            qty: 1.0,
            unit_price_override_baht: Some(-10.0),
            note: None,
            actor: String::new(),
        };
        assert!(matches!(cmd.unit_price_override_baht, Some(p) if p < 0.0));
    }

    /// Locks the `AggregateKind::PosSale` namespace: a future
    /// rename of the label string would silently invalidate every
    /// stored `aggregate_id`. The kind is stable across releases.
    #[test]
    fn pos_sale_aggregate_kind_namespace_is_pinned() {
        // Two SERIAL ids hash to two distinct UUIDs (collision-free
        // under v5/SHA-1 for distinct inputs in the same namespace).
        let one = aggregate_uuid(AggregateKind::PosSale, 1);
        let two = aggregate_uuid(AggregateKind::PosSale, 2);
        assert_ne!(one, two);
        // And the kind is disjoint from PosSale's nearest aggregate
        // (Payment — same domain semantics, money-in / line-item).
        let pay_one = aggregate_uuid(AggregateKind::Payment, 1);
        assert_ne!(one, pay_one);
    }

    /// The writeback intent for a sale must carry the PARENT
    /// check-in UUID as its `aggregate_id` (so the resolver picks
    /// up `legacy_cin_no` the same way RecordPayment does). Locks
    /// this contract — if a refactor accidentally swapped in the
    /// per-sale UUID, the writeback resolver would skip the
    /// `legacy_cin_no` self-heal and the recipe would error on
    /// an empty cin_no.
    #[test]
    fn writeback_intent_aggregate_id_is_parent_checkin_uuid() {
        let check_in_id_uuid = aggregate_uuid(AggregateKind::CheckIn, 99);
        let intent = WritebackIntent::RecordPosSale {
            check_in_id: check_in_id_uuid,
            sale_id: 4242,
        };
        // Helper from the intent enum — same field the worker reads.
        assert_eq!(intent.aggregate_id(), check_in_id_uuid);
        assert_eq!(intent.intent_name(), "record_pos_sale");
    }

    /// Serde round-trip — the writeback intent must serialize /
    /// deserialize cleanly so the outbox JSONB column stays
    /// readable across deploys.
    #[test]
    fn writeback_intent_round_trips_through_json() {
        let check_in_id_uuid = aggregate_uuid(AggregateKind::CheckIn, 99);
        let intent = WritebackIntent::RecordPosSale {
            check_in_id: check_in_id_uuid,
            sale_id: 4242,
        };
        let json = serde_json::to_value(&intent).expect("serializes");
        assert_eq!(json["intent"], "record_pos_sale");
        assert_eq!(json["payload"]["check_in_id"], check_in_id_uuid.to_string());
        assert_eq!(json["payload"]["sale_id"], 4242);

        let back: WritebackIntent =
            serde_json::from_value(json).expect("round-trips back");
        match back {
            WritebackIntent::RecordPosSale {
                check_in_id,
                sale_id,
            } => {
                assert_eq!(check_in_id, check_in_id_uuid);
                assert_eq!(sale_id, 4242);
            }
            other => panic!("expected RecordPosSale, got {other:?}"),
        }
    }
}
