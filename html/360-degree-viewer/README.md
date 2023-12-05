360-degree-viewer
===

全天球画像をHTMLで見るためのコードです。

機器の傾き等にも対応しています。

全天球画像は `tmp.jpg` として `index.html` と同じディレクトリに配置してください。

```bash
docker run --rm -p '8080:80' -v "$(pwd):/usr/local/apache2/htdocs/" httpd:2.4
# => http://localhost:8080/
```
