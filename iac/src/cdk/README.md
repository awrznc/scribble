AWS CDK
===

https://aws.amazon.com/jp/cdk/

## 環境構築

```
npm install
```

## init

```bash
# ディレクトリを作成
mkdir piyo && cd piyo

# カレントディレクトリの下に展開されるので注意
npx cdk init --language=typescript
```

## deploy

```bash
# Build
npm run build

# 定義情報の確認
npx cdk synth

# CDKが利用するリソース置き場の作成(最初の一回)
npx cdk bootstrap

# Deploy
npx cdk deploy
# CloudFormationにAbcStackのような名前でスタックが作成される

# 削除
npx cdk destroy

# boostrapの削除
aws s3 rm --recursive s3://<boostrapのバケット名>
aws cloudformation delete-stack --stack-name CDKToolkit
```
