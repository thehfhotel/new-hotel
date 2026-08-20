//! Multi-room walk-in fixture — PROCESS.md P3 promotion (Track H,
//! audit-2026-05-13) + **Track B4 closure (2026-05-14)**.
//!
//! ## Why this exists
//!
//! `docs/legacy-spike/raw/walkin3-20260424-100000/` and
//! `docs/legacy-spike/raw/booking-checkin-20260424-101838/` captured the
//! end-to-end walk-in and booking-linked check-in flows on 2026-04-24.
//! Per PROCESS.md P3 ("Promote spike captures to fixtures"), each capture
//! MUST be promoted to a regression fixture on the day it was archived.
//! That step was skipped for three weeks — root cause of the 2026-05-12
//! multi-room cardinality post-mortem (audit `audit-2026-05-13.md` Theme
//! 1, T1 CRIT-1).
//!
//! ## Track B4 close-out
//!
//! Track B4 (writeback per-room apportionment) lands the multi-room
//! emission path:
//!   1. `WalkInInputs.room_lines` carries the canonical
//!      `ht_checkin_rooms` slice. Empty ⇒ legacy single-room shape;
//!      non-empty ⇒ N rows.
//!   2. `build_statements()` iterates the slice and emits one
//!      `HT_CheckIn_Ds` + `HT_POWER_LOG` + N×`HT_Room_Status` per room.
//!   3. The header (`HT_CheckIn_H`) stays single — `Cin_Room_ALL`
//!      concatenates every room number with the legacy trailing-space
//!      pattern (`'508 509 '`).
//!
//! The walk-in 2-room test below is the regression guard for #2 + #3.
//! The 3-room → 2-room "edit-down" assertion in `room_lines_drop_rooms`
//! pins the contract that dropping a junction row reduces the emitted
//! `HT_CheckIn_Ds` count from 3 to 2 — the orchestrator (the writeback
//! worker's `back_populate_legacy_ds_ids` step) is responsible for the
//! corresponding legacy DELETE; that integration sits outside this
//! pure-`build_statements` unit fixture and is covered by the live
//! coexistence acceptance test described in the audit doc.
//!
//! ## Spike capture provenance
//!
//! Both `walkin3-20260424-100000/writes.txt` and
//! `booking-checkin-20260424-101838/writes.txt` show one `HT_CheckIn_Ds`
//! INSERT per check-in. `findings.md` §7 "What we still don't know"
//! records the open question ("Whether multi-room check-ins use the same
//! flow"); the `COMPAT_CHEATSHEET` (landed 2026-05-11) answered it —
//! `HT_CheckIn_Ds` is one row per room — and Track B4 finally closes the
//! writeback gap.

#![allow(clippy::needless_collect)]

use chrono::{NaiveDate, TimeZone, Utc};

use hotel_backend::outbox::intent::RoomLine;
use hotel_backend::writeback::recipes::walkin::{build_statements, WalkInInputs};

/// Build a single-room walk-in input set that mirrors the
/// `CH26-005230` capture in
/// `docs/legacy-spike/raw/walkin3-20260424-100000/writes.txt` (room 508,
/// guest "SPIKE TEST WALKIN 3", 1 night). The capture is currently the
/// only single-room walk-in we have to anchor multi-room work against.
///
/// `created_at` is pinned to the captured wall-clock so any byte-parity
/// assertion stays deterministic.
fn walkin3_single_room_inputs() -> WalkInInputs<'static> {
    WalkInInputs {
        cin_no: "CH26-005230",
        cust_no: "C21609",
        customer_id_int: 21609,
        customer_name: "SPIKE TEST WALKIN 3",
        customer_phone: None,
        guest_name_for_registry: "SPIKE TEST WALKIN 3",
        guest_country: "",
        created_by: "Admin",
        room_no: "508",
        room_type: "Standard",
        // 10:01:11 BKK = 03:01:11 UTC on 2026-04-24.
        stay_start: Utc.with_ymd_and_hms(2026, 4, 24, 3, 1, 11).unwrap(),
        stay_end: Utc.with_ymd_and_hms(2026, 4, 25, 4, 59, 59).unwrap(),
        price_per_night_baht: 890.0,
        nights: 1,
        price_total_baht: 890.0,
        deposit_baht: 0.0,
        room_status_id_base: 50232,
        nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 24).unwrap()],
        checkin_ds_id: 25101,
        photo_tmp_no: None,
        created_at: Utc.with_ymd_and_hms(2026, 4, 24, 3, 1, 11).unwrap(),
        // Track B4 — legacy single-room path (empty slice).
        room_lines: Vec::new(),
    }
}

