pub mod lcu_schema_service {
    use crate::data::apis::lcu_schema;
    use crate::data::metadata::Info;
    use crate::data::types::StandardError;

    pub async fn get_info() -> Result<Info, StandardError> {
        let data = lcu_schema::fetch().await.unwrap();
        data.map(|data| data.info)
    }
}
