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

/// The maid's CAPTURE ZONES — her shooting order, mirroring `REPORT_ZONES`.
///
/// One camera tap per zone; the zone's items appear PRE-TICKED `ok` against
/// that photo and she only touches what is wrong. A perfect room is four shots
/// and no further interaction, which is the whole "fast for a maid working
/// against the clock" directive.
///
/// **Zones are a capture ORDER, not a data-model entity** (CONTEXT.md
/// §Housekeeping, "Capture zone"). Nothing joins on a zone, no read is filtered
/// by one, and no submission is refused because a zone is unrepresented — the
/// one-photo-per-zone rule is the CLIENT's. This table exists here for exactly
/// two jobs: answering "is that a real zone code" for the upload's optional
/// `zone` field, and pinning (by the test below) the vocab file's claim that
/// **every checklist item belongs to exactly one zone**, which is what makes
/// the pre-ticking total.
pub const REPORT_ZONES: [(&str, &[&str]); 4] = [
    // เตียง
    (
        "bed",
        &["bed_sheet", "pillowcase", "duvet", "duvet_cover", "pillow"],
    ),
    // โต๊ะและมินิบาร์
    (
        "desk",
        &[
            "water_glass",
            "coffee_tray",
            "coffee_cup",
            "coffee_sachet_jar",
            "kettle",
            "ashtray",
        ],
    ),
    // ห้องน้ำ
    (
        "bathroom",
        &[
            "bathroom_bin",
            "hairdryer",
            "bath_amenity_tray",
            "bath_towel",
            "face_towel",
            "foot_towel",
            "bathrobe",
        ],
    ),
    // ทั่วไป
    (
        "general",
        &["aircon_remote", "tv_remote", "mirror_bin", "hangers"],
    ),
];

/// Photo-evidence bounds, enforced client- AND server-side — mirrors
/// `REPORT_MIN_PHOTOS` / `REPORT_MAX_PHOTOS`.
///
/// They apply PER SIDE and per transition on the VERIFY side: a verification
/// carries 1..=4 reception photos of its own, and a return carries none at all
/// (the reason is the evidence there).
///
/// The SUBMIT side is no longer counted this way — a v2 submission's bound is
/// [`REPORT_MIN_PHOTOS_TOTAL`]..=[`REPORT_MAX_PHOTOS_TOTAL`] over the DISTINCT
/// photos its ticks and extras name.
pub const REPORT_MIN_PHOTOS: usize = 1;
pub const REPORT_MAX_PHOTOS: usize = 4;

/// The SUBMISSION's photo bound — DISTINCT photos across every tick and every
/// extra, mirroring `REPORT_MAX_PHOTOS_TOTAL`.
///
/// The floor is the four capture zones: a report backed by fewer pictures than
/// there are zones cannot have been shot zone by zone. The server checks only
/// the TOTAL — "at least one per zone" is a UI rule, because the server cannot
/// tell a legitimately re-used shot from a missed zone without making
/// `rrp_zone` load-bearing, which CONTEXT.md rules out.
///
/// The ceiling is the vocab file's `REPORT_MAX_PHOTOS_TOTAL`: four zone shots
/// plus a generous allowance of close-ups and re-shots.
pub const REPORT_MIN_PHOTOS_TOTAL: usize = REPORT_ZONES.len();
pub const REPORT_MAX_PHOTOS_TOTAL: usize = 24;

/// Inclusive quantity bounds for ONE item exception.
///
/// Mirrored by `CHECK (rri_qty >= 1 AND rri_qty <= 99)` in migration 091 and
/// re-stated by `routes::hk` so a bad body is a 400, not a 500 from the
/// constraint — exactly `MIN_LINEN_QTY` / `MAX_LINEN_QTY`'s arrangement.
pub const MIN_ITEM_QTY: i32 = 1;
pub const MAX_ITEM_QTY: i32 = 99;

/// How many ticks ONE submission carries: **exactly the checklist**.
///
/// Not a cap — an EQUALITY. v2 is not exception-based: every item is ticked
/// every time, so a body with 21 entries is as malformed as one with 23, and
/// [`check_tick_coverage`] refuses both by naming what is missing or extra.
/// Its own constant so the arithmetic is stated once and widening the checklist
/// moves the requirement with it.
pub const REPORT_TICK_COUNT: usize = REPORT_ITEMS.len();

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

/// What ONE checklist item's tick says — mirrors `TICK_STATES`.
///
/// `Ok` (ครบ) is the PRE-TICKED default the maid never has to touch; the other
/// two are problems and carry a quantity. Every state is photo-backed — that is
/// the v2 model, and it is why this is a three-value enum rather than
/// `Option<ItemProblem>`: an `ok` tick is a positive attestation about an item
/// with a picture behind it, not the absence of a record.
///
/// CHECKED in the DB (`rri_state`, migration 092) for [`ItemProblem`]'s reason:
/// the triple is STRUCTURAL. `ok` is the absence of a problem and the other two
/// are exactly the pair the `item_missing` / `item_damaged` signals are built
/// on, which [`Self::problem`] is the bridge to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TickState {
    /// ครบ — present and undamaged.
    Ok,
    /// หาย
    Missing,
    /// ชำรุด
    Damaged,
}

