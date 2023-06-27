use std::{
    error::Error,
    time::{SystemTime, UNIX_EPOCH},
};

use futures_util::StreamExt;
use irelia::ws::{EventType, LCUWebSocket};
use tauri::{App, Manager};

use crate::data::models::{WebSocketMessage, WebSocketPayload};

pub fn inject_events(app: &mut App) -> Result<(), Box<dyn Error>> {
    let handle = app.handle();
    tauri::async_runtime::spawn(async move {
        let mut ws = LCUWebSocket::new().await.unwrap();
        ws.subscribe(EventType::OnJsonApiEvent);
        loop {
            let _handle = handle.clone();
            if let Some(event) = ws.next().await {
                let _ = event.map(|v| {
                    let message = v.as_array().unwrap();
                    let opcode = message.get(0).unwrap().as_i64().unwrap();
                    let event = message.get(1).unwrap().as_str().unwrap();
                    let payload: WebSocketPayload =
                        serde_json::from_value(message.get(2).unwrap().clone()).unwrap();
                    let wsm = WebSocketMessage {
                        opcode,
                        event: event.into(),
                        data: payload.data,
                        event_type: payload.event_type,
                        uri: payload.uri,
                        timestamp: SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_millis(),
                    };
                    let _ = _handle.emit_all("lcu-ws-event", wsm);
                });
            };
        }
    });

    Ok(())
}
