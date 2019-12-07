# PHP

## Laravel

学習コストが低いMVCフレームワーク

```bash

# プロジェクトの作成
$ composer create-project laravel/laravel {プロジェクト名} --prefer-dist

# プロジェクトのディレクトリに移動
$ cd {プロジェクト名}

# コントローラーの作成 
$ php artisan make:controller SampleController

# 簡易サーバの立ち上げ
$ php artisan serve -host 0.0.0.0

```

laravelのORMは効率が悪いらしい

下のように直接した方が効率がよいらしい
https://readouble.com/laravel/5.8/ja/queries.html
