Open Match
===

ゲームのためのマッチメイキングシステム。

https://open-match.dev/site/


## Demo

実行環境が構築できているか確認。
ブラウザでうまく動いてそうならOK。

* https://open-match.dev/site/docs/getting-started/
* https://open-match.dev/site/docs/installation/yaml/

```bash
# Open Match Core のインストール
kubectl apply --namespace open-match \
  -f https://open-match.dev/install/v1.8.0/yaml/01-open-match-core.yaml

# Default Evaluator のインストール
# MEMO: マッチングの評価者
kubectl apply --namespace open-match \
  -f https://open-match.dev/install/v1.8.0/yaml/06-open-match-override-configmap.yaml \
  -f https://open-match.dev/install/v1.8.0/yaml/07-open-match-default-evaluator.yaml

# デモのデプロイ
kubectl create namespace open-match-demo
kubectl apply --namespace open-match-demo \
  -f https://open-match.dev/install/v1.8.0/yaml/02-open-match-demo.yaml

# ポートフォワーディング
kubectl port-forward --namespace open-match-demo service/om-demo 51507:51507
# => http://localhost:51507/

# 削除
kubectl delete namespace open-match
kubectl delete namespace open-match-demo
```
