Scripting with Lua
===

RedisにはLuaスクリプトを実行することができる機能が存在する。
スクリプトはサーバ上で実行されるため、読み書きがとても効率的らしい。

https://redis.io/docs/manual/programmability/eval-intro/

## 環境構築

```
docker run --rm -p "6379:6379" redis:7.0.10
```
