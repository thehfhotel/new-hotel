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
use crate::repository::payment::{PaymentInsert, PaymentRepository, RefundInsert};

use super::error::{ServiceError, ServiceResult};
use super::ids::{aggregate_uuid, AggregateKind};
use super::shifts::ShiftService;

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
    /// Wave 5c — VAT percent read from `ht_settings.vat_percent` by the
    /// route. Threads into the writeback recipe so the printed receipt
    /// honors the hotel's configured rate (0%/7%) instead of the
    /// hardcoded legacy constant. `None` lets the recipe fall back to
    /// the hardcoded default — used by tests / older queue rows.
    pub vat_percent: Option<i32>,
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

/// Command for [`PaymentService::refund_payment`].
///
/// Track G2 / T4 CRIT-1 (`docs/coexistence/audit-2026-05-13.md`). The
/// receptionist initiates a refund against an existing payment row. The
/// service writes a new `ht_payments` row with negative `pay_amount`,
/// enqueues a `WritebackIntent::RefundPayment`, and publishes a
/// `DomainEvent::PaymentRefunded`.
///
/// `amount_satang` is POSITIVE — the magnitude of the refund. The
/// repository negates it before writing canonical PG; the writeback
/// recipe negates it before emitting legacy SQL. Keeping the carrier
/// positive across the wire matches the audit doc's recommendation and
/// keeps `Money::from_satang` (which rejects negative) usable.
#[derive(Debug, Clone)]
pub struct RefundPaymentCommand {
    /// `ht_payments.pay_id` of the payment being refunded.
    pub original_payment_id: i32,
    /// Magnitude of the refund in satang (must be > 0). Service validates
    /// `amount_satang ≤ (original_amount - already_refunded)`.
    pub amount_satang: i64,
    /// Operator-supplied reason. Empty string treated as "no reason
    /// provided" — captured into `ht_payments.refund_reason` and
    /// surfaced via the writeback recipe's `Cin_Pay_Note` prefix.
    pub reason: Option<String>,
    pub created_by: Option<String>,
    pub source: EventSource,
}

/// Outcome of a successful `refund_payment`.
#[derive(Debug, Clone)]
pub struct RefundPaymentOutcome {
    /// New SERIAL `pay_id` of the refund row inserted into `ht_payments`.
    pub refund_pay_id: i32,
    /// Deterministic UUID stamped onto the refund row's `aggregate_id`.
    pub refund_aggregate_id: Uuid,
    /// Aggregate UUID of the parent check-in. Surfaced for callers that
    /// need to correlate the refund to the folio (e.g. SSE listeners).
    pub check_in_aggregate_id: Uuid,
}

/// Service handle for the payment aggregate.
#[derive(Clone)]
pub struct PaymentService {
    pub(crate) repo: Arc<dyn PaymentRepository>,
    pub(crate) outbox: Arc<OutboxRepository>,
    pub(crate) events: Arc<EventBus>,
    pub(crate) pg: PgPool,
    /// Optional shift gate — Track F2 / T1 HIGH-5. When wired,
    /// `record_payment` refuses to insert unless `shifts.current_open_shift()`
    /// returns `Some`. Stays `None` in unit tests and any deployment that
    /// has not yet provisioned a shift workflow so existing flows keep
    /// working until the cashier UI lands.
    pub(crate) shifts: Option<Arc<ShiftService>>,
}

impl PaymentService {
    pub fn new(
        repo: Arc<dyn PaymentRepository>,
        outbox: Arc<OutboxRepository>,
        events: Arc<EventBus>,
        pg: PgPool,
    ) -> Self {
        Self {
            repo,
            outbox,
            events,
            pg,
            shifts: None,
        }
    }

