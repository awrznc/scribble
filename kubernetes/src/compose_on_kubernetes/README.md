# Compose on Kubernetes

Compose on Kubernetes なるものがあるらしいので使ってみる。

## Run

```bash

# デプロイ
docker stack deploy --compose-file docker-compose.yml mystack

# 削除
docker stack rm mystack

```

## 所感

* Compose on Kubernetes は手軽でいい
   * volumeを絶対パスで書く必要があるのはちょっと使いにくい（回避策はあるけど開発が面倒くさくなる）
   * docker-compose.yml の command が効かなくなるので注意
