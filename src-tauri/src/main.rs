// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
pub mod commands;
use common::{database::Database, fs::AppDirStruct};

fn main() {
    if cfg!(linux) {
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    }

    AppDirStruct::create();
    Database::init();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::payment_rest::add_payment,
            commands::payment_rest::find_payment_method,
            commands::payment_rest::load_payments
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
