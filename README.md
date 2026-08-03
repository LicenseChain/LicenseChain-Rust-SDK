# LicenseChain Rust SDK

[![License](https://img.shields.io/badge/license-Elastic--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-blue.svg)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/licensechain-sdk)](https://crates.io/crates/licensechain-sdk)
[![Documentation](https://docs.rs/licensechain-sdk/badge.svg)](https://docs.rs/licensechain-sdk)

Official Rust SDK for LicenseChain - Secure license management for Rust applications.

## 🚀 Features

- **🔐 Secure Authentication** - User registration, login, and session management
- **📜 License Management** - Create, validate, update, and revoke licenses
- **🛡️ Hardware ID Validation** - Prevent license sharing and unauthorized access
- **🔔 Webhook Support** - Real-time license events and notifications
- **📊 Analytics Integration** - Track license usage and performance metrics
- **⚡ High Performance** - Optimized for production workloads
- **🔄 Async Operations** - Non-blocking HTTP requests and data processing
- **🛠️ Easy Integration** - Simple API with comprehensive documentation

## 📦 Installation

### Method 1: Cargo (Recommended)

Add to your `Cargo.toml`:

```toml
[dependencies]
licensechain-sdk = "1.0.0"
```

### Method 2: Git Repository

```toml
[dependencies]
licensechain-sdk = { git = "https://github.com/LicenseChain/LicenseChain-Rust-SDK.git" }
```

### Method 3: Local Path

```toml
[dependencies]
licensechain-sdk = { path = "path/to/licensechain-sdk" }
```

## 🚀 Quick Start

### Basic Setup

```rust
use licensechain_sdk::{LicenseChainClient, LicenseChainConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let config = LicenseChainConfig::new()
        .api_key("your-api-key")
        .app_name("your-app-name")
        .version("1.0.0");
        
    let mut client = LicenseChainClient::new(config);
    
    // Connect to LicenseChain
    client.connect().await?;
    println!("Connected to LicenseChain successfully!");
    
    Ok(())
}
```

### User Authentication

```rust
// Register a new user
match client.register("username", "password", "email@example.com").await {
    Ok(user) => {
        println!("User registered successfully!");
        println!("User ID: {}", user.id);
    }
    Err(e) => eprintln!("Registration failed: {}", e),
}

// Login existing user
match client.login("username", "password").await {
    Ok(user) => {
        println!("User logged in successfully!");
        println!("Session ID: {}", user.session_id);
    }
    Err(e) => eprintln!("Login failed: {}", e),
}
```

### License Management

```rust
// Validate a license
match client.validate_license("LICENSE-KEY-HERE").await {
    Ok(license) => {
        println!("License is valid!");
        println!("License Key: {}", license.key);
        println!("Status: {}", license.status);
        println!("Expires: {}", license.expires);
        println!("Features: {:?}", license.features);
        println!("User: {}", license.user);
    }
    Err(e) => eprintln!("License validation failed: {}", e),
}

// Get user's licenses
match client.get_user_licenses().await {
    Ok(licenses) => {
        println!("Found {} licenses:", licenses.len());
        for (i, license) in licenses.iter().enumerate() {
            println!("  {}. {} - {} (Expires: {})", 
                i + 1, license.key, license.status, license.expires);
        }
    }
    Err(e) => eprintln!("Failed to get licenses: {}", e),
}
```

### Hardware ID Validation

```rust
// Get hardware ID (automatically generated)
let hardware_id = client.get_hardware_id();
println!("Hardware ID: {}", hardware_id);

// Validate hardware ID with license
match client.validate_hardware_id("LICENSE-KEY-HERE", &hardware_id).await {
    Ok(is_valid) => {
        if is_valid {
            println!("Hardware ID is valid for this license!");
        } else {
            println!("Hardware ID is not valid for this license.");
        }
    }
    Err(e) => eprintln!("Hardware ID validation failed: {}", e),
}
```

### Webhook Integration

```rust
// Set up webhook handler
client.set_webhook_handler(|event, data| {
    println!("Webhook received: {}", event);
    
    match event.as_str() {
        "license.created" => {
            if let Some(license_key) = data.get("licenseKey") {
                println!("New license created: {}", license_key);
            }
        }
        "license.updated" => {
            if let Some(license_key) = data.get("licenseKey") {
                println!("License updated: {}", license_key);
            }
        }
        "license.revoked" => {
            if let Some(license_key) = data.get("licenseKey") {
                println!("License revoked: {}", license_key);
            }
        }
        _ => {}
    }
});

// Start webhook listener
client.start_webhook_listener().await?;
```

## 🌐 API Endpoints

Use the canonical LicenseChain API base URL `https://api.licensechain.app/v1`. The SDK also accepts the root host and normalizes requests to the same API version.

### Base URL
```
https://api.licensechain.app/v1
```

### Available Endpoints

All endpoints use the `/v1` prefix:

- **Health Check**: `GET /v1/health`
- **Authentication**:
  - `POST /v1/auth/register` - Register new user
  - `POST /v1/auth/login` - User login
  - `GET /v1/auth/me` - Get current user
  - `POST /v1/auth/logout` - User logout
- **Applications**:
  - `GET /v1/apps` - List all apps
  - `GET /v1/apps/:id` - Get app by ID
  - `POST /v1/apps` - Create new app
  - `PUT /v1/apps/:id` - Update app
  - `DELETE /v1/apps/:id` - Delete app
- **Licenses**:
  - `GET /v1/licenses` - List licenses
  - `GET /v1/licenses/:id` - Get license by ID
  - `POST /v1/apps/:id/licenses` - Create license
  - `POST /v1/licenses/verify` - Verify license key
  - `PATCH /v1/licenses/:id` - Update license
  - `PATCH /v1/licenses/:id/revoke` - Revoke license
- **Webhooks**:
  - `GET /v1/webhooks` - List webhooks
  - `POST /v1/webhooks` - Create webhook
  - `GET /v1/webhooks/:id` - Get webhook by ID
  - `PUT /v1/webhooks/:id` - Update webhook
  - `DELETE /v1/webhooks/:id` - Delete webhook
- **Analytics**:
  - `GET /v1/analytics/stats` - Get analytics data

> **Note**: The SDK accepts either the root host or the canonical `/v1` base and normalizes endpoint calls automatically.

## 📚 API Reference

### LicenseChainClient

#### Constructor

```rust
let config = LicenseChainConfig::new()
    .api_key("your-api-key")
    .app_name("your-app-name")
    .version("1.0.0")
    .base_url("https://api.licensechain.app/v1"); // Optional
    
let mut client = LicenseChainClient::new(config);
```

#### Methods

##### Connection Management

```rust
// Connect to LicenseChain
client.connect().await?;

// Disconnect from LicenseChain
client.disconnect().await?;

// Check connection status
let is_connected = client.is_connected();
```

##### User Authentication

```rust
// Register a new user
let user = client.register(username, password, email).await?;

// Login existing user
let user = client.login(username, password).await?;

// Logout current user
client.logout().await?;

// Get current user info
let user = client.get_current_user().await?;
```

##### License Management

```rust
// Validate a license
let license = client.validate_license(license_key).await?;

// Get user's licenses
let licenses = client.get_user_licenses().await?;

// Create a new license
let license = client.create_license(app_id, user_email, metadata).await?;

// Update a license
let license = client.update_license(license_key, updates).await?;

// Revoke a license
client.revoke_license(license_key).await?;

// Extend a license
let license = client.extend_license(license_key, days).await?;
```

##### Hardware ID Management

```rust
// Get hardware ID
let hardware_id = client.get_hardware_id();

// Validate hardware ID
let is_valid = client.validate_hardware_id(license_key, &hardware_id).await?;

// Bind hardware ID to license
client.bind_hardware_id(license_key, &hardware_id).await?;
```

##### Webhook Management

```rust
// Set webhook handler
client.set_webhook_handler(handler);

// Start webhook listener
client.start_webhook_listener().await?;

// Stop webhook listener
client.stop_webhook_listener().await?;
```

##### Analytics

```rust
// Track event
let mut properties = HashMap::new();
properties.insert("level".to_string(), "1".to_string());
properties.insert("playerCount".to_string(), "10".to_string());
client.track_event("app.started", properties).await?;

// Get analytics data
let analytics = client.get_analytics(time_range).await?;
```

## 🔧 Configuration

### Environment Variables

Set these in your environment or through your build process:

```bash
# Required
export LICENSECHAIN_API_KEY=your-api-key
export LICENSECHAIN_APP_NAME=your-app-name
export LICENSECHAIN_APP_VERSION=1.0.0

# Optional
export LICENSECHAIN_BASE_URL=https://api.licensechain.app/v1
export LICENSECHAIN_DEBUG=true
```

### Advanced Configuration

```rust
let config = LicenseChainConfig::new()
    .api_key("your-api-key")
    .app_name("your-app-name")
    .version("1.0.0")
    .base_url("https://api.licensechain.app/v1")
    .timeout(Duration::from_secs(30))  // Request timeout
    .retries(3)                        // Number of retry attempts
    .debug(false)                      // Enable debug logging
    .user_agent("MyApp/1.0.0");       // Custom user agent
```

## 🛡️ Security Features

### Hardware ID Protection

The SDK automatically generates and manages hardware IDs to prevent license sharing:

```rust
// Hardware ID is automatically generated and stored
let hardware_id = client.get_hardware_id();

// Validate against license
let is_valid = client.validate_hardware_id(license_key, &hardware_id).await?;
```

### Secure Communication

- All API requests use HTTPS
- API keys are securely stored and transmitted
- Session tokens are automatically managed
- Webhook signatures are verified

### License Validation

- Real-time license validation
- Hardware ID binding
- Expiration checking
- Feature-based access control

## 📊 Analytics and Monitoring

### Event Tracking

```rust
// Track custom events
let mut properties = HashMap::new();
properties.insert("level".to_string(), "1".to_string());
properties.insert("playerCount".to_string(), "10".to_string());
client.track_event("app.started", properties).await?;

// Track license events
let mut license_properties = HashMap::new();
license_properties.insert("licenseKey".to_string(), "LICENSE-KEY".to_string());
license_properties.insert("features".to_string(), "premium,unlimited".to_string());
client.track_event("license.validated", license_properties).await?;
```

### Performance Monitoring

```rust
// Get performance metrics
let metrics = client.get_performance_metrics().await?;
println!("API Response Time: {}ms", metrics.average_response_time);
println!("Success Rate: {:.2}%", metrics.success_rate * 100.0);
println!("Error Count: {}", metrics.error_count);
```

## 🔄 Error Handling

### Custom Error Types

```rust
match client.validate_license("invalid-key").await {
    Ok(license) => {
        // Handle valid license
    }
    Err(LicenseChainError::InvalidLicense) => {
        eprintln!("License key is invalid");
    }
    Err(LicenseChainError::ExpiredLicense) => {
        eprintln!("License has expired");
    }
    Err(LicenseChainError::NetworkError(e)) => {
        eprintln!("Network connection failed: {}", e);
    }
    Err(e) => {
        eprintln!("LicenseChain error: {}", e);
    }
}
```

### Retry Logic

```rust
// Automatic retry for network errors
let config = LicenseChainConfig::new()
    .api_key("your-api-key")
    .app_name("your-app-name")
    .version("1.0.0")
    .retries(3)                        // Retry up to 3 times
    .timeout(Duration::from_secs(30)); // Wait 30 seconds for each request
```

## 🧪 Testing

### Unit Tests

```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_validate_license
```

### Integration Tests

```bash
# Test with real API
cargo test --test integration_tests
```

## 📝 Examples

See the `examples/` directory:

- `basic_usage.rs` — Basic SDK usage
- `license_assertion_jwks.rs` — `POST /v1/licenses/verify` then RS256 **`license_token`** via JWKS (`cargo run --example license_assertion_jwks`; JWKS at `https://api.licensechain.app/v1/licenses/jwks`)
- `jwks_only.rs` — token + `license_jwks_uri` only (`cargo run --example jwks_only`; [JWKS_EXAMPLE_PRIORITY](https://docs.licensechain.app/))

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

1. Clone the repository
2. Install Rust 1.70 or later
3. Build: `cargo build`
4. Test: `cargo test`

## 📄 License

This project is licensed under the Elastic License 2.0 (ELv2) — see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: [https://docs.licensechain.app/rust](https://docs.licensechain.app/rust)
- **Issues**: [GitHub Issues](https://github.com/LicenseChain/LicenseChain-Rust-SDK/issues)
- **Discord**: [LicenseChain Discord](https://discord.gg/licensechain)
- **Email**: support@licensechain.app

## 🔗 Related Projects

- [LicenseChain JavaScript SDK](https://github.com/LicenseChain/LicenseChain-JavaScript-SDK)
- [LicenseChain Python SDK](https://github.com/LicenseChain/LicenseChain-Python-SDK)
- [LicenseChain Node.js SDK](https://github.com/LicenseChain/LicenseChain-NodeJS-SDK)
---

**Made with ❤️ for the Rust community**

## LicenseChain API (v1)

This SDK targets the **LicenseChain HTTP API v1** implemented by the LicenseChain API service.

- **Production base URL:** https://api.licensechain.app/v1
- **API reference:** [docs.licensechain.app](https://docs.licensechain.app/)
- **Baseline REST mapping (documented for integrators):**
  - GET /health
  - POST /auth/register
  - POST /licenses/verify
  - PATCH /licenses/:id/revoke
  - PATCH /licenses/:id/activate
  - PATCH /licenses/:id/extend
  - GET /analytics/stats

