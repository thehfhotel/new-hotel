//! The iHOTEL process/window probe and the reception-invoked refresh action.
//!
//! Stage 3 shipped this module as an honest stub. Stage 4 (this revision)
//! fills in the real Windows implementation: locate iHOTEL's own "Refresh"
//! button with UI Automation, then act on it with a posted Win32 mouse
//! click.
//!
//! **Why this never drives iHOTEL on its own.** ADR 0006 Decision §1
//! (`docs/adr/0006-legacy-stale-notification.md`) is explicit: this
//! middleware notifies, it never automates the refresh. [`trigger_refresh`]
//! is the single code path both `POST /ihotel/refresh` and the tray's
//! "รีเฟรช iHOTEL" item call, and both of those are reception-invoked (a
//! click) — nothing a writeback commit does can reach it. [`snapshot`] is
//! read-only by construction.
//!
//! # The lever, and why it is the one we use
//!
//! Per `docs/legacy-app/ROOM_GRID_REFRESH.md` §5, iHOTEL's only
//! externally-reachable refresh is its own `ButtonX3` ("Refresh") on
//! `FormRoomMain`, whose `ButtonX3_Click` handler is **not** gated on
//! `MSSQL.CodeErr` — it is the one path that works while iHOTEL is
//! unfocused, which is the entire point (the 60.56s timer is focus-gated and
//! therefore useless to us).
//!
//! # Why UIA to locate, Win32 to act
//!
//! - **Locate with UIA.** WinForms maps a control's `Name` onto the UIA
//!   `AutomationId` property, so `"ButtonX3"` / `"ButtonX6"` are directly
//!   queryable and survive any relayout, unlike coordinates or z-order
//!   guesses. `FormRoomMain` is an MDI *child* (`ROOM_GRID_REFRESH.md` §10),
//!   never a top-level window, so we resolve the top-level `HOTEL.exe` shell
//!   (`frmMain1`) first and then search its UIA descendants.
//! - **Act with Win32, not UIA.** `ButtonX` is a DevComponents control
//!   deriving from `Control`, not `ButtonBase`, so `BM_CLICK` would be
//!   silently swallowed (nothing handles it) and the standard UIA `Invoke`
//!   pattern is not reliably exposed for a hand-painted `Control` subclass.
//!   A posted `WM_LBUTTONDOWN`/`WM_LBUTTONUP` pair on the button's own HWND
//!   is what actually drives `Control`'s click machinery.
//! - **`PostMessage`, never `SendMessage`.** `SendMessage` blocks the
//!   calling thread until the target's message pump answers — on a wedged
//!   iHOTEL that is forever, and this middleware also serves the card
//!   reader. `PostMessage` queues and returns.
//! - **Never `SendInput` / `SetForegroundWindow` / `SetActiveWindow`.**
//!   Those hijack the receptionist's cursor and focus mid-typing. Everything
//!   here is invisible to whatever she is doing.
//!
//! # Guards
//!
//! A refresh is **not free** (`ROOM_GRID_REFRESH.md` §4-5): `LoadRooms` can
//! `UPDATE HT_Rooms`/`HT_Book_Date` via `AutoAddBookingRooms` and toggle
//! physical room power, and `ButtonX3_Click` calls `ClearCheck()` which
//! destroys reception's in-progress multi-room tap-selection. So a **guarded
//! skip is always preferred over a risky click**: when in doubt we do
//! nothing and explain why, in Thai, on a toast.
//!
//! All guard policy lives in [`decide`], a pure function over
//! [`IhotelSnapshot`] that makes no OS calls — so the guard ordering is
//! exhaustively unit-testable on macOS (this crate's CI leg). The Windows
//! side only fills the snapshot in and executes the resulting [`Decision`].

use std::sync::OnceLock;

use serde::Serialize;
use thiserror::Error;

use crate::toast;

#[cfg(target_os = "windows")]
mod windows_impl;

