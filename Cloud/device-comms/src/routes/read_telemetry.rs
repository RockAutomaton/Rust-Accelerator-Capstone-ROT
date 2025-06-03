use rocket::serde::json::Json;
use crate::domain::telemetry::Telemetry; // If Telemetry is a struct, ensure it's pub in the module
use crate::app_state::AppState;

use rocket::State;

async fn read_telemetry(
    device_id: String,
    state: &State<AppState>,
) -> Result<Json<Vec<Telemetry>>, Box<dyn std::error::Error>> {
    println!("Reading telemetry for device: {:?}", device_id);

    let cosmos_client = state.inner().cosmos_client.clone();

    println!("Cosmos client created successfully");

    let container: Vec<Telemetry> = cosmos_client.read_telemetry(&device_id).await?;
    println!("Container client created successfully");

    Ok(Json(container))
}

#[get("/read/<device_id>")]
pub async fn read(
    device_id: String,
    state: &State<AppState>,
) -> Result<Json<Vec<Telemetry>>, rocket::response::status::NotFound<String>> {
    println!("Received request for device: {:?}", device_id);
    match read_telemetry(device_id.clone(), state).await {
        Ok(telemetry) => Ok(telemetry),
        Err(e) => {
            println!("Error reading telemetry: {:?}", e);
            Err(rocket::response::status::NotFound(format!(
                "No telemetry found for device {}: {}",
                device_id, e
            )))
        }
    }
}
