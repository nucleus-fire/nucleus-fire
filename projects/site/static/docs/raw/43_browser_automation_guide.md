# Browser Automation Guide

Nucleus Browser provides built-in headless Chrome automation for web scraping, screenshots, and testing.

## Prerequisites

Browser automation requires Chrome or Chromium to be installed.

### Check Installation

```bash
nucleus browser check
```

### Install Chrome/Chromium

```bash
# Automatic installation (macOS/Linux)
nucleus browser install

# Or install manually:
# macOS: brew install --cask chromium
# Ubuntu/Debian: sudo apt install chromium-browser
# Fedora: sudo dnf install chromium
# Windows: https://www.google.com/chrome/
```

## Quick Start

```rust
use nucleus_std::browser::{Browser, BrowserOptions, BrowserError};

// Launch browser
let browser = Browser::launch()?;

// Navigate and get HTML
let html = browser.goto("https://example.com")?;

// Take screenshot
let png = browser.screenshot("https://example.com")?;
std::fs::write("screenshot.png", png)?;
```

> [!TIP]
> If Chrome is not installed, `Browser::launch()` will return a `BrowserError::LaunchFailed` error with a helpful message.

## Configuration

### Browser Options

```rust
// Default (headless, 1920x1080)
let browser = Browser::launch()?;

// Custom options
let opts = BrowserOptions::headless()
    .with_size(1280, 720)
    .with_timeout(60);

let browser = Browser::launch_with_options(opts)?;
```

### Visible Mode (for debugging)

```rust
let opts = BrowserOptions::visible()
    .with_size(1024, 768);

let browser = Browser::launch_with_options(opts)?;
```

### Docker/Container Mode

```rust
// Disable sandbox for containers
let opts = BrowserOptions::headless()
    .with_no_sandbox();

let browser = Browser::launch_with_options(opts)?;
```

## Core Operations

### Navigate to URL

```rust
let html = browser.goto("https://example.com")?;
println!("Page HTML: {}", html);
```

### Get Page Title

```rust
let title = browser.get_title("https://example.com")?;
println!("Title: {}", title);
```

### Execute JavaScript

```rust
// Simple evaluation
let result = browser.eval("https://example.com", "1 + 1")?;
println!("Result: {}", result); // "2"

// Get element text
let text = browser.eval(
    "https://example.com",
    "document.querySelector('h1').textContent"
)?;

// Check if element exists
let exists = browser.eval(
    "https://example.com",
    "!!document.querySelector('.my-class')"
)?;
```

## Screenshots

### PNG (lossless)

```rust
let png = browser.screenshot("https://example.com")?;
std::fs::write("screenshot.png", png)?;

// Verify it's a valid PNG
assert_eq!(&png[0..4], &[0x89, 0x50, 0x4E, 0x47]);
```

### JPEG (smaller file size)

```rust
// Quality 0-100
let jpeg = browser.screenshot_jpeg("https://example.com", 80)?;
std::fs::write("screenshot.jpg", jpeg)?;
```

## Error Handling

```rust
use nucleus_std::browser::BrowserError;

match browser.goto("https://example.com") {
    Ok(html) => println!("Got HTML"),
    Err(BrowserError::LaunchFailed(e)) => {
        eprintln!("Chrome not found: {}", e);
    }
    Err(BrowserError::NavigationFailed(e)) => {
        eprintln!("Navigation failed: {}", e);
    }
    Err(BrowserError::TabFailed(e)) => {
        eprintln!("Tab creation failed: {}", e);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Practical Examples

### Web Scraping

```rust
let browser = Browser::launch()?;

let html = browser.goto("https://news.ycombinator.com")?;

// Extract data with JavaScript
let titles = browser.eval(
    "https://news.ycombinator.com",
    r#"
    Array.from(document.querySelectorAll('.titleline > a'))
        .map(a => a.textContent)
        .slice(0, 10)
        .join('\n')
    "#
)?;

println!("Top stories:\n{}", titles);
```

### Screenshot Service

```rust
async fn screenshot_handler(url: &str) -> Result<Vec<u8>, BrowserError> {
    let browser = Browser::launch()?;
    browser.screenshot(url)
}

// Usage
let png = screenshot_handler("https://github.com")?;
```

### SEO Checker

```rust
let browser = Browser::launch()?;
let url = "https://example.com";

let title = browser.get_title(url)?;
let description = browser.eval(url, 
    "document.querySelector('meta[name=\"description\"]')?.content || 'None'"
)?;
let h1_count = browser.eval(url, 
    "document.querySelectorAll('h1').length"
)?;

println!("SEO Report for {}:", url);
println!("  Title: {}", title);
println!("  Description: {}", description);
println!("  H1 count: {}", h1_count);
```

### PDF Generation (via print)

```rust
// Navigate first
let _ = browser.goto("https://example.com")?;

// Use JavaScript to trigger print
browser.eval(
    "https://example.com",
    "window.print()"
)?;
```

## Docker Setup

```dockerfile
FROM rust:1.75 as builder
# ... build your app ...

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    chromium \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

ENV CHROME_PATH=/usr/bin/chromium

COPY --from=builder /app/target/release/myapp /app/myapp
CMD ["/app/myapp"]
```

## Best Practices

1. **Reuse browser instances** - Creating browsers is expensive
2. **Set reasonable timeouts** - Pages can be slow
3. **Use headless mode in production** - Visible mode is for debugging
4. **Enable no-sandbox in containers** - Required for Docker
5. **Handle errors gracefully** - Network can fail
6. **Close tabs/browsers** - Prevent memory leaks
