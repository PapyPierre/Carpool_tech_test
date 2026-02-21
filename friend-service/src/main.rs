mod errors;
mod state;
mod models;
mod handlers;
mod routes;

use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use axum::{
    extract::{State, Json},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{get, Router, post},
};
use serde::{Deserialize, Serialize};

async fn hello() -> &'static str {
    "Server is running"
}

#[tokio::main] // Allows us to use async fn main()
async fn main() {
    // Create shared state
    let state :Arc<Mutex<state::AppState>> = Arc::new(Mutex::new(state::AppState {
        friends: HashMap::new(),
        pending: HashMap::new(),
    }));

    let app = Router::new()
        .route("/", get(hello))
        .route("/friends/request", post(handlers::send_friend_request))
        .route("/friends/response", post(handlers::respond_to_friend_request))
        .route("/friends", get(handlers::list_friends))
        .route("/friends/pending", get(handlers::list_pending))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app, ).await.unwrap();
}