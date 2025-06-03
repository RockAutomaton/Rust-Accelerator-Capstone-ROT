use rocket::{
    local::asynchronous::Client,
    routes,
};
use rocket_cors::{AllowedOrigins, CorsOptions};

pub struct TestApp {
    pub client: Client,
    pub address: String,
    pub port: u16,
}

impl TestApp {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let cors = CorsOptions {
            allowed_origins: AllowedOrigins::All,
            ..Default::default()
        }
        .to_cors()?;

        let server = rocket::build()
            .configure(rocket::Config::figment()
                .merge(("secret_key", "test_secret_key"))
                .merge(("address", "0.0.0.0")))
            .attach(cors)
            .mount("/iot/data", routes![
                device_comms::routes::ingest_telemetry::ingest,
                device_comms::routes::read_telemetry::read
            ]);

        let client = Client::tracked(server).await?;

        Ok(Self {
            client,
            address: "0.0.0.0".to_string(),
            port: 8000,
        })
    }
}