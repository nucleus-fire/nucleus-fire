# Image Processing Guide

Process images with Lens - resize, crop, convert, and transform.

---

## Quick Start

```rust
use nucleus_std::lens::{Lens, ImageFormat};

// Read image file
let data = std::fs::read("photo.jpg")?;

// Resize to exact dimensions
let resized = Lens::resize(&data, 800, 600)?;

// Save result
std::fs::write("photo_resized.jpg", resized)?;
```

---

## Operations

### Resize

```rust
// Exact dimensions (may distort)
let exact = Lens::resize(&data, 800, 600)?;

// Fit within bounds (preserves aspect ratio)
let fit = Lens::resize_fit(&data, 800, 600)?;

// Square thumbnail
let thumb = Lens::thumbnail(&data, 150)?;
```

### Crop

```rust
// Crop region: x, y, width, height
let cropped = Lens::crop(&data, 100, 100, 400, 300)?;
```

### Format Conversion

```rust
use nucleus_std::lens::ImageFormat;

// Convert to JPEG with quality (1-100)
let jpeg = Lens::convert(&data, ImageFormat::Jpeg(85))?;

// Convert to PNG (lossless)
let png = Lens::convert(&data, ImageFormat::Png)?;

// Convert to WebP with quality
let webp = Lens::convert(&data, ImageFormat::WebP(80))?;
```

### Effects

```rust
// Gaussian blur (sigma controls intensity)
let blurred = Lens::blur(&data, 5.0)?;

// Adjust brightness (-100 to 100)
let bright = Lens::brightness(&data, 30)?;

// Adjust contrast
let contrast = Lens::contrast(&data, 1.5)?;

// Grayscale
let gray = Lens::grayscale(&data)?;

// Invert colors
let inverted = Lens::invert(&data)?;
```

### Transformations

```rust
// Flip
let h_flip = Lens::flip_horizontal(&data)?;
let v_flip = Lens::flip_vertical(&data)?;

// Rotate
let r90 = Lens::rotate_90(&data)?;   // 90° clockwise
let r180 = Lens::rotate_180(&data)?; // 180°
let r270 = Lens::rotate_270(&data)?; // 270° (90° counter-clockwise)
```

### Get Dimensions

```rust
let dims = Lens::dimensions(&data)?;
println!("{}x{}", dims.width, dims.height);
```

---

## Image Formats

| Format | Extension | Quality | Best For |
|--------|-----------|---------|----------|
| `Jpeg(q)` | .jpg | 1-100 | Photos |
| `Png` | .png | Lossless | Graphics, transparency |
| `WebP(q)` | .webp | 1-100 | Web (smaller files) |
| `Gif` | .gif | - | Animations |
| `Bmp` | .bmp | Lossless | Raw/uncompressed |

```rust
let format = ImageFormat::Jpeg(85);
println!("Extension: {}", format.extension()); // "jpg"
println!("MIME: {}", format.mime_type());      // "image/jpeg"
```

---

## Error Handling

```rust
match Lens::resize(&data, 800, 600) {
    Ok(resized) => {
        std::fs::write("output.jpg", resized)?;
    }
    Err(e) => {
        eprintln!("Image error: {}", e);
    }
}
```

---

## Web Server Example

```rust
use axum::{extract::Multipart, response::Response, http::header};

async fn upload_avatar(mut multipart: Multipart) -> Response {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();
        
        // Create 200x200 thumbnail
        let thumb = Lens::thumbnail(&data, 200).unwrap();
        
        return Response::builder()
            .header(header::CONTENT_TYPE, "image/jpeg")
            .body(thumb.into())
            .unwrap();
    }
    
    Response::builder().status(400).body("No file".into()).unwrap()
}
```

---

## Performance Tips

1. **Use thumbnails**: Generate small previews for listings
2. **WebP for web**: 25-35% smaller than JPEG at same quality
3. **Lazy processing**: Process images on first request, cache results
4. **Quality 80-85**: Best balance of size and quality for JPEG
