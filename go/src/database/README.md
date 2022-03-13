CURD
===

Database操作のラッパーです。


## 環境構築

```bash
apt install -y postgresql postgresql-client
/etc/init.d/postgresql restart
su - postgres
psql
```

```sql
CREATE USER example_user PASSWORD 'example_password';
exit
```

```bash
exit
PGPASSWORD=example_password psql -h localhost -d example_db -U example_user;
```

```sql
CREATE DATABASE example_db;
CREATE TABLE example_table(
    id          serial primary key,
    label       text default '',
    x           float default 0.0,
    y           float default 0.0,
    create_at   timestamp default CURRENT_TIMESTAMP,
    update_at   timestamp default CURRENT_TIMESTAMP
);
INSERT INTO example_table(label, x, y) VALUES('sample point', 10.0, 12.0);
INSERT INTO example_table(label, x, y) VALUES('sample point2', 109.1, 120.2);
```

## Run

```bash
go get github.com/lib/pq
go run ./example/main.go
```
