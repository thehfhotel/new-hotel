//! `MirrorGuestImage` recipe — Phase 2 (check-in registration).
//!
//! Mirrors a captured guest document (Thai ID card / passport / face photo)
//! into the legacy `Tb_Save_Image` table so iHOTEL's registration screen shows
//! the same photo. The row is **provisional** — `cust_no` / `cin_no` are empty
//! and only `tmp_no` links it; the check-in writeback's
//! `UPDATE Tb_Save_Image SET cin_no=…, cust_no=… WHERE tmp_no=<tmp_no>`
//! (walk-in recipe stmt 1b) stamps the identifiers later. This is the same
//! two-step the legacy .NET app uses (photo saved before the check-in exists).
//!
//! ## The one bound-parameter recipe
//!
//! Every other recipe emits pure literal SQL text (byte-parity with the .NET
//! app's captured statements). This recipe is the sole exception: the image
//! blob is bound as a **varbinary parameter** (`@P1`) via `tiberius::Query`
//! rather than interpolated as a `0x…` hex literal, because (a) the contract
//! specifies a bytes param and (b) a multi-hundred-KB hex literal in the
//! statement text is wasteful. The non-binary columns (`ttype`, `tmp_no`) stay
//! plain `'…'` literals via [`sql_quote`] — NEVER `N'…'`, which would send the
//! Thai `ttype` as NVARCHAR and corrupt the legacy TIS-620 column (invariant #3).
//!
//! **Shipped DARK** behind `GUEST_DOCUMENT_STORAGE_ENABLED` — the emitter (the
//! `POST /api/guest-documents` route) is gated; this recipe is always compiled.

use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::{WritebackError, WritebackResult};
use crate::writeback::format::sql_quote;

/// Map a canonical `doc_type` to the legacy `Tb_Save_Image.ttype` Thai literal.
///
/// | `doc_type`      | `ttype`            |
/// |-----------------|--------------------|
/// | `thai_id_card`  | `บัตรประชาชน`      |
/// | `face_photo`    | `รูปลูกค้า`        |
/// | `passport`      | `หนังสือเดินทาง`  |
pub fn ttype_for_doc_type(doc_type: &str) -> WritebackResult<&'static str> {
    match doc_type {
        "thai_id_card" => Ok("บัตรประชาชน"),
        "face_photo" => Ok("รูปลูกค้า"),
        "passport" => Ok("หนังสือเดินทาง"),
        other => Err(WritebackError::Recipe(format!(
            "MirrorGuestImage: unknown doc_type {other:?} (expected \
             thai_id_card | face_photo | passport)"
        ))),
    }
}

/// Build the provisional `Tb_Save_Image` INSERT with a `@P1` placeholder for the
/// varbinary `pic`. PURE — no I/O. `ttype` / `tmp_no` are plain `'…'` literals
/// (never `N'…'`). The `pic` value is supplied by `execute` as a bound param.
pub fn build_insert_sql(ttype: &str, tmp_no: &str) -> String {
    format!(
        "INSERT INTO Tb_Save_Image (pic, cust_no, cin_no, ttype, tmp_no, pic_date) \
         VALUES (@P1, '', '', {ttype_q}, {tmp_no_q}, GETDATE())",
        ttype_q = sql_quote(ttype),
        tmp_no_q = sql_quote(tmp_no),
    )
}

/// Execute the provisional-photo INSERT, binding `image` as a varbinary
/// parameter. Returns an empty [`LegacyIds`] — `Tb_Save_Image.id` is IDENTITY;
/// the ledger (keyed on the job's idempotency_key) guards the duplicate-on-retry
/// class, so no id capture / back-population is needed for the provisional row.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    image: &[u8],
    doc_type: &str,
    tmp_no: &str,
) -> WritebackResult<LegacyIds> {
    if image.is_empty() {
        return Err(WritebackError::Recipe(
            "MirrorGuestImage: refusing to write an empty pic to Tb_Save_Image".into(),
        ));
    }
    let ttype = ttype_for_doc_type(doc_type)?;
    let sql = build_insert_sql(ttype, tmp_no);

    // The one place a recipe uses a bound parameter (varbinary pic). tiberius
    // maps `&[u8]` to `ColumnData::Binary` (varbinary), so the blob round-trips
    // byte-exact. `conn` derefs to the tiberius `Client`.
    let mut query = tiberius::Query::new(sql);
    query.bind(image);
    query
        .execute(&mut **conn)
        .await
        .map_err(WritebackError::Tiberius)?;

    Ok(LegacyIds::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ttype_maps_the_three_doc_types() {
        assert_eq!(ttype_for_doc_type("thai_id_card").unwrap(), "บัตรประชาชน");
        assert_eq!(ttype_for_doc_type("face_photo").unwrap(), "รูปลูกค้า");
        assert_eq!(ttype_for_doc_type("passport").unwrap(), "หนังสือเดินทาง");
    }

    #[test]
    fn ttype_rejects_unknown_doc_type() {
        assert!(ttype_for_doc_type("driver_license").is_err());
    }

    /// The INSERT keeps `pic` as the `@P1` placeholder, writes empty
    /// `cust_no`/`cin_no` (provisional), and uses plain `'…'` literals — never
    /// `N'…'` — for the Thai `ttype` (TIS-620 safety, invariant #3).
    #[test]
    fn insert_sql_shape_is_provisional_and_plain_quoted() {
        let sql = build_insert_sql("บัตรประชาชน", "abc-123");
        assert!(
            sql.contains("(@P1, '', '', 'บัตรประชาชน', 'abc-123', GETDATE())"),
            "{sql}"
        );
        assert!(
            !sql.contains("N'"),
            "must never emit N'…' for the Thai ttype: {sql}"
        );
    }

    /// An embedded quote in `tmp_no` is doubled (defense-in-depth — tmp_no is
    /// app-generated, but the quoter is the shared one used everywhere).
    #[test]
    fn tmp_no_quote_is_escaped() {
        let sql = build_insert_sql("รูปลูกค้า", "a'b");
        assert!(sql.contains("'a''b'"), "{sql}");
    }
}
