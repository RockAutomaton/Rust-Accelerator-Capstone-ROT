// src/services/device_service.rs
use gloo_net::http::Request;
use crate::domain::telemetry::Telemetry;
use crate::domain::config::DeviceConfig;
use tracing::{info, instrument, Level};

pub struct DeviceService;

impl DeviceService {
    // Base URL for your device data service
    const BASE_URL: &'static str = env!("ROT_API_URL");
    
    #[instrument(skip_all, fields(device_id = %device_id), level = Level::INFO)]
    pub async fn get_telemetry(device_id: &str) -> Result<Vec<Telemetry>, String> {
        info!("Fetching telemetry data for device");
        
        // Ensure BASE_URL is properly formatted
        let base_url = Self::BASE_URL.trim_end_matches('/');
        info!(base_url = %base_url, "Using base URL");
        
        let url = format!("{}/iot/data/read/{}", base_url, device_id);
        info!(url = %url, "Making request to URL");
        
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| {
                info!(error = %e, "Failed to fetch telemetry data");
                format!("Request failed: {}", e)
            })?;
        
        if response.status() == 404 {
            info!("No telemetry data found for device");
            return Err("404".to_string());
        }
        
        response
            .json::<Vec<Telemetry>>()
            .await
            .map_err(|e| {
                info!(error = %e, "Failed to parse telemetry data");
                format!("JSON parse failed: {}", e)
            })
    }

    #[instrument(skip_all, fields(device_id = %device_id), level = Level::INFO)]
    pub async fn get_latest_telemetry(device_id: &str) -> Result<Telemetry, String> {
        info!("Fetching latest telemetry data for device");
        let telemetry_list = Self::get_telemetry(device_id).await?;
        // use the last one by timestamp
        telemetry_list
            .into_iter()
            .max_by_key(|t| t.timestamp)
            .ok_or_else(|| {
                info!("No telemetry data found for device");
                "No telemetry data found".to_string()
            })
    }

    #[instrument(skip_all, fields(device_id = %device_id), level = Level::INFO)]
    pub async fn update_device_config(device_id: &str, config: &DeviceConfig) -> Result<(), String> {
        info!("Updating device configuration");
        
        let base_url = env!("ROT_DC_URL").trim_end_matches('/');
        let url = format!("{}/device-config/update", base_url);
        info!(url = %url, "Making request to URL");
        
        let response = Request::post(&url)
            .json(config)
            .map_err(|e| {
                info!(error = %e, "Failed to serialize config");
                format!("JSON serialize failed: {}", e)
            })?
            .send()
            .await
            .map_err(|e| {
                info!(error = %e, "Failed to update device config");
                format!("Request failed: {}", e)
            })?;
        
        if response.status() == 404 {
            info!("Device not found for config update");
            return Err("404".to_string());
        }
        
        let status_code = response.status();
        if status_code < 200 || status_code >= 300 {
            info!(status = %status_code, "Config update failed");
            return Err(format!("Update failed with status: {}", status_code));
        }
        
        info!("Device configuration updated successfully");
        Ok(())
    }
}