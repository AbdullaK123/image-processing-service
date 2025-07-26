use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::Serialize;
use crate::models::user::UserResponse;

#[derive(FromRow, Clone, Debug)]
pub struct Image {
    pub id: Uuid,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: Uuid
}

impl From<Image> for ImageResponse {
    fn from(image: Image) -> Self {
        Self {
            id: image.id,
            url: image.url,
            created_at: image.created_at,
            updated_at: image.updated_at
        }
    }   
}

impl From<&Image> for ImageResponse {
    fn from(image: &Image) -> Self {
        Self {
            id: image.id,
            url: image.url.clone(),
            created_at: image.created_at,
            updated_at: image.updated_at
        }
    }  
}

#[derive(Serialize, Clone, Debug)]
pub struct ImageResponse {
    pub id: Uuid,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Serialize, Clone, Debug)]
pub struct ImageWithUser {
    pub image: ImageResponse,
    pub user: UserResponse
}

#[derive(Serialize, Clone, Debug)]
pub struct ImageGalleryResponse {
    pub user: UserResponse,
    pub images: Vec<ImageResponse>
}

