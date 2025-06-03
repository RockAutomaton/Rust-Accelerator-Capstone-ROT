use rocket::serde::json::Json;
use rocket::{State, http::Status};

use crate::domain::telemetry::{Telemetry, TelemetryError};
use crate::app_state::AppState;

async fn insert_telemetry(state: &AppState, telemetry: Json<Telemetry>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Inserting telemetry: {:?}", telemetry);

    // Parse and validate the telemetry data
    let document = Telemetry::parse(
        telemetry.device_id.clone(),
        telemetry.telemetry_data.clone(),
        telemetry.timestamp.unwrap_or_default()
    )?;

    let inserted_document = serde_json::to_value(&document)?;

    state.cosmos_client.insert_telemetry(&inserted_document).await?;
    println!("Telemetry inserted successfully");
    Ok(())
}

#[post("/ingest", data = "<telemetry>")]
pub async fn ingest(
    state: &State<AppState>, 
    telemetry: Json<Telemetry>
) -> Result<&'static str, Status> {
    println!("Received telemetry: {:?}", telemetry);
    
    match insert_telemetry(state.inner(), telemetry).await {
        Ok(()) => {
            println!("Successfully processed telemetry");
            Ok("Telemetry ingested")
        }
        Err(e) => {
            eprintln!("Error inserting telemetry: {}", e);
            match e.downcast_ref::<TelemetryError>() {
                Some(telemetry_error) => {
                    match telemetry_error {
                        TelemetryError::InvalidDeviceId => Err(Status::BadRequest),
                        TelemetryError::InvalidTimestamp => Err(Status::BadRequest),
                        TelemetryError::EmptyTelemetryData => Err(Status::BadRequest),
                        TelemetryError::InvalidTelemetryValue(_) => Err(Status::BadRequest),
                    }
                }
                None => Err(Status::InternalServerError)
            }
        }
    }
}