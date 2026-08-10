//! The Windows half of [`crate::ihotel`]: fill in an
//! [`IhotelSnapshot`], and post the click the pure [`crate::ihotel::decide`]
//! asked for. **No policy lives here** — every "should we?" question is
//! answered by `decide`, which is testable on macOS. This file only answers
//! "what is actually on the machine?" and "do the thing".
//!
//! Every API used here is from the `windows` crate (see `Cargo.toml`'s
//! `[target."cfg(windows)".dependencies]`), pinned to the same 0.61.x that
//! `tauri` already resolves so no second copy of the Windows metadata gets
//! compiled.
//!
//! # Threading and COM
//!
//! UI Automation is COM. Rather than call `CoInitializeEx` on whichever
//! tokio worker or Tauri thread happens to be running the request — and
//! risk `RPC_E_CHANGED_MODE` against an apartment somebody else established,
//! or worse, leaving one initialized behind — every probe runs on its own
//! short-lived thread that owns its apartment for exactly the duration of
//! the probe.
//!
//! That thread is waited on with a **deadline, and is deliberately never
//! joined**. UIA calls are cross-process: against a wedged `HOTEL.exe` whose
//! message pump has stopped, they can block for a long time. Joining would
//! hand that hang straight to an HTTP handler or, worse, to the tray menu —
//! visible to the receptionist as a frozen shell. Instead we abandon the
//! thread, report [`SkipReason::ProbeTimeout`], and let it die whenever the
//! OS unblocks it. That is the same reasoning that rules out `SendMessage`
//! in [`post_click`].
//!
//! [`IhotelSnapshot`]: crate::ihotel::IhotelSnapshot

use std::mem::size_of;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use windows::core::BOOL;
use windows::Win32::Foundation::{
    CloseHandle, HWND, LPARAM, RECT, RPC_E_CHANGED_MODE, WPARAM,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED,
};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Variant::VARIANT;
use windows::Win32::UI::Accessibility::{
    CUIAutomation, IUIAutomation, IUIAutomationElement, TreeScope_Descendants,
    UIA_AutomationIdPropertyId,
};
use windows::Win32::UI::Input::KeyboardAndMouse::IsWindowEnabled;
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetClientRect, GetWindow, GetWindowLongPtrW, GetWindowThreadProcessId,
    IsWindowVisible, PostMessageW, GWL_STYLE, GW_OWNER, WM_LBUTTONDOWN, WM_LBUTTONUP, WS_VISIBLE,
};

use super::{IhotelSnapshot, SkipReason, Targets};

/// How long a probe may take before we give up on it.
///
/// Sized for the pathological case, not the normal one: a healthy probe is
/// a handful of milliseconds (one ToolHelp snapshot, one window enumeration,
/// two UIA `FindFirst` calls). Five seconds is generous enough that a merely
/// busy `HOTEL.exe` still answers, and short enough that a receptionist who
/// clicked the tray item gets a Thai explanation instead of a frozen menu.
const PROBE_TIMEOUT: Duration = Duration::from_secs(5);

/// `MK_LBUTTON`, the wParam flag saying "the left button is down" that a
/// real `WM_LBUTTONDOWN` carries.
///
/// Value taken from `windows::Win32::System::SystemServices::MK_LBUTTON`
/// (`MODIFIERKEYS_FLAGS(1u32)`); inlined as a plain constant rather than
/// imported so this crate does not have to enable the whole
/// `Win32_System_SystemServices` feature for one integer.
const MK_LBUTTON: usize = 0x0001;

/// Fallback click point, in client coordinates, if `GetClientRect` fails.
/// Inside the top-left corner of any button big enough for a human to hit.
const FALLBACK_CLICK_POINT: (i32, i32) = (5, 5);

// -- Probe ----------------------------------------------------------------

/// Fill in an [`IhotelSnapshot`] from the live machine.
pub fn probe() -> Result<IhotelSnapshot, SkipReason> {
    let targets = super::targets();
    run_with_deadline(move || probe_inner(&targets)).ok_or(SkipReason::ProbeTimeout)
}

