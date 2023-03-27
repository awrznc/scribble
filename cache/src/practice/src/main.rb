# ===== Memcached Sample ===== ===== =====

require 'memcache'

# memcachedサーバのIPとポートの指定
# ドメイン名にはdockerのサービス名を指定
memc = MemCache::new 'memcached:11211'

# 値をセット
memc["foo"] = "bar"

# 値の取得
memc["foo"]
# => "bar"
memc["ooo"]
# => nil


# ===== Redis Sample ===== ===== =====

require 'redis'

# redisサーバのIPとポートの指定
# ドメイン名にはdockerのサービス名を指定
redis = Redis.new(:url => "redis://redis:6379")

# 値をセット
redis.set "foo", "bar"

# 値の取得
redis.get "foo"
# => "bar"
redis.get "ooo"
# => nil
