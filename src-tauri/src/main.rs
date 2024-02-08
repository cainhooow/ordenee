// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use common::fs::AppDirStruct;

fn main() {
  std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

  AppDirStruct::build_app_dir();

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
