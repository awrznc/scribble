Hello, World!
===

```bash
docker compose up
# => http://localhost:5601/

# Add data
curl -H 'Content-Type: application/json' \
     -XPOST http://localhost:9200/hello-world-index/user \
     -d '{ "@timestamp":"2023-07-01T00:00:00.000+09:00", "message":"Hello, World!" }'

# Check
curl -X GET http://localhost:9200/hello-world-index/user/_search?pretty

# Create index
curl -X POST http://localhost:5601/api/saved_objects/index-pattern/hello-world \
    --header "kbn-xsrf: true" \
    --header "Content-Type: application/json" \
    --user "kibana_system:${kibana_system_user_password}" \
    --data '{ "attributes": { "title" : "hello-world-*", "timeFieldName": "@timestamp" } }'

# => http://localhost:5601/app/discover#/?_g=(filters:!(),refreshInterval:(pause:!t,value:0),time:(from:now-255y,to:now))&_a=(columns:!(_source),filters:!(),index:hello-world,interval:auto,query:(language:kuery,query:''),sort:!())
```
