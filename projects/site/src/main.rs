// Import generated AOT module
mod app_generated;

mod controllers;

#[tokio::main]
async fn main() {
    // 1. Get the generated router
    let app = app_generated::app()
        .await
        .merge(controllers::newsletter::router());

    // 2. Start Server
    let config = nucleus_std::config::Config::load();
    let addr = format!("{}:{}", config.server.host, config.server.port);

    println!("⚛️  Nucleus Site (AOT) starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
