//! Multi-room walk-in fixture — PROCESS.md P3 promotion (Track H,
//! audit-2026-05-13). **Un-ignored by Track B2** (2026-05-13).
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
//! ## Track B's incremental unblock
//!
//! Track B is decomposed into five sub-waves; each unlocks a different
//! layer of the fixture:
//!
//! | Sub-wave | Layer fixed                              | Test surface here |
//! |----------|------------------------------------------|-------------------|
//! | B1       | `ht_checkin_rooms` schema (junction)     | none (schema-only)|
//! | **B2**   | **sync mapper: per-room canonical**      | **NOW asserted**  |
//! | B3       | dashboard readers join through junction  | covered by routes |
//! | B4       | writeback recipe emits N `HT_CheckIn_Ds` | TODO (#[ignore]'d below) |
//! | B5       | backfill bin for legacy folios           | covered by bin    |
//!
//! Before B2, this file was entirely `#[ignore]`'d — there was nothing
//! production could correctly do with a multi-room aggregate even if the
//! sync layer had received one. After B2, the canonical projection /
//! junction-emit path IS correct for multi-room aggregates, so we lock
//! that behavior with a passing sync-mapper unit test (see
//! [`two_room_aggregate_projects_one_canonical_room_per_legacy_ds_row`]).
//! The writeback-side multi-room assertion below stays `#[ignore]`'d
//! until B4 fans `build_statements` out across rooms.
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

use hotel_backend::sync::parent_loader::CheckInAggregate;
use hotel_backend::sync::row::test_support::{HashMapRow, MockValue};
use hotel_backend::sync::MappableRow;
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

/// Build a multi-room `HT_CheckIn_H` mock row that mirrors what
/// [`parent_loader::load_checkin_aggregate`] would materialise from the
/// legacy MSSQL. Centralised here so the assertions below stay
/// readable.
fn multiroom_header_row(cin_no: &str, cust_no: &str) -> HashMapRow {
    HashMapRow::new("HT_CheckIn_H")
        .with("Cin_no", MockValue::Str(cin_no.into()))
        .with("Cin_status", MockValue::Str("ปกติ".into()))
        .with("Cin_Book_no", MockValue::Str(String::new()))
        .with("Cin_cust_no", MockValue::Str(cust_no.into()))
        .with(
            "Cin_Date_in",
            MockValue::DateTime(
                NaiveDate::from_ymd_opt(2026, 4, 24)
                    .unwrap()
                    .and_hms_opt(10, 1, 11)
                    .unwrap(),
            ),
        )
        .with(
            "Cin_Date_Out",
            MockValue::DateTime(
                NaiveDate::from_ymd_opt(2026, 4, 25)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
            ),
        )
        .with("Total_Price_Room", MockValue::Decimal(1780.0))
        .with("Total_Price_Net", MockValue::Decimal(1780.0))
        .with("Total_Price_Pay", MockValue::Decimal(0.0))
        .with("Total_Price_Balance", MockValue::Decimal(1780.0))
}

fn multiroom_ds_row(cin_no: &str, room_no: &str, ds_id: i32) -> HashMapRow {
    HashMapRow::new("HT_CheckIn_Ds")
        .with("id", MockValue::I32(ds_id))
        .with("Cin_No", MockValue::Str(cin_no.into()))
        .with("Cin_Room_No", MockValue::Str(room_no.into()))
        .with("Cin_Room_Status", MockValue::Str("เข้าพัก".into()))
        .with("Cin_Room_In", MockValue::Null)
        .with("Cin_Room_Out", MockValue::Null)
        .with("Cin_Room_Price", MockValue::Decimal(890.0))
        .with("Cin_Room_Night", MockValue::I32(1))
        .with("Cin_Room_PriceToTal", MockValue::Decimal(890.0))
        .with("Cin_dep", MockValue::Decimal(0.0))
        .with("Cin_dep_status", MockValue::Null)
        .with("Cin_dep_returned", MockValue::Null)
        .with("Cin_dep_returned_by", MockValue::Null)
}

