/// # Device Configuration Structures
///
/// This module defines the data structures for device configuration.
/// These structures are used for deserializing configuration data from the cloud
/// and storing it locally on the device.

use heapless::{String, Vec};
use serde::{Deserialize, Serialize};

// Maximum lengths for fixed-capacity types
// These constants define the size limits for our heapless collections
/// Maximum length of a device ID string
pub const MAX_DEVICE_ID_LEN: usize = 16;
/// Maximum length of a configuration value string
pub const MAX_VALUE_LEN: usize = 16;
/// Maximum number of device configurations in a response
pub const MAX_CONFIGS: usize = 1;

/// Represents a configuration item for a specific device.
///
/// This struct is the main container for device configuration data.
/// It includes the device's identifier and its specific configuration settings.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DeviceConfigItem {
    /// Unique identifier for the device
    pub device_id: String<MAX_DEVICE_ID_LEN>,
    
    /// Configuration settings for the device
    pub config: Config,
}

/// Contains specific configuration settings for a device.
///
/// This struct holds various configuration parameters that control
/// the behavior of the device.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Config {
    /// LED state: "on" to enable, "off" to disable
    /// This is optional - if not provided, the LED state remains unchanged
    pub LED: Option<String<MAX_VALUE_LEN>>,
    
    // Add more configuration fields as needed for future enhancements:
    // pub reporting_interval: Option<String<MAX_VALUE_LEN>>,
    // pub power_mode: Option<String<MAX_VALUE_LEN>>,
    // etc.
}

/// Represents the response from the configuration API.
///
/// The configuration API returns an array of device configurations.
/// This type alias defines that response structure with a fixed capacity.
pub type DeviceConfigResponse = Vec<DeviceConfigItem, MAX_CONFIGS>;
