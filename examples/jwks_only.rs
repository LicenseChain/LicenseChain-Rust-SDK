//! JWKS-only: verify a `license_token` when you already have the token and `license_jwks_uri`
//! (no `verify_license_with_details` call). Matches [JWKS_EXAMPLE_PRIORITY.md](https://docs.licensechain.app/).
//!
//! ```bash
//! export LICENSECHAIN_LICENSE_TOKEN="eyJ..."
//! export LICENSECHAIN_LICENSE_JWKS_URI="https://api.licensechain.app/v1/licenses/jwks"
//! # optional: LICENSECHAIN_EXPECTED_APP_ID=<uuid>
//! cargo run --example jwks_only
//! ```

use licensechain::{verify_license_assertion_jwt, VerifyLicenseAssertionOptions};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let token = std::env::var("LICENSECHAIN_LICENSE_TOKEN").unwrap_or_default();
    let jwks = std::env::var("LICENSECHAIN_LICENSE_JWKS_URI").unwrap_or_default();
    if token.is_empty() || jwks.is_empty() {
        eprintln!("Set LICENSECHAIN_LICENSE_TOKEN and LICENSECHAIN_LICENSE_JWKS_URI");
        std::process::exit(1);
    }

    let http = reqwest::Client::builder()
        .timeout(Duration::from_secs(20))
        .build()?;

    let mut opts = VerifyLicenseAssertionOptions::default();
    if let Ok(app) = std::env::var("LICENSECHAIN_EXPECTED_APP_ID") {
        if !app.is_empty() {
            opts.expected_app_id = Some(app);
        }
    }

    let claims = verify_license_assertion_jwt(&http, &token, &jwks, Some(opts)).await?;
    println!("verified: {claims}");
    Ok(())
}
