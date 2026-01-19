# Standard Library Reference

Complete API documentation for `nucleus-std`, the Nucleus Framework standard library.

---

## Overview

The Nucleus standard library provides batteries-included modules for common web application needs:

| Module | Description |
|--------|-------------|
| [Photon](#photon) | Type-safe SQL query builder |
| [Fortress](#fortress) | Authentication & security |
| [OAuth](#oauth) | Social login (Google, GitHub, etc.) |
| [Neutron](#neutron) | Reactive state management |
| [Vault](#vault) | Ledger-based money operations |
| [Pulse](#pulse) | Background job queue |
| [Postman](#postman) | Email sending (SMTP & SES) |
| [Sonar](#sonar) | Full-text search |
| [Gondola](#gondola) | Offline sync (CRDTs) |
| [Chain](#chain) | Blockchain utilities (EVM) |
| [Stream](#stream) | WebSocket channels |
| [Lens](#lens) | Image processing |
| [Polyglot](#polyglot) | Internationalization (i18n) |
| [Beacon](#beacon) | Privacy-first analytics |
| [Federation](#federation) | CMS content aggregation |
| [Payments](#payments) | Stripe integration |
| [Neural](#neural) | AI/ML utilities |
| [Browser](#browser) | Headless browser testing |
| [Config](#config) | Configuration management |
| [RPC](#rpc) | Server function client |
| [DevTools](#devtools) | Development utilities |
| [Forms](#forms) | Schema-driven forms & validation |

---

## Photon

Multi-database query builder supporting PostgreSQL, MySQL, and SQLite.

### Import

```rust
use nucleus_std::photon::{init_db, db, Model, Op, Builder, transaction};
use nucleus_std::impl_model;
```

### Database Initialization

```rust
// Auto-detect from URL
init_db("sqlite:./data.db").await?;
init_db("postgres://localhost/mydb").await?;
init_db("mysql://localhost/mydb").await?;

// Access global pool
let pool = db();
```

### Model Trait

```rust
use sqlx::FromRow;
use nucleus_std::{impl_model, photon::Model};

#[derive(FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
}

impl_model!(User, "users");

// Generated methods:
User::table_name();  // "users"
User::query();       // Returns Builder
User::find::<User>(1).await?;
User::delete_by_id(1).await?;
```

### Query Builder

```rust
// Basic query
let users = User::query()
    .filter_op("status", Op::Eq, "active")
    .order_by("created_at", "DESC")
    .limit(10)
    .all::<User>()
    .await?;

// OR conditions
let users = User::query()
    .r#where("role", "admin")
    .or_where("role", "moderator")
    .all::<User>()
    .await?;

// Count & exists
let count = User::query().count().await?;
let has_admin = User::query().filter_op("role", Op::Eq, "admin").exists().await?;

// Joins
let posts = Post::query()
    .join("users", "posts.user_id", "users.id")
    .left_join("comments", "posts.id", "comments.post_id")
    .all::<Post>()
    .await?;

// CRUD
User::query().insert().value("name", "Alice").execute().await?;
User::query().update().value("name", "Bob").r#where("id", 1).execute().await?;
User::query().delete().r#where("id", 1).execute().await?;
```

### Operators

```rust
pub enum Op {
    Eq, Ne, Gt, Gte, Lt, Lte,  // Comparison
    Like, ILike,                // Pattern matching
    In, NotIn,                  // List membership
    IsNull, IsNotNull,          // NULL checks
}
```

### Transactions

```rust
use nucleus_std::photon::transaction;

transaction(|tx| Box::pin(async move {
    sqlx::query("UPDATE accounts SET balance = balance - 100 WHERE id = ?")
        .bind(1)
        .execute(&mut **tx)
        .await?;
    Ok(())
})).await?;
```

### CLI Commands

| Command | Description |
|---------|-------------|
| `nucleus db init` | Create migrations directory |
| `nucleus db new <name>` | Create migration file |
| `nucleus db up` | Apply migrations |
| `nucleus db down` | Rollback migrations |
| `nucleus db status` | Show migration status |

> [!TIP]
> See the [Database Guide](#20_database_guide) for complete documentation.

---

## Fortress

Authentication, authorization, and security utilities.

### Import

```rust
use nucleus_std::fortress::*;
```

### Password Hashing

```rust
/// Hash a password using Argon2id
pub fn hash_password(password: &str) -> Result<String>;

/// Verify a password against a hash
pub fn verify_password(password: &str, hash: &str) -> Result<bool>;
```

### JWT Tokens

```rust
/// Generate a JWT token
pub fn generate_token(subject: &str, duration: Duration) -> Result<String>;

/// Verify and decode a JWT token
pub fn verify_token(token: &str) -> Result<Claims>;

/// Token claims
pub struct Claims {
    pub sub: String,           // Subject (usually user ID)
    pub exp: usize,            // Expiration timestamp
    pub iat: usize,            // Issued at timestamp
    pub custom: Option<Value>, // Custom claims
}
```

### Role-Based Access Control

```rust
/// Define a role
pub struct Role {
    pub name: String,
    pub permissions: HashSet<Permission>,
}

impl Role {
    pub fn new(name: &str) -> Self;
    pub fn with_permission(self, perm: Permission) -> Self;
}

/// Predefined permissions
pub enum Permission {
    Read,
    Write,
    Delete,
    Admin,
    Custom(String),
}

/// Check if user has permission
pub fn check_permission(user: &impl HasRole, perm: Permission) -> bool;
```

### Security Headers

Fortress automatically adds these headers:

| Header | Value |
|--------|-------|
| `Content-Security-Policy` | Strict CSP rules |
| `X-Content-Type-Options` | `nosniff` |
| `X-Frame-Options` | `DENY` |
| `X-XSS-Protection` | `1; mode=block` |
| `Strict-Transport-Security` | `max-age=31536000` |
| `Referrer-Policy` | `strict-origin-when-cross-origin` |

### CSP Configuration

```rust
pub struct CspConfig {
    pub default_src: Vec<String>,
    pub script_src: Vec<String>,
    pub style_src: Vec<String>,
    pub img_src: Vec<String>,
    pub font_src: Vec<String>,
    pub connect_src: Vec<String>,
    pub frame_ancestors: Vec<String>,
}

impl Default for CspConfig {
    fn default() -> Self {
        CspConfig {
            default_src: vec!["'self'".into()],
            script_src: vec!["'self'".into()],
            style_src: vec!["'self'".into(), "https://fonts.googleapis.com".into()],
            img_src: vec!["'self'".into(), "data:".into()],
            font_src: vec!["'self'".into(), "https://fonts.gstatic.com".into()],
            connect_src: vec!["'self'".into()],
            frame_ancestors: vec!["'none'".into()],
        }
    }
}
```

---

## OAuth

Social login with Google, GitHub, Discord, Apple, Facebook, and Microsoft.

### Import

```rust
use nucleus_std::oauth::{OAuth, OAuthConfig, OAuthProvider};
```

### Configuration

```rust
// Load from environment variables
let config = OAuthConfig::from_env();

// Or configure programmatically
let config = OAuthConfig {
    redirect_uri: "https://myapp.com/auth/callback".to_string(),
    google: Some(ProviderConfig {
        client_id: "your-client-id".to_string(),
        client_secret: "your-secret".to_string(),
        scopes: None, // Uses defaults
        enabled: true,
    }),
    ..Default::default()
};
```

### OAuth Flow

```rust
let oauth = OAuth::new(config);

// 1. Generate login URL
let (url, state) = oauth.authorize_url(OAuthProvider::Google)?;
// Store state in session, redirect user to url

// 2. Handle callback
let user = oauth.exchange_code(
    OAuthProvider::Google,
    &code,       // From query params
    &state,      // From query params  
    &stored_state // From session
).await?;

println!("Welcome, {}!", user.name.unwrap_or_default());
```

### UI Helpers

```rust
// Generate login buttons HTML
let html = render_social_buttons(&config, Some("btn btn-social"));

// Get default CSS
let css = social_buttons_css();
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `OAUTH_REDIRECT_URI` | Base callback URL |
| `GOOGLE_CLIENT_ID` | Google OAuth client ID |
| `GOOGLE_CLIENT_SECRET` | Google OAuth secret |
| `GITHUB_CLIENT_ID` | GitHub OAuth client ID |
| ... | Same pattern for other providers |

> [!TIP]
> See the [Social Login Guide](#27_social_login_guide) for complete setup instructions.

---

## Neutron
 
 Reactive state management with fine-grained updates, computed values, and automatic cleanup.
 
 ### Import
 
 ```rust
 use nucleus_std::neutron::{store, Signal, Computed, Global, create_effect, batch};
 ```
 
 ### Store (`#[store]`)
 
 Zero-boilerplate reactive state container.
 
 ```rust
 #[store]
 struct AppState {
     count: i32,
     user: Option<String>,
 }
 
 let state = AppState::new(0, None);
 ```
 
 ### Signal<T>
 
 ```rust
 impl<T: Clone + 'static> Signal<T> {
     pub fn get(&self) -> T;              // Read & track dependency
     pub fn set(&self, value: T);         // Set & notify
     pub fn modify(&self, f: impl FnOnce(&mut T)); // Modify in-place
     pub fn get_untracked(&self) -> T;    // Read without tracking
 }
 ```
 
 ### Computed<T>
 
 ```rust
 use nucleus_std::neutron::computed;
 
 // Automatic memoization
 let doubled = computed(state.count.clone(), |c| c * 2);
 ```
 
 ### Global State
 
 ```rust
 static THEME: Global<Signal<String>> = Global::new(|| Signal::new("dark".to_string()));
 ```
 
 ### Effects
 
 ```rust
 create_effect({
     let count = state.count.clone();
     move || println!("Count: {}", count.get())
 });
 ```
 
 ### Batching
 
 ```rust
 batch(|| {
     state.count.set(5);
     state.user.set(Some("Alice".to_string()));
 });
 ```

> [!TIP]
> See the [State Management Guide](#15_state_management) for complete documentation.

---

## Vault

Ledger-based money operations with 64-bit cent precision.

### Import

```rust
use nucleus_std::vault::*;
```

### Money Type

```rust
/// Represents money as cents (i64)
pub struct Money(i64);

impl Money {
    pub fn from_cents(cents: i64) -> Self;
    pub fn from_dollars(dollars: f64) -> Self;  // ⚠️ Panics! Use from_cents
    pub fn cents(&self) -> i64;
    pub fn to_string(&self) -> String;  // "$12.34"
}

// Arithmetic
impl Add for Money { ... }
impl Sub for Money { ... }
```

### Ledger

```rust
pub struct Ledger {
    accounts: HashMap<String, Money>,
}

impl Ledger {
    /// Create a new ledger
    pub fn new() -> Self;
    
    /// Create account with initial balance
    pub fn create_account(&mut self, id: &str, initial: Money) -> Result<()>;
    
    /// Get account balance
    pub fn balance(&self, id: &str) -> Result<Money>;
    
    /// Transfer between accounts (atomic)
    pub fn transfer(&mut self, from: &str, to: &str, amount: Money) -> Result<()>;
    
    /// Deposit to account
    pub fn deposit(&mut self, id: &str, amount: Money) -> Result<()>;
    
    /// Withdraw from account
    pub fn withdraw(&mut self, id: &str, amount: Money) -> Result<()>;
}
```

### Examples

```rust
let mut ledger = Ledger::new();

ledger.create_account("user_1", Money::from_cents(10000))?; // $100.00
ledger.create_account("user_2", Money::from_cents(5000))?;  // $50.00

ledger.transfer("user_1", "user_2", Money::from_cents(2500))?;

assert_eq!(ledger.balance("user_1")?.cents(), 7500);  // $75.00
assert_eq!(ledger.balance("user_2")?.cents(), 7500);  // $75.00
```

---

## Pulse

Background job queue with retries and scheduling.

### Import

```rust
use nucleus_std::pulse::*;
```

### Job Definition

```rust
pub struct Job {
    pub id: String,
    pub handler: String,
    pub payload: Value,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub retry_count: u32,
    pub max_retries: u32,
}
```

### Queue Operations

```rust
pub struct Queue;

impl Queue {
    /// Enqueue a job for immediate execution
    pub async fn push(handler: &str, payload: impl Serialize) -> Result<String>;
    
    /// Schedule a job for later
    pub async fn schedule(
        handler: &str,
        payload: impl Serialize,
        run_at: DateTime<Utc>
    ) -> Result<String>;
    
    /// Process pending jobs
    pub async fn process() -> Result<()>;
    
    /// Cancel a job
    pub async fn cancel(job_id: &str) -> Result<()>;
}
```

### Examples

```rust
// Immediate job
Queue::push("send_email", json!({
    "to": "user@example.com",
    "subject": "Welcome!"
})).await?;

// Scheduled job (1 hour from now)
Queue::schedule(
    "send_reminder",
    json!({ "user_id": 123 }),
    Utc::now() + Duration::hours(1)
).await?;
```

---

## Postman

Email sending with templates.

### Import

```rust
use nucleus_std::postman::*;
```

### Send Email

```rust
pub struct Email {
    pub to: String,
    pub subject: String,
    pub body: String,
    pub html: Option<String>,
}

impl Email {
    pub fn new(to: &str, subject: &str, body: &str) -> Self;
    pub fn with_html(self, html: &str) -> Self;
    pub async fn send(self) -> Result<()>;
}
```

### Configuration

```toml
# nucleus.config
[email]
provider = "smtp"  # or "sendgrid", "mailgun", "ses"
host = "smtp.example.com"
port = 587
username = "${SMTP_USER}"
password = "${SMTP_PASS}"
from = "noreply@example.com"
```

---

## Sonar

Full-text search with BM25 ranking.

### Import

```rust
use nucleus_std::sonar::*;
```

### Index Operations

```rust
pub struct SearchIndex;

impl SearchIndex {
    /// Index a document
    pub async fn index(id: &str, content: &str, metadata: Value) -> Result<()>;
    
    /// Search with query
    pub async fn search(query: &str, limit: usize) -> Result<Vec<SearchResult>>;
    
    /// Remove from index
    pub async fn remove(id: &str) -> Result<()>;
}

pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub metadata: Value,
    pub highlights: Vec<String>,
}
```

---

## Gondola

Offline-first sync with CRDTs.

### Import

```rust
use nucleus_std::gondola::*;
```

### Sync Document

```rust
pub struct SyncDoc<T> {
    pub id: String,
    pub data: T,
    pub version: u64,
    pub last_synced: Option<DateTime<Utc>>,
}

impl<T: Serialize + DeserializeOwned> SyncDoc<T> {
    pub fn new(id: &str, data: T) -> Self;
    pub async fn sync(&mut self) -> Result<SyncStatus>;
    pub async fn push(&self) -> Result<()>;
    pub async fn pull(&mut self) -> Result<()>;
}
```

---

## Stream

WebSocket channels for real-time features.

### Import

```rust
use nucleus_std::stream::*;
```

### Channel

```rust
pub struct Channel {
    pub name: String,
}

impl Channel {
    pub fn new(name: &str) -> Self;
    pub async fn broadcast(&self, event: &str, data: impl Serialize) -> Result<()>;
    pub async fn send_to(&self, user_id: &str, event: &str, data: impl Serialize) -> Result<()>;
}
```

### StreamHandler Trait

```rust
pub trait StreamHandler: Send + Sync {
    async fn on_connect(&self, socket_id: &str);
    async fn on_message(&self, socket_id: &str, message: Value);
    async fn on_disconnect(&self, socket_id: &str);
}
```

---

## Error Handling

### NucleusError

```rust
pub enum NucleusError {
    NotFound(String),
    Validation(String),
    Database(String),
    Auth(String),
    Network(String),
    CryptoError(String),
    Internal(String),
}

impl IntoResponse for NucleusError {
    // Automatically converts to appropriate HTTP status
}
```

### Result Type

```rust
pub type Result<T> = std::result::Result<T, NucleusError>;
```

---

## Federation

Content federation for aggregating from multiple headless CMS platforms.

### Import

```rust
use nucleus_std::federation::*;
```

### Supported Platforms

| Platform | Query Language | Protocol |
|----------|---------------|----------|
| Directus | SQL-like filters | REST |
| Sanity | GROQ | HTTP |
| Strapi | Strapi v4 filters | REST |

### Federation Query

```rust
let articles = Federation::query("articles")
    .filter("status", "published")
    .order_by("created_at", Direction::Desc)
    .limit(10)
    .fetch()
    .await?;
```

### Source-Specific Query

```rust
// Directus
let items = Federation::source("directus")
    .collection("posts")
    .fetch()
    .await?;

// Sanity with raw GROQ
let items = Federation::source("sanity")
    .raw("*[_type == 'post'][0...5]")
    .fetch()
    .await?;
```

### FederationCache

```rust
let cache = FederationCache::new(300); // 5 min TTL
cache.set("articles", articles);
let cached = cache.get("articles");
cache.invalidate("articles");
```

### Configuration

Federation requires environment variables for each CMS platform:

| Variable | Required For | Description |
|----------|-------------|-------------|
| `DIRECTUS_URL` | Directus | API base URL (e.g., `http://localhost:8055`) |
| `DIRECTUS_TOKEN` | Directus | API access token for authentication |
| `SANITY_PROJECT_ID` | Sanity | Your Sanity project ID |
| `SANITY_DATASET` | Sanity | Dataset name (e.g., `production`) |
| `SANITY_TOKEN` | Sanity | Optional API token for private content |
| `STRAPI_URL` | Strapi | API base URL (e.g., `http://localhost:1337`) |
| `STRAPI_TOKEN` | Strapi | API token for authentication |

**Example `.env` file:**

```env
DIRECTUS_URL=https://cms.example.com
DIRECTUS_TOKEN=your-directus-token

SANITY_PROJECT_ID=abc123
SANITY_DATASET=production

STRAPI_URL=https://strapi.example.com
STRAPI_TOKEN=your-strapi-token
```

---

## Payments

Stripe payment integration with subscriptions and webhooks.

### Import

```rust
use nucleus_std::payments::*;
```

### Checkout Session

```rust
let session = Stripe::checkout(CheckoutRequest {
    amount: Money::from_cents(2999),
    currency: "usd",
    product_name: "Pro Plan",
    success_url: "https://example.com/success",
    cancel_url: "https://example.com/cancel",
}).await?;

// Redirect user to session.url
```

### Subscriptions

```rust
let customer = Stripe::create_customer(CreateCustomer {
    email: "user@example.com",
    name: Some("John Doe".into()),
}).await?;

let sub = Stripe::create_subscription(CreateSubscription {
    customer_id: customer.id,
    price_id: "price_xxxxx",
}).await?;
```

### Webhook Verification

```rust
let payload = request.body();
let sig_header = headers.get("stripe-signature")?;

let event = Stripe::verify_webhook(&payload, &sig_header)?;

match event.event_type.as_str() {
    "checkout.session.completed" => handle_checkout(event),
    "invoice.paid" => handle_payment(event),
    _ => Ok(())
}
```

---

## Chain

Blockchain utilities for EVM-compatible chains.

### Import

```rust
use nucleus_std::chain::*;
```

### Signature Verification (SIWE)

```rust
let is_valid = Chain::verify_signature(
    "0x1234...abcd",  // message
    "0xabcd...",      // signature
    "0x742d...",      // expected address
)?;
```

### Balance Queries

```rust
let balance = Chain::get_balance("0x742d...").await?;
println!("Balance: {} wei", balance);
```

### Configuration

```toml
[chain]
rpc_url = "https://mainnet.infura.io/v3/YOUR_KEY"
chain_id = 1
```

---

## Neural

AI/ML utilities for content generation.

### Import

```rust
use nucleus_std::neural::*;
```

### Text Generation

```rust
let response = Neural::generate(
    "Write a product description for a wireless mouse"
).await?;
```

---

## Browser

Headless browser automation for testing.

### Import

```rust
use nucleus_std::browser::*;
```

### Basic Usage

```rust
let browser = Browser::new().await?;
let page = browser.new_page("http://localhost:3000").await?;

// Navigate
page.goto("/login").await?;

// Interact
page.fill("#email", "test@example.com").await?;
page.click("button[type=submit]").await?;

// Assert
let text = page.text_content("h1").await?;
assert!(text.contains("Dashboard"));
```

---

## Config

Application configuration with environment variable interpolation.

### Import

```rust
use nucleus_std::config::*;
```

### Load Configuration

```rust
let config = Config::load();

println!("Port: {}", config.server.port);
println!("DB: {}", config.database.url);
```

### Configuration Sections

```rust
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub app: AppConfig,
    pub performance: PerformanceConfig,
    pub payments: Option<PaymentsConfig>,
    pub chain: Option<ChainConfig>,
}
```

### Environment Variables

```toml
[database]
url = "${DATABASE_URL|sqlite:dev.db}"  # Fallback syntax
```

---

## RPC

Server function RPC client for `#[server]` macro.

### Import

```rust
use nucleus_std::rpc::*;
```

### Server Functions

```rust
#[server]
async fn get_user(id: i64) -> Result<User> {
    User::find(id).await
}

// Client-side call (auto-generated RPC)
let user = get_user(123).await?;
```

The `#[server]` macro automatically:
- Generates RPC endpoint at `/_rpc/{function_name}`
- Handles serialization/deserialization
- Provides type-safe client call

---

## DevTools

Development utilities and debugging helpers.

### Import

```rust
use nucleus_std::devtools::*;
```

### Live Reload

```rust
// Automatically injected in development mode
// Watches file changes and reloads browser
```

### Debug Logging

```rust
devtools::log("Request received", &request);
devtools::time("database_query", || {
    User::query().fetch().await
});
```

---

## Forms

Schema-driven form system with validation, multi-step wizards, and component integration.

### Import

```rust
use nucleus_std::forms::*;
```

### Quick Example

```rust
let form = FormSchema::new("login")
    .action("/login")
    .field(Field::email("email").label("Email").required())
    .field(Field::password("password").label("Password").required().min(8.0))
    .submit("Sign In");

let html = form.render();
```

### Field Types

| Type | Description |
|------|-------------|
| `Text` | Single-line text input |
| `Email` | Email input with validation |
| `Password` | Password input (masked) |
| `Number` | Numeric input |
| `Textarea` | Multi-line text |
| `Select` | Dropdown selection |
| `Checkbox` | Boolean checkbox |
| `Radio` | Radio button group |
| `Date` | Date picker |
| `File` | File upload |
| `Component(name)` | Custom component |

### Validation Rules

```rust
Field::text("username")
    .required()
    .min(3.0)               // Min length/value
    .max(20.0)              // Max length/value
    .pattern("^[a-z]+$", Some("Lowercase only"));

let result = validate(&schema, &data);
if !result.valid {
    for error in result.errors {
        println!("{}: {}", error.field, error.message);
    }
}
```

### Multi-Step Wizard

```rust
let wizard = FormSchema::new("signup")
    .step(WizardStep::new("account", "Account")
        .field(Field::email("email").required()))
    .step(WizardStep::new("profile", "Profile")
        .field(Field::text("name").required()));
```

### Using Custom Components

```rust
Field::component("avatar", "AvatarUpload")
    .prop("maxSize", "5MB")
    .prop("accept", "image/*")
```

> **Full Guide**: See [Forms & Validation Guide](#07_forms_and_validation) for complete documentation.

---

## Lens

Image processing pipeline for resizing, cropping, and converting images.

### Import

```rust
use nucleus_std::lens::{Lens, ImageFormat};
```

### Basic Usage

```rust
// Resize image
let resized = Lens::resize(&data, 800, 600)?;

// Create thumbnail
let thumb = Lens::thumbnail(&data, 150)?;

// Convert format
let webp = Lens::convert(&data, ImageFormat::WebP(80))?;
```

### Features

- **Format Conversion**: JPEG, PNG, WebP, GIF, BMP
- **Transformations**: Blur, Crop, Rotate, Flip
- **Adjustments**: Brightness, Contrast, Grayscale

> **Full Guide**: See [Image Processing Guide](#26_image_processing_guide) for complete documentation.

---

## Beacon

Privacy-first analytics tracking module.

### Import

```rust
use nucleus_std::beacon::{Beacon, AnalyticsProvider};
```

### Basic Usage

```rust
// Initialize (In-Memory for testing)
let beacon = Beacon::new(AnalyticsProvider::InMemory);

// Initialize from Environment (Production)
let beacon = Beacon::from_env();
```

### Features

- **Privacy First**: No PII collection by default
- **Providers**: Pluggable backends (GA, plausible, etc.)
- **Events**: Custom event tracking

> **Full Guide**: See [Analytics Guide](#29_analytics_guide) for complete documentation.

---

## Polyglot

Internationalization (i18n) and localization module.

### Import

```rust
use nucleus_std::polyglot::Polyglot;
```

### Basic Usage

```rust
// Initialize with locale
let i18n = Polyglot::new("en-US");

// Check direction
if i18n.is_rtl() {
    println!("Right-to-Left language");
}
```

### Features

- **Locale Parsing**: Handles full BCP-47 tags
- **Pluralization**: CLDR-compliant rules
- **Formatting**: Numbers, Dates, Currencies

> **Full Guide**: See [i18n Guide](#28_i18n_guide) for complete documentation.

---

## See Also

- [Getting Started](#01_getting_started)
- [Core Concepts](#02_core_concepts)
- [API Development](#22_api_development)
- [Federation Guide](#25_federation_guide)
- [Email Guide](#25_email_guide)
- [Image Processing Guide](#26_image_processing_guide)
- [Offline Sync Guide](#27_offline_sync_guide)
- [i18n Guide](#28_i18n_guide)
- [Analytics Guide](#29_analytics_guide)
- [Forms & Validation](#07_forms_and_validation)
- [Configuration](#configuration)

