//! Idempotency-gate ⊇ reconcile-hash guard for the CT mappers.
//!
//! Sibling of [`crate::sync::projection_guard`] (Track J1). Where that
//! module locks a legacy **projection** against the columns that really
//! exist, this one locks a mapper's **idempotency gate** against the
//! fields the reconcile sweep really hashes.
//!
//! ## The invariant
//!
//! > Every CT mapper's compared-field set MUST be a SUPERSET of that
//! > entity's reconcile-hash inputs.
//!
//! Every mapper decides whether an inbound legacy change is a no-op by
//! comparing the canonical PG row (`fetch_existing`) to the freshly
//! projected legacy row (`existing_matches`). Separately,
//! `crate::scheduler::sync` hashes a narrow field set per entity to
//! detect sync lag. Violate the superset relation and a legacy edit that
//! touches ONLY an ungated-but-hashed field is:
//!
//! 1. silently skipped by the mapper (`Ok(None)`, counted `skipped`),
//! 2. so the watcher advances its watermark past the CT version,
//! 3. so the delta ages out inside the 2-day CT retention window,
//! 4. and the reconcile sweep then flags a row it can NEVER close —
//!    because `force_converge_reconcile_row` repairs by re-driving the
//!    very same gate. One blind spot disables detection AND self-heal.
//!
//! Three production incidents in this class:
//!
//! - **2026-07-06 check-ins** (`d09e756`, CH26-006020 / CH26-006039) —
//!   `Cin_Date_in`-only and `Cin_Room_Out`-only re-saves changed no
//!   compared field while both feed `checkin_canonical_hash`.
//! - **2026-07-28 bookings** — the gate compared a room COUNT while
//!   iHOTEL's field-agnostic `FrmAddBook2.SAVE_EDIT` re-writes all room
//!   lines on any edit, so a 402→403 swap held every gate term.
//! - **2026-07-28 customers** — `cust_address` was a hash input on both
//!   sides but invisible to the gate.
//!
//! Each was fixed with a hand-written one-off test. This module makes
//! the check MECHANICAL, so the next hash input cannot ship ungated.
//!
//! ## Why the lists are executable
//!
//! A pair of plain `&[&str]` consts plus a superset assert does not
//! work: nothing would bind `CUSTOMER_GATE_FIELDS` to the boolean chain
//! inside `existing_matches`, so the test stays satisfiable by editing
//! the list — exactly how the one-off tests decay. Instead:
//!
//! * **Gate side** — [`GateField`] is a *named comparator*. The
//!   production gate is `FIELDS.iter().all(|f| (f.matches)(ex, p))`, so
//!   deleting a name deletes the comparison. There is no list to edit
//!   independently of the behaviour.
//! * **Hash side** — [`HashInput`] carries the `segment` closure that
//!   produces that field's bytes, so the entity's golden-vector test
//!   (`*_hash_bytes_unchanged_for_golden_inputs`) fails the moment the
//!   table stops reproducing the production hash byte-for-byte. Adding a
//!   phantom entry or deleting a real one both break the join.
//! * **Behaviour** — each mapper's
//!   `*_hash_mutations_all_defeat_the_idempotency_gate` test runs the
//!   REAL gate against a genuinely mutated projection, so the pairing
//!   cannot be satisfied by list-editing at all.
//!
//! ## Byte compatibility is load-bearing
//!
//! `ht_reconcile_log.mssql_hash` and every `ht_*_legacy.sync_hash` ack
//! are stored SHA256s. A single byte change in the hashed body
//! invalidates all of them and triggers a full re-diff storm on the next
//! tick. [`join_hash_segments`] is therefore the single definition of
//! the `|` separator, and each entity pins its bytes with a golden
//! vector built from the literal format string it replaced.

use crate::sync::mappers::{booking, checkin, customer, room};

/// The separator between hashed field segments.
///
/// Every reconcile hash in `crate::scheduler::sync` is a `|`-joined
/// projection. Single-sourced here so a hash body and its descriptor
/// table can never disagree on the separator — see the byte-compat note
/// in the module docs.
pub(crate) const HASH_SEGMENT_SEPARATOR: &str = "|";

