
## 注意点

### sqlxのv2マクロを使うと、`cargo sqlx prepare`か`DATABASE_URL`の設定が必要

```bash
DATABASE_URL = "postgresql://localhost:5432/app?user=app&password=passwd"
nvim
```


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


ログイン(トークンの取得)
```bash
export TOKEN=$(curl -s -X POST "http://localhost:8080/auth/login" \
  -H 'content-type: application/json' \
  -d '{"email": "eleazar.fig@example.com", "password": "Pa55w0rd"}' | jq -r ".accessToken")
echo $TOKEN
```

ユーザー一覧の取得

```bash
curl -v -X GET "http://localhost:8080/api/v1/users" \
  -H "Authorization: Bearer $TOKEN"
```

ログアウト
```bash
curl -v -X POST "http://localhost:8080/auth/logout" \
  -H "Authorization: Bearer $TOKEN"
```

本の登録

```bash
curl -v -X POST "http://localhost:8080/api/v1/books" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"title": "Rust book", "author":"me", "isbn":"1234567890", "description":""}'
```

蔵書の確認

```bash
export BOOK_ID="$(curl -s -X GET "http://localhost:8080/api/v1/books" \
  -H "Authorization: Bearer $TOKEN" | jq -r '.items[0].id')"
echo $BOOK_ID

curl -s -X GET "http://localhost:8080/api/v1/books/$BOOK_ID" \
  -H "Authorization: Bearer $TOKEN" | jq
```

貸出の実行

```bash
curl -X POST "http://localhost:8080/api/v1/books/$BOOK_ID/checkouts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json"
```

貸出一覧
```bash
curl -X GET "http://localhost:8080/api/v1/books/checkouts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json"

export CHECKOUT_ID=$(curl -s -X GET "http://localhost:8080/api/v1/books/checkouts" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" | jq -r ".items[0].id")
echo $CHECKOUT_ID
```

返却
```bash
curl -v -X PUT "http://localhost:8080/api/v1/books/$BOOK_ID/checkouts/$CHECKOUT_ID/returned" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json"
```

```bash
curl -v "http://localhost:8080/api/v1/books" \
  -H "Authorization: Bearer $TOKEN"
```
