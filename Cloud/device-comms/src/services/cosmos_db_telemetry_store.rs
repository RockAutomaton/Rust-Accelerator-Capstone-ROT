use super::AzureAuth;
use azure_data_cosmos::CosmosClient;
use futures::StreamExt;
use crate::domain::Telemetry::Telemetry;

pub struct CosmosDbTelemetryStore {
    database_name: String,
    container_name: String,
    azure_credential: std::sync::Arc<azure_identity::ClientSecretCredential>,
}

impl CosmosDbTelemetryStore {
    pub fn new(database_name: String, container_name: String) -> Self {
        CosmosDbTelemetryStore {
            database_name,
            container_name,
            azure_credential: AzureAuth::get_credential_from_env(),
        }
    }

    pub async fn insert_telemetry(
        &self,
        document: &serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create Cosmos client
        let cosmos_endpoint = std::env::var("COSMOS_ENDPOINT")
            .expect("COSMOS_ENDPOINT environment variable not set");
        let client: CosmosClient = CosmosClient::new(&cosmos_endpoint, self.azure_credential.clone(), None)
            .expect("Failed to create CosmosClient");

    let container = client
        .database_client(&self.database_name)
        .container_client(&self.container_name);

        // Create document with id field
        let mut document_with_id = document.clone();
        let id = format!(
            "{}-{}",
            document["device_id"],
            chrono::Utc::now().to_rfc3339()
        );
        document_with_id["id"] = serde_json::Value::String(id.clone());

        // Create an item
        let device_id = document["device_id"].as_str().unwrap().to_string();
        container
            .create_item(&device_id, &document_with_id, None)
            .await?;

        Ok(())
    }

    pub async fn read_telemetry(
        &self,
        device_id: &str,
    ) -> Result<Vec<Telemetry>, Box<dyn std::error::Error>> {
        // Create Cosmos client
        let cosmos_endpoint = std::env::var("COSMOS_ENDPOINT")
            .expect("COSMOS_ENDPOINT environment variable not set");
        let client: CosmosClient = CosmosClient::new(&cosmos_endpoint, self.azure_credential.clone(), None)
            .expect("Failed to create CosmosClient");

        let container = client
            .database_client(&self.database_name)
            .container_client(&self.container_name);

        // Query items
        let query = format!("SELECT * FROM c WHERE c.device_id = '{}'", device_id);
        let partition_key = device_id.to_string();
        let mut pager = container.query_items::<Telemetry>(query, partition_key, None)?;

        let mut items = Vec::new();
        while let Some(page_response) = pager.next().await {
            let page = page_response?;
            items.extend(page.items().into_iter().cloned());
        }

        Ok(items)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_insert_telemetry() {
        let store = CosmosDbTelemetryStore::new("device-data".to_string(), "telemetry".to_string());
        let document = json!({
            "device_id": "test_device",
            "telemetry_data": {
                "temperature": 22.5,
                "humidity": 60
            }
        });

        let result = store.insert_telemetry(&document).await;
        assert!(result.is_ok(), "Failed to insert telemetry: {:?}", result);
    }
    #[tokio::test]
    async fn test_insert_telemetry_with_missing_id() {
        let store = CosmosDbTelemetryStore::new("device-data".to_string(), "telemetry".to_string());
        let document = json!({
            "telemetry_data": {
                "temperature": 22.5,
                "humidity": 60
            }
        });

        let result = store.insert_telemetry(&document).await;
        assert!(result.is_err(), "Expected error when device_id is missing");
    }
}