//! Phase 6-C — generic mirror-table probe, DARK behind
//! `RECONCILE_MIRROR_PROBE_ENABLED`.
//!
//! The canonical reconcile arms (`customers`, `rooms`, `bookings`,
//! `checkins`, and the Phase 6-A/6-B `payments` / `guest_registry` arms)
//! each hash a hand-written projection of ONE entity. That does not scale
//! to the opaque pass-through mirrors: `legacy_mirror.*` and
//! `ht_room_calendar` are written by CT mappers that copy legacy columns
//! verbatim, so there is no projection to argue about — only the question
//! "did every legacy row actually land, and is the money still the same
//! money". This module answers exactly that question for all of them with
//! one generic descriptor.
//!
//! ## Cost contract
//!
//! Steady state is **ONE UNION-ALL aggregate batch per side** per tick:
//!
//! * one PG query returning `(COUNT(*), MAX(pk), SUM(money), MIN(pk))`
//!   per mirror table;
//! * one MSSQL query returning `(COUNT_BIG(*), MAX(pk), SUM(money))` per
//!   legacy table, floored at the mirror's own `MIN(pk)` (see below).
//!
//! Only a probe whose aggregate MISMATCHES pays for a per-PK diff (two
//! narrow key scans), and only then are per-PK rows recorded. A converged
//! site therefore costs two queries total, not two per table.
//!
//! ## Scope floor — `MIN(mirror pk)`, derived, never configured
//!
//! Same lesson as [`crate::scheduler::sync`]'s `PAYMENTS_ERA_FLOOR_SQL`:
//! a mirror that was never backfilled cannot be held responsible for
//! legacy history that predates it, and reporting that history builds a
//! PERMANENTLY unresolvable backlog which starves every other entity out
//! of `auto_resolve_reconcile_log`'s age-ordered 500-row batch.
//!
//! Every mirror pk here is an app- or identity-allocated integer, so
//! `MIN(pk)` on the mirror side IS the coverage boundary and needs no date
//! column, no timezone reasoning, and no configuration. Live read-only
//! evidence, 2026-07-28 (legacy rows → floored rows vs mirror rows):
//!
//! | probe                   | HF Hotel            | HF Ville          |
//! |-------------------------|---------------------|-------------------|
//! | `HT_Cupon`              | 17894 → 17894 / 17894 | 0 / 0           |
//! | `HT_Changed_Room`       | 4063 → 4063 / 4063  | 230 / 230         |
//! | `HT_Rooms_Cancel`       | **315 → 13 / 13**   | **52 → 16 / 16**  |
//! | `HT_CheckIn_Product`    | 0 / 0               | 85 / 85           |
//! | `HT_Book_Pro`           | 0 / 0               | 47 / 47           |
//! | `HT_Deposit`            | 0 / 0               | 0 / 0             |
//! | `HT_Bill_Debt_H` / `_Ds`| 0 / 0               | 0 / 0             |
//! | `HT_Room_Status`        | **4702 → 1507 / 1298** | **2115 → 1302 / 1071** |
//!
//! `HT_Rooms_Cancel` is the case that proves the floor: it is the ONE
//! CT-mirrored table missing from
//! [`crate::scheduler::mirror::MIRROR_TRANSACTIONAL_TABLES`], so
//! `--bootstrap` never snapshotted it and its mirror starts at the day CT
//! was enabled. Unfloored it would open 302 (HF Hotel) + 36 (Ville) rows
//! that can NEVER close. Floored it is exactly converged.
//!
//! ## `ht_room_calendar` — business-key DETECTION (issue #273 remainder)
//!
//! It is in the probe set (it is a mirror in every sense that matters —
//! `HT_Room_Status` copied per night) but its `MirrorProbe` entry's id-keyed
//! shape (`rcal_legacy_id` vs `HT_Room_Status.id`) is NOT how it is detected:
//! `run_mirror_probe` excludes `ht_room_calendar` from the generic id-keyed
//! UNION-ALL batch and dispatches it to
//! [`super::sync::probe_room_calendar_business_key`] instead, which compares
//! the BUSINESS key `(room, night)` — the same key
//! [`super::sync::compute_room_calendar_business_key_pg_hash`] /
//! [`super::sync::compute_room_calendar_business_key_legacy_hash`] resolve
//! on. `observe_only: false` on the registry entry — a genuine business-key
//! divergence IS recorded.
//!
//! ### Why detection could not stay id-keyed once resolution moved
//!
//! * `RoomCalendarMapper` UPSERTs on the BUSINESS key `(rcal_room_id,
//!   rcal_date)` and explicitly documents `rcal_legacy_id` as "captured but
//!   not the conflict target"; when iHOTEL's `MAX(id)+1` allocator rebinds an
//!   id to a different (room, date) slot the mapper NULLs the old row's
//!   `rcal_legacy_id` on purpose and NOTHING ever restores it. The mirror's
//!   non-NULL id population is therefore structurally below the legacy row
//!   count — an id-keyed diff reports every one of those as `missing_pg`
//!   FOREVER, no matter what the business-key state is.
//! * The closure arm (issue #273, first half) re-keyed RESOLUTION onto the
//!   business key so a genuine gap could actually converge. Leaving
//!   DETECTION on the old id key after that would have been worse than
//!   useless: detection would open a row on the never-equal id gap, the
//!   business-key resolve arm would close that SAME row the moment the
//!   unrelated business-key counts happened to agree, and detection would
//!   re-open it the very next tick — a record/resolve churn loop, forever.
//!   [`resolve_pg_hash`] / [`resolve_legacy_hash`] (the GENERIC, id-keyed
//!   resolve arms below) are therefore never reached for this probe's
//!   `<aggregate>` row today: `compute_current_pg_hash` /
//!   `compute_current_legacy_hash` in `scheduler::sync` dispatch the
//!   business-key arm FIRST, ahead of the generic `probe_for_table` arm,
//!   for exactly this key. They remain registered as a safety net for a
//!   per-PK row a pre-#273 binary might have written (this probe is
//!   `per_pk: false`, so detection itself can never produce one).
//!
//! ### What "recorded" means here, honestly
//!
//! This change re-keys DETECTION and flips the flag; it does NOT ship a
//! remediation/re-drive path for the underlying night gap (that remains a
//! separate follow-on). A recorded calendar divergence behaves exactly like
//! `guest_registry` or `payment_ledger_probe`'s `missing_pg` rows: real,
//! actionable sync lag that no self-heal list touches (probe keys are in
//! neither `FORCE_CONVERGE_VALUE_DRIFT_TABLES` nor
//! `REINGEST_MISSING_PG_TABLES`), so it stays open, and `check_level_drift_and_alert`
//! / `send_stale_level_digest` carry NO minimum-count threshold — past 72h
//! the `:bangbang:` escalation tier WILL fire on it, same as any other
//! genuine unremediated gap. That is the intended, honest outcome of
//! detecting a real gap with no closure path yet — not a defect to route
//! around with `observe_only` again. It will converge on its own the moment
//! the business-key counts actually agree (a re-drive, or the gap closing by
//! whatever means), exactly as `should_auto_resolve` already knows how to
//! recognise (`room_calendar_business_key_row_closes_when_both_sides_agree`).
//!
//! Live counts, 2026-07-28 — the numbers this arm will act on the first time
//! it is enabled: id-keyed HF Hotel 1507 in-era legacy rows vs 1298 canonical
//! (1302 vs 1071 at Ville); business-key HF Hotel 1546 legacy nights vs 1420
//! canonical (a DIFFERENT, not-directly-comparable pair of numbers — see
//! `room_calendar_detection_and_resolution_share_one_definition_of_converged`).
//! Ville's business-key gap has not been independently measured; do not
//! assume it equals the id-keyed 1302/1071 pair above.
//!
//! ## What gets recorded
//!
//! Only [`DivergenceKind::MissingPg`], [`DivergenceKind::MissingMssql`] and
//! [`DivergenceKind::Value`]. **Never `Cardinality`** — it is filtered out
//! of BOTH `check_drift_and_alert` (`divergence_kind <> 'cardinality'`) and
//! the level digest's per-table roll-up, so a `cardinality` row is
//! alert-invisible by construction. A count mismatch here is therefore
//! classified by DIRECTION (`missing_pg` / `missing_mssql`), which is both
//! more informative and actually reaches an operator. That also means
//! `classify_divergence` is NOT used for the aggregate row: it would map a
//! count difference straight to `Cardinality`.
//!
//! ### Two row shapes
//!
//! * **per-PK** — a real legacy/mirror key, recorded when the whole diff
//!   fits in [`MIRROR_PROBE_MAX_PK_FINDINGS`];
//! * **aggregate** — `legacy_pk = "<aggregate>"`, one row standing for a
//!   whole-table divergence, used when the diff is too big to enumerate
//!   (or the probe is `per_pk: false`). Its `mssql_hash` / `pg_hash` are a
//!   STABLE per-probe sentinel rather than the live aggregate hash,
//!   deliberately: `record_divergence` dedupes on
//!   `(table_name, legacy_pk, divergence_kind, mssql_hash)`, so a moving
//!   hash would mint a fresh unresolved row every time the legacy table
//!   grew — one per day, forever, for a table with a structural gap. With
//!   the sentinel, at most one unresolved aggregate row per probe per
//!   direction accumulates. The live counts live in
//!   `mssql_row_json` / `pg_row_json`, which is what an operator reads.
//!
//! ## Resolution — resolvable, NOT excluded
//!
//! Every probe key is listed in
//! [`crate::scheduler::sync::RECONCILE_RESOLVABLE_TABLES`] and both
//! `compute_current_pg_hash` / `compute_current_legacy_hash` dispatch to
//! [`resolve_pg_hash`] / [`resolve_legacy_hash`], so the
//! `reconcile_table_fk_rank` wildcard `debug_assert!` stays quiet for the
//! right reason (an arm exists) rather than because the entity was hidden
//! from the list. The alternative — leaving probe rows OUT of
//! `RECONCILE_RESOLVABLE_TABLES` — would not trip the assert either (it
//! only fires for listed-but-undispatched names), and that is exactly why
//! it was rejected: the rows would sit open forever, silently, which is
//! the 2026-05-18 `rooms` defect with a fresh coat of paint.
//!
//! Both resolve arms return a hash for an ABSENT key too
//! ([`mirror_absent_hash`]) rather than `None`. Absent-on-both is a real,
//! converged state for a mirror — a legacy row deleted and then removed
//! from the mirror by the CT delete mapper has nothing left to reconcile —
//! and `should_auto_resolve` closes a row only when BOTH sides produce
//! equal non-empty hashes. Returning `None` would leave every such row open
//! forever.
//!
//! No self-heal: the probe writes NOTHING but `ht_reconcile_log`. Probe
//! keys are in neither `FORCE_CONVERGE_VALUE_DRIFT_TABLES` nor
//! `REINGEST_MISSING_PG_TABLES`, and re-driving an opaque mirror row is a
//! separate, coordinated decision (plan 6-D).

