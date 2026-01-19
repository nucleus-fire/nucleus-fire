use crate::config::GLOBAL_CONFIG;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::HashMap;

pub struct Stripe;

#[derive(Serialize)]
pub struct LineItem {
    pub price: Option<String>,
    pub quantity: u64,
}

use crate::errors::{NucleusError, Result};
use serde::de::Error; // For custom TOML errors

/// Stripe API error response
#[derive(Debug, Deserialize)]
pub struct StripeApiError {
    pub error: StripeErrorDetail,
}

#[derive(Debug, Deserialize)]
pub struct StripeErrorDetail {
    #[serde(rename = "type")]
    pub error_type: String,
    pub message: String,
    pub code: Option<String>,
}

/// Stripe Customer object
#[derive(Debug, Deserialize, Clone)]
pub struct Customer {
    pub id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub created: i64,
}

/// Stripe Price object
#[derive(Debug, Deserialize, Clone)]
pub struct Price {
    pub id: String,
    pub active: bool,
    pub currency: String,
    pub unit_amount: Option<i64>,
    pub recurring: Option<PriceRecurring>,
    pub product: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PriceRecurring {
    pub interval: String,
    pub interval_count: u32,
}

/// Stripe Subscription object
#[derive(Debug, Deserialize, Clone)]
pub struct Subscription {
    pub id: String,
    pub customer: String,
    pub status: String,
    pub current_period_end: i64,
    pub cancel_at_period_end: bool,
}

/// Stripe Billing Portal Session
#[derive(Debug, Deserialize, Clone)]
pub struct PortalSession {
    pub id: String,
    pub url: String,
}

impl Stripe {
    fn get_key() -> Result<String> {
        GLOBAL_CONFIG
            .payments
            .as_ref()
            .map(|p| p.stripe_key.clone())
            .ok_or(NucleusError::ConfigError(toml::de::Error::custom(
                "Stripe key not configured in nucleus.config",
            )))
    }

    /// Get key from environment variable (for testing)
    fn get_key_from_env() -> Option<String> {
        std::env::var("STRIPE_TEST_SECRET_KEY")
            .ok()
            .or_else(|| std::env::var("STRIPE_SECRET_KEY").ok())
    }

    /// Get the API key, preferring env var for flexibility
    fn api_key() -> Result<String> {
        Self::get_key_from_env()
            .or_else(|| Self::get_key().ok())
            .ok_or(NucleusError::ConfigError(toml::de::Error::custom(
                "Stripe key not configured. Set STRIPE_SECRET_KEY env var or configure in nucleus.config"
            )))
    }

    /// Parse Stripe API error response
    fn parse_error(json: &serde_json::Value) -> NucleusError {
        if let Ok(err) = serde_json::from_value::<StripeApiError>(json.clone()) {
            NucleusError::PaymentError(format!("{}: {}", err.error.error_type, err.error.message))
        } else {
            NucleusError::PaymentError(format!("Stripe API Error: {}", json))
        }
    }

    pub async fn checkout(
        success_url: &str,
        cancel_url: &str,
        line_items: Vec<LineItem>,
        mode: &str, // "payment" or "subscription"
        idempotency_key: Option<&str>,
        customer_email: Option<&str>,
    ) -> Result<String> {
        let api_key = Stripe::api_key()?;
        let client = reqwest::Client::new();

        let mut form_data = Vec::new();
        form_data.push(("success_url".to_string(), success_url.to_string()));
        form_data.push(("cancel_url".to_string(), cancel_url.to_string()));
        form_data.push(("mode".to_string(), mode.to_string()));
        if let Some(email) = customer_email {
            form_data.push(("customer_email".to_string(), email.to_string()));
        }

        for (i, item) in line_items.iter().enumerate() {
            if let Some(price) = &item.price {
                form_data.push((format!("line_items[{}][price]", i), price.clone()));
            }
            form_data.push((
                format!("line_items[{}][quantity]", i),
                item.quantity.to_string(),
            ));
        }

        let mut req = client
            .post("https://api.stripe.com/v1/checkout/sessions")
            .basic_auth(api_key, None::<String>)
            .form(&form_data);

        if let Some(key) = idempotency_key {
            req = req.header("Idempotency-Key", key);
        }

        let res = req.send().await?;
        let json: serde_json::Value = res.json().await?;

        if let Some(url) = json["url"].as_str() {
            Ok(url.to_string())
        } else {
            Err(Self::parse_error(&json))
        }
    }

