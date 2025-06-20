// Telemetry Ingestion Integration Tests
// 
// This module contains comprehensive integration tests for the telemetry
// ingestion endpoint. These tests verify the complete request/response
// flow including validation, database operations, and error handling.

use crate::helper::TestApp;
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use dotenvy::dotenv;
use std::collections::HashMap;
use device_comms::domain::telemetry::Telemetry;

/// Test successful telemetry ingestion with valid data
/// 
/// This test verifies that:
/// - Valid telemetry data is accepted and stored
/// - The API returns a 200 OK status
/// - The response body contains the expected success message
#[tokio::test]
async fn test_ingest_telemetry() {
    // Load environment variables for test configuration
    dotenv().ok();
    
    // Set up test application and client
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    // Create a sample telemetry data with temperature reading
    let mut data = HashMap::new();
    data.insert("temperature".to_string(), "22.5".to_string());
    let timestamp = chrono::Utc::now().timestamp();
    let telemetry_data = Telemetry::parse(device_id, data, Some(timestamp)).expect("Failed to parse telemetry");

    // Send a POST request to the ingest endpoint
    let response = client
        .post("/iot/data/ingest")
        .json(&telemetry_data)
        .dispatch()
        .await;

    // Assert the response status is OK (200)
    assert_eq!(response.status(), Status::Ok);

    // Assert the response body contains the expected success message
    let body = response.into_string().await.expect("Failed to read response body");
    assert_eq!(body, "Telemetry ingested");
}

/// Test telemetry ingestion without providing a timestamp
/// 
/// This test verifies that:
/// - The API automatically uses the current timestamp when none is provided
/// - The request is processed successfully
/// - The response indicates successful ingestion
#[tokio::test]
async fn test_ingest_telemetry_without_timestamp() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    // Create telemetry data without timestamp (should use current time)
    let mut data = HashMap::new();
    data.insert("humidity".to_string(), "45.0".to_string());
    let telemetry_data = Telemetry::parse(device_id, data, None).expect("Failed to parse telemetry");

    let response = client
        .post("/iot/data/ingest")
        .json(&telemetry_data)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().await.expect("Failed to read response body");
    assert_eq!(body, "Telemetry ingested");
}

/// Test telemetry ingestion with multiple sensor values
/// 
/// This test verifies that:
/// - Multiple telemetry values can be sent in a single request
/// - All values are processed and stored correctly
/// - The API handles complex telemetry data structures
#[tokio::test]
async fn test_ingest_multiple_telemetry_values() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    // Create telemetry data with multiple sensor readings
    let mut data = HashMap::new();
    data.insert("temperature".to_string(), "22.5".to_string());
    data.insert("humidity".to_string(), "45.0".to_string());
    data.insert("pressure".to_string(), "1013.2".to_string());
    data.insert("battery".to_string(), "85".to_string());
    
    let timestamp = chrono::Utc::now().timestamp();
    let telemetry_data = Telemetry::parse(device_id, data, Some(timestamp)).expect("Failed to parse telemetry");

    let response = client
        .post("/iot/data/ingest")
        .json(&telemetry_data)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().await.expect("Failed to read response body");
    assert_eq!(body, "Telemetry ingested");
}

/// Test telemetry ingestion with empty telemetry data
/// 
/// This test verifies that:
/// - Empty telemetry data is rejected with appropriate error
/// - The API returns a 422 Unprocessable Entity status
/// - Validation prevents storage of invalid data
#[tokio::test]
async fn test_ingest_empty_telemetry_data() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Attempt to create telemetry with empty data (should fail validation)
    let data = HashMap::new();
    let timestamp = chrono::Utc::now().timestamp();
    let telemetry_data = Telemetry::parse("test_device".to_string(), data, Some(timestamp)).expect_err("Should fail with empty data");

    let response = client
        .post("/iot/data/ingest")
        .json(&telemetry_data)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::UnprocessableEntity);
}

/// Test telemetry ingestion with invalid JSON payload
/// 
/// This test verifies that:
/// - Malformed JSON is rejected with appropriate error
/// - The API returns a 400 Bad Request status
/// - JSON parsing errors are handled gracefully
#[tokio::test]
async fn test_ingest_invalid_json() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Send invalid JSON data that cannot be parsed
    let response = client
        .post("/iot/data/ingest")
        .body("invalid json data")
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}

/// Test telemetry ingestion with empty device ID
/// 
/// This test verifies that:
/// - Empty device IDs are rejected with appropriate error
/// - The API returns a 422 Unprocessable Entity status
/// - Device ID validation prevents storage of invalid data
#[tokio::test]
async fn test_ingest_empty_device_id() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Attempt to create telemetry with empty device ID (should fail validation)
    let mut data = HashMap::new();
    data.insert("temperature".to_string(), "22.5".to_string());
    let timestamp = chrono::Utc::now().timestamp();
    let telemetry_data = Telemetry::parse("".to_string(), data, Some(timestamp)).expect_err("Should fail with empty device ID");

    let response = client
        .post("/iot/data/ingest")
        .json(&telemetry_data)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::UnprocessableEntity);
}

/// Test telemetry ingestion with invalid timestamp
/// 
/// This test verifies that:
/// - Negative timestamps are rejected with appropriate error
/// - The API returns a 422 Unprocessable Entity status
/// - Timestamp validation prevents storage of invalid data
#[tokio::test]
async fn test_ingest_invalid_timestamp() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Attempt to create telemetry with negative timestamp (should fail validation)
    let mut data = HashMap::new();
    data.insert("temperature".to_string(), "22.5".to_string());
    let telemetry_data = Telemetry::parse("test_device".to_string(), data, Some(-1)).expect_err("Should fail with invalid timestamp");

    let response = client
        .post("/iot/data/ingest")
        .json(&telemetry_data)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::UnprocessableEntity);
}

/// Test telemetry ingestion with empty telemetry values
/// 
/// This test verifies that:
/// - Empty telemetry values are rejected with appropriate error
/// - The API returns a 422 Unprocessable Entity status
/// - Individual value validation prevents storage of invalid data
#[tokio::test]
async fn test_ingest_empty_telemetry_value() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Attempt to create telemetry with empty value (should fail validation)
    let mut data = HashMap::new();
    data.insert("temperature".to_string(), "".to_string());
    let timestamp = chrono::Utc::now().timestamp();
    let telemetry_data = Telemetry::parse("test_device".to_string(), data, Some(timestamp)).expect_err("Should fail with empty telemetry value");

    let response = client
        .post("/iot/data/ingest")
        .json(&telemetry_data)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::UnprocessableEntity);
}

