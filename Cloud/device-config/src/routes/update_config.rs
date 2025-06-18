use rocket::serde::json::Json;
use rocket::{State, http::Status};
use tracing::{info, error};

use crate::domain::config::Config;
use crate::domain::error::ConfigError; 
use crate::app_state::AppState;

async fn update_config(state: &AppState, config: Json<Config>) -> Result<(), ConfigError> {
    info!("Updating config: {:?}", config);

    // Parse and validate the telemetry data
    let document = Config::parse(
        config.device_id.clone(),
        config.config.clone(),

    ).map_err(|e| match e {
        crate::domain::error::ConfigError::InvalidDeviceId => ConfigError::InvalidDeviceId,
        crate::domain::error::ConfigError::InvalidConfig => ConfigError::InvalidConfig,
        crate::domain::error::ConfigError::DatabaseError(e) => ConfigError::DatabaseError(e),
    })?;

    let inserted_document = serde_json::to_value(&document)
        .map_err(|e| ConfigError::DatabaseError(e.to_string()))?;

    state.cosmos_client.insert_config(&inserted_document)
        .await
        .map_err(|e| ConfigError::DatabaseError(e.to_string()))?;

    info!("Telemetry inserted successfully");
    Ok(())
}

#[post("/update", data = "<config>")]
pub async fn update_config_route(
    state: &State<AppState>, 
    config: Json<Config>
) -> Result<&'static str, Status> {
    info!("Received config: {:?}", config);

    match update_config(state.inner(), config).await {
        Ok(_) => {
            info!("Successfully processed config");
            Ok("Config ingested")
        }
        Err(e) => {
            error!("Error inserting telemetry: {}", e);
            Err(Status::InternalServerError)
        }
    }
}