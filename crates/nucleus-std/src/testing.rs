//! Nucleus Testing Utilities
//!
//! Comprehensive testing helpers for Nucleus applications:
//! - MockServer for HTTP mocking
//! - TestClient for request simulation
//! - Factory pattern for test data generation
//!
//! # Example
//!
//! ```rust,ignore
//! use nucleus_std::testing::{MockServer, TestClient, Factory};
//!
//! // Mock external APIs
//! let mock = MockServer::start().await;
//! mock.expect("POST", "/api/users")
//!     .with_status(201)
//!     .with_json(json!({"id": 1}))
//!     .times(1);
//!
//! // Create test data
//! let user = Factory::user()
//!     .email("test@example.com")
//!     .name("Test User")
//!     .build();
//!
//! // Test client with auth
//! let client = TestClient::new(app).with_auth(user);
//! let response = client.get("/dashboard").await;
//! assert_eq!(response.status(), 200);
//! ```

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

// ═══════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════

/// Testing error types
#[derive(Debug, thiserror::Error)]
pub enum TestError {
    #[error("Server failed to start: {0}")]
    ServerError(String),

    #[error("Request failed: {0}")]
    RequestError(String),

    #[error("Assertion failed: {0}")]
    AssertionFailed(String),

    #[error("Timeout waiting for condition")]
    Timeout,

