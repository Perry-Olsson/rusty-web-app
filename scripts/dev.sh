#!/bin/bash

script_dir="$(dirname "$(readlink -f "$0")")"

source $script_dir/metadata.sh

if [[ "$1" == "down" ]]; then
    docker stop $APP_NAME
    exit 0
fi

docker stop $APP_NAME

docker rm $APP_NAME

docker build --build-arg APP_NAME="$APP_NAME" . --tag $APP_NAME --target dev

docker run \
    --name $APP_NAME \
    -v $script_dir/..:/$APP_NAME \
    -p 8080:8080 \
    -d \
    $APP_NAME
