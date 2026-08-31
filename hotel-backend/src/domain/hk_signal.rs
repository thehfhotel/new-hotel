//! Room signals — the canned reception ⇄ maid vocabulary and its rules.
//!
//! Per `docs/adr/0008-room-signals-not-chat.md` and `CONTEXT.md`
//! §Housekeeping. PURE: no `sqlx`, no `axum`, no I/O — every rule in this file
//! is a total function over its inputs and is unit-tested below, which is what
//! lets `service::hk_signals` and `routes::hk` share one enforcement point
//! instead of two drifting copies.
//!
//! ## The wire codes are mirrored, not owned, here
//!
//! `app/hk/signal-vocab.ts` is the ONE place a signal type is spelled for the
//! frontend; the constants below mirror those codes and MUST be kept in
//! lock-step with it. The database deliberately does not mirror them at all —
//! `ht_hk_room_signals.sig_type` is `TEXT` with no CHECK (migration 089,
//! inheriting migration 088's rationale), so extending the canned vocabulary —
//! which ADR 0008 names "the sanctioned cheap change" — stays a constant edit
//! that ships with the frontend rather than an `ALTER` on a live table at two
//! sites.
//!
//! ## What is NOT here
//!
//! Thai labels. They belong to the client, which knows how to render them; the
//! wire codes stay ASCII snake_case. And free text of any kind — ADR 0008
//! rejected "canned + optional note" explicitly, so there is no field for one.

use serde::{Deserialize, Serialize};

// ============================================================================
// The canned vocabulary (mirrors `app/hk/signal-vocab.ts`)
// ============================================================================

/// ขอเช็คห้อง — the checkout coordinator. The one type whose completion is an
/// ANSWER rather than a tap, and the only type the escalation valve may push
/// for (ADR 0008 §Decision 3).
pub const ROOM_CHECK: &str = "room_check";
/// ทำห้องนี้ก่อน
pub const PRIORITY_CLEAN: &str = "priority_clean";
/// แขกขอผ้าเพิ่ม
pub const DELIVER_LINEN: &str = "deliver_linen";
/// งดทำห้องนี้
pub const SKIP_ROOM: &str = "skip_room";
/// แขกเช็คเอาท์แล้ว
pub const CHECKED_OUT: &str = "checked_out";

/// ลูกค้ายังอยู่ในห้อง
pub const GUEST_IN_ROOM: &str = "guest_in_room";
/// พบของลืมในห้อง
pub const FOUND_BELONGINGS: &str = "found_belongings";
/// มีของหาย — a guest-accountability signal (CONTEXT.md §Housekeeping).
pub const ITEM_MISSING: &str = "item_missing";
/// มีของเสียหาย — a guest-accountability signal.
pub const ITEM_DAMAGED: &str = "item_damaged";

/// Desk→maid types, in display order (mirrors `DESK_SIGNALS`).
pub const DESK_TO_MAID_TYPES: [&str; 5] = [
    ROOM_CHECK,
    PRIORITY_CLEAN,
    DELIVER_LINEN,
    SKIP_ROOM,
    CHECKED_OUT,
];

/// Maid→desk types, in display order (mirrors `MAID_SIGNALS`).
pub const MAID_TO_DESK_TYPES: [&str; 4] =
    [GUEST_IN_ROOM, FOUND_BELONGINGS, ITEM_MISSING, ITEM_DAMAGED];

/// The two problems a `room_check` answer may carry (mirrors
/// `ROOM_CHECK_PROBLEMS`). Both are maid→desk types, which is exactly why a
/// `problems` answer can spawn them as standing child signals with no special
/// case: they are ordinary signals from the maid, with a parent.
pub const ROOM_CHECK_PROBLEM_TYPES: [&str; 2] = [ITEM_MISSING, ITEM_DAMAGED];

/// The cleaning-urgency signals a maid's เสร็จแล้ว report auto-completes
/// (CONTEXT.md §Housekeeping: "ทำห้องนี้ก่อน, แขกเช็คเอาท์แล้ว").
///
/// `room_check` is deliberately NOT in this list even though the desk raises it
/// around a checkout: its completion is a judgement the maid must state
/// (เคลียร์ / มีของหาย / มีของเสียหาย), and auto-closing it on a cleaning tap
/// would silently answer เคลียร์ on her behalf while a guest is at the counter.
pub const CLEAN_REPORT_AUTO_COMPLETE_TYPES: [&str; 2] = [PRIORITY_CLEAN, CHECKED_OUT];

// ============================================================================
// Structural enums
// ============================================================================

/// Which way a signal travels. Structural, and CHECKed in the DB — the role
/// rules are written over exactly these two values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalDirection {
    DeskToMaid,
    MaidToDesk,
}

