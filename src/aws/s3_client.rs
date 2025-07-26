use aws_sdk_s3::{
    Client,
    primitives::ByteStream
};
use aws_config::BehaviorVersion;
use anyhow::Result;


pub struct S3Client {
    client: Client,
}

impl S3Client {
    pub async fn new() -> Self {
        let behavior = BehaviorVersion::latest();
        let config = aws_config::load_defaults(behavior).await;
        let client = Client::new(&config);
        Self {
            client
        }
    }

    pub async fn download(&self, bucket: &str, key: &str) -> Result<ByteStream>{
        let result =
            self.client
                .get_object()
                .bucket(bucket)
                .key(key)
                .send()
                .await?
                .body;
            Ok(result)
    }

    pub async fn upload(
        &self,
        bucket: &str,
        key: &str,
        content_type: &str,
        body: ByteStream
    ) -> Result<()> {
        let result =
            self.client
                .put_object()
                .bucket(bucket)
                .key(key)
                .content_type(content_type)
                .body(body)
                .send()
                .await?;
        if let Some(etag) = result.e_tag() {
            println!("Successfully uploaded: {}", etag);
        }
        Ok(())
    }

    pub async fn delete(&self, bucket: &str, key: &str) -> Result<()> {
        let result =
            self.client
                .delete_object()
                .bucket(bucket)
                .key(key)
                .send()
                .await?;
        println!("Successfully deleted: {}", key);
        Ok(())
    }

    pub async fn list_objects(&self, bucket: &str) -> Result<Vec<String>> {
        let result =
            self.client
                .list_objects_v2()
                .bucket(bucket)
                .send()
                .await?;
        let objects =
            result.contents()
                .iter()
                .filter_map(|object| object.key().map(String::from))
                .collect();
        Ok(objects)
    }

    pub async fn exists(&self, bucket: &str, key: &str) -> Result<bool> {
        let result =
            self.client
                .head_object()
                .bucket(bucket)
                .key(key)
                .send()
                .await;

        match result {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
    }
}