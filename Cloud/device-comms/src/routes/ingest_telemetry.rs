use rocket::serde::json::Json;
use rocket::{State, http::Status};
use tracing::{info, error};

use crate::domain::telemetry::Telemetry;
use crate::domain::error::ApiError;
use crate::app_state::AppState;

async fn insert_telemetry(state: &AppState, telemetry: Json<Telemetry>) -> Result<(), ApiError> {
    info!("Inserting telemetry: {:?}", telemetry);

    // Parse and validate the telemetry data
    let document = Telemetry::parse(
        telemetry.device_id.clone(),
        telemetry.telemetry_data.clone(),
        telemetry.timestamp
    ).map_err(|e| match e {
        crate::domain::telemetry::TelemetryError::InvalidDeviceId => ApiError::InvalidDeviceId,
        crate::domain::telemetry::TelemetryError::InvalidTimestamp => ApiError::InvalidTimestamp,
        crate::domain::telemetry::TelemetryError::EmptyTelemetryData => ApiError::EmptyTelemetryData,
        crate::domain::telemetry::TelemetryError::InvalidTelemetryValue(msg) => ApiError::InvalidTelemetryValue(msg),
    })?;

    let inserted_document = serde_json::to_value(&document)
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    state.cosmos_client.insert_telemetry(&inserted_document)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    info!("Telemetry inserted successfully");
    Ok(())
}

#[post("/ingest", data = "<telemetry>")]
pub async fn ingest(
    state: &State<AppState>, 
    telemetry: Json<Telemetry>
) -> Result<&'static str, Status> {
    info!("Received telemetry: {:?}", telemetry);
    
    match insert_telemetry(state.inner(), telemetry).await {
        Ok(()) => {
            info!("Successfully processed telemetry");
            Ok("Telemetry ingested")
        }
        Err(e) => {
            error!("Error inserting telemetry: {}", e);
            Err(e.into())
        }
    }
}