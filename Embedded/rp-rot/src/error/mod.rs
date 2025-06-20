/// # Error Types
///
/// This module defines error types for various parts of the application.
/// These error types are used for structured error handling and logging.

/// Errors that can occur during telemetry operations.
///
/// This enum represents the various failure modes when collecting
/// and transmitting telemetry data.
#[derive(Debug, defmt::Format)]
pub enum TelemetryError {
    /// DNS resolution failed (couldn't resolve hostname to IP)
    DnsResolve,
    
    /// Failed to establish a TCP connection with the server
    Connect,
    
    /// Failed to write data to the TCP socket
    Write,
    
    /// Failed to read data from the TCP socket
    Read,
    
    /// Server response was invalid or unexpected
    InvalidResponse,
}

/// Errors that can occur during WiFi operations.
///
/// This enum represents the various failure modes when connecting
/// to and using a WiFi network.
#[derive(Debug, defmt::Format)]
pub enum WiFiError {
    /// Failed to join the WiFi network with a specific status code
    JoinFailed(u32),
    
    /// Exceeded the maximum number of connection retry attempts
    MaxRetriesExceeded,
    
    /// Failed to initialize the WiFi hardware
    InitFailed,
    
    /// Generic join failure
    Join,
    
    /// Operation timed out
    Timeout
}