    /// Builder-style attach for the shift gate.
    ///
    /// Wired from `AppState::wire_services` so production deployments
    /// refuse to record a payment unless an `ht_shifts` row is open for
    /// the site. Tests construct `PaymentService::new(...)` without a
    /// shift service when they want to exercise the payment flow alone.
    pub fn with_shifts(mut self, shifts: Arc<ShiftService>) -> Self {
        self.shifts = Some(shifts);
        self
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

        // Track F2 / T1 HIGH-5: refuse the payment when a shift gate is
        // wired and no shift is open. The gate is `Option<_>` so unit
        // tests not exercising the shift flow stay isolated; production
        // construction in `AppState::wire_services` always wires it on.
        if let Some(shifts) = &self.shifts {
            if shifts.current_open_shift().await?.is_none() {
                return Err(ServiceError::validation(
                    "no open shift — please open a cashier shift before taking payment",
                ));
            }
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
            // Wave 5c: thread `ht_settings.vat_percent` through so the
            // legacy receipt header carries the hotel-configured rate.
            vat_percent: cmd.vat_percent,
        };
        // Use the payment id (not the check-in id) as the idempotency
        // discriminator so multiple payments against the same check-in
        // don't collide on the unique key.
        let key = generate_idempotency_key(&intent, payment_aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(ServiceError::from_enqueue_error)?;

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

    /// Record a refund — inserts a negative-amount `ht_payments` row,
    /// enqueues [`WritebackIntent::RefundPayment`], publishes
    /// [`DomainEvent::PaymentRefunded`].
    ///
    /// Track G2 / T4 CRIT-1 (`docs/coexistence/audit-2026-05-13.md`).
    /// Validation:
    /// - `amount_satang > 0`
    /// - original payment exists, is not voided, is not itself a refund
    /// - sum of magnitudes (this refund + prior refunds) ≤ original amount
    ///
    /// The shift gate is intentionally NOT applied: a refund is a
    /// settlement adjustment against an existing tender row, not a new
    /// cash-drawer event. If we required an open shift for refunds the
    /// receptionist would be unable to issue one during the cashier
    /// handover window — exactly when guest complaints surface most.
    pub async fn refund_payment(
        &self,
        cmd: RefundPaymentCommand,
    ) -> ServiceResult<RefundPaymentOutcome> {
        if cmd.amount_satang <= 0 {
            return Err(ServiceError::validation(format!(
                "refund amount must be positive (got {} satang)",
                cmd.amount_satang
            )));
        }

        let original = self
            .repo
            .find_for_refund(&self.pg, cmd.original_payment_id)
            .await?
            .ok_or_else(|| {
                ServiceError::not_found(format!(
                    "payment {} does not exist",
                    cmd.original_payment_id
                ))
            })?;

        if original.pay_voided.unwrap_or(false) {
            return Err(ServiceError::conflict(format!(
                "payment {} is already voided — cannot refund",
                cmd.original_payment_id
            )));
        }

        if original.refund_of_payment_id.is_some() {
            return Err(ServiceError::conflict(format!(
                "payment {} is itself a refund — cannot refund a refund",
                cmd.original_payment_id
            )));
        }

        let original_amount_baht = original.pay_amount.unwrap_or(0.0);
        let refund_amount_baht = (cmd.amount_satang as f64) / 100.0;
        let already_refunded_baht = self
            .repo
            .sum_refunded_against(&self.pg, cmd.original_payment_id)
            .await?;
        // 0.5 satang tolerance — guards against f64 round-trip drift on
        // the magnitude comparison (the canonical column is NUMERIC(12,2),
        // so the round-trip is exact in practice; the slack covers the
        // legacy float decimal-conversion path).
        if refund_amount_baht + already_refunded_baht > original_amount_baht + 0.005 {
            return Err(ServiceError::validation(format!(
                "refund amount {refund_amount_baht:.2} would exceed remaining \
                 refundable balance on payment {orig_id} \
                 (original {original_amount_baht:.2}, already refunded \
                 {already_refunded_baht:.2})",
                orig_id = cmd.original_payment_id,
            )));
        }

        let method = parse_legacy_method(&original.pay_method).ok_or_else(|| {
            ServiceError::validation(format!(
                "unknown legacy payment method {:?} on payment {}",
                original.pay_method, cmd.original_payment_id
            ))
        })?;

        let mut tx = self.pg.begin().await?;

        let refund_pay_id = self
            .repo
            .insert_refund(
                &mut tx,
                RefundInsert {
                    cin_id: original.pay_cin_id,
                    amount: refund_amount_baht,
                    method: &original.pay_method,
                    original_payment_id: cmd.original_payment_id,
                    reason: cmd.reason.as_deref(),
                    created_by: cmd.created_by.as_deref(),
                },
            )
            .await?;

        let check_in_aggregate_id =
            aggregate_uuid(AggregateKind::CheckIn, original.pay_cin_id);
        let refund_aggregate_id = aggregate_uuid(AggregateKind::Payment, refund_pay_id);
        let original_payment_aggregate_id =
            aggregate_uuid(AggregateKind::Payment, cmd.original_payment_id);

        self.repo
            .stamp_aggregate_id(&mut tx, refund_pay_id, refund_aggregate_id)
            .await?;

        let refund_reason = cmd.reason.clone().unwrap_or_default();
        let intent = WritebackIntent::RefundPayment {
            check_in_id: check_in_aggregate_id,
            original_payment_aggregate_id: Some(original_payment_aggregate_id),
            amount: Money::from_satang(cmd.amount_satang),
            method,
            refund_reason: refund_reason.clone(),
            payment_aggregate_id: Some(refund_aggregate_id),
        };
        // Discriminate by the refund row's aggregate id so a retried
        // refund against the same original payment doesn't collide on
        // the outbox unique key.
        let key = generate_idempotency_key(&intent, refund_aggregate_id);
        OutboxRepository::enqueue(&mut tx, &intent, key)
            .await
            .map_err(ServiceError::from_enqueue_error)?;

        let event = DomainEvent::PaymentRefunded {
            check_in_id: check_in_aggregate_id,
            original_payment_id: original_payment_aggregate_id,
            refund_payment_id: refund_aggregate_id,
            amount: Money::from_satang(cmd.amount_satang),
            method,
            source: cmd.source.clone(),
        };
        EventBus::publish(&mut tx, &event)
            .await
            .map_err(|err| ServiceError::outbox(err.to_string()))?;

        tx.commit().await?;

        Ok(RefundPaymentOutcome {
            refund_pay_id,
            refund_aggregate_id,
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
///
/// `Web` deliberately renders as `"qr"` (not `"web"`) so the canonical PG row
/// keeps the wire-contract value the dashboard / SSE feed has shipped with
/// since the QR feature landed — Wave 5c only routes the **legacy** column
/// (`Cin_Pay_web`); the canonical name is unchanged.
fn method_to_legacy_string(method: PaymentMethod) -> &'static str {
    match method {
        PaymentMethod::Cash => "cash",
        PaymentMethod::Credit => "credit",
        PaymentMethod::Transfer => "transfer",
        PaymentMethod::Web => "qr",
    }
}

/// Inverse of [`method_to_legacy_string`] — parses the canonical
/// `ht_payments.pay_method` string back into a [`PaymentMethod`]. Used by
/// `refund_payment` so the refund debits the same tender column as the
/// original payment.
fn parse_legacy_method(method: &str) -> Option<PaymentMethod> {
    match method {
        "cash" => Some(PaymentMethod::Cash),
        "credit" => Some(PaymentMethod::Credit),
        "transfer" => Some(PaymentMethod::Transfer),
        "qr" => Some(PaymentMethod::Web),
        _ => None,
    }
}

// -----------------------------------------------------------------------------
// tests
// -----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    //! Track F2 / T1 HIGH-5 — exercise the shift gate. Tests use the same
    //! skip-on-no-DB pattern as `service::shifts::tests` so a `cargo test`
    //! without PG still passes the rest of the suite.
    use super::*;
    use crate::domain::payment::PaymentMethod;
    use crate::outbox::event::EventSource;
    use crate::outbox::intent::RecordPaymentReceipt;
    use crate::repository::payment::PgPaymentRepository;
    use crate::service::shifts::{OpenShiftCommand, ShiftService};

    async fn try_pool() -> Option<PgPool> {
        let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgresql://postgres:REDACTED-pg-2026@localhost:5439/hotelnew".to_string()
        });
        PgPool::connect(&url).await.ok()
    }

    /// Build a `PaymentService` wired with a real `ShiftService` so the
    /// gate is exercised end-to-end.
    fn build_service_with_gate(pool: PgPool, site_id: &str) -> PaymentService {
        let repo: Arc<dyn PaymentRepository> = Arc::new(PgPaymentRepository::new());
        let outbox = Arc::new(crate::outbox::OutboxRepository::new());
        let events = Arc::new(crate::outbox::EventBus::new());
        let shifts = Arc::new(ShiftService::new(pool.clone(), site_id));
        PaymentService::new(repo, outbox, events, pool).with_shifts(shifts)
    }

    /// Minimal command that satisfies validation but stays harmless if
    /// the gate lets it through (it would still fail at the FK to
    /// `ht_checkins` — the gate must intercept first).
    fn sample_cmd() -> RecordPaymentCommand {
        RecordPaymentCommand {
            check_in_id: -1,
            // 100.00 baht = 10_000 satang. `_` separator chosen
            // unambiguously per clippy's `inconsistent_digit_grouping`.
            amount_satang: 10_000,
            method: PaymentMethod::Cash,
            reference: None,
            notes: None,
            created_by: Some("test".into()),
            receipt: RecordPaymentReceipt {
                customer_name: "Test Guest".into(),
                customer_address: String::new(),
                customer_tel: String::new(),
            },
            checkin_ds_id: None,
            price_per_night_baht: Some(100.0),
            nights: Some(1),
            vat_percent: Some(7),
            source: EventSource::System {
                reason: "F2_gate_test".into(),
            },
        }
    }

    #[tokio::test]
    async fn rejects_payment_when_no_open_shift() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping rejects_payment_when_no_open_shift — PG not reachable");
            return;
        };
        let site = "TEST_pay_gate_no_shift";
        // Defensive cleanup — make sure no leftover open shift from a
        // crashed previous run lets the test pass spuriously.
        let _ = sqlx::query("DELETE FROM ht_shifts WHERE shift_site_id = $1")
            .bind(site)
            .execute(&pool)
            .await;

        let svc = build_service_with_gate(pool.clone(), site);

        let err = svc
            .record_payment(sample_cmd())
            .await
            .expect_err("must reject when no shift is open");

        match err {
            ServiceError::Validation(msg) => {
                assert!(
                    msg.contains("no open shift"),
                    "expected gate message, got: {msg}"
                );
            }
            other => panic!("expected Validation, got {other:?}"),
        }
    }

