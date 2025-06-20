/// # Telemetry Configuration
///
/// This module defines the configuration for sending telemetry data to the cloud backend.
/// It includes server information and API endpoint details.

/// Configuration for telemetry data transmission.
///
/// This struct provides constants for connecting to the telemetry ingestion service.
pub struct TelemetryConfig;

impl TelemetryConfig {
    /// Hostname of the telemetry server, included from environment variables
    pub const HOST: &'static str = env!("TELEMETRY_HOST");
    
    /// Port number of the telemetry server (standard HTTP port)
    pub const PORT: u16 = 80;
    
    /// API endpoint path for telemetry data ingestion
    pub const PATH: &'static str = "/iot/data/ingest";
}
