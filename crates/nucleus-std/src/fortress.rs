use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    async_trait,
    extract::{FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::Response,
};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::{HashMap, HashSet};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Permission {
    Read,
    Write,
    Admin,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    pub permissions: HashSet<Permission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub roles: Vec<Role>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CspConfig {
    pub default_src: Vec<String>,
    pub script_src: Vec<String>,
    pub style_src: Vec<String>,
    pub img_src: Vec<String>,
    pub connect_src: Vec<String>,
    pub font_src: Vec<String>,
    pub object_src: Vec<String>,
    pub media_src: Vec<String>,
    pub frame_src: Vec<String>,
    pub sandbox: Option<Vec<String>>,
    pub report_uri: Option<String>,
    pub report_only: bool,
    pub upgrade_insecure_requests: bool,
    pub block_all_mixed_content: bool,
}

impl Default for CspConfig {
    fn default() -> Self {
        Self {
            default_src: vec!["'self'".to_string()],
            // External scripts - unsafe-inline required for inline event handlers
            script_src: vec![
                "'self'".to_string(),
                "'unsafe-inline'".to_string(),
                "https://cdn.jsdelivr.net".to_string(),
            ],
            // External styles - unsafe-inline required for Google Fonts inline styles
            style_src: vec![
                "'self'".to_string(),
                "'unsafe-inline'".to_string(),
                "https://fonts.googleapis.com".to_string(),
            ],
            img_src: vec![
                "'self'".to_string(),
                "data:".to_string(),
                "https:".to_string(),
            ],
            connect_src: vec!["'self'".to_string()],
            font_src: vec![
                "'self'".to_string(),
                "https://fonts.gstatic.com".to_string(),
            ],
            object_src: vec!["'none'".to_string()],
            media_src: vec!["'self'".to_string()],
            frame_src: vec!["'none'".to_string()],
            sandbox: None,
            report_uri: None,
            report_only: false,
            upgrade_insecure_requests: true,
            block_all_mixed_content: true,
        }
    }
}

impl CspConfig {
    pub fn new_strict() -> Self {
        Self::default()
    }

    pub fn new_lax() -> Self {
        let mut csp = Self::default();
        csp.script_src.push("'unsafe-inline'".to_string());
        csp.style_src.push("'unsafe-inline'".to_string());
        csp
    }

    pub fn to_header_value(&self) -> String {
        let mut parts = Vec::new();

        parts.push(format!("default-src {}", self.default_src.join(" ")));
        parts.push(format!("script-src {}", self.script_src.join(" ")));
        parts.push(format!("style-src {}", self.style_src.join(" ")));
        parts.push(format!("img-src {}", self.img_src.join(" ")));
        parts.push(format!("connect-src {}", self.connect_src.join(" ")));
        parts.push(format!("font-src {}", self.font_src.join(" ")));
        parts.push(format!("object-src {}", self.object_src.join(" ")));
        parts.push(format!("media-src {}", self.media_src.join(" ")));
        parts.push(format!("frame-src {}", self.frame_src.join(" ")));

        if let Some(sandbox) = &self.sandbox {
            parts.push(format!("sandbox {}", sandbox.join(" ")));
        }

        if let Some(uri) = &self.report_uri {
            parts.push(format!("report-uri {}", uri));
        }

        if self.upgrade_insecure_requests {
            parts.push("upgrade-insecure-requests".to_string());
        }

        if self.block_all_mixed_content {
            parts.push("block-all-mixed-content".to_string());
        }

        parts.join("; ")
    }
}

pub struct Fortress;

impl Fortress {
    pub fn security_headers(csp: &CspConfig) -> Vec<(String, String)> {
        let mut headers = vec![
            ("X-Content-Type-Options".to_string(), "nosniff".to_string()),
            ("X-Frame-Options".to_string(), "DENY".to_string()),
            ("X-XSS-Protection".to_string(), "1; mode=block".to_string()),
            (
                "Referrer-Policy".to_string(),
                "strict-origin-when-cross-origin".to_string(),
            ),
            (
                "Strict-Transport-Security".to_string(),
                "max-age=63072000; includeSubDomains; preload".to_string(),
            ),
        ];

        let csp_header_name = if csp.report_only {
            "Content-Security-Policy-Report-Only"
        } else {
            "Content-Security-Policy"
        };

        headers.push((csp_header_name.to_string(), csp.to_header_value()));

        headers
    }

    pub fn hash_password(password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| e.to_string())?
            .to_string();
        Ok(password_hash)
    }

    pub fn verify_password(hash: &str, password: &str) -> bool {
        let parsed_hash = match PasswordHash::new(hash) {
            Ok(h) => h,
            Err(_) => return false,
        };
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }

    pub fn generate_token(user_id: &str, secret: &str) -> String {
        let mut mac =
            HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take any size key");
        mac.update(user_id.as_bytes());
        let result = mac.finalize();
        let signature = hex::encode(result.into_bytes());
        let id_hex = hex::encode(user_id.as_bytes());
        format!("{}.{}", id_hex, signature)
    }

    pub fn verify_token(token: &str, user_id: &str, secret: &str) -> bool {
        let parts: Vec<&str> = token.split('.').collect();
        // Support both old (sig only) and new (id.sig) formats
        let token_sig = if parts.len() == 2 {
            // Check ID match first if present
            let token_id_hex = parts[0];
            let expected_id_hex = hex::encode(user_id.as_bytes());
            if token_id_hex != expected_id_hex {
                return false;
            }
            parts[1]
        } else {
            token
        };

        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC error");
        mac.update(user_id.as_bytes());
        let expected_sig = hex::encode(mac.finalize().into_bytes());
        token_sig == expected_sig
    }

    pub fn validate_token(token: &str, secret: &str) -> Result<String, String> {
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 2 {
            return Err("Invalid token format".to_string());
        }

        let id_hex = parts[0];
        let sig = parts[1];

        let user_id_bytes = hex::decode(id_hex).map_err(|_| "Invalid ID encoding".to_string())?;
        let user_id =
            String::from_utf8(user_id_bytes).map_err(|_| "Invalid ID UTF-8".to_string())?;

        // Recompute sig
        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC error");
        mac.update(user_id.as_bytes());
        let expected_sig = hex::encode(mac.finalize().into_bytes());

        if sig != expected_sig {
            return Err("Invalid signature".to_string());
        }

        Ok(user_id)
    }

    pub fn check_permission(user: &User, required_perm: &Permission) -> bool {
        for role in &user.roles {
            if role.permissions.contains(required_perm) {
                return true;
            }
        }
        false
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// RATE LIMITING
// ═══════════════════════════════════════════════════════════════════════════

use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

/// Rate limit key type
#[derive(Debug, Clone)]
pub enum RateLimitKey {
    /// Rate limit by IP address
    Ip,
    /// Rate limit by user ID
    UserId,
    /// Custom key (e.g., API key, endpoint)
    Custom(String),
}

/// Rate limiter configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests allowed in the window
    pub max_requests: u32,
    /// Time window duration
    pub window: Duration,
    /// Key type for rate limiting
    pub key_type: RateLimitKey,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        }
    }
}

