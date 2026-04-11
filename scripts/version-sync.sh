#!/usr/bin/env bash
# Sync the version from crates/reverb-api-cli/Cargo.toml → npm/package.json and package.json
set -euo pipefail

VERSION=$(grep '^version' crates/reverb-cli/Cargo.toml | head -1 | sed 's/version = "//;s/"//')

echo "Syncing version: $VERSION"

# Update root package.json
sed -i.bak "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json && rm package.json.bak
sed -i.bak "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" npm/package.json && rm npm/package.json.bak

# Keep library crate in sync
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" crates/reverb/Cargo.toml && rm crates/reverb/Cargo.toml.bak

echo "Done."
