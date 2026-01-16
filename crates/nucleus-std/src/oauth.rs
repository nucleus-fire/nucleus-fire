//! Nucleus OAuth2 / Social Login Module
//!
//! Provides easy, secure social login integration with major providers.
//!
//! # Supported Providers
//! - Google
//! - GitHub
//! - Discord
//! - Apple
//! - Facebook
//! - Microsoft
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::oauth::{OAuthConfig, OAuthProvider, OAuth};
//!
//! let config = OAuthConfig::from_env();
//! let oauth = OAuth::new(config);
//!
//! // Generate login URL
//! let (url, state) = oauth.authorize_url(OAuthProvider::Google);
//!
//! // Handle callback
//! let user = oauth.exchange_code(OAuthProvider::Google, &code, &state).await?;
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════
// PROVIDERS
// ═══════════════════════════════════════════════════════════════════════════

/// Supported OAuth providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OAuthProvider {
    Google,
    GitHub,
    Discord,
    Apple,
    Facebook,
    Microsoft,
}

impl OAuthProvider {
    /// Get the authorization endpoint URL
    pub fn auth_url(&self) -> &'static str {
        match self {
            OAuthProvider::Google => "https://accounts.google.com/o/oauth2/v2/auth",
            OAuthProvider::GitHub => "https://github.com/login/oauth/authorize",
            OAuthProvider::Discord => "https://discord.com/api/oauth2/authorize",
            OAuthProvider::Apple => "https://appleid.apple.com/auth/authorize",
            OAuthProvider::Facebook => "https://www.facebook.com/v18.0/dialog/oauth",
            OAuthProvider::Microsoft => "https://login.microsoftonline.com/common/oauth2/v2.0/authorize",
        }
    }
    
    /// Get the token exchange endpoint URL
    pub fn token_url(&self) -> &'static str {
        match self {
            OAuthProvider::Google => "https://oauth2.googleapis.com/token",
            OAuthProvider::GitHub => "https://github.com/login/oauth/access_token",
            OAuthProvider::Discord => "https://discord.com/api/oauth2/token",
            OAuthProvider::Apple => "https://appleid.apple.com/auth/token",
            OAuthProvider::Facebook => "https://graph.facebook.com/v18.0/oauth/access_token",
            OAuthProvider::Microsoft => "https://login.microsoftonline.com/common/oauth2/v2.0/token",
        }
    }
    
    /// Get the user info endpoint URL
    pub fn userinfo_url(&self) -> &'static str {
        match self {
            OAuthProvider::Google => "https://www.googleapis.com/oauth2/v3/userinfo",
            OAuthProvider::GitHub => "https://api.github.com/user",
            OAuthProvider::Discord => "https://discord.com/api/users/@me",
            OAuthProvider::Apple => "", // Apple returns user info in ID token
            OAuthProvider::Facebook => "https://graph.facebook.com/me?fields=id,name,email,picture",
            OAuthProvider::Microsoft => "https://graph.microsoft.com/v1.0/me",
        }
    }
    
    /// Get default scopes for the provider
    pub fn default_scopes(&self) -> &'static str {
        match self {
            OAuthProvider::Google => "openid email profile",
            OAuthProvider::GitHub => "read:user user:email",
            OAuthProvider::Discord => "identify email",
            OAuthProvider::Apple => "name email",
            OAuthProvider::Facebook => "email public_profile",
            OAuthProvider::Microsoft => "openid email profile User.Read",
        }
    }
    
    /// Get provider display name
    pub fn display_name(&self) -> &'static str {
        match self {
            OAuthProvider::Google => "Google",
            OAuthProvider::GitHub => "GitHub",
            OAuthProvider::Discord => "Discord",
            OAuthProvider::Apple => "Apple",
            OAuthProvider::Facebook => "Facebook",
            OAuthProvider::Microsoft => "Microsoft",
        }
    }
    
    /// Get provider icon (for UI)
    pub fn icon(&self) -> &'static str {
        match self {
            OAuthProvider::Google => "🔵",
            OAuthProvider::GitHub => "⬛",
            OAuthProvider::Discord => "💜",
            OAuthProvider::Apple => "🍎",
            OAuthProvider::Facebook => "📘",
            OAuthProvider::Microsoft => "🪟",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

/// OAuth provider credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub client_id: String,
    pub client_secret: String,
    pub scopes: Option<String>,
    pub enabled: bool,
}

