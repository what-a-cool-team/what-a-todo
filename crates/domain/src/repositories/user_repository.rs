use std::sync::Arc;

use axum::async_trait;
use sqlx::{query, Pool, Postgres, Row};
use crate::models::user::User;
use serde::{Serialize, Deserialize};

use bcrypt::verify;
use chrono::{Duration, Utc};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use sha2::Sha256;
use base64;

pub type DynUserRepository = Arc<dyn UserRepository + Send + Sync>;

#[async_trait]
pub trait UserRepository {
    async fn user_login(&self, id: String, password: &str) -> anyhow::Result<String>;
    async fn create_user(&self, user : User) -> anyhow::Result<User>;
}

#[derive(Clone)]
pub struct DomainUserRepository {
    pool: Pool<Postgres>,
    auth: settings::settings::Auth,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

impl DomainUserRepository {
    pub fn new(pool: Pool<Postgres>, auth: settings::settings::Auth) -> Self {
        Self { pool, auth }
    }
}

#[async_trait]
impl UserRepository for DomainUserRepository {
    async fn user_login(&self, id: String, password: &str) -> anyhow::Result<String> {
        let row = query(
            "
            SELECT id, password_hash
            FROM users
            WHERE id = $1
            ",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await?;
        
        let user = User {
            id: row.get("id"),
            password_hash: row.get("password_hash"),
        };


        if verify(password, &user.password_hash)? {
            let expiration = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp() as usize;

            let decoded_key = base64::decode(&self.auth.jwk)
                .map_err(|e| anyhow::anyhow!("Failed to decode JWK from base64: {}", e))?;
    
            let key: Hmac<Sha256> = Hmac::new_from_slice(&decoded_key)?;

            let claims = Claims {
                sub: user.id.clone(),
                exp: expiration,
            };

            let token_str = claims.sign_with_key(&key)?;         
            Ok(token_str) 
        }else {
            Err(anyhow::Error::msg("Invalid password"))
        }
    }

    async fn create_user(&self, user : User) -> anyhow::Result<User> {
        let row = query(
            "
            INSERT INTO users (id, password_hash)
            VALUES ($1, $2)
            RETURNING *
            ",
        )
        .bind(user.id)
        .bind(user.password_hash)
        .fetch_one(&self.pool)
        .await?;
        let user = User {
            id: row.get("id"),
            password_hash: row.get("password_hash"),
        };
        Ok(user)
    }
}