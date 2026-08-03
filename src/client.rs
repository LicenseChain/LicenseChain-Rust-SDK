use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use sha2::{Digest, Sha256};
use tokio::time::timeout;

#[derive(Debug, Clone)]
pub struct LicenseChainConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout: Duration,
    pub retries: u32,
}

impl Default for LicenseChainConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.licensechain.app/v1".to_string(),
            timeout: Duration::from_secs(30),
            retries: 3,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct License {
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub license_key: String,
    pub status: String,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub status: String,
    pub created_at: String,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub currency: String,
    pub status: String,
    pub created_at: String,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: String,
    pub r#type: String,
    pub data: serde_json::Value,
    pub timestamp: String,
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: u32,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseStats {
    pub total: u32,
    pub active: u32,
    pub expired: u32,
    pub suspended: u32,
    pub revoked: u32,
    pub revenue: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStats {
    pub total: u32,
    pub active: u32,
    pub inactive: u32,
    pub suspended: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductStats {
    pub total: u32,
    pub active: u32,
    pub inactive: u32,
    pub archived: u32,
    pub revenue: f64,
}

pub struct LicenseChainClient {
    client: Client,
    config: LicenseChainConfig,
}

impl LicenseChainClient {
    pub fn new(config: LicenseChainConfig) -> Self {
        let client = Client::builder()
            .timeout(config.timeout)
            .build()
            .expect("Failed to create HTTP client");

        Self { client, config }
    }

    pub fn with_api_key(api_key: String) -> Self {
        let config = LicenseChainConfig {
            api_key,
            ..Default::default()
        };
        Self::new(config)
    }

    // License Management
    pub async fn create_license(
        &self,
        app_id: &str,
        user_email: &str,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<License, Box<dyn std::error::Error + Send + Sync>> {
        let payload = serde_json::json!({
            "appId": app_id,
            "issuedEmail": user_email,
            "metadata": metadata
        });

        let response = self
            .make_request("POST", &format!("/apps/{}/licenses", app_id), Some(payload))
            .await?;

        let api_response: ApiResponse<License> = response.json().await?;
        
        if api_response.success {
            Ok(api_response.data.unwrap())
        } else {
            Err(format!("API Error: {}", api_response.error.unwrap_or("Unknown error".to_string())).into())
        }
    }

    pub async fn get_license(&self, license_id: &str) -> Result<License, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .make_request("GET", &format!("/licenses/{}", license_id), None)
            .await?;

        let api_response: ApiResponse<License> = response.json().await?;
        
        if api_response.success {
            Ok(api_response.data.unwrap())
        } else {
            Err(format!("API Error: {}", api_response.error.unwrap_or("Unknown error".to_string())).into())
        }
    }

    pub async fn update_license(
        &self,
        license_id: &str,
        updates: HashMap<String, serde_json::Value>,
    ) -> Result<License, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .make_request("PATCH", &format!("/licenses/{}", license_id), Some(serde_json::Value::Object(updates.into_iter().collect())))
            .await?;

        let api_response: ApiResponse<License> = response.json().await?;
        
        if api_response.success {
            Ok(api_response.data.unwrap())
        } else {
            Err(format!("API Error: {}", api_response.error.unwrap_or("Unknown error".to_string())).into())
        }
    }

    pub async fn revoke_license(&self, license_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .make_request("PATCH", &format!("/licenses/{}/revoke", license_id), Some(serde_json::json!({})))
            .await?;

        let api_response: ApiResponse<()> = response.json().await?;
        
        if api_response.success {
            Ok(())
        } else {
            Err(format!("API Error: {}", api_response.error.unwrap_or("Unknown error".to_string())).into())
        }
    }

    pub async fn validate_license(&self, license_key: &str) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        self.validate_license_with_hwuid(license_key, None).await
    }

    /// Validate license with optional hwuid (ecosystem HMAC/HWUID contract).
    pub async fn validate_license_with_hwuid(&self, license_key: &str, hwuid: Option<&str>) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let mut payload = serde_json::json!({ "key": license_key });
        if let Some(h) = hwuid.filter(|s| !s.trim().is_empty()) {
            payload["hwuid"] = serde_json::json!(h.trim());
        } else {
            payload["hwuid"] = serde_json::json!(self.default_hwuid());
        }
        let response = self
            .make_request("POST", "/licenses/verify", Some(payload))
            .await?;

        let api_response: ApiResponse<serde_json::Value> = response.json().await?;
        
        if api_response.success {
            let data = api_response.data.unwrap();
            Ok(data.get("valid").and_then(|v| v.as_bool()).unwrap_or(false))
        } else {
            Err(format!("API Error: {}", api_response.error.unwrap_or("Unknown error".to_string())).into())
        }
    }

    /// Full POST /v1/licenses/verify payload (`valid`, optional `license_token`, `license_jwks_uri`, etc.).
    pub async fn verify_license_with_details(
        &self,
        license_key: &str,
        hwuid: Option<&str>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        let mut payload = serde_json::json!({ "key": license_key });
        if let Some(h) = hwuid.filter(|s| !s.trim().is_empty()) {
            payload["hwuid"] = serde_json::json!(h.trim());
        } else {
            payload["hwuid"] = serde_json::json!(self.default_hwuid());
        }
        let response = self
            .make_request("POST", "/licenses/verify", Some(payload))
            .await?;
        let v: serde_json::Value = response.json().await?;
        if v.get("success").and_then(|s| s.as_bool()) == Some(true) {
            Ok(v.get("data").cloned().unwrap_or(v))
        } else if v.get("success").and_then(|s| s.as_bool()) == Some(false) {
            Err(format!(
                "API Error: {}",
                v.get("error")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown error")
            )
            .into())
        } else {
            Ok(v)
        }
    }

    /// Verify `license_token` from verify response using JWKS URL (RS256).
    pub async fn verify_license_assertion_jwt(
        &self,
        token: &str,
        jwks_url: &str,
        opts: Option<crate::license_assertion::VerifyLicenseAssertionOptions>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        crate::license_assertion::verify_license_assertion_jwt(
            &self.client,
            token,
            jwks_url,
            opts,
        )
        .await
    }

    pub async fn get_user_licenses(
        &self,
        user_id: &str,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<PaginatedResponse<License>, Box<dyn std::error::Error + Send + Sync>> {
        let mut params = vec![("user_id", user_id.to_string())];
        if let Some(p) = page {
            params.push(("page", p.to_string()));
        }
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }

        let response = self
            .make_request_with_params("GET", "/licenses", None, params)
            .await?;

        let api_response: ApiResponse<PaginatedResponse<License>> = response.json().await?;
        
        if api_response.success {
            Ok(api_response.data.unwrap())
        } else {
            Err(format!("API Error: {}", api_response.error.unwrap_or("Unknown error".to_string())).into())
        }
    }

    pub async fn get_license_stats(&self) -> Result<LicenseStats, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .make_request("GET", "/licenses/stats", None)
            .await?;

        let api_response: ApiResponse<LicenseStats> = response.json().await?;
        
        if api_response.success {
            Ok(api_response.data.unwrap())
        } else {
            Err(format!("API Error: {}", api_response.error.unwrap_or("Unknown error".to_string())).into())
        }
    }

    // User Management
    pub async fn create_user(
        &self,
        email: &str,
        name: Option<&str>,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        let _ = (email, name, metadata);
        Err("create_user is not available in API v1".into())
    }

    pub async fn get_user(&self, user_id: &str) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        let _ = user_id;
        Err("get_user is not available in API v1".into())
    }

    pub async fn update_user(
        &self,
        user_id: &str,
        updates: HashMap<String, serde_json::Value>,
    ) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
        let _ = (user_id, updates);
        Err("update_user is not available in API v1".into())
    }

    pub async fn get_user_stats(&self) -> Result<UserStats, Box<dyn std::error::Error + Send + Sync>> {
        Ok(UserStats {
            total: 0,
            active: 0,
            inactive: 0,
            suspended: 0,
        })
    }

    // Product Management
    pub async fn create_product(
        &self,
        name: &str,
        description: Option<&str>,
        price: f64,
        currency: &str,
        metadata: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<Product, Box<dyn std::error::Error + Send + Sync>> {
        let _ = (name, description, price, currency, metadata);
        Err("create_product is not available in API v1".into())
    }

    pub async fn get_product(&self, product_id: &str) -> Result<Product, Box<dyn std::error::Error + Send + Sync>> {
        let _ = product_id;
        Err("get_product is not available in API v1".into())
    }

    pub async fn update_product(
        &self,
        product_id: &str,
        updates: HashMap<String, serde_json::Value>,
    ) -> Result<Product, Box<dyn std::error::Error + Send + Sync>> {
        let _ = (product_id, updates);
        Err("update_product is not available in API v1".into())
    }

    pub async fn get_product_stats(&self) -> Result<ProductStats, Box<dyn std::error::Error + Send + Sync>> {
        Ok(ProductStats {
            total: 0,
            active: 0,
            inactive: 0,
            archived: 0,
            revenue: 0.0,
        })
    }

    // Webhook Management
    pub async fn create_webhook(
        &self,
        url: &str,
        events: Vec<String>,
        secret: Option<&str>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        let mut payload = serde_json::Map::new();
        payload.insert("url".to_string(), serde_json::Value::String(url.to_string()));
        payload.insert("events".to_string(), serde_json::Value::Array(events.into_iter().map(serde_json::Value::String).collect()));
        
        if let Some(s) = secret {
            payload.insert("secret".to_string(), serde_json::Value::String(s.to_string()));
        }

        let response = self
            .make_request("POST", "/webhooks", Some(serde_json::Value::Object(payload)))
            .await?;

        let api_response: ApiResponse<serde_json::Value> = response.json().await?;
        
        if api_response.success {
            Ok(api_response.data.unwrap())
        } else {
            Err(format!("API Error: {}", api_response.error.unwrap_or("Unknown error".to_string())).into())
        }
    }

    pub async fn get_webhooks(&self) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .make_request("GET", "/webhooks", None)
            .await?;

        let api_response: ApiResponse<Vec<serde_json::Value>> = response.json().await?;
        
        if api_response.success {
            Ok(api_response.data.unwrap())
        } else {
            Err(format!("API Error: {}", api_response.error.unwrap_or("Unknown error".to_string())).into())
        }
    }

    pub async fn delete_webhook(&self, webhook_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .make_request("DELETE", &format!("/webhooks/{}", webhook_id), None)
            .await?;

        let api_response: ApiResponse<()> = response.json().await?;
        
        if api_response.success {
            Ok(())
        } else {
            Err(format!("API Error: {}", api_response.error.unwrap_or("Unknown error".to_string())).into())
        }
    }

    // Private helper methods
    async fn make_request(
        &self,
        method: &str,
        endpoint: &str,
        body: Option<serde_json::Value>,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error + Send + Sync>> {
        let normalized_endpoint = normalize_endpoint(&self.config.base_url, endpoint);
        
        let url = format!("{}{}", self.config.base_url, normalized_endpoint);
        
        let mut request = self.client.request(
            method.parse()?,
            &url,
        );

        request = request.header("Authorization", format!("Bearer {}", self.config.api_key));
        request = request.header("Content-Type", "application/json");
        request = request.header("X-API-Version", "1.0");
        request = request.header("X-Platform", "rust-sdk");

        if let Some(b) = body {
            request = request.json(&b);
        }

        let response = timeout(self.config.timeout, request.send()).await??;
        
        if !response.status().is_success() {
            return Err(format!("HTTP Error: {}", response.status()).into());
        }

        Ok(response)
    }

    async fn make_request_with_params(
        &self,
        method: &str,
        endpoint: &str,
        body: Option<serde_json::Value>,
        params: Vec<(&str, String)>,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error + Send + Sync>> {
        let normalized_endpoint = normalize_endpoint(&self.config.base_url, endpoint);
        
        let mut url = format!("{}{}", self.config.base_url, normalized_endpoint);
        
        if !params.is_empty() {
            let query_string = params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            url = format!("{}?{}", url, query_string);
        }
        
        let mut request = self.client.request(
            method.parse()?,
            &url,
        );

        request = request.header("Authorization", format!("Bearer {}", self.config.api_key));
        request = request.header("Content-Type", "application/json");
        request = request.header("X-API-Version", "1.0");
        request = request.header("X-Platform", "rust-sdk");

        if let Some(b) = body {
            request = request.json(&b);
        }

        let response = timeout(self.config.timeout, request.send()).await??;
        
        if !response.status().is_success() {
            return Err(format!("HTTP Error: {}", response.status()).into());
        }

        Ok(response)
    }

    fn default_hwuid(&self) -> String {
        let host = std::env::var("HOSTNAME")
            .or_else(|_| std::env::var("COMPUTERNAME"))
            .unwrap_or_else(|_| "unknown".to_string());
        let raw = format!("licensechain|rust|{}|{}|{}", host, std::env::consts::OS, std::env::consts::ARCH);
        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

fn normalize_endpoint(base_url: &str, endpoint: &str) -> String {
    let base_has_v1 = base_url.ends_with("/v1");
    if endpoint.starts_with("/v1/") {
        if base_has_v1 {
            endpoint.trim_start_matches("/v1").to_string()
        } else {
            endpoint.to_string()
        }
    } else if endpoint.starts_with('/') {
        if base_has_v1 {
            endpoint.to_string()
        } else {
            format!("/v1{}", endpoint)
        }
    } else if base_has_v1 {
        format!("/{}", endpoint)
    } else {
        format!("/v1/{}", endpoint)
    }
}

#[cfg(test)]
mod hwuid_hash_spec_tests {
    use super::*;

    #[test]
    fn default_hwuid_matches_hash_spec() {
        let client = LicenseChainClient::new(LicenseChainConfig::default());
        let h1 = client.default_hwuid();
        let h2 = client.default_hwuid();
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
        assert!(h1.chars().all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
    }
}
