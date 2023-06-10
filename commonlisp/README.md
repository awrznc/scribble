CommonLisp
===

## 環境構築

```bash
docker build -t local_cl:latest .
docker run --rm -it -v "$(pwd):/opt" local_cl:latest bash
```


## 実行

```
sbcl --script ./src/hello-world/main.lisp
```