/// Join pre-rendered hash segments into the string that gets SHA256'd.
///
/// Byte-identical to the `format!("{}|{}|…", …)` calls it replaced.
pub(crate) fn join_hash_segments(segments: &[String]) -> String {
    segments.join(HASH_SEGMENT_SEPARATOR)
}

// =============================================================================
// Gate side — named comparators
// =============================================================================

/// One named term of a mapper's idempotency gate.
///
/// `E` is the canonical row read back from PG, `P` the projected legacy
/// row. Both are `?Sized` so a set-comparison stage can be modelled as
/// `GateField<[ExistingRoom], [CanonicalRoom]>`.
///
/// Non-capturing closures coerce to `fn` pointers in const position, so
/// these tables are `const` and cost nothing at runtime.
pub(crate) struct GateField<E: ?Sized, P: ?Sized> {
    /// The canonical (PG) column this term compares. Cited by
    /// [`HashInput::gated_by`], so renaming one without the other fails
    /// [`tests::every_hash_input_names_a_gate_compared_field`].
    pub(crate) name: &'static str,
    /// `true` when the term is a Some-only compare that mirrors
    /// `COALESCE($n, existing)` write semantics: a `None` projection
    /// must NOT force a re-apply, because the write could never converge
    /// it and the mapper would re-emit its event on every tick forever.
    ///
    /// A guarded term still satisfies the superset invariant for
    /// Some→Some movement (which is what every observed legacy edit
    /// does, including the `'C0000'` delete-cascade re-point). It does
    /// NOT protect a Some→None transition — deliberately, since the
    /// canonical write refuses that transition too.
    ///
    /// Read only by the per-mapper tests that document which hash inputs
    /// rest on a guarded term (i.e. carry that residual Some→None
    /// weakness); production never branches on it.
    #[allow(dead_code)]
    pub(crate) guarded: bool,
    /// The comparison itself. THIS is the binding: the production gate
    /// is nothing but `.all()` over these.
    pub(crate) matches: fn(&E, &P) -> bool,
}

/// Names of every term in a gate table, in declaration order.
pub(crate) fn gate_field_names<E: ?Sized, P: ?Sized>(
    fields: &[GateField<E, P>],
) -> Vec<&'static str> {
    fields.iter().map(|f| f.name).collect()
}

/// Names of the [`GateField::guarded`] (Some-only) terms.
///
/// Used by the per-mapper tests to pin WHICH terms carry the residual
/// Some→None weakness, so widening a guard is a visible decision rather
/// than a quiet one.
#[cfg(test)]
pub(crate) fn guarded_gate_field_names<E: ?Sized, P: ?Sized>(
    fields: &[GateField<E, P>],
) -> Vec<&'static str> {
    fields
        .iter()
        .filter(|f| f.guarded)
        .map(|f| f.name)
        .collect()
}

// =============================================================================
// Hash side — named segment descriptors
// =============================================================================

