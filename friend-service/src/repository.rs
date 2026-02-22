use async_trait::async_trait;
use crate::errors::AppError;

#[async_trait]
pub trait FriendRepository: Send + Sync {
    async fn send_request(&self, from: &str, to: &str) -> Result<(), AppError>;
    async fn respond_request(&self, user: &str, requester: &str, accept: bool) -> Result<(), AppError>;
    async fn list_friends(&self, user: &str) -> Result<Vec<String>, AppError>;
    async fn list_pending(&self, user: &str) -> Result<(Vec<String>, Vec<String>), AppError>;
}