impl SignalDirection {
    /// The `sig_direction` literal. PURE.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeskToMaid => "desk_to_maid",
            Self::MaidToDesk => "maid_to_desk",
        }
    }

    /// Parse a stored / wire literal. EXACT — an unrecognised value is `None`,
    /// never coerced onto a direction.
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "desk_to_maid" => Some(Self::DeskToMaid),
            "maid_to_desk" => Some(Self::MaidToDesk),
            _ => None,
        }
    }

    /// The closed type list for this direction.
    pub fn types(self) -> &'static [&'static str] {
        match self {
            Self::DeskToMaid => &DESK_TO_MAID_TYPES,
            Self::MaidToDesk => &MAID_TO_DESK_TYPES,
        }
    }
}

/// Lifecycle position. Structural, and CHECKed in the DB.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalStatus {
    Open,
    Acked,
    Done,
    Cancelled,
}

impl SignalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Acked => "acked",
            Self::Done => "done",
            Self::Cancelled => "cancelled",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "open" => Some(Self::Open),
            "acked" => Some(Self::Acked),
            "done" => Some(Self::Done),
            "cancelled" => Some(Self::Cancelled),
            _ => None,
        }
    }

    /// Is this signal still on the boards? The list endpoints' predicate, and
    /// the partial index's, expressed once.
    pub fn is_live(self) -> bool {
        matches!(self, Self::Open | Self::Acked)
    }
}

/// How a signal reached `done`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalDoneSource {
    /// Someone pressed done on the board.
    Tap,
    /// A maid's เสร็จแล้ว cleaning report auto-completed it.
    CleanReport,
    /// The ขอเช็คห้อง answer completed the check.
    RoomCheckAnswer,
}

impl SignalDoneSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Tap => "tap",
            Self::CleanReport => "clean_report",
            Self::RoomCheckAnswer => "room_check_answer",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "tap" => Some(Self::Tap),
            "clean_report" => Some(Self::CleanReport),
            "room_check_answer" => Some(Self::RoomCheckAnswer),
            _ => None,
        }
    }
}

/// The ขอเช็คห้อง answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoomCheckOutcome {
    /// เคลียร์ — settle now.
    Clear,
    /// One or both of มีของหาย / มีของเสียหาย, which then stand as
    /// guest-accountability child signals.
    Problems,
}

impl RoomCheckOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::Problems => "problems",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "clear" => Some(Self::Clear),
            "problems" => Some(Self::Problems),
            _ => None,
        }
    }
}

/// Which side of the conversation the caller is on.
///
/// NOT a new permission concept: on `/hk` it is derived from the single
/// boolean the Access middleware already resolved
/// ([`crate::middleware::hk_access::HkIdentity::can_report`] — `true` = maid,
/// `false` = read-only reception viewer), and on the desk surface it is
/// constantly [`SignalRole::Desk`]. Deriving it in ONE place
/// ([`SignalRole::from_can_report`]) is what keeps the two surfaces from
/// growing separate answers to "who is this".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalRole {
    /// A housekeeping identity: sends maid→desk types, acts on desk→maid ones.
    Maid,
    /// Reception — the `/hk` viewer or the desk surface: sends desk→maid
    /// types, acts on maid→desk ones.
    Desk,
}

impl SignalRole {
    /// `HkIdentity::can_report` → role. The ONE derivation. PURE.
    pub fn from_can_report(can_report: bool) -> Self {
        if can_report {
            Self::Maid
        } else {
            Self::Desk
        }
    }

    /// The direction this role SENDS.
    pub fn sends(self) -> SignalDirection {
        match self {
            Self::Maid => SignalDirection::MaidToDesk,
            Self::Desk => SignalDirection::DeskToMaid,
        }
    }

    /// The direction this role ACTS ON (acks / dones / answers) — the other
    /// one. "Nobody acts on their own direction's signals except
    /// cancel-own-while-open" (the contract) is exactly this asymmetry.
    pub fn acts_on(self) -> SignalDirection {
        match self {
            Self::Maid => SignalDirection::DeskToMaid,
            Self::Desk => SignalDirection::MaidToDesk,
        }
    }

    /// For logs and error text.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Maid => "maid",
            Self::Desk => "desk",
        }
    }
}

/// What a caller is trying to do to an existing signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalAction {
    Ack,
    Done,
    Cancel,
    Answer,
}

impl SignalAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ack => "ack",
            Self::Done => "done",
            Self::Cancel => "cancel",
            Self::Answer => "answer",
        }
    }
}

// ============================================================================
// The rules
// ============================================================================

