// Device Configuration Domain Model
// 
// This module defines the core configuration data structures and validation logic
// for the device configuration service. It handles the representation and
// validation of IoT device configuration data.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Core configuration data structure representing IoT device settings
/// 
/// This struct represents a device configuration, including the device identifier
/// and a collection of configuration parameters as key-value pairs.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Unique identifier of the IoT device
    pub device_id: String,
    /// Key-value pairs representing device configuration parameters
    /// 
    /// Examples: {"sampling_rate": "1000", "threshold": "25.5", "wifi_ssid": "MyNetwork"}
    pub config: HashMap<String, String>,
}

/// Error types that can occur during configuration validation
#[derive(Debug, Serialize)]
pub enum ConfigError {
    /// Device ID is empty or invalid
    InvalidDeviceId,
    /// Configuration data is empty or invalid
    InvalidConfig,
    /// Database operation error
    DatabaseError(String),
    /// Device configuration not found in database
    DeviceNotFound(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::InvalidDeviceId => write!(f, "Device ID cannot be empty"),
            ConfigError::InvalidConfig => write!(f, "Configuration data cannot be empty"),
            ConfigError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ConfigError::DeviceNotFound(msg) => write!(f, "Device configuration not found: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

impl Config {
    /// Creates a new configuration instance with the provided data
    /// 
    /// This constructor assumes all data is valid and doesn't perform validation.
    /// Use `parse()` for validated construction.
    /// 
    /// # Arguments
    /// * `device_id` - The device identifier
    /// * `config` - The configuration parameters as key-value pairs
    /// 
    /// # Returns
    /// * `Self` - A new Config instance
    pub fn new(device_id: String, config: HashMap<String, String>) -> Self {
        Config {
            device_id,
            config,
        }
    }

    /// Creates a new configuration instance with validation
    /// 
    /// This method validates all input data and returns an error if any
    /// validation fails.
    /// 
    /// # Arguments
    /// * `device_id` - The device identifier (must not be empty)
    /// * `config` - The configuration parameters (must not be empty)
    /// 
    /// # Returns
    /// * `Result<Self, ConfigError>` - The validated configuration or an error
    pub fn parse(device_id: String, config: HashMap<String, String>) -> Result<Self, ConfigError> {
        // Validate device_id is not empty
        if device_id.trim().is_empty() {
            return Err(ConfigError::InvalidDeviceId);
        }

        // Validate configuration data is not empty
        if config.is_empty() {
            return Err(ConfigError::InvalidConfig);
        }

        // Validate all configuration values are not empty
        for (key, value) in &config {
            if value.trim().is_empty() {
                return Err(ConfigError::InvalidConfig);
            }
        }

        // Create and return the validated configuration instance
        Ok(Config {
            device_id,
            config,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_new() {
        let mut config_data = HashMap::new();
        config_data.insert("sampling_rate".to_string(), "1000".to_string());
        config_data.insert("threshold".to_string(), "25.5".to_string());

        let config = Config::new("test-device".to_string(), config_data.clone());

        assert_eq!(config.device_id, "test-device");
        assert_eq!(config.config, config_data);
    }

    #[test]
    fn test_config_parse_valid() {
        let mut config_data = HashMap::new();
        config_data.insert("sampling_rate".to_string(), "1000".to_string());
        config_data.insert("threshold".to_string(), "25.5".to_string());

        let result = Config::parse("test-device".to_string(), config_data.clone());

        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.device_id, "test-device");
        assert_eq!(config.config, config_data);
    }

    #[test]
    fn test_config_parse_empty_device_id() {
        let mut config_data = HashMap::new();
        config_data.insert("sampling_rate".to_string(), "1000".to_string());

        let result = Config::parse("".to_string(), config_data);

        assert!(result.is_err());
        match result.unwrap_err() {
            ConfigError::InvalidDeviceId => {},
            _ => panic!("Expected InvalidDeviceId error"),
        }
    }

    #[test]
    fn test_config_parse_whitespace_device_id() {
        let mut config_data = HashMap::new();
        config_data.insert("sampling_rate".to_string(), "1000".to_string());

        let result = Config::parse("   ".to_string(), config_data);

        assert!(result.is_err());
        match result.unwrap_err() {
            ConfigError::InvalidDeviceId => {},
            _ => panic!("Expected InvalidDeviceId error"),
        }
    }

    #[test]
    fn test_config_parse_empty_config() {
        let config_data = HashMap::new();

        let result = Config::parse("test-device".to_string(), config_data);

        assert!(result.is_err());
        match result.unwrap_err() {
            ConfigError::InvalidConfig => {},
            _ => panic!("Expected InvalidConfig error"),
        }
    }

    #[test]
    fn test_config_parse_empty_config_value() {
        let mut config_data = HashMap::new();
        config_data.insert("sampling_rate".to_string(), "".to_string());

        let result = Config::parse("test-device".to_string(), config_data);

        assert!(result.is_err());
        match result.unwrap_err() {
            ConfigError::InvalidConfig => {},
            _ => panic!("Expected InvalidConfig error"),
        }
    }

    #[test]
    fn test_config_parse_whitespace_config_value() {
        let mut config_data = HashMap::new();
        config_data.insert("sampling_rate".to_string(), "   ".to_string());

        let result = Config::parse("test-device".to_string(), config_data);

        assert!(result.is_err());
        match result.unwrap_err() {
            ConfigError::InvalidConfig => {},
            _ => panic!("Expected InvalidConfig error"),
        }
    }

    #[test]
    fn test_config_parse_complex_config() {
        let mut config_data = HashMap::new();
        config_data.insert("sampling_rate".to_string(), "1000".to_string());
        config_data.insert("threshold".to_string(), "25.5".to_string());
        config_data.insert("wifi_ssid".to_string(), "MyNetwork".to_string());
        config_data.insert("wifi_password".to_string(), "secret123".to_string());
        config_data.insert("mqtt_broker".to_string(), "mqtt.example.com".to_string());

        let result = Config::parse("sensor-001".to_string(), config_data.clone());

        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.device_id, "sensor-001");
        assert_eq!(config.config, config_data);
    }

    #[test]
    fn test_config_error_display() {
        let error = ConfigError::InvalidDeviceId;
        assert_eq!(error.to_string(), "Device ID cannot be empty");

        let error = ConfigError::InvalidConfig;
        assert_eq!(error.to_string(), "Configuration data cannot be empty");

        let error = ConfigError::DatabaseError("Connection failed".to_string());
        assert_eq!(error.to_string(), "Database error: Connection failed");

        let error = ConfigError::DeviceNotFound("Device not found".to_string());
        assert_eq!(error.to_string(), "Device configuration not found: Device not found");
    }
}