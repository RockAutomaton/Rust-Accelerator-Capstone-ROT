/// # WiFi Configuration
///
/// This module defines the WiFi connection parameters and settings.
/// The credentials are included from environment variables at build time
/// to avoid hardcoding sensitive information.

/// Configuration for WiFi connection parameters.
///
/// This struct contains all the information needed to connect to a WiFi network
/// and handle reconnection attempts.
#[derive(Debug, Clone)]
pub struct WiFiConfig {
    /// SSID (network name) of the WiFi network to connect to
    pub network: &'static str,
    
    /// Password for the WiFi network
    pub password: &'static str,
    
    /// Maximum number of connection retry attempts before entering error state
    pub max_retries: u8,
    
    /// Delay in seconds between connection retry attempts
    pub retry_delay_secs: u64,
}

impl Default for WiFiConfig {
    /// Creates a default WiFi configuration using environment variables.
    ///
    /// The default configuration:
    /// - Uses credentials from environment variables
    /// - Allows up to 10 retry attempts
    /// - Waits 5 seconds between retry attempts
    fn default() -> Self {
        Self {
            // Network credentials from environment variables (set at build time)
            network: env!("WIFI_NETWORK"),
            password: env!("WIFI_PASSWORD"),
            
            // Connection retry parameters
            max_retries: 10,
            retry_delay_secs: 5,
        }
    }
}