impl RateLimitConfig {
    /// Create config for API endpoints (stricter)
    pub fn api() -> Self {
        Self {
            max_requests: 60,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        }
    }

    /// Create config for login attempts (very strict)
    pub fn login() -> Self {
        Self {
            max_requests: 5,
            window: Duration::from_secs(300), // 5 minutes
            key_type: RateLimitKey::Ip,
        }
    }

    /// Create lenient config for general pages
    pub fn lenient() -> Self {
        Self {
            max_requests: 1000,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        }
    }
}

/// Rate limit entry for tracking
#[derive(Debug, Clone)]
struct RateLimitEntry {
    count: u32,
    window_start: Instant,
}

/// Result of rate limit check
#[derive(Debug, Clone)]
pub struct RateLimitResult {
    /// Whether the request is allowed
    pub allowed: bool,
    /// Remaining requests in current window
    pub remaining: u32,
    /// Maximum requests allowed
    pub limit: u32,
    /// When the window resets
    pub reset_at: Instant,
}

impl RateLimitResult {
    /// Get standard rate limit headers (RFC 6585 / draft-ietf-httpapi-ratelimit-headers)
    pub fn headers(&self) -> Vec<(String, String)> {
        let reset_secs = self.reset_at.duration_since(Instant::now()).as_secs();

        let mut headers = vec![
            ("X-RateLimit-Limit".to_string(), self.limit.to_string()),
            (
                "X-RateLimit-Remaining".to_string(),
                self.remaining.to_string(),
            ),
            ("X-RateLimit-Reset".to_string(), reset_secs.to_string()),
        ];

        // Add Retry-After when rate limited (RFC 7231)
        if !self.allowed {
            headers.push(("Retry-After".to_string(), reset_secs.to_string()));
        }

        headers
    }

