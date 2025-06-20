// Telemetry Domain Model
// 
// This module defines the core telemetry data structures and validation logic
// for the device monitoring service. It handles the representation and
// validation of IoT device telemetry data for monitoring purposes.

use serde::{Deserialize, Serialize, Deserializer};
use std::{collections::HashMap};
use chrono::{DateTime, Utc};

/// Custom deserializer for timestamp fields that can handle multiple formats
/// 
/// This function can deserialize timestamps from:
/// - Unix timestamp numbers (i64)
/// - RFC3339 datetime strings
/// - Null values (returns None)
/// 
/// # Arguments
/// * `deserializer` - The serde deserializer instance
/// 
/// # Returns
/// * `Result<Option<i64>, D::Error>` - The parsed timestamp or None if null
fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match s {
        // Handle Unix timestamp numbers
        serde_json::Value::Number(num) => num.as_i64().ok_or_else(|| serde::de::Error::custom("Invalid number")).map(Some),
        // Handle RFC3339 datetime strings
        serde_json::Value::String(ref s) => {
            let dt = DateTime::parse_from_rfc3339(s)
                .map_err(|_| serde::de::Error::custom("Invalid datetime string"))?;
            Ok(Some(dt.timestamp()))
        }
        // Handle null values
        serde_json::Value::Null => Ok(None),
        // Reject other types
        _ => Err(serde::de::Error::custom("Invalid type for timestamp")),
    }
}

/// Core telemetry data structure representing IoT device sensor readings
/// 
/// This struct represents a single telemetry reading from an IoT device,
/// including the device identifier, sensor data, and timestamp. It also
/// includes Cosmos DB metadata fields for storage operations.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Telemetry {
    /// Unique identifier for this telemetry record
    /// 
    /// Generated as "{device_id}-{timestamp}" when not provided
    #[serde(
        rename = "id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub id: Option<String>,
    
    /// Unique identifier of the IoT device that generated this telemetry
    pub device_id: String,
    
    /// Key-value pairs representing sensor readings and device state
    /// 
    /// Examples: {"temperature": "23.5", "humidity": "45.2", "status": "online"}
    pub telemetry_data: HashMap<String, String>,
    
    /// Unix timestamp when this telemetry was generated
    /// 
    /// Uses custom deserializer to handle multiple timestamp formats
    #[serde(deserialize_with = "deserialize_timestamp", default)]
    pub timestamp: Option<i64>,
    
    // Cosmos DB metadata fields (not part of business logic)
    #[serde(rename = "_rid", skip_serializing_if = "Option::is_none")]
    rid: Option<String>,
    #[serde(rename = "_self", skip_serializing_if = "Option::is_none")]
    self_link: Option<String>,
    #[serde(rename = "_etag", skip_serializing_if = "Option::is_none")]
    etag: Option<String>,
    #[serde(rename = "_attachments", skip_serializing_if = "Option::is_none")]
    attachments: Option<String>,
}

/// Error types that can occur during telemetry validation
#[derive(Debug, Serialize)]
pub enum TelemetryError {
    /// Device ID is empty or invalid
    InvalidDeviceId,
    /// Timestamp is negative or invalid
    InvalidTimestamp,
    /// No telemetry data provided
    EmptyTelemetryData,
    /// Individual telemetry value is invalid
    InvalidTelemetryValue(String),
}

impl std::fmt::Display for TelemetryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TelemetryError::InvalidDeviceId => write!(f, "Device ID cannot be empty"),
            TelemetryError::InvalidTimestamp => write!(f, "Timestamp must be a valid Unix timestamp"),
            TelemetryError::EmptyTelemetryData => write!(f, "Telemetry data cannot be empty"),
            TelemetryError::InvalidTelemetryValue(msg) => write!(f, "Invalid telemetry value: {}", msg),
        }
    }
}

impl std::error::Error for TelemetryError {}

impl Telemetry {
    /// Creates a new telemetry instance with the provided data
    /// 
    /// This constructor assumes all data is valid and doesn't perform validation.
    /// Use `parse()` for validated construction.
    /// 
    /// # Arguments
    /// * `device_id` - The device identifier
    /// * `telemetry_data` - The sensor readings as key-value pairs
    /// * `timestamp` - Unix timestamp of the reading
    /// 
    /// # Returns
    /// * `Self` - A new Telemetry instance
    pub fn new(
        device_id: String,
        telemetry_data: HashMap<String, String>,
        timestamp: i64,
    ) -> Self {
        Telemetry {
            id: Some(format!("{}-{}", device_id, timestamp)),
            device_id,
            telemetry_data,
            rid: None,
            self_link: None,
            etag: None,
            attachments: None,
            timestamp: Some(timestamp),
        }
    }

    /// Creates a new telemetry instance with validation
    /// 
    /// This method validates all input data and returns an error if any
    /// validation fails. If no timestamp is provided, the current time is used.
    /// 
    /// # Arguments
    /// * `device_id` - The device identifier (must not be empty)
    /// * `telemetry_data` - The sensor readings (must not be empty)
    /// * `timestamp` - Optional Unix timestamp (uses current time if None)
    /// 
    /// # Returns
    /// * `Result<Self, TelemetryError>` - The validated telemetry or an error
    pub fn parse(device_id: String, telemetry_data: HashMap<String, String>, timestamp: Option<i64>) -> Result<Self, TelemetryError> {
        // Validate device_id is not empty
        if device_id.trim().is_empty() {
            return Err(TelemetryError::InvalidDeviceId);
        }

        // Use current timestamp if none provided
        let timestamp = timestamp.unwrap_or_else(|| Utc::now().timestamp());

        // Validate timestamp is not negative
        if timestamp < 0 {
            return Err(TelemetryError::InvalidTimestamp);
        }

        // Validate telemetry data is not empty
        if telemetry_data.is_empty() {
            return Err(TelemetryError::EmptyTelemetryData);
        }

        // Validate all telemetry values are not empty
        for (key, value) in &telemetry_data {
            if value.trim().is_empty() {
                return Err(TelemetryError::InvalidTelemetryValue(
                    format!("Empty value for key: {}", key)
                ));
            }
        }

        // Create and return the validated telemetry instance
        Ok(Telemetry {
            id: Some(format!("{}-{}", device_id, timestamp)),
            device_id,
            telemetry_data,
            rid: None,
            self_link: None,
            etag: None,
            attachments: None,
            timestamp: Some(timestamp),
        })
    }
}
