//! Report HK — the room report's canned vocabulary and its rules.
//!
//! The owner's `Report HK.xlsx` digitized (decisions grilled 2026-09-02);
//! domain language in `CONTEXT.md` §Housekeeping ("Room report (Report HK)",
//! "Report verification"). PURE: no `sqlx`, no `axum`, no I/O — every rule here
//! is a total function over its inputs and is unit-tested below, which is what
//! lets `service::hk_reports` and `routes::hk` share ONE enforcement point
//! instead of two drifting copies. Same construction as
//! [`crate::domain::hk_signal`].
//!
//! ## The wire codes are mirrored, not owned, here
//!
//! **`app/hk/report-vocab.ts` is the ONE place a report code is spelled**, and
//! the constants below mirror it. Keep them in lock-step: the frontend imports
//! that file, this module is what the 400 for an unknown code is served from,
//! and the database deliberately mirrors NEITHER — `rr_room_status`,
//! `rr_return_reason` and `rri_item` are `TEXT` with no CHECK (migration 091,
//! inheriting 088's rationale), so extending a list stays a constant edit that
//! ships with the frontend rather than an `ALTER` on a live table at two sites.
//!
//! `rri_problem` and `rr_status` ARE checked in the DB, because they are
//! structural rather than product vocabulary — see the migration.
//!
//! ## What is NOT here
//!
//! Thai labels. They belong to the client, which knows how to render them
//! (`reportItemLabel` in the vocab file); the wire codes stay ASCII
//! snake_case. And free text of any kind: the return reason is canned, there
//! are no remarks on a report, and there is no field for either — the same
//! discipline ADR 0008 records for room signals.

use serde::{Deserialize, Serialize};

// ============================================================================
// The canned vocabulary (mirrors `app/hk/report-vocab.ts`)
// ============================================================================

/// The in-room equipment checklist (อุปกรณ์ภายในห้อง), **in the paper form's
/// order** — mirrors `REPORT_ITEMS`.
///
/// ORDER IS THE DISPLAY ORDER and the frontend renders it from its own copy;
/// this array's job is to answer "is that a real item code". Items that ARE
/// linen reuse the exact `routes::hk::VALID_LINEN_KINDS` codes (`bath_towel`,
/// `face_towel`, `foot_towel`, `bed_sheet`, `pillowcase`, `duvet_cover`) so an
/// item exception and a ขาดผ้า report name the same thing — pinned by
/// [`tests::linen_items_reuse_the_linen_vocabulary`].
pub const REPORT_ITEMS: [&str; 22] = [
    "water_glass",        // แก้วน้ำ
    "coffee_tray",        // ถาดรองแก้วกาแฟ
    "coffee_cup",         // แก้วกาแฟ
    "coffee_sachet_jar",  // แก้วใส่ซองกาแฟ
    "kettle",             // กาน้ำร้อน
    "bathroom_bin",       // ถังขยะในห้องน้ำ
    "hairdryer",          // ไดร์เป่าผม
    "bath_amenity_tray",  // ถาดไม้รองอุปกรณ์อาบน้ำ
    "aircon_remote",      // รีโมทแอร์
    "tv_remote",          // รีโมทโทรทัศน์
    "mirror_bin",         // ถังขยะหน้ากระจก
    "hangers",            // ไม้แขวนเสื้อ
    "bath_towel",         // ผ้าขนหนู (รวมสีฟ้า)
    "face_towel",         // ผ้าเช็ดหน้า
    "foot_towel",         // ผ้าเช็ดเท้า
    "duvet",              // ผ้านวม
    "bed_sheet",          // ผ้าปูที่นอน
    "pillowcase",         // ปลอกหมอน
    "duvet_cover",        // ซองนวม
    "pillow",             // หมอน
    "ashtray",            // ที่เขี่ยบุหรี่
    "bathrobe",           // ผ้าคลุมอาบน้ำสีน้ำเงิน
];

/// The paper form's room-status legend (VC/CO/OO/SO) — mirrors
/// `ROOM_STATUS_CODES`.
///
/// `vc` ห้องทำความสะอาดแล้ว · `co` เช็คเอาท์ · `oo` รอซ่อม · `so` พักต่อ.
///
/// Prefilled client-side from known room facts, but the maid may override it
/// and what is stored is what SHE reported — so this list is a validity check,
/// never a claim about the room.
pub const ROOM_STATUS_CODES: [&str; 4] = ["vc", "co", "oo", "so"];

/// The WHOLE rejection vocabulary — mirrors `RETURN_REASONS`.
///
/// `not_clean` ยังไม่สะอาด · `items_mismatch` อุปกรณ์ไม่ตรงกับที่รายงาน ·
/// `photos_unclear` รูปไม่ชัดเจน.
///
/// Canned, like everything else on this surface: there is no free-text
/// rejection note and there must never be one (CONTEXT.md §Housekeeping,
/// "Report verification" — _Avoid_: free-text rejection notes).
pub const RETURN_REASONS: [&str; 3] = ["not_clean", "items_mismatch", "photos_unclear"];