    #[error("Mock not matched: {method} {path}")]
    MockNotMatched { method: String, path: String },

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl From<reqwest::Error> for TestError {
    fn from(err: reqwest::Error) -> Self {
        TestError::RequestError(err.to_string())
    }
}

impl From<serde_json::Error> for TestError {
    fn from(err: serde_json::Error) -> Self {
        TestError::SerializationError(err.to_string())
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MOCK EXPECTATION
// ═══════════════════════════════════════════════════════════════════════════

/// Expected request/response pair
#[derive(Debug, Clone)]
pub struct MockExpectation {
    pub method: String,
    pub path: String,
    pub request_headers: HashMap<String, String>,
    pub request_body: Option<serde_json::Value>,
    pub response_status: u16,
    pub response_headers: HashMap<String, String>,
    pub response_body: Option<serde_json::Value>,
    pub times: Option<usize>,
    pub hits: usize,
}

impl MockExpectation {
    fn new(method: &str, path: &str) -> Self {
        Self {
            method: method.to_uppercase(),
            path: path.to_string(),
            request_headers: HashMap::new(),
            request_body: None,
            response_status: 200,
            response_headers: HashMap::new(),
            response_body: None,
            times: None,
            hits: 0,
        }
    }

    fn matches(&self, method: &str, path: &str) -> bool {
        self.method == method.to_uppercase() && self.path == path
    }
}

/// Builder for mock expectation
pub struct MockExpectationBuilder {
    expectation: MockExpectation,
    server: Arc<RwLock<Vec<MockExpectation>>>,
}

impl MockExpectationBuilder {
    fn new(server: Arc<RwLock<Vec<MockExpectation>>>, method: &str, path: &str) -> Self {
        Self {
            expectation: MockExpectation::new(method, path),
            server,
        }
    }

    /// Expect specific request header
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.expectation.request_headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Expect specific request body
    pub fn with_body(mut self, body: serde_json::Value) -> Self {
        self.expectation.request_body = Some(body);
        self
    }

    /// Set response status code
    pub fn respond_with_status(mut self, status: u16) -> Self {
        self.expectation.response_status = status;
        self
    }

    /// Set response JSON body
    pub fn respond_with_json(mut self, json: serde_json::Value) -> Self {
        self.expectation.response_body = Some(json);
        self.expectation.response_headers.insert(
            "content-type".to_string(),
            "application/json".to_string(),
        );
        self
    }

    /// Set response header
    pub fn respond_with_header(mut self, key: &str, value: &str) -> Self {
        self.expectation.response_headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Set expected call count (None = any)
    pub fn times(mut self, n: usize) -> Self {
        self.expectation.times = Some(n);
        self
    }

    /// Register the expectation
    pub async fn mount(self) {
        let mut expectations = self.server.write().await;
        expectations.push(self.expectation);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MOCK SERVER
// ═══════════════════════════════════════════════════════════════════════════

/// Mock HTTP server for testing external API calls
pub struct MockServer {
    expectations: Arc<RwLock<Vec<MockExpectation>>>,
    addr: SocketAddr,
    shutdown: Option<tokio::sync::oneshot::Sender<()>>,
}

impl MockServer {
    /// Start a new mock server on a random port
    pub async fn start() -> Result<Self, TestError> {
        let expectations: Arc<RwLock<Vec<MockExpectation>>> = Arc::new(RwLock::new(Vec::new()));
        let expectations_clone = expectations.clone();

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .map_err(|e| TestError::ServerError(e.to_string()))?;
        let addr = listener.local_addr()
            .map_err(|e| TestError::ServerError(e.to_string()))?;

        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();

        // Spawn the mock server
        tokio::spawn(async move {
            let app = axum::Router::new()
                .fallback(move |req: axum::extract::Request| {
                    let exp = expectations_clone.clone();
                    async move {
                        handle_mock_request(exp, req).await
                    }
                });

            let server = axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    shutdown_rx.await.ok();
                });

            let _ = server.await;
        });

        Ok(Self {
            expectations,
            addr,
            shutdown: Some(shutdown_tx),
        })
    }

    /// Get the server URL
    pub fn url(&self) -> String {
        format!("http://{}", self.addr)
    }

    /// Get the server address
    pub fn addr(&self) -> SocketAddr {
        self.addr
    }

    /// Create an expectation for a request
    pub fn expect(&self, method: &str, path: &str) -> MockExpectationBuilder {
        MockExpectationBuilder::new(self.expectations.clone(), method, path)
    }

    /// Verify all expectations were met
    pub async fn verify(&self) -> Result<(), TestError> {
        let expectations = self.expectations.read().await;
        for exp in expectations.iter() {
            if let Some(times) = exp.times {
                if exp.hits != times {
                    return Err(TestError::AssertionFailed(format!(
                        "{} {} expected {} calls, got {}",
                        exp.method, exp.path, times, exp.hits
                    )));
                }
            }
        }
        Ok(())
    }

    /// Reset all expectations
    pub async fn reset(&self) {
        let mut expectations = self.expectations.write().await;
        expectations.clear();
    }

    /// Get call count for a path
    pub async fn call_count(&self, method: &str, path: &str) -> usize {
        let expectations = self.expectations.read().await;
        expectations
            .iter()
            .filter(|e| e.matches(method, path))
            .map(|e| e.hits)
            .sum()
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            let _ = shutdown.send(());
        }
    }
}

async fn handle_mock_request(
    expectations: Arc<RwLock<Vec<MockExpectation>>>,
    req: axum::extract::Request,
) -> axum::response::Response {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    let mut expectations = expectations.write().await;
    
    // Find matching expectation
    for exp in expectations.iter_mut() {
        if exp.matches(&method, &path) {
            // If we have a limit, check if we reached it
            if let Some(times) = exp.times {
                if exp.hits >= times {
                    continue;
                }
            }

            exp.hits += 1;
            
            let mut response = axum::http::Response::builder()
                .status(exp.response_status);
            
            for (key, value) in &exp.response_headers {
                response = response.header(key.as_str(), value.as_str());
            }
            
            let body = exp.response_body
                .as_ref()
                .map(|b| serde_json::to_string(b).unwrap_or_default())
                .unwrap_or_default();
            
            return response.body(axum::body::Body::from(body))
                .unwrap()
                .into_response();
        }
    }

    // No match found
    (StatusCode::NOT_FOUND, format!("No mock for {} {}", method, path)).into_response()
}

// ═══════════════════════════════════════════════════════════════════════════
// TEST CLIENT
// ═══════════════════════════════════════════════════════════════════════════

/// HTTP client for testing applications
pub struct TestClient {
    base_url: String,
    client: reqwest::Client,
    headers: HashMap<String, String>,
}

impl TestClient {
    /// Create a new test client
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
            headers: HashMap::new(),
        }
    }

