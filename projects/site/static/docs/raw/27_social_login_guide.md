# Social Login Guide

> Implement secure OAuth2 social login with Google, GitHub, Discord, Apple, Facebook, and Microsoft in minutes.

## Quick Start

### 1. Configure Environment Variables

```bash
# Redirect URI (used for all providers)
OAUTH_REDIRECT_URI=https://yourapp.com/auth/callback

# Google (console.cloud.google.com)
GOOGLE_CLIENT_ID=your-client-id.apps.googleusercontent.com
GOOGLE_CLIENT_SECRET=your-client-secret

# GitHub (github.com/settings/developers)
GITHUB_CLIENT_ID=your-client-id
GITHUB_CLIENT_SECRET=your-client-secret

# Discord (discord.com/developers/applications)
DISCORD_CLIENT_ID=your-client-id
DISCORD_CLIENT_SECRET=your-client-secret
```

### 2. Add Login Buttons

```rust
use nucleus_std::oauth::{OAuthConfig, render_social_buttons};

let config = OAuthConfig::from_env();
let buttons_html = render_social_buttons(&config, None);
// Renders styled buttons for all enabled providers
```

### 3. Handle OAuth Flow

```rust
use nucleus_std::oauth::{OAuth, OAuthConfig, OAuthProvider};

// Initialize
let config = OAuthConfig::from_env();
let oauth = OAuth::new(config);

// Route: GET /auth/google
async fn start_google_login(session: Session) -> impl IntoResponse {
    let (url, state) = oauth.authorize_url(OAuthProvider::Google)?;
    session.set("oauth_state", state);
    Redirect::to(&url)
}

// Route: GET /auth/callback/google
async fn google_callback(
    Query(params): Query<CallbackParams>,
    session: Session,
) -> impl IntoResponse {
    let expected_state = session.get("oauth_state").unwrap();
    
    let user = oauth.exchange_code(
        OAuthProvider::Google,
        &params.code,
        &params.state,
        &expected_state
    ).await?;
    
    // user.email, user.name, user.avatar are now available
    // Create or update user in database, set session, etc.
    
    Redirect::to("/dashboard")
}
```

---

## Supported Providers

