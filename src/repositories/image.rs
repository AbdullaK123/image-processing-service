use aws_sdk_s3::primitives::ByteStream;
use sqlx::PgPool;
use crate::aws::S3Client;
use std::env::var;
use crate::models::Image;
use anyhow::Result;
use uuid::Uuid;

pub struct ImageRepository {
    pool: PgPool,
    s3: S3Client,
    bucket: String
}

impl ImageRepository {
    pub async fn new(pool: PgPool) -> Self {
        let s3 = S3Client::new().await;
        let bucket = var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME must be set");
        Self { pool, s3, bucket }
    }

    fn get_user_key(&self, user_id: Uuid) -> String {
        format!("images/{}", user_id)
    }

    fn get_image_url(&self, user_key: String, file_name: &str) -> String {
        format!("s3://{}/{}/{}", &self.bucket, user_key, file_name)
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        content_type: &str,
        file_name: &str,
        payload: ByteStream
    ) -> Result<Image> {
        let user_key = self.get_user_key(user_id);
        let object_key = format!("{}/{}", user_key, file_name);
        self.s3.upload(&self.bucket, &object_key, content_type, payload).await?;
        let url = self.get_image_url(user_key, file_name);
        let result =
            sqlx::query_as!(
                Image,
                "INSERT INTO images (url, user_id) VALUES ($1, $2) RETURNING *",
                url,
                user_id
            ).fetch_one(&self.pool).await;
        match result {
            Ok(image) => Ok(image),
            Err(e) => {
                let _ = self.s3.delete(&self.bucket, &object_key).await;
                Err(e.into())
            }
        }
    }

    pub async fn get_by_user_id(
        &self,
        user_id: Uuid
    ) -> Result<Vec<Image>> {
        let images = sqlx::query_as!(
            Image,
            "SELECT * FROM images WHERE user_id = $1",
            user_id
        ).fetch_all(&self.pool).await?;
        Ok(images)
    }

    pub async fn get_by_id(
        &self,
        user_id: Uuid,
        image_id: Uuid
    ) -> Result<Option<Image>> {
        let image = sqlx::query_as!(
            Image,
            "SELECT * FROM images WHERE user_id = $1 AND id = $2",
            user_id,
            image_id
        ).fetch_optional(&self.pool).await?;
        Ok(image)
    }

    pub async fn delete(
        &self,
        user_id: Uuid,
        image_id: Uuid,
        file_name: &str
    ) -> Result<bool> {
        let user_key = self.get_user_key(user_id);
        let object_key = format!("{}/{}", user_key, file_name);
        self.s3.delete(&self.bucket, &object_key).await?;
        let result = sqlx::query!(
            "DELETE FROM images WHERE user_id = $1 AND id = $2",
            user_id,
            image_id
        ).execute(&self.pool).await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn update(
        &self,
        user_id: Uuid,
        image_id: Uuid,
        file_name: &str,
        content_type: &str,
        updated_payload: ByteStream
    ) -> Result<Image> {
       let current_image = self.get_by_id(user_id, image_id).await?;

        let user_key = self.get_user_key(user_id);
        let object_key = format!("{}/{}", user_key, file_name);

        let result =
            if let Some(image) = current_image {
                self.s3.upload(
                    &self.bucket,
                    &object_key,
                    content_type,
                    updated_payload
                ).await?;
                Ok(image)
            } else {
                Err(anyhow::anyhow!("Image not found"))
            };

        result
    }
}