use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;

use serde_json::json;

use crate::db::mssql_timeout::{
    query_with_timeout_pooled, simple_query_with_timeout_pooled, MssqlOpKind,
};
use crate::db::{DbPool, PgPool};
use crate::sync::gate_guard::join_hash_segments;

use super::sync::{money_hash_segment, record_divergence, sha256, DivergenceKind};

type AnyError = Box<dyn std::error::Error + Send + Sync>;

/// `ht_reconcile_log.legacy_pk` for a whole-table (aggregate) finding.
///
/// Cannot collide with a real key: every probed PK is an integer or a
/// legacy business code, never a bracketed word.
pub(crate) const MIRROR_AGGREGATE_PK: &str = "<aggregate>";

/// Most per-PK rows a single probe may record in one tick.
///
/// The cap is what makes this arm safe to enable: `auto_resolve_reconcile_log`
/// selects its 500-row batch by `detected_at` alone with NO per-table
/// fairness, so an arm that can manufacture unclosable rows in bulk starves
/// every other entity out of the sweep. Past the cap the probe records the
/// single aggregate row instead and logs the true delta.
pub(crate) const MIRROR_PROBE_MAX_PK_FINDINGS: usize = 20;

/// One mirrored table pair.
///
/// The plan sketched `{ legacy_table, mirror_table, pk_col, sum_col }`; the
/// live schema needs three more fields, each earning its place:
/// `mirror_pk_col` (the calendar's canonical column is `rcal_legacy_id`, not
/// `id`), `mirror_filter` (canonical-only calendar rows carry no legacy id
/// and must not read as `missing_mssql`), and `numeric_pk` +`per_pk` (see
/// their docs).
pub(crate) struct MirrorProbe {
    /// `ht_reconcile_log.table_name`. Deliberately free of
    /// `COOLDOWN_KEY_NAMESPACE_SEP` (`:`) — the level-drift all-clear
    /// treats any cooldown key containing one as belonging to a different
    /// tripwire and refuses to clear it.
    pub key: &'static str,
    pub legacy_table: &'static str,
    pub mirror_table: &'static str,
    pub legacy_pk_col: &'static str,
    pub mirror_pk_col: &'static str,
    /// Money total to `SUM` on both sides. Present only where the table
    /// actually carries a charge total — a COUNT/MAX pair cannot see a
    /// re-priced row.
    pub sum_col: Option<&'static str>,
    /// Extra canonical-side predicate. Applied to the mirror aggregate,
    /// the mirror key scan AND the per-PK resolve, so all three see the
    /// same row set.
    pub mirror_filter: Option<&'static str>,
    /// `false` when the PK is not an integer, which costs the `MAX(pk)`
    /// fingerprint and the `MIN(pk)` coverage floor (`HT_Bill_Debt_H`'s
    /// `Bill_No` is a business code). Lexicographic MAX across two engines
    /// would depend on collation, so it is not attempted.
    pub numeric_pk: bool,
    /// `false` for a table whose per-row identity is not stable across the
    /// mirror boundary — `ht_room_calendar`, see the module docs. Such a
    /// probe reports only the aggregate row.
    pub per_pk: bool,
    /// `true` for a probe whose divergence is a KNOWN structural gap with no
    /// closure path yet: the aggregate is still computed and a mismatch is
    /// logged with both counts, but NOTHING is written to
    /// `ht_reconcile_log`.
    ///
    /// Only `ht_room_calendar` (see the module docs). Recording a row that
    /// `should_auto_resolve` can never close, that no self-heal repairs, and
    /// that re-mints itself when an operator resolves it by hand would pin
    /// the 4h digest and the >72h `:bangbang:` escalation tier at both sites
    /// forever. Detection with no possible remediation is an alert defect,
    /// not thoroughness — so the finding lives in the log until the arm that
    /// can close it exists.
    ///
    /// Implies `per_pk: false` (pinned by test): a probe we do not record is
    /// never worth two extra key scans.
    pub observe_only: bool,
}