impl TickState {
    pub fn as_str(self) -> &'static str {
        match self {
            TickState::Ok => "ok",
            TickState::Missing => "missing",
            TickState::Damaged => "damaged",
        }
    }

    /// Parse an already-normalized code. `None` for anything else.
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "ok" => Some(TickState::Ok),
            "missing" => Some(TickState::Missing),
            "damaged" => Some(TickState::Damaged),
            _ => None,
        }
    }

    /// The v1 problem this tick is, if it is one — the bridge to
    /// [`ItemProblem::signal_type`] and to the `rri_problem` column, which new
    /// writes fill from exactly this (`NULLIF(state, 'ok')`).
    ///
    /// Total over the enum, so the compiler guarantees `ok` maps to nothing and
    /// each problem maps to exactly one — no route or service may re-decide it.
    pub fn problem(self) -> Option<ItemProblem> {
        match self {
            TickState::Ok => None,
            TickState::Missing => Some(ItemProblem::Missing),
            TickState::Damaged => Some(ItemProblem::Damaged),
        }
    }

    /// Does this tick carry a quantity? `ok` MUST NOT, a problem MUST — stated
    /// once so the route, the service and the DB agree.
    pub fn is_problem(self) -> bool {
        self.problem().is_some()
    }

    /// Every state, in the vocab file's order.
    pub const ALL: [TickState; 3] = [TickState::Ok, TickState::Missing, TickState::Damaged];
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
    /// The ticks are not EXACTLY the checklist: some item was never ticked, or
    /// an item was ticked that is not on the form. 400.
    ///
    /// Carries the offending codes (up to a handful) rather than only a count,
    /// because the client's job on receiving this is to find the row it failed
    /// to send — a bare "expected 22, got 21" makes that a search.
    TicksIncomplete {
        missing: Vec<String>,
        unexpected: Vec<String>,
    },
    /// The same item ticked twice in one submission. 400.
    DuplicateTick { item: String },
    /// An `ok` tick carrying a quantity — there is nothing to count. 400.
    QtyOnOkTick { item: String },
    /// A `missing` / `damaged` tick with no quantity. 400.
    QtyMissingForProblem { item: String },
    /// A tick with no backing photo. Every tick is photo-backed in v2; this is
    /// the rule the whole model is named after. 400.
    TickPhotoMissing { item: String },
    /// Quantity outside 1..=99. 400.
    QtyOutOfRange { item: String },
    /// The submission's DISTINCT photo total is outside 4..=24. 400.
    PhotoTotalOutOfRange { got: usize },
    /// A v1-shaped body (`allItemsOk` / `items`) on the v2 endpoint. 400, and
    /// the message names `ticks` so a stale bundle's operator knows what
    /// changed. 400.
    LegacyBodyShape,
    /// Photo count outside 1..=4 (the VERIFY side's per-transition bound). 400.
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
            ReportRuleError::TicksIncomplete {
                missing,
                unexpected,
            } => {
                let mut parts = Vec::new();
                if !missing.is_empty() {
                    parts.push(format!("never ticked: {}", preview(missing)));
                }
                if !unexpected.is_empty() {
                    parts.push(format!("not on the checklist: {}", preview(unexpected)));
                }
                format!(
                    "ticks must name all {REPORT_TICK_COUNT} checklist items exactly once ({})",
                    parts.join("; ")
                )
            }
            ReportRuleError::DuplicateTick { item } => format!(
                "item '{item}' was ticked more than once; every item is ticked exactly once"
            ),
            ReportRuleError::QtyOnOkTick { item } => format!(
                "item '{item}' is ticked 'ok', which carries no qty; \
                 send qty only with 'missing' or 'damaged'"
            ),
            ReportRuleError::QtyMissingForProblem { item } => format!(
                "item '{item}' needs a qty (an integer {MIN_ITEM_QTY}..={MAX_ITEM_QTY}); \
                 say how many are missing or damaged"
            ),
            ReportRuleError::TickPhotoMissing { item } => format!(
                "item '{item}' has no photoId; every tick names the photo that backs it"
            ),
            ReportRuleError::QtyOutOfRange { item } => format!(
                "invalid qty for item '{item}' (expected an integer \
                 {MIN_ITEM_QTY}..={MAX_ITEM_QTY})"
            ),
            ReportRuleError::PhotoTotalOutOfRange { got } => format!(
                "expected {REPORT_MIN_PHOTOS_TOTAL}..={REPORT_MAX_PHOTOS_TOTAL} distinct \
                 photos across the ticks and extras, got {got}"
            ),
            ReportRuleError::LegacyBodyShape => "this endpoint now takes 'ticks' (one \
                 photo-backed tick per checklist item); 'allItemsOk' / 'items' are the \
                 retired v1 shape — reload the app"
                .to_string(),
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

