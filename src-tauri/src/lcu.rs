use irelia::{rest::LCUClient, RequestClient};
use serde_json::Value;
use tap::prelude::*;
use tauri::State;

use crate::{
    data::{ClientInfo, StandardError},
    Data,
};

pub async fn send_request(
    method: &str,
    path: &str,
    body: Option<&str>,
) -> Result<Option<Value>, StandardError> {
    let client = RequestClient::new();
    match LCUClient::new() {
        Ok(lcu_client) => match method {
            "get" => lcu_client
                .get::<Value, &str>(path, &client)
                .await
                .map_err(StandardError::from_lcu_error),
            "post" => lcu_client
                .post::<Value, Value, &str>(path, body.map(deserialize), &client)
                .await
                .map_err(StandardError::from_lcu_error),
            "put" => lcu_client
                .put::<Value, Value, &str>(path, body.map(deserialize), &client)
                .await
                .map_err(StandardError::from_lcu_error),
            "delete" => lcu_client
                .delete::<Value, &str>(path, &client)
                .await
                .map_err(StandardError::from_lcu_error),
            "patch" => lcu_client
                .patch::<Value, Value, &str>(path, body.map(deserialize), &client)
                .await
                .map_err(StandardError::from_lcu_error),
            "head" => lcu_client
                .head::<Value, &str>(path, &client)
                .await
                .map_err(StandardError::from_lcu_error),
            _ => Err(StandardError::new("Invalid method operation")),
        },
        Err(e) => Err(StandardError::from_lcu_error(e)),
    }
}

pub async fn get_client_info() -> Result<ClientInfo, StandardError> {
    match LCUClient::new() {
        Ok(lcu_client) => Ok(ClientInfo {
            url: lcu_client.url().to_string(),
            auth_header: lcu_client.auth_header().to_string(),
        }),
        Err(e) => Err(StandardError::from_lcu_error(e)),
    }
}

pub async fn produce_payload(
    key: &str,
    payload: &str,
    state: State<'_, Data>,
) -> Result<(), StandardError> {
    let mut payloads = state.payloads.lock().await;
    payloads.insert(key.into(), payload.into());

    Ok(())
}

pub async fn consume_payload(key: &str, state: State<'_, Data>) -> Result<String, StandardError> {
    let mut payloads = state.payloads.lock().await;
    let _key = String::from(key);
    let payload = payloads
        .get(&_key)
        .map_or(Err(StandardError::new("Payload not found")), |v| {
            Ok(v.to_string())
        });
    payloads.remove(&_key);
    payload
}

fn deserialize(stream: &str) -> Value {
    let data: Value = serde_json::from_str(stream).unwrap();
    data.tap(|v| println!("Data: {:?}", v))
}