/// The 8 CT-mirrored `legacy_mirror.*` tables plus `ht_room_calendar`.
///
/// The 8 come from `sync/mappers/mirror.rs` (one mapper each: `HT_Cupon`,
/// `HT_CheckIn_Product`, `HT_Deposit`, `HT_Changed_Room`, `HT_Bill_Debt_H`,
/// `HT_Bill_Debt_Ds`, `HT_Rooms_Cancel`, `HT_Book_Pro`). Note this is NOT
/// the same set as `MIRROR_TRANSACTIONAL_TABLES` (7): that list drives the
/// `--bootstrap` snapshot and is missing `HT_Rooms_Cancel`, which is why
/// that mirror needs the coverage floor most.
///
/// The 4 `legacy_mirror.*` DIMENSION tables (`ht_continuetime`,
/// `ht_rooms_price`, `ht_order_up`, `ht_order_down`) are deliberately NOT
/// here: `reload_mirror_dimensions` DELETEs and re-INSERTs them wholesale
/// every tick, so a divergence cannot survive a tick and a probe could only
/// ever report a race against the reload it shares a cycle with.
///
/// SQL identifiers are written in the LEGACY casing throughout. MSSQL is
/// case-insensitive and unquoted identifiers fold to lower case in
/// PostgreSQL, so one spelling addresses both sides.
pub(crate) const MIRROR_PROBES: &[MirrorProbe] = &[
    MirrorProbe {
        key: "mirror_ht_cupon",
        legacy_table: "HT_Cupon",
        mirror_table: "legacy_mirror.ht_cupon",
        legacy_pk_col: "cupon_no",
        mirror_pk_col: "cupon_no",
        sum_col: None,
        mirror_filter: None,
        numeric_pk: true,
        per_pk: true,
        observe_only: false,
    },
    MirrorProbe {
        key: "mirror_ht_checkin_product",
        legacy_table: "HT_CheckIn_Product",
        mirror_table: "legacy_mirror.ht_checkin_product",
        legacy_pk_col: "id",
        mirror_pk_col: "id",
        // Per-line POS/minibar total. This is the folio money iHOTEL bills
        // the guest for, so a silently re-priced line is exactly what the
        // COUNT/MAX pair cannot see.
        sum_col: Some("Cin_Pro_priceTotal"),
        mirror_filter: None,
        numeric_pk: true,
        per_pk: true,
        observe_only: false,
    },
    MirrorProbe {
        key: "mirror_ht_deposit",
        legacy_table: "HT_Deposit",
        mirror_table: "legacy_mirror.ht_deposit",
        legacy_pk_col: "id",
        mirror_pk_col: "id",
        sum_col: Some("Dep_Price"),
        mirror_filter: None,
        numeric_pk: true,
        per_pk: true,
        observe_only: false,
    },
    MirrorProbe {
        key: "mirror_ht_changed_room",
        legacy_table: "HT_Changed_Room",
        mirror_table: "legacy_mirror.ht_changed_room",
        legacy_pk_col: "id",
        mirror_pk_col: "id",
        // `room_before_price` is a snapshot of the vacated room's rate, not
        // a charge total — summing it would compare bookkeeping trivia and
        // is deliberately skipped.
        sum_col: None,
        mirror_filter: None,
        numeric_pk: true,
        per_pk: true,
        observe_only: false,
    },
    MirrorProbe {
        key: "mirror_ht_bill_debt_h",
        legacy_table: "HT_Bill_Debt_H",
        mirror_table: "legacy_mirror.ht_bill_debt_h",
        legacy_pk_col: "Bill_No",
        mirror_pk_col: "bill_no",
        sum_col: Some("Bill_Total"),
        mirror_filter: None,
        // TEXT business key — no MAX fingerprint, no coverage floor.
        numeric_pk: false,
        per_pk: true,
        observe_only: false,
    },
    MirrorProbe {
        key: "mirror_ht_bill_debt_ds",
        legacy_table: "HT_Bill_Debt_Ds",
        mirror_table: "legacy_mirror.ht_bill_debt_ds",
        legacy_pk_col: "id",
        mirror_pk_col: "id",
        sum_col: Some("DS_PRICE_TOTAL"),
        mirror_filter: None,
        numeric_pk: true,
        per_pk: true,
        observe_only: false,
    },
    MirrorProbe {
        key: "mirror_ht_rooms_cancel",
        legacy_table: "HT_Rooms_Cancel",
        mirror_table: "legacy_mirror.ht_rooms_cancel",
        legacy_pk_col: "id",
        mirror_pk_col: "id",
        sum_col: None,
        mirror_filter: None,
        numeric_pk: true,
        per_pk: true,
        observe_only: false,
    },
    MirrorProbe {
        key: "mirror_ht_book_pro",
        legacy_table: "HT_Book_Pro",
        mirror_table: "legacy_mirror.ht_book_pro",
        legacy_pk_col: "id",
        mirror_pk_col: "id",
        sum_col: Some("B_PRICE_TOTAL"),
        mirror_filter: None,
        numeric_pk: true,
        per_pk: true,
        observe_only: false,
    },
    MirrorProbe {
        key: "mirror_ht_room_calendar",
        legacy_table: "HT_Room_Status",
        mirror_table: "ht_room_calendar",
        legacy_pk_col: "id",
        mirror_pk_col: "rcal_legacy_id",
        sum_col: None,
        // App-authored calendar tiles have no legacy counterpart at all
        // (122 of 1420 rows at HF Hotel on 2026-07-28); counting them would
        // manufacture permanent `missing_mssql`.
        mirror_filter: Some("rcal_legacy_id IS NOT NULL"),
        numeric_pk: true,
        // Aggregate-only — see the module docs. The legacy id is NOT the
        // mapper's conflict target and is deliberately NULLed on rebind.
        per_pk: false,
        // Issue #273 (remainder): DETECTION is re-keyed off this id-keyed
        // shape entirely — `run_mirror_probe` skips `ht_room_calendar` in
        // the generic UNION-ALL batch below and dispatches it to
        // `sync::probe_room_calendar_business_key` instead, which compares
        // the same `(room, night)` business key the closure arm
        // (`sync::compute_room_calendar_business_key_{pg,legacy}_hash`)
        // resolves on. `observe_only: false` — a genuine business-key
        // divergence now IS recorded.
        //
        // The id-keyed fields above (`legacy_pk_col`, `mirror_pk_col`,
        // `mirror_filter`) stay as documentation of the shape this probe
        // WOULD have used and as the target for a per-PK row a pre-#273
        // binary might have recorded (the generic resolve arm below still
        // answers for it) — they are otherwise unused: the id-keyed
        // aggregate is never-equal by construction (nothing restores a
        // `rcal_legacy_id` NULLed on an allocator rebind), which is exactly
        // why detection had to move off it. `observe_only` was `true` while
        // that gap made every recorded row unclosable; the flip is safe now
        // because DETECTION and the closure arm's RESOLUTION agree on one
        // definition of "converged" (issue #273; the pairing is pinned by
        // `room_calendar_detection_is_no_longer_observe_only`).
        observe_only: false,
    },
];

/// Probe keys as they must appear in
/// [`crate::scheduler::sync::RECONCILE_RESOLVABLE_TABLES`]. Pinned against
/// that list by a unit test in `scheduler::sync`.
#[cfg(test)]
pub(crate) fn mirror_probe_keys() -> Vec<&'static str> {
    MIRROR_PROBES.iter().map(|p| p.key).collect()
}

/// Look up the probe behind an `ht_reconcile_log.table_name`.
///
/// This IS the resolve dispatch's membership test — a probe that is
/// registered above is automatically resolvable, so detection and
/// resolution cannot be added apart.
pub(crate) fn probe_for_table(table_name: &str) -> Option<&'static MirrorProbe> {
    MIRROR_PROBES.iter().find(|p| p.key == table_name)
}

// =============================================================================
// Hashes
// =============================================================================

/// Hash for ONE mirrored row that EXISTS on the side being hashed.
///
/// `sum` is `None` for a probe with no money column, and for a money row
/// whose column is NULL — the two are indistinguishable here on purpose:
/// within one probe the shape is fixed, so nothing can be confused.
pub(crate) fn mirror_row_hash(key: &str, pk: &str, sum: Option<f64>) -> String {
    sha256(&join_hash_segments(&[
        key.to_string(),
        pk.to_string(),
        sum.map(money_hash_segment).unwrap_or_default(),
    ]))
}

/// Hash for a mirrored row that is ABSENT on the side being hashed.
///
/// Absent-on-both is a converged state (see the module docs), so this must
/// be a real hash rather than `None`. `"absent"` can never be produced by
/// [`money_hash_segment`], so it cannot alias a present row.
pub(crate) fn mirror_absent_hash(key: &str, pk: &str) -> String {
    sha256(&join_hash_segments(&[
        key.to_string(),
        pk.to_string(),
        "absent".to_string(),
    ]))
}

/// Hash of a whole-table aggregate, used by the auto-resolve sweep to
/// decide whether an aggregate row has converged.
pub(crate) fn mirror_aggregate_hash(
    key: &str,
    row_count: i64,
    max_pk: Option<i64>,
    sum: Option<f64>,
) -> String {
    sha256(&join_hash_segments(&[
        key.to_string(),
        MIRROR_AGGREGATE_PK.to_string(),
        row_count.to_string(),
        max_pk.map(|v| v.to_string()).unwrap_or_default(),
        sum.map(money_hash_segment).unwrap_or_default(),
    ]))
}

/// STABLE dedupe sentinel stored in an aggregate row's hash columns.
///
/// Deliberately independent of the live counts — see the module docs on the
/// two row shapes. Changing this value would orphan the currently-open
/// aggregate rows (they would dedupe as new), so treat it as a stored
/// format.
pub(crate) fn mirror_aggregate_sentinel(key: &str) -> String {
    sha256(&join_hash_segments(&[
        key.to_string(),
        MIRROR_AGGREGATE_PK.to_string(),
        "sentinel".to_string(),
    ]))
}

// =============================================================================
// Aggregate batch
// =============================================================================

/// One side's aggregate for one probe.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct MirrorAggregate {
    pub row_count: i64,
    pub max_pk: Option<i64>,
    pub sum: Option<f64>,
    /// Mirror side only: the coverage floor pushed into the legacy scan.
    pub min_pk: Option<i64>,
}

impl MirrorAggregate {
    fn json(&self) -> serde_json::Value {
        json!({
            "row_count": self.row_count,
            "max_pk": self.max_pk,
            "sum": self.sum,
        })
    }
}

/// Do two aggregates agree? Money is compared at the same 2-decimal
/// resolution every reconcile hash uses, so a last-ULP difference between
/// MSSQL's and PostgreSQL's summation order can never masquerade as drift.
pub(crate) fn aggregates_match(legacy: &MirrorAggregate, mirror: &MirrorAggregate) -> bool {
    legacy.row_count == mirror.row_count
        && legacy.max_pk == mirror.max_pk
        && money_opt_segment(legacy.sum) == money_opt_segment(mirror.sum)
}

fn money_opt_segment(v: Option<f64>) -> String {
    v.map(money_hash_segment).unwrap_or_default()
}

