// Main entry point for the device configuration service
// This service handles device configuration management and retrieval
use device_config::{services::CosmosDbTelemetryStore, Application};
use device_config::utils::tracing::init_tracing;

/// Main application entry point
/// 
/// This function:
/// 1. Loads environment variables from .env file
/// 2. Initializes tracing/logging infrastructure
/// 3. Configures the Cosmos DB client for configuration storage
/// 4. Creates the application state with the database client
/// 5. Builds and launches the Rocket web server
#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file if it exists
    dotenvy::dotenv().ok();
    
    // Initialize structured logging and tracing infrastructure
    init_tracing()?;
    
    // Configure and create the Cosmos DB client for configuration storage
    let cosmos_client = configure_cosmos_client().await;
    
    // Create application state with the configured database client
    let app_state = device_config::app_state::AppState::new(cosmos_client);
    
    // Build the Rocket application with the configured state
    let app = Application::build(app_state).await?;
    
    // Launch the web server and wait for it to complete
    app.server.launch().await?;
    Ok(())
}

/// Configures and initializes the Cosmos DB configuration store client
/// 
/// Creates a new CosmosDbTelemetryStore instance with:
/// - Database name: "device-config"
/// - Container name: "config"
/// 
/// Returns a configured client ready for device configuration operations
async fn configure_cosmos_client() -> CosmosDbTelemetryStore {
   let cosmos_client = CosmosDbTelemetryStore::new("device-config".to_string(), "config".to_string());
   cosmos_client.await.unwrap()
}