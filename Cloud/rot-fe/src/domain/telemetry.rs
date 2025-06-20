/// # Telemetry Domain Models
///
/// This module defines the data structures and business logic for device telemetry data.
/// It includes models for telemetry readings, validation logic, and error handling.

use serde::{Deserialize, Serialize, Deserializer};
use std::{collections::HashMap};
use chrono::{DateTime, Utc};

/// Custom deserializer for timestamp field that handles multiple formats.
///
/// This function allows the timestamp to be deserialized from:
/// - Numeric Unix timestamps
/// - RFC3339 formatted date strings
/// - Null values (converts to None)
///
/// # Parameters
/// * `deserializer` - The deserializer to use
///
/// # Returns
/// * `Result<Option<i64>, D::Error>` - The parsed timestamp as Unix time or None
fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match s {
        // Handle numeric timestamps (Unix time)
        serde_json::Value::Number(num) => num.as_i64()
            .ok_or_else(|| serde::de::Error::custom("Invalid number"))
            .map(Some),
        
        // Handle string timestamps (RFC3339 format)
        serde_json::Value::String(ref s) => {
            let dt = DateTime::parse_from_rfc3339(s)
                .map_err(|_| serde::de::Error::custom("Invalid datetime string"))?;
            Ok(Some(dt.timestamp()))
        }
        
        // Handle null values
        serde_json::Value::Null => Ok(None),
        
        // Reject other formats
        _ => Err(serde::de::Error::custom("Invalid type for timestamp")),
    }
}

/// Represents a telemetry data point from a device.
///
/// This struct contains all telemetry information for a single reading from a device,
/// including sensor values, timestamp, and metadata. It maps directly to the API schema
/// and includes Cosmos DB specific fields.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Telemetry {
    /// Unique identifier for the telemetry record
    #[serde(
        rename = "id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub id: Option<String>,
    
    /// Identifier of the device that sent this telemetry
    pub device_id: String,
    
    /// Map of sensor readings (key-value pairs)
    /// Keys are sensor names (e.g., "temperature", "voltage")
    /// Values are the corresponding readings as strings
    pub telemetry_data: HashMap<String, String>,
    
    /// When the telemetry was recorded (Unix timestamp)
    #[serde(deserialize_with = "deserialize_timestamp", default)]
    pub timestamp: Option<i64>,
    
    // Cosmos DB specific fields
    /// Cosmos DB resource ID
    #[serde(rename = "_rid", skip_serializing_if = "Option::is_none")]
    rid: Option<String>,
    
    /// Cosmos DB self link
    #[serde(rename = "_self", skip_serializing_if = "Option::is_none")]
    self_link: Option<String>,
    
    /// Cosmos DB entity tag (for concurrency)
    #[serde(rename = "_etag", skip_serializing_if = "Option::is_none")]
    etag: Option<String>,
    
    /// Cosmos DB attachments
    #[serde(rename = "_attachments", skip_serializing_if = "Option::is_none")]
    attachments: Option<String>,
}

/// Error types for telemetry validation and processing.
///
/// This enum represents the various error conditions that can occur
/// when validating or processing telemetry data.
#[derive(Debug, Serialize)]
pub enum TelemetryError {
    /// Device ID is missing or empty
    InvalidDeviceId,
    
    /// Timestamp is invalid (e.g., negative)
    InvalidTimestamp,
    
    /// No telemetry data provided
    EmptyTelemetryData,
    
    /// A specific telemetry value is invalid (contains the error message)
    InvalidTelemetryValue(String),
}

/// Implements display formatting for TelemetryError
///
/// This allows error messages to be displayed to users in a readable format.
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

/// Implement standard error trait for TelemetryError
///
/// This allows TelemetryError to be used in contexts that expect standard errors.
impl std::error::Error for TelemetryError {}

impl Telemetry {
    /// Creates a new telemetry record with the specified values.
    ///
    /// This is a simple constructor that doesn't perform validation.
    /// For validated creation, use the `parse` method instead.
    ///
    /// # Parameters
    /// * `device_id` - The ID of the device that sent the telemetry
    /// * `telemetry_data` - Map of sensor readings
    /// * `timestamp` - Unix timestamp when the reading was taken
    ///
    /// # Returns
    /// A new Telemetry instance
    pub fn new(
        device_id: String,
        telemetry_data: HashMap<String, String>,
        timestamp: i64,
    ) -> Self {
        Telemetry {
            // Generate ID from device_id and timestamp for uniqueness
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

    /// Parses and validates telemetry data from raw inputs.
    ///
    /// This method performs validation on all fields and returns
    /// appropriate errors if any validation fails.
    ///
    /// # Parameters
    /// * `device_id` - The ID of the device that sent the telemetry
    /// * `telemetry_data` - Map of sensor readings
    /// * `timestamp` - Optional timestamp (current time used if None)
    ///
    /// # Returns
    /// * `Ok(Telemetry)` - If validation passes
    /// * `Err(TelemetryError)` - If any validation fails
    pub fn parse(device_id: String, telemetry_data: HashMap<String, String>, timestamp: Option<i64>) -> Result<Self, TelemetryError> {
        // Validate device_id
        if device_id.trim().is_empty() {
            return Err(TelemetryError::InvalidDeviceId);
        }

        // Use current timestamp if none provided
        let timestamp = timestamp.unwrap_or_else(|| Utc::now().timestamp());

        // Validate timestamp
        if timestamp < 0 {
            return Err(TelemetryError::InvalidTimestamp);
        }

        // Validate telemetry data
        if telemetry_data.is_empty() {
            return Err(TelemetryError::EmptyTelemetryData);
        }

        // Validate telemetry values
        for (key, value) in &telemetry_data {
            if value.trim().is_empty() {
                return Err(TelemetryError::InvalidTelemetryValue(
                    format!("Empty value for key: {}", key)
                ));
            }
        }

        // Create the telemetry instance
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