/// Photo-evidence bounds, enforced client- AND server-side — mirrors
/// `REPORT_MIN_PHOTOS` / `REPORT_MAX_PHOTOS`.
///
/// They apply PER SIDE and per transition: a submission carries 1..=4 maid
/// photos, and a verification carries 1..=4 reception photos of its own. A
/// return carries none at all (the reason is the evidence there).
pub const REPORT_MIN_PHOTOS: usize = 1;
pub const REPORT_MAX_PHOTOS: usize = 4;

/// Inclusive quantity bounds for ONE item exception.
///
/// Mirrored by `CHECK (rri_qty >= 1 AND rri_qty <= 99)` in migration 091 and
/// re-stated by `routes::hk` so a bad body is a 400, not a 500 from the
/// constraint — exactly `MIN_LINEN_QTY` / `MAX_LINEN_QTY`'s arrangement.
pub const MIN_ITEM_QTY: i32 = 1;
pub const MAX_ITEM_QTY: i32 = 99;

/// The most exceptions ONE report may carry.
///
/// Equal to `REPORT_ITEMS.len() * 2` — every item, both problems — and that is
/// not a coincidence: exceptions are unique per (item, problem), so a body with
/// more entries than there are pairs is necessarily malformed regardless of the
/// duplicate check. Its own constant so widening the checklist later does not
/// silently widen the accepted body size before anyone has thought about it
/// (the `MAX_LINEN_ITEMS` precedent).
pub const MAX_REPORT_ITEMS: usize = REPORT_ITEMS.len() * 2;

// ============================================================================
// Structural enums
// ============================================================================

/// What can be wrong with an item — mirrors `ITEM_PROBLEMS`.
///
/// STRUCTURAL, and CHECKed in the DB (`rri_problem`): this is the same pair the
/// guest-accountability signals are built on, and [`Self::signal_type`] is the
/// mapping the submit transaction runs. A third problem would mean those
/// signals no longer cover the checklist — a redesign, not a vocabulary
/// addition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemProblem {
    /// หาย
    Missing,
    /// ชำรุด
    Damaged,
}

impl ItemProblem {
    pub fn as_str(self) -> &'static str {
        match self {
            ItemProblem::Missing => "missing",
            ItemProblem::Damaged => "damaged",
        }
    }

    /// Parse an already-normalized code. `None` for anything else.
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "missing" => Some(ItemProblem::Missing),
            "damaged" => Some(ItemProblem::Damaged),
            _ => None,
        }
    }

    /// The EXISTING room-signal type this problem raises (ADR 0008 /
    /// migration 089) — มีของหาย / มีของเสียหาย.
    ///
    /// THE bridge between the two features, and the reason [`ItemProblem`] is
    /// an enum rather than a validated string: an item exception is also a
    /// guest-accountability fact the desk must hear about BEFORE the guest
    /// settles, so `service::hk_reports` raises one of these in the submit's
    /// own transaction. Total over the enum, so the compiler — not a reviewer —
    /// guarantees every problem has exactly one signal.
    pub fn signal_type(self) -> &'static str {
        match self {
            ItemProblem::Missing => crate::domain::hk_signal::ITEM_MISSING,
            ItemProblem::Damaged => crate::domain::hk_signal::ITEM_DAMAGED,
        }
    }

    /// Both problems, in the vocab file's order — the iteration order the
    /// submit's signal-raising uses, so two reports with the same exceptions
    /// always raise their signals in the same order.
    pub const ALL: [ItemProblem; 2] = [ItemProblem::Missing, ItemProblem::Damaged];
}

/// Report lifecycle — mirrors the `ReportStatus` union.
///
/// CHECKed in the DB (`rr_status`) because it is structural: every transition
/// rule and the overview's "is this room's day settled" reading are written
/// over exactly this closed set.
///
/// APPEND-ONLY: a `Returned` report is never reopened or edited. It is
/// superseded by a NEW submission carrying `parentReportId`, which is what
/// keeps the whole chain readable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportStatus {
    Submitted,
    Verified,
    Returned,
}

impl ReportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            ReportStatus::Submitted => "submitted",
            ReportStatus::Verified => "verified",
            ReportStatus::Returned => "returned",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "submitted" => Some(ReportStatus::Submitted),
            "verified" => Some(ReportStatus::Verified),
            "returned" => Some(ReportStatus::Returned),
            _ => None,
        }
    }

    /// Is this report still awaiting a verdict? The submit guard's predicate
    /// and reception's queue filter, in one place.
    pub fn is_open(self) -> bool {
        matches!(self, ReportStatus::Submitted)
    }
}

