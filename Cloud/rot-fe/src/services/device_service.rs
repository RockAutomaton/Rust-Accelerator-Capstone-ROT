// src/services/device_service.rs
use gloo_net::http::Request;
use crate::domain::telemetry::Telemetry;

pub struct DeviceService;

impl DeviceService {
    // Base URL for your device data service
    const BASE_URL: &'static str = "http://127.0.0.1:8001";

    pub async fn get_telemetry(device_id: &str) -> Result<Vec<Telemetry>, String> {
        let url = format!("{}/iot/data/read/{}", Self::BASE_URL, device_id);
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;
        
        if response.status() == 404 {
            return Err("404".to_string());
        }
        
        response
            .json::<Vec<Telemetry>>()
            .await
            .map_err(|e| format!("JSON parse failed: {}", e))
    }

    pub async fn get_latest_telemetry(device_id: &str) -> Result<Telemetry, String> {

        let telemetry_list = Self::get_telemetry(device_id).await?;
        // use the last one by timestamp
        telemetry_list
            .into_iter()
            .max_by_key(|t| t.timestamp)
            .ok_or_else(|| "No telemetry data found".to_string())
    }
}