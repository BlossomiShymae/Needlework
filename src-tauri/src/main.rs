// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod data;
pub mod events;
pub mod handlers;
pub mod lcu;
pub mod lcu_schema;

use std::{collections::HashMap, sync::Arc};

use data::{Endpoint, PluginSchema};
use events::inject_events;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct Data {
    pub endpoints: Arc<Mutex<HashMap<String, Endpoint>>>,
    pub schemas: Arc<Mutex<HashMap<String, PluginSchema>>>,
    pub payloads: Arc<Mutex<HashMap<String, String>>>,
}

fn main() {
    tauri::Builder::default()
        // Use the function pointer directly, clippy lint
        .setup(inject_events)
        .manage(Data::default())
        .invoke_handler(tauri::generate_handler![
            handlers::get_info,
            handlers::get_endpoints,
            handlers::get_endpoint,
            handlers::send_request,
            handlers::get_schema,
            handlers::get_client_info,
            handlers::get_schemas,
            handlers::open_data_window,
            handlers::get_data_payload,
            handlers::restart_application
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
