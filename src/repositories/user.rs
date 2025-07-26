use sqlx::PgPool;
use anyhow::Result;
use uuid::Uuid;
use crate::security::{
    hash_password
};
use crate::models::{
    User,
    SignupRequest
};

pub struct UserRepository {
    pool: PgPool
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, payload: SignupRequest) -> Result<User> {
        let user =
            sqlx::query_as!(
                User,
                "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING *",
                payload.username,
                payload.email,
                hash_password(payload.raw_password.as_str())?
            ).fetch_one(&self.pool).await?;
        Ok(user)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user =
            sqlx::query_as!(
                User,
                "SELECT * FROM users WHERE id = $1",
                id
            ).fetch_optional(&self.pool).await?;
        Ok(user)
    }

    pub async fn get_by_email(&self, email: String) -> Result<Option<User>> {
        let user =
            sqlx::query_as!(
                User,
                "SELECT * FROM users WHERE email = $1",
                email
            ).fetch_optional(&self.pool).await?;
        Ok(user)
    }
}