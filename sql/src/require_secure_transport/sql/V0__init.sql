-- SSL接続できているか確認
SHOW VARIABLES LIKE 'require_secure_transport';
SHOW SESSION STATUS LIKE 'Ssl_version';
SHOW SESSION STATUS LIKE 'Ssl_cipher';

-- 接続時にPEMを必須化
-- ALTER USER 'root' REQUIRE X509;
-- ALTER USER 'root' REQUIRE SSL;

-- 認証プラグインの確認
SELECT user, host, plugin FROM mysql.user;
