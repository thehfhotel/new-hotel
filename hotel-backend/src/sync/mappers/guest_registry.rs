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
//! Per the user's standing constraint legacy literals are passed
//! through unchanged — the deliberate typo `Cin_contry` (sic — kept
//! by iHOTEL since the original schema) is preserved in the SELECT
//! projection.

use async_trait::async_trait;

use crate::outbox::event::DomainEvent;
use crate::sync::change_op::ChangeOp;
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

                sqlx::query(
                    "INSERT INTO ht_guest_registry \
                        (guest_cin_id, guest_firstname, guest_nationality, \
                         guest_is_primary, guest_legacy_id) \
                     VALUES ($1, $2, $3, false, $4) \
                     ON CONFLICT (guest_legacy_id) DO UPDATE SET \
                        guest_cin_id     = EXCLUDED.guest_cin_id, \
                        guest_firstname  = EXCLUDED.guest_firstname, \
                        guest_nationality = EXCLUDED.guest_nationality",
                )
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
}
