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

pub mod routes;
pub mod services;
pub mod domain;
pub mod app_state;
pub mod utils;

use crate::app_state::AppState;
use crate::utils::tracing::{make_span_with_request_id, on_request, on_response};

pub struct TracingFairing;

#[rocket::async_trait]
impl Fairing for TracingFairing {
    fn info(&self) -> Info {
        Info {
            name: "Tracing Fairing",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut rocket::Data<'_>) {
        let span = make_span_with_request_id(request);
        let _guard = span.enter();
        on_request(request, &span);
        request.local_cache(|| (Arc::clone(&span), Instant::now()));
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if let Some((span, start)) = request.local_cache(|| None::<(Arc<Span>, Instant)>) {
            let latency = start.elapsed();
            on_response(response, latency, &span);
        }
    }
}

pub struct Application {
    pub server: rocket::Rocket<rocket::Build>,
    pub address: String,
    pub port: u16,
}

impl Application {
    pub async fn build(app_state: AppState) -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let cors = CorsOptions {
            allowed_origins: AllowedOrigins::All,
            ..Default::default()
        }
        .to_cors()?;

        let server = rocket::build()
            .configure(rocket::Config::figment()
                .merge(("secret_key", std::env::var("SECRET_KEY").unwrap()))
                .merge(("address", "0.0.0.0")))
            .manage(app_state)
            .attach(cors)
            .attach(TracingFairing)
            .mount("/iot/data", routes![
                routes::ingest_telemetry::ingest, 
            ]);

        println!("listening on 0.0.0.0:8000");
        Ok(Self {
            server,
            address: "0.0.0.0".to_string(),
            port: 8000,
        })
    }
}