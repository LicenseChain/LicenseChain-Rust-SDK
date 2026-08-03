use crate::errors::LicenseChainError;
use std::collections::HashMap;

/// Validates an email address format
pub fn validate_email(email: &str) -> bool {
    let email_regex = regex::Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    email_regex.is_match(email)
}

/// Validates a license key format
pub fn validate_license_key(license_key: &str) -> bool {
    if license_key.len() != 32 {
        return false;
    }
    
    license_key
        .chars()
        .all(|c| c.is_ascii_digit() || (c.is_ascii_alphabetic() && c.is_ascii_uppercase()))
}

/// Validates a UUID format
pub fn validate_uuid(uuid: &str) -> bool {
    let uuid_regex = regex::Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$").unwrap();
    uuid_regex.is_match(uuid)
}

/// Validates an amount (must be positive)
pub fn validate_amount(amount: f64) -> bool {
    amount > 0.0 && amount.is_finite()
}

/// Validates a currency code
pub fn validate_currency(currency: &str) -> bool {
    let valid_currencies = ["USD", "EUR", "GBP", "CAD", "AUD", "JPY", "CHF", "CNY"];
    valid_currencies.contains(&currency.to_uppercase().as_str())
}

/// Validates a status against allowed values
pub fn validate_status(status: &str, allowed_statuses: &[&str]) -> bool {
    allowed_statuses.contains(&status)
}

/// Sanitizes input by escaping HTML characters
pub fn sanitize_input(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Sanitizes a HashMap of string values
pub fn sanitize_metadata(metadata: &mut HashMap<String, serde_json::Value>) {
    for (_, value) in metadata.iter_mut() {
        if let Some(str_value) = value.as_str() {
            *value = serde_json::Value::String(sanitize_input(str_value));
        }
    }
}

/// Generates a random license key
pub fn generate_license_key() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    
    (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Generates a random UUID v4
pub fn generate_uuid() -> String {
    use uuid::Uuid;
    Uuid::new_v4().to_string()
}

/// Formats a timestamp to ISO 8601 string
pub fn format_timestamp(timestamp: i64) -> String {
    use chrono::{DateTime, Utc};
    let dt = DateTime::from_timestamp(timestamp, 0).unwrap_or_else(|| Utc::now());
    dt.to_rfc3339()
}

/// Parses an ISO 8601 timestamp string
pub fn parse_timestamp(timestamp: &str) -> Result<i64, LicenseChainError> {
    use chrono::{DateTime, Utc};
    let dt = DateTime::parse_from_rfc3339(timestamp)
        .map_err(|e| LicenseChainError::ValidationError(format!("Invalid timestamp format: {}", e)))?;
    Ok(dt.timestamp())
}

/// Validates pagination parameters
pub fn validate_pagination(page: Option<u32>, limit: Option<u32>) -> (u32, u32) {
    let page = page.unwrap_or(1).max(1);
    let limit = limit.unwrap_or(10).max(1).min(100);
    (page, limit)
}

/// Validates date range
pub fn validate_date_range(start_date: &str, end_date: &str) -> Result<(), LicenseChainError> {
    let start = parse_timestamp(start_date)?;
    let end = parse_timestamp(end_date)?;
    
    if start > end {
        return Err(LicenseChainError::ValidationError(
            "Start date must be before or equal to end date".to_string()
        ));
    }
    
    Ok(())
}

/// Creates a webhook signature
pub fn create_webhook_signature(payload: &str, secret: &str) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    
    type HmacSha256 = Hmac<Sha256>;
    
    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(payload.as_bytes());
    
    hex::encode(mac.finalize().into_bytes())
}

/// Verifies a webhook signature (HMAC-SHA256). Strips "sha256=" prefix if present. Constant-time.
pub fn verify_webhook_signature(payload: &str, signature: &str, secret: &str) -> bool {
    if payload.is_empty() || signature.is_empty() || secret.is_empty() {
        return false;
    }
    let expected = create_webhook_signature(payload, secret);
    let received = signature.strip_prefix("sha256=").unwrap_or(signature);
    use subtle::ConstantTimeEq;
    let a = hex::decode(expected).unwrap_or_default();
    let b = hex::decode(received).unwrap_or_default();
    a.len() == b.len() && a.ct_eq(&b).into()
}

/// Retries a function with exponential backoff
pub async fn retry_with_backoff<F, T, E>(
    mut operation: F,
    max_retries: u32,
    initial_delay: std::time::Duration,
) -> Result<T, E>
where
    F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send>>,
    E: Clone,
{
    let mut delay = initial_delay;
    
    for attempt in 0..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(err) => {
                if attempt == max_retries {
                    return Err(err);
                }
                
                tokio::time::sleep(delay).await;
                delay = std::time::Duration::from_millis(delay.as_millis() as u64 * 2);
            }
        }
    }
    
    unreachable!()
}

/// Sleeps for a specified duration
pub async fn sleep(duration: std::time::Duration) {
    tokio::time::sleep(duration).await;
}

/// Formats bytes to human readable format
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    const THRESHOLD: u64 = 1024;
    
    if bytes < THRESHOLD {
        return format!("{} B", bytes);
    }
    
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= THRESHOLD as f64 && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD as f64;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}

/// Formats duration to human readable format
pub fn format_duration(duration: std::time::Duration) -> String {
    let total_seconds = duration.as_secs();
    
    if total_seconds < 60 {
        return format!("{}s", total_seconds);
    }
    
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    
    if minutes < 60 {
        return format!("{}m {}s", minutes, seconds);
    }
    
    let hours = minutes / 60;
    let minutes = minutes % 60;
    
    if hours < 24 {
        return format!("{}h {}m {}s", hours, minutes, seconds);
    }
    
    let days = hours / 24;
    let hours = hours % 24;
    
    format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
}

/// Capitalizes the first letter of a string
pub fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Converts a string to snake_case
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c.is_uppercase() && !result.is_empty() {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    
    result
}

/// Converts a string to PascalCase
pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(capitalize_first)
        .collect::<Vec<String>>()
        .join("")
}

/// Truncates a string to a maximum length
pub fn truncate_string(s: &str, max_length: usize) -> String {
    if s.len() <= max_length {
        s.to_string()
    } else {
        format!("{}...", &s[..max_length.saturating_sub(3)])
    }
}

/// Removes special characters from a string
pub fn remove_special_chars(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect()
}

/// Creates a slug from a string
pub fn slugify(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_lowercase().next().unwrap()
            } else if c.is_whitespace() {
                '-'
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}

/// Validates that a string is not empty
pub fn validate_not_empty(s: &str, field_name: &str) -> Result<(), LicenseChainError> {
    if s.trim().is_empty() {
        Err(LicenseChainError::ValidationError(
            format!("{} cannot be empty", field_name)
        ))
    } else {
        Ok(())
    }
}

/// Validates that a number is positive
pub fn validate_positive(n: f64, field_name: &str) -> Result<(), LicenseChainError> {
    if n <= 0.0 {
        Err(LicenseChainError::ValidationError(
            format!("{} must be positive", field_name)
        ))
    } else {
        Ok(())
    }
}

/// Validates that a number is within a range
pub fn validate_range(n: f64, min: f64, max: f64, field_name: &str) -> Result<(), LicenseChainError> {
    if n < min || n > max {
        Err(LicenseChainError::ValidationError(
            format!("{} must be between {} and {}", field_name, min, max)
        ))
    } else {
        Ok(())
    }
}
