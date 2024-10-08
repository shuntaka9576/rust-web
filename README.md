
## アプリ関連

```bash
cargo make before-build
cargo make test
```

## コンテナ起動

```bash
# アプリだけの起動
docker compose up app -d --build

# 全て起動
docker compose up -d --build
```

## ミドルウェア操作

```bash
# install redis
brew install redis

# install libpq
brew install libpq

# psqlはパス設定も必要
```

```bash
redis-cli -h localhost -p 6379
```

```bash
psql "postgresql://localhost:5432/app?user=app&password=passwd"
```

## モジュール追加

```bash
cargo add axum@0.7.5 --features macros
cargo add tokio@1.37.0 --features full
cargo add --dev rstest@0.18.2
cargo add cargo-nextest
cargo add sqlx@0.7.3 --features \
runtime-tokio,\
uuid,\
chrono,\
macros,\
postgres,\
migrate
```

## DB関連

マイグレーションファイルの用意(初回のみ)
```bash
sqlx migrate add -r start --source adapter/migrations
```


```bash
cargo make migrate
```

## テスト

```bash
curl -v -X POST "http://localhost:8080/books" \
  -H 'content-type: application/json' \
  -d '{"title": "t", "author": "a", "isbn": "i", "description": "d"}'
```

```bash
curl -v http://localhost:8080/books | jq .
```
