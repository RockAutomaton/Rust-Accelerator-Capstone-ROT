use rocket::{
    local::asynchronous::Client,
    routes,
};
use rocket_cors::{AllowedOrigins, CorsOptions};
use device_comms::{app_state::AppState, services::CosmosDbTelemetryStore};

pub struct TestApp {
    pub client: Client,
    pub address: String,
    pub port: u16,
}

impl TestApp {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Create test cosmos client - you might want to use test database/container names
        let cosmos_client = CosmosDbTelemetryStore::new(
            "test-device-data".to_string(), 
            "test-telemetry".to_string()
        ).await?;
        
        let app_state = AppState::new(cosmos_client);

        let cors = CorsOptions {
            allowed_origins: AllowedOrigins::All,
            ..Default::default()
        }
        .to_cors()?;

        let server = rocket::build()
            .configure(rocket::Config::figment()
                .merge(("secret_key", "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890")) // 64 hex chars
                .merge(("address", "0.0.0.0")))
            .manage(app_state) 
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