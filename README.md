# mini-s3

S3互換のオブジェクトストレージサーバー (Rust + axum + SQLite)

## セットアップ

```bash
cp .env.example .env

# DB作成 & マイグレーション
sqlx database create --database-url "sqlite:./database.db"
sqlx migrate run --database-url sqlite:./database.db

# サーバー起動
cargo run
# -> listening on http://127.0.0.1:8080
```

## AWS CLIでの操作

[install](https://docs.aws.amazon.com/ja_jp/cli/latest/userguide/getting-started-install.html#getting-started-install-instructions)

ダミーのクレデンシャルを設定:

```bash
aws configure

AWS Access Key ID: test
AWS Secret Access Key: test
Default region name: us-east-1
Default output format: json
```

`--endpoint-url` でローカルサーバーに向ける。
`s3api` を使うとAPI単位で1リクエストずつ叩けるのでデバッグしやすい:

```bash
# 1. バケット作成 (PUT /test-bucket)
aws --endpoint-url http://127.0.0.1:8080 s3api create-bucket --bucket test-bucket

# 2. バケット一覧取得 (GET /)
aws --endpoint-url http://127.0.0.1:8080 s3api list-buckets

# 3. テストファイルを作ってPutObject (PUT /test-bucket/hello.txt)
echo "Hello, S3!" > /tmp/hello.txt
aws --endpoint-url http://127.0.0.1:8080 s3api put-object --bucket test-bucket --key hello.txt --body /tmp/hello.txt --content-type text/plain

# 4. バケット削除 (DELETE /test-bucket)
aws --endpoint-url http://127.0.0.1:8080 s3api delete-bucket --bucket test-bucket
```

## DBテーブル構成

```
sqlite> .tables
_sqlx_migrations  buckets  credentials  objects
```

| テーブル    | 用途                   |
| ----------- | ---------------------- |
| buckets     | バケット管理           |
| objects     | オブジェクトメタデータ |
| credentials | アクセスキー管理       |

## 参考

- https://ivov.dev/notes/s3-object-storage
- https://blog.bytebytego.com/p/design-a-s3-like-storage-system
- https://zenn.dev/magurotuna/books/tokio-tutorial-ja/viewer/io (tokio)
- https://zenn.dev/kengoku123/articles/rust-lesson-using-sqlx (sqlx)
