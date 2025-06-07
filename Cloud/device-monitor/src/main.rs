use device_monitor::{services::CosmosDbTelemetryStore, Application};
use device_monitor::utils::tracing::init_tracing;

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    
    // Initialize tracing
    init_tracing()?;
    
    let cosmos_client = configure_cosmos_client().await;
    let app_state = device_monitor::app_state::AppState::new(cosmos_client);
    let app = Application::build(app_state).await?;
    app.server.launch().await?;
    Ok(())
}

async fn configure_cosmos_client() -> CosmosDbTelemetryStore {
   let cosmos_client = CosmosDbTelemetryStore::new("device-data".to_string(), "telemetry".to_string());
   cosmos_client.await.unwrap()
}