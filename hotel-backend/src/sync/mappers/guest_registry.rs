//! Track E1 — Companion-guest CT mapper for `HT_CheckIn_Other_People`
//! (audit 2026-05-13 T2 HIGH-3).
//!
//! ## Problem
//!
//! The legacy iHOTEL app records additional guests in the same room
//! (companions) by INSERTing into `HT_CheckIn_Other_People` on save
//! (FrmCheckIn.cs:9490) and DELETE-then-reinsert on edit
//! (FrmCheckIn.cs:9975). Until Track E1 the table had no PK and no CT
//! subscription, so canonical `ht_guest_registry` was silently stale —
//! TM.30 immigration reporting (legal obligation under Thai foreign-
//! guest registration law) was under-counting companion entries every
//! time the receptionist used the iHOTEL "Other People" tab.
//!
//! ## Mapper shape
//!
//! Per-row dispatch (not aggregate coalescing). CT delivers one I/U/D
//! event per companion row and we UPSERT / DELETE the canonical row
//! keyed on `(legacy_id, guest_cin_id)` where:
//!
//! * `legacy_id` — the legacy IDENTITY `HT_CheckIn_Other_People.id`,
//!   stored in a new `guest_legacy_id` column added for this mapper.
//!   We key on `legacy_id` (not `(guest_firstname, guest_idcard)`)
//!   because the legacy edit path is DELETE+REINSERT — every edit
//!   produces a new identity even though the displayed companion data
//!   is unchanged. Without the legacy ID we'd accumulate duplicate
//!   rows on every edit.
//!
//! * `guest_cin_id` — resolved from the legacy `Cin_no` (a varchar)
//!   via canonical `ht_checkins.legacy_cin_no`. When the parent
//!   check-in hasn't landed yet the apply returns an ERROR so the
//!   watcher holds the watermark and retries loudly. (Pre-2026-06-11
//!   this deferred with `Ok(None)` under the false belief that "the
//!   next CT tick on the parent re-fires this row" — it never does;
//!   the consumed companion CT row was silently dropped and the TM.30
//!   immigration registry under-counted. Same class as the June-3
//!   booking loss.)
//!
//! ## Schema mapping
//!
//! | MSSQL column            | PG canonical column                     |
//! |-------------------------|-----------------------------------------|
//! | `HT_CheckIn_Other_People.id`         | `ht_guest_registry.guest_legacy_id`        |
//! | `HT_CheckIn_Other_People.Cin_no`     | `ht_guest_registry.guest_cin_id` (via lookup) |
//! | `HT_CheckIn_Other_People.Cin_name`   | `ht_guest_registry.guest_firstname` (verbatim) |
//! | `HT_CheckIn_Other_People.Cin_contry` | `ht_guest_registry.guest_nationality`      |
//!
//! `Cin_name` is one free-text field on the legacy side (sometimes
//! "Mr. John Smith", sometimes "John Smith", sometimes the Thai full
//! name). We project it verbatim into `guest_firstname` — splitting
//! into first/last is unreliable on a free-text source and would
//! introduce loss-on-roundtrip.
//!
//! ## Name authority (issue #271)
//!
//! Companions OUR app authored are the one case where canonical holds
//! MORE name structure than legacy: the check-in UI captures two boxes, so
//! the canonical row carries a SPLIT name (`guest_firstname` +
//! `guest_lastname`) while the OUT-leg concatenates them into the single
//! `Cin_name`. The CT echo of that INSERT therefore carries a LOSSY
//! DERIVATIVE of two canonical columns — our own concat coming back — and
//! must never be allowed to overwrite the authored state with it (the same
//! lesson as the payment un-void arm). The rule this mapper applies:
//!
//! * incoming `Cin_name` == what the canonical row already RENDERS
//!   (via [`canonical_companion_name_sql!`]) → the echo carries no new name
//!   information; the authored split is left ALONE and only the non-name
//!   mirrored columns are refreshed
//!   ([`PRESERVE_AUTHORED_COMPANION_NAME_SQL`]);
//! * otherwise → the legacy free text is authoritative: it replaces
//!   `guest_firstname` and `guest_lastname` is CLEARED to NULL, which is
//!   exactly the shape a legacy-originated row has
//!   ([`UPSERT_COMPANION_SQL`]).
//!
//! Both branches leave canonical rendering the legacy bytes, which is what
//! keeps the Phase 6-B folio hash convergeable. The pre-#271 behaviour —
//! take `Cin_name` into `guest_firstname` and leave `guest_lastname` alone
//! — satisfied neither: it rendered `First Last Last`, doubling the surname
//! on every re-import.
//!
//! Per the user's standing constraint legacy literals are passed
//! through unchanged — the deliberate typo `Cin_contry` (sic — kept
//! by iHOTEL since the original schema) is preserved in the SELECT
//! projection.

use async_trait::async_trait;

use crate::outbox::event::DomainEvent;
use crate::sync::change_op::ChangeOp;
use crate::sync::gate_guard::{self, HashInput, HashInputContract};
use crate::sync::mapper::MssqlChangeMapper;
use crate::sync::row::MappableRow;
use crate::sync::SyncError;

const TABLE: &str = "HT_CheckIn_Other_People";

/// CT JOIN projection. Lowercase `Cin_no` per the live schema (NOT
/// `Cin_No` — the capital-N variant is `HT_CheckIn_Ds`'s column).
/// Preserve the iHOTEL typo `Cin_contry` verbatim.
///
/// Held as a module-private const so Track J1's projection-lock test
/// can pin every column against the baseline schema dump.
const GUEST_REGISTRY_SELECT_COLS: &str = "t.id, t.Cin_no, t.Cin_name, t.Cin_contry";

