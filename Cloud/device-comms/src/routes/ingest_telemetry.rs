use std::collections::HashMap;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::services::AzureAuth;
use crate::services::CosmosDbTelemetryStore;

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


    let cosmos_client =
        CosmosDbTelemetryStore::new("device-data".to_string(), "telemetry".to_string());


    let id = format!("{}-{}", document.device_id, timestamp);
    let inserted_document = serde_json::json!({
        "id": id,
        "device_id": document.device_id,
        "telemetry_data": document.telemetry_data,
    });

    cosmos_client.insert_telemetry(&inserted_document).await?;
    println!("Telemetry inserted with ID: {}", id);
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
