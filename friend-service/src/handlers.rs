use crate::{errors::AppError, models::*};
use axum::{
    extract::{Json, State},
    http::HeaderMap,
};

use std::sync::Arc;
use crate::repository::FriendRepository;

fn extract_user_id(headers: &HeaderMap) -> Result<String, AppError> {
    headers
        .get("X-User-Id")
        .ok_or_else(|| AppError::BadRequest("Missing X-User-Id header".into()))?
        .to_str()
        .map(|s| s.to_string())
        .map_err(|_| AppError::BadRequest("Invalid X-User-Id header".into()))
}

pub async fn send_friend_request(
    State(repo): State<Arc<dyn FriendRepository>>,
    headers: HeaderMap,
    Json(payload): Json<FriendRequest>,
) -> Result<(), AppError> {

    let user_id = extract_user_id(&headers)?;

    repo.send_request(&user_id, &payload.target_id).await?;

    Ok(())
}

pub async fn respond_to_friend_request(
    State(repo): State<Arc<dyn FriendRepository>>,
    headers: HeaderMap,
    Json(payload): Json<FriendResponse>,
) -> Result<(), AppError> {

    let user_id = extract_user_id(&headers)?;

    repo.respond_request(&user_id, &payload.requester_id, payload.accept).await?;

    Ok(())
}

pub async fn list_friends(
    State(repo): State<Arc<dyn FriendRepository>>,
    headers: HeaderMap,
) -> Result<Json<FriendsList>, AppError> {

    let user_id = extract_user_id(&headers)?;

    let friends = repo.list_friends(&user_id).await?;

    Ok(Json(FriendsList { friends }))
}

pub async fn list_pending(
    State(repo): State<Arc<dyn FriendRepository>>,
    headers: HeaderMap,
) -> Result<Json<PendingList>, AppError> {

    let user_id = extract_user_id(&headers)?;

    let list = repo.list_pending(&user_id).await?;

    Ok(Json(PendingList { received: list.0, sent: list.1 }))
}