/// The canonical re-concatenation of a companion's display name:
/// `first [+ ' ' + last]`, space-trimmed.
///
/// Canonical `ht_guest_registry` splits the name across two columns while
/// legacy `HT_CheckIn_Other_People.Cin_name` is ONE free-text field, so
/// every canonical-vs-legacy name comparison has to rebuild the legacy
/// shape. Kept as a macro (expanding to a string literal) so BOTH readers
/// are the SAME BYTES at compile time rather than by convention:
///
/// * [`ADOPT_UNSTAMPED_COMPANION_SQL`] — the echo-adoption match below;
/// * `scheduler::sync::canonical_registry_folio_sql` — the Phase 6-B
///   `guest_registry` reconcile arm's canonical projection.
///
/// If those two ever disagreed, a companion our app created and the
/// writeback echoed back would be adopted by the mapper (no duplicate) but
/// hash differently in the reconcile arm — permanent, unfixable sync lag on
/// a legally-load-bearing table (TM.30).
///
/// `TRIM(BOTH FROM …)` is PostgreSQL's default trim: SPACES only, not the
/// broader Unicode-whitespace set. The reconcile arm's legacy side matches
/// that exactly (`trim_matches(' ')`).
macro_rules! canonical_companion_name_sql {
    () => {
        "TRIM(BOTH FROM guest_firstname || CASE \
         WHEN COALESCE(guest_lastname, '') = '' THEN '' \
         ELSE ' ' || guest_lastname END)"
    };
}

/// Non-macro handle on [`canonical_companion_name_sql!`] for callers that
/// just need to interpolate the expression into a larger statement.
pub(crate) const CANONICAL_COMPANION_NAME_SQL: &str = canonical_companion_name_sql!();

/// Echo-adoption UPDATE (convergent companion mirror, 2026-07-01 echo-loop
/// fix). When OUR app added a companion, the delta writeback INSERTed the
/// legacy row VERBATIM and the worker back-populates the captured IDENTITY
/// onto `guest_legacy_id`. If the CT echo of that INSERT arrives BEFORE the
/// back-population lands (the echo-before-stamp race), this UPDATE stamps the
/// matching unstamped canonical row instead of letting the upsert below
/// insert a duplicate. Match is exact-content: the writeback wrote
/// `first [+ ' ' + last]` verbatim, so the comparison re-concatenates the
/// split canonical columns into the same shape ($3 = imported `Cin_name`
/// verbatim). Primary rows never match (`guest_is_primary = false`) — the
/// primary's legacy row is owned by the check-in writeback and its canonical
/// row must never be stamped with a companion id.
const ADOPT_UNSTAMPED_COMPANION_SQL: &str = concat!(
    "UPDATE ht_guest_registry SET guest_legacy_id = $1 \
     WHERE guest_id = ( \
        SELECT guest_id FROM ht_guest_registry \
        WHERE guest_cin_id = $2 AND guest_legacy_id IS NULL \
          AND guest_is_primary = false \
          AND ",
    canonical_companion_name_sql!(),
    " = $3 \
          AND COALESCE(guest_nationality, '') = COALESCE($4, '') \
        ORDER BY guest_id LIMIT 1)"
);

/// Echo-preserve UPDATE (issue #271 — the `First Last Last` class). Runs
/// after [`ADOPT_UNSTAMPED_COMPANION_SQL`], on the row this CT event is
/// ALREADY stamped onto, and fires only when that row RENDERS exactly the
/// incoming `Cin_name`.
///
/// **Why the upsert cannot be left to handle this.** For an app-authored
/// companion the incoming name is our OWN concat echoing back (see the
/// module doc's "Name authority"). Feeding it into `guest_firstname` while
/// `guest_lastname` keeps its value makes the row render `First Last Last`:
/// the surname doubles on every re-import, the reconcile arm's canonical
/// projection (which re-concatenates with these exact bytes) stops matching
/// `Cin_name`, and the `guest_registry` folio drifts permanently on a table
/// TM.30 depends on.
///
/// **The discriminator is the render, not a marker column.** Matching on
/// "`guest_lastname` IS NOT NULL" would also protect the authored split, but
/// it would pin the name against ANY legacy change forever. Comparing the
/// render instead distinguishes the two cases exactly:
///
/// * echo of what we wrote → the renders are equal → preserve;
/// * genuinely changed in iHOTEL → the renders differ → this UPDATE matches
///   0 rows and the change propagates through [`UPSERT_COMPANION_SQL`].
///
/// The realistic iHOTEL rename never even reaches here — iHOTEL edits
/// companions by DELETE-then-REINSERT (`FrmCheckIn.cs:9975`), which arrives
/// as a CT delete plus a fresh IDENTITY — but an in-place UPDATE (a DBA fix,
/// another legacy tool) still converges, and it converges on the legacy
/// bytes.
///
/// Equality is the SAME expression against the SAME raw `$3` bind as the
/// adoption match above, so the two arms are provably one predicate rather
/// than two hand-rolled string shapes. A near-miss (say a legacy name padded
/// with a trailing space, which our OUT-leg never writes — it concatenates
/// pre-trimmed parts) simply falls through to the legacy-authoritative
/// branch; safety does not depend on this comparison being generous, because
/// that branch is convergent too.
///
/// Binds are positionally identical to [`ADOPT_UNSTAMPED_COMPANION_SQL`]:
/// `$1` legacy id, `$2` canonical `cin_id`, `$3` `Cin_name`, `$4`
/// `Cin_contry`.
const PRESERVE_AUTHORED_COMPANION_NAME_SQL: &str = concat!(
    "UPDATE ht_guest_registry \
        SET guest_cin_id = $2, guest_nationality = $4 \
      WHERE guest_legacy_id = $1 \
        AND ",
    canonical_companion_name_sql!(),
    " = $3"
);

