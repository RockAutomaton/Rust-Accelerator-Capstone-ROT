use crate::TestApp;
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use rocket::serde::json::Json;

use device_comms::domain::telemetry::Telemetry;
use device_comms::routes::ingest_telemetry::ingest;

#[rocket::async_test]
async fn test_ingest_telemetry() {
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Create a sample telemetry data
    let telemetry_data = Telemetry {
        device_id: "test_device".to_string(),
        telemetry_data: vec![("temperature".to_string(), 22.5)],
        timestamp: Some(chrono::Utc::now()),
    };

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