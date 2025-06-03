use crate::helper::TestApp;
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use dotenvy::dotenv;
use std::collections::HashMap;
use device_comms::domain::telemetry::Telemetry;

#[tokio::test]
async fn test_ingest_telemetry() {
    // Load environment variables first
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    // Create a sample telemetry data
    let mut data = HashMap::new();
    data.insert("temperature".to_string(), "22.5".to_string());
    let timestamp = chrono::Utc::now().timestamp();
    let telemetry_data = Telemetry::parse(device_id, data, timestamp).expect("Failed to parse telemetry");

    // Send a POST request to the ingest endpoint
    let response = client
        .post("/iot/data/ingest")
        .json(&telemetry_data)
        .dispatch()
        .await;

    // Assert the response status is OK
    assert_eq!(response.status(), Status::Ok);

    // Assert the response body is as expected
    let body = response.into_string().await.expect("Failed to read response body");
    assert_eq!(body, "Telemetry ingested");
}

#[tokio::test]
async fn test_ingest_telemetry_without_timestamp() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    let mut data = HashMap::new();
    data.insert("humidity".to_string(), "45.0".to_string());
    let telemetry_data = Telemetry::parse(device_id, data, 0).expect("Failed to parse telemetry");

    let response = client
        .post("/iot/data/ingest")
        .json(&telemetry_data)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().await.expect("Failed to read response body");
    assert_eq!(body, "Telemetry ingested");
}

#[tokio::test]
async fn test_ingest_multiple_telemetry_values() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    let mut data = HashMap::new();
    data.insert("temperature".to_string(), "22.5".to_string());
    data.insert("humidity".to_string(), "45.0".to_string());
    data.insert("pressure".to_string(), "1013.2".to_string());
    data.insert("battery".to_string(), "85".to_string());
    
    let timestamp = chrono::Utc::now().timestamp();
    let telemetry_data = Telemetry::parse(device_id, data, timestamp).expect("Failed to parse telemetry");

    let response = client
        .post("/iot/data/ingest")
        .json(&telemetry_data)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().await.expect("Failed to read response body");
    assert_eq!(body, "Telemetry ingested");
}

#[tokio::test]
async fn test_ingest_empty_telemetry_data() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    let data = HashMap::new();
    let timestamp = chrono::Utc::now().timestamp();
    let telemetry_data = Telemetry::parse("test_device".to_string(), data, timestamp).expect_err("Should fail with empty data");

    let response = client
        .post("/iot/data/ingest")
        .json(&telemetry_data)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[tokio::test]
async fn test_ingest_invalid_json() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Send invalid JSON data
    let response = client
        .post("/iot/data/ingest")
        .body("invalid json data")
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_ingest_empty_device_id() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    let mut data = HashMap::new();
    data.insert("temperature".to_string(), "22.5".to_string());
    let timestamp = chrono::Utc::now().timestamp();
    let telemetry_data = Telemetry::parse("".to_string(), data, timestamp).expect_err("Should fail with empty device ID");

    let response = client
        .post("/iot/data/ingest")
        .json(&telemetry_data)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[tokio::test]
async fn test_ingest_invalid_timestamp() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    let mut data = HashMap::new();
    data.insert("temperature".to_string(), "22.5".to_string());
    let telemetry_data = Telemetry::parse("test_device".to_string(), data, -1).expect_err("Should fail with invalid timestamp");

    let response = client
        .post("/iot/data/ingest")
        .json(&telemetry_data)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::UnprocessableEntity);
}

#[tokio::test]
async fn test_ingest_empty_telemetry_value() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    let mut data = HashMap::new();
    data.insert("temperature".to_string(), "".to_string());
    let timestamp = chrono::Utc::now().timestamp();
    let telemetry_data = Telemetry::parse("test_device".to_string(), data, timestamp).expect_err("Should fail with empty telemetry value");

    let response = client
        .post("/iot/data/ingest")
        .json(&telemetry_data)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::UnprocessableEntity);
}

