#[macro_use] extern crate rocket;

mod routes;

use routes::ingest_telemetry;

use dotenvy::dotenv;
use rocket::{
    routes,
};

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let rocket = rocket::build()
        .configure(rocket::Config::figment()
            .merge(("secret_key", std::env::var("SECRET_KEY").unwrap()))
            .merge(("address", "0.0.0.0")))
        .mount("/", routes![hello])
        .mount("/iot/data", routes![ingest_telemetry::ingest]);

    println!("listening on 0.0.0.0:8000");
    rocket.launch().await?;
    Ok(())
}