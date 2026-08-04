//! Phase 6-D — `ht_payment_ledger` per-FOLIO SUM probe, DARK behind
//! `RECONCILE_PAYMENT_LEDGER_PROBE_ENABLED`.
//!
//! The Phase 6-A `payments` arm reconciles the RECEIPT artefact
//! (`HT_Receipt_H` ↔ `ht_payments`). This one reconciles the per-line
//! TENDER LEDGER underneath it: legacy `HT_CheckIn_Pay` ↔ canonical
//! `ht_payment_ledger`, the table `sync/mappers/payment.rs::mirror_payment_ledger`
//! writes and `routes/new_shifts.rs::round_report` reads. A folio missing
//! from that mirror is money missing from the round report, silently.
//!
//! ## The unit is the FOLIO (`Cin_No`), not the line
//!
//! Same lesson as the 6-B guest-registry arm, and it is forced by the
//! mapper: `mirror_payment_ledger` does
//! `DELETE … WHERE ledger_cin_no = $1` then re-INSERTs the loader's
//! current line set for that `Cin_No`. The whole folio is therefore the
//! unit that is written atomically, and a partially-mirrored folio is not
//! a state the mapper can produce. A line-keyed probe would additionally
//! report churn: iHOTEL re-writes a folio's `HT_CheckIn_Pay` lines on
//! edit, minting fresh IDENTITY values.
//!
//! ## What is compared, and the tender-replication trap
//!
//! Three values per `Cin_No`, all of them cheap by-products of one
//! GROUP BY on each side:
//!
//! * `line_count` — every line, cancelled ones included. Catches a
//!   dropped or duplicated ledger line.
//! * `amount_sum` — `SUM` of the per-line total, summed RAW across all
//!   lines. `ledger_amount` is genuinely itemized (one line per room on a
//!   multi-room stay, plus one per product), so raw is correct — this is
//!   the same basis `round_report`'s sales-by-category uses.
//! * `tender_sum` — `SUM` of `cash+credit+free+tran+web` over ACTIVE
//!   lines, **DEDUPED to one line per receipt**.
//!
//! That dedupe is the house convention and it is not optional. iHOTEL
//! stores the tender split REPLICATED on every line of a multi-line
//! receipt, so a raw tender sum double/triple-counts the money a receipt
//! actually took (Ville round 816: raw 17,255 vs iHOTEL 11,005). The
//! dedupe key is the SAME expression `ROUND_INCOME_BY_TENDER_SQL` uses —
//! `COALESCE(NULLIF(pay_no, ''), 'lid:' || <line id>)`, so a blank/NULL
//! `Pay_No` line counts as its own receipt instead of collapsing every
//! such line into one — and the representative is the lowest line id on
//! both sides. Strictly, a raw-vs-raw comparison would also be a valid
//! equality test (both sides replicate identically); it is deduped anyway
//! because the number is WRITTEN INTO the reconcile row an operator then
//! reads, and a 3×-inflated "legacy took ฿17,255" would send them
//! hunting a discrepancy that does not exist.
//!
//! Restricting the tender sum to ACTIVE lines (`Cin_Status = '1'`, the
//! same filter `round_report` applies) is what makes a missed cancel flip
//! visible: iHOTEL cancels a folio by UPDATEing
//! `HT_CheckIn_Pay.Cin_Status` to `'ยกเลิก'`, which moves neither the line
//! count nor the line total, so without this the drift with the largest
//! money consequence would be the one drift the probe could not see.
//!
//! Money is compared through [`money_hash_segment`] (2 dp) on both sides,
//! so a last-ULP difference between MSSQL's and PostgreSQL's summation
//! order can never masquerade as drift. That guards the SUM, not its
//! INPUTS: legacy `float` lines carry sub-satang values `numeric(14,2)`
//! structurally cannot store, so each side is also reduced to 2 dp PER
//! LINE before it is summed — see [`pg_folio_sql`] for the invariant and
//! the live folio `CH26-004002` that forced it.
//!
//! ## Scope floor — derived `MIN(ledger_legacy_id)`, CLAMPED to a persisted
//! watermark
//!
//! Identical reasoning to Phase 6-C's `MIN(mirror pk)` and 6-A's
//! `PAYMENTS_ERA_FLOOR_SQL`: `ht_payment_ledger` was born with the Track
//! J7e backfill, while `HT_CheckIn_Pay` goes back to 2021. Live counts,
//! read-only 2026-07-28:
//!
//! | side                          | HF Hotel        | HF Ville      |
//! |-------------------------------|-----------------|---------------|
//! | legacy lines / folios (all)   | 28,598 / 20,281 | 2,888 / 2,146 |
//! | canonical lines / folios      | 1,828 / 1,281   | 1,431 / 1,015 |
//! | legacy folios IN ERA          | 1,300           | 1,016         |
//!
//! Unfloored, the first enabled tick would weigh ~20k legacy folios
//! against ~1.3k canonical ones. Floored, HF Ville is EXACTLY converged
//! (1,016 folios, all three values equal on both sides) and HF Hotel
//! yields exactly 19 findings, all `missing_pg`, all contiguous at the era
//! boundary (`CH26-004952`…`CH26-004971`, minus `CH26-004960`) — real
//! folios whose payments the backfill did not reach, i.e. money the round
//! report under-counts today. Zero `value` and zero `missing_mssql` at
//! either site.
//!
//! Coverage state as of 2026-08-01, and it differs per site:
//!
//! * **HF Ville** — the `--all` backfill HAS run. Canonical coverage is
//!   complete, the derived floor is 39014 (the absolute legacy minimum) and
//!   the in-era set is the whole legacy table.
//! * **HF Hotel** — the `--all` backfill has **NOT** run; it is scheduled
//!   for the night of 2026-08-01. Until it completes, canonical coverage is
//!   still the Track J7e window, the derived floor is still that window's
//!   boundary, and the 19 `missing_pg` folios above are still uncovered.
//!
//! That asymmetry is an ENABLE-ORDER constraint, not trivia: do not turn
//! this probe on at a site before that site's `--all` backfill has
//! completed. The ratchet below persists whatever floor the first enabled
//! tick sees, and a floor seeded from the narrow pre-backfill window would
//! then HOLD after the backfill widens coverage — permanently excluding the
//! folios the backfill just landed. If that happens, the fix is to
//! `DELETE FROM ht_reconcile_era_floor WHERE table_name =
//! 'payment_ledger_probe'` at that site and let the next tick re-derive.
//!
//! **In-era means the WHOLE folio is in era** (`HAVING MIN(id) >= floor`,
//! not `WHERE id >= floor`). One HF Hotel folio straddles the boundary;
//! admitting it on a filtered line subset would manufacture a permanent
//! `value` finding out of a folio the mirror never claimed to cover.
//!
//! **BOTH scans carry the floor**, on the same whole-folio `HAVING` rule —
//! `MIN(ledger_legacy_id)` canonically, `MIN(id)` on legacy. Flooring only
//! the legacy side is a defect, not an economy: whenever the ratchet holds
//! (`effective > derived`) every canonical folio under the watermark would
//! have no legacy counterpart in scope and would be reported as
//! `missing_mssql`. Small-N that churns per tick (the probe re-mints the row
//! each tick, the sweep closes it again through the UNfloored single-folio
//! projection); large-N it collapses into one `<aggregate>` row that can
//! NEVER resolve, because resolution would be comparing an unfloored
//! canonical total against a floored legacy one. Both scans are pinned to
//! [`LedgerEraFloor::effective`] by test.
//!
//! ### The floor RATCHETS (migration 084) — it happened, it is not theory
//!
//! A raw `MIN(ledger_legacy_id)` is a LOW-water mark, and it is only a valid
//! coverage boundary while coverage is an **id-contiguous suffix** of the
//! legacy table. Mirroring does not preserve that:
//! `mirror_payment_ledger` DELETEs a `Cin_No`'s lines and re-INSERTs the
//! whole current set, so anything that selects folios by DATE lands WHOLE
//! folios, old line ids included.
//!
//! Live, 2026-07-30 → 2026-08-01. A `backfill_payment_ledger --days=212`
//! run mirrored whole folios, and ONE monthly-billed long-stay at HF Ville
//! (`CH25-000076`) carried a payment line from 2025-08. That dragged
//! `MIN(ledger_legacy_id)` from ~40470 back to 39113 — about seven months —
//! and the next tick's `HAVING MIN(id) >= floor` admitted **404 legacy
//! folios the mirror had never covered**, reporting every one as
//! `missing_pg`. Zero were genuine gaps. Remediated the same day at the
//! affected site by a Ville `--all` backfill (Ville's floor is now 39014,
//! the absolute legacy minimum); HF Hotel's `--all` is scheduled for the
//! night of 2026-08-01 and has not run yet, which is why the probe stays
//! dark there until it has. The same shape reaches here without any
//! backfill at all: iHOTEL editing a payment on a pre-era folio makes the
//! CT tick mirror that folio whole.
//!
//! So the derived value is now CLAMPED to a persisted, non-decreasing
//! watermark — the same machinery as the 6-B `guest_registry` arm
//! (`RECONCILE_ERA_FLOOR_UPSERT_SQL`, `clamped_era_floor`), on the ID basis
//! `ht_reconcile_era_floor.era_floor_id` that migration 084 adds. The floor
//! can only ratchet FORWARD; a poisoning row can never LOWER it again.
//!
//! **Not a date basis, deliberately.** The obvious alternative — floor on
//! `ledger_pay_date` instead — is worse: that column is `TIMESTAMPTZ`
//! against a legacy side storing naive local Thai time, so it reintroduces
//! exactly the timezone reasoning the integer IDENTITY basis avoids.
//!
//! [`PAYMENT_LEDGER_MAX_FOLIO_FINDINGS`] remains the second belt, for the
//! case where the very FIRST (bootstrap) reading is already poisoned:
//! the diff exceeds the cap and lands ONE self-describing aggregate row
//! ("legacy 20,281 folios vs canonical 1,300"), not a flood. The 404-folio
//! episode above is precisely what that looks like from the outside —
//! which is why [`payment_ledger_era_floor`] LOGS the derived and effective
//! floors whenever they differ, and why a hold that PERSISTS raises a Slack
//! alert (`scheduler::sync::note_payment_ledger_era_floor_hold`), so the
//! next attempt to drag the floor — or a stale watermark left behind by a
//! coverage-widening backfill — is visible rather than silent.
//!
//! ## Cost contract
//!
//! Per tick: THREE PostgreSQL round trips (the derived `MIN`, the
//! clamp-and-persist upsert that returns the watermark, one grouped scan)
//! and ONE MSSQL grouped scan. There is no cheap "aggregate first, diff only
//! on mismatch" gate as in 6-C, and there deliberately shouldn't be: with
//! a single table the totals cannot be computed without the same GROUP BY
//! the diff needs, so a gate would double the scans it was meant to
//! avoid. What crosses the wire is ~1.3k rows per side.
//!
//! Neither scan is floored in its `WHERE` clause — on both sides the floor
//! is a `HAVING` over the unfiltered per-folio `MIN(id)`, which is what
//! makes the straddling-folio rule above expressible. The legacy half is one
//! hash-aggregate pass over a 28k-row table on a shared server, growing ~10k
//! rows/year.
//!
//! ## What gets recorded
//!
//! Only [`DivergenceKind::MissingPg`], [`DivergenceKind::MissingMssql`]
//! and [`DivergenceKind::Value`]. **Never `Cardinality`** — it is filtered
//! out of BOTH `check_drift_and_alert` and the level digest
//! (`divergence_kind <> 'cardinality'`), so such a row is alert-invisible
//! by construction. A folio-count mismatch is therefore classified by
//! DIRECTION, and `classify_divergence` is deliberately not used for the
//! aggregate row (it maps a count difference straight to `Cardinality`).
//!
//! Two row shapes, exactly as 6-C: per-folio while the diff fits in
//! [`PAYMENT_LEDGER_MAX_FOLIO_FINDINGS`], otherwise ONE
//! `legacy_pk = "<aggregate>"` row whose stored hash is a STABLE sentinel
//! — `record_divergence` dedupes on
//! `(table_name, legacy_pk, divergence_kind, mssql_hash)`, so a hash that
//! moved with the live counts would mint a fresh unresolved row every time
//! a payment was taken. The live numbers live in the row JSON, which is
//! what an operator actually reads.
//!
//! ## Resolution — resolvable, NOT excluded
//!
//! [`PAYMENT_LEDGER_PROBE_KEY`] is in
//! [`crate::scheduler::sync::RECONCILE_RESOLVABLE_TABLES`], ranked in
//! `reconcile_table_fk_rank`, and dispatched by BOTH
//! `compute_current_pg_hash` and `compute_current_legacy_hash` — the same
//! mechanism Phase 6-C chose for the mirror probes, and for the same
//! reason: a row nothing can close sits open forever and, being selected
//! by age alone, eventually owns the auto-resolve sweep's whole 500-row
//! batch (the 2026-05-18 `rooms` defect).
//!
//! Both resolve arms hash an ABSENT folio ([`folio_absent_hash`]) rather
//! than returning `None`, so a folio deleted on both sides converges.
//!
//! **The resolve path NEVER writes the watermark.** Both `<aggregate>` arms
//! go through [`payment_ledger_era_floor_readonly`] — the same derived `MIN`
//! and the same clamp as detection, but a plain `SELECT` of the persisted
//! floor instead of the ratchet upsert. The reason is that the auto-resolve
//! dispatch in `scheduler::sync` is NOT gated on
//! `RECONCILE_PAYMENT_LEDGER_PROBE_ENABLED` (it dispatches on
//! `ht_reconcile_log.table_name`), so a sweep at a site where this probe is
//! still DARK could otherwise bake in a watermark before that site's `--all`
//! backfill has widened coverage — persisting exactly the stale floor the
//! enable-order constraint above exists to avoid. The probe's own tick is
//! the single writer.
//!
//! And these rows really are closeable, which is why — unlike 6-C's
//! `ht_room_calendar` — this arm records rather than merely observes: the
//! remedy for a `missing_pg` folio is the existing one-shot
//! `cargo run --release --bin backfill_payment_ledger`, which re-drives
//! `mirror_payment_ledger` for the affected `Cin_No`s. The next sweep then
//! finds equal hashes and resolves the row.
//!
//! ## No self-heal
//!
//! [`PAYMENT_LEDGER_PROBE_KEY`] is in NEITHER
//! `FORCE_CONVERGE_VALUE_DRIFT_TABLES` nor `REINGEST_MISSING_PG_TABLES`,
//! and must not be added to either here. Per-arm self-heal extensions come
//! only AFTER that arm's detection has soaked in production (plan Phase 6
//! rollout: Ville → 48h → HF Hotel), because a self-heal wired at the same
//! time as its detector repairs against an unproven projection.

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

