//! `MoveRoomTiles` recipe — layout-edit mode (จัดผัง) board rearrange (#236).
//!
//! Mirrors a canonical `ht_rooms_new.room_x`/`room_y` UPDATE → legacy
//! `HT_Rooms.Room_X`/`Room_y`. The board is SHARED with iHOTEL FormRoomMain,
//! so every drop must land on both sides with identical pixel values
//! (identical bytes ⇒ the CT-watcher echo of our own legacy write converges
//! to a no-op in the room mapper's `COALESCE` UPDATE — see
//! `sync/mappers/room.rs`).
//!
//! ## Byte-shape contract
//!
//! One statement per move, exactly the iHOTEL FormRoomMain drag/drop capture
//! (`docs/legacy-app/COMPAT_CHEATSHEET.md` §"Update grid layout"):
//!
//! ```text
//! update HT_Rooms set Room_X=<x>,Room_y=<y> where Room_no='<room>'
//! ```
//!
//! Lowercase keywords, no spaces around `=`/`,` — matching the capture's
//! casing verbatim (same fidelity convention as `checkin_cancel` /
//! `booking_cancel`). `Room_X`/`Room_y` are `int NOT NULL`, so plain integer
//! formatting; the `Room_no` literal goes through `sql_quote` — plain `'…'`,
//! never `N'…'` (cheatsheet §1.8, varchar `Thai_CI_AS`).
//!
//! ## Move count semantics
//!
//! 1 move = place / move-to-empty-cell; 2 moves = swap (the two tiles
//! exchange their existing pixel pairs verbatim). Both UPDATEs of a swap run
//! inside the worker's single recipe transaction, so the legacy board never
//! persists half-swapped. An empty `moves` vec degrades to a no-op (empty
//! statement list) rather than an error — same defensive shape as
//! `update_room`'s empty payload.
//!
//! Idempotent absolute-SET UPDATE — NOT ledgered: a crash-after-commit retry
//! re-applies the same coordinates and converges (see `intent_facts`).

use crate::outbox::intent::RoomTileMove;
use crate::writeback::allocate::LegacyConn;
use crate::writeback::dispatcher::LegacyIds;
use crate::writeback::error::WritebackResult;
use crate::writeback::format::sql_quote;

/// Build one UPDATE per move. PURE — no I/O, deterministic on inputs
/// (T6 HIGH-1 purity convention). Statement order follows payload order so
/// a swap's two UPDATEs apply exactly as the route enqueued them.
pub fn build_statements(moves: &[RoomTileMove]) -> WritebackResult<Vec<String>> {
    Ok(moves
        .iter()
        .map(|m| {
            let room_no_q = sql_quote(&m.room_no);
            format!(
                "update HT_Rooms set Room_X={},Room_y={} where Room_no={room_no_q}",
                m.room_x, m.room_y
            )
        })
        .collect())
}

/// Execute the recipe. Returns an empty `LegacyIds` — the legacy rows
/// already exist and we only shift coordinate values, so no new identifier
/// is minted. The moved `Room_no`s are stashed in `LegacyIds.extra` so the
/// writeback worker's `mark_done` log line can correlate.
pub async fn execute(
    conn: &mut LegacyConn<'_>,
    moves: &[RoomTileMove],
) -> WritebackResult<LegacyIds> {
    let statements = build_statements(moves)?;
    super::execute_all(conn, &statements).await?;
    let mut ids = LegacyIds::new();
    ids.extra.insert(
        "room_nos".into(),
        serde_json::Value::from(
            moves
                .iter()
                .map(|m| m.room_no.clone())
                .collect::<Vec<String>>(),
        ),
    );
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tile(room_no: &str, x: i32, y: i32) -> RoomTileMove {
        RoomTileMove {
            room_no: room_no.to_string(),
            room_x: x,
            room_y: y,
        }
    }

    /// Byte-pin against the FormRoomMain drag/drop capture
    /// (`COMPAT_CHEATSHEET.md` §"Update grid layout"): lowercase keywords,
    /// no spaces around `=` / `,`, plain `'…'` room literal.
    #[test]
    fn build_statements_single_move_matches_ihotel_capture_bytes() {
        let statements =
            build_statements(&[tile("306", 120, 240)]).expect("single move must build");
        assert_eq!(statements.len(), 1, "exactly one UPDATE per move");
        assert_eq!(
            statements[0],
            "update HT_Rooms set Room_X=120,Room_y=240 where Room_no='306'"
        );
    }

    /// A swap is TWO statements in payload order — each byte-identical to
    /// the single-move shape (iHOTEL itself fires one UPDATE per dragged
    /// tile; our swap batches both into one recipe transaction).
    #[test]
    fn build_statements_swap_emits_two_updates_in_payload_order() {
        let statements = build_statements(&[tile("306", 120, 240), tile("A2-1", 10, 10)])
            .expect("swap must build");
        assert_eq!(statements.len(), 2);
        assert_eq!(
            statements[0],
            "update HT_Rooms set Room_X=120,Room_y=240 where Room_no='306'"
        );
        assert_eq!(
            statements[1],
            "update HT_Rooms set Room_X=10,Room_y=10 where Room_no='A2-1'"
        );
    }

    /// The `WHERE` clause's room number must be quote-escaped — `sql_quote`
    /// is the only injection barrier on this code path (defense-in-depth,
    /// same pin as `update_room`).
    #[test]
    fn build_statements_escapes_embedded_single_quote_in_room_no() {
        let statements = build_statements(&[tile("A'1", 50, 60)]).expect("must build");
        assert_eq!(
            statements[0],
            "update HT_Rooms set Room_X=50,Room_y=60 where Room_no='A''1'"
        );
    }

    /// Empty move list degrades to a no-op (zero statements) so
    /// `execute_all` skips the wire call entirely — defends against a
    /// future caller that enqueues speculatively.
    #[test]
    fn build_statements_empty_moves_is_no_op() {
        let statements = build_statements(&[]).expect("empty moves must build");
        assert!(
            statements.is_empty(),
            "empty moves must produce zero statements, got: {statements:?}"
        );
    }

    /// Building twice with identical inputs produces byte-identical output —
    /// same purity / determinism contract as the other recipes (T6 HIGH-1).
    #[test]
    fn build_statements_is_pure_for_identical_inputs() {
        let moves = [tile("306", 120, 240), tile("A2-1", 10, 10)];
        let first = build_statements(&moves).unwrap();
        let second = build_statements(&moves).unwrap();
        assert_eq!(first, second, "build_statements must be deterministic");
    }

    /// Negative coordinates are never produced by the route (it validates
    /// `>= 0`), but the recipe must still format them losslessly rather
    /// than corrupt the statement — the byte shape stays a plain integer.
    #[test]
    fn build_statements_formats_negative_coordinates_losslessly() {
        let statements = build_statements(&[tile("306", -5, 0)]).expect("must build");
        assert_eq!(
            statements[0],
            "update HT_Rooms set Room_X=-5,Room_y=0 where Room_no='306'"
        );
    }
}
