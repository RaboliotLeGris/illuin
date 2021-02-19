#!/bin/bash

PORT="${PORT:-8080}"
STORAGE_FOLDER="${ILLUIN_STORAGE_FOLDER:-"/illuin/data"}"

echo "ENV VAR ARE: $PORT $STORAGE_FOLDER"

exec "$@" "--port" "$PORT" "--storage-path" "$STORAGE_FOLDER"
