use crate::errors::LicenseChainError;
use crate::utils::{create_webhook_signature, verify_webhook_signature};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub id: String,
    pub r#type: String,
    pub data: serde_json::Value,
    pub timestamp: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    pub url: String,
    pub events: Vec<String>,
    pub secret: Option<String>,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct WebhookHandler {
    secret: String,
    tolerance: u64, // seconds
}

impl WebhookHandler {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            tolerance: 300, // 5 minutes default
        }
    }

    pub fn with_tolerance(secret: String, tolerance: u64) -> Self {
        Self { secret, tolerance }
    }

    /// Verifies a webhook signature
    pub fn verify_signature(&self, payload: &str, signature: &str) -> bool {
        verify_webhook_signature(payload, signature, &self.secret)
    }

    /// Verifies a webhook timestamp
    pub fn verify_timestamp(&self, timestamp: &str) -> Result<(), LicenseChainError> {
        use chrono::{DateTime, Utc};
        
        let webhook_time = DateTime::parse_from_rfc3339(timestamp)
            .map_err(|e| LicenseChainError::ValidationError(format!("Invalid timestamp format: {}", e)))?;
        
        let current_time = Utc::now().fixed_offset();
        let time_diff = (current_time - webhook_time).num_seconds().abs() as u64;
        
        if time_diff > self.tolerance {
            return Err(LicenseChainError::ValidationError(
                format!("Webhook timestamp too old: {} seconds", time_diff)
            ));
        }
        
        Ok(())
    }

    /// Verifies a complete webhook
    pub fn verify_webhook(&self, payload: &str, signature: &str, timestamp: &str) -> Result<(), LicenseChainError> {
        self.verify_timestamp(timestamp)?;
        
        if !self.verify_signature(payload, signature) {
            return Err(LicenseChainError::AuthenticationError(
                "Invalid webhook signature".to_string()
            ));
        }
        
        Ok(())
    }

    /// Processes a webhook event
    pub fn process_event(&self, event: WebhookEvent) -> Result<(), LicenseChainError> {
        // Verify the event signature
        let payload = serde_json::to_string(&event.data)
            .map_err(|e| LicenseChainError::SerializationError(e.to_string()))?;
        
        self.verify_webhook(&payload, &event.signature, &event.timestamp)?;
        
        // Process the event based on type
        match event.r#type.as_str() {
            "license.created" => self.handle_license_created(event),
            "license.updated" => self.handle_license_updated(event),
            "license.revoked" => self.handle_license_revoked(event),
            "license.expired" => self.handle_license_expired(event),
            "user.created" => self.handle_user_created(event),
            "user.updated" => self.handle_user_updated(event),
            "user.deleted" => self.handle_user_deleted(event),
            "product.created" => self.handle_product_created(event),
            "product.updated" => self.handle_product_updated(event),
            "product.deleted" => self.handle_product_deleted(event),
            "payment.completed" => self.handle_payment_completed(event),
            "payment.failed" => self.handle_payment_failed(event),
            "payment.refunded" => self.handle_payment_refunded(event),
            _ => {
                log::warn!("Unknown webhook event type: {}", event.r#type);
                Ok(())
            }
        }
    }

    fn handle_license_created(&self, event: WebhookEvent) -> Result<(), LicenseChainError> {
        log::info!("License created: {}", event.id);
        // Add custom logic for license created event
        Ok(())
    }

    fn handle_license_updated(&self, event: WebhookEvent) -> Result<(), LicenseChainError> {
        log::info!("License updated: {}", event.id);
        // Add custom logic for license updated event
        Ok(())
    }

    fn handle_license_revoked(&self, event: WebhookEvent) -> Result<(), LicenseChainError> {
        log::info!("License revoked: {}", event.id);
        // Add custom logic for license revoked event
        Ok(())
    }

    fn handle_license_expired(&self, event: WebhookEvent) -> Result<(), LicenseChainError> {
        log::info!("License expired: {}", event.id);
        // Add custom logic for license expired event
        Ok(())
    }

    fn handle_user_created(&self, event: WebhookEvent) -> Result<(), LicenseChainError> {
        log::info!("User created: {}", event.id);
        // Add custom logic for user created event
        Ok(())
    }

    fn handle_user_updated(&self, event: WebhookEvent) -> Result<(), LicenseChainError> {
        log::info!("User updated: {}", event.id);
        // Add custom logic for user updated event
        Ok(())
    }

    fn handle_user_deleted(&self, event: WebhookEvent) -> Result<(), LicenseChainError> {
        log::info!("User deleted: {}", event.id);
        // Add custom logic for user deleted event
        Ok(())
    }

    fn handle_product_created(&self, event: WebhookEvent) -> Result<(), LicenseChainError> {
        log::info!("Product created: {}", event.id);
        // Add custom logic for product created event
        Ok(())
    }

    fn handle_product_updated(&self, event: WebhookEvent) -> Result<(), LicenseChainError> {
        log::info!("Product updated: {}", event.id);
        // Add custom logic for product updated event
        Ok(())
    }

    fn handle_product_deleted(&self, event: WebhookEvent) -> Result<(), LicenseChainError> {
        log::info!("Product deleted: {}", event.id);
        // Add custom logic for product deleted event
        Ok(())
    }

    fn handle_payment_completed(&self, event: WebhookEvent) -> Result<(), LicenseChainError> {
        log::info!("Payment completed: {}", event.id);
        // Add custom logic for payment completed event
        Ok(())
    }

    fn handle_payment_failed(&self, event: WebhookEvent) -> Result<(), LicenseChainError> {
        log::info!("Payment failed: {}", event.id);
        // Add custom logic for payment failed event
        Ok(())
    }

    fn handle_payment_refunded(&self, event: WebhookEvent) -> Result<(), LicenseChainError> {
        log::info!("Payment refunded: {}", event.id);
        // Add custom logic for payment refunded event
        Ok(())
    }
}

