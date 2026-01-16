use atom::NucleusRuntime;
use std::collections::HashMap;
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let mut routes = HashMap::new();
    // Pre-allocate static content
    routes.insert("home".to_string(), "Hello World".to_string());
    routes.insert("plaintext".to_string(), "Hello, World!".to_string());
    
    // Start the optimized runtime
    // No middleware = pure throughput test
    NucleusRuntime::start(Some(routes), None).await;
}