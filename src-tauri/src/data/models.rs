use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
    pub name: String,
    pub plugins: Vec<Plugin>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    pub method: String,
    pub path: String,
    pub description: Option<String>,
    pub operation_id: String,
    pub parameters: Vec<Value>,
    pub response: Option<Map<String, Value>>,
    pub summary: Option<String>,
}
