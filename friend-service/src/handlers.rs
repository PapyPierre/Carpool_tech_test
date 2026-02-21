use crate::{errors::AppError, models::*, state::SharedState};
use axum::{
    extract::{Json, State},
    http::HeaderMap,
};
use std::collections::HashSet;

fn extract_user_id(headers: &HeaderMap) -> Result<String, AppError> {
    headers
        .get("X-User-Id")
        .ok_or_else(|| AppError::BadRequest("Missing X-User-Id header".into()))?
        .to_str()
        .map(|s| s.to_string())
        .map_err(|_| AppError::BadRequest("Invalid X-User-Id header".into()))
}

pub async fn send_friend_request(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Json(payload): Json<FriendRequest>,
) -> Result<(), AppError> {
    let user_id = extract_user_id(&headers)?;
    let target_id = payload.target_id;

    println!("Received request from {0} to {1}", user_id, target_id);

    if user_id == target_id {
        return Err(AppError::BadRequest(
            "Cannot send friend request to yourself".into(),
        ));
    }

    let mut app_state = state
        .lock()
        .map_err(|_| AppError::Internal("State lock poisoned".into()))?;

    if let Some(friends) = app_state.friends.get(&user_id) {
        if friends.contains(&target_id) {
            return Err(AppError::Conflict("Already friends".into()));
        }
    }

    app_state
        .pending
        .entry(target_id.clone())
        .or_insert_with(HashSet::new)
        .insert(user_id.clone());

    Ok(())
}

pub async fn respond_to_friend_request(
    State(state): State<SharedState>,
    headers: HeaderMap,
    Json(payload): Json<FriendResponse>,
) -> Result<(), AppError> {
    let user_id = extract_user_id(&headers)?;
    let requester_id = payload.requester_id;

    println!(
        "Received response from {0} to {1} request",
        user_id, requester_id
    );

    let mut app_state = state
        .lock()
        .map_err(|_| AppError::Internal("State lock poisoned".into()))?;

    match app_state.pending.get_mut(&user_id) {
        Some(requests) => {
            if !requests.remove(&requester_id) {
                return Err(AppError::NotFound("Already friends".into()));
            }
        }
        None => {
            return Err(AppError::NotFound("No pending requests".into()));
        }
    }

    if payload.accept {
        app_state
            .friends
            .entry(user_id.clone())
            .or_insert_with(HashSet::new)
            .insert(requester_id.clone());

        app_state
            .friends
            .entry(requester_id.clone())
            .or_insert_with(HashSet::new)
            .insert(user_id.clone());
    }

    Ok(())
}

pub async fn list_friends(
    State(state): State<SharedState>,
    headers: HeaderMap,
) -> Result<Json<FriendsList>, AppError> {
    let user_id = extract_user_id(&headers)?;

    println!("Received list_friends from {0}", user_id);

    let app_state = state
        .lock()
        .map_err(|_| AppError::Internal("State lock poisoned".into()))?;

    let friends = app_state
        .friends
        .get(&user_id)
        .map(|set| set.iter().cloned().collect())
        .unwrap_or_else(Vec::new);

    Ok(Json(FriendsList { friends }))
}

pub async fn list_pending(
    State(state): State<SharedState>,
    headers: HeaderMap,
) -> Result<Json<PendingList>, AppError> {
    let user_id = extract_user_id(&headers)?;

    println!("Received list_pending from {0}", user_id);

    let app_state = state
        .lock()
        .map_err(|_| AppError::Internal("State lock poisoned".into()))?;

    let received = app_state
        .pending
        .get(&user_id)
        .map(|set| set.iter().cloned().collect())
        .unwrap_or_else(Vec::new);

    let mut sent = Vec::new();

    for (target, requesters) in &app_state.pending {
        if requesters.contains(&user_id) {
            sent.push(target.clone());
        }
    }

    Ok(Json(PendingList { received, sent }))
}
