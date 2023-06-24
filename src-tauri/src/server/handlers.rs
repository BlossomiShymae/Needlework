use serde_json::Value;
use std::collections::HashMap;
use tauri::State;

use crate::application::services::{lcu_schema_service, lcu_service};
use crate::data::metadata::Info;
use crate::data::models::{ClientInfo, Endpoint, Schema};
use crate::Data;

#[tauri::command]
pub async fn get_info() -> Result<Info, String> {
    let data = lcu_schema_service::get_info().await;
    data.map_err(|e| e.message)
}

#[tauri::command]
pub async fn get_client_info() -> Result<ClientInfo, String> {
    let data = lcu_service::get_client_info().await;
    data.map_err(|e| e.message)
}

#[tauri::command]
pub async fn get_endpoints(state: State<'_, Data>) -> Result<HashMap<String, Endpoint>, String> {
    let data = lcu_schema_service::get_endpoints(state).await;
    data.map_err(|e| e.message)
}

#[tauri::command]
pub async fn get_endpoint(name: &str, state: State<'_, Data>) -> Result<Endpoint, String> {
    let data = lcu_schema_service::get_endpoint(name, state).await;
    data.map_err(|e| e.message)
}

#[tauri::command]
pub async fn get_schema(name: &str, state: State<'_, Data>) -> Result<Schema, String> {
    let data = lcu_schema_service::get_schema(name, state).await;
    data.map_err(|e| e.message)
}

#[tauri::command]
pub async fn get_schemas(state: State<'_, Data>) -> Result<HashMap<String, Schema>, String> {
    let data = lcu_schema_service::get_schemas(state).await;
    data.map_err(|e| e.message)
}

#[tauri::command]
pub async fn send_request(
    method: &str,
    path: &str,
    body: Option<&str>,
) -> Result<Option<Value>, String> {
    let data = lcu_service::send_request(method, path, body).await;
    data.map_err(|e| e.message)
}