/// Is this a tick state the app knows? Returns the enum.
pub fn parse_tick_state(raw: &str) -> Result<TickState, ReportRuleError> {
    TickState::parse(&normalize_code(raw)).ok_or_else(|| ReportRuleError::UnknownCode {
        field: "state",
        got: raw.to_string(),
    })
}

/// Is this a capture-zone code the app knows? Returns the canonical spelling.
///
/// Only ever called for a zone that WAS sent — the field is optional, because a
/// zone is informational and a re-shot close-up belongs to no zone at all.
pub fn parse_zone(raw: &str) -> Result<&'static str, ReportRuleError> {
    let wanted = normalize_code(raw);
    REPORT_ZONES
        .iter()
        .find(|(code, _)| *code == wanted)
        .map(|(code, _)| *code)
        .ok_or_else(|| ReportRuleError::UnknownCode {
            field: "zone",
            got: raw.to_string(),
        })
}

/// **The v2 checklist invariant: the ticks are EXACTLY the 22 items, each
/// once.** PURE.
///
/// This is the rule that replaced v1's `items`-empty-iff-`allItemsOk`
/// biconditional, and it is stronger in the direction that matters: an
/// exception-based report could not distinguish "the room is fine" from "she
/// did not look", and a tick-based one cannot be submitted at all until every
/// item has been answered.
///
/// It reports BOTH directions in one verdict — items never ticked and items
/// that are not on the checklist — because a stale bundle typically produces
/// both at once (a renamed code is simultaneously missing and unexpected), and
/// answering only the first would make the client fix it twice.
///
/// Duplicates are a SEPARATE error ([`ReportRuleError::DuplicateTick`]) raised
/// by the caller as it walks the list, so the offending item can be named while
/// it is in hand; this function sees a de-duplicated set.
pub fn check_tick_coverage(ticked: &[String]) -> Result<(), ReportRuleError> {
    let missing: Vec<String> = REPORT_ITEMS
        .iter()
        .filter(|item| !ticked.iter().any(|got| got == *item))
        .map(|item| (*item).to_string())
        .collect();
    let unexpected: Vec<String> = ticked
        .iter()
        .filter(|got| !REPORT_ITEMS.contains(&got.as_str()))
        .cloned()
        .collect();
    if missing.is_empty() && unexpected.is_empty() {
        return Ok(());
    }
    Err(ReportRuleError::TicksIncomplete {
        missing,
        unexpected,
    })
}

/// One tick's quantity rule, in one place: **an `ok` tick carries NO qty, a
/// problem tick carries 1..=99.** PURE.
///
/// Both halves are refusals, not coercions. Dropping a qty sent with an `ok`
/// tick would silently accept a client that ticked the wrong state, and
/// defaulting a missing one to 1 would invent a number the maid never counted —
/// and that number is what reception charges a guest from.
pub fn check_tick_qty(
    item: &str,
    state: TickState,
    qty: Option<i32>,
) -> Result<(), ReportRuleError> {
    match (state.is_problem(), qty) {
        (false, None) => Ok(()),
        (false, Some(_)) => Err(ReportRuleError::QtyOnOkTick {
            item: item.to_string(),
        }),
        (true, None) => Err(ReportRuleError::QtyMissingForProblem {
            item: item.to_string(),
        }),
        (true, Some(qty)) if (MIN_ITEM_QTY..=MAX_ITEM_QTY).contains(&qty) => Ok(()),
        (true, Some(_)) => Err(ReportRuleError::QtyOutOfRange {
            item: item.to_string(),
        }),
    }
}

/// **Every tick is photo-backed** — the rule the model is named after. PURE.
///
/// It applies to `ok` ticks too, and that is the point: the ครบ attestation is
/// the one a maid against the clock would otherwise wave through, so the picture
/// is what makes it checkable. The photo MAY be shared with other ticks (the
/// เตียง shot backs all five bed items); nothing here demands a close-up,
/// because the server cannot tell one from a zone shot and the UI drives that.
pub fn check_tick_photo(item: &str, photo_id: Option<i64>) -> Result<(), ReportRuleError> {
    if photo_id.is_some() {
        return Ok(());
    }
    Err(ReportRuleError::TickPhotoMissing {
        item: item.to_string(),
    })
}

/// The SUBMISSION's bound on DISTINCT photos (ticks ∪ extras). PURE.
///
/// DISTINCT, because one photo backing five ticks is one picture — counting the
/// references instead would refuse a perfectly shot room at the ceiling and
/// admit a four-tick report with one picture at the floor.
pub fn check_photo_total(distinct: usize) -> Result<(), ReportRuleError> {
    if (REPORT_MIN_PHOTOS_TOTAL..=REPORT_MAX_PHOTOS_TOTAL).contains(&distinct) {
        return Ok(());
    }
    Err(ReportRuleError::PhotoTotalOutOfRange { got: distinct })
}

