//! Nucleus File Upload Module
//!
//! Provides comprehensive file upload handling with:
//! - Multipart form parsing
//! - File size and MIME type validation
//! - Secure file storage
//! - Unique filename generation
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::upload::{Upload, UploadConfig, UploadedFile};
//!
//! let config = UploadConfig::default()
//!     .max_size(5_000_000) // 5MB
//!     .allowed_types(vec!["image/jpeg", "image/png"]);
//!
//! let files = Upload::from_multipart(body, content_type, &config).await?;
//! for file in files {
//!     println!("Uploaded: {} ({} bytes)", file.original_name, file.size);
//! }
//! ```

use std::path::{Path, PathBuf};
// use std::io::Write;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


// ═══════════════════════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

/// Configuration for file uploads
#[derive(Debug, Clone)]
pub struct UploadConfig {
    /// Maximum file size in bytes (default: 10MB)
    pub max_file_size: usize,
    /// Allowed MIME types (empty = allow all)
    pub allowed_types: Vec<String>,
    /// Storage directory path
    pub storage_path: PathBuf,
    /// Whether to generate unique filenames
    pub generate_unique_names: bool,
    /// Preserve original extension
    pub preserve_extension: bool,
}

impl Default for UploadConfig {
    fn default() -> Self {
        Self {
            max_file_size: 10 * 1024 * 1024, // 10MB
            allowed_types: vec![],
            storage_path: PathBuf::from("uploads"),
            generate_unique_names: true,
            preserve_extension: true,
        }
    }
}

impl UploadConfig {
    /// Create a new upload config
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set maximum file size in bytes
    pub fn max_size(mut self, bytes: usize) -> Self {
        self.max_file_size = bytes;
        self
    }
    
    /// Set allowed MIME types
    pub fn allowed_types(mut self, types: Vec<&str>) -> Self {
        self.allowed_types = types.iter().map(|s| s.to_string()).collect();
        self
    }
    
    /// Allow only images
    pub fn images_only(mut self) -> Self {
        self.allowed_types = vec![
            "image/jpeg".to_string(),
            "image/png".to_string(),
            "image/gif".to_string(),
            "image/webp".to_string(),
            "image/svg+xml".to_string(),
        ];
        self
    }
    
    /// Allow only documents
    pub fn documents_only(mut self) -> Self {
        self.allowed_types = vec![
            "application/pdf".to_string(),
            "application/msword".to_string(),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
            "text/plain".to_string(),
        ];
        self
    }
    
    /// Set storage path
    pub fn storage(mut self, path: impl Into<PathBuf>) -> Self {
        self.storage_path = path.into();
        self
    }
    
    /// Disable unique name generation
    pub fn keep_original_names(mut self) -> Self {
        self.generate_unique_names = false;
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// UPLOADED FILE
// ═══════════════════════════════════════════════════════════════════════════

/// Metadata for an uploaded file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadedFile {
    /// Original filename from the client
    pub original_name: String,
    /// Name used for storage
    pub stored_name: String,
    /// Full path to the stored file
    pub path: PathBuf,
    /// File size in bytes
    pub size: usize,
    /// MIME type
    pub mime_type: String,
    /// URL path to access the file
    pub url: String,
    /// Field name from the form
    pub field_name: String,
}

impl UploadedFile {
    /// Check if this is an image
    pub fn is_image(&self) -> bool {
        self.mime_type.starts_with("image/")
    }
    
    /// Check if this is a document
    pub fn is_document(&self) -> bool {
        matches!(
            self.mime_type.as_str(),
            "application/pdf" | "application/msword" | "text/plain"
        ) || self.mime_type.contains("document")
    }
    
    /// Get file extension
    pub fn extension(&self) -> Option<&str> {
        self.stored_name.rsplit('.').next()
    }
    
