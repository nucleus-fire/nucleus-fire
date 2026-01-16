# Payments Guide

Nucleus Payments provides a simplified wrapper around the Stripe API for common tasks like Checkout and Webhooks.

## Quick Start

### 1. Configuration

Add your Stripe secret key to `nucleus.config`:

```toml
[payments]
stripe_key = "sk_test_..."
```

### 2. Create a Checkout Session

```rust
use nucleus_std::payments::{Stripe, LineItem};

async fn create_checkout() -> Result<String, NucleusError> {
    let url = Stripe::checkout(
        "https://example.com/success",
        "https://example.com/cancel",
        vec![
            LineItem {
                price: Some("price_123".to_string()),
                quantity: 1,
            }
        ],
        "payment", // or "subscription"
        None,      // idempotency key
        Some("user@example.com"),
    ).await?;
    
    Ok(url) // Redirect user to this URL
}
```

## Features

### Customers

create a new customer in Stripe:

```rust
let id = Stripe::create_customer("user@example.com", Some("Alice")).await?;
```

### Subscriptions

Create a subscription for an existing customer:

```rust
let status = Stripe::create_subscription("cus_123", "price_premium").await?;
```

### Webhook Verification

Verify incoming Stripe webhooks securely:

```rust
use nucleus_std::payments::Stripe;

async fn handle_webhook(
    headers: HeaderMap,
    body: String
) -> Result<impl IntoResponse, AppError> {
    let signature = headers.get("Stripe-Signature")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::BadRequest)?;
        
    let secret = std::env::var("STRIPE_WEBHOOK_SECRET")?;
    
    if Stripe::verify_webhook(&body, signature, &secret)? {
        // Process webhook event
        let event: serde_json::Value = serde_json::from_str(&body)?;
        
        match event["type"].as_str() {
            Some("checkout.session.completed") => {
                // Fulfill order
            }
            _ => {}
        }
    }
    
    Ok(())
}
```

## Error Handling

Stripe methods return `Result<T, NucleusError>`. Errors are mapped to `NucleusError::PaymentError` or `NucleusError::NetworkError`.
