## マクロとは

data/test-bucket は存在していますね。ということは DB側のクエリがエラーの可能性が高いです。

問題は sqlite_buckets.rs:104 の sqlx::query! マクロです。query!（マクロ版）はコンパイル時にSQL検証しますが、実行時に期待と異なる挙動になることがあります。また、DELETE 文は行が見つからなくても RowNotFound エラーにはならず正常に rows_affected = 0 を返すので、エラーハンドリングの分岐も意味がありません。

ログにSQLiteの具体的なエラーが出ていないので、inspect_err を追加して原因を特定するか、sqlx::query! → sqlx::query に変えてみましょう。query! マクロは DATABASE_URL 環境変数の設定次第で実行時に問題を起こすことがあります。

sqlx::query（非マクロ版）に統一するのが確実です:
