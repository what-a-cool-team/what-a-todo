use std::sync::Arc;

use axum::async_trait;

use crate::errors::{ApiResult, ApiError};
use crate::models::user::User;
use crate::repositories::user_repository::DynUserRepository;

pub type DynUserService = Arc<dyn UserService + Send + Sync>;
use bcrypt::hash;

#[async_trait]
pub trait UserService {
    async fn user_login(&self, id: &str, password: &str) -> ApiResult<String>;
    async fn user_signup(&self, id: &str, password: &str) -> ApiResult<User>;
}

#[derive(Clone)]
pub struct DomainUserService {
    pub user_repository: DynUserRepository,
}

impl DomainUserService {
    pub fn new(user_repository: DynUserRepository) -> Self {
        Self {
            user_repository,
        }
    }
}

#[async_trait]
impl UserService for DomainUserService {
    async fn user_login(&self, id: &str, password: &str) -> ApiResult<String> {
        Ok(self.user_repository.user_login(id.to_string(), password).await?)
    }

    async fn user_signup(&self, id: &str, password: &str) -> ApiResult<User> {
        let hashed_password = hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| ApiError::InternalServerError(format!("Hashing error: {}", e)))?;

        Ok(self
        .user_repository
        .create_user(User::new(id.to_string(), hashed_password))
        .await?)
    }
}