#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

cd $SCRIPT_DIR

npm install tailwindcss @tailwindcss/cli

npx @tailwindcss/cli \
    -i ./packages/boilermaker_ui/tailwind.css \
    -o ./packages/boilermaker_ui/assets/css/tailwind.css \
    --watch
