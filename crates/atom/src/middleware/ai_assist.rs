//! AI Error Assistant Middleware
//!
//! Intercepts errors in development mode and uses AI to suggest fixes.
//! Uses the Neural module to analyze errors and provide actionable suggestions.

use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response};
use nucleus_std::neural::Neural;

// ═══════════════════════════════════════════════════════════════════════════
// MIDDLEWARE
// ═══════════════════════════════════════════════════════════════════════════

/// AI Error Assistant Middleware
pub async fn error_assistant(request: Request, next: Next) -> Response {
    error_assistant_with_config(request, next, AiAssistConfig::default()).await
}

/// Middleware with custom config
pub async fn error_assistant_with_config(
    request: Request,
    next: Next,
    config: AiAssistConfig,
) -> Response {
    let response = next.run(request).await;

    // Only analyze server errors (5xx) or explicit bad requests if useful
    if !config.enabled || !response.status().is_server_error() {
        return response;
    }

    let status = response.status();
    let (parts, body) = response.into_parts();

    // Read body with limit (16KB)
    // Note: axum 0.7 body collection
    match axum::body::to_bytes(body, 16 * 1024).await {
        Ok(bytes) => {
            let error_body = String::from_utf8_lossy(&bytes).to_string();
            // Reconstruct body for the client
            let new_body = Body::from(bytes);
            let response = Response::from_parts(parts, new_body);

            // Run analysis in background to avoid blocking response
            tokio::spawn(async move {
                analyze_and_report(error_body, status, config).await;
            });

            response
        }
        Err(e) => {
            // If body reading fails, we can't reconstruct gracefully with original data
            // but we can return a generic error or the parts with empty body
            println!("Failed to read error body for analysis: {}", e);
            Response::from_parts(parts, Body::empty())
        }
    }
}

async fn analyze_and_report(error: String, status: StatusCode, config: AiAssistConfig) {
    // 1. Fast sync analysis
    let mut analysis = analyze_error_sync(&error, status);

    // 2. Deep AI analysis if configured and no sync suggestion found
    if analysis.suggestion.is_none() {
        if let Some(api_key) = config.api_key {
            // Truncate error if too long
            let safe_error = if error.len() > config.max_context {
                format!("{}...", &error[..config.max_context])
            } else {
                error
            };

            let neural = Neural::new(&api_key).with_model(&config.model);
            let prompt = format!(
                "You are an expert Rust/Web developer. Analyze this server error and suggest a specific fix.\n\
                 Error: {}\n\
                 Status: {}\n\
                 Keep the explanation concise (1-2 sentences) and provide a command if applicable.", 
                safe_error, status
            );

            match neural.ask(&prompt).await {
                Ok(ai_response) => {
                    analysis.suggestion = Some(ai_response);
                    analysis.confidence = Some(85);
                }
                Err(_e) => {
                    // Fail silently in logs, don't spam
                }
            }
        }
    }

    log_error_analysis(&analysis);
}

// ═══════════════════════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

/// AI Assistant configuration
#[derive(Debug, Clone)]
pub struct AiAssistConfig {
    /// Enable AI assistance (defaults to true in dev)
    pub enabled: bool,
    /// OpenAI API key (optional, falls back to OPENAI_API_KEY env)
    pub api_key: Option<String>,
    /// Model to use (default: gpt-4o-mini)
    pub model: String,
    /// Maximum error context characters
    pub max_context: usize,
}

impl Default for AiAssistConfig {
    fn default() -> Self {
        Self {
            enabled: cfg!(debug_assertions) && std::env::var("OPENAI_API_KEY").is_ok(),
            api_key: std::env::var("OPENAI_API_KEY").ok(),
            model: "gpt-4o-mini".to_string(),
            max_context: 2000,
        }
    }
}

impl AiAssistConfig {
    /// Create with custom API key
    pub fn with_api_key(mut self, key: impl Into<String>) -> Self {
        self.api_key = Some(key.into());
        self.enabled = true;
        self
    }

    /// Set model
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ERROR ANALYSIS
// ═══════════════════════════════════════════════════════════════════════════

/// Analyzed error with AI suggestions
#[derive(Debug, Clone)]
pub struct ErrorAnalysis {
    /// Original error message
    pub error: String,
    /// HTTP status code
    pub status: StatusCode,
    /// AI-generated suggestion (if available)
    pub suggestion: Option<String>,
    /// Suggested fix command (if applicable)
    pub fix_command: Option<String>,
    /// Confidence score (0-100)
    pub confidence: Option<u8>,
}

impl ErrorAnalysis {
    /// Create a new analysis without AI suggestion
    pub fn new(error: impl Into<String>, status: StatusCode) -> Self {
        Self {
            error: error.into(),
            status,
            suggestion: None,
            fix_command: None,
            confidence: None,
        }
    }

