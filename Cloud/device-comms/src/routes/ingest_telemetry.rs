use std::collections::HashMap;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Telemetry {
    device_id: String,
    telemetry_data: HashMap<String, String>,
}

#[post("/ingest", data = "<telemetry>")]
pub async fn ingest(telemetry: Json<Telemetry>) -> &'static str {
    println!("Received telemetry: {:?}", telemetry);
    "Telemetry ingested"
}

#[cfg(test)]
mod test {
    use super::*;

    #[async_test]
    async fn test_ingest() {
        let telemetry = Telemetry {
            device_id: "test_device".to_string(),
            telemetry_data: HashMap::new(),
        };

        let result = ingest(Json(telemetry)).await;
        assert_eq!(result, "Telemetry ingested");
    }
}

