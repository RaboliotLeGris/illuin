#!/bin/bash

PORT="${PORT:-8080}"
STORAGE_FOLDER="${ILLUIN_STORAGE_FOLDER:-"/illuin/data"}"
BASE_URL="${ILLUIN_BASE_URL:-"/"}"

echo "ENV VAR ARE: $PORT $STORAGE_FOLDER"

exec "$@" "--port" "$PORT" "--storage-path" "$STORAGE_FOLDER" "--base-route" "$BASE_URL"
