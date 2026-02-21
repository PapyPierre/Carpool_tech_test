use axum::{routing::{get, post}, Router};
use crate::handlers::*;
use crate::state::SharedState;

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route("/friends/request", post(send_friend_request))
        .route("/friends/response", post(respond_to_friend_request))
        .route("/friends", get(list_friends))
        .route("/friends/pending", get(list_pending))
        .with_state(state)
}