    /// Track G2 / T4 CRIT-1 — refund must reject a zero or negative
    /// `amount_satang` at the validation gate (before any repository
    /// query). Mirrors the same shape as `record_payment`'s amount
    /// check. No PG dependency.
    #[tokio::test]
    async fn refund_rejects_zero_amount_without_touching_repo() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping refund_rejects_zero_amount — PG not reachable");
            return;
        };
        let svc = build_service_with_gate(pool.clone(), "TEST_refund_zero_amount");

        let err = svc
            .refund_payment(RefundPaymentCommand {
                original_payment_id: 1,
                amount_satang: 0,
                reason: None,
                created_by: Some("test".into()),
                source: EventSource::System {
                    reason: "G2_refund_zero".into(),
                },
            })
            .await
            .expect_err("zero refund amount must be rejected");
        match err {
            ServiceError::Validation(msg) => {
                assert!(
                    msg.contains("refund amount must be positive"),
                    "expected positive-amount message, got: {msg}"
                );
            }
            other => panic!("expected Validation, got {other:?}"),
        }
    }

    /// Track G2 / T4 CRIT-1 — refund must surface NotFound when the
    /// original payment id has no canonical row.
    #[tokio::test]
    async fn refund_rejects_unknown_original_payment() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping refund_rejects_unknown_original_payment — PG not reachable");
            return;
        };
        let svc = build_service_with_gate(pool.clone(), "TEST_refund_unknown_original");
        let err = svc
            .refund_payment(RefundPaymentCommand {
                // i32::MIN guarantees no canonical row matches (the SERIAL
                // sequence starts at 1).
                original_payment_id: -987_654,
                amount_satang: 10_000,
                reason: None,
                created_by: Some("test".into()),
                source: EventSource::System {
                    reason: "G2_refund_unknown_orig".into(),
                },
            })
            .await
            .expect_err("missing original payment must surface as NotFound");
        match err {
            ServiceError::NotFound(msg) => {
                assert!(
                    msg.contains("does not exist"),
                    "expected does-not-exist message, got: {msg}"
                );
            }
            other => panic!("expected NotFound, got {other:?}"),
        }
    }

    /// Track G2 / T4 CRIT-1 — when the original payment is already
    /// soft-voided the refund must be rejected as Conflict (refunding
    /// a voided payment would re-credit money that's already been
    /// reversed via the void path).
    #[tokio::test]
    async fn refund_rejects_when_original_already_voided() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping refund_rejects_when_original_already_voided — PG not reachable");
            return;
        };
        let site = "TEST_refund_voided_original";
        // Seed: open shift, record one payment against a freshly-inserted
        // check-in, void it, then attempt a refund. The shift, check-in,
        // and payment all get torn down at the end so reruns are clean.
        let svc = build_service_with_gate(pool.clone(), site);
        let cin_id = match seed_temp_checkin(&pool).await {
            Some(id) => id,
            None => {
                eprintln!("skipping refund_rejects_when_original_already_voided — seed failed");
                return;
            }
        };
        let pay_id = match seed_payment_for_checkin(&pool, cin_id, "cash", 100.0).await {
            Some(id) => id,
            None => {
                cleanup_temp_checkin(&pool, cin_id).await;
                eprintln!("skipping — payment seed failed");
                return;
            }
        };
        mark_payment_voided(&pool, pay_id).await;

        let err = svc
            .refund_payment(RefundPaymentCommand {
                original_payment_id: pay_id,
                amount_satang: 5_000,
                reason: Some("guest complaint".into()),
                created_by: Some("test".into()),
                source: EventSource::System {
                    reason: "G2_refund_voided".into(),
                },
            })
            .await
            .expect_err("refund against a voided payment must be rejected");

        // Cleanup before assertion so a panic still leaves the DB clean.
        cleanup_temp_payment(&pool, pay_id).await;
        cleanup_temp_checkin(&pool, cin_id).await;

        match err {
            ServiceError::Conflict(msg) => {
                assert!(
                    msg.contains("already voided"),
                    "expected already-voided message, got: {msg}"
                );
            }
            other => panic!("expected Conflict, got {other:?}"),
        }
    }

    /// Track G2 / T4 CRIT-1 — the refund magnitude (this refund plus
    /// any prior refunds against the same original) must not exceed
    /// the original payment amount. Asking for more must surface as
    /// Validation.
    #[tokio::test]
    async fn refund_rejects_when_amount_exceeds_original() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping refund_rejects_when_amount_exceeds_original — PG not reachable");
            return;
        };
        let site = "TEST_refund_amount_exceeds";
        let svc = build_service_with_gate(pool.clone(), site);
        let cin_id = match seed_temp_checkin(&pool).await {
            Some(id) => id,
            None => {
                eprintln!("skipping — seed failed");
                return;
            }
        };
        // Original payment of 100 baht. The refund attempt asks for 250 —
        // must be rejected.
        let pay_id = match seed_payment_for_checkin(&pool, cin_id, "cash", 100.0).await {
            Some(id) => id,
            None => {
                cleanup_temp_checkin(&pool, cin_id).await;
                eprintln!("skipping — payment seed failed");
                return;
            }
        };

        let err = svc
            .refund_payment(RefundPaymentCommand {
                original_payment_id: pay_id,
                amount_satang: 25_000, // 250.00 baht > 100.00 original
                reason: Some("guest complaint".into()),
                created_by: Some("test".into()),
                source: EventSource::System {
                    reason: "G2_refund_exceeds".into(),
                },
            })
            .await
            .expect_err("over-refund must be rejected");

        cleanup_temp_payment(&pool, pay_id).await;
        cleanup_temp_checkin(&pool, cin_id).await;

        match err {
            ServiceError::Validation(msg) => {
                assert!(
                    msg.contains("exceed"),
                    "expected exceed-remaining-balance message, got: {msg}"
                );
            }
            other => panic!("expected Validation, got {other:?}"),
        }
    }

    // ----- test seed helpers -----

    /// Seed a minimal `ht_checkins` row whose FK chain is satisfied (we
    /// borrow an existing customer + room when present; otherwise
    /// return None and let the test skip). Returns the new `cin_id`.
    async fn seed_temp_checkin(pool: &PgPool) -> Option<i32> {
        let cust_id: Option<i32> =
            sqlx::query_scalar("SELECT cust_id FROM ht_customers ORDER BY cust_id LIMIT 1")
                .fetch_optional(pool)
                .await
                .ok()
                .flatten();
        let room_id: Option<i32> =
            sqlx::query_scalar("SELECT room_id FROM ht_rooms_new ORDER BY room_id LIMIT 1")
                .fetch_optional(pool)
                .await
                .ok()
                .flatten();
        let (cust_id, room_id) = (cust_id?, room_id?);

        let row: Option<(i32,)> = sqlx::query_as(
            "INSERT INTO ht_checkins (cin_cust_id, cin_room_id, cin_checkin_time, \
             cin_expected_checkout, cin_total_amount, cin_status, cin_rate_per_night) \
             VALUES ($1, $2, NOW(), NOW() + INTERVAL '1 day', 100, 'occupied', 100) \
             RETURNING cin_id",
        )
        .bind(cust_id)
        .bind(room_id)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten();
        row.map(|(id,)| id)
    }

    async fn cleanup_temp_checkin(pool: &PgPool, cin_id: i32) {
        let _ = sqlx::query("DELETE FROM ht_payments WHERE pay_cin_id = $1")
            .bind(cin_id)
            .execute(pool)
            .await;
        let _ = sqlx::query("DELETE FROM ht_checkins WHERE cin_id = $1")
            .bind(cin_id)
            .execute(pool)
            .await;
    }

    async fn seed_payment_for_checkin(
        pool: &PgPool,
        cin_id: i32,
        method: &str,
        amount: f64,
    ) -> Option<i32> {
        let row: Option<(i32,)> = sqlx::query_as(
            "INSERT INTO ht_payments (pay_cin_id, pay_amount, pay_method) \
             VALUES ($1, $2::float8, $3) RETURNING pay_id",
        )
        .bind(cin_id)
        .bind(amount)
        .bind(method)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten();
        row.map(|(id,)| id)
    }

    async fn cleanup_temp_payment(pool: &PgPool, pay_id: i32) {
        let _ = sqlx::query("DELETE FROM ht_payments WHERE pay_id = $1")
            .bind(pay_id)
            .execute(pool)
            .await;
    }

    async fn mark_payment_voided(pool: &PgPool, pay_id: i32) {
        let _ = sqlx::query(
            "UPDATE ht_payments SET pay_voided = true, pay_voided_at = NOW() WHERE pay_id = $1",
        )
        .bind(pay_id)
        .execute(pool)
        .await;
    }

    /// Track G2 / T4 CRIT-1 — round-trip the canonical `pay_method`
    /// strings (`cash`/`credit`/`transfer`/`qr`) back to their
    /// [`PaymentMethod`] variants so the refund recipe debits the same
    /// tender column as the original payment.
    #[test]
    fn parse_legacy_method_roundtrips_all_variants() {
        for variant in [
            PaymentMethod::Cash,
            PaymentMethod::Credit,
            PaymentMethod::Transfer,
            PaymentMethod::Web,
        ] {
            let s = method_to_legacy_string(variant);
            assert_eq!(
                parse_legacy_method(s),
                Some(variant),
                "round-trip failed for {variant:?} via {s:?}"
            );
        }
        assert_eq!(parse_legacy_method("unknown"), None);
        assert_eq!(parse_legacy_method(""), None);
    }

    #[tokio::test]
    async fn accepts_payment_when_shift_is_open() {
        let Some(pool) = try_pool().await else {
            eprintln!("skipping accepts_payment_when_shift_is_open — PG not reachable");
            return;
        };
        let site = "TEST_pay_gate_open_shift";
        let _ = sqlx::query("DELETE FROM ht_shifts WHERE shift_site_id = $1")
            .bind(site)
            .execute(&pool)
            .await;

        // round_writeback ON so `open_shift` is permitted (Track J6 guard);
        // it enqueues an `open_round` job we clear in cleanup below.
        let shifts =
            Arc::new(ShiftService::new(pool.clone(), site).with_round_writeback(true));
        shifts
            .open_shift(OpenShiftCommand {
                opened_by: "alice".into(),
                opening_float: 500.0,
                notes: None,
            })
            .await
            .expect("open shift");

        let repo: Arc<dyn PaymentRepository> = Arc::new(PgPaymentRepository::new());
        let outbox = Arc::new(crate::outbox::OutboxRepository::new());
        let events = Arc::new(crate::outbox::EventBus::new());
        let svc = PaymentService::new(repo, outbox, events, pool.clone())
            .with_shifts(shifts.clone());

        // The gate is now open. `record_payment` will progress past the
        // gate and attempt to insert against `ht_checkins(cin_id=-1)` —
        // that FK will fail (Repository error). We assert NOT a
        // Validation/no-open-shift error to prove the gate let us
        // through.
        let err = svc
            .record_payment(sample_cmd())
            .await
            .expect_err("FK to ht_checkins must fail for cin_id=-1");

        match &err {
            ServiceError::Validation(msg) if msg.contains("no open shift") => {
                panic!("gate incorrectly rejected with open shift: {msg}");
            }
            _ => {
                // Any other error (Repository / outbox) means the gate
                // let the request through, which is the assertion.
            }
        }

        // Cleanup so reruns are deterministic.
        let _ = sqlx::query("DELETE FROM ht_shifts WHERE shift_site_id = $1")
            .bind(site)
            .execute(&pool)
            .await;
        // Track J6 — also clear the open_round job the gate-open emitted, so a
        // rerun's `MAX(shift_no)+1=1` doesn't collide on the idempotency key.
        let _ = sqlx::query(
            "DELETE FROM writeback_jobs \
             WHERE intent IN ('open_round','close_round') \
               AND payload->'payload'->>'site_id' = $1",
        )
        .bind(site)
        .execute(&pool)
        .await;
    }
}
