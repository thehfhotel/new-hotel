//! `CancelCheckIn` recipe — spike `findings.md` §3i.
//!
//! Clean cancel: 7 statements, no destructive Phase 1. Allocates one new id
//! (`HT_Rooms_Cancel.id`) under TABLOCKX.
//!
//! Reference SQL (verbatim from `cancel-checkin-20260424-114805/writes.txt`):
//!
//! ```text
//! 1. delete from HT_Room_Status where room_no='306' and room_CheckIn_No='CH26-005233'
//! 2. delete from HT_CheckIn_Ds where Cin_Room_No='306' and Cin_No='CH26-005233'
//! 3. update HT_Rooms set Room_Clean='yes',Room_Use='no' where room_no='306'
//! 4. INSERT INTO [HT_Rooms_Cancel](id, room_no, cin_no, cancel_date, cancel_by, cancel_note)
//!      VALUES(298, '306', 'CH26-005233', getdate(), 'Admin', 'ยกเลิกคุณนัท')
//! 5. UPDATE [HT_CheckIn_H] SET
//!      [Total_Price_Room]    = Total_Price_Room    - 890,
//!      [Total_Price_Net]     = [Total_Price_Net]   - 890,
//!      [Total_Price_Pay]     = [Total_Price_Pay]   - 0,
//!      [Total_Price_Balance] = ([Total_Price_Balance] - 890) + 0
//!    where [Cin_no]='CH26-005233'
//! 6. update HT_CheckIn_H set cin_status='ยกเลิก' where cin_no='CH26-005233'
//! 7. update HT_POWER_LOG SET
//!      ROOM_POWER_END    = GETDATE(),
//!      ROOM_POWER_END_BY = 'Admin',
//!      ROOM_POWER_NOTE2  = 'ปิดไฟ เนื่องจากยกเลิกห้องพัก'
//!    where room_no='306' and ROOM_POWER_END_BY=''
//! ```
//!
//! Spike §3i critical findings:
//! - **Subtraction, not zeroing** — `Total_Price_Room - 890`. Multi-room safe:
//!   if a check-in covers two rooms and only one is cancelled, the other
//!   remains. This recipe takes `room_price` as input so we can subtract
//!   the right amount.
//! - **Power-log note differs from check-out** — uses `'ปิดไฟ เนื่องจากยกเลิกห้องพัก'`
//!   (no Cin_no suffix), distinct from check-out's
//!   `'ปิดไฟ อัตโนมัติ จากเช็คเอ้าท์ No.{cin_no}'`.
//! - `Room_Clean='yes'` set after cancel = "this room needs cleaning".
//! - `HT_CheckIn_Other_People` is intentionally NOT deleted — accompanying
//!   guests stay attached for the audit trail.

use crate::domain::shared::Money;
use crate::writeback::allocate::{allocate_rooms_cancel_id, LegacyConn};
use crate::writeback::constants::{
    CIN_STATUS_CANCELLED, DEFAULT_OPERATOR, POWER_LOG_NOTE_CHECKIN_CANCELLED,
};
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{sql_quote, sql_quote_or_empty};

/// Inputs needed to build the cancel-check-in statements.
#[derive(Debug, Clone)]
pub struct CancelCheckInInputs<'a> {
    pub cin_no: &'a str,
    pub room_no: &'a str,
    pub rooms_cancel_id: i32,
    /// Amount in baht to subtract from `HT_CheckIn_H.Total_Price_*`. Per
    /// spike §3i this is the cancelled room's `Cin_Room_Price` — the recipe
    /// uses 0.0 if not yet known (resolved-job lookup hasn't supplied one).
    pub price_to_subtract: f64,
    /// Pay amount to subtract — usually 0 unless a deposit was paid against
    /// only the cancelled room (rare). Per spike capture this was 0.
    pub pay_to_subtract: f64,
    pub cancel_by: &'a str,
    pub cancel_note: Option<&'a str>,
}

