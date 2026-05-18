//! `UpdateRoom` recipe — admin / staff edit of `HT_Rooms` master data.
//!
//! Mirrors a PG `ht_rooms_new` UPDATE → legacy `HT_Rooms` UPDATE keyed
//! by `Room_no`. Closes the gap between the admin `PUT /api/new/rooms/:id`
//! route and the legacy MSSQL mirror — prior to this recipe every admin
//! price / type / notes edit silently diverged.
//!
//! ## Byte-shape contract
//!
//! Exactly one statement is built. Columns whose payload field is `None`
//! are omitted from the `SET` list — preserves any iHOTEL-side value the
//! operator did not touch (narrow update). The `WHERE` clause always
//! targets `Room_no = N'...'`.
//!
//! ```text
//! UPDATE HT_Rooms SET
//!     Room_Type    = N'<ห้องแฟมิลี่ 2>',
//!     Room_PriceA  = 890,
//!     Room_PriceB  = 1090,
//!     Room_PriceC  = 0,
//!     Room_Details = N'<notes>'
//!   WHERE Room_no = N'<A2-1>'
//! ```
//!
//! ## Skipped columns (NOT propagated)
//!
//! - `Room_Use` — legacy occupancy flag (`'yes'`/`'no'`), owned by the
//!   walk-in / check-in / cancel / check-out recipes. An admin edit
//!   must NEVER touch occupancy state.
//! - `Room_Clean` / `Room_Manternace` — owned by `mark_clean` + the
//!   maintenance request flow.
//! - PG-only fields (`room_features`, `room_floor`, `room_view`) — no
//!   `HT_Rooms` column exists for them.
//!
//! ## Empty payload (no-op)
//!
//! If every optional field is `None` the recipe returns an empty
//! `Vec<String>` so `execute_all` skips the wire call entirely. The
//! route never enqueues an empty intent today (it always has at least
//! `room_no`, and any meaningful edit touches one of the mirrored
//! columns) but the empty-vec path defends against future callers that
//! enqueue speculatively.

use crate::outbox::intent::UpdateRoomPayload;
use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::{f64_sql, sql_quote};

/// Build the single UPDATE statement. PURE — no I/O, deterministic on
/// inputs. Returns an empty `Vec` when every optional field is `None`
/// (the recipe degrades to a no-op rather than emitting an invalid
/// `UPDATE … SET WHERE …`).
///
/// `payload.room_no` is the `HT_Rooms.Room_no` business key the WHERE
/// clause targets. `f64_sql` is used for money columns so NaN / Inf
/// surface as a recipe error instead of corrupting the legacy row.
pub fn build_statements(payload: &UpdateRoomPayload) -> WritebackResult<Vec<String>> {
    let mut set_fragments: Vec<String> = Vec::new();

    if let Some(type_name) = payload.room_type_name.as_deref() {
        let quoted = sql_quote(type_name);
        // `N'...'` prefix preserves the Thai literal verbatim — same
        // convention `booking_create` / `mark_clean` use for Thai-text
        // legacy columns.
        set_fragments.push(format!("Room_Type = N{quoted}"));
    }
    if let Some(price) = payload.price_weekday {
        let value = f64_sql(price)?;
        set_fragments.push(format!("Room_PriceA = {value}"));
    }
    if let Some(price) = payload.price_weekend {
        let value = f64_sql(price)?;
        set_fragments.push(format!("Room_PriceB = {value}"));
    }
    if let Some(price) = payload.price_special {
        let value = f64_sql(price)?;
        set_fragments.push(format!("Room_PriceC = {value}"));
    }
    if let Some(notes) = payload.notes.as_deref() {
        let quoted = sql_quote(notes);
        set_fragments.push(format!("Room_Details = N{quoted}"));
    }

    if set_fragments.is_empty() {
        return Ok(Vec::new());
    }

    let where_room_no = sql_quote(&payload.room_no);
    let set_clause = set_fragments.join(", ");
    Ok(vec![format!(
        "UPDATE HT_Rooms SET {set_clause} WHERE Room_no = N{where_room_no}"
    )])
}

