#!/usr/bin/env bash

BT_LOGO=$(cat <<'BT_TEXT'
 ___      _ _                    _             ___     _
| _ ) ___(_) |___ _ _ _ __  __ _| |_____ _ _  | _ \___| |___ __ _ ___ ___
| _ \/ _ \ | / -_) '_| '  \/ _` | / / -_) '_| |   / -_) / -_) _` (_-</ -_)
|___/\___/_|_\___|_| |_|_|_\__,_|_\_\___|_|   |_|_\___|_\___\__,_/__/\___|

BT_TEXT
)

BOLD='\033[1m'
ITAL='\033[3m'
BLUE='\033[0;34m'
RED='\033[0;31m'
GREEN='\033[0;32m'
ORANGE='\033[0;33m'
NC='\033[0m'

ERROR="${RED}[ERROR]${NC}"
HINT="${ORANGE}[HINT]${NC}"
INFO="${BLUE}[INFO]${NC}"
SUCCESS="${GREEN}[OK]${NC}"

echo -e "${BLUE}${BT_LOGO}${NC}\n"

script_dir=$(cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd)
cd $script_dir

set -euo pipefail

if [ "$#" -ne 1 ]; then
    echo -e "      ${BOLD}Usage:${NC} $0 <new-version>"
    echo -e "    ${BOLD}Example:${NC} $0 1.2.3"
    echo -e "\n$ERROR Missing required version argument."
    exit 1
fi

if [[ ! "$1" =~ ^(v)?[0-9]+\.[0-9]+\.[0-9]+(-.*)?$ ]]; then
    echo -e "$ERROR Version must be in the format X.Y.Z (e.g., 1.2.3)"
    exit 1
fi

OLD_VERSION=$(grep '^version = ' Cargo.toml | head -n 1 | sed -E 's/version = "(.*)"/\1/')
NEW_VERSION="${1#v}"
TAG="v$NEW_VERSION"

echo -e "${INFO} Found the following:"
echo -e "  ${BLUE}OLD_VERSION: ${NC}$OLD_VERSION"
echo -e "  ${BLUE}NEW_VERSION: ${NC}$NEW_VERSION"
echo -e "          ${BLUE}TAG: ${NC}$TAG"

echo -e "$HINT Make sure you have committed all your changes and are on the correct branch before proceeding.\n"
CONFIG_MSG=$(echo -e "${ORANGE}Continue? ${NC}(${GREEN}y${NC}/${RED}n${NC}): ")
read -p "$CONFIG_MSG " -n 1 -r; echo; [[ $REPLY =~ ^[Yy]$ ]] || exit 1

echo -e "\n${INFO} Starting release process for version ${BLUE}${NEW_VERSION}${NC}"

# Ensure the git working directory is clean before we start mutating files
if ! git diff --quiet || ! git diff --cached --quiet; then
    echo "Error: Git working directory is not clean. Commit or stash changes first."
    exit 1
fi

echo "$INFO 1. Setting version in Cargo.toml to $NEW_VERSION ${INFO}"
# Using a cross-platform (macOS/Linux) sed command to replace the workspace version.
# Assumes `version = "..."` is at the top level of your main Cargo.toml.
sed -i.bak -e "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
sed -i.bak -E "s/^(boilermaker_core[[:space:]]*=.*version[[:space:]]*=[[:space:]]*\")[^\"]+(\".*)$/\1$NEW_VERSION\2/" Cargo.toml
rm Cargo.toml.bak

# # Update Cargo.lock to reflect the new version
# cargo check --quiet
#
# # Commit the version bump so the tag points to the correct state
# echo "=== Committing version bump ==="
# git add Cargo.toml Cargo.lock
# git commit -m "chore: bump version to $NEW_VERSION"
#
# echo "=== 2. Creating tag $TAG ==="
# git tag "$TAG"
#
# echo "=== 3. Pushing to GitHub (Triggering cargo-dist) ==="
# CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
# git push origin "$CURRENT_BRANCH"
# git push origin "$TAG"
#
# echo "=== 4. Publishing 'boilermaker-core' ==="
# echo "--> Dry run: boilermaker-core..."
# cargo publish -p boilermaker-core --dry-run
# echo "--> Actual publish: boilermaker-core..."
# cargo publish -p boilermaker-core
#
# echo "=== 5. Waiting for 'boilermaker-core' to be ready on crates.io ==="
# # We poll the crates.io API until the new version returns a 200 OK status.
# MAX_RETRIES=30
# for ((i=1; i<=MAX_RETRIES; i++)); do
#     HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" "https://crates.io/api/v1/crates/boilermaker-core/$NEW_VERSION")
#     if [ "$HTTP_CODE" -eq 200 ]; then
#         echo "--> boilermaker-core v$NEW_VERSION is live on crates.io!"
#         break
#     fi
#     echo "--> Not found yet (Status $HTTP_CODE). Retrying in 10s... ($i/$MAX_RETRIES)"
#     sleep 10
#
#     if [ "$i" -eq "$MAX_RETRIES" ]; then
#         echo "Error: Timed out waiting for crates.io to index boilermaker-core."
#         exit 1
#     fi
# done
#
# # Force Cargo to update its local registry index so the next step can find the core crate
# echo "--> Syncing local Cargo registry..."
# cargo update -p boilermaker-core || cargo search boilermaker-core --limit 1 > /dev/null
#
# echo "=== 6. Publishing 'boilermaker' ==="
# # The dry run here will now succeed because boilermaker-core is available in the index.
# echo "--> Dry run: boilermaker..."
# cargo publish -p boilermaker --dry-run
# echo "--> Actual publish: boilermaker..."
# cargo publish -p boilermaker
#
# echo "=== 7. Success ==="
# echo "Successfully deployed and published version $NEW_VERSION!"
