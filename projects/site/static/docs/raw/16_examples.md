# Examples & Demos

Nucleus comes with a suite of reference applications demonstrating various architectural patterns. You can find them in the `examples/` directory.

## 1. Shop (State Management)
**Path**: `examples/shop`

A fully functional E-Commerce cart demonstrating the **Store Pattern** and **Derived Signals**.

*   **Key Features**:
    *   `ShopStore` (Rust) manages logic.
    *   Derived Signals (`total`, `count`) update automatically.
    *   Serialization between Rust and NCL.
*   **Run**:
    ```bash
    cd examples/shop
    cargo run
    # Open http://127.0.0.1:3000
    ```

## 2. Dashboard (Analytics)
**Path**: `examples/dashboard`

A high-performance analytics dashboard using Glassmorphism and CSS-only charts.

*   **Key Features**:
    *   **Glassmorphism UI**: Backdrop blur, translucent layers.
    *   **CSS Charts**: Data visualization without heavy JS libraries.
    *   **Simulated Real-time**: Signals update metrics live.
*   **Run**:
    ```bash
    cd examples/dashboard
    cargo run
    # Open http://127.0.0.1:3000
    ```

## 3. Chat (Real-time)
**Path**: `examples/chat`

A Slack-like messaging interface demonstrating Optimistic UI updates.

*   **Key Features**:
    *   **Optimistic UI**: Messages appear instantly before server confirmation.
    *   **Avatars**: Generated user profiles.
    *   **Tailwind CSS**: Complex layouts (Sidebar, Feed, Input).
*   **Run**:
    ```bash
    cd examples/chat
    cargo run
    # Open http://127.0.0.1:3000
    ```

## 4. Neutron Demo (ToDo)
**Path**: `examples/neutron-demo`

The classic TodoMVC implementation.

*   **Key Features**:
    *   **CRUD**: Create, Read, Update, Delete.
    *   **SQLite**: Persistence via `Nucleus::Photon`.
*   **Run**:
    ```bash
    cd examples/neutron-demo
    cargo run
    # Open http://127.0.0.1:3000
    ```

---

## 5. Music App (Spotify Clone)
**Path**: `examples/music`

A fully featured local music player demonstrating **Active Record**, **Scanning**, and **Multimedia**.

*   **Key Features**:
    *   **Active Record**: Uses `impl_model!` for simple DB logic (`impl_model!(Tracks, "tracks")`).
    *   **File Scanning**: Recursively scans directories for MP3/FLAC metadata.
    *   **Glassmorphism UI**: "Midnight Glass" aesthetic with persistent player.
*   **Run**:
    ```bash
    cd examples/music
    cargo run
    # Open http://127.0.0.1:3001
    ```

---

## 6. Recipes (V3 Showcase)
**Path**: `examples/recipes`

A collection of small, focused examples demonstrating specific Nucleus capabilities.

*   **Available Recipes**:
    | Recipe | Demonstrates |
    |--------|--------------|
    | **Hello World** | Basic routing and server rendering |
    | **Counter** | Client-side WASM interactivity |
    | **Todo App** | Database integration (SQLite) |
    | **Auth** | Login flow and guards |

*   **Run**:
    ```bash
    cd examples/recipes
    nucleus run
    # Open http://127.0.0.1:3000
    ```

---

## 7. Module Examples (Standalone)

Reference implementations for specific stdlib modules. These are single-file examples you can copy into your project.

| Example | File | Module |
|---------|------|--------|
| **Email Sending** | `email_sending.ncl` | Postman |
| **Analytics Tracking** | `analytics_tracking.ncl` | Beacon |
| **Image Processing** | `media_pipeline.ncl` | Lens |
| **Offline Sync** | `offline_sync.ncl` | Gondola |
| **i18n & Localization** | `i18n_localization.ncl` | Polyglot |

### Email Sending (`email_sending.ncl`)
- Welcome emails with HTML templates
- Transactional order confirmations
- CC/BCC and reply-to handling
- Testing with Mock provider

### Analytics Tracking (`analytics_tracking.ncl`)
- Page view and click tracking
- E-commerce events (purchase, add-to-cart)
- User identification and traits
- Error tracking with context
- A/B testing events

### Image Processing (`media_pipeline.ncl`)
- Avatar upload with resize
- Responsive image generation (thumbnail/medium/large)
- Social media crop presets (Instagram, Twitter)
- Vintage filter pipeline

### Offline Sync (`offline_sync.ncl`)
- Mobile sync with Merkle trees
- Collaborative counters (likes/views)
- Inventory with PN-Counter
- Last-Write-Wins settings

### i18n Localization (`i18n_localization.ncl`)
- JSON/TOML translation loading
- Pluralization rules
- Number and currency formatting
- Date formatting by locale
- RTL layout detection

---

## Running Any Example

All examples follow the same pattern:

```bash
cd examples/<name>
nucleus run     # Development mode with HMR
# or
cargo run       # Standard Rust execution
```

For production builds:
```bash
nucleus build
./target/release/server
```

