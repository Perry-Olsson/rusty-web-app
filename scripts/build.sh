#!/bin/bash

script_dir="$(dirname "$(readlink -f "$0")")"

source $script_dir/metadata.sh

docker exec -t $APP_NAME cargo build --manifest-path /$APP_NAME/Cargo.toml
