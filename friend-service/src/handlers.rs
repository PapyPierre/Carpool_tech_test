use crate::models::FriendRequest;
use axum::{
    extract::{State, Json},
    http::HeaderMap,
};
use crate::{state::SharedState, models::*, errors::AppError, state::AppState};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use axum::http::StatusCode;
use axum::response::IntoResponse;

fn extract_user_id(headers: &HeaderMap) -> Result<String, AppError> {
    headers
        .get("X-User-Id")
        .ok_or_else(|| AppError::BadRequest("Missing X-User-Id header".into()))?
        .to_str()
        .map(|s| s.to_string())
        .map_err(|_| AppError::BadRequest("Invalid X-User-Id header".into()))
}

pub async fn send_friend_request(
    State(state): State<Arc<Mutex<crate::state::AppState>>>,
    headers: HeaderMap,
    Json(payload): Json<FriendRequest>,
) -> impl IntoResponse {

    let user_id = match headers.get("X-User-Id") {
        Some(value) => match value.to_str() {
            Ok(v) => v.to_string(),
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    "Invalid X-User-Id header",
                )
            }
        },
        None => {
            return (
                StatusCode::BAD_REQUEST,
                "Missing X-User-Id header",
            )
        }
    };

    let target_id = payload.target_id;

    if user_id == target_id {
        return (
            StatusCode::BAD_REQUEST,
            "Cannot send friend request to yourself",
        );
    }

    let mut app_state = state.lock().unwrap();

    if let Some(friends) = app_state.friends.get(&user_id) {
        if friends.contains(&target_id) {
            return (
                StatusCode::CONFLICT,
                "Already friends",
            );
        }
    }

    app_state
        .pending
        .entry(target_id.clone())
        .or_insert_with(HashSet::new)
        .insert(user_id.clone());

    (
        StatusCode::OK,
        "Friend request sent",
    )
}

pub async fn respond_to_friend_request(
    State(state): State<Arc<Mutex<AppState>>>,
    headers: HeaderMap,
    Json(payload): Json<FriendResponse>,
) -> impl IntoResponse {
    let user_id = match headers.get("X-User-Id") {
        Some(value) => match value.to_str() {
            Ok(v) => v.to_string(),
            Err(_) => {
                return (StatusCode::BAD_REQUEST, "Invalid X-User-Id header")
            }
        },
        None => {
            return (StatusCode::BAD_REQUEST, "Missing X-User-Id header")
        }
    };

    let requester_id = payload.requester_id;

    let mut app_state = state.lock().unwrap();

    match app_state.pending.get_mut(&user_id) {
        Some(requests) => {
            if !requests.remove(&requester_id) {
                return (StatusCode::NOT_FOUND, "Friend request not found");
            }
        }
        None => {
            return (StatusCode::NOT_FOUND, "No pending requests");
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

    (StatusCode::OK, "Friend request processed")
}

pub async fn list_friends(
    State(state): State<Arc<Mutex<AppState>>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let user_id = match headers.get("X-User-Id") {
        Some(value) => match value.to_str() {
            Ok(v) => v.to_string(),
            Err(_) => return (StatusCode::BAD_REQUEST, "Invalid X-User-Id header").into_response(),
        },
        None => return (StatusCode::BAD_REQUEST, "Missing X-User-Id header").into_response(),
    };

    let app_state = state.lock().unwrap();

    let friends = app_state
        .friends
        .get(&user_id)
        .map(|set| set.iter().cloned().collect())
        .unwrap_or_else(Vec::new);

    Json(FriendsList { friends }).into_response()
}

pub async fn list_pending(
    State(state): State<Arc<Mutex<AppState>>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let user_id = match headers.get("X-User-Id") {
        Some(value) => match value.to_str() {
            Ok(v) => v.to_string(),
            Err(_) => return (StatusCode::BAD_REQUEST, "Invalid X-User-Id header").into_response(),
        },
        None => return (StatusCode::BAD_REQUEST, "Missing X-User-Id header").into_response(),
    };

    let app_state = state.lock().unwrap();

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

    Json(PendingList { received, sent }).into_response()
}