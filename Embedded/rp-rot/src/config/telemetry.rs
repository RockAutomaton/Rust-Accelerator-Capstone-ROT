pub struct TelemetryConfig;

impl TelemetryConfig {
    pub const HOST: &'static str = env!("TELEMETRY_HOST");
    pub const PORT: u16 = 80;
    pub const PATH: &'static str = "/iot/data/ingest";
}
