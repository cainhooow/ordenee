// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod commands;
use common::{database::Database, fs::AppDirStruct};
use tauri::Manager;
use window_vibrancy::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if cfg!(linux) {
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    }

    AppDirStruct::create();
    Database::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let splashscreen = app.get_webview_window("splashscreen").unwrap();
            let mainwindow = app.get_webview_window("main").unwrap();

            let window = mainwindow.clone();

            tauri::async_runtime::spawn(async move {
                std::thread::sleep(std::time::Duration::from_secs(2));

                splashscreen.close().unwrap();
                window.show().unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::payment_rest::add_payment,
            commands::payment_rest::find_payment_method,
            commands::payment_rest::load_payments,
            commands::clients_rest::add_client,
            commands::clients_rest::load_clients,
            commands::address_rest::create_address
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
