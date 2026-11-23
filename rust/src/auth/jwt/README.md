jwt
===

Rustでjsonwebtokenの署名を作成・検証する例です。

公開鍵暗号方式で実装しているため、秘密鍵で署名の作成、公開鍵で署名の検証を行います。


## 準備

```shell
openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:2048 -out ./private.pem
openssl rsa -in private.pem -pubout -out ./public.pem
```