/// Every way a signal command can be refused, WITH the class of refusal.
///
/// The class is carried on the variant rather than decided at the HTTP edge so
/// the maid surface and the desk surface cannot answer the same refusal with
/// different status codes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalRuleError {
    /// A type that is in NEITHER direction's vocabulary → 400.
    UnknownType(String),
    /// A well-formed type belonging to the OTHER role's direction → 403. It is
    /// a permission fact ("you may not speak for the other side"), the same
    /// class as `routes::hk::REPORT_NOT_PERMITTED_ERROR`, not a malformed body.
    WrongDirectionForRole {
        role: SignalRole,
        signal_type: String,
    },
    /// Acting on a signal travelling in the caller's OWN direction → 403.
    NotYourDirection {
        role: SignalRole,
        direction: SignalDirection,
    },
    /// `done` tapped on a `room_check` → 400, pointing at the answer endpoint.
    RoomCheckNeedsAnswer,
    /// `answer` aimed at anything but a `room_check` → 400.
    AnswerOnlyOnRoomCheck,
    /// The signal is not in a status this action can move it from → 400 (see
    /// the module note in `service::hk_signals` on 400-vs-409).
    InvalidTransition {
        action: SignalAction,
        from: SignalStatus,
    },
    /// `outcome: "problems"` with an empty `problems` list → 400.
    ProblemsRequired,
    /// A `problems` entry outside [`ROOM_CHECK_PROBLEM_TYPES`] → 400.
    UnknownProblem(String),
}

impl SignalRuleError {
    /// Is this a permission refusal (403) rather than a bad request (400)?
    /// PURE and total — the single place the split is decided.
    pub fn is_forbidden(&self) -> bool {
        matches!(
            self,
            Self::WrongDirectionForRole { .. } | Self::NotYourDirection { .. }
        )
    }

    /// Operator/caller-facing message. ASCII and machine-greppable: these reach
    /// a client that renders its own Thai copy from the endpoint, unlike the
    /// `/hk` gate messages which a maid reads verbatim.
    pub fn message(&self) -> String {
        match self {
            Self::UnknownType(t) => format!(
                "unknown signal type '{t}' (expected one of {DESK_TO_MAID_TYPES:?} or {MAID_TO_DESK_TYPES:?})"
            ),
            Self::WrongDirectionForRole { role, signal_type } => format!(
                "signal type '{}' belongs to the {} direction; a {} identity may only send {:?}",
                signal_type,
                match role {
                    SignalRole::Maid => SignalDirection::DeskToMaid,
                    SignalRole::Desk => SignalDirection::MaidToDesk,
                }
                .as_str(),
                role.as_str(),
                role.sends().types()
            ),
            Self::NotYourDirection { role, direction } => format!(
                "a {} identity may not act on a {} signal (only cancel its own while open)",
                role.as_str(),
                direction.as_str()
            ),
            Self::RoomCheckNeedsAnswer => format!(
                "'{ROOM_CHECK}' is completed by answering it, not by a done tap — \
                 POST /signals/{{id}}/answer with {{\"outcome\":\"clear\"}} or \
                 {{\"outcome\":\"problems\",\"problems\":[…]}}"
            ),
            Self::AnswerOnlyOnRoomCheck => {
                format!("only a '{ROOM_CHECK}' signal can be answered")
            }
            Self::InvalidTransition { action, from } => format!(
                "cannot {} a signal that is already '{}'",
                action.as_str(),
                from.as_str()
            ),
            Self::ProblemsRequired => format!(
                "outcome 'problems' requires at least one of {ROOM_CHECK_PROBLEM_TYPES:?}"
            ),
            Self::UnknownProblem(p) => format!(
                "unknown problem '{p}' (expected one of {ROOM_CHECK_PROBLEM_TYPES:?})"
            ),
        }
    }
}

/// The direction a signal type belongs to, or `None` for a code in neither
/// vocabulary. PURE.
pub fn direction_for_type(signal_type: &str) -> Option<SignalDirection> {
    if DESK_TO_MAID_TYPES.contains(&signal_type) {
        Some(SignalDirection::DeskToMaid)
    } else if MAID_TO_DESK_TYPES.contains(&signal_type) {
        Some(SignalDirection::MaidToDesk)
    } else {
        None
    }
}

/// Normalize a type off the wire the way `routes::hk::parse_cleaning_status`
/// forgives a status: trim + lowercase, nothing else. PURE.
pub fn normalize_type(raw: &str) -> String {
    raw.trim().to_lowercase()
}

/// May this role raise this type, and if so in which direction? PURE.
///
/// Two distinct refusals on purpose: an unknown code is a malformed request
/// (400), while a well-formed code from the other side's list is a permission
/// refusal (403) — the client sent something real that this identity may not
/// say.
pub fn direction_for_role_type(
    role: SignalRole,
    signal_type: &str,
) -> Result<SignalDirection, SignalRuleError> {
    let Some(direction) = direction_for_type(signal_type) else {
        return Err(SignalRuleError::UnknownType(signal_type.to_string()));
    };
    if direction != role.sends() {
        return Err(SignalRuleError::WrongDirectionForRole {
            role,
            signal_type: signal_type.to_string(),
        });
    }
    Ok(direction)
}

