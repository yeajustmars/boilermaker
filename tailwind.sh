#!/usr/bin/env bash 

# TODO: Figure out how to add generated tailwind.css to all UI assets (mobile, web, etc)
npx @tailwindcss/cli \
    -i ./tailwind.css \
    -o ./web/assets/tailwind.css \
    --watch
