use defmt::*;

#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    pub host: &'static str,
    pub port: u16,
    pub path: &'static str,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            host: env!("TELEMETRY_HOST"),
            port: 80,
            path: "/iot/data/ingest",
        }
    }
}
