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

    pub async fn get_endpoint(name: &str) -> Result<Endpoint, StandardError> {
        let data = get_endpoints().await;
        match data {
            Ok(v) => match v.get(name) {
                Some(value) => Ok(value.clone()),
                None => Err(StandardError),
            },
            Err(e) => Err(e),
        }
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

        Ok(endpoints)
    }
}
