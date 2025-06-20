// Configuration Error Handling
// 
// This module defines the error types used throughout the configuration API
// and their corresponding HTTP status codes for proper error responses.

use std::fmt;
use rocket::http::Status;

/// Configuration error types that can occur during request processing
/// 
/// These errors are mapped to appropriate HTTP status codes and
/// provide meaningful error messages to API clients.
#[derive(Debug)]
pub enum ConfigError {
    /// Device ID is empty, malformed, or invalid
    InvalidDeviceId,
    /// Configuration data is invalid or malformed
    InvalidConfig,
    /// Generic database operation error with details
    DatabaseError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Config error: {:?}", self)
    }
}

/// Converts configuration errors to appropriate HTTP status codes
/// 
/// This implementation maps different types of errors to standard
/// HTTP status codes for proper REST API error handling:
/// - Validation errors -> 400 Bad Request
/// - Database errors -> 500 Internal Server Error
impl From<ConfigError> for rocket::http::Status {
    fn from(error: ConfigError) -> Self {
        match error {
            // Client errors (4xx) - invalid request data
            ConfigError::InvalidDeviceId | 
            ConfigError::InvalidConfig => Status::BadRequest,
            
            // Server errors (5xx) - internal processing failure
            ConfigError::DatabaseError(_) => Status::InternalServerError,
        }
    }
}

