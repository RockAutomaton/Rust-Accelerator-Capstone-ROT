use rocket::serde::json::Json;
use rocket::State;

use crate::domain::telemetry::Telemetry;
use crate::app_state::AppState;

async fn insert_telemetry(state: &AppState, telemetry: Json<Telemetry>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Inserting telemetry: {:?}", telemetry);

    let document: Telemetry = Telemetry::new(telemetry.device_id.clone(), telemetry.telemetry_data.clone(), telemetry.timestamp.unwrap_or_default());

    let inserted_document = serde_json::to_value(&document).unwrap();

    state.cosmos_client.insert_telemetry(&inserted_document).await?;
    println!("Telemetry inserted successfully");
    Ok(())
}

#[post("/ingest", data = "<telemetry>")]
pub async fn ingest(state: &State<AppState>, telemetry: Json<Telemetry>) -> &'static str {
    println!("Received telemetry: {:?}", telemetry);
    insert_telemetry(state.inner(), telemetry).await.unwrap();
    "Telemetry ingested"
}

