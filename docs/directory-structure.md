# mini-s3 ディレクトリ構成（クリーンアーキテクチャ）

このプロジェクトは **依存の向きを内側（ドメイン）に向ける** 形でモジュールを分割しています。外側のレイヤー（HTTP・DB）が内側の型やルールに依存し、ドメインはフレームワークや SQL に依存しません。

## ツリー概要

```text
mini-s3/
├── Cargo.toml
├── migrations/                 # SQLx マイグレーション（スキーマ）
├── docs/
│   └── directory-structure.md  # 本ドキュメント
└── src/
    ├── main.rs                 # エントリ: 環境変数・DB 接続・マイグレーション・サーバ起動
    ├── lib.rs                  # クレート公開モジュールの宣言
    ├── domain/                 # ドメイン層
    ├── application/            # アプリケーション層（ユースケース・ポート）
    ├── infrastructure/         # インフラ層（DB 実装など）
    └── http/                   # インターフェース層（Axum ルート・ハンドラ）
```

## 各レイヤーの役割

| ディレクトリ      | 役割                                                                                                         |
| ----------------- | ------------------------------------------------------------------------------------------------------------ |
| `domain/`         | エンティティ・値オブジェクト・ドメインエラー。`axum` / `sqlx` に依存しない。                                 |
| `application/`    | ユースケース（`BucketService` 等）と **ポート**（`BucketRepository` などの `trait`）。実装の詳細は持たない。 |
| `infrastructure/` | ポートの **具体実装**（例: `SqliteBucketRepository`）、DB 接続・マイグレーション実行。                       |
| `http/`           | リクエストの解釈・レスポンス整形・ステータスコード。ユースケースを呼び出す薄いハンドラ。                     |

## `src/` 以下の詳細

```text
src/
├── lib.rs
├── main.rs
├── domain/
│   ├── mod.rs
│   ├── bucket.rs       # バケットのドメインモデル
│   └── error.rs        # DomainError
├── application/
│   ├── mod.rs
│   ├── ports.rs        # リポジトリ等の async trait（境界）
│   ├── buckets.rs      # バケット関連ユースケース
│   └── objects.rs      # オブジェクト関連（現状プレースホルダ）
├── infrastructure/
│   ├── mod.rs
│   ├── database.rs     # プール接続・migrate! の実行
│   └── persistence/
│       ├── mod.rs
│       └── sqlite_buckets.rs   # BucketRepository の SQLite 実装
└── http/
    ├── mod.rs
    ├── routes.rs       # Router の組み立て
    ├── state.rs        # AppState（サービス・リポジトリの合成）
    ├── error.rs        # DomainError → HTTP（ApiError / IntoResponse）
    └── handlers/
        ├── mod.rs
        ├── buckets.rs  # バケット API
        └── objects.rs  # オブジェクト API（スタブ）
```

## 依存関係のルール

- `domain` → 他の自前モジュールに依存しない。
- `application` → `domain` と、自前の `ports`（trait）のみ。
- `infrastructure` → `application::ports` を実装し、`domain` の型を使う。`sqlx` はここに閉じる。
- `http` → `application` と `domain`（エラー写像）を使う。ハンドラから直接 `sqlx` を呼ばないのが望ましい。

## 合成（Composition Root）

`main.rs` と `http/state.rs` が近い役割を持ちます。

- `main.rs`: 設定の読み込み、DB プール作成、マイグレーション、ルータへ渡す `AppState` の生成。
- `http/state.rs`: リポジトリ実装とユースケースの `Arc` を束ね、ハンドラに `State` で注入する。

テストで差し替えたい場合は、`AppState::new` を別コンストラクタにする、または `BucketRepository` をモック実装して `BucketService::new` に渡す、といった拡張がしやすくなります。

## 環境変数（参考）

| 変数                       | 説明                                                     |
| -------------------------- | -------------------------------------------------------- |
| `DATABASE_URL`             | 未設定時は `sqlite:database.db`                          |
| `DEFAULT_OWNER_ACCESS_KEY` | バケット作成時の `owner_access_key`（既定: `local-dev`） |

## 今後の拡張の指針

- **オブジェクトの永続化**: `application::ports` に `ObjectRepository` とファイルストレージ用の trait を追加し、`infrastructure` に SQLite / ローカルファイルの実装を置く。
- **認証**: `http` で署名やヘッダを検証し、検証結果だけ（アカウント ID 等）をユースケースに渡す。
- **クレート分割**: 規模が大きくなったら `mini-s3-domain` / `mini-s3-application` のようにワークスペース化する選択肢がある。
