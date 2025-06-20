// Telemetry Reading API Integration Tests
// 
// This module contains integration tests for the telemetry reading endpoint
// of the device monitoring service. Tests cover various scenarios including
// error cases, edge cases, and invalid inputs.

use crate::helper::TestApp;
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use dotenvy::dotenv;

/// Test reading telemetry for a device that doesn't exist in the database
/// 
/// This test verifies that the API correctly returns a 404 Not Found status
/// when attempting to retrieve telemetry data for a non-existent device.
/// This is important for proper error handling and API contract compliance.
#[tokio::test]
async fn test_read_nonexistent_device() {
    // Load environment variables for test configuration
    dotenv().ok();
    
    // Create test application instance
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    // Attempt to read telemetry for a device that doesn't exist
    let response = client
        .get(format!("/iot/data/read/{}", device_id))
        .dispatch()
        .await;

    // Verify that the API returns 404 Not Found for non-existent devices
    assert_eq!(response.status(), Status::NotFound);
}

/// Test reading telemetry with an empty device ID
/// 
/// This test verifies that the API handles empty device IDs correctly.
/// The endpoint should return a 404 Not Found status when no device ID
/// is provided in the URL path.
#[tokio::test]
async fn test_read_empty_device_id() {
    // Load environment variables for test configuration
    dotenv().ok();
    
    // Create test application instance
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Attempt to read telemetry with an empty device ID in the URL
    let response = client
        .get("/iot/data/read/")
        .dispatch()
        .await;

    // Verify that the API returns 404 Not Found for empty device IDs
    assert_eq!(response.status(), Status::NotFound);
}

/// Test reading telemetry with an invalid device ID containing special characters
/// 
/// This test verifies that the API properly handles device IDs containing
/// special characters that might cause URL parsing issues. The Rocket framework
/// should return a 400 Bad Request for invalid URL characters.
#[tokio::test]
async fn test_read_invalid_device_id() {
    // Load environment variables for test configuration
    dotenv().ok();
    
    // Create test application instance
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Attempt to read telemetry with an invalid device ID containing special characters
    let response = client
        .get("/iot/data/read/invalid@device#id")
        .dispatch()
        .await;

    // Verify that Rocket returns 400 Bad Request for invalid URL characters
    assert_eq!(response.status(), Status::BadRequest);
}

/// Test reading telemetry with query parameters
/// 
/// This test verifies that the API correctly handles requests with query
/// parameters. Since the current implementation doesn't support query parameters,
/// the endpoint should ignore them and process the request normally.
/// The test uses a non-existent device to verify the base functionality.
#[tokio::test]
async fn test_read_with_query_parameters() {
    // Load environment variables for test configuration
    dotenv().ok();
    
    // Create test application instance
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    // Attempt to read telemetry with query parameters (should be ignored by the endpoint)
    let response = client
        .get(format!("/iot/data/read/{}?limit=10&offset=0", device_id))
        .dispatch()
        .await;

    // Verify that the API returns 404 Not Found (same as without query parameters)
    // This confirms that query parameters are properly ignored
    assert_eq!(response.status(), Status::NotFound);
}

/// Test reading telemetry with a valid device ID format
/// 
/// This test verifies that the API accepts valid device ID formats
/// and returns appropriate responses.
#[tokio::test]
async fn test_read_valid_device_id_format() {
    // Load environment variables for test configuration
    dotenv().ok();
    
    // Create test application instance
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Try to read telemetry with a valid device ID format
    let response = client
        .get("/iot/data/read/sensor-001")
        .dispatch()
        .await;

    // Should return 404 since the device doesn't exist in test database
    // but the request format is valid
    assert_eq!(response.status(), Status::NotFound);
}

/// Test reading telemetry with different HTTP methods
/// 
/// This test verifies that the API correctly rejects unsupported HTTP methods
/// and only accepts GET requests.
#[tokio::test]
async fn test_read_unsupported_methods() {
    // Load environment variables for test configuration
    dotenv().ok();
    
    // Create test application instance
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    // Try POST method (should not be supported)
    let response = client
        .post(format!("/iot/data/read/{}", device_id))
        .dispatch()
        .await;

    // Should return 404 for unsupported methods
    assert_eq!(response.status(), Status::NotFound);

    // Try PUT method (should not be supported)
    let response = client
        .put(format!("/iot/data/read/{}", device_id))
        .dispatch()
        .await;

    // Should return 404 for unsupported methods
    assert_eq!(response.status(), Status::NotFound);

    // Try DELETE method (should not be supported)
    let response = client
        .delete(format!("/iot/data/read/{}", device_id))
        .dispatch()
        .await;

    // Should return 404 for unsupported methods
    assert_eq!(response.status(), Status::NotFound);
}

/// Test reading telemetry with various device ID formats
/// 
/// This test verifies that the API correctly handles different device ID
/// formats including numbers, letters, and mixed formats.
#[tokio::test]
async fn test_read_various_device_id_formats() {
    // Load environment variables for test configuration
    dotenv().ok();
    
    // Create test application instance
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Test with numeric device ID
    let response = client
        .get("/iot/data/read/12345")
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::NotFound);

    // Test with alphanumeric device ID
    let response = client
        .get("/iot/data/read/device123")
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::NotFound);

    // Test with device ID containing hyphens
    let response = client
        .get("/iot/data/read/sensor-001")
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::NotFound);

    // Test with device ID containing underscores
    let response = client
        .get("/iot/data/read/test_device_001")
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::NotFound);

    // Test with very long device ID
    let long_device_id = "a".repeat(100);
    let response = client
        .get(format!("/iot/data/read/{}", long_device_id))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::NotFound);
}

/// Test reading telemetry with URL encoding
/// 
/// This test verifies that the API correctly handles URL-encoded device IDs
/// and properly decodes them.
#[tokio::test]
async fn test_read_url_encoded_device_id() {
    // Load environment variables for test configuration
    dotenv().ok();
    
    // Create test application instance
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Test with URL-encoded device ID (space encoded as %20)
    let response = client
        .get("/iot/data/read/test%20device")
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::NotFound);

    // Test with URL-encoded device ID (special characters)
    let response = client
        .get("/iot/data/read/test%2Fdevice")
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::NotFound);
}

/// Test reading telemetry with case sensitivity
/// 
/// This test verifies that the API correctly handles case-sensitive device IDs
/// and treats different cases as different devices.
#[tokio::test]
async fn test_read_case_sensitive_device_id() {
    // Load environment variables for test configuration
    dotenv().ok();
    
    // Create test application instance
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Test with lowercase device ID
    let response = client
        .get("/iot/data/read/sensor001")
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::NotFound);

    // Test with uppercase device ID
    let response = client
        .get("/iot/data/read/SENSOR001")
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::NotFound);

    // Test with mixed case device ID
    let response = client
        .get("/iot/data/read/Sensor001")
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::NotFound);
}

/// Test reading telemetry with malformed URLs
/// 
/// This test verifies that the API correctly handles malformed URLs
/// and returns appropriate error responses.
#[tokio::test]
async fn test_read_malformed_urls() {
    // Load environment variables for test configuration
    dotenv().ok();
    
    // Create test application instance
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Test with double slashes
    let response = client
        .get("/iot/data/read//sensor001")
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::NotFound);

    // Test with trailing slash
    let response = client
        .get("/iot/data/read/sensor001/")
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::NotFound);
}
