# Payments Guide

Nucleus Payments provides a production-ready wrapper around the Stripe API for common SaaS billing tasks.

## Quick Start

### 1. Configuration

Add your Stripe secret key to `nucleus.config`:

```toml
[payments]
stripe_key = "${STRIPE_SECRET_KEY}"
currency = "usd"
```

Or set the environment variable directly:

```bash
export STRIPE_SECRET_KEY="sk_live_..." # or sk_test_... for testing
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
        "subscription", // or "payment" for one-time
        None,           // idempotency key
        Some("user@example.com"),
    ).await?;
    
    Ok(url) // Redirect user to this URL
}
```

## Customer Management

### Create Customer

```rust
use nucleus_std::payments::{Stripe, Customer};

let customer: Customer = Stripe::create_customer(
    "user@example.com",
    Some("Alice Smith")
).await?;

println!("Created customer: {}", customer.id);
```

### Get Customer

```rust
let customer = Stripe::get_customer("cus_123").await?;
println!("Email: {:?}", customer.email);
```

## Subscriptions

### Create Subscription

```rust
use nucleus_std::payments::{Stripe, Subscription};

let subscription: Subscription = Stripe::create_subscription(
    "cus_123",
    "price_premium"
).await?;

println!("Status: {}", subscription.status); // "active", "trialing", etc.
```

### Cancel Subscription

```rust
let cancelled = Stripe::cancel_subscription("sub_123").await?;
assert!(cancelled.cancel_at_period_end); // Cancels at end of billing period
```

## Billing Portal

Allow customers to manage their own subscriptions:

```rust
use nucleus_std::payments::{Stripe, PortalSession};

let portal: PortalSession = Stripe::create_portal_session(
    "cus_123",
    "https://example.com/account"
).await?;

// Redirect to portal.url for customer self-service
```

## Prices & Products

### List Available Prices

```rust
use nucleus_std::payments::{Stripe, Price};

let prices: Vec<Price> = Stripe::list_prices(Some(10)).await?;

for price in prices {
    println!("{}: {} {}/{}",
        price.id,
        price.unit_amount.unwrap_or(0) / 100,
        price.currency,
        price.recurring.map(|r| r.interval).unwrap_or_default()
    );
}
```

## Webhook Verification

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
        let event: serde_json::Value = serde_json::from_str(&body)?;
        
        match event["type"].as_str() {
            Some("checkout.session.completed") => {
                // Activate subscription
                let customer_id = event["data"]["object"]["customer"].as_str();
            }
            Some("customer.subscription.deleted") => {
                // Deactivate access
            }
            _ => {}
        }
    }
    
    Ok(())
}
```

## Testing

### Unit Tests (no API key needed)

```bash
cargo test -p nucleus-std payments::tests
```

### Integration Tests (requires Stripe test key)

```bash
export STRIPE_TEST_SECRET_KEY="sk_test_..."
export STRIPE_TEST_PRICE_ID="price_..."  # Optional, for checkout tests
cargo test -p nucleus-std payments::integration_tests -- --nocapture
```

## Error Handling

All methods return `Result<T, NucleusError>`. Stripe errors are parsed and mapped to `NucleusError::PaymentError`:

```rust
match Stripe::get_customer("invalid_id").await {
    Ok(customer) => println!("Found: {}", customer.id),
    Err(e) => eprintln!("Stripe error: {}", e), // "invalid_request_error: No such customer"
}
```

## Types Reference

| Type | Fields |
|------|--------|
| `Customer` | `id`, `email`, `name`, `created` |
| `Price` | `id`, `active`, `currency`, `unit_amount`, `recurring`, `product` |
| `Subscription` | `id`, `customer`, `status`, `current_period_end`, `cancel_at_period_end` |
| `PortalSession` | `id`, `url` |
| `LineItem` | `price`, `quantity` |
