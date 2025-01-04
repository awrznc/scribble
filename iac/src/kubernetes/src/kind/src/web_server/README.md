Web Server
===

`Hello, kind!` を返すオブジェクトを構築します。


## Init

```bash
# 作成
kubectl apply -f https://kind.sigs.k8s.io/examples/ingress/deploy-ingress-nginx.yaml
kubectl wait --namespace ingress-nginx \
  --for=condition=ready pod \
  --selector=app.kubernetes.io/component=controller \
  --timeout=90s
kubectl apply -f ./src/web_server/manifest/

# 確認
kubectl get pods -n default
curl -i http://localhost/httpd/hello.html
# => Hello, kind!

# 再起動
kubectl rollout restart deployment piyo-app -n default

# 削除
kubectl delete  -f ./src/web_server/manifest/
kubectl delete --all deployment,pod,svc
```
