//! Windows toast display, via `tauri-winrt-notification`.
//!
//! Per `docs/adr/0006-legacy-stale-notification.md`'s Toast deliverable:
//! Stage 3 ships a plain, TEXT-ONLY toast — deliberately no "Refresh iHOTEL
//! now" action button yet. Wiring a clickable action is held behind a human
//! verification gate (the pilot acceptance criteria in ADR 0006 §5: ≤1
//! toast per 30 minutes sustained over a shift, AND a direct question to
//! the receptionist about whether she used it). Shipping the button before
//! that pilot has run would make it easy to mistake "reception can dismiss
//! the toast" for "reception acts on the toast", which is exactly the
//! distinction the pilot exists to measure.
//!
//! Non-Windows targets (this crate's macOS CI leg, any dev machine) get a
//! log-only stub, so [`crate::stale::Action::ShowToast`] /
//! [`crate::stale::Action::UpdateCount`] handling in `server.rs` can be
//! exercised uniformly on every platform without `cfg` spreading past this
//! one module.

/// Show a plain toast with the given title and (already-sanitized, per
/// `crate::stale::sanitize`) body text.
pub fn show(title: &str, body: &str) {
    #[cfg(target_os = "windows")]
    {
        windows_impl::show(title, body);
    }
    #[cfg(not(target_os = "windows"))]
    {
        println!("[toast] {title}: {body}");
    }
}

#[cfg(target_os = "windows")]
mod windows_impl {
    use tauri_winrt_notification::{Duration, Toast};

    pub fn show(title: &str, body: &str) {
        let result = Toast::new(Toast::POWERSHELL_APP_ID)
            .title(title)
            .text1(body)
            .duration(Duration::Long)
            .show();

        if let Err(e) = result {
            eprintln!("[toast] failed to show Windows toast: {e}");
        }
    }
}
