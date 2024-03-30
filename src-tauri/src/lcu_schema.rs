use std::collections::HashMap;

use irelia::rest::types::Info;
use irelia::rest::types::Type;
use tauri::State;

use crate::data::Endpoint;
use crate::data::Plugin;
use crate::data::PluginSchema;
use crate::data::StandardError;
use crate::lcu_schema;
use crate::Data;

pub async fn get_info() -> Result<Info, StandardError> {
    let data = lcu_schema::fetch().await?;
    Ok(data.info)
}

pub async fn get_endpoint(name: &str, state: State<'_, Data>) -> Result<Endpoint, StandardError> {
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

    let data = lcu_schema::fetch().await?;

    let mut endpoints: HashMap<String, Endpoint> = HashMap::new();
    for (path, operations) in data.paths {
        for (method, operation) in operations {
            let mut plugins: Vec<Plugin> = Vec::new();
            let mut key_name: String = "_unknown".to_string();

            // Process and group endpoints into the following formats:
            // "_unknown" - group that should not be possible
            // "default" - no tags
            // "builtin" - 'builtin' not associated with an endpoint
            // "lol-summoner" etc. - 'plugin' associated with an endpoint
            // "performance", "tracing", etc.
            if operation.tags.is_empty() {
                key_name = "default".into();
            } else {
                for tag in operation.tags {
                    let lowercase_tag = tag.to_lowercase();
                    match lowercase_tag.as_str() {
                        "plugins" => {
                            continue;
                        }
                        x if lowercase_tag.contains("plugin ") => {
                            key_name = x.split(" ").last().unwrap().into()
                        }
                        _ => key_name = lowercase_tag.clone().into(),
                    }
                    break;
                }
            }
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

            let mut value = endpoints
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

            // Sort endpoint paths alphabetically
            value.plugins.sort_by_key(|k| k.path.to_lowercase());

            endpoints.insert(key_name.clone(), value);
        }
    }

    {
        let mut _endpoints = state.endpoints.lock().await;
        *_endpoints = endpoints.clone();
    }
    Ok(endpoints)
}

pub async fn get_schema(name: &str, state: State<'_, Data>) -> Result<PluginSchema, StandardError> {
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
) -> Result<HashMap<String, PluginSchema>, StandardError> {
    {
        let _schemas = state.schemas.lock().await;
        if _schemas.len() > 0 {
            return Ok(_schemas.clone());
        }
    }

    let data = lcu_schema::fetch().await?;
    let mut schemas: HashMap<String, PluginSchema> = HashMap::new();
    for (k, schema) in data.components.schemas {
        let mut schema_clone = schema.clone();
        // Scan for all descendent schemas for object-type schemas
        let mut schema_ids: Vec<String> = Vec::new();
        if schema_clone.schema_type == Some(Type::Object) {
            // Scan properties for possible schemas
            let default = &mut HashMap::new();
            let properties = schema_clone.properties.as_mut().unwrap_or(default);
            for (_property_name, property) in properties {
                let mut reformat_type = false;
                let mut type_value: String = Default::default();
                property.property_ref.as_ref().map(|schema_id| {
                    schema_ids.push(schema_id.clone());
                    type_value = schema_id.replace("#/components/schemas/", "");
                    reformat_type = true;
                });
                property.property_type.as_ref().map(|_type| {
                    if *_type == Type::Array {
                        property.items.as_ref().map(|items| {
                            items.property_ref.as_ref().map(|v| {
                                let parameter_type = v.replace("#/components/schemas/", "");
                                type_value = format!("{parameter_type}[]");
                            });
                            items.property_type.as_ref().map(|v| {
                                type_value = format!("{:?}[]", v);
                            });
                        });
                        reformat_type = true;
                    }
                });

                if reformat_type {
                    property.property_ref = Some(type_value.clone());
                    property.items.as_mut().map(|f| f.property_ref = Some(type_value));
                }
            }
        }
        schemas.insert(
            ["#/components/schemas/", k.clone().as_str()].concat(),
            PluginSchema {
                name: k,
                description: schema_clone.description,
                properties: schema_clone.properties,
                _enum: schema_clone.schema_enum,
                // This is safe, I checked
                _type: schema_clone.schema_type.unwrap(),
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

pub async fn fetch() -> Result<irelia::rest::types::Schema, StandardError> {
    const REMOTE: &'static str =
        "https://raw.githubusercontent.com/dysolix/hasagi-types/main/swagger.json";
    let schema = irelia::rest::LCUClient::schema(REMOTE).await?;
    Ok(schema)
}
