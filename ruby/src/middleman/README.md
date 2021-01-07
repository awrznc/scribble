middleman
===

## Build

```bash
# nodejsのインストール
# インストールしていないと `bundle exec middleman init sample_project` 実行時に以下のエラーが発生する
# ExecJS::RuntimeUnavailable: Could not find a JavaScript runtime. See https://github.com/rails/execjs for a list of available runtimes.
curl -sL https://deb.nodesource.com/setup_10.x | bash -
apt install -y nodejs

# bundle install
bundle install --path vendor/bundle

# init
bundle exec middleman init sample_project

# up server
cd sample_project
bundle install
bundle exec middleman server
# => http://localhost:4567/
```
