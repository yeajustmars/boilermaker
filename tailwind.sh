#!/usr/bin/env bash

npx @tailwindcss/cli \
    -i ./tailwind.css \
    -o ./views/assets/tailwind.css \
    --watch
