//! `UpdateCustomer` recipe — standalone customer-edit writeback
//! (coexistence audit 2026-06-11 P2).
//!
//! One statement: the same 31-field `UPDATE [HT_Customers] SET ... where
//! Cust_no=...` the legacy .NET app fires on every customer re-save
//! (spike §3c "re-save customer" — built by
//! [`super::helpers::build_customer_resave_update`], shared byte-for-byte
//! with the `booking_modify` recipe's embedded re-save).
//!
//! The payload is hydrated by `service::customer::update` from the
//! canonical `ht_customers` row AFTER the PG UPDATE: the operator-edited
//! fields come from the request, every other column comes from the
//! CT-mirrored `cust_add_*` / `cust_work_*` / `cust_name2` / … columns so
//! the re-save never blanks values a receptionist entered in iHOTEL.
//!
//! ## Customer DELETEs are deliberately NOT written back
//!
//! iHOTEL's delete (FrmManageCustomersNew, cheatsheet §3.24) is a
//! destructive hard-DELETE of the `HT_Customers` row + `Tb_Save_Image`
//! photos, followed by a 6-statement cascade that rewrites every FK-like
//! reference (`HT_CheckIn_H`, `HT_CheckIn_Pay`, `HT_Book_H`,
//! `HT_Bill_Debt_H`, `HT_Invoice_H`, `HT_Receipt_H`) to the `'C0000'`
//! sentinel. Our canonical delete is a reversible soft-delete
//! (`cust_active=false`, `routes::new_customers::delete_customer`) —
//! mirroring it as the legacy hard-DELETE would destroy data that a
//! single un-delete on our side could never restore, and the `'C0000'`
//! cascade is irreversible by construction. A soft-deleted customer
//! therefore stays visible in iHOTEL; if ops ever needs parity here it
//! must be a deliberate product decision, not a writeback default.

use crate::outbox::intent::CustomerResave;
use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;

/// Build the single re-save statement. PURE — no I/O.
pub fn build_statements(resave: &CustomerResave) -> Vec<String> {
    vec![super::helpers::build_customer_resave_update(resave)]
}

/// Execute the customer-edit recipe.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    resave: &CustomerResave,
) -> WritebackResult<LegacyIds> {
    let statements = build_statements(resave);
    super::execute_all(conn, &statements).await?;
    // No new legacy IDs are minted — the row already exists. Echo the
    // targeted Cust_no so `writeback_jobs.legacy_ids` records which legacy
    // row this job touched (operator triage parity with other recipes).
    Ok(LegacyIds::new().with_cust_no(resave.legacy_cust_no.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Byte-pin the full statement for a fully-hydrated payload. The
    /// 31-field order, the double space after `SET`, the lowercase
    /// `[Cust_Type_main]`, and the lowercase `where` all mirror the
    /// `/tmp/legacy-events-full.log` capture for `C21624` (line 3988).
    #[test]
    fn build_statements_matches_legacy_resave_capture_shape() {
        let resave = CustomerResave {
            legacy_cust_no: "C21624".into(),
            cust_name: "Alberto Calvo Alvarez".into(),
            cust_name2: "".into(),
            cust_type: "ราคาปกติ".into(),
            cust_type_main: "บุคคลธรรมดา".into(),
            cust_email: "a@example.com".into(),
            cust_add_no: "99/1".into(),
            cust_add_tel: "0900000088".into(),
            cust_perfix: "925".into(),
            cust_sex: "ชาย".into(),
            cust_idcard: "1234567890123".into(),
            ..CustomerResave::default()
        };
        let statements = build_statements(&resave);
        assert_eq!(statements.len(), 1, "exactly one re-save UPDATE");
        // `\` line-continuations strip the newline + leading whitespace, so
        // the expected literal below is one contiguous statement.
        assert_eq!(
            statements[0],
            "UPDATE [HT_Customers] SET  [Cust_name]='Alberto Calvo Alvarez',\
             [Cust_name2]='',[Cust_Type]='ราคาปกติ',\
             [Cust_Type_main]='บุคคลธรรมดา',[Cust_Email]='a@example.com',\
             [Cust_Add_no]='99/1',[Cust_Add_moo]='',[Cust_Add_soi]='',\
             [Cust_Add_road]='',[Cust_Add_tambon]='',[Cust_Add_ampore]='',\
             [Cust_Add_province]='',[Cust_Add_code]='',\
             [Cust_Add_tel]='0900000088',[Cust_Add_fax]='',[Cust_Work_Name]='',\
             [Cust_Work_no]='',[Cust_Work_moo]='',[Cust_Work_soi]='',\
             [Cust_Work_road]='',[Cust_Work_tambon]='',[Cust_Work_ampore]='',\
             [Cust_Work_province]='',[Cust_Work_code]='',[Cust_Work_tel]='',\
             [Cust_Work_fax]='',[Cust_Work_tax]='',[Cust_perfix]='925',\
             [Cust_sex]='ชาย',[Cust_IDcard]='1234567890123',[Cust_Contry]='' \
             where Cust_no='C21624'",
        );
    }

    /// Plain `'…'` literals only — never `N'…'` (cheatsheet §1.8: the
    /// legacy varchar columns are TIS-620; an `N` prefix triggers an
    /// nvarchar conversion that corrupts Thai text outside TIS-620).
    #[test]
    fn build_statements_never_emits_nvarchar_literals() {
        let resave = CustomerResave {
            legacy_cust_no: "C21624".into(),
            cust_name: "สมชาย ใจดี".into(),
            cust_type_main: "บุคคลธรรมดา".into(),
            ..CustomerResave::default()
        };
        let stmt = &build_statements(&resave)[0];
        assert!(
            !stmt.contains("N'"),
            "must not emit N'…' literals (cheatsheet §1.8); got:\n{stmt}"
        );
        assert!(stmt.contains("[Cust_name]='สมชาย ใจดี'"));
    }

    /// Apostrophes in operator-entered values must be doubled by
    /// `sql_quote` — including in the `WHERE` key.
    #[test]
    fn build_statements_quotes_values_safely() {
        let resave = CustomerResave {
            legacy_cust_no: "C21'624".into(),
            cust_name: "O'Brien".into(),
            ..CustomerResave::default()
        };
        let stmt = &build_statements(&resave)[0];
        assert!(stmt.contains("[Cust_name]='O''Brien'"));
        assert!(stmt.ends_with("where Cust_no='C21''624'"));
    }

    /// The shared helper and this recipe must stay byte-identical with the
    /// `booking_modify` embedded re-save (the extraction's whole point).
    #[test]
    fn recipe_uses_the_shared_resave_builder_verbatim() {
        let resave = CustomerResave {
            legacy_cust_no: "C21624".into(),
            cust_name: "Alberto Calvo Alvarez".into(),
            ..CustomerResave::default()
        };
        assert_eq!(
            build_statements(&resave)[0],
            crate::writeback::recipes::helpers::build_customer_resave_update(&resave),
        );
    }
}
