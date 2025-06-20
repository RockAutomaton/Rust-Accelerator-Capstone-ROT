// API Routes Module
// 
// This module contains all the HTTP route handlers for the device
// configuration service API endpoints.

pub mod update_config;
pub mod get_config;

// Re-export route handlers for convenient access
pub use update_config::*;
pub use get_config::*;