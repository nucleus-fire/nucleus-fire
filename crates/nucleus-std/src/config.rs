use serde::Deserialize;
use std::fs;
use std::env;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub database: DatabaseConfig,
    #[serde(default)]
    pub app: AppConfig,
    #[serde(default)]
    pub performance: PerformanceConfig,
    
    pub payments: Option<PaymentsConfig>,
    pub chain: Option<ChainConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_env")]
    pub environment: String,
    #[serde(default)]
    pub omit_signature: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            environment: default_env(),
            omit_signature: false,
        }
    }
}

fn default_host() -> String { "0.0.0.0".to_string() }
fn default_port() -> u16 { 3000 }
fn default_env() -> String { "development".to_string() }

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    #[serde(default = "default_db_url")]
    pub url: String,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: default_db_url(),
            max_connections: default_max_connections(),
        }
    }
}

fn default_db_url() -> String { "sqlite:nucleus.db".to_string() }
fn default_max_connections() -> u32 { 5 }

#[derive(Debug, Deserialize, Default, Clone)]
pub struct AppConfig {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub secret_key: String,
    #[serde(default)]
    pub admin_username: String,
    #[serde(default)]
    pub admin_password: String,
}

/// Performance Configuration
/// Controls caching, font loading, and asset optimization
#[derive(Debug, Deserialize, Clone)]
pub struct PerformanceConfig {
    /// Cache settings for different asset types
    #[serde(default)]
    pub cache: CacheConfig,
    /// Font loading optimization
    #[serde(default)]
    pub fonts: FontConfig,
    /// Enable gzip/brotli compression
    #[serde(default = "default_true")]
    pub compression: bool,
    /// Inline critical CSS
    #[serde(default = "default_true")]
    pub inline_critical_css: bool,
    /// Custom origins to preconnect to
    #[serde(default)]
    pub preconnect_origins: Vec<String>,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            cache: CacheConfig::default(),
            fonts: FontConfig::default(),
            compression: true,
            inline_critical_css: true,
            preconnect_origins: Vec::new(),
        }
    }
}

/// Cache-Control header settings
#[derive(Debug, Deserialize, Clone)]
pub struct CacheConfig {
    /// CSS files (default: 1 year, immutable)
    #[serde(default = "default_css_max_age")]
    pub css_max_age: u32,
    /// JavaScript files (default: 1 year, immutable)
    #[serde(default = "default_js_max_age")]
    pub js_max_age: u32,
    /// Font files (default: 1 year, immutable)
    #[serde(default = "default_font_max_age")]
    pub font_max_age: u32,
    /// Image files (default: 1 week)
    #[serde(default = "default_image_max_age")]
    pub image_max_age: u32,
    /// HTML pages (default: no-cache)
    #[serde(default)]
    pub html_no_cache: bool,
    /// Use immutable directive for static assets
    #[serde(default = "default_true")]
    pub immutable: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            css_max_age: default_css_max_age(),
            js_max_age: default_js_max_age(),
            font_max_age: default_font_max_age(),
            image_max_age: default_image_max_age(),
            html_no_cache: true,
            immutable: true,
        }
    }
}

fn default_css_max_age() -> u32 { 31536000 }  // 1 year
fn default_js_max_age() -> u32 { 31536000 }   // 1 year
fn default_font_max_age() -> u32 { 31536000 } // 1 year
fn default_image_max_age() -> u32 { 604800 }  // 1 week
fn default_true() -> bool { true }

/// Font loading optimization settings
#[derive(Debug, Deserialize, Clone)]
pub struct FontConfig {
    /// Use font-display: swap (prevents FOIT)
    #[serde(default = "default_true")]
    pub display_swap: bool,
    /// Preconnect to Google Fonts
    #[serde(default = "default_true")]
    pub preconnect: bool,
    /// Load fonts asynchronously (non-render-blocking)
    #[serde(default = "default_true")]
    pub async_load: bool,
    /// Google Fonts URL (if using)
    #[serde(default)]
    pub google_fonts_url: Option<String>,
}

impl Default for FontConfig {
    fn default() -> Self {
        Self {
            display_swap: true,
            preconnect: true,
            async_load: true,
            google_fonts_url: None,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct PaymentsConfig {
    pub stripe_key: String,
    pub currency: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChainConfig {
    pub rpc_url: String,
    pub chain_id: u64,
}

use crate::errors::{Result, NucleusError};

impl Config {
    pub fn load() -> Config {
        let content = if let Ok(c) = fs::read_to_string("nucleus.config") {
            c
        } else if let Ok(c) = fs::read_to_string("nucleus.toml") {
            c
        } else {
            return Self::default();
        };

        let interpolated = Self::interpolate_env(&content);
        toml::from_str(&interpolated).unwrap_or_else(|e| {
            eprintln!("⚠️  Nucleus Config Error: {}", NucleusError::ConfigError(e));
            // Fallback to default mostly safe for dev, but critical for prod?
            // For now keeping existing behavior but using proper error formatting in log
            Self::default()
        })
    }

    /// Load with explicit error handling (preferred for CLI tools)
    pub fn try_load() -> Result<Self> {
         let content = if let Ok(c) = fs::read_to_string("nucleus.config") {
            c
        } else if let Ok(c) = fs::read_to_string("nucleus.toml") {
            c
        } else {
            return Ok(Self::default());
        };

        let interpolated = Self::interpolate_env(&content);
        toml::from_str(&interpolated).map_err(NucleusError::ConfigError)
    }

    /// Interpolates environment variables: ${VAR} or ${VAR|default}
    fn interpolate_env(content: &str) -> String {
        let re = Regex::new(r"\$\{([A-Z0-9_]+)(?:\|([^}]+))?\}").unwrap();
        
        re.replace_all(content, |caps: &regex::Captures| {
            let var_name = &caps[1];
            let default_val = caps.get(2).map(|m| m.as_str());
            
            match env::var(var_name) {
                Ok(val) => val,
                Err(_) => default_val.unwrap_or("").to_string(),
            }
        }).to_string()
    }
}

lazy_static! {
    pub static ref GLOBAL_CONFIG: Config = Config::load();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_default_loading() {
        // Ensure no config file interferes (if running in unclean dir)
        // Ideally unit tests mock fs, but here we test the struct defaults
        let config = Config::default();
        assert_eq!(config.server.port, 3000);
        assert_eq!(config.database.url, "sqlite:nucleus.db");
    }

    #[test]
    fn test_env_interpolation() {
        env::set_var("TEST_PORT", "8080");
        let raw = "
        [server]
        port = ${TEST_PORT}
        ";
        let inter = Config::interpolate_env(raw);
        assert!(inter.contains("port = 8080"));
        env::remove_var("TEST_PORT");
    }

    #[test]
    fn test_env_defaults() {
        let raw = "
        [server]
        host = \"${MISSING_HOST|127.0.0.1}\"
        ";
        let inter = Config::interpolate_env(raw);
        assert!(inter.contains("host = \"127.0.0.1\""));
    }
}
