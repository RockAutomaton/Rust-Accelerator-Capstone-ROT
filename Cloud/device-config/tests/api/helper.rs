// API Test Helper Utilities
// 
// This module provides helper functions and test fixtures for integration testing
// of the device configuration service API endpoints. It includes test application
// setup and utility functions for test data generation.

use rocket::{
    local::asynchronous::Client,
    routes,
    serde::json::Json,
};
use rocket_cors::{AllowedOrigins, CorsOptions};
use device_config::{app_state::AppState, services::CosmosDbTelemetryStore};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Global counter for generating unique test device IDs
/// 
/// This counter ensures each test gets a unique device identifier
/// to avoid conflicts between concurrent test runs.
static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Error response structure for test error handling
/// 
/// Provides a consistent error response format for test error catchers
#[derive(serde::Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

/// Test error catchers for proper error handling in tests
#[rocket::catch(422)]
fn unprocessable_entity() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "Unprocessable Entity".to_string(),
        message: "Invalid JSON format or missing required fields".to_string(),
    })
}

#[rocket::catch(400)]
fn bad_request() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "Bad Request".to_string(),
        message: "Invalid request data or validation failed".to_string(),
    })
}

#[rocket::catch(500)]
fn internal_server_error() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "Internal Server Error".to_string(),
        message: "An unexpected error occurred".to_string(),
    })
}

#[rocket::catch(404)]
fn not_found() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "Not Found".to_string(),
        message: "The requested resource was not found".to_string(),
    })
}

/// Test application instance for integration testing
/// 
/// This struct holds all the components needed to test the device
/// configuration service, including the Rocket client, configuration,
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
    /// - Error catchers for proper error handling
    /// 
    /// # Returns
    /// * `Result<Self, Box<dyn std::error::Error>>` - The configured test app or an error
    /// 
    /// # Test Configuration
    /// - Uses test database: "test-device-data"
    /// - Uses test container: "test-config"
    /// - Uses hardcoded secret key for testing
    /// - Binds to 0.0.0.0:8000
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Create test cosmos client with test database/container names
        // This ensures tests don't interfere with production data
        let cosmos_client = CosmosDbTelemetryStore::new(
            "test-device-data".to_string(), 
            "test-config".to_string()
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
            // Register error catchers for proper error handling
            .register("/", rocket::catchers![
                unprocessable_entity,
                bad_request,
                internal_server_error,
                not_found,
            ])
            .mount("/device-config", routes![
                device_config::routes::get_config::get_config_route,
                device_config::routes::update_config::update_config_route,
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

    /// Creates a sample configuration for testing
    /// 
    /// This method generates a valid configuration object that can be used
    /// in tests for both getting and updating configuration data.
    /// 
    /// # Arguments
    /// * `device_id` - The device identifier for the configuration
    /// 
    /// # Returns
    /// * `serde_json::Value` - A JSON object representing the configuration
    pub fn create_test_config(&self, device_id: &str) -> serde_json::Value {
        serde_json::json!({
            "device_id": device_id,
            "config": {
                "sampling_rate": "1000",
                "threshold": "25.5",
                "wifi_ssid": "TestNetwork",
                "wifi_password": "testpass123"
            }
        })
    }
} 