    /// Add authentication header
    pub fn with_auth(mut self, token: &str) -> Self {
        self.headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        self
    }

    /// Add custom header
    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    /// Set cookie
    pub fn with_cookie(mut self, name: &str, value: &str) -> Self {
        let existing = self.headers.get("Cookie").cloned().unwrap_or_default();
        let new_cookie = if existing.is_empty() {
            format!("{}={}", name, value)
        } else {
            format!("{}; {}={}", existing, name, value)
        };
        self.headers.insert("Cookie".to_string(), new_cookie);
        self
    }

    fn build_request(&self, method: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let url = format!("{}{}", self.base_url, path);
        let mut req = self.client.request(method, &url);
        for (key, value) in &self.headers {
            req = req.header(key.as_str(), value.as_str());
        }
        req
    }

    /// Send GET request
    pub async fn get(&self, path: &str) -> Result<TestResponse, TestError> {
        let response = self.build_request(reqwest::Method::GET, path).send().await?;
        Ok(TestResponse::from_response(response).await)
    }

    /// Send POST request with JSON body
    pub async fn post<T: Serialize>(&self, path: &str, body: &T) -> Result<TestResponse, TestError> {
        let response = self
            .build_request(reqwest::Method::POST, path)
            .json(body)
            .send()
            .await?;
        Ok(TestResponse::from_response(response).await)
    }

    /// Send PUT request with JSON body
    pub async fn put<T: Serialize>(&self, path: &str, body: &T) -> Result<TestResponse, TestError> {
        let response = self
            .build_request(reqwest::Method::PUT, path)
            .json(body)
            .send()
            .await?;
        Ok(TestResponse::from_response(response).await)
    }

    /// Send PATCH request with JSON body
    pub async fn patch<T: Serialize>(&self, path: &str, body: &T) -> Result<TestResponse, TestError> {
        let response = self
            .build_request(reqwest::Method::PATCH, path)
            .json(body)
            .send()
            .await?;
        Ok(TestResponse::from_response(response).await)
    }

    /// Send DELETE request
    pub async fn delete(&self, path: &str) -> Result<TestResponse, TestError> {
        let response = self.build_request(reqwest::Method::DELETE, path).send().await?;
        Ok(TestResponse::from_response(response).await)
    }

    /// Send form data
    pub async fn post_form(&self, path: &str, form: &[(&str, &str)]) -> Result<TestResponse, TestError> {
        let response = self
            .build_request(reqwest::Method::POST, path)
            .form(form)
            .send()
            .await?;
        Ok(TestResponse::from_response(response).await)
    }
}

/// Test response wrapper
#[derive(Debug)]
pub struct TestResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl TestResponse {
    async fn from_response(response: reqwest::Response) -> Self {
        let status = response.status().as_u16();
        let headers: HashMap<String, String> = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();
        let body = response.text().await.unwrap_or_default();

        Self { status, headers, body }
    }

    /// Get status code
    pub fn status(&self) -> u16 {
        self.status
    }

    /// Check if status is success (2xx)
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    /// Check if status is redirect (3xx)
    pub fn is_redirect(&self) -> bool {
        (300..400).contains(&self.status)
    }