/// Complete OAuth configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OAuthConfig {
    pub redirect_uri: String,
    pub google: Option<ProviderConfig>,
    pub github: Option<ProviderConfig>,
    pub discord: Option<ProviderConfig>,
    pub apple: Option<ProviderConfig>,
    pub facebook: Option<ProviderConfig>,
    pub microsoft: Option<ProviderConfig>,
}

impl OAuthConfig {
    /// Create config from environment variables
    ///
    /// Expected env vars:
    /// - OAUTH_REDIRECT_URI
    /// - GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET
    /// - GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET
    /// - DISCORD_CLIENT_ID, DISCORD_CLIENT_SECRET
    /// - APPLE_CLIENT_ID, APPLE_CLIENT_SECRET
    /// - FACEBOOK_CLIENT_ID, FACEBOOK_CLIENT_SECRET
    /// - MICROSOFT_CLIENT_ID, MICROSOFT_CLIENT_SECRET
    pub fn from_env() -> Self {
        let get_env = |key: &str| std::env::var(key).ok();
        
        let mut config = Self {
            redirect_uri: get_env("OAUTH_REDIRECT_URI")
                .unwrap_or_else(|| "http://localhost:3000/auth/callback".to_string()),
            ..Default::default()
        };
        
        // Google
        if let (Some(id), Some(secret)) = (get_env("GOOGLE_CLIENT_ID"), get_env("GOOGLE_CLIENT_SECRET")) {
            config.google = Some(ProviderConfig {
                client_id: id,
                client_secret: secret,
                scopes: get_env("GOOGLE_SCOPES"),
                enabled: true,
            });
        }
        
        // GitHub
        if let (Some(id), Some(secret)) = (get_env("GITHUB_CLIENT_ID"), get_env("GITHUB_CLIENT_SECRET")) {
            config.github = Some(ProviderConfig {
                client_id: id,
                client_secret: secret,
                scopes: get_env("GITHUB_SCOPES"),
                enabled: true,
            });
        }
        
        // Discord
        if let (Some(id), Some(secret)) = (get_env("DISCORD_CLIENT_ID"), get_env("DISCORD_CLIENT_SECRET")) {
            config.discord = Some(ProviderConfig {
                client_id: id,
                client_secret: secret,
                scopes: get_env("DISCORD_SCOPES"),
                enabled: true,
            });
        }
        
        // Apple
        if let (Some(id), Some(secret)) = (get_env("APPLE_CLIENT_ID"), get_env("APPLE_CLIENT_SECRET")) {
            config.apple = Some(ProviderConfig {
                client_id: id,
                client_secret: secret,
                scopes: get_env("APPLE_SCOPES"),
                enabled: true,
            });
        }
        
        // Facebook
        if let (Some(id), Some(secret)) = (get_env("FACEBOOK_CLIENT_ID"), get_env("FACEBOOK_CLIENT_SECRET")) {
            config.facebook = Some(ProviderConfig {
                client_id: id,
                client_secret: secret,
                scopes: get_env("FACEBOOK_SCOPES"),
                enabled: true,
            });
        }
        
        // Microsoft
        if let (Some(id), Some(secret)) = (get_env("MICROSOFT_CLIENT_ID"), get_env("MICROSOFT_CLIENT_SECRET")) {
            config.microsoft = Some(ProviderConfig {
                client_id: id,
                client_secret: secret,
                scopes: get_env("MICROSOFT_SCOPES"),
                enabled: true,
            });
        }
        
        config
    }
    
    /// Get config for a specific provider
    pub fn get_provider(&self, provider: OAuthProvider) -> Option<&ProviderConfig> {
        match provider {
            OAuthProvider::Google => self.google.as_ref(),
            OAuthProvider::GitHub => self.github.as_ref(),
            OAuthProvider::Discord => self.discord.as_ref(),
            OAuthProvider::Apple => self.apple.as_ref(),
            OAuthProvider::Facebook => self.facebook.as_ref(),
            OAuthProvider::Microsoft => self.microsoft.as_ref(),
        }
    }
    
