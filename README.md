# LicenseChain Rust SDK

This SDK allows you to validate licenses using LicenseChain's API in Rust.

## Usage

Add this to your `Cargo.toml`:

```toml
licensechain-rust-sdk = "0.1.0"
```

## Example

```rust
use licensechain_rust_sdk::LicenseChainClient;

#[tokio::main]
async fn main() {
    let client = LicenseChainClient::new("your_api_key".to_string(), "https://licensechain.app/api".to_string());
    match client.validate_license("license_key").await {
        Ok(response) => println!("License is valid: {:?}", response),
        Err(e) => println!("Error validating license: {:?}", e),
    }
}
```

## Features
- Validate license keys
- Fetch license information

# Bugs
If the default example that wasn’t included in your software isn’t working as expected, please pop over to https://t.me/LicenseChainBot and lodge a bug report via the Support Option.

However, we don't offer support for integrating LicenseChain into your project. If you’re having trouble, you might want to have a look on Google or YouTube for tutorials on the programming language you're using to build your programme.

# Copyright License
LicenseChain is under the Elastic License 2.0.

- You’re not allowed to offer the software to third parties as a hosted or managed service, where users get access to any significant part of the software’s features or functionality.
- You mustn’t move, alter, disable, or bypass the licence key functionality in the software, and you can’t remove or hide any features protected by the licence key.
- You’re also not permitted to change, remove, or obscure any licensing, copyright, or other notices from the licensor within the software. Any use of the licensor’s trademarks must comply with relevant laws.

Cheers for sticking to these guidelines. We put a lot of effort into developing LicenseChain and don't take copyright breaches lightly.

## Support

If you have any questions or need help, feel free to open an issue or contact us at support@licensechain.app.
