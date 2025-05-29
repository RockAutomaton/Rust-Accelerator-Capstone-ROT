use std::collections::HashMap;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use azure_data_cosmos::{CosmosClient, Query, PartitionKey};
use azure_identity::DefaultAzureCredential;
use futures::StreamExt; // Add this import


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Telemetry {
    id: String,
    device_id: String,
    telemetry_data: HashMap<String, String>,
    // Optional Cosmos DB metadata fields
    #[serde(rename = "_rid", skip_serializing_if = "Option::is_none")]
    rid: Option<String>,
    #[serde(rename = "_self", skip_serializing_if = "Option::is_none")]
    self_link: Option<String>,
    #[serde(rename = "_etag", skip_serializing_if = "Option::is_none")]
    etag: Option<String>,
    #[serde(rename = "_attachments", skip_serializing_if = "Option::is_none")]
    attachments: Option<String>,
    #[serde(rename = "_ts", skip_serializing_if = "Option::is_none")]
    timestamp: Option<i64>,
}

async fn read_telemetry(device_id: String) -> Result<Json<Vec<Telemetry>>, Box<dyn std::error::Error>> {
    println!("Reading telemetry for device: {:?}", device_id);
    
    let cosmos_endpoint = std::env::var("COSMOS_ENDPOINT")?;
    let credential = DefaultAzureCredential::new()?;
    let cosmos_client = CosmosClient::new(&cosmos_endpoint, credential, None)?;
    let database_name = std::env::var("COSMOS_DATABASE").unwrap_or_else(|_| "device-data".to_string());
    let container_name = std::env::var("COSMOS_CONTAINER").unwrap_or_else(|_| "telemetry".to_string());
    let container = cosmos_client
        .database_client(&database_name)
        .container_client(&container_name);

    // Create query using the Query struct with parameters
    let query = Query::from("SELECT * FROM c WHERE c.device_id = @device_id")
        .with_parameter("@device_id", &device_id)?;
    
    // Query items using the partition key
    let partition_key = PartitionKey::from(&device_id);
    let mut pager = container.query_items::<Telemetry>(query, partition_key, None)?;
    
    let mut items = Vec::new();
    while let Some(page_response) = pager.next().await {
        let page = page_response?;
        items.extend(page.items().iter().cloned());
    }

    println!("Found {} items for device {}", items.len(), device_id);
    Ok(Json(items))
}

#[get("/read/<device_id>")]
pub async fn read(device_id: String) -> Result<Json<Vec<Telemetry>>, rocket::response::status::NotFound<String>> {
    println!("Received request for device: {:?}", device_id);
    match read_telemetry(device_id.clone()).await {
        Ok(telemetry) => Ok(telemetry),
        Err(e) => {
            println!("Error reading telemetry: {:?}", e);
            Err(rocket::response::status::NotFound(format!("No telemetry found for device {}: {}", device_id, e)))
        }
    }
}