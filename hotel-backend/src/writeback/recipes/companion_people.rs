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
