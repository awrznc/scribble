#!/bin/bash -eux

SCRIPT_DIR_PATH=$(cd $(dirname $0); pwd)
YAML_FILE_PATH=${SCRIPT_DIR_PATH}/docker-compose.yml
BUILDER_NAME="piyo_builder"

docker compose -f ${YAML_FILE_PATH} up -d
trap "docker buildx rm ${BUILDER_NAME}; docker compose -f ${YAML_FILE_PATH} down" EXIT

# Builder の作成
docker buildx create \
    --name ${BUILDER_NAME} \
    --driver docker-container \
    --driver-opt network=piyo_network

# BuildX でビルド
docker buildx build \
    --load \
    --no-cache \
    --add-host server:$(docker inspect piyo_server -f '{{.NetworkSettings.Networks.piyo_network.IPAddress}}') \
    --builder=${BUILDER_NAME} \
    -t with_buildx:latest .
    # --cache-from=type=local,src='./build/'
    # --cache-to=type=local,dest='./build/'
