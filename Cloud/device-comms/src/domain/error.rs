use std::fmt;
use rocket::http::Status;

#[derive(Debug)]
pub enum ApiError {
    // Telemetry validation errors
    InvalidDeviceId,
    InvalidTimestamp,
    EmptyTelemetryData,
    InvalidTelemetryValue(String),

    // Database errors
    DatabaseError(String),

    // Resource errors
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

impl From<ApiError> for rocket::http::Status {
    fn from(error: ApiError) -> Self {
        match error {
            ApiError::InvalidDeviceId | 
            ApiError::InvalidTimestamp | 
            ApiError::EmptyTelemetryData | 
            ApiError::InvalidTelemetryValue(_) => Status::BadRequest,
            ApiError::DeviceNotFound(_) => Status::NotFound,
            ApiError::DatabaseError(_) => Status::InternalServerError,
        }
    }
}
