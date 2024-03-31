use std::{
    error::Error,
    ops::ControlFlow,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use irelia::{
    ws::{EventType, Flow, LCUWebSocket},
    LCUError,
};
use tauri::{App, Manager};
use tokio::time::{sleep_until, Instant};

use crate::data::{WebSocketMessage, WebSocketPayload};

pub fn inject_events(app: &mut App) -> Result<(), Box<dyn Error>> {
    let handle = app.handle();
    let handle = handle.clone();
    tauri::async_runtime::spawn(async move {
        match LCUWebSocket::new(move |event| match event {
            Ok(v) => {
                let message = v;
                let opcode = message.first().unwrap().as_i64().unwrap();
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
                let _ = handle.emit_all("lcu-ws-event", wsm);
                ControlFlow::Continue(Flow::Continue)
            }
            Err(e) => match e {
                LCUError::LCUProcessNotRunning => ControlFlow::Break(()),
                _ => {
                    println!("{:?}", e);
                    ControlFlow::Continue(Flow::Continue)
                }
            },
        })
        .await
        {
            Ok(mut ws) => {
                ws.subscribe(EventType::OnJsonApiEvent);
            }
            // The League client is probs not running
            Err(_) => {
                // Wait a second before trying again
                sleep_until(Instant::now() + Duration::from_secs(1)).await;
            }
        }
    });

    Ok(())
}
