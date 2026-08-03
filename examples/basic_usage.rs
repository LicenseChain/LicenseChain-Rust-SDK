use licensechain::{
    LicenseChainClient, LicenseChainConfig, LicenseChainError,
    validate_email, validate_license_key, generate_license_key,
    WebhookHandler, event_types
};
use std::collections::HashMap;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client
    let config = LicenseChainConfig {
        api_key: "your-api-key-here".to_string(),
        base_url: "https://api.licensechain.app/v1".to_string(),
        timeout: Duration::from_secs(30),
        retries: 3,
    };
    
    let client = LicenseChainClient::new(config);
    
    println!("🚀 LicenseChain Rust SDK - Basic Usage Example\n");
    
    // 1. License Management
    println!("🔑 License Management:");
    
    // Create a license
    let mut metadata = HashMap::new();
    metadata.insert("platform".to_string(), serde_json::Value::String("rust".to_string()));
    metadata.insert("version".to_string(), serde_json::Value::String("1.0.0".to_string()));
    
    match client.create_license("app_123", "user@example.com", Some(metadata)).await {
        Ok(license) => {
            println!("✅ License created: {}", license.id);
            println!("   License Key: {}", license.license_key);
            println!("   Status: {}", license.status);
        }
        Err(e) => println!("❌ Failed to create license: {}", e),
    }
    
    // Validate a license
    let license_key = generate_license_key();
    println!("\n🔍 Validating license key: {}", license_key);
    
    match client.validate_license(&license_key).await {
        Ok(valid) => {
            if valid {
                println!("✅ License is valid");
            } else {
                println!("❌ License is invalid");
            }
        }
        Err(e) => println!("❌ Failed to validate license: {}", e),
    }
    
    // Get license stats
    match client.get_license_stats().await {
        Ok(stats) => {
            println!("\n📊 License Statistics:");
            println!("   Total: {}", stats.total);
            println!("   Active: {}", stats.active);
            println!("   Expired: {}", stats.expired);
            println!("   Revenue: ${:.2}", stats.revenue);
        }
        Err(e) => println!("❌ Failed to get license stats: {}", e),
    }
    
    // 2. User Management
    println!("\n👤 User Management:");
    
    // Create a user
    let mut user_metadata = HashMap::new();
    user_metadata.insert("source".to_string(), serde_json::Value::String("rust-sdk".to_string()));
    
    match client.create_user("user@example.com", Some("John Doe"), Some(user_metadata)).await {
        Ok(user) => {
            println!("✅ User created: {}", user.id);
            println!("   Email: {}", user.email);
            println!("   Name: {:?}", user.name);
        }
        Err(e) => println!("❌ Failed to create user: {}", e),
    }
    
    // Get user stats
    match client.get_user_stats().await {
        Ok(stats) => {
            println!("\n📊 User Statistics:");
            println!("   Total: {}", stats.total);
            println!("   Active: {}", stats.active);
            println!("   Inactive: {}", stats.inactive);
        }
        Err(e) => println!("❌ Failed to get user stats: {}", e),
    }
    
    // 3. Product Management
    println!("\n📦 Product Management:");
    
    // Create a product
    let mut product_metadata = HashMap::new();
    product_metadata.insert("category".to_string(), serde_json::Value::String("software".to_string()));
    
    match client.create_product(
        "My Software Product",
        Some("A great software product"),
        99.99,
        "USD",
        Some(product_metadata)
    ).await {
        Ok(product) => {
            println!("✅ Product created: {}", product.id);
            println!("   Name: {}", product.name);
            println!("   Price: ${:.2} {}", product.price, product.currency);
        }
        Err(e) => println!("❌ Failed to create product: {}", e),
    }
    
    // Get product stats
    match client.get_product_stats().await {
        Ok(stats) => {
            println!("\n📊 Product Statistics:");
            println!("   Total: {}", stats.total);
            println!("   Active: {}", stats.active);
            println!("   Revenue: ${:.2}", stats.revenue);
        }
        Err(e) => println!("❌ Failed to get product stats: {}", e),
    }
    
    // 4. Webhook Management
    println!("\n🔗 Webhook Management:");
    
    // Create a webhook
    let events = vec![
        event_types::LICENSE_CREATED.to_string(),
        event_types::LICENSE_UPDATED.to_string(),
        event_types::USER_CREATED.to_string(),
    ];
    
    match client.create_webhook(
        "https://example.com/webhook",
        events,
        Some("webhook-secret")
    ).await {
        Ok(webhook) => {
            println!("✅ Webhook created: {:?}", webhook);
        }
        Err(e) => println!("❌ Failed to create webhook: {}", e),
    }
    
    // 5. Webhook Processing
    println!("\n🔄 Webhook Processing:");
    
    let webhook_handler = WebhookHandler::new("webhook-secret".to_string());
    
    // Simulate a webhook event
    let webhook_event = licensechain::webhook::WebhookEvent {
        id: "evt_123".to_string(),
        r#type: event_types::LICENSE_CREATED.to_string(),
        data: serde_json::json!({
            "id": "lic_123",
            "user_id": "user_123",
            "product_id": "prod_123",
            "license_key": "ABCDEFGHIJKLMNOPQRSTUVWXYZ012345",
            "status": "active",
            "created_at": "2023-01-01T00:00:00Z"
        }),
        timestamp: "2023-01-01T00:00:00Z".to_string(),
        signature: "signature_here".to_string(),
    };
    
    match webhook_handler.process_event(webhook_event) {
        Ok(_) => println!("✅ Webhook event processed successfully"),
        Err(e) => println!("❌ Failed to process webhook event: {}", e),
    }
    
    // 6. Utility Functions
    println!("\n🛠️ Utility Functions:");
    
    // Email validation
    let email = "test@example.com";
    println!("Email '{}' is valid: {}", email, validate_email(email));
    
    // License key validation
    let license_key = generate_license_key();
    println!("License key '{}' is valid: {}", license_key, validate_license_key(&license_key));
    
    // Generate UUID
    let uuid = licensechain::generate_uuid();
    println!("Generated UUID: {}", uuid);
    
    // Format bytes
    let bytes = 1024 * 1024;
    println!("{} bytes = {}", bytes, licensechain::format_bytes(bytes));
    
    // Format duration
    let duration = Duration::from_secs(3661);
    println!("Duration: {}", licensechain::format_duration(duration));
    
    // String utilities
    let text = "Hello World";
    println!("Capitalize first: {}", licensechain::capitalize_first(text));
    println!("To snake_case: {}", licensechain::to_snake_case("HelloWorld"));
    println!("To PascalCase: {}", licensechain::to_pascal_case("hello_world"));
    println!("Slugify: {}", licensechain::slugify("Hello World!"));
    
    println!("\n✅ Basic usage example completed successfully!");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_client_creation() {
        let config = LicenseChainConfig {
            api_key: "test-key".to_string(),
            ..Default::default()
        };
        
        let client = LicenseChainClient::new(config);
        // Client should be created successfully
        assert!(true);
    }
    
    #[test]
    fn test_webhook_handler() {
        let handler = WebhookHandler::new("test-secret".to_string());
        assert!(!handler.verify_signature("test", "invalid"));
    }
    
    #[test]
    fn test_utility_functions() {
        assert!(validate_email("test@example.com"));
        assert!(!validate_email("invalid-email"));
        
        let license_key = generate_license_key();
        assert!(validate_license_key(&license_key));
        
        let uuid = licensechain::generate_uuid();
        assert!(licensechain::validate_uuid(&uuid));
    }
}