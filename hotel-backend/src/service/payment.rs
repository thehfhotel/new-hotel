//! Payment service — orchestrates `ht_payments` writes plus outbox + events.
//!
//! Per `docs/architecture.md` §1, §6 and `docs/legacy-spike/findings.md`
//! §3h (record payment) — the legacy recipe inserts `HT_CheckIn_Pay` and
//! updates `HT_CheckIn_H` totals atomically.
//!
//! This service today owns:
//!
//! - **`record_payment`**: inserts `ht_payments`, enqueues
//!   [`WritebackIntent::RecordPayment`], publishes
//!   [`DomainEvent::PaymentReceived`] — all in one PG transaction.
//! - **`generate_receipt`**: a stub for the receipt-issuance flow. Today it
//!   only validates inputs + reserves the call site. Wave 4 will add the
//!   `INSERT HT_Receipt_H / HT_Receipt_Ds` writeback intent variant + a
//!   matching domain event variant.

use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::payment::PaymentMethod;
use crate::domain::shared::Money;
use crate::outbox::event::{DomainEvent, EventSource};
use crate::outbox::intent::{RecordPaymentReceipt, WritebackIntent};
use crate::outbox::{generate_idempotency_key, EventBus, OutboxRepository};
use crate::repository::payment::{PaymentInsert, PaymentRepository};

use super::error::{ServiceError, ServiceResult};
use super::ids::{aggregate_uuid, AggregateKind};

/// Command for [`PaymentService::record_payment`].
#[derive(Debug, Clone)]
pub struct RecordPaymentCommand {
    pub check_in_id: i32,
    pub amount_satang: i64,
    pub method: PaymentMethod,
    pub reference: Option<String>,
    pub notes: Option<String>,
    pub created_by: Option<String>,
    /// Customer + room metadata copied straight into the
    /// [`WritebackIntent::RecordPayment`] payload so the receipt header
    /// (`HT_Receipt_H.Receipt_Name` / `Address` / `Tel`) lands populated.
    /// Routes look these up from `ht_customers` before issuing the command.
    pub receipt: RecordPaymentReceipt,
    /// Specific `HT_CheckIn_Ds.id` the payment is being apportioned against —
    /// per spike §3h capture line 3, the .NET app fires
    /// `UPDATE HT_CheckIn_Ds SET Cin_Room_Pay_Total=<amt>, Cin_note='' WHERE id=<ds_id>`
    /// just before inserting `HT_CheckIn_Pay`. Routes resolve this via the
    /// canonical PG state when the payment maps to a single room. `None` for
    /// multi-room allocations — the recipe then skips the per-room UPDATE
    /// and only refreshes the header totals.
    pub checkin_ds_id: Option<i32>,
    /// Wave 5a item 2 — canonical per-night rate looked up from
    /// `ht_checkins.cin_rate_per_night` by the route. Threads into the
    /// recipe's `HT_CheckIn_Pay.Cin_Pay_Ds_PriceOne` column so the
    /// printed receipt line shows the real per-night price instead of
    /// the recipe's `amount/nights` fallback. `None` means the route
    /// couldn't resolve a rate (orphaned check-in) — the recipe then
    /// uses the fallback.
    pub price_per_night_baht: Option<f64>,
    /// Wave 5a item 2 — nights covered by the payment (>=1). Routes
    /// pass `ht_checkins.expected_checkout - check_in_time` clamped to
    /// >=1. `None` defaults to 1 in the recipe.
    pub nights: Option<i32>,
    pub source: EventSource,
}

/// Command for [`PaymentService::generate_receipt`].
///
/// `pay_ids` lists the payments included on the receipt (one receipt may
/// settle multiple `ht_payments` rows in the legacy recipe).
#[derive(Debug, Clone)]
pub struct GenerateReceiptCommand {
    pub check_in_id: i32,
    pub pay_ids: Vec<i32>,
    pub source: EventSource,
}

/// Outcome of a successful `record_payment`.
#[derive(Debug, Clone)]
pub struct RecordPaymentOutcome {
    pub pay_id: i32,
    pub payment_aggregate_id: Uuid,
    pub check_in_aggregate_id: Uuid,
}

/// Outcome of a successful `generate_receipt`.
///
/// The receipt has no canonical PG row today — Wave 4 adds `ht_receipts`.
/// The aggregate id is therefore derived from the check-in for now.
#[derive(Debug, Clone)]
pub struct GenerateReceiptOutcome {
    pub check_in_aggregate_id: Uuid,
}

/// Service handle for the payment aggregate.
#[derive(Clone)]
pub struct PaymentService {
    pub(crate) repo: Arc<dyn PaymentRepository>,
    pub(crate) outbox: Arc<OutboxRepository>,
    pub(crate) events: Arc<EventBus>,
    pub(crate) pg: PgPool,
}

impl PaymentService {
    pub fn new(
        repo: Arc<dyn PaymentRepository>,
        outbox: Arc<OutboxRepository>,
        events: Arc<EventBus>,
        pg: PgPool,
    ) -> Self {
        Self { repo, outbox, events, pg }
    }

