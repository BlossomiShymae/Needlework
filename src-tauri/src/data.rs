use std::collections::HashMap;

use irelia::rest::types::Parameter;
use irelia::rest::types::Property;
use irelia::rest::types::RequestBody;
use irelia::rest::types::Type;
use irelia::rest::types::Responses;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::{error::Error, fmt};

use irelia::LCUError;

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
    pub parameters: Vec<Parameter>,
    pub responses: Option<HashMap<String, Responses>>,
    pub summary: Option<String>,
    pub request_body: Option<RequestBody>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginSchema {
    pub name: String,
    pub description: Option<String>,
    pub properties: Option<HashMap<String, Property>>,
    #[serde(rename = "enum")]
    pub _enum: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub _type: Type,
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

#[derive(Debug, Clone)]
pub struct StandardError {
    pub message: String,
}

impl StandardError {
    pub fn new(message: &str) -> StandardError {
        StandardError {
            message: message.to_string(),
        }
    }

    pub fn from_error(error: impl Error) -> StandardError {
        StandardError {
            message: error.to_string(),
        }
    }
}

impl From<LCUError> for StandardError {
    fn from(error: LCUError) -> Self {
        StandardError {
            message: error.to_string(),
        }
    }
}

impl Error for StandardError {}

impl fmt::Display for StandardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something has gone wrong")
    }
}
