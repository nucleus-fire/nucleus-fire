# File Upload Guide

Nucleus Upload provides secure file upload handling with validation, storage, and metadata extraction.

## Quick Start

```rust
use nucleus_std::upload::{Upload, UploadConfig, UploadedFile};

// Configure upload
let config = UploadConfig::new()
    .max_size(10 * 1024 * 1024)  // 10MB
    .allowed_types(vec!["image/png", "image/jpeg"]);

// Handle upload (Static API)
async fn upload_handler(
    headers: HeaderMap,
    body: Body,
) -> Result<Json<Vec<UploadedFile>>, UploadError> {
    let content_type = headers
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let files = Upload::from_multipart(body, content_type, &config).await?;
    
    Ok(Json(files))
}
```

## Configuration

### Basic Setup

```rust
let config = UploadConfig::new()
    .max_size(5 * 1024 * 1024)           // 5MB max
    .storage("./uploads")                // Storage directory
    .allowed_types(vec!["image/*"]);     // MIME types
```

### Presets

```rust
// Image only
let config = UploadConfig::images_only()
    .max_size(2 * 1024 * 1024);

// Documents only
let config = UploadConfig::documents_only()
    .max_size(20 * 1024 * 1024);
```

### Options

```rust
let config = UploadConfig::new()
    .keep_original_names(true)  // Disable unique UUID filenames
    .preserve_extension(true);  // Keep or discard extension
```

## Handling Uploads

### Multiple Files (Multipart)

```rust
// Parse entire multipart body
let files = Upload::from_multipart(body, content_type, &config).await?;

for file in files {
    println!("Saved: {}", file.path.display());
}
```

### Single File

```rust
// Expect exactly one file (returns error if 0 or >1)
let file = Upload::single(body, content_type, &config).await?;
```

## UploadedFile Structure

```rust
pub struct UploadedFile {
    pub original_name: String,      // User's filename
    pub stored_name: String,        // Saved filename (UUID-based)
    pub path: PathBuf,              // Full path to file
    pub size: usize,                // File size in bytes
    pub mime_type: String,          // MIME type
    pub url: String,                // Relative URL
    pub field_name: String,         // Form field name
}
```

## Manual Validation

If you have an `UploadedFile` (e.g., from another source), you can validate it manually:

```rust
// Check if file meets config (size, mime)
Upload::validate(&file, &config)?;
```

## Cleanup

```rust
// Delete file
Upload::delete(&file)?;

// Clean up old files (e.g. temporary uploads)
// Delete files older than 24 hours
Upload::cleanup_old_files(path, Duration::from_secs(86400))?;
```

## Axum Integration Pattern

```rust
use axum::{
    body::Body,
    extract::State,
    http::HeaderMap,
    response::IntoResponse,
};

async fn upload_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Body,
) -> Result<impl IntoResponse, AppError> {
    let content_type = headers
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::BadRequest("Missing content type".into()))?;

    let config = UploadConfig::images_only();
    
    let files = Upload::from_multipart(body, content_type, &config).await
        .map_err(|e| AppError::UploadFailed(e.to_string()))?;
        
    Ok(Json(files))
}
```

## Security Best Practices

1. **Always validate file types** - The module does this if `allowed_types` is set.
2. **Use UUID filenames** - Default is true (`generate_unique_names`).
3. **Store outside webroot** - Serve via a route handler or static services.
4. **Set size limits** - Always use `max_size`.
