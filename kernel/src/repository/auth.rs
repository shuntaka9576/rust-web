use async_trait::async_trait;
use shared::error::AppResult;

use crate::model::{
    auth::{event::CreateToken, AccessToken},
    id::UserId,
};

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn verify_user(&self, email: &str, pasword: &str) -> AppResult<UserId>;
    async fn create_token(&self, event: CreateToken) -> AppResult<AccessToken>;
    async fn delete_token(&self, access_token: AccessToken) -> AppResult<()>;
}
