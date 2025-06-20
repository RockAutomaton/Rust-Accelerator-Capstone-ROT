// Application State Management
// 
// This module defines the shared application state that is injected into
// all request handlers via Rocket's state management system.

use crate::services::CosmosDbTelemetryStore;

/// Application state containing shared resources and dependencies
/// 
/// This struct holds all the shared state that needs to be accessible
/// across different request handlers. It's managed by Rocket and injected
/// into handlers as needed.
/// 
/// The state is cloneable to allow multiple handlers to access it concurrently.
#[derive(Clone)]
pub struct AppState {
    /// Cosmos DB client for device configuration storage operations
    /// 
    /// This client is used by configuration handlers to store and retrieve
    /// device configuration data in the Cosmos DB database.
    pub cosmos_client: CosmosDbTelemetryStore,
}

impl AppState {
    /// Creates a new application state instance
    /// 
    /// # Arguments
    /// * `cosmos_client` - The configured Cosmos DB configuration store client
    /// 
    /// # Returns
    /// * `Self` - A new AppState instance with the provided dependencies
    pub fn new(cosmos_client: CosmosDbTelemetryStore) -> Self {
        Self { cosmos_client }
    }
}