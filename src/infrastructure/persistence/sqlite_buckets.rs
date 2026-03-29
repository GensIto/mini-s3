use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::application::ports::BucketRepository;
use crate::domain::{Bucket, DomainError};

pub struct SqliteBucketRepository {
    pool: SqlitePool,
}

impl SqliteBucketRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[derive(sqlx::FromRow)]
struct BucketRow {
    bucket_id: String,
    name: String,
    created_at: String,
}

impl From<BucketRow> for Bucket {
    fn from(row: BucketRow) -> Self {
        Bucket {
            bucket_id: row.bucket_id,
            name: row.name,
            created_at: row.created_at,
        }
    }
}

#[async_trait]
impl BucketRepository for SqliteBucketRepository {
    async fn list_buckets(&self, owner_access_key: &str) -> Result<Vec<Bucket>, DomainError> {
        let rows = sqlx::query_as::<_, BucketRow>(
            "SELECT bucket_id, name, created_at FROM buckets WHERE owner_access_key = ? ORDER BY name",
        )
        .bind(owner_access_key)
        .fetch_all(&self.pool)
        .await
        .inspect_err(|e| tracing::error!(error=%e, "sqlite list_buckets query failed"))
        .map_err(|_| DomainError::Internal)?;

        Ok(rows.into_iter().map(Into::into).collect::<Vec<Bucket>>())
    }

    async fn get_bucket(
        &self,
        bucket_name: &str,
        owner_access_key: &str,
    ) -> Result<Bucket, DomainError> {
        let row = sqlx::query_as::<_, BucketRow>(
            "SELECT bucket_id, name, created_at FROM buckets WHERE name = ? AND owner_access_key = ?",
        )
        .bind(bucket_name)
        .bind(owner_access_key)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::RowNotFound => DomainError::NotFound,
            _ => DomainError::Internal,
        })?;

        Ok(row.into())
    }

    async fn create_bucket(
        &self,
        name: &str,
        owner_access_key: &str,
    ) -> Result<Bucket, DomainError> {
        let bucket_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            r#"
            INSERT INTO buckets (bucket_id, name, owner_access_key, created_at)
            VALUES (?, ?, ?, ?)
            "#,
        )
        .bind(&bucket_id)
        .bind(name)
        .bind(owner_access_key)
        .bind(&now)
        .execute(&self.pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db) if db.is_unique_violation() => {
                DomainError::BucketAlreadyExists
            }
            _ => DomainError::Internal,
        })?;

        Ok(Bucket {
            bucket_id,
            name: name.to_string(),
            created_at: now,
        })
    }
}