/// Direction of a whole-table divergence.
///
/// NEVER `Cardinality`, even though the counts differ — see the module
/// docs: the hourly drift digest filters that kind out (`divergence_kind
/// <> 'cardinality'`), so it would go unpaged by the primary alert.
pub(crate) fn aggregate_divergence_kind(
    legacy: &MirrorAggregate,
    mirror: &MirrorAggregate,
) -> DivergenceKind {
    if legacy.row_count > mirror.row_count {
        DivergenceKind::MissingPg
    } else if mirror.row_count > legacy.row_count {
        DivergenceKind::MissingMssql
    } else {
        DivergenceKind::Value
    }
}

/// The ONE PostgreSQL aggregate batch. `MIN(pk)` rides along because it IS
/// the coverage floor for the legacy batch that follows.
pub(crate) fn pg_aggregate_sql(probes: &[&MirrorProbe]) -> String {
    probes
        .iter()
        .map(|p| {
            let (max_expr, min_expr) = if p.numeric_pk {
                (
                    format!("MAX({})::bigint", p.mirror_pk_col),
                    format!("MIN({})::bigint", p.mirror_pk_col),
                )
            } else {
                ("NULL::bigint".to_string(), "NULL::bigint".to_string())
            };
            let sum_expr = match p.sum_col {
                // Cast to NUMERIC before summing: exact decimal addition is
                // order-independent, so this cannot disagree with MSSQL's
                // DECIMAL sum for arithmetic reasons.
                Some(c) => format!("SUM({c}::numeric(19,4))::float8"),
                None => "NULL::float8".to_string(),
            };
            format!(
                "SELECT '{key}' AS probe_key, COUNT(*)::bigint AS row_count, \
                 {max_expr} AS max_pk, {sum_expr} AS sum_val, {min_expr} AS min_pk \
                 FROM {table}{filter}",
                key = p.key,
                table = p.mirror_table,
                filter = p
                    .mirror_filter
                    .map(|f| format!(" WHERE {f}"))
                    .unwrap_or_default(),
            )
        })
        .collect::<Vec<_>>()
        .join(" UNION ALL ")
}

/// The ONE MSSQL aggregate batch, floored per probe.
///
/// The floor is a `MIN(pk)` PostgreSQL itself produced and Rust formats as
/// a bare integer, so there is no injection surface and no locale-dependent
/// literal.
pub(crate) fn legacy_aggregate_sql(probes: &[(&MirrorProbe, Option<i64>)]) -> String {
    probes
        .iter()
        .map(|(p, floor)| {
            let max_expr = if p.numeric_pk {
                format!("MAX(CAST({} AS BIGINT))", p.legacy_pk_col)
            } else {
                "CAST(NULL AS BIGINT)".to_string()
            };
            let sum_expr = match p.sum_col {
                Some(c) => format!("CAST(SUM(CAST({c} AS DECIMAL(19,4))) AS FLOAT)"),
                None => "CAST(NULL AS FLOAT)".to_string(),
            };
            format!(
                "SELECT CAST('{key}' AS VARCHAR(64)) AS probe_key, \
                 COUNT_BIG(*) AS row_count, {max_expr} AS max_pk, {sum_expr} AS sum_val \
                 FROM {table}{filter}",
                key = p.key,
                table = p.legacy_table,
                filter = legacy_floor_filter(p, *floor),
            )
        })
        .collect::<Vec<_>>()
        .join(" UNION ALL ")
}

/// `WHERE pk >= <floor>`, or nothing when the mirror is empty / the PK is
/// not orderable. An empty mirror deliberately scans the WHOLE legacy table
/// — with no coverage at all, "legacy has N rows and we have none" is the
/// finding, and it lands as one bounded aggregate row.
fn legacy_floor_filter(probe: &MirrorProbe, floor: Option<i64>) -> String {
    match floor {
        Some(f) if probe.numeric_pk => format!(" WHERE {} >= {}", probe.legacy_pk_col, f),
        _ => String::new(),
    }
}

// =============================================================================
// Per-PK diff
// =============================================================================

/// One per-PK finding. The three variants map 1:1 onto the three
/// alert-visible divergence kinds.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum MirrorFinding {
    /// Legacy has the key, the mirror does not.
    MissingPg { pk: String, legacy_sum: Option<f64> },
    /// The mirror has the key, legacy does not.
    MissingMssql { pk: String, pg_sum: Option<f64> },
    /// Both sides have the key; the money moved.
    Value {
        pk: String,
        legacy_sum: Option<f64>,
        pg_sum: Option<f64>,
    },
}

impl MirrorFinding {
    pub(crate) fn pk(&self) -> &str {
        match self {
            Self::MissingPg { pk, .. } | Self::MissingMssql { pk, .. } | Self::Value { pk, .. } => {
                pk
            }
        }
    }

    fn kind(&self) -> DivergenceKind {
        match self {
            Self::MissingPg { .. } => DivergenceKind::MissingPg,
            Self::MissingMssql { .. } => DivergenceKind::MissingMssql,
            Self::Value { .. } => DivergenceKind::Value,
        }
    }
}

/// Pure diff of two key→money maps. Iteration order is the union of two
/// `BTreeMap`s, i.e. sorted and deterministic — the truncation below keeps
/// reporting the SAME prefix every tick, so `record_divergence`'s dedupe
/// suppresses the repeat instead of accumulating a fresh slice each time.
///
/// `compare_sum` is false for a probe with no money column: presence is
/// then the only observable, and two present rows are always equal.
pub(crate) fn diff_mirror_rows(
    legacy: &BTreeMap<String, Option<f64>>,
    mirror: &BTreeMap<String, Option<f64>>,
    compare_sum: bool,
) -> Vec<MirrorFinding> {
    let keys: BTreeSet<&String> = legacy.keys().chain(mirror.keys()).collect();
    let mut out = Vec::new();
    for k in keys {
        match (legacy.get(k), mirror.get(k)) {
            (Some(l), None) => out.push(MirrorFinding::MissingPg {
                pk: k.clone(),
                legacy_sum: *l,
            }),
            (None, Some(p)) => out.push(MirrorFinding::MissingMssql {
                pk: k.clone(),
                pg_sum: *p,
            }),
            (Some(l), Some(p)) => {
                if compare_sum && money_opt_segment(*l) != money_opt_segment(*p) {
                    out.push(MirrorFinding::Value {
                        pk: k.clone(),
                        legacy_sum: *l,
                        pg_sum: *p,
                    });
                }
            }
            (None, None) => unreachable!("key came from one of the two maps"),
        }
    }
    out
}

// =============================================================================
// The probe itself
// =============================================================================

/// What one probe tick measured, reported to `sync_status` by the caller.
///
/// The counts are returned rather than written here because
/// `record_success` / `record_error` live in [`crate::scheduler::sync`]
/// next to `run_sync`, which is where every other arm's status write is
/// made. A tick that returns `Ok` MUST reach `record_success`: it is the
/// only thing that zeroes `consecutive_failures` and stamps `last_sync_at`
/// (`last_error`/`last_error_at` are left as a record of the most recent
/// failure), so without it the counter migration 082 seeds would be a
/// monotonic lifetime failure count and `last_sync_at` would stay NULL
/// forever.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct MirrorProbeOutcome {
    /// Probes whose two aggregates agreed (or disagreed only by read skew).
    pub(crate) converged: usize,
    /// `ht_reconcile_log` rows written this tick (per-PK + aggregate).
    pub(crate) recorded: usize,
    /// Observe-only probes that mismatched and were logged, not recorded.
    pub(crate) observed: usize,
    /// Wall time of the whole tick.
    pub(crate) duration_ms: i32,
}

