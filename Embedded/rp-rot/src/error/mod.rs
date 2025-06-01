#[derive(Debug, defmt::Format)]
pub enum TelemetryError {
    DnsResolve,
    Connect,
    Write,
    Read,
    InvalidResponse,
}

#[derive(Debug, defmt::Format)]
pub enum WiFiError {
    JoinFailed(u32),
    MaxRetriesExceeded,
    InitFailed,
    InvalidCredentials,
}
