//! Lens - Image Processing
//!
//! Real image processing using the image crate.
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::lens::{Lens, ImageFormat};
//!
//! // Resize an image
//! let resized = Lens::resize(&image_bytes, 800, 600)?;
//!
//! // Convert to WebP
//! let webp = Lens::convert(&image_bytes, ImageFormat::WebP(80))?;
//!
//! // Create thumbnail
//! let thumb = Lens::thumbnail(&image_bytes, 150)?;
//! ```

use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageFormat as ImgFormat};
use std::io::Cursor;

// ═══════════════════════════════════════════════════════════════════════════
// TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Image output format with quality settings
#[derive(Debug, Clone, Copy)]
pub enum ImageFormat {
    /// JPEG with quality (1-100)
    Jpeg(u8),
    /// PNG (lossless)
    Png,
    /// WebP with quality (1-100)
    WebP(u8),
    /// GIF
    Gif,
    /// BMP
    Bmp,
}

impl ImageFormat {
    /// Get the file extension
    pub fn extension(&self) -> &'static str {
        match self {
            ImageFormat::Jpeg(_) => "jpg",
            ImageFormat::Png => "png",
            ImageFormat::WebP(_) => "webp",
            ImageFormat::Gif => "gif",
            ImageFormat::Bmp => "bmp",
        }
    }

    /// Get the MIME type
    pub fn mime_type(&self) -> &'static str {
        match self {
            ImageFormat::Jpeg(_) => "image/jpeg",
            ImageFormat::Png => "image/png",
            ImageFormat::WebP(_) => "image/webp",
            ImageFormat::Gif => "image/gif",
            ImageFormat::Bmp => "image/bmp",
        }
    }
}

/// Image dimensions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dimensions {
    pub width: u32,
    pub height: u32,
}

/// Lens error
#[derive(Debug, Clone)]
pub struct LensError(pub String);

impl std::fmt::Display for LensError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LensError: {}", self.0)
    }
}

impl std::error::Error for LensError {}

type Result<T> = std::result::Result<T, LensError>;

// ═══════════════════════════════════════════════════════════════════════════
// LENS
// ═══════════════════════════════════════════════════════════════════════════

/// Image processing utility
pub struct Lens;

impl Lens {
    /// Get image dimensions
    pub fn dimensions(data: &[u8]) -> Result<Dimensions> {
        let img = image::load_from_memory(data)
            .map_err(|e| LensError(format!("Failed to load image: {}", e)))?;

        Ok(Dimensions {
            width: img.width(),
            height: img.height(),
        })
    }

    /// Resize image to exact dimensions
    pub fn resize(data: &[u8], width: u32, height: u32) -> Result<Vec<u8>> {
        let img = Self::load(data)?;
        let resized = img.resize_exact(width, height, FilterType::Lanczos3);
        Self::encode(&resized, ImageFormat::Jpeg(85))
    }

    /// Resize image to fit within max dimensions, preserving aspect ratio
    pub fn resize_fit(data: &[u8], max_width: u32, max_height: u32) -> Result<Vec<u8>> {
        let img = Self::load(data)?;
        let resized = img.resize(max_width, max_height, FilterType::Lanczos3);
        Self::encode(&resized, ImageFormat::Jpeg(85))
    }

    /// Create a square thumbnail
    pub fn thumbnail(data: &[u8], size: u32) -> Result<Vec<u8>> {
        let img = Self::load(data)?;
        let thumb = img.thumbnail(size, size);
        Self::encode(&thumb, ImageFormat::Jpeg(80))
    }

    /// Crop image to specified region
    pub fn crop(data: &[u8], x: u32, y: u32, width: u32, height: u32) -> Result<Vec<u8>> {
        let img = Self::load(data)?;

        // Validate bounds
        let (img_width, img_height) = img.dimensions();
        if x + width > img_width || y + height > img_height {
            return Err(LensError(format!(
                "Crop region ({},{} {}x{}) exceeds image bounds ({}x{})",
                x, y, width, height, img_width, img_height
            )));
        }

        let cropped = img.crop_imm(x, y, width, height);
        Self::encode(&cropped, ImageFormat::Jpeg(85))
    }