/// The mirror UPSERT. The conflict target is the UNIQUE `guest_legacy_id`
/// (migration 034), so a re-import of a row we already mirrored updates in
/// place instead of duplicating.
///
/// The conflict arm is reached ONLY when
/// [`PRESERVE_AUTHORED_COMPANION_NAME_SQL`] did not fire — i.e. the incoming
/// `Cin_name` is NOT what the existing canonical row renders, so the legacy
/// side genuinely holds a different name and its free text is authoritative.
/// Canonical then adopts the legacy shape WHOLE: the free text into
/// `guest_firstname`, `guest_lastname` back to NULL.
///
/// **Clearing the surname is load-bearing, not tidiness.** Keeping a stale
/// `guest_lastname` beside a freshly-taken full name is precisely the
/// `First Last Last` render this pair of statements exists to prevent, and
/// it would leave the folio hash unconvergeable — canonical would render
/// bytes that exist nowhere in legacy.
///
/// Columns the legacy table does not carry (`guest_idcard`,
/// `guest_passport`, `guest_cust_id`) are untouched in both arms: legacy is
/// authoritative for the NAME, never for the TM.30 identity capture.
const UPSERT_COMPANION_SQL: &str = "INSERT INTO ht_guest_registry \
        (guest_cin_id, guest_firstname, guest_nationality, \
         guest_is_primary, guest_legacy_id) \
     VALUES ($1, $2, $3, false, $4) \
     ON CONFLICT (guest_legacy_id) DO UPDATE SET \
        guest_cin_id     = EXCLUDED.guest_cin_id, \
        guest_firstname  = EXCLUDED.guest_firstname, \
        guest_lastname   = NULL, \
        guest_nationality = EXCLUDED.guest_nationality";

// =============================================================================
// Reconcile-hash contract — see `crate::sync::gate_guard` (Phase 6-B)
// =============================================================================

/// One companion as hashed by the `guest_registry` reconcile arm: the
/// display name and the country, and NOTHING else.
///
/// **Ids are deliberately absent.** iHOTEL edits companions by
/// DELETE-then-REINSERT (`FrmCheckIn.cs:9975`), so every edit mints a fresh
/// `HT_CheckIn_Other_People.id` — and the canonical mirror faithfully
/// follows, minting a fresh `guest_id` too. Hashing either id would report
/// a divergence on every companion edit that both sides had already applied
/// correctly: a false positive per edit, forever.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct CompanionEntry {
    /// Legacy `Cin_name` / canonical `first [+ ' ' + last]`, space-trimmed
    /// on BOTH sides — see [`canonical_companion_name_sql!`].
    pub(crate) name: String,
    /// Legacy `Cin_contry` (sic) / canonical `guest_nationality`, with NULL
    /// collapsed to the empty string exactly as
    /// [`ADOPT_UNSTAMPED_COMPANION_SQL`] collapses it. NOT trimmed —
    /// that matches the adoption match, which COALESCEs but never trims.
    pub(crate) country: String,
}

impl CompanionEntry {
    /// This companion's hashed line: `"{name}|{country}"`.
    fn line(&self) -> String {
        format!("{}|{}", self.name, self.country)
    }
}

/// One check-in's companion FOLIO — the unit of reconciliation for Phase
/// 6-B.
///
/// **Why the folio and not the row.** The obvious per-row arm (key on
/// `HT_CheckIn_Other_People.id`) would fire on every legacy edit: the
/// DELETE+REINSERT pattern retires the old id and the reconcile row keyed
/// on it can never converge, while the reinserted id shows up as a brand
/// new `missing_pg`. Reconciling the SET of companions attached to a
/// `Cin_no` is invariant under that churn — it changes only when the
/// companion CONTENT changes, which is the thing we actually care about
/// (TM.30 immigration reporting under-counting).
///
/// Built by both sides of the arm — legacy rows grouped by `Cin_no`,
/// canonical rows grouped by `ht_checkins.legacy_cin_no` — so the two
/// projections are literally the same type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegistryFolioProjection {
    /// Legacy `Cin_no`, i.e. `ht_reconcile_log.legacy_pk` for this entity.
    pub(crate) legacy_cin_no: String,
    /// Unordered; [`Self::companion_lines`] imposes the canonical order.
    pub(crate) companions: Vec<CompanionEntry>,
}

impl RegistryFolioProjection {
    /// An empty folio — a check-in with no companions. A legitimate state
    /// on BOTH sides (and the state most check-ins are in), NOT an "absent
    /// row": empty-vs-empty is convergence, and the arm relies on that to
    /// let a folio whose companions were legitimately deleted everywhere
    /// auto-resolve instead of sitting open forever.
    pub(crate) fn empty(legacy_cin_no: impl Into<String>) -> Self {
        Self {
            legacy_cin_no: legacy_cin_no.into(),
            companions: Vec::new(),
        }
    }

    /// Add one companion, normalising both fields the way the canonical
    /// SQL projection does (space-trim the name, NULL country → `""`).
    /// Applying it on the legacy side too is what keeps the comparison
    /// SYMMETRIC — canonical is trimmed by `TRIM(BOTH FROM …)`, so an
    /// untrimmed legacy `Cin_name` would otherwise diverge permanently.
    pub(crate) fn push_companion(&mut self, name: &str, country: Option<&str>) {
        self.companions.push(CompanionEntry {
            // PostgreSQL's `TRIM(BOTH FROM …)` trims SPACES only — `.trim()`
            // would also eat tabs/newlines and the two sides would disagree.
            name: name.trim_matches(' ').to_string(),
            country: country.unwrap_or_default().to_string(),
        });
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.companions.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.companions.len()
    }

    /// The hashed lines in canonical order: `"{name}|{country}"`, sorted.
    ///
    /// Sorting happens HERE, in Rust, on the UTF-8 bytes — never via a SQL
    /// `ORDER BY`, whose result depends on the server's collation and would
    /// therefore differ between the PG and MSSQL sides (and between the two
    /// sites). Legacy stores no ordering for companions anyway, so the set —
    /// not the sequence — is the meaningful comparison.
    pub(crate) fn companion_lines(&self) -> Vec<String> {
        let mut lines: Vec<String> = self.companions.iter().map(CompanionEntry::line).collect();
        lines.sort();
        lines
    }

    /// The folio's contribution to the reconcile hash: sorted companion
    /// lines joined by `\n`.
    ///
    /// A newline (not the `|` of [`gate_guard::join_hash_segments`])
    /// because each LINE already contains a `|` separating name from
    /// country; reusing `|` at both levels would make `("a|b", "c")` and
    /// `("a", "b|c")` hash alike. Neither field can contain a newline —
    /// both are single-line form inputs in iHOTEL and in our check-in UI.
    pub(crate) fn companions_segment(&self) -> String {
        self.companion_lines().join("\n")
    }
}