| Provider | Features | Setup Link |
|----------|----------|------------|
| **Google** | Email, profile, avatar | [Google Cloud Console](https://console.cloud.google.com/apis/credentials) |
| **GitHub** | Email, username, avatar | [GitHub Developer Settings](https://github.com/settings/developers) |
| **Discord** | Email, username, avatar | [Discord Developer Portal](https://discord.com/developers/applications) |
| **Apple** | Email, name | [Apple Developer](https://developer.apple.com/account/resources/identifiers/list/serviceId) |
| **Facebook** | Email, name, avatar | [Facebook Developers](https://developers.facebook.com/apps) |
| **Microsoft** | Email, name | [Azure Portal](https://portal.azure.com/#blade/Microsoft_AAD_RegisteredApps) |

---

## Configuration

### Environment Variables

```bash
# Required: Base redirect URI
OAUTH_REDIRECT_URI=https://yourapp.com/auth/callback

# Provider credentials (add only providers you need)
GOOGLE_CLIENT_ID=...
GOOGLE_CLIENT_SECRET=...
GOOGLE_SCOPES=openid email profile  # Optional, defaults used

GITHUB_CLIENT_ID=...
GITHUB_CLIENT_SECRET=...

DISCORD_CLIENT_ID=...
DISCORD_CLIENT_SECRET=...

APPLE_CLIENT_ID=...
APPLE_CLIENT_SECRET=...

FACEBOOK_CLIENT_ID=...
FACEBOOK_CLIENT_SECRET=...

MICROSOFT_CLIENT_ID=...
MICROSOFT_CLIENT_SECRET=...
```

### Programmatic Configuration

```rust
use nucleus_std::oauth::{OAuthConfig, ProviderConfig};

let config = OAuthConfig {
    redirect_uri: "https://myapp.com/auth/callback".to_string(),
    google: Some(ProviderConfig {
        client_id: "...".to_string(),
        client_secret: "...".to_string(),
        scopes: Some("openid email profile".to_string()),
        enabled: true,
    }),
    github: Some(ProviderConfig {
        client_id: "...".to_string(),
        client_secret: "...".to_string(),
        scopes: None,  // Uses default scopes
        enabled: true,
    }),
    ..Default::default()  // Other providers disabled
};
```

---

## Full Implementation Example

### Routes Setup

```rust
use axum::{Router, routing::get};
use nucleus_std::oauth::{OAuth, OAuthConfig, OAuthProvider};

pub fn auth_routes() -> Router {
    let config = OAuthConfig::from_env();
    let oauth = OAuth::new(config.clone());
    
    Router::new()
        // Login initiation
        .route("/auth/google", get(|s| start_login(s, OAuthProvider::Google)))
        .route("/auth/github", get(|s| start_login(s, OAuthProvider::GitHub)))
        .route("/auth/discord", get(|s| start_login(s, OAuthProvider::Discord)))
        
        // Callbacks
        .route("/auth/callback/google", get(|q, s| handle_callback(q, s, OAuthProvider::Google)))
        .route("/auth/callback/github", get(|q, s| handle_callback(q, s, OAuthProvider::GitHub)))
        .route("/auth/callback/discord", get(|q, s| handle_callback(q, s, OAuthProvider::Discord)))
        
        .with_state(oauth)
}
```

### Login Initiation Handler

```rust
async fn start_login(
    State(oauth): State<OAuth>,
    session: Session,
    provider: OAuthProvider,
) -> Result<Redirect, AppError> {
    let (url, state) = oauth.authorize_url(provider)?;
    
    // Store state in session for CSRF protection
    session.insert("oauth_state", state)?;
    session.insert("oauth_provider", format!("{:?}", provider))?;
    
    Ok(Redirect::to(&url))
}
```

### Callback Handler

```rust
#[derive(Deserialize)]
struct CallbackParams {
    code: String,
    state: String,
}

async fn handle_callback(
    State(oauth): State<OAuth>,
    Query(params): Query<CallbackParams>,
    session: Session,
    provider: OAuthProvider,
) -> Result<Redirect, AppError> {
    // Verify state (CSRF protection)
    let expected_state: String = session.get("oauth_state")
        .ok_or(AppError::InvalidState)?;
    
    // Exchange code for user info
    let oauth_user = oauth.exchange_code(
        provider,
        &params.code,
        &params.state,
        &expected_state
    ).await?;
    
    // Find or create user in database
    let user = find_or_create_user(&oauth_user).await?;
    
    // Set session
    session.insert("user_id", user.id)?;
    session.remove("oauth_state");
    
    Ok(Redirect::to("/dashboard"))
}

async fn find_or_create_user(oauth_user: &OAuthUser) -> Result<User, DbError> {
    // Check if user exists by provider + provider_id
    if let Some(user) = User::find_by_oauth(&oauth_user.provider, &oauth_user.provider_id).await? {
        return Ok(user);
    }
    
    // Create new user
    let user = User::create(CreateUser {
        email: oauth_user.email.clone(),
        name: oauth_user.name.clone(),
        avatar: oauth_user.avatar.clone(),
        oauth_provider: Some(oauth_user.provider.clone()),
        oauth_id: Some(oauth_user.provider_id.clone()),
    }).await?;
    
    Ok(user)
}
```

---

## Login UI

### Generated Buttons

```rust
use nucleus_std::oauth::{render_social_buttons, social_buttons_css};

// In your login page template
let buttons = render_social_buttons(&config, Some("btn btn-social"));
let css = social_buttons_css();
```

Generated HTML:
```html
<div class="social-login-buttons">
  <a href="/auth/google" class="btn btn-social social-google">🔵 Continue with Google</a>
  <a href="/auth/github" class="btn btn-social social-github">⬛ Continue with GitHub</a>
  <a href="/auth/discord" class="btn btn-social social-discord">💜 Continue with Discord</a>
</div>
```

### Custom Button Styling (Tailwind)

```xml
<div class="flex flex-col gap-3 max-w-sm">
  <a href="/auth/google" class="flex items-center justify-center gap-2 px-4 py-3 
     bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition">
    <img src="/icons/google.svg" class="w-5 h-5" />
    Continue with Google
  </a>
  
  <a href="/auth/github" class="flex items-center justify-center gap-2 px-4 py-3 
     bg-gray-900 text-white rounded-lg hover:bg-gray-800 transition">
    <img src="/icons/github.svg" class="w-5 h-5" />
    Continue with GitHub
  </a>
  
  <a href="/auth/discord" class="flex items-center justify-center gap-2 px-4 py-3 
     bg-indigo-600 text-white rounded-lg hover:bg-indigo-700 transition">
    <img src="/icons/discord.svg" class="w-5 h-5" />
    Continue with Discord
  </a>
</div>
```

---

## Security Best Practices

### ✅ State Parameter (CSRF Protection)

Always verify the state parameter in callbacks:

```rust
if params.state != session.get("oauth_state") {
    return Err("Invalid state - possible CSRF attack");
}
```

### ✅ Secure Cookie Settings

```rust
// Session cookie configuration
SessionConfig {
    secure: true,      // HTTPS only
    http_only: true,   // No JS access
    same_site: SameSite::Lax,
    max_age: Duration::hours(24),
}
```

### ✅ Environment Variables

Never commit secrets to version control:

```bash
# .env (in .gitignore)
GOOGLE_CLIENT_SECRET=your-secret

# .env.example (committed)
GOOGLE_CLIENT_SECRET=your-secret-here
```

### ✅ Email Verification

Some providers may not verify emails. Consider:

```rust
if oauth_user.provider == "github" && oauth_user.email.is_none() {
    // GitHub users can hide email - prompt for it
    return Redirect::to("/complete-profile");
}
```

---

## Provider-Specific Notes

### Google

- Requires Google Cloud project with OAuth consent screen
- Set "Authorized redirect URIs" to `https://yourapp.com/auth/callback/google`
- Request `openid`, `email`, `profile` scopes

### GitHub

- Email may be null if user has private email
- Fetch emails separately: `GET /user/emails` with `user:email` scope

### Discord

- User may need to verify email in Discord
- Avatar URL is constructed from user ID + avatar hash

### Apple

- Name is only provided on first login
- Requires Apple Developer account ($99/year)
- More complex setup with key files

### Facebook

- Requires app review for `email` permission in production
- Profile picture URL expires

### Microsoft

- Works with personal, work, and school accounts
- Use `common` tenant for all account types

---

## API Reference

### OAuthConfig

| Method | Description |
|--------|-------------|
| `from_env()` | Load config from environment variables |
| `get_provider(p)` | Get config for specific provider |
| `enabled_providers()` | List all enabled providers |

### OAuth

| Method | Description |
|--------|-------------|
| `new(config)` | Create OAuth client |
| `generate_state()` | Generate secure random state |
| `authorize_url(provider)` | Get login URL + state |
| `exchange_code(...)` | Exchange code for user info |

### OAuthUser

| Field | Type | Description |
|-------|------|-------------|
| `provider` | String | Provider name (google, github, etc.) |
| `provider_id` | String | User's ID on the provider |
| `email` | Option | User's email (may be None) |
| `name` | Option | User's display name |
| `avatar` | Option | Profile picture URL |
| `raw` | HashMap | Full response from provider |

---

## Troubleshooting

### "Invalid state parameter"
- State expired (session timed out)
- User opened multiple login tabs
- Solution: Regenerate state and retry

### "Token exchange failed"
- Check client secret is correct
- Verify redirect URI matches exactly (including trailing slash)

### "No email returned"
- GitHub: User has private email, request `user:email` scope
- Apple: Email only returned on first login
- Facebook: App needs email permission approval

### "Provider not configured"
- Add environment variables for the provider
- Restart application after adding env vars
