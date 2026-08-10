//! The "grid staleness latch" — the pure state machine behind
//! `docs/adr/0006-legacy-stale-notification.md` and its "Grid staleness
//! latch" glossary entry in `CONTEXT.md`.
//!
//! Per the ADR: every writeback that lands in the shared legacy MSSQL can
//! leave iHOTEL's own room grid stale, because `FormRoomMain`'s ~60.56s
//! refresh timer only runs while iHOTEL holds foreground
//! (`docs/legacy-app/ROOM_GRID_REFRESH.md` §1-2). A toast on every single
//! writeback would retrain reception to ignore toasts within a shift (ADR
//! 0006 §5, echoing CLAUDE.md's alerting guardrail), so staleness is
//! modeled as one open "episode" per stale spell: the first writeback while
//! iHOTEL can't self-heal opens it and fires exactly one toast; every
//! writeback after that, while the episode is still open, only bumps a
//! counter.
//!
//! **This module makes no OS calls and starts no timers of its own** — it is
//! a plain struct that reacts to observations the caller feeds in, each
//! carrying the caller's own idea of "now". That keeps the transition table
//! exhaustively unit-testable without mocking the clock or a window
//! manager. The caller (Stage 4's Windows polling loop, or `server.rs`'s
//! HTTP handlers today) is responsible for:
//! - calling [`StaleLatch::on_notify`] when a `legacy_stale` signal arrives
//!   over `POST /notify`,
//! - calling [`StaleLatch::on_foreground`] repeatedly (e.g. every poll
//!   tick) for as long as iHOTEL is observed to hold the foreground window,
//! - calling [`StaleLatch::on_background`] exactly once when iHOTEL loses
//!   foreground,
//! - calling [`StaleLatch::on_refresh_performed`] when the receptionist's
//!   "Refresh iHOTEL now" action actually runs (Stage 4; today that action
//!   is an unconditional 501, see `crate::ihotel::trigger_refresh`, so this
//!   is currently unreachable from production but fully exercised by tests).

use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde::{Deserialize, Serialize};

/// Continuous foreground hold required to auto-clear a stale episode.
///
/// ADR 0006 §5 / `docs/legacy-app/ROOM_GRID_REFRESH.md` §1: iHOTEL's own
/// grid-refresh timer fires roughly every 60.56s, but only while
/// `FormRoomMain` holds foreground (focus clears `MSSQL.CodeErr`, the gate
/// the timer checks). 65s gives that timer room to have ticked at least
/// once, plus slack — long enough that the grid is presumed self-healed.
///
/// Not yet called from production code under a plain `cargo check` (only
/// from this module's own tests and, transitively, from
/// [`StaleLatch::on_foreground`] below) — Stage 4's Windows foreground-poll
/// loop is the intended caller of `on_foreground`/`on_background`. The
/// `#[allow(dead_code)]` here (and on those two methods / the
/// `foreground_since` field) documents that this is deliberate forward
/// wiring, not an accidental unused item.
#[allow(dead_code)]
pub const FOREGROUND_CLEAR_HOLD_SECS: i64 = 65;

/// Character cap (char-wise, not byte-wise — every label here is Thai and
/// multi-byte in UTF-8) applied to whatever text ends up in a toast.
///
/// `summary`/`items` in the inbound payload come from OUR OWN backend
/// (`hotel-backend/src/outbox/legacy_stale.rs`), which already clamps them
/// (`MAX_SUMMARY_CHARS` = 200, up to `MAX_ITEMS` = 3 items of
/// `MAX_ITEM_CHARS` = 80 chars each there). This module re-clamps anyway,
/// treating the payload as untrusted: `POST /notify` sits behind this
/// middleware's CORS allowlist, not behind any auth, so anything that can
/// reach `127.0.0.1:9898` from an allowed origin can post an arbitrary
/// body. Defense in depth costs nothing here.
pub const MAX_TOAST_CHARS: usize = 240;

