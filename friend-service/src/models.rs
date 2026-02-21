use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct FriendRequest {
    pub target_id: String,
}

#[derive(Deserialize)]
pub struct FriendResponse {
    pub requester_id: String,
    pub accept: bool,
}

#[derive(Serialize)]
pub struct FriendsList {
    pub friends: Vec<String>,
}

#[derive(Serialize)]
pub struct PendingList {
    pub received: Vec<String>,
    pub sent: Vec<String>,
}