// Azure Authentication Service
// 
// This module handles Azure authentication for accessing Azure services
// like Cosmos DB. It provides methods to create client secret credentials
// from environment variables or direct configuration.

use azure_identity::{ClientSecretCredential};
use azure_core::credentials::Secret;

/// Azure authentication configuration using client secret credentials
/// 
/// This struct holds the necessary credentials for authenticating with
/// Azure services using the client secret flow (service principal).
pub struct AzureAuth {
    /// Azure AD application (client) ID
    pub client_id: String,
    /// Azure AD application client secret
    pub client_secret: Secret,
    /// Azure AD tenant ID
    pub tenant_id: String,
}

impl AzureAuth {
    /// Creates a new AzureAuth instance with the provided credentials
    /// 
    /// # Arguments
    /// * `client_id` - The Azure AD application client ID
    /// * `client_secret` - The Azure AD application client secret
    /// * `tenant_id` - The Azure AD tenant ID
    /// 
    /// # Returns
    /// * `Self` - A new AzureAuth instance
    pub fn new(client_id: String, client_secret: Secret, tenant_id: String) -> Self {
        AzureAuth {
            client_id,
            client_secret,
            tenant_id,
        }
    }

    /// Creates Azure client secret credentials from environment variables
    /// 
    /// This method reads the following environment variables:
    /// - AZURE_CLIENT_ID: The Azure AD application client ID
    /// - AZURE_CLIENT_SECRET: The Azure AD application client secret
    /// - AZURE_TENANT_ID: The Azure AD tenant ID
    /// 
    /// # Returns
    /// * `std::sync::Arc<ClientSecretCredential>` - Thread-safe credential for Azure services
    /// 
    /// # Panics
    /// Panics if any of the required environment variables are not set
    /// 
    /// # Environment Variables Required
    /// * `AZURE_CLIENT_ID` - Azure AD application client ID
    /// * `AZURE_CLIENT_SECRET` - Azure AD application client secret
    /// * `AZURE_TENANT_ID` - Azure AD tenant ID
    pub fn get_credential_from_env() ->std::sync::Arc<ClientSecretCredential> {
        // Read Azure authentication credentials from environment variables
        let tenant_id = std::env::var("AZURE_TENANT_ID").expect("AZURE_TENANT_ID not set");
        let client_id = std::env::var("AZURE_CLIENT_ID").expect("AZURE_CLIENT_ID not set");
        let client_secret = Secret::new(std::env::var("AZURE_CLIENT_SECRET").expect("AZURE_CLIENT_SECRET not set"));

        // Create and return the client secret credential
        ClientSecretCredential::new(
            &tenant_id,
            client_id,
            client_secret,
            None,
        )
        .expect("Failed to create ClientSecretCredential")
    }

    /// Creates Azure client secret credentials from the instance fields
    /// 
    /// This method uses the credentials stored in the AzureAuth instance
    /// to create a client secret credential for Azure service authentication.
    /// 
    /// # Returns
    /// * `std::sync::Arc<ClientSecretCredential>` - Thread-safe credential for Azure services
    /// 
    /// # Panics
    /// Panics if the credential creation fails
    pub fn get_credential(&self) -> std::sync::Arc<ClientSecretCredential> {
        ClientSecretCredential::new(
            &self.tenant_id,
            self.client_id.clone(),
            self.client_secret.clone(),
            None,
        )
        .expect("Failed to create ClientSecretCredential")
    }
}

    // let tenant_id = std::env::var("AZURE_TENANT_ID").unwrap();
    // let client_id = std::env::var("AZURE_CLIENT_ID").unwrap();
    // let client_secret = Secret::new(std::env::var("AZURE_CLIENT_SECRET").unwrap());