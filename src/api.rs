use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::models::{LicenseResponse, ValidationError};

pub struct LicenseChainClient {
    api_key: String,
    base_url: String,
    client: Client,
}

impl LicenseChainClient {
    pub fn new(api_key: String, base_url: String) -> Self {
        LicenseChainClient {
            api_key,
            base_url,
            client: Client::new(),
        }
    }

    pub async fn validate_license(&self, license_key: &str) -> Result<LicenseResponse, ValidationError> {
        let url = format!("{}/validate", self.base_url);
        let response = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&serde_json::json!({ "license_key": license_key }))
            .send()
            .await?;

        if response.status().is_success() {
            let license: LicenseResponse = response.json().await?;
            Ok(license)
        } else {
            Err(ValidationError::from_response(response).await)
        }
    }
}
