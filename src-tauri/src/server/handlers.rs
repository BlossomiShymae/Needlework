use serde_json::Value;
use std::collections::HashMap;
use tauri::State;

use crate::application::services::{lcu_schema_service, lcu_service};
use crate::data::metadata::Info;
use crate::data::models::Endpoint;
use crate::Data;

#[tauri::command]
pub async fn get_info() -> Result<Info, String> {
    let data = lcu_schema_service::get_info().await;
    data.map_err(|_err| "Failed to get schema information!".to_string())
}

#[tauri::command]
pub async fn get_endpoints(data_bus: State<'_, Data>) -> Result<HashMap<String, Endpoint>, String> {
    let data = lcu_schema_service::get_endpoints(data_bus).await;
    data.map_err(|_err| "Failed to get endpoints!".to_string())
}

#[tauri::command]
pub async fn get_endpoint(name: &str, data_bus: State<'_, Data>) -> Result<Endpoint, String> {
    let data = lcu_schema_service::get_endpoint(name, data_bus).await;
    data.map_err(|_err| "Failed to get endpoint!".to_string())
}

#[tauri::command]
pub async fn send_request(
    method: &str,
    path: &str,
    body: Option<&str>,
) -> Result<Option<Value>, String> {
    let data = lcu_service::send_request(method, path, body).await;
    data.map_err(|_err| "Failed to send request!".to_string())
}
