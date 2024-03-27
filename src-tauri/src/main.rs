
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod login;
mod app_checks;
mod config;


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app_checks::check_port,
            login::request_device_authorization,
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
  