/// Title on every "we did not refresh, and here is why" toast. Deliberately
/// distinct from `server.rs`'s `TOAST_TITLE` ("iHOTEL", used for the
/// legacy-stale notification) so reception can tell an *outcome* toast from
/// a *staleness* toast at a glance.
const SKIP_TOAST_TITLE: &str = "รีเฟรช iHOTEL ไม่สำเร็จ";

// -- Configurable targets -------------------------------------------------

/// Env var overriding the AutomationId of iHOTEL's Refresh button.
pub const ENV_REFRESH_BUTTON_ID: &str = "IHOTEL_REFRESH_BUTTON_ID";
/// `ButtonX3` — `FormRoomMain.cs:1857` (`Text = "Refresh"`).
pub const DEFAULT_REFRESH_BUTTON_ID: &str = "ButtonX3";

/// Env var overriding the AutomationId of iHOTEL's "confirm multi-room
/// selection" button, whose visibility is our selection-pending tripwire.
pub const ENV_SELECTION_BUTTON_ID: &str = "IHOTEL_SELECTION_BUTTON_ID";
/// `ButtonX6` — hidden by `ClearCheck()` (`FormRoomMain.cs:4123`), so
/// *visible* means a multi-room selection is mid-flight.
pub const DEFAULT_SELECTION_BUTTON_ID: &str = "ButtonX6";

/// Env var overriding the process image name we look for.
pub const ENV_PROCESS_NAME: &str = "IHOTEL_PROCESS_NAME";
/// iHOTEL's executable.
pub const DEFAULT_PROCESS_NAME: &str = "HOTEL.exe";

/// What this middleware looks for on the machine.
///
/// All three are env-overridable so that a **vendor rename is config, not a
/// release**: iHOTEL is a third-party binary we do not control, and a
/// designer-renamed `ButtonX3` would otherwise turn a five-second `.env`
/// edit into a build-sign-redeploy cycle across every reception PC.
/// `IHOTEL_PROCESS_NAME` additionally lets the throwaway WinForms harness
/// (`scripts/dev/ihotel-uia-harness.ps1`) stand in for `HOTEL.exe`, so the
/// whole path can be exercised without touching a live terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Targets {
    pub process_name: String,
    pub refresh_button_id: String,
    pub selection_button_id: String,
}

impl Default for Targets {
    fn default() -> Self {
        Targets {
            process_name: DEFAULT_PROCESS_NAME.to_string(),
            refresh_button_id: DEFAULT_REFRESH_BUTTON_ID.to_string(),
            selection_button_id: DEFAULT_SELECTION_BUTTON_ID.to_string(),
        }
    }
}

/// Resolve [`Targets`] from the environment, falling back to the iHOTEL
/// defaults. A set-but-blank/whitespace-only value is treated as unset —
/// an empty AutomationId would match nothing and produce a mystifying
/// `grid-not-found`, so we prefer the working default over honoring
/// obviously-accidental config.
pub fn targets() -> Targets {
    Targets {
        process_name: env_or(ENV_PROCESS_NAME, DEFAULT_PROCESS_NAME),
        refresh_button_id: env_or(ENV_REFRESH_BUTTON_ID, DEFAULT_REFRESH_BUTTON_ID),
        selection_button_id: env_or(ENV_SELECTION_BUTTON_ID, DEFAULT_SELECTION_BUTTON_ID),
    }
}

fn env_or(key: &str, default: &str) -> String {
    match std::env::var(key) {
        Ok(value) if !value.trim().is_empty() => value.trim().to_string(),
        _ => default.to_string(),
    }
}

// -- Observations ---------------------------------------------------------

/// A point-in-time read of whether iHOTEL is running / has its room grid
/// open, as far as this middleware can tell from outside the process. This
/// is the shape `GET /ihotel/status` and the tray's status item report; the
/// richer [`IhotelSnapshot`] is what the guards actually reason over.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Snapshot {
    /// Whether a `HOTEL.exe` process was found running on this machine.
    pub process_found: bool,
    /// Whether iHOTEL's room-grid window (`FormRoomMain`) was found —
    /// operationally, whether its Refresh button resolved.
    pub grid_window_found: bool,
}

