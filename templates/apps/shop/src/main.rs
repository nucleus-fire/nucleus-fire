use atom::NucleusRuntime;

mod models;
use models::store;

#[tokio::main]
async fn main() {
    println!("🛒 Nucleus Shop active on http://127.0.0.1:3000");
    // Initialize store to verify compilation/logic
    let _store = store::ShopStore::demo();
    NucleusRuntime::start(None, None).await;
}
