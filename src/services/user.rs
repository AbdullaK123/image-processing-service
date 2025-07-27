use sqlx::PgPool;
use crate::models::{LoginRequest, SignupRequest, UserResponse, UserSession};
use crate::repositories::UserRepository;
use anyhow::{anyhow, Result};
use tower_sessions::Session;
use uuid::Uuid;
use chrono::{
    Utc,
    Duration
};
use crate::security::{
    verify_password
};


pub struct UserService {
    repo: UserRepository
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        let repo = UserRepository::new(pool);
        Self { repo }
    }

    pub async fn sign_up_user(&self, payload: SignupRequest) -> Result<UserResponse> {
        let user = self.repo.create(payload).await?;
        Ok(UserResponse::from(user))
    }

    async fn authenticate_user(&self, credentials: LoginRequest) -> Result<bool> {
        let user = self.repo.get_by_email(credentials.email).await?;
        if let Some(user) = user {
            let credentials_valid =
                verify_password(
                    credentials.raw_password.as_str(),
                    user.password.as_str()
                )?;
            Ok(credentials_valid)
        } else {
            Err(anyhow!("User not found"))
        }
    }

    async fn login_user(&self, session: Session, credentials: LoginRequest) -> Result<UserResponse> {
        let user = self.repo.get_by_email(credentials.email).await?;
        match user {
            Some(user) => {
                let credentials_valid =
                    verify_password(
                        credentials.raw_password.as_str(),
                        user.password.as_str()
                    )?;
                if credentials_valid {
                    // create a user session and set the browser cookie
                    let session_id = Uuid::new_v4().to_string();
                    let session_id = session_id.as_str();
                    let user_id = user.id;
                    let expires_at = Utc::now() + Duration::days(1);
                    let user_session = UserSession::new(user_id, expires_at);
                    session.insert(session_id, user_session).await?;
                    session.save().await?;
                    Ok(UserResponse::from(user))
                } else {
                    Err(anyhow!("Invalid credentials"))
                }
            },
            None => {
                Err(anyhow!("User not found"))
            }
        }
    }

    async fn logout_user(&self, session: Session) -> Result<()> {
        session.flush().await?;
        Ok(())
    }

}

