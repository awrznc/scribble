SET client_encoding = 'UTF8';

CREATE TABLE sample (
  -- id SERIAL PRIMARY KEY,
  id integer,
  value TEXT
);

-- samples の後に括弧を付けてカラム名を指定することもできるらしい
-- 別のサーバから参照する場合は \COPY メタコマンドを使うこと
COPY sample FROM '/docker-entrypoint-initdb.d/sample.csv' WITH CSV;
