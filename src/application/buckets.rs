use std::sync::Arc;

use crate::application::ports::BucketRepository;
use crate::domain::{Bucket, DomainError};

pub struct BucketService {
    repo: Arc<dyn BucketRepository>,
}

impl BucketService {
    pub fn new(repo: Arc<dyn BucketRepository>) -> Self {
        Self { repo }
    }

    pub async fn list_buckets(&self, owner_access_key: &str) -> Result<Vec<Bucket>, DomainError> {
        self.repo.list_buckets(owner_access_key).await
    }

    pub async fn create_bucket(
        &self,
        bucket_name: &str,
        owner_access_key: &str,
    ) -> Result<Bucket, DomainError> {
        let bucket = self
            .repo
            .create_bucket(bucket_name, owner_access_key)
            .await?;

        tokio::fs::create_dir_all(format!("data/{}", bucket.name)).await?;

        Ok(bucket)
    }
}
