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
//! order can never masquerade as drift.
//!
//! ## Scope floor — `MIN(ledger_legacy_id)`, derived, never configured
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
//! **In-era means the WHOLE folio is in era** (`HAVING MIN(id) >= floor`,
//! not `WHERE id >= floor`). One HF Hotel folio straddles the boundary;
//! admitting it on a filtered line subset would manufacture a permanent
//! `value` finding out of a folio the mirror never claimed to cover.
//!
//! The floor is a low-water mark and could in principle be dragged
//! backwards, exactly as `guest_registry_era_floor_sql` documents: if
//! iHOTEL edits a payment on a pre-era folio, the CT tick mirrors that
//! folio (all of its lines, old ids included) and `MIN(ledger_legacy_id)`
//! collapses to that era. Unlike 6-B this arm does NOT persist a clamped
//! watermark, because the consequence is already bounded by
//! [`PAYMENT_LEDGER_MAX_FOLIO_FINDINGS`]: the next tick's diff exceeds the
//! cap and lands ONE self-describing aggregate row ("legacy 20,281 folios
//! vs canonical 1,300"), not a flood. If that ever fires, the fix is to
//! give this arm a clamped floor of its own —
//! `ht_reconcile_era_floor.era_floor` is a `TIMESTAMP` and cannot hold an
//! id, so it is a schema change, deliberately not made on speculation.
//!
//! ## Cost contract
//!
//! Per tick: TWO PostgreSQL reads (a scalar floor + one grouped scan) and
//! ONE MSSQL grouped scan. There is no cheap "aggregate first, diff only
//! on mismatch" gate as in 6-C, and there deliberately shouldn't be: with
//! a single table the totals cannot be computed without the same GROUP BY
//! the diff needs, so a gate would double the scans it was meant to
//! avoid. What crosses the wire is ~1.3k rows per side.
//!
//! The legacy scan is NOT floored in its `WHERE` clause — the floor is a
//! `HAVING` over the unfiltered per-folio `MIN(id)`, which is what makes
//! the straddling-folio rule above expressible. It is one hash-aggregate
//! pass over a 28k-row table on a shared server, growing ~10k rows/year.
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
/// 50, against the 19 findings HF Hotel actually has today (Ville has 0):
/// comfortably above the live number so the arm reports actionable
/// per-folio detail rather than an opaque total, and far enough below 500
/// that it can never own the sweep. It is also the bound on the
/// dragged-floor failure mode described in the module docs.
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

/// The mirror's own coverage boundary. A `MIN` over an integer IDENTITY
/// column: no date column, no timezone reasoning, no configuration.
pub(crate) const PG_FLOOR_SQL: &str =
    "SELECT MIN(ledger_legacy_id)::bigint FROM ht_payment_ledger";

/// Canonical per-folio aggregate.
///
/// `single = true` narrows it to one `Cin_No` (bound as `$1`) for the
/// auto-resolve sweep; the projection is otherwise IDENTICAL, so a row's
/// resolve hash is computed exactly the way detection computed it.
///
/// The inner `ROW_NUMBER()` is the receipt dedupe; `rn = 1` picks the
/// lowest-id line of each receipt, matching the legacy side and
/// `ROUND_INCOME_BY_TENDER_SQL`'s `DISTINCT ON … ORDER BY … ledger_id`.
/// `numeric(19,4)` before summing keeps the addition exact and
/// order-independent, so it cannot disagree with MSSQL's DECIMAL sum for
/// arithmetic reasons.
pub(crate) fn pg_folio_sql(single: bool) -> String {
    format!(
        "SELECT btrim(ledger_cin_no) AS folio, COUNT(*)::bigint AS line_count, \
                COALESCE(SUM(amt), 0)::float8 AS amount_sum, \
                COALESCE(SUM(CASE WHEN rn = 1 AND st = '1' THEN tender ELSE 0 END), 0)::float8 \
                    AS tender_sum \
           FROM (SELECT ledger_cin_no, \
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
          GROUP BY 1",
        single_filter = if single {
            " AND btrim(ledger_cin_no) = $1"
        } else {
            ""
        },
    )
}

