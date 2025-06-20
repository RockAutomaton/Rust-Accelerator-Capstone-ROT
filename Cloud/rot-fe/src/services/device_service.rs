/// # Device Service
///
/// This module provides a service for interacting with device-related APIs.
/// It handles fetching telemetry data and updating device configurations.
///
/// The service communicates with two backend services:
/// - Device Monitor API - for fetching telemetry data
/// - Device Config API - for updating device configurations

use gloo_net::http::Request;
use crate::domain::telemetry::Telemetry;
use crate::domain::config::DeviceConfig;
use tracing::{info, instrument, Level};

/// Service for interacting with device APIs.
///
/// This struct provides static methods for communicating with
/// backend services to fetch and update device data.
pub struct DeviceService;

impl DeviceService {
    /// Base URL for the device monitor API.
    ///
    /// This is set from the ROT_API_URL environment variable
    /// at build time to avoid hardcoding URLs.
    const BASE_URL: &'static str = env!("ROT_API_URL");
    
    /// Fetches all telemetry data for a specific device.
    ///
    /// This method queries the device monitor API to retrieve all
    /// historical telemetry records for the specified device.
    ///
    /// # Parameters
    /// * `device_id` - ID of the device to fetch telemetry for
    ///
    /// # Returns
    /// * `Ok(Vec<Telemetry>)` - List of telemetry records if successful
    /// * `Err(String)` - Error message if the request fails
    ///
    /// # Instrumentation
    /// This method is instrumented with tracing to track API calls
    #[instrument(skip_all, fields(device_id = %device_id), level = Level::INFO)]
    pub async fn get_telemetry(device_id: &str) -> Result<Vec<Telemetry>, String> {
        info!("Fetching telemetry data for device");
        
        // Ensure BASE_URL is properly formatted (remove trailing slash if present)
        let base_url = Self::BASE_URL.trim_end_matches('/');
        info!(base_url = %base_url, "Using base URL");
        
        // Construct the full API URL
        let url = format!("{}/iot/data/read/{}", base_url, device_id);
        info!(url = %url, "Making request to URL");
        
        // Make the HTTP request to the API
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| {
                info!(error = %e, "Failed to fetch telemetry data");
                format!("Request failed: {}", e)
            })?;
        
        // Handle 404 (device not found) specially
        if response.status() == 404 {
            info!("No telemetry data found for device");
            return Err("404".to_string());
        }
        
        // Parse the JSON response into Vec<Telemetry>
        response
            .json::<Vec<Telemetry>>()
            .await
            .map_err(|e| {
                info!(error = %e, "Failed to parse telemetry data");
                format!("JSON parse failed: {}", e)
            })
    }

    /// Fetches the latest telemetry data for a specific device.
    ///
    /// This method retrieves all telemetry records for the device
    /// and returns only the most recent one based on timestamp.
    ///
    /// # Parameters
    /// * `device_id` - ID of the device to fetch telemetry for
    ///
    /// # Returns
    /// * `Ok(Telemetry)` - Most recent telemetry record if available
    /// * `Err(String)` - Error message if the request fails or no data found
    ///
    /// # Instrumentation
    /// This method is instrumented with tracing to track API calls
    #[instrument(skip_all, fields(device_id = %device_id), level = Level::INFO)]
    pub async fn get_latest_telemetry(device_id: &str) -> Result<Telemetry, String> {
        info!("Fetching latest telemetry data for device");
        
        // Get all telemetry data for the device
        let telemetry_list = Self::get_telemetry(device_id).await?;
        
        // Find the entry with the latest timestamp
        telemetry_list
            .into_iter()
            .max_by_key(|t| t.timestamp)  // Sort by timestamp (descending)
            .ok_or_else(|| {
                info!("No telemetry data found for device");
                "No telemetry data found".to_string()
            })
    }

    /// Updates the configuration for a specific device.
    ///
    /// This method sends a configuration update request to the device
    /// configuration API to change device settings.
    ///
    /// # Parameters
    /// * `device_id` - ID of the device to update
    /// * `config` - New configuration settings for the device
    ///
    /// # Returns
    /// * `Ok(())` - If update was successful
    /// * `Err(String)` - Error message if the update fails
    ///
    /// # Instrumentation
    /// This method is instrumented with tracing to track API calls
    #[instrument(skip_all, fields(device_id = %device_id), level = Level::INFO)]
    pub async fn update_device_config(device_id: &str, config: &DeviceConfig) -> Result<(), String> {
        info!("Updating device configuration");
        
        // Get the base URL for the device configuration API
        let base_url = env!("ROT_DC_URL").trim_end_matches('/');
        let url = format!("{}/device-config/update", base_url);
        info!(url = %url, "Making request to URL");
        
        // Create a POST request with the config as JSON body
        let response = Request::post(&url)
            // Serialize the config to JSON
            .json(config)
            .map_err(|e| {
                info!(error = %e, "Failed to serialize config");
                format!("JSON serialize failed: {}", e)
            })?
            // Send the request
            .send()
            .await
            .map_err(|e| {
                info!(error = %e, "Failed to update device config");
                format!("Request failed: {}", e)
            })?;
        
        // Handle 404 (device not found) specially
        if response.status() == 404 {
            info!("Device not found for config update");
            return Err("404".to_string());
        }
        
        // Check for other error status codes
        let status_code = response.status();
        if status_code < 200 || status_code >= 300 {
            info!(status = %status_code, "Config update failed");
            return Err(format!("Update failed with status: {}", status_code));
        }
        
        // Update was successful
        info!("Device configuration updated successfully");
        Ok(())
    }
}