/// `ht_reconcile_log.table_name` / `sync_status.entity_type` for this
/// probe. One literal for both, so a single operator query joins them.
///
/// Deliberately free of `COOLDOWN_KEY_NAMESPACE_SEP` (`:`) — the
/// level-drift all-clear treats any cooldown key containing one as
/// belonging to a different tripwire and refuses to clear it. The
/// `_probe` suffix marks it as an OBSERVED table rather than a healable
/// entity arm, the same job 6-C's `mirror_` prefix does.
pub(crate) const PAYMENT_LEDGER_PROBE_KEY: &str = "payment_ledger_probe";

/// `ht_reconcile_log.legacy_pk` for a whole-table (aggregate) finding.
///
/// Cannot collide with a real key: every `Cin_No` is a legacy business
/// code (`CH26-004951`), never a bracketed word.
pub(crate) const PAYMENT_LEDGER_AGGREGATE_PK: &str = "<aggregate>";

/// Most per-folio rows this probe may record in one tick.
///
/// The cap is what makes the arm safe to enable: `auto_resolve_reconcile_log`
/// selects its 500-row batch by `detected_at` alone with NO per-table
/// fairness, so an arm that can manufacture rows in bulk starves every
/// other entity out of the sweep. Past the cap the probe records ONE
/// aggregate row instead and logs the true finding count.
///
/// 50, chosen against the 19 findings HF Hotel had on 2026-07-28 (Ville had
/// 0) — findings the HF Hotel `--all` backfill scheduled for the night of
/// 2026-08-01 is expected to close, and which are still open until it runs:
/// comfortably above the live number so the arm reports actionable per-folio
/// detail rather than an opaque total, and far enough below 500 that it can
/// never own the sweep. It stays the SECOND belt on the dragged-floor failure
/// mode — the first is now the persisted ratchet
/// ([`payment_ledger_era_floor`]), which bounds the case the cap only
/// contains.
pub(crate) const PAYMENT_LEDGER_MAX_FOLIO_FINDINGS: usize = 50;

/// Is this `ht_reconcile_log.table_name` this probe's?
///
/// The membership test behind BOTH resolve dispatches — sibling of
/// `mirror_probe::probe_for_table`, so detection and resolution cannot be
/// added apart.
pub(crate) fn is_payment_ledger_probe(table_name: &str) -> bool {
    table_name == PAYMENT_LEDGER_PROBE_KEY
}

// =============================================================================
// The compared shape
// =============================================================================

/// One folio's three compared values, on one side.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct FolioSums {
    /// Every ledger line of the folio, cancelled ones included.
    pub line_count: i64,
    /// Itemized per-line totals, summed RAW (never deduped).
    pub amount: f64,
    /// Tenders over ACTIVE lines, deduped to one line per receipt.
    pub tender: f64,
}

impl FolioSums {
    fn json(&self) -> serde_json::Value {
        json!({
            "line_count": self.line_count,
            "amount_sum": self.amount,
            "tender_sum": self.tender,
        })
    }

    /// Equality at the reconcile money resolution (2 dp), never raw `f64`.
    fn agrees_with(&self, other: &Self) -> bool {
        self.line_count == other.line_count
            && money_hash_segment(self.amount) == money_hash_segment(other.amount)
            && money_hash_segment(self.tender) == money_hash_segment(other.tender)
    }
}

/// Both sides' totals over the in-era folio set. Only ever stored in an
/// aggregate row's JSON and hashed by [`aggregate_hash`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LedgerTotals {
    pub folio_count: i64,
    pub line_count: i64,
    pub amount: f64,
    pub tender: f64,
}

impl LedgerTotals {
    fn of(folios: &BTreeMap<String, FolioSums>) -> Self {
        Self {
            folio_count: folios.len() as i64,
            line_count: folios.values().map(|f| f.line_count).sum(),
            amount: folios.values().map(|f| f.amount).sum(),
            tender: folios.values().map(|f| f.tender).sum(),
        }
    }

    fn json(&self) -> serde_json::Value {
        json!({
            "folio_count": self.folio_count,
            "line_count": self.line_count,
            "amount_sum": self.amount,
            "tender_sum": self.tender,
        })
    }
}

// =============================================================================
// Hashes
// =============================================================================

/// Hash for one folio that EXISTS on the side being hashed.
pub(crate) fn folio_hash(cin_no: &str, sums: &FolioSums) -> String {
    sha256(&join_hash_segments(&[
        PAYMENT_LEDGER_PROBE_KEY.to_string(),
        cin_no.to_string(),
        format!("lines={}", sums.line_count),
        money_hash_segment(sums.amount),
        money_hash_segment(sums.tender),
    ]))
}

/// Hash for a folio that is ABSENT on the side being hashed.
///
/// Absent-on-both is a real converged state (a folio voided away in
/// iHOTEL and removed from the mirror has nothing left to reconcile), and
/// `should_auto_resolve` closes a row only when BOTH sides produce equal
/// non-empty hashes — returning `None` would leave every such row open
/// forever. `"absent"` can never be produced by [`money_hash_segment`], so
/// it cannot alias a present folio.
pub(crate) fn folio_absent_hash(cin_no: &str) -> String {
    sha256(&join_hash_segments(&[
        PAYMENT_LEDGER_PROBE_KEY.to_string(),
        cin_no.to_string(),
        "absent".to_string(),
    ]))
}

/// Hash of one side's whole in-era totals — what the auto-resolve sweep
/// compares for an `<aggregate>` row.
pub(crate) fn aggregate_hash(totals: &LedgerTotals) -> String {
    sha256(&join_hash_segments(&[
        PAYMENT_LEDGER_PROBE_KEY.to_string(),
        PAYMENT_LEDGER_AGGREGATE_PK.to_string(),
        format!("folios={}", totals.folio_count),
        format!("lines={}", totals.line_count),
        money_hash_segment(totals.amount),
        money_hash_segment(totals.tender),
    ]))
}

/// STABLE dedupe sentinel stored in an aggregate row's hash columns.
///
/// Deliberately independent of the live totals — see the module docs on
/// the two row shapes. Changing this value would orphan the currently-open
/// aggregate rows (they would dedupe as new), so treat it as a stored
/// format.
pub(crate) fn aggregate_sentinel() -> String {
    sha256(&join_hash_segments(&[
        PAYMENT_LEDGER_PROBE_KEY.to_string(),
        PAYMENT_LEDGER_AGGREGATE_PK.to_string(),
        "sentinel".to_string(),
    ]))
}

// =============================================================================
// SQL
// =============================================================================

/// The mirror's own coverage boundary, DERIVED. A `MIN` over an integer
/// IDENTITY column: no date column, no timezone reasoning, no
/// configuration.
///
/// This value is an input to [`payment_ledger_era_floor`], not the floor
/// itself — see the module docs on the ratchet. It is a LOW-water mark and
/// one whole-folio mirror of a pre-coverage row drags it backwards.
pub(crate) const PG_FLOOR_SQL: &str =
    "SELECT MIN(ledger_legacy_id)::bigint FROM ht_payment_ledger";

/// `ht_reconcile_era_floor.table_name` for this arm.
///
/// The row identity is [`PAYMENT_LEDGER_PROBE_KEY`] itself — the SAME
/// literal this arm already uses for `ht_reconcile_log.table_name` and
/// `sync_status.entity_type`, so one operator query joins all four and the
/// key cannot drift between them. It is a different literal from the 6-B
/// arm's `guest_registry` row in the same table (pinned by test below), so
/// the two arms share the table without colliding on its primary key.
///
/// The table lives in the CANONICAL database, and each site runs this probe
/// against its own logical PG database (`hotelnew` / `hotelville`, see the
/// canonical-PG split — site scoping is connection-level). One row per site
/// per arm therefore falls out for free: HF Hotel's watermark cannot clamp
/// Ville's, and the two sites' legacy IDENTITY ranges (which overlap and
/// mean nothing to each other) never meet.
const PAYMENT_LEDGER_ERA_FLOOR_KEY: &str = PAYMENT_LEDGER_PROBE_KEY;

/// Persist-and-clamp in ONE statement, ID basis: the durable floor only
/// ever moves FORWARD.
///
/// Sibling of `scheduler::sync::RECONCILE_ERA_FLOOR_UPSERT_SQL` — same
/// table, same idiom, the other column (`era_floor_id BIGINT`, migration
/// 084). `GREATEST` lives in SQL rather than Rust for the same reason it
/// does there: the backend scheduler and `bin/sync` can both tick the same
/// database, so the monotonic guarantee has to hold under concurrency, not
/// just within one process. `RETURNING` hands back the post-clamp value, so
/// the read and the write are the same round trip.
///
/// PostgreSQL's `GREATEST` ignores NULL inputs, so a row that somehow
/// exists with a NULL `era_floor_id` is seeded by the first upsert rather
/// than swallowing it.
///
/// An operator CAN still move the floor forward by hand
/// (`UPDATE ht_reconcile_era_floor SET era_floor_id = … WHERE table_name =
/// 'payment_ledger_probe'`) — that is the documented remedy when a
/// bootstrap reading came out too low — and `GREATEST` makes the edit
/// stick.
const ERA_FLOOR_ID_UPSERT_SQL: &str =
    "INSERT INTO ht_reconcile_era_floor (table_name, era_floor_id) VALUES ($1, $2) \
     ON CONFLICT (table_name) DO UPDATE \
        SET era_floor_id = GREATEST(ht_reconcile_era_floor.era_floor_id, EXCLUDED.era_floor_id), \
            updated_at = NOW() \
     RETURNING era_floor_id";

/// Read the durable floor without writing. Two callers, via
/// [`persisted_floor`]: the whole read-only resolve path, and the detection
/// path when the derived floor is NULL (nothing to clamp with, and an INSERT
/// would violate the migration-084 CHECK that a row carry a floor in at
/// least one basis).
const ERA_FLOOR_ID_SELECT_SQL: &str =
    "SELECT era_floor_id FROM ht_reconcile_era_floor WHERE table_name = $1";

