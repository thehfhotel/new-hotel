//! Deterministic idempotency keys for [`WritebackIntent`] enqueues.
//!
//! Per `docs/architecture.md` §4c — `writeback_jobs.idempotency_key` is `UUID
//! NOT NULL UNIQUE`. The unique constraint is the safety net that makes
//! duplicate enqueues no-ops: if the service-layer transaction is retried (e.g.
//! a network blip after the canonical INSERT but before the commit reply gets
//! back to the client), the second `OutboxRepository::enqueue` will collide
//! with the first row's key and surface a `sqlx::Error::Database` with
//! constraint name `writeback_jobs_idempotency_key_key`. The caller's
//! transaction rolls back cleanly, leaving exactly one job queued.
//!
//! For this to work, the key must be a **pure function of the intent**:
//! identical inputs ⇒ identical key. We use [`uuid::Uuid::new_v5`] (RFC 4122
//! §4.3, name-based hashing with SHA-1) over the tuple
//! `(intent_name, aggregate_id)`.
//!
//! ## Namespace choice
//!
//! UUID v5 requires a namespace UUID. We derive a stable per-project namespace
//! by v5-hashing the literal string `"new-hotel.writeback"` under
//! [`uuid::Uuid::NAMESPACE_OID`] (one of the four well-known roots from RFC
//! 4122 Appendix C). This yields a constant — committed below as a doctest
//! invariant — that lives forever: changing the literal would silently
//! invalidate every previously-issued idempotency key.
//!
//! Computed value (verified by `idempotency_namespace_is_stable` test):
//! `d86fe320-5424-58cd-8c00-50ea7d998b36`
//!
//! ## What gets hashed
//!
//! `format!("{intent_name}:{aggregate_id}")` — e.g.
//! `"create_booking:550e8400-e29b-41d4-a716-446655440000"`. Including the
//! aggregate id means the same intent on different aggregates produces
//! different keys (correct — they're different jobs); excluding payload bytes
//! means the same intent on the same aggregate is a duplicate even if e.g. a
//! `created_by` field differs (correct — we don't want two writebacks racing
//! against the same legacy row).
//!
//! If a flow legitimately needs to enqueue the same `(intent, aggregate)`
//! combo twice — e.g. two separate payments against the same check-in — the
//! caller is responsible for adding a discriminator into the key, or for using
//! a distinct intent variant. None of the current 9 variants need this.

use uuid::Uuid;

use super::intent::WritebackIntent;

/// Project-specific UUID v5 namespace for writeback idempotency keys.
///
/// Computed once as `Uuid::new_v5(&Uuid::NAMESPACE_OID, b"new-hotel.writeback")`
/// and frozen here. Tests below verify the value never drifts.
pub const WRITEBACK_NAMESPACE: Uuid =
    Uuid::from_bytes([
        0xd8, 0x6f, 0xe3, 0x20,
        0x54, 0x24,
        0x58, 0xcd,
        0x8c, 0x00,
        0x50, 0xea, 0x7d, 0x99, 0x8b, 0x36,
    ]);

/// Deterministically derive the idempotency key for a writeback intent.
///
/// Same `(intent variant, aggregate_id)` ⇒ same key. The unique constraint on
/// `writeback_jobs.idempotency_key` then turns retried enqueues into safe
/// no-ops (the second INSERT errors and rolls back the calling transaction).
pub fn generate_idempotency_key(intent: &WritebackIntent, aggregate_id: Uuid) -> Uuid {
    let payload = format!("{}:{}", intent.intent_name(), aggregate_id);
    Uuid::new_v5(&WRITEBACK_NAMESPACE, payload.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::shared::Money;
    use crate::outbox::intent::CreateBookingPayload;
    use crate::domain::shared::DateRange;
    use chrono::{TimeZone, Utc};

    /// Lock the namespace constant. If anyone edits the literal string above
    /// they must also rotate `WRITEBACK_NAMESPACE` — and breaking this test
    /// is the alarm bell that says "you just invalidated every queued job's
    /// key derivation".
    #[test]
    fn idempotency_namespace_is_stable() {
        let computed = Uuid::new_v5(&Uuid::NAMESPACE_OID, b"new-hotel.writeback");
        assert_eq!(
            computed, WRITEBACK_NAMESPACE,
            "WRITEBACK_NAMESPACE constant has drifted from its derivation. \
             If this is intentional, every `idempotency_key` produced by the \
             old constant becomes unreachable and the unique-constraint \
             dedup guarantee is silently broken."
        );
    }

    #[test]
    fn same_intent_same_aggregate_produces_same_key() {
        let booking_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let intent = WritebackIntent::CancelBooking { booking_id };

        let first = generate_idempotency_key(&intent, booking_id);
        let second = generate_idempotency_key(&intent, booking_id);

        assert_eq!(first, second);
    }

    #[test]
    fn different_aggregates_produce_different_keys() {
        let booking_a = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let booking_b = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap();

        let intent_a = WritebackIntent::CancelBooking { booking_id: booking_a };
        let intent_b = WritebackIntent::CancelBooking { booking_id: booking_b };

        assert_ne!(
            generate_idempotency_key(&intent_a, booking_a),
            generate_idempotency_key(&intent_b, booking_b),
        );
    }

    #[test]
    fn different_intents_same_aggregate_produce_different_keys() {
        let agg = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let cancel = WritebackIntent::CancelBooking { booking_id: agg };
        let check_out = WritebackIntent::CheckOut {
            check_in_id: agg,
            nights: None,
            room_price_total: None,
            product_total: None,
            net_total: None,
            pay_total: None,
            balance: None,
            cr_id: None,
            room_ds_price_total: None,
            room_ds_nights: None,
            room_ds_pay_total: None,
        };

        assert_ne!(
            generate_idempotency_key(&cancel, agg),
            generate_idempotency_key(&check_out, agg),
        );
    }

    #[test]
    fn key_ignores_payload_only_intent_variant_and_aggregate_matter() {
        // Two CreateBooking intents with the same booking_id but different
        // customer details should still collapse to the same key — we don't
        // want two parallel writebacks against the same booking row.
        let booking_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let cust = Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap();

        let payload_a = CreateBookingPayload {
            customer_id: cust,
            legacy_cust_no: None,
            customer_name: "Alice".into(),
            customer_phone: None,
            stay: DateRange {
                start: Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap(),
                end: Utc.with_ymd_and_hms(2026, 1, 2, 0, 0, 0).unwrap(),
            },
            room_no: "101".into(),
            room_type: "STD".into(),
            price: Money::from_satang(100_00),
            nights: 1,
            deposit: Money::from_satang(0),
            created_by: "user_a".into(),
            notes: None,
        };
        let mut payload_b = payload_a.clone();
        payload_b.customer_name = "Bob".into();
        payload_b.created_by = "user_b".into();

        let intent_a = WritebackIntent::CreateBooking { booking_id, payload: payload_a };
        let intent_b = WritebackIntent::CreateBooking { booking_id, payload: payload_b };

        assert_eq!(
            generate_idempotency_key(&intent_a, booking_id),
            generate_idempotency_key(&intent_b, booking_id),
        );
    }
}
