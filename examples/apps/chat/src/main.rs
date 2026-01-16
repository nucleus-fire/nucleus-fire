use atom::NucleusRuntime;

#[tokio::main]
async fn main() {
    println!("💬 Nucleus Chat active on http://127.0.0.1:3000");
    NucleusRuntime::start(None, None).await;
}