impl Snapshot {
    /// The honest "nothing observed" snapshot — what every non-Windows
    /// target reports unconditionally.
    pub const fn unavailable() -> Self {
        Snapshot {
            process_found: false,
            grid_window_found: false,
        }
    }
}

/// Everything [`decide`] is allowed to reason over. Deliberately made of
/// plain `bool`/`Option`/`isize` rather than OS handles so the entire guard
/// matrix is constructible — and therefore testable — on a machine that has
/// never seen a Windows HWND.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IhotelSnapshot {
    /// Any process matching [`Targets::process_name`] exists.
    pub process_found: bool,
    /// `IsWindowEnabled` on iHOTEL's top-level shell window (`frmMain1`).
    /// `None` means no top-level window was observed at all — which is
    /// deliberately **not** the same as "modal open", see [`decide`].
    pub main_window_enabled: Option<bool>,
    /// The `WS_VISIBLE` **style bit** on the selection button's own HWND.
    ///
    /// Checked as a window style, not via UIA's `IsOffscreen`: a control
    /// that is merely scrolled out of view or covered is still logically
    /// "showing" as far as reception is concerned, whereas `ClearCheck()`
    /// clears the style outright. An entirely absent handle (WinForms never
    /// created one because the control was never made visible) means the
    /// selection was never started — safe, so `false`.
    pub selection_pending: bool,
    /// Native window handle of the Refresh button, as a raw `isize` (`HWND`
    /// is a pointer and not `Send`; the numeric form crosses threads and
    /// test boundaries freely). `None` when it could not be resolved.
    pub refresh_button_hwnd: Option<isize>,
}

impl IhotelSnapshot {
    /// The honest "nothing observed" snapshot.
    pub const fn unavailable() -> Self {
        IhotelSnapshot {
            process_found: false,
            main_window_enabled: None,
            selection_pending: false,
            refresh_button_hwnd: None,
        }
    }

    /// Project down to the `GET /ihotel/status` wire shape.
    pub const fn status(&self) -> Snapshot {
        Snapshot {
            process_found: self.process_found,
            grid_window_found: self.refresh_button_hwnd.is_some(),
        }
    }
}

// -- Decision -------------------------------------------------------------

/// Why a refresh was not performed. Every variant is a stable,
/// machine-readable string on the `POST /ihotel/refresh` wire (`reason`)
/// **and** carries the Thai sentence reception sees on the toast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Error)]
#[serde(rename_all = "kebab-case")]
pub enum SkipReason {
    /// Guard (a): no process matching [`Targets::process_name`].
    #[error("process-not-found")]
    ProcessNotFound,
    /// Guard (b): iHOTEL's top-level shell is disabled, which is what a
    /// WinForms `ShowDialog` modal does to its owner. Clicking behind a
    /// modal is the classic way to corrupt a half-finished check-in.
    #[error("modal-open")]
    ModalOpen,
    /// Guard (c): the selection button is visible, i.e. reception has a
    /// multi-room tap-selection in flight that `ClearCheck()` would destroy
    /// (`ROOM_GRID_REFRESH.md` §5).
    #[error("selection-pending")]
    SelectionPending,
    /// Guard (d): the Refresh button could not be resolved — the room grid
    /// is not open, or the AutomationId no longer matches.
    #[error("grid-not-found")]
    GridNotFound,
    /// **Not a guard.** The Windows probe exceeded its own deadline, which
    /// in practice means iHOTEL's message pump is wedged and UI Automation's
    /// cross-process calls are hanging. Reported rather than folded into
    /// `process-not-found`, because "I could not tell" and "it is not
    /// running" are operationally very different answers.
    ///
    /// Only the Windows probe can construct this, so a non-Windows build
    /// legitimately has no constructor for it. The variant still exists on
    /// every target on purpose: the `POST /ihotel/refresh` wire contract
    /// must not vary by build host.
    #[cfg_attr(not(target_os = "windows"), allow(dead_code))]
    #[error("probe-timeout")]
    ProbeTimeout,
    /// **Not a guard.** Every guard passed but `PostMessage` itself failed —
    /// almost always because the window was destroyed in the microseconds
    /// between the probe and the post.
    #[error("post-failed")]
    PostFailed,
}

