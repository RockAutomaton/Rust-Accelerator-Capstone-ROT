use heapless::{String, Vec};
use serde::{Deserialize, Serialize};

// Max lengths for heapless types
pub const MAX_DEVICE_ID_LEN: usize = 16;
pub const MAX_VALUE_LEN: usize = 16;
pub const MAX_CONFIGS: usize = 1;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DeviceConfigItem {
    pub device_id: String<MAX_DEVICE_ID_LEN>,
    pub config: Config,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Config {
    pub LED: Option<String<MAX_VALUE_LEN>>,
    // Add more fields as needed
}

// The top-level response is a list of DeviceConfigItem
pub type DeviceConfigResponse = Vec<DeviceConfigItem, MAX_CONFIGS>;
