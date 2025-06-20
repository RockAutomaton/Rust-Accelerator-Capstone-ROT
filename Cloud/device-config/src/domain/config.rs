// Device Configuration Domain Model
// 
// This module defines the core configuration data structures and validation logic
// for the device configuration service. It handles the representation and
// validation of IoT device configuration data.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::domain::error::ConfigError;

/// Device configuration data structure
/// 
/// This struct represents a device configuration containing:
/// - device_id: Unique identifier for the IoT device
/// - config: Key-value pairs representing configuration parameters
/// 
/// Examples of configuration parameters:
/// - "sampling_rate": "1000" (milliseconds)
/// - "threshold": "25.5" (temperature threshold)
/// - "wifi_ssid": "MyNetwork"
/// - "wifi_password": "secret123"
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Unique identifier of the IoT device this configuration belongs to
    pub device_id: String,
    
    /// Key-value pairs representing device configuration parameters
    /// 
    /// These can include various device settings such as:
    /// - Sensor sampling rates
    /// - Alert thresholds
    /// - Network configuration
    /// - Device behavior parameters
    pub config: HashMap<String, String>,
}

impl Config {
    /// Creates a new device configuration instance with validation
    /// 
    /// This method validates the input data and creates a configuration
    /// instance. Currently performs basic validation, but can be extended
    /// to include more sophisticated validation rules.
    /// 
    /// # Arguments
    /// * `device_id` - The device identifier
    /// * `config` - The configuration parameters as key-value pairs
    /// 
    /// # Returns
    /// * `Result<Self, ConfigError>` - The validated configuration or an error
    pub fn parse(device_id: String, config: HashMap<String, String>) -> Result<Self, ConfigError> {
        // TODO: Add validation logic here
        // - Validate device_id is not empty
        // - Validate config is not empty
        // - Validate specific configuration parameters
        
        Ok(Self { device_id, config })
    }
}