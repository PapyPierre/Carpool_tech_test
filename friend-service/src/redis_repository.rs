use redis::{AsyncCommands, Client};
use crate::{errors::AppError, repository::FriendRepository};
use async_trait::async_trait;

pub struct RedisRepository {
    client: Client,
}

impl RedisRepository {
    pub fn new(redis_url: &str) -> Result<Self, AppError> {
        let client = Client::open(redis_url)
            .map_err(|e| AppError::Internal(e.to_string()))?;
        Ok(Self { client })
    }
}

#[async_trait]
impl FriendRepository for RedisRepository {

    async fn send_request(&self, from: &str, to: &str) -> Result<(), AppError> {
        if from == to {
            return Err(AppError::BadRequest(
                "Cannot send friend request to yourself".into(),
            ));
        }

        let mut conn = self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let is_friend: bool = conn
            .sismember(format!("friends:{from}"), to)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if is_friend {
            return Err(AppError::Conflict("Already friends".into()));
        }

        let _: usize = conn
            .sadd(format!("pending:received:{to}"), from)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let _: usize = conn
            .sadd(format!("pending:sent:{from}"), to)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(())
    }

    async fn respond_request(&self, user: &str, requester: &str, accept: bool) -> Result<(), AppError> {

        let mut conn = self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let exists: bool = conn
            .sismember(format!("pending:received:{user}"), requester)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if !exists {
            return Err(AppError::NotFound("Friend request not found".into()));
        }

        let _: usize =conn.srem(format!("pending:received:{user}"), requester)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let _: usize =conn.srem(format!("pending:sent:{requester}"), user)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        if accept {
            let _: usize =conn.sadd(format!("friends:{user}"), requester)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;

            let _: usize =conn.sadd(format!("friends:{requester}"), user)
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?;
        }

        Ok(())
    }

    async fn list_friends(&self, user: &str) -> Result<Vec<String>, AppError> {

        let mut conn = self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let friends: Vec<String> = conn
            .smembers(format!("friends:{user}"))
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok(friends)
    }

    async fn list_pending(&self, user: &str) -> Result<(Vec<String>, Vec<String>), AppError> {

        let mut conn = self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let received: Vec<String> = conn
            .smembers(format!("pending:received:{user}"))
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        let sent: Vec<String> = conn
            .smembers(format!("pending:sent:{user}"))
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        Ok((received, sent))
    }
}