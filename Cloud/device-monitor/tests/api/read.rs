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
