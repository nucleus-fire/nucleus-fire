#![allow(unused_imports)]
        use axum::{response::{Html, IntoResponse}, routing::get, extract::{Query, Form}, Router};
        use tower_http::services::ServeDir;
        use tower_http::compression::CompressionLayer;
        use tokio::net::TcpListener;
        use serde::{Serialize, Deserialize};
        
        // --- Static Assets (Zero-Allocation) ---
        #[allow(non_snake_case)]
async fn handle_index() -> impl IntoResponse { Html("No view found in src/views/index.ncl") }#[allow(non_snake_case)]
async fn handle_components() -> impl IntoResponse { Html("No view found in src/views/components.ncl") }#[allow(non_snake_case)]
async fn handle_docs() -> impl IntoResponse { Html("No view found in src/views/docs.ncl") }

        // Middleware Module Support
        

        // Logic Module Support
        

        // Models Module Support
        

        use mimalloc::MiMalloc;
        #[global_allocator]
        static GLOBAL: MiMalloc = MiMalloc;

        #[tokio::main]
        async fn main() {
            // Load Configuration
            let config = nucleus_std::config::Config::load();
            let addr = format!("{}:{}", config.server.host, config.server.port);

            // Initialize Database
            // Only init if URL is set (simple check)
            if !config.database.url.is_empty() {
                 match nucleus_std::photon::init_db(&config.database.url).await {
                     Ok(_) => println!("✅ Database initialized on {}", config.database.url),
                     Err(e) => eprintln!("❌ Database initialization failed: {}", e),
                 }
            }

            // Static Router with Zero-Allocation Assets
            let app = Router::new()
                .route("/", get(handle_index)).route("/index", get(handle_index)).route("/components", get(handle_components)).route("/docs", get(handle_docs))
                .nest_service("/pkg", ServeDir::new("static/pkg"))
                .layer(CompressionLayer::new().br(true).gzip(true))
                ;
                
            // Auto-Inject Middleware if `src/middleware.rs` exists
            let app = app;
            

            println!("⚛️  Nucleus Hyperdrive (AOT) running on http://{}", addr);
            let listener = TcpListener::bind(&addr).await.unwrap();
            axum::serve(listener, app).await.unwrap();
        }
        