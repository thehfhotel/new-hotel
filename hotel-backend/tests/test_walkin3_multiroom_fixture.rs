//! Multi-room walk-in fixture — PROCESS.md P3 promotion (Track H,
//! audit-2026-05-13).
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
//! This file is the **belated** promotion. Today the captures are
//! single-room (only one `HT_CheckIn_Ds` INSERT per `Cin_no`), so a fixture
//! that asserts on the current single-room shape would just freeze the
//! buggy behavior in place. Instead this fixture is **#[ignore]'d** with a
//! `TODO Track B` annotation: it codifies the EXPECTED multi-room SHAPE
//! once Track B lands the `ht_checkin_rooms` junction table + multi-room
//! payload, so we don't ship multi-room without a 2-room regression test.
//!
//! ## How Track B should unblock this
//!
//! When Track B (T1 CRIT-1 + T2 CRIT-1) lands:
//!   1. Schema migration: add `ht_checkin_rooms` junction.
//!   2. `CreateCheckInPayload` (or equivalent walk-in intent) grows from a
//!      single `room_no` / `room_type` to a `Vec<RoomLine>` (or similar
//!      multi-room shape).
//!   3. The walk-in recipe's `build_statements()` emits N `HT_CheckIn_Ds`
//!      INSERTs — one per room.
//!   4. Remove the `#[ignore]` attribute below and update the assertions
//!      to reflect the new payload struct.
//!
//! When step 3 lands without step 4, the test still won't run (ignored),
//! but the file's presence is the standing reminder to come back and flip
//! it. CI surfaces ignored tests in its output — see `cargo test -- --ignored`.
//!
//! ## Spike capture provenance
//!
//! Both `walkin3-20260424-100000/writes.txt` and
//! `booking-checkin-20260424-101838/writes.txt` show one `HT_CheckIn_Ds`
//! INSERT per check-in. `findings.md:648` records the open question
//! ("Whether multi-room check-ins use the same flow") that motivated this
//! fixture. The COMPAT_CHEATSHEET (landed 2026-05-11) answered that
//! question — `HT_CheckIn_Ds` is one row per room — but the schema
//! re-audit that would have surfaced the gap didn't happen, hence
//! PROCESS.md P2 (re-audit on reference-doc landing) + P3 (this file).

#![allow(clippy::needless_collect)]

use chrono::{NaiveDate, TimeZone, Utc};

use hotel_backend::writeback::recipes::walkin::{build_statements, WalkInInputs};

/// Build a single-room walk-in input set that mirrors the
/// `CH26-005230` capture in
/// `docs/legacy-spike/raw/walkin3-20260424-100000/writes.txt` (room 508,
/// guest "SPIKE TEST WALKIN 3", 1 night). The capture is currently the
/// only single-room walk-in we have to anchor multi-room work against —
/// the multi-room assertions below extrapolate from it.
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
        room_status_id_base: 50232,
        nights_calendar: vec![NaiveDate::from_ymd_opt(2026, 4, 24).unwrap()],
        checkin_ds_id: 25101,
        photo_tmp_no: None,
        created_at: Utc.with_ymd_and_hms(2026, 4, 24, 3, 1, 11).unwrap(),
    }
}

/// Sanity test: the current single-room build produces exactly ONE
/// `HT_CheckIn_Ds` INSERT. This is the BUG today — Track B's multi-room
/// rewrite will fan this out to N rows. The assertion is intentionally
/// stated as the documented single-room behavior so a future change that
/// accidentally regresses single-room emission (e.g. emitting zero rows)
/// still fails loud. The companion ignored test below covers the
/// expected post-Track-B multi-room shape.
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

/// **#[ignore]'d Track-B blocker.** The COMPAT_CHEATSHEET says
/// `HT_CheckIn_Ds` is "one row per room (a single check-in can cover
/// multiple rooms)" (lines 427-430). Once Track B lands the
/// `ht_checkin_rooms` junction + multi-room walk-in payload, this test
/// must un-ignore and assert that a 2-room walk-in emits TWO
/// `HT_CheckIn_Ds` INSERTs — one per room — plus a single `HT_CheckIn_H`
/// header.
///
/// The test is structured so a future engineer can:
///   1. Remove the `#[ignore]`.
///   2. Replace `walkin3_single_room_inputs()` with the multi-room
///      input builder Track B introduces.
///   3. Run `cargo test -- multiroom` and watch the new assertions hold.
///
/// TODO Track B — see docs/coexistence/audit-2026-05-13.md theme 1.
#[test]
#[ignore = "Track B (multi-room canonical schema) blocker — un-ignore when ht_checkin_rooms lands"]
fn two_room_walkin_emits_two_checkin_ds_rows_and_one_header() {
    // === Placeholder: Track B will replace this with a 2-room input set. ===
    //
    // Pseudocode for the post-Track-B shape:
    //
    //     let inputs = WalkInInputs {
    //         room_lines: vec![
    //             RoomLine { room_no: "508", room_type: "Standard", price: 890.0, ... },
    //             RoomLine { room_no: "509", room_type: "Standard", price: 890.0, ... },
    //         ],
    //         ...
    //     };
    let inputs = walkin3_single_room_inputs();

    let statements = build_statements(&inputs);

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
    // `'508 509 '`. Track B should preserve that format.
    let h_insert = statements
        .iter()
        .find(|s| s.contains("[HT_CheckIn_H]"))
        .expect("HT_CheckIn_H INSERT must exist");
    assert!(
        h_insert.contains("'508 509 '"),
        "Cin_Room_ALL must enumerate all rooms (expected '508 509 '): {h_insert}"
    );

    // Expected #4: per-room HT_POWER_LOG and HT_Room_Status rows scale
    // with the room count too. Track B must verify both — leaving the
    // assertion shape here as a TODO.
    let power_log_count = statements
        .iter()
        .filter(|s| s.contains("[HT_POWER_LOG]"))
        .count();
    assert_eq!(
        power_log_count, 2,
        "multi-room walk-in must emit one HT_POWER_LOG per room (got {power_log_count})"
    );
}
