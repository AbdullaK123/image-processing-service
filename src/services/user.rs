use sqlx::PgPool;
use crate::models::{LoginRequest, SignupRequest, UserResponse, UserSession};
use crate::repositories::UserRepository;
use anyhow::{anyhow, Result};
use tower_sessions::Session;
use chrono::{
    Utc,
    Duration
};
use uuid::Uuid;
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

    pub async fn signup_user(&self, payload: SignupRequest) -> Result<UserResponse> {
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

    pub async fn login_user(&self, session: Session, credentials: LoginRequest) -> Result<UserResponse> {
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
                    let user_id = user.id;
                    let expires_at = Utc::now() + Duration::days(1);
                    let user_session = UserSession::new(user_id, expires_at);
                    session.insert("user_session", user_session).await?;
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

    pub async fn logout_user(&self, session: Session) -> Result<()> {
        session.flush().await?;
        Ok(())
    }

    async fn extract_session_data(&self, session: &Session) -> Result<UserSession> {
        let user_session = session.get::<UserSession>("user_session").await?;
        if let Some(user_session) = user_session {
            Ok(user_session)
        } else {
            Err(anyhow!("User session not found"))
        }
    }

    async fn get_user_response_by_id(&self, user_id: Uuid) -> Result<Option<UserResponse>> {
        let user = self.repo.get_by_id(user_id).await?;
        if let Some(user) = user {
            Ok(Some(UserResponse::from(user)))
        } else {
            Ok(None)
        }
    }

    pub async fn get_current_user(&self, session: Session) -> Result<Option<UserResponse>> {
        let user_session = self.extract_session_data(&session).await?;
        if Utc::now() < user_session.expires_at {
            self.get_user_response_by_id(user_session.user_id).await
        } else {
            session.flush().await?;
            Ok(None)
        }
    }
}

