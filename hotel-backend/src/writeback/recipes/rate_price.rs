//! `UpsertRatePrice` recipe — admin edit of the `HT_Rooms_Price` pricing
//! matrix (Task #51).
//!
//! Mirrors a PG `ht_rate_tiers` UPSERT → legacy `HT_Rooms_Price` UPSERT keyed
//! by the composite natural key `(Room_Type, Room_CustType)`. Closes the gap
//! between the canonical rate-tier write path (`PUT /api/rate-tiers/:id`) and
//! the legacy MSSQL pricing matrix the .NET booking form reads every time it
//! draws "ราคา / คืน".
//!
//! ## Byte-shape contract
//!
//! `HT_Rooms_Price` (`docs/legacy-app/SCHEMA.sql`):
//! `id int IDENTITY, Room_Type varchar(50), Room_CustType varchar(50),
//!  Room_Price float, Room_Price_H float, Room_Price_M float`.
//!
//! Exactly one batch statement is built — an idempotent merge keyed on the
//! composite `(Room_Type, Room_CustType)`:
//!
//! ```text
//! IF EXISTS (SELECT 1 FROM HT_Rooms_Price
//!             WHERE Room_Type = '<rt>' AND Room_CustType = '<ct>')
//!   UPDATE HT_Rooms_Price
//!      SET Room_Price = <p>, Room_Price_H = <h>, Room_Price_M = <m>
//!    WHERE Room_Type = '<rt>' AND Room_CustType = '<ct>'
//! ELSE
//!   INSERT INTO HT_Rooms_Price (Room_Type, Room_CustType, Room_Price, Room_Price_H, Room_Price_M)
//!   VALUES ('<rt>', '<ct>', <p>, <h>, <m>)
//! ```
//!
//! ### Why key on the composite, not the legacy `id`
//!
//! iHOTEL edits this table with a **delete-then-reinsert** pattern (cheatsheet
//! §`HT_Rooms_Price`), so the IDENTITY `id` is not stable across legacy edits.
//! Keying our UPSERT on `(Room_Type, Room_CustType)` — the same natural key the
//! sync mapper UPSERTs `ht_rate_tiers` on — survives that churn. The legacy
//! IDENTITY is server-allocated on the INSERT branch, so there is no app-side
//! id race (unlike the `HT_Rooms` / `HT_Products` MAX+1 hazard); the 15-minute
//! mirror poll (`sync/mappers/rate_tiers.rs`) re-pins `rate_tier_legacy_id`.
//!
//! ### Literals
//!
//! Plain `'…'` literals, never `N'…'` — every `HT_Rooms_Price` text column is
//! `varchar` with Thai collation (cheatsheet §1.8 forbids `N'…'` against
//! varchar). `f64_sql` is used for the float columns so NaN / Inf surface as a
//! recipe error instead of corrupting the legacy row. `NULL` is emitted
//! verbatim for absent hourly / monthly prices (the columns are nullable).

use crate::outbox::intent::RatePricePayload;
use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::{WritebackError, WritebackResult};
use crate::writeback::format::{f64_sql, sql_quote};

/// Build the single idempotent merge statement. PURE — no I/O, deterministic
/// on inputs. Returns a recipe error when the composite key is blank (a blank
/// `Room_Type` / `Room_CustType` would silently match/insert the wrong row) or
/// when any price is non-finite.
pub fn build_statements(payload: &RatePricePayload) -> WritebackResult<Vec<String>> {
    if payload.room_type.trim().is_empty() {
        return Err(WritebackError::Recipe(
            "UpsertRatePrice requires a non-empty room_type".into(),
        ));
    }
    if payload.cust_type.trim().is_empty() {
        return Err(WritebackError::Recipe(
            "UpsertRatePrice requires a non-empty cust_type".into(),
        ));
    }

    let rt = sql_quote(&payload.room_type);
    let ct = sql_quote(&payload.cust_type);
    let price = f64_sql(payload.price)?;
    let price_h = opt_f64_sql(payload.price_hourly)?;
    let price_m = opt_f64_sql(payload.price_monthly)?;

    let where_key = format!("Room_Type = {rt} AND Room_CustType = {ct}");

    Ok(vec![format!(
        "IF EXISTS (SELECT 1 FROM HT_Rooms_Price WHERE {where_key}) \
         UPDATE HT_Rooms_Price SET Room_Price = {price}, \
         Room_Price_H = {price_h}, Room_Price_M = {price_m} WHERE {where_key} \
         ELSE INSERT INTO HT_Rooms_Price \
         (Room_Type, Room_CustType, Room_Price, Room_Price_H, Room_Price_M) \
         VALUES ({rt}, {ct}, {price}, {price_h}, {price_m})"
    )])
}

