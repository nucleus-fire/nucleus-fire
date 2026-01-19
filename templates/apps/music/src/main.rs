mod models;
mod services;

#[cfg(test)]
mod tests;

use axum::Router;
use std::net::SocketAddr;
use tower_http::services::{ServeDir, ServeFile};
// use network_interface::NetworkInterface; // Removed
// use network_interface::NetworkInterfaceConfig; // Removed
use nucleus_std::photon;
// use sqlx::sqlite::SqlitePoolOptions; // Removed unused import
use std::time::Duration;

#[tokio::main]
async fn main() {
    // 1. Init Database
    println!("🎵 Initializing Nucleus Music...");

    // Ensure db file exists
    if !std::path::Path::new("music.db").exists() {
        std::fs::File::create("music.db").unwrap();
    }

    photon::init_db("sqlite:music.db")
        .await
        .expect("Failed to init DB");

    // 2. Run Migrations
    let pool = photon::db().as_sqlite().unwrap();
    let schema = include_str!("../migrations/20250101_initial_schema.sql");
    sqlx::query(schema)
        .execute(pool)
        .await
        .expect("Failed to run migrations");
    if std::path::Path::new("seed_data.sql").exists() {
        let seed = std::fs::read_to_string("seed_data.sql").unwrap();
        // Ignore constraints (if already exists)
        let _ = sqlx::query(&seed).execute(pool).await;
    }

    // 3. Scan Library (Demo Mode: Scan ./static/music)
    // Create static/music if not exists
    std::fs::create_dir_all("static/music").ok();

    tokio::spawn(async {
        // scan in background after startup
        tokio::time::sleep(Duration::from_secs(2)).await;
        services::scanner::scan_library("static/music").await;
    });

    // 4. Setup Router
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route_service("/", ServeFile::new("static/index.html"))
        .merge(services::api::routes());

    // 5. Start Server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("🚀 Music App listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// TODO: In a real Nucleus app, NCC compiler would generate the NCL route handlers.
// For this example, we might rely on client-side fetching or simple View returning if using the new View engine.
// Since 'nucleus_std::view' engine isn't fully integrated here yet (usually handled by 'atom' macro or 'ncc'),
// we will serve a static index.html that fetches data via an API we will add.
