use axum::{Router, routing::get};
use nucleus_std::config::Config;

mod logic;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() {
    // Load config (supports .env and nucleus.config)
    let config = Config::load();
    println!("🚀 Demo App running on {}:{}", config.server.host, config.server.port);
    println!("   Environment: {}", config.server.environment);
    
    let app = Router::new()
        .route("/", get(|| async { "Hello from Demo App" }));
        
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
