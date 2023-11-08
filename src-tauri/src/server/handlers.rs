use serde_json::Value;
use std::collections::HashMap;
use tauri::{AppHandle, State, WindowBuilder, WindowUrl};

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

#[tauri::command]
pub async fn open_data_window(
    key: &str,
    subtitle: &str,
    payload: &str,
    handle: AppHandle,
    state: State<'_, Data>,
) -> Result<(), String> {
    let _data_window = WindowBuilder::new(
        &handle,
        format!("data-{key}-window"),
        WindowUrl::App(format!("/data?key={key}").parse().unwrap()),
    )
    .title(format!("Needlework - {subtitle}"))
    .build()
    .unwrap();

    lcu_service::produce_payload(key, payload, state)
        .await
        .map_err(|e| e.message)
}

#[tauri::command]
pub async fn get_data_payload(key: &str, state: State<'_, Data>) -> Result<String, String> {
    lcu_service::consume_payload(key, state)
        .await
        .map_err(|e| e.message)
}

#[tauri::command]
pub async fn restart_application(handle: AppHandle) -> Result<(), String> {
    handle.restart();
    Ok(())
}