/// Which side's evidence a photo is — mirrors `rrp_side`.
///
/// DERIVED FROM THE UPLOADER'S ROLE, never sent by the client: a maid's upload
/// is `Maid` evidence and a receptionist's is `Reception`, so neither can
/// manufacture the other's. CHECKed in the DB for `sig_direction`'s reason —
/// the two-sided-evidence rule is written over exactly these two values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PhotoSide {
    Maid,
    Reception,
}

impl PhotoSide {
    pub fn as_str(self) -> &'static str {
        match self {
            PhotoSide::Maid => "maid",
            PhotoSide::Reception => "reception",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "maid" => Some(PhotoSide::Maid),
            "reception" => Some(PhotoSide::Reception),
            _ => None,
        }
    }

    /// The uploader's side, from the ONE boolean `middleware::hk_access`
    /// resolved.
    ///
    /// `can_report == true` IS the maid side — the same derivation
    /// `routes::hk::hk_role` makes for signals, reading the same boolean, so a
    /// grant becomes a permission in exactly one place. A maid who ALSO holds
    /// the reception grant is still `can_report == true` and therefore still
    /// the maid here, which is precisely the "a maid never verifies" rule.
    pub fn from_can_report(can_report: bool) -> Self {
        if can_report {
            PhotoSide::Maid
        } else {
            PhotoSide::Reception
        }
    }
}

// ============================================================================
// Rules — total functions, no I/O
// ============================================================================

/// Why a report command was refused. Carries its own 400-vs-403 class, exactly
/// as [`crate::domain::hk_signal::SignalRuleError`] does, so a route can never
/// re-decide it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportRuleError {
    /// A code that is in NO vocabulary — a typo or a stale bundle. 400.
    UnknownCode { field: &'static str, got: String },
    /// The checklist contradicts its own attestation. 400.
    ItemsContradictAttestation { all_items_ok: bool, items: usize },
    /// The same (item, problem) pair twice. 400.
    DuplicateItem { item: String, problem: &'static str },
    /// Quantity outside 1..=99. 400.
    QtyOutOfRange { item: String },
    /// Too many exception lines to be a real report. 400.
    TooManyItems { got: usize },
    /// Photo count outside 1..=4. 400.
    PhotoCountOutOfRange { got: usize },
    /// The same photo id twice in one command. 400.
    DuplicatePhoto { photo_id: i64 },
    /// The caller's ROLE forbids this act outright — a maid verifying, or
    /// reception submitting. **403**, not 400: the command is well-formed and
    /// the answer is still no.
    WrongSide { action: &'static str },
    /// The report is not in a status this action can leave. Conflict.
    WrongStatus {
        action: &'static str,
        status: ReportStatus,
    },
}

impl ReportRuleError {
    /// Does this refusal mean 403 rather than 400/conflict? The ONE place the
    /// class is decided.
    pub fn is_forbidden(&self) -> bool {
        matches!(self, ReportRuleError::WrongSide { .. })
    }

    /// Is this a state-precondition failure (as opposed to a malformed
    /// command)? The service raises these as `ServiceError::Conflict`.
    pub fn is_conflict(&self) -> bool {
        matches!(self, ReportRuleError::WrongStatus { .. })
    }

    pub fn message(&self) -> String {
        match self {
            ReportRuleError::UnknownCode { field, got } => {
                format!("invalid {field} '{got}'")
            }
            ReportRuleError::ItemsContradictAttestation {
                all_items_ok,
                items,
            } => {
                if *all_items_ok {
                    format!(
                        "allItemsOk is true but {items} item exception(s) were sent; \
                         a report is either ครบทุกรายการ or a list of exceptions"
                    )
                } else {
                    "allItemsOk is false but no item exceptions were sent; \
                     name what is missing or damaged"
                        .to_string()
                }
            }
            ReportRuleError::DuplicateItem { item, problem } => format!(
                "duplicate exception for item '{item}' / '{problem}'; \
                 report each item-problem pair once with its total"
            ),
            ReportRuleError::QtyOutOfRange { item } => format!(
                "invalid qty for item '{item}' (expected an integer \
                 {MIN_ITEM_QTY}..={MAX_ITEM_QTY})"
            ),
            ReportRuleError::TooManyItems { got } => format!(
                "too many item exceptions ({got}); at most {MAX_REPORT_ITEMS} are accepted"
            ),
            ReportRuleError::PhotoCountOutOfRange { got } => format!(
                "expected {REPORT_MIN_PHOTOS}..={REPORT_MAX_PHOTOS} photos, got {got}"
            ),
            ReportRuleError::DuplicatePhoto { photo_id } => {
                format!("photo {photo_id} was listed twice")
            }
            ReportRuleError::WrongSide { action } => match *action {
                "verify" | "return" => {
                    "การตรวจรับรายงานเป็นหน้าที่ของแผนกต้อนรับ".to_string()
                }
                _ => "การส่งรายงานห้องเป็นหน้าที่ของแม่บ้าน".to_string(),
            },
            ReportRuleError::WrongStatus { action, status } => format!(
                "cannot {action} a report that is already '{}'",
                status.as_str()
            ),
        }
    }
}

/// Normalize a wire code the way every other `/hk` surface does: trim, then
/// lower-case. PURE.
///
/// The normalized value is what gets STORED, so the tables never accumulate
/// casing variants of one code — the same rule `routes::hk::parse_linen_items`
/// follows for linen kinds.
pub fn normalize_code(raw: &str) -> String {
    raw.trim().to_lowercase()
}

/// Is this a room-status code the app knows? Returns the canonical spelling.
pub fn parse_room_status(raw: &str) -> Result<&'static str, ReportRuleError> {
    let wanted = normalize_code(raw);
    ROOM_STATUS_CODES
        .iter()
        .find(|code| **code == wanted)
        .copied()
        .ok_or_else(|| ReportRuleError::UnknownCode {
            field: "roomStatus",
            got: raw.to_string(),
        })
}

/// Is this an item code the app knows? Returns the canonical spelling.
pub fn parse_item(raw: &str) -> Result<&'static str, ReportRuleError> {
    let wanted = normalize_code(raw);
    REPORT_ITEMS
        .iter()
        .find(|code| **code == wanted)
        .copied()
        .ok_or_else(|| ReportRuleError::UnknownCode {
            field: "item",
            got: raw.to_string(),
        })
}