/// Track B4 — synthesize a 2-room walk-in by extending the single-room
/// fixture with a 2-entry `room_lines` slice. Mirrors what the service
/// layer will pack from `ht_checkin_rooms` once the multi-room walk-in
/// route (T4 HIGH-1) lands.
fn walkin3_two_room_inputs() -> WalkInInputs<'static> {
    let base = walkin3_single_room_inputs();
    WalkInInputs {
        room_lines: vec![
            RoomLine {
                room_no: "508".to_string(),
                room_type: "Standard".to_string(),
                price_per_night: 890.0,
                nights: 1,
                room_total: 890.0,
                room_status: String::new(),
                legacy_ds_id: None,
            },
            RoomLine {
                room_no: "509".to_string(),
                room_type: "Standard".to_string(),
                price_per_night: 890.0,
                nights: 1,
                room_total: 890.0,
                room_status: String::new(),
                legacy_ds_id: None,
            },
        ],
        ..base
    }
}

/// Sanity test: the current single-room build produces exactly ONE
/// `HT_CheckIn_Ds` INSERT. Track B4 preserves this byte-for-byte — the
/// new multi-room path only activates when `room_lines` is non-empty.
#[test]
fn single_room_walkin_emits_exactly_one_checkin_ds_row() {
    let statements = build_statements(&walkin3_single_room_inputs());
    let ds_inserts: Vec<&String> = statements
        .iter()
        .filter(|s| s.contains("[HT_CheckIn_Ds]"))
        .collect();
    assert_eq!(
        ds_inserts.len(),
        1,
        "single-room walk-in must emit exactly one HT_CheckIn_Ds INSERT; \
         got {} statements: {:?}",
        ds_inserts.len(),
        ds_inserts
    );
    // Confirm the captured room number rides through.
    assert!(
        ds_inserts[0].contains("'508'"),
        "HT_CheckIn_Ds INSERT must reference room 508 (captured walkin3): {:?}",
        ds_inserts[0]
    );
}

/// **Track B4 closes T2 CRIT-1.** The COMPAT_CHEATSHEET says
/// `HT_CheckIn_Ds` is "one row per room (a single check-in can cover
/// multiple rooms)" (lines 427-430). Track B4 wires
/// `WalkInInputs.room_lines` into `build_statements` so a 2-room walk-in
/// emits TWO `HT_CheckIn_Ds` INSERTs (one per room) plus a single
/// `HT_CheckIn_H` header carrying both room numbers in `Cin_Room_ALL`.
///
/// Was `#[ignore]`'d as the Track-B blocker; un-ignored 2026-05-14.
#[test]
fn two_room_walkin_emits_two_checkin_ds_rows_and_one_header() {
    let statements = build_statements(&walkin3_two_room_inputs());

    // Expected #1: TWO HT_CheckIn_Ds INSERTs (one per room).
    let ds_count = statements
        .iter()
        .filter(|s| s.contains("[HT_CheckIn_Ds]"))
        .count();
    assert_eq!(
        ds_count, 2,
        "multi-room walk-in must emit one HT_CheckIn_Ds per room (got {ds_count})"
    );

    // Expected #2: exactly ONE HT_CheckIn_H header (per-folio, not per-room).
    let h_count = statements
        .iter()
        .filter(|s| s.contains("[HT_CheckIn_H]"))
        .count();
    assert_eq!(
        h_count, 1,
        "multi-room walk-in must emit exactly one HT_CheckIn_H header (got {h_count})"
    );

    // Expected #3: Cin_Room_ALL on the header carries BOTH room numbers,
    // legacy format is space-separated with a trailing space — e.g.
    // `'508 509 '`.
    let h_insert = statements
        .iter()
        .find(|s| s.contains("[HT_CheckIn_H]"))
        .expect("HT_CheckIn_H INSERT must exist");
    assert!(
        h_insert.contains("'508 509 '"),
        "Cin_Room_ALL must enumerate all rooms (expected '508 509 '): {h_insert}"
    );

    // Expected #4: per-room HT_POWER_LOG rows scale with the room count.
    let power_log_count = statements
        .iter()
        .filter(|s| s.contains("[HT_POWER_LOG]"))
        .count();
    assert_eq!(
        power_log_count, 2,
        "multi-room walk-in must emit one HT_POWER_LOG per room (got {power_log_count})"
    );

    // Expected #5: per-room HT_Rooms occupancy UPDATEs.
    let room_use_updates = statements
        .iter()
        .filter(|s| s.starts_with("update HT_Rooms set room_use='yes'"))
        .count();
    assert_eq!(
        room_use_updates, 2,
        "multi-room walk-in must emit one `update HT_Rooms set room_use='yes'` per room \
         (got {room_use_updates})"
    );

    // Expected #6: HT_Room_Status row count = rooms × nights. The
    // 2-room × 1-night fixture should emit 2 rows.
    let room_status_inserts = statements
        .iter()
        .filter(|s| s.contains("INSERT INTO [HT_Room_Status]"))
        .count();
    assert_eq!(
        room_status_inserts, 2,
        "multi-room walk-in must emit one HT_Room_Status row per (room × night) \
         (got {room_status_inserts})"
    );

    // Expected #7: sequential ds_ids — first room's id = base, second = base+1.
    let ds_rows: Vec<&String> = statements
        .iter()
        .filter(|s| s.contains("[HT_CheckIn_Ds]"))
        .collect();
    assert!(
        ds_rows[0].contains("VALUES( 25101,"),
        "first HT_CheckIn_Ds row must use the allocated id base: {}",
        ds_rows[0]
    );
    assert!(
        ds_rows[1].contains("VALUES( 25102,"),
        "second HT_CheckIn_Ds row must use base+1 for race-safe contiguous ids: {}",
        ds_rows[1]
    );
}

