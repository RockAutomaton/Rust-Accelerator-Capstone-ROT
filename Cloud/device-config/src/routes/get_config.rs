// Configuration Retrieval Route Handler
// 
// This module handles the GET /device-config/get/<device_id> endpoint for
// retrieving device configuration data from the database.

use rocket::serde::json::Json;
use rocket::{State, http::Status};
use tracing::{info, error};

use crate::domain::config::Config;
use crate::domain::error::ConfigError; 
use crate::app_state::AppState;

/// Retrieves configuration data for a specific device from the database
/// 
/// This function queries the Cosmos DB container for all configuration
/// records associated with the given device ID. It uses the device_id
/// as the partition key for efficient querying.
/// 
/// # Arguments
/// * `state` - Application state containing the database client
/// * `device_id` - The unique identifier of the device
/// 
/// # Returns
/// * `Result<Vec<Config>, ConfigError>` - List of configuration records or an error
async fn get_config(state: &AppState, device_id: String) -> Result<Vec<Config>, ConfigError> {
    info!("Getting config: {:?}", device_id);

    // Query the database for configuration data for the specified device
    let config = state.cosmos_client.read_config(&device_id)
        .await
        .map_err(|e| ConfigError::DatabaseError(e.to_string()))?;

    info!("Config retrieved successfully");
    Ok(config)
}

/// GET endpoint for retrieving device configuration data
/// 
/// This endpoint retrieves all configuration data for a specific device
/// from the database. The endpoint expects a device ID as a path parameter
/// and returns a JSON array of configuration records.
/// 
/// # Arguments
/// * `state` - Application state injected by Rocket
/// * `device_id` - The device identifier from the URL path
/// 
/// # Returns
/// * `Result<Json<Vec<Config>>, Status>` - JSON array of configurations or HTTP error status
/// 
/// # Example Request
/// ```
/// GET /device-config/get/sensor-001
/// ```
/// 
/// # Example Response
/// ```json
/// [
///   {
///     "device_id": "sensor-001",
///     "config": {
///       "sampling_rate": "1000",
///       "threshold": "25.5"
///     }
///   }
/// ]
/// ```
#[get("/get/<device_id>")]
pub async fn get_config_route(
    state: &State<AppState>, 
    device_id: String
) -> Result<Json<Vec<Config>>, Status> {
    info!("Received config request for device: {:?}", device_id);

    // Retrieve the configuration data and handle any errors
    match get_config(state.inner(), device_id).await {
        Ok(config) => {
            info!("Successfully retrieved configuration data");
            Ok(Json(config))
        }
        Err(e) => {
            error!("Error retrieving configuration: {}", e);
            // Convert the configuration error to an appropriate HTTP status
            Err(e.into())
        }
    }
}