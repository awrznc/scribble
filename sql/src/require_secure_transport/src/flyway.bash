#!/bin/bash -eux

mkdir -p /flyway/certs/

# ローカル用
# https://documentation.red-gate.com/flyway/database-development-using-flyway/connecting-to-environments/tutorial-configure-ssl-for-database-connections?utm_source=chatgpt.com
keytool \
    -import \
    -file /opts/mysql/certs/ca.pem \
    -storetype PKCS12 \
    -keystore /flyway/certs/truststore-local.p12 \
    -alias "local" \
    -storepass storepass \
    -noprompt

# AWS用
curl -o /flyway/certs/global-bundle.pem -LO https://truststore.pki.rds.amazonaws.com/global/global-bundle.pem
keytool \
    -import \
    -file /flyway/certs/global-bundle.pem \
    -storetype PKCS12 \
    -keystore /flyway/certs/truststore-global.p12 \
    -alias "global" \
    -storepass storepass \
    -noprompt