const TOAST_PREFIX: &str = "iHOTEL อาจไม่ใช่ข้อมูลล่าสุด: ";
const TOAST_FALLBACK: &str = "iHOTEL อาจไม่ใช่ข้อมูลล่าสุด กรุณากด Refresh ที่หน้าจอห้องพัก";

/// The inbound `POST /notify` body.
///
/// Same wire shape as `hotel-backend`'s `LegacyStaleSignal`
/// (`hotel-backend/src/outbox/legacy_stale.rs`): `{id, site, count,
/// summary, items, emitted_at}`, plain snake_case field names (that struct
/// has no `#[serde(rename_all)]` either). Deliberately NOT shared as a
/// cross-repo type — this middleware only reads `summary`/`items` to build
/// toast text and passes the rest through for diagnostics
/// (`GET /ihotel/status` doesn't even echo them today), so duplicating this
/// tiny shape here avoids a cross-repo dependency for four extra fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleSignalPayload {
    pub id: String,
    pub site: String,
    pub count: u32,
    pub summary: String,
    #[serde(default)]
    pub items: Vec<String>,
    pub emitted_at: String,
}

/// What the caller should do in reaction to a latch transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// First surfacing of a stale episode (or the first surfacing that
    /// wasn't suppressed by foreground) — show a brand-new toast with this
    /// (already sanitized) text.
    ShowToast(String),
    /// The episode's toast already fired once; this just reflects however
    /// many writebacks have now landed. No new toast — see the module docs
    /// on alarm fatigue.
    UpdateCount(u32),
    /// Nothing for the caller to do: suppressed while foreground, no
    /// change, or a repeat notify that was already reflected.
    Nothing,
}

/// Internal state. Kept private — external code only ever sees it through
/// [`StaleLatch::snapshot`].
#[derive(Debug)]
enum LatchState {
    Clear,
    Stale {
        count: u32,
        /// Highest `count` already surfaced to the caller via
        /// `ShowToast`/`UpdateCount`. `count > shown_count` means there is
        /// un-surfaced staleness pending — always because notifies arrived
        /// while iHOTEL was foreground and got suppressed on arrival (see
        /// [`StaleLatch::on_notify`] and [`StaleLatch::on_background`]).
        shown_count: u32,
    },
}

/// `GET /ihotel/status`'s `latchState`/`staleCount`/`lastNotifyAt` fields,
/// read out of a [`StaleLatch`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LatchSnapshot {
    pub state: &'static str,
    pub stale_count: u32,
    pub last_notify_at: Option<DateTime<Utc>>,
}

/// The grid-staleness latch. See the module docs for the full contract.
#[derive(Debug)]
pub struct StaleLatch {
    state: LatchState,
    /// Whether iHOTEL is currently believed to hold foreground, per the
    /// caller's most recent `on_foreground`/`on_background` call.
    foreground: bool,
    /// When the CURRENT unbroken foreground hold started. Reset to `None`
    /// on every `on_background` — the hold must be continuous; a single
    /// background dip resets the clock even if immediately followed by
    /// another `on_foreground`.
    #[allow(dead_code)] // see FOREGROUND_CLEAR_HOLD_SECS doc — Stage 4 wiring
    foreground_since: Option<DateTime<Utc>>,
    /// Sanitized text from the most recent notify, cached so
    /// `on_background` can show a toast without a payload in hand.
    latest_text: Option<String>,
    /// Timestamp of the most recent `on_notify` call, foreground or not —
    /// surfaced by `/ihotel/status` as `lastNotifyAt` regardless of whether
    /// that particular notify produced a visible toast.
    last_notify_at: Option<DateTime<Utc>>,
}

impl Default for StaleLatch {
    fn default() -> Self {
        Self::new()
    }
}

impl StaleLatch {
    pub fn new() -> Self {
        Self {
            state: LatchState::Clear,
            foreground: false,
            foreground_since: None,
            latest_text: None,
            last_notify_at: None,
        }
    }

