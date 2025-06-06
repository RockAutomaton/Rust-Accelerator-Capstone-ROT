use crate::helper::TestApp;
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use dotenvy::dotenv;
use std::collections::HashMap;
use device_comms::domain::telemetry::Telemetry;

#[tokio::test]
async fn test_read_telemetry() {
    // Load environment variables first
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    // First, insert some test data
    let mut data = HashMap::new();
    data.insert("temperature".to_string(), "22.5".to_string());
    let timestamp = chrono::Utc::now().timestamp();
    let telemetry_data = Telemetry::parse(device_id.clone(), data, Some(timestamp)).expect("Failed to parse telemetry");

    // Insert the test data
    let response = client
        .post("/iot/data/ingest")
        .json(&telemetry_data)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    // Now read the telemetry data
    let response = client
        .get(format!("/iot/data/read/{}", device_id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    // Parse the response body
    let body = response.into_string().await.expect("Failed to read response body");
    let telemetry: Vec<Telemetry> = serde_json::from_str(&body).expect("Failed to parse response body");

    // Verify the response
    assert!(!telemetry.is_empty());
    assert_eq!(telemetry[0].device_id, device_id);
    assert_eq!(telemetry[0].telemetry_data.get("temperature").unwrap(), "22.5");
}

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
async fn test_read_multiple_telemetry_entries() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    // Insert multiple telemetry entries for the same device
    for i in 0..3 {
        let mut data = HashMap::new();
        data.insert("temperature".to_string(), format!("{}.5", 20 + i));
        let timestamp = chrono::Utc::now().timestamp() + i;
        let telemetry_data = Telemetry::parse(device_id.clone(), data, Some(timestamp)).expect("Failed to parse telemetry");

        let response = client
            .post("/iot/data/ingest")
            .json(&telemetry_data)
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Ok);
    }

    // Read all telemetry entries for the device
    let response = client
        .get(format!("/iot/data/read/{}", device_id))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    // Parse the response body
    let body = response.into_string().await.expect("Failed to read response body");
    let telemetry: Vec<Telemetry> = serde_json::from_str(&body).expect("Failed to parse response body");

    // Verify we got all entries
    assert_eq!(telemetry.len(), 3);
    
    // Verify the entries are ordered by timestamp
    let mut timestamps: Vec<i64> = telemetry.iter()
        .map(|t| t.timestamp.unwrap())
        .collect();
    timestamps.sort();
    assert_eq!(timestamps, timestamps);
}