/// `f64_sql` for an `Option<f64>` — `None` ⇒ the literal `NULL`.
fn opt_f64_sql(value: Option<f64>) -> WritebackResult<String> {
    match value {
        Some(v) => f64_sql(v),
        None => Ok("NULL".to_string()),
    }
}

/// Execute the recipe. Returns an empty-but-annotated `LegacyIds` — the legacy
/// IDENTITY (on the INSERT branch) is re-pinned by the mirror poll, not
/// back-populated here, so we only stash the composite key for the worker's
/// `mark_done` log line.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    payload: &RatePricePayload,
) -> WritebackResult<LegacyIds> {
    let statements = build_statements(payload)?;
    super::execute_all(conn, &statements).await?;
    let mut ids = LegacyIds::new();
    ids.extra.insert(
        "room_type".into(),
        serde_json::Value::from(payload.room_type.clone()),
    );
    ids.extra.insert(
        "cust_type".into(),
        serde_json::Value::from(payload.cust_type.clone()),
    );
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn payload(price: f64) -> RatePricePayload {
        RatePricePayload {
            site_id: "hfhotel".into(),
            room_type: "เตียงคู่".into(),
            cust_type: "ราคาปกติ".into(),
            price,
            price_hourly: Some(200.0),
            price_monthly: Some(15000.0),
        }
    }

    #[test]
    fn build_statements_full_payload_emits_if_exists_merge() {
        let stmts = build_statements(&payload(800.0)).expect("must build");
        assert_eq!(stmts.len(), 1, "exactly one merge statement");
        assert_eq!(
            stmts[0],
            "IF EXISTS (SELECT 1 FROM HT_Rooms_Price WHERE Room_Type = 'เตียงคู่' \
             AND Room_CustType = 'ราคาปกติ') UPDATE HT_Rooms_Price SET \
             Room_Price = 800, Room_Price_H = 200, Room_Price_M = 15000 \
             WHERE Room_Type = 'เตียงคู่' AND Room_CustType = 'ราคาปกติ' \
             ELSE INSERT INTO HT_Rooms_Price (Room_Type, Room_CustType, \
             Room_Price, Room_Price_H, Room_Price_M) VALUES ('เตียงคู่', \
             'ราคาปกติ', 800, 200, 15000)"
        );
    }

    #[test]
    fn build_statements_null_optional_prices() {
        let mut p = payload(800.0);
        p.price_hourly = None;
        p.price_monthly = None;
        let stmts = build_statements(&p).expect("must build");
        assert!(
            stmts[0].contains("Room_Price_H = NULL, Room_Price_M = NULL"),
            "absent hourly/monthly must emit NULL, got: {}",
            stmts[0]
        );
        assert!(
            stmts[0].contains("VALUES ('เตียงคู่', 'ราคาปกติ', 800, NULL, NULL)"),
            "INSERT branch must carry NULLs, got: {}",
            stmts[0]
        );
    }

    #[test]
    fn build_statements_escapes_embedded_quote() {
        let mut p = payload(800.0);
        p.cust_type = "O'Brien".into();
        let stmts = build_statements(&p).expect("must build");
        assert!(
            stmts[0].contains("Room_CustType = 'O''Brien'"),
            "embedded single-quote must be doubled, got: {}",
            stmts[0]
        );
    }

    #[test]
    fn build_statements_rejects_blank_room_type() {
        let mut p = payload(800.0);
        p.room_type = "  ".into();
        let err = build_statements(&p).expect_err("blank room_type must error");
        assert!(matches!(err, WritebackError::Recipe(_)));
    }

    #[test]
    fn build_statements_rejects_non_finite_price() {
        let err = build_statements(&payload(f64::NAN)).expect_err("NaN price must error");
        assert!(err.to_string().contains("non-finite"));
    }

    #[test]
    fn build_statements_is_pure_for_identical_inputs() {
        let first = build_statements(&payload(800.0)).unwrap();
        let second = build_statements(&payload(800.0)).unwrap();
        assert_eq!(first, second, "build_statements must be deterministic");
    }
}
