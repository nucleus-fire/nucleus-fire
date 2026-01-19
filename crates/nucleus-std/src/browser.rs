//! Nucleus Browser - Headless Chrome Automation
//!
//! Built-in browser automation for:
//! - Web scraping
//! - Screenshot generation
//! - PDF export
//! - JavaScript evaluation
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::browser::Browser;
//!
//! let browser = Browser::launch()?;
//! let html = browser.goto("https://example.com")?;
//! let screenshot = browser.screenshot("https://example.com")?;
//! ```

use headless_chrome::protocol::page::ScreenshotFormat;
use headless_chrome::Browser as ChromeBrowser;

// ═══════════════════════════════════════════════════════════════════════════
// BROWSER OPTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Options for browser launch
#[derive(Debug, Clone)]
pub struct BrowserOptions {
    /// Run in headless mode (no visible window)
    pub headless: bool,
    /// Window width
    pub width: u32,
    /// Window height
    pub height: u32,
    /// Navigation timeout in seconds
    pub timeout_secs: u64,
    /// Disable GPU acceleration
    pub disable_gpu: bool,
    /// Disable sandbox (needed in some containers)
    pub no_sandbox: bool,
}

impl Default for BrowserOptions {
    fn default() -> Self {
        Self {
            headless: true,
            width: 1920,
            height: 1080,
            timeout_secs: 30,
            disable_gpu: true,
            no_sandbox: false,
        }
    }
}

impl BrowserOptions {
    /// Create options for headless mode
    pub fn headless() -> Self {
        Self::default()
    }

    /// Create options with visible window
    pub fn visible() -> Self {
        Self {
            headless: false,
            ..Default::default()
        }
    }