    /// Get headers as HashMap for easier access
    pub fn headers_map(&self) -> HashMap<String, String> {
        self.headers().into_iter().collect()
    }

    /// Apply rate limit headers to response
    pub fn apply_headers(&self, response: &mut axum::http::Response<axum::body::Body>) {
        use axum::http::HeaderValue;

        let headers = response.headers_mut();
        for (name, value) in self.headers() {
            if let Ok(val) = HeaderValue::from_str(&value) {
                headers.insert(
                    axum::http::HeaderName::from_bytes(name.as_bytes()).unwrap(),
                    val,
                );
            }
        }
    }

    /// Create a 429 Too Many Requests response with proper headers
    pub fn rate_limit_response(&self) -> axum::http::Response<axum::body::Body> {
        use axum::body::Body;
        use axum::http::{Response, StatusCode};

        let reset_secs = self.reset_at.duration_since(Instant::now()).as_secs();

        let body = serde_json::json!({
            "error": "Too Many Requests",
            "message": "Rate limit exceeded. Please retry later.",
            "retry_after": reset_secs,
            "limit": self.limit,
            "remaining": 0
        });

        let mut response = Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .header("Content-Type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();

        self.apply_headers(&mut response);
        response
    }

    /// Get HTTP status code for this result
    pub fn status_code(&self) -> u16 {
        if self.allowed {
            200
        } else {
            429
        }
    }

    /// Check if request should proceed
    pub fn is_allowed(&self) -> bool {
        self.allowed
    }

    /// Get seconds until reset
    pub fn retry_after_secs(&self) -> u64 {
        self.reset_at.duration_since(Instant::now()).as_secs()
    }
}

/// Thread-safe rate limiter using sliding window algorithm
pub struct RateLimiter {
    config: RateLimitConfig,
    entries: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
}

impl RateLimiter {
    /// Create a new rate limiter with the given config
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create rate limiter with default config (100 req/min)
    pub fn default_limiter() -> Self {
        Self::new(RateLimitConfig::default())
    }

    /// Check if a request is allowed and consume one request
    pub fn check(&self, key: &str) -> RateLimitResult {
        let mut entries = self.entries.write().unwrap();
        let now = Instant::now();

        let entry = entries.entry(key.to_string()).or_insert(RateLimitEntry {
            count: 0,
            window_start: now,
        });

        // Check if window has expired
        if now.duration_since(entry.window_start) >= self.config.window {
            // Reset window
            entry.count = 0;
            entry.window_start = now;
        }

        let reset_at = entry.window_start + self.config.window;

        if entry.count >= self.config.max_requests {
            // Rate limited
            RateLimitResult {
                allowed: false,
                remaining: 0,
                limit: self.config.max_requests,
                reset_at,
            }
        } else {
            // Allow and increment
            entry.count += 1;
            RateLimitResult {
                allowed: true,
                remaining: self.config.max_requests - entry.count,
                limit: self.config.max_requests,
                reset_at,
            }
        }
    }

    /// Check without consuming a request (peek)
    pub fn peek(&self, key: &str) -> RateLimitResult {
        let entries = self.entries.read().unwrap();
        let now = Instant::now();

        if let Some(entry) = entries.get(key) {
            if now.duration_since(entry.window_start) >= self.config.window {
                // Window expired, would reset
                RateLimitResult {
                    allowed: true,
                    remaining: self.config.max_requests,
                    limit: self.config.max_requests,
                    reset_at: now + self.config.window,
                }
            } else {
                let reset_at = entry.window_start + self.config.window;
                RateLimitResult {
                    allowed: entry.count < self.config.max_requests,
                    remaining: self.config.max_requests.saturating_sub(entry.count),
                    limit: self.config.max_requests,
                    reset_at,
                }
            }
        } else {
            // No entry yet
            RateLimitResult {
                allowed: true,
                remaining: self.config.max_requests,
                limit: self.config.max_requests,
                reset_at: now + self.config.window,
            }
        }
    }

    /// Get remaining requests for a key
    pub fn remaining(&self, key: &str) -> u32 {
        self.peek(key).remaining
    }

    /// Reset rate limit for a key
    pub fn reset(&self, key: &str) {
        let mut entries = self.entries.write().unwrap();
        entries.remove(key);
    }

    /// Clear all rate limit entries
    pub fn clear(&self) {
        let mut entries = self.entries.write().unwrap();
        entries.clear();
    }