/// Build the 7 statements that cancel a check-in. PURE — no I/O.
pub fn build_statements(inputs: &CancelCheckInInputs<'_>) -> Vec<String> {
    let cin_no_q = sql_quote(inputs.cin_no);
    let room_no_q = sql_quote(inputs.room_no);
    let by_q = sql_quote(inputs.cancel_by);
    let note_q = sql_quote_or_empty(inputs.cancel_note);
    let cancel_id = inputs.rooms_cancel_id;
    // Wave 6 LOW item 4: pre-format money to 2dp for consistency with the
    // HT_CheckIn_H VALUES that already use 2dp on create / checkout.
    let price = format!("{:.2}", inputs.price_to_subtract);
    let pay = format!("{:.2}", inputs.pay_to_subtract);
    let cancel_status_q = sql_quote(CIN_STATUS_CANCELLED);
    let power_note_q = sql_quote(POWER_LOG_NOTE_CHECKIN_CANCELLED);

    vec![
        // 1. Drop the room_status row
        format!(
            "delete from HT_Room_Status where room_no={room_no_q} and room_CheckIn_No={cin_no_q}"
        ),
        // 2. Drop the room from check-in detail
        format!(
            "delete from HT_CheckIn_Ds where Cin_Room_No={room_no_q} and Cin_No={cin_no_q}"
        ),
        // 3. Free the room — Room_Clean='yes' = needs cleaning
        format!(
            "update HT_Rooms set Room_Clean='yes',Room_Use='no' where room_no={room_no_q}"
        ),
        // 4. Audit log row — id allocated with TABLOCKX
        format!(
            "INSERT INTO [HT_Rooms_Cancel]([id],[room_no],[cin_no],[cancel_date],[cancel_by],\
             [cancel_note])VALUES({cancel_id},{room_no_q},{cin_no_q},getdate(),{by_q},{note_q})"
        ),
        // 5. SUBTRACT this room's price from totals (multi-room safe)
        format!(
            "UPDATE [HT_CheckIn_H] SET  [Total_Price_Room]=Total_Price_Room-{price},\
             [Total_Price_Net]=[Total_Price_Net]-{price},\
             [Total_Price_Pay]=[Total_Price_Pay]-{pay},\
             [Total_Price_Balance]=([Total_Price_Balance]-{price})+{pay} \
             where [Cin_no]={cin_no_q}"
        ),
        // 6. Mark the whole check-in cancelled
        format!(
            "update HT_CheckIn_H set cin_status={cancel_status_q} where cin_no={cin_no_q}"
        ),
        // 7. Lights off with cancel-specific note — Wave 5b item 3: restrict
        //    to the **most-recent** open row for this room. If a prior
        //    check-in left an open row (rare, but possible after a crashed
        //    session), the previous shape closed all of them in one shot,
        //    rewriting their `ROOM_POWER_END_BY` / `ROOM_POWER_NOTE2` and
        //    polluting the power-log audit trail. The `id =
        //    (SELECT MAX(id) … WHERE ROOM_POWER_END_BY='')` subquery
        //    targets only the row this cancel actually owns.
        format!(
            "update HT_POWER_LOG SET ROOM_POWER_END=GETDATE(),ROOM_POWER_END_BY={by_q},\
             ROOM_POWER_NOTE2={power_note_q} where room_no={room_no_q} and ROOM_POWER_END_BY='' \
             and id = (SELECT MAX(id) FROM HT_POWER_LOG WHERE room_no={room_no_q} and ROOM_POWER_END_BY='')"
        ),
    ]
}

