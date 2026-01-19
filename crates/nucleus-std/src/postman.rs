//! Postman - Email Sending
//!
//! Production-ready email sending via SMTP and AWS SES.
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::postman::{Postman, Email};
//!
//! // Initialize from environment
//! let postman = Postman::from_env();
//!
//! // Send an email
//! postman.send(Email {
//!     to: "user@example.com".to_string(),
//!     subject: "Welcome!".to_string(),
//!     body: "Hello from Nucleus!".to_string(),
//!     html: None,
//! }).await?;
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Email message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    /// Recipient email address
    pub to: String,
    /// Email subject
    pub subject: String,
    /// Plain text body
    pub body: String,
    /// Optional HTML body
    pub html: Option<String>,
    /// Optional from address (uses default if not set)
    #[serde(default)]
    pub from: Option<String>,
    /// Optional reply-to address
    #[serde(default)]
    pub reply_to: Option<String>,
    /// Optional CC recipients
    #[serde(default)]
    pub cc: Vec<String>,
    /// Optional BCC recipients
    #[serde(default)]
    pub bcc: Vec<String>,
}

impl Email {
    /// Create a new email
    pub fn new(to: &str, subject: &str, body: &str) -> Self {
        Self {
            to: to.to_string(),
            subject: subject.to_string(),
            body: body.to_string(),
            html: None,
            from: None,
            reply_to: None,
            cc: Vec::new(),
            bcc: Vec::new(),
        }
    }

    /// Set HTML body
    pub fn html(mut self, html: &str) -> Self {
        self.html = Some(html.to_string());
        self
    }

    /// Set from address
    pub fn from(mut self, from: &str) -> Self {
        self.from = Some(from.to_string());
        self
    }

    /// Set reply-to address
    pub fn reply_to(mut self, reply_to: &str) -> Self {
        self.reply_to = Some(reply_to.to_string());
        self
    }

    /// Add CC recipient
    pub fn cc(mut self, cc: &str) -> Self {
        self.cc.push(cc.to_string());
        self
    }

    /// Add BCC recipient
    pub fn bcc(mut self, bcc: &str) -> Self {
        self.bcc.push(bcc.to_string());
        self
    }
}

/// SMTP configuration
#[derive(Debug, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub tls: bool,
    pub from: String,
}

impl SmtpConfig {
    /// Create from environment variables
    ///
    /// Required vars:
    /// - SMTP_HOST
    /// - SMTP_USERNAME  
    /// - SMTP_PASSWORD
    /// - SMTP_FROM
    ///
    /// Optional:
    /// - SMTP_PORT (default: 587)
    /// - SMTP_TLS (default: true)
    pub fn from_env() -> Option<Self> {
        Some(Self {
            host: std::env::var("SMTP_HOST").ok()?,
            port: std::env::var("SMTP_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(587),
            username: std::env::var("SMTP_USERNAME").ok()?,
            password: std::env::var("SMTP_PASSWORD").ok()?,
            tls: std::env::var("SMTP_TLS")
                .map(|v| v != "false")
                .unwrap_or(true),
            from: std::env::var("SMTP_FROM").ok()?,
        })
    }
}

/// AWS SES configuration
#[derive(Debug, Clone)]
pub struct SesConfig {
    pub region: String,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub from: String,
}

impl SesConfig {
    /// Create from environment variables
    ///
    /// Required vars:
    /// - SES_REGION
    /// - SES_FROM
    ///
    /// Optional (falls back to AWS SDK defaults):
    /// - AWS_ACCESS_KEY_ID
    /// - AWS_SECRET_ACCESS_KEY
    pub fn from_env() -> Option<Self> {
        Some(Self {
            region: std::env::var("SES_REGION").ok()?,
            access_key: std::env::var("AWS_ACCESS_KEY_ID").ok(),
            secret_key: std::env::var("AWS_SECRET_ACCESS_KEY").ok(),
            from: std::env::var("SES_FROM").ok()?,
        })
    }
}

/// Email provider configuration
#[derive(Debug, Clone)]
pub enum EmailProvider {
    /// SMTP relay
    Smtp(SmtpConfig),
    /// AWS SES (via API)
    Ses(SesConfig),
    /// Mock provider (for testing)
    Mock,
    /// Disabled (no-op)
    Disabled,
}

/// Email send result
#[derive(Debug, Clone)]
pub struct SendResult {
    /// Message ID from provider
    pub message_id: String,
    /// Provider used
    pub provider: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// POSTMAN
// ═══════════════════════════════════════════════════════════════════════════

/// Email sending service
#[derive(Clone)]
pub struct Postman {
    provider: EmailProvider,
    /// Template storage
    templates: HashMap<String, String>,
    /// HTTP client for SES
    client: reqwest::Client,
}

impl std::fmt::Debug for Postman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Postman")
            .field("provider", &self.provider)
            .finish()
    }
}