    /// Clean up expired entries (call periodically)
    pub fn cleanup(&self) {
        let mut entries = self.entries.write().unwrap();
        let now = Instant::now();

        entries.retain(|_, entry| now.duration_since(entry.window_start) < self.config.window);
    }
}

impl Clone for RateLimiter {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            entries: Arc::clone(&self.entries),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        let password = "supersecret";
        let hash = Fortress::hash_password(password).unwrap();
        assert!(Fortress::verify_password(&hash, password));
        assert!(!Fortress::verify_password(&hash, "wrongp@ssword"));
    }

    #[test]
    fn test_password_hashing_edge_cases() {
        // Empty password
        let hash = Fortress::hash_password("").unwrap();
        assert!(Fortress::verify_password(&hash, ""));
        assert!(!Fortress::verify_password(&hash, " "));

        // Very long password
        let long_password = "a".repeat(1000);
        let hash = Fortress::hash_password(&long_password).unwrap();
        assert!(Fortress::verify_password(&hash, &long_password));

        // Unicode characters
        let unicode_pass = "пароль密码🔐";
        let hash = Fortress::hash_password(unicode_pass).unwrap();
        assert!(Fortress::verify_password(&hash, unicode_pass));

        // Invalid hash format
        assert!(!Fortress::verify_password("invalid_hash", "password"));
        assert!(!Fortress::verify_password("", "password"));
    }

