use std::sync::Arc;

use sqlx::SqlitePool;

use crate::application::{BucketService, ObjectService};
use crate::infrastructure::persistence::{SqliteBucketRepository, SqliteObjectRepository};

#[derive(Clone)]
pub struct AppState {
    pub bucket_service: Arc<BucketService>,
    pub object_service: Arc<ObjectService>,
}

impl AppState {
    pub fn new(pool: SqlitePool) -> Self {
        let bucket_repo = Arc::new(SqliteBucketRepository::new(pool.clone()));
        let object_repo = Arc::new(SqliteObjectRepository::new(pool));
        let bucket_service = Arc::new(BucketService::new(bucket_repo.clone()));
        let object_service = Arc::new(ObjectService::new(bucket_repo.clone(), object_repo));

        Self {
            bucket_service,
            object_service,
        }
    }
}
