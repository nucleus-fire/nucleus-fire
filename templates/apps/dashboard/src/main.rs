use atom::NucleusRuntime;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Define Routes
    let mut routes = HashMap::new();
    routes.insert("index".to_string(), "Dashboard Home".to_string());
    
    println!("🚀 Nucleus Dashboard launching on http://127.0.0.1:3000");
    
    // Start Reactor
    // Note: In a real app, we would inject a custom 'dashboard' module 
    // into the runtime, but Nucleus's 'NucleusRuntime::start' is a high-level wrapper.
    // We are relying on the declarative 'index.ncl' which the runtime will pick up.
    
    // We pass None for config to use defaults (which includes looking in src/views)
    NucleusRuntime::start(None, None).await;
}