    /// Check if status is client error (4xx)
    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.status)
    }

    /// Check if status is server error (5xx)
    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.status)
    }

    /// Get header value
    pub fn header(&self, key: &str) -> Option<&str> {
        self.headers.get(&key.to_lowercase()).map(|s| s.as_str())
    }

    /// Parse body as JSON
    pub fn json<T: DeserializeOwned>(&self) -> Result<T, TestError> {
        serde_json::from_str(&self.body).map_err(|e| TestError::SerializationError(e.to_string()))
    }

    /// Get raw body
    pub fn text(&self) -> &str {
        &self.body
    }

    /// Assert status code
    pub fn assert_status(self, expected: u16) -> Self {
        assert_eq!(self.status, expected, "Expected status {} but got {}", expected, self.status);
        self
    }

    /// Assert JSON body contains field
    pub fn assert_json_has(self, field: &str) -> Self {
        let json: serde_json::Value = serde_json::from_str(&self.body).expect("Body is not JSON");
        assert!(json.get(field).is_some(), "JSON missing field: {}", field);
        self
    }

    /// Assert body contains text
    pub fn assert_contains(self, text: &str) -> Self {
        assert!(self.body.contains(text), "Body does not contain: {}", text);
        self
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// FACTORY
// ═══════════════════════════════════════════════════════════════════════════

/// Factory for generating test data
pub struct Factory;

impl Factory {
    /// Create a user factory
    pub fn user() -> UserFactory {
        UserFactory::default()
    }

    /// Create a random string
    pub fn random_string(len: usize) -> String {
        use std::iter;
        const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        
        iter::repeat_with(|| {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            CHARSET[(rng % CHARSET.len() as u64) as usize] as char
        })
        .take(len)
        .collect()
    }

    /// Create a random email
    pub fn random_email() -> String {
        format!("{}@test.com", Self::random_string(8))
    }

    /// Create a random ID
    pub fn random_id() -> String {
        Self::random_string(12)
    }

    /// Create a sequence of items
    pub fn sequence<T, F>(count: usize, builder: F) -> Vec<T>
    where
        F: Fn(usize) -> T,
    {
        (0..count).map(builder).collect()
    }
}

/// User factory builder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFactory {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
    pub active: bool,
}

impl Default for UserFactory {
    fn default() -> Self {
        Self {
            id: Factory::random_id(),
            email: Factory::random_email(),
            name: "Test User".to_string(),
            role: "user".to_string(),
            active: true,
        }
    }
}

