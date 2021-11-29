SQL
===

Structured Query Language.

## 環境構築

```bash
$ docker compose build
$ docker compose up -d
```


## 接続

```bash
# コンテナに接続
$ docker compose exec sql_client bash

# PostgreSQL
$ psql -h postgresql_server -U postgres_user
# => パスワードを入力して接続完了

# MySQL
$ mysql -h mysql_server -u root -pmysql_root_pass
# => -pオプションでパスワード未入力でも接続可能
```

アクション | PostgreSQL | MySQL
--- | --- | ---
起動 | `postgres -D /usr/local/var/postgres` | `mysql.server start`
DB一覧 | `\list` | `show databases;`
DB移動 | `\connect [DB名]` | `use [DB名];`
テーブル一覧表示 | `\d` | `show tables;`
テーブル内容表示 | `\d [テーブル名]` | `show columns from [テーブル名];`
退出 | `\quit` | `exit`
停止 | `Ctrl + c` | `mysql.server stop`


## 実行

```bash
# PostgreSQL ( PGPASSWORD=[password] psql -h [hostname] -U [username]  -d [databasename] -f [sourcefile])
$ PGPASSWORD=postgres_pass psql -h postgresql_server -U postgres_user -d postgres -f ./src/hello_world/main.sql

# MySQL ( mysql -h [hostname] -u [username] -p[password] [databasename] < [sourcefile] )
$ mysql -h mysql_server -u root -pmysql_root_pass mysql < ./src/hello_world/main.sql
```