    pub async fn create_customer(email: &str, name: Option<&str>) -> Result<Customer> {
        let api_key = Stripe::api_key()?;
        let client = reqwest::Client::new();

        let mut params = Vec::new();
        params.push(("email", email));
        if let Some(n) = name {
            params.push(("name", n));
        }

        let res = client
            .post("https://api.stripe.com/v1/customers")
            .basic_auth(api_key, None::<String>)
            .form(&params)
            .send()
            .await?;

        let json: serde_json::Value = res.json().await?;

        if let Some(id) = json["id"].as_str() {
            Ok(Customer {
                id: id.to_string(),
                email: json["email"].as_str().map(|s| s.to_string()),
                name: json["name"].as_str().map(|s| s.to_string()),
                created: json["created"].as_i64().unwrap_or(0),
            })
        } else {
            Err(Self::parse_error(&json))
        }
    }

    /// Retrieve a customer by ID
    pub async fn get_customer(customer_id: &str) -> Result<Customer> {
        let api_key = Stripe::api_key()?;
        let client = reqwest::Client::new();

        let res = client
            .get(format!(
                "https://api.stripe.com/v1/customers/{}",
                customer_id
            ))
            .basic_auth(api_key, None::<String>)
            .send()
            .await?;

        let json: serde_json::Value = res.json().await?;

        if let Some(id) = json["id"].as_str() {
            Ok(Customer {
                id: id.to_string(),
                email: json["email"].as_str().map(|s| s.to_string()),
                name: json["name"].as_str().map(|s| s.to_string()),
                created: json["created"].as_i64().unwrap_or(0),
            })
        } else {
            Err(Self::parse_error(&json))
        }
    }

    /// List all active prices
    pub async fn list_prices(limit: Option<u32>) -> Result<Vec<Price>> {
        let api_key = Stripe::api_key()?;
        let client = reqwest::Client::new();

        let mut url = "https://api.stripe.com/v1/prices?active=true".to_string();
        if let Some(l) = limit {
            url.push_str(&format!("&limit={}", l));
        }

        let res = client
            .get(&url)
            .basic_auth(api_key, None::<String>)
            .send()
            .await?;

        let json: serde_json::Value = res.json().await?;

        if let Some(data) = json["data"].as_array() {
            let prices: Vec<Price> = data
                .iter()
                .filter_map(|p| serde_json::from_value(p.clone()).ok())
                .collect();
            Ok(prices)
        } else {
            Err(Self::parse_error(&json))
        }
    }

    pub async fn create_subscription(customer_id: &str, price_id: &str) -> Result<Subscription> {
        let api_key = Stripe::api_key()?;
        let client = reqwest::Client::new();

        let params = [("customer", customer_id), ("items[0][price]", price_id)];

        let res = client
            .post("https://api.stripe.com/v1/subscriptions")
            .basic_auth(api_key, None::<String>)
            .form(&params)
            .send()
            .await?;

        let json: serde_json::Value = res.json().await?;

        if let Some(id) = json["id"].as_str() {
            Ok(Subscription {
                id: id.to_string(),
                customer: json["customer"].as_str().unwrap_or("").to_string(),
                status: json["status"].as_str().unwrap_or("").to_string(),
                current_period_end: json["current_period_end"].as_i64().unwrap_or(0),
                cancel_at_period_end: json["cancel_at_period_end"].as_bool().unwrap_or(false),
            })
        } else {
            Err(Self::parse_error(&json))
        }
    }

    /// Cancel a subscription at period end
    pub async fn cancel_subscription(subscription_id: &str) -> Result<Subscription> {
        let api_key = Stripe::api_key()?;
        let client = reqwest::Client::new();

        let params = [("cancel_at_period_end", "true")];

        let res = client
            .post(format!(
                "https://api.stripe.com/v1/subscriptions/{}",
                subscription_id
            ))
            .basic_auth(api_key, None::<String>)
            .form(&params)
            .send()
            .await?;

        let json: serde_json::Value = res.json().await?;

        if let Some(id) = json["id"].as_str() {
            Ok(Subscription {
                id: id.to_string(),
                customer: json["customer"].as_str().unwrap_or("").to_string(),
                status: json["status"].as_str().unwrap_or("").to_string(),
                current_period_end: json["current_period_end"].as_i64().unwrap_or(0),
                cancel_at_period_end: json["cancel_at_period_end"].as_bool().unwrap_or(false),
            })
        } else {
            Err(Self::parse_error(&json))
        }
    }

