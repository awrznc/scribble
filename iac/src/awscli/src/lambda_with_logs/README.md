Lambda with Logs
===

`Lambda` と `CloudWatch Logs` を作成するサンプル。


## 操作

```bash
TARGET_LAMBDA_NAME="LambdaWithLogsExample"
STACK_NAME="${TARGET_LAMBDA_NAME}Stack"

# デプロイ
aws cloudformation deploy \
    --template-file "$(pwd)/template.yaml" \
    --stack-name ${STACK_NAME} \
    --capabilities CAPABILITY_NAMED_IAM \
    --parameter-overrides LambdaName="${TARGET_LAMBDA_NAME}" LambdaBody="$(<$(pwd)/src/handler.py)"

# 状態の確認
aws cloudformation describe-stacks --stack-name ${STACK_NAME}

# テスト実行
aws lambda invoke --function-name ${TARGET_LAMBDA_NAME} /dev/stdout

# 削除
aws cloudformation delete-stack --stack-name ${STACK_NAME}
```


## 作成したログを閲覧する方法

ログは `event` と呼ばれる単位で `stream` と呼ばれるグループに格納されている。
従って、ログを閲覧したい場合は `stream` -> `event` の順番で参照する必要がある。

```bash
# Lambdaの名前
$ TARGET_LAMBDA_NAME="LambdaWithLogsExample"

# stream情報の取得（最新順）
$ aws logs describe-log-streams \
    --log-group-name "/aws/lambda/${TARGET_LAMBDA_NAME}" \
    --order-by LastEventTime \
    --descending

# stream情報を解析して名前を取得する例
$ STREAM_NAME=$(aws logs describe-log-streams \
    --log-group-name "/aws/lambda/${TARGET_LAMBDA_NAME}" \
    | jq -r '.logStreams[0].logStreamName')

# event情報の取得
$ aws logs get-log-events \
    --log-group-name "/aws/lambda/${TARGET_LAMBDA_NAME}" \
    --log-stream-name "${STREAM_NAME}" \
    --query "events[].[message]" \
    --output text

# 以下のような情報が表示される

START RequestId: aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa Version: $LATEST

Example message.

END RequestId: aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa

REPORT RequestId: aaaaaaaa-aaaa-aaaa-aaaa-aaaaaaaaaaaa  Duration: 1.17 ms       Billed Duration: 2 ms   Memory Size: 128 MB     Max Memory Used: 35 MB  Init Duration: 112.55 ms
```
