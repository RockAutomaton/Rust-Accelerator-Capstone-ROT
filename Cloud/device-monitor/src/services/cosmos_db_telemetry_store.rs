use super::AzureAuth;
use azure_data_cosmos::CosmosClient;
use azure_data_cosmos::clients::ContainerClient;
use futures::StreamExt;
use crate::domain::telemetry::Telemetry;
use std::sync::Arc;

#[derive(Clone)]
pub struct CosmosDbTelemetryStore {
    pub container_client: Arc<ContainerClient>,
}

impl CosmosDbTelemetryStore {
    pub async fn new(
        database_name: String, 
        container_name: String
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let cosmos_endpoint = std::env::var("COSMOS_ENDPOINT")
            .expect("COSMOS_ENDPOINT environment variable not set");
        
        let azure_credential = AzureAuth::get_credential_from_env();
        
        // Create the client once during initialization
        let cosmos_client = CosmosClient::new(&cosmos_endpoint, azure_credential, None)?;
        
        let container_client = cosmos_client
            .database_client(&database_name)
            .container_client(&container_name);

        Ok(CosmosDbTelemetryStore {
            container_client: Arc::new(container_client),
        })
    }

    pub async fn insert_telemetry(
        &self,
        document: &serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create document with id field
        let mut document_with_id = document.clone();
        let id = format!(
            "{}-{}",
            document["device_id"],
            chrono::Utc::now().to_rfc3339()
        );
        document_with_id["id"] = serde_json::Value::String(id.clone());

        // Create an item using the stored container client
        let device_id = document["device_id"].as_str().unwrap().to_string();
        self.container_client
            .create_item(&device_id, &document_with_id, None)
            .await?;

        Ok(())
    }

    pub async fn read_telemetry(
        &self,
        device_id: &str,
    ) -> Result<Vec<Telemetry>, Box<dyn std::error::Error>> {
        // Query items using the stored container client
        let query = format!("SELECT * FROM c WHERE c.device_id = '{}'", device_id);
        let partition_key = device_id.to_string();
        let mut pager = self.container_client.query_items::<Telemetry>(query, partition_key, None)?;

        let mut items = Vec::new();
        while let Some(page_response) = pager.next().await {
            let page = page_response?;
            items.extend(page.items().into_iter().cloned());
        }

        Ok(items)
    }
}
