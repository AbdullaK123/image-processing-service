mod aws;
mod config;
mod models;
mod repositories;
mod security;
mod middleware;
mod services;

use aws_sdk_s3::primitives::ByteStream;
use anyhow::Result;
use crate::aws::S3Client;

#[tokio::main]
async fn main() -> Result<()> {
    let s3 = S3Client::new().await;
    let body = ByteStream::from_path("test_images/blurred.png").await?;

    let response =
        s3.upload(
            "rust-image-service-bucket",
            "images/blurred.png",
            "image/png",
            body
        ).await?;

    println!("Object uploaded successfully!");

    Ok(())

}
