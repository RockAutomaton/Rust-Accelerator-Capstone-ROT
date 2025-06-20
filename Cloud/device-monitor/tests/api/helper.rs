// API Test Helper Utilities
// 
// This module provides helper functions and test fixtures for integration testing
// of the device monitoring service API endpoints. It includes test application
// setup and utility functions for test data generation.

use rocket::{
    local::asynchronous::Client,
    routes,
};
use rocket_cors::{AllowedOrigins, CorsOptions};
use device_monitor::{app_state::AppState, services::CosmosDbTelemetryStore};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Global counter for generating unique test device IDs
/// 
/// This counter ensures each test gets a unique device identifier
/// to avoid conflicts between concurrent test runs.
static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Test application instance for integration testing
/// 
/// This struct holds all the components needed to test the device
/// monitoring service, including the Rocket client, configuration,
/// and application state.
#[allow(dead_code)]
pub struct TestApp {
    /// Rocket client for making HTTP requests to the test application
    pub client: Client,
    /// Server address for the test application
    pub address: String,
    /// Server port for the test application
    pub port: u16,
    /// Application state with test database client
    pub app_state: AppState,
}

impl TestApp {
    /// Creates a new test application instance
    /// 
    /// This method sets up a complete test environment including:
    /// - Test Cosmos DB client with test database/container names
    /// - Rocket application with test configuration
    /// - CORS configuration for cross-origin requests
    /// - Mounted API routes for testing
    /// 
    /// # Returns
    /// * `Result<Self, Box<dyn std::error::Error>>` - The configured test app or an error
    /// 
    /// # Test Configuration
    /// - Uses test database: "test-device-data"
    /// - Uses test container: "test-telemetry"
    /// - Uses hardcoded secret key for testing
    /// - Binds to 0.0.0.0:8000
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Create test cosmos client with test database/container names
        // In a real test environment, you might want to use separate test resources
        let cosmos_client = CosmosDbTelemetryStore::new(
            "test-device-data".to_string(), 
            "test-telemetry".to_string()
        ).await?;
        
        // Create application state with the test database client
        let app_state = AppState::new(cosmos_client);

        // Configure CORS for test requests (allows all origins for testing)
        let cors = CorsOptions {
            allowed_origins: AllowedOrigins::All,
            ..Default::default()
        }
        .to_cors()?;

        // Build the Rocket test server with test configuration
        let server = rocket::build()
            .configure(rocket::Config::figment()
                // Use a hardcoded secret key for testing (64 hex characters)
                .merge(("secret_key", "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"))
                .merge(("address", "0.0.0.0")))
            .manage(app_state.clone()) // Inject the test application state
            .attach(cors) // Enable CORS for test requests
            .mount("/iot/data", routes![
                device_monitor::routes::read_telemetry::read,
            ]);

        // Create a tracked client for making requests to the test server
        let client = Client::tracked(server).await?;

        Ok(Self {
            client,
            address: "0.0.0.0".to_string(),
            port: 8000,
            app_state,
        })
    }

    /// Generates a unique test device ID for each test
    /// 
    /// This method uses an atomic counter to ensure each test gets
    /// a unique device identifier, preventing conflicts between
    /// concurrent test runs.
    /// 
    /// # Returns
    /// * `String` - A unique device ID in the format "test_device_{counter}"
    pub fn generate_test_device_id(&self) -> String {
        let count = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("test_device_{}", count)
    }
}