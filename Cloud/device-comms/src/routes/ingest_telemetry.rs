use std::collections::HashMap;

use azure_core::credentials::Secret;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use azure_data_cosmos::CosmosClient;
use azure_identity::{ClientSecretCredential};

#[derive(Debug, Serialize, Deserialize)]
pub struct Telemetry {
    device_id: String,
    telemetry_data: HashMap<String, String>,
}

async fn insert_telemetry(telemetry: Json<Telemetry>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Inserting telemetry: {:?}", telemetry);
    let mut document = telemetry.into_inner();

    // Generate timestamp if not present
    let timestamp = document
        .telemetry_data
        .get("timestamp")
        .cloned()
        .unwrap_or_else(|| chrono::Utc::now().to_rfc3339());

    // Ensure timestamp is in telemetry_data
    document
        .telemetry_data
        .insert("timestamp".to_string(), timestamp.clone());

    // Create unique ID
    let id = format!("{}-{}", document.device_id, timestamp);

    let tenant_id = std::env::var("AZURE_TENANT_ID").unwrap();
    let client_id = std::env::var("AZURE_CLIENT_ID").unwrap();
    let client_secret = Secret::new(std::env::var("AZURE_CLIENT_SECRET").unwrap());

    let cosmos_endpoint = std::env::var("COSMOS_ENDPOINT").unwrap();
    let credential = ClientSecretCredential::new(&tenant_id, client_id, client_secret, None)?;
    let cosmos_client = CosmosClient::new(&cosmos_endpoint, credential, None).unwrap();
    let database_name =
        std::env::var("COSMOS_DATABASE").unwrap_or_else(|_| "device-data".to_string());
    let container_name =
        std::env::var("COSMOS_CONTAINER").unwrap_or_else(|_| "telemetry".to_string());
    let container = cosmos_client
        .database_client(&database_name)
        .container_client(&container_name);

    // Create document with id field
    let mut document_with_id = serde_json::to_value(&document)?;
    document_with_id["id"] = serde_json::Value::String(id.clone());

    // Create an item
    container
        .create_item(&document.device_id, &document_with_id, None)
        .await?;
    Ok(())
}

#[post("/ingest", data = "<telemetry>")]
pub async fn ingest(telemetry: Json<Telemetry>) -> &'static str {
    println!("Received telemetry: {:?}", telemetry);
    insert_telemetry(telemetry).await.unwrap();
    "Telemetry ingested"
}

#[cfg(test)]
mod test {
    use super::*;

    #[async_test]
    async fn test_ingest() {
        let telemetry = Telemetry {
            device_id: "test_device".to_string(),
            telemetry_data: HashMap::new(),
        };

        let result = ingest(Json(telemetry)).await;
        assert_eq!(result, "Telemetry ingested");
    }
}
