pub mod lcu_schema_service {
    use std::collections::HashMap;

    use serde_json::{json, Map, Value};
    use tauri::State;

    use crate::data::apis::lcu_schema;
    use crate::data::metadata::Info;
    use crate::data::models::{Endpoint, Plugin, Schema};
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
                None => Err(StandardError::new(
                    format!("Invalid endpoint key: {}", name).as_str(),
                )),
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
                return Ok(_endpoints.clone());
            }
        }

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

    pub async fn get_schema(name: &str, state: State<'_, Data>) -> Result<Schema, StandardError> {
        let data = get_schemas(state).await;
        match data {
            Ok(v) => match v.get(name) {
                Some(value) => Ok(value.clone()),
                None => Err(StandardError::new(
                    format!("Invalid schema key: {}", name).as_str(),
                )),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn get_schemas(
        state: State<'_, Data>,
    ) -> Result<HashMap<String, Schema>, StandardError> {
        {
            let _schemas = state.schemas.lock().await;
            if _schemas.len() > 0 {
                return Ok(_schemas.clone());
            }
        }

        let data = lcu_schema::fetch().await.unwrap();
        if data.is_err() {
            return Err(data.unwrap_err());
        }
        let mut schemas: HashMap<String, Schema> = HashMap::new();
        for (k, schema) in data.unwrap().components.schemas {
            let mut schema_clone = schema.clone();
            // Scan for all descendent schemas for object-type schemas
            let mut schema_ids: Vec<String> = Vec::new();
            if schema_clone._type.eq("object") {
                // Scan properties for possible schemas
                let properties = schema_clone.properties.as_mut().unwrap();
                for (_property_name, property) in properties {
                    let property_ref: &mut Map<String, Value> = property.as_object_mut().unwrap();
                    let mut is_schema_ref = false;
                    let mut is_array = false;
                    let mut type_value: Value = json!("");
                    property_ref.get_mut("$ref").map(|v| {
                        let schema_id = v.as_str().unwrap().to_string();
                        schema_ids.push(schema_id.clone());
                        type_value = json!(schema_id.replace("#/components/schemas/", ""));
                        is_schema_ref = true;
                    });
                    property_ref.get("type").map(|v| {
                        let _type = v.as_str().unwrap();
                        if _type.eq("array") {
                            let mut parameter_type = "";
                            property_ref.get("items").map(|v| {
                                let items: &Map<String, Value> = v.as_object().unwrap();
                                items.get("$ref").map(|v| {
                                    parameter_type = v.as_str().clone().unwrap();
                                });
                                items.get("type").map(|v| {
                                    parameter_type = v.as_str().clone().unwrap();
                                });
                            });
                            type_value = json!("x[]".replace(
                                "x",
                                parameter_type.replace("#/components/schemas/", "").as_str()
                            ));
                            is_array = true;
                        }
                    });

                    if is_schema_ref || is_array {
                        property_ref.insert("type".into(), type_value);
                    }
                }
            }
            schemas.insert(
                ["#/components/schemas/", k.clone().as_str()].concat(),
                Schema {
                    name: k,
                    description: schema_clone.description,
                    properties: schema_clone.properties,
                    _enum: schema_clone._enum,
                    _type: schema_clone._type,
                    schema_ids: schema_ids.clone(),
                },
            );
        }

        {
            let mut _schemas = state.schemas.lock().await;
            *_schemas = schemas.clone();
        }

        Ok(schemas)
    }
}

pub mod lcu_service {
    use irelia::{rest::LCUClient, RequestClient};
    use serde_json::Value;
    use std::str;
    use tap::prelude::*;

    use crate::data::{models::ClientInfo, types::StandardError};

    pub async fn send_request(
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<Option<Value>, StandardError> {
        let client = RequestClient::new();
        match LCUClient::new(&client) {
            Ok(lcu_client) => match method {
                "get" => lcu_client
                    .get::<Value>(path)
                    .await
                    .map_err(StandardError::from_lcu_error),
                "post" => lcu_client
                    .post::<Value, Value>(path, body.map(deserialize))
                    .await
                    .map_err(StandardError::from_lcu_error),
                "put" => lcu_client
                    .put::<Value, Value>(path, body.map(deserialize))
                    .await
                    .map_err(StandardError::from_lcu_error),
                "delete" => lcu_client
                    .delete::<Value>(path)
                    .await
                    .map_err(StandardError::from_lcu_error),
                "patch" => lcu_client
                    .patch::<Value, Value>(path, body.map(deserialize))
                    .await
                    .map_err(StandardError::from_lcu_error),
                "head" => lcu_client
                    .head::<Value>(path)
                    .await
                    .map_err(StandardError::from_lcu_error),
                _ => Err(StandardError::new("Invalid method operation")),
            },
            Err(e) => Err(StandardError::from_lcu_error(e)),
        }
    }

    pub async fn get_client_info() -> Result<ClientInfo, StandardError> {
        let client = RequestClient::new();
        match LCUClient::new(&client) {
            Ok(lcu_client) => Ok(ClientInfo {
                url: lcu_client.url().to_string(),
                auth_header: lcu_client.auth_header().to_string(),
            }),
            Err(e) => Err(StandardError::from_lcu_error(e)),
        }
    }

    fn deserialize(stream: &str) -> Value {
        let data: Value = serde_json::from_str(stream).unwrap();
        data.tap(|v| println!("Data: {:?}", v))
    }
}