    /// Get list of enabled providers
    pub fn enabled_providers(&self) -> Vec<OAuthProvider> {
        let mut providers = Vec::new();
        if self.google.as_ref().map(|p| p.enabled).unwrap_or(false) {
            providers.push(OAuthProvider::Google);
        }
        if self.github.as_ref().map(|p| p.enabled).unwrap_or(false) {
            providers.push(OAuthProvider::GitHub);
        }
        if self.discord.as_ref().map(|p| p.enabled).unwrap_or(false) {
            providers.push(OAuthProvider::Discord);
        }
        if self.apple.as_ref().map(|p| p.enabled).unwrap_or(false) {
            providers.push(OAuthProvider::Apple);
        }
        if self.facebook.as_ref().map(|p| p.enabled).unwrap_or(false) {
            providers.push(OAuthProvider::Facebook);
        }
        if self.microsoft.as_ref().map(|p| p.enabled).unwrap_or(false) {
            providers.push(OAuthProvider::Microsoft);
        }
        providers
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// OAUTH CLIENT
// ═══════════════════════════════════════════════════════════════════════════

/// OAuth client for handling social login flows
pub struct OAuth {
    config: OAuthConfig,
}

/// User information returned from OAuth provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthUser {
    pub provider: String,
    pub provider_id: String,
    pub email: Option<String>,
    pub name: Option<String>,
    pub avatar: Option<String>,
    pub raw: HashMap<String, serde_json::Value>,
}

/// Token response from OAuth provider
#[derive(Debug, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: Option<String>,
    pub expires_in: Option<i64>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
}

impl OAuth {
    /// Create a new OAuth client
    pub fn new(config: OAuthConfig) -> Self {
        Self { config }
    }
    