impl SkipReason {
    /// The Thai sentence shown to reception. Each one says what happened
    /// **and** what she can do about it — a toast that only says "failed"
    /// trains her to dismiss toasts, which ADR 0006 §5 treats as the primary
    /// failure mode of this whole feature.
    pub const fn toast_text(&self) -> &'static str {
        match self {
            SkipReason::ProcessNotFound => {
                "ไม่พบโปรแกรม iHOTEL ที่เปิดอยู่ กรุณาเปิด iHOTEL ก่อน แล้วลองใหม่อีกครั้ง"
            }
            SkipReason::ModalOpen => {
                "iHOTEL มีหน้าต่างอื่นเปิดค้างอยู่ จึงยังไม่รีเฟรชให้ กรุณาปิดหน้าต่างนั้นก่อน แล้วลองใหม่อีกครั้ง"
            }
            SkipReason::SelectionPending => {
                "มีการเลือกห้องค้างอยู่ใน iHOTEL จึงยังไม่รีเฟรชให้ (การรีเฟรชจะล้างห้องที่เลือกไว้) กรุณาทำรายการให้เสร็จก่อน"
            }
            SkipReason::GridNotFound => {
                "ไม่พบหน้าจอผังห้องพักของ iHOTEL กรุณาเปิดหน้าผังห้องพัก แล้วลองใหม่อีกครั้ง"
            }
            SkipReason::ProbeTimeout => {
                "iHOTEL ไม่ตอบสนอง จึงยังไม่รีเฟรชให้ กรุณากดปุ่ม Refresh ที่หน้าจอผังห้องพักด้วยตนเอง"
            }
            SkipReason::PostFailed => {
                "ส่งคำสั่งรีเฟรชไปยัง iHOTEL ไม่สำเร็จ กรุณากดปุ่ม Refresh ที่หน้าจอผังห้องพักด้วยตนเอง"
            }
        }
    }
}

/// What the guards concluded from an [`IhotelSnapshot`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// All guards passed: post a click to this native window handle.
    Click { hwnd: isize },
    /// Do nothing, and tell reception why.
    Skip(SkipReason),
}

/// The whole guard policy, as a pure function.
///
/// **Order is the contract**, and it is priority order, not convenience
/// order — an earlier guard's answer must win even when a later field also
/// looks bad, because the earlier answer is the more actionable one for
/// reception ("iHOTEL is not open" beats "the grid is not open"):
///
/// 1. [`SkipReason::ProcessNotFound`] — nothing to talk to.
/// 2. [`SkipReason::ModalOpen`] — a `ShowDialog` is up; do not poke behind it.
/// 3. [`SkipReason::SelectionPending`] — refusing to destroy her selection.
/// 4. [`SkipReason::GridNotFound`] — the Refresh button is not there.
///
/// Note the deliberate gap: `main_window_enabled == None` (process alive,
/// no top-level window) is **not** `modal-open`. A missing window is not a
/// disabled window, and telling reception to "close the dialog" when there
/// is no dialog would be a lie. It falls through to `grid-not-found`, which
/// is exactly what she can act on (open the room grid).
pub fn decide(snapshot: IhotelSnapshot) -> Decision {
    if !snapshot.process_found {
        return Decision::Skip(SkipReason::ProcessNotFound);
    }
    if snapshot.main_window_enabled == Some(false) {
        return Decision::Skip(SkipReason::ModalOpen);
    }
    if snapshot.selection_pending {
        return Decision::Skip(SkipReason::SelectionPending);
    }
    match snapshot.refresh_button_hwnd {
        // A zero handle is the null HWND — resolvable-but-useless, which is
        // the same practical situation as not resolving at all.
        Some(hwnd) if hwnd != 0 => Decision::Click { hwnd },
        _ => Decision::Skip(SkipReason::GridNotFound),
    }
}

