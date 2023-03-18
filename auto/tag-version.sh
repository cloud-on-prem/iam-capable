#!/bin/bash

set -e

# Read the version from Cargo.toml
version=$(grep -E '^version' Cargo.toml | head -1 | awk -F' ' '{print $3}' | tr -d '\"')

# Check if the version is already tagged
if git tag --list "v$version" | grep -q "v$version"; then
  echo "Version v$version is already tagged. Update the version in Cargo.toml before tagging."
  exit 1
fi

# Tag the version and push the tag
git tag "v$version"
git push origin "v$version"

echo "Tagged and pushed the new version: v$version"