    /// Generate a secure random state parameter
    pub fn generate_state() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let bytes: [u8; 32] = rng.gen();
        hex::encode(bytes)
    }
    
    /// Generate authorization URL for a provider
    ///
    /// Returns (url, state) - store the state in session for verification
    pub fn authorize_url(&self, provider: OAuthProvider) -> Result<(String, String), String> {
        let provider_config = self.config.get_provider(provider)
            .ok_or_else(|| format!("{} is not configured", provider.display_name()))?;
        
        if !provider_config.enabled {
            return Err(format!("{} is not enabled", provider.display_name()));
        }
        
        let state = Self::generate_state();
        let scopes = provider_config.scopes.as_deref()
            .unwrap_or(provider.default_scopes());
        
        let callback_uri = format!("{}/{}", 
            self.config.redirect_uri.trim_end_matches('/'),
            format!("{:?}", provider).to_lowercase()
        );
        
        let mut params = vec![
            ("client_id", provider_config.client_id.as_str()),
            ("redirect_uri", &callback_uri),
            ("response_type", "code"),
            ("scope", scopes),
            ("state", &state),
        ];
        
        // Provider-specific params
        match provider {
            OAuthProvider::Google => {
                params.push(("access_type", "offline"));
                params.push(("prompt", "select_account"));
            }
            OAuthProvider::Discord => {
                params.push(("prompt", "consent"));
            }
            OAuthProvider::Apple => {
                params.push(("response_mode", "form_post"));
            }
            _ => {}
        }
        
        let query = params.iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");
        
        let url = format!("{}?{}", provider.auth_url(), query);
        
        Ok((url, state))
    }
    
    /// Exchange authorization code for tokens and user info
    pub async fn exchange_code(
        &self,
        provider: OAuthProvider,
        code: &str,
        state: &str,
        expected_state: &str,
    ) -> Result<OAuthUser, String> {
        // Verify state
        if state != expected_state {
            return Err("Invalid state parameter - possible CSRF attack".to_string());
        }
        
        let provider_config = self.config.get_provider(provider)
            .ok_or_else(|| format!("{} is not configured", provider.display_name()))?;
        
        let callback_uri = format!("{}/{}", 
            self.config.redirect_uri.trim_end_matches('/'),
            format!("{:?}", provider).to_lowercase()
        );
        
        // Exchange code for token
        let client = reqwest::Client::new();
        
        let token_response = client
            .post(provider.token_url())
            .header("Accept", "application/json")
            .form(&[
                ("client_id", provider_config.client_id.as_str()),
                ("client_secret", provider_config.client_secret.as_str()),
                ("code", code),
                ("redirect_uri", &callback_uri),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await
            .map_err(|e| format!("Token request failed: {}", e))?;
        
        if !token_response.status().is_success() {
            let error_text = token_response.text().await.unwrap_or_default();
            return Err(format!("Token exchange failed: {}", error_text));
        }
        
        let tokens: TokenResponse = token_response
            .json()
            .await
            .map_err(|e| format!("Failed to parse token response: {}", e))?;
        
        // Get user info
        self.get_user_info(provider, &tokens.access_token).await
    }
    
    /// Get user information from the provider
    async fn get_user_info(&self, provider: OAuthProvider, access_token: &str) -> Result<OAuthUser, String> {
        let client = reqwest::Client::new();
        
        let userinfo_url = provider.userinfo_url();
        if userinfo_url.is_empty() {
            return Err("Provider does not support userinfo endpoint".to_string());
        }
        
        let mut request = client.get(userinfo_url);
        
        // Provider-specific auth headers
        match provider {
            OAuthProvider::GitHub => {
                request = request.header("Authorization", format!("token {}", access_token));
                request = request.header("User-Agent", "Nucleus-App");
            }
            _ => {
                request = request.header("Authorization", format!("Bearer {}", access_token));
            }
        }
        
        let response = request
            .send()
            .await
            .map_err(|e| format!("User info request failed: {}", e))?;
        
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("User info request failed: {}", error_text));
        }
        
        let raw: HashMap<String, serde_json::Value> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse user info: {}", e))?;
        
        // Normalize user data across providers
        let user = match provider {
            OAuthProvider::Google => OAuthUser {
                provider: "google".to_string(),
                provider_id: raw.get("sub").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                email: raw.get("email").and_then(|v| v.as_str()).map(String::from),
                name: raw.get("name").and_then(|v| v.as_str()).map(String::from),
                avatar: raw.get("picture").and_then(|v| v.as_str()).map(String::from),
                raw,
            },
            OAuthProvider::GitHub => OAuthUser {
                provider: "github".to_string(),
                provider_id: raw.get("id").and_then(|v| v.as_i64()).map(|v| v.to_string()).unwrap_or_default(),
                email: raw.get("email").and_then(|v| v.as_str()).map(String::from),
                name: raw.get("name").and_then(|v| v.as_str()).map(String::from),
                avatar: raw.get("avatar_url").and_then(|v| v.as_str()).map(String::from),
                raw,
            },
            OAuthProvider::Discord => OAuthUser {
                provider: "discord".to_string(),
                provider_id: raw.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                email: raw.get("email").and_then(|v| v.as_str()).map(String::from),
                name: raw.get("username").and_then(|v| v.as_str()).map(String::from),
                avatar: raw.get("avatar").and_then(|v| v.as_str()).map(|a| {
                    let id = raw.get("id").and_then(|v| v.as_str()).unwrap_or("");
                    format!("https://cdn.discordapp.com/avatars/{}/{}.png", id, a)
                }),
                raw,
            },
            OAuthProvider::Facebook => OAuthUser {
                provider: "facebook".to_string(),
                provider_id: raw.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                email: raw.get("email").and_then(|v| v.as_str()).map(String::from),
                name: raw.get("name").and_then(|v| v.as_str()).map(String::from),
                avatar: raw.get("picture")
                    .and_then(|v| v.get("data"))
                    .and_then(|v| v.get("url"))
                    .and_then(|v| v.as_str())
                    .map(String::from),
                raw,
            },
            OAuthProvider::Microsoft => OAuthUser {
                provider: "microsoft".to_string(),
                provider_id: raw.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                email: raw.get("mail").or(raw.get("userPrincipalName")).and_then(|v| v.as_str()).map(String::from),
                name: raw.get("displayName").and_then(|v| v.as_str()).map(String::from),
                avatar: None, // Microsoft Graph requires separate call for photo
                raw,
            },
            OAuthProvider::Apple => OAuthUser {
                provider: "apple".to_string(),
                provider_id: raw.get("sub").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                email: raw.get("email").and_then(|v| v.as_str()).map(String::from),
                name: None, // Apple provides name only on first login
                avatar: None,
                raw,
            },
        };
        
        Ok(user)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HTML HELPERS
// ═══════════════════════════════════════════════════════════════════════════

/// Generate HTML for social login buttons
pub fn render_social_buttons(config: &OAuthConfig, class: Option<&str>) -> String {
    let providers = config.enabled_providers();
    if providers.is_empty() {
        return String::new();
    }
    
    let button_class = class.unwrap_or("social-login-btn");
    let mut html = String::from("<div class=\"social-login-buttons\">\n");
    
    for provider in providers {
        html.push_str(&format!(
            r#"  <a href="/auth/{}" class="{} social-{}">{} Continue with {}</a>
"#,
            format!("{:?}", provider).to_lowercase(),
            button_class,
            format!("{:?}", provider).to_lowercase(),
            provider.icon(),
            provider.display_name()
        ));
    }
    
    html.push_str("</div>\n");
    html
}

/// Generate default CSS for social login buttons
pub fn social_buttons_css() -> &'static str {
    r#"
.social-login-buttons {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    width: 100%;
    max-width: 320px;
}

.social-login-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    padding: 0.75rem 1rem;
    border: 1px solid #e2e8f0;
    border-radius: 0.5rem;
    font-weight: 500;
    text-decoration: none;
    transition: all 0.2s;
}

