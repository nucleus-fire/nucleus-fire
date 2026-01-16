use nucleus_std::server;
use nucleus_std::errors::{Result, NucleusError};
// use std::path::Path;

#[server]
pub async fn get_source(path: String) -> Result<String> {
    // SECURITY: Whitelist allowing only files inside src/
    if path.contains("..") || !path.starts_with("src/") {
        return Err(NucleusError::ValidationError("Invalid file path".into()));
    }

    // Read file
    let content = tokio::fs::read_to_string(&path)
        .await
        .map_err(NucleusError::IOError)?;
        
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_source_security_traversal() {
        let err = get_source("../Cargo.toml".to_string()).await.unwrap_err();
        match err {
            NucleusError::ValidationError(msg) => assert_eq!(msg, "Invalid file path"),
            _ => panic!("Expected ValidationError, got {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_source_security_outside_src() {
        let err = get_source("Cargo.toml".to_string()).await.unwrap_err();
        match err {
            NucleusError::ValidationError(msg) => assert_eq!(msg, "Invalid file path"),
            _ => panic!("Expected ValidationError, got {:?}", err),
        }
    }

    #[tokio::test]
    async fn test_valid_source_read() {
        // We know src/logic/source.rs exists because we are in it!
        let content = get_source("src/logic/source.rs".to_string()).await.unwrap();
        assert!(content.contains("pub async fn get_source"));
    }
}
