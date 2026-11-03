FROM mysql:8.4.11

# 権限を変えないと設定ファイルを読み込んでくれない
# mysqld: [Warning] World-writable config file '/etc/mysql/conf.d/my.cnf' is ignored.
COPY ./my.cnf /etc/mysql/conf.d/my.cnf
RUN chmod 644 /etc/mysql/conf.d/my.cnf
