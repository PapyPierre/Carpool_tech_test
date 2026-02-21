mod errors;
mod handlers;
mod models;
mod routes;
mod state;

use routes::create_router;
use state::{AppState, SharedState};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[tokio::main] // Allows us to use async fn main()
async fn main() {
    let state: SharedState = Arc::new(Mutex::new(AppState::new()));

    let app = create_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server running on http://{}", addr);

    if let Err(e) = axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await {
        eprintln!("Server error: {}", e);
    }
}
