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
}