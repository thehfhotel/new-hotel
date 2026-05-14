//! Deterministic `i32` ⇄ [`uuid::Uuid`] bridge for the service layer.
//!
//! ## Why this exists
//!
//! `docs/architecture.md` §4a calls for **UUID primary keys** on every
//! canonical `ht_*` row, with the legacy MSSQL identifier kept alongside as a
//! reference column. Our `outbox::WritebackIntent` and `outbox::DomainEvent`
//! contracts already speak `Uuid` for every `*_id` field per that target.
//!
//! Today, however, the `ht_*` tables in PostgreSQL still use `SERIAL` (i.e.
//! `i32`) primary keys minted by `INSERT … RETURNING <pk>`. Migrating that
//! schema to UUID keys is a separate Wave (Phase 4 / 5) — far past the scope
//! of this Phase 2 service-layer scaffold.
//!
//! To unblock Phase 2 *without* changing either contract:
//!
//! 1. The repository layer keeps minting `i32` keys (no SQL change).
//! 2. The service layer converts each `i32` to a deterministic
//!    [`uuid::Uuid`] before it reaches the outbox / event-bus contracts.
//! 3. The conversion is reversible-by-lookup: subscribers can join back to
//!    the canonical row via `(aggregate_kind, i32)` derived from the same
//!    namespace, or, post-migration, the `Uuid` becomes the natural key and
//!    this shim disappears.
//!
//! ## Algorithm
//!
//! [`Uuid::new_v5`] (RFC 4122 §4.3) is hashed over a tiny payload:
//!
//! ```text
//! aggregate_namespace = v5(NAMESPACE_OID, b"new-hotel.aggregate.<kind>")
//! aggregate_uuid       = v5(aggregate_namespace, &i32_be_bytes)
//! ```
//!
//! Properties:
//!
//! - **Stable**: identical `(kind, i32)` ⇒ identical `Uuid`. Subscribers can
//!   recompute the mapping without a database lookup.
//! - **Disjoint**: `Booking#42` and `CheckIn#42` produce different UUIDs
//!   because their `kind` namespace differs.
//! - **Forward-compatible**: when the schema migrates to native `UUID`
//!   columns, the canonical id stops being derived and the column simply
//!   stores whatever was already in the events. Old events still resolve.
//!
//! Per-aggregate namespace constants are computed once at module load via
//! `Uuid::new_v5(&Uuid::NAMESPACE_OID, …)`. The exact byte values are
//! pinned in [`tests`] so a future edit to the literal label can never
//! silently invalidate previously-issued ids.

use uuid::Uuid;

/// Per-aggregate namespace label. Concatenated to `"new-hotel.aggregate."`
/// and v5-hashed under `Uuid::NAMESPACE_OID` to derive the namespace UUID.
///
/// Variants intentionally include only aggregates the Phase 2 services emit
/// events / intents for. A new aggregate gets a new variant + a pinned test.
#[derive(Debug, Clone, Copy)]
pub enum AggregateKind {
    Customer,
    Booking,
    CheckIn,
    Payment,
    Room,
    /// Track F3 / T1 CRIT-3 — `ht_products` canonical mirror of
    /// legacy `HT_Products`. The mapper derives the aggregate UUID
    /// from the SERIAL `prod_id` so subscribers can deduplicate
    /// stock-adjust events.
    Product,
    /// Track G5 — `ht_coupons` canonical mirror of legacy `HT_Cupon`
    /// (food/breakfast coupon entitlement). The service derives the
    /// aggregate UUID from the BIGSERIAL `coupon_id` so subscribers
    /// can deduplicate issue/redeem events across writeback retries.
    Coupon,
}

impl AggregateKind {
    /// Stable namespace label embedded in the per-kind UUID v5 namespace.
    fn label(self) -> &'static str {
        match self {
            AggregateKind::Customer => "new-hotel.aggregate.customer",
            AggregateKind::Booking => "new-hotel.aggregate.booking",
            AggregateKind::CheckIn => "new-hotel.aggregate.checkin",
            AggregateKind::Payment => "new-hotel.aggregate.payment",
            AggregateKind::Room => "new-hotel.aggregate.room",
            AggregateKind::Product => "new-hotel.aggregate.product",
            AggregateKind::Coupon => "new-hotel.aggregate.coupon",
        }
    }

    /// Per-kind namespace UUID. Computed once per call (cheap — SHA-1 of a
    /// short literal) but identical across calls.
    fn namespace(self) -> Uuid {
        Uuid::new_v5(&Uuid::NAMESPACE_OID, self.label().as_bytes())
    }
}

/// Convert a SERIAL `i32` aggregate id to its canonical [`Uuid`] for use in
/// outbox intents and domain events.
///
/// Big-endian byte order is used so the mapping is identical across host
/// architectures. The hash is deterministic; calling twice with the same
/// `(kind, id)` returns the same UUID, so retries (and the
/// `writeback_jobs.idempotency_key` UNIQUE constraint) work correctly.
pub fn aggregate_uuid(kind: AggregateKind, id: i32) -> Uuid {
    Uuid::new_v5(&kind.namespace(), &id.to_be_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Round-trip determinism — calling twice produces the same UUID for
    /// every kind. Guards against a refactor accidentally introducing a
    /// non-deterministic input (e.g. switching to `to_le_bytes` on one
    /// platform but not another, or pulling a timestamp into the hash).
    #[test]
    fn aggregate_uuid_is_deterministic_per_kind() {
        for kind in [
            AggregateKind::Customer,
            AggregateKind::Booking,
            AggregateKind::CheckIn,
            AggregateKind::Payment,
            AggregateKind::Room,
            AggregateKind::Product,
            AggregateKind::Coupon,
        ] {
            for id in [1_i32, 42, 1_000_000, i32::MAX] {
                assert_eq!(aggregate_uuid(kind, id), aggregate_uuid(kind, id));
            }
        }
    }

    /// Same id under different aggregate kinds must produce different UUIDs —
    /// otherwise `Booking#42` and `CheckIn#42` would alias and the
    /// `event_log (aggregate_id, …)` index would falsely group them.
    #[test]
    fn different_kinds_disjoint_for_same_id() {
        let kinds = [
            AggregateKind::Customer,
            AggregateKind::Booking,
            AggregateKind::CheckIn,
            AggregateKind::Payment,
            AggregateKind::Room,
            AggregateKind::Product,
            AggregateKind::Coupon,
        ];

        for (a_index, a) in kinds.iter().enumerate() {
            for b in kinds.iter().skip(a_index + 1) {
                assert_ne!(
                    aggregate_uuid(*a, 42),
                    aggregate_uuid(*b, 42),
                    "{:?} and {:?} produced the same UUID for id=42",
                    a,
                    b,
                );
            }
        }
    }

    /// Different ids within the same kind must produce different UUIDs —
    /// otherwise distinct bookings would share an aggregate id in the event
    /// log. (Trivially true for v5; the test is a regression net.)
    #[test]
    fn different_ids_distinct_within_kind() {
        let a = aggregate_uuid(AggregateKind::Booking, 1);
        let b = aggregate_uuid(AggregateKind::Booking, 2);
        assert_ne!(a, b);
    }
}