/// The inputs `scheduler::sync::guest_registry_canonical_hash` consumes.
///
/// **`HT_CheckIn_Other_People` has NO idempotency gate**: the mapper's I/U
/// branch runs `ADOPT_UNSTAMPED_COMPANION_SQL`, then
/// [`PRESERVE_AUTHORED_COMPANION_NAME_SQL`], then an
/// `INSERT … ON CONFLICT (guest_legacy_id) DO UPDATE` — there is no
/// `existing_matches` chain that could skip a hashed change. The contract
/// therefore declares `always_writes: true` and every `gated_by` is empty,
/// exactly like `rooms` and `payments`.
///
/// The preserve arm added for issue #271 is conditional, but it is NOT a
/// gate in that sense and does not weaken the flag: its condition is that
/// the canonical row already RENDERS the incoming `Cin_name`, i.e. the
/// hashed `companions` segment is byte-identical either way. It can only
/// skip re-writing a name that would hash the same; a name that would move
/// the hash falls through to the upsert. (It still writes the country
/// unconditionally, so the other hashed field cannot be skipped at all.)
///
/// The `id` columns on both sides are excluded ON PURPOSE — see
/// [`CompanionEntry`].
const HASH_INPUTS: [HashInput<RegistryFolioProjection>; 2] = [
    HashInput {
        name: "legacy_cin_no",
        // Row identity: the folio IS the check-in, and both sides resolve
        // their companion set BY this value. A different `Cin_no` is a
        // different folio, never a comparison.
        gated_by: &[],
        segmented: true,
        lookup_key: true,
        segment: |f| f.legacy_cin_no.clone(),
        mutate: |f| f.legacy_cin_no = "CH26-009999".into(),
    },
    HashInput {
        name: "companions",
        gated_by: &[],
        segmented: true,
        lookup_key: false,
        segment: |f| f.companions_segment(),
        mutate: |f| f.push_companion("Somsak Extra", Some("TH")),
    },
];

/// Name-level hash contract, for
/// [`crate::sync::gate_guard::reconcile_entity_contracts`].
pub(crate) fn hash_input_contract() -> Vec<HashInputContract> {
    gate_guard::hash_input_contracts(&HASH_INPUTS)
}

/// No gate exists — see [`HASH_INPUTS`]. Returns empty, and the contract
/// declares `always_writes: true` so that emptiness is a stated fact rather
/// than a silently vacuous check.
pub(crate) fn gate_field_names() -> Vec<&'static str> {
    Vec::new()
}

/// Render the `guest_registry` reconcile-hash body from [`HASH_INPUTS`].
/// Test-only — the production hash is
/// `scheduler::sync::guest_registry_canonical_hash`.
#[cfg(test)]
fn hash_body(f: &RegistryFolioProjection) -> String {
    gate_guard::hash_body(&HASH_INPUTS, f)
}

pub struct GuestRegistryMapper;

