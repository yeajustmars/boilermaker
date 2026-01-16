#!/usr/bin/env bash

# #####################################################################
# boilermaker/serve-web.sh
#
# (Dev only!)
#
# Shorthand for running the boilermaker web package.
#
# Usage:
#
# ./serve-web.sh  # default, watches for template changes
#
# ./serve-web.sh --watch  # watches for template and Rust code changes via cargo-watch
#
# #####################################################################

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" >/dev/null 2>&1 && pwd)"
cd $SCRIPT_DIR

should_watch=false
if [ "$1" == "--watch" ]; then
  should_watch=true
fi

cd packages/boilermaker_web

if $should_watch; then
  cargo watch -x 'run'
else
  cargo run
fi

exit $?
