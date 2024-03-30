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
    let lcu_client = LCUClient::new()?;
    let maybe_value = match method {
        "get" => lcu_client
            .get::<Value, &str>(path, &client)
            .await?,
        "post" => lcu_client
            .post::<Value, Value, &str>(path, body.map(deserialize), &client)
            .await?,
        "put" => lcu_client
            .put::<Value, Value, &str>(path, body.map(deserialize), &client)
            .await?,
        "delete" => lcu_client
            .delete::<Value, &str>(path, &client)
            .await?,
        "patch" => lcu_client
            .patch::<Value, Value, &str>(path, body.map(deserialize), &client)
            .await?,
        "head" => lcu_client
            .head::<Value, &str>(path, &client)
            .await?,
        _ => Err(StandardError::new("Invalid method operation"))?,
    };
    Ok(maybe_value)
}

pub async fn get_client_info() -> Result<ClientInfo, StandardError> {
    let lcu_client = LCUClient::new()?;
    Ok(ClientInfo {
        url: lcu_client.url().to_string(),
        auth_header: lcu_client.auth_header().to_string(),
    })
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
