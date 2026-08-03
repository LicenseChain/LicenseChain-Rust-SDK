//! Verify a license, then validate `license_token` with JWKS (RS256).
//!
//! ```bash
//! export LICENSECHAIN_API_KEY=...
//! export LICENSECHAIN_LICENSE_KEY=...
//! cargo run --example license_assertion_jwks
//! ```

use licensechain::{LicenseChainClient, LicenseChainConfig};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let api_key = std::env::var("LICENSECHAIN_API_KEY").unwrap_or_default();
    let license_key = std::env::var("LICENSECHAIN_LICENSE_KEY").unwrap_or_default();
    if api_key.is_empty() || license_key.is_empty() {
        eprintln!("Set LICENSECHAIN_API_KEY and LICENSECHAIN_LICENSE_KEY");
        std::process::exit(1);
    }

    let config = LicenseChainConfig {
        api_key,
        base_url: std::env::var("LICENSECHAIN_BASE_URL")
            .unwrap_or_else(|_| "https://api.licensechain.app/v1".to_string()),
        timeout: Duration::from_secs(30),
        retries: 2,
    };
    let client = LicenseChainClient::new(config);

    let details = client
        .verify_license_with_details(&license_key, None)
        .await?;
    let valid = details
        .get("valid")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if !valid {
        eprintln!("License not valid: {details}");
        std::process::exit(1);
    }

    let token = details
        .get("license_token")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let jwks = details
        .get("license_jwks_uri")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if token.is_empty() || jwks.is_empty() {
        println!("No license_token or license_jwks_uri — enable LICENSE_JWT_* on Core API for this seller.");
        return Ok(());
    }

    let claims = client
        .verify_license_assertion_jwt(token, jwks, None)
        .await?;
    println!("verified claims: {claims}");
    Ok(())
}