    #[test]
    fn test_token_gen() {
        let token = Fortress::generate_token("user_123", "secret_key");
        // assert_eq!(token.len(), 64); // Old check
        // New format: hex(id).hex(sig)
        assert!(token.contains('.'));
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[1].len(), 64); // Signature part is still SHA256 hex
    }

    #[test]
    fn test_validate_token() {
        let secret = "secret";
        let user_id = "user_123";
        let token = Fortress::generate_token(user_id, secret);

        // Valid
        let decoded_id = Fortress::validate_token(&token, secret).unwrap();
        assert_eq!(decoded_id, user_id);

        // Invalid sig
        let parts: Vec<&str> = token.split('.').collect();
        let tampered = format!("{}.{}", parts[0], "a".repeat(64));
        assert!(Fortress::validate_token(&tampered, secret).is_err());
    }

    #[test]
    fn test_token_verification() {
        let secret = "my_secret";
        let user_id = "user_456";
        let token = Fortress::generate_token(user_id, secret);

        // Valid token
        assert!(Fortress::verify_token(&token, user_id, secret));

        // Wrong user_id
        assert!(!Fortress::verify_token(&token, "wrong_user", secret));

        // Wrong secret
        assert!(!Fortress::verify_token(&token, user_id, "wrong_secret"));

        // Tampered token
        let mut tampered = token.clone();
        tampered.replace_range(0..1, "0");
        assert!(!Fortress::verify_token(&tampered, user_id, secret));
    }

    #[test]
    fn test_rbac() {
        let mut permissions = HashSet::new();
        permissions.insert(Permission::Read);

        let role = Role {
            name: "Viewer".to_string(),
            permissions,
        };
        let user = User {
            id: "1".into(),
            username: "u".into(),
            password_hash: "".into(),
            roles: vec![role],
        };

        assert!(Fortress::check_permission(&user, &Permission::Read));
        assert!(!Fortress::check_permission(&user, &Permission::Write));
    }

    #[test]
    fn test_rbac_multiple_roles() {
        let mut read_perms = HashSet::new();
        read_perms.insert(Permission::Read);

        let mut write_perms = HashSet::new();
        write_perms.insert(Permission::Write);

        let user = User {
            id: "1".into(),
            username: "u".into(),
            password_hash: "".into(),
            roles: vec![
                Role {
                    name: "Reader".to_string(),
                    permissions: read_perms,
                },
                Role {
                    name: "Writer".to_string(),
                    permissions: write_perms,
                },
            ],
        };

        assert!(Fortress::check_permission(&user, &Permission::Read));
        assert!(Fortress::check_permission(&user, &Permission::Write));
        assert!(!Fortress::check_permission(&user, &Permission::Admin));
    }

    #[test]
    fn test_rbac_custom_permission() {
        let mut perms = HashSet::new();
        perms.insert(Permission::Custom("delete_posts".to_string()));

        let user = User {
            id: "1".into(),
            username: "u".into(),
            password_hash: "".into(),
            roles: vec![Role {
                name: "Moderator".to_string(),
                permissions: perms,
            }],
        };

        assert!(Fortress::check_permission(
            &user,
            &Permission::Custom("delete_posts".to_string())
        ));
        assert!(!Fortress::check_permission(
            &user,
            &Permission::Custom("ban_users".to_string())
        ));
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // RATE LIMITER TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_rate_limiter_basic() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 5,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        });

        // First 5 requests should be allowed
        for i in 0..5 {
            let result = limiter.check("192.168.1.1");
            assert!(result.allowed, "Request {} should be allowed", i + 1);
            assert_eq!(result.remaining, 4 - i as u32);
        }

        // 6th request should be blocked
        let result = limiter.check("192.168.1.1");
        assert!(!result.allowed);
        assert_eq!(result.remaining, 0);
    }

    #[test]
    fn test_rate_limiter_different_keys() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 2,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        });

        // User A uses 2 requests
        assert!(limiter.check("user_a").allowed);
        assert!(limiter.check("user_a").allowed);
        assert!(!limiter.check("user_a").allowed);

        // User B should still have full quota
        assert!(limiter.check("user_b").allowed);
        assert_eq!(limiter.remaining("user_b"), 1);
    }

    #[test]
    fn test_rate_limiter_window_expiration() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 2,
            window: Duration::from_millis(50),
            key_type: RateLimitKey::Ip,
        });

        // Exhaust quota
        limiter.check("key");
        limiter.check("key");
        assert!(!limiter.check("key").allowed);

        // Wait for window to expire
        std::thread::sleep(Duration::from_millis(60));

        // Should be allowed again
        let result = limiter.check("key");
        assert!(result.allowed);
        assert_eq!(result.remaining, 1);
    }

    #[test]
    fn test_rate_limiter_peek_vs_check() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 3,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        });

        // Peek should not consume
        let peek1 = limiter.peek("key");
        assert!(peek1.allowed);
        assert_eq!(peek1.remaining, 3);

        // Peek again - still 3
        let peek2 = limiter.peek("key");
        assert_eq!(peek2.remaining, 3);

        // Check should consume
        let check1 = limiter.check("key");
        assert!(check1.allowed);
        assert_eq!(check1.remaining, 2);

        // Peek now shows 2
        assert_eq!(limiter.peek("key").remaining, 2);
    }

    #[test]
    fn test_rate_limiter_reset() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 2,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        });

        limiter.check("key");
        limiter.check("key");
        assert!(!limiter.check("key").allowed);

        // Reset the key
        limiter.reset("key");

        // Should have full quota again
        assert!(limiter.check("key").allowed);
        assert_eq!(limiter.remaining("key"), 1);
    }

    #[test]
    fn test_rate_limiter_clear() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 1,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        });

        limiter.check("key1");
        limiter.check("key2");
        assert!(!limiter.check("key1").allowed);
        assert!(!limiter.check("key2").allowed);

        // Clear all
        limiter.clear();

        assert!(limiter.check("key1").allowed);
        assert!(limiter.check("key2").allowed);
    }

    #[test]
    fn test_rate_limiter_headers() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 10,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        });

        let result = limiter.check("key");
        let headers = result.headers();

        assert_eq!(headers.len(), 3);
        assert!(headers
            .iter()
            .any(|(k, v)| k == "X-RateLimit-Limit" && v == "10"));
        assert!(headers
            .iter()
            .any(|(k, v)| k == "X-RateLimit-Remaining" && v == "9"));
    }

    #[test]
    fn test_rate_limiter_configs() {
        // API config
        let api = RateLimitConfig::api();
        assert_eq!(api.max_requests, 60);
        assert_eq!(api.window, Duration::from_secs(60));

        // Login config (strict)
        let login = RateLimitConfig::login();
        assert_eq!(login.max_requests, 5);
        assert_eq!(login.window, Duration::from_secs(300));

        // Lenient config
        let lenient = RateLimitConfig::lenient();
        assert_eq!(lenient.max_requests, 1000);
    }

    #[test]
    fn test_rate_limiter_clone_shares_state() {
        let limiter1 = RateLimiter::new(RateLimitConfig {
            max_requests: 3,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        });
        let limiter2 = limiter1.clone();

        // Use some quota via limiter1
        limiter1.check("key");
        limiter1.check("key");

        // limiter2 should see the same state
        assert_eq!(limiter2.remaining("key"), 1);
    }

    #[test]
    fn test_rate_limiter_empty_key() {
        let limiter = RateLimiter::default_limiter();

        // Empty string key should work
        let result = limiter.check("");
        assert!(result.allowed);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // RATE LIMIT HEADERS TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_rate_limit_headers_when_allowed() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 10,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        });

        let result = limiter.check("test_key");
        let headers = result.headers();

        // Should have 3 headers when allowed
        assert_eq!(headers.len(), 3);
        assert!(headers.iter().any(|(k, _)| k == "X-RateLimit-Limit"));
        assert!(headers.iter().any(|(k, _)| k == "X-RateLimit-Remaining"));
        assert!(headers.iter().any(|(k, _)| k == "X-RateLimit-Reset"));
        // Should NOT have Retry-After when allowed
        assert!(!headers.iter().any(|(k, _)| k == "Retry-After"));
    }

    #[test]
    fn test_rate_limit_headers_when_blocked() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 1,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        });

        // Exhaust quota
        limiter.check("key");
        let result = limiter.check("key");

        assert!(!result.allowed);
        let headers = result.headers();

        // Should have 4 headers when blocked (includes Retry-After)
        assert_eq!(headers.len(), 4);
        assert!(headers.iter().any(|(k, _)| k == "Retry-After"));
    }

    #[test]
    fn test_rate_limit_headers_map() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 100,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        });

        let result = limiter.check("key");
        let map = result.headers_map();

        assert_eq!(map.get("X-RateLimit-Limit"), Some(&"100".to_string()));
        assert_eq!(map.get("X-RateLimit-Remaining"), Some(&"99".to_string()));
    }

    #[test]
    fn test_rate_limit_status_code() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 1,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        });

        let result1 = limiter.check("key");
        assert_eq!(result1.status_code(), 200);
        assert!(result1.is_allowed());

        let result2 = limiter.check("key");
        assert_eq!(result2.status_code(), 429);
        assert!(!result2.is_allowed());
    }

    #[test]
    fn test_rate_limit_retry_after() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 1,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        });

        limiter.check("key");
        let result = limiter.check("key");

        let retry = result.retry_after_secs();
        assert!(retry > 0);
        assert!(retry <= 60);
    }

    #[test]
    fn test_rate_limit_remaining_decrements() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 5,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        });

        for i in (0..5).rev() {
            let result = limiter.check("key");
            assert_eq!(result.remaining, i as u32);
        }
    }

    #[test]
    fn test_rate_limit_different_keys_independent() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 5,
            window: Duration::from_secs(60),
            key_type: RateLimitKey::Ip,
        });

        limiter.check("user_1");
        limiter.check("user_1");

        let result_user1 = limiter.peek("user_1");
        let result_user2 = limiter.peek("user_2");

        assert_eq!(result_user1.remaining, 3);
        assert_eq!(result_user2.remaining, 5); // Full quota
    }

    #[test]
    fn test_rate_limit_unicode_keys() {
        let limiter = RateLimiter::default_limiter();

        let result1 = limiter.check("用户123");
        let result2 = limiter.check("🔒🔑");

        assert!(result1.allowed);
        assert!(result2.allowed);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// AUTH EXTRACTORS & MIDDLEWARE
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "Missing Authorization header".to_string(),
            ))?;

        if !auth_header.starts_with("Bearer ") {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Invalid Authorization header".to_string(),
            ));
        }

        let token = &auth_header[7..];
        let secret = std::env::var("SECRET_KEY").map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "SECRET_KEY not set".to_string(),
            )
        })?;

        match Fortress::validate_token(token, &secret) {
            Ok(user_id) => Ok(AuthUser { user_id }),
            Err(_) => Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string())),
        }
    }
}

pub struct OptionalAuth {
    pub user_id: Option<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuth
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok());

        match auth_header {
            Some(header) if header.starts_with("Bearer ") => {
                let token = &header[7..];
                if let Ok(secret) = std::env::var("SECRET_KEY") {
                    if let Ok(user_id) = Fortress::validate_token(token, &secret) {
                        return Ok(OptionalAuth {
                            user_id: Some(user_id),
                        });
                    }
                }
            }
            _ => {}
        }

        Ok(OptionalAuth { user_id: None })
    }
}

pub async fn require_auth(req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|value| value.to_str().ok());

    match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            let token = &header[7..];
            if let Ok(secret) = std::env::var("SECRET_KEY") {
                if Fortress::validate_token(token, &secret).is_ok() {
                    return Ok(next.run(req).await);
                }
            }
        }
        _ => {}
    }

    Err(StatusCode::UNAUTHORIZED)
}
