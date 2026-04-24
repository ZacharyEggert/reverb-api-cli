#!/usr/bin/env bash
# Create and push a release tag from the current Cargo.toml version
set -euo pipefail

VERSION=$(grep '^version' crates/reverb-api-cli/Cargo.toml | head -1 | sed 's/version = "//;s/"//')
TAG="v$VERSION"

echo "Creating tag $TAG"
git tag "$TAG"
echo "Push with: git push origin $TAG"
