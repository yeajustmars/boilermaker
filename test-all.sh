#!/usr/bin/env bash

SCRIPT_DIR=$(dirname "$0")
cd "$SCRIPT_DIR"

cargo test --workspace
