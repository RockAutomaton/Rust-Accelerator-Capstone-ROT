use defmt::*;

#[derive(Debug, Clone)]
pub struct WiFiConfig {
    pub network: &'static str,
    pub password: &'static str,
    pub max_retries: u8,
    pub retry_delay_secs: u64,
}

impl Default for WiFiConfig {
    fn default() -> Self {
        Self {
            network: env!("WIFI_NETWORK"),
            password: env!("WIFI_PASSWORD"),
            max_retries: 10,
            retry_delay_secs: 5,
        }
    }
}