    /// A writeback landed in legacy MSSQL and a `legacy_stale` signal
    /// arrived. `now` is the caller's clock.
    pub fn on_notify(&mut self, payload: &StaleSignalPayload, now: DateTime<Utc>) -> Action {
        let text = compose_toast_text(payload);
        self.latest_text = Some(text.clone());
        self.last_notify_at = Some(now);

        match &mut self.state {
            LatchState::Clear => {
                if self.foreground {
                    // Opens the episode but suppresses the toast — ADR 0006
                    // §5: "suppressed entirely while iHOTEL is foreground,"
                    // because focus alone already lets iHOTEL's own timer
                    // resume. Not yet surfaced: `on_background` flushes it
                    // if the foreground hold breaks before self-healing
                    // (see that method's doc for the full "pending toast on
                    // background" decision).
                    self.state = LatchState::Stale {
                        count: 1,
                        shown_count: 0,
                    };
                    Action::Nothing
                } else {
                    self.state = LatchState::Stale {
                        count: 1,
                        shown_count: 1,
                    };
                    Action::ShowToast(text)
                }
            }
            LatchState::Stale { count, shown_count } => {
                *count += 1;
                if self.foreground {
                    // Suppressed the same way a fresh episode would be —
                    // still tracked (count incremented above), nothing
                    // shown while she's already looking at iHOTEL.
                    Action::Nothing
                } else {
                    *shown_count = *count;
                    Action::UpdateCount(*count)
                }
            }
        }
    }

    /// The receptionist's "Refresh iHOTEL now" action actually ran (Stage 4
    /// owns making that real — see `crate::ihotel::trigger_refresh`).
    /// Clears unconditionally, regardless of foreground state or how many
    /// writebacks had accumulated.
    pub fn on_refresh_performed(&mut self, _now: DateTime<Utc>) -> Action {
        self.state = LatchState::Clear;
        Action::Nothing
    }

    /// iHOTEL is (still) observed to hold foreground. Call this repeatedly
    /// while it remains foregrounded — e.g. once per poll tick — not just
    /// once on the focus-gained edge. A single call only starts the hold
    /// clock; the clock advances across successive calls that share the
    /// same unbroken foreground streak (no intervening [`Self::on_background`]
    /// in between). Once that streak reaches [`FOREGROUND_CLEAR_HOLD_SECS`],
    /// an open episode clears silently — no [`Action`], because iHOTEL's
    /// own timer is presumed to have already refreshed the grid by then, so
    /// there is nothing left to tell reception.
    #[allow(dead_code)] // see FOREGROUND_CLEAR_HOLD_SECS doc — Stage 4 wiring
    pub fn on_foreground(&mut self, now: DateTime<Utc>) -> Action {
        self.foreground = true;
        match self.foreground_since {
            None => self.foreground_since = Some(now),
            Some(since) => {
                let elapsed = now.signed_duration_since(since);
                if elapsed >= ChronoDuration::seconds(FOREGROUND_CLEAR_HOLD_SECS) {
                    self.state = LatchState::Clear;
                }
            }
        }
        Action::Nothing
    }

    /// iHOTEL lost foreground (or this is just a state refresh while it was
    /// already known to be backgrounded). Always resets the hold clock —
    /// see the `foreground_since` field doc.
    ///
    /// **The pending-toast-on-background decision**, documented here
    /// because this is the one branch that embodies it: if a stale episode
    /// opened or grew while iHOTEL was foreground — suppressed per ADR 0006
    /// §5 — and iHOTEL backgrounds again before holding foreground
    /// continuously for [`FOREGROUND_CLEAR_HOLD_SECS`], that pending
    /// staleness is flushed to the caller HERE, exactly once: a fresh
    /// [`Action::ShowToast`] if the episode never surfaced before, or an
    /// [`Action::UpdateCount`] if its toast already fired earlier and this
    /// just reflects writebacks that landed during the foreground dip.
    ///
    /// Rationale: a foreground window that closes before the ~65s hold
    /// threshold gives no guarantee iHOTEL's own 60.56s timer ran even
    /// once, so the suppression while foreground was provisional, not a
    /// confirmed self-heal. The moment reception looks away again is
    /// exactly when that provisional suppression stops being justified —
    /// she is no longer looking at a grid that might still catch up on its
    /// own, so it's fair to tell her now.
    #[allow(dead_code)] // see FOREGROUND_CLEAR_HOLD_SECS doc — Stage 4 wiring
    pub fn on_background(&mut self, now: DateTime<Utc>) -> Action {
        self.foreground = false;
        self.foreground_since = None;
        let _ = now; // No OS call here; kept for signature symmetry with
                      // on_foreground/on_notify and headroom for a future
                      // "how long was she away" refinement.

        if let LatchState::Stale { count, shown_count } = &mut self.state {
            if *count > *shown_count {
                let never_shown = *shown_count == 0;
                *shown_count = *count;
                return if never_shown {
                    match &self.latest_text {
                        Some(text) => Action::ShowToast(text.clone()),
                        // Unreachable: `latest_text` is set by every
                        // `on_notify` call, and `count` only rises via
                        // `on_notify`, so a positive count implies text
                        // exists. Kept as a safe fallback rather than an
                        // `unwrap`.
                        None => Action::Nothing,
                    }
                } else {
                    Action::UpdateCount(*count)
                };
            }
        }
        Action::Nothing
    }

