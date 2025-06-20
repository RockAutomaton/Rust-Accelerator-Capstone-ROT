// Configuration Update Route Handler
// 
// This module handles the POST /device-config/update endpoint for
// updating device configuration data in the database.

use rocket::serde::json::Json;
use rocket::{State, http::Status};
use tracing::{info, error};

use crate::domain::config::Config;
use crate::domain::error::ConfigError; 
use crate::app_state::AppState;

/// Processes and stores configuration data in the database
/// 
/// This function validates the incoming configuration data and stores it
/// in the Cosmos DB database. It performs the following steps:
/// 1. Validates the configuration data using domain validation rules
/// 2. Converts the validated data to JSON format for storage
/// 3. Inserts the data into the Cosmos DB container
/// 
/// # Arguments
/// * `state` - Application state containing the database client
/// * `config` - The configuration data to be processed and stored
/// 
/// # Returns
/// * `Result<(), ConfigError>` - Success or an appropriate error
async fn update_config(state: &AppState, config: Json<Config>) -> Result<(), ConfigError> {
    info!("Updating config: {:?}", config);

    // Parse and validate the configuration data using domain validation rules
    let document = Config::parse(
        config.device_id.clone(),
        config.config.clone(),

    ).map_err(|e| match e {
        // Map domain validation errors to configuration errors
        crate::domain::error::ConfigError::InvalidDeviceId => ConfigError::InvalidDeviceId,
        crate::domain::error::ConfigError::InvalidConfig => ConfigError::InvalidConfig,
        crate::domain::error::ConfigError::DatabaseError(e) => ConfigError::DatabaseError(e),
    })?;

    // Convert the validated configuration to JSON format for database storage
    let inserted_document = serde_json::to_value(&document)
        .map_err(|e| ConfigError::DatabaseError(e.to_string()))?;

    // Insert the configuration data into the Cosmos DB container
    state.cosmos_client.insert_config(&inserted_document)
        .await
        .map_err(|e| ConfigError::DatabaseError(e.to_string()))?;

    info!("Configuration updated successfully");
    Ok(())
}

/// POST endpoint for updating device configuration data
/// 
/// This endpoint receives configuration data for a device and stores it
/// in the database. The endpoint expects a JSON payload containing:
/// - device_id: Unique identifier for the IoT device
/// - config: Key-value pairs of configuration parameters
/// 
/// # Arguments
/// * `state` - Application state injected by Rocket
/// * `config` - JSON payload containing the configuration data
/// 
/// # Returns
/// * `Result<&'static str, Status>` - Success message or HTTP error status
/// 
/// # Example Request
/// ```json
/// {
///   "device_id": "sensor-001",
///   "config": {
///     "sampling_rate": "1000",
///     "threshold": "25.5",
///     "wifi_ssid": "MyNetwork"
///   }
/// }
/// ```
/// 
/// # Example Response
/// ```
/// Config ingested
/// ```
#[post("/update", data = "<config>")]
pub async fn update_config_route(
    state: &State<AppState>, 
    config: Json<Config>
) -> Result<&'static str, Status> {
    info!("Received configuration update request: {:?}", config);

    // Process the configuration data and handle any errors
    match update_config(state.inner(), config).await {
        Ok(_) => {
            info!("Successfully processed configuration update");
            Ok("Config ingested")
        }
        Err(e) => {
            error!("Error updating configuration: {}", e);
            // Convert the configuration error to an appropriate HTTP status
            Err(e.into())
        }
    }
}