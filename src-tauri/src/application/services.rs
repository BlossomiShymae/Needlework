pub mod lcu_schema_service {
    use std::collections::HashMap;

    use tauri::State;

    use crate::data::apis::lcu_schema;
    use crate::data::metadata::Info;
    use crate::data::models::{Endpoint, Plugin};
    use crate::data::types::StandardError;
    use crate::Data;

    pub async fn get_info() -> Result<Info, StandardError> {
        let data = lcu_schema::fetch().await.unwrap();
        data.map(|data| data.info)
    }

    pub async fn get_endpoint(
        name: &str,
        state: State<'_, Data>,
    ) -> Result<Endpoint, StandardError> {
        let data = get_endpoints(state).await;
        match data {
            Ok(v) => match v.get(name) {
                Some(value) => Ok(value.clone()),
                None => Err(StandardError),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn get_endpoints(
        state: State<'_, Data>,
    ) -> Result<HashMap<String, Endpoint>, StandardError> {
        {
            let _endpoints = state.endpoints.lock().await;
            if _endpoints.len() > 0 {
                println!("Using cached endpoints...");
                return Ok(_endpoints.clone());
            }
        }
        println!("Fetching new endpoints...");

        let data = lcu_schema::fetch().await.unwrap();
        if data.is_err() {
            return Err(data.unwrap_err());
        }
        let mut endpoints: HashMap<String, Endpoint> = HashMap::new();
        for (path, operations) in data.unwrap().paths {
            for (method, operation) in operations {
                let mut plugins: Vec<Plugin> = Vec::new();
                let mut key_name: String = "".to_string();
                for tag in operation.tags {
                    let name = tag.to_lowercase();
                    if name.contains("plugins") {
                        continue;
                    } else if name.contains("plugin") {
                        plugins.push(Plugin {
                            method: method.clone(),
                            path: path.clone(),
                            description: operation.description.clone(),
                            operation_id: operation.operation_id.clone(),
                            parameters: operation.parameters.clone(),
                            responses: operation.responses.clone(),
                            summary: operation.summary.clone(),
                            request_body: operation.request_body.clone(),
                        });
                        key_name = name.clone().split(" ").last().unwrap().into();
                        break;
                    }
                }
                if !key_name.contains("lol") {
                    continue;
                }
                let value = endpoints
                    .get_mut(&key_name)
                    .map(|endpoint| Endpoint {
                        plugins: endpoint
                            .plugins
                            .clone()
                            .into_iter()
                            .chain(plugins.clone().into_iter())
                            .collect(),
                    })
                    .unwrap_or(Endpoint {
                        plugins: plugins.clone(),
                    });
                endpoints.insert(key_name.clone(), value);
            }
        }

        {
            println!("Accessing state under mutex...");
            let mut _endpoints = state.endpoints.lock().await;
            println!("Holding lock...");
            *_endpoints = endpoints.clone();
            println!("End");
        }
        println!("Returning endpoints...");
        Ok(endpoints)
    }
}

pub mod lcu_service {
    use irelia::{rest::LCUClient, RequestClient};
    use serde_json::Value;
    use tap::prelude::*;

    use crate::data::types::StandardError;

    pub async fn send_request(
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<Option<Value>, StandardError> {
        let client = RequestClient::new();
        let lcu_client = LCUClient::new(&client).unwrap();
        let data = match method {
            "get" => lcu_client
                .get::<Value>(path)
                .await
                .map_err(|_err| StandardError),
            "post" => lcu_client
                .post::<Value, Value>(path, body.map(deserialize))
                .await
                .map_err(|_err| StandardError),
            "put" => lcu_client
                .put::<Value, Value>(path, body.map(deserialize))
                .await
                .map_err(|_err| StandardError),
            "delete" => lcu_client
                .delete::<Value>(path)
                .await
                .map_err(|_err| StandardError),
            "patch" => lcu_client
                .patch::<Value, Value>(path, body.map(deserialize))
                .await
                .map_err(|_err| StandardError),
            "head" => lcu_client
                .head::<Value>(path)
                .await
                .map_err(|_err| StandardError),
            _ => Err(StandardError),
        };
        data
    }

    fn deserialize(stream: &str) -> Value {
        let data: Value = serde_json::from_str(stream).unwrap();
        data.tap(|v| println!("Data: {:?}", v))
    }
}
