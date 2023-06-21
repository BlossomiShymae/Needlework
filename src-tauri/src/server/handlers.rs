use crate::application::services::lcu_schema_service;
use crate::data::metadata::Info;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
pub async fn get_info() -> Result<Info, String> {
    let data = lcu_schema_service::get_info().await;
    data.map_err(|_err| "Failed to get schema information!".to_string())
}
