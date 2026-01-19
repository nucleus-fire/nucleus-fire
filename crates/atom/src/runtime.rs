use axum::{
    body::Bytes,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tower_http::compression::CompressionLayer;

#[cfg(feature = "hot-reload")]
use crate::hot_swap::HotSwapListener;
#[cfg(feature = "middleware-arena")]
use crate::memory::arena_middleware;
#[cfg(feature = "middleware-fortress")]
use crate::middleware::fortress; // Fortress
use nucleus_std::stream::{SocketMessage, StreamHandler, StreamHub, WebSocket as NucleusWebSocket};

use ahash::AHashMap;
use arc_swap::ArcSwap;

#[derive(Clone)]
pub struct AppState {
    pub routes: Arc<ArcSwap<AHashMap<String, Bytes>>>,
    pub stream_handler: Option<Arc<dyn StreamHandler>>,
    pub stream_hub: Arc<StreamHub>,
    pub tx: tokio::sync::broadcast::Sender<String>,
    pub is_dev: bool,
    pub cached_date: Arc<ArcSwap<String>>,
}

pub struct NucleusRuntime;

impl Default for NucleusRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl NucleusRuntime {
    pub fn new() -> Self {
        Self
    }

    pub async fn start(
        routes: Option<HashMap<String, String>>,
        stream_handler: Option<Arc<dyn StreamHandler>>,
    ) {
        Self::start_with_router(routes, stream_handler, None).await;
    }

    pub async fn start_with_router(
        routes: Option<HashMap<String, String>>,
        stream_handler: Option<Arc<dyn StreamHandler>>,
        extra_router: Option<Router>,
    ) {
        // Optimize: Convert String -> Bytes for zero-copy cloning
        let optimized_routes: AHashMap<String, Bytes> = routes
            .unwrap_or_default()
            .into_iter()
            .map(|(k, v)| (k, Bytes::from(v)))
            .collect();

        // Use ArcSwap for Wait-Free Reads + HMR Support (Enhanced Safe Mode)
        let routes = Arc::new(ArcSwap::from_pointee(optimized_routes));

        // Create HMR Broadcast Channel
        let (tx, _rx) = tokio::sync::broadcast::channel(100);

        // Date Header Optimization (RFC 7231) - Cached update every 1s
        let now = chrono::Utc::now().to_rfc2822().replace("+0000", "GMT");
        let cached_date = Arc::new(ArcSwap::from_pointee(now));

        let date_clone = cached_date.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_millis(1000));
            loop {
                interval.tick().await;
                let now = chrono::Utc::now().to_rfc2822().replace("+0000", "GMT");
                date_clone.store(Arc::new(now));
            }
        });

        // Cache Environment State (Hot Path Optimization)
        let is_dev = std::env::var("NUCLEUS_ENV").unwrap_or_default() == "development";

        // Start Hot Swap Listener (Only if enabled)
        #[cfg(feature = "hot-reload")]
        {
            let routes_clone = routes.clone();
            let tx_clone = tx.clone();
            tokio::spawn(async move {
                let mut hot_swap = HotSwapListener::new(routes_clone, tx_clone);
                hot_swap.listen().await;
            });
        }

        let state = AppState {
            routes,
            stream_handler,
            stream_hub: Arc::new(StreamHub::new()),
            tx,
            is_dev,
            cached_date,
        };

        // Initialize Router
        let mut app = Self::make_router(state);

        // Merge extra router if provided (before layers to ensure consistency)
        if let Some(router) = extra_router {
            app = app.merge(router);
        }

        let app = app.layer(CompressionLayer::new().br(true).gzip(true));

        #[cfg(feature = "middleware-fortress")]
        let app = app.layer(axum::middleware::from_fn(fortress::fortress));

        #[cfg(feature = "middleware-arena")]
        let app = app.layer(axum::middleware::from_fn(arena_middleware));

        // Developer Tools Middleware
        let app = app.layer(axum::middleware::from_fn(
            crate::middleware::profiler::profile,
        ));
        let app = app.layer(axum::middleware::from_fn(
            crate::middleware::ai_assist::error_assistant,
        ));

        // Start Reactor
        println!("Atom Reactor starting on 0.0.0.0:3000");
        let listener = TcpListener::bind("0.0.0.0:3000")
            .await
            .expect("Failed to bind to port 3000. Is the port already in use?");
        axum::serve(listener, app)
            .await
            .expect("Server failed to start");
    }

    pub fn make_router(state: AppState) -> Router {
        Router::new()
            .route("/ws", get(ws_handler))
            // Dedicated benchmark route - bypasses fallback routing for max speed
            .route("/plaintext", get(plaintext_handler))
            .fallback(get(dynamic_handler))
            .with_state(state)
    }
}

// Ultra-fast plaintext handler for benchmarks
// Avoids: routing lookup, string parsing, MIME type detection
static PLAINTEXT_RESPONSE: &[u8] = b"Hello, World!";

async fn plaintext_handler() -> impl IntoResponse {
    (
        [
            (
                axum::http::header::CONTENT_TYPE,
                "text/plain; charset=utf-8",
            ),
            (axum::http::header::CONTENT_LENGTH, "13"),
        ],
        PLAINTEXT_RESPONSE,
    )
}