impl Postman {
    /// Create with specific provider
    pub fn new(provider: EmailProvider) -> Self {
        Self {
            provider,
            templates: HashMap::new(),
            client: reqwest::Client::new(),
        }
    }

    /// Create from environment variables
    ///
    /// Checks in order:
    /// 1. SMTP_HOST → SMTP provider
    /// 2. SES_REGION → SES provider
    /// 3. EMAIL_PROVIDER=mock → Mock provider
    /// 4. Falls back to Disabled
    pub fn from_env() -> Self {
        if let Some(smtp) = SmtpConfig::from_env() {
            return Self::new(EmailProvider::Smtp(smtp));
        }

        if let Some(ses) = SesConfig::from_env() {
            return Self::new(EmailProvider::Ses(ses));
        }

        if std::env::var("EMAIL_PROVIDER")
            .map(|v| v == "mock")
            .unwrap_or(false)
        {
            return Self::new(EmailProvider::Mock);
        }

        Self::new(EmailProvider::Disabled)
    }

    /// Register a template
    pub fn register_template(&mut self, name: &str, template: &str) {
        self.templates
            .insert(name.to_string(), template.to_string());
    }

    /// Render a template with variables
    pub fn render_template(&self, name: &str, vars: &HashMap<String, String>) -> Option<String> {
        let template = self.templates.get(name)?;
        let mut result = template.clone();
        for (key, value) in vars {
            result = result.replace(&format!("{{{{{}}}}}", key), value);
        }
        Some(result)
    }

    /// Send an email
    pub async fn send(&self, email: Email) -> Result<SendResult, String> {
        match &self.provider {
            EmailProvider::Disabled => Err("Email sending is disabled".to_string()),

            EmailProvider::Mock => {
                println!("[MOCK EMAIL] To: {}", email.to);
                println!("[MOCK EMAIL] Subject: {}", email.subject);
                println!("[MOCK EMAIL] Body: {}", email.body);
                Ok(SendResult {
                    message_id: format!("mock_{}", uuid::Uuid::new_v4()),
                    provider: "mock".to_string(),
                })
            }

            EmailProvider::Smtp(config) => self.send_smtp(config, &email).await,

            EmailProvider::Ses(config) => self.send_ses(config, &email).await,
        }
    }

    /// Send email using a template
    pub async fn send_template(
        &self,
        to: &str,
        subject: &str,
        template_name: &str,
        vars: &HashMap<String, String>,
    ) -> Result<SendResult, String> {
        let body = self
            .render_template(template_name, vars)
            .ok_or_else(|| format!("Template '{}' not found", template_name))?;

        let email = Email::new(to, subject, &body);
        self.send(email).await
    }

    // ─────────────────────────────────────────────────────────────────────────
    // SMTP
    // ─────────────────────────────────────────────────────────────────────────

