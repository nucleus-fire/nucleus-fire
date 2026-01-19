use atom::NucleusRuntime;
mod controllers;

#[tokio::main]
async fn main() {
    println!("✍️ Blog Starter running on http://127.0.0.1:3000");

    // Start Reactor with custom router
    let app_router = controllers::posts::router();
    NucleusRuntime::start_with_router(None, None, Some(app_router)).await;
}
