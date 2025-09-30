#!/usr/bin/env bash 

target="$1"

if [ -z "$target" ]; then
    echo -e "Usage: $0 <target>"
    echo -e "Example: $0 web # or desktop"
    exit 1
fi
    
# TODO: Figure out how to add generated tailwind.css to all UI assets (mobile, web, etc)
npx @tailwindcss/cli \
    -i ./tailwind.css \
    -o ./$target/assets/tailwind.css \
    --watch
