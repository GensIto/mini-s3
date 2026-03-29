-- Add up migration script here
CREATE TABLE credentials (
    access_key_id TEXT PRIMARY KEY,
    secret_access_key TEXT NOT NULL,
    account_name TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE buckets (
    bucket_id TEXT PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    owner_access_key TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE objects (
    object_id TEXT PRIMARY KEY,
    bucket_id TEXT NOT NULL REFERENCES buckets (bucket_id),
    key TEXT NOT NULL,
    size INTEGER NOT NULL,
    content_type TEXT NOT NULL,
    etag TEXT NOT NULL,
    storage_path TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE UNIQUE INDEX idx_objects_bucket_id_key ON objects (bucket_id, key);