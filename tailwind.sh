#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

cd $SCRIPT_DIR

npx @tailwindcss/cli \
    -i ./tailwind.css \
    -o ./boilermaker_views/assets/tailwind.css \
    --watch
