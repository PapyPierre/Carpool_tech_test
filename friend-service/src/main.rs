use axum::{routing::get, Router};
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::{Arc, Mutex},
};

#[derive(Debug)]
struct AppState {
    friends: HashMap<String, HashSet<String>>,
    pending: HashMap<String, HashSet<String>>,
}

async fn hello() -> &'static str {
    "Server is running"
}

#[tokio::main] // Allows us to use async fn main()
async fn main() {
    // Create shared state
    let state :Arc<Mutex<AppState>> = Arc::new(Mutex::new(AppState {
        friends: HashMap::new(),
        pending: HashMap::new(),
    }));

    let app = Router::new().route("/", get(hello)).with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server running on http://{}", addr);

    // Start server
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app, ).await.unwrap();
}