/// Is this a problem code the app knows?
pub fn parse_problem(raw: &str) -> Result<ItemProblem, ReportRuleError> {
    ItemProblem::parse(&normalize_code(raw)).ok_or_else(|| ReportRuleError::UnknownCode {
        field: "problem",
        got: raw.to_string(),
    })
}

/// Is this a return reason the app knows? Returns the canonical spelling.
pub fn parse_return_reason(raw: &str) -> Result<&'static str, ReportRuleError> {
    let wanted = normalize_code(raw);
    RETURN_REASONS
        .iter()
        .find(|code| **code == wanted)
        .copied()
        .ok_or_else(|| ReportRuleError::UnknownCode {
            field: "reason",
            got: raw.to_string(),
        })
}

/// The exception-based checklist's ONE invariant: `items` is empty **iff**
/// `all_items_ok`. PURE.
///
/// Stated once, here, because it spans two tables (the flag is on the header,
/// the exceptions are rows) and is therefore not expressible as a DB CHECK —
/// so this function is the only thing standing between the wire and a report
/// that claims the room is fine while listing three broken items.
pub fn check_attestation(all_items_ok: bool, item_count: usize) -> Result<(), ReportRuleError> {
    if all_items_ok == (item_count == 0) {
        return Ok(());
    }
    Err(ReportRuleError::ItemsContradictAttestation {
        all_items_ok,
        items: item_count,
    })
}

/// The photo-count bound, per side and per transition. PURE.
pub fn check_photo_count(count: usize) -> Result<(), ReportRuleError> {
    if (REPORT_MIN_PHOTOS..=REPORT_MAX_PHOTOS).contains(&count) {
        return Ok(());
    }
    Err(ReportRuleError::PhotoCountOutOfRange { got: count })
}

/// May a report in `status` be verified or returned? PURE.
///
/// Only `submitted` may leave — a verified report is final and a returned one
/// is superseded by a NEW submission, never re-judged in place. Both terminal
/// statuses are answered with the SAME error carrying the status it is already
/// in, which is what a client needs to re-render.
pub fn check_can_judge(
    action: &'static str,
    status: ReportStatus,
) -> Result<(), ReportRuleError> {
    if status.is_open() {
        return Ok(());
    }
    Err(ReportRuleError::WrongStatus { action, status })
}

/// The role rule, in one place: **a maid never verifies, and reception never
/// submits.** PURE.
///
/// `can_report` is the single boolean `middleware::hk_access` resolved. An
/// identity that holds BOTH grants is `can_report == true` — the maid side —
/// which is exactly the CONTEXT.md rule "A maid never verifies, including one
/// who also holds the reception grant". There is deliberately no way to express
/// "act as the other side" here.
/// The action set is CLOSED and the fall-through DENIES: a typo in a caller's
/// action string must refuse everyone, never quietly hand the caller the other
/// side's rule. (`_ => !can_report` would have made `"submitt"` a
/// reception-only action, i.e. exactly the wrong answer, silently.)
pub fn check_side(action: &'static str, can_report: bool) -> Result<(), ReportRuleError> {
    let allowed = match action {
        "submit" => can_report,
        "verify" | "return" => !can_report,
        _ => false,
    };
    if allowed {
        Ok(())
    } else {
        Err(ReportRuleError::WrongSide { action })
    }
}

// ============================================================================
// The wire shape
// ============================================================================

