use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    BucketAlreadyExists,
    BucketNotEmpty,
    NotFound,
    Internal,
}

impl fmt::Display for DomainError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DomainError::BucketAlreadyExists => write!(f, "bucket already exists"),
            DomainError::BucketNotEmpty => write!(f, "bucket not empty"),
            DomainError::NotFound => write!(f, "not found"),
            DomainError::Internal => write!(f, "internal error"),
        }
    }
}

impl std::error::Error for DomainError {}

impl From<std::io::Error> for DomainError {
    fn from(_: std::io::Error) -> Self {
        DomainError::Internal
    }
}
