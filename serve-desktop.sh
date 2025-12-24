#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"

cd $SCRIPT_DIR

OS_TYPE=$(uname -s)

platform=""
case "$OS_TYPE" in
    Darwin*) platform="macos";;
    Linux*)  platform="linux";;
    *)       echo "Unsupported OS: $OS_TYPE"; exit 1;;
esac

cd boilermaker_desktop && dx serve --hotpatch --platform "$platform"
