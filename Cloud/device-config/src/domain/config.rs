use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::domain::error::ConfigError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub device_id: String,
    pub config: HashMap<String, String>,
}

impl Config {
    pub fn parse(device_id: String, config: HashMap<String, String>) -> Result<Self, ConfigError> {
        Ok(Self { device_id, config })
    }
}