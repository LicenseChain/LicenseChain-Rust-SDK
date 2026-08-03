use std::fmt;

#[derive(Debug)]
pub enum LicenseChainError {
    NetworkError(String),
    ApiError(String),
    ValidationError(String),
    AuthenticationError(String),
    NotFoundError(String),
    RateLimitError(String),
    TimeoutError(String),
    SerializationError(String),
    DeserializationError(String),
    ConfigurationError(String),
    UnknownError(String),
}

impl fmt::Display for LicenseChainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LicenseChainError::NetworkError(msg) => write!(f, "Network Error: {}", msg),
            LicenseChainError::ApiError(msg) => write!(f, "API Error: {}", msg),
            LicenseChainError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            LicenseChainError::AuthenticationError(msg) => write!(f, "Authentication Error: {}", msg),
            LicenseChainError::NotFoundError(msg) => write!(f, "Not Found Error: {}", msg),
            LicenseChainError::RateLimitError(msg) => write!(f, "Rate Limit Error: {}", msg),
            LicenseChainError::TimeoutError(msg) => write!(f, "Timeout Error: {}", msg),
            LicenseChainError::SerializationError(msg) => write!(f, "Serialization Error: {}", msg),
            LicenseChainError::DeserializationError(msg) => write!(f, "Deserialization Error: {}", msg),
            LicenseChainError::ConfigurationError(msg) => write!(f, "Configuration Error: {}", msg),
            LicenseChainError::UnknownError(msg) => write!(f, "Unknown Error: {}", msg),
        }
    }
}

impl std::error::Error for LicenseChainError {}

impl From<reqwest::Error> for LicenseChainError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            LicenseChainError::TimeoutError(err.to_string())
        } else if err.is_connect() {
            LicenseChainError::NetworkError(err.to_string())
        } else {
            LicenseChainError::NetworkError(err.to_string())
        }
    }
}

impl From<serde_json::Error> for LicenseChainError {
    fn from(err: serde_json::Error) -> Self {
        LicenseChainError::SerializationError(err.to_string())
    }
}

impl From<std::io::Error> for LicenseChainError {
    fn from(err: std::io::Error) -> Self {
        LicenseChainError::UnknownError(err.to_string())
    }
}

impl From<tokio::time::error::Elapsed> for LicenseChainError {
    fn from(_: tokio::time::error::Elapsed) -> Self {
        LicenseChainError::TimeoutError("Request timed out".to_string())
    }
}

impl From<url::ParseError> for LicenseChainError {
    fn from(err: url::ParseError) -> Self {
        LicenseChainError::ConfigurationError(format!("Invalid URL: {}", err))
    }
}

impl From<http::Error> for LicenseChainError {
    fn from(err: http::Error) -> Self {
        LicenseChainError::ConfigurationError(format!("HTTP Error: {}", err))
    }
}

pub type Result<T> = std::result::Result<T, LicenseChainError>;

pub fn map_http_status_to_error(status: u16, message: String) -> LicenseChainError {
    match status {
        400 => LicenseChainError::ValidationError(message),
        401 => LicenseChainError::AuthenticationError(message),
        403 => LicenseChainError::AuthenticationError(message),
        404 => LicenseChainError::NotFoundError(message),
        429 => LicenseChainError::RateLimitError(message),
        500..=599 => LicenseChainError::ApiError(message),
        _ => LicenseChainError::UnknownError(message),
    }
}

pub fn map_reqwest_error(err: reqwest::Error) -> LicenseChainError {
    if err.is_timeout() {
        LicenseChainError::TimeoutError("Request timed out".to_string())
    } else if err.is_connect() {
        LicenseChainError::NetworkError("Failed to connect to server".to_string())
    } else if err.is_decode() {
        LicenseChainError::DeserializationError("Failed to decode response".to_string())
    } else if let Some(status) = err.status() {
        map_http_status_to_error(status.as_u16(), err.to_string())
    } else {
        LicenseChainError::NetworkError(err.to_string())
    }
}