/// Run every registered probe. Called ONLY when
/// `RECONCILE_MIRROR_PROBE_ENABLED` is true — with the flag off (the
/// shipped default on every service) this is never entered and the arm
/// issues zero MSSQL and zero PG queries.
///
/// `ht_room_calendar` is deliberately EXCLUDED from the generic id-keyed
/// UNION-ALL batch below (issue #273): it is dispatched to
/// [`super::sync::probe_room_calendar_business_key`] instead, which compares
/// the `(room, night)` business key rather than `rcal_legacy_id`. See that
/// function's docs and the `MirrorProbe` entry's `observe_only` comment for
/// why detection cannot share the generic batch's shape and why it must
/// agree with the closure arm's resolve on one definition of "converged".
pub(crate) async fn run_mirror_probe(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<MirrorProbeOutcome, AnyError> {
    let start = Instant::now();
    tracing::info!("[Sync] Probing legacy mirror tables...");

    let probes: Vec<&MirrorProbe> = MIRROR_PROBES
        .iter()
        .filter(|p| p.key != super::sync::ROOM_CALENDAR_PROBE_KEY)
        .collect();

    // ── ONE PG aggregate batch ────────────────────────────────────────
    let pg_sql = pg_aggregate_sql(&probes);
    let pg_rows = sqlx::query_as::<_, (String, i64, Option<i64>, Option<f64>, Option<i64>)>(
        sqlx::AssertSqlSafe(&*pg_sql),
    )
    .fetch_all(pg_pool)
    .await?;
    let mirror_aggregates: BTreeMap<String, MirrorAggregate> = pg_rows
        .into_iter()
        .map(|(key, row_count, max_pk, sum, min_pk)| {
            (
                key,
                MirrorAggregate {
                    row_count,
                    max_pk,
                    sum,
                    min_pk,
                },
            )
        })
        .collect();

    // ── ONE MSSQL aggregate batch, floored from the mirror side ───────
    let floored: Vec<(&MirrorProbe, Option<i64>)> = probes
        .iter()
        .map(|p| {
            let floor = mirror_aggregates.get(p.key).and_then(|a| a.min_pk);
            (*p, floor)
        })
        .collect();
    let legacy_sql = legacy_aggregate_sql(&floored);
    let mut conn = legacy_pool.get().await?;
    let rows = simple_query_with_timeout_pooled(&mut conn, &legacy_sql, MssqlOpKind::Read).await?;
    drop(conn);

    let mut legacy_aggregates: BTreeMap<String, MirrorAggregate> = BTreeMap::new();
    for r in &rows {
        let Some(key) = r.get::<&str, _>("probe_key") else {
            continue;
        };
        legacy_aggregates.insert(
            key.to_string(),
            MirrorAggregate {
                row_count: r.get::<i64, _>("row_count").unwrap_or(0),
                max_pk: r.try_get::<i64, _>("max_pk").ok().flatten(),
                sum: r.try_get::<f64, _>("sum_val").ok().flatten(),
                min_pk: None,
            },
        );
    }

    let mut converged = 0usize;
    let mut recorded = 0usize;
    let mut observed = 0usize;
    // Per-key scan failures are scoped to ONE table (a lock timeout, a
    // dropped connection mid-scan). Propagating them with `?` from inside
    // the loop would abort every LATER probe in the same tick — and the
    // probe order is fixed, so a table that keeps failing would keep the
    // ones after it permanently unmeasured. That is the repo's silent-drop
    // shape. Collect the failures instead, keep measuring, and fail the
    // tick at the END so `record_error` still fires.
    let mut failed: Vec<&str> = Vec::new();

    for probe in &probes {
        let (Some(legacy), Some(mirror)) = (
            legacy_aggregates.get(probe.key),
            mirror_aggregates.get(probe.key),
        ) else {
            tracing::warn!(
                probe = probe.key,
                "[Sync] Mirror probe: aggregate batch returned no row for this probe \
                 — skipping it this tick"
            );
            continue;
        };

        if aggregates_match(legacy, mirror) {
            converged += 1;
            continue;
        }

        // KNOWN structural gap with no closure path (today: only
        // `ht_room_calendar`). Log both live counts and record NOTHING: an
        // `ht_reconcile_log` row here can never be closed by the sweep, is
        // repaired by no self-heal, and re-mints itself if an operator
        // resolves it by hand, so it would pin the 4h digest and the >72h
        // escalation tier at both sites forever. See the module docs.
        if probe.observe_only {
            observed += 1;
            tracing::warn!(
                probe = probe.key,
                legacy_rows = legacy.row_count,
                mirror_rows = mirror.row_count,
                delta = legacy.row_count - mirror.row_count,
                floor = mirror.min_pk,
                "[Sync] Mirror probe: KNOWN structural gap, observed only — no \
                 divergence recorded (no closure path exists yet; see \
                 mirror_probe module docs)"
            );
            continue;
        }

        // Skip the per-PK scans entirely when the count delta ALREADY
        // exceeds the cap: the answer would be thrown away, and for a
        // table with a structural gap (the calendar) that is two full key
        // scans every tick, forever.
        let delta = (legacy.row_count - mirror.row_count).unsigned_abs() as usize;
        if !probe.per_pk || delta > MIRROR_PROBE_MAX_PK_FINDINGS {
            record_aggregate_divergence(pg_pool, probe, legacy, mirror, delta, None).await;
            recorded += 1;
            continue;
        }

        let legacy_keys = match load_legacy_keys(legacy_pool, probe, mirror.min_pk).await {
            Ok(keys) => keys,
            Err(e) => {
                tracing::warn!(
                    probe = probe.key,
                    error = %e,
                    "[Sync] Mirror probe: legacy key scan failed — this probe is \
                     unmeasured this tick, the remaining probes still run"
                );
                failed.push(probe.key);
                continue;
            }
        };
        let mirror_keys = match load_mirror_keys(pg_pool, probe).await {
            Ok(keys) => keys,
            Err(e) => {
                tracing::warn!(
                    probe = probe.key,
                    error = %e,
                    "[Sync] Mirror probe: mirror key scan failed — this probe is \
                     unmeasured this tick, the remaining probes still run"
                );
                failed.push(probe.key);
                continue;
            }
        };
        let findings = diff_mirror_rows(&legacy_keys, &mirror_keys, probe.sum_col.is_some());

        if findings.is_empty() {
            // The aggregates disagreed but no key does: the two sides were
            // read a few milliseconds apart and a write landed between
            // them. Nothing to record — the next tick sees a stable pair.
            tracing::debug!(
                probe = probe.key,
                legacy_rows = legacy.row_count,
                mirror_rows = mirror.row_count,
                "[Sync] Mirror probe: aggregate mismatch with no per-key difference \
                 (read skew) — no divergence recorded"
            );
            converged += 1;
            continue;
        }

        if findings.len() > MIRROR_PROBE_MAX_PK_FINDINGS {
            record_aggregate_divergence(
                pg_pool,
                probe,
                legacy,
                mirror,
                delta,
                Some(findings.len()),
            )
            .await;
            recorded += 1;
            continue;
        }

        for finding in &findings {
            record_pk_divergence(pg_pool, probe, finding).await;
            recorded += 1;
        }
        tracing::warn!(
            probe = probe.key,
            findings = findings.len(),
            legacy_rows = legacy.row_count,
            mirror_rows = mirror.row_count,
            "[Sync] Mirror probe: per-key divergences recorded"
        );
    }

    // ── `ht_room_calendar`, business-key detection (issue #273) ───────
    // Excluded from the batch above; runs its own two-query comparison
    // (see `super::sync::probe_room_calendar_business_key`). A failure here
    // is scoped to this ONE probe, same isolation contract as a per-key scan
    // failure above: collected into `failed` and the tick still fails at the
    // end so `record_error` fires, but every other probe was still measured.
    match super::sync::probe_room_calendar_business_key(legacy_pool, pg_pool).await {
        Ok(super::sync::RoomCalendarProbeOutcome::Converged) => converged += 1,
        Ok(super::sync::RoomCalendarProbeOutcome::Diverged) => recorded += 1,
        Err(e) => {
            tracing::warn!(
                probe = super::sync::ROOM_CALENDAR_PROBE_KEY,
                error = %e,
                "[Sync] Mirror probe: calendar business-key probe failed — this \
                 probe is unmeasured this tick, the remaining probes still ran"
            );
            failed.push(super::sync::ROOM_CALENDAR_PROBE_KEY);
        }
    }

    let duration_ms = start.elapsed().as_millis() as i32;
    tracing::info!(
        // +1: the calendar business-key probe runs outside `probes` above.
        probes = probes.len() + 1,
        converged,
        recorded,
        observed,
        failed = failed.len(),
        duration_ms,
        "[Sync] Mirror probe complete"
    );

    // Any per-key scan failure fails the TICK (so `record_error` fires and
    // `consecutive_failures` climbs) — but only after every other probe has
    // been measured and recorded.
    if !failed.is_empty() {
        return Err(format!(
            "mirror probe: key scan failed for {} of {} probes ({}); the remaining \
             probes were measured",
            failed.len(),
            probes.len(),
            failed.join(", "),
        )
        .into());
    }

    Ok(MirrorProbeOutcome {
        converged,
        recorded,
        observed,
        duration_ms,
    })
}

async fn record_aggregate_divergence(
    pg_pool: &PgPool,
    probe: &MirrorProbe,
    legacy: &MirrorAggregate,
    mirror: &MirrorAggregate,
    delta: usize,
    findings_over_cap: Option<usize>,
) {
    let kind = aggregate_divergence_kind(legacy, mirror);
    // STABLE sentinel, not the live aggregate hash — see the module docs.
    // Placed on the side(s) that actually HAVE rows so the row still reads
    // the way its `divergence_kind` says it should.
    let sentinel = mirror_aggregate_sentinel(probe.key);
    let (mssql_hash, pg_hash) = match kind {
        DivergenceKind::MissingPg => (Some(sentinel.clone()), None),
        DivergenceKind::MissingMssql => (None, Some(sentinel.clone())),
        _ => (Some(sentinel.clone()), Some(sentinel.clone())),
    };

    tracing::warn!(
        probe = probe.key,
        legacy_rows = legacy.row_count,
        mirror_rows = mirror.row_count,
        delta,
        per_key_findings = findings_over_cap,
        kind = kind.as_str(),
        floor = mirror.min_pk,
        "[Sync] Mirror probe: whole-table divergence recorded as ONE aggregate row \
         (too large to enumerate per key, or the probe is aggregate-only)"
    );

    record_divergence(
        pg_pool,
        probe.key,
        MIRROR_AGGREGATE_PK,
        pg_hash.as_deref(),
        mssql_hash.as_deref(),
        json!({
            "scope": "aggregate",
            "legacy_table": probe.legacy_table,
            "coverage_floor_pk": mirror.min_pk,
            "aggregate": legacy.json(),
            "per_key_findings": findings_over_cap,
        }),
        Some(json!({
            "scope": "aggregate",
            "mirror_table": probe.mirror_table,
            "aggregate": mirror.json(),
        })),
        kind,
        // Real counts — informative on the row itself. They differ for the
        // two missing_* kinds by construction, which is exactly why the
        // kind above is NOT taken from `classify_divergence`.
        legacy.row_count.min(i32::MAX as i64) as i32,
        mirror.row_count.min(i32::MAX as i64) as i32,
    )
    .await;
}

async fn record_pk_divergence(pg_pool: &PgPool, probe: &MirrorProbe, finding: &MirrorFinding) {
    let pk = finding.pk();
    let (mssql_hash, pg_hash, legacy_json, pg_json, legacy_count, pg_count) = match finding {
        MirrorFinding::MissingPg { legacy_sum, .. } => (
            Some(mirror_row_hash(probe.key, pk, *legacy_sum)),
            None,
            json!({ "pk": pk, "sum": legacy_sum, "legacy_table": probe.legacy_table }),
            None,
            1,
            0,
        ),
        MirrorFinding::MissingMssql { pg_sum, .. } => (
            None,
            Some(mirror_row_hash(probe.key, pk, *pg_sum)),
            json!({ "pk": pk, "legacy_table": probe.legacy_table, "present": false }),
            Some(json!({ "pk": pk, "sum": pg_sum, "mirror_table": probe.mirror_table })),
            0,
            1,
        ),
        MirrorFinding::Value {
            legacy_sum, pg_sum, ..
        } => (
            Some(mirror_row_hash(probe.key, pk, *legacy_sum)),
            Some(mirror_row_hash(probe.key, pk, *pg_sum)),
            json!({ "pk": pk, "sum": legacy_sum, "legacy_table": probe.legacy_table }),
            Some(json!({ "pk": pk, "sum": pg_sum, "mirror_table": probe.mirror_table })),
            1,
            1,
        ),
    };

    record_divergence(
        pg_pool,
        probe.key,
        pk,
        pg_hash.as_deref(),
        mssql_hash.as_deref(),
        legacy_json,
        pg_json,
        finding.kind(),
        legacy_count,
        pg_count,
    )
    .await;
}

/// Legacy key → money map for one probe, floored at the mirror's coverage.
async fn load_legacy_keys(
    legacy_pool: &DbPool,
    probe: &MirrorProbe,
    floor: Option<i64>,
) -> Result<BTreeMap<String, Option<f64>>, AnyError> {
    let sum_expr = match probe.sum_col {
        Some(c) => format!("CAST({c} AS FLOAT)"),
        None => "CAST(NULL AS FLOAT)".to_string(),
    };
    let sql = format!(
        "SELECT CAST({pk} AS VARCHAR(64)) AS pk_text, {sum_expr} AS sum_val \
         FROM {table}{filter}",
        pk = probe.legacy_pk_col,
        table = probe.legacy_table,
        filter = legacy_floor_filter(probe, floor),
    );
    let mut conn = legacy_pool.get().await?;
    let rows = simple_query_with_timeout_pooled(&mut conn, &sql, MssqlOpKind::Read).await?;
    drop(conn);

    let mut out = BTreeMap::new();
    for r in &rows {
        let Some(pk) = r.get::<&str, _>("pk_text") else {
            continue;
        };
        out.insert(pk.to_string(), r.try_get::<f64, _>("sum_val").ok().flatten());
    }
    Ok(out)
}

/// Mirror key → money map for one probe.
async fn load_mirror_keys(
    pg_pool: &PgPool,
    probe: &MirrorProbe,
) -> Result<BTreeMap<String, Option<f64>>, sqlx::Error> {
    let sum_expr = match probe.sum_col {
        Some(c) => format!("{c}::float8"),
        None => "NULL::float8".to_string(),
    };
    let sql = format!(
        "SELECT {pk}::text AS pk_text, {sum_expr} AS sum_val FROM {table}{filter}",
        pk = probe.mirror_pk_col,
        table = probe.mirror_table,
        filter = probe
            .mirror_filter
            .map(|f| format!(" WHERE {f}"))
            .unwrap_or_default(),
    );
    let rows = sqlx::query_as::<_, (String, Option<f64>)>(sqlx::AssertSqlSafe(&*sql))
        .fetch_all(pg_pool)
        .await?;
    Ok(rows.into_iter().collect())
}

// =============================================================================
// Auto-resolve dispatch
// =============================================================================

/// Canonical-side hash for one `ht_reconcile_log` row of a probe table.
///
/// Never returns `Ok(None)` for a known probe: an absent key hashes to
/// [`mirror_absent_hash`] so absent-on-both converges (module docs).
pub(crate) async fn resolve_pg_hash(
    pg_pool: &PgPool,
    probe: &MirrorProbe,
    legacy_pk: &str,
) -> Result<Option<String>, sqlx::Error> {
    if legacy_pk == MIRROR_AGGREGATE_PK {
        let sql = pg_aggregate_sql(&[probe]);
        let row = sqlx::query_as::<_, (String, i64, Option<i64>, Option<f64>, Option<i64>)>(
            sqlx::AssertSqlSafe(&*sql),
        )
        .fetch_one(pg_pool)
        .await?;
        return Ok(Some(mirror_aggregate_hash(probe.key, row.1, row.2, row.3)));
    }

    let sum_expr = match probe.sum_col {
        Some(c) => format!("{c}::float8"),
        None => "NULL::float8".to_string(),
    };
    let sql = format!(
        "SELECT {sum_expr} AS sum_val FROM {table} WHERE {pk}::text = $1{filter} LIMIT 1",
        table = probe.mirror_table,
        pk = probe.mirror_pk_col,
        filter = probe
            .mirror_filter
            .map(|f| format!(" AND {f}"))
            .unwrap_or_default(),
    );
    let found = sqlx::query_as::<_, (Option<f64>,)>(sqlx::AssertSqlSafe(&*sql))
        .bind(legacy_pk)
        .fetch_optional(pg_pool)
        .await?;
    Ok(Some(match found {
        Some((sum,)) => mirror_row_hash(probe.key, legacy_pk, sum),
        None => mirror_absent_hash(probe.key, legacy_pk),
    }))
}

/// Legacy-side hash for one `ht_reconcile_log` row of a probe table.
///
/// Takes `pg_pool` because the aggregate comparison is only meaningful
/// inside the mirror's own coverage floor, and that floor is a `MIN(pk)`
/// over the MIRROR — the same value the detection pass used. Re-deriving it
/// here (rather than storing it on the row) means a floor that MOVES,
/// because the mirror finally received its missing history, is picked up on
/// the next sweep instead of pinning the row to a stale boundary.
pub(crate) async fn resolve_legacy_hash(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
    probe: &MirrorProbe,
    legacy_pk: &str,
) -> Result<Option<String>, AnyError> {
    if legacy_pk == MIRROR_AGGREGATE_PK {
        let floor_sql = pg_aggregate_sql(&[probe]);
        let mirror = sqlx::query_as::<_, (String, i64, Option<i64>, Option<f64>, Option<i64>)>(
            sqlx::AssertSqlSafe(&*floor_sql),
        )
        .fetch_one(pg_pool)
        .await?;
        let sql = legacy_aggregate_sql(&[(probe, mirror.4)]);
        let mut conn = legacy_pool.get().await?;
        let rows = simple_query_with_timeout_pooled(&mut conn, &sql, MssqlOpKind::Read).await?;
        drop(conn);
        let Some(r) = rows.first() else {
            return Ok(None);
        };
        return Ok(Some(mirror_aggregate_hash(
            probe.key,
            r.get::<i64, _>("row_count").unwrap_or(0),
            r.try_get::<i64, _>("max_pk").ok().flatten(),
            r.try_get::<f64, _>("sum_val").ok().flatten(),
        )));
    }

    let sum_expr = match probe.sum_col {
        Some(c) => format!("CAST({c} AS FLOAT)"),
        None => "CAST(NULL AS FLOAT)".to_string(),
    };
    let sql = format!(
        "SELECT {sum_expr} AS sum_val FROM {table} WHERE CAST({pk} AS VARCHAR(64)) = @P1",
        table = probe.legacy_table,
        pk = probe.legacy_pk_col,
    );
    let mut conn = legacy_pool.get().await?;
    let mut q = tiberius::Query::new(sql.as_str());
    q.bind(legacy_pk);
    let rows = query_with_timeout_pooled(&mut conn, &sql, q, MssqlOpKind::Read).await?;
    drop(conn);
    Ok(Some(match rows.first() {
        Some(r) => mirror_row_hash(
            probe.key,
            legacy_pk,
            r.try_get::<f64, _>("sum_val").ok().flatten(),
        ),
        None => mirror_absent_hash(probe.key, legacy_pk),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn probe(key: &str) -> &'static MirrorProbe {
        probe_for_table(key).expect("probe must be registered")
    }

    /// The registry must cover every CT mirror mapper. `sync/mappers/mirror.rs`
    /// owns 8 of them; `ht_room_calendar` is the 9th mirrored table.
    #[test]
    fn registry_covers_every_ct_mirror_mapper() {
        let src = include_str!("../sync/mappers/mirror.rs");
        for table in [
            "legacy_mirror.ht_cupon",
            "legacy_mirror.ht_checkin_product",
            "legacy_mirror.ht_deposit",
            "legacy_mirror.ht_changed_room",
            "legacy_mirror.ht_bill_debt_h",
            "legacy_mirror.ht_bill_debt_ds",
            "legacy_mirror.ht_rooms_cancel",
            "legacy_mirror.ht_book_pro",
        ] {
            assert!(
                src.contains(table),
                "{table} is not written by sync/mappers/mirror.rs — the registry \
                 below was built from that file, so re-derive it"
            );
            assert!(
                MIRROR_PROBES.iter().any(|p| p.mirror_table == table),
                "CT mirror table {table} has no MirrorProbe — a mirror with no \
                 probe is exactly the blind spot this arm exists to remove"
            );
        }
        assert!(
            MIRROR_PROBES
                .iter()
                .any(|p| p.mirror_table == "ht_room_calendar"),
            "ht_room_calendar must be probed too"
        );
        assert_eq!(MIRROR_PROBES.len(), 9);
    }

    /// The 4 wholesale-reloaded dimension mirrors must NOT be probed —
    /// `reload_mirror_dimensions` DELETEs and re-INSERTs them every tick,
    /// so any finding would be a race against that reload.
    #[test]
    fn wholesale_reloaded_dimensions_are_not_probed() {
        for table in [
            "legacy_mirror.ht_continuetime",
            "legacy_mirror.ht_rooms_price",
            "legacy_mirror.ht_order_up",
            "legacy_mirror.ht_order_down",
        ] {
            assert!(
                !MIRROR_PROBES.iter().any(|p| p.mirror_table == table),
                "{table} is reloaded wholesale each tick — probing it reports \
                 races, not sync lag"
            );
        }
    }

    /// Probe keys become `ht_reconcile_log.table_name` values AND cooldown
    /// keys in the shared `ht_level_drift_alert_cooldowns` table, where any
    /// key containing `:` is read as belonging to a different tripwire and
    /// is never cleared by the sync-lag all-clear.
    #[test]
    fn probe_keys_are_not_namespaced_and_are_unique() {
        let keys = mirror_probe_keys();
        let unique: BTreeSet<&str> = keys.iter().copied().collect();
        assert_eq!(unique.len(), keys.len(), "duplicate probe key");
        for k in keys {
            assert!(
                !k.contains(':'),
                "probe key `{k}` contains the cooldown namespace separator — \
                 the level-drift all-clear would never clear its cooldown"
            );
            assert!(
                k.starts_with("mirror_"),
                "probe key `{k}` must carry the `mirror_` prefix so an operator \
                 can tell a probe row from an entity row at a glance"
            );
        }
    }

    /// `cardinality` is filtered out of the hourly drift digest (and carries
    /// no direction an operator can act on), so a probe must never emit it —
    /// a count mismatch is reported by DIRECTION instead.
    #[test]
    fn aggregate_kind_is_never_cardinality() {
        let a = MirrorAggregate {
            row_count: 10,
            max_pk: Some(10),
            sum: None,
            min_pk: Some(1),
        };
        let fewer = MirrorAggregate {
            row_count: 4,
            max_pk: Some(4),
            sum: None,
            min_pk: Some(1),
        };
        assert_eq!(
            aggregate_divergence_kind(&a, &fewer),
            DivergenceKind::MissingPg
        );
        assert_eq!(
            aggregate_divergence_kind(&fewer, &a),
            DivergenceKind::MissingMssql
        );
        let same_count_other_money = MirrorAggregate {
            row_count: 10,
            max_pk: Some(10),
            sum: Some(1.0),
            min_pk: Some(1),
        };
        assert_eq!(
            aggregate_divergence_kind(&a, &same_count_other_money),
            DivergenceKind::Value
        );
        for legacy in [&a, &fewer, &same_count_other_money] {
            for mirror in [&a, &fewer, &same_count_other_money] {
                assert_ne!(
                    aggregate_divergence_kind(legacy, mirror),
                    DivergenceKind::Cardinality,
                    "a probe must never emit `cardinality` — the hourly drift \
                     digest excludes it and it would go unpaged"
                );
            }
        }
    }

    #[test]
    fn aggregates_match_compares_money_at_two_decimals() {
        let base = MirrorAggregate {
            row_count: 3,
            max_pk: Some(9),
            sum: Some(100.0),
            min_pk: Some(1),
        };
        let ulp = MirrorAggregate {
            sum: Some(100.000000000001),
            ..base
        };
        assert!(
            aggregates_match(&base, &ulp),
            "a last-ULP summation difference between the two engines must not \
             read as drift"
        );
        let real = MirrorAggregate {
            sum: Some(100.01),
            ..base
        };
        assert!(!aggregates_match(&base, &real));
        let count = MirrorAggregate {
            row_count: 4,
            ..base
        };
        assert!(!aggregates_match(&base, &count));
        let max = MirrorAggregate {
            max_pk: Some(10),
            ..base
        };
        assert!(!aggregates_match(&base, &max));
    }

    #[test]
    fn diff_classifies_each_direction_in_sorted_order() {
        let legacy: BTreeMap<String, Option<f64>> = [
            ("1".to_string(), Some(10.0)),
            ("2".to_string(), Some(20.0)),
            ("4".to_string(), Some(40.0)),
        ]
        .into_iter()
        .collect();
        let mirror: BTreeMap<String, Option<f64>> = [
            ("1".to_string(), Some(10.0)),
            ("2".to_string(), Some(22.0)),
            ("3".to_string(), Some(30.0)),
        ]
        .into_iter()
        .collect();
        let found = diff_mirror_rows(&legacy, &mirror, true);
        assert_eq!(
            found,
            vec![
                MirrorFinding::Value {
                    pk: "2".to_string(),
                    legacy_sum: Some(20.0),
                    pg_sum: Some(22.0)
                },
                MirrorFinding::MissingMssql {
                    pk: "3".to_string(),
                    pg_sum: Some(30.0)
                },
                MirrorFinding::MissingPg {
                    pk: "4".to_string(),
                    legacy_sum: Some(40.0)
                },
            ]
        );
    }

    /// With no money column, two present rows are always equal — otherwise
    /// a table with no comparable content would report every row forever.
    #[test]
    fn diff_without_money_only_reports_presence() {
        let legacy: BTreeMap<String, Option<f64>> =
            [("1".to_string(), Some(10.0))].into_iter().collect();
        let mirror: BTreeMap<String, Option<f64>> =
            [("1".to_string(), Some(99.0))].into_iter().collect();
        assert!(diff_mirror_rows(&legacy, &mirror, false).is_empty());
        assert_eq!(diff_mirror_rows(&legacy, &mirror, true).len(), 1);
    }

    /// The absent sentinel is what lets a key deleted on BOTH sides close.
    #[test]
    fn absent_hash_is_distinct_from_every_present_hash() {
        let key = "mirror_ht_cupon";
        let absent = mirror_absent_hash(key, "7");
        assert_ne!(absent, mirror_row_hash(key, "7", None));
        assert_ne!(absent, mirror_row_hash(key, "7", Some(0.0)));
        assert_ne!(absent, mirror_absent_hash(key, "8"));
        assert_eq!(absent, mirror_absent_hash(key, "7"));
    }

    /// The aggregate row's stored hash must NOT move with the counts, or a
    /// growing legacy table mints a fresh unresolved row every tick.
    #[test]
    fn aggregate_sentinel_is_stable_while_the_aggregate_hash_is_not() {
        let key = "mirror_ht_room_calendar";
        assert_eq!(mirror_aggregate_sentinel(key), mirror_aggregate_sentinel(key));
        assert_ne!(
            mirror_aggregate_hash(key, 10, Some(10), None),
            mirror_aggregate_hash(key, 11, Some(11), None),
        );
        assert_ne!(
            mirror_aggregate_sentinel(key),
            mirror_aggregate_hash(key, 10, Some(10), None),
        );
    }

    #[test]
    fn pg_aggregate_sql_is_one_union_all_batch() {
        let probes: Vec<&MirrorProbe> = MIRROR_PROBES.iter().collect();
        let sql = pg_aggregate_sql(&probes);
        assert_eq!(sql.matches("UNION ALL").count(), MIRROR_PROBES.len() - 1);
        assert_eq!(sql.matches("SELECT '").count(), MIRROR_PROBES.len());
        // The calendar's canonical-only tiles must be filtered out.
        assert!(sql.contains("FROM ht_room_calendar WHERE rcal_legacy_id IS NOT NULL"));
        // Money sums cast to NUMERIC first so the addition is exact.
        assert!(sql.contains("SUM(Cin_Pro_priceTotal::numeric(19,4))::float8"));
        // A TEXT pk gets no MAX/MIN fingerprint (collation-dependent).
        assert!(sql.contains("SELECT 'mirror_ht_bill_debt_h' AS probe_key, COUNT(*)::bigint AS row_count, NULL::bigint AS max_pk"));
    }

    #[test]
    fn legacy_aggregate_sql_pushes_the_coverage_floor_into_the_scan() {
        let sql = legacy_aggregate_sql(&[(probe("mirror_ht_rooms_cancel"), Some(303))]);
        assert!(
            sql.contains("FROM HT_Rooms_Cancel WHERE id >= 303"),
            "got: {sql}"
        );
        // No mirror coverage at all ⇒ no floor: "legacy has rows, we have
        // none" is the finding, and it lands as one aggregate row.
        let unfloored = legacy_aggregate_sql(&[(probe("mirror_ht_rooms_cancel"), None)]);
        assert!(unfloored.contains("FROM HT_Rooms_Cancel"));
        assert!(!unfloored.contains("WHERE"));
        // A TEXT pk is not orderable across engines, so it never gets one.
        let text_pk = legacy_aggregate_sql(&[(probe("mirror_ht_bill_debt_h"), Some(5))]);
        assert!(!text_pk.contains("WHERE"), "got: {text_pk}");
    }

    /// `ht_room_calendar` must stay aggregate-only: its legacy id is not the
    /// mapper's conflict target and is deliberately NULLed on rebind, so a
    /// per-id diff would report hundreds of unclosable `missing_pg` rows
    /// (1507 in-era legacy rows vs 1298 canonical at HF Hotel, 2026-07-28).
    #[test]
    fn room_calendar_probe_is_aggregate_only() {
        let p = probe("mirror_ht_room_calendar");
        assert!(!p.per_pk);
        assert_eq!(p.mirror_filter, Some("rcal_legacy_id IS NOT NULL"));
        assert_eq!(p.mirror_pk_col, "rcal_legacy_id");
    }

    /// Issue #273 (remainder): the flip. `observe_only` must now be
    /// `false` — detection was re-keyed onto the business key in the SAME
    /// change (this test's sibling
    /// `room_calendar_detection_and_resolution_share_one_definition_of_converged`
    /// pins that pairing), so a genuine gap is no longer unclosable and no
    /// longer merely logged.
    #[test]
    fn room_calendar_detection_is_no_longer_observe_only() {
        assert!(!probe("mirror_ht_room_calendar").observe_only);
    }

    /// The `observe_only` escape hatch is unused today (no probe sets it),
    /// but the invariant it exists to protect stays pinned for whichever
    /// probe needs it next: an observe-only probe must never also be
    /// `per_pk`, or its per-key scans would be run and their output thrown
    /// away.
    #[test]
    fn no_probe_is_observe_only_and_the_aggregate_only_invariant_still_holds() {
        let observe_only: Vec<&str> = MIRROR_PROBES
            .iter()
            .filter(|p| p.observe_only)
            .map(|p| p.key)
            .collect();
        assert_eq!(
            observe_only,
            Vec::<&str>::new(),
            "issue #273 flipped the last observe-only probe (mirror_ht_room_calendar); \
             if this fails, a NEW probe just set observe_only — apply the same rigor \
             `mirror_ht_room_calendar` got before recording anything for it"
        );
        for p in MIRROR_PROBES {
            assert!(
                !(p.observe_only && p.per_pk),
                "{} is observe-only, so its per-key scans would be discarded",
                p.key
            );
        }
    }

    /// Issue #273 (remainder) — the re-key itself: `ht_room_calendar` must
    /// be excluded from the generic id-keyed UNION-ALL batch and dispatched
    /// to the dedicated business-key probe instead. Without this,
    /// `observe_only: false` alone would make `run_mirror_probe` record the
    /// never-equal ID-KEYED aggregate — exactly the churn-loop hazard the
    /// business-key closure arm was built to avoid, just moved from
    /// "unsafe to flip" to "flipped unsafely".
    #[test]
    fn room_calendar_detection_and_resolution_share_one_definition_of_converged() {
        let src = include_str!("mirror_probe.rs");
        let start = src
            .find("pub(crate) async fn run_mirror_probe(")
            .expect("run_mirror_probe must exist");
        let body = &src[start..];

        assert!(
            body.contains(".filter(|p| p.key != super::sync::ROOM_CALENDAR_PROBE_KEY)"),
            "run_mirror_probe must exclude the calendar probe from the generic \
             id-keyed batch — it is detected on the business key instead"
        );
        assert!(
            body.contains("super::sync::probe_room_calendar_business_key(legacy_pool, pg_pool)"),
            "run_mirror_probe must dispatch ht_room_calendar to the dedicated \
             business-key probe — the one the closure arm's resolve already uses"
        );
    }

    /// Exactly the tables that carry a charge total get a SUM. The plan
    /// said "the 3 money tables"; the live schema has 5.
    #[test]
    fn money_columns_are_declared_only_where_a_charge_total_exists() {
        let with_money: Vec<&str> = MIRROR_PROBES
            .iter()
            .filter(|p| p.sum_col.is_some())
            .map(|p| p.key)
            .collect();
        assert_eq!(
            with_money,
            vec![
                "mirror_ht_checkin_product",
                "mirror_ht_deposit",
                "mirror_ht_bill_debt_h",
                "mirror_ht_bill_debt_ds",
                "mirror_ht_book_pro",
            ]
        );
    }

    /// BOTH outcomes must reach `sync_status`. `record_success` is the only
    /// statement that zeroes `consecutive_failures` and stamps `last_sync_at`
    /// (`last_error`/`last_error_at` keep the most recent failure for
    /// forensics); wiring only `record_error` (as 6-C originally shipped)
    /// turns the counter migration 082 seeds into a monotonic LIFETIME
    /// failure count and leaves `last_sync_at` NULL forever.
    #[test]
    fn run_sync_reports_both_probe_outcomes_to_sync_status() {
        let src = include_str!("sync.rs");
        let at = src
            .find("mirror_probe::run_mirror_probe(legacy_pool, pg_pool)")
            .expect("run_sync must call the probe");
        let call_site = &src[at..(at + 1200).min(src.len())];
        assert!(
            call_site.contains("record_success("),
            "the mirror-probe call site records failures but not successes — \
             `consecutive_failures` would never reset and `last_sync_at` would \
             stay NULL forever"
        );
        assert!(
            call_site.contains("record_error(pg_pool, \"mirror_probe\""),
            "the mirror-probe call site must still record failures"
        );
    }

    /// A per-key scan failure is scoped to ONE table, so it must not abort
    /// the probes after it in the fixed `MIRROR_PROBES` order — that is the
    /// repo's silent-drop shape (a failure that quietly disables downstream
    /// coverage). The tick still fails at the END so `record_error` fires.
    #[test]
    fn a_per_key_scan_failure_does_not_abort_the_remaining_probes() {
        let src = include_str!("mirror_probe.rs");
        let from = src
            .find("let mut failed: Vec<&str> = Vec::new();")
            .expect("the probe loop must collect per-probe failures");
        let to = src
            .find("let findings = diff_mirror_rows(")
            .expect("the probe loop must diff keys");
        let loop_body = &src[from..to];
        assert!(
            !loop_body.contains(".await?"),
            "a `?` inside the probe loop aborts every LATER probe in the tick"
        );
        assert_eq!(
            loop_body.matches("failed.push(probe.key);").count(),
            2,
            "both key scans must record the failure and continue"
        );
        let tail_end = src.find("\nmod tests {").expect("tests module exists");
        assert!(
            src[to..tail_end].contains("if !failed.is_empty() {"),
            "a tick with any failed probe must still return Err so \
             `record_error` fires"
        );
    }
}