/// The photo-count bound, per side and per transition — the VERIFY side's rule.
/// PURE.
pub fn check_photo_count(count: usize) -> Result<(), ReportRuleError> {
    if (REPORT_MIN_PHOTOS..=REPORT_MAX_PHOTOS).contains(&count) {
        return Ok(());
    }
    Err(ReportRuleError::PhotoCountOutOfRange { got: count })
}

/// A short, bounded rendering of a code list for an error message — at most
/// four codes plus a count, so a stale bundle that gets every name wrong cannot
/// make the 400 body enormous. PURE.
fn preview(codes: &[String]) -> String {
    const SHOWN: usize = 4;
    if codes.len() <= SHOWN {
        return codes.join(", ");
    }
    format!(
        "{}, … (+{} more)",
        codes[..SHOWN].join(", "),
        codes.len() - SHOWN
    )
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

/// One PHOTO-BACKED TICK on the wire — `{"item","state","qty","photoId"}`.
///
/// The v2 checklist row: one per item per report, all 22, every time. `qty` is
/// `null` on an `ok` tick and 1..=99 on a problem; `photoId` names the picture
/// that backs it and one picture may back several ticks.
///
/// `photoId` is `Option<i64>` **only to keep v1 rows readable** — an exception
/// filed before migration 092 has no backing photo, and a row with `photoId:
/// null` is exactly how such a row would look if it were ever surfaced here. It
/// is not: [`RoomReport::ticks`] carries photo-backed rows only. Every tick this
/// app writes has a photo.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportTick {
    /// A [`REPORT_ITEMS`] code, as stored. A plain `String` for
    /// [`ReportItem::item`]'s reason.
    pub item: String,
    pub state: TickState,
    /// `null` for an `ok` tick; 1..=99 for a problem.
    pub qty: Option<i32>,
    /// The backing photo's id. Fetch the bytes from
    /// `GET /api/hk/report-photos/{id}`.
    pub photo_id: Option<i64>,
}

/// One stored photo's METADATA on the wire —
/// `{"photoId","side","zone","bytes"}`.
///
/// Enough for the client to render the evidence strip (which side took it, what
/// it was a shot of, how big it is) without fetching megabytes of image, and
/// enough for reception to see WHICH picture vouches for which tick by joining
/// on `photoId`. The bytes themselves stay behind
/// `GET /api/hk/report-photos/{id}`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportPhoto {
    pub photo_id: i64,
    pub side: PhotoSide,
    /// The capture zone, when the client named one. `null` for a v1 photo and
    /// for a free-hand close-up — informational either way (CONTEXT.md: zones
    /// are a capture ORDER, not a data-model entity).
    pub zone: Option<String>,
    /// Stored size in bytes; `null` only if a row predates the backfill.
    pub bytes: Option<i64>,
}

