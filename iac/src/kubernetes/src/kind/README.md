kind
===

Kubernetes in Docker.

その名の通りDockerコンテナ上にk8sクラスタを構築してくれるk8s公式のツール。<br>
クラスタの作成と破棄が割と高速らしい。


## Document

https://kind.sigs.k8s.io/


## Init

```powershell
# 構築
.\init.bat

# 作成した環境へのログインとログアウト
wsl -d kind
sudo service docker start
su - kinduser
exit

# 削除
wsl --shutdown
wsl --unregister kind
rm .\.tmp
rm .\kind
```


### Cluster

```bash
# 作成
kind create cluster --config=./kind.yaml

# docker pull kindest/node:v1.32.0
# kind create cluster --image kindest/node:v1.32.0

# 開始 ( コンテナを残す。調査用 )
# kind create cluster --image kindest/node:v1.32.0 --retain

# 削除
kind delete cluster
```


### Deployment

詳しくはminikubeを参照 ( kubectl部分は同じなので )
