//! The iHOTEL process/window-probing seam.
//!
//! Stage 3 (this module, as first shipped) is a clean, honest STUB: it
//! never claims to see a running `HOTEL.exe` or its room-grid window,
//! because it never actually looks. Stage 4 fills in the real Windows
//! UI Automation (UIA) implementation behind `windows_impl::snapshot`
//! below — that is the ONLY function Stage 4 needs to change; every caller
//! (`server.rs`'s `GET /ihotel/status`, `main.rs`'s tray "สถานะ iHOTEL"
//! item) already goes through [`snapshot`].
//!
//! **Why a seam and not just a `todo!()`.** `GET /ihotel/status` and the
//! tray status item both need *something* to call today, on macOS (this
//! crate's CI leg) as well as on the Windows reception PCs this middleware
//! actually runs on. A stub that honestly reports "not found" on every
//! platform lets both callers exist now, compiles everywhere, and gives
//! Stage 4 a single, obvious insertion point instead of a search-and-modify
//! across two call sites.
//!
//! **Why this never drives iHOTEL.** ADR 0006 Decision §1
//! (`docs/adr/0006-legacy-stale-notification.md`) is explicit: this
//! middleware notifies, it never automates the refresh. [`trigger_refresh`]
//! exists as the single code path both `POST /ihotel/refresh` and the
//! tray's "รีเฟรช iHOTEL" item call, but even once Stage 4 makes it real,
//! it is reception-invoked (a click) — never something a writeback commit
//! triggers on its own. [`snapshot`] is read-only by construction: it has
//! no way to return anything BUT an observation.
//!
//! See also: `docs/legacy-app/ROOM_GRID_REFRESH.md` §10 — `FormRoomMain` is
//! an MDI child of `frmMain1`, never the top-level window, which is the
//! first thing Stage 4's window enumeration needs to account for (a modal
//! check-in/check-out dialog disables the MDI *parent*, not `FormRoomMain`
//! directly).

use serde::Serialize;
use thiserror::Error;

/// A point-in-time read of whether iHOTEL is running / has its room grid
/// open, as far as this middleware can tell from outside the process.
/// Everything here is a best-effort OBSERVATION, never a control — see the
/// module docs: this middleware never drives iHOTEL, it only notices things
/// about it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Snapshot {
    /// Whether a `HOTEL.exe` process was found running on this machine.
    pub process_found: bool,
    /// Whether iHOTEL's room-grid window (`FormRoomMain`) was found.
    pub grid_window_found: bool,
}

impl Snapshot {
    /// The honest "nothing observed" snapshot — what every non-Windows
    /// target reports unconditionally, and what the Windows stub reports
    /// until Stage 4 wires real probing in.
    pub const fn unavailable() -> Self {
        Snapshot {
            process_found: false,
            grid_window_found: false,
        }
    }
}

/// Take a snapshot of iHOTEL's process/window state on this machine.
///
/// Non-Windows targets (this crate's macOS CI leg, any dev machine) always
/// report [`Snapshot::unavailable`] — there is no iHOTEL process to find.
/// On Windows, Stage 3 ships the same honest stub; Stage 4 replaces
/// `windows_impl::snapshot`'s body with real UIA probing.
pub fn snapshot() -> Snapshot {
    #[cfg(target_os = "windows")]
    {
        windows_impl::snapshot()
    }
    #[cfg(not(target_os = "windows"))]
    {
        Snapshot::unavailable()
    }
}

/// Errors [`trigger_refresh`] can return.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Error)]
#[serde(rename_all = "kebab-case")]
pub enum RefreshError {
    /// Stage 4 has not shipped the real refresh action yet. Per ADR 0006
    /// Decision §1, that action will always be reception-invoked (a click
    /// on the toast or the tray item), never automatic — this is the
    /// stub's only possible error today.
    #[error("refresh-not-available")]
    RefreshNotAvailable,
}

/// The single code path both `POST /ihotel/refresh` and the tray's
/// "รีเฟรช iHOTEL" menu item call, so the two surfaces can never disagree
/// about what "refresh" does. Stage 4 owns making this real (driving
/// iHOTEL's own `ButtonX3`/Refresh — see `ROOM_GRID_REFRESH.md` §5); today
/// it unconditionally reports unavailable.
pub fn trigger_refresh() -> Result<(), RefreshError> {
    Err(RefreshError::RefreshNotAvailable)
}

#[cfg(target_os = "windows")]
mod windows_impl {
    use super::Snapshot;

    /// Stage 4 fills this in with real process/window probing (enumerate
    /// processes for `HOTEL.exe`, then locate the `FormRoomMain` MDI
    /// child — see the module doc's note on `ROOM_GRID_REFRESH.md` §10).
    /// Kept as its own function, not inlined into [`super::snapshot`], so
    /// the seam is a single obvious line to change.
    pub fn snapshot() -> Snapshot {
        Snapshot::unavailable()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_is_honestly_unavailable_on_this_target() {
        // This test runs on whatever target `cargo test` is built for. On
        // the macOS CI leg (and any non-Windows dev machine) it exercises
        // the cfg(not(windows)) branch directly; on Windows it exercises
        // today's stub, which is equally `unavailable()` until Stage 4.
        let snap = snapshot();
        assert!(!snap.process_found);
        assert!(!snap.grid_window_found);
    }

    #[test]
    fn unavailable_constant_matches_snapshot_default_shape() {
        assert_eq!(
            Snapshot::unavailable(),
            Snapshot {
                process_found: false,
                grid_window_found: false,
            }
        );
    }

    #[test]
    fn trigger_refresh_is_unavailable_until_stage_4() {
        assert_eq!(trigger_refresh(), Err(RefreshError::RefreshNotAvailable));
    }

    #[test]
    fn refresh_error_display_matches_wire_string() {
        // POST /ihotel/refresh's JSON body reuses this Display impl for its
        // `error` field — keep the two from drifting apart.
        assert_eq!(
            RefreshError::RefreshNotAvailable.to_string(),
            "refresh-not-available"
        );
    }

    #[test]
    fn refresh_error_serializes_kebab_case() {
        let json = serde_json::to_string(&RefreshError::RefreshNotAvailable).unwrap();
        assert_eq!(json, "\"refresh-not-available\"");
    }
}