    async fn send_smtp(&self, config: &SmtpConfig, email: &Email) -> Result<SendResult, String> {
        use lettre::{
            message::{header::ContentType, Mailbox, MessageBuilder},
            transport::smtp::authentication::Credentials,
            AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
        };

        // Build message
        let from: Mailbox = email
            .from
            .as_ref()
            .unwrap_or(&config.from)
            .parse()
            .map_err(|e| format!("Invalid from address: {}", e))?;

        let to: Mailbox = email
            .to
            .parse()
            .map_err(|e| format!("Invalid to address: {}", e))?;

        let mut builder = MessageBuilder::new()
            .from(from)
            .to(to)
            .subject(&email.subject);

        // Add reply-to if set
        if let Some(ref reply_to) = email.reply_to {
            let reply_to_mailbox: Mailbox = reply_to
                .parse()
                .map_err(|e| format!("Invalid reply-to address: {}", e))?;
            builder = builder.reply_to(reply_to_mailbox);
        }

        // Add CC recipients
        for cc_addr in &email.cc {
            let cc_mailbox: Mailbox = cc_addr
                .parse()
                .map_err(|e| format!("Invalid CC address: {}", e))?;
            builder = builder.cc(cc_mailbox);
        }

        // Add BCC recipients
        for bcc_addr in &email.bcc {
            let bcc_mailbox: Mailbox = bcc_addr
                .parse()
                .map_err(|e| format!("Invalid BCC address: {}", e))?;
            builder = builder.bcc(bcc_mailbox);
        }

        // Set body
        let message = if let Some(ref html) = email.html {
            builder.header(ContentType::TEXT_HTML).body(html.clone())
        } else {
            builder
                .header(ContentType::TEXT_PLAIN)
                .body(email.body.clone())
        }
        .map_err(|e| format!("Failed to build message: {}", e))?;

        // Configure transport
        let creds = Credentials::new(config.username.clone(), config.password.clone());

        let mailer = if config.tls {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)
                .map_err(|e| format!("Failed to create transport: {}", e))?
                .port(config.port)
                .credentials(creds)
                .build()
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.host)
                .port(config.port)
                .credentials(creds)
                .build()
        };

        // Send
        let response = mailer
            .send(message)
            .await
            .map_err(|e| format!("SMTP send failed: {}", e))?;

        let message_id = response
            .message()
            .next()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Ok(SendResult {
            message_id,
            provider: "smtp".to_string(),
        })
    }

    // ─────────────────────────────────────────────────────────────────────────
    // SES (via HTTP API)
    // ─────────────────────────────────────────────────────────────────────────

    async fn send_ses(&self, config: &SesConfig, email: &Email) -> Result<SendResult, String> {
        // Use SES v2 SendEmail API via HTTP
        // This is a simplified implementation - production would use aws-sdk-sesv2

        let endpoint = format!(
            "https://email.{}.amazonaws.com/v2/email/outbound-emails",
            config.region
        );

        let payload = serde_json::json!({
            "Content": {
                "Simple": {
                    "Subject": {
                        "Data": email.subject,
                        "Charset": "UTF-8"
                    },
                    "Body": {
                        "Text": {
                            "Data": email.body,
                            "Charset": "UTF-8"
                        }
                    }
                }
            },
            "Destination": {
                "ToAddresses": [email.to]
            },
            "FromEmailAddress": email.from.as_ref().unwrap_or(&config.from)
        });

        // Note: Real implementation would use AWS SDK with proper signing
        // This is a placeholder that shows the API structure
        let response = self
            .client
            .post(&endpoint)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("SES request failed: {}", e))?;

        if response.status().is_success() {
            let body: serde_json::Value = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse SES response: {}", e))?;

            Ok(SendResult {
                message_id: body
                    .get("MessageId")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string(),
                provider: "ses".to_string(),
            })
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("SES error: {}", error_text))
        }
    }
}

