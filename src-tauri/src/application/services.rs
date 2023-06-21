pub mod lcu_schema_service {
    use std::collections::HashMap;

    use crate::data::apis::lcu_schema;
    use crate::data::metadata::Info;
    use crate::data::models::{Endpoint, Plugin};
    use crate::data::types::StandardError;

    pub async fn get_info() -> Result<Info, StandardError> {
        let data = lcu_schema::fetch().await.unwrap();
        data.map(|data| data.info)
    }

    pub async fn get_endpoints() -> Result<HashMap<String, Endpoint>, StandardError> {
        let data = lcu_schema::fetch().await.unwrap();
        if data.is_err() {
            return Err(data.unwrap_err());
        }
        let mut endpoints: HashMap<String, Endpoint> = HashMap::new();
        for (path, operations) in data.unwrap().paths {
            for (method, operation) in operations {
                let mut plugins: Vec<Plugin> = Vec::new();
                let mut path_name: String = "".to_string();
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
                            response: operation.response.clone(),
                            summary: operation.summary.clone(),
                        });
                        path_name = name.clone();
                        break;
                    }
                }
                let value = endpoints
                    .get_mut(&path)
                    .map(|endpoint| Endpoint {
                        name: endpoint.name.clone(),
                        plugins: endpoint
                            .plugins
                            .clone()
                            .into_iter()
                            .chain(plugins.clone().into_iter())
                            .collect(),
                    })
                    .unwrap_or(Endpoint {
                        name: path_name.clone(),
                        plugins: plugins.clone(),
                    });
                endpoints.insert(path.clone(), value);
            }
        }

        Ok(endpoints)
    }
}
