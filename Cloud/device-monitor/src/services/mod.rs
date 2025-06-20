// External Services Module
// 
// This module contains integrations with external services like Azure Cosmos DB
// and Azure authentication. These services handle data persistence and
// cloud infrastructure interactions.

pub mod cosmos_db_telemetry_store;
pub mod azure_auth;

// Re-export service types for convenient access
pub use azure_auth::AzureAuth;
pub use cosmos_db_telemetry_store::CosmosDbTelemetryStore;