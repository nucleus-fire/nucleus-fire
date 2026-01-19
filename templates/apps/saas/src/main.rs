use atom::NucleusRuntime;

#[tokio::main]
async fn main() {
    println!("🚀 SaaS Starter running on http://127.0.0.1:3000");
    
    // Start Reactor with default config (scans src/views)
    NucleusRuntime::start(None, None).await;
}