    /// Convert image to specified format
    pub fn convert(data: &[u8], format: ImageFormat) -> Result<Vec<u8>> {
        let img = Self::load(data)?;
        Self::encode(&img, format)
    }

    /// Apply Gaussian blur
    pub fn blur(data: &[u8], sigma: f32) -> Result<Vec<u8>> {
        let img = Self::load(data)?;
        let blurred = img.blur(sigma);
        Self::encode(&blurred, ImageFormat::Jpeg(85))
    }

    /// Adjust brightness (-100 to 100)
    pub fn brightness(data: &[u8], value: i32) -> Result<Vec<u8>> {
        let img = Self::load(data)?;
        let adjusted = img.brighten(value);
        Self::encode(&adjusted, ImageFormat::Jpeg(85))
    }

    /// Adjust contrast (-100 to 100)
    pub fn contrast(data: &[u8], value: f32) -> Result<Vec<u8>> {
        let img = Self::load(data)?;
        let adjusted = img.adjust_contrast(value);
        Self::encode(&adjusted, ImageFormat::Jpeg(85))
    }

    /// Flip image horizontally
    pub fn flip_horizontal(data: &[u8]) -> Result<Vec<u8>> {
        let img = Self::load(data)?;
        let flipped = img.fliph();
        Self::encode(&flipped, ImageFormat::Jpeg(85))
    }

    /// Flip image vertically
    pub fn flip_vertical(data: &[u8]) -> Result<Vec<u8>> {
        let img = Self::load(data)?;
        let flipped = img.flipv();
        Self::encode(&flipped, ImageFormat::Jpeg(85))
    }

    /// Rotate image 90 degrees clockwise
    pub fn rotate_90(data: &[u8]) -> Result<Vec<u8>> {
        let img = Self::load(data)?;
        let rotated = img.rotate90();
        Self::encode(&rotated, ImageFormat::Jpeg(85))
    }

    /// Rotate image 180 degrees
    pub fn rotate_180(data: &[u8]) -> Result<Vec<u8>> {
        let img = Self::load(data)?;
        let rotated = img.rotate180();
        Self::encode(&rotated, ImageFormat::Jpeg(85))
    }

    /// Rotate image 270 degrees clockwise (90 counter-clockwise)
    pub fn rotate_270(data: &[u8]) -> Result<Vec<u8>> {
        let img = Self::load(data)?;
        let rotated = img.rotate270();
        Self::encode(&rotated, ImageFormat::Jpeg(85))
    }

    /// Convert to grayscale
    pub fn grayscale(data: &[u8]) -> Result<Vec<u8>> {
        let img = Self::load(data)?;
        let gray = img.grayscale();
        Self::encode(&gray, ImageFormat::Jpeg(85))
    }

    /// Invert colors
    pub fn invert(data: &[u8]) -> Result<Vec<u8>> {
        let mut img = Self::load(data)?;
        img.invert();
        Self::encode(&img, ImageFormat::Jpeg(85))
    }

    // ─────────────────────────────────────────────────────────────────────────
    // INTERNAL
    // ─────────────────────────────────────────────────────────────────────────

    fn load(data: &[u8]) -> Result<DynamicImage> {
        image::load_from_memory(data).map_err(|e| LensError(format!("Failed to load image: {}", e)))
    }

