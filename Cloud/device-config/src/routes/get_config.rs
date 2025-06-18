use rocket::serde::json::Json;
use rocket::{State, http::Status};
use tracing::{info, error};

use crate::domain::config::Config;
use crate::domain::error::ConfigError; 
use crate::app_state::AppState;

async fn get_config(state: &AppState, device_id: String) -> Result<Vec<Config>, ConfigError> {
    info!("Getting config: {:?}", device_id);

    let config = state.cosmos_client.read_config(&device_id)
        .await
        .map_err(|e| ConfigError::DatabaseError(e.to_string()))?;

    info!("Config retrieved successfully");
    Ok(config)
}

#[get("/get/<device_id>")]
pub async fn get_config_route(
    state: &State<AppState>, 
    device_id: String
) -> Result<Json<Vec<Config>>, Status> {
    info!("Received config: {:?}", device_id);

    match get_config(state.inner(), device_id).await {
        Ok(config) => {
            info!("Successfully processed config");
            Ok(Json(config))
        }
        Err(e) => {
            error!("Error inserting telemetry: {}", e);
            Err(Status::InternalServerError)
        }
    }
}