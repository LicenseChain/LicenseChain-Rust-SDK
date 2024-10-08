use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LicenseResponse {
    pub valid: bool,
    pub message: String,
    pub expiration_date: Option<String>,
}

#[derive(Debug)]
pub struct ValidationError {
    pub status_code: u16,
    pub message: String,
}

impl ValidationError {
    pub async fn from_response(response: reqwest::Response) -> Self {
        let status_code = response.status().as_u16();
        let message = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        ValidationError { status_code, message }
    }
}
