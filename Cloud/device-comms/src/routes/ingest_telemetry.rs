use rocket::serde::json::Json;


use crate::domain::Telemetry::Telemetry;
use crate::services::CosmosDbTelemetryStore;

async fn insert_telemetry(telemetry: Json<Telemetry>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Inserting telemetry: {:?}", telemetry);

    let document: Telemetry = Telemetry::new(telemetry.device_id.clone(), telemetry.telemetry_data.clone(), telemetry.timestamp.unwrap_or_default());

    // Create unique ID

    let cosmos_client =
        CosmosDbTelemetryStore::new("device-data".to_string(), "telemetry".to_string());
    let inserted_document = serde_json::to_value(&document).unwrap();


    cosmos_client.insert_telemetry(&inserted_document).await?;
    println!("Telemetry inserted successfully");
    Ok(())
}

#[post("/ingest", data = "<telemetry>")]
pub async fn ingest(telemetry: Json<Telemetry>) -> &'static str {
    println!("Received telemetry: {:?}", telemetry);
    insert_telemetry(telemetry).await.unwrap();
    "Telemetry ingested"
}