.social-login-btn:hover {
    background: #f8fafc;
    border-color: #cbd5e1;
}

.social-google { background: #fff; color: #1a1a2e; }
.social-github { background: #24292e; color: #fff; border-color: #24292e; }
.social-github:hover { background: #1b1f23; }
.social-discord { background: #5865F2; color: #fff; border-color: #5865F2; }
.social-discord:hover { background: #4752c4; }
.social-apple { background: #000; color: #fff; border-color: #000; }
.social-facebook { background: #1877f2; color: #fff; border-color: #1877f2; }
.social-facebook:hover { background: #166fe5; }
.social-microsoft { background: #00a4ef; color: #fff; border-color: #00a4ef; }
.social-microsoft:hover { background: #0095d9; }
"#
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    // ─────────────────────────────────────────────────────────────────────────
    // PROVIDER TESTS
    // ─────────────────────────────────────────────────────────────────────────
    
    #[test]
    fn test_provider_auth_urls() {
        assert!(OAuthProvider::Google.auth_url().contains("accounts.google.com"));
        assert!(OAuthProvider::GitHub.auth_url().contains("github.com"));
        assert!(OAuthProvider::Discord.auth_url().contains("discord.com"));
        assert!(OAuthProvider::Apple.auth_url().contains("appleid.apple.com"));
        assert!(OAuthProvider::Facebook.auth_url().contains("facebook.com"));
        assert!(OAuthProvider::Microsoft.auth_url().contains("microsoftonline.com"));
    }
    
    #[test]
    fn test_provider_token_urls() {
        assert!(OAuthProvider::Google.token_url().contains("googleapis.com"));
        assert!(OAuthProvider::GitHub.token_url().contains("github.com"));
        assert!(OAuthProvider::Discord.token_url().contains("discord.com"));
        assert!(OAuthProvider::Apple.token_url().contains("appleid.apple.com"));
        assert!(OAuthProvider::Facebook.token_url().contains("graph.facebook.com"));
        assert!(OAuthProvider::Microsoft.token_url().contains("microsoftonline.com"));
    }
    
    #[test]
    fn test_provider_userinfo_urls() {
        assert!(OAuthProvider::Google.userinfo_url().contains("googleapis.com"));
        assert!(OAuthProvider::GitHub.userinfo_url().contains("api.github.com"));
        assert!(OAuthProvider::Discord.userinfo_url().contains("discord.com"));
        assert!(OAuthProvider::Apple.userinfo_url().is_empty()); // Apple uses ID token
        assert!(OAuthProvider::Facebook.userinfo_url().contains("graph.facebook.com"));
        assert!(OAuthProvider::Microsoft.userinfo_url().contains("graph.microsoft.com"));
    }
    
    #[test]
    fn test_provider_default_scopes() {
        assert!(OAuthProvider::Google.default_scopes().contains("email"));
        assert!(OAuthProvider::GitHub.default_scopes().contains("user:email"));
        assert!(OAuthProvider::Discord.default_scopes().contains("identify"));
        assert!(OAuthProvider::Apple.default_scopes().contains("email"));
        assert!(OAuthProvider::Facebook.default_scopes().contains("public_profile"));
        assert!(OAuthProvider::Microsoft.default_scopes().contains("User.Read"));
    }
    
    #[test]
    fn test_provider_display_names() {
        assert_eq!(OAuthProvider::Google.display_name(), "Google");
        assert_eq!(OAuthProvider::GitHub.display_name(), "GitHub");
        assert_eq!(OAuthProvider::Discord.display_name(), "Discord");
        assert_eq!(OAuthProvider::Apple.display_name(), "Apple");
        assert_eq!(OAuthProvider::Facebook.display_name(), "Facebook");
        assert_eq!(OAuthProvider::Microsoft.display_name(), "Microsoft");
    }
    
    #[test]
    fn test_provider_icons() {
        assert!(!OAuthProvider::Google.icon().is_empty());
        assert!(!OAuthProvider::GitHub.icon().is_empty());
        assert!(!OAuthProvider::Discord.icon().is_empty());
        assert!(!OAuthProvider::Apple.icon().is_empty());
        assert!(!OAuthProvider::Facebook.icon().is_empty());
        assert!(!OAuthProvider::Microsoft.icon().is_empty());
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // CONFIG TESTS
    // ─────────────────────────────────────────────────────────────────────────
    
    #[test]
    fn test_config_default() {
        let config = OAuthConfig::default();
        assert!(config.redirect_uri.is_empty());
        assert!(config.google.is_none());
        assert!(config.github.is_none());
        assert!(config.enabled_providers().is_empty());
    }
    
    #[test]
    fn test_config_get_provider() {
        let config = OAuthConfig {
            redirect_uri: "http://localhost:3000/auth/callback".to_string(),
            google: Some(ProviderConfig {
                client_id: "google-id".to_string(),
                client_secret: "google-secret".to_string(),
                scopes: Some("custom scope".to_string()),
                enabled: true,
            }),
            ..Default::default()
        };
        
        let google = config.get_provider(OAuthProvider::Google);
        assert!(google.is_some());
        assert_eq!(google.unwrap().client_id, "google-id");
        assert_eq!(google.unwrap().scopes, Some("custom scope".to_string()));
        
        let github = config.get_provider(OAuthProvider::GitHub);
        assert!(github.is_none());
    }
    
    #[test]
    fn test_enabled_providers_multiple() {
        let config = OAuthConfig {
            redirect_uri: "http://localhost:3000/auth/callback".to_string(),
            google: Some(ProviderConfig {
                client_id: "test".to_string(),
                client_secret: "secret".to_string(),
                scopes: None,
                enabled: true,
            }),
            github: Some(ProviderConfig {
                client_id: "test".to_string(),
                client_secret: "secret".to_string(),
                scopes: None,
                enabled: true,
            }),
            discord: Some(ProviderConfig {
                client_id: "test".to_string(),
                client_secret: "secret".to_string(),
                scopes: None,
                enabled: false, // Disabled
            }),
            ..Default::default()
        };
        
        let providers = config.enabled_providers();
        assert_eq!(providers.len(), 2);
        assert!(providers.contains(&OAuthProvider::Google));
        assert!(providers.contains(&OAuthProvider::GitHub));
        assert!(!providers.contains(&OAuthProvider::Discord));
    }
    
    #[test]
    fn test_enabled_providers_all() {
        let provider_config = ProviderConfig {
            client_id: "test".to_string(),
            client_secret: "secret".to_string(),
            scopes: None,
            enabled: true,
        };
        
        let config = OAuthConfig {
            redirect_uri: "http://localhost:3000".to_string(),
            google: Some(provider_config.clone()),
            github: Some(provider_config.clone()),
            discord: Some(provider_config.clone()),
            apple: Some(provider_config.clone()),
            facebook: Some(provider_config.clone()),
            microsoft: Some(provider_config),
        };
        
        assert_eq!(config.enabled_providers().len(), 6);
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // OAUTH CLIENT TESTS
    // ─────────────────────────────────────────────────────────────────────────
    
    #[test]
    fn test_state_generation() {
        let state1 = OAuth::generate_state();
        let state2 = OAuth::generate_state();
        
        // Should be 64 hex characters (32 bytes)
        assert_eq!(state1.len(), 64);
        assert_eq!(state2.len(), 64);
        
        // Should be unique
        assert_ne!(state1, state2);
        
        // Should be valid hex
        assert!(state1.chars().all(|c| c.is_ascii_hexdigit()));
    }
    
    #[test]
    fn test_authorize_url_google() {
        let config = OAuthConfig {
            redirect_uri: "http://localhost:3000/auth/callback".to_string(),
            google: Some(ProviderConfig {
                client_id: "my-client-id".to_string(),
                client_secret: "my-secret".to_string(),
                scopes: None,
                enabled: true,
            }),
            ..Default::default()
        };
        
        let oauth = OAuth::new(config);
        let result = oauth.authorize_url(OAuthProvider::Google);
        
        assert!(result.is_ok());
        let (url, state) = result.unwrap();
        
        assert!(url.contains("accounts.google.com"));
        assert!(url.contains("client_id=my-client-id"));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("scope="));
        assert!(url.contains(&format!("state={}", state)));
        assert!(url.contains("access_type=offline"));
        assert!(url.contains("prompt=select_account"));
    }
    
    #[test]
    fn test_authorize_url_github() {
        let config = OAuthConfig {
            redirect_uri: "http://localhost:3000/auth/callback".to_string(),
            github: Some(ProviderConfig {
                client_id: "github-client".to_string(),
                client_secret: "github-secret".to_string(),
                scopes: Some("repo user".to_string()),
                enabled: true,
            }),
            ..Default::default()
        };
        
        let oauth = OAuth::new(config);
        let result = oauth.authorize_url(OAuthProvider::GitHub);
        
        assert!(result.is_ok());
        let (url, _) = result.unwrap();
        
        assert!(url.contains("github.com"));
        assert!(url.contains("client_id=github-client"));
        assert!(url.contains("scope=repo"));
    }
    
    #[test]
    fn test_authorize_url_discord() {
        let config = OAuthConfig {
            redirect_uri: "http://localhost:3000/auth/callback".to_string(),
            discord: Some(ProviderConfig {
                client_id: "discord-client".to_string(),
                client_secret: "discord-secret".to_string(),
                scopes: None,
                enabled: true,
            }),
            ..Default::default()
        };
        
        let oauth = OAuth::new(config);
        let result = oauth.authorize_url(OAuthProvider::Discord);
        
        assert!(result.is_ok());
        let (url, _) = result.unwrap();
        
        assert!(url.contains("discord.com"));
        assert!(url.contains("prompt=consent"));
    }
    
    #[test]
    fn test_authorize_url_apple() {
        let config = OAuthConfig {
            redirect_uri: "http://localhost:3000/auth/callback".to_string(),
            apple: Some(ProviderConfig {
                client_id: "apple-client".to_string(),
                client_secret: "apple-secret".to_string(),
                scopes: None,
                enabled: true,
            }),
            ..Default::default()
        };
        
        let oauth = OAuth::new(config);
        let result = oauth.authorize_url(OAuthProvider::Apple);
        
        assert!(result.is_ok());
        let (url, _) = result.unwrap();
        
        assert!(url.contains("appleid.apple.com"));
        assert!(url.contains("response_mode=form_post"));
    }
    
    #[test]
    fn test_authorize_url_not_configured() {
        let config = OAuthConfig::default();
        let oauth = OAuth::new(config);
        
        let result = oauth.authorize_url(OAuthProvider::Google);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not configured"));
    }
    
    #[test]
    fn test_authorize_url_disabled() {
        let config = OAuthConfig {
            redirect_uri: "http://localhost:3000/auth/callback".to_string(),
            google: Some(ProviderConfig {
                client_id: "my-client-id".to_string(),
                client_secret: "my-secret".to_string(),
                scopes: None,
                enabled: false, // Disabled
            }),
            ..Default::default()
        };
        
        let oauth = OAuth::new(config);
        let result = oauth.authorize_url(OAuthProvider::Google);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not enabled"));
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // HTML HELPER TESTS
    // ─────────────────────────────────────────────────────────────────────────
    
    #[test]
    fn test_render_social_buttons_empty() {
        let config = OAuthConfig::default();
        let html = render_social_buttons(&config, None);
        assert!(html.is_empty());
    }
    
    #[test]
    fn test_render_social_buttons() {
        let config = OAuthConfig {
            redirect_uri: "http://localhost:3000".to_string(),
            google: Some(ProviderConfig {
                client_id: "test".to_string(),
                client_secret: "secret".to_string(),
                scopes: None,
                enabled: true,
            }),
            github: Some(ProviderConfig {
                client_id: "test".to_string(),
                client_secret: "secret".to_string(),
                scopes: None,
                enabled: true,
            }),
            ..Default::default()
        };
        
        let html = render_social_buttons(&config, None);
        
        assert!(html.contains("social-login-buttons"));
        assert!(html.contains("/auth/google"));
        assert!(html.contains("/auth/github"));
        assert!(html.contains("Continue with Google"));
        assert!(html.contains("Continue with GitHub"));
        assert!(html.contains("social-login-btn"));
    }
    
    #[test]
    fn test_render_social_buttons_custom_class() {
        let config = OAuthConfig {
            redirect_uri: "http://localhost:3000".to_string(),
            google: Some(ProviderConfig {
                client_id: "test".to_string(),
                client_secret: "secret".to_string(),
                scopes: None,
                enabled: true,
            }),
            ..Default::default()
        };
        
        let html = render_social_buttons(&config, Some("btn btn-custom"));
        assert!(html.contains("btn btn-custom"));
    }
    
    #[test]
    fn test_social_buttons_css() {
        let css = social_buttons_css();
        
        assert!(css.contains(".social-login-buttons"));
        assert!(css.contains(".social-login-btn"));
        assert!(css.contains(".social-google"));
        assert!(css.contains(".social-github"));
        assert!(css.contains(".social-discord"));
        assert!(css.contains(".social-apple"));
        assert!(css.contains(".social-facebook"));
        assert!(css.contains(".social-microsoft"));
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // OAUTH USER TESTS
    // ─────────────────────────────────────────────────────────────────────────
    
    #[test]
    fn test_oauth_user_serialization() {
        let user = OAuthUser {
            provider: "google".to_string(),
            provider_id: "12345".to_string(),
            email: Some("test@example.com".to_string()),
            name: Some("Test User".to_string()),
            avatar: Some("https://example.com/avatar.png".to_string()),
            raw: HashMap::new(),
        };
        
        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("google"));
        assert!(json.contains("test@example.com"));
        
        let deserialized: OAuthUser = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.provider, "google");
        assert_eq!(deserialized.email, Some("test@example.com".to_string()));
    }
    
    // ─────────────────────────────────────────────────────────────────────────
    // PROVIDER CONFIG TESTS
    // ─────────────────────────────────────────────────────────────────────────
    
    #[test]
    fn test_provider_config_serialization() {
        let config = ProviderConfig {
            client_id: "my-client".to_string(),
            client_secret: "my-secret".to_string(),
            scopes: Some("email profile".to_string()),
            enabled: true,
        };
        
        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("my-client"));
        assert!(json.contains("email profile"));
        
        let deserialized: ProviderConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.client_id, "my-client");
        assert!(deserialized.enabled);
    }
    
    #[test]
    fn test_oauth_config_serialization() {
        let config = OAuthConfig {
            redirect_uri: "http://localhost:3000/callback".to_string(),
            google: Some(ProviderConfig {
                client_id: "google-id".to_string(),
                client_secret: "google-secret".to_string(),
                scopes: None,
                enabled: true,
            }),
            ..Default::default()
        };
        
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: OAuthConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.redirect_uri, "http://localhost:3000/callback");
        assert!(deserialized.google.is_some());
        assert!(deserialized.github.is_none());
    }
}