/// One tick's coverage boundary, in both the form the mirror currently
/// implies and the form the scan actually uses.
///
/// Carrying both is what makes a drag attempt VISIBLE: `effective >
/// derived` means the persisted watermark is holding the scan forward, i.e.
/// something just mirrored a pre-coverage folio whole.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LedgerEraFloor {
    /// `MIN(ledger_legacy_id)` — what the mirror's CURRENT content implies.
    pub(crate) derived: Option<i64>,
    /// Post-clamp `GREATEST(persisted watermark, derived)`. What the legacy
    /// scan is floored at, and what gets written into an aggregate row.
    pub(crate) effective: Option<i64>,
}

impl LedgerEraFloor {
    /// No coverage boundary in either form — the only value the two
    /// SINGLE-folio projections are ever built with, since those drop the
    /// floor by construction (an already-admitted row must stay
    /// re-projectable if the floor later moves forward). Naming it, rather
    /// than passing a hand-built `LedgerEraFloor { derived: None, effective:
    /// None }`, keeps "this call is deliberately unfloored" readable at the
    /// call site.
    pub(crate) const UNFLOORED: Self = Self {
        derived: None,
        effective: None,
    };

    /// Is the persisted watermark HOLDING the scope forward against a
    /// derived value that dropped below it? (The 2026-07-30 shape.)
    ///
    /// `pub(crate)` because the ALERT for a hold that persists lives in
    /// [`crate::scheduler::sync`], next to every other Slack send in this
    /// subsystem — the same split the outcome counts already use
    /// (`record_success` / `record_error` are made at the call site, not
    /// here).
    pub(crate) fn watermark_holding(&self) -> bool {
        matches!((self.effective, self.derived), (Some(e), Some(d)) if e > d)
    }
}

/// Effective coverage floor = `GREATEST(persisted era_floor_id,
/// MIN(ledger_legacy_id))`, persisting the max back through the ratchet.
///
/// First enabled tick at a site: nothing persisted, so the derived `MIN`
/// seeds the watermark — the honest absolute legacy minimum ONLY IF that
/// site's `--all` backfill has already made coverage complete (true at Ville
/// since 2026-08-01, not yet at HF Hotel). That is the whole enable-order
/// constraint in the module docs: this seeding is durable, and a floor
/// seeded from a narrower coverage window keeps holding afterwards. From
/// then on a whole-folio mirror of a pre-coverage row moves `derived` but
/// not `effective`.
///
/// **Empty-ledger contract, preserved.** With NO canonical rows at all and
/// nothing ever persisted, both are `None` and the caller's existing
/// behaviour stands: an unfloored `legacy_folio_sql` emits no `HAVING`, admits
/// the WHOLE legacy table, and the cap lands the "legacy has 20k folios and
/// we have none" finding as ONE bounded aggregate row. Nothing is written
/// to `ht_reconcile_era_floor` in that state (an INSERT with neither basis
/// would violate the migration-084 CHECK anyway). If a watermark DOES exist
/// and the mirror has since been emptied, the watermark is KEPT — same rule
/// as the 6-B arm: re-widening to "no coverage" is the same flood by
/// another route, and the aggregate row plus the warning below is the
/// correct way to say a mirror vanished.
pub(crate) async fn payment_ledger_era_floor(
    pg_pool: &PgPool,
) -> Result<LedgerEraFloor, sqlx::Error> {
    let derived = derived_floor(pg_pool).await?;

    let persisted: Option<i64> = match derived {
        Some(d) => sqlx::query_scalar::<_, Option<i64>>(ERA_FLOOR_ID_UPSERT_SQL)
            .bind(PAYMENT_LEDGER_ERA_FLOOR_KEY)
            .bind(d)
            .fetch_optional(pg_pool)
            .await?
            .flatten(),
        None => persisted_floor(pg_pool).await?,
    };

    let floor = LedgerEraFloor {
        derived,
        effective: crate::scheduler::sync::clamped_era_floor(persisted, derived),
    };

    // Say it out loud when the watermark is holding: that is a
    // pre-coverage folio having been mirrored whole (the 2026-07-30
    // `--days=212` shape, or an iHOTEL edit on an old folio), which would
    // otherwise silently widen the scan by months.
    if floor.watermark_holding() {
        tracing::info!(
            derived_floor = ?floor.derived,
            effective_floor = ?floor.effective,
            "[Sync] Payment-ledger probe: derived coverage floor sits BELOW the \
             persisted watermark (a pre-coverage folio was mirrored whole) — \
             holding the watermark; scope stays monotonically narrowing"
        );
    } else if floor.derived.is_none() && floor.effective.is_some() {
        tracing::warn!(
            effective_floor = ?floor.effective,
            "[Sync] Payment-ledger probe: ht_payment_ledger is now EMPTY but a \
             coverage watermark exists — keeping it. Expect one aggregate \
             divergence row if the mirror really was lost"
        );
    }

    Ok(floor)
}

/// The SAME floor [`payment_ledger_era_floor`] computes, WITHOUT the
/// ratchet write. Used by both auto-resolve arms.
///
/// The sweep must floor its re-projection exactly the way detection floored
/// the scan that wrote the row, or an `<aggregate>` row is compared against a
/// different folio set every time and can never close. It must NOT, however,
/// persist anything: `auto_resolve_reconcile_log` dispatches on
/// `ht_reconcile_log.table_name` and is not gated on
/// `RECONCILE_PAYMENT_LEDGER_PROBE_ENABLED`, so a sweep at a DARK site would
/// otherwise seed the watermark from whatever coverage that site happens to
/// have — which, before its `--all` backfill, is precisely the stale narrow
/// floor the ratchet would then hold forever. Detection is the single writer.
///
/// Two reads, both trivial: the derived `MIN` and the persisted row. They are
/// paid only when an `<aggregate>` row is actually being resolved.
pub(crate) async fn payment_ledger_era_floor_readonly(
    pg_pool: &PgPool,
) -> Result<LedgerEraFloor, sqlx::Error> {
    let derived = derived_floor(pg_pool).await?;
    let persisted = persisted_floor(pg_pool).await?;
    Ok(LedgerEraFloor {
        derived,
        effective: crate::scheduler::sync::clamped_era_floor(persisted, derived),
    })
}

/// The derived half of the floor — the ONLY reader of [`PG_FLOOR_SQL`], so
/// nothing can reach the raw `MIN` and skip the clamp.
///
/// `AssertSqlSafe`: the statement is a compile-time const; no runtime value
/// reaches the text.
async fn derived_floor(pg_pool: &PgPool) -> Result<Option<i64>, sqlx::Error> {
    sqlx::query_scalar::<_, Option<i64>>(sqlx::AssertSqlSafe(PG_FLOOR_SQL))
        .fetch_one(pg_pool)
        .await
}

/// The persisted half, READ-ONLY. Used by the resolve path always, and by
/// the detection path when there is nothing to clamp with (a derived `NULL`
/// would otherwise INSERT a row carrying neither basis, violating the
/// migration-084 CHECK).
async fn persisted_floor(pg_pool: &PgPool) -> Result<Option<i64>, sqlx::Error> {
    Ok(
        sqlx::query_scalar::<_, Option<i64>>(ERA_FLOOR_ID_SELECT_SQL)
            .bind(PAYMENT_LEDGER_ERA_FLOOR_KEY)
            .fetch_optional(pg_pool)
            .await?
            .flatten(),
    )
}

/// Canonical per-folio aggregate, floored at the SAME coverage boundary as
/// the legacy scan.
///
/// Takes the whole [`LedgerEraFloor`] and reads
/// [`LedgerEraFloor::effective`] itself, rather than an `Option<i64>` the
/// caller extracts: the one mistake this builder has to be immune to is
/// being fed the DERIVED value, and a caller that cannot pass a bare integer
/// cannot make it. The remaining way to get it wrong — reading `.derived`
/// HERE — is a behavioural change the floor tests catch.
///
/// `single = true` narrows it to one `Cin_No` (bound as `$1`) for the
/// auto-resolve sweep and DROPS the floor, mirroring
/// [`legacy_folio_sql`]: that path re-projects a folio the scan already
/// admitted and logged, so it must reproduce that row's hash even if the
/// floor has since moved forward. The projection is otherwise IDENTICAL, so
/// a row's resolve hash is computed exactly the way detection computed it.
///
/// The floor is a `HAVING` over the per-folio `MIN(ledger_legacy_id)` — the
/// whole-folio rule, identical to the legacy side, so the two scans admit
/// the same era. Flooring only legacy would turn every canonical folio under
/// a HELD watermark into a phantom `missing_mssql` (module docs).
/// `ledger_legacy_id` is `NOT NULL`, so the `MIN` cannot go NULL and quietly
/// drop a folio from the canonical side alone.
///
/// The inner `ROW_NUMBER()` is the receipt dedupe; `rn = 1` picks the
/// lowest-id line of each receipt, matching the legacy side and
/// `ROUND_INCOME_BY_TENDER_SQL`'s `DISTINCT ON … ORDER BY … ledger_id`.
/// `numeric(19,4)` before summing keeps the addition exact and
/// order-independent — but that rules out ACCUMULATION error only, and on
/// its own it does NOT make the two sides comparable.
///
/// ## The comparable-precision invariant (live case: folio `CH26-004002`)
///
/// `ledger_amount` and the five tender columns are `numeric(14,2)`:
/// canonical stores money at 2 dp and structurally cannot hold anything
/// finer — it rounds on write. `HT_CheckIn_Pay` holds the same values as
/// `float`, and iHOTEL derives a line total as `Cin_Pay_Ds_PriceOne *
/// Cin_Pay_Ds_Num` in float, so legacy routinely carries SUB-SATANG line
/// values canonical has already rounded away.
///
/// Line 55149 of folio `CH26-004002` (HF Hotel, found 2026-08-04) is the
/// live case: legacy `Cin_Pay_Ds_Price = 2552.2649999999999`, canonical
/// `ledger_amount = 2552.27`, and the money actually taken — the tender —
/// is 2552.27 on BOTH sides. Summed as-is the folio read legacy 6380.665
/// against canonical 6380.67 and landed a `value` finding that NO backfill
/// could ever close: re-mirroring the folio re-derives exactly the 2 dp
/// value canonical already holds. Same unfixable family as the mirror
/// arm's app-authored rows (#273 / #281) — detection with no possible
/// remediation, aging into the >72h escalation tier over half a satang.
///
/// The real invariant is therefore: **each side must be reduced to the
/// precision canonical can store BEFORE the `SUM`.** The canonical side
/// needs no rounding — `numeric(14,2)` storage already did it, per line —
/// which is precisely why [`legacy_folio_sql`] rounds the legacy side per
/// LINE and not per folio. Rounding the folio total instead would let many
/// lines' sub-satang residue accumulate past a satang and disagree anyway,
/// and would mask genuine per-line drift behind a total that happens to
/// match.
pub(crate) fn pg_folio_sql(floor: LedgerEraFloor, single: bool) -> String {
    format!(
        "SELECT btrim(ledger_cin_no) AS folio, COUNT(*)::bigint AS line_count, \
                COALESCE(SUM(amt), 0)::float8 AS amount_sum, \
                COALESCE(SUM(CASE WHEN rn = 1 AND st = '1' THEN tender ELSE 0 END), 0)::float8 \
                    AS tender_sum \
           FROM (SELECT ledger_cin_no, ledger_legacy_id, \
                        COALESCE(ledger_status, '1') AS st, \
                        COALESCE(ledger_amount, 0)::numeric(19,4) AS amt, \
                        (COALESCE(ledger_cash, 0) + COALESCE(ledger_credit, 0) \
                         + COALESCE(ledger_free, 0) + COALESCE(ledger_tran, 0) \
                         + COALESCE(ledger_web, 0))::numeric(19,4) AS tender, \
                        ROW_NUMBER() OVER (PARTITION BY ledger_cin_no, \
                            COALESCE(NULLIF(ledger_pay_no, ''), \
                                     'lid:' || ledger_legacy_id::text) \
                            ORDER BY ledger_legacy_id) AS rn \
                   FROM ht_payment_ledger \
                  WHERE ledger_cin_no IS NOT NULL{single_filter}) t \
          GROUP BY 1{having}",
        single_filter = if single {
            " AND btrim(ledger_cin_no) = $1"
        } else {
            ""
        },
        having = match floor.effective {
            Some(f) if !single => format!(" HAVING MIN(ledger_legacy_id) >= {f}"),
            _ => String::new(),
        },
    )
}

