// Domain Module
// 
// This module contains the core business logic and data structures
// for the device monitoring service, including telemetry models
// and error handling.

pub mod telemetry;
pub mod error;

// Re-export all telemetry-related types for convenient access
pub use telemetry::*;