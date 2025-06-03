use rocket::{
    local::asynchronous::Client,
    routes,
};
use rocket_cors::{AllowedOrigins, CorsOptions};
use device_comms::{app_state::AppState, services::CosmosDbTelemetryStore};
use std::sync::atomic::{AtomicUsize, Ordering};

static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[allow(dead_code)]
pub struct TestApp {
    pub client: Client,
    pub address: String,
    pub port: u16,
    pub app_state: AppState,
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
            .manage(app_state.clone()) 
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
            app_state,
        })
    }

    pub fn generate_test_device_id(&self) -> String {
        let count = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("test_device_{}", count)
    }
}