    /// Delete the uploaded file
    pub async fn delete(&self) -> Result<(), std::io::Error> {
        tokio::fs::remove_file(&self.path).await
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════

/// Upload error types
#[derive(Debug, thiserror::Error)]
pub enum UploadError {
    #[error("File too large: {actual} bytes exceeds limit of {max} bytes")]
    FileTooLarge { max: usize, actual: usize },
    
    #[error("Invalid file type: {actual}. Allowed: {expected:?}")]
    InvalidMimeType { expected: Vec<String>, actual: String },
    
    #[error("Storage error: {0}")]
    StorageError(#[from] std::io::Error),
    
    #[error("Multipart parse error: {0}")]
    ParseError(String),
    
    #[error("No file provided")]
    NoFile,
    
    #[error("Invalid content type header")]
    InvalidContentType,
}

// ═══════════════════════════════════════════════════════════════════════════
// UPLOAD HANDLER
// ═══════════════════════════════════════════════════════════════════════════

/// File upload handler
pub struct Upload;

impl Upload {
    /// Parse multipart request and save files
    pub async fn from_multipart(
        body: axum::body::Body,
        content_type: &str,
        config: &UploadConfig,
    ) -> Result<Vec<UploadedFile>, UploadError> {
        // Extract boundary from content type
        let boundary = content_type
            .split("boundary=")
            .nth(1)
            .ok_or(UploadError::InvalidContentType)?
            .trim_matches('"');
        
        // Ensure storage directory exists
        tokio::fs::create_dir_all(&config.storage_path).await?;
        
        // Parse multipart
        let stream = body.into_data_stream();
        let mut multipart = multer::Multipart::new(stream, boundary);
        
        let mut files = Vec::new();
        
        while let Some(field) = multipart
            .next_field()
            .await
            .map_err(|e| UploadError::ParseError(e.to_string()))?
        {
            let field_name = field.name().unwrap_or("file").to_string();
            let original_name = field
                .file_name()
                .map(|s| s.to_string())
                .unwrap_or_else(|| format!("upload_{}", Uuid::new_v4()));
            
            let content_type = field
                .content_type()
                .map(|m| m.to_string())
                .unwrap_or_else(|| {
                    mime_guess::from_path(&original_name)
                        .first_or_octet_stream()
                        .to_string()
                });
            
            // Validate MIME type
            if !config.allowed_types.is_empty() 
                && !config.allowed_types.contains(&content_type) 
            {
                return Err(UploadError::InvalidMimeType {
                    expected: config.allowed_types.clone(),
                    actual: content_type,
                });
            }
            
            // Read file data
            let data = field
                .bytes()
                .await
                .map_err(|e| UploadError::ParseError(e.to_string()))?;
            
            // Validate size
            if data.len() > config.max_file_size {
                return Err(UploadError::FileTooLarge {
                    max: config.max_file_size,
                    actual: data.len(),
                });
            }
            
            // Generate filename
            let stored_name = if config.generate_unique_names {
                let uuid = Uuid::new_v4();
                if config.preserve_extension {
                    if let Some(ext) = Path::new(&original_name).extension() {
                        format!("{}.{}", uuid, ext.to_string_lossy())
                    } else {
                        uuid.to_string()
                    }
                } else {
                    uuid.to_string()
                }
            } else {
                original_name.clone()
            };
            
            // Save file
            let path = config.storage_path.join(&stored_name);
            use tokio::io::AsyncWriteExt;
            let mut file = tokio::fs::File::create(&path).await?;
            file.write_all(&data).await?;
            
            let url = format!("/uploads/{}", stored_name);
            
            files.push(UploadedFile {
                original_name,
                stored_name,
                path,
                size: data.len(),
                mime_type: content_type,
                url,
                field_name,
            });
        }
        
        Ok(files)
    }
    
    /// Parse a single file from multipart
    pub async fn single(
        body: axum::body::Body,
        content_type: &str,
        config: &UploadConfig,
    ) -> Result<UploadedFile, UploadError> {
        let files = Self::from_multipart(body, content_type, config).await?;
        files.into_iter().next().ok_or(UploadError::NoFile)
    }
    
    /// Validate a file against config
    pub fn validate(file: &UploadedFile, config: &UploadConfig) -> Result<(), UploadError> {
        // Check size
        if file.size > config.max_file_size {
            return Err(UploadError::FileTooLarge {
                max: config.max_file_size,
                actual: file.size,
            });
        }
        
        // Check MIME type
        if !config.allowed_types.is_empty() 
            && !config.allowed_types.contains(&file.mime_type) 
        {
            return Err(UploadError::InvalidMimeType {
                expected: config.allowed_types.clone(),
                actual: file.mime_type.clone(),
            });
        }
        
        Ok(())
    }
    
    /// Delete an uploaded file
    pub async fn delete(file: &UploadedFile) -> Result<(), std::io::Error> {
        tokio::fs::remove_file(&file.path).await
    }
    
    /// Delete files older than specified duration
    pub async fn cleanup_old_files(
        storage_path: &Path,
        max_age: std::time::Duration,
    ) -> Result<usize, std::io::Error> {
        let mut deleted = 0;
        let now = std::time::SystemTime::now();
        
        // Read dir is a bit complex with tokio, reading entries into a vec first
        let mut entries = tokio::fs::read_dir(storage_path).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;
            
            if let Ok(modified) = metadata.modified() {
                if let Ok(age) = now.duration_since(modified) {
                    if age > max_age {
                        tokio::fs::remove_file(entry.path()).await?;
                        deleted += 1;
                    }
                }
            }
        }
        
        Ok(deleted)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_defaults() {
        let config = UploadConfig::default();
        assert_eq!(config.max_file_size, 10 * 1024 * 1024);
        assert!(config.allowed_types.is_empty());
        assert!(config.generate_unique_names);
    }
    
    #[test]
    fn test_config_builder() {
        let config = UploadConfig::new()
            .max_size(5_000_000)
            .images_only()
            .storage("custom/path");
        
        assert_eq!(config.max_file_size, 5_000_000);
        assert!(config.allowed_types.contains(&"image/jpeg".to_string()));
        assert_eq!(config.storage_path, PathBuf::from("custom/path"));
    }
    
    #[test]
    fn test_uploaded_file_helpers() {
        let file = UploadedFile {
            original_name: "test.jpg".to_string(),
            stored_name: "abc123.jpg".to_string(),
            path: PathBuf::from("/uploads/abc123.jpg"),
            size: 1024,
            mime_type: "image/jpeg".to_string(),
            url: "/uploads/abc123.jpg".to_string(),
            field_name: "avatar".to_string(),
        };
        
        assert!(file.is_image());
        assert!(!file.is_document());
        assert_eq!(file.extension(), Some("jpg"));
    }
    
    // ═══════════════════════════════════════════════════════════════════════════
    // EDGE CASE TESTS
    // ═══════════════════════════════════════════════════════════════════════════
    
    #[test]
    fn test_config_images_only() {
        let config = UploadConfig::new().images_only();
        
        assert!(config.allowed_types.contains(&"image/jpeg".to_string()));
        assert!(config.allowed_types.contains(&"image/png".to_string()));
        assert!(config.allowed_types.contains(&"image/gif".to_string()));
        assert!(config.allowed_types.contains(&"image/webp".to_string()));
        assert!(config.allowed_types.contains(&"image/svg+xml".to_string()));
        assert_eq!(config.allowed_types.len(), 5);
    }
    
    #[test]
    fn test_config_documents_only() {
        let config = UploadConfig::new().documents_only();
        
        assert!(config.allowed_types.contains(&"application/pdf".to_string()));
        assert!(config.allowed_types.contains(&"application/msword".to_string()));
        assert!(config.allowed_types.contains(&"text/plain".to_string()));
    }
    
    #[test]
    fn test_config_keep_original_names() {
        let config = UploadConfig::new().keep_original_names();
        assert!(!config.generate_unique_names);
    }
    
    #[test]
    fn test_uploaded_file_is_image() {
        let jpeg = UploadedFile {
            original_name: "".into(), stored_name: "".into(), path: PathBuf::new(),
            size: 0, mime_type: "image/jpeg".into(), url: "".into(), field_name: "".into(),
        };
        let png = UploadedFile { mime_type: "image/png".into(), ..jpeg.clone() };
        let gif = UploadedFile { mime_type: "image/gif".into(), ..jpeg.clone() };
        let webp = UploadedFile { mime_type: "image/webp".into(), ..jpeg.clone() };
        let svg = UploadedFile { mime_type: "image/svg+xml".into(), ..jpeg.clone() };
        let pdf = UploadedFile { mime_type: "application/pdf".into(), ..jpeg.clone() };
        
        assert!(jpeg.is_image());
        assert!(png.is_image());
        assert!(gif.is_image());
        assert!(webp.is_image());
        assert!(svg.is_image());
        assert!(!pdf.is_image());
    }
    
    #[test]
    fn test_uploaded_file_is_document() {
        let base = UploadedFile {
            original_name: "".into(), stored_name: "".into(), path: PathBuf::new(),
            size: 0, mime_type: "".into(), url: "".into(), field_name: "".into(),
        };
        
        let pdf = UploadedFile { mime_type: "application/pdf".into(), ..base.clone() };
        let doc = UploadedFile { mime_type: "application/msword".into(), ..base.clone() };
        let txt = UploadedFile { mime_type: "text/plain".into(), ..base.clone() };
        let docx = UploadedFile { 
            mime_type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document".into(), 
            ..base.clone() 
        };
        let jpg = UploadedFile { mime_type: "image/jpeg".into(), ..base.clone() };
        
        assert!(pdf.is_document());
        assert!(doc.is_document());
        assert!(txt.is_document());
        assert!(docx.is_document()); // Contains "document"
        assert!(!jpg.is_document());
    }
    
    #[test]
    fn test_uploaded_file_extension() {
        let with_ext = UploadedFile {
            original_name: "".into(), stored_name: "file.png".into(), path: PathBuf::new(),
            size: 0, mime_type: "".into(), url: "".into(), field_name: "".into(),
        };
        let multi_ext = UploadedFile { stored_name: "file.tar.gz".into(), ..with_ext.clone() };
        let no_ext = UploadedFile { stored_name: "noextension".into(), ..with_ext.clone() };
        let dot_only = UploadedFile { stored_name: ".hidden".into(), ..with_ext.clone() };
        
        assert_eq!(with_ext.extension(), Some("png"));
        assert_eq!(multi_ext.extension(), Some("gz"));
        assert_eq!(no_ext.extension(), Some("noextension")); // rsplit behavior
        assert_eq!(dot_only.extension(), Some("hidden"));
    }
    
    #[test]
    fn test_validate_size() {
        let config = UploadConfig::new().max_size(1000);
        
        let small_file = UploadedFile {
            original_name: "".into(), stored_name: "".into(), path: PathBuf::new(),
            size: 500, mime_type: "image/png".into(), url: "".into(), field_name: "".into(),
        };
        let large_file = UploadedFile { size: 2000, ..small_file.clone() };
        let exact_file = UploadedFile { size: 1000, ..small_file.clone() };
        
        assert!(Upload::validate(&small_file, &config).is_ok());
        assert!(Upload::validate(&exact_file, &config).is_ok());
        
        let err = Upload::validate(&large_file, &config).unwrap_err();
        match err {
            UploadError::FileTooLarge { max, actual } => {
                assert_eq!(max, 1000);
                assert_eq!(actual, 2000);
            }
            _ => panic!("Expected FileTooLarge error"),
        }
    }
    
    #[test]
    fn test_validate_mime_type() {
        let config = UploadConfig::new().images_only();
        
        let image = UploadedFile {
            original_name: "".into(), stored_name: "".into(), path: PathBuf::new(),
            size: 100, mime_type: "image/png".into(), url: "".into(), field_name: "".into(),
        };
        let pdf = UploadedFile { mime_type: "application/pdf".into(), ..image.clone() };
        
        assert!(Upload::validate(&image, &config).is_ok());
        
        let err = Upload::validate(&pdf, &config).unwrap_err();
        match err {
            UploadError::InvalidMimeType { expected, actual } => {
                assert!(expected.contains(&"image/png".to_string()));
                assert_eq!(actual, "application/pdf");
            }
            _ => panic!("Expected InvalidMimeType error"),
        }
    }
    
    #[test]
    fn test_validate_all_allowed() {
        let config = UploadConfig::new(); // Empty allowed_types = allow all
        
        let any_file = UploadedFile {
            original_name: "".into(), stored_name: "".into(), path: PathBuf::new(),
            size: 100, mime_type: "some/random-type".into(), url: "".into(), field_name: "".into(),
        };
        
        assert!(Upload::validate(&any_file, &config).is_ok());
    }
    
    #[test]
    fn test_error_messages() {
        let err = UploadError::FileTooLarge { max: 1000, actual: 2000 };
        assert!(err.to_string().contains("2000"));
        assert!(err.to_string().contains("1000"));
        
        let err = UploadError::InvalidMimeType { 
            expected: vec!["image/png".into()], 
            actual: "text/html".into() 
        };
        assert!(err.to_string().contains("text/html"));
        
        let err = UploadError::NoFile;
        assert!(err.to_string().contains("No file"));
        
        let err = UploadError::InvalidContentType;
        assert!(err.to_string().contains("content type"));
    }
    
    #[test]
    fn test_config_chaining() {
        let config = UploadConfig::new()
            .max_size(1_000_000)
            .allowed_types(vec!["image/png", "image/jpeg"])
            .storage("/custom/uploads")
            .keep_original_names();
        
        assert_eq!(config.max_file_size, 1_000_000);
        assert_eq!(config.allowed_types.len(), 2);
        assert_eq!(config.storage_path, PathBuf::from("/custom/uploads"));
        assert!(!config.generate_unique_names);
    }
}