/// The five legacy tender columns, in the order both the raw sum and the
/// per-column-rounded sum add them. ONE list, so those two forms of the
/// same expression can never drift apart about WHICH columns are tender —
/// and so the rounding test can iterate the real list instead of a copy.
const LEGACY_TENDER_COLUMNS: [&str; 5] = [
    "Cin_Pay_Cash",
    "Cin_Pay_Credit",
    "Cin_Pay_Free",
    "Cin_Pay_Tran",
    "Cin_Pay_web",
];

/// Legacy per-folio aggregate, floored at the mirror's coverage.
///
/// Takes the whole [`LedgerEraFloor`] and reads
/// [`LedgerEraFloor::effective`] itself — same reasoning as
/// [`pg_folio_sql`]: the caller cannot hand it the derived value by mistake.
///
/// The floor is a `HAVING` over the UNFILTERED per-folio `MIN(id)`, so a
/// folio that straddles the boundary is excluded whole rather than
/// compared on a partial line set (see the module docs). It is an integer
/// PostgreSQL itself produced and Rust formats as a bare number, so there
/// is no injection surface and no locale-dependent literal.
///
/// An effective floor of `None` (an empty canonical ledger that never had a
/// watermark) deliberately admits the WHOLE legacy table: with no coverage
/// at all, "legacy has 20k folios and we have none" IS the finding, and the
/// cap lands it as one bounded aggregate row.
///
/// `single` narrows to one `Cin_No` (bound as `@P1`) for the auto-resolve
/// sweep and drops the floor — same rule as `fetch_legacy_payment_hash`:
/// this path re-projects a folio the scan ALREADY admitted and logged, so
/// it must reproduce that row's hash unconditionally, or a floor that
/// moved forward would make an open row un-re-projectable.
///
/// ## Money is rounded to 2 dp per LINE, before the `SUM`
///
/// Canonical stores every line at `numeric(14,2)`, so the legacy scan has
/// to be reduced to that same precision or the comparison is asking
/// canonical for a value it cannot hold — see [`pg_folio_sql`] for the
/// invariant and the `CH26-004002` case that proved it.
///
/// The ORDER `ROUND(CAST(<float> AS DECIMAL(19,4)), 2)` is load-bearing.
/// Read-only against the live HF Hotel server on that folio's line
/// (2026-08-04):
///
/// | expression                                  | result        |
/// |---------------------------------------------|---------------|
/// | `ROUND(<float>, 2)`                         | `2552.26`   ✗ |
/// | `CAST(<float> AS DECIMAL(19,2))`            | `2552.26`   ✗ |
/// | `ROUND(CAST(<float> AS DECIMAL(19,4)), 2)`  | `2552.2700` ✓ |
///
/// `ROUND` on an APPROXIMATE type rounds the binary value, and
/// 2552.2649999999999 sits just below the midpoint, so it goes DOWN; a
/// direct `DECIMAL(19,2)` cast rounds that same binary value and goes down
/// too. Only once the value is an EXACT numeric does T-SQL `ROUND` use
/// half-away-from-zero (verified: `2552.2650 → 2552.2700`, `-2552.2650 →
/// -2552.2700`) — the same mode PostgreSQL `numeric` uses
/// (`round(2552.265::numeric, 2) = 2552.27`) and, crucially, the same mode
/// the mirror itself lands on, because PG converts a float through its
/// shortest round-trip decimal first
/// (`2552.2649999999999::float8::numeric(14,2) = 2552.27`). Cast-then-round
/// is the EXACT analogue of the mapper's storage conversion, not merely a
/// close one.
///
/// The tenders get the same round PER COLUMN rather than on their sum:
/// canonical holds five separate `numeric(14,2)` columns and
/// [`pg_folio_sql`] adds five already-rounded values, so rounding their raw
/// float sum once could disagree whenever two components each carry
/// sub-satang residue. They are the same `float` type as
/// `Cin_Pay_Ds_Price` and demonstrably carry residue (`Cin_Pay_web =
/// 1248.4000000000001` on line 55121 of the very same folio), so leaving
/// them raw would only be waiting for the identical unfixable finding on
/// the money-TAKEN half.
///
/// The `Cin_Pay_Ds_Price IS NULL` fallback deliberately keeps summing the
/// RAW tenders and rounds ONCE, at the end: `project_payment_line` adds the
/// five in `f64` and stores a single `numeric(14,2)`, so rounding each
/// column there would model a conversion the mapper never performs.
pub(crate) fn legacy_folio_sql(floor: LedgerEraFloor, single: bool) -> String {
    // Raw f64 sum — ONLY for the NULL-price fallback, which the mapper
    // computes in f64 and stores as one numeric(14,2) (rounded once, below).
    let tender_raw = LEGACY_TENDER_COLUMNS
        .iter()
        .map(|c| format!("COALESCE({c}, 0)"))
        .collect::<Vec<_>>()
        .join(" + ");
    // Per-column satang round — the exact analogue of canonical's five
    // independent numeric(14,2) tender columns.
    let tender_2dp = LEGACY_TENDER_COLUMNS
        .iter()
        .map(|c| format!("ROUND(CAST(COALESCE({c}, 0) AS DECIMAL(19,4)), 2)"))
        .collect::<Vec<_>>()
        .join(" + ");
    format!(
        "SELECT LTRIM(RTRIM(t.Cin_No)) AS folio, COUNT_BIG(*) AS line_count, \
                CAST(SUM(t.amt) AS FLOAT) AS amount_sum, \
                CAST(SUM(CASE WHEN t.rn = 1 AND t.st = '1' THEN t.tender ELSE 0 END) AS FLOAT) \
                    AS tender_sum \
           FROM (SELECT Cin_No, id, \
                        ROUND(CAST(COALESCE(Cin_Pay_Ds_Price, {tender_raw}) \
                                   AS DECIMAL(19,4)), 2) AS amt, \
                        CAST({tender_2dp} AS DECIMAL(19,4)) AS tender, \
                        COALESCE(Cin_Status, '1') AS st, \
                        ROW_NUMBER() OVER (PARTITION BY Cin_No, \
                            COALESCE(NULLIF(Pay_No, ''), \
                                     'lid:' + CAST(id AS VARCHAR(32))) \
                            ORDER BY id) AS rn \
                   FROM HT_CheckIn_Pay \
                  WHERE Cin_No IS NOT NULL{single_filter}) t \
          GROUP BY t.Cin_No{having}",
        single_filter = if single {
            " AND LTRIM(RTRIM(Cin_No)) = @P1"
        } else {
            ""
        },
        having = match floor.effective {
            Some(f) if !single => format!(" HAVING MIN(CAST(t.id AS BIGINT)) >= {f}"),
            _ => String::new(),
        },
    )
}

/// `Cin_Pay_Ds_Price` is NULL on some legacy lines; the mapper falls back
/// to the tender sum there (`project_payment_line`), so the probe must
/// too or every such folio would read as `value` drift forever. Pinned by
/// test — this const exists so the pin cannot drift from the SQL.
#[cfg(test)]
const LEGACY_AMOUNT_FALLBACK: &str = "COALESCE(Cin_Pay_Ds_Price,";

// =============================================================================
// Diff
// =============================================================================

/// One per-folio finding. The three variants map 1:1 onto the three
/// alert-visible divergence kinds.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum LedgerFinding {
    /// Legacy has the folio, canonical does not — a ledger the mirror
    /// never landed. The remedy is `backfill_payment_ledger`.
    MissingPg { folio: String, legacy: FolioSums },
    /// Canonical has the folio, legacy does not.
    MissingMssql { folio: String, pg: FolioSums },
    /// Both sides have it; a count or a total moved.
    Value {
        folio: String,
        legacy: FolioSums,
        pg: FolioSums,
    },
}

