#!/bin/bash -eu

SCRIPT_DIR_PATH=$(cd $(dirname $0); pwd)
YAML_FILE_PATH=${SCRIPT_DIR_PATH}/share/docker/docker-compose.yaml
BUILD_FILE_PATH=${SCRIPT_DIR_PATH}/share/docker/prod.Dockerfile

# テスト用のイメージを作成
BEFORE_ID=$(docker images test_app:latest --format '{{.ID}}')
docker compose -f ${YAML_FILE_PATH} build
AFTER_ID=$(docker images test_app:latest --format '{{.ID}}')

# ソースコードに更新がある場合はテストを実行及び本番で利用するイメージを作成
if [ "${BEFORE_ID}" != "${AFTER_ID}" ]; then
  # テストでエラーが出た場合は rmi で Untag する（キャッシュは残る）
  trap "docker rmi ${AFTER_ID}" ERR
  trap "docker compose -f ${YAML_FILE_PATH} down" EXIT
  docker compose -f ${YAML_FILE_PATH} run --rm test
  docker build -f ${BUILD_FILE_PATH} -t prod_app:latest .
fi

echo -e "\nfinished!\n"
