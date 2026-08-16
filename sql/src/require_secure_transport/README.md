require_secure_transport
===

サーバ証明書の確認とSSL接続をやってみる。

ちなみに前者はあくまで確認だけしかしないのでクライアントの認証を行っているわけではないらしい ( なので必須ではない )


## 作業しながら確認してみる

Docker を立ち上げて接続。

```bash
docker compose up -d
docker compose logs migrator
docker compose exec sql_client bash

# mariadbのコマンドなので注意
mysql -h mysql_server_8.4.11_auto_generated_server_certificate -P 3306 -u root -pmysql_root_pass --ssl-ca=/opts/mysql/certs/ca.pem
```

立ち上げる時に cnf ファイルを読み込ませるか、立ち上げ後に動的設定をすると有効化できる。

```sql
-- デフォルトはOFF
SHOW VARIABLES LIKE 'require_secure_transport';
-- =>
-- +--------------------------+----------------+
-- | VARIABLE_NAME            | VARIABLE_VALUE |
-- +--------------------------+----------------+
-- | require_secure_transport | ON             |
-- +--------------------------+----------------+

-- 動的設定とその確認
-- SET GLOBAL require_secure_transport = ON;
-- SET PERSIST require_secure_transport = ON;
-- SELECT * FROM performance_schema.persisted_variables;
```

```bash
docker compose down --volumes
```

## 参考

* https://dev.mysql.com/doc/refman/8.4/en/creating-ssl-rsa-files-using-mysql.html
