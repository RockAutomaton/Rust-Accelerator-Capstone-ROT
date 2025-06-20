// API Error Handling
// 
// This module defines the error types used throughout the API and their
// corresponding HTTP status codes for proper error responses.

use std::fmt;
use rocket::http::Status;

/// API error types that can occur during request processing
/// 
/// These errors are mapped to appropriate HTTP status codes and
/// provide meaningful error messages to API clients.
#[derive(Debug)]
pub enum ApiError {
    // Telemetry validation errors
    /// Device ID is empty, malformed, or invalid
    InvalidDeviceId,
    /// Timestamp is invalid or in wrong format
    InvalidTimestamp,
    /// No telemetry data provided in request
    EmptyTelemetryData,
    /// Individual telemetry value is invalid or empty
    InvalidTelemetryValue(String),

    // Database errors
    /// Generic database operation error with details
    DatabaseError(String),

    // Resource errors
    /// Requested device telemetry not found in database
    DeviceNotFound(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::InvalidDeviceId => write!(f, "Invalid device ID format"),
            ApiError::InvalidTimestamp => write!(f, "Invalid timestamp format"),
            ApiError::EmptyTelemetryData => write!(f, "Telemetry data cannot be empty"),
            ApiError::InvalidTelemetryValue(msg) => write!(f, "Invalid telemetry value: {}", msg),
            ApiError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            ApiError::DeviceNotFound(device_id) => write!(f, "No telemetry found for device {}", device_id),
        }
    }
}

impl std::error::Error for ApiError {}

/// Converts API errors to appropriate HTTP status codes
/// 
/// This implementation maps different types of errors to standard
/// HTTP status codes for proper REST API error handling:
/// - Validation errors -> 400 Bad Request
/// - Not found errors -> 404 Not Found  
/// - Database errors -> 500 Internal Server Error
impl From<ApiError> for rocket::http::Status {
    fn from(error: ApiError) -> Self {
        match error {
            // Client errors (4xx) - invalid request data
            ApiError::InvalidDeviceId | 
            ApiError::InvalidTimestamp | 
            ApiError::EmptyTelemetryData | 
            ApiError::InvalidTelemetryValue(_) => Status::BadRequest,
            
            // Not found errors (4xx) - resource doesn't exist
            ApiError::DeviceNotFound(_) => Status::NotFound,
            
            // Server errors (5xx) - internal processing failure
            ApiError::DatabaseError(_) => Status::InternalServerError,
        }
    }
}
