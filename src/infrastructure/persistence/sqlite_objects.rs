use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::application::ports::ObjectRepository;
use crate::domain::{DomainError, Object};

pub struct SqliteObjectRepository {
    pool: SqlitePool,
}

impl SqliteObjectRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ObjectRepository for SqliteObjectRepository {
    async fn put_object(&self, object: &Object) -> Result<(), DomainError> {
        sqlx::query(
            r#"
            INSERT INTO objects (object_id, bucket_id, key, size, content_type, etag, storage_path, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(bucket_id, key) DO UPDATE SET
                object_id = excluded.object_id,
                size = excluded.size,
                content_type = excluded.content_type,
                etag = excluded.etag,
                storage_path = excluded.storage_path,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(&object.object_id)
        .bind(&object.bucket_id)
        .bind(&object.key)
        .bind(object.size)
        .bind(&object.content_type)
        .bind(&object.etag)
        .bind(&object.storage_path)
        .bind(&object.created_at)
        .bind(&object.updated_at)
        .execute(&self.pool)
        .await
        .inspect_err(|e| tracing::error!(error=%e, object_id=%object.object_id, "sqlite insert/update failed"))
        .map_err(|_| DomainError::Internal)?;

        Ok(())
    }
}