/// Creates a webhook signature for outgoing webhooks
pub fn create_outgoing_webhook_signature(payload: &str, secret: &str) -> String {
    create_webhook_signature(payload, secret)
}

/// Verifies an incoming webhook signature
pub fn verify_incoming_webhook_signature(payload: &str, signature: &str, secret: &str) -> bool {
    verify_webhook_signature(payload, signature, secret)
}

/// Webhook event types
pub mod event_types {
    pub const LICENSE_CREATED: &str = "license.created";
    pub const LICENSE_UPDATED: &str = "license.updated";
    pub const LICENSE_REVOKED: &str = "license.revoked";
    pub const LICENSE_EXPIRED: &str = "license.expired";
    pub const USER_CREATED: &str = "user.created";
    pub const USER_UPDATED: &str = "user.updated";
    pub const USER_DELETED: &str = "user.deleted";
    pub const PRODUCT_CREATED: &str = "product.created";
    pub const PRODUCT_UPDATED: &str = "product.updated";
    pub const PRODUCT_DELETED: &str = "product.deleted";
    pub const PAYMENT_COMPLETED: &str = "payment.completed";
    pub const PAYMENT_FAILED: &str = "payment.failed";
    pub const PAYMENT_REFUNDED: &str = "payment.refunded";
}

/// Webhook event data structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseEventData {
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub license_key: String,
    pub status: String,
    pub created_at: String,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEventData {
    pub id: String,
    pub email: String,
    pub name: Option<String>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductEventData {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub currency: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentEventData {
    pub id: String,
    pub license_id: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    pub created_at: String,
}

/// Webhook middleware for HTTP frameworks
pub struct WebhookMiddleware {
    handler: WebhookHandler,
}

impl WebhookMiddleware {
    pub fn new(secret: String) -> Self {
        Self {
            handler: WebhookHandler::new(secret),
        }
    }

    pub fn with_tolerance(secret: String, tolerance: u64) -> Self {
        Self {
            handler: WebhookHandler::with_tolerance(secret, tolerance),
        }
    }

    /// Processes a webhook request
    pub fn process_request(
        &self,
        payload: &str,
        signature: &str,
        timestamp: &str,
    ) -> Result<WebhookEvent, LicenseChainError> {
        // Verify the webhook
        self.handler.verify_webhook(payload, signature, timestamp)?;
        
        // Parse the event
        let event: WebhookEvent = serde_json::from_str(payload)
            .map_err(|e| LicenseChainError::DeserializationError(e.to_string()))?;
        
        Ok(event)
    }
}
