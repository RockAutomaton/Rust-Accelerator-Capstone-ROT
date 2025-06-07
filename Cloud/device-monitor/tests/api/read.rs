use crate::helper::TestApp;
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use dotenvy::dotenv;
use device_monitor::domain::telemetry::Telemetry;
use std::collections::HashMap;

#[tokio::test]
async fn test_read_nonexistent_device() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    // Try to read telemetry for a device that doesn't exist
    let response = client
        .get(format!("/iot/data/read/{}", device_id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NotFound);
}

#[tokio::test]
async fn test_read_empty_device_id() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Try to read telemetry with an empty device ID
    let response = client
        .get("/iot/data/read/")
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NotFound);
}

#[tokio::test]
async fn test_read_invalid_device_id() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Try to read telemetry with an invalid device ID (containing special characters)
    let response = client
        .get("/iot/data/read/invalid@device#id")
        .dispatch()
        .await;

    // Rocket returns 400 for invalid URL characters
    assert_eq!(response.status(), Status::BadRequest);
}

#[tokio::test]
async fn test_read_with_query_parameters() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    // Try to read telemetry with query parameters (should be ignored)
    let response = client
        .get(format!("/iot/data/read/{}?limit=10&offset=0", device_id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::NotFound);
}
