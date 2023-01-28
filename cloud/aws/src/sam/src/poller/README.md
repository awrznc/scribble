Poller
===

特定のサブネットから接続できるSamba内の特定のファイルを監視します。
ファイルに更新があった場合はS3にアップロードします。

つまるところ一方通行の同期システムです。

## 環境構築

```bash
# テンプレート用のファイルをコピー
$ cp ./samconfig.template.toml ./samconfig.toml

# テンプレート用のファイルをよしなに書き換え
$ vi ./samconfig.template.toml
```

## Run

```bash
$ make local_run
```

## Deploy

```bash
# VPC, Subnet, InternetGateway, RouteTable, VPC Endpoint を作成してそれぞれを紐づけておく

# VPC LambdaはパブリックIPを標準では持っていないため、S3の操作(例: ListObjectsV2)などをSDKを用いて行う場合はタイムアウトしてしまう
# 従って、 S3 の操作が可能な VPC Endpoint を作成しておく必要がある

# 初回
$ sam deploy --guided

# 次から
$ make deploy
```

## Delete

```bash
$ make delete
```
