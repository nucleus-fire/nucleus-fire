use atom::NucleusRuntime;

mod models;
mod services;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("💬 Nucleus Chat active on http://127.0.0.1:3000");

    let ws_handler = Arc::new(services::websocket::ChatHandler);
    NucleusRuntime::start(None, Some(ws_handler)).await;
}
