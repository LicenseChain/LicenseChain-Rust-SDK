pub mod client;
pub mod errors;
pub mod license_assertion;
pub mod utils;
pub mod webhook;

pub use client::{LicenseChainClient, LicenseChainConfig, License, User, Product, WebhookEvent, ApiResponse, PaginatedResponse, LicenseStats, UserStats, ProductStats};
pub use license_assertion::{
    verify_license_assertion_jwt, VerifyLicenseAssertionOptions, LICENSE_TOKEN_USE_CLAIM,
};
pub use errors::{LicenseChainError, Result};
pub use utils::*;
pub use webhook::{WebhookHandler, WebhookConfig, WebhookMiddleware, event_types};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com"));
        assert!(!validate_email("invalid-email"));
        assert!(!validate_email(""));
    }

    #[test]
    fn test_validate_license_key() {
        assert!(validate_license_key("ABCDEFGHIJKLMNOPQRSTUVWXYZ012345"));
        assert!(!validate_license_key("invalid-key"));
        assert!(!validate_license_key(""));
    }

    #[test]
    fn test_validate_uuid() {
        assert!(validate_uuid("550e8400-e29b-41d4-a716-446655440000"));
        assert!(!validate_uuid("invalid-uuid"));
        assert!(!validate_uuid(""));
    }

    #[test]
    fn test_validate_amount() {
        assert!(validate_amount(10.0));
        assert!(!validate_amount(0.0));
        assert!(!validate_amount(-5.0));
        assert!(!validate_amount(f64::NAN));
    }

    #[test]
    fn test_validate_currency() {
        assert!(validate_currency("USD"));
        assert!(validate_currency("eur"));
        assert!(!validate_currency("INVALID"));
        assert!(!validate_currency(""));
    }

    #[test]
    fn test_generate_license_key() {
        let key = generate_license_key();
        assert_eq!(key.len(), 32);
        assert!(validate_license_key(&key));
    }

    #[test]
    fn test_generate_uuid() {
        let uuid = generate_uuid();
        assert!(validate_uuid(&uuid));
    }

    #[test]
    fn test_sanitize_input() {
        assert_eq!(sanitize_input("test"), "test");
        assert_eq!(sanitize_input("test<script>"), "test&lt;script&gt;");
        assert_eq!(sanitize_input("test&value"), "test&amp;value");
    }

    #[test]
    fn test_webhook_signature() {
        let payload = "test payload";
        let secret = "test secret";
        
        let signature = create_webhook_signature(payload, secret);
        assert!(verify_webhook_signature(payload, &signature, secret));
        assert!(!verify_webhook_signature(payload, "invalid", secret));
    }

    #[test]
    fn test_validate_pagination() {
        assert_eq!(validate_pagination(Some(1), Some(10)), (1, 10));
        assert_eq!(validate_pagination(Some(0), Some(0)), (1, 1));
        assert_eq!(validate_pagination(Some(5), Some(200)), (5, 100));
        assert_eq!(validate_pagination(None, None), (1, 10));
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
        assert_eq!(format_bytes(500), "500 B");
    }

    #[test]
    fn test_format_duration() {
        let duration = std::time::Duration::from_secs(3661);
        assert_eq!(format_duration(duration), "1h 1m 1s");
    }

    #[test]
    fn test_capitalize_first() {
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first(""), "");
        assert_eq!(capitalize_first("a"), "A");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("helloWorld"), "hello_world");
        assert_eq!(to_snake_case("hello"), "hello");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("hello_world"), "HelloWorld");
        assert_eq!(to_pascal_case("hello"), "Hello");
        assert_eq!(to_pascal_case(""), "");
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("hello", 10), "hello");
        assert_eq!(truncate_string("hello world", 5), "he...");
        assert_eq!(truncate_string("hello", 3), "...");
    }

    #[test]
    fn test_remove_special_chars() {
        assert_eq!(remove_special_chars("hello world!"), "hello world");
        assert_eq!(remove_special_chars("test@example.com"), "testexamplecom");
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("test@example.com"), "test-example-com");
        assert_eq!(slugify("hello_world"), "hello-world");
    }

    #[test]
    fn test_validate_not_empty() {
        assert!(validate_not_empty("hello", "name").is_ok());
        assert!(validate_not_empty("", "name").is_err());
        assert!(validate_not_empty("   ", "name").is_err());
    }

    #[test]
    fn test_validate_positive() {
        assert!(validate_positive(10.0, "amount").is_ok());
        assert!(validate_positive(0.0, "amount").is_err());
        assert!(validate_positive(-5.0, "amount").is_err());
    }

    #[test]
    fn test_validate_range() {
        assert!(validate_range(5.0, 0.0, 10.0, "value").is_ok());
        assert!(validate_range(15.0, 0.0, 10.0, "value").is_err());
        assert!(validate_range(-5.0, 0.0, 10.0, "value").is_err());
    }
}