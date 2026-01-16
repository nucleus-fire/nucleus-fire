use crate::config::GLOBAL_CONFIG;
use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::Sha256;
use std::collections::HashMap;

pub struct Stripe;

#[derive(Serialize)]
pub struct LineItem {
    pub price: Option<String>,
    pub quantity: u64,
}

use crate::errors::{Result, NucleusError};
use serde::de::Error; // For custom TOML errors

impl Stripe {
    fn get_key() -> Result<String> {
        GLOBAL_CONFIG.payments.as_ref()
            .map(|p| p.stripe_key.clone())
            .ok_or(NucleusError::ConfigError(toml::de::Error::custom("Stripe key not configured in nucleus.config")))
    }

    pub async fn checkout(
        success_url: &str,
        cancel_url: &str,
        line_items: Vec<LineItem>,
        mode: &str, // "payment" or "subscription"
        idempotency_key: Option<&str>,
        customer_email: Option<&str>,
    ) -> Result<String> {
        let api_key = Stripe::get_key()?;
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
            form_data.push((format!("line_items[{}][quantity]", i), item.quantity.to_string()));
        }

        let mut req = client.post("https://api.stripe.com/v1/checkout/sessions")
            .basic_auth(api_key, None::<String>)
            .form(&form_data);
            
        if let Some(key) = idempotency_key {
            req = req.header("Idempotency-Key", key);
        }

        let res = req.send().await?; 
        // reqwest::Error -> NucleusError::NetworkError

        let json: serde_json::Value = res.json().await?;
        
        if let Some(url) = json["url"].as_str() {
            Ok(url.to_string())
        } else {
            Err(NucleusError::PaymentError(format!("Stripe API Error: {}", json)))
        }
    }

    pub async fn create_customer(email: &str, name: Option<&str>) -> Result<String> {
        let api_key = Stripe::get_key()?;
        let client = reqwest::Client::new();
        
        let mut params = Vec::new();
        params.push(("email", email));
        if let Some(n) = name {
            params.push(("name", n));
        }

        let res = client.post("https://api.stripe.com/v1/customers")
            .basic_auth(api_key, None::<String>)
            .form(&params)
            .send()
            .await?;

        let json: serde_json::Value = res.json().await?;
        
        json["id"].as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| NucleusError::PaymentError(format!("Failed to create customer: {:?}", json)))
    }

    pub async fn create_subscription(customer_id: &str, price_id: &str) -> Result<String> {
        let api_key = Stripe::get_key()?;
        let client = reqwest::Client::new();
        
        let params = [
            ("customer", customer_id),
            ("items[0][price]", price_id),
        ];

        let res = client.post("https://api.stripe.com/v1/subscriptions")
            .basic_auth(api_key, None::<String>)
            .form(&params)
            .send()
            .await?;

        let json: serde_json::Value = res.json().await?;
        
        json["status"].as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| NucleusError::PaymentError(format!("Failed to create subscription: {:?}", json)))
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

        let timestamp = parts.get("t").ok_or(NucleusError::PaymentError("Missing timestamp in webhook signature".to_string()))?;
        let signature = parts.get("v1").ok_or(NucleusError::PaymentError("Missing v1 signature in webhook header".to_string()))?;

        let signed_payload = format!("{}.{}", timestamp, payload);
        
        let sig_bytes = hex::decode(signature).map_err(|_| NucleusError::PaymentError("Invalid signature hex".to_string()))?;
        
        type HmacSha256 = Hmac<Sha256>;
        let mut verify_mac = HmacSha256::new_from_slice(secret.as_bytes())
            .map_err(|_| NucleusError::PaymentError("Invalid webhook secret".to_string()))?;
        verify_mac.update(signed_payload.as_bytes());
        
        verify_mac.verify_slice(&sig_bytes).map_err(|_| NucleusError::PaymentError("Signature verification failed".to_string()))?;

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
}
