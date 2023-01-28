需要がありそうでないAWSワンライナー集
===

きっと役に立つときもあるだろう。

## 1年以上更新されていないCloudWatch LogsのグループをCSVで取得する

Amazon CloudWatch Logsの更新されていないグループを把握するためのワンライナー。

以下のコマンドに依存。

* `aws`
* `jq`
* `date` (GNU)

```bash
aws logs describe-log-groups \
    | jq -r '.logGroups[].logGroupName' \
    | while read log_name; do {
        aws logs describe-log-streams \
            --log-group-name ${log_name} \
            --order-by LastEventTime \
            --descending \
            --max-items 1 \
            | jq \
                --arg threshold $(($(date +%s%N -d '1 year ago') / 1000000)) \
                --arg log_name ${log_name} \
                -r '.logStreams[0] | select(.lastEventTimestamp != null and .lastEventTimestamp < ($threshold | tonumber)) | [ $log_name, (.lastEventTimestamp | ./1000 | todate) ] | @csv'
    }; done > aws-cloudwatch-logs-groups-$(TZ=UTC date '+%Y-%m-%dT%TZ').csv
```


## 最新バージョンのRuntimeがnodejs12.xのLambdaのarnリストを取得する

非推奨になるランタイムを利用したLambdaを洗い出すときなどに利用できるかも。

以下のコマンドに依存。

* `aws`
* `jq`
* `date` (GNU)

```bash
for v in $(aws ec2 describe-regions | jq -r '.Regions[].RegionName') ; do {
  aws lambda list-functions \
    --function-version ALL \
    --region $v \
    --output json \
    --query "Functions[?Runtime=='nodejs12.x'].FunctionArn" \
  | jq -r '.[]' | grep ':$LATEST' | sort
}; done > aws-lambda-nodejs.12.x-$(TZ=UTC date '+%Y-%m-%dT%TZ').csv
```

## 特定のタグ名及び値が紐づいているEC2インスタンスの名前一覧を取得する（ユニーク）

```bash
TAG_ID="Color"
TAG_VALUE="Red"
aws ec2 describe-instances --filter "Name=tag:${TAG_ID},Values=${TAG_VALUE}" | jq -r '.Reservations[].Instances[].Tags[] | select(.Key == "Name") | .Value' | sort | uniq
```

## 特定のバケットに最後にアップロードされたS3オブジェクトを取得する

```bash
S3_BUCKET="piyo-piyo"
PREFIX="piyo-"
aws s3api list-objects --bucket ${S3_BUCKET} --prefix ${PREFIX} --query 'sort_by(Contents[],&LastModified)|[-1]'
```

## タグ付可能かつタグが紐づいているリソースのARNを部分一致で検索する

```bash
aws resourcegroupstaggingapi get-resources --query 'ResourceTagMappingList[?contains(ResourceARN,`piyo`)][].ResourceARN'
```
