#!/usr/bin/env bash
# -------------------------------------------------------------------------------
# Boilermaker Release script
#           Usage: ./release.sh <new-version>
#         Example: ./release.sh 1.2.3
#     Description: This script automates the release process for Boilermaker.
#
# It performs the following steps:
#     1. Validates the input version format.
#     2. Updates the version in Cargo.toml.
#     3. Commits the version bump.
#     4. Creates a git tag for the new version.
#     5. Pushes the commit and tag to GitHub (triggering cargo-dist).
#     6. Publishes the 'boilermaker-core' crate to crates.io.
#     7. Waits for 'boilermaker-core' to be available on crates.io.
#     8. Publishes the 'boilermaker' crate to crates.io.
# -------------------------------------------------------------------------------

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





#### ________________________________________________________ CONFIGURE RELEASE

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

# TODO: PUT BACK!
# Check if the git working directory is clean
# if ! git diff --quiet || ! git diff --cached --quiet; then
#     echo -e "$ERROR Git working directory is not clean. Commit or stash changes first."
#     exit 1
# fi

OLD_VERSION=$(grep '^version = ' Cargo.toml | head -n 1 | sed -E 's/version = "(.*)"/\1/')
NEW_VERSION="${1#v}"
TAG="v$NEW_VERSION"

# Print config for confirmation
echo -e "${INFO} Found the following:"
echo -e "  ${BLUE}OLD_VERSION: ${NC}$OLD_VERSION"
echo -e "  ${BLUE}NEW_VERSION: ${NC}$NEW_VERSION"
echo -e "          ${BLUE}TAG: ${NC}$TAG"

# Confirm accuracy of versions
CONFIG_MSG=$(echo -e "\n${ORANGE}Continue? ${NC}(${GREEN}y${NC}/${RED}n${NC}): ")
read -p "$CONFIG_MSG " -n 1 -r; echo; [[ $REPLY =~ ^[Yy]$ ]] || exit 1





#### ________________________________________________________ START RELEASE

echo -e "\n${INFO} 0. ${BOLD}Starting release:${NC} ${BLUE}${NEW_VERSION}${NC}"

# Update Cargo.toml with the new version
echo -e "$INFO 1. ${BOLD}Cargo.toml:${NC} ${BLUE}${OLD_VERSION}${NC} --> ${BLUE}${NEW_VERSION}${NC}"
sed -i.bak -e "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
sed -i.bak -E "s/^(boilermaker_core[[:space:]]*=.*version[[:space:]]*=[[:space:]]*\")[^\"]+(\".*)$/\1$NEW_VERSION\2/" Cargo.toml
rm Cargo.toml.bak

# Update Cargo.lock to reflect the new version
echo -e "$INFO 2. ${BOLD}Cargo.lock:${NC} syncing with Cargo.toml"
cargo check --quiet


# Sync to GitHub
echo -e "$INFO 3. ${BOLD}Committing version bump ${NC}"
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to $NEW_VERSION"

echo -e "$INFO 4. ${BOLD}Creating tag $TAG ${NC}"
git tag "$TAG"

echo -e "$INFO 5. ${BOLD}Pushing to GitHub (Triggering cargo-dist) ${NC}"
CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
git push origin "$CURRENT_BRANCH"
git push origin "$TAG"

echo -e "$INFO 6a. ${BOLD}boilermaker-core (crate):${NC} Dry run "
cargo publish -p boilermaker-core --dry-run
echo -e "$INFO 6b. ${BOLD}boilermaker-core (crate):${NC} Publishing "
#cargo publish -p boilermaker-core

# echo -e "$INFO 7. Waiting for 'boilermaker-core' to be ready on crates.io "
# # We poll the crates.io API until the new version returns a 200 OK status.
# MAX_RETRIES=30
# for ((i=1; i<=MAX_RETRIES; i++)); do
#     HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" "https://crates.io/api/v1/crates/boilermaker-core/$NEW_VERSION")
#     if [ "$HTTP_CODE" -eq 200 ]; then
#         echo -e "--> boilermaker-core v$NEW_VERSION is live on crates.io!"
#         break
#     fi
#     echo -e "--> Not found yet (Status $HTTP_CODE). Retrying in 10s... ($i/$MAX_RETRIES)"
#     sleep 10
#
#     if [ "$i" -eq "$MAX_RETRIES" ]; then
#         echo -e "Error: Timed out waiting for crates.io to index boilermaker-core."
#         exit 1
#     fi
# done
#
# # Force Cargo to update its local registry index so the next step can find the core crate
# echo -e "--> Syncing local Cargo registry..."
# cargo update -p boilermaker-core || cargo search boilermaker-core --limit 1 > /dev/null
#
# echo -e "$INFO 8. Publishing 'boilermaker' "
# # The dry run here will now succeed because boilermaker-core is available in the index.
# echo -e "--> Dry run: boilermaker..."
# cargo publish -p boilermaker --dry-run
# echo -e "--> Actual publish: boilermaker..."
# cargo publish -p boilermaker
#
# echo -e "$INFO 9. Success "
# echo -e "Successfully deployed and published version $NEW_VERSION!"
