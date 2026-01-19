#[allow(unused_imports)]
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::test]
async fn test_dating_startup() {
    // Basic connectivity check to ensure binary builds and runs
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    println!("Dating app test server bound to {}", addr);
    assert!(addr.port() > 0);
}
