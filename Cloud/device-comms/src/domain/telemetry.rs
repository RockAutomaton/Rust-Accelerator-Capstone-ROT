use serde::{Deserialize, Serialize, Deserializer};
use std::{collections::HashMap};
use chrono::DateTime;

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: serde_json::Value = Deserialize::deserialize(deserializer)?;
    match s {
        serde_json::Value::Number(num) => num.as_i64().ok_or_else(|| serde::de::Error::custom("Invalid number")).map(Some),
        serde_json::Value::String(ref s) => {
            let dt = DateTime::parse_from_rfc3339(s)
                .map_err(|_| serde::de::Error::custom("Invalid datetime string"))?;
            Ok(Some(dt.timestamp()))
        }
        serde_json::Value::Null => Ok(None),
        _ => Err(serde::de::Error::custom("Invalid type for timestamp")),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Telemetry {
    #[serde(
        rename = "id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub id: Option<String>,
    pub device_id: String,
    pub telemetry_data: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_timestamp")]
    pub timestamp: Option<i64>,
    #[serde(rename = "_rid", skip_serializing_if = "Option::is_none")]
    rid: Option<String>,
    #[serde(rename = "_self", skip_serializing_if = "Option::is_none")]
    self_link: Option<String>,
    #[serde(rename = "_etag", skip_serializing_if = "Option::is_none")]
    etag: Option<String>,
    #[serde(rename = "_attachments", skip_serializing_if = "Option::is_none")]
    attachments: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum TelemetryError {
    InvalidDeviceId,
    InvalidTimestamp,
    EmptyTelemetryData,
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

    pub fn parse(device_id: String, telemetry_data: HashMap<String, String>, timestamp: i64) -> Result<Self, TelemetryError> {
        // Validate device_id
        if device_id.trim().is_empty() {
            return Err(TelemetryError::InvalidDeviceId);
        }

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
