// Cosmos DB Telemetry Store Service
// 
// This module provides the interface for storing and retrieving telemetry data
// from Azure Cosmos DB. It handles all database operations for the device
// communications service.

use super::AzureAuth;
use azure_data_cosmos::CosmosClient;
use azure_data_cosmos::clients::ContainerClient;
use futures::StreamExt;
use crate::domain::telemetry::Telemetry;
use std::sync::Arc;

/// Cosmos DB client for telemetry data storage and retrieval
/// 
/// This struct provides a thread-safe interface to Azure Cosmos DB for
/// storing and querying IoT device telemetry data. It uses the device_id
/// as the partition key for efficient querying and storage.
#[derive(Clone)]
pub struct CosmosDbTelemetryStore {
    /// Thread-safe reference to the Cosmos DB container client
    /// 
    /// This client is used for all database operations and is shared
    /// across multiple request handlers.
    pub container_client: Arc<ContainerClient>,
}

impl CosmosDbTelemetryStore {
    /// Creates a new Cosmos DB telemetry store client
    /// 
    /// This method initializes the connection to Azure Cosmos DB using
    /// environment variables for configuration. It creates a container
    /// client that will be used for all subsequent database operations.
    /// 
    /// # Arguments
    /// * `database_name` - The name of the Cosmos DB database
    /// * `container_name` - The name of the container within the database
    /// 
    /// # Returns
    /// * `Result<Self, Box<dyn std::error::Error>>` - The configured client or an error
    /// 
    /// # Environment Variables Required
    /// * `COSMOS_ENDPOINT` - The Cosmos DB endpoint URL
    /// * Azure authentication credentials (handled by AzureAuth)
    pub async fn new(
        database_name: String, 
        container_name: String
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Get the Cosmos DB endpoint from environment variables
        let cosmos_endpoint = std::env::var("COSMOS_ENDPOINT")
            .expect("COSMOS_ENDPOINT environment variable not set");
        
        // Get Azure authentication credentials
        let azure_credential = AzureAuth::get_credential_from_env();
        
        // Create the Cosmos DB client with authentication
        let cosmos_client = CosmosClient::new(&cosmos_endpoint, azure_credential, None)?;
        
        // Create a container client for the specified database and container
        let container_client = cosmos_client
            .database_client(&database_name)
            .container_client(&container_name);

        Ok(CosmosDbTelemetryStore {
            container_client: Arc::new(container_client),
        })
    }

    /// Inserts a new telemetry document into the Cosmos DB container
    /// 
    /// This method creates a new document in the database with a unique ID
    /// generated from the device ID and current timestamp. The device_id
    /// is used as the partition key for efficient storage and querying.
    /// 
    /// # Arguments
    /// * `document` - The telemetry data as a JSON value
    /// 
    /// # Returns
    /// * `Result<(), Box<dyn std::error::Error>>` - Success or an error
    pub async fn insert_telemetry(
        &self,
        document: &serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create a copy of the document and add a unique ID
        let mut document_with_id = document.clone();
        let id = format!(
            "{}-{}",
            document["device_id"],
            chrono::Utc::now().to_rfc3339()
        );
        document_with_id["id"] = serde_json::Value::String(id.clone());

        // Extract device_id for use as partition key
        let device_id = document["device_id"].as_str().unwrap().to_string();
        
        // Insert the document into the Cosmos DB container
        self.container_client
            .create_item(&device_id, &document_with_id, None)
            .await?;

        Ok(())
    }

    /// Retrieves all telemetry data for a specific device
    /// 
    /// This method queries the Cosmos DB container for all telemetry
    /// records associated with the given device ID. It uses the device_id
    /// as the partition key for efficient querying.
    /// 
    /// # Arguments
    /// * `device_id` - The unique identifier of the device
    /// 
    /// # Returns
    /// * `Result<Vec<Telemetry>, Box<dyn std::error::Error>>` - List of telemetry records or an error
    pub async fn read_telemetry(
        &self,
        device_id: &str,
    ) -> Result<Vec<Telemetry>, Box<dyn std::error::Error>> {
        // Build SQL query to find all telemetry for the specified device
        let query = format!("SELECT * FROM c WHERE c.device_id = '{}'", device_id);
        let partition_key = device_id.to_string();
        
        // Execute the query and get a pager for handling large result sets
        let mut pager = self.container_client.query_items::<Telemetry>(query, partition_key, None)?;

        // Collect all results from the pager
        let mut items = Vec::new();
        while let Some(page_response) = pager.next().await {
            let page = page_response?;
            items.extend(page.items().into_iter().cloned());
        }

        Ok(items)
    }
}
