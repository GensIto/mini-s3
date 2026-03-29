use axum::extract::{Path, State};
use axum::http::header::CONTENT_TYPE;
use axum::response::IntoResponse;

use crate::http::error::ApiError;
use crate::http::state::AppState;

fn default_owner() -> String {
    std::env::var("DEFAULT_OWNER_ACCESS_KEY").unwrap_or_else(|_| "local-dev".to_string())
}

pub async fn list_buckets(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Phase 2 で認証ミドルウェアから Extension<AccessKey> で受け取る
    let owner_access_key = default_owner();
    tracing::debug!(owner=%owner_access_key, "list_buckets called");
    let buckets = state.bucket_service.list_buckets(&owner_access_key).await
        .inspect_err(|e| tracing::error!(error=%e, "list_buckets failed"))?;
    tracing::info!(count=buckets.len(), "list_buckets succeeded");

    let bucket_entries: String = buckets
        .iter()
        .map(|b| {
            format!(
                "    <Bucket><Name>{}</Name><CreationDate>{}</CreationDate></Bucket>",
                b.name, b.created_at
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<ListAllMyBucketsResult>
  <Owner>
    <ID>{owner}</ID>
    <DisplayName>{owner}</DisplayName>
  </Owner>
  <Buckets>
{bucket_entries}
  </Buckets>
</ListAllMyBucketsResult>"#,
        owner = owner_access_key,
    );

    Ok(([(CONTENT_TYPE, "application/xml")], xml))
}

pub async fn create_bucket(
    State(state): State<AppState>,
    Path(bucket): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    // TODO: Phase 2 で認証ミドルウェアから Extension<AccessKey> で受け取る
    let owner_access_key = default_owner();
    let bucket = state
        .bucket_service
        .create_bucket(&bucket, &owner_access_key)
        .await?;

    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<CreateBucketResult>
  <BucketArn>arn:aws:s3:::{}</BucketArn>
</CreateBucketResult>"#,
        bucket.name
    );

    Ok(([(CONTENT_TYPE, "application/xml")], xml))
}