/// Execute the cancel-check-in recipe.
///
/// `room_price` + `pay_to_subtract` flow in from the
/// [`WritebackIntent::CancelCheckIn`] payload (route enrichment populates them
/// from the canonical PG state per spike §3i). Both default to `Money::ZERO`
/// when the route lookup couldn't resolve a per-room price — the cancel
/// still proceeds, but `HT_CheckIn_H` totals stay unchanged on MSSQL.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    cin_no: &str,
    room_no: &str,
    reason: Option<&str>,
    room_price: Money,
    pay_to_subtract: Money,
) -> WritebackResult<LegacyIds> {
    // MED-1: reject NaN/Infinity before SQL formatting. Today these come from
    // `Money` (always finite), but `build_statements` interpolates them
    // directly into the `[Total_Price_*]` arithmetic — defense-in-depth
    // against a future code path that introduces a non-Money f64.
    let price_to_subtract_baht = (room_price.as_satang() as f64) / 100.0;
    let pay_to_subtract_baht = (pay_to_subtract.as_satang() as f64) / 100.0;
    super::helpers::validate_finite(&[
        ("price_to_subtract_baht", price_to_subtract_baht),
        ("pay_to_subtract_baht", pay_to_subtract_baht),
    ])?;

    let cancel_id = allocate_rooms_cancel_id(conn).await?;
    let inputs = CancelCheckInInputs {
        cin_no,
        room_no,
        rooms_cancel_id: cancel_id,
        price_to_subtract: price_to_subtract_baht,
        pay_to_subtract: pay_to_subtract_baht,
        cancel_by: DEFAULT_OPERATOR,
        cancel_note: reason,
    };
    let statements = build_statements(&inputs);
    super::execute_all(conn, &statements).await?;

    let mut ids = LegacyIds::new().with_cin_no(cin_no.to_string());
    ids.extra
        .insert("rooms_cancel_id".into(), serde_json::Value::from(cancel_id));
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Matches `cancel-checkin-20260424-114805/writes.txt` lines 1-7.
    #[test]
    fn build_statements_matches_spike_capture() {
        let inputs = CancelCheckInInputs {
            cin_no: "CH26-005233",
            room_no: "306",
            rooms_cancel_id: 298,
            price_to_subtract: 890.0,
            pay_to_subtract: 0.0,
            cancel_by: "Admin",
            cancel_note: Some("ยกเลิกคุณนัท"),
        };
        let statements = build_statements(&inputs);
        assert_eq!(statements.len(), 7);

        assert_eq!(
            statements[0],
            "delete from HT_Room_Status where room_no='306' and room_CheckIn_No='CH26-005233'"
        );
        assert_eq!(
            statements[1],
            "delete from HT_CheckIn_Ds where Cin_Room_No='306' and Cin_No='CH26-005233'"
        );
        assert_eq!(
            statements[2],
            "update HT_Rooms set Room_Clean='yes',Room_Use='no' where room_no='306'"
        );
        assert!(statements[3].contains("[HT_Rooms_Cancel]"));
        assert!(statements[3].contains("VALUES(298,'306','CH26-005233',getdate(),'Admin','ยกเลิกคุณนัท')"));
        // Wave 6 LOW item 4: 2dp money formatting (was raw `890`/`0`).
        assert!(statements[4].contains("Total_Price_Room-890.00"));
        assert!(statements[4].contains("[Total_Price_Net]-890.00"));
        assert!(statements[4].contains("([Total_Price_Balance]-890.00)+0.00"));
        assert_eq!(
            statements[5],
            "update HT_CheckIn_H set cin_status='ยกเลิก' where cin_no='CH26-005233'"
        );
        assert!(statements[6].contains("ROOM_POWER_NOTE2='ปิดไฟ เนื่องจากยกเลิกห้องพัก'"));
        assert!(statements[6].contains("ROOM_POWER_END_BY=''"));
    }

    #[test]
    fn price_to_subtract_propagates_to_totals_update() {
        // After fix #15 the recipe takes the room_price as input rather than
        // defaulting to 0. Verify that a real value lands in the SUBTRACT.
        let inputs = CancelCheckInInputs {
            cin_no: "CH26-005233",
            room_no: "306",
            rooms_cancel_id: 298,
            price_to_subtract: 1500.0,
            pay_to_subtract: 250.0,
            cancel_by: "Admin",
            cancel_note: None,
        };
        let s = build_statements(&inputs);
        let totals = s.iter().find(|s| s.contains("[Total_Price_Room]")).unwrap();
        // Wave 6 LOW item 4: 2dp money formatting.
        assert!(totals.contains("Total_Price_Room-1500.00"));
        assert!(totals.contains("[Total_Price_Net]-1500.00"));
        assert!(totals.contains("[Total_Price_Pay]-250.00"));
        assert!(totals.contains("([Total_Price_Balance]-1500.00)+250.00"));
    }

    #[test]
    fn validate_finite_rejects_nan_price_to_subtract() {
        // MED-1 guard: a non-finite f64 in `price_to_subtract` would otherwise
        // emit `Total_Price_Room-NaN` and fail the entire legacy transaction.
        // The execute() guard wraps the same call we verify here directly —
        // keeping this unit test pure (no MSSQL conn).
        let result = super::super::helpers::validate_finite(&[
            ("price_to_subtract_baht", f64::NAN),
            ("pay_to_subtract_baht", 0.0),
        ]);
        let err = result.expect_err("NaN must be rejected");
        let msg = err.to_string();
        assert!(msg.contains("price_to_subtract_baht"), "msg: {msg}");
        assert!(msg.contains("NaN"), "msg: {msg}");
    }

    #[test]
    fn validate_finite_rejects_infinity_in_pay_to_subtract() {
        // MED-1 guard: `pay_to_subtract` lands in `[Total_Price_Pay]-{pay}` and
        // `([Total_Price_Balance]-{price})+{pay}` — both would emit invalid
        // SQL if `pay` is non-finite.
        let result = super::super::helpers::validate_finite(&[
            ("price_to_subtract_baht", 890.0),
            ("pay_to_subtract_baht", f64::NEG_INFINITY),
        ]);
        let err = result.expect_err("Infinity must be rejected");
        assert!(err.to_string().contains("pay_to_subtract_baht"));
    }

    /// Wave 5b item 3: HT_POWER_LOG cancel must close only the row this
    /// cancel actually owns — the **most-recent** open row for the room.
    /// Prior shape (`WHERE room_no=… AND ROOM_POWER_END_BY=''`) would close
    /// every open row, rewriting `ROOM_POWER_NOTE2` on a prior crashed
    /// session's leftover row and corrupting the audit trail.
    #[test]
    fn ht_power_log_closes_only_most_recent_open_row() {
        let inputs = CancelCheckInInputs {
            cin_no: "CH26-005233",
            room_no: "306",
            rooms_cancel_id: 298,
            price_to_subtract: 890.0,
            pay_to_subtract: 0.0,
            cancel_by: "Admin",
            cancel_note: None,
        };
        let statements = build_statements(&inputs);
        let power = statements
            .iter()
            .find(|s| s.starts_with("update HT_POWER_LOG"))
            .expect("HT_POWER_LOG update must be emitted");
        assert!(
            power.contains(
                "id = (SELECT MAX(id) FROM HT_POWER_LOG WHERE room_no='306' and ROOM_POWER_END_BY='')"
            ),
            "must restrict to most-recent open row: {power}"
        );
        // Defense: original room_no + ROOM_POWER_END_BY filters are still in
        // the WHERE so the subquery scope and outer-update scope match.
        assert!(power.contains("where room_no='306' and ROOM_POWER_END_BY=''"));
    }

    #[test]
    fn cancel_note_defaults_to_empty_when_none() {
        let inputs = CancelCheckInInputs {
            cin_no: "CH26-005233",
            room_no: "306",
            rooms_cancel_id: 298,
            price_to_subtract: 0.0,
            pay_to_subtract: 0.0,
            cancel_by: "Admin",
            cancel_note: None,
        };
        let statements = build_statements(&inputs);
        // 4th statement is the HT_Rooms_Cancel INSERT — note column is last
        assert!(statements[3].ends_with("'Admin','')"));
    }
}
