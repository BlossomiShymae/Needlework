// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod application;
pub mod data;
pub mod server;

use std::{collections::HashMap, sync::Arc};

use data::models::{Endpoint, Schema};
use server::handlers;
use tokio::sync::Mutex;

pub struct Data {
    pub endpoints: Arc<Mutex<HashMap<String, Endpoint>>>,
    pub schemas: Arc<Mutex<HashMap<String, Schema>>>,
}

impl Data {
    pub fn new() -> Data {
        Data {
            endpoints: Arc::new(Mutex::new(HashMap::new())),
            schemas: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Data::new())
        .invoke_handler(tauri::generate_handler![
            handlers::get_info,
            handlers::get_endpoints,
            handlers::get_endpoint,
            handlers::send_request,
            handlers::get_schema,
            handlers::get_client_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
