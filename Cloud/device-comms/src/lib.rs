#[macro_use] extern crate rocket;

use dotenvy::dotenv;
use rocket::{
    routes,
};
use rocket_cors::{AllowedOrigins, CorsOptions};


pub mod routes;
pub mod services;
pub mod domain;

pub struct Application {
    pub server: rocket::Rocket<rocket::Build>,
    pub address: String,
    pub port: u16,
}

impl Application {
    pub async fn build() -> Result<Self, Box<dyn std::error::Error>> {
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
            .attach(cors)
            .mount("/iot/data", routes![routes::ingest_telemetry::ingest, routes::read_telemetry::read]);

        println!("listening on 0.0.0.0:8000");
        Ok(Self {
            server,
            address: "0.0.0.0".to_string(),
            port: 8000,
        })
    }
}