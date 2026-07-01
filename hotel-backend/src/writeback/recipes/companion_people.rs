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

use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::sql_quote;

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
pub async fn execute_replace_all(
    conn: &mut LegacyConn<'_>,
    cin_no: &str,
    companions: &[crate::outbox::intent::CompanionEntry],
) -> WritebackResult<LegacyIds> {
    let stmts = build_replace_all_stmts(cin_no, companions);
    super::execute_all(conn, &stmts).await?;
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
}
