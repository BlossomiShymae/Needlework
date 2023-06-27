use futures_util::StreamExt;
use irelia::ws::{EventType, LCUWebSocket};
use serde::Serialize;
use std::error::Error;
use tauri::App;

#[derive(Clone, Serialize)]
struct Payload {
    message: String,
}

pub fn inject_events(app: &mut App) -> Result<(), Box<dyn Error>> {
    let handle = app.handle();
    tauri::async_runtime::spawn(async move {
        let mut ws = LCUWebSocket::new().await.unwrap();
        ws.subscribe(EventType::OnJsonApiEvent);
        loop {
            if let Some(event) = ws.next().await {
                event.map(|v| {
                    let message = v.as_array().unwrap();
                    let payload = message.get(2);
                    payload.map(|v| {
                        let obj = v.as_object().unwrap();
                        let uri = obj.get("uri").unwrap();
                        let event_type = obj.get("eventType").unwrap();
                        println!("{:?}", (event_type, uri));
                    });
                });
            };
        }
    });

    Ok(())
}
