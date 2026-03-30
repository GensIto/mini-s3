use async_trait::async_trait;

use crate::domain::{Bucket, DomainError, Object};

#[async_trait]
pub trait BucketRepository: Send + Sync {
    async fn list_buckets(&self, owner_access_key: &str) -> Result<Vec<Bucket>, DomainError>;
    async fn get_bucket(
        &self,
        bucket_name: &str,
        owner_access_key: &str,
    ) -> Result<Bucket, DomainError>;
    async fn create_bucket(
        &self,
        name: &str,
        owner_access_key: &str,
    ) -> Result<Bucket, DomainError>;
    async fn delete_bucket(&self, name: &str, owner_access_key: &str) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ObjectRepository: Send + Sync {
    async fn put_object(&self, object: &Object) -> Result<(), DomainError>;
}
