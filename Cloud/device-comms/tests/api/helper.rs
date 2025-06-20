// Test Helper Utilities
// 
// This module provides utilities for setting up and managing test instances
// of the device communications service. It includes test application setup
// and helper functions for integration testing.

use rocket::{
    local::asynchronous::Client,
    routes,
};
use rocket_cors::{AllowedOrigins, CorsOptions};
use device_comms::{app_state::AppState, services::CosmosDbTelemetryStore};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Global counter for generating unique test device IDs
/// 
/// This counter ensures that each test uses a unique device ID,
/// preventing test interference and ensuring test isolation.
static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Test application instance for integration testing
/// 
/// This struct holds all the components needed to run integration tests
/// against the device communications service, including the Rocket client,
/// application state, and configuration.
#[allow(dead_code)]
pub struct TestApp {
    /// Rocket test client for making HTTP requests to the test server
    pub client: Client,
    /// Server address for the test instance
    pub address: String,
    /// Server port for the test instance
    pub port: u16,
    /// Application state with test database client
    pub app_state: AppState,
}

impl TestApp {
    /// Creates a new test application instance
    /// 
    /// This method sets up a complete test environment including:
    /// - Test Cosmos DB client with test database/container names
    /// - Rocket server with test configuration
    /// - CORS configuration for test requests
    /// - Application state with test dependencies
    /// 
    /// # Returns
    /// * `Result<Self, Box<dyn std::error::Error>>` - The configured test app or an error
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Create test cosmos client with test database and container names
        // This ensures tests don't interfere with production data
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
                // Use a test secret key (64 hex characters)
                .merge(("secret_key", "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"))
                .merge(("address", "0.0.0.0")))
            .manage(app_state.clone()) // Inject the test application state
            .attach(cors) // Enable CORS for test requests
            .mount("/iot/data", routes![
                device_comms::routes::ingest_telemetry::ingest,
            ]);

        // Create a tracked client for making test requests
        let client = Client::tracked(server).await?;

        Ok(Self {
            client,
            address: "0.0.0.0".to_string(),
            port: 8000,
            app_state,
        })
    }

    /// Generates a unique test device ID
    /// 
    /// This method uses an atomic counter to ensure each test gets a unique
    /// device ID, preventing test interference and making test results
    /// more reliable and debuggable.
    /// 
    /// # Returns
    /// * `String` - A unique device ID for testing
    pub fn generate_test_device_id(&self) -> String {
        let count = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("test_device_{}", count)
    }
}