/// The whole transition table, as one total function. PURE.
///
/// Returns the status the signal moves to. Ordered so the most specific
/// refusal wins: role first ("you may not touch this at all"), then the
/// room_check carve-out ("not with a tap"), then the status guard.
pub fn next_status(
    action: SignalAction,
    role: SignalRole,
    direction: SignalDirection,
    signal_type: &str,
    from: SignalStatus,
) -> Result<SignalStatus, SignalRuleError> {
    match action {
        // Cancel is the ONE thing you do to your OWN direction, and only
        // while nobody has taken it.
        SignalAction::Cancel => {
            if direction != role.sends() {
                return Err(SignalRuleError::NotYourDirection { role, direction });
            }
            match from {
                SignalStatus::Open => Ok(SignalStatus::Cancelled),
                other => Err(SignalRuleError::InvalidTransition {
                    action,
                    from: other,
                }),
            }
        }
        SignalAction::Ack => {
            if direction != role.acts_on() {
                return Err(SignalRuleError::NotYourDirection { role, direction });
            }
            match from {
                SignalStatus::Open => Ok(SignalStatus::Acked),
                other => Err(SignalRuleError::InvalidTransition {
                    action,
                    from: other,
                }),
            }
        }
        SignalAction::Done => {
            if direction != role.acts_on() {
                return Err(SignalRuleError::NotYourDirection { role, direction });
            }
            if signal_type == ROOM_CHECK {
                return Err(SignalRuleError::RoomCheckNeedsAnswer);
            }
            match from {
                SignalStatus::Open | SignalStatus::Acked => Ok(SignalStatus::Done),
                other => Err(SignalRuleError::InvalidTransition {
                    action,
                    from: other,
                }),
            }
        }
        SignalAction::Answer => {
            if direction != role.acts_on() {
                return Err(SignalRuleError::NotYourDirection { role, direction });
            }
            if signal_type != ROOM_CHECK {
                return Err(SignalRuleError::AnswerOnlyOnRoomCheck);
            }
            match from {
                SignalStatus::Open | SignalStatus::Acked => Ok(SignalStatus::Done),
                other => Err(SignalRuleError::InvalidTransition {
                    action,
                    from: other,
                }),
            }
        }
    }
}

/// Normalize + validate the `problems` list of a `problems` answer. PURE.
///
/// Deduplicates on the NORMALIZED code (so `"Item_Missing"` and
/// `"item_missing"` cannot land as two child signals for one problem), and
/// preserves [`ROOM_CHECK_PROBLEM_TYPES`] order so the spawned children are
/// always in the same order regardless of how the client listed them.
pub fn parse_problems(raw: &[String]) -> Result<Vec<&'static str>, SignalRuleError> {
    let mut wanted: Vec<&'static str> = Vec::new();
    for entry in raw {
        let code = normalize_type(entry);
        let matched = ROOM_CHECK_PROBLEM_TYPES
            .iter()
            .find(|known| **known == code)
            .ok_or_else(|| SignalRuleError::UnknownProblem(entry.clone()))?;
        if !wanted.contains(matched) {
            wanted.push(matched);
        }
    }
    if wanted.is_empty() {
        return Err(SignalRuleError::ProblemsRequired);
    }
    // Canonical (display) order, not arrival order.
    Ok(ROOM_CHECK_PROBLEM_TYPES
        .iter()
        .copied()
        .filter(|known| wanted.contains(known))
        .collect())
}

/// Is this signal eligible for the 2-minute LINE escalation? PURE — the whole
/// predicate ADR 0008 §Decision 3 closes, in one place, so the scheduler's SQL
/// and this function cannot drift into disagreeing about "which signals push".
///
/// The SQL already filters on type / status / `sig_escalated_at IS NULL` / age;
/// this function restates all of it because a partial restatement is how the
/// quota stop gets skipped when someone later widens the query.
pub fn escalation_eligible(
    signal_type: &str,
    status: SignalStatus,
    age_seconds: i64,
    already_escalated: bool,
    escalations_this_month: i64,
    monthly_cap: i64,
) -> bool {
    signal_type == ROOM_CHECK
        && status == SignalStatus::Open
        && age_seconds > ESCALATION_AGE_SECONDS
        && !already_escalated
        && escalations_this_month < monthly_cap
}

/// How long a `room_check` may sit unacked before the escalation valve opens.
/// Two minutes (ADR 0008 §Decision 3) — the guest is at the counter.
pub const ESCALATION_AGE_SECONDS: i64 = 120;

/// Default `HK_ESCALATION_MONTHLY_CAP`. ADR 0008's "hard stop (~150)", chosen
/// against the LINE free tier's ~200/month floor so a runaway month cannot
/// silently spend into the ฿1,200 tier.
pub const DEFAULT_ESCALATION_MONTHLY_CAP: i64 = 150;

// ============================================================================
// The wire shape
// ============================================================================

/// Who did something, as every surface renders it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalActor {
    /// Verified HF ID badge (or the desk's operator label). Never client-typed.
    pub badge: String,
    /// Display-name snapshot; `null` when the identity carries none.
    pub name: Option<String>,
}

