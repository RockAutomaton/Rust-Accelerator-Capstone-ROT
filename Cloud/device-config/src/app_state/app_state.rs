use crate::services::CosmosDbTelemetryStore;

#[derive(Clone)]
pub struct AppState {
    pub cosmos_client: CosmosDbTelemetryStore,
}

impl AppState {
    pub fn new(cosmos_client: CosmosDbTelemetryStore) -> Self {
        Self { cosmos_client }
    }
}