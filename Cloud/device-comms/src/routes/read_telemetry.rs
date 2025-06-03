use rocket::serde::json::Json;
use rocket::{State, http::Status};
use crate::domain::telemetry::Telemetry;
use crate::app_state::AppState;

#[derive(Debug)]
pub enum ReadTelemetryError {
    DeviceNotFound(String),
    DatabaseError(String),
}

impl std::fmt::Display for ReadTelemetryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadTelemetryError::DeviceNotFound(device_id) => 
                write!(f, "No telemetry found for device {}", device_id),
            ReadTelemetryError::DatabaseError(msg) => 
                write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for ReadTelemetryError {}

async fn read_telemetry(
    device_id: String,
    state: &State<AppState>,
) -> Result<Json<Vec<Telemetry>>, ReadTelemetryError> {
    println!("Reading telemetry for device: {:?}", device_id);

    // Validate device_id
    if device_id.trim().is_empty() {
        return Err(ReadTelemetryError::DeviceNotFound(device_id));
    }

    let cosmos_client = state.inner().cosmos_client.clone();
    let container = cosmos_client.read_telemetry(&device_id)
        .await
        .map_err(|e| ReadTelemetryError::DatabaseError(e.to_string()))?;

    if container.is_empty() {
        return Err(ReadTelemetryError::DeviceNotFound(device_id));
    }

    Ok(Json(container))
}

#[get("/read/<device_id>")]
pub async fn read(
    device_id: String,
    state: &State<AppState>,
) -> Result<Json<Vec<Telemetry>>, Status> {
    println!("Received request for device: {:?}", device_id);
    
    match read_telemetry(device_id.clone(), state).await {
        Ok(telemetry) => Ok(telemetry),
        Err(e) => {
            println!("Error reading telemetry: {:?}", e);
            match e {
                ReadTelemetryError::DeviceNotFound(_) => Err(Status::NotFound),
                ReadTelemetryError::DatabaseError(_) => Err(Status::InternalServerError),
            }
        }
    }
}