impl UserFactory {
    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = email.to_string();
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn role(mut self, role: &str) -> Self {
        self.role = role.to_string();
        self
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn admin(self) -> Self {
        self.role("admin")
    }

    pub fn inactive(self) -> Self {
        self.active(false)
    }

    pub fn build(self) -> Self {
        self
    }

    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ASSERTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Assert two JSON values are equal
pub fn assert_json_eq(actual: &serde_json::Value, expected: &serde_json::Value) {
    assert_eq!(actual, expected, "JSON mismatch:\nActual: {}\nExpected: {}", actual, expected);
}

/// Assert JSON contains subset
pub fn assert_json_contains(actual: &serde_json::Value, subset: &serde_json::Value) {
    if let (Some(actual_obj), Some(subset_obj)) = (actual.as_object(), subset.as_object()) {
        for (key, value) in subset_obj {
            assert!(
                actual_obj.get(key) == Some(value),
                "JSON missing or mismatched key '{}'. Expected: {}, Actual: {:?}",
                key, value, actual_obj.get(key)
            );
        }
    } else {
        panic!("Both values must be objects");
    }
}

/// Wait for a condition with timeout
pub async fn wait_for<F, Fut>(timeout_ms: u64, condition: F) -> Result<(), TestError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    let start = std::time::Instant::now();
    loop {
        if condition().await {
            return Ok(());
        }
        if start.elapsed().as_millis() as u64 > timeout_ms {
            return Err(TestError::Timeout);
        }
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ═══════════════════════════════════════════════════════════════════════
    // ERROR TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_error_server() {
        let err = TestError::ServerError("port in use".to_string());
        assert!(err.to_string().contains("Server"));
    }

    #[test]
    fn test_error_request() {
        let err = TestError::RequestError("connection refused".to_string());
        assert!(err.to_string().contains("Request"));
    }

    #[test]
    fn test_error_assertion() {
        let err = TestError::AssertionFailed("expected 200".to_string());
        assert!(err.to_string().contains("Assertion"));
    }

    #[test]
    fn test_error_timeout() {
        let err = TestError::Timeout;
        assert!(err.to_string().contains("Timeout"));
    }

    #[test]
    fn test_error_mock_not_matched() {
        let err = TestError::MockNotMatched {
            method: "GET".to_string(),
            path: "/api/users".to_string(),
        };
        assert!(err.to_string().contains("GET"));
        assert!(err.to_string().contains("/api/users"));
    }

    #[test]
    fn test_error_serialization() {
        let err = TestError::SerializationError("invalid json".to_string());
        assert!(err.to_string().contains("Serialization"));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // MOCK EXPECTATION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_mock_expectation_new() {
        let exp = MockExpectation::new("GET", "/api/users");
        assert_eq!(exp.method, "GET");
        assert_eq!(exp.path, "/api/users");
        assert_eq!(exp.response_status, 200);
    }

    #[test]
    fn test_mock_expectation_matches() {
        let exp = MockExpectation::new("POST", "/api/users");
        assert!(exp.matches("POST", "/api/users"));
        assert!(exp.matches("post", "/api/users")); // case insensitive method
        assert!(!exp.matches("GET", "/api/users"));
        assert!(!exp.matches("POST", "/api/other"));
    }

    #[test]
    fn test_mock_expectation_defaults() {
        let exp = MockExpectation::new("GET", "/");
        assert!(exp.request_headers.is_empty());
        assert!(exp.request_body.is_none());
        assert!(exp.response_body.is_none());
        assert!(exp.times.is_none());
        assert_eq!(exp.hits, 0);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // TEST CLIENT TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_client_new() {
        let client = TestClient::new("http://localhost:3000");
        assert_eq!(client.base_url, "http://localhost:3000");
    }

    #[test]
    fn test_client_trailing_slash() {
        let client = TestClient::new("http://localhost:3000/");
        assert_eq!(client.base_url, "http://localhost:3000");
    }

    #[test]
    fn test_client_with_auth() {
        let client = TestClient::new("http://localhost:3000").with_auth("token123");
        assert_eq!(client.headers.get("Authorization"), Some(&"Bearer token123".to_string()));
    }

    #[test]
    fn test_client_with_header() {
        let client = TestClient::new("http://localhost:3000")
            .with_header("X-Custom", "value");
        assert_eq!(client.headers.get("X-Custom"), Some(&"value".to_string()));
    }

    #[test]
    fn test_client_with_cookie() {
        let client = TestClient::new("http://localhost:3000").with_cookie("session", "abc");
        assert_eq!(client.headers.get("Cookie"), Some(&"session=abc".to_string()));
    }

    #[test]
    fn test_client_with_multiple_cookies() {
        let client = TestClient::new("http://localhost:3000")
            .with_cookie("session", "abc")
            .with_cookie("theme", "dark");
        assert_eq!(client.headers.get("Cookie"), Some(&"session=abc; theme=dark".to_string()));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // TEST RESPONSE TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_response_status_checks() {
        let response = TestResponse {
            status: 200,
            headers: HashMap::new(),
            body: String::new(),
        };
        assert!(response.is_success());
        assert!(!response.is_redirect());
        assert!(!response.is_client_error());
        assert!(!response.is_server_error());
    }

    #[test]
    fn test_response_redirect() {
        let response = TestResponse {
            status: 302,
            headers: HashMap::new(),
            body: String::new(),
        };
        assert!(response.is_redirect());
    }

    #[test]
    fn test_response_client_error() {
        let response = TestResponse {
            status: 404,
            headers: HashMap::new(),
            body: String::new(),
        };
        assert!(response.is_client_error());
    }

    #[test]
    fn test_response_server_error() {
        let response = TestResponse {
            status: 500,
            headers: HashMap::new(),
            body: String::new(),
        };
        assert!(response.is_server_error());
    }

    #[test]
    fn test_response_json() {
        let response = TestResponse {
            status: 200,
            headers: HashMap::new(),
            body: r#"{"id": 1, "name": "test"}"#.to_string(),
        };
        let json: serde_json::Value = response.json().unwrap();
        assert_eq!(json["id"], 1);
        assert_eq!(json["name"], "test");
    }

    #[test]
    fn test_response_header() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        let response = TestResponse {
            status: 200,
            headers,
            body: String::new(),
        };
        assert_eq!(response.header("content-type"), Some("application/json"));
    }

