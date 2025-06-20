// Configuration Error Handling
// 
// This module defines the error types used throughout the configuration API
// and their corresponding HTTP status codes for proper error responses.

use std::fmt;
use rocket::http::Status;
use crate::domain::config::ConfigError;

/// Converts configuration errors to appropriate HTTP status codes
/// 
/// This implementation maps different types of errors to standard
/// HTTP status codes for proper REST API error handling:
/// - Validation errors -> 400 Bad Request
/// - Not found errors -> 404 Not Found
/// - Database errors -> 500 Internal Server Error
impl From<ConfigError> for rocket::http::Status {
    fn from(error: ConfigError) -> Self {
        match error {
            // Client errors (4xx) - invalid request data
            ConfigError::InvalidDeviceId | 
            ConfigError::InvalidConfig => Status::BadRequest,
            
            // Not found errors (4xx) - resource doesn't exist
            ConfigError::DeviceNotFound(_) => Status::NotFound,
            
            // Server errors (5xx) - internal processing failure
            ConfigError::DatabaseError(_) => Status::InternalServerError,
        }
    }
}

