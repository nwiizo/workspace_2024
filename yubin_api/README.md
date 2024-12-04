# yubin_api

yubin_apiは、日本の郵便番号システムへのRESTful APIインターフェースを提供するサービスです。郵便番号による住所検索や、住所からの郵便番号検索機能を提供します。

## 機能

- 郵便番号による住所検索
- 住所文字列による郵便番号検索
- Prometheusメトリクス
- 構造化ログ出力
- OpenAPI(Swagger)ドキュメント


## 実行方法

```bash
cargo run
```

デフォルトで以下のポートで起動します：
- API: http://localhost:3000
- メトリクス: http://localhost:9000

## API例

### 郵便番号による検索
```bash
curl http://localhost:3000/postal/1000001
```

### 住所による検索
```bash
curl -X POST http://localhost:3000/address/search \
  -H "Content-Type: application/json" \
  -d '{"query": "東京都千代田区", "limit": 10}'
