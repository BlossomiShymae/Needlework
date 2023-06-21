pub mod lcu_schema {
    use hyper::{body, Client};
    use hyper_tls::HttpsConnector;

    use crate::data::{metadata::LCUSchema, types::StandardError};

    pub async fn fetch(
    ) -> Result<Result<LCUSchema, StandardError>, Box<dyn std::error::Error + Send + Sync>> {
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let uri = "https://www.mingweisamuel.com/lcu-schema/lcu/openapi.json".parse()?;
        let res = client.get(uri).await?;

        if !res.status().is_success() {
            return Ok(Err(StandardError));
        }

        let bytes = body::to_bytes(res.into_body()).await?;
        let data: LCUSchema = serde_json::from_slice(&bytes.to_vec()).unwrap();
        Ok(Ok(data))
    }
}
