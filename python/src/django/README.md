Django
===

Webアプリケーションフレームワーク。

## 環境構築

```bash
docker run --rm -it -v "$(pwd):/root/workspace" -p '8000:8000' almalinux:9 bash
dnf install -y python3 python3-pip
pip install -r requirements.txt

# init
django-admin startproject <project-name>
```


## Run

```bash
python3 src/piyo/manage.py runserver 0.0.0.0:8000
# => http://localhost:8000/hello/
```


## SECRET_KEY

Django にはインストールごとに設定される個別の秘密鍵が存在する。

https://docs.djangoproject.com/ja/4.1/ref/settings/#std-setting-SECRET_KEY

これは Django で行われる匿名セッション等で利用される。

匿名セッションを利用することで、任意のデータをサイトの訪問者ごとに保存し、取得することができるようになる。ただし、データはサーバー側に保存され、クッキーの送受信によって抽象化されるため、この秘密鍵が流出することはリスクである。

秘密鍵自体は、 `django-admin startproject <project-name>` の実行により生成される、 `settings.py` に記載されている。

ちなみに、この鍵が記載されている状態で GitHub に push するとセキュリティアラートが上がり、メールが飛んでくる。

キーを再生成したい場合は以下のスクリプトで再生成できる。

```bash
python3 src/generate_secret_key/main.py
```
