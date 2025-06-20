// Telemetry Retrieval Route Handler
// 
// This module handles the GET /iot/data/read/<device_id> endpoint for
// retrieving telemetry data from IoT devices for monitoring purposes.

use rocket::serde::json::Json;
use rocket::{State, http::Status};
use tracing::{info, error};
use crate::domain::telemetry::Telemetry;
use crate::domain::error::ApiError;
use crate::app_state::AppState;

/// Retrieves telemetry data for a specific device from the database
/// 
/// This function queries the Cosmos DB container for all telemetry
/// records associated with the given device ID. It performs validation
/// and error handling for the monitoring use case.
/// 
/// # Arguments
/// * `device_id` - The unique identifier of the device to monitor
/// * `state` - Application state containing the database client
/// 
/// # Returns
/// * `Result<Json<Vec<Telemetry>>, ApiError>` - List of telemetry records or an error
async fn read_telemetry(
    device_id: &str,
    state: &State<AppState>,
) -> Result<Json<Vec<Telemetry>>, ApiError> {
    info!("Reading telemetry for device: {}", device_id);

    // Validate device_id is not empty
    if device_id.trim().is_empty() {
        error!("Empty device ID provided");
        return Err(ApiError::DeviceNotFound(device_id.to_string()));
    }

    // Get a clone of the Cosmos DB client for database operations
    let cosmos_client = state.inner().cosmos_client.clone();
    
    // Query the database for telemetry data for the specified device
    let container = cosmos_client.read_telemetry(device_id)
        .await
        .map_err(|e| {
            error!("Database error reading telemetry: {}", e);
            ApiError::DatabaseError(e.to_string())
        })?;

    // Check if any telemetry data was found for the device
    if container.is_empty() {
        info!("No telemetry found for device: {}", device_id);
        return Err(ApiError::DeviceNotFound(device_id.to_string()));
    }

    info!("Found {} telemetry entries for device: {}", container.len(), device_id);
    Ok(Json(container))
}

/// GET endpoint for retrieving device telemetry data for monitoring
/// 
/// This endpoint retrieves all telemetry data for a specific device
/// from the database. The endpoint expects a device ID as a path parameter
/// and returns a JSON array of telemetry records for monitoring purposes.
/// 
/// # Arguments
/// * `device_id` - The device identifier from the URL path
/// * `state` - Application state injected by Rocket
/// 
/// # Returns
/// * `Result<Json<Vec<Telemetry>>, Status>` - JSON array of telemetry records or HTTP error status
/// 
/// # Example Request
/// ```bash
/// GET /iot/data/read/sensor-001
/// ```
/// 
/// # Example Response
/// ```json
/// [
///   {
///     "device_id": "sensor-001",
///     "telemetry_data": {
///       "temperature": "23.5",
///       "humidity": "45.2"
///     },
///     "timestamp": 1640995200
///   },
///   {
///     "device_id": "sensor-001",
///     "telemetry_data": {
///       "temperature": "24.1",
///       "humidity": "44.8"
///     },
///     "timestamp": 1640995260
///   }
/// ]
/// ```
#[get("/read/<device_id>")]
pub async fn read(
    device_id: &str,
    state: &State<AppState>,
) -> Result<Json<Vec<Telemetry>>, Status> {
    info!("Received telemetry monitoring request for device: {}", device_id);
    
    // Retrieve the telemetry data and handle any errors
    match read_telemetry(device_id, state).await {
        Ok(telemetry) => {
            info!("Successfully retrieved telemetry for device: {}", device_id);
            Ok(telemetry)
        }
        Err(e) => {
            error!("Error reading telemetry: {}", e);
            // Convert the API error to an appropriate HTTP status code
            Err(e.into())
        }
    }
}
