use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Endpoint {
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
    pub responses: Option<Map<String, Value>>,
    pub summary: Option<String>,
    pub request_body: Option<Map<String, Value>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub name: String,
    pub description: Option<String>,
    pub properties: Option<Map<String, Value>>,
    #[serde(rename = "enum")]
    pub _enum: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub _type: String,
    pub schema_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    pub url: String,
    pub auth_header: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketMessage {
    pub opcode: i64,
    pub event: String,
    pub data: Value,
    pub uri: String,
    pub event_type: String,
    pub timestamp: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketPayload {
    pub data: Value,
    pub uri: String,
    pub event_type: String,
}
