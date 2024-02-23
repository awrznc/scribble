Terraform
===

クラウド及びオンプレミスの環境をコードで管理することができるツール。

## 環境構築

```bash
bash ./init.bash
```

## Run

```bash
# init
./bin/terraform -chdir=./src/hello_world/ init

# 事前確認
./bin/terraform -chdir=./src/hello_world/ plan

# 構築
./bin/terraform -chdir=./src/hello_world/ apply -auto-approve
```