/// Run `work` on a throwaway thread and wait at most [`PROBE_TIMEOUT`].
///
/// Returns `None` on timeout (or if the thread could not even be spawned).
/// See the module docs for why the thread is abandoned rather than joined.
fn run_with_deadline<T, F>(work: F) -> Option<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    let spawned = thread::Builder::new()
        .name("ihotel-uia-probe".to_string())
        .spawn(move || {
            // The receiver may already be gone (we timed out); a failed send
            // is the expected, uninteresting case, not an error.
            let _ = tx.send(work());
        });

    if let Err(e) = spawned {
        eprintln!("[ihotel] could not spawn probe thread: {e}");
        return None;
    }

    rx.recv_timeout(PROBE_TIMEOUT).ok()
}

fn probe_inner(targets: &Targets) -> IhotelSnapshot {
    let pids = matching_pids(&targets.process_name);
    if pids.is_empty() {
        // Nothing else can be true, and every subsequent call would be
        // wasted work — including COM initialization.
        return IhotelSnapshot::unavailable();
    }

    let Some(main_window) = find_shell_window(&pids) else {
        return IhotelSnapshot {
            process_found: true,
            main_window_enabled: None,
            selection_pending: false,
            refresh_button_hwnd: None,
        };
    };

    // SAFETY: `main_window` came from EnumWindows moments ago. A window that
    // died in between makes this return FALSE, which reads as "modal open" —
    // a conservative skip, never a stray click.
    let enabled = unsafe { IsWindowEnabled(main_window) }.as_bool();

    let _com = ComApartment::enter();
    let (selection_pending, refresh_button_hwnd) = match find_buttons(main_window, targets) {
        Ok(found) => found,
        Err(e) => {
            eprintln!("[ihotel] UI Automation probe failed: {e}");
            (false, None)
        }
    };

    IhotelSnapshot {
        process_found: true,
        main_window_enabled: Some(enabled),
        selection_pending,
        refresh_button_hwnd,
    }
}

// -- COM apartment --------------------------------------------------------

/// Owns a COM apartment for the lifetime of a probe.
struct ComApartment {
    /// Whether *we* were the ones who initialized it, and therefore owe a
    /// matching `CoUninitialize`. Uninitializing an apartment we did not
    /// establish would break whoever did.
    owns_init: bool,
}

impl ComApartment {
    fn enter() -> Self {
        // SAFETY: plain COM initialization on a thread we just created.
        let hr = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) };
        if hr == RPC_E_CHANGED_MODE {
            // Somebody already made this an STA. COM is still usable; we
            // just must not balance an initialization we did not perform.
            return ComApartment { owns_init: false };
        }
        if !hr.is_ok() {
            eprintln!("[ihotel] CoInitializeEx failed: {hr:?}");
        }
        ComApartment {
            owns_init: hr.is_ok(),
        }
    }
}

impl Drop for ComApartment {
    fn drop(&mut self) {
        if self.owns_init {
            // SAFETY: balances exactly one successful CoInitializeEx above.
            unsafe { CoUninitialize() };
        }
    }
}

// -- Process enumeration --------------------------------------------------

/// Every PID whose image name matches `process_name`, case-insensitively.
///
/// Uses a ToolHelp snapshot rather than `OpenProcess` +
/// `QueryFullProcessImageNameW` on purpose: the snapshot hands back image
/// names without opening a handle to anything, so this works without any
/// elevated rights even when `HOTEL.exe` runs as a different console user.
fn matching_pids(process_name: &str) -> Vec<u32> {
    let mut pids = Vec::new();

    // SAFETY: standard ToolHelp usage; the handle is closed on every path.
    let snapshot = match unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) } {
        Ok(handle) => handle,
        Err(e) => {
            eprintln!("[ihotel] CreateToolhelp32Snapshot failed: {e}");
            return pids;
        }
    };

    let mut entry = PROCESSENTRY32W {
        dwSize: size_of::<PROCESSENTRY32W>() as u32,
        ..Default::default()
    };

    // SAFETY: `entry.dwSize` is set as the API requires; `snapshot` is live.
    if unsafe { Process32FirstW(snapshot, &mut entry) }.is_ok() {
        loop {
            if wide_to_string(&entry.szExeFile).eq_ignore_ascii_case(process_name) {
                pids.push(entry.th32ProcessID);
            }
            // SAFETY: same live snapshot; Err simply means "no more rows".
            if unsafe { Process32NextW(snapshot, &mut entry) }.is_err() {
                break;
            }
        }
    }

    // SAFETY: `snapshot` is a valid handle we own and never use again.
    unsafe {
        let _ = CloseHandle(snapshot);
    }

    pids
}