/// Legacy per-folio aggregate, floored at the mirror's coverage.
///
/// The floor is a `HAVING` over the UNFILTERED per-folio `MIN(id)`, so a
/// folio that straddles the boundary is excluded whole rather than
/// compared on a partial line set (see the module docs). It is an integer
/// PostgreSQL itself produced and Rust formats as a bare number, so there
/// is no injection surface and no locale-dependent literal.
///
/// `floor = None` (an empty canonical ledger) deliberately admits the
/// WHOLE legacy table: with no coverage at all, "legacy has 20k folios and
/// we have none" IS the finding, and the cap lands it as one bounded
/// aggregate row.
///
/// `single` narrows to one `Cin_No` (bound as `@P1`) for the auto-resolve
/// sweep and drops the floor — same rule as `fetch_legacy_payment_hash`:
/// this path re-projects a folio the scan ALREADY admitted and logged, so
/// it must reproduce that row's hash unconditionally, or a floor that
/// moved forward would make an open row un-re-projectable.
pub(crate) fn legacy_folio_sql(floor: Option<i64>, single: bool) -> String {
    let tender = "COALESCE(Cin_Pay_Cash, 0) + COALESCE(Cin_Pay_Credit, 0) \
                  + COALESCE(Cin_Pay_Free, 0) + COALESCE(Cin_Pay_Tran, 0) \
                  + COALESCE(Cin_Pay_web, 0)";
    format!(
        "SELECT LTRIM(RTRIM(t.Cin_No)) AS folio, COUNT_BIG(*) AS line_count, \
                CAST(SUM(t.amt) AS FLOAT) AS amount_sum, \
                CAST(SUM(CASE WHEN t.rn = 1 AND t.st = '1' THEN t.tender ELSE 0 END) AS FLOAT) \
                    AS tender_sum \
           FROM (SELECT Cin_No, id, \
                        CAST(COALESCE(Cin_Pay_Ds_Price, {tender}) AS DECIMAL(19,4)) AS amt, \
                        CAST({tender} AS DECIMAL(19,4)) AS tender, \
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
        having = match floor {
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

    // ── Coverage floor, derived from the mirror itself ────────────────
    let floor: Option<i64> = sqlx::query_scalar::<_, Option<i64>>(sqlx::AssertSqlSafe(PG_FLOOR_SQL))
        .fetch_one(pg_pool)
        .await?;

    // ── ONE canonical grouped read ────────────────────────────────────
    let pg_sql = pg_folio_sql(false);
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

    // ── ONE legacy grouped read, floored ──────────────────────────────
    let legacy_sql = legacy_folio_sql(floor, false);
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
            floor,
            duration_ms = duration_ms_of(start),
            "[Sync] Payment-ledger probe: converged"
        );
        return Ok(PaymentLedgerProbeOutcome {
            converged,
            recorded: 0,
            duration_ms: duration_ms_of(start),
        });
    }

    let recorded = if findings.len() > PAYMENT_LEDGER_MAX_FOLIO_FINDINGS {
        record_aggregate_divergence(
            pg_pool,
            &LedgerTotals::of(&legacy_folios),
            &LedgerTotals::of(&pg_folios),
            findings.len(),
            floor,
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
            floor,
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
    })
}

/// ONE row standing for a whole-table divergence, used when the diff is
/// too big to enumerate.
async fn record_aggregate_divergence(
    pg_pool: &PgPool,
    legacy: &LedgerTotals,
    pg: &LedgerTotals,
    findings: usize,
    floor: Option<i64>,
) {
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
        "[Sync] Payment-ledger probe: whole-ledger divergence recorded as ONE \
         aggregate row (too large to enumerate per folio — check whether the \
         coverage floor collapsed onto a pre-era folio)"
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
            "coverage_floor_legacy_id": floor,
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
pub(crate) async fn resolve_pg_hash(
    pg_pool: &PgPool,
    legacy_pk: &str,
) -> Result<Option<String>, sqlx::Error> {
    if legacy_pk == PAYMENT_LEDGER_AGGREGATE_PK {
        let sql = pg_folio_sql(false);
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

    let sql = pg_folio_sql(true);
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
/// coverage floor, and that floor is a `MIN` over the CANONICAL side.
/// Re-deriving it here (rather than freezing it onto the row) means a
/// floor that MOVES because the mirror finally received its missing
/// history is picked up on the next sweep.
pub(crate) async fn resolve_legacy_hash(
    legacy_pool: &DbPool,
    pg_pool: &PgPool,
    legacy_pk: &str,
) -> Result<Option<String>, AnyError> {
    if legacy_pk == PAYMENT_LEDGER_AGGREGATE_PK {
        let floor: Option<i64> =
            sqlx::query_scalar::<_, Option<i64>>(sqlx::AssertSqlSafe(PG_FLOOR_SQL))
                .fetch_one(pg_pool)
                .await?;
        let sql = legacy_folio_sql(floor, false);
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

    let sql = legacy_folio_sql(None, true);
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
    #[test]
    fn legacy_sql_floors_whole_folios_not_lines() {
        let sql = legacy_folio_sql(Some(56353), false);
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
        let unfloored = legacy_folio_sql(None, false);
        assert!(!unfloored.contains("HAVING"));
    }

    /// The single-folio resolve projection must be IDENTICAL to the bulk
    /// one apart from the key filter, or a row's resolve hash could never
    /// equal the hash detection stored. It also drops the floor, matching
    /// `fetch_legacy_payment_hash`: an already-admitted row must stay
    /// re-projectable if the floor moves forward.
    #[test]
    fn single_folio_sql_matches_the_bulk_projection() {
        let bulk = legacy_folio_sql(Some(1), false);
        let single = legacy_folio_sql(Some(1), true);
        assert!(single.contains("AND LTRIM(RTRIM(Cin_No)) = @P1"));
        assert!(!single.contains("HAVING"), "got: {single}");
        assert!(bulk.contains("HAVING"));
        let projection = |s: &str| s.split(" FROM (SELECT").next().unwrap().to_string();
        assert_eq!(projection(&bulk), projection(&single));

        let pg_bulk = pg_folio_sql(false);
        let pg_single = pg_folio_sql(true);
        assert!(pg_single.contains("AND btrim(ledger_cin_no) = $1"));
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
        let pg = pg_folio_sql(false);
        let legacy = legacy_folio_sql(None, false);
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
        assert!(legacy_folio_sql(None, false).contains(LEGACY_AMOUNT_FALLBACK));
        let mapper = include_str!("../sync/mappers/payment.rs");
        assert!(
            mapper.contains("None => cash + credit + free + tran + web,"),
            "the ledger mapper no longer falls back to the tender sum for a \
             NULL Cin_Pay_Ds_Price — re-derive this probe's amount basis"
        );
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
        let call_site = &src[at..(at + 1200).min(src.len())];
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
