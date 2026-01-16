<n:script>
    use nucleus_std::lens::{Lens, ImageFormat, Dimensions};
    use std::io::Cursor;

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 1: Avatar Upload Pipeline
    // ═══════════════════════════════════════════════════════════════════════════

    async fn upload_avatar(user_id: &str, raw_bytes: Vec<u8>) -> Result<Vec<u8>, String> {
        // Resize to 200x200 square thumbnail
        let resized = Lens::resize(&raw_bytes, 200, 200)?;
        
        // Convert to WebP for smaller file size (falls back to PNG if WebP unavailable)
        let optimized = Lens::convert(&resized, ImageFormat::WebP(80))?;
        
        // In production: save to S3/disk
        // save_file(&format!("avatars/{}.webp", user_id), &optimized).await;
        
        Ok(optimized)
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 2: Responsive Image Generation
    // ═══════════════════════════════════════════════════════════════════════════

    struct ResponsiveImages {
        thumbnail: Vec<u8>,  // 150px
        medium: Vec<u8>,     // 600px
        large: Vec<u8>,      // 1200px
    }

    fn generate_responsive_images(raw: &[u8]) -> Result<ResponsiveImages, String> {
        let dims = Lens::dimensions(raw)?;
        
        Ok(ResponsiveImages {
            thumbnail: Lens::thumbnail(raw, 150)?,
            medium: Lens::resize_fit(raw, 600, 600)?,
            large: Lens::resize_fit(raw, 1200, 1200)?,
        })
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 3: Image Upload Handler (Axum)
    // ═══════════════════════════════════════════════════════════════════════════

    use axum::{extract::Multipart, response::Response, http::header};

    async fn handle_image_upload(mut multipart: Multipart) -> Response {
        while let Some(field) = multipart.next_field().await.unwrap() {
            let data = field.bytes().await.unwrap();
            
            // Validate image
            match Lens::dimensions(&data) {
                Ok(dims) => {
                    // Create thumbnail
                    let thumb = Lens::thumbnail(&data, 200)
                        .map_err(|e| format!("Processing failed: {}", e))
                        .unwrap();
                    
                    return Response::builder()
                        .header(header::CONTENT_TYPE, "image/jpeg")
                        .body(thumb.into())
                        .unwrap();
                }
                Err(e) => {
                    return Response::builder()
                        .status(400)
                        .body(format!("Invalid image: {}", e).into())
                        .unwrap();
                }
            }
        }
        
        Response::builder()
            .status(400)
            .body("No file uploaded".into())
            .unwrap()
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 4: Image Effects Pipeline
    // ═══════════════════════════════════════════════════════════════════════════

    fn apply_vintage_filter(raw: &[u8]) -> Result<Vec<u8>, String> {
        // Step 1: Convert to grayscale
        let gray = Lens::grayscale(raw)?;
        
        // Step 2: Adjust contrast
        let contrast = Lens::contrast(&gray, 1.2)?;
        
        // Step 3: Slight blur for soft look
        let soft = Lens::blur(&contrast, 0.5)?;
        
        // Step 4: Adjust brightness
        let final_image = Lens::brightness(&soft, -10)?;
        
        Ok(final_image)
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // EXAMPLE 5: Crop to Social Media Sizes
    // ═══════════════════════════════════════════════════════════════════════════

    fn crop_for_instagram(raw: &[u8]) -> Result<Vec<u8>, String> {
        let dims = Lens::dimensions(raw)?;
        
        // Calculate center crop for 1:1 aspect ratio
        let size = dims.width.min(dims.height);
        let x = (dims.width - size) / 2;
        let y = (dims.height - size) / 2;
        
        let cropped = Lens::crop(raw, x, y, size, size)?;
        
        // Resize to Instagram standard 1080x1080
        Lens::resize(&cropped, 1080, 1080)
    }

    fn crop_for_twitter_header(raw: &[u8]) -> Result<Vec<u8>, String> {
        let dims = Lens::dimensions(raw)?;
        
        // Twitter header: 1500x500 (3:1 ratio)
        let target_ratio = 3.0;
        let current_ratio = dims.width as f64 / dims.height as f64;
        
        let (crop_w, crop_h, x, y) = if current_ratio > target_ratio {
            // Image is wider, crop sides
            let new_w = (dims.height as f64 * target_ratio) as u32;
            (new_w, dims.height, (dims.width - new_w) / 2, 0)
        } else {
            // Image is taller, crop top/bottom
            let new_h = (dims.width as f64 / target_ratio) as u32;
            (dims.width, new_h, 0, (dims.height - new_h) / 2)
        };
        
        let cropped = Lens::crop(raw, x, y, crop_w, crop_h)?;
        Lens::resize(&cropped, 1500, 500)
    }
</n:script>
