

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
