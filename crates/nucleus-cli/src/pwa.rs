//! PWA Generation Module
//!
//! Generate Progressive Web App assets including manifest.json,
//! service worker, and offline page.

use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

/// PWA Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwaConfig {
    /// App name shown in install prompts
    pub name: String,
    /// Short name for home screen
    pub short_name: String,
    /// App description
    pub description: String,
    /// Theme color for browser chrome
    pub theme_color: String,
    /// Background color for splash screen
    pub background_color: String,
    /// Display mode: standalone, fullscreen, minimal-ui, browser
    pub display: String,
    /// Start URL (usually "/")
    pub start_url: String,
    /// Scope of the PWA
    pub scope: String,
    /// Cache strategy: cache-first, network-first, stale-while-revalidate
    pub cache_strategy: String,
    /// Icon paths (relative to static dir)
    pub icons: Vec<PwaIcon>,
}

/// PWA Icon definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PwaIcon {
    pub src: String,
    pub sizes: String,
    #[serde(rename = "type")]
    pub mime_type: String,
    pub purpose: String,
}

impl Default for PwaConfig {
    fn default() -> Self {
        Self {
            name: "Nucleus App".to_string(),
            short_name: "App".to_string(),
            description: "A Nucleus Framework Application".to_string(),
            theme_color: "#8B5CF6".to_string(),
            background_color: "#18181B".to_string(),
            display: "standalone".to_string(),
            start_url: "/".to_string(),
            scope: "/".to_string(),
            cache_strategy: "cache-first".to_string(),
            icons: vec![
                PwaIcon {
                    src: "/icons/icon-192.png".to_string(),
                    sizes: "192x192".to_string(),
                    mime_type: "image/png".to_string(),
                    purpose: "any maskable".to_string(),
                },
                PwaIcon {
                    src: "/icons/icon-512.png".to_string(),
                    sizes: "512x512".to_string(),
                    mime_type: "image/png".to_string(),
                    purpose: "any maskable".to_string(),
                },
            ],
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// MANIFEST GENERATION
// ═══════════════════════════════════════════════════════════════════════════

/// Generate a Web App Manifest (manifest.json)
pub fn generate_manifest(config: &PwaConfig) -> String {
    let manifest = serde_json::json!({
        "name": config.name,
        "short_name": config.short_name,
        "description": config.description,
        "start_url": config.start_url,
        "scope": config.scope,
        "display": config.display,
        "theme_color": config.theme_color,
        "background_color": config.background_color,
        "icons": config.icons,
        "orientation": "any",
        "categories": ["utilities"],
        "lang": "en"
    });

    serde_json::to_string_pretty(&manifest).unwrap_or_else(|_| "{}".to_string())
}

// ═══════════════════════════════════════════════════════════════════════════
// SERVICE WORKER GENERATION
// ═══════════════════════════════════════════════════════════════════════════

/// Generate a Service Worker script
pub fn generate_service_worker(routes: &[String], config: &PwaConfig) -> String {
    let cache_name = format!("nucleus-cache-v{}", env!("CARGO_PKG_VERSION"));
    let strategy = &config.cache_strategy;

    // Build the list of assets to precache
    let precache_urls: Vec<String> = routes.iter().map(|r| format!("'{}'", r)).collect();

    let sw = format!(
        r#"/**
 * Nucleus Framework Service Worker
 * Generated automatically - do not edit manually
 * 
 * Cache Strategy: {strategy}
 */

const CACHE_NAME = '{cache_name}';
const OFFLINE_URL = '/offline.html';

// Assets to precache on install
const PRECACHE_URLS = [
    '/',
    '/offline.html',
    '/manifest.json',
    '/assets/neutron-store.js',
    {precache_list}
];

// ═══════════════════════════════════════════════════════════════════════════
// INSTALL EVENT - Precache critical assets
// ═══════════════════════════════════════════════════════════════════════════

self.addEventListener('install', (event) => {{
    console.log('[SW] Installing...');
    event.waitUntil(
        caches.open(CACHE_NAME)
            .then((cache) => {{
                console.log('[SW] Precaching assets');
                return cache.addAll(PRECACHE_URLS);
            }})
            .then(() => self.skipWaiting())
    );
}});

// ═══════════════════════════════════════════════════════════════════════════
// ACTIVATE EVENT - Clean up old caches
// ═══════════════════════════════════════════════════════════════════════════

self.addEventListener('activate', (event) => {{
    console.log('[SW] Activating...');
    event.waitUntil(
        caches.keys()
            .then((cacheNames) => {{
                return Promise.all(
                    cacheNames
                        .filter((name) => name !== CACHE_NAME)
                        .map((name) => {{
                            console.log('[SW] Deleting old cache:', name);
                            return caches.delete(name);
                        }})
                );
            }})
            .then(() => self.clients.claim())
    );
}});

// ═══════════════════════════════════════════════════════════════════════════
// FETCH EVENT - Handle requests with caching strategy
// ═══════════════════════════════════════════════════════════════════════════

self.addEventListener('fetch', (event) => {{
    const {{ request }} = event;
    const url = new URL(request.url);

    // Skip non-GET requests
    if (request.method !== 'GET') return;

    // Skip cross-origin requests
    if (url.origin !== location.origin) return;

    // API routes: Network-first
    if (url.pathname.startsWith('/api/')) {{
        event.respondWith(networkFirst(request));
        return;
    }}

    // Static assets: {strategy}
    event.respondWith({strategy_fn}(request));
}});

// ═══════════════════════════════════════════════════════════════════════════
// CACHING STRATEGIES
// ═══════════════════════════════════════════════════════════════════════════

/**
 * Cache-First: Try cache, fall back to network
 * Best for: Static assets that rarely change
 */
async function cacheFirst(request) {{
    const cached = await caches.match(request);
    if (cached) {{
        // Refresh cache in background
        fetch(request).then((response) => {{
            if (response.ok) {{
                caches.open(CACHE_NAME).then((cache) => cache.put(request, response));
            }}
        }}).catch(() => {{}});
        return cached;
    }}

    try {{
        const response = await fetch(request);
        if (response.ok) {{
            const cache = await caches.open(CACHE_NAME);
            cache.put(request, response.clone());
        }}
        return response;
    }} catch (error) {{
        return caches.match(OFFLINE_URL);
    }}
}}

/**
 * Network-First: Try network, fall back to cache
 * Best for: API calls, dynamic content
 */
async function networkFirst(request) {{
    try {{
        const response = await fetch(request);
        if (response.ok) {{
            const cache = await caches.open(CACHE_NAME);
            cache.put(request, response.clone());
        }}
        return response;
    }} catch (error) {{
        const cached = await caches.match(request);
        if (cached) return cached;
        return caches.match(OFFLINE_URL);
    }}
}}

/**
 * Stale-While-Revalidate: Return cache immediately, update in background
 * Best for: Content that can be slightly stale
 */
async function staleWhileRevalidate(request) {{
    const cache = await caches.open(CACHE_NAME);
    const cached = await cache.match(request);

    const fetchPromise = fetch(request).then((response) => {{
        if (response.ok) {{
            cache.put(request, response.clone());
        }}
        return response;
    }}).catch(() => cached || caches.match(OFFLINE_URL));

    return cached || fetchPromise;
}}

// ═══════════════════════════════════════════════════════════════════════════
// BACKGROUND SYNC (for offline mutations)
// ═══════════════════════════════════════════════════════════════════════════

self.addEventListener('sync', (event) => {{
    if (event.tag === 'nucleus-sync') {{
        event.waitUntil(syncPendingRequests());
    }}
}});

async function syncPendingRequests() {{
    // This would be implemented to replay queued mutations
    console.log('[SW] Syncing pending requests...');
}}

// ═══════════════════════════════════════════════════════════════════════════
// PUSH NOTIFICATIONS
// ═══════════════════════════════════════════════════════════════════════════

self.addEventListener('push', (event) => {{
    const data = event.data?.json() || {{ title: 'Notification', body: '' }};
    event.waitUntil(
        self.registration.showNotification(data.title, {{
            body: data.body,
            icon: '/icons/icon-192.png',
            badge: '/icons/badge-72.png'
        }})
    );
}});

self.addEventListener('notificationclick', (event) => {{
    event.notification.close();
    event.waitUntil(
        clients.openWindow('/')
    );
}});
"#,
        strategy = strategy,
        cache_name = cache_name,
        precache_list = precache_urls.join(",\n    "),
        strategy_fn = match strategy.as_str() {
            "network-first" => "networkFirst",
            "stale-while-revalidate" => "staleWhileRevalidate",
            _ => "cacheFirst",
        }
    );

    sw
}

// ═══════════════════════════════════════════════════════════════════════════
// OFFLINE PAGE GENERATION
// ═══════════════════════════════════════════════════════════════════════════

/// Generate an offline fallback page
pub fn generate_offline_page(config: &PwaConfig) -> String {
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="theme-color" content="{theme_color}">
    <title>Offline - {name}</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: {bg_color};
            color: #e4e4e7;
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            text-align: center;
            padding: 2rem;
        }}
        .container {{
            max-width: 400px;
        }}
        .icon {{
            font-size: 4rem;
            margin-bottom: 1.5rem;
            opacity: 0.8;
        }}
        h1 {{
            font-size: 1.5rem;
            font-weight: 600;
            margin-bottom: 0.75rem;
            color: #fafafa;
        }}
        p {{
            color: #a1a1aa;
            line-height: 1.6;
            margin-bottom: 2rem;
        }}
        .retry-btn {{
            display: inline-flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.75rem 1.5rem;
            background: {theme_color};
            color: white;
            border: none;
            border-radius: 0.5rem;
            font-size: 1rem;
            font-weight: 500;
            cursor: pointer;
            transition: opacity 0.2s;
        }}
        .retry-btn:hover {{
            opacity: 0.9;
        }}
        .status {{
            margin-top: 2rem;
            padding: 0.75rem 1rem;
            background: rgba(139, 92, 246, 0.1);
            border-radius: 0.5rem;
            font-size: 0.875rem;
            color: #a78bfa;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="icon">📴</div>
        <h1>You're Offline</h1>
        <p>It looks like you've lost your internet connection. Some features may be unavailable until you're back online.</p>
        <button class="retry-btn" onclick="window.location.reload()">
            <span>↻</span> Try Again
        </button>
        <div class="status" id="status">
            Waiting for connection...
        </div>
    </div>
    <script>
        // Auto-reload when back online
        window.addEventListener('online', () => {{
            document.getElementById('status').textContent = 'Connection restored! Reloading...';
            setTimeout(() => window.location.reload(), 500);
        }});

        // Check connection status
        if (navigator.onLine) {{
            document.getElementById('status').textContent = 'Connection detected. Retrying...';
            setTimeout(() => window.location.reload(), 1000);
        }}
    </script>
</body>
</html>"#,
        name = config.name,
        theme_color = config.theme_color,
        bg_color = config.background_color
    );

    html
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRATION SNIPPET
// ═══════════════════════════════════════════════════════════════════════════

/// Generate the service worker registration script to be included in HTML
pub fn generate_sw_registration() -> String {
    r#"<script>
if ('serviceWorker' in navigator) {
    window.addEventListener('load', () => {
        navigator.serviceWorker.register('/sw.js')
            .then((registration) => {
                console.log('[App] Service Worker registered:', registration.scope);
            })
            .catch((error) => {
                console.warn('[App] Service Worker registration failed:', error);
            });
    });
}
</script>"#
        .to_string()
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PwaConfig::default();
        assert_eq!(config.name, "Nucleus App");
        assert_eq!(config.display, "standalone");
        assert_eq!(config.cache_strategy, "cache-first");
    }

    #[test]
    fn test_generate_manifest() {
        let config = PwaConfig::default();
        let manifest = generate_manifest(&config);

        assert!(manifest.contains("\"name\": \"Nucleus App\""));
        assert!(manifest.contains("\"theme_color\": \"#8B5CF6\""));
        assert!(manifest.contains("\"display\": \"standalone\""));
    }

    #[test]
    fn test_generate_service_worker() {
        let config = PwaConfig::default();
        let routes = vec!["/".to_string(), "/about".to_string()];
        let sw = generate_service_worker(&routes, &config);

        assert!(sw.contains("const CACHE_NAME"));
        assert!(sw.contains("cacheFirst"));
        assert!(sw.contains("addEventListener('install'"));
        assert!(sw.contains("addEventListener('fetch'"));
    }

    #[test]
    fn test_generate_service_worker_network_first() {
        let mut config = PwaConfig::default();
        config.cache_strategy = "network-first".to_string();
        let routes = vec![];
        let sw = generate_service_worker(&routes, &config);

        assert!(sw.contains("networkFirst(request)"));
    }

    #[test]
    fn test_generate_offline_page() {
        let config = PwaConfig::default();
        let page = generate_offline_page(&config);

        assert!(page.contains("<!DOCTYPE html>"));
        assert!(page.contains("You're Offline"));
        assert!(page.contains("#8B5CF6")); // theme color
    }

    #[test]
    fn test_sw_registration() {
        let script = generate_sw_registration();
        assert!(script.contains("serviceWorker"));
        assert!(script.contains("register('/sw.js')"));
    }
}
