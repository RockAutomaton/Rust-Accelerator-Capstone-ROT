// Device Communications Service Library
// 
// This library provides a REST API for ingesting telemetry data from IoT devices.
// It uses Rocket web framework with structured logging, CORS support, and Cosmos DB storage.

#[macro_use] extern crate rocket;

use dotenvy::dotenv;
use rocket::{
    routes,
    fairing::{Fairing, Info, Kind},
    Request, Response,
};
use rocket_cors::{AllowedOrigins, CorsOptions};
use std::time::Instant;
use std::sync::Arc;
use tracing::Span;

// Module declarations for the service components
pub mod routes;      // API route handlers
pub mod services;    // External service integrations (Cosmos DB, Azure Auth)
pub mod domain;      // Domain models and business logic
pub mod app_state;   // Application state management
pub mod utils;       // Utility functions and helpers

use crate::app_state::AppState;
use crate::utils::tracing::{make_span_with_request_id, on_request, on_response};

/// Rocket fairing for request/response tracing and observability
/// 
/// This fairing automatically creates tracing spans for each HTTP request,
/// measures request latency, and logs request/response details for monitoring.
pub struct TracingFairing;

#[rocket::async_trait]
impl Fairing for TracingFairing {
    /// Returns information about this fairing
    fn info(&self) -> Info {
        Info {
            name: "Tracing Fairing",
            kind: Kind::Request | Kind::Response, // Attaches to both request and response phases
        }
    }

    /// Called when a request is received
    /// 
    /// Creates a new tracing span with a unique request ID and stores timing information
    /// for later use in response handling.
    async fn on_request(&self, request: &mut Request<'_>, _data: &mut rocket::Data<'_>) {
        // Create a new tracing span with request ID for this request
        let span = make_span_with_request_id(request);
        let _guard = span.enter();
        
        // Log request details
        on_request(request, &span);
        
        // Store span and start time in request-local cache for response handling
        request.local_cache(|| (Arc::clone(&span), Instant::now()));
    }

    /// Called when a response is being sent
    /// 
    /// Calculates request latency and logs response details for monitoring and debugging.
    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        // Retrieve the span and start time from request-local cache
        if let Some((span, start)) = request.local_cache(|| None::<(Arc<Span>, Instant)>) {
            // Calculate total request processing time
            let latency = start.elapsed();
            
            // Log response details with latency information
            on_response(response, latency, &span);
        }
    }
}

/// Main application structure containing the Rocket server instance
/// 
/// Holds the configured Rocket server along with address and port information
/// for the IoT telemetry ingestion service.
pub struct Application {
    pub server: rocket::Rocket<rocket::Build>,
    pub address: String,
    pub port: u16,
}

impl Application {
    /// Builds and configures the Rocket application with all necessary middleware and routes
    /// 
    /// This method:
    /// 1. Loads environment variables
    /// 2. Configures CORS for cross-origin requests
    /// 3. Sets up Rocket configuration with secret key and address
    /// 4. Attaches the application state and middleware
    /// 5. Mounts the telemetry ingestion route
    /// 
    /// # Arguments
    /// * `app_state` - The application state containing database connections and other shared resources
    /// 
    /// # Returns
    /// * `Result<Self, Box<dyn std::error::Error>>` - The configured application or an error
    pub async fn build(app_state: AppState) -> Result<Self, Box<dyn std::error::Error>> {
        // Load environment variables from .env file
        dotenv().ok();

        // Configure CORS to allow all origins (for development - should be restricted in production)
        let cors = CorsOptions {
            allowed_origins: AllowedOrigins::All,
            ..Default::default()
        }
        .to_cors()?;

        // Build and configure the Rocket server
        let server = rocket::build()
            // Configure Rocket with secret key and binding address
            .configure(rocket::Config::figment()
                .merge(("secret_key", std::env::var("SECRET_KEY").unwrap()))
                .merge(("address", "0.0.0.0")))
            // Attach application state for dependency injection
            .manage(app_state)
            // Enable CORS for cross-origin requests
            .attach(cors)
            // Add request/response tracing for observability
            .attach(TracingFairing)
            // Mount the telemetry ingestion endpoint
            .mount("/iot/data", routes![
                routes::ingest_telemetry::ingest, 
            ]);

        // Log the server startup information
        println!("listening on 0.0.0.0:8000");
        
        // Return the configured application
        Ok(Self {
            server,
            address: "0.0.0.0".to_string(),
            port: 8000,
        })
    }
}