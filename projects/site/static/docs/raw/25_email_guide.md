# Email Guide

Send emails with Postman - supporting SMTP and AWS SES.

---

## Quick Start

```rust
use nucleus_std::postman::{Postman, Email, EmailProvider};

// Initialize from environment
let postman = Postman::from_env();

// Send an email
let email = Email::new("user@example.com", "Welcome!", "Thanks for signing up!");
postman.send(email).await?;
```

---

## Configuration

### SMTP

```bash
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
SMTP_FROM=noreply@yourapp.com
SMTP_TLS=true
```

### AWS SES

```bash
SES_REGION=us-east-1
SES_FROM=noreply@yourapp.com
AWS_ACCESS_KEY_ID=AKIA...
AWS_SECRET_ACCESS_KEY=...
```

### Mock (Testing)

```bash
EMAIL_PROVIDER=mock
```

---

## Email Builder

```rust
let email = Email::new("to@example.com", "Subject", "Plain text body")
    .html("<h1>Hello!</h1><p>HTML content</p>")
    .from("custom@example.com")
    .reply_to("support@example.com")
    .cc("manager@example.com")
    .bcc("archive@example.com");

postman.send(email).await?;
```

---

## Templates

```rust
let mut postman = Postman::from_env();

// Register template
postman.register_template("welcome", r#"
    Hello {{name}},
    
    Welcome to {{app}}! Your account is ready.
    
    Best,
    The Team
"#);

// Send with template
let mut vars = HashMap::new();
vars.insert("name".to_string(), "Alice".to_string());
vars.insert("app".to_string(), "Nucleus".to_string());

postman.send_template(
    "user@example.com",
    "Welcome to Nucleus!",
    "welcome",
    &vars
).await?;
```

---

## Email Validation

```rust
use nucleus_std::postman::is_valid_email;

if is_valid_email("user@example.com") {
    // Valid email format
}
```

---

## Error Handling

```rust
match postman.send(email).await {
    Ok(result) => {
        println!("Sent! Message ID: {}", result.message_id);
        println!("Provider: {}", result.provider);
    }
    Err(e) => {
        eprintln!("Failed to send: {}", e);
    }
}
```

---

## Testing

```rust
#[tokio::test]
async fn test_email() {
    let postman = Postman::new(EmailProvider::Mock);
    
    let email = Email::new("test@example.com", "Test", "Body");
    let result = postman.send(email).await;
    
    assert!(result.is_ok());
    assert!(result.unwrap().message_id.starts_with("mock_"));
}
```

---

## Provider Comparison

| Feature | SMTP | SES |
|---------|------|-----|
| Setup | Easy | AWS account required |
| Cost | Varies | $0.10/1000 emails |
| Deliverability | Depends on host | High |
| Rate limits | Provider-dependent | 200/sec default |
| Authentication | Username/password | AWS credentials |
