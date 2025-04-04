Logs panel
===

```bash
docker compose up -d

# lokiにログを送る
curl -H 'Content-Type: application/json' -XPOST -s 'http://localhost:3100/loki/api/v1/push' --data-raw "$(NOW=$(date +%s%N) envsubst < log.json)"
# => http://localhost:3100/loki/api/v1/labels
# => http://localhost:3000/d/dehbfoqcowsg0a/logs
# admin/admin

docker compose down
```
