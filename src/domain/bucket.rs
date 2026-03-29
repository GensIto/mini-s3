use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Bucket {
    pub bucket_id: String,
    pub name: String,
    pub created_at: String,
}
