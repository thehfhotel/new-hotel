#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod card_reader;
mod commands;
mod ihotel;
mod mrz;
mod server;
mod stale;
mod toast;

use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

use card_reader::{set_debug_mode, CardReader};
use commands::AppState;
use server::start_http_server;
use stale::StaleLatch;
use tauri::menu::{Menu, MenuItem};
use tauri::{image::Image, tray::TrayIconBuilder, Manager};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};

/// Tray menu item ids — shared between construction and the
/// `on_menu_event` match so a typo can't silently desync the two.
const MENU_ID_IHOTEL_REFRESH: &str = "ihotel_refresh";
const MENU_ID_IHOTEL_STATUS: &str = "ihotel_status";

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let debug_mode = args.iter().any(|arg| arg == "--debug" || arg == "-d");

    if debug_mode {
        set_debug_mode(true);
        eprintln!("[DEBUG] Debug mode enabled via command-line flag");
    }

    // Initialize card lock for serializing card operations (used by IPC commands)
    let card_lock = Arc::new(Mutex::new(()));

    // Initialize card reader wrapper for HTTP server
    let card_reader = Arc::new(Mutex::new(CardReader::new()));

    // Grid-staleness latch (docs/adr/0006-legacy-stale-notification.md).
    // Shared between the HTTP server (`POST /notify`, `GET /ihotel/status`,
    // `POST /ihotel/refresh`) and the tray menu's "รีเฟรช iHOTEL" /
    // "สถานะ iHOTEL" items below, so both surfaces observe/clear the same
    // state.
    let stale_latch = Arc::new(Mutex::new(StaleLatch::new()));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // Reception PCs need this middleware running before anyone opens a
        // browser tab (ADR 0006 §3 depends on both being up) — nothing
        // else starts it today, so autostart is the fix. `None` args: no
        // special CLI flags are needed on an autostart launch yet.
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        .manage(AppState {
            card_lock: card_lock.clone(),
        })
        .setup(move |app| {
            // Enable autostart by default. Idempotent — `is_enabled` is
            // checked first so a normal (non-first) launch doesn't rewrite
            // the OS autostart entry every time.
            match app.autolaunch().is_enabled() {
                Ok(true) => {}
                Ok(false) => match app.autolaunch().enable() {
                    Ok(()) => println!("Autostart enabled"),
                    Err(e) => eprintln!("Failed to enable autostart: {}", e),
                },
                Err(e) => eprintln!("Failed to check autostart status: {}", e),
            }

            // Create system tray icon
            let icon = Image::from_path("icons/32x32.png").unwrap_or_else(|_| {
                Image::from_bytes(include_bytes!("../icons/32x32.png"))
                    .expect("Failed to load tray icon")
            });

            let refresh_item = MenuItem::with_id(
                app,
                MENU_ID_IHOTEL_REFRESH,
                "รีเฟรช iHOTEL",
                true,
                None::<&str>,
            )?;
            let status_item = MenuItem::with_id(
                app,
                MENU_ID_IHOTEL_STATUS,
                "สถานะ iHOTEL",
                true,
                None::<&str>,
            )?;
            let tray_menu = Menu::with_items(app, &[&refresh_item, &status_item])?;

            let menu_latch = stale_latch.clone();
            let _tray = TrayIconBuilder::new()
                .icon(icon)
                .tooltip("Thai ID Middleware")
                .menu(&tray_menu)
                .on_menu_event(move |_app, event| {
                    // Same code path `POST /ihotel/refresh` calls
                    // (`crate::ihotel::trigger_refresh`) — see that
                    // module's docs for why refresh is one function shared
                    // by both surfaces.
                    if event.id().as_ref() == MENU_ID_IHOTEL_REFRESH {
                        let latch = menu_latch.clone();
                        tauri::async_runtime::spawn(async move {
                            match ihotel::trigger_refresh() {
                                Ok(()) => {
                                    latch.lock().await.on_refresh_performed(chrono::Utc::now());
                                    println!("[tray] iHOTEL refresh triggered");
                                }
                                Err(e) => {
                                    println!("[tray] iHOTEL refresh not available: {}", e);
                                }
                            }
                        });
                    } else if event.id().as_ref() == MENU_ID_IHOTEL_STATUS {
                        let latch = menu_latch.clone();
                        tauri::async_runtime::spawn(async move {
                            let probe = ihotel::snapshot();
                            let snap = latch.lock().await.snapshot();
                            let text = format!(
                                "พบโปรเซส iHOTEL: {} | พบหน้าต่างห้องพัก: {} | สถานะ: {} | ค้างแจ้งเตือน: {}",
                                yes_no(probe.process_found),
                                yes_no(probe.grid_window_found),
                                snap.state,
                                snap.stale_count
                            );
                            toast::show("สถานะ iHOTEL", &text);
                        });
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click { .. } = event {
                        if let Some(window) = tray.app_handle().get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // Start HTTP server in background - it will run for the lifetime of the app
            let reader_clone = card_reader.clone();
            let server_latch = stale_latch.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = start_http_server(reader_clone, server_latch).await {
                    eprintln!("HTTP server error: {}", e);
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_status,
            commands::get_version,
            commands::read_card,
            commands::debug_card,
            commands::set_debug,
            commands::get_debug,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Thai yes/no for the tray status toast — "พบ" (found) / "ไม่พบ" (not found).
fn yes_no(found: bool) -> &'static str {
    if found {
        "พบ"
    } else {
        "ไม่พบ"
    }
}