    // ═══════════════════════════════════════════════════════════════════════
    // FACTORY TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_factory_random_string() {
        let s1 = Factory::random_string(10);
        let s2 = Factory::random_string(10);
        assert_eq!(s1.len(), 10);
        assert_eq!(s2.len(), 10);
        // Note: might be the same in fast succession, but usually different
    }

    #[test]
    fn test_factory_random_email() {
        let email = Factory::random_email();
        assert!(email.contains('@'));
        assert!(email.ends_with("@test.com"));
    }

    #[test]
    fn test_factory_random_id() {
        let id = Factory::random_id();
        assert_eq!(id.len(), 12);
    }

    #[test]
    fn test_factory_sequence() {
        let items: Vec<i32> = Factory::sequence(5, |i| i as i32 * 2);
        assert_eq!(items, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_user_factory_default() {
        let user = Factory::user().build();
        assert!(!user.id.is_empty());
        assert!(user.email.contains('@'));
        assert_eq!(user.name, "Test User");
        assert_eq!(user.role, "user");
        assert!(user.active);
    }

    #[test]
    fn test_user_factory_custom() {
        let user = Factory::user()
            .id("user_1")
            .email("custom@example.com")
            .name("Custom Name")
            .build();
        assert_eq!(user.id, "user_1");
        assert_eq!(user.email, "custom@example.com");
        assert_eq!(user.name, "Custom Name");
    }

    #[test]
    fn test_user_factory_admin() {
        let user = Factory::user().admin().build();
        assert_eq!(user.role, "admin");
    }

    #[test]
    fn test_user_factory_inactive() {
        let user = Factory::user().inactive().build();
        assert!(!user.active);
    }

    #[test]
    fn test_user_factory_to_json() {
        let user = Factory::user().id("123").build();
        let json = user.to_json();
        assert_eq!(json["id"], "123");
    }

    // ═══════════════════════════════════════════════════════════════════════
    // ASSERTION TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[test]
    fn test_assert_json_eq() {
        let a = serde_json::json!({"id": 1});
        let b = serde_json::json!({"id": 1});
        assert_json_eq(&a, &b);
    }

    #[test]
    fn test_assert_json_contains() {
        let actual = serde_json::json!({"id": 1, "name": "test", "extra": true});
        let subset = serde_json::json!({"id": 1, "name": "test"});
        assert_json_contains(&actual, &subset);
    }

    // ═══════════════════════════════════════════════════════════════════════
    // MOCK SERVER TESTS
    // ═══════════════════════════════════════════════════════════════════════

    #[tokio::test]
    async fn test_mock_server_start() {
        let server = MockServer::start().await.unwrap();
        assert!(server.url().starts_with("http://127.0.0.1:"));
    }

    #[tokio::test]
    async fn test_mock_server_expectation() {
        let server = MockServer::start().await.unwrap();
        server.expect("GET", "/api/test")
            .respond_with_status(200)
            .respond_with_json(serde_json::json!({"success": true}))
            .mount()
            .await;

        let client = TestClient::new(&server.url());
        let response = client.get("/api/test").await.unwrap();
        assert_eq!(response.status(), 200);
    }

    #[tokio::test]
    async fn test_mock_server_reset() {
        let server = MockServer::start().await.unwrap();
        server.expect("GET", "/test").respond_with_status(200).mount().await;
        server.reset().await;
        // After reset, no expectations
    }
}