    /// Snapshot for `GET /ihotel/status`.
    pub fn snapshot(&self) -> LatchSnapshot {
        match &self.state {
            LatchState::Clear => LatchSnapshot {
                state: "clear",
                stale_count: 0,
                last_notify_at: self.last_notify_at,
            },
            LatchState::Stale { count, .. } => LatchSnapshot {
                state: "stale",
                stale_count: *count,
                last_notify_at: self.last_notify_at,
            },
        }
    }
}

/// Build the (unsanitized) toast body from a payload: prefer `summary`,
/// fall back to a joined `items` list, fall back to a generic Thai message
/// if both are blank.
fn compose_toast_text(payload: &StaleSignalPayload) -> String {
    let body = if !payload.summary.trim().is_empty() {
        payload.summary.clone()
    } else if !payload.items.is_empty() {
        payload.items.join(", ")
    } else {
        String::new()
    };

    let sanitized = sanitize(&body);
    if sanitized.is_empty() {
        TOAST_FALLBACK.to_string()
    } else {
        format!("{TOAST_PREFIX}{sanitized}")
    }
}

/// Strip control characters and cap length, char-wise. The payload is
/// untrusted input (see [`MAX_TOAST_CHARS`]'s doc) — this is the only place
/// that text from it reaches a rendered surface (a Windows toast).
fn sanitize(raw: &str) -> String {
    let cleaned: String = raw.chars().filter(|c| !c.is_control()).collect();
    let cleaned = cleaned.trim();

    if cleaned.chars().count() > MAX_TOAST_CHARS {
        let mut truncated: String = cleaned.chars().take(MAX_TOAST_CHARS).collect();
        truncated.push('…');
        truncated
    } else {
        cleaned.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base_time() -> DateTime<Utc> {
        "2026-08-10T09:00:00Z".parse().unwrap()
    }

    fn plus_secs(secs: i64) -> DateTime<Utc> {
        base_time() + ChronoDuration::seconds(secs)
    }

    fn payload(summary: &str) -> StaleSignalPayload {
        StaleSignalPayload {
            id: "sig-1".to_string(),
            site: "hfhotel".to_string(),
            count: 1,
            summary: summary.to_string(),
            items: vec![],
            emitted_at: "2026-08-10T09:00:00Z".to_string(),
        }
    }

    // -- Transition table ---------------------------------------------

    #[test]
    fn notify_while_clear_shows_toast() {
        let mut latch = StaleLatch::new();
        let action = latch.on_notify(&payload("เช็คอิน ห้อง 302"), base_time());
        assert_eq!(
            action,
            Action::ShowToast(format!("{TOAST_PREFIX}เช็คอิน ห้อง 302"))
        );
        let snap = latch.snapshot();
        assert_eq!(snap.state, "stale");
        assert_eq!(snap.stale_count, 1);
    }

    #[test]
    fn notify_while_stale_bumps_count_no_new_toast() {
        let mut latch = StaleLatch::new();
        latch.on_notify(&payload("เช็คอิน ห้อง 302"), plus_secs(0));
        let action = latch.on_notify(&payload("เช็คเอาต์ ห้อง 210"), plus_secs(5));
        assert_eq!(action, Action::UpdateCount(2));
        assert_eq!(latch.snapshot().stale_count, 2);
    }

    #[test]
    fn multiple_notifies_while_stale_increment_each_time() {
        let mut latch = StaleLatch::new();
        latch.on_notify(&payload("a"), plus_secs(0));
        assert_eq!(latch.on_notify(&payload("b"), plus_secs(1)), Action::UpdateCount(2));
        assert_eq!(latch.on_notify(&payload("c"), plus_secs(2)), Action::UpdateCount(3));
        assert_eq!(latch.on_notify(&payload("d"), plus_secs(3)), Action::UpdateCount(4));
        assert_eq!(latch.snapshot().stale_count, 4);
    }

    #[test]
    fn foreground_64s_does_not_clear() {
        let mut latch = StaleLatch::new();
        latch.on_notify(&payload("a"), plus_secs(0));
        latch.on_foreground(plus_secs(1));
        latch.on_foreground(plus_secs(1 + 64));
        assert_eq!(latch.snapshot().state, "stale");
    }

    #[test]
    fn foreground_66s_clears() {
        let mut latch = StaleLatch::new();
        latch.on_notify(&payload("a"), plus_secs(0));
        latch.on_foreground(plus_secs(1));
        latch.on_foreground(plus_secs(1 + 66));
        assert_eq!(latch.snapshot().state, "clear");
        assert_eq!(latch.snapshot().stale_count, 0);
    }

    #[test]
    fn foreground_exactly_at_threshold_clears() {
        // Boundary check: the contract is ">= 65s", so exactly 65 clears.
        let mut latch = StaleLatch::new();
        latch.on_notify(&payload("a"), plus_secs(0));
        latch.on_foreground(plus_secs(0));
        latch.on_foreground(plus_secs(FOREGROUND_CLEAR_HOLD_SECS));
        assert_eq!(latch.snapshot().state, "clear");
    }

    #[test]
    fn foreground_hold_resets_after_background() {
        let mut latch = StaleLatch::new();
        latch.on_notify(&payload("a"), plus_secs(0));
        latch.on_foreground(plus_secs(1));
        // 40s foreground, then away, then foreground again — the clock
        // must restart, not accumulate across the gap.
        latch.on_background(plus_secs(41));
        latch.on_foreground(plus_secs(42));
        latch.on_foreground(plus_secs(42 + 64));
        assert_eq!(
            latch.snapshot().state,
            "stale",
            "a background dip must reset the continuous-hold clock"
        );
    }

    #[test]
    fn foreground_while_clear_is_noop() {
        let mut latch = StaleLatch::new();
        latch.on_foreground(plus_secs(0));
        latch.on_foreground(plus_secs(1000));
        assert_eq!(latch.snapshot().state, "clear");
    }

    #[test]
    fn refresh_performed_clears_from_stale() {
        let mut latch = StaleLatch::new();
        latch.on_notify(&payload("a"), plus_secs(0));
        latch.on_notify(&payload("b"), plus_secs(1));
        let action = latch.on_refresh_performed(plus_secs(2));
        assert_eq!(action, Action::Nothing);
        assert_eq!(latch.snapshot().state, "clear");
        assert_eq!(latch.snapshot().stale_count, 0);
    }

    #[test]
    fn refresh_performed_is_noop_from_clear() {
        let mut latch = StaleLatch::new();
        let action = latch.on_refresh_performed(plus_secs(0));
        assert_eq!(action, Action::Nothing);
        assert_eq!(latch.snapshot().state, "clear");
    }

    #[test]
    fn notify_after_clear_reopens_a_fresh_episode() {
        let mut latch = StaleLatch::new();
        latch.on_notify(&payload("a"), plus_secs(0));
        latch.on_notify(&payload("b"), plus_secs(1));
        latch.on_refresh_performed(plus_secs(2));

        let action = latch.on_notify(&payload("c"), plus_secs(3));
        assert_eq!(action, Action::ShowToast(format!("{TOAST_PREFIX}c")));
        assert_eq!(
            latch.snapshot().stale_count,
            1,
            "count must not carry over from the cleared episode"
        );
    }

    #[test]
    fn notify_while_foreground_is_suppressed() {
        let mut latch = StaleLatch::new();
        latch.on_foreground(plus_secs(0));
        let action = latch.on_notify(&payload("a"), plus_secs(1));
        assert_eq!(action, Action::Nothing);
        // Still tracked internally even though nothing was shown.
        assert_eq!(latch.snapshot().state, "stale");
        assert_eq!(latch.snapshot().stale_count, 1);
    }

    #[test]
    fn suppressed_while_foreground_then_background_toasts_once() {
        // The documented pending-toast-on-background decision: a notify
        // that arrives while iHOTEL is foreground is suppressed, but if
        // foreground breaks before the self-heal hold completes, the
        // pending episode surfaces exactly once, on the background edge.
        let mut latch = StaleLatch::new();
        latch.on_foreground(plus_secs(0));
        let notify_action = latch.on_notify(&payload("เช็คอิน ห้อง 105"), plus_secs(1));
        assert_eq!(notify_action, Action::Nothing);

        let background_action = latch.on_background(plus_secs(10));
        assert_eq!(
            background_action,
            Action::ShowToast(format!("{TOAST_PREFIX}เช็คอิน ห้อง 105"))
        );

        // And it fires only once — a second background call with nothing
        // new pending must not re-toast.
        assert_eq!(latch.on_background(plus_secs(11)), Action::Nothing);
    }

    #[test]
    fn suppressed_while_foreground_then_background_after_already_shown_updates_count_only() {
        // Episode opened (and toasted) while NOT foreground; then iHOTEL
        // comes to foreground for a while and two more writebacks land,
        // suppressed; then background again — should bump the count, not
        // re-fire the original toast.
        let mut latch = StaleLatch::new();
        assert_eq!(
            latch.on_notify(&payload("a"), plus_secs(0)),
            Action::ShowToast(format!("{TOAST_PREFIX}a"))
        );

        latch.on_foreground(plus_secs(1));
        assert_eq!(latch.on_notify(&payload("b"), plus_secs(2)), Action::Nothing);
        assert_eq!(latch.on_notify(&payload("c"), plus_secs(3)), Action::Nothing);

        let background_action = latch.on_background(plus_secs(4));
        assert_eq!(background_action, Action::UpdateCount(3));
    }

    #[test]
    fn background_without_pending_does_nothing() {
        let mut latch = StaleLatch::new();
        assert_eq!(latch.on_background(plus_secs(0)), Action::Nothing);

        // Also true once an episode's toast has already been shown while
        // not foreground — nothing new to flush.
        latch.on_notify(&payload("a"), plus_secs(1));
        assert_eq!(latch.on_background(plus_secs(2)), Action::Nothing);
    }

    #[test]
    fn background_after_notify_shown_while_not_foreground_does_not_reshow() {
        let mut latch = StaleLatch::new();
        latch.on_notify(&payload("a"), plus_secs(0)); // shown immediately, not foreground
        assert_eq!(latch.on_background(plus_secs(1)), Action::Nothing);
    }

    // -- Snapshot / diagnostics -----------------------------------------

    #[test]
    fn snapshot_reports_clear_initially() {
        let latch = StaleLatch::new();
        let snap = latch.snapshot();
        assert_eq!(snap.state, "clear");
        assert_eq!(snap.stale_count, 0);
        assert_eq!(snap.last_notify_at, None);
    }

    #[test]
    fn snapshot_reports_stale_count_and_last_notify_at() {
        let mut latch = StaleLatch::new();
        latch.on_notify(&payload("a"), plus_secs(0));
        latch.on_notify(&payload("b"), plus_secs(7));
        let snap = latch.snapshot();
        assert_eq!(snap.state, "stale");
        assert_eq!(snap.stale_count, 2);
        assert_eq!(snap.last_notify_at, Some(plus_secs(7)));
    }

    #[test]
    fn last_notify_at_updates_even_when_suppressed_by_foreground() {
        let mut latch = StaleLatch::new();
        latch.on_foreground(plus_secs(0));
        latch.on_notify(&payload("a"), plus_secs(5));
        assert_eq!(latch.snapshot().last_notify_at, Some(plus_secs(5)));
    }

    // -- Sanitization / text composition ---------------------------------

    #[test]
    fn sanitize_strips_control_chars() {
        let dirty = "ห้อง\u{0007}302\u{001b}[31m";
        let clean = sanitize(dirty);
        assert!(!clean.contains('\u{0007}'));
        assert!(!clean.contains('\u{001b}'));
        assert_eq!(clean, "ห้อง302[31m");
    }

    #[test]
    fn sanitize_strips_newlines_and_trims() {
        let dirty = "  line one\nline two\r\n  ";
        let clean = sanitize(dirty);
        assert_eq!(clean, "line oneline two");
    }

    #[test]
    fn sanitize_caps_length() {
        let long = "ก".repeat(MAX_TOAST_CHARS + 50);
        let clean = sanitize(&long);
        // MAX_TOAST_CHARS chars plus the ellipsis marker.
        assert_eq!(clean.chars().count(), MAX_TOAST_CHARS + 1);
        assert!(clean.ends_with('…'));
    }

    #[test]
    fn sanitize_under_cap_is_untouched() {
        let short = "ห้อง 302 รอทำความสะอาด";
        assert_eq!(sanitize(short), short);
    }

    #[test]
    fn compose_toast_text_uses_summary_when_present() {
        let p = payload("รับชำระเงิน ห้อง 401");
        assert_eq!(
            compose_toast_text(&p),
            format!("{TOAST_PREFIX}รับชำระเงิน ห้อง 401")
        );
    }

    #[test]
    fn compose_toast_text_falls_back_to_items_when_summary_blank() {
        let mut p = payload("");
        p.items = vec!["เช็คอิน ห้อง 101".to_string(), "เช็คเอาต์ ห้อง 102".to_string()];
        assert_eq!(
            compose_toast_text(&p),
            format!("{TOAST_PREFIX}เช็คอิน ห้อง 101, เช็คเอาต์ ห้อง 102")
        );
    }

    #[test]
    fn compose_toast_text_uses_fallback_when_everything_blank() {
        let p = payload("   ");
        assert_eq!(compose_toast_text(&p), TOAST_FALLBACK);
    }

    #[test]
    fn notify_toast_text_is_sanitized_end_to_end() {
        let mut latch = StaleLatch::new();
        let dirty = payload("ห้อง\u{0007}302");
        let action = latch.on_notify(&dirty, plus_secs(0));
        assert_eq!(action, Action::ShowToast(format!("{TOAST_PREFIX}ห้อง302")));
    }

    // -- Wire shape --------------------------------------------------

    #[test]
    fn stale_signal_payload_deserializes_backend_shape() {
        let json = r#"{
            "id": "9f5c...",
            "site": "hfhotel",
            "count": 3,
            "summary": "เช็คอิน ห้อง 302, รับชำระเงิน ห้อง 302",
            "items": ["เช็คอิน ห้อง 302", "รับชำระเงิน ห้อง 302"],
            "emitted_at": "2026-08-10T09:00:00Z"
        }"#;
        let payload: StaleSignalPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.site, "hfhotel");
        assert_eq!(payload.count, 3);
        assert_eq!(payload.items.len(), 2);
    }

    #[test]
    fn stale_signal_payload_items_defaults_when_absent() {
        let json = r#"{
            "id": "1",
            "site": "hfhotel",
            "count": 1,
            "summary": "x",
            "emitted_at": "2026-08-10T09:00:00Z"
        }"#;
        let payload: StaleSignalPayload = serde_json::from_str(json).unwrap();
        assert!(payload.items.is_empty());
    }
}