// -- Public seam ----------------------------------------------------------

/// Probe iHOTEL's live state.
///
/// The only `Err` is [`SkipReason::ProbeTimeout`], and only on Windows.
/// Non-Windows targets always report [`IhotelSnapshot::unavailable`] — there
/// is no iHOTEL process to find, and saying so honestly is what lets every
/// caller exist unchanged on this crate's macOS CI leg.
pub fn probe() -> Result<IhotelSnapshot, SkipReason> {
    #[cfg(target_os = "windows")]
    {
        windows_impl::probe()
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(IhotelSnapshot::unavailable())
    }
}

/// Take a snapshot for `GET /ihotel/status` and the tray's status item.
///
/// A probe timeout degrades to [`Snapshot::unavailable`] here rather than
/// propagating: a status read has nothing useful to do with the distinction,
/// and the endpoint should never fail because iHOTEL is being slow.
pub fn snapshot() -> Snapshot {
    probe().map(|s| s.status()).unwrap_or_else(|reason| {
        eprintln!("[ihotel] status probe failed: {reason}");
        Snapshot::unavailable()
    })
}

/// The single code path both `POST /ihotel/refresh` and the tray's
/// "รีเฟรช iHOTEL" item call, so the two surfaces can never disagree about
/// what "refresh" does.
///
/// On any skip this raises the explanatory Thai toast itself — one place, so
/// a future third caller cannot forget to — and returns the reason. Callers
/// must **not** clear the staleness latch on `Err`: nothing was refreshed,
/// so the grid is still stale and the episode is still open.
pub fn trigger_refresh() -> Result<(), SkipReason> {
    let outcome = perform_refresh();
    if let Err(reason) = outcome {
        println!("[ihotel] refresh skipped: {reason}");
        toast::show(SKIP_TOAST_TITLE, reason.toast_text());
    }
    outcome
}

fn perform_refresh() -> Result<(), SkipReason> {
    match decide(probe()?) {
        Decision::Skip(reason) => Err(reason),
        Decision::Click { hwnd } => post_click(hwnd),
    }
}

#[cfg(target_os = "windows")]
fn post_click(hwnd: isize) -> Result<(), SkipReason> {
    windows_impl::post_click(hwnd)
}

/// Unreachable in practice off Windows — [`probe`] reports
/// `process_found: false` there, so [`decide`] always short-circuits to
/// `process-not-found` long before this. Written honestly rather than as an
/// `unreachable!()` so a future non-Windows probe can't turn a wrong
/// assumption into a panic on a reception PC.
#[cfg(not(target_os = "windows"))]
fn post_click(_hwnd: isize) -> Result<(), SkipReason> {
    Err(SkipReason::PostFailed)
}