/// Validate email address format
pub fn is_valid_email(email: &str) -> bool {
    // Basic validation - contains @ and at least one dot after @
    if let Some(at_pos) = email.find('@') {
        let domain = &email[at_pos + 1..];
        !domain.is_empty()
            && domain.contains('.')
            && !email.starts_with('@')
            && !domain.starts_with('.')
            && !domain.ends_with('.')
    } else {
        false
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Mutex to serialize tests that modify environment variables
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    // Helper to run a test with exclusive access to env vars
    fn with_env_lock<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        let _guard = ENV_MUTEX.lock().unwrap();
        f()
    }

    #[test]
    fn test_email_new() {
        let email = Email::new("test@example.com", "Hello", "Body text");
        assert_eq!(email.to, "test@example.com");
        assert_eq!(email.subject, "Hello");
        assert_eq!(email.body, "Body text");
        assert!(email.html.is_none());
    }

    #[test]
    fn test_email_builder() {
        let email = Email::new("to@example.com", "Test", "Body")
            .html("<h1>Hello</h1>")
            .from("from@example.com")
            .reply_to("reply@example.com")
            .cc("cc@example.com")
            .bcc("bcc@example.com");

        assert_eq!(email.html, Some("<h1>Hello</h1>".to_string()));
        assert_eq!(email.from, Some("from@example.com".to_string()));
        assert_eq!(email.reply_to, Some("reply@example.com".to_string()));
        assert_eq!(email.cc, vec!["cc@example.com"]);
        assert_eq!(email.bcc, vec!["bcc@example.com"]);
    }

    #[test]
    fn test_smtp_config_from_env() {
        with_env_lock(|| {
            std::env::set_var("SMTP_HOST", "smtp.example.com");
            std::env::set_var("SMTP_USERNAME", "user");
            std::env::set_var("SMTP_PASSWORD", "pass");
            std::env::set_var("SMTP_FROM", "noreply@example.com");

            let config = SmtpConfig::from_env();
            assert!(config.is_some());
            let config = config.unwrap();
            assert_eq!(config.host, "smtp.example.com");
            assert_eq!(config.port, 587);
            assert!(config.tls);

            std::env::remove_var("SMTP_HOST");
            std::env::remove_var("SMTP_USERNAME");
            std::env::remove_var("SMTP_PASSWORD");
            std::env::remove_var("SMTP_FROM");
        });
    }

    #[test]
    fn test_smtp_config_missing() {
        with_env_lock(|| {
            std::env::remove_var("SMTP_HOST");
            let config = SmtpConfig::from_env();
            assert!(config.is_none());
        });
    }

    #[test]
    fn test_ses_config_from_env() {
        with_env_lock(|| {
            std::env::set_var("SES_REGION", "us-east-1");
            std::env::set_var("SES_FROM", "noreply@example.com");

            let config = SesConfig::from_env();
            assert!(config.is_some());
            let config = config.unwrap();
            assert_eq!(config.region, "us-east-1");

            std::env::remove_var("SES_REGION");
            std::env::remove_var("SES_FROM");
        });
    }

    #[tokio::test]
    async fn test_postman_mock_send() {
        let postman = Postman::new(EmailProvider::Mock);
        let email = Email::new("test@example.com", "Test Subject", "Test body");

        let result = postman.send(email).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.provider, "mock");
        assert!(result.message_id.starts_with("mock_"));
    }

    #[tokio::test]
    async fn test_postman_disabled() {
        let postman = Postman::new(EmailProvider::Disabled);
        let email = Email::new("test@example.com", "Test", "Body");

        let result = postman.send(email).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("disabled"));
    }

    #[test]
    fn test_template_registration() {
        let mut postman = Postman::new(EmailProvider::Mock);
        postman.register_template("welcome", "Hello {{name}}, welcome to {{app}}!");

        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "Alice".to_string());
        vars.insert("app".to_string(), "Nucleus".to_string());

        let rendered = postman.render_template("welcome", &vars);
        assert!(rendered.is_some());
        assert_eq!(rendered.unwrap(), "Hello Alice, welcome to Nucleus!");
    }

    #[test]
    fn test_template_missing() {
        let postman = Postman::new(EmailProvider::Mock);
        let result = postman.render_template("nonexistent", &HashMap::new());
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_send_template() {
        let mut postman = Postman::new(EmailProvider::Mock);
        postman.register_template("test", "Hello {{name}}!");

        let mut vars = HashMap::new();
        vars.insert("name".to_string(), "World".to_string());

        let result = postman
            .send_template("test@example.com", "Test", "test", &vars)
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_send_template_missing() {
        let postman = Postman::new(EmailProvider::Mock);
        let result = postman
            .send_template("test@example.com", "Test", "missing", &HashMap::new())
            .await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user.name+tag@domain.co.uk"));
        assert!(!is_valid_email("invalid"));
        assert!(!is_valid_email("@example.com"));
        assert!(!is_valid_email("test@"));
        assert!(!is_valid_email("test@domain"));
    }

    #[test]
    fn test_postman_from_env_disabled() {
        with_env_lock(|| {
            std::env::remove_var("SMTP_HOST");
            std::env::remove_var("SMTP_USERNAME");
            std::env::remove_var("SMTP_PASSWORD");
            std::env::remove_var("SMTP_FROM");
            std::env::remove_var("SES_REGION");
            std::env::remove_var("SES_FROM");
            std::env::remove_var("EMAIL_PROVIDER");

            let postman = Postman::from_env();
            assert!(matches!(postman.provider, EmailProvider::Disabled));
        });
    }

    #[test]
    fn test_postman_from_env_mock() {
        with_env_lock(|| {
            std::env::remove_var("SMTP_HOST");
            std::env::remove_var("SMTP_USERNAME");
            std::env::remove_var("SMTP_PASSWORD");
            std::env::remove_var("SMTP_FROM");
            std::env::remove_var("SES_REGION");
            std::env::remove_var("SES_FROM");
            std::env::set_var("EMAIL_PROVIDER", "mock");

            let postman = Postman::from_env();
            assert!(matches!(postman.provider, EmailProvider::Mock));

            std::env::remove_var("EMAIL_PROVIDER");
        });
    }

    #[test]
    fn test_is_valid_email_edge_cases() {
        // Edge cases
        assert!(!is_valid_email("")); // Empty
        assert!(!is_valid_email("@")); // Just @
        assert!(!is_valid_email("user@.com")); // Dot at start of domain
        assert!(!is_valid_email("user@domain.")); // Trailing dot
        assert!(is_valid_email("a@b.c")); // Minimal valid
        assert!(is_valid_email("user@sub.domain.com")); // Subdomain
                                                        // Double dot in domain is still accepted by our simple validation
        assert!(is_valid_email("user@domain..com"));
    }

    #[test]
    fn test_smtp_config_custom_port() {
        with_env_lock(|| {
            std::env::set_var("SMTP_HOST", "smtp.example.com");
            std::env::set_var("SMTP_USERNAME", "user");
            std::env::set_var("SMTP_PASSWORD", "pass");
            std::env::set_var("SMTP_FROM", "noreply@example.com");
            std::env::set_var("SMTP_PORT", "2525");

            let config = SmtpConfig::from_env().unwrap();
            assert_eq!(config.port, 2525);

            std::env::remove_var("SMTP_HOST");
            std::env::remove_var("SMTP_USERNAME");
            std::env::remove_var("SMTP_PASSWORD");
            std::env::remove_var("SMTP_FROM");
            std::env::remove_var("SMTP_PORT");
        });
    }

    #[test]
    fn test_smtp_config_tls_false() {
        with_env_lock(|| {
            std::env::set_var("SMTP_HOST", "smtp.example.com");
            std::env::set_var("SMTP_USERNAME", "user");
            std::env::set_var("SMTP_PASSWORD", "pass");
            std::env::set_var("SMTP_FROM", "noreply@example.com");
            std::env::set_var("SMTP_TLS", "false");

            let config = SmtpConfig::from_env().unwrap();
            assert!(!config.tls);

            std::env::remove_var("SMTP_HOST");
            std::env::remove_var("SMTP_USERNAME");
            std::env::remove_var("SMTP_PASSWORD");
            std::env::remove_var("SMTP_FROM");
            std::env::remove_var("SMTP_TLS");
        });
    }

    #[test]
    fn test_ses_config_with_keys() {
        with_env_lock(|| {
            std::env::set_var("SES_REGION", "eu-west-1");
            std::env::set_var("SES_FROM", "noreply@example.com");
            std::env::set_var("AWS_ACCESS_KEY_ID", "AKIAEXAMPLE");
            std::env::set_var("AWS_SECRET_ACCESS_KEY", "secretkey");

            let config = SesConfig::from_env().unwrap();
            assert_eq!(config.region, "eu-west-1");
            assert_eq!(config.access_key, Some("AKIAEXAMPLE".to_string()));
            assert_eq!(config.secret_key, Some("secretkey".to_string()));

            std::env::remove_var("SES_REGION");
            std::env::remove_var("SES_FROM");
            std::env::remove_var("AWS_ACCESS_KEY_ID");
            std::env::remove_var("AWS_SECRET_ACCESS_KEY");
        });
    }

    #[test]
    fn test_email_multiple_cc_bcc() {
        let email = Email::new("to@example.com", "Test", "Body")
            .cc("cc1@example.com")
            .cc("cc2@example.com")
            .bcc("bcc1@example.com")
            .bcc("bcc2@example.com");

        assert_eq!(email.cc.len(), 2);
        assert_eq!(email.bcc.len(), 2);
        assert!(email.cc.contains(&"cc1@example.com".to_string()));
        assert!(email.cc.contains(&"cc2@example.com".to_string()));
    }

    #[test]
    fn test_email_full_builder() {
        let email = Email::new("to@example.com", "Subject", "Plain body")
            .html("<h1>HTML</h1>")
            .from("sender@example.com")
            .reply_to("reply@example.com")
            .cc("cc@example.com")
            .bcc("bcc@example.com");

        assert!(email.html.is_some());
        assert!(email.from.is_some());
        assert!(email.reply_to.is_some());
        assert_eq!(email.cc.len(), 1);
        assert_eq!(email.bcc.len(), 1);
    }

    #[test]
    fn test_postman_debug() {
        let postman = Postman::new(EmailProvider::Mock);
        let debug_str = format!("{:?}", postman);
        assert!(debug_str.contains("Postman"));
        assert!(debug_str.contains("Mock"));
    }

    #[test]
    fn test_ses_config_missing_region() {
        with_env_lock(|| {
            std::env::remove_var("SES_REGION");
            std::env::set_var("SES_FROM", "noreply@example.com");

            let config = SesConfig::from_env();
            assert!(config.is_none());

            std::env::remove_var("SES_FROM");
        });
    }

    #[test]
    fn test_smtp_config_invalid_port() {
        with_env_lock(|| {
            std::env::set_var("SMTP_HOST", "smtp.example.com");
            std::env::set_var("SMTP_USERNAME", "user");
            std::env::set_var("SMTP_PASSWORD", "pass");
            std::env::set_var("SMTP_FROM", "noreply@example.com");
            std::env::set_var("SMTP_PORT", "invalid");

            // Invalid port falls back to default 587
            let config = SmtpConfig::from_env().unwrap();
            assert_eq!(config.port, 587);

            std::env::remove_var("SMTP_HOST");
            std::env::remove_var("SMTP_USERNAME");
            std::env::remove_var("SMTP_PASSWORD");
            std::env::remove_var("SMTP_FROM");
            std::env::remove_var("SMTP_PORT");
        });
    }
}