    /// Add AI-generated suggestion
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Add fix command
    pub fn with_fix(mut self, command: impl Into<String>) -> Self {
        self.fix_command = Some(command.into());
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// COMMON ERROR PATTERNS
// ═══════════════════════════════════════════════════════════════════════════

/// Analyze error without AI, using pattern matching
pub fn analyze_error_sync(error: &str, status: StatusCode) -> ErrorAnalysis {
    let mut analysis = ErrorAnalysis::new(error, status);

    // Column not found patterns
    if error.contains("column") && error.contains("not found") {
        if let Some(col) = extract_quoted(error) {
            analysis.suggestion = Some(format!(
                "Column '{}' doesn't exist. Check your schema or run a migration.",
                col
            ));
            analysis.fix_command = Some("nucleus db status".to_string());
        }
    }
    // Table not found
    else if error.contains("no such table")
        || error.contains("table") && error.contains("does not exist")
    {
        if let Some(table) = extract_quoted(error) {
            analysis.suggestion = Some(format!(
                "Table '{}' not found. Run migrations or check the table name.",
                table
            ));
            analysis.fix_command = Some("nucleus db migrate".to_string());
        }
    }
    // Connection errors
    else if error.contains("connection refused") || error.contains("ECONNREFUSED") {
        analysis.suggestion =
            Some("Database connection refused. Is the database running?".to_string());
        analysis.fix_command = Some("nucleus db status".to_string());
    }
    // Auth errors
    else if status == StatusCode::UNAUTHORIZED {
        analysis.suggestion =
            Some("Authentication required. Token missing or invalid.".to_string());
    } else if status == StatusCode::FORBIDDEN {
        analysis.suggestion =
            Some("Permission denied. User lacks required role or capability.".to_string());
    }
    // Not found
    else if status == StatusCode::NOT_FOUND {
        analysis.suggestion = Some("Resource not found. Check the ID or path.".to_string());
    }
    // Validation errors
    else if error.contains("validation") || error.contains("invalid") {
        analysis.suggestion =
            Some("Validation failed. Check the request payload format.".to_string());
    }

    analysis
}

/// Extract quoted text from error message
fn extract_quoted(s: &str) -> Option<String> {
    let start = s.find('\'')?;
    let end = s[start + 1..].find('\'')?;
    Some(s[start + 1..start + 1 + end].to_string())
}

// ═══════════════════════════════════════════════════════════════════════════
// LOG HELPER
// ═══════════════════════════════════════════════════════════════════════════

/// Log error analysis to console
pub fn log_error_analysis(analysis: &ErrorAnalysis) {
    println!("\n  \x1b[31m❌ Error {}\x1b[0m", analysis.status);
    println!("     {}", analysis.error);

    if let Some(ref suggestion) = analysis.suggestion {
        println!("  \x1b[33m💡 Suggestion:\x1b[0m {}", suggestion);
    }

    if let Some(ref cmd) = analysis.fix_command {
        println!("  \x1b[36m🔧 Fix:\x1b[0m {}", cmd);
    }

    println!();
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = AiAssistConfig::default();
        assert_eq!(config.model, "gpt-4o-mini");
        assert_eq!(config.max_context, 2000);
    }

    #[test]
    fn test_config_builder() {
        let config = AiAssistConfig::default()
            .with_api_key("test-key")
            .with_model("gpt-4");
        assert!(config.enabled);
        assert_eq!(config.api_key, Some("test-key".to_string()));
        assert_eq!(config.model, "gpt-4");
    }

    #[test]
    fn test_error_analysis_column_not_found() {
        let analysis = analyze_error_sync(
            "column 'user_id' not found in table",
            StatusCode::INTERNAL_SERVER_ERROR,
        );
        assert!(analysis.suggestion.is_some());
        assert!(analysis.suggestion.unwrap().contains("user_id"));
    }

    #[test]
    fn test_error_analysis_table_not_found() {
        let analysis =
            analyze_error_sync("no such table: 'users'", StatusCode::INTERNAL_SERVER_ERROR);
        assert!(analysis.suggestion.is_some());
        assert!(analysis.fix_command.is_some());
        assert_eq!(analysis.fix_command.unwrap(), "nucleus db migrate");
    }

    #[test]
    fn test_error_analysis_auth() {
        let analysis = analyze_error_sync("Unauthorized", StatusCode::UNAUTHORIZED);
        assert!(analysis.suggestion.is_some());
        assert!(analysis.suggestion.unwrap().contains("Authentication"));
    }

    #[test]
    fn test_extract_quoted() {
        assert_eq!(
            extract_quoted("column 'foo' not found"),
            Some("foo".to_string())
        );
        assert_eq!(extract_quoted("no quotes"), None);
    }
}