/// Who did something, as both report DTOs render it. Structurally identical to
/// [`crate::domain::hk_signal::SignalActor`] and deliberately its own type: a
/// report actor is stamped BY NAME on a countersignature, so the two shapes
/// must be free to move apart without dragging the signal DTO with them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportActor {
    /// Verified HF ID badge. Never client-typed.
    pub badge: String,
    /// Display-name snapshot; `null` when the identity carries none.
    pub name: Option<String>,
}

/// One equipment exception on the wire — `{"item","problem","qty"}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportItem {
    /// A [`REPORT_ITEMS`] code, as stored (already normalized on write). A
    /// plain `String`, not an enum: server→client must be able to carry an item
    /// a deployed bundle predates rather than fail to serialize it (the client
    /// falls back to the raw code — `reportItemLabel`).
    pub item: String,
    pub problem: ItemProblem,
    pub qty: i32,
}

/// The FULL report — `GET /api/hk/reports/{id}` and every mutation's response.
///
/// Optional fields are emitted as explicit `null` rather than omitted, the same
/// rule [`crate::domain::hk_signal::RoomSignal`] follows: always-present keys
/// make the shape self-describing in a `curl` trace and let a client branch on
/// the VALUE rather than on whether the key exists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomReport {
    pub report_id: i64,
    pub room_id: i32,
    pub room_no: String,
    /// The Bangkok civil day this report is FOR, `YYYY-MM-DD`.
    pub date: String,
    pub status: ReportStatus,
    /// The code the maid REPORTED (vc | co | oo | so). A plain `String` for
    /// [`ReportItem::item`]'s reason.
    pub room_status: String,
    pub all_items_ok: bool,
    /// The exceptions; `[]` whenever `allItemsOk` is true.
    pub items: Vec<ReportItem>,
    /// A [`RETURN_REASONS`] code; `null` unless `status == "returned"`.
    pub return_reason: Option<String>,
    /// The returned report this submission supersedes; `null` for a first
    /// submission.
    pub parent_report_id: Option<i64>,
    pub submitted_by: ReportActor,
    /// RFC 3339 UTC. The client formats it; nothing here is Thai-local (these
    /// are `TIMESTAMPTZ` columns our own app wrote, NOT the naive Thai-local
    /// datetimes the legacy MSSQL side stores).
    pub submitted_at: String,
    /// Reception's countersignature — set for BOTH terminal statuses (who
    /// judged it), `null` while `submitted`.
    pub verified_by: Option<ReportActor>,
    pub verified_at: Option<String>,
    /// Photo ids, in upload order. Fetch the bytes from
    /// `GET /api/hk/report-photos/{id}`.
    pub maid_photo_ids: Vec<i64>,
    pub reception_photo_ids: Vec<i64>,
}

/// How many photos each side attached — the SUMMARY's stand-in for the two id
/// arrays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotoCounts {
    pub maid: usize,
    pub reception: usize,
}

/// The SUMMARY report — the day overview's per-room payload.
///
/// **Exactly [`RoomReport`] minus `items` / the two photo-id arrays, plus
/// `photoCounts`.** The overview lists every active room of a branch, so
/// carrying every room's exception rows and photo ids would make one screen's
/// payload grow with the property; the counts are what the card actually
/// renders, and the detail endpoint serves the rest for the one room a person
/// taps. Pinned field-for-field against [`RoomReport`] by
/// [`tests::the_summary_is_the_full_dto_minus_the_heavy_arrays`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomReportSummary {
    pub report_id: i64,
    pub room_id: i32,
    pub room_no: String,
    pub date: String,
    pub status: ReportStatus,
    pub room_status: String,
    pub all_items_ok: bool,
    pub return_reason: Option<String>,
    pub parent_report_id: Option<i64>,
    pub submitted_by: ReportActor,
    pub submitted_at: String,
    pub verified_by: Option<ReportActor>,
    pub verified_at: Option<String>,
    pub photo_counts: PhotoCounts,
}