/// Read a NUL-terminated fixed-size UTF-16 buffer as a `String`.
fn wide_to_string(buffer: &[u16]) -> String {
    let len = buffer.iter().position(|&c| c == 0).unwrap_or(buffer.len());
    String::from_utf16_lossy(&buffer[..len])
}

// -- Window enumeration ---------------------------------------------------

/// Context handed to [`enum_windows_proc`] through `EnumWindows`' lParam.
/// Owns its PID list (rather than borrowing) so no lifetime has to survive
/// the round-trip through a raw `isize`.
struct ShellSearch {
    pids: Vec<u32>,
    found: Option<HWND>,
}

/// Find iHOTEL's top-level shell window (`frmMain1`).
///
/// The filter is: belongs to one of our PIDs, is visible, and has **no
/// owner**. That last clause is what separates the MDI shell from its
/// dialogs — WinForms gives every `ShowDialog` an owner, main forms have
/// none. It matters because guard (b) asks whether the *shell* is disabled,
/// which is precisely the state a modal puts it in
/// (`ROOM_GRID_REFRESH.md` §10: `FormRoomMain` is an MDI child, so a modal
/// disables the parent, not the grid itself).
fn find_shell_window(pids: &[u32]) -> Option<HWND> {
    let mut search = ShellSearch {
        pids: pids.to_vec(),
        found: None,
    };

    // SAFETY: the pointer stays valid for the whole synchronous call, and
    // the callback only ever dereferences it on this thread. EnumWindows
    // reports Err when the callback stops enumeration early — which is our
    // success path — so the result is deliberately discarded.
    let _ = unsafe {
        EnumWindows(
            Some(enum_windows_proc),
            LPARAM(&mut search as *mut ShellSearch as isize),
        )
    };

    search.found
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    const CONTINUE: BOOL = BOOL(1);
    const STOP: BOOL = BOOL(0);

    // SAFETY: lParam is the `&mut ShellSearch` set up in find_shell_window.
    let search = unsafe { &mut *(lparam.0 as *mut ShellSearch) };

    let mut pid = 0u32;
    // SAFETY: `hwnd` comes from EnumWindows; `pid` is a live local. The
    // raw-pointer cast is spelled out rather than left to coercion because
    // this file is never compiled on the dev machine.
    unsafe { GetWindowThreadProcessId(hwnd, Some(&mut pid as *mut u32)) };
    if pid == 0 || !search.pids.contains(&pid) {
        return CONTINUE;
    }

    // SAFETY: plain reads on an enumerated top-level window.
    unsafe {
        if !IsWindowVisible(hwnd).as_bool() {
            return CONTINUE;
        }
        // Ok(_) means the window HAS an owner, i.e. it is a dialog/popup.
        if GetWindow(hwnd, GW_OWNER).is_ok() {
            return CONTINUE;
        }
    }

    search.found = Some(hwnd);
    STOP
}

// -- UI Automation --------------------------------------------------------

