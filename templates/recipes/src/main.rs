use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // Init Database Global Pool for Active Record
    nucleus_std::photon::init_db("sqlite:nucleus.db").await.ok();
    
    // ... rest of init
    println!("Nucleus Recipes V3 Running...");
    
    // Load routes from views (simplified - normally ncc does this)
    let mut routes = HashMap::new();
    routes.insert("home".to_string(), include_str!("views/index.ncl").to_string());
    routes.insert("hello".to_string(), include_str!("views/hello.ncl").to_string());
    routes.insert("counter".to_string(), include_str!("views/counter.ncl").to_string());
    routes.insert("todo".to_string(), include_str!("views/todo.ncl").to_string());
    
    atom::start_reactor(Some(routes), None).await;
}
