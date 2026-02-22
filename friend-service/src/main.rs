mod errors;
mod handlers;
mod models;
mod routes;
mod repository;
mod redis_repository;

use routes::create_router;
use repository::FriendRepository;
use redis_repository::RedisRepository;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main] // Allows us to use async fn main()
async fn main() {
    
    let repo = RedisRepository::new("redis://redis:6379/")
        .expect("Failed to create Redis repository");

    let shared_repo: Arc<dyn FriendRepository> = Arc::new(repo);

    let app = create_router(shared_repo);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("Server running on http://{}", addr);

    if let Err(e) = axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app).await {
        eprintln!("Server error: {}", e);
    }
}
