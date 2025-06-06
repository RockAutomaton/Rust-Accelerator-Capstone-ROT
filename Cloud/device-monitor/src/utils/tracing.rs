use std::time::Duration;
use rocket::{Request, Response};

use tracing::{Level, Span};

use color_eyre::eyre::Result;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};
use std::sync::Arc;

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

// Creates a new tracing span with a unique request ID for each incoming request.
// This helps in tracking and correlating logs for individual requests.
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

// Logs an event indicating the start of a request.
pub fn on_request(_request: &Request, _span: &Span) {
    tracing::event!(Level::INFO, "[REQUEST START]");
}

// Logs an event indicating the end of a request, including its latency and status code.
// If the status code indicates an error (4xx or 5xx), it logs at the ERROR level.
pub fn on_response(response: &Response, latency: Duration, _span: &Span) {
    let status = response.status();
    let status_code = status.code;
    let status_code_class = status_code / 100;

    match status_code_class {
        4..=5 => {
            tracing::event!(
                Level::ERROR,
                latency = ?latency,
                status = status_code,
                "[REQUEST END]"
            )
        }
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