/// Resolve both buttons in one UIA session.
///
/// Returns `(selection_pending, refresh_button_hwnd)`.
fn find_buttons(
    main_window: HWND,
    targets: &Targets,
) -> windows::core::Result<(bool, Option<isize>)> {
    // SAFETY: all of this is ordinary COM usage on a thread whose apartment
    // the caller entered; every interface is reference-counted by windows-rs.
    unsafe {
        let automation: IUIAutomation =
            CoCreateInstance(&CUIAutomation, None, CLSCTX_INPROC_SERVER)?;
        let root = automation.ElementFromHandle(main_window)?;

        // Guard (c) input. An absent handle means WinForms never created one
        // — the control has never been shown, so no selection was ever
        // started. That is a safe `false`, not an unknown.
        let selection_pending = find_native_handle(&automation, &root, &targets.selection_button_id)?
            .map(is_style_visible)
            .unwrap_or(false);

        // Guard (d) input. Also style-visibility-filtered, for the same
        // reason the whole module prefers a guarded skip: a Refresh button
        // whose WS_VISIBLE is clear belongs to a grid that is not showing,
        // and posting a click at it would be acting on a screen nobody has
        // open. Note this does NOT reject a merely covered or unfocused
        // grid — a control on a shown form keeps the style bit when it is
        // behind another window, which is precisely the case this feature
        // exists to serve.
        let refresh_button_hwnd =
            find_native_handle(&automation, &root, &targets.refresh_button_id)?
                .filter(|hwnd| is_style_visible(*hwnd))
                .map(|hwnd| hwnd.0 as isize);

        Ok((selection_pending, refresh_button_hwnd))
    }
}

/// Search `root`'s descendants for an element whose UIA `AutomationId`
/// equals `automation_id`, and return its native window handle.
///
/// AutomationId is the right key because WinForms projects a control's
/// `Name` (the designer field name — `ButtonX3`, `ButtonX6`) straight onto
/// it. Unlike a caption it is not localized, and unlike a position it
/// survives any relayout of the room grid.
unsafe fn find_native_handle(
    automation: &IUIAutomation,
    root: &IUIAutomationElement,
    automation_id: &str,
) -> windows::core::Result<Option<HWND>> {
    unsafe {
        let value = VARIANT::from(automation_id);
        let condition = automation.CreatePropertyCondition(UIA_AutomationIdPropertyId, &value)?;

        // UIA signals "no match" by handing back a null element, which
        // windows-rs surfaces as an Err — so a miss and a genuine failure
        // are indistinguishable here by design of the API. Both mean "we
        // could not resolve it", which is exactly what `None` says.
        let Ok(element) = root.FindFirst(TreeScope_Descendants, &condition) else {
            return Ok(None);
        };

        let hwnd = element.CurrentNativeWindowHandle()?;
        Ok((!hwnd.is_invalid()).then_some(hwnd))
    }
}

/// Whether a window carries the `WS_VISIBLE` **style bit**.
///
/// Deliberately the style and not UIA's `IsOffscreen`: `ClearCheck()` sets
/// `ButtonX6.Visible = false`, which clears this bit, whereas a button that
/// is merely scrolled out of view or covered by another window is offscreen
/// but still logically shown. Only the style answers "does reception have a
/// selection in flight?".
///
/// Read via `GetWindowLongPtrW` and narrowed to `u32` — window styles all
/// live in the low 32 bits, and the narrowing keeps this identical on a
/// 32-bit build where the API returns `i32` instead of `isize`.
fn is_style_visible(hwnd: HWND) -> bool {
    // SAFETY: a plain style read on a handle UIA just handed us.
    let style = unsafe { GetWindowLongPtrW(hwnd, GWL_STYLE) } as u32;
    style & WS_VISIBLE.0 != 0
}

// -- The click ------------------------------------------------------------