    /// Record a payment — inserts `ht_payments`, enqueues legacy writeback,
    /// publishes the domain event.
    pub async fn record_payment(
        &self,
        cmd: RecordPaymentCommand,
    ) -> ServiceResult<RecordPaymentOutcome> {
        if cmd.amount_satang <= 0 {
            return Err(ServiceError::validation(format!(
                "payment amount must be positive (got {} satang)",
                cmd.amount_satang
            )));
        }

        let amount_baht = (cmd.amount_satang as f64) / 100.0;
        let method_str = method_to_legacy_string(cmd.method);

        let mut tx = self.pg.begin().await?;

        let pay_id = self
            .repo
            .insert(
                &mut tx,
                PaymentInsert {
                    cin_id: cmd.check_in_id,
                    amount: amount_baht,
                    method: method_str,
                    reference: cmd.reference.as_deref(),
                    notes: cmd.notes.as_deref(),
                    created_by: cmd.created_by.as_deref(),
                },
            )
            .await?;

        let check_in_aggregate_id = aggregate_uuid(AggregateKind::CheckIn, cmd.check_in_id);
        let payment_aggregate_id = aggregate_uuid(AggregateKind::Payment, pay_id);

        // Wave 5a item 3: stamp the aggregate_id onto the canonical row so
        // the writeback worker's `back_populate_legacy_ids` step can target
        // it via `WHERE aggregate_id = $1` (matches the pattern from
        // migration 014 for ht_bookings / ht_checkins).
        self.repo
            .stamp_aggregate_id(&mut tx, pay_id, payment_aggregate_id)
            .await?;

        let intent = WritebackIntent::RecordPayment {
            check_in_id: check_in_aggregate_id,
            amount: Money::from_satang(cmd.amount_satang),
            method: cmd.method,
            receipt: cmd.receipt.clone(),
            checkin_ds_id: cmd.checkin_ds_id,
            // Wave 5a item 2: surface the canonical per-night rate so the
            // recipe stops deriving it as `amount/nights` (which is wrong
            // for partial-night payments and multi-room apportionment).
            price_per_night_baht: cmd.price_per_night_baht,
            nights: cmd.nights,
            // Wave 5a item 3: target row for `back_populate_legacy_ids` to
            // stamp `legacy_pay_no` / `legacy_receipt_no` on after the
            // recipe allocates them.
            payment_aggregate_id: Some(payment_aggregate_id),
        };
        // Use the payment id (not the check-in id) as the idempotency
        // discriminator so multiple payments against the same check-in
        // don't collide on the unique key.
        let key = generate_idempotency_key(&intent, payment_aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        let event = DomainEvent::PaymentReceived {
            check_in_id: check_in_aggregate_id,
            amount: Money::from_satang(cmd.amount_satang),
            method: cmd.method,
            source: cmd.source.clone(),
        };
        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        tx.commit().await?;

        Ok(RecordPaymentOutcome {
            pay_id,
            payment_aggregate_id,
            check_in_aggregate_id,
        })
    }

    /// Generate a receipt — currently a placeholder for Wave 4.
    ///
    /// Validates the input shape (`pay_ids` non-empty) so callers learn
    /// about programming errors today. Once `WritebackIntent::IssueReceipt`
    /// + `DomainEvent::ReceiptIssued` land in the outbox contract, this
    /// method gains the standard transaction-+-publish body. Touching the
    /// other service collaborators here keeps the field wiring greppable.
    pub async fn generate_receipt(
        &self,
        cmd: GenerateReceiptCommand,
    ) -> ServiceResult<GenerateReceiptOutcome> {
        if cmd.pay_ids.is_empty() {
            return Err(ServiceError::validation(
                "generate_receipt requires at least one pay_id",
            ));
        }

        // No canonical write today — but consult the repo so any missing
        // payment surfaces as 404 rather than producing an empty receipt.
        for pay_id in &cmd.pay_ids {
            let status = self.repo.find_for_void(&self.pg, *pay_id).await?;
            if status.is_none() {
                return Err(ServiceError::not_found(format!(
                    "payment {pay_id} does not exist"
                )));
            }
        }

        // Touch outbox + events fields so Wave 4 doesn't need to add them
        // back when the writeback intent + event variants are introduced.
        let _ = (&self.outbox, &self.events);
        let _ = cmd.source;

        Ok(GenerateReceiptOutcome {
            check_in_aggregate_id: aggregate_uuid(AggregateKind::CheckIn, cmd.check_in_id),
        })
    }
}

/// Map [`PaymentMethod`] to the lowercase string the `ht_payments.pay_method`
/// column accepts. Mirrors the convention used by the existing route layer.
fn method_to_legacy_string(method: PaymentMethod) -> &'static str {
    match method {
        PaymentMethod::Cash => "cash",
        PaymentMethod::Credit => "credit",
        PaymentMethod::Transfer => "transfer",
    }
}
