use crate::handlers::*;
use std::sync::Arc;
use crate::repository::FriendRepository;
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_router(repo: Arc<dyn FriendRepository>) -> Router {
    Router::new()
        .route("/friends/request", post(send_friend_request))
        .route("/friends/response", post(respond_to_friend_request))
        .route("/friends", get(list_friends))
        .route("/friends/pending", get(list_pending))
        .with_state(repo)
}
