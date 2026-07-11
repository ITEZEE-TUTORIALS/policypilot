#!/usr/bin/env bash
set -e

cd "$(dirname "$0")"

exec cargo run -- --serve "${POLICYPILOT_ADDR:-127.0.0.1:7878}"
