connection
===

同じPod内のコンテナ間通信を行う例。

```bash
# Pod内の特定のコンテナに接続
minikube kubectl -- exec -it connection-app-xxxxxxxxxx-xxxxx -c send-container -- bash

# 確認
cat /etc/redhat-release
# => AlmaLinux release 9.1 (Lime Lynx)

# send!
curl -i 'http://localhost:80/'
```