/// Log the resolved [`Targets`] exactly once per process.
///
/// Worth its own function because the single most likely field failure for
/// this feature is a silent AutomationId mismatch after a vendor update —
/// having the effective ids in the log means the first diagnostic step is
/// reading, not guessing.
pub fn log_targets_once() {
    static ONCE: OnceLock<()> = OnceLock::new();
    ONCE.get_or_init(|| {
        let t = targets();
        println!(
            "[ihotel] targets: process={} refresh_button={} selection_button={}",
            t.process_name, t.refresh_button_id, t.selection_button_id
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, MutexGuard};

    /// `std::env` is process-global; these tests mutate three vars that
    /// [`targets`] reads. Serialize them so a parallel test runner can't
    /// interleave a set with another test's read.
    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn env_guard() -> MutexGuard<'static, ()> {
        ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn clear_env() {
        std::env::remove_var(ENV_PROCESS_NAME);
        std::env::remove_var(ENV_REFRESH_BUTTON_ID);
        std::env::remove_var(ENV_SELECTION_BUTTON_ID);
    }

    /// A snapshot with every guard passing and a usable handle.
    fn healthy() -> IhotelSnapshot {
        IhotelSnapshot {
            process_found: true,
            main_window_enabled: Some(true),
            selection_pending: false,
            refresh_button_hwnd: Some(0x1234),
        }
    }

    // -- decide(): the full ordering matrix -------------------------------

    #[test]
    fn healthy_snapshot_clicks_the_resolved_handle() {
        assert_eq!(decide(healthy()), Decision::Click { hwnd: 0x1234 });
    }

    #[test]
    fn process_not_found_wins_over_every_later_guard() {
        // Every later field is ALSO in a skip-worthy state, plus a usable
        // handle. Priority order must still surface the first guard.
        let snap = IhotelSnapshot {
            process_found: false,
            main_window_enabled: Some(false),
            selection_pending: true,
            refresh_button_hwnd: Some(0x1234),
        };
        assert_eq!(decide(snap), Decision::Skip(SkipReason::ProcessNotFound));
    }

    #[test]
    fn process_not_found_even_with_otherwise_healthy_fields() {
        let snap = IhotelSnapshot {
            process_found: false,
            ..healthy()
        };
        assert_eq!(decide(snap), Decision::Skip(SkipReason::ProcessNotFound));
    }

    #[test]
    fn modal_open_wins_over_selection_and_grid() {
        let snap = IhotelSnapshot {
            main_window_enabled: Some(false),
            selection_pending: true,
            refresh_button_hwnd: None,
            ..healthy()
        };
        assert_eq!(decide(snap), Decision::Skip(SkipReason::ModalOpen));
    }

    #[test]
    fn modal_open_beats_an_otherwise_clickable_button() {
        let snap = IhotelSnapshot {
            main_window_enabled: Some(false),
            ..healthy()
        };
        assert_eq!(decide(snap), Decision::Skip(SkipReason::ModalOpen));
    }

    #[test]
    fn absent_main_window_is_not_modal_open() {
        // The documented gap: process alive, no top-level window observed.
        // "Close the dialog" would be a lie; "the grid is not there" is the
        // actionable truth.
        let snap = IhotelSnapshot {
            main_window_enabled: None,
            refresh_button_hwnd: None,
            ..healthy()
        };
        assert_eq!(decide(snap), Decision::Skip(SkipReason::GridNotFound));
    }

    #[test]
    fn selection_pending_wins_over_grid_not_found() {
        let snap = IhotelSnapshot {
            selection_pending: true,
            refresh_button_hwnd: None,
            ..healthy()
        };
        assert_eq!(decide(snap), Decision::Skip(SkipReason::SelectionPending));
    }

    #[test]
    fn selection_pending_beats_a_perfectly_clickable_button() {
        // The single most important guard: we would rather do nothing than
        // wipe reception's in-progress multi-room selection.
        let snap = IhotelSnapshot {
            selection_pending: true,
            ..healthy()
        };
        assert_eq!(decide(snap), Decision::Skip(SkipReason::SelectionPending));
    }

    #[test]
    fn grid_not_found_when_handle_missing() {
        let snap = IhotelSnapshot {
            refresh_button_hwnd: None,
            ..healthy()
        };
        assert_eq!(decide(snap), Decision::Skip(SkipReason::GridNotFound));
    }

    #[test]
    fn grid_not_found_when_handle_is_null() {
        let snap = IhotelSnapshot {
            refresh_button_hwnd: Some(0),
            ..healthy()
        };
        assert_eq!(decide(snap), Decision::Skip(SkipReason::GridNotFound));
    }

    #[test]
    fn unavailable_snapshot_decides_process_not_found() {
        assert_eq!(
            decide(IhotelSnapshot::unavailable()),
            Decision::Skip(SkipReason::ProcessNotFound)
        );
    }

    #[test]
    fn decide_never_produces_a_non_guard_reason() {
        // ProbeTimeout / PostFailed are executor outcomes, not guard
        // outcomes. Sweep the whole 2x3x2x2 input space and assert decide()
        // can only ever emit the four documented guards.
        for process_found in [false, true] {
            for main_window_enabled in [None, Some(false), Some(true)] {
                for selection_pending in [false, true] {
                    for refresh_button_hwnd in [None, Some(0), Some(7)] {
                        let snap = IhotelSnapshot {
                            process_found,
                            main_window_enabled,
                            selection_pending,
                            refresh_button_hwnd,
                        };
                        if let Decision::Skip(reason) = decide(snap) {
                            assert!(
                                matches!(
                                    reason,
                                    SkipReason::ProcessNotFound
                                        | SkipReason::ModalOpen
                                        | SkipReason::SelectionPending
                                        | SkipReason::GridNotFound
                                ),
                                "decide() emitted a non-guard reason {reason} for {snap:?}"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn decide_clicks_only_when_every_guard_passes() {
        for process_found in [false, true] {
            for main_window_enabled in [None, Some(false), Some(true)] {
                for selection_pending in [false, true] {
                    for refresh_button_hwnd in [None, Some(0), Some(7)] {
                        let snap = IhotelSnapshot {
                            process_found,
                            main_window_enabled,
                            selection_pending,
                            refresh_button_hwnd,
                        };
                        let expected_click = process_found
                            && main_window_enabled != Some(false)
                            && !selection_pending
                            && refresh_button_hwnd.is_some_and(|h| h != 0);
                        assert_eq!(
                            matches!(decide(snap), Decision::Click { .. }),
                            expected_click,
                            "unexpected click verdict for {snap:?}"
                        );
                    }
                }
            }
        }
    }

    // -- Skip reason wire strings + toast texts ---------------------------

    const ALL_REASONS: [SkipReason; 6] = [
        SkipReason::ProcessNotFound,
        SkipReason::ModalOpen,
        SkipReason::SelectionPending,
        SkipReason::GridNotFound,
        SkipReason::ProbeTimeout,
        SkipReason::PostFailed,
    ];

    #[test]
    fn skip_reasons_match_the_documented_wire_strings() {
        assert_eq!(SkipReason::ProcessNotFound.to_string(), "process-not-found");
        assert_eq!(SkipReason::ModalOpen.to_string(), "modal-open");
        assert_eq!(
            SkipReason::SelectionPending.to_string(),
            "selection-pending"
        );
        assert_eq!(SkipReason::GridNotFound.to_string(), "grid-not-found");
        assert_eq!(SkipReason::ProbeTimeout.to_string(), "probe-timeout");
        assert_eq!(SkipReason::PostFailed.to_string(), "post-failed");
    }

    #[test]
    fn skip_reasons_serialize_to_the_same_strings_as_display() {
        // `reason` on the POST /ihotel/refresh wire is built from Display;
        // the enum is also Serialize. Keep the two from ever drifting.
        for reason in ALL_REASONS {
            let json = serde_json::to_string(&reason).unwrap();
            assert_eq!(json, format!("\"{reason}\""));
        }
    }

    #[test]
    fn every_skip_reason_has_a_distinct_thai_toast_text() {
        let mut seen: Vec<&str> = Vec::new();
        for reason in ALL_REASONS {
            let text = reason.toast_text();
            assert!(!text.trim().is_empty(), "{reason} has an empty toast text");
            assert!(
                text.chars().any(|c| ('\u{0e00}'..='\u{0e7f}').contains(&c)),
                "{reason}'s toast text is not Thai: {text}"
            );
            assert!(
                !seen.contains(&text),
                "{reason} reuses another reason's toast text"
            );
            seen.push(text);
        }
    }

    #[test]
    fn skip_toast_text_fits_the_stale_module_length_budget() {
        // Toasts are rendered by the same Windows surface as the
        // legacy-stale ones, which crate::stale caps at MAX_TOAST_CHARS.
        // Ours are static, so a regression here is a compile-time-known
        // authoring mistake, not a runtime surprise.
        for reason in ALL_REASONS {
            assert!(
                reason.toast_text().chars().count() <= crate::stale::MAX_TOAST_CHARS,
                "{reason}'s toast text exceeds MAX_TOAST_CHARS"
            );
        }
    }

    // -- Snapshot projection ----------------------------------------------

    #[test]
    fn status_projection_reports_grid_found_from_the_resolved_handle() {
        assert_eq!(
            healthy().status(),
            Snapshot {
                process_found: true,
                grid_window_found: true,
            }
        );
    }

    #[test]
    fn status_projection_reports_grid_missing_without_a_handle() {
        let snap = IhotelSnapshot {
            refresh_button_hwnd: None,
            ..healthy()
        };
        assert_eq!(
            snap.status(),
            Snapshot {
                process_found: true,
                grid_window_found: false,
            }
        );
    }

    #[test]
    fn unavailable_constants_agree() {
        assert_eq!(IhotelSnapshot::unavailable().status(), Snapshot::unavailable());
        assert_eq!(
            Snapshot::unavailable(),
            Snapshot {
                process_found: false,
                grid_window_found: false,
            }
        );
    }

    // -- Live seam on this target -----------------------------------------

    #[test]
    fn probe_is_honestly_unavailable_on_this_target() {
        // On the macOS CI leg this exercises the cfg(not(windows)) branch
        // directly; on Windows it is a real probe against a machine that is
        // not expected to be running iHOTEL in CI.
        let snap = probe().unwrap_or(IhotelSnapshot::unavailable());
        assert!(!snap.process_found);
        assert!(!snap.status().grid_window_found);
    }

    #[test]
    fn trigger_refresh_reports_process_not_found_on_this_target() {
        assert_eq!(trigger_refresh(), Err(SkipReason::ProcessNotFound));
    }

    // -- Env override plumbing --------------------------------------------

    #[test]
    fn targets_default_to_the_ihotel_ids() {
        let _guard = env_guard();
        clear_env();
        let t = targets();
        assert_eq!(t.process_name, "HOTEL.exe");
        assert_eq!(t.refresh_button_id, "ButtonX3");
        assert_eq!(t.selection_button_id, "ButtonX6");
        assert_eq!(t, Targets::default());
    }

    #[test]
    fn targets_honor_env_overrides() {
        let _guard = env_guard();
        clear_env();
        std::env::set_var(ENV_PROCESS_NAME, "powershell.exe");
        std::env::set_var(ENV_REFRESH_BUTTON_ID, "btnRefreshV2");
        std::env::set_var(ENV_SELECTION_BUTTON_ID, "btnConfirmV2");

        let t = targets();
        assert_eq!(t.process_name, "powershell.exe");
        assert_eq!(t.refresh_button_id, "btnRefreshV2");
        assert_eq!(t.selection_button_id, "btnConfirmV2");

        clear_env();
    }

    #[test]
    fn targets_trim_whitespace_and_ignore_blank_overrides() {
        let _guard = env_guard();
        clear_env();
        std::env::set_var(ENV_REFRESH_BUTTON_ID, "  ButtonX9  ");
        // Blank/whitespace-only is treated as unset, not as "match nothing".
        std::env::set_var(ENV_SELECTION_BUTTON_ID, "   ");

        let t = targets();
        assert_eq!(t.refresh_button_id, "ButtonX9");
        assert_eq!(t.selection_button_id, DEFAULT_SELECTION_BUTTON_ID);

        clear_env();
    }
}
