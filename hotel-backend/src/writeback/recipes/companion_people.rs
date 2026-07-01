//! `MirrorCompanion` recipe — Phase 4 (TM.30 companion guests).
//!
//! Mirrors a companion guest added to a folio (via `POST
//! /api/checkins/{id}/guests`) into the legacy `HT_CheckIn_Other_People` table.
//! The legacy table carries only name + country (no id-card column), so this
//! recipe is a single INSERT mirroring the walk-in recipe's TM.30 primary-guest
//! row (walk-in stmt N+1):
//!
//! ```text
//! INSERT INTO [HT_CheckIn_Other_People]([Cin_no],[Cin_name],[Cin_contry])
//!   VALUES(<cin_no>, '<prefix> <name> ', <country>)
//! ```
//!
//! `Cin_name` carries the country-derived personal prefix
//! ([`super::helpers::guest_prefix_for_country`]) plus a trailing space — the
//! exact byte shape captured from iHOTEL and already emitted by the walk-in
//! recipe (`'Mr. NAME '` / `'นาย NAME '`).
//!
//! **Shipped DARK** behind `TM30_COMPANION_WRITEBACK_ENABLED` — the emitter
//! (the `create_guest` route) is gated; this recipe is always compiled.

use crate::db::mssql_timeout::{simple_query_with_timeout, MssqlOpKind};
use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::{WritebackError, WritebackResult};
use crate::writeback::format::sql_quote;

/// `LegacyIds.extra` key carrying the freshly-allocated
/// `HT_CheckIn_Other_People.id` (IDENTITY) captured by [`execute_add`]. The
/// writeback worker's `back_populate_legacy_ids` step reads it back to stamp
/// `ht_guest_registry.guest_legacy_id`.
pub const EXTRA_OTHER_PEOPLE_ID: &str = "other_people_id";

/// Build the `HT_CheckIn_Other_People` INSERT. PURE — no I/O. Mirrors the
/// walk-in recipe's TM.30 row byte-for-byte: `Cin_name` = `prefix + ' ' + name
/// + ' '`, `Cin_contry` = country verbatim.
pub fn build_insert_sql(cin_no: &str, name: &str, country: &str) -> String {
    let prefix = super::helpers::guest_prefix_for_country(country);
    // Trailing space matches the captured pattern (`prefix + ' ' + name + ' '`,
    // where the final space comes from the empty `name2` in the legacy app).
    let registry_name = format!("{prefix} {name} ");
    format!(
        "INSERT INTO [HT_CheckIn_Other_People]([Cin_no],[Cin_name],[Cin_contry])\
         VALUES({cin_no_q},{name_q},{country_q})",
        cin_no_q = sql_quote(cin_no),
        name_q = sql_quote(&registry_name),
        country_q = sql_quote(country),
    )
}

/// Execute the companion INSERT. Returns an empty [`LegacyIds`] —
/// `HT_CheckIn_Other_People.id` is IDENTITY; the ledger (keyed on the job's
/// idempotency_key) guards the duplicate-on-retry class.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    cin_no: &str,
    name: &str,
    country: &str,
) -> WritebackResult<LegacyIds> {
    let sql = build_insert_sql(cin_no, name, country);
    super::execute_all(conn, &[sql]).await?;
    Ok(LegacyIds::new())
}

/// Build the replace-all statement list (iHOTEL parity, FrmCheckIn.cs:9975):
/// one `DELETE … WHERE Cin_no = '<cin>'` followed by one `build_insert_sql`
/// per companion, in order. PURE — no I/O — so the byte shape is unit-testable.
fn build_replace_all_stmts(
    cin_no: &str,
    companions: &[crate::outbox::intent::CompanionEntry],
) -> Vec<String> {
    let mut stmts: Vec<String> = Vec::with_capacity(companions.len() + 1);
    stmts.push(format!(
        "DELETE FROM HT_CheckIn_Other_People WHERE Cin_no = {}",
        sql_quote(cin_no)
    ));
    for c in companions {
        stmts.push(build_insert_sql(cin_no, &c.name, &c.country));
    }
    stmts
}

/// Replace-all companion mirror (iHOTEL parity, FrmCheckIn.cs:9975): DELETE every
/// HT_CheckIn_Other_People row for the Cin_no, then INSERT the current list.
///
/// **DEPRECATED — nothing emits `MirrorCompanionList` anymore** (2026-07-01
/// echo-loop incident; see the intent doc). Kept routable for historical
/// queue rows.
pub async fn execute_replace_all(
    conn: &mut LegacyConn<'_>,
    cin_no: &str,
    companions: &[crate::outbox::intent::CompanionEntry],
) -> WritebackResult<LegacyIds> {
    let stmts = build_replace_all_stmts(cin_no, companions);
    super::execute_all(conn, &stmts).await?;
    Ok(LegacyIds::new())
}