/// Post a synthetic left-click to `hwnd`.
///
/// A `WM_LBUTTONDOWN`/`WM_LBUTTONUP` pair, in that order, both carrying the
/// same client-relative point, because that is the sequence `Control`'s
/// click machinery is written against: the DOWN records that a press
/// happened on this control, the UP is what turns it into a `Click`.
/// `BM_CLICK` is not an option — `ButtonX` derives from `Control`, not
/// `ButtonBase`, so nothing in it handles that message and the call would
/// succeed while doing nothing at all.
///
/// **`PostMessage`, never `SendMessage`** (see the module docs), and never
/// `SendInput`: this must not move the receptionist's cursor or steal her
/// focus. Posting also means the click is queued behind whatever iHOTEL is
/// already doing, which is the polite ordering.
///
/// # KNOWN RUNTIME UNCERTAINTY — read before debugging "the click did nothing"
///
/// `Control.WmMouseUp` in WinForms raises `Click` only when **all** of
/// `ControlStyles.StandardClick`, `STATE_MOUSEPRESSED`, `!IsDisposed`, and
/// `WindowFromPoint(PointToScreen(lParam)) == Handle` hold. Verified
/// verbatim against Microsoft's published reference source for .NET
/// Framework `Control.cs` and against current `dotnet/winforms`
/// `Control.cs` — the check is unchanged between them, and `Button`
/// re-implements the same `WindowFromPoint` gate in its own `OnMouseUp`.
///
/// That last clause asks what is *actually on screen* at the click point.
/// So if `Control`'s standard click path is what raises `ButtonX3_Click`,
/// a posted click **will not fire `Click` while iHOTEL is covered by
/// another window** — `MouseDown` and `MouseUp` still fire, `Click` does
/// not — which is precisely the situation this feature exists for.
///
/// It may nonetheless work: `ButtonX` is a hand-painted DevComponents
/// control, and controls of that family commonly raise `Click` from their
/// own `OnMouseUp` override, which runs unconditionally and never consults
/// `WindowFromPoint`. We cannot tell from here, and `BM_CLICK` is not a
/// fallback — `ButtonBase.WndProc` is what intercepts that message, and
/// `ButtonX` does not derive from `ButtonBase`.
///
/// **This is measurable, and must be measured before trusting the feature:**
/// `scripts/dev/ihotel-uia-harness.ps1` row F drives exactly this case
/// against an obscured window and reads a click counter. Run it on the
/// harness AND once on a real terminal. The `Ok(())` returned here only
/// means the messages were queued — never that iHOTEL acted on them.
pub fn post_click(hwnd: isize) -> Result<(), SkipReason> {
    let hwnd = HWND(hwnd as *mut core::ffi::c_void);
    let (x, y) = client_centre(hwnd).unwrap_or(FALLBACK_CLICK_POINT);
    let lparam = make_lparam(x, y);

    // SAFETY: both calls are non-blocking queue posts. A destroyed window
    // fails cleanly rather than corrupting anything.
    unsafe {
        PostMessageW(Some(hwnd), WM_LBUTTONDOWN, WPARAM(MK_LBUTTON), lparam).map_err(|e| {
            eprintln!("[ihotel] PostMessage(WM_LBUTTONDOWN) failed: {e}");
            SkipReason::PostFailed
        })?;
        PostMessageW(Some(hwnd), WM_LBUTTONUP, WPARAM(0), lparam).map_err(|e| {
            // A dangling DOWN leaves the button visually pressed until the
            // next real mouse event. Loud, because it is a state we cannot
            // clean up from here.
            eprintln!("[ihotel] PostMessage(WM_LBUTTONUP) failed after a successful DOWN: {e}");
            SkipReason::PostFailed
        })?;
    }

    println!("[ihotel] posted refresh click to hwnd {hwnd:?} at client ({x}, {y})");
    Ok(())
}

/// Centre of a window's client area, in client coordinates.
fn client_centre(hwnd: HWND) -> Option<(i32, i32)> {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    // SAFETY: `rect` is a live local of the right type.
    unsafe { GetClientRect(hwnd, &mut rect) }.ok()?;
    if rect.right <= rect.left || rect.bottom <= rect.top {
        return None;
    }
    Some((
        (rect.right - rect.left) / 2,
        (rect.bottom - rect.top) / 2,
    ))
}

/// `MAKELPARAM(x, y)`: x in the low word, y in the high word, sign-extended
/// the way a real mouse message's lParam is.
fn make_lparam(x: i32, y: i32) -> LPARAM {
    let packed = (((y as u32) & 0xFFFF) << 16) | ((x as u32) & 0xFFFF);
    LPARAM(packed as i32 as isize)
}