/// Track B4 — edit-down path. A 3-room folio that gets edited to 2
/// rooms must emit only 2 `HT_CheckIn_Ds` INSERTs from the canonical
/// junction slice. The dropped room's legacy DELETE is the worker's
/// responsibility (it diffs `ht_checkin_rooms.cr_legacy_ds_id` against
/// the prior write's `LegacyIds.checkin_ds_ids_by_room` snapshot and
/// emits a targeted `DELETE FROM HT_CheckIn_Ds WHERE id=…` per
/// `findings.md` §3i recipe). This unit test pins the recipe-side
/// invariant: `room_lines.len()` ALWAYS equals the emitted
/// `HT_CheckIn_Ds` count.
#[test]
fn room_lines_drop_rooms_reduces_checkin_ds_emission() {
    // Start with 3 rooms.
    let three_room = WalkInInputs {
        room_lines: vec![
            RoomLine {
                room_no: "508".to_string(),
                room_type: "Standard".to_string(),
                price_per_night: 890.0,
                nights: 1,
                room_total: 890.0,
                room_status: String::new(),
                legacy_ds_id: None,
            },
            RoomLine {
                room_no: "509".to_string(),
                room_type: "Standard".to_string(),
                price_per_night: 890.0,
                nights: 1,
                room_total: 890.0,
                room_status: String::new(),
                legacy_ds_id: None,
            },
            RoomLine {
                room_no: "510".to_string(),
                room_type: "Standard".to_string(),
                price_per_night: 890.0,
                nights: 1,
                room_total: 890.0,
                room_status: String::new(),
                legacy_ds_id: None,
            },
        ],
        ..walkin3_single_room_inputs()
    };
    let three_ds = build_statements(&three_room)
        .iter()
        .filter(|s| s.contains("[HT_CheckIn_Ds]"))
        .count();
    assert_eq!(three_ds, 3, "3-room folio must emit 3 HT_CheckIn_Ds rows");

    // After the edit, the junction holds 2 rooms (room 510 dropped).
    let two_room_after_edit = walkin3_two_room_inputs();
    let two_ds = build_statements(&two_room_after_edit)
        .iter()
        .filter(|s| s.contains("[HT_CheckIn_Ds]"))
        .count();
    assert_eq!(
        two_ds, 2,
        "after edit-down to 2 rooms the recipe emits 2 HT_CheckIn_Ds rows \
         (the worker DELETEs the dropped legacy row outside this scope)"
    );

    // And Cin_Room_ALL on the post-edit header reflects the new room list,
    // not the original 3-room one.
    let post_edit_statements = build_statements(&two_room_after_edit);
    let h_insert = post_edit_statements
        .iter()
        .find(|s| s.contains("[HT_CheckIn_H]"))
        .expect("HT_CheckIn_H INSERT must exist");
    assert!(
        h_insert.contains("'508 509 '"),
        "post-edit Cin_Room_ALL must list only the surviving rooms ('508 509 '), \
         not the dropped 510: {h_insert}"
    );
    assert!(
        !h_insert.contains("510"),
        "dropped room 510 must NOT appear in the post-edit header: {h_insert}"
    );
}

/// Track B4 — verify the Thai status literal passes through verbatim
/// (constraint: "Preserve Thai literals in `Cin_R_Status` — pass
/// `cr_room_status` through verbatim"). A junction row with
/// `cr_room_status='จอง'` (Thai "reserved", e.g. a booking-linked
/// check-in where one of N rooms hasn't been physically occupied yet)
/// must surface that exact byte sequence in `Cin_Room_Status`.
#[test]
fn per_room_thai_status_literal_passes_through_verbatim() {
    let inputs = WalkInInputs {
        room_lines: vec![RoomLine {
            room_no: "508".to_string(),
            room_type: "Standard".to_string(),
            price_per_night: 890.0,
            nights: 1,
            room_total: 890.0,
            // Custom Thai status (verbatim from the legacy enum captured
            // in `findings.md` §3a — same byte sequence the .NET app
            // writes).
            room_status: "จอง".to_string(),
            legacy_ds_id: None,
        }],
        ..walkin3_single_room_inputs()
    };
    let statements = build_statements(&inputs);
    let ds = statements
        .iter()
        .find(|s| s.contains("[HT_CheckIn_Ds]"))
        .expect("HT_CheckIn_Ds INSERT must exist");
    assert!(
        ds.contains("'จอง'"),
        "Thai junction status 'จอง' must pass through verbatim into Cin_Room_Status: {ds}"
    );
}