/// Build the VERBATIM companion INSERT (delta path). Unlike the primary-row
/// shape there is NO country prefix and NO trailing space — iHOTEL's own
/// companion rows are the raw ListView text (FrmCheckIn.cs:9490), and any
/// transformation here would stack on the round-trip (the 2026-07-01 echo
/// incident). Emits a SELECT of SCOPE_IDENTITY() so execute can capture the id.
pub fn build_add_sql(cin_no: &str, name: &str, country: &str) -> String {
    format!(
        "INSERT INTO [HT_CheckIn_Other_People]([Cin_no],[Cin_name],[Cin_contry])\
         VALUES({cin_no_q},{name_q},{country_q}); SELECT CAST(SCOPE_IDENTITY() AS INT)",
        cin_no_q = sql_quote(cin_no),
        name_q = sql_quote(name),
        country_q = sql_quote(country),
    )
}

/// Build the delta companion DELETE. Keyed on the known legacy IDENTITY `id`;
/// the `Cin_no` guard prevents a stale/wrong id from deleting another folio's
/// row. NEVER a Cin_no-wide delete (that was the replace-all echo-loop shape).
/// PURE — no I/O.
pub fn build_delete_sql(cin_no: &str, legacy_id: i64) -> String {
    format!(
        "DELETE FROM HT_CheckIn_Other_People WHERE id = {legacy_id} AND Cin_no = {cin_no_q}",
        cin_no_q = sql_quote(cin_no),
    )
}

/// Delta companion ADD — one VERBATIM INSERT, capturing the freshly-allocated
/// IDENTITY id via the same-batch `SELECT CAST(SCOPE_IDENTITY() AS INT)` (one
/// `simple_query` batch, so no cross-batch scope loss — the audit-H12 hazard
/// only bites when INSERT and SELECT are separate calls). The id lands in
/// `LegacyIds.extra[`[`EXTRA_OTHER_PEOPLE_ID`]`]` for the worker to stamp onto
/// `ht_guest_registry.guest_legacy_id`, which is what lets the CT echo be
/// absorbed idempotently by the mapper's ON CONFLICT upsert.
pub async fn execute_add(
    conn: &mut LegacyConn<'_>,
    cin_no: &str,
    name: &str,
    country: &str,
) -> WritebackResult<LegacyIds> {
    let sql = build_add_sql(cin_no, name, country);
    // Write budget: the INSERT is the write; the trailing SELECT rides the
    // same batch (same envelope as allocate.rs's MAX+1 SELECT reads).
    let rows = simple_query_with_timeout(conn, &sql, MssqlOpKind::Write).await?;
    let id: i32 = rows
        .first()
        .and_then(|r| r.get::<i32, _>(0))
        .ok_or_else(|| {
            WritebackError::Recipe(
                "CompanionAdd INSERT returned no SCOPE_IDENTITY id — cannot \
                 back-populate guest_legacy_id"
                    .into(),
            )
        })?;
    let mut ids = LegacyIds::new();
    ids.extra
        .insert(EXTRA_OTHER_PEOPLE_ID.to_string(), serde_json::json!(id));
    Ok(ids)
}

