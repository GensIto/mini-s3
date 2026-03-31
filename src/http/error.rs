use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;
use axum::response::{IntoResponse, Response};

use crate::domain::DomainError;

pub struct ApiError(DomainError);

impl From<DomainError> for ApiError {
    fn from(value: DomainError) -> Self {
        ApiError(value)
    }
}

fn s3_error_xml(code: &str, message: &str, resource: Option<&str>) -> String {
    let resource_tag = resource
        .map(|r| format!("<Resource>{}</Resource>", r))
        .unwrap_or_default();

    let request_id = uuid::Uuid::new_v4().to_string();
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<Error>
  <Code>{code}</Code>
  <Message>{message}</Message>
  {resource_tag}
  <RequestId>{request_id}</RequestId>
</Error>"#
    )
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let ApiError(err) = self;
        let (status, code, message, resource) = match err {
            DomainError::BucketAlreadyExists => (
                StatusCode::CONFLICT,
                "BucketAlreadyOwnedByYou",
                "Your previous request to create the named bucket succeeded and you already own it.",
                None,
            ),
            DomainError::BucketNotEmpty => (
                StatusCode::CONFLICT,
                "BucketNotEmpty",
                "The bucket you tried to delete is not empty.",
                None,
            ),
            DomainError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "InternalError",
                "We encountered an internal error. Please try again.",
                None,
            ),
            DomainError::NoSuchBucket(resource) => (
                StatusCode::NOT_FOUND,
                "NoSuchBucket",
                "The specified bucket does not exist.",
                Some(resource),
            ),
            DomainError::NoSuchKey(resource) => (
                StatusCode::NOT_FOUND,
                "NoSuchKey",
                "The specified key does not exist.",
                Some(resource),
            ),
            DomainError::InvalidBucketName => (
                StatusCode::BAD_REQUEST,
                "InvalidBucketName",
                "The specified bucket is not valid.",
                None,
            ),
            DomainError::AccessDenied => (
                StatusCode::FORBIDDEN,
                "AccessDenied",
                "Access denied.",
                None,
            ),
            DomainError::SignatureDoesNotMatch => (
                StatusCode::FORBIDDEN,
                "SignatureDoesNotMatch",
                "The request signature does not match the signature sent.",
                None,
            ),
        };

        let body = s3_error_xml(code, message, resource.as_deref());
        (status, [(CONTENT_TYPE, "application/xml")], body).into_response()
    }
}