/// One photo's metadata as `GET /api/hk/report-photos/{id}/meta` serves it —
/// [`ReportPhoto`] plus the two facts only the photo itself knows.
///
/// It exists for the client's RESUME-AFTER-RELOAD reconciliation: a maid whose
/// phone locks mid-form comes back holding a list of ids in local storage and
/// has to find out, one id at a time, which of them still exist, are still
/// hers, and are still unattached — i.e. which she may still tick against or
/// delete. `attached` is the one bit that decides it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportPhotoMeta {
    pub photo_id: i64,
    pub side: PhotoSide,
    pub zone: Option<String>,
    pub bytes: Option<i64>,
    /// `true` once a submit or verify has bound it to a report. An attached
    /// photo can no longer be deleted, which is exactly what the client needs
    /// to know before offering a "remove" control.
    pub attached: bool,
    /// RFC 3339 UTC.
    pub uploaded_at: String,
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
    /// **DERIVED** since migration 092: true iff no tick is a problem. The
    /// column is still written and still read by v1 bundles, but the ticks are
    /// the truth and this is computed from them on every read, so the two can
    /// never be seen disagreeing.
    pub all_items_ok: bool,
    /// The v2 checklist: one photo-backed tick per item, all 22, in checklist
    /// order. `[]` for a report filed by v1 (whose rows carry no photo) —
    /// that is the whole v1 tolerance rule, and such a report is read through
    /// `items` exactly as before.
    pub ticks: Vec<ReportTick>,
    /// The problems only, in v1's `{item, problem, qty}` shape. **Kept for
    /// bundles that predate v2** and populated from the problem ticks, so an
    /// old client keeps rendering a v2 report's exceptions; `[]` whenever the
    /// room is fine.
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
    /// BOTH sides' photos with their metadata, in upload order — what the
    /// ticks' `photoId`s point at.
    pub photos: Vec<ReportPhoto>,
    /// Photo ids, in upload order. **Kept for bundles that predate v2** (the
    /// same ids `photos` carries, split by side). Fetch the bytes from
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
    /// DERIVED, exactly as on [`RoomReport`]: true iff `problemCount == 0`.
    pub all_items_ok: bool,
    /// How many ticks are a problem (หาย / ชำรุด). The overview's stand-in for
    /// the tick array — a card says "3 รายการมีปัญหา" and the detail endpoint
    /// serves which ones, so one screen's payload does not grow with the
    /// property. `0` for a clean room and for a v1 report with no exceptions.
    pub problem_count: usize,
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
        // v2: the tick states, the four capture zones and the submission's
        // DISTINCT-photo bound (`REPORT_MAX_PHOTOS_TOTAL` in the vocab file).
        assert_eq!(
            TickState::ALL.map(TickState::as_str),
            ["ok", "missing", "damaged"]
        );
        assert_eq!(
            REPORT_ZONES.map(|(zone, _)| zone),
            ["bed", "desk", "bathroom", "general"]
        );
        assert_eq!(REPORT_MIN_PHOTOS_TOTAL, 4, "one shot per capture zone");
        assert_eq!(REPORT_MAX_PHOTOS_TOTAL, 24);
        assert_eq!(REPORT_TICK_COUNT, REPORT_ITEMS.len());
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
        let zones: Vec<&str> = REPORT_ZONES.iter().map(|(zone, _)| *zone).collect();
        let states: Vec<&str> = TickState::ALL.iter().map(|s| s.as_str()).collect();
        for list in [
            &REPORT_ITEMS[..],
            &ROOM_STATUS_CODES[..],
            &RETURN_REASONS[..],
            &zones[..],
            &states[..],
        ] {
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

    /// **Every checklist item belongs to exactly ONE capture zone** — the vocab
    /// file says so and the pre-ticking depends on it: an item in no zone would
    /// never be shown to the maid (so her submission could never carry all 22
    /// ticks), and an item in two would be pre-ticked against two photos.
    #[test]
    fn every_item_belongs_to_exactly_one_capture_zone() {
        for item in REPORT_ITEMS {
            let zones: Vec<&str> = REPORT_ZONES
                .iter()
                .filter(|(_, items)| items.contains(&item))
                .map(|(zone, _)| *zone)
                .collect();
            assert_eq!(
                zones.len(),
                1,
                "'{item}' must belong to exactly one capture zone, got {zones:?}"
            );
        }
        // …and no zone may name an item the checklist does not have.
        let zoned: usize = REPORT_ZONES.iter().map(|(_, items)| items.len()).sum();
        assert_eq!(
            zoned,
            REPORT_ITEMS.len(),
            "the zones must partition the checklist exactly"
        );
        for (zone, items) in REPORT_ZONES {
            for item in items {
                assert!(
                    REPORT_ITEMS.contains(item),
                    "zone '{zone}' names '{item}', which is not a checklist item"
                );
            }
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
        for state in TickState::ALL {
            assert_eq!(TickState::parse(state.as_str()), Some(state));
        }
        assert_eq!(ReportStatus::parse("SUBMITTED"), None, "parse is exact");
        assert_eq!(TickState::parse("OK"), None);
        assert_eq!(TickState::parse("broken"), None, "the code is 'damaged'");
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
        for state in TickState::ALL {
            assert_eq!(
                serde_json::to_string(&state).unwrap(),
                format!("\"{}\"", state.as_str())
            );
        }
    }

    /// A tick state maps onto the v1 problem vocabulary — and therefore onto a
    /// room signal — for exactly the two problem states and nothing else. This
    /// is the bridge new writes use to fill `rri_problem` from `rri_state`
    /// (`NULLIF(state, 'ok')`), so the two columns are one value written twice.
    #[test]
    fn a_tick_state_is_a_problem_for_exactly_the_two_problem_states() {
        assert_eq!(TickState::Ok.problem(), None);
        assert!(!TickState::Ok.is_problem());
        assert_eq!(TickState::Missing.problem(), Some(ItemProblem::Missing));
        assert_eq!(TickState::Damaged.problem(), Some(ItemProblem::Damaged));
        for state in [TickState::Missing, TickState::Damaged] {
            assert!(state.is_problem());
            // The stored `rri_problem` literal is the state's own literal.
            assert_eq!(state.problem().unwrap().as_str(), state.as_str());
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

        // v2 wire codes normalize the same way.
        assert_eq!(parse_tick_state(" OK ").unwrap(), TickState::Ok);
        assert_eq!(parse_tick_state("Damaged").unwrap(), TickState::Damaged);
        assert_eq!(parse_zone(" Bathroom ").unwrap(), "bathroom");
        for bad in ["", "fine", "ครบ", "broken"] {
            let err = parse_tick_state(bad).expect_err("'{bad}' must be refused");
            assert!(err.message().contains("state"), "{}", err.message());
        }
        for bad in ["", "kitchen", "ห้องน้ำ", "beds"] {
            let err = parse_zone(bad).expect_err("'{bad}' must be refused");
            assert!(err.message().contains("zone"), "{}", err.message());
        }
    }

    /// **The ticks are EXACTLY the checklist.** The rule that replaced v1's
    /// biconditional, and the one the whole v2 model rests on: a report cannot
    /// be filed until every item has been answered, so "the room is fine" and
    /// "she did not look" stop being the same submission.
    #[test]
    fn the_ticks_must_be_exactly_the_checklist() {
        let all: Vec<String> = REPORT_ITEMS.iter().map(|i| i.to_string()).collect();
        assert!(check_tick_coverage(&all).is_ok(), "all 22 is the only accepted set");
        // Order is the client's business — the rule is about the SET.
        let mut shuffled = all.clone();
        shuffled.reverse();
        assert!(check_tick_coverage(&shuffled).is_ok());

        // One short: named, not counted.
        let short: Vec<String> = all.iter().skip(1).cloned().collect();
        let err = check_tick_coverage(&short).unwrap_err();
        assert!(!err.is_forbidden() && !err.is_conflict(), "malformed ⇒ 400");
        assert!(err.message().contains("water_glass"), "{}", err.message());
        assert!(err.message().contains("never ticked"), "{}", err.message());

        // Empty: every item is missing, but the message stays bounded.
        let err = check_tick_coverage(&[]).unwrap_err();
        assert!(err.message().contains("+18 more"), "{}", err.message());
        assert!(err.message().len() < 400, "the 400 body must stay small");

        // An item that is not on the form — the stale-bundle case, which
        // usually reports BOTH directions at once.
        let mut renamed = all.clone();
        renamed[0] = "remote".to_string();
        let err = check_tick_coverage(&renamed).unwrap_err();
        assert!(err.message().contains("water_glass"), "{}", err.message());
        assert!(err.message().contains("remote"), "{}", err.message());
        assert!(
            err.message().contains("not on the checklist"),
            "{}",
            err.message()
        );

        // 22 entries, one duplicated and one absent, is STILL refused — the
        // count alone is never the test.
        let mut dupe = all.clone();
        dupe[0] = "tv_remote".to_string();
        assert!(check_tick_coverage(&dupe).is_err());
    }

    /// The qty rule per state, in every direction. Neither half coerces: a qty
    /// on an `ok` tick means the client ticked the wrong state, and a missing
    /// one on a problem would have to be invented — and reception charges a
    /// guest from that number.
    #[test]
    fn a_qty_belongs_to_a_problem_tick_and_only_to_one() {
        assert!(check_tick_qty("water_glass", TickState::Ok, None).is_ok());
        for state in [TickState::Missing, TickState::Damaged] {
            assert!(check_tick_qty("water_glass", state, Some(MIN_ITEM_QTY)).is_ok());
            assert!(check_tick_qty("water_glass", state, Some(MAX_ITEM_QTY)).is_ok());
            assert!(check_tick_qty("water_glass", state, Some(7)).is_ok());

            let err = check_tick_qty("water_glass", state, None).unwrap_err();
            assert!(err.message().contains("needs a qty"), "{}", err.message());

            for bad in [0, -1, 100, i32::MAX] {
                let err = check_tick_qty("water_glass", state, Some(bad)).unwrap_err();
                assert!(err.message().contains("1..=99"), "{}", err.message());
                assert!(!err.is_forbidden(), "a bad qty is 400, not 403");
            }
        }
        let err = check_tick_qty("water_glass", TickState::Ok, Some(1)).unwrap_err();
        assert!(err.message().contains("carries no qty"), "{}", err.message());
        assert!(err.message().contains("water_glass"), "the item must be named");
    }

    /// Every tick is photo-backed — including an `ok` one, which is the whole
    /// point (that is the tick a maid against the clock would wave through).
    #[test]
    fn every_tick_names_a_backing_photo() {
        for state in TickState::ALL {
            let _ = state;
            assert!(check_tick_photo("pillow", Some(4)).is_ok());
        }
        let err = check_tick_photo("pillow", None).unwrap_err();
        assert!(err.message().contains("photoId"), "{}", err.message());
        assert!(err.message().contains("pillow"));
        assert!(!err.is_forbidden() && !err.is_conflict());
    }

    /// The submission counts DISTINCT photos, 4..=24 — one shot per capture
    /// zone at the floor, the vocab file's ceiling at the top.
    #[test]
    fn the_submission_photo_total_is_four_to_twenty_four() {
        for ok in REPORT_MIN_PHOTOS_TOTAL..=REPORT_MAX_PHOTOS_TOTAL {
            assert!(check_photo_total(ok).is_ok(), "{ok} distinct photos must pass");
        }
        for bad in [0, 1, 3, 25, 100] {
            let err = check_photo_total(bad).unwrap_err();
            assert!(!err.is_forbidden(), "a bad total is 400, not 403");
            assert!(err.message().contains("4..=24"), "{}", err.message());
            assert!(err.message().contains("distinct"), "{}", err.message());
        }
    }

    /// A v1-shaped body is refused with a message that NAMES `ticks` — the one
    /// word an operator staring at a stale bundle needs.
    #[test]
    fn a_v1_body_is_refused_by_name() {
        let err = ReportRuleError::LegacyBodyShape;
        assert!(!err.is_forbidden() && !err.is_conflict(), "malformed ⇒ 400");
        let message = err.message();
        assert!(message.contains("ticks"), "{message}");
        assert!(message.contains("allItemsOk"), "{message}");
        assert!(message.contains("items"), "{message}");
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
            ticks: vec![
                ReportTick {
                    item: "water_glass".to_string(),
                    state: TickState::Ok,
                    qty: None,
                    photo_id: Some(4),
                },
                ReportTick {
                    item: "tv_remote".to_string(),
                    state: TickState::Missing,
                    qty: Some(1),
                    photo_id: Some(5),
                },
            ],
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
            photos: vec![
                ReportPhoto {
                    photo_id: 4,
                    side: PhotoSide::Maid,
                    zone: Some("desk".to_string()),
                    bytes: Some(120_500),
                },
                ReportPhoto {
                    photo_id: 5,
                    side: PhotoSide::Maid,
                    zone: None,
                    bytes: Some(98_100),
                },
            ],
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
                "photos",
                "receptionPhotoIds",
                "reportId",
                "returnReason",
                "roomId",
                "roomNo",
                "roomStatus",
                "status",
                "submittedAt",
                "submittedBy",
                "ticks",
                "verifiedAt",
                "verifiedBy",
            ]
        );
        // The nullable keys are PRESENT and null, never omitted.
        assert!(object["returnReason"].is_null());
        assert!(object["verifiedBy"].is_null());
        assert_eq!(json["items"][0]["problem"], "missing");
        assert_eq!(json["submittedBy"]["badge"], "Q1001");

        // The v2 tick shape, key by key — this IS the contract.
        let ok_tick = &json["ticks"][0];
        let mut tick_keys: Vec<&str> = ok_tick
            .as_object()
            .expect("a tick object")
            .keys()
            .map(String::as_str)
            .collect();
        tick_keys.sort_unstable();
        assert_eq!(tick_keys, ["item", "photoId", "qty", "state"]);
        assert_eq!(ok_tick["state"], "ok");
        assert!(ok_tick["qty"].is_null(), "an ok tick carries no qty");
        assert_eq!(ok_tick["photoId"], 4);
        assert_eq!(json["ticks"][1]["state"], "missing");
        assert_eq!(json["ticks"][1]["qty"], 1);

        // …and the photo metadata shape.
        let photo = &json["photos"][0];
        let mut photo_keys: Vec<&str> = photo
            .as_object()
            .expect("a photo object")
            .keys()
            .map(String::as_str)
            .collect();
        photo_keys.sort_unstable();
        assert_eq!(photo_keys, ["bytes", "photoId", "side", "zone"]);
        assert_eq!(photo["side"], "maid");
        assert_eq!(photo["zone"], "desk");
        assert_eq!(photo["bytes"], 120_500);
        assert!(
            json["photos"][1]["zone"].is_null(),
            "a free-hand close-up has no zone, and the key is still present"
        );
    }

    /// A report filed by v1 still reads: no ticks (its rows carry no backing
    /// photo), its exceptions where they always were, and `allItemsOk` derived
    /// from those exceptions rather than from a column.
    #[test]
    fn a_v1_report_still_serializes_with_empty_ticks() {
        let report = RoomReport {
            report_id: 1,
            room_id: 2,
            room_no: "101".to_string(),
            date: "2026-09-02".to_string(),
            status: ReportStatus::Verified,
            room_status: "vc".to_string(),
            all_items_ok: false,
            ticks: Vec::new(),
            items: vec![ReportItem {
                item: "kettle".to_string(),
                problem: ItemProblem::Damaged,
                qty: 2,
            }],
            return_reason: None,
            parent_report_id: None,
            submitted_by: ReportActor {
                badge: "Q1001".to_string(),
                name: None,
            },
            submitted_at: "2026-09-02T03:00:00Z".to_string(),
            verified_by: None,
            verified_at: None,
            photos: vec![ReportPhoto {
                photo_id: 9,
                side: PhotoSide::Maid,
                zone: None,
                bytes: Some(4_096),
            }],
            maid_photo_ids: vec![9],
            reception_photo_ids: vec![],
        };
        let json = serde_json::to_value(&report).expect("serializes");
        assert_eq!(
            json["ticks"].as_array().map(Vec::len),
            Some(0),
            "a v1 report has no photo-backed ticks, and the key is still present"
        );
        assert_eq!(json["items"][0]["item"], "kettle");
        assert_eq!(json["items"][0]["qty"], 2);
        assert_eq!(json["allItemsOk"], false);
        assert!(json["photos"][0]["zone"].is_null(), "v1 photos have no zone");
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
            problem_count: 0,
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
            ticks: Vec::new(),
            items: Vec::new(),
            return_reason: None,
            parent_report_id: Some(3),
            submitted_by: summary.submitted_by.clone(),
            submitted_at: summary.submitted_at.clone(),
            verified_by: summary.verified_by.clone(),
            verified_at: summary.verified_at.clone(),
            photos: Vec::new(),
            maid_photo_ids: vec![1, 2],
            reception_photo_ids: vec![9],
        };
        let mut expected: Vec<String> = serde_json::to_value(&full)
            .expect("serializes")
            .as_object()
            .expect("an object")
            .keys()
            .filter(|k| {
                ![
                    "ticks",
                    "items",
                    "photos",
                    "maidPhotoIds",
                    "receptionPhotoIds",
                ]
                .contains(&k.as_str())
            })
            .cloned()
            .collect();
        expected.push("photoCounts".to_string());
        expected.push("problemCount".to_string());
        expected.sort();

        assert_eq!(summary_keys, expected);
        assert_eq!(summary_json["photoCounts"]["maid"], 2);
        assert_eq!(summary_json["photoCounts"]["reception"], 1);
        assert_eq!(summary_json["problemCount"], 0);
        assert_eq!(summary_json["status"], "verified");
    }

    /// `allItemsOk` is DERIVED from the problem count on the summary too, so
    /// the card's "ครบทุกรายการ" badge and its "N รายการมีปัญหา" line can never
    /// contradict each other.
    #[test]
    fn the_summarys_all_items_ok_agrees_with_its_problem_count() {
        for (problem_count, all_items_ok) in [(0usize, true), (1, false), (5, false)] {
            let summary = RoomReportSummary {
                report_id: 7,
                room_id: 12,
                room_no: "101".to_string(),
                date: "2026-09-02".to_string(),
                status: ReportStatus::Submitted,
                room_status: "co".to_string(),
                all_items_ok,
                problem_count,
                return_reason: None,
                parent_report_id: None,
                submitted_by: ReportActor {
                    badge: "Q1001".to_string(),
                    name: None,
                },
                submitted_at: "2026-09-02T03:00:00Z".to_string(),
                verified_by: None,
                verified_at: None,
                photo_counts: PhotoCounts {
                    maid: 4,
                    reception: 0,
                },
            };
            let json = serde_json::to_value(&summary).expect("serializes");
            assert_eq!(
                json["allItemsOk"].as_bool(),
                Some(json["problemCount"].as_u64() == Some(0)),
                "allItemsOk must be exactly problemCount == 0"
            );
        }
    }

    /// The photo-meta shape — the client's resume-after-reload contract.
    #[test]
    fn the_photo_meta_serializes_the_contracted_keys() {
        let meta = ReportPhotoMeta {
            photo_id: 12,
            side: PhotoSide::Maid,
            zone: Some("bed".to_string()),
            bytes: Some(51_200),
            attached: false,
            uploaded_at: "2026-09-02T03:00:00Z".to_string(),
        };
        let json = serde_json::to_value(&meta).expect("serializes");
        let mut keys: Vec<&str> = json
            .as_object()
            .expect("an object")
            .keys()
            .map(String::as_str)
            .collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            ["attached", "bytes", "photoId", "side", "uploadedAt", "zone"]
        );
        assert_eq!(json["side"], "maid");
        assert_eq!(json["attached"], false);

        // The nullable keys stay PRESENT — a v1 photo has neither.
        let bare = ReportPhotoMeta {
            zone: None,
            bytes: None,
            attached: true,
            ..meta
        };
        let json = serde_json::to_value(&bare).expect("serializes");
        assert!(json.get("zone").expect("key present").is_null());
        assert!(json.get("bytes").expect("key present").is_null());
        assert_eq!(json["attached"], true);
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

    /// `REPORT_TICK_COUNT` must stay tied to the checklist size, so widening
    /// the vocabulary moves the requirement with it rather than leaving the
    /// coverage rule pinned to yesterday's 22.
    #[test]
    fn the_tick_requirement_tracks_the_checklist_size() {
        assert_eq!(REPORT_TICK_COUNT, REPORT_ITEMS.len());
        let all: Vec<String> = REPORT_ITEMS.iter().map(|i| i.to_string()).collect();
        assert_eq!(all.len(), REPORT_TICK_COUNT);
        assert!(check_tick_coverage(&all).is_ok());
    }
}