/// One named input of an entity's reconcile hash.
///
/// `S` is the entity's canonical-shape projection — the same type the
/// mapper's gate compares against — so `segment` and `mutate` operate on
/// exactly what the gate sees. That shared type is what lets the
/// behavioural test mutate a hash input and then assert the REAL gate
/// notices.
pub(crate) struct HashInput<S> {
    /// Canonical field name as it appears in the hash body.
    pub(crate) name: &'static str,
    /// Gate term names that make this input's movement visible to the
    /// mapper.
    ///
    /// Normally the same name as the input. It is a LIST because some
    /// inputs are legitimately covered by differently-named terms:
    ///
    /// * an alias — customers' `cust_address` is written in lock-step
    ///   with `cust_add_no` by the UPSERT, so it is gated by both;
    /// * a derived value — check-ins' effective checkout is
    ///   `cin_checkout_time` when set, else `cin_expected_checkout`, so
    ///   BOTH terms must exist;
    /// * a stage — check-ins' `legacy_room_no` is covered by the
    ///   `rooms` set-comparison stage.
    ///
    /// Every such alias is an explicit, reviewable declaration. Silence
    /// is not an option: the enforcement test rejects an empty
    /// `gated_by` on any non-lookup-key input, so "just delete the
    /// names" is not a way to make the test pass.
    pub(crate) gated_by: &'static [&'static str],
    /// `false` when the input selects the hash SHAPE instead of
    /// contributing a `|`-joined segment — check-ins' `cancelled`
    /// sentinel is the only case today. Such an input is excluded from
    /// [`hash_body`] but still carries its gate obligation and its
    /// behavioural mutation. Mislabelling a real segment this way breaks
    /// the entity's golden-vector test immediately (the join loses
    /// bytes), so this is not an escape hatch.
    pub(crate) segmented: bool,
    /// `true` for the row-identity key (`Cust_no`, `Book_ID`, `Cin_no`,
    /// `Room_no`).
    ///
    /// A lookup key is structurally stronger than a compared field: the
    /// gate SELECTs `existing` BY this value, so a changed key resolves
    /// a different canonical row (or none at all → INSERT) rather than
    /// reaching any comparator. It is therefore exempt from the
    /// behavioural mutation test, and MUST declare an empty `gated_by`.
    /// Exactly one per entity — enforced below — so demoting the real PK
    /// to smuggle a field through is not silent.
    pub(crate) lookup_key: bool,
    /// Renders this input's contribution to the hashed body. Must be
    /// byte-identical to what the production hash function receives for
    /// the same field.
    ///
    /// Invoked by the golden-vector and behavioural tests only —
    /// production hashes both sides from loose column values pulled
    /// straight out of PG / MSSQL rows. That is precisely what makes
    /// this the BINDING: the golden test fails the moment this table
    /// stops reproducing those bytes.
    #[allow(dead_code)]
    pub(crate) segment: fn(&S) -> String,
    /// Applies a realistic legacy-side edit to this input, for the
    /// behavioural test. Must (a) change `segment` and (b) defeat the
    /// gate. For a [`GateField::guarded`] term the mutation has to be
    /// Some→Some, mirroring what iHOTEL actually writes.
    ///
    /// Test-only, same as [`Self::segment`].
    #[allow(dead_code)]
    pub(crate) mutate: fn(&mut S),
}

/// Render the hashed body for `s` from its descriptor table.
///
/// Non-segmented inputs (shape selectors) are skipped; their effect is
/// applied by the entity's hash function before this is ever called.
#[cfg(test)]
pub(crate) fn hash_body<S>(inputs: &[HashInput<S>], s: &S) -> String {
    let segments: Vec<String> = inputs
        .iter()
        .filter(|i| i.segmented)
        .map(|i| (i.segment)(s))
        .collect();
    join_hash_segments(&segments)
}

// =============================================================================
// Registry
// =============================================================================

/// Name-level view of one [`HashInput`], flattened so the enforcement
/// test can reason about every entity uniformly without knowing the
/// projection types.
pub struct HashInputContract {
    pub name: &'static str,
    pub gated_by: &'static [&'static str],
    pub lookup_key: bool,
    pub segmented: bool,
}

/// Flatten a descriptor table into its name-level contract.
pub(crate) fn hash_input_contracts<S>(inputs: &[HashInput<S>]) -> Vec<HashInputContract> {
    inputs
        .iter()
        .map(|i| HashInputContract {
            name: i.name,
            gated_by: i.gated_by,
            lookup_key: i.lookup_key,
            segmented: i.segmented,
        })
        .collect()
}

/// The gate-vs-hash contract for one reconciled entity.
///
/// Both field lists are `fn()` accessors DERIVED from the executable
/// tables rather than `&'static [...]` literals. That is deliberate: a
/// literal would be a second list to keep in sync with the comparators —
/// precisely the decay mode this module exists to remove.
pub struct EntityContract {
    /// `ht_reconcile_log.table_name` vocabulary: `"customers"`,
    /// `"bookings"`, `"checkins"`, `"rooms"`.
    pub entity: &'static str,
    pub hash_inputs: fn() -> Vec<HashInputContract>,
    pub gate_field_names: fn() -> Vec<&'static str>,
    /// `true` when the mapper has NO idempotency gate and always writes
    /// (`room::apply_room_upsert`). Trivially satisfies the invariant —
    /// an always-writing mapper can never skip a hashed change.
    pub always_writes: bool,
}