#[async_trait]
impl MssqlChangeMapper for GuestRegistryMapper {
    fn table(&self) -> &'static str {
        TABLE
    }

    fn primary_key_cols(&self) -> &'static [&'static str] {
        // IDENTITY column per COMPAT_CHEATSHEET line 571 (schema says
        // NOT IDENTITY but live INFORMATION_SCHEMA.COLUMNS.is_identity=1).
        // Track E1 migration 022 enforces the PK at the CT level.
        &["id"]
    }

    fn select_sql(&self) -> &'static str {
        GUEST_REGISTRY_SELECT_COLS
    }

    async fn apply(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        op: ChangeOp,
        row: Option<&dyn MappableRow>,
    ) -> Result<Option<DomainEvent>, SyncError> {
        let row = row.ok_or_else(|| SyncError::Mapper {
            table: TABLE,
            message: "row required for both I/U and D".into(),
        })?;
        let legacy_id = row.try_get_i32("id")?.ok_or_else(|| SyncError::Mapper {
            table: TABLE,
            message: "id NULL — IDENTITY column should never be NULL post-migration 022".into(),
        })?;

        match op {
            ChangeOp::Delete => {
                // D rows: only `id` (the PK alias) is populated; the
                // joined projection columns are NULL per the LEFT JOIN.
                // Delete the canonical row by legacy_id alone — the
                // (legacy_id, guest_cin_id) pair is also unique but
                // legacy_id alone is globally unique on the IDENTITY
                // column.
                sqlx::query(
                    "DELETE FROM ht_guest_registry WHERE guest_legacy_id = $1",
                )
                .bind(legacy_id)
                .execute(&mut **tx)
                .await?;
                Ok(None)
            }
            ChangeOp::Insert | ChangeOp::Update => {
                let cin_no = row.try_get_str("Cin_no")?;
                let cin_name = row.try_get_str("Cin_name")?;
                let cin_country = row.try_get_str("Cin_contry")?;

                // Resolve parent check-in. NULL Cin_no is a legacy
                // data-quality issue (orphan companion row) — log and
                // skip. Empty string is treated the same.
                let Some(cin_no_str) = cin_no.filter(|s| !s.is_empty()) else {
                    tracing::warn!(
                        legacy_id,
                        "HT_CheckIn_Other_People row has NULL/empty Cin_no — skipping"
                    );
                    return Ok(None);
                };

                // Resolve via canonical PG lookup. A miss MUST error so
                // the watcher holds the watermark — nothing ever
                // re-fires a consumed companion CT row, and a silent
                // skip permanently under-counts the TM.30 registry
                // (June-3 silent-drop class; see `sync::resolve` doc).
                let cin_id_opt: Option<i32> = sqlx::query_scalar(
                    "SELECT cin_id FROM ht_checkins WHERE legacy_cin_no = $1 LIMIT 1",
                )
                .bind(cin_no_str)
                .fetch_optional(&mut **tx)
                .await?;

                let Some(cin_id) = cin_id_opt else {
                    return Err(SyncError::Mapper {
                        table: TABLE,
                        message: format!(
                            "parent check-in FK unresolvable for companion \
                             legacy_id={legacy_id} legacy_cin_no={cin_no_str} \
                             — holding watermark for loud retry"
                        ),
                    });
                };

                // Default firstname to empty string — `NOT NULL` per
                // the schema, but the legacy app sometimes saves a
                // blank "Other People" row when the receptionist
                // tabs through without entering data. Preserve that
                // shape rather than aborting the apply.
                let firstname = cin_name.unwrap_or_default();

                // Echo adoption: if OUR app created this companion (canonical row
                // exists, not yet stamped with a legacy id) and this CT row is the
                // writeback's own INSERT echoing back, stamp the existing row
                // instead of inserting a duplicate. Exact-content match — the
                // delta writeback writes name/country VERBATIM, so an echo always
                // matches byte-for-byte. Back-population usually stamps first
                // (making this a no-op); this closes the race when CT wins.
                let adopted = sqlx::query(ADOPT_UNSTAMPED_COMPANION_SQL)
                    .bind(legacy_id)
                    .bind(cin_id)
                    .bind(&firstname)
                    .bind(&cin_country)
                    .execute(&mut **tx)
                    .await?;
                if adopted.rows_affected() > 0 {
                    return Ok(None);
                }

                // Echo preserve (issue #271): this CT row is the echo of a
                // companion canonical ALREADY owns and already renders
                // identically, so the legacy name carries no new information
                // — for an app-authored row it is our own `first + ' ' + last`
                // concat coming back. Leave the authored split alone and
                // refresh only the non-name mirrored columns. Letting the
                // upsert below run instead would write the concat into
                // `guest_firstname` beside an untouched `guest_lastname` and
                // render `First Last Last`, doubling the surname on every
                // re-import. A row whose legacy name GENUINELY differs matches
                // 0 rows here and falls through, so real changes still
                // propagate.
                let preserved = sqlx::query(PRESERVE_AUTHORED_COMPANION_NAME_SQL)
                    .bind(legacy_id)
                    .bind(cin_id)
                    .bind(firstname)
                    .bind(cin_country)
                    .execute(&mut **tx)
                    .await?;
                if preserved.rows_affected() > 0 {
                    return Ok(None);
                }

                sqlx::query(UPSERT_COMPANION_SQL)
                    .bind(cin_id)
                    .bind(firstname)
                    .bind(cin_country)
                    .bind(legacy_id)
                    .execute(&mut **tx)
                    .await?;
                Ok(None)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sync::row::test_support::HashMapRow;

    #[test]
    fn guest_registry_mapper_advertises_correct_table_and_pk() {
        let m = GuestRegistryMapper;
        assert_eq!(m.table(), "HT_CheckIn_Other_People");
        assert_eq!(m.primary_key_cols(), &["id"]);
    }

    #[test]
    fn guest_registry_mapper_select_projects_all_four_columns() {
        let select = GuestRegistryMapper.select_sql();
        for col in &["id", "Cin_no", "Cin_name", "Cin_contry"] {
            assert!(
                select.contains(col),
                "select_sql must project {col}; got: {select}"
            );
        }
    }

    #[test]
    fn guest_registry_mapper_preserves_legacy_typo_cin_contry() {
        let select = GuestRegistryMapper.select_sql();
        assert!(
            select.contains("Cin_contry"),
            "must keep legacy typo verbatim (Cin_contry, not Cin_country)"
        );
        assert!(
            !select.contains("Cin_country"),
            "must NOT silently 'fix' the legacy spelling"
        );
    }

    #[test]
    fn guest_registry_mapper_uses_lowercase_cin_no_for_other_people() {
        // HT_CheckIn_Other_People uses lowercase n, distinct from
        // HT_CheckIn_Ds which uses capital N. Schema dump verified.
        let select = GuestRegistryMapper.select_sql();
        assert!(select.contains("Cin_no"));
        // Watch for accidental copy-paste from HT_CheckIn_Ds.
        assert!(
            !select.contains("Cin_No"),
            "Other_People uses lowercase Cin_no — caught a capital-N copy-paste"
        );
    }

    /// Mirror mappers don't coalesce — this is a flat-row dispatch.
    /// Locking that the trait surface stays per-row keyed.
    #[test]
    fn guest_registry_mapper_does_not_coalesce() {
        let m = GuestRegistryMapper;
        let row = HashMapRow::new(TABLE);
        assert!(m.coalesce_key(&row).is_none());
    }

    /// Track J1 — projection-lock guard.
    #[test]
    fn guest_registry_select_cols_are_subset_of_legacy_schema() {
        crate::assert_projection_subset!(
            GUEST_REGISTRY_SELECT_COLS,
            "HT_CheckIn_Other_People"
        );
    }

    /// Echo-adoption SQL shape (convergent companion mirror). Pins the
    /// invariants that make the 2026-07-01 echo loop impossible to re-open:
    /// only UNSTAMPED (`guest_legacy_id IS NULL`) NON-primary rows are
    /// adoptable, and the name comparison re-concatenates the split canonical
    /// columns (`first [+ ' ' + last]`) because the OUT-leg wrote that exact
    /// verbatim shape into `Cin_name`.
    #[test]
    fn adoption_sql_targets_unstamped_non_primary_with_concat_name_match() {
        let sql = ADOPT_UNSTAMPED_COMPANION_SQL;
        assert!(sql.contains("SET guest_legacy_id = $1"), "{sql}");
        assert!(sql.contains("guest_legacy_id IS NULL"), "{sql}");
        assert!(sql.contains("guest_is_primary = false"), "{sql}");
        // Name match must be against the concatenated first+last shape,
        // never guest_firstname alone (the OUT-leg name was first + ' ' + last).
        assert!(
            sql.contains("guest_firstname || CASE"),
            "concatenated name comparison: {sql}"
        );
        assert!(sql.contains("' ' || guest_lastname"), "{sql}");
        assert!(
            sql.contains("COALESCE(guest_nationality, '') = COALESCE($4, '')"),
            "{sql}"
        );
        // Deterministic single-row adoption.
        assert!(sql.contains("ORDER BY guest_id LIMIT 1"), "{sql}");
    }

    // ----- reconcile-hash contract (see `crate::sync::gate_guard`) -------

    fn folio(cin_no: &str, companions: &[(&str, Option<&str>)]) -> RegistryFolioProjection {
        let mut f = RegistryFolioProjection::empty(cin_no);
        for (name, country) in companions {
            f.push_companion(name, *country);
        }
        f
    }

    /// The adoption match and the reconcile arm's canonical projection MUST
    /// rebuild the legacy name the same way. Compile-time single-sourced via
    /// `canonical_companion_name_sql!`; this pins that the adoption SQL still
    /// carries those exact bytes, so a "tidy-up" of either cannot silently
    /// split them.
    #[test]
    fn adoption_sql_uses_the_shared_canonical_name_expression() {
        assert!(
            ADOPT_UNSTAMPED_COMPANION_SQL.contains(CANONICAL_COMPANION_NAME_SQL),
            "adoption SQL no longer contains the shared name expression:\n{}\n{}",
            ADOPT_UNSTAMPED_COMPANION_SQL,
            CANONICAL_COMPANION_NAME_SQL,
        );
        // The bytes themselves, so a reformat of the macro is a visible
        // decision (the reconcile arm interpolates this into its SELECT).
        assert_eq!(
            CANONICAL_COMPANION_NAME_SQL,
            "TRIM(BOTH FROM guest_firstname || CASE WHEN COALESCE(guest_lastname, '') = '' \
             THEN '' ELSE ' ' || guest_lastname END)"
        );
    }

    // ----- issue #271: the "First Last Last" class -----------------------

    /// Collapse runs of spaces so an assertion reads like the SQL rather
    /// than like its line-continuation whitespace.
    fn squash(sql: &str) -> String {
        sql.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    /// Rust model of [`canonical_companion_name_sql!`]: what a stored row
    /// RENDERS. That render is both the preserve arm's comparison value and
    /// the value the Phase 6-B reconcile arm hashes on the canonical side,
    /// which is why these tests can reason about folio convergence.
    ///
    /// Checked against live PostgreSQL 17 (the production major, PG
    /// 17-alpine) while developing the fix: split, un-split, empty-string
    /// surname and padded cases all agree with the SQL expression.
    fn canonical_render(firstname: &str, lastname: Option<&str>) -> String {
        let joined = match lastname.unwrap_or_default() {
            // `COALESCE(guest_lastname, '') = ''` — NULL and '' alike.
            "" => firstname.to_string(),
            last => format!("{firstname} {last}"),
        };
        // `TRIM(BOTH FROM …)` — SPACES only, as `push_companion` documents.
        joined.trim_matches(' ').to_string()
    }

    /// The bytes the OUT-leg puts in `Cin_name` for an app-authored
    /// companion: `routes::new_checkins::create_guest` builds
    /// `first [+ ' ' + last]` and
    /// `writeback::recipes::companion_people::build_add_sql` writes it
    /// VERBATIM (no country prefix, no trailing space — unlike the
    /// primary-row shape).
    fn out_leg_cin_name(first: &str, last: Option<&str>) -> String {
        match last.map(str::trim).filter(|s| !s.is_empty()) {
            Some(last) => format!("{first} {last}"),
            None => first.to_string(),
        }
    }

    /// The preserve arm keys on the row this CT event is already stamped
    /// onto and discriminates on the SHARED render expression — not on a
    /// marker column, so a genuine legacy rename is not pinned out.
    #[test]
    fn preserve_sql_is_keyed_on_the_stamped_row_and_the_shared_name_expression() {
        let sql = squash(PRESERVE_AUTHORED_COMPANION_NAME_SQL);
        assert!(sql.contains("UPDATE ht_guest_registry"), "{sql}");
        assert!(sql.contains("WHERE guest_legacy_id = $1"), "{sql}");
        assert!(
            PRESERVE_AUTHORED_COMPANION_NAME_SQL.contains(CANONICAL_COMPANION_NAME_SQL),
            "preserve arm must reuse the shared render expression verbatim, \
             else it could preserve a name the reconcile arm hashes \
             differently:\n{sql}"
        );
        assert!(sql.ends_with("= $3"), "compared against the raw Cin_name: {sql}");
    }

    /// The whole point of the arm: on an echo it must not touch EITHER name
    /// column. Writing `guest_firstname` here would reintroduce
    /// `First Last Last` by another route.
    #[test]
    fn preserve_sql_never_writes_either_name_column() {
        let sql = squash(PRESERVE_AUTHORED_COMPANION_NAME_SQL);
        let set = sql
            .split_once("SET ")
            .expect("preserve arm is an UPDATE … SET")
            .1
            .split_once(" WHERE ")
            .expect("… WHERE")
            .0
            .to_string();
        assert_eq!(
            set, "guest_cin_id = $2, guest_nationality = $4",
            "the echo path may refresh ONLY the non-name mirrored columns"
        );
    }

    /// Both echo arms take the same four binds in the same order, so the two
    /// `sqlx::query` call sites cannot drift into passing the name where the
    /// country belongs.
    #[test]
    fn preserve_sql_binds_positionally_like_the_adoption_sql() {
        let adopt = squash(ADOPT_UNSTAMPED_COMPANION_SQL);
        let preserve = squash(PRESERVE_AUTHORED_COMPANION_NAME_SQL);
        // $1 = legacy id, $2 = canonical cin_id, $3 = Cin_name, $4 = Cin_contry.
        assert!(adopt.contains("SET guest_legacy_id = $1"), "{adopt}");
        assert!(preserve.contains("guest_legacy_id = $1"), "{preserve}");
        assert!(adopt.contains("guest_cin_id = $2"), "{adopt}");
        assert!(preserve.contains("guest_cin_id = $2"), "{preserve}");
        // Raw consts here, not `squash`ed: the shared expression contains a
        // quoted space (`' '`) that whitespace normalisation must not touch.
        for sql in [
            ADOPT_UNSTAMPED_COMPANION_SQL,
            PRESERVE_AUTHORED_COMPANION_NAME_SQL,
        ] {
            assert!(
                sql.contains(&format!("{CANONICAL_COMPANION_NAME_SQL} = $3")),
                "$3 must be the name, compared via the shared expression, on \
                 both echo arms: {sql}"
            );
        }
        assert!(adopt.contains("COALESCE($4, '')"), "{adopt}");
        assert!(preserve.contains("guest_nationality = $4"), "{preserve}");
    }

    /// Fall-through means the legacy name genuinely differs, so canonical
    /// takes the legacy free text WHOLE — surname cleared. Leaving a stale
    /// `guest_lastname` beside a freshly-taken full name is the
    /// `First Last Last` render itself.
    #[test]
    fn upsert_conflict_arm_clears_the_lastname_when_it_takes_the_legacy_name() {
        let sql = squash(UPSERT_COMPANION_SQL);
        assert!(sql.contains("ON CONFLICT (guest_legacy_id) DO UPDATE SET"), "{sql}");
        assert!(sql.contains("guest_firstname = EXCLUDED.guest_firstname"), "{sql}");
        assert!(
            sql.contains("guest_lastname = NULL"),
            "taking the legacy name without clearing the surname renders \
             `First Last Last` and makes the folio hash unconvergeable: {sql}"
        );
        assert!(sql.contains("guest_nationality = EXCLUDED.guest_nationality"), "{sql}");
        // Identity capture is canonical-only — legacy owns the NAME, not TM.30 docs.
        for col in ["guest_idcard", "guest_passport", "guest_cust_id"] {
            assert!(!sql.contains(col), "conflict arm must not touch {col}: {sql}");
        }
    }

    /// Regression guard for the whole class, stated structurally: no
    /// statement in this mapper may adopt the legacy name without also
    /// dropping the authored surname.
    #[test]
    fn no_statement_takes_the_legacy_name_without_clearing_the_lastname() {
        for sql in [
            ADOPT_UNSTAMPED_COMPANION_SQL,
            PRESERVE_AUTHORED_COMPANION_NAME_SQL,
            UPSERT_COMPANION_SQL,
        ] {
            let s = squash(sql);
            if s.contains("guest_firstname = EXCLUDED.guest_firstname") {
                assert!(
                    s.contains("guest_lastname = NULL"),
                    "statement takes the legacy name but keeps the authored \
                     surname — that is the #271 `First Last Last` render: {s}"
                );
            }
        }
    }

    /// Echo re-import of an app-authored row: the split survives and the
    /// folio stays converged.
    #[test]
    fn echo_reimport_of_an_authored_row_preserves_the_split_and_stays_converged() {
        use crate::scheduler::sync::guest_registry_canonical_hash;

        // Two boxes in our check-in UI …
        let (first, last) = ("Somchai", Some("Jaidee"));
        // … one field in legacy, written VERBATIM by the delta writeback.
        let cin_name = out_leg_cin_name(first, last);
        assert_eq!(cin_name, "Somchai Jaidee");
        let legacy_folio = folio("CH26-005228", &[(cin_name.as_str(), Some("TH"))]);

        // The stamped row already renders those exact bytes, so the preserve
        // arm's predicate holds: no name write, no surname doubling.
        let preserved = canonical_render(first, last);
        assert_eq!(preserved, cin_name, "no `Last Last`");
        assert_eq!(
            guest_registry_canonical_hash(&folio(
                "CH26-005228",
                &[(preserved.as_str(), Some("TH"))]
            )),
            guest_registry_canonical_hash(&legacy_folio),
            "the preserved split must hash exactly like the legacy Cin_name"
        );

        // Pre-#271: the conflict arm wrote `Cin_name` into `guest_firstname`
        // and left `guest_lastname` alone.
        let doubled = canonical_render(&cin_name, last);
        assert_eq!(doubled, "Somchai Jaidee Jaidee", "the reported symptom");
        assert_ne!(
            guest_registry_canonical_hash(&folio(
                "CH26-005228",
                &[(doubled.as_str(), Some("TH"))]
            )),
            guest_registry_canonical_hash(&legacy_folio),
            "canonical would render bytes that exist nowhere in legacy — \
             permanent folio sync lag on a TM.30 table"
        );
        // It is stable-but-wrong rather than compounding: `Cin_name` stays
        // constant, so a further echo re-derives the same doubled render. It
        // only STACKED under the retired replace-all shape, which wrote the
        // canonical render back to legacy (the 2026-07-01 echo loop).
        assert_eq!(canonical_render(&cin_name, last), doubled);
    }

    /// A genuine legacy-side rename still propagates — the discriminator is
    /// the render, not a marker column.
    #[test]
    fn a_genuine_legacy_rename_propagates_and_converges_on_the_legacy_bytes() {
        use crate::scheduler::sync::guest_registry_canonical_hash;

        // A receptionist typo-fix in iHOTEL normally arrives as
        // DELETE+REINSERT (`FrmCheckIn.cs:9975`) — a fresh IDENTITY, so the
        // mapper inserts a fresh row and neither echo arm is involved. This
        // pins the in-place UPDATE case (a DBA fix, another legacy tool).
        let renamed = "Somchai Jaidii";
        let legacy_folio = folio("CH26-005228", &[(renamed, Some("TH"))]);

        // The preserve arm cannot fire: the stored row renders the OLD name.
        assert_ne!(canonical_render("Somchai", Some("Jaidee")), renamed);

        // So the conflict arm takes the free text and clears the surname —
        // canonical lands in the legacy-originated shape and converges.
        let after = canonical_render(renamed, None);
        assert_eq!(after, renamed);
        assert_eq!(
            guest_registry_canonical_hash(&folio("CH26-005228", &[(after.as_str(), Some("TH"))])),
            guest_registry_canonical_hash(&legacy_folio),
        );

        // Keeping the authored surname (option (a): "preserve whenever a
        // split exists") would NOT converge here.
        assert_ne!(canonical_render(renamed, Some("Jaidee")), renamed);
    }

    /// Legacy-originated companions have no authored split, so their render
    /// IS the stored free text: the preserve arm fires on every re-import and
    /// the name column is never rewritten. Prefixes and Thai full names
    /// survive verbatim, exactly as the module doc promises — this fix does
    /// not sneak a name split in through the back door.
    #[test]
    fn legacy_originated_rows_keep_their_free_text_name_verbatim() {
        for name in ["Mr. John Smith", "สมชาย ใจดี", "นาย สมชาย ใจดี", ""] {
            assert_eq!(canonical_render(name, None), name);
            // Blank surnames are the same state as NULL (`COALESCE`).
            assert_eq!(canonical_render(name, Some("")), name);
        }
    }

    /// Byte-parity pin for the NEW `guest_registry` descriptor (Phase 6-B).
    /// `ht_reconcile_log.mssql_hash` and `ht_guest_registry_legacy.sync_hash`
    /// are stored SHA256s of this exact body — one byte of drift invalidates
    /// every ack and triggers a full re-diff storm.
    ///
    /// No behavioural gate-mutation test, same reason as rooms/payments:
    /// this mapper has no idempotency gate to defeat (`always_writes: true`).
    #[test]
    fn guest_registry_hash_bytes_unchanged_for_golden_inputs() {
        use crate::scheduler::sync::{guest_registry_canonical_hash, sha256};

        let f = folio(
            "CH26-005228",
            &[("Somchai Jaidee", Some("TH")), ("Somsri Kaew", None)],
        );

        // Body shape: cin_no | sorted "name|country" lines joined by \n.
        let expected = sha256("CH26-005228|Somchai Jaidee|TH\nSomsri Kaew|");

        assert_eq!(
            guest_registry_canonical_hash(&f),
            expected,
            "production guest-registry folio hash changed bytes"
        );
        assert_eq!(
            sha256(&hash_body(&f)),
            expected,
            "HASH_INPUTS join no longer reproduces the production hash body"
        );
    }

    /// An empty folio is a real, hashable state — a check-in with no
    /// companions. Both sides must produce the SAME bytes for it, or a
    /// folio whose companions were legitimately deleted everywhere could
    /// never converge.
    #[test]
    fn empty_folio_hashes_to_a_stable_value_on_both_sides() {
        use crate::scheduler::sync::{guest_registry_canonical_hash, sha256};

        let empty = RegistryFolioProjection::empty("CH26-005228");
        assert!(empty.is_empty());
        assert_eq!(
            guest_registry_canonical_hash(&empty),
            sha256("CH26-005228|"),
            "the empty folio's body is the key plus an empty companion segment"
        );
        assert_ne!(
            guest_registry_canonical_hash(&empty),
            guest_registry_canonical_hash(&folio("CH26-005228", &[("Somchai", None)])),
            "an empty folio must not hash like a populated one"
        );
    }

    /// Legacy stores no ordering for companions and iHOTEL rewrites the
    /// whole set on edit, so the SAME set inserted in a different order
    /// MUST hash identically — otherwise every DELETE+reinsert edit is a
    /// false positive.
    #[test]
    fn companion_order_does_not_affect_the_folio_hash() {
        use crate::scheduler::sync::guest_registry_canonical_hash;

        let a = folio(
            "CH26-005228",
            &[("Somchai", Some("TH")), ("Anong", Some("LA"))],
        );
        let b = folio(
            "CH26-005228",
            &[("Anong", Some("LA")), ("Somchai", Some("TH"))],
        );
        assert_eq!(
            guest_registry_canonical_hash(&a),
            guest_registry_canonical_hash(&b)
        );
    }

    /// Duplicate companions are NOT collapsed: two guests with the same
    /// name in one room is a real (if uncommon) folio, and iHOTEL stores
    /// two rows for it. Collapsing them would hide a dropped CT delete.
    #[test]
    fn duplicate_companions_are_kept_distinct() {
        use crate::scheduler::sync::guest_registry_canonical_hash;

        let one = folio("CH26-005228", &[("Somchai", Some("TH"))]);
        let two = folio(
            "CH26-005228",
            &[("Somchai", Some("TH")), ("Somchai", Some("TH"))],
        );
        assert_eq!(two.len(), 2);
        assert_ne!(
            guest_registry_canonical_hash(&one),
            guest_registry_canonical_hash(&two)
        );
    }

    /// The canonical side is trimmed by `TRIM(BOTH FROM …)` (SPACES only),
    /// so the legacy side must normalise identically — otherwise a padded
    /// `Cin_name` diverges permanently with nothing to fix. A tab is NOT
    /// trimmed, because PostgreSQL's default `TRIM` does not trim it either.
    #[test]
    fn companion_name_is_space_trimmed_exactly_like_postgres_trim() {
        use crate::scheduler::sync::guest_registry_canonical_hash;

        assert_eq!(
            guest_registry_canonical_hash(&folio("CH1", &[("  Somchai  ", None)])),
            guest_registry_canonical_hash(&folio("CH1", &[("Somchai", None)])),
        );
        assert_ne!(
            guest_registry_canonical_hash(&folio("CH1", &[("\tSomchai", None)])),
            guest_registry_canonical_hash(&folio("CH1", &[("Somchai", None)])),
            "PostgreSQL TRIM(BOTH FROM …) leaves a tab in place — so must we"
        );
    }

    /// NULL country and empty-string country are the SAME state: the
    /// adoption match COALESCEs both sides to `''` and the canonical
    /// projection does too. Live evidence 2026-07-28: 20,423 of 20,434
    /// legacy companion rows carry an empty `Cin_contry`.
    #[test]
    fn null_and_empty_country_are_the_same_folio() {
        use crate::scheduler::sync::guest_registry_canonical_hash;

        assert_eq!(
            guest_registry_canonical_hash(&folio("CH1", &[("Somchai", None)])),
            guest_registry_canonical_hash(&folio("CH1", &[("Somchai", Some(""))])),
        );
    }

    /// Self-validating mutators: each descriptor entry must actually move
    /// its own segment, else a future behavioural test built on this table
    /// would pass vacuously.
    #[test]
    fn guest_registry_hash_mutators_all_move_their_segment() {
        let base = folio("CH26-005228", &[("Somchai Jaidee", Some("TH"))]);
        for input in HASH_INPUTS.iter() {
            let before = (input.segment)(&base);
            let mut mutated = base.clone();
            (input.mutate)(&mut mutated);
            let after = (input.segment)(&mutated);
            assert_ne!(
                before, after,
                "hash input `{}`: mutator did not move the hashed segment",
                input.name,
            );
        }
    }
}
