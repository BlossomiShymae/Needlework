use std::collections::HashMap;

use irelia::rest::types::Info;
use irelia::rest::types::Schema;
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
    let data = get_endpoints(state).await?;
    match data.get(name) {
        Some(value) => Ok(value.clone()),
        None => Err(StandardError::new(format!(
            "Invalid endpoint key: {}",
            name
        ))),
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
                            // Replace with space char, as per request of clippy
                            key_name = x.split(' ').last().unwrap().into()
                        }
                        _ => key_name = lowercase_tag,
                    }
                    break;
                }
            }
            // Because the schema isn't pulling from a cache, we don't actually need to clone these values
            plugins.push(Plugin {
                method,
                path: path.clone(),
                description: operation.description,
                operation_id: operation.operation_id,
                parameters: operation.parameters,
                responses: operation.responses,
                summary: operation.summary,
                request_body: operation.request_body,
            });

            // map or else performs slightly better
            if let Some(endpoint) = endpoints.get_mut(&key_name) {
                // Extend the plugins list directly
                endpoint.plugins.extend(plugins);
                // Sort endpoint paths alphabetically
                endpoint.plugins.sort_by_key(|k| k.path.to_lowercase());
            } else {
                // Insert into the list if it does not exist already
                endpoints.insert(key_name.clone(), Endpoint { plugins });
            };
        }
    }

    {
        let mut _endpoints = state.endpoints.lock().await;
        *_endpoints = endpoints.clone();
    }
    Ok(endpoints)
}

pub async fn get_schema(name: &str, state: State<'_, Data>) -> Result<PluginSchema, StandardError> {
    let data = get_schemas(state).await?;
    match data.get(name) {
        Some(value) => Ok(value.clone()),
        None => Err(StandardError::new(format!("Invalid schema key: {}", name))),
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
    // Build this directly from an iterator, rather than making a new one with a for loop
    let schemas = HashMap::from_iter(data.components.schemas.into_iter().map(
        |(k, mut schema_value)| {
            // Scan for all descendent schemas for object-type schemas
            let mut schema_ids: Vec<String> = Vec::new();
            if schema_value.schema_type == Some(Type::Object) {
                // Scan properties for possible schemas
                if let Some(properties) = &mut schema_value.properties {
                    // The first field was ignored, so just iter values instead
                    for property in properties.values_mut() {
                        // Change .as_ref().map() to if let Some() as per clippy lints
                        if let Some(schema_id) = &mut property.property_ref {
                            schema_ids.push(schema_id.clone());
                            *schema_id = schema_id.replace("#/components/schemas/", "");
                        };
                        let mut unset_prop_type = false;
                        if let Some(_type) = &property.property_type {
                            // This used to alter the `property_type` field every time, but because the structs in
                            // irelia don't use value, property_ref is altered instead
                            if *_type == Type::Array {
                                if let Some(items) = &property.items {
                                    if let Some(v) = &items.property_ref {
                                        let parameter_type = v.replace("#/components/schemas/", "");
                                        property.property_ref = Some(format!("{parameter_type}[]"));
                                        unset_prop_type = true;
                                    };
                                    if let Some(v) = &items.property_type {
                                        property.property_ref = Some(format!("{:?}[]", v));
                                    };
                                };
                            }
                        };

                        // Unset the type here, this is needed for the front end
                        if unset_prop_type {
                            property.property_type = None;
                        }
                    }
                }
            }

            // Reduce clones of data
            (
                ["#/components/schemas/", k.as_str()].concat(),
                PluginSchema {
                    name: k,
                    description: schema_value.description,
                    properties: schema_value.properties,
                    _enum: schema_value.schema_enum,
                    // This is safe, I checked, will fix in next Irelia release
                    _type: schema_value.schema_type.unwrap(),
                    schema_ids,
                },
            )
        },
    ));

    {
        let mut _schemas = state.schemas.lock().await;
        *_schemas = schemas.clone();
    }

    Ok(schemas)
}

// This can be changed to Result<Schema, Error> now, as that's all it does
pub async fn fetch() -> Result<Schema, StandardError> {
    const REMOTE: &str = "https://raw.githubusercontent.com/dysolix/hasagi-types/main/swagger.json";
    let schema = irelia::rest::LCUClient::schema(REMOTE).await?;
    Ok(schema)
}