/// Every entity the reconcile sweep hashes today.
///
/// Phase 6 arms append here; the enforcement tests then cover the new
/// entity automatically, and
/// [`crate::scheduler::sync::RECONCILE_RESOLVABLE_TABLES`] must gain the
/// matching `table_name`.
pub fn reconcile_entity_contracts() -> Vec<EntityContract> {
    vec![
        EntityContract {
            entity: "customers",
            hash_inputs: customer::hash_input_contract,
            gate_field_names: customer::gate_field_names,
            always_writes: false,
        },
        EntityContract {
            entity: "bookings",
            hash_inputs: booking::hash_input_contract,
            gate_field_names: booking::gate_field_names,
            always_writes: false,
        },
        EntityContract {
            entity: "checkins",
            hash_inputs: checkin::hash_input_contract,
            gate_field_names: checkin::gate_field_names,
            always_writes: false,
        },
        EntityContract {
            entity: "rooms",
            hash_inputs: room::hash_input_contract,
            // `apply_room_upsert` has no gate — see `always_writes`.
            gate_field_names: room::gate_field_names,
            always_writes: true,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    /// THE enforcement test. Every reconcile-hash input must name at
    /// least one gate term that actually compares it.
    #[test]
    fn every_hash_input_names_a_gate_compared_field() {
        for contract in reconcile_entity_contracts() {
            let gate: HashSet<&str> = (contract.gate_field_names)().into_iter().collect();
            for input in (contract.hash_inputs)() {
                if input.lookup_key {
                    assert!(
                        input.gated_by.is_empty(),
                        "{}: hash input `{}` is the row-identity key, so it must \
                         declare an EMPTY gated_by — the gate resolves `existing` \
                         BY this value and a change resolves a different row",
                        contract.entity,
                        input.name,
                    );
                    continue;
                }
                if contract.always_writes {
                    // No gate exists to name. The mapper writes on every
                    // CT row, so no hashed change can be skipped.
                    continue;
                }
                assert!(
                    !input.gated_by.is_empty(),
                    "{}: hash input `{}` declares no gating field. Every hashed \
                     input needs at least one gate term (normally the same name); \
                     an alias must be named explicitly, never left empty.",
                    contract.entity,
                    input.name,
                );
                for gated_by in input.gated_by {
                    assert!(
                        gate.contains(gated_by),
                        "GATE/HASH INVARIANT VIOLATED — {entity}: reconcile-hash \
                         input `{input}` claims to be gated by `{gated_by}`, but no \
                         such term exists in the mapper's idempotency gate \
                         (terms: {gate:?}).\n\
                         \n\
                         This is the d09e756 mechanism (2026-07-06, CH26-006020 / \
                         CH26-006039): a legacy edit touching ONLY an \
                         ungated-but-hashed field is idempotency-skipped, the \
                         watcher advances its watermark, the CT delta ages out \
                         inside the 2-day retention window, and the reconcile \
                         sweep then flags a row it can never close — \
                         `force_converge_reconcile_row` repairs by re-driving this \
                         same gate, so detection AND self-heal die together.\n\
                         \n\
                         Fix by WIDENING THE GATE (add the comparator), not by \
                         widening `gated_by`.",
                        entity = contract.entity,
                        input = input.name,
                        gated_by = gated_by,
                        gate = gate,
                    );
                }
            }
        }
    }

    /// Exactly one row-identity key per entity. Without this, demoting a
    /// real compared field to `lookup_key: true` would silently exempt it
    /// from the behavioural mutation tests.
    #[test]
    fn every_contract_declares_exactly_one_lookup_key() {
        for contract in reconcile_entity_contracts() {
            let keys: Vec<&str> = (contract.hash_inputs)()
                .into_iter()
                .filter(|i| i.lookup_key)
                .map(|i| i.name)
                .collect();
            assert_eq!(
                keys.len(),
                1,
                "{}: expected exactly one lookup-key hash input, got {:?}",
                contract.entity,
                keys,
            );
        }
    }

    /// `HT_Rooms` has no `fetch_existing` + `existing_matches` pair at
    /// all — `apply_room_upsert` UPSERTs unconditionally. That trivially
    /// satisfies the invariant, and the contract must say so explicitly
    /// rather than by having an empty gate list nobody notices.
    #[test]
    fn rooms_contract_declares_always_writes() {
        let contracts = reconcile_entity_contracts();
        let rooms = contracts
            .iter()
            .find(|c| c.entity == "rooms")
            .expect("rooms contract must be registered");
        assert!(
            rooms.always_writes,
            "room::apply_room_upsert has no idempotency gate — the contract \
             must declare always_writes so the superset check is knowingly \
             skipped rather than silently vacuous"
        );
        assert!(
            (rooms.gate_field_names)().is_empty(),
            "always_writes entities must expose no gate terms"
        );
        assert!(
            !(rooms.hash_inputs)().is_empty(),
            "rooms is still hashed by the reconcile sweep — its descriptor \
             table must stay populated so the golden-vector pin holds"
        );
    }

    /// Every registered entity must have a resolve arm in BOTH reconcile
    /// dispatches, or its rows sit unresolvable forever (the 2026-05-18
    /// rooms mistake: detection shipped, `compute_current_pg_hash` fell
    /// through to `_ => Ok(None)`).
    #[test]
    fn resolvable_tables_const_covers_every_contract_entity() {
        let resolvable: HashSet<&str> = crate::scheduler::sync::RECONCILE_RESOLVABLE_TABLES
            .iter()
            .copied()
            .collect();
        for contract in reconcile_entity_contracts() {
            assert!(
                resolvable.contains(contract.entity),
                "entity `{}` is hashed by the reconcile sweep but is not in \
                 RECONCILE_RESOLVABLE_TABLES — add its arm to BOTH \
                 `compute_current_pg_hash` and `compute_current_legacy_hash`, \
                 then list it there. Without both arms the auto-resolve sweep \
                 falls through to `_ => Ok(None)` and every row for this \
                 entity stays open forever (2026-05-18, rooms).",
                contract.entity,
            );
        }
    }

    /// Sanity: the descriptor tables are non-empty and every entity is
    /// registered exactly once.
    #[test]
    fn contract_registry_has_one_entry_per_entity() {
        let contracts = reconcile_entity_contracts();
        let names: HashSet<&str> = contracts.iter().map(|c| c.entity).collect();
        assert_eq!(
            names.len(),
            contracts.len(),
            "duplicate entity in reconcile_entity_contracts()"
        );
        for c in &contracts {
            assert!(
                !(c.hash_inputs)().is_empty(),
                "{}: empty hash-input table",
                c.entity
            );
        }
    }

    #[test]
    fn join_hash_segments_is_pipe_separated() {
        // Pins the separator the stored hashes were computed with. A
        // change here silently invalidates every `ht_reconcile_log`
        // row and every `ht_*_legacy.sync_hash` ack.
        assert_eq!(
            join_hash_segments(&["a".into(), "".into(), "c".into()]),
            "a||c"
        );
    }

    #[test]
    fn hash_body_skips_non_segmented_inputs() {
        struct S {
            a: String,
            shape: bool,
        }
        const INPUTS: [HashInput<S>; 2] = [
            HashInput {
                name: "a",
                gated_by: &["a"],
                segmented: true,
                lookup_key: false,
                segment: |s: &S| s.a.clone(),
                mutate: |s: &mut S| s.a = "z".into(),
            },
            HashInput {
                name: "shape",
                gated_by: &["shape"],
                segmented: false,
                lookup_key: false,
                segment: |s: &S| format!("shape={}", s.shape),
                mutate: |s: &mut S| s.shape = true,
            },
        ];
        let s = S {
            a: "x".into(),
            shape: false,
        };
        assert_eq!(hash_body(&INPUTS, &s), "x");
    }
}
