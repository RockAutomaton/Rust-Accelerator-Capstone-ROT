use device_comms::{services::CosmosDbTelemetryStore, Application};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let cosmos_client = configure_cosmos_client().await;
    let app_state = device_comms::app_state::AppState::new(cosmos_client);
    let app = Application::build(app_state).await?;
    app.server.launch().await?;
    Ok(())
}

async fn configure_cosmos_client() -> CosmosDbTelemetryStore {
   let cosmos_client = CosmosDbTelemetryStore::new("device-data".to_string(), "telemetry".to_string());
   cosmos_client.await.unwrap()
}