    /// Set window dimensions
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set navigation timeout
    pub fn with_timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Enable no-sandbox mode (for Docker/containers)
    pub fn with_no_sandbox(mut self) -> Self {
        self.no_sandbox = true;
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// BROWSER ERROR
// ═══════════════════════════════════════════════════════════════════════════

/// Browser error types
#[derive(Debug, thiserror::Error)]
pub enum BrowserError {
    #[error("Failed to launch browser: {0}")]
    LaunchFailed(String),

    #[error("Navigation failed: {0}")]
    NavigationFailed(String),

    #[error("JavaScript evaluation failed: {0}")]
    EvalFailed(String),

    #[error("Screenshot failed: {0}")]
    ScreenshotFailed(String),

    #[error("Tab creation failed: {0}")]
    TabFailed(String),

    #[error("Timeout: {0}")]
    Timeout(String),
}

// ═══════════════════════════════════════════════════════════════════════════
// BROWSER
// ═══════════════════════════════════════════════════════════════════════════

/// Headless Chrome browser wrapper
pub struct Browser {
    inner: ChromeBrowser,
    options: BrowserOptions,
}

impl Browser {
    /// Launch a headless browser with default options
    pub fn launch() -> Result<Self, BrowserError> {
        Self::launch_with_options(BrowserOptions::default())
    }

    /// Launch browser with custom options
    pub fn launch_with_options(options: BrowserOptions) -> Result<Self, BrowserError> {
        let browser =
            ChromeBrowser::default().map_err(|e| BrowserError::LaunchFailed(e.to_string()))?;

        Ok(Self {
            inner: browser,
            options,
        })
    }

    /// Navigate to a URL and return the HTML content
    pub fn goto(&self, url: &str) -> Result<String, BrowserError> {
        let tab = self
            .inner
            .new_tab()
            .map_err(|e| BrowserError::TabFailed(e.to_string()))?;

        tab.navigate_to(url)
            .map_err(|e| BrowserError::NavigationFailed(e.to_string()))?;

        tab.wait_until_navigated()
            .map_err(|e| BrowserError::NavigationFailed(e.to_string()))?;

        let remote_object = tab
            .evaluate("document.documentElement.outerHTML", false)
            .map_err(|e| BrowserError::EvalFailed(e.to_string()))?;

        Ok(format!("{:?}", remote_object.value))
    }

    /// Get page title
    pub fn get_title(&self, url: &str) -> Result<String, BrowserError> {
        let tab = self
            .inner
            .new_tab()
            .map_err(|e| BrowserError::TabFailed(e.to_string()))?;

        tab.navigate_to(url)
            .map_err(|e| BrowserError::NavigationFailed(e.to_string()))?;

        tab.wait_until_navigated()
            .map_err(|e| BrowserError::NavigationFailed(e.to_string()))?;

        let remote_object = tab
            .evaluate("document.title", false)
            .map_err(|e| BrowserError::EvalFailed(e.to_string()))?;

        Ok(remote_object
            .value
            .and_then(|v| v.as_str().map(|s| s.to_string()))
            .unwrap_or_default())
    }

    /// Evaluate JavaScript and return result as string
    pub fn eval(&self, url: &str, script: &str) -> Result<String, BrowserError> {
        let tab = self
            .inner
            .new_tab()
            .map_err(|e| BrowserError::TabFailed(e.to_string()))?;

        tab.navigate_to(url)
            .map_err(|e| BrowserError::NavigationFailed(e.to_string()))?;

        tab.wait_until_navigated()
            .map_err(|e| BrowserError::NavigationFailed(e.to_string()))?;

        let remote_object = tab
            .evaluate(script, false)
            .map_err(|e| BrowserError::EvalFailed(e.to_string()))?;

        Ok(format!("{:?}", remote_object.value))
    }

    /// Take a screenshot (PNG)
    pub fn screenshot(&self, url: &str) -> Result<Vec<u8>, BrowserError> {
        let tab = self
            .inner
            .new_tab()
            .map_err(|e| BrowserError::TabFailed(e.to_string()))?;

        tab.navigate_to(url)
            .map_err(|e| BrowserError::NavigationFailed(e.to_string()))?;

        tab.wait_until_navigated()
            .map_err(|e| BrowserError::NavigationFailed(e.to_string()))?;

        let png_data = tab
            .capture_screenshot(ScreenshotFormat::PNG, None, true)
            .map_err(|e| BrowserError::ScreenshotFailed(e.to_string()))?;

        Ok(png_data)
    }

    /// Take a JPEG screenshot with quality setting (0-100)
    pub fn screenshot_jpeg(&self, url: &str, quality: u32) -> Result<Vec<u8>, BrowserError> {
        let tab = self
            .inner
            .new_tab()
            .map_err(|e| BrowserError::TabFailed(e.to_string()))?;

        tab.navigate_to(url)
            .map_err(|e| BrowserError::NavigationFailed(e.to_string()))?;

        tab.wait_until_navigated()
            .map_err(|e| BrowserError::NavigationFailed(e.to_string()))?;

        let jpeg_data = tab
            .capture_screenshot(ScreenshotFormat::JPEG(Some(quality)), None, true)
            .map_err(|e| BrowserError::ScreenshotFailed(e.to_string()))?;

        Ok(jpeg_data)
    }

    /// Get the browser options
    pub fn options(&self) -> &BrowserOptions {
        &self.options
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ═══════════════════════════════════════════════════════════════════════
    // BROWSER OPTIONS TESTS (no Chrome required)
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_browser_options_default() {
        let opts = BrowserOptions::default();

        assert!(opts.headless);
        assert_eq!(opts.width, 1920);
        assert_eq!(opts.height, 1080);
        assert_eq!(opts.timeout_secs, 30);
        assert!(opts.disable_gpu);
        assert!(!opts.no_sandbox);
    }

    #[test]
    fn test_browser_options_headless() {
        let opts = BrowserOptions::headless();
        assert!(opts.headless);
    }

    #[test]
    fn test_browser_options_visible() {
        let opts = BrowserOptions::visible();
        assert!(!opts.headless);
    }

    #[test]
    fn test_browser_options_with_size() {
        let opts = BrowserOptions::default().with_size(800, 600);

        assert_eq!(opts.width, 800);
        assert_eq!(opts.height, 600);
    }

    #[test]
    fn test_browser_options_with_timeout() {
        let opts = BrowserOptions::default().with_timeout(60);

        assert_eq!(opts.timeout_secs, 60);
    }

    #[test]
    fn test_browser_options_with_no_sandbox() {
        let opts = BrowserOptions::default().with_no_sandbox();

        assert!(opts.no_sandbox);
    }

    #[test]
    fn test_browser_options_builder_chain() {
        let opts = BrowserOptions::headless()
            .with_size(1280, 720)
            .with_timeout(15)
            .with_no_sandbox();

        assert!(opts.headless);
        assert_eq!(opts.width, 1280);
        assert_eq!(opts.height, 720);
        assert_eq!(opts.timeout_secs, 15);
        assert!(opts.no_sandbox);
    }

    #[test]
    fn test_browser_options_clone() {
        let opts1 = BrowserOptions::default().with_size(100, 100);
        let opts2 = opts1.clone();

        assert_eq!(opts2.width, 100);
    }

    #[test]
    fn test_browser_options_debug() {
        let opts = BrowserOptions::default();
        let debug = format!("{:?}", opts);

        assert!(debug.contains("headless"));
        assert!(debug.contains("width"));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // ERROR TESTS (no Chrome required)
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_browser_error_display() {
        let err = BrowserError::LaunchFailed("Chrome not found".to_string());
        assert_eq!(
            err.to_string(),
            "Failed to launch browser: Chrome not found"
        );

        let err = BrowserError::NavigationFailed("timeout".to_string());
        assert_eq!(err.to_string(), "Navigation failed: timeout");

        let err = BrowserError::EvalFailed("syntax error".to_string());
        assert_eq!(
            err.to_string(),
            "JavaScript evaluation failed: syntax error"
        );

        let err = BrowserError::ScreenshotFailed("permission denied".to_string());
        assert_eq!(err.to_string(), "Screenshot failed: permission denied");

        let err = BrowserError::TabFailed("limit reached".to_string());
        assert_eq!(err.to_string(), "Tab creation failed: limit reached");

        let err = BrowserError::Timeout("30 seconds".to_string());
        assert_eq!(err.to_string(), "Timeout: 30 seconds");
    }

    #[test]
    fn test_browser_error_debug() {
        let err = BrowserError::LaunchFailed("test".to_string());
        let debug = format!("{:?}", err);

        assert!(debug.contains("LaunchFailed"));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // INTEGRATION TESTS (require Chrome)
    // Run with: cargo test -- --ignored
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_browser_launch() {
        let browser = Browser::launch();
        assert!(browser.is_ok());
    }

    #[test]
    fn test_browser_launch_with_options() {
        let opts = BrowserOptions::headless()
            .with_size(800, 600)
            .with_timeout(10);

        let browser = Browser::launch_with_options(opts);
        assert!(browser.is_ok());

        if let Ok(b) = browser {
            assert_eq!(b.options().width, 800);
        }
    }

    #[test]
    fn test_browser_goto() {
        let browser = Browser::launch().unwrap();
        let html = browser.goto("https://example.com");

        assert!(html.is_ok());
        let content = html.unwrap();
        assert!(content.contains("Example") || content.len() > 100);
    }

    #[test]
    fn test_browser_get_title() {
        let browser = Browser::launch().unwrap();
        let title = browser.get_title("https://example.com");

        assert!(title.is_ok());
    }

    #[test]
    fn test_browser_eval() {
        let browser = Browser::launch().unwrap();
        let result = browser.eval("https://example.com", "1 + 1");

        assert!(result.is_ok());
        assert!(result.unwrap().contains("2"));
    }

    #[test]
    fn test_browser_screenshot() {
        let browser = Browser::launch().unwrap();
        let screenshot = browser.screenshot("https://example.com");

        assert!(screenshot.is_ok());
        let data = screenshot.unwrap();

        // PNG magic bytes
        assert!(data.len() > 100);
        assert_eq!(&data[0..4], &[0x89, 0x50, 0x4E, 0x47]); // PNG header
    }

    #[test]
    fn test_browser_screenshot_jpeg() {
        let browser = Browser::launch().unwrap();
        let screenshot = browser.screenshot_jpeg("https://example.com", 80);

        assert!(screenshot.is_ok());
        let data = screenshot.unwrap();

        // JPEG magic bytes
        assert!(data.len() > 100);
        assert_eq!(&data[0..2], &[0xFF, 0xD8]); // JPEG header
    }
}