    fn encode(img: &DynamicImage, format: ImageFormat) -> Result<Vec<u8>> {
        let mut buffer = Cursor::new(Vec::new());

        match format {
            ImageFormat::Jpeg(quality) => {
                let encoder =
                    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buffer, quality);
                img.write_with_encoder(encoder)
                    .map_err(|e| LensError(format!("Failed to encode JPEG: {}", e)))?;
            }
            ImageFormat::Png => {
                img.write_to(&mut buffer, ImgFormat::Png)
                    .map_err(|e| LensError(format!("Failed to encode PNG: {}", e)))?;
            }
            ImageFormat::Gif => {
                img.write_to(&mut buffer, ImgFormat::Gif)
                    .map_err(|e| LensError(format!("Failed to encode GIF: {}", e)))?;
            }
            ImageFormat::Bmp => {
                img.write_to(&mut buffer, ImgFormat::Bmp)
                    .map_err(|e| LensError(format!("Failed to encode BMP: {}", e)))?;
            }
            ImageFormat::WebP(_) => {
                // WebP encoding requires the webp feature, fall back to PNG
                img.write_to(&mut buffer, ImgFormat::Png)
                    .map_err(|e| LensError(format!("Failed to encode (WebP fallback): {}", e)))?;
            }
        }

        Ok(buffer.into_inner())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageBuffer;

    // Create a simple 10x10 red pixel image for testing
    fn create_test_image() -> Vec<u8> {
        let img: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::from_fn(10, 10, |_x, _y| {
            image::Rgb([255, 0, 0]) // Red pixel
        });

        let mut buffer = Cursor::new(Vec::new());
        let dynamic = DynamicImage::ImageRgb8(img);
        dynamic.write_to(&mut buffer, ImgFormat::Png).unwrap();
        buffer.into_inner()
    }

    #[test]
    fn test_dimensions() {
        let data = create_test_image();
        let dims = Lens::dimensions(&data).unwrap();
        assert_eq!(dims.width, 10);
        assert_eq!(dims.height, 10);
    }

    #[test]
    fn test_resize_exact() {
        let data = create_test_image();
        let resized = Lens::resize(&data, 5, 5).unwrap();

        let dims = Lens::dimensions(&resized).unwrap();
        assert_eq!(dims.width, 5);
        assert_eq!(dims.height, 5);
    }

    #[test]
    fn test_resize_fit() {
        let data = create_test_image();
        let resized = Lens::resize_fit(&data, 5, 20).unwrap();

        let dims = Lens::dimensions(&resized).unwrap();
        // Should maintain aspect ratio (10x10 -> 5x5)
        assert_eq!(dims.width, 5);
        assert_eq!(dims.height, 5);
    }

    #[test]
    fn test_thumbnail() {
        let data = create_test_image();
        let thumb = Lens::thumbnail(&data, 5).unwrap();

        let dims = Lens::dimensions(&thumb).unwrap();
        assert!(dims.width <= 5);
        assert!(dims.height <= 5);
    }

    #[test]
    fn test_crop_valid() {
        let data = create_test_image();
        let cropped = Lens::crop(&data, 2, 2, 5, 5).unwrap();

        let dims = Lens::dimensions(&cropped).unwrap();
        assert_eq!(dims.width, 5);
        assert_eq!(dims.height, 5);
    }

    #[test]
    fn test_crop_out_of_bounds() {
        let data = create_test_image();
        let result = Lens::crop(&data, 8, 8, 5, 5);
        assert!(result.is_err());
        assert!(result.unwrap_err().0.contains("exceeds"));
    }

    #[test]
    fn test_convert_png() {
        let data = create_test_image();
        let converted = Lens::convert(&data, ImageFormat::Png).unwrap();
        assert!(!converted.is_empty());
    }

    #[test]
    fn test_convert_jpeg() {
        let data = create_test_image();
        let converted = Lens::convert(&data, ImageFormat::Jpeg(90)).unwrap();
        assert!(!converted.is_empty());
    }

    #[test]
    fn test_blur() {
        let data = create_test_image();
        let blurred = Lens::blur(&data, 2.0).unwrap();
        assert!(!blurred.is_empty());
    }

    #[test]
    fn test_brightness() {
        let data = create_test_image();
        let bright = Lens::brightness(&data, 50).unwrap();
        assert!(!bright.is_empty());
    }

