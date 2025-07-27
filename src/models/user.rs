use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(FromRow, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String, // hashed
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Debug, Clone)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email
        }
    }
}

impl From<&User> for UserResponse {
    fn from(user: &User) -> Self {
        Self {
            id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct UserWithImages {
    pub user: UserResponse,
    pub image_urls: Vec<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub raw_password: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct SignupRequest {
    pub username: String,
    pub email: String,
    pub raw_password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSession {
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>
}

impl UserSession {
    pub fn new(user_id: Uuid, expires_at: DateTime<Utc>) -> Self {
        Self {
            user_id,
            expires_at
        }
    }
}