/// One signal, exactly as `app/hk/signal-vocab.ts`'s `RoomSignal` interface
/// spells it — camelCase on the wire, `type` as a bare `type` key.
///
/// The optional fields are emitted as explicit `null` rather than omitted
/// (`?: T | null` in the TS interface accepts both, and always-present keys
/// make the shape self-describing in a `curl -N /api/hk/events` trace).
///
/// `Deserialize` as well as `Serialize` because the DTO travels INSIDE a
/// [`crate::outbox::event::DomainEvent`] through `event_log` and
/// `pg_notify` — the SSE relay reads it back out on the way to the maid's
/// stream, so a serialize-only shape would need a second parallel struct.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomSignal {
    pub signal_id: i64,
    pub room_id: i32,
    pub room_no: String,
    pub direction: SignalDirection,
    /// The canned code. A plain `String`, not an enum: server→client must be
    /// able to carry a type a deployed bundle predates rather than fail to
    /// serialize it (the client falls back to the raw code — `signalLabel`).
    #[serde(rename = "type")]
    pub signal_type: String,
    pub status: SignalStatus,
    pub outcome: Option<RoomCheckOutcome>,
    pub parent_id: Option<i64>,
    pub created_by: SignalActor,
    /// RFC 3339 UTC. The client formats it; nothing here is Thai-local (these
    /// are `TIMESTAMPTZ` columns our own app wrote, NOT the naive Thai-local
    /// datetimes the legacy MSSQL side stores).
    pub created_at: String,
    pub acked_by: Option<SignalActor>,
    pub acked_at: Option<String>,
    pub done_by: Option<SignalActor>,
    pub done_at: Option<String>,
    pub done_source: Option<SignalDoneSource>,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// The wire codes are a CONTRACT with `app/hk/signal-vocab.ts`. Pinned
    /// literally: a typo here is a signal the frontend renders as a raw code
    /// and the backend refuses, which is the kind of break no type system
    /// catches across a language boundary.
    #[test]
    fn wire_codes_match_the_frontend_vocabulary() {
        assert_eq!(
            DESK_TO_MAID_TYPES,
            [
                "room_check",
                "priority_clean",
                "deliver_linen",
                "skip_room",
                "checked_out"
            ]
        );
        assert_eq!(
            MAID_TO_DESK_TYPES,
            [
                "guest_in_room",
                "found_belongings",
                "item_missing",
                "item_damaged"
            ]
        );
        assert_eq!(ROOM_CHECK_PROBLEM_TYPES, ["item_missing", "item_damaged"]);
        assert_eq!(CLEAN_REPORT_AUTO_COMPLETE_TYPES, ["priority_clean", "checked_out"]);
    }

    /// Every problem type must also be a real maid→desk signal type — that is
    /// what lets a spawned child be an ORDINARY signal with a parent rather
    /// than a special row shape.
    #[test]
    fn every_problem_type_is_a_maid_to_desk_type() {
        for problem in ROOM_CHECK_PROBLEM_TYPES {
            assert!(
                MAID_TO_DESK_TYPES.contains(&problem),
                "{problem} must be a maid→desk type"
            );
        }
    }

    /// The two vocabularies must not overlap, or `direction_for_type` would be
    /// ambiguous and the role gate meaningless.
    #[test]
    fn the_two_directions_share_no_type() {
        for desk in DESK_TO_MAID_TYPES {
            assert!(!MAID_TO_DESK_TYPES.contains(&desk), "{desk} is in both lists");
        }
    }

    #[test]
    fn direction_for_type_is_exhaustive_and_exact() {
        for t in DESK_TO_MAID_TYPES {
            assert_eq!(direction_for_type(t), Some(SignalDirection::DeskToMaid));
        }
        for t in MAID_TO_DESK_TYPES {
            assert_eq!(direction_for_type(t), Some(SignalDirection::MaidToDesk));
        }
        for unknown in ["", "ROOM_CHECK", "room_checks", "chat", "room check"] {
            assert_eq!(direction_for_type(unknown), None, "{unknown:?}");
        }
    }

    #[test]
    fn normalize_forgives_only_case_and_surrounding_space() {
        assert_eq!(normalize_type("  Room_Check "), "room_check");
        assert_eq!(normalize_type("ROOM_CHECK"), "room_check");
        // NOT normalised: an inner space is a different code, not a typo we fix.
        assert_eq!(normalize_type("room check"), "room check");
    }

    // ---- role → direction ------------------------------------------------

    #[test]
    fn role_derivation_is_the_can_report_boolean() {
        assert_eq!(SignalRole::from_can_report(true), SignalRole::Maid);
        assert_eq!(SignalRole::from_can_report(false), SignalRole::Desk);
        assert_eq!(SignalRole::Maid.sends(), SignalDirection::MaidToDesk);
        assert_eq!(SignalRole::Maid.acts_on(), SignalDirection::DeskToMaid);
        assert_eq!(SignalRole::Desk.sends(), SignalDirection::DeskToMaid);
        assert_eq!(SignalRole::Desk.acts_on(), SignalDirection::MaidToDesk);
    }

    #[test]
    fn a_maid_may_only_raise_maid_to_desk_types() {
        for t in MAID_TO_DESK_TYPES {
            assert_eq!(
                direction_for_role_type(SignalRole::Maid, t),
                Ok(SignalDirection::MaidToDesk)
            );
        }
        for t in DESK_TO_MAID_TYPES {
            let err = direction_for_role_type(SignalRole::Maid, t).unwrap_err();
            assert!(
                matches!(err, SignalRuleError::WrongDirectionForRole { .. }),
                "{t} from a maid must be a role refusal, got {err:?}"
            );
            assert!(err.is_forbidden(), "{t} must be 403, not 400");
        }
    }

    #[test]
    fn a_desk_identity_may_only_raise_desk_to_maid_types() {
        for t in DESK_TO_MAID_TYPES {
            assert_eq!(
                direction_for_role_type(SignalRole::Desk, t),
                Ok(SignalDirection::DeskToMaid)
            );
        }
        for t in MAID_TO_DESK_TYPES {
            let err = direction_for_role_type(SignalRole::Desk, t).unwrap_err();
            assert!(matches!(err, SignalRuleError::WrongDirectionForRole { .. }));
            assert!(err.is_forbidden());
        }
    }

    /// An unknown code is a MALFORMED REQUEST, never a permission refusal —
    /// otherwise a client typo reads to the operator as an authorization
    /// problem.
    #[test]
    fn an_unknown_type_is_400_not_403() {
        let err = direction_for_role_type(SignalRole::Maid, "gossip").unwrap_err();
        assert_eq!(err, SignalRuleError::UnknownType("gossip".to_string()));
        assert!(!err.is_forbidden());
    }

    // ---- the transition table -------------------------------------------

    const MAID: SignalRole = SignalRole::Maid;
    const DESK: SignalRole = SignalRole::Desk;
    const D2M: SignalDirection = SignalDirection::DeskToMaid;
    const M2D: SignalDirection = SignalDirection::MaidToDesk;

    #[test]
    fn ack_moves_open_to_acked_for_the_acting_side_only() {
        assert_eq!(
            next_status(SignalAction::Ack, MAID, D2M, PRIORITY_CLEAN, SignalStatus::Open),
            Ok(SignalStatus::Acked)
        );
        assert_eq!(
            next_status(SignalAction::Ack, DESK, M2D, ITEM_MISSING, SignalStatus::Open),
            Ok(SignalStatus::Acked)
        );
        // Own direction ⇒ 403, for BOTH roles.
        for (role, dir, t) in [(MAID, M2D, ITEM_MISSING), (DESK, D2M, PRIORITY_CLEAN)] {
            let err =
                next_status(SignalAction::Ack, role, dir, t, SignalStatus::Open).unwrap_err();
            assert!(matches!(err, SignalRuleError::NotYourDirection { .. }));
            assert!(err.is_forbidden());
        }
    }

    #[test]
    fn ack_is_refused_from_every_non_open_status() {
        for from in [SignalStatus::Acked, SignalStatus::Done, SignalStatus::Cancelled] {
            assert_eq!(
                next_status(SignalAction::Ack, MAID, D2M, PRIORITY_CLEAN, from),
                Err(SignalRuleError::InvalidTransition {
                    action: SignalAction::Ack,
                    from
                })
            );
        }
    }

    #[test]
    fn done_accepts_open_and_acked_and_refuses_the_terminal_pair() {
        for from in [SignalStatus::Open, SignalStatus::Acked] {
            assert_eq!(
                next_status(SignalAction::Done, MAID, D2M, PRIORITY_CLEAN, from),
                Ok(SignalStatus::Done)
            );
        }
        for from in [SignalStatus::Done, SignalStatus::Cancelled] {
            assert!(matches!(
                next_status(SignalAction::Done, MAID, D2M, PRIORITY_CLEAN, from),
                Err(SignalRuleError::InvalidTransition { .. })
            ));
        }
    }

    /// The carve-out that stops a checkout being silently answered เคลียร์:
    /// a bare done tap on ขอเช็คห้อง is refused, from EVERY status it would
    /// otherwise have accepted, and the refusal names the answer endpoint.
    #[test]
    fn room_check_can_never_be_completed_by_a_done_tap() {
        for from in [
            SignalStatus::Open,
            SignalStatus::Acked,
            SignalStatus::Done,
            SignalStatus::Cancelled,
        ] {
            let err = next_status(SignalAction::Done, MAID, D2M, ROOM_CHECK, from).unwrap_err();
            assert_eq!(err, SignalRuleError::RoomCheckNeedsAnswer, "from {from:?}");
            assert!(!err.is_forbidden(), "the carve-out is a 400, not a 403");
        }
    }

    /// …but the role gate still outranks it: a DESK identity tapping done on
    /// its own ขอเช็คห้อง gets the permission refusal, not the "use answer"
    /// hint, because it may not complete that signal by any route.
    #[test]
    fn the_role_gate_outranks_the_room_check_carve_out() {
        let err = next_status(SignalAction::Done, DESK, D2M, ROOM_CHECK, SignalStatus::Open)
            .unwrap_err();
        assert!(matches!(err, SignalRuleError::NotYourDirection { .. }));
    }

    #[test]
    fn answer_completes_only_a_room_check_and_only_from_the_maid_side() {
        for from in [SignalStatus::Open, SignalStatus::Acked] {
            assert_eq!(
                next_status(SignalAction::Answer, MAID, D2M, ROOM_CHECK, from),
                Ok(SignalStatus::Done)
            );
        }
        assert_eq!(
            next_status(SignalAction::Answer, MAID, D2M, PRIORITY_CLEAN, SignalStatus::Open),
            Err(SignalRuleError::AnswerOnlyOnRoomCheck)
        );
        assert!(matches!(
            next_status(SignalAction::Answer, DESK, D2M, ROOM_CHECK, SignalStatus::Open),
            Err(SignalRuleError::NotYourDirection { .. })
        ));
        for from in [SignalStatus::Done, SignalStatus::Cancelled] {
            assert!(matches!(
                next_status(SignalAction::Answer, MAID, D2M, ROOM_CHECK, from),
                Err(SignalRuleError::InvalidTransition { .. })
            ));
        }
    }

    /// Cancel is the ONE inversion: it acts on the caller's OWN direction, and
    /// only while nobody has acked it.
    #[test]
    fn cancel_is_own_direction_and_open_only() {
        assert_eq!(
            next_status(SignalAction::Cancel, DESK, D2M, ROOM_CHECK, SignalStatus::Open),
            Ok(SignalStatus::Cancelled)
        );
        assert_eq!(
            next_status(SignalAction::Cancel, MAID, M2D, ITEM_MISSING, SignalStatus::Open),
            Ok(SignalStatus::Cancelled)
        );
        // Once acked it is somebody's work, not the sender's to withdraw.
        assert!(matches!(
            next_status(SignalAction::Cancel, DESK, D2M, ROOM_CHECK, SignalStatus::Acked),
            Err(SignalRuleError::InvalidTransition { .. })
        ));
        // Cancelling the OTHER side's signal is a permission refusal.
        let err = next_status(SignalAction::Cancel, MAID, D2M, ROOM_CHECK, SignalStatus::Open)
            .unwrap_err();
        assert!(matches!(err, SignalRuleError::NotYourDirection { .. }));
        assert!(err.is_forbidden());
    }

    // ---- problems --------------------------------------------------------

    #[test]
    fn problems_are_normalised_deduplicated_and_ordered() {
        assert_eq!(
            parse_problems(&["item_damaged".into(), "Item_Missing".into()]).unwrap(),
            vec!["item_missing", "item_damaged"],
            "canonical display order, not arrival order"
        );
        assert_eq!(
            parse_problems(&["item_missing".into(), " ITEM_MISSING ".into()]).unwrap(),
            vec!["item_missing"],
            "a normalised duplicate must not spawn two children"
        );
    }

    #[test]
    fn problems_rejects_empty_and_unknown() {
        assert_eq!(parse_problems(&[]), Err(SignalRuleError::ProblemsRequired));
        assert_eq!(
            parse_problems(&["guest_in_room".into()]),
            Err(SignalRuleError::UnknownProblem("guest_in_room".to_string())),
            "a real maid→desk type is still not a room-check PROBLEM"
        );
    }

    // ---- escalation ------------------------------------------------------

    /// The eligibility predicate, one clause at a time. Each row flips exactly
    /// one input away from the eligible baseline.
    #[test]
    fn escalation_predicate_requires_every_clause() {
        let eligible = || escalation_eligible(ROOM_CHECK, SignalStatus::Open, 121, false, 0, 150);
        assert!(eligible());

        // Only ขอเช็คห้อง pushes. No other signal type, ever (ADR 0008).
        for other in [PRIORITY_CLEAN, CHECKED_OUT, ITEM_MISSING, DELIVER_LINEN] {
            assert!(!escalation_eligible(other, SignalStatus::Open, 121, false, 0, 150));
        }
        // Acked = someone is on it; done/cancelled are terminal.
        for status in [SignalStatus::Acked, SignalStatus::Done, SignalStatus::Cancelled] {
            assert!(!escalation_eligible(ROOM_CHECK, status, 121, false, 0, 150));
        }
        // Age is STRICTLY greater than two minutes.
        assert!(!escalation_eligible(ROOM_CHECK, SignalStatus::Open, 120, false, 0, 150));
        assert!(!escalation_eligible(ROOM_CHECK, SignalStatus::Open, 0, false, 0, 150));
        // One push per signal, never repeated.
        assert!(!escalation_eligible(ROOM_CHECK, SignalStatus::Open, 600, true, 0, 150));
        // The hard stop: at the cap, not one over it.
        assert!(escalation_eligible(ROOM_CHECK, SignalStatus::Open, 121, false, 149, 150));
        assert!(!escalation_eligible(ROOM_CHECK, SignalStatus::Open, 121, false, 150, 150));
        assert!(!escalation_eligible(ROOM_CHECK, SignalStatus::Open, 121, false, 999, 150));
        // A cap of 0 disables escalation outright rather than allowing one.
        assert!(!escalation_eligible(ROOM_CHECK, SignalStatus::Open, 121, false, 0, 0));
    }

    #[test]
    fn escalation_constants_match_the_adr() {
        assert_eq!(ESCALATION_AGE_SECONDS, 120, "ADR 0008: two minutes");
        assert_eq!(DEFAULT_ESCALATION_MONTHLY_CAP, 150, "ADR 0008: hard stop ~150");
    }

    // ---- literals + wire shape ------------------------------------------

    #[test]
    fn literals_round_trip_through_parse() {
        for d in [SignalDirection::DeskToMaid, SignalDirection::MaidToDesk] {
            assert_eq!(SignalDirection::parse(d.as_str()), Some(d));
        }
        for s in [
            SignalStatus::Open,
            SignalStatus::Acked,
            SignalStatus::Done,
            SignalStatus::Cancelled,
        ] {
            assert_eq!(SignalStatus::parse(s.as_str()), Some(s));
        }
        for s in [
            SignalDoneSource::Tap,
            SignalDoneSource::CleanReport,
            SignalDoneSource::RoomCheckAnswer,
        ] {
            assert_eq!(SignalDoneSource::parse(s.as_str()), Some(s));
        }
        for o in [RoomCheckOutcome::Clear, RoomCheckOutcome::Problems] {
            assert_eq!(RoomCheckOutcome::parse(o.as_str()), Some(o));
        }
        assert_eq!(SignalStatus::parse("Open"), None, "exact, never case-folded");
    }

    #[test]
    fn live_is_exactly_open_and_acked() {
        assert!(SignalStatus::Open.is_live());
        assert!(SignalStatus::Acked.is_live());
        assert!(!SignalStatus::Done.is_live());
        assert!(!SignalStatus::Cancelled.is_live());
    }

    /// The DTO is a cross-language contract with `app/hk/signal-vocab.ts`.
    /// Pinned key-by-key: camelCase everywhere, a bare `type` key, and the
    /// optional fields present as explicit `null`.
    #[test]
    fn dto_serializes_exactly_as_the_typescript_interface_spells_it() {
        let dto = RoomSignal {
            signal_id: 7,
            room_id: 42,
            room_no: "104".to_string(),
            direction: SignalDirection::DeskToMaid,
            signal_type: ROOM_CHECK.to_string(),
            status: SignalStatus::Acked,
            outcome: None,
            parent_id: None,
            created_by: SignalActor {
                badge: "Q1001".to_string(),
                name: Some("สมศรี".to_string()),
            },
            created_at: "2026-09-01T03:00:00Z".to_string(),
            acked_by: Some(SignalActor {
                badge: "Q2002".to_string(),
                name: None,
            }),
            acked_at: Some("2026-09-01T03:01:00Z".to_string()),
            done_by: None,
            done_at: None,
            done_source: None,
        };
        let json = serde_json::to_value(&dto).expect("serializes");
        let obj = json.as_object().expect("object");
        let mut keys: Vec<&str> = obj.keys().map(String::as_str).collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            vec![
                "ackedAt",
                "ackedBy",
                "createdAt",
                "createdBy",
                "direction",
                "doneAt",
                "doneBy",
                "doneSource",
                "outcome",
                "parentId",
                "roomId",
                "roomNo",
                "signalId",
                "status",
                "type",
            ]
        );
        assert_eq!(obj["type"], "room_check");
        assert_eq!(obj["direction"], "desk_to_maid");
        assert_eq!(obj["status"], "acked");
        assert!(obj["outcome"].is_null());
        assert!(obj["doneSource"].is_null());
        assert_eq!(obj["createdBy"]["badge"], "Q1001");
        assert!(obj["ackedBy"]["name"].is_null());

        // Round-trips: the SSE relay reads this shape back out of a
        // DomainEvent payload, so a serialize-only contract is not enough.
        let back: RoomSignal = serde_json::from_value(json).expect("deserializes");
        assert_eq!(back, dto);
    }

    #[test]
    fn done_source_and_outcome_serialize_as_snake_case_codes() {
        assert_eq!(
            serde_json::to_value(SignalDoneSource::RoomCheckAnswer).unwrap(),
            serde_json::json!("room_check_answer")
        );
        assert_eq!(
            serde_json::to_value(SignalDoneSource::CleanReport).unwrap(),
            serde_json::json!("clean_report")
        );
        assert_eq!(
            serde_json::to_value(RoomCheckOutcome::Problems).unwrap(),
            serde_json::json!("problems")
        );
    }
}
