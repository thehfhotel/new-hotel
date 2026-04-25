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
use crate::outbox::intent::WritebackIntent;
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

        let intent = WritebackIntent::RecordPayment {
            check_in_id: check_in_aggregate_id,
            amount: Money::from_satang(cmd.amount_satang),
            method: cmd.method,
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
