//! S3 単一パートオブジェクトの ETag は、オブジェクト本体の MD5（16 進小文字）とされる。

pub fn s3_etag_hex(content: &[u8]) -> String {
    let digest = md5::compute(content);
    digest
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}