/// One row of the day overview: an active room and its LATEST report for the
/// requested date (`null` when she has not filed one yet).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomReportRow {
    pub room_id: i32,
    pub room_no: String,
    pub floor: Option<i32>,
    pub building: Option<String>,
    /// `null` = no report for this room on this date. **Every active room is
    /// listed either way** — the overview is the paper sheet's heir and each
    /// side's work queue, so a missing report is the most important thing on
    /// it.
    pub report: Option<RoomReportSummary>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---- the mirrored vocabulary ----------------------------------------

    /// The four lists must be exactly what `app/hk/report-vocab.ts` spells.
    /// Sizes and membership are checked here; the frontend owns the labels.
    #[test]
    fn the_vocabulary_matches_the_locked_vocab_file() {
        assert_eq!(REPORT_ITEMS.len(), 22, "the paper form has 22 checklist rows");
        assert_eq!(ROOM_STATUS_CODES, ["vc", "co", "oo", "so"]);
        assert_eq!(
            RETURN_REASONS,
            ["not_clean", "items_mismatch", "photos_unclear"]
        );
        assert_eq!(REPORT_MIN_PHOTOS, 1);
        assert_eq!(REPORT_MAX_PHOTOS, 4);
        // First and last of the checklist pin the ORDER, which is the paper
        // form's and which the frontend renders from its own copy.
        assert_eq!(REPORT_ITEMS[0], "water_glass");
        assert_eq!(REPORT_ITEMS[REPORT_ITEMS.len() - 1], "bathrobe");
    }

    /// Every code is ASCII snake_case and unique — no Thai in the wire
    /// vocabulary (labels are the client's) and no duplicate that would make
    /// the duplicate-exception check meaningless.
    #[test]
    fn every_code_is_unique_ascii_snake_case() {
        for list in [&REPORT_ITEMS[..], &ROOM_STATUS_CODES[..], &RETURN_REASONS[..]] {
            for code in list {
                assert!(
                    code.chars()
                        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
                    "'{code}' must be ASCII snake_case"
                );
            }
            let mut seen = list.to_vec();
            seen.sort_unstable();
            let before = seen.len();
            seen.dedup();
            assert_eq!(seen.len(), before, "duplicate code in {list:?}");
        }
    }

    /// Items that ARE linen must reuse the EXACT `VALID_LINEN_KINDS` codes, so
    /// an item exception and a ขาดผ้า report name the same thing. A rename on
    /// either side breaks this.
    #[test]
    fn linen_items_reuse_the_linen_vocabulary() {
        for kind in crate::routes::hk::VALID_LINEN_KINDS {
            assert!(
                REPORT_ITEMS.contains(&kind),
                "linen kind '{kind}' must also be a checklist item code"
            );
        }
    }

    // ---- the structural enums -------------------------------------------

    /// Both problems map onto REAL room-signal types — the bridge the submit
    /// transaction walks. A rename in `domain::hk_signal` breaks this test
    /// rather than silently raising a signal nobody's board renders.
    #[test]
    fn every_problem_maps_to_a_maid_to_desk_signal() {
        use crate::domain::hk_signal::{MAID_TO_DESK_TYPES, ROOM_CHECK_PROBLEM_TYPES};
        for problem in ItemProblem::ALL {
            let signal = problem.signal_type();
            assert!(
                MAID_TO_DESK_TYPES.contains(&signal),
                "{} must raise a maid→desk signal, got '{signal}'",
                problem.as_str()
            );
            assert!(
                ROOM_CHECK_PROBLEM_TYPES.contains(&signal),
                "{} must raise a guest-accountability signal, got '{signal}'",
                problem.as_str()
            );
        }
        assert_eq!(ItemProblem::Missing.signal_type(), "item_missing");
        assert_eq!(ItemProblem::Damaged.signal_type(), "item_damaged");
    }

    /// Every enum round-trips through its stored literal, so a value written
    /// today is still readable tomorrow.
    #[test]
    fn the_stored_literals_round_trip() {
        for status in [
            ReportStatus::Submitted,
            ReportStatus::Verified,
            ReportStatus::Returned,
        ] {
            assert_eq!(ReportStatus::parse(status.as_str()), Some(status));
        }
        for problem in ItemProblem::ALL {
            assert_eq!(ItemProblem::parse(problem.as_str()), Some(problem));
        }
        for side in [PhotoSide::Maid, PhotoSide::Reception] {
            assert_eq!(PhotoSide::parse(side.as_str()), Some(side));
        }
        assert_eq!(ReportStatus::parse("SUBMITTED"), None, "parse is exact");
        assert_eq!(PhotoSide::parse("housekeeping"), None);
    }

    /// The wire spelling of each enum is the stored literal — one vocabulary,
    /// not two.
    #[test]
    fn serde_spells_the_enums_the_way_the_columns_do() {
        for status in [
            ReportStatus::Submitted,
            ReportStatus::Verified,
            ReportStatus::Returned,
        ] {
            assert_eq!(
                serde_json::to_string(&status).unwrap(),
                format!("\"{}\"", status.as_str())
            );
        }
        for problem in ItemProblem::ALL {
            assert_eq!(
                serde_json::to_string(&problem).unwrap(),
                format!("\"{}\"", problem.as_str())
            );
        }
        for side in [PhotoSide::Maid, PhotoSide::Reception] {
            assert_eq!(
                serde_json::to_string(&side).unwrap(),
                format!("\"{}\"", side.as_str())
            );
        }
    }

    /// `can_report` IS the maid side — including for an identity that also
    /// holds the reception grant, which is the whole "a maid never verifies"
    /// rule.
    #[test]
    fn the_photo_side_follows_can_report() {
        assert_eq!(PhotoSide::from_can_report(true), PhotoSide::Maid);
        assert_eq!(PhotoSide::from_can_report(false), PhotoSide::Reception);
    }

    // ---- the rules -------------------------------------------------------

    #[test]
    fn code_parsers_normalize_and_reject() {
        assert_eq!(parse_room_status(" VC ").unwrap(), "vc");
        assert_eq!(parse_item("TV_Remote").unwrap(), "tv_remote");
        assert_eq!(parse_problem(" Missing ").unwrap(), ItemProblem::Missing);
        assert_eq!(parse_return_reason("NOT_CLEAN").unwrap(), "not_clean");

        for bad in ["", "vacant", "clean", "ห้องสะอาด", "vc "] {
            let trimmed = bad.trim();
            if ROOM_STATUS_CODES.contains(&trimmed) {
                continue;
            }
            assert!(parse_room_status(bad).is_err(), "'{bad}' must be rejected");
        }
        assert!(parse_item("remote").is_err());
        assert!(parse_problem("broken").is_err(), "the code is 'damaged'");
        assert!(parse_return_reason("dirty").is_err());
    }

    /// The biconditional, both ways round. This is the one rule the DB cannot
    /// enforce, so it is the one most worth pinning.
    #[test]
    fn items_are_empty_iff_all_items_ok() {
        assert!(check_attestation(true, 0).is_ok());
        assert!(check_attestation(false, 1).is_ok());
        assert!(check_attestation(false, 7).is_ok());

        let err = check_attestation(true, 2).unwrap_err();
        assert!(!err.is_forbidden() && !err.is_conflict(), "malformed ⇒ 400");
        assert!(err.message().contains("allItemsOk is true"));

        let err = check_attestation(false, 0).unwrap_err();
        assert!(err.message().contains("allItemsOk is false"));
    }

    #[test]
    fn the_photo_bound_is_one_to_four() {
        for ok in REPORT_MIN_PHOTOS..=REPORT_MAX_PHOTOS {
            assert!(check_photo_count(ok).is_ok(), "{ok} photos must be accepted");
        }
        for bad in [0, 5, 40] {
            let err = check_photo_count(bad).unwrap_err();
            assert!(!err.is_forbidden(), "a bad count is 400, not 403");
            assert!(err.message().contains("1..=4"));
        }
    }

    /// Only `submitted` may be judged; both terminal statuses answer with a
    /// CONFLICT naming the status the report is already in.
    #[test]
    fn only_a_submitted_report_can_be_judged() {
        assert!(check_can_judge("verify", ReportStatus::Submitted).is_ok());
        assert!(check_can_judge("return", ReportStatus::Submitted).is_ok());
        for terminal in [ReportStatus::Verified, ReportStatus::Returned] {
            for action in ["verify", "return"] {
                let err = check_can_judge(action, terminal).unwrap_err();
                assert!(err.is_conflict(), "a wrong status is a precondition failure");
                assert!(!err.is_forbidden());
                assert!(
                    err.message().contains(terminal.as_str()),
                    "the message must name the status it is already in"
                );
            }
        }
    }

    /// The role rule in both directions — the reason this feature has two
    /// endpoints rather than one with a mode flag.
    #[test]
    fn a_maid_never_verifies_and_reception_never_submits() {
        assert!(check_side("submit", true).is_ok(), "a maid submits");
        assert!(check_side("verify", false).is_ok(), "reception verifies");
        assert!(check_side("return", false).is_ok(), "reception returns");

        for action in ["verify", "return"] {
            let err = check_side(action, true).unwrap_err();
            assert!(err.is_forbidden(), "a maid verifying is 403, not 400");
        }
        let err = check_side("submit", false).unwrap_err();
        assert!(err.is_forbidden(), "reception submitting is 403, not 400");

        // The action set is CLOSED and the fall-through DENIES. A typo must
        // refuse BOTH sides rather than silently granting one of them the
        // other's rule.
        for typo in ["submitt", "approve", "", "SUBMIT"] {
            for can_report in [true, false] {
                assert!(
                    check_side(typo, can_report).is_err(),
                    "'{typo}' must refuse can_report={can_report}"
                );
            }
        }
    }

    // ---- the wire shapes -------------------------------------------------

    /// The FULL DTO's key names are the contract. A rename here is a breaking
    /// API change and must fail loudly.
    #[test]
    fn the_full_report_serializes_the_contracted_keys() {
        let report = RoomReport {
            report_id: 7,
            room_id: 12,
            room_no: "101".to_string(),
            date: "2026-09-02".to_string(),
            status: ReportStatus::Submitted,
            room_status: "vc".to_string(),
            all_items_ok: false,
            items: vec![ReportItem {
                item: "tv_remote".to_string(),
                problem: ItemProblem::Missing,
                qty: 1,
            }],
            return_reason: None,
            parent_report_id: None,
            submitted_by: ReportActor {
                badge: "Q1001".to_string(),
                name: Some("มาลี".to_string()),
            },
            submitted_at: "2026-09-02T03:00:00+00:00".to_string(),
            verified_by: None,
            verified_at: None,
            maid_photo_ids: vec![4, 5],
            reception_photo_ids: vec![],
        };
        let json = serde_json::to_value(&report).expect("serializes");
        let object = json.as_object().expect("an object");
        let mut keys: Vec<&str> = object.keys().map(String::as_str).collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            [
                "allItemsOk",
                "date",
                "items",
                "maidPhotoIds",
                "parentReportId",
                "receptionPhotoIds",
                "reportId",
                "returnReason",
                "roomId",
                "roomNo",
                "roomStatus",
                "status",
                "submittedAt",
                "submittedBy",
                "verifiedAt",
                "verifiedBy",
            ]
        );
        // The nullable keys are PRESENT and null, never omitted.
        assert!(object["returnReason"].is_null());
        assert!(object["verifiedBy"].is_null());
        assert_eq!(json["items"][0]["problem"], "missing");
        assert_eq!(json["submittedBy"]["badge"], "Q1001");
    }

    /// The SUMMARY is the FULL DTO minus `items` / the two id arrays, plus
    /// `photoCounts` — stated in the contract and checked here so the two
    /// shapes cannot drift into different spellings of the same field.
    #[test]
    fn the_summary_is_the_full_dto_minus_the_heavy_arrays() {
        let summary = RoomReportSummary {
            report_id: 7,
            room_id: 12,
            room_no: "101".to_string(),
            date: "2026-09-02".to_string(),
            status: ReportStatus::Verified,
            room_status: "co".to_string(),
            all_items_ok: true,
            return_reason: None,
            parent_report_id: Some(3),
            submitted_by: ReportActor {
                badge: "Q1001".to_string(),
                name: None,
            },
            submitted_at: "2026-09-02T03:00:00+00:00".to_string(),
            verified_by: Some(ReportActor {
                badge: "R2002".to_string(),
                name: Some("ฝ่ายต้อนรับ".to_string()),
            }),
            verified_at: Some("2026-09-02T04:00:00+00:00".to_string()),
            photo_counts: PhotoCounts {
                maid: 2,
                reception: 1,
            },
        };
        let summary_json = serde_json::to_value(&summary).expect("serializes");
        let mut summary_keys: Vec<String> = summary_json
            .as_object()
            .expect("an object")
            .keys()
            .cloned()
            .collect();
        summary_keys.sort();

        // Rebuild the expectation FROM the full DTO's own key set, so adding a
        // field to `RoomReport` without deciding about the summary fails here.
        let full = RoomReport {
            report_id: 7,
            room_id: 12,
            room_no: "101".to_string(),
            date: "2026-09-02".to_string(),
            status: ReportStatus::Verified,
            room_status: "co".to_string(),
            all_items_ok: true,
            items: Vec::new(),
            return_reason: None,
            parent_report_id: Some(3),
            submitted_by: summary.submitted_by.clone(),
            submitted_at: summary.submitted_at.clone(),
            verified_by: summary.verified_by.clone(),
            verified_at: summary.verified_at.clone(),
            maid_photo_ids: vec![1, 2],
            reception_photo_ids: vec![9],
        };
        let mut expected: Vec<String> = serde_json::to_value(&full)
            .expect("serializes")
            .as_object()
            .expect("an object")
            .keys()
            .filter(|k| !["items", "maidPhotoIds", "receptionPhotoIds"].contains(&k.as_str()))
            .cloned()
            .collect();
        expected.push("photoCounts".to_string());
        expected.sort();

        assert_eq!(summary_keys, expected);
        assert_eq!(summary_json["photoCounts"]["maid"], 2);
        assert_eq!(summary_json["photoCounts"]["reception"], 1);
        assert_eq!(summary_json["status"], "verified");
    }

    /// A room with no report still appears on the overview, with `report:
    /// null` — the overview is a work queue, so an absent report is the most
    /// important row on it.
    #[test]
    fn an_overview_row_carries_an_explicit_null_report() {
        let row = RoomReportRow {
            room_id: 3,
            room_no: "205".to_string(),
            floor: Some(2),
            building: Some("A".to_string()),
            report: None,
        };
        let json = serde_json::to_value(&row).expect("serializes");
        assert!(json.get("report").expect("key present").is_null());
        assert_eq!(json["roomNo"], "205");
        assert_eq!(json["floor"], 2);
    }

    /// `MAX_REPORT_ITEMS` must stay tied to the checklist size, so widening
    /// the vocabulary cannot silently widen the accepted body first.
    #[test]
    fn the_body_cap_tracks_the_checklist_size() {
        assert_eq!(MAX_REPORT_ITEMS, REPORT_ITEMS.len() * ItemProblem::ALL.len());
    }
}
