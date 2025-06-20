// Domain Module
// 
// This module contains the core business logic and data structures
// for the device configuration service, including configuration models
// and error handling.

pub mod config;
pub mod error;

// Re-export all domain types for convenient access
pub use config::*;
pub use error::*;