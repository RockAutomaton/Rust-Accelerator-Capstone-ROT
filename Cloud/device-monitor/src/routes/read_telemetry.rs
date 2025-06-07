use rocket::serde::json::Json;
use rocket::{State, http::Status};
use tracing::{info, error};
use crate::domain::telemetry::Telemetry;
use crate::domain::error::ApiError;
use crate::app_state::AppState;

async fn read_telemetry(
    device_id: &str,
    state: &State<AppState>,
) -> Result<Json<Vec<Telemetry>>, ApiError> {
    info!("Reading telemetry for device: {}", device_id);

    // Validate device_id
    if device_id.trim().is_empty() {
        error!("Empty device ID provided");
        return Err(ApiError::DeviceNotFound(device_id.to_string()));
    }

    let cosmos_client = state.inner().cosmos_client.clone();
    let container = cosmos_client.read_telemetry(device_id)
        .await
        .map_err(|e| {
            error!("Database error reading telemetry: {}", e);
            ApiError::DatabaseError(e.to_string())
        })?;

    if container.is_empty() {
        info!("No telemetry found for device: {}", device_id);
        return Err(ApiError::DeviceNotFound(device_id.to_string()));
    }

    info!("Found {} telemetry entries for device: {}", container.len(), device_id);
    Ok(Json(container))
}

#[get("/read/<device_id>")]
pub async fn read(
    device_id: &str,
    state: &State<AppState>,
) -> Result<Json<Vec<Telemetry>>, Status> {
    info!("Received request for device: {}", device_id);
    
    match read_telemetry(device_id, state).await {
        Ok(telemetry) => {
            info!("Successfully retrieved telemetry for device: {}", device_id);
            Ok(telemetry)
        }
        Err(e) => {
            error!("Error reading telemetry: {}", e);
            Err(e.into())
        }
    }
}