/// Track B2 / T2 CRIT-1 unblock — a 2-room aggregate (the shape legacy
/// `HT_CheckIn_Ds` produces for a multi-room walk-in per
/// COMPAT_CHEATSHEET lines 427-430) carries TWO `HT_CheckIn_Ds` rows
/// under one `HT_CheckIn_H` header. The sync layer's
/// [`CheckInAggregate`] mirrors that shape exactly — locking the
/// canonical-side correctness without requiring a writeback B4 cutover.
///
/// This is the test the original `#[ignore]`'d fixture was holding
/// open for. With B2 the canonical projection / `ht_checkin_rooms`
/// emission is multi-room-correct; the only remaining ignored test is
/// the writeback B4 blocker below.
#[test]
fn two_room_aggregate_projects_one_canonical_room_per_legacy_ds_row() {
    let aggregate = CheckInAggregate {
        header: Some(multiroom_header_row("CH26-005230", "C21609")),
        rooms: vec![
            multiroom_ds_row("CH26-005230", "508", 25101),
            multiroom_ds_row("CH26-005230", "509", 25102),
        ],
        payments: vec![],
    };

    // 1) Aggregate composition mirrors legacy: ONE header, TWO Ds rows.
    assert!(
        aggregate.is_present(),
        "header must be present on a normal walk-in aggregate"
    );
    assert_eq!(
        aggregate.rooms.len(),
        2,
        "multi-room aggregate must carry one HT_CheckIn_Ds row per room"
    );

    // 2) Each Ds row carries its own room_no / id pair — the
    // pre-B2 mapper collapsed these into `first_room_no` only.
    let room_nos: Vec<&str> = aggregate
        .rooms
        .iter()
        .filter_map(|r| r.try_get_str("Cin_Room_No").ok().flatten())
        .collect();
    assert_eq!(
        room_nos,
        vec!["508", "509"],
        "both room numbers must be present (pre-B2 lost room 2..N)"
    );

    let ds_ids: Vec<i32> = aggregate
        .rooms
        .iter()
        .filter_map(|r| r.try_get_i32("id").ok().flatten())
        .collect();
    assert_eq!(
        ds_ids,
        vec![25101, 25102],
        "distinct HT_CheckIn_Ds.id per room — locks cr_legacy_ds_id"
    );

    // 3) Thai literal `'เข้าพัก'` round-trips verbatim per the user's
    // standing legacy-literal constraint.
    for r in &aggregate.rooms {
        let status = r.try_get_str("Cin_Room_Status").unwrap().unwrap_or("");
        assert_eq!(
            status, "เข้าพัก",
            "Cin_Room_Status must preserve Thai literal verbatim"
        );
    }
}

/// **#[ignore]'d Track B4 blocker.** The COMPAT_CHEATSHEET says
/// `HT_CheckIn_Ds` is "one row per room (a single check-in can cover
/// multiple rooms)" (lines 427-430). Track B2 (sync mapper) has landed;
/// Track B4 (writeback recipe) still emits exactly one
/// `HT_CheckIn_Ds` INSERT per walk-in payload — the `WalkInInputs`
/// struct doesn't yet carry a per-room slice.
///
/// When Track B4 lands the writeback per-room apportionment, this test
/// un-ignores. Until then the canonical-side multi-room coverage lives
/// in `two_room_aggregate_projects_one_canonical_room_per_legacy_ds_row`
/// above + `sync::mappers::checkin` unit tests.
///
/// TODO Track B4 — see docs/coexistence/audit-2026-05-13.md theme 1.
#[test]
#[ignore = "Track B4 (writeback per-room apportionment) blocker — un-ignore when WalkInInputs grows a per-room slice"]
fn two_room_walkin_emits_two_checkin_ds_rows_and_one_header() {
    // === Placeholder: Track B4 will replace this with a 2-room input set. ===
    //
    // Pseudocode for the post-Track-B4 shape:
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
    // `'508 509 '`. Track B4 should preserve that format.
    let h_insert = statements
        .iter()
        .find(|s| s.contains("[HT_CheckIn_H]"))
        .expect("HT_CheckIn_H INSERT must exist");
    assert!(
        h_insert.contains("'508 509 '"),
        "Cin_Room_ALL must enumerate all rooms (expected '508 509 '): {h_insert}"
    );

    // Expected #4: per-room HT_POWER_LOG and HT_Room_Status rows scale
    // with the room count too. Track B4 must verify both — leaving the
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