/// Execute the recipe. Returns an empty `LegacyIds` — the legacy row
/// already exists and we only shift its column values, so no new
/// identifier is minted. The `room_no` is stashed in `LegacyIds.extra`
/// so the writeback worker's `mark_done` log line can correlate.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    payload: &UpdateRoomPayload,
) -> WritebackResult<LegacyIds> {
    let statements = build_statements(payload)?;
    super::execute_all(conn, &statements).await?;
    let mut ids = LegacyIds::new();
    ids.extra
        .insert("room_no".into(), serde_json::Value::from(payload.room_no.clone()));
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_payload(room_no: &str) -> UpdateRoomPayload {
        UpdateRoomPayload {
            room_no: room_no.to_string(),
            room_type_name: None,
            price_weekday: None,
            price_weekend: None,
            price_special: None,
            notes: None,
        }
    }

    /// Full payload — every mirrored column is present, so every fragment
    /// appears in the SET list in the canonical order documented at the
    /// top of the module.
    #[test]
    fn build_statements_full_payload_emits_every_column() {
        let payload = UpdateRoomPayload {
            room_no: "A2-1".into(),
            room_type_name: Some("ห้องแฟมิลี่ 2".into()),
            price_weekday: Some(890.0),
            price_weekend: Some(1090.0),
            price_special: Some(0.0),
            notes: Some("ห้องริมน้ำ".into()),
        };
        let statements = build_statements(&payload).expect("full payload must build");
        assert_eq!(statements.len(), 1, "exactly one UPDATE statement");
        assert_eq!(
            statements[0],
            "UPDATE HT_Rooms SET Room_Type = N'ห้องแฟมิลี่ 2', \
             Room_PriceA = 890, Room_PriceB = 1090, Room_PriceC = 0, \
             Room_Details = N'ห้องริมน้ำ' \
             WHERE Room_no = N'A2-1'"
        );
    }

    /// Closes the prod bug captured in the prompt: room A2-1 had
    /// `price_weekday = 890.00` in canonical PG but `Room_PriceA = 0.00`
    /// in legacy MSSQL because no writeback fired. A price-only edit
    /// must still emit a valid UPDATE that targets `Room_PriceA`.
    #[test]
    fn build_statements_partial_price_only_targets_room_pricea() {
        let mut payload = empty_payload("A2-1");
        payload.price_weekday = Some(890.0);
        let statements = build_statements(&payload).expect("partial payload must build");
        assert_eq!(statements.len(), 1);
        assert_eq!(
            statements[0],
            "UPDATE HT_Rooms SET Room_PriceA = 890 WHERE Room_no = N'A2-1'"
        );
    }

    /// Every optional field is `None` — recipe degrades to a no-op so
    /// `execute_all` never issues an invalid `UPDATE … SET WHERE …`.
    #[test]
    fn build_statements_empty_payload_is_no_op() {
        let payload = empty_payload("A2-1");
        let statements = build_statements(&payload).expect("empty payload must build");
        assert!(
            statements.is_empty(),
            "empty payload must produce zero statements, got: {statements:?}"
        );
    }

    /// Thai literals with embedded single quotes (synthetic edge case
    /// for the notes column) must double the quote via `sql_quote` —
    /// otherwise the trailing `'` terminates the legacy `varchar` and
    /// the UPDATE fails / corrupts the row.
    #[test]
    fn build_statements_escapes_embedded_single_quote_in_notes() {
        let mut payload = empty_payload("A2-1");
        payload.notes = Some("O'Brien suite".into());
        let statements = build_statements(&payload).expect("notes-only payload must build");
        assert!(
            statements[0].contains("Room_Details = N'O''Brien suite'"),
            "embedded single-quote must be doubled, got: {}",
            statements[0]
        );
    }

    /// The `WHERE` clause's room number must also be quote-escaped — a
    /// `room_no` containing an apostrophe is unlikely in practice but the
    /// recipe must be safe regardless (defense-in-depth — `sql_quote` is
    /// the only injection barrier on this code path).
    #[test]
    fn build_statements_escapes_embedded_single_quote_in_room_no() {
        let mut payload = empty_payload("A'1");
        payload.price_weekday = Some(800.0);
        let statements = build_statements(&payload).expect("partial payload must build");
        assert!(
            statements[0].contains("WHERE Room_no = N'A''1'"),
            "embedded single-quote in room_no must be doubled, got: {}",
            statements[0]
        );
    }

    /// Building twice with identical inputs produces byte-identical
    /// output — same purity / determinism contract as the other recipes
    /// (T6 HIGH-1 / coexistence audit).
    #[test]
    fn build_statements_is_pure_for_identical_inputs() {
        let payload = UpdateRoomPayload {
            room_no: "A2-1".into(),
            room_type_name: Some("ห้องแฟมิลี่ 2".into()),
            price_weekday: Some(890.0),
            price_weekend: None,
            price_special: None,
            notes: None,
        };
        let first = build_statements(&payload).unwrap();
        let second = build_statements(&payload).unwrap();
        assert_eq!(first, second, "build_statements must be deterministic");
    }

    /// `room_type_name` set, every other field `None` — recipe emits a
    /// single `Room_Type = N'...'` fragment, no extra commas, no stray
    /// `WHERE` artifacts.
    #[test]
    fn build_statements_room_type_only_emits_single_fragment() {
        let mut payload = empty_payload("306");
        payload.room_type_name = Some("ห้องสแตนดาร์ด".into());
        let statements = build_statements(&payload).expect("type-only payload must build");
        assert_eq!(
            statements[0],
            "UPDATE HT_Rooms SET Room_Type = N'ห้องสแตนดาร์ด' \
             WHERE Room_no = N'306'"
        );
    }

    /// NaN / Inf prices must surface as a recipe error (via `f64_sql`)
    /// rather than emit literal `NaN` SQL that would fail the whole
    /// transaction. Mirrors the convention in `format::f64_sql`.
    #[test]
    fn build_statements_rejects_non_finite_price() {
        let mut payload = empty_payload("A2-1");
        payload.price_weekday = Some(f64::NAN);
        let err = build_statements(&payload).expect_err("NaN price must error");
        let msg = err.to_string();
        assert!(
            msg.contains("non-finite"),
            "expected non-finite error, got: {msg}"
        );
    }
}
