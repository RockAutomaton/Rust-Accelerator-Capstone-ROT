// Tracing and Logging Utilities
// 
// This module provides structured logging and tracing functionality for
// the device configuration service. It includes request/response logging,
// error tracking, and performance monitoring capabilities.

use std::time::Duration;
use rocket::{Request, Response};

use tracing::{Level, Span};

use color_eyre::eyre::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};
use std::sync::Arc;

/// Initializes the tracing and logging system
/// 
/// This function sets up structured logging with the following features:
/// - Compact log formatting for readability
/// - Environment-based log level filtering
/// - Error context tracking for better debugging
/// - Request/response correlation
/// 
/// # Returns
/// * `Result<()>` - Success or an error if initialization fails
/// 
/// # Environment Variables
/// * `RUST_LOG` - Controls log level (e.g., "info", "debug", "warn")
/// 
/// # Example
/// ```rust
/// use device_config::utils::init_tracing;
/// 
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     init_tracing()?;
///     Ok(())
/// }
/// ```
pub fn init_tracing() -> Result<()> {
    // Create a formatting layer for tracing output with a compact format
    let fmt_layer = fmt::layer().compact();

    // Create a filter layer to control the verbosity of logs
    // Try to get the filter configuration from the environment variables
    // If it fails, default to the "info" log level
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    // Build the tracing subscriber registry with the formatting layer,
    // the filter layer, and the error layer for enhanced error reporting
    tracing_subscriber::registry()
        .with(filter_layer) // Add the filter layer to control log verbosity
        .with(fmt_layer) // Add the formatting layer for compact log output
        .with(ErrorLayer::default()) // Add the error layer to capture error contexts
        .init(); // Initialize the tracing subscriber

    Ok(())
}

/// Creates a new tracing span with a unique request ID for each incoming request
/// 
/// This function generates a unique identifier for each HTTP request and creates
/// a tracing span that can be used to correlate all logs and events related to
/// that specific request. This is essential for debugging and monitoring.
/// 
/// # Arguments
/// * `request` - The incoming HTTP request
/// 
/// # Returns
/// * `Arc<Span>` - A thread-safe reference to the tracing span
/// 
/// # Fields Included
/// * `method` - HTTP method (GET, POST, etc.)
/// * `uri` - Request URI path
/// * `request_id` - Unique identifier for request correlation
pub fn make_span_with_request_id(request: &Request) -> Arc<Span> {
    let request_id = uuid::Uuid::new_v4();
    Arc::new(tracing::span!(
        Level::INFO,
        "[REQUEST]",
        method = tracing::field::display(request.method()),
        uri = tracing::field::display(request.uri()),
        request_id = tracing::field::display(request_id),
    ))
}

/// Logs the start of an HTTP request
/// 
/// This function is called when a new request begins processing.
/// It logs basic information about the request for monitoring purposes.
/// 
/// # Arguments
/// * `_request` - The incoming HTTP request (currently unused)
/// * `_span` - The tracing span for this request (currently unused)
pub fn on_request(_request: &Request, _span: &Span) {
    tracing::event!(Level::INFO, "[REQUEST START]");
}

/// Logs the completion of an HTTP request with performance metrics
/// 
/// This function logs the end of request processing, including:
/// - Request processing latency
/// - HTTP status code
/// - Error classification (4xx/5xx vs 2xx/3xx)
/// 
/// The log level is automatically adjusted based on the status code:
/// - ERROR level for 4xx and 5xx status codes
/// - INFO level for 2xx and 3xx status codes
/// 
/// # Arguments
/// * `response` - The HTTP response being sent
/// * `latency` - The total time taken to process the request
/// * `_span` - The tracing span for this request (currently unused)
pub fn on_response(response: &Response, latency: Duration, _span: &Span) {
    let status = response.status();
    let status_code = status.code;
    let status_code_class = status_code / 100;

    match status_code_class {
        // Log errors (4xx client errors, 5xx server errors) at ERROR level
        4..=5 => {
            tracing::event!(
                Level::ERROR,
                latency = ?latency,
                status = status_code,
                "[REQUEST END]"
            )
        }
        // Log successful requests (2xx success, 3xx redirects) at INFO level
        _ => {
            tracing::event!(
                Level::INFO,
                latency = ?latency,
                status = status_code,
                "[REQUEST END]"
            )
        }
    };
}