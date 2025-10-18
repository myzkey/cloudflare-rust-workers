# Cloudflare Workers with Rust

RustとWebAssemblyで構築されたCloudflare Workersサンプルプロジェクトです。

## 機能

- **REST APIエンドポイント**: JSONレスポンスを持つ複数のAPIルート
- **非同期ルート処理**: `get_async`, `post_async`, `delete_async`を使用
- **JSONシリアライゼーション**: serdeによる型安全なJSON処理
- **エラーハンドリング**: 適切なHTTPステータスコードとエラーレスポンス
- **TypeScriptライクな開発体験**: 優れたツールチェーンによる型安全なRust

## APIエンドポイント

- `GET /foo` - GETリクエストのサンプル
- `POST /bar` - POSTリクエストのサンプル
- `DELETE /baz` - DELETEリクエストのサンプル

## 前提条件

- **Rust nightly**: edition2024サポートのため

```bash
rustup install nightly
rustup default nightly
```

- **WebAssemblyターゲット**:

```bash
rustup target add wasm32-unknown-unknown
```

- **Wrangler CLI**: Cloudflareの開発者プラットフォームCLI

```bash
npm install -g wrangler@latest
```

- **worker-build**: 自動でインストールされます（初回`wrangler dev`時）

## 開発

### ローカル開発

```bash
wrangler dev
```

初回は時間がかかります（worker-buildのインストールとビルド）。

### 手動ビルド（デバッグ用）

```bash
cargo check --target wasm32-unknown-unknown
```

### Cloudflareへのデプロイ

```bash
wrangler deploy
```

## プロジェクト構造

```txt
├── Cargo.toml          # Rustプロジェクト設定
├── wrangler.toml       # Cloudflare Workers設定
├── src/
│   └── lib.rs          # APIルートを含むメインWorkerコード
└── README.md           # このファイル
```

## 設定

### wrangler.toml

- `name`フィールドを希望するWorker名に更新
- 必要に応じてKVネームスペースを設定
- `[vars]`セクションに環境変数を追加
- `wrangler secret put <NAME>`を使用してシークレットを追加

### 環境変数

`wrangler.toml`に環境変数を追加：

```toml
[vars]
MY_VARIABLE = "my-value"
```

### シークレット

Wrangler CLIを使用して機密データを追加：

```bash
wrangler secret put SECRET_KEY
```

## 依存関係

- **worker**: 0.6系 - Rust用Cloudflare Workersランタイム
- **serde**: JSON処理用シリアライゼーションフレームワーク

## 注意事項

- **Rust nightly**: edition2024サポートのため必須
- **初回起動**: worker-buildのインストールで時間がかかります
- **非同期ハンドラ**: `*_async`メソッドを使用
- **ホットリロード**: 一度起動すれば、コード変更時は高速

## トラブルシューティング

### edition2024エラー

```bash
rustup default nightly
rustup target add wasm32-unknown-unknown
```

### worker-buildインストール失敗

Rust nightlyを使用していることを確認してください。

### デプロイの問題

```bash
wrangler auth login
wrangler deploy
```

## パフォーマンス

- **初回ビルド**: 5-10分（worker-buildインストール含む）
- **増分ビルド**: 10-30秒
- **ホットリロード**: 1-3秒
