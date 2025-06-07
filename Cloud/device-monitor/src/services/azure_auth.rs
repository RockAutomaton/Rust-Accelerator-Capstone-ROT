use azure_identity::{ClientSecretCredential};
use azure_core::credentials::Secret;

pub struct AzureAuth {
    pub client_id: String,
    pub client_secret: Secret,
    pub tenant_id: String,
}

impl AzureAuth {
    pub fn new(client_id: String, client_secret: Secret, tenant_id: String) -> Self {
        AzureAuth {
            client_id,
            client_secret,
            tenant_id,
        }
    }

    /// Create AzureAuth from environment variables:
    /// AZURE_CLIENT_ID, AZURE_CLIENT_SECRET, AZURE_TENANT_ID
    pub fn get_credential_from_env() ->std::sync::Arc<ClientSecretCredential> {
        let tenant_id = std::env::var("AZURE_TENANT_ID").expect("AZURE_TENANT_ID not set");
        let client_id = std::env::var("AZURE_CLIENT_ID").expect("AZURE_CLIENT_ID not set");
        let client_secret = Secret::new(std::env::var("AZURE_CLIENT_SECRET").expect("AZURE_CLIENT_SECRET not set"));

        ClientSecretCredential::new(
            &tenant_id,
            client_id,
            client_secret,
            None,
        )
        .expect("Failed to create ClientSecretCredential")
    }

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