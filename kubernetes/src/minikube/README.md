minikube
===

Local Kubernetes.


## Document

https://minikube.sigs.k8s.io/docs/


## 環境構築

WSL2 を利用して構築する例。

```powershell
# Debian イメージの取得
mkdir .tmp
cd .tmp
$ProgressPreference = 'SilentlyContinue'
Invoke-WebRequest -Uri https://aka.ms/wsl-debian-gnulinux -OutFile debian.zip -UseBasicParsing
Expand-Archive -Path debian.zip -DestinationPath debian\
Rename-Item debian\DistroLauncher-Appx_1.12.2.0_x64.appx DistroLauncher.zip
Expand-Archive -Path debian\DistroLauncher.zip -DestinationPath launcher\
cd ..

# import
$DistroName = 'minicube-chan'
mkdir $DistroName
wsl --import $DistroName $DistroName\ .tmp\launcher\install.tar.gz

# check
wsl -l -v
wsl -d minicube-chan cat /etc/debian_version
# => 11.3

# Debian 上でのコマンド
wsl -d minicube-chan ./init.sh
wsl -d minicube-chan sudo service docker start
wsl -d minicube-chan su - minicube-chan
# 移動

# 削除
wsl --shutdown
wsl --unregister minicube-chan
```


## コマンド

以下の概念に沿ったメモ。

概念 | 説明
--- | ---
Cluster | k8sの実行環境。
Deployment | k8sで管理するアプリケーション。
Pod | コンテナやストレージリソースなどをカプセル化したもの。
Service | Podを公開するためのもの。


```bash
# コマンドの実行前にDockerは起動しておく
sudo service docker start

# イメージも用意しておく
docker pull nginx:1.23.3

su - minicube-chan
```


### Cluster

```bash
# 開始
minikube start

# 状態の確認
minikube status

# 削除
minikube delete
```


### Deployment

```bash
# 作成
minikube kubectl -- create deployment piyo-app --image=nginx:1.23.3

# 状態の確認
minikube kubectl -- get deployments

# Podの確認
minikube kubectl -- get pods

# Pod内のコンテナの確認
minikube kubectl -- describe pods/piyo-app-xxxxxxxxxx-xxxxx

# Podをスケールする
minikube kubectl -- scale deployments/piyo-app --replicas=4

# 削除（サービス含め全て）
minikube kubectl -- delete --all deployment,pod,svc
```


### Service

```bash
# 外部からクラスターにアクセスできるようにする
minikube kubectl -- expose deployment/piyo-app --type=NodePort --port 80

# ポートフォワーディング
minikube kubectl -- port-forward service/piyo-app --address=0.0.0.0 8080:80

# 状態の確認
minikube kubectl get services
```


### With file

```bash
# 設定ファイルを利用して作成（サービス含め全て）
minikube kubectl -- create -f ./src/practice/deployment.yaml

# ローリングアップデート
minikube kubectl -- apply -f ./src/practice/deployment.yaml

# ポートフォワーディング
minikube kubectl -- port-forward service/piyo-app --address=0.0.0.0 8080:8080

# 削除（サービス含め全て）
minikube kubectl -- delete --all deployment,pod,svc
```
