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

fn s3_error_xml(code: &str, message: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<Error>
  <Code>{code}</Code>
  <Message>{message}</Message>
</Error>"#
    )
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let ApiError(err) = self;
        let (status, code, message) = match err {
            DomainError::BucketAlreadyExists => (
                StatusCode::CONFLICT,
                "BucketAlreadyOwnedByYou",
                "Your previous request to create the named bucket succeeded and you already own it.",
            ),
            DomainError::BucketNotEmpty => (
                StatusCode::CONFLICT,
                "BucketNotEmpty",
                "The bucket you tried to delete is not empty.",
            ),
            DomainError::NotFound => (
                StatusCode::NOT_FOUND,
                "NoSuchBucket",
                "The specified bucket does not exist.",
            ),
            DomainError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "InternalError",
                "We encountered an internal error. Please try again.",
            ),
        };

        let body = s3_error_xml(code, message);
        (status, [(CONTENT_TYPE, "application/xml")], body).into_response()
    }
}
