# Be sure to restart your server when you modify this file.

# Version of your assets, change this if you want to expire all your assets.
Rails.application.config.assets.version = '1.0'

# Add additional assets to the asset load path.
# Rails.application.config.assets.paths << Emoji.images_path
# Add Yarn node_modules folder to the asset load path.
Rails.application.config.assets.paths << Rails.root.join('node_modules')

# Precompile additional assets.
# application.js, application.css, and all non-JS/CSS in the app/assets
# folder are already added.
# Rails.application.config.assets.precompile += %w( admin.js admin.css )

# /tmp 以下にキャッシュを作成するようにする
# Windowsのフォルダをマウントしている場合、そのフォルダ内では
# 大文字・小文字違いの同名のディレクトリが作成できない
Rails.application.config.assets.configure do |env|
  env.cache = Sprockets::Cache::FileStore.new(
    ENV.fetch("SPROCKETS_CACHE", "/tmp/cache/assets"),
    Rails.application.config.assets.cache_limit,
    env.logger
  )
end
