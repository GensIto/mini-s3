use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Object {
    pub object_id: String,
    pub bucket_id: String,
    pub key: String,
    pub size: i64,
    pub content_type: String,
    pub etag: String,
    pub storage_path: String,
    pub created_at: String,
    pub updated_at: String,
}
