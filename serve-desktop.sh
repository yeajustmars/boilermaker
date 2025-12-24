#!/usr/bin/env bash

OS_TYPE=$(uname -s)

platform=""
case "$OS_TYPE" in
    Darwin*) platform="macos";;
    Linux*)  platform="linux";;
    *)       echo "Unsupported OS: $OS_TYPE"; exit 1;;
esac

cd boilermaker_desktop && dx serve --hotpatch --platform "$platform"
