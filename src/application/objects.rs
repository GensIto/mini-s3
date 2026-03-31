use std::sync::Arc;

use crate::application::ports::{BucketRepository, ObjectRepository};
use crate::domain::{DomainError, Object, s3_etag_hex};

fn default_owner() -> String {
    std::env::var("DEFAULT_OWNER_ACCESS_KEY").unwrap_or_else(|_| "local-dev".to_string())
}

pub struct ObjectService {
    bucket_repo: Arc<dyn BucketRepository>,
    object_repo: Arc<dyn ObjectRepository>,
}

impl ObjectService {
    pub fn new(
        bucket_repo: Arc<dyn BucketRepository>,
        object_repo: Arc<dyn ObjectRepository>,
    ) -> Self {
        Self {
            bucket_repo,
            object_repo,
        }
    }

    pub async fn head_object(&self, bucket_name: &str, key: &str) -> Result<Object, DomainError> {
        let owner_access_key = default_owner();
        let bucket = self
            .bucket_repo
            .get_bucket(&bucket_name, &owner_access_key)
            .await
            .inspect_err(
                |e| tracing::warn!(bucket=%bucket_name, error=%e, "bucket lookup failed"),
            )?;

        let object = self.object_repo
            .get_object(&bucket.bucket_id, key)
            .await
            .inspect_err(
                |e| tracing::warn!(bucket_id=%bucket.bucket_id, key=%key, error=%e, "object lookup failed"),
            )?;

        Ok(object)
    }

    pub async fn get_object(
        &self,
        bucket_name: &str,
        key: &str,
    ) -> Result<(Object, Vec<u8>), DomainError> {
        let owner_access_key = default_owner();
        let bucket = self
            .bucket_repo
            .get_bucket(&bucket_name, &owner_access_key)
            .await
            .inspect_err(
                |e| tracing::warn!(bucket=%bucket_name, error=%e, "bucket lookup failed"),
            )?;

        let object = self.object_repo
            .get_object(&bucket.bucket_id, key)
            .await
            .inspect_err(
                |e| tracing::warn!(bucket_id=%bucket.bucket_id, key=%key, error=%e, "object lookup failed"),
            )?;

        let full_path = std::path::Path::new(&object.storage_path);
        let body = tokio::fs::read(full_path).await?;

        Ok((object, body.into()))
    }

    pub async fn put_object(
        &self,
        body: axum::body::Bytes,
        bucket_name: String,
        key: String,
        content_type: Option<String>,
    ) -> Result<Object, DomainError> {
        let owner_access_key = default_owner();
        let bucket = self
            .bucket_repo
            .get_bucket(&bucket_name, &owner_access_key)
            .await
            .inspect_err(
                |e| tracing::warn!(bucket=%bucket_name, error=%e, "bucket lookup failed"),
            )?;

        let etag = s3_etag_hex(body.as_ref());
        let content_type = content_type
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| "application/octet-stream".to_string());
        let size = body.len() as i64;
        let now = chrono::Utc::now().to_rfc3339();
        let object_id = uuid::Uuid::new_v4().to_string();
        let storage_path = format!("data/{}/{}", bucket.name, key);

        let full_path = std::path::Path::new(&storage_path);
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        tokio::fs::write(full_path, &body).await.inspect_err(
            |e| tracing::error!(path=%storage_path, error=%e, "failed to write object to disk"),
        )?;

        tracing::debug!(object_id=%object_id, storage_path=%storage_path, "object written to disk");

        let object = Object {
            object_id,
            bucket_id: bucket.bucket_id,
            key,
            size,
            content_type,
            etag,
            storage_path,
            created_at: now.clone(),
            updated_at: now,
        };

        self.object_repo.put_object(&object).await?;

        Ok(object)
    }

    pub async fn delete_object(&self, bucket_name: &str, key: &str) -> Result<(), DomainError> {
        let owner_access_key = default_owner();
        let bucket = self
            .bucket_repo
            .get_bucket(&bucket_name, &owner_access_key)
            .await?;
        self.object_repo
            .delete_object(&bucket.bucket_id, &key)
            .await?;

        let storage_path = format!("data/{}/{}", bucket.name, key);
        let full_path = std::path::Path::new(&storage_path);

        tokio::fs::remove_file(full_path).await.inspect_err(
            |e| tracing::error!(path=%full_path.to_string_lossy(), error=%e, "failed to delete object from disk"),
        )?;

        tracing::debug!(path=%full_path.to_string_lossy(), "object deleted from disk");
        Ok(())
    }
}