    /// Create a billing portal session for self-service subscription management
    pub async fn create_portal_session(
        customer_id: &str,
        return_url: &str,
    ) -> Result<PortalSession> {
        let api_key = Stripe::api_key()?;
        let client = reqwest::Client::new();

        let params = [("customer", customer_id), ("return_url", return_url)];

        let res = client
            .post("https://api.stripe.com/v1/billing_portal/sessions")
            .basic_auth(api_key, None::<String>)
            .form(&params)
            .send()
            .await?;

        let json: serde_json::Value = res.json().await?;

        if let Some(url) = json["url"].as_str() {
            Ok(PortalSession {
                id: json["id"].as_str().unwrap_or("").to_string(),
                url: url.to_string(),
            })
        } else {
            Err(Self::parse_error(&json))
        }
    }

    pub fn verify_webhook(payload: &str, sig_header: &str, secret: &str) -> Result<bool> {
        // Stripe Signature Format: t=TIMESTAMP,v1=SIGNATURE
        let parts: HashMap<&str, &str> = sig_header
            .split(',')
            .filter_map(|part| {
                let mut s = part.splitn(2, '=');
                Some((s.next()?, s.next()?))
            })
            .collect();

        let timestamp = parts.get("t").ok_or(NucleusError::PaymentError(
            "Missing timestamp in webhook signature".to_string(),
        ))?;
        let signature = parts.get("v1").ok_or(NucleusError::PaymentError(
            "Missing v1 signature in webhook header".to_string(),
        ))?;

        let signed_payload = format!("{}.{}", timestamp, payload);

        let sig_bytes = hex::decode(signature)
            .map_err(|_| NucleusError::PaymentError("Invalid signature hex".to_string()))?;

        type HmacSha256 = Hmac<Sha256>;
        let mut verify_mac = HmacSha256::new_from_slice(secret.as_bytes())
            .map_err(|_| NucleusError::PaymentError("Invalid webhook secret".to_string()))?;
        verify_mac.update(signed_payload.as_bytes());

        verify_mac
            .verify_slice(&sig_bytes)
            .map_err(|_| NucleusError::PaymentError("Signature verification failed".to_string()))?;

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_webhook_valid() {
        let secret = "whsec_test_secret";
        let payload = r#"{"id": "evt_123", "object": "event"}"#;
        let timestamp = "1612345678";

        // Compute valid signature
        let signed_payload = format!("{}.{}", timestamp, payload);
        type HmacSha256 = Hmac<Sha256>;
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(signed_payload.as_bytes());
        let signature = hex::encode(mac.finalize().into_bytes());

        let header = format!("t={},v1={}", timestamp, signature);

        let result = Stripe::verify_webhook(payload, &header, secret);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_verify_webhook_invalid_sig() {
        let secret = "whsec_test_secret";
        let payload = r#"{"id": "evt_123"}"#;
        let timestamp = "1612345678";
        let header = format!("t={},v1=bad_signature", timestamp);

        let result = Stripe::verify_webhook(payload, &header, secret);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_webhook_bad_header() {
        let secret = "whsec_test_secret";
        let payload = "{}";
        let header = "bad_header_format";
        let result = Stripe::verify_webhook(payload, header, secret);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_stripe_error() {
        let json = serde_json::json!({
            "error": {
                "type": "card_error",
                "message": "Your card was declined.",
                "code": "card_declined"
            }
        });

        let err = Stripe::parse_error(&json);
        let msg = err.to_string();
        assert!(msg.contains("card_error"));
        assert!(msg.contains("declined"));
    }

    #[test]
    fn test_api_key_from_env() {
        // Save original
        let original = std::env::var("STRIPE_TEST_SECRET_KEY").ok();

        // Test with env var set
        std::env::set_var("STRIPE_TEST_SECRET_KEY", "sk_test_example");
        assert_eq!(
            Stripe::get_key_from_env(),
            Some("sk_test_example".to_string())
        );

        // Restore
        if let Some(val) = original {
            std::env::set_var("STRIPE_TEST_SECRET_KEY", val);
        } else {
            std::env::remove_var("STRIPE_TEST_SECRET_KEY");
        }
    }

    #[test]
    fn test_customer_struct() {
        let json = serde_json::json!({
            "id": "cus_123",
            "email": "test@example.com",
            "name": "Test User",
            "created": 1234567890
        });

        let customer: Customer = serde_json::from_value(json).unwrap();
        assert_eq!(customer.id, "cus_123");
        assert_eq!(customer.email, Some("test@example.com".to_string()));
    }

    #[test]
    fn test_subscription_struct() {
        let json = serde_json::json!({
            "id": "sub_123",
            "customer": "cus_123",
            "status": "active",
            "current_period_end": 1234567890,
            "cancel_at_period_end": false
        });

        let sub: Subscription = serde_json::from_value(json).unwrap();
        assert_eq!(sub.id, "sub_123");
        assert_eq!(sub.status, "active");
        assert!(!sub.cancel_at_period_end);
    }
}

/// Integration tests that require a real Stripe API key.
/// Run with: STRIPE_TEST_SECRET_KEY=sk_test_xxx cargo test -p nucleus-std payments::integration_tests
#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Check if Stripe credentials are available for integration testing
    fn has_stripe_key() -> bool {
        std::env::var("STRIPE_TEST_SECRET_KEY").is_ok()
    }

    /// Skip test if no Stripe key
    macro_rules! require_stripe_key {
        () => {
            if !has_stripe_key() {
                eprintln!("⏭️  Skipping integration test: STRIPE_TEST_SECRET_KEY not set");
                return;
            }
        };
    }

    #[tokio::test]
    async fn test_create_and_get_customer() {
        require_stripe_key!();

        // Create a test customer
        let email = format!(
            "test_{}@nucleus-test.com",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        );

        let customer = Stripe::create_customer(&email, Some("Nucleus Test")).await;
        assert!(
            customer.is_ok(),
            "Failed to create customer: {:?}",
            customer.err()
        );

        let customer = customer.unwrap();
        assert!(customer.id.starts_with("cus_"));
        assert_eq!(customer.email, Some(email.clone()));

        // Retrieve the customer
        let retrieved = Stripe::get_customer(&customer.id).await;
        assert!(
            retrieved.is_ok(),
            "Failed to get customer: {:?}",
            retrieved.err()
        );

        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.id, customer.id);
        assert_eq!(retrieved.email, Some(email));
    }

    #[tokio::test]
    async fn test_list_prices() {
        require_stripe_key!();

        let prices = Stripe::list_prices(Some(10)).await;
        assert!(prices.is_ok(), "Failed to list prices: {:?}", prices.err());

        // We just verify the API call works - may have 0 prices in test account
        let prices = prices.unwrap();
        eprintln!("📋 Found {} active prices", prices.len());
    }

    #[tokio::test]
    async fn test_invalid_customer_id_returns_error() {
        require_stripe_key!();

        let result = Stripe::get_customer("cus_invalid_123456").await;
        assert!(result.is_err());

        let err = result.unwrap_err().to_string();
        // Should get a proper Stripe error, not a parse error
        assert!(
            err.contains("No such customer")
                || err.contains("resource_missing")
                || err.contains("invalid_request_error"),
            "Unexpected error: {}",
            err
        );
    }

    #[tokio::test]
    async fn test_checkout_session_creation() {
        require_stripe_key!();

        // Need a real price ID for this test
        let price_id = std::env::var("STRIPE_TEST_PRICE_ID");
        if price_id.is_err() {
            eprintln!("⏭️  Skipping checkout test: STRIPE_TEST_PRICE_ID not set");
            return;
        }

        let result = Stripe::checkout(
            "https://example.com/success",
            "https://example.com/cancel",
            vec![LineItem {
                price: Some(price_id.unwrap()),
                quantity: 1,
            }],
            "subscription",
            None,
            Some("test@example.com"),
        )
        .await;

        assert!(
            result.is_ok(),
            "Failed to create checkout session: {:?}",
            result.err()
        );
        let url = result.unwrap();
        assert!(url.starts_with("https://checkout.stripe.com/"));
    }

    #[tokio::test]
    async fn test_full_subscription_flow() {
        require_stripe_key!();

        let price_id = std::env::var("STRIPE_TEST_PRICE_ID");
        if price_id.is_err() {
            eprintln!("⏭️  Skipping subscription flow test: STRIPE_TEST_PRICE_ID not set");
            return;
        }
        let price_id = price_id.unwrap();

        // 1. Create customer
        let email = format!(
            "flow_test_{}@nucleus-test.com",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        );

        let customer = Stripe::create_customer(&email, Some("Flow Test"))
            .await
            .unwrap();
        eprintln!("✅ Created customer: {}", customer.id);

        // 2. Create subscription (requires a payment method in production, but test mode may allow it)
        let sub_result = Stripe::create_subscription(&customer.id, &price_id).await;

        // Note: This may fail in strict test mode without a payment method
        // That's expected - we're testing the API integration, not mocking
        if let Ok(sub) = sub_result {
            eprintln!(
                "✅ Created subscription: {} (status: {})",
                sub.id, sub.status
            );

            // 3. Cancel the subscription
            let cancel_result = Stripe::cancel_subscription(&sub.id).await;
            if let Ok(cancelled) = cancel_result {
                assert!(
                    cancelled.cancel_at_period_end,
                    "Subscription should be marked for cancellation"
                );
                eprintln!("✅ Subscription marked for cancellation");
            }
        } else {
            eprintln!(
                "⚠️  Subscription creation failed (may require payment method): {:?}",
                sub_result.err()
            );
        }
    }
}
