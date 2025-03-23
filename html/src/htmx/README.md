htmx
===

属性を利用して非同期通信するためのライブラリ。

https://htmx.org/


## Run

```bash
docker run --rm -p 8080:80 --mount type=bind,source="$(pwd)/src/",target=/usr/local/apache2/htdocs/ httpd:2.4
# => http://localhost:8080/hello-world/index.html
```
