use thiserror::Error;

#[derive(Error, Debug)]
pub enum NucleusError {
    #[error("Configuration Error: {0}")]
    ConfigError(#[from] toml::de::Error),

    #[error("Network Request Failed: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Database Error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Payment Provider Error: {0}")]
    PaymentError(String),

    #[error("Blockchain Error: {0}")]
    CryptoError(String),

    #[error("Validation Error: {0}")]
    ValidationError(String),

    #[error("Internal Error: {0}")]
    InternalError(String),
}

// Convenience alias
pub type Result<T> = std::result::Result<T, NucleusError>;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

impl IntoResponse for NucleusError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            NucleusError::ValidationError(m) => (StatusCode::BAD_REQUEST, m),
            NucleusError::ConfigError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Configuration Error".to_string(),
            ),
            NucleusError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database Error".to_string(),
            ),
            NucleusError::NetworkError(_) => {
                (StatusCode::BAD_GATEWAY, "Upstream Error".to_string())
            }
            NucleusError::IOError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "IO Error".to_string()),
            NucleusError::PaymentError(m) => {
                (StatusCode::BAD_REQUEST, format!("Payment Error: {}", m))
            }
            NucleusError::CryptoError(m) => {
                (StatusCode::BAD_REQUEST, format!("Crypto Error: {}", m))
            }
            NucleusError::InternalError(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
        };

        (status, Json(json!({ "error": msg }))).into_response()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_display() {
        let err = NucleusError::ValidationError("Invalid email".to_string());
        assert_eq!(err.to_string(), "Validation Error: Invalid email");
    }

    #[test]
    fn test_payment_error_display() {
        let err = NucleusError::PaymentError("Card declined".to_string());
        assert_eq!(err.to_string(), "Payment Provider Error: Card declined");
    }

    #[test]
    fn test_crypto_error_display() {
        let err = NucleusError::CryptoError("Invalid signature".to_string());
        assert_eq!(err.to_string(), "Blockchain Error: Invalid signature");
    }

    #[test]
    fn test_internal_error_display() {
        let err = NucleusError::InternalError("Something went wrong".to_string());
        assert_eq!(err.to_string(), "Internal Error: Something went wrong");
    }

    #[test]
    fn test_result_type_alias() {
        fn example_function() -> Result<i32> {
            Ok(42)
        }

        assert_eq!(example_function().unwrap(), 42);
    }

    #[test]
    fn test_result_with_error() {
        fn failing_function() -> Result<()> {
            Err(NucleusError::ValidationError("test".to_string()))
        }

        assert!(failing_function().is_err());
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let nucleus_err: NucleusError = io_err.into();

        assert!(matches!(nucleus_err, NucleusError::IOError(_)));
        assert!(nucleus_err.to_string().contains("IO Error"));
    }

    #[test]
    fn test_error_debug() {
        let err = NucleusError::ValidationError("test".to_string());
        let debug_str = format!("{:?}", err);

        assert!(debug_str.contains("ValidationError"));
    }
}