impl LedgerFinding {
    pub(crate) fn folio(&self) -> &str {
        match self {
            Self::MissingPg { folio, .. }
            | Self::MissingMssql { folio, .. }
            | Self::Value { folio, .. } => folio,
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

/// Pure diff of two folio maps. Iteration order is the union of two
/// `BTreeMap`s, i.e. sorted and deterministic — so a truncated report
/// keeps naming the SAME folios every tick and `record_divergence`'s
/// dedupe suppresses the repeat instead of accumulating a fresh slice.
pub(crate) fn diff_folios(
    legacy: &BTreeMap<String, FolioSums>,
    pg: &BTreeMap<String, FolioSums>,
) -> Vec<LedgerFinding> {
    let folios: BTreeSet<&String> = legacy.keys().chain(pg.keys()).collect();
    let mut out = Vec::new();
    for f in folios {
        match (legacy.get(f), pg.get(f)) {
            (Some(l), None) => out.push(LedgerFinding::MissingPg {
                folio: f.clone(),
                legacy: *l,
            }),
            (None, Some(p)) => out.push(LedgerFinding::MissingMssql {
                folio: f.clone(),
                pg: *p,
            }),
            (Some(l), Some(p)) => {
                if !l.agrees_with(p) {
                    out.push(LedgerFinding::Value {
                        folio: f.clone(),
                        legacy: *l,
                        pg: *p,
                    });
                }
            }
            (None, None) => unreachable!("folio came from one of the two maps"),
        }
    }
    out
}

/// Direction of a whole-table divergence.
///
/// NEVER `Cardinality`, even though the folio counts differ — see the
/// module docs: that kind is filtered out of both alert queries.
pub(crate) fn aggregate_divergence_kind(
    legacy: &LedgerTotals,
    pg: &LedgerTotals,
) -> DivergenceKind {
    if legacy.folio_count > pg.folio_count {
        DivergenceKind::MissingPg
    } else if pg.folio_count > legacy.folio_count {
        DivergenceKind::MissingMssql
    } else {
        DivergenceKind::Value
    }
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
/// only thing that clears `consecutive_failures`, clears `last_error` and
/// stamps `last_sync_at`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PaymentLedgerProbeOutcome {
    /// In-era folios that agreed on all three values.
    pub(crate) converged: usize,
    /// `ht_reconcile_log` rows written this tick (per-folio or the one
    /// aggregate row).
    pub(crate) recorded: usize,
    /// Wall time of the whole tick.
    pub(crate) duration_ms: i32,
    /// The coverage floor this tick actually scanned with, BOTH halves.
    ///
    /// Returned rather than alerted on here for the same reason the counts
    /// are: the Slack send lives at the `run_sync` call site next to every
    /// other alert in the subsystem. `effective > derived` sustained across
    /// [`crate::scheduler::sync::ERA_FLOOR_HELD_ALERT_TICKS`] ticks is what
    /// raises the held-watermark alert.
    pub(crate) era_floor: LedgerEraFloor,
}

/// Run the probe. Called ONLY when
/// `RECONCILE_PAYMENT_LEDGER_PROBE_ENABLED` is true — with the flag off
/// (the shipped default on every service) this is never entered and the
/// arm issues zero MSSQL and zero PG queries.
pub(crate) async fn run_payment_ledger_probe(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
) -> Result<PaymentLedgerProbeOutcome, AnyError> {
    let start = Instant::now();
    tracing::info!("[Sync] Probing the payment ledger per folio...");

    // ── Coverage floor: derived from the mirror, clamped to the ratchet ─
    // Passed to BOTH scans whole; neither side may be built from the raw
    // derived value (module docs — an unfloored canonical side turns every
    // pre-watermark folio into a phantom `missing_mssql`).
    let era_floor = payment_ledger_era_floor(pg_pool).await?;

    // ── ONE canonical grouped read, floored ───────────────────────────
    let pg_sql = pg_folio_sql(era_floor, false);
    let pg_rows = sqlx::query_as::<_, (String, i64, f64, f64)>(sqlx::AssertSqlSafe(&*pg_sql))
        .fetch_all(pg_pool)
        .await?;
    let pg_folios: BTreeMap<String, FolioSums> = pg_rows
        .into_iter()
        .map(|(folio, line_count, amount, tender)| {
            (
                folio,
                FolioSums {
                    line_count,
                    amount,
                    tender,
                },
            )
        })
        .collect();

    // ── ONE legacy grouped read, floored the SAME way ─────────────────
    let legacy_sql = legacy_folio_sql(era_floor, false);
    let mut conn = legacy_pool.get().await?;
    let rows =
        simple_query_with_timeout_pooled(&mut conn, &legacy_sql, MssqlOpKind::Read).await?;
    drop(conn);

    let mut legacy_folios: BTreeMap<String, FolioSums> = BTreeMap::new();
    for r in &rows {
        let Some(folio) = r.get::<&str, _>("folio") else {
            continue;
        };
        legacy_folios.insert(
            folio.to_string(),
            FolioSums {
                line_count: r.get::<i64, _>("line_count").unwrap_or(0),
                amount: r.try_get::<f64, _>("amount_sum").ok().flatten().unwrap_or(0.0),
                tender: r.try_get::<f64, _>("tender_sum").ok().flatten().unwrap_or(0.0),
            },
        );
    }

    let findings = diff_folios(&legacy_folios, &pg_folios);
    let compared = legacy_folios.len().max(pg_folios.len());
    let converged = compared.saturating_sub(findings.len());
    let duration_ms_of = |s: Instant| s.elapsed().as_millis() as i32;

    if findings.is_empty() {
        tracing::info!(
            folios = compared,
            floor = ?era_floor.effective,
            derived_floor = ?era_floor.derived,
            duration_ms = duration_ms_of(start),
            "[Sync] Payment-ledger probe: converged"
        );
        return Ok(PaymentLedgerProbeOutcome {
            converged,
            recorded: 0,
            duration_ms: duration_ms_of(start),
            era_floor,
        });
    }

    let recorded = if findings.len() > PAYMENT_LEDGER_MAX_FOLIO_FINDINGS {
        record_aggregate_divergence(
            pg_pool,
            &LedgerTotals::of(&legacy_folios),
            &LedgerTotals::of(&pg_folios),
            findings.len(),
            era_floor,
        )
        .await;
        1
    } else {
        for finding in &findings {
            record_folio_divergence(pg_pool, finding).await;
        }
        tracing::warn!(
            findings = findings.len(),
            legacy_folios = legacy_folios.len(),
            pg_folios = pg_folios.len(),
            floor = ?era_floor.effective,
            derived_floor = ?era_floor.derived,
            "[Sync] Payment-ledger probe: per-folio divergences recorded (a \
             missing_pg folio is money the round report under-counts; re-drive \
             it with the backfill_payment_ledger bin)"
        );
        findings.len()
    };

    Ok(PaymentLedgerProbeOutcome {
        converged,
        recorded,
        duration_ms: duration_ms_of(start),
        era_floor,
    })
}

/// ONE row standing for a whole-table divergence, used when the diff is
/// too big to enumerate.
async fn record_aggregate_divergence(
    pg_pool: &PgPool,
    legacy: &LedgerTotals,
    pg: &LedgerTotals,
    findings: usize,
    era_floor: LedgerEraFloor,
) {
    let floor = era_floor.effective;
    let kind = aggregate_divergence_kind(legacy, pg);
    // STABLE sentinel, not the live aggregate hash — see the module docs.
    // Placed on the side(s) that actually HAVE folios so the row still
    // reads the way its `divergence_kind` says it should.
    let sentinel = aggregate_sentinel();
    let (mssql_hash, pg_hash) = match kind {
        DivergenceKind::MissingPg => (Some(sentinel.clone()), None),
        DivergenceKind::MissingMssql => (None, Some(sentinel.clone())),
        _ => (Some(sentinel.clone()), Some(sentinel.clone())),
    };

    tracing::warn!(
        findings,
        legacy_folios = legacy.folio_count,
        pg_folios = pg.folio_count,
        kind = kind.as_str(),
        floor,
        derived_floor = ?era_floor.derived,
        "[Sync] Payment-ledger probe: whole-ledger divergence recorded as ONE \
         aggregate row (too large to enumerate per folio — the persisted \
         era-floor ratchet rules out a COLLAPSED floor, so compare \
         derived_floor against floor and then look at the mirror itself)"
    );

    record_divergence(
        pg_pool,
        PAYMENT_LEDGER_PROBE_KEY,
        PAYMENT_LEDGER_AGGREGATE_PK,
        pg_hash.as_deref(),
        mssql_hash.as_deref(),
        json!({
            "scope": "aggregate",
            "legacy_table": "HT_CheckIn_Pay",
            // Both, always: an operator reading this row must be able to see
            // whether the ratchet was holding the scan forward at the moment
            // it was written.
            "coverage_floor_legacy_id": floor,
            "derived_floor_legacy_id": era_floor.derived,
            "per_folio_findings": findings,
            "totals": legacy.json(),
        }),
        Some(json!({
            "scope": "aggregate",
            "canonical_table": "ht_payment_ledger",
            "totals": pg.json(),
        })),
        kind,
        // Real folio counts — informative on the row itself. They differ
        // for the two missing_* kinds by construction, which is exactly
        // why the kind above is NOT taken from `classify_divergence`.
        legacy.folio_count.min(i32::MAX as i64) as i32,
        pg.folio_count.min(i32::MAX as i64) as i32,
    )
    .await;
}

async fn record_folio_divergence(pg_pool: &PgPool, finding: &LedgerFinding) {
    let folio = finding.folio();
    let (mssql_hash, pg_hash, legacy_json, pg_json, legacy_count, pg_count) = match finding {
        LedgerFinding::MissingPg { legacy, .. } => (
            Some(folio_hash(folio, legacy)),
            None,
            json!({ "cin_no": folio, "legacy_table": "HT_CheckIn_Pay", "sums": legacy.json() }),
            None,
            legacy.line_count.min(i32::MAX as i64) as i32,
            0,
        ),
        LedgerFinding::MissingMssql { pg, .. } => (
            None,
            Some(folio_hash(folio, pg)),
            json!({ "cin_no": folio, "legacy_table": "HT_CheckIn_Pay", "present": false }),
            Some(json!({ "cin_no": folio, "canonical_table": "ht_payment_ledger", "sums": pg.json() })),
            0,
            pg.line_count.min(i32::MAX as i64) as i32,
        ),
        LedgerFinding::Value { legacy, pg, .. } => (
            Some(folio_hash(folio, legacy)),
            Some(folio_hash(folio, pg)),
            json!({ "cin_no": folio, "legacy_table": "HT_CheckIn_Pay", "sums": legacy.json() }),
            Some(json!({ "cin_no": folio, "canonical_table": "ht_payment_ledger", "sums": pg.json() })),
            legacy.line_count.min(i32::MAX as i64) as i32,
            pg.line_count.min(i32::MAX as i64) as i32,
        ),
    };

    record_divergence(
        pg_pool,
        PAYMENT_LEDGER_PROBE_KEY,
        folio,
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

// =============================================================================
// Auto-resolve dispatch
// =============================================================================

/// Canonical-side hash for one `ht_reconcile_log` row of this probe.
///
/// Never returns `Ok(None)`: an absent folio hashes to
/// [`folio_absent_hash`] so absent-on-both converges (module docs).
///
/// The `<aggregate>` arm floors the canonical scan exactly as detection did
/// — through [`payment_ledger_era_floor_readonly`], so the totals it hashes
/// are over the same in-era folio set the row was written from, and so
/// nothing on the resolve path writes the watermark.
pub(crate) async fn resolve_pg_hash(
    pg_pool: &PgPool,
    legacy_pk: &str,
) -> Result<Option<String>, sqlx::Error> {
    if legacy_pk == PAYMENT_LEDGER_AGGREGATE_PK {
        let era_floor = payment_ledger_era_floor_readonly(pg_pool).await?;
        let sql = pg_folio_sql(era_floor, false);
        let rows = sqlx::query_as::<_, (String, i64, f64, f64)>(sqlx::AssertSqlSafe(&*sql))
            .fetch_all(pg_pool)
            .await?;
        let folios: BTreeMap<String, FolioSums> = rows
            .into_iter()
            .map(|(folio, line_count, amount, tender)| {
                (
                    folio,
                    FolioSums {
                        line_count,
                        amount,
                        tender,
                    },
                )
            })
            .collect();
        return Ok(Some(aggregate_hash(&LedgerTotals::of(&folios))));
    }

    let sql = pg_folio_sql(LedgerEraFloor::UNFLOORED, true);
    let found = sqlx::query_as::<_, (String, i64, f64, f64)>(sqlx::AssertSqlSafe(&*sql))
        .bind(legacy_pk)
        .fetch_optional(pg_pool)
        .await?;
    Ok(Some(match found {
        Some((_, line_count, amount, tender)) => folio_hash(
            legacy_pk,
            &FolioSums {
                line_count,
                amount,
                tender,
            },
        ),
        None => folio_absent_hash(legacy_pk),
    }))
}

/// Legacy-side hash for one `ht_reconcile_log` row of this probe.
///
/// Takes `pg_pool` for the same reason 6-C's `resolve_legacy_hash` does:
/// the `<aggregate>` comparison is only meaningful inside the mirror's own
/// coverage floor, and that floor is derived from the CANONICAL side.
/// Re-deriving it here (rather than freezing it onto the row) means a
/// floor that MOVES because the mirror finally received its missing
/// history is picked up on the next sweep.
///
/// It goes through [`payment_ledger_era_floor_readonly`] — the SAME clamped
/// floor detection used, not the raw `MIN` — or a resolve could re-project a
/// WIDER folio set than the row was written from and never reproduce its
/// hash. It is the READ-ONLY variant on purpose: the sweep that calls this
/// is not gated on the probe's own flag, so re-running the ratchet upsert
/// from here would let a DARK site persist a watermark from pre-backfill
/// coverage (module docs, "The resolve path NEVER writes the watermark").
pub(crate) async fn resolve_legacy_hash(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
    legacy_pk: &str,
) -> Result<Option<String>, AnyError> {
    if legacy_pk == PAYMENT_LEDGER_AGGREGATE_PK {
        let era_floor = payment_ledger_era_floor_readonly(pg_pool).await?;
        let sql = legacy_folio_sql(era_floor, false);
        let mut conn = legacy_pool.get().await?;
        let rows = simple_query_with_timeout_pooled(&mut conn, &sql, MssqlOpKind::Read).await?;
        drop(conn);
        let mut folios: BTreeMap<String, FolioSums> = BTreeMap::new();
        for r in &rows {
            let Some(folio) = r.get::<&str, _>("folio") else {
                continue;
            };
            folios.insert(
                folio.to_string(),
                FolioSums {
                    line_count: r.get::<i64, _>("line_count").unwrap_or(0),
                    amount: r.try_get::<f64, _>("amount_sum").ok().flatten().unwrap_or(0.0),
                    tender: r.try_get::<f64, _>("tender_sum").ok().flatten().unwrap_or(0.0),
                },
            );
        }
        return Ok(Some(aggregate_hash(&LedgerTotals::of(&folios))));
    }

    let sql = legacy_folio_sql(LedgerEraFloor::UNFLOORED, true);
    let mut conn = legacy_pool.get().await?;
    let mut q = tiberius::Query::new(sql.as_str());
    q.bind(legacy_pk);
    let rows = query_with_timeout_pooled(&mut conn, &sql, q, MssqlOpKind::Read).await?;
    drop(conn);
    Ok(Some(match rows.first() {
        Some(r) => folio_hash(
            legacy_pk,
            &FolioSums {
                line_count: r.get::<i64, _>("line_count").unwrap_or(0),
                amount: r.try_get::<f64, _>("amount_sum").ok().flatten().unwrap_or(0.0),
                tender: r.try_get::<f64, _>("tender_sum").ok().flatten().unwrap_or(0.0),
            },
        ),
        None => folio_absent_hash(legacy_pk),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sums(line_count: i64, amount: f64, tender: f64) -> FolioSums {
        FolioSums {
            line_count,
            amount,
            tender,
        }
    }

    /// The probe key becomes an `ht_reconcile_log.table_name` AND a
    /// cooldown key in the shared `ht_level_drift_alert_cooldowns` table,
    /// where any key containing `:` is read as belonging to a different
    /// tripwire and is never cleared by the sync-lag all-clear. It must
    /// also not collide with a 6-C mirror-probe key.
    #[test]
    fn probe_key_is_not_namespaced_and_does_not_collide() {
        assert!(!PAYMENT_LEDGER_PROBE_KEY.contains(':'));
        assert!(!crate::scheduler::mirror_probe::mirror_probe_keys()
            .contains(&PAYMENT_LEDGER_PROBE_KEY));
        assert!(is_payment_ledger_probe(PAYMENT_LEDGER_PROBE_KEY));
        assert!(!is_payment_ledger_probe("payments"));
        assert!(!is_payment_ledger_probe("mirror_ht_cupon"));
    }

    /// `cardinality` is filtered out of BOTH alert queries, so this probe
    /// must never emit it — a folio-count mismatch is reported by
    /// DIRECTION instead.
    #[test]
    fn aggregate_kind_is_never_cardinality() {
        let base = LedgerTotals {
            folio_count: 10,
            line_count: 20,
            amount: 100.0,
            tender: 100.0,
        };
        let fewer = LedgerTotals {
            folio_count: 4,
            ..base
        };
        let other_money = LedgerTotals {
            amount: 999.0,
            ..base
        };
        assert_eq!(
            aggregate_divergence_kind(&base, &fewer),
            DivergenceKind::MissingPg
        );
        assert_eq!(
            aggregate_divergence_kind(&fewer, &base),
            DivergenceKind::MissingMssql
        );
        assert_eq!(
            aggregate_divergence_kind(&base, &other_money),
            DivergenceKind::Value
        );
        for legacy in [&base, &fewer, &other_money] {
            for pg in [&base, &fewer, &other_money] {
                assert_ne!(
                    aggregate_divergence_kind(legacy, pg),
                    DivergenceKind::Cardinality,
                    "a probe must never emit `cardinality` — it is excluded from \
                     both alert queries and would be alert-invisible"
                );
            }
        }
    }

    /// Every finding this arm can produce must be one of the three
    /// alert-VISIBLE kinds.
    #[test]
    fn every_folio_finding_kind_is_alert_visible() {
        let legacy: BTreeMap<String, FolioSums> = [
            ("A".to_string(), sums(1, 10.0, 10.0)),
            ("B".to_string(), sums(1, 20.0, 20.0)),
        ]
        .into_iter()
        .collect();
        let pg: BTreeMap<String, FolioSums> = [
            ("B".to_string(), sums(1, 22.0, 22.0)),
            ("C".to_string(), sums(1, 30.0, 30.0)),
        ]
        .into_iter()
        .collect();
        let found = diff_folios(&legacy, &pg);
        assert_eq!(found.len(), 3);
        for f in &found {
            assert_ne!(f.kind(), DivergenceKind::Cardinality);
        }
    }

    #[test]
    fn diff_classifies_each_direction_in_sorted_order() {
        let legacy: BTreeMap<String, FolioSums> = [
            ("CH26-000001".to_string(), sums(2, 100.0, 100.0)),
            ("CH26-000002".to_string(), sums(1, 50.0, 50.0)),
            ("CH26-000004".to_string(), sums(1, 40.0, 40.0)),
        ]
        .into_iter()
        .collect();
        let pg: BTreeMap<String, FolioSums> = [
            ("CH26-000001".to_string(), sums(2, 100.0, 100.0)),
            ("CH26-000002".to_string(), sums(1, 50.0, 0.0)),
            ("CH26-000003".to_string(), sums(1, 30.0, 30.0)),
        ]
        .into_iter()
        .collect();
        let found = diff_folios(&legacy, &pg);
        assert_eq!(
            found,
            vec![
                // A cancel flip that landed on one side only: same line
                // count, same itemized total, different ACTIVE tender.
                LedgerFinding::Value {
                    folio: "CH26-000002".to_string(),
                    legacy: sums(1, 50.0, 50.0),
                    pg: sums(1, 50.0, 0.0),
                },
                LedgerFinding::MissingMssql {
                    folio: "CH26-000003".to_string(),
                    pg: sums(1, 30.0, 30.0),
                },
                LedgerFinding::MissingPg {
                    folio: "CH26-000004".to_string(),
                    legacy: sums(1, 40.0, 40.0),
                },
            ]
        );
    }

    /// Money is compared at 2 dp on both sides, so a last-ULP difference
    /// between MSSQL's and PostgreSQL's summation order cannot read as
    /// drift — while a real satang does.
    #[test]
    fn folio_comparison_is_at_two_decimals() {
        let base = sums(3, 1000.0, 950.0);
        assert!(base.agrees_with(&sums(3, 1000.000000000001, 950.0)));
        assert!(!base.agrees_with(&sums(3, 1000.01, 950.0)));
        assert!(!base.agrees_with(&sums(3, 1000.0, 950.01)));
        assert!(!base.agrees_with(&sums(4, 1000.0, 950.0)));
    }

    /// The absent sentinel is what lets a folio deleted on BOTH sides
    /// close instead of sitting open forever.
    #[test]
    fn absent_hash_is_distinct_from_every_present_hash() {
        let folio = "CH26-004952";
        let absent = folio_absent_hash(folio);
        assert_ne!(absent, folio_hash(folio, &sums(0, 0.0, 0.0)));
        assert_ne!(absent, folio_hash(folio, &sums(1, 890.0, 890.0)));
        assert_ne!(absent, folio_absent_hash("CH26-004953"));
        assert_eq!(absent, folio_absent_hash(folio));
    }

    /// Each of the three compared values must move the folio hash, or the
    /// probe would detect a divergence it could never resolve (the
    /// auto-resolve sweep compares these same hashes).
    #[test]
    fn folio_hash_moves_with_every_compared_value() {
        let base = folio_hash("CH26-000001", &sums(2, 100.0, 100.0));
        assert_ne!(base, folio_hash("CH26-000001", &sums(3, 100.0, 100.0)));
        assert_ne!(base, folio_hash("CH26-000001", &sums(2, 100.01, 100.0)));
        assert_ne!(base, folio_hash("CH26-000001", &sums(2, 100.0, 100.01)));
        assert_ne!(base, folio_hash("CH26-000002", &sums(2, 100.0, 100.0)));
        assert_eq!(base, folio_hash("CH26-000001", &sums(2, 100.0, 100.0)));
    }

    /// The aggregate row's STORED hash must NOT move with the live totals,
    /// or every payment taken mints a fresh unresolved row; the RESOLVE
    /// hash must.
    #[test]
    fn aggregate_sentinel_is_stable_while_the_aggregate_hash_is_not() {
        let a = LedgerTotals {
            folio_count: 1300,
            line_count: 1852,
            amount: 100.0,
            tender: 100.0,
        };
        let b = LedgerTotals {
            folio_count: 1301,
            ..a
        };
        assert_eq!(aggregate_sentinel(), aggregate_sentinel());
        assert_ne!(aggregate_hash(&a), aggregate_hash(&b));
        assert_ne!(aggregate_sentinel(), aggregate_hash(&a));
    }

    /// The coverage floor must be a `HAVING` over the UNFILTERED per-folio
    /// `MIN(id)`. A `WHERE id >= floor` would admit the one HF Hotel folio
    /// that straddles the boundary on a PARTIAL line set and manufacture a
    /// permanent `value` finding.
    /// A floor in both halves, as one tick sees it.
    fn floor_at(derived: i64, effective: i64) -> LedgerEraFloor {
        LedgerEraFloor {
            derived: Some(derived),
            effective: Some(effective),
        }
    }

    #[test]
    fn legacy_sql_floors_whole_folios_not_lines() {
        let sql = legacy_folio_sql(floor_at(56353, 56353), false);
        assert!(
            sql.contains("HAVING MIN(CAST(t.id AS BIGINT)) >= 56353"),
            "got: {sql}"
        );
        assert!(
            !sql.contains("WHERE Cin_No IS NOT NULL AND id >="),
            "the floor must not filter LINES: {sql}"
        );
        // No canonical coverage at all ⇒ no floor: "legacy has folios and
        // we have none" is the finding, and the cap lands it as ONE row.
        let unfloored = legacy_folio_sql(LedgerEraFloor::UNFLOORED, false);
        assert!(!unfloored.contains("HAVING"));
    }

    /// The canonical scan carries the floor too, on the same whole-folio
    /// `HAVING MIN(id)` rule.
    ///
    /// RED before the fix (the builder took no floor at all): an unfloored
    /// canonical side reports every folio below a HELD watermark as
    /// `missing_mssql` — churning per tick while the count is small, and
    /// landing ONE never-resolvable `<aggregate>` row once it is large
    /// (unfloored-pg totals versus floored-legacy totals can never be equal).
    #[test]
    fn pg_sql_floors_whole_folios_not_lines() {
        let sql = pg_folio_sql(floor_at(56353, 56353), false);
        assert!(sql.contains("HAVING MIN(ledger_legacy_id) >= 56353"), "got: {sql}");
        assert!(
            !sql.contains("ledger_cin_no IS NOT NULL AND ledger_legacy_id >="),
            "the floor must not filter LINES: {sql}"
        );
        // The outer HAVING can only see the id if the inner projection
        // carries it.
        assert!(
            sql.contains("SELECT ledger_cin_no, ledger_legacy_id,"),
            "the id must be projected out of the dedupe subquery: {sql}"
        );
        assert!(!pg_folio_sql(LedgerEraFloor::UNFLOORED, false).contains("HAVING"));
    }

    /// D1 — the exact pin the review asked for: with the 2026-07-30 numbers
    /// persisted 40470 / derived 39113, BOTH bulk statements carry
    /// `>= 40470`, and neither carries the dragged value.
    ///
    /// This is also the mutation guard for `.effective` → `.derived` INSIDE
    /// either builder, which is the only place that mistake can still be
    /// written now that the builders take the whole [`LedgerEraFloor`].
    #[test]
    fn both_scans_are_floored_at_the_effective_watermark() {
        let held = floor_at(39113, 40470);

        let legacy = legacy_folio_sql(held, false);
        let pg = pg_folio_sql(held, false);
        assert!(
            legacy.contains("HAVING MIN(CAST(t.id AS BIGINT)) >= 40470"),
            "legacy scan must floor at the WATERMARK: {legacy}"
        );
        assert!(
            pg.contains("HAVING MIN(ledger_legacy_id) >= 40470"),
            "canonical scan must floor at the WATERMARK too, or every folio \
             below it reads as a phantom missing_mssql: {pg}"
        );
        assert!(!legacy.contains("39113"), "dragged floor reached legacy: {legacy}");
        assert!(!pg.contains("39113"), "dragged floor reached canonical: {pg}");

        // Both SINGLE forms drop the floor — an already-admitted row must
        // stay re-projectable after the floor moves forward.
        assert!(!legacy_folio_sql(held, true).contains("HAVING"));
        assert!(!pg_folio_sql(held, true).contains("HAVING"));
    }

    // ── Era-floor ratchet (migration 084) ─────────────────────────────
    //
    // Clamp-case idiom borrowed from the 6-B arm's
    // `era_floor_is_clamped_to_a_non_decreasing_watermark`; the rule
    // itself is single-sourced (`scheduler::sync::clamped_era_floor`), so
    // these pin the ID-BASIS behaviour and this arm's numbers, not a
    // second copy of the logic.

    use crate::scheduler::sync::clamped_era_floor;

    /// THE 2026-07-30 SCENARIO, with its live numbers. A
    /// `backfill_payment_ledger --days=212` mirrored a monthly-billed
    /// long-stay (`CH25-000076`) WHOLE, and its 2025-08 line dragged
    /// `MIN(ledger_legacy_id)` from ~40470 back to 39113 — sweeping 404
    /// never-mirrored folios into the scan as `missing_pg`, none of them a
    /// genuine gap. A derived floor BELOW the persisted watermark must not
    /// widen the scan.
    ///
    /// RED without the clamp: `effective` would be the derived 39113 and
    /// the `HAVING MIN(id) >= 39113` scan would re-admit those folios.
    #[test]
    fn a_dragged_derived_floor_does_not_widen_the_scan() {
        let persisted = 40470_i64; // watermark before the backfill
        let derived = 39113_i64; // poisoned by one 2025-08 line

        let effective = clamped_era_floor(Some(persisted), Some(derived));
        assert_eq!(
            effective,
            Some(persisted),
            "a derived MIN below the watermark is the whole-folio drag; the \
             effective floor must stay at the watermark"
        );

        // And the scans really are floored at the watermark, not the drag —
        // BOTH of them (see `both_scans_are_floored_at_the_effective_watermark`
        // for the full pin).
        let floor = LedgerEraFloor {
            derived: Some(derived),
            effective,
        };
        for sql in [legacy_folio_sql(floor, false), pg_folio_sql(floor, false)] {
            assert!(sql.contains(">= 40470"), "got: {sql}");
            assert!(
                !sql.contains(">= 39113"),
                "the poisoned derived floor must never reach a scan: {sql}"
            );
        }
    }

    /// The ratchet still moves FORWARD: real coverage growth (a `--all`
    /// backfill completing, or history the mirror finally received) is
    /// genuine narrowing and wins — and, at the other end, the first tick
    /// after deploy seeds the watermark from the derived value.
    #[test]
    fn the_era_floor_ratchets_forward_and_seeds_from_derived() {
        assert_eq!(
            clamped_era_floor(Some(39014_i64), Some(40470_i64)),
            Some(40470),
            "a derived floor ABOVE the watermark is genuine narrowing"
        );
        assert_eq!(
            clamped_era_floor(None, Some(39014_i64)),
            Some(39014),
            "first tick after deploy: the derived MIN (Ville's absolute legacy \
             minimum after the --all backfill) becomes the baseline"
        );
    }

    /// The empty-`ht_payment_ledger` contract, unchanged by the ratchet:
    /// with no coverage AND no watermark the floor is `None`, the legacy
    /// scan emits no `HAVING` and admits the whole table, and the cap lands
    /// "legacy has folios, we have none" as ONE aggregate row. With a
    /// watermark and an emptied mirror the watermark is KEPT — re-widening
    /// to no-coverage is the same flood by another route (the 6-B rule).
    #[test]
    fn empty_ledger_contract_is_preserved() {
        assert_eq!(
            clamped_era_floor(None, None::<i64>),
            None,
            "no coverage was ever established ⇒ the scan stays unfloored"
        );
        assert!(!legacy_folio_sql(LedgerEraFloor::UNFLOORED, false).contains("HAVING"));
        assert!(!pg_folio_sql(LedgerEraFloor::UNFLOORED, false).contains("HAVING"));

        assert_eq!(
            clamped_era_floor(Some(40470_i64), None),
            Some(40470),
            "the mirror vanishing must NOT reopen the whole legacy history"
        );
    }

    /// The ratchet has to hold across PROCESSES (the backend scheduler and
    /// `bin/sync` can tick the same database), so `GREATEST` lives in SQL
    /// and the post-clamp read is the same round trip as the write. The
    /// statement must touch the ID column, never the sibling arm's
    /// TIMESTAMP one.
    #[test]
    fn era_floor_id_sql_is_monotonic_and_single_round_trip() {
        assert!(
            ERA_FLOOR_ID_UPSERT_SQL.contains(
                "GREATEST(ht_reconcile_era_floor.era_floor_id, EXCLUDED.era_floor_id)"
            ),
            "without GREATEST the upsert would happily write a LOWER floor and \
             the 2026-07-30 drag would land again: {ERA_FLOOR_ID_UPSERT_SQL}"
        );
        assert!(
            ERA_FLOOR_ID_UPSERT_SQL.ends_with("RETURNING era_floor_id"),
            "the post-clamp value must come back from the same statement, else a \
             concurrent tick's value is silently ignored: {ERA_FLOOR_ID_UPSERT_SQL}"
        );
        assert!(ERA_FLOOR_ID_SELECT_SQL.contains("FROM ht_reconcile_era_floor"));
        assert!(ERA_FLOOR_ID_SELECT_SQL.contains("table_name = $1"));
        // The ID basis, never the 6-B arm's TIMESTAMP column: a date basis
        // here would have to cross the naive-Thai/TIMESTAMPTZ boundary the
        // integer IDENTITY floor exists to avoid. (The table NAME also ends
        // in `era_floor`, so mask it before counting columns.)
        for sql in [ERA_FLOOR_ID_UPSERT_SQL, ERA_FLOOR_ID_SELECT_SQL] {
            let cols = sql.replace("ht_reconcile_era_floor", "<tbl>");
            assert_eq!(
                cols.matches("era_floor").count(),
                cols.matches("era_floor_id").count(),
                "every era_floor reference must be the ID column — writing the \
                 6-B arm's TIMESTAMP column from here would corrupt its \
                 watermark: {sql}"
            );
        }
        // The derived half stays an integer MIN over the IDENTITY column.
        assert!(PG_FLOOR_SQL.contains("MIN(ledger_legacy_id)"), "{PG_FLOOR_SQL}");
        assert!(
            !PG_FLOOR_SQL.contains("ledger_pay_date"),
            "a date basis reintroduces the Thai→UTC reasoning the id basis \
             avoids: {PG_FLOOR_SQL}"
        );
    }

    /// `ht_reconcile_era_floor` is keyed on `table_name`, and this arm now
    /// shares it with the 6-B `guest_registry` arm. The two row identities
    /// must differ, or one arm's floor would clobber the other's — across
    /// two INCOMPATIBLE bases (a TIMESTAMP and a legacy IDENTITY).
    #[test]
    fn era_floor_row_identity_does_not_collide_with_guest_registry() {
        assert_eq!(
            PAYMENT_LEDGER_ERA_FLOOR_KEY, PAYMENT_LEDGER_PROBE_KEY,
            "one literal for ht_reconcile_era_floor.table_name, \
             ht_reconcile_log.table_name and sync_status.entity_type, so one \
             operator query joins them"
        );
        assert_ne!(
            PAYMENT_LEDGER_ERA_FLOOR_KEY,
            crate::scheduler::sync::GUEST_REGISTRY_ERA_FLOOR_KEY,
            "both arms upsert into ht_reconcile_era_floor; a shared key would \
             mean one arm's watermark overwrites the other's"
        );
        assert!(
            PAYMENT_LEDGER_ERA_FLOOR_KEY.len() <= 50,
            "table_name is VARCHAR(50)"
        );
    }

    /// The tick and the auto-resolve sweep must floor BOTH scans the SAME
    /// way, or an `<aggregate>` row is written from one folio set and
    /// re-projected from another and can never close. Everything goes
    /// through the clamped helpers; nothing may reach for the raw `MIN`
    /// again.
    #[test]
    fn detection_and_resolve_share_the_clamped_floor() {
        let body = module_body();
        assert_eq!(
            body.matches("payment_ledger_era_floor(pg_pool)").count(),
            1,
            "exactly ONE caller of the RATCHETING floor — the probe's own \
             tick. The resolve arms must use the read-only variant"
        );
        assert_eq!(
            body.matches("payment_ledger_era_floor_readonly(pg_pool)")
                .count(),
            2,
            "both <aggregate> resolve arms (pg + legacy) must re-derive the \
             clamped floor, read-only"
        );
        assert_eq!(
            body.matches("AssertSqlSafe(PG_FLOOR_SQL)").count(),
            1,
            "the raw derived MIN may be read in ONE place only (inside \
             `derived_floor`); anything else bypasses the ratchet"
        );
    }

    /// D5 — the auto-resolve sweep is dispatched on
    /// `ht_reconcile_log.table_name` and is NOT gated on
    /// `RECONCILE_PAYMENT_LEDGER_PROBE_ENABLED`, so it runs at sites where
    /// this probe is still dark. It must therefore never PERSIST a
    /// watermark: doing so would let a site bake in a floor derived from
    /// pre-`--all`-backfill coverage, which the ratchet would then hold
    /// forever, permanently excluding the folios that backfill lands.
    ///
    /// RED before the fix: `resolve_legacy_hash` called the ratcheting
    /// `payment_ledger_era_floor`, whose upsert writes.
    #[test]
    fn the_resolve_path_never_writes_the_watermark() {
        let body = module_body();
        // Both resolve fns live at the end of the module body, in this
        // order; slicing from the first covers both.
        let resolve = &body[body
            .find("pub(crate) async fn resolve_pg_hash")
            .expect("resolve_pg_hash is part of the module body")..];
        assert!(
            resolve.contains("fn resolve_legacy_hash"),
            "the slice must cover BOTH resolve arms"
        );
        assert!(
            !resolve.contains("ERA_FLOOR_ID_UPSERT_SQL"),
            "the resolve path must not issue the ratchet upsert"
        );
        assert!(
            !resolve.contains("payment_ledger_era_floor(pg_pool)"),
            "the resolve path must not call the WRITING floor helper"
        );
        assert_eq!(
            body.matches("ERA_FLOOR_ID_UPSERT_SQL)").count(),
            1,
            "the watermark is written from exactly one statement, in the \
             probe's own tick"
        );
    }

    /// The module body with its tests stripped — every source-scan pin
    /// reads this, so a pin can never match itself.
    fn module_body() -> &'static str {
        include_str!("payment_ledger_probe.rs")
            .split("mod tests")
            .next()
            .expect("the module body precedes its tests")
    }

    /// A drag attempt must be VISIBLE. `watermark_holding` is what the log
    /// line keys on, and it is true exactly when the persisted floor is
    /// pulling the scan forward.
    #[test]
    fn watermark_holding_flags_exactly_the_drag_case() {
        let dragged = LedgerEraFloor {
            derived: Some(39113),
            effective: Some(40470),
        };
        let quiet = LedgerEraFloor {
            derived: Some(39014),
            effective: Some(39014),
        };
        let empty = LedgerEraFloor {
            derived: None,
            effective: None,
        };
        assert!(dragged.watermark_holding());
        assert!(!quiet.watermark_holding());
        assert!(!empty.watermark_holding());
    }

    /// The single-folio resolve projection must be IDENTICAL to the bulk
    /// one apart from the key filter, or a row's resolve hash could never
    /// equal the hash detection stored. It also drops the floor, matching
    /// `fetch_legacy_payment_hash`: an already-admitted row must stay
    /// re-projectable if the floor moves forward.
    #[test]
    fn single_folio_sql_matches_the_bulk_projection() {
        let floor = floor_at(1, 1);
        let bulk = legacy_folio_sql(floor, false);
        let single = legacy_folio_sql(floor, true);
        assert!(single.contains("AND LTRIM(RTRIM(Cin_No)) = @P1"));
        assert!(!single.contains("HAVING"), "got: {single}");
        assert!(bulk.contains("HAVING"));
        let projection = |s: &str| s.split(" FROM (SELECT").next().unwrap().to_string();
        assert_eq!(projection(&bulk), projection(&single));

        let pg_bulk = pg_folio_sql(floor, false);
        let pg_single = pg_folio_sql(floor, true);
        assert!(pg_single.contains("AND btrim(ledger_cin_no) = $1"));
        assert!(!pg_single.contains("HAVING"), "got: {pg_single}");
        assert!(pg_bulk.contains("HAVING"));
        assert_eq!(projection(&pg_bulk), projection(&pg_single));
    }

    /// The tender sum must be DEDUPED per receipt on BOTH sides with the
    /// same key `ROUND_INCOME_BY_TENDER_SQL` uses, and restricted to
    /// ACTIVE lines; the itemized amount must NOT be deduped. Summing
    /// replicated tenders raw would write a 3×-inflated "legacy took ฿X"
    /// into the row an operator reads (Ville round 816: raw 17,255 vs
    /// iHOTEL 11,005).
    #[test]
    fn tenders_are_deduped_per_receipt_and_amounts_are_not() {
        let pg = pg_folio_sql(LedgerEraFloor::UNFLOORED, false);
        let legacy = legacy_folio_sql(LedgerEraFloor::UNFLOORED, false);
        // Dedupe key: blank/NULL pay_no falls back to the LINE id, so such
        // lines stay separate receipts instead of collapsing into one.
        assert!(
            pg.contains("COALESCE(NULLIF(ledger_pay_no, ''), 'lid:' || ledger_legacy_id::text)"),
            "got: {pg}"
        );
        assert!(
            legacy.contains("COALESCE(NULLIF(Pay_No, ''), 'lid:' + CAST(id AS VARCHAR(32)))"),
            "got: {legacy}"
        );
        // Tenders: deduped (rn = 1) AND active-only.
        assert!(pg.contains("CASE WHEN rn = 1 AND st = '1' THEN tender"));
        assert!(legacy.contains("CASE WHEN t.rn = 1 AND t.st = '1' THEN t.tender"));
        // Amounts: summed raw across every line, no rn / status guard.
        assert!(pg.contains("SUM(amt)"));
        assert!(legacy.contains("SUM(t.amt)"));
        // The house convention this pins lives in routes/new_shifts.rs.
        let round = include_str!("../routes/new_shifts.rs");
        assert!(
            round.contains("DISTINCT ON (COALESCE(NULLIF(ledger_pay_no, '')"),
            "round_report no longer dedupes tenders per receipt — re-derive \
             this probe's tender basis from it"
        );
    }

    /// The legacy amount must fall back to the tender sum when
    /// `Cin_Pay_Ds_Price` is NULL, exactly as the mapper's
    /// `project_payment_line` does — otherwise every folio holding such a
    /// line reads as `value` drift forever.
    #[test]
    fn legacy_amount_uses_the_mappers_null_fallback() {
        assert!(
            legacy_folio_sql(LedgerEraFloor::UNFLOORED, false).contains(LEGACY_AMOUNT_FALLBACK)
        );
        let mapper = include_str!("../sync/mappers/payment.rs");
        assert!(
            mapper.contains("None => cash + credit + free + tran + web,"),
            "the ledger mapper no longer falls back to the tender sum for a \
             NULL Cin_Pay_Ds_Price — re-derive this probe's amount basis"
        );
    }

    // ── Comparable precision (folio CH26-004002, HF Hotel 2026-08-04) ──

    /// SQL SHAPE: the legacy money must be reduced to canonical's
    /// `numeric(14,2)` storage precision PER LINE, inside the dedupe
    /// subquery, before the aggregate sees it — and the `DECIMAL(19,4)`
    /// cast must happen BEFORE the `ROUND`.
    ///
    /// Cast-then-round is not stylistic. Verified read-only against the
    /// live server on line 55149 of `CH26-004002`
    /// (`Cin_Pay_Ds_Price = 2552.2649999999999`):
    ///
    /// * `ROUND(<float>, 2)` → `2552.26` — `ROUND` on an APPROXIMATE type
    ///   rounds the binary value, which sits below the midpoint.
    /// * `CAST(<float> AS DECIMAL(19,2))` → `2552.26` — same trap by
    ///   another spelling.
    /// * `ROUND(CAST(<float> AS DECIMAL(19,4)), 2)` → `2552.2700` — the
    ///   only form that reproduces what the mirror stored (`2552.27`).
    #[test]
    fn legacy_money_is_rounded_to_canonical_storage_precision_per_line() {
        let sql = legacy_folio_sql(LedgerEraFloor::UNFLOORED, false);

        let raw_tender = LEGACY_TENDER_COLUMNS
            .iter()
            .map(|c| format!("COALESCE({c}, 0)"))
            .collect::<Vec<_>>()
            .join(" + ");
        assert!(
            sql.contains(&format!(
                "ROUND(CAST(COALESCE(Cin_Pay_Ds_Price, {raw_tender}) AS DECIMAL(19,4)), 2) AS amt,"
            )),
            "the line total must be CAST to an exact numeric and THEN rounded \
             to 2 dp — canonical's numeric(14,2) cannot hold anything finer: \
             {sql}"
        );
        // The mapper's NULL fallback adds the five tenders in f64 and
        // stores ONE numeric(14,2), so the fallback rounds once, at the
        // end — per-column rounding there would model a conversion
        // `project_payment_line` never performs.
        assert!(
            sql.contains(&format!("COALESCE(Cin_Pay_Ds_Price, {raw_tender})")),
            "the NULL-price fallback must sum the RAW tenders: {sql}"
        );

        // Tenders: rounded PER COLUMN, mirroring canonical's five separate
        // numeric(14,2) columns that `pg_folio_sql` adds already-rounded.
        for col in LEGACY_TENDER_COLUMNS {
            assert!(
                sql.contains(&format!(
                    "ROUND(CAST(COALESCE({col}, 0) AS DECIMAL(19,4)), 2)"
                )),
                "tender column {col} is the same legacy `float` type as \
                 Cin_Pay_Ds_Price and can carry sub-satang residue too \
                 (Cin_Pay_web = 1248.4000000000001 on line 55121): {sql}"
            );
        }

        // The two spellings that silently round the FLOAT must not appear.
        assert!(
            !sql.contains("ROUND(Cin_Pay"),
            "rounding a `float` rounds its binary value, not its decimal \
             one — 2552.2649999999999 would go DOWN to 2552.26: {sql}"
        );
        assert!(
            !sql.contains("DECIMAL(19,2)"),
            "a direct DECIMAL(19,2) cast is the same trap as ROUND(<float>) \
             — it rounds the binary value and yields 2552.26: {sql}"
        );

        // Rounding is per LINE, never on the folio total: rounding the sum
        // hides per-line drift and still disagrees once enough lines each
        // carry sub-satang residue.
        assert!(
            !sql.contains("ROUND(SUM("),
            "the round belongs on the LINE, before the aggregate: {sql}"
        );
        assert!(sql.contains("CAST(SUM(t.amt) AS FLOAT)"), "got: {sql}");

        // The canonical side needs no round at all — `numeric(14,2)`
        // storage already applied it, per line. That asymmetry IS the
        // invariant; a `round()` appearing here would mean canonical had
        // started holding sub-satang money.
        let pg = pg_folio_sql(LedgerEraFloor::UNFLOORED, false);
        assert!(
            pg.contains("COALESCE(ledger_amount, 0)::numeric(19,4) AS amt"),
            "got: {pg}"
        );
        assert!(
            !pg.contains("round("),
            "canonical money is already stored at 2 dp; rounding it again \
             would be modelling a conversion that does not happen: {pg}"
        );
    }

    /// Reproduce ONE legacy line the way [`legacy_folio_sql`] now reads it:
    /// `ROUND(CAST(<float> AS DECIMAL(19,4)), 2)`.
    ///
    /// `{:.4}` is the float→`DECIMAL(19,4)` step — SQL Server converts the
    /// float through its decimal rendering, so 2552.2649999999999 becomes
    /// the exact 2552.2650, not the binary value just below it. The
    /// integer arithmetic is the `ROUND(…, 2)` step: half AWAY from zero,
    /// the mode T-SQL uses on exact numerics and PostgreSQL `numeric` uses
    /// on storage.
    fn round_line_to_satang(v: f64) -> f64 {
        let dec4 = (format!("{v:.4}")
            .parse::<f64>()
            .expect("a 4 dp rendering always parses")
            * 10_000.0)
            .round() as i64;
        let satang = (dec4.abs() + 50) / 100;
        satang as f64 / 100.0 * if dec4 < 0 { -1.0 } else { 1.0 }
    }

    /// Add a folio's lines the way MSSQL/PostgreSQL do — EXACT decimal
    /// addition at `dp` places — then hand the total over as the
    /// `FLOAT` / `float8` the probe actually reads back. Rust `f64`
    /// addition would introduce accumulation error neither engine has.
    fn sum_decimal(lines: &[f64], dp: usize) -> f64 {
        let scale = 10f64.powi(dp as i32);
        let units: i64 = lines
            .iter()
            .map(|v| (format!("{v:.dp$}").parse::<f64>().unwrap() * scale).round() as i64)
            .sum();
        units as f64 / scale
    }

    /// THE LIVE CASE. HF Hotel folio `CH26-004002`, the single `value`
    /// finding out of 20,389 in-era folios on 2026-08-04.
    ///
    /// iHOTEL derived line 55149's total as `Cin_Pay_Ds_PriceOne *
    /// Cin_Pay_Ds_Num` in float and got 2552.2649999999999; the mirror
    /// stored 2552.27 because `numeric(14,2)` cannot hold the rest; the
    /// money actually taken was 2552.27 on both sides. The folio still
    /// read legacy 6380.665 vs canonical 6380.67 — a `value` row NO
    /// backfill could close, headed for the >72h page over half a satang.
    ///
    /// The second half is the one that matters: the fix must NOT be a
    /// tolerance. A real one-satang difference has to keep diverging.
    #[test]
    fn sub_satang_legacy_line_converges_but_a_genuine_satang_still_diverges() {
        // Read back from HT_CheckIn_Pay / ht_payment_ledger, ids 55121 /
        // 55127 / 55149, three distinct Pay_No values (so all three lines
        // survive the receipt dedupe).
        const LEGACY_LINES: [f64; 3] = [1248.4000000000001, 2580.0, 2552.2649999999999];
        const LEGACY_TENDERS: [f64; 3] = [1248.4000000000001, 2580.0, 2552.27];
        const CANONICAL_LINES: [f64; 3] = [1248.40, 2580.00, 2552.27];

        // Per line, the legacy round lands exactly on what canonical
        // stored — it is the same conversion, not an approximation of it.
        for (raw, stored) in LEGACY_LINES.iter().zip(CANONICAL_LINES) {
            assert_eq!(
                money_hash_segment(round_line_to_satang(*raw)),
                money_hash_segment(stored),
                "rounding legacy {raw} must reproduce the mirrored {stored}"
            );
        }
        // Half away from zero, both signs — NOT half-to-even, and not the
        // binary-value rounding a `float` ROUND would do.
        assert_eq!(round_line_to_satang(2552.265), 2552.27);
        assert_eq!(round_line_to_satang(-2552.265), -2552.27);
        assert_eq!(round_line_to_satang(2552.255), 2552.26);

        let canonical = sum_decimal(&CANONICAL_LINES, 2);
        // Each line of this folio carries its whole total in ONE tender
        // column, so the deduped tender sum equals the itemized one.
        let canonical_tender = canonical;
        let legacy_before = sum_decimal(&LEGACY_LINES, 4);
        let legacy_after = sum_decimal(&LEGACY_LINES.map(round_line_to_satang), 2);
        let legacy_tender = sum_decimal(&LEGACY_TENDERS.map(round_line_to_satang), 2);

        // The bug, pinned to the numbers the live reconcile row recorded.
        assert_eq!(legacy_before, 6380.665);
        assert_eq!(money_hash_segment(legacy_before), "6380.66");
        assert_eq!(money_hash_segment(canonical), "6380.67");

        let folio = "CH26-004002".to_string();
        let pg: BTreeMap<String, FolioSums> =
            [(folio.clone(), sums(3, canonical, canonical_tender))]
                .into_iter()
                .collect();

        let before: BTreeMap<String, FolioSums> =
            [(folio.clone(), sums(3, legacy_before, legacy_tender))]
                .into_iter()
                .collect();
        assert_eq!(
            diff_folios(&before, &pg).len(),
            1,
            "RED: un-rounded, the folio reports drift canonical is incapable \
             of ever converging on"
        );

        // GREEN: rounded per line, the two sides agree.
        let after: BTreeMap<String, FolioSums> =
            [(folio.clone(), sums(3, legacy_after, legacy_tender))]
                .into_iter()
                .collect();
        assert_eq!(money_hash_segment(legacy_after), "6380.67");
        assert!(
            diff_folios(&after, &pg).is_empty(),
            "legacy {legacy_after} vs canonical {canonical} is the SAME money \
             at the precision canonical can store"
        );

        // ...and this is NOT a tolerance. One satang of real difference —
        // canonical short by 0.01 on that same line — still diverges.
        let canonical_one_satang_low = sum_decimal(&[1248.40, 2580.00, 2552.26], 2);
        assert_eq!(money_hash_segment(canonical_one_satang_low), "6380.66");
        let short: BTreeMap<String, FolioSums> = [(
            folio.clone(),
            sums(3, canonical_one_satang_low, canonical_tender),
        )]
        .into_iter()
        .collect();
        assert_eq!(
            diff_folios(&after, &short),
            vec![LedgerFinding::Value {
                folio: folio.clone(),
                legacy: sums(3, legacy_after, legacy_tender),
                pg: sums(3, canonical_one_satang_low, canonical_tender),
            }],
            "a genuine one-satang difference MUST still be reported"
        );
        // The same must hold on the tender half, where the money actually
        // taken lives.
        let tender_short: BTreeMap<String, FolioSums> =
            [(folio, sums(3, canonical, canonical_tender - 0.01))]
                .into_iter()
                .collect();
        assert_eq!(diff_folios(&after, &tender_short).len(), 1);
    }

    /// BOTH outcomes must reach `sync_status`. `record_success` is the only
    /// statement that zeroes `consecutive_failures`, clears `last_error`
    /// and stamps `last_sync_at`; wiring only `record_error` turns the
    /// counter migration 083 seeds into a monotonic LIFETIME failure count.
    #[test]
    fn run_sync_reports_both_probe_outcomes_to_sync_status() {
        let src = include_str!("sync.rs");
        let at = src
            .find("payment_ledger_probe::run_payment_ledger_probe(")
            .expect("run_sync must call the probe");
        // Wide enough to span the whole `match`, which now also feeds the
        // held-watermark tripwire on the Ok arm
        // (`scheduler::sync::note_payment_ledger_era_floor_hold`).
        let call_site = &src[at..(at + 1600).min(src.len())];
        assert!(
            call_site.contains("record_success("),
            "the payment-ledger probe call site records failures but not \
             successes — `consecutive_failures` would never reset"
        );
        assert!(
            call_site.contains("record_error(pg_pool, PAYMENT_LEDGER_PROBE_KEY"),
            "the payment-ledger probe call site must still record failures"
        );
    }
}