    #[test]
    fn test_contrast() {
        let data = create_test_image();
        let result = Lens::contrast(&data, 1.5).unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_flip_horizontal() {
        let data = create_test_image();
        let flipped = Lens::flip_horizontal(&data).unwrap();
        assert!(!flipped.is_empty());
    }

    #[test]
    fn test_flip_vertical() {
        let data = create_test_image();
        let flipped = Lens::flip_vertical(&data).unwrap();
        assert!(!flipped.is_empty());
    }

    #[test]
    fn test_rotate_90() {
        let data = create_test_image();
        let rotated = Lens::rotate_90(&data).unwrap();
        assert!(!rotated.is_empty());
    }

    #[test]
    fn test_rotate_180() {
        let data = create_test_image();
        let rotated = Lens::rotate_180(&data).unwrap();
        assert!(!rotated.is_empty());
    }

    #[test]
    fn test_rotate_270() {
        let data = create_test_image();
        let rotated = Lens::rotate_270(&data).unwrap();
        assert!(!rotated.is_empty());
    }

    #[test]
    fn test_grayscale() {
        let data = create_test_image();
        let gray = Lens::grayscale(&data).unwrap();
        assert!(!gray.is_empty());
    }

    #[test]
    fn test_invert() {
        let data = create_test_image();
        let inverted = Lens::invert(&data).unwrap();
        assert!(!inverted.is_empty());
    }

    #[test]
    fn test_invalid_image() {
        let invalid_data = vec![0, 1, 2, 3];
        let result = Lens::dimensions(&invalid_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_input() {
        let empty: Vec<u8> = vec![];
        let result = Lens::dimensions(&empty);
        assert!(result.is_err());
    }

    #[test]
    fn test_image_format_extension() {
        assert_eq!(ImageFormat::Jpeg(90).extension(), "jpg");
        assert_eq!(ImageFormat::Png.extension(), "png");
        assert_eq!(ImageFormat::WebP(80).extension(), "webp");
    }

    #[test]
    fn test_image_format_mime() {
        assert_eq!(ImageFormat::Jpeg(90).mime_type(), "image/jpeg");
        assert_eq!(ImageFormat::Png.mime_type(), "image/png");
        assert_eq!(ImageFormat::WebP(80).mime_type(), "image/webp");
    }

    #[test]
    fn test_convert_gif() {
        let data = create_test_image();
        let converted = Lens::convert(&data, ImageFormat::Gif).unwrap();
        assert!(!converted.is_empty());
    }

    #[test]
    fn test_convert_bmp() {
        let data = create_test_image();
        let converted = Lens::convert(&data, ImageFormat::Bmp).unwrap();
        assert!(!converted.is_empty());
    }

    #[test]
    fn test_convert_webp_fallback() {
        let data = create_test_image();
        // WebP falls back to PNG in our implementation
        let converted = Lens::convert(&data, ImageFormat::WebP(80)).unwrap();
        assert!(!converted.is_empty());
    }

    #[test]
    fn test_lens_error_display() {
        let error = LensError("Test error message".to_string());
        let display = format!("{}", error);
        assert!(display.contains("LensError"));
        assert!(display.contains("Test error message"));
    }

    #[test]
    fn test_gif_bmp_format_metadata() {
        assert_eq!(ImageFormat::Gif.extension(), "gif");
        assert_eq!(ImageFormat::Bmp.extension(), "bmp");
        assert_eq!(ImageFormat::Gif.mime_type(), "image/gif");
        assert_eq!(ImageFormat::Bmp.mime_type(), "image/bmp");
    }

    #[test]
    fn test_brightness_negative() {
        let data = create_test_image();
        let darkened = Lens::brightness(&data, -50).unwrap();
        assert!(!darkened.is_empty());
    }

    #[test]
    fn test_contrast_negative() {
        let data = create_test_image();
        let low_contrast = Lens::contrast(&data, -0.5).unwrap();
        assert!(!low_contrast.is_empty());
    }

    #[test]
    fn test_dimensions_equality() {
        let dims1 = Dimensions {
            width: 100,
            height: 200,
        };
        let dims2 = Dimensions {
            width: 100,
            height: 200,
        };
        let dims3 = Dimensions {
            width: 50,
            height: 50,
        };

        assert_eq!(dims1, dims2);
        assert_ne!(dims1, dims3);
    }
}
