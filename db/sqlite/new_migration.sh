#!/bin/bash

SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"
MIGRATION_NAME=$1
NOW=$(date +%s%3N)

echo "-- Write your SQL here" > "${SCRIPT_DIR}/migrations/${NOW}_${MIGRATION_NAME}.sql"