/// Delta companion DELETE — single guarded statement (id + Cin_no). Naturally
/// idempotent: a second apply matches 0 rows. Returns empty [`LegacyIds`].
pub async fn execute_delete(
    conn: &mut LegacyConn<'_>,
    cin_no: &str,
    legacy_id: i64,
) -> WritebackResult<LegacyIds> {
    let sql = build_delete_sql(cin_no, legacy_id);
    super::execute_all(conn, std::slice::from_ref(&sql)).await?;
    Ok(LegacyIds::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn companion_uses_mr_prefix_for_non_thai_country() {
        let sql = build_insert_sql("CH26-005228", "Thomas Meininghaus", "DE");
        assert!(sql.contains("'Mr. Thomas Meininghaus '"), "{sql}");
        assert!(sql.contains("'CH26-005228'"), "{sql}");
        assert!(sql.contains("'DE'"), "{sql}");
    }

    #[test]
    fn companion_uses_thai_prefix_for_th_country() {
        let sql = build_insert_sql("CH26-005228", "อุทัย สุขผล", "TH");
        assert!(sql.contains("'นาย อุทัย สุขผล '"), "{sql}");
    }

    #[test]
    fn replace_all_deletes_then_inserts_in_order() {
        use crate::outbox::intent::CompanionEntry;
        let companions = vec![
            CompanionEntry {
                name: "Thomas Meininghaus".to_string(),
                country: "DE".to_string(),
            },
            CompanionEntry {
                name: "อุทัย สุขผล".to_string(),
                country: "TH".to_string(),
            },
        ];
        let stmts = build_replace_all_stmts("CH26-005228", &companions);
        // DELETE first, then one INSERT per companion (N+1 total).
        assert_eq!(stmts.len(), 3, "{stmts:?}");
        assert_eq!(
            stmts[0],
            "DELETE FROM HT_CheckIn_Other_People WHERE Cin_no = 'CH26-005228'",
            "{stmts:?}"
        );
        // Plain `'…'` literal, never `N'…'` (TIS-620 byte-parity).
        assert!(!stmts[0].contains("N'"), "{stmts:?}");
        // INSERTs follow in list order, each carrying the prefixed Cin_name.
        assert!(stmts[1].contains("'Mr. Thomas Meininghaus '"), "{stmts:?}");
        assert!(stmts[2].contains("'นาย อุทัย สุขผล '"), "{stmts:?}");
        assert!(stmts[1].starts_with("INSERT INTO [HT_CheckIn_Other_People]"), "{stmts:?}");
    }

    #[test]
    fn replace_all_empty_list_is_delete_only() {
        let stmts = build_replace_all_stmts("CH26-005228", &[]);
        assert_eq!(stmts.len(), 1, "{stmts:?}");
        assert!(stmts[0].starts_with("DELETE FROM HT_CheckIn_Other_People"), "{stmts:?}");
    }

    #[test]
    fn companion_targets_other_people_table() {
        let sql = build_insert_sql("CH26-005228", "A", "");
        assert!(
            sql.contains("[HT_CheckIn_Other_People]([Cin_no],[Cin_name],[Cin_contry])"),
            "{sql}"
        );
        // Empty country → Mr. prefix (non-Thai default).
        assert!(sql.contains("'Mr. A '"), "{sql}");
    }

    /// Delta ADD is VERBATIM: no `guest_prefix_for_country` prefix, no
    /// trailing space (both belong to the PRIMARY row's byte shape only —
    /// the 2026-07-01 echo incident stacked the prefix on every round-trip).
    #[test]
    fn add_sql_is_verbatim_no_prefix_no_trailing_space() {
        let sql = build_add_sql("CH26-005228", "Thomas Meininghaus", "DE");
        assert!(sql.contains("'Thomas Meininghaus'"), "verbatim name: {sql}");
        assert!(!sql.contains("'Mr. Thomas Meininghaus"), "no prefix: {sql}");
        assert!(!sql.contains("Meininghaus '"), "no trailing space: {sql}");
        assert!(sql.contains("'CH26-005228'"), "{sql}");
        assert!(sql.contains("'DE'"), "{sql}");
        // Plain `'…'` literal, never `N'…'` (TIS-620 byte-parity).
        assert!(!sql.contains("N'"), "{sql}");

        let thai = build_add_sql("CH26-005228", "อุทัย สุขผล", "TH");
        assert!(thai.contains("'อุทัย สุขผล'"), "no Thai prefix either: {thai}");
        assert!(!thai.contains("'นาย"), "no นาย prefix: {thai}");
    }

    /// Delta ADD captures the IDENTITY in the SAME batch so the worker can
    /// back-populate `guest_legacy_id` (the echo-absorption key).
    #[test]
    fn add_sql_carries_scope_identity_select() {
        let sql = build_add_sql("CH26-005228", "A", "");
        assert!(
            sql.contains("[HT_CheckIn_Other_People]([Cin_no],[Cin_name],[Cin_contry])"),
            "{sql}"
        );
        assert!(
            sql.ends_with("; SELECT CAST(SCOPE_IDENTITY() AS INT)"),
            "same-batch identity capture: {sql}"
        );
    }

    /// Delta DELETE is keyed on the known legacy id AND guarded by the quoted
    /// Cin_no — never a Cin_no-wide delete (the replace-all echo-loop shape).
    #[test]
    fn delete_sql_targets_id_with_cin_no_guard() {
        let sql = build_delete_sql("CH26-005228", 4217);
        assert_eq!(
            sql,
            "DELETE FROM HT_CheckIn_Other_People WHERE id = 4217 AND Cin_no = 'CH26-005228'"
        );
    }

    /// Single quotes in name / country / cin_no are doubled via `sql_quote`
    /// so the literal can't break out (companion name is free operator text).
    #[test]
    fn add_and_delete_sql_escape_apostrophes() {
        let sql = build_add_sql("CH26-005228", "O'Neil", "CÔTE D'IVOIRE");
        assert!(sql.contains("'O''Neil'"), "name apostrophe doubled: {sql}");
        assert!(
            sql.contains("'CÔTE D''IVOIRE'"),
            "country apostrophe doubled: {sql}"
        );

        let del = build_delete_sql("CH'26", 1);
        assert!(del.contains("'CH''26'"), "cin_no apostrophe doubled: {del}");
    }
}
