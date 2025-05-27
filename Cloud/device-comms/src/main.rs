#[macro_use] extern crate rocket;

mod routes;

use routes::ingest_telemetry;

use rocket::{
    routes,
};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rocket = rocket::build()
        .mount("/iot/data", routes![ingest_telemetry::ingest]);

    println!("listening on 127.0.0.1:8000");
    rocket.launch().await?;
    Ok(())
}