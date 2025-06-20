// Telemetry Ingestion Route Handler
// 
// This module handles the POST /iot/data/ingest endpoint for receiving
// and storing telemetry data from IoT devices.

use rocket::serde::json::Json;
use rocket::{State, http::Status};
use tracing::{info, error};

use crate::domain::telemetry::Telemetry;
use crate::domain::error::ApiError;
use crate::app_state::AppState;

/// Processes and stores telemetry data in the database
/// 
/// This function validates the incoming telemetry data and stores it
/// in the Cosmos DB database. It performs the following steps:
/// 1. Validates the telemetry data using domain validation rules
/// 2. Converts the validated data to JSON format for storage
/// 3. Inserts the data into the Cosmos DB container
/// 
/// # Arguments
/// * `state` - Application state containing the database client
/// * `telemetry` - The telemetry data to be processed and stored
/// 
/// # Returns
/// * `Result<(), ApiError>` - Success or an appropriate error
async fn insert_telemetry(state: &AppState, telemetry: Json<Telemetry>) -> Result<(), ApiError> {
    info!("Inserting telemetry: {:?}", telemetry);

    // Parse and validate the telemetry data using domain validation rules
    let document = Telemetry::parse(
        telemetry.device_id.clone(),
        telemetry.telemetry_data.clone(),
        telemetry.timestamp
    ).map_err(|e| match e {
        // Map domain validation errors to API errors
        crate::domain::telemetry::TelemetryError::InvalidDeviceId => ApiError::InvalidDeviceId,
        crate::domain::telemetry::TelemetryError::InvalidTimestamp => ApiError::InvalidTimestamp,
        crate::domain::telemetry::TelemetryError::EmptyTelemetryData => ApiError::EmptyTelemetryData,
        crate::domain::telemetry::TelemetryError::InvalidTelemetryValue(msg) => ApiError::InvalidTelemetryValue(msg),
    })?;

    // Convert the validated telemetry to JSON format for database storage
    let inserted_document = serde_json::to_value(&document)
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    // Insert the telemetry data into the Cosmos DB container
    state.cosmos_client.insert_telemetry(&inserted_document)
        .await
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    info!("Telemetry inserted successfully");
    Ok(())
}

/// POST endpoint for ingesting telemetry data from IoT devices
/// 
/// This endpoint receives telemetry data from IoT devices and stores it
/// in the database. The endpoint expects a JSON payload containing:
/// - device_id: Unique identifier for the IoT device
/// - telemetry_data: Key-value pairs of sensor readings
/// - timestamp: Optional Unix timestamp (uses current time if not provided)
/// 
/// # Arguments
/// * `state` - Application state injected by Rocket
/// * `telemetry` - JSON payload containing the telemetry data
/// 
/// # Returns
/// * `Result<&'static str, Status>` - Success message or HTTP error status
/// 
/// # Example Request
/// ```json
/// {
///   "device_id": "sensor-001",
///   "telemetry_data": {
///     "temperature": "23.5",
///     "humidity": "45.2"
///   },
///   "timestamp": 1640995200
/// }
/// ```
#[post("/ingest", data = "<telemetry>")]
pub async fn ingest(
    state: &State<AppState>, 
    telemetry: Json<Telemetry>
) -> Result<&'static str, Status> {
    info!("Received telemetry: {:?}", telemetry);
    
    // Process the telemetry data and handle any errors
    match insert_telemetry(state.inner(), telemetry).await {
        Ok(()) => {
            info!("Successfully processed telemetry");
            Ok("Telemetry ingested")
        }
        Err(e) => {
            error!("Error inserting telemetry: {}", e);
            // Convert the API error to an appropriate HTTP status code
            Err(e.into())
        }
    }
}