// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod commands;
use common::{database::Database, fs::AppDirStruct};
use tauri::Manager;
use window_vibrancy::*;

fn main() {
    if cfg!(linux) {
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    }

    AppDirStruct::create();
    Database::init();

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "windows")]
            apply_acrylic(&window, Some((0, 255, 0, 0)))
            .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::payment_rest::add_payment,
            commands::payment_rest::find_payment_method,
            commands::payment_rest::load_payments,
            commands::clients_rest::add_client,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
