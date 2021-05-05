## CGI

```bash
dnf install -y httpd

cp ./cgi/hello_world/main.cgi /var/www/html/
cp ./cgi/httpd.conf /etc/httpd/conf/httpd.conf

exec httpd -DFOREGROUND "$@"

curl http://localhost:8080/main.cgi
```
