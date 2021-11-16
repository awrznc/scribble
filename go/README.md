Go
===

ビルドの待ち時間が少ないので開発効率がよい

## 環境構築

```bash
docker build -t local_golang:latest .
docker run --rm -it -v "$(pwd):/opt" local_golang:latest bash
```

## 新規プロジェクト作成

```bash
mkdir {{Project Name}} && cd {{Project Name}}
goenv local {{Version}}
go mod init {{Project Name}}
touch main.go
# 編集
go run .
```
