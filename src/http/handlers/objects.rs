use crate::domain::object;
use crate::http::error::ApiError;
use axum::extract::{Path, Query, State};
use axum::http::header::{CONTENT_TYPE, ETAG};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::IntoResponse;
use serde::Deserialize;

use crate::http::state::AppState;

#[derive(Deserialize)]
pub struct ListObjectsQuery {
    #[serde(rename = "list-type", default)]
    pub list_type: Option<i32>,
}

pub async fn list_or_head_bucket(
    State(_state): State<AppState>,
    Query(q): Query<ListObjectsQuery>,
) -> impl IntoResponse {
    let xml = match q.list_type {
        Some(2) => r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult>
  <Contents/>
</ListBucketResult>"#
            .to_string(),
        _ => r#"<?xml version="1.0" encoding="UTF-8"?>
<ListBucketResult>
  <Contents/>
</ListBucketResult>"#
            .to_string(),
    };

    ([(CONTENT_TYPE, "application/xml")], xml)
}

pub async fn head_object_keyed(
    State(state): State<AppState>,
    Path((bucket, key)): Path<(String, String)>,
) -> Result<StatusCode, ApiError> {
    match state.object_service.head_object(&bucket, &key).await {
        Ok(_object) => Ok(StatusCode::OK),
        Err(crate::domain::DomainError::NoSuchBucket(_))
        | Err(crate::domain::DomainError::NoSuchKey(_)) => Ok(StatusCode::NOT_FOUND),
        Err(e) => Err(e.into()),
    }
}

pub async fn get_object_keyed(
    State(state): State<AppState>,
    Path((bucket, key)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    let (object, body) = state.object_service.get_object(&bucket, &key).await?;
    Ok((
        StatusCode::OK,
        [
            (CONTENT_TYPE, object.content_type),
            (ETAG, format!("\"{}\"", object.etag)),
        ],
        body,
    ))
}

pub async fn put_object_keyed(
    State(state): State<AppState>,
    Path((bucket, key)): Path<(String, String)>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!(bucket=%bucket, key=%key, body_size=body.len(), "put_object request received");

    let content_type = headers
        .get(CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);

    let object = state
        .object_service
        .put_object(body, bucket, key, content_type)
        .await?;

    tracing::info!(
        bucket_id=%object.bucket_id,
        key=%object.key,
        etag=%object.etag,
        size=object.size,
        "put_object succeeded"
    );

    let mut response_headers = HeaderMap::new();
    let etag_header = format!("\"{}\"", object.etag);
    if let Ok(value) = HeaderValue::from_str(&etag_header) {
        response_headers.insert(ETAG, value);
    }

    Ok((StatusCode::OK, response_headers))
}

pub async fn delete_object_keyed(
    State(state): State<AppState>,
    Path((bucket, key)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    state.object_service.delete_object(&bucket, &key).await?;
    Ok(StatusCode::NO_CONTENT)
}
