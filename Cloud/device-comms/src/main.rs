#[macro_use] extern crate rocket;

mod routes;

use routes::ingest_telemetry;

use dotenvy::dotenv;
use rocket::{
    routes,
};
use rocket_cors::{AllowedOrigins, CorsOptions};

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}


#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let cors = CorsOptions {
        allowed_origins: AllowedOrigins::All,
        ..Default::default()
    }.to_cors()?;
    
    let rocket = rocket::build()
        .configure(rocket::Config::figment()
            .merge(("secret_key", std::env::var("SECRET_KEY").unwrap()))
            .merge(("address", "0.0.0.0")))
            .attach(cors)
        .mount("/", routes![hello])
        .mount("/iot/data", routes![ingest_telemetry::ingest]);

    println!("listening on 0.0.0.0:8000");
    rocket.launch().await?;
    Ok(())
}