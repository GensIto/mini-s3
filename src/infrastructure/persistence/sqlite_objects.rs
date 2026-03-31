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

#[derive(sqlx::FromRow)]
struct ObjectRow {
    object_id: String,
    bucket_id: String,
    key: String,
    size: i64,
    content_type: String,
    etag: String,
    storage_path: String,
    created_at: String,
    updated_at: String,
}

impl From<ObjectRow> for Object {
    fn from(row: ObjectRow) -> Self {
        Object {
            object_id: row.object_id,
            bucket_id: row.bucket_id,
            key: row.key,
            size: row.size,
            content_type: row.content_type,
            etag: row.etag,
            storage_path: row.storage_path,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[async_trait]
impl ObjectRepository for SqliteObjectRepository {
    async fn get_object(&self, bucket_id: &str, key: &str) -> Result<Object, DomainError> {
        let row = sqlx::query_as::<_, ObjectRow>("SELECT * FROM objects WHERE bucket_id = ? AND key = ?")
            .bind(bucket_id)
            .bind(key)
            .fetch_one(&self.pool)
            .await
            .inspect_err(|e| tracing::error!(error=%e, bucket_id=%bucket_id, key=%key, "sqlite get_object query failed"))
            .map_err(|_| DomainError::Internal)?;

        Ok(row.into())
    }

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

    async fn delete_object(&self, bucket_id: &str, key: &str) -> Result<(), DomainError> {
        sqlx::query("DELETE FROM objects WHERE bucket_id = ? AND key = ?")
            .bind(bucket_id)
            .bind(key)
            .execute(&self.pool)
            .await
            .inspect_err(|e| tracing::error!(error=%e, bucket_id=%bucket_id, key=%key, "sqlite delete_object query failed"))
            .map_err(|_| DomainError::Internal)?;

        Ok(())
    }
}
