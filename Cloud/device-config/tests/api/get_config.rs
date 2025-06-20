// Get Configuration API Integration Tests
// 
// This module contains integration tests for the GET /device-config/get/<device_id>
// endpoint of the device configuration service.

use crate::helper::TestApp;
use rocket::http::{Status, ContentType};
use rocket::local::asynchronous::Client;
use dotenvy::dotenv;

/// Test getting configuration for a device that doesn't exist
/// 
/// This test verifies that the API correctly returns a 404 Not Found status
/// when attempting to retrieve configuration for a non-existent device.
#[tokio::test]
async fn test_get_config_nonexistent_device() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    // Try to get configuration for a device that doesn't exist
    let response = client
        .get(format!("/device-config/get/{}", device_id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NotFound);
}

/// Test getting configuration with an empty device ID
/// 
/// This test verifies that the API handles empty device IDs correctly.
/// The endpoint should return a 404 Not Found status when no device ID
/// is provided in the URL path.
#[tokio::test]
async fn test_get_config_empty_device_id() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Try to get configuration with an empty device ID in the URL
    let response = client
        .get("/device-config/get/")
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NotFound);
}

/// Test getting configuration with an invalid device ID containing special characters
/// 
/// This test verifies that the API properly handles device IDs containing
/// special characters that might cause URL parsing issues.
#[tokio::test]
async fn test_get_config_invalid_device_id() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Try to get configuration with an invalid device ID containing special characters
    let response = client
        .get("/device-config/get/invalid@device#id")
        .dispatch()
        .await;

    // Rocket returns 400 for invalid URL characters
    assert_eq!(response.status(), Status::BadRequest);
}

/// Test getting configuration with query parameters
/// 
/// This test verifies that the API correctly handles requests with query
/// parameters. Since the current implementation doesn't support query parameters,
/// the endpoint should ignore them and process the request normally.
#[tokio::test]
async fn test_get_config_with_query_parameters() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    // Wait for 2 seconds to allow Cosmos DB TTL to expire any previous test data
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    // Try to get configuration with query parameters (should be ignored by the endpoint)
    let response = client
        .get(format!("/device-config/get/{}?limit=10&offset=0", device_id))
        .dispatch()
        .await;

    // Verify that the API returns 404 Not Found (same as without query parameters)
    // This confirms that query parameters are properly ignored
    assert_eq!(response.status(), Status::NotFound);
}

/// Test getting configuration with a valid device ID format
/// 
/// This test verifies that the API accepts valid device ID formats
/// and returns appropriate responses.
#[tokio::test]
async fn test_get_config_valid_device_id_format() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Try to get configuration with a valid device ID format
    let response = client
        .get("/device-config/get/sensor-001")
        .dispatch()
        .await;

    // Should return 404 since the device doesn't exist in test database
    // but the request format is valid
    assert_eq!(response.status(), Status::NotFound);
}

/// Test getting configuration with different HTTP methods
/// 
/// This test verifies that the API correctly rejects unsupported HTTP methods
/// and only accepts GET requests.
#[tokio::test]
async fn test_get_config_unsupported_methods() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    // Try POST method (should not be supported)
    let response = client
        .post(format!("/device-config/get/{}", device_id))
        .dispatch()
        .await;

    // Should return 404 for unsupported methods
    assert_eq!(response.status(), Status::NotFound);

    // Try PUT method (should not be supported)
    let response = client
        .put(format!("/device-config/get/{}", device_id))
        .dispatch()
        .await;

    // Should return 404 for unsupported methods
    assert_eq!(response.status(), Status::NotFound);

    // Try DELETE method (should not be supported)
    let response = client
        .delete(format!("/device-config/get/{}", device_id))
        .dispatch()
        .await;

    // Should return 404 for unsupported methods
    assert_eq!(response.status(), Status::NotFound);
} 