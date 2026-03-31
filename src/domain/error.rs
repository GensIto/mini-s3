use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    BucketAlreadyExists,
    BucketNotEmpty,
    Internal,
    NoSuchBucket(String),
    NoSuchKey(String),
    InvalidBucketName,
    AccessDenied,
    SignatureDoesNotMatch,
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::BucketAlreadyExists => write!(f, "bucket already exists"),
            DomainError::BucketNotEmpty => write!(f, "bucket not empty"),
            DomainError::Internal => write!(f, "internal error"),
            DomainError::NoSuchBucket(bucket_name) => write!(f, "no such bucket: {}", bucket_name),
            DomainError::NoSuchKey(key) => write!(f, "no such key: {}", key),
            DomainError::InvalidBucketName => write!(f, "invalid bucket name"),
            DomainError::AccessDenied => write!(f, "access denied"),
            DomainError::SignatureDoesNotMatch => write!(f, "signature does not match"),
        }
    }
}

impl std::error::Error for DomainError {}

impl From<std::io::Error> for DomainError {
    fn from(_: std::io::Error) -> Self {
        DomainError::Internal
    }
}
