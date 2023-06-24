use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::Map;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LCUSchema {
    pub components: Components,
    pub info: Info,
    pub openapi: String,
    pub paths: HashMap<String, HashMap<String, Operation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub title: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    pub schemas: HashMap<String, Schema>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub description: Option<String>,
    pub properties: Option<Map<String, Value>>,
    #[serde(rename = "enum")]
    pub _enum: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub description: Option<String>,
    pub operation_id: String,
    pub parameters: Vec<Value>,
    pub responses: Option<Map<String, Value>>,
    pub summary: Option<String>,
    pub tags: Vec<String>,
    pub request_body: Option<Map<String, Value>>,
}