async fn dynamic_handler(State(state): State<AppState>, uri: axum::http::Uri) -> impl IntoResponse {
    let path = uri.path();

    // Optimization: ArcSwap load (Wait-Free)
    // Returns a Guard that derefs to AHashMap
    let routes = state.routes.load();

    let key = if path == "/" { "home" } else { &path[1..] };
    if let Some(content) = routes.get(key).or_else(|| routes.get(path)) {
        // HOT PATH OPTIMIZATION:
        // In Production, we avoid string allocation and utf8 validation.
        // We simply clone the Bytes (cheap ref-count incr) and return.
        if !state.is_dev {
            // Note: Middleware handles Date headers now.
            // Determine MIME type and Cache-Control based on extension
            let (content_type, cache_control) = if key.ends_with(".css") {
                ("text/css", "public, max-age=31536000, immutable") // 1 year for CSS
            } else if key.ends_with(".js") {
                (
                    "application/javascript",
                    "public, max-age=31536000, immutable",
                ) // 1 year for JS
            } else if key.ends_with(".json") {
                ("application/json", "public, max-age=3600") // 1 hour for JSON
            } else if key.ends_with(".ico") {
                ("image/x-icon", "public, max-age=86400") // 1 day for icons
            } else if key.ends_with(".woff2") || key.ends_with(".woff") || key.ends_with(".ttf") {
                ("font/woff2", "public, max-age=31536000, immutable") // 1 year for fonts
            } else if key == "plaintext" {
                ("text/plain; charset=utf-8", "no-cache") // Benchmark compliance
            } else {
                (
                    "text/html; charset=utf-8",
                    "no-cache, no-store, must-revalidate",
                ) // HTML should not be cached
            };

            return (
                [
                    (axum::http::header::CONTENT_TYPE, content_type),
                    (axum::http::header::CACHE_CONTROL, cache_control),
                ],
                content.clone(),
            )
                .into_response();
        }

        // DEVELOPMENT MODE SLOW PATH:
        // Determine MIME type based on extension
        let content_type = if key.ends_with(".css") {
            "text/css"
        } else if key.ends_with(".js") {
            "application/javascript"
        } else if key.ends_with(".json") {
            "application/json"
        } else if key.ends_with(".ico") {
            "image/x-icon"
        } else if key == "plaintext" {
            "text/plain; charset=utf-8"
        } else {
            "text/html; charset=utf-8"
        };

        let mut body = String::from_utf8_lossy(content).to_string();

        // Inject DevTools (Only for HTML)
        if content_type.starts_with("text/html") {
            let script = nucleus_std::devtools::get_script();
            body = body.replace("</body>", &format!("<script>{}</script></body>", script));
            return Html(body).into_response();
        }

        return ([(axum::http::header::CONTENT_TYPE, content_type)], body).into_response();
    }

    (axum::http::StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| {
        handle_socket(socket, state.stream_handler, state.stream_hub, state.tx)
    })
}

async fn handle_socket(
    mut socket: WebSocket,
    handler: Option<Arc<dyn StreamHandler>>,
    hub: Arc<StreamHub>,
    tx: tokio::sync::broadcast::Sender<String>,
) {
    // 1. Subscribe to HMR events
    let mut hmr_rx = tx.subscribe();

    if let Some(handler) = handler {
        let (mut sender, mut receiver) = socket.split();
        let (msg_tx, mut rx) = mpsc::channel::<SocketMessage>(100);

        // Create Nucleus WebSocket wrapper with unique ID
        let socket_id = uuid::Uuid::new_v4().to_string();
        let n_socket = NucleusWebSocket::new(socket_id.clone(), msg_tx);

        let n_socket_clone = n_socket.clone();
        let handler_clone = handler.clone();
        let hub_clone = hub.clone();

        // Notify connect
        handler.on_connect(&hub, &n_socket).await;

        // Task to send messages from Nucleus -> Client
        let mut send_task = tokio::spawn(async move {
            loop {
                tokio::select! {
                     // Application Messages
                     Some(msg) = rx.recv() => {
                        let axum_msg = match msg {
                            SocketMessage::Text(t) => Message::Text(t),
                            SocketMessage::Binary(b) => Message::Binary(b),
                            SocketMessage::Ping(b) => Message::Ping(b),
                            SocketMessage::Pong(b) => Message::Pong(b),
                            SocketMessage::Close => Message::Close(None),
                            SocketMessage::Json(v) => Message::Text(v.to_string()),
                        };
                        if sender.send(axum_msg).await.is_err() {
                            break;
                        }
                     }
                     // HMR Messages
                     Ok(msg) = hmr_rx.recv() => {
                         if msg == "hmr:reload" && sender.send(Message::Text("hmr:reload".into())).await.is_err() {
                             break;
                         }
                     }
                }
            }
        });

        // Loop to receive messages from Client -> Nucleus
        let mut recv_task = tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                let n_msg = match msg {
                    Message::Text(t) => Some(SocketMessage::Text(t)),
                    Message::Binary(b) => Some(SocketMessage::Binary(b)),
                    Message::Ping(b) => Some(SocketMessage::Ping(b)),
                    Message::Pong(b) => Some(SocketMessage::Pong(b)),
                    Message::Close(_) => Some(SocketMessage::Close),
                    // _ => None, // Unreachable
                };

                if let Some(m) = n_msg {
                    handler_clone
                        .on_message(&hub_clone, &n_socket_clone, m)
                        .await;
                }
            }
        });

        tokio::select! {
            _ = (&mut send_task) => recv_task.abort(),
            _ = (&mut recv_task) => send_task.abort(),
        };

        handler.on_disconnect(&hub, &socket_id).await;
    } else {
        // Echo fallback if no handler (Still support HMR?)
        // Ideally checking for HMR here too, but for now focusing on app mode
        while let Some(msg) = socket.recv().await {
            if let Ok(msg) = msg {
                if socket.send(msg).await.is_err() {
                    break;
                }
            } else {
                break;
            }
        }
    }
}
