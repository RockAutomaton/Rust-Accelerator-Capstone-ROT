// Update Configuration API Integration Tests
// 
// This module contains integration tests for the POST /device-config/update
// endpoint of the device configuration service.

use crate::helper::TestApp;
use rocket::http::{Status, ContentType};
use rocket::local::asynchronous::Client;
use dotenvy::dotenv;

/// Test updating configuration with valid data
/// 
/// This test verifies that the API correctly accepts and processes
/// valid configuration data for a device.
#[tokio::test]
async fn test_update_config_valid_data() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();
    let config_data = app.create_test_config(&device_id);

    // Try to update configuration with valid data
    let response = client
        .post("/device-config/update")
        .header(ContentType::JSON)
        .body(config_data.to_string())
        .dispatch()
        .await;

    // Should return 200 OK for successful configuration update
    assert_eq!(response.status(), Status::Ok);
    
    // Verify the response body contains the success message
    let body = response.into_string().await.unwrap();
    assert_eq!(body, "Config ingested");
}

/// Test updating configuration with invalid JSON
/// 
/// This test verifies that the API correctly rejects malformed JSON
/// and returns an appropriate error status.
#[tokio::test]
async fn test_update_config_invalid_json() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Try to update configuration with invalid JSON
    let response = client
        .post("/device-config/update")
        .header(ContentType::JSON)
        .body("{ invalid json }")
        .dispatch()
        .await;

    // Should return 400 Bad Request for invalid JSON
    assert_eq!(response.status(), Status::BadRequest);
}

/// Test updating configuration with missing device_id
/// 
/// This test verifies that the API correctly validates required fields
/// and returns an error when device_id is missing.
#[tokio::test]
async fn test_update_config_missing_device_id() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Try to update configuration without device_id
    let config_data = serde_json::json!({
        "config": {
            "sampling_rate": "1000",
            "threshold": "25.5"
        }
    });

    let response = client
        .post("/device-config/update")
        .header(ContentType::JSON)
        .body(config_data.to_string())
        .dispatch()
        .await;

    // Should return 422 Unprocessable Entity for missing required field
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

/// Test updating configuration with empty device_id
/// 
/// This test verifies that the API correctly validates that device_id
/// is not empty and returns an error when it is.
#[tokio::test]
async fn test_update_config_empty_device_id() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Try to update configuration with empty device_id
    let config_data = serde_json::json!({
        "device_id": "",
        "config": {
            "sampling_rate": "1000",
            "threshold": "25.5"
        }
    });

    let response = client
        .post("/device-config/update")
        .header(ContentType::JSON)
        .body(config_data.to_string())
        .dispatch()
        .await;

    // Should return 400 Bad Request for empty device_id
    assert_eq!(response.status(), Status::BadRequest);
}

/// Test updating configuration with missing config data
/// 
/// This test verifies that the API correctly validates that config
/// data is provided and returns an error when it is missing.
#[tokio::test]
async fn test_update_config_missing_config() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Try to update configuration without config data
    let config_data = serde_json::json!({
        "device_id": "test-device"
    });

    let response = client
        .post("/device-config/update")
        .header(ContentType::JSON)
        .body(config_data.to_string())
        .dispatch()
        .await;

    // Should return 422 Unprocessable Entity for missing config data
    assert_eq!(response.status(), Status::UnprocessableEntity);
}

/// Test updating configuration with empty config data
/// 
/// This test verifies that the API correctly validates that config
/// data is not empty and returns an error when it is.
#[tokio::test]
async fn test_update_config_empty_config() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;

    // Try to update configuration with empty config data
    let config_data = serde_json::json!({
        "device_id": "test-device",
        "config": {}
    });

    let response = client
        .post("/device-config/update")
        .header(ContentType::JSON)
        .body(config_data.to_string())
        .dispatch()
        .await;

    // Should return 400 Bad Request for empty config data
    assert_eq!(response.status(), Status::BadRequest);
}

/// Test updating configuration with different HTTP methods
/// 
/// This test verifies that the API correctly rejects unsupported HTTP methods
/// and only accepts POST requests.
#[tokio::test]
async fn test_update_config_unsupported_methods() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let config_data = app.create_test_config("test-device");

    // Try GET method (should not be supported)
    let response = client
        .get("/device-config/update")
        .dispatch()
        .await;

    // Should return 404 for unsupported methods
    assert_eq!(response.status(), Status::NotFound);

    // Try PUT method (should not be supported)
    let response = client
        .put("/device-config/update")
        .header(ContentType::JSON)
        .body(config_data.to_string())
        .dispatch()
        .await;

    // Should return 404 for unsupported methods
    assert_eq!(response.status(), Status::NotFound);

    // Try DELETE method (should not be supported)
    let response = client
        .delete("/device-config/update")
        .dispatch()
        .await;

    // Should return 404 for unsupported methods
    assert_eq!(response.status(), Status::NotFound);
}

/// Test updating configuration without Content-Type header
/// 
/// This test verifies that the API correctly handles requests without
/// the proper Content-Type header.
#[tokio::test]
async fn test_update_config_no_content_type() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let config_data = app.create_test_config("test-device");

    // Try to update configuration without Content-Type header
    let response = client
        .post("/device-config/update")
        .body(config_data.to_string())
        .dispatch()
        .await;

    // Rocket accepts the request and processes it successfully
    assert_eq!(response.status(), Status::Ok);
    
    // Verify the response body contains the success message
    let body = response.into_string().await.unwrap();
    assert_eq!(body, "Config ingested");
}

/// Test updating configuration with complex config data
/// 
/// This test verifies that the API correctly handles complex configuration
/// data with multiple parameters.
#[tokio::test]
async fn test_update_config_complex_data() {
    dotenv().ok();
    
    let app = TestApp::new().await.expect("Failed to create test app");
    let client: &Client = &app.client;
    let device_id = app.generate_test_device_id();

    // Try to update configuration with complex data (flat key-value pairs)
    let config_data = serde_json::json!({
        "device_id": device_id,
        "config": {
            "sampling_rate": "1000",
            "threshold": "25.5",
            "wifi_ssid": "TestNetwork",
            "wifi_password": "testpass123",
            "mqtt_broker": "mqtt.example.com",
            "mqtt_port": "1883",
            "mqtt_topic": "device/telemetry",
            "temperature_enabled": "true",
            "temperature_calibration": "0.1",
            "humidity_enabled": "true",
            "humidity_calibration": "0.0"
        }
    });

    let response = client
        .post("/device-config/update")
        .header(ContentType::JSON)
        .body(config_data.to_string())
        .dispatch()
        .await;

    // Should return 200 OK for successful configuration update
    assert_eq!(response.status(), Status::Ok);
    
    // Verify the response body contains the success message
    let body = response.into_string().await.unwrap();
    assert_eq!(body, "Config ingested");
} 