#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod card_reader;
mod commands;
mod server;

use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

use card_reader::{CardReader, set_debug_mode};
use commands::AppState;
use server::start_http_server;
use tauri::{
    tray::TrayIconBuilder,
    image::Image,
    Manager,
};

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

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            card_lock: card_lock.clone(),
        })
        .setup(move |app| {
            // Create system tray icon
            let icon = Image::from_path("icons/32x32.png").unwrap_or_else(|_| {
                Image::from_bytes(include_bytes!("../icons/32x32.png")).expect("Failed to load tray icon")
            });

            let _tray = TrayIconBuilder::new()
                .icon(icon)
                .tooltip("Thai ID Middleware")
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
            tauri::async_runtime::spawn(async move {
                if let Err(e) = start_http_server(reader_clone).await {
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
