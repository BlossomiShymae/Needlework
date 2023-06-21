// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod application;
pub mod data;
pub mod server;

use server::handlers;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handlers::get_info,
            handlers::get_endpoints,
            handlers::get_endpoint
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
