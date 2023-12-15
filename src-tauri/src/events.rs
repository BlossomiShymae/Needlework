use std::{
    error::Error,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use futures_util::StreamExt;
use irelia::{
    ws::{EventType, LCUWebSocket},
    LCUError,
};
use tauri::{App, Manager};
use tokio::time::{sleep_until, Instant};

use crate::data::{WebSocketMessage, WebSocketPayload};

pub fn inject_events(app: &mut App) -> Result<(), Box<dyn Error>> {
    let handle = app.handle();
    tauri::async_runtime::spawn(async move {
        // Loop that handles the lifetime of the LCU
        loop {
            match LCUWebSocket::new().await {
                Ok(mut ws) => {
                    ws.subscribe(EventType::OnJsonApiEvent);
                    // Loop that handles incoming WebSocket events
                    let _handle = handle.clone();
                    while let Some(event) = ws.next().await {
                        match event {
                            Ok(v) => {
                                let message = v;
                                let opcode = message.get(0).unwrap().as_i64().unwrap();
                                let event = message.get(1).unwrap().as_str().unwrap();
                                let payload: WebSocketPayload =
                                    serde_json::from_value(message.get(2).unwrap().clone())
                                        .unwrap();
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
                            }
                            Err(e) => {
                                match e {
                                    LCUError::LCUProcessNotRunning => {
                                        // Break out of incoming event loop
                                        ws.terminate();
                                        break;
                                    }
                                    _ => println!("{:?}", e),
                                }
                            }
                        };
                    }
                }
                // The League client is probs not running
                Err(_) => {
                    // Wait a second before trying again
                    sleep_until(Instant::now() + Duration::from_secs(1)).await;
                }
            }
        }
    });

    Ok(())
}
