#!/bin/bash

set -e

# Step 1: Ensure we are in the main branch
current_branch=$(git rev-parse --abbrev-ref HEAD)
if [[ "$current_branch" != "main" ]]; then
    echo "You are not on the main branch. Please switch to the main branch."
    exit 1
fi

# Step 2: Ensure the main branch is up to date
git fetch origin && git pull
if ! git diff --quiet origin/main; then
    echo "The main branch is not up to date with origin/main. Please pull the latest changes."
    exit 1
fi

# Step 3: Get the latest tag
latest_tag=$(git describe --tags --abbrev=0 2>/dev/null || echo "0.0.0")
IFS='.' read -r major minor fix <<< "$latest_tag"

# Step 4: Update version based on the type
type=$1
if [[ -z "$type" ]]; then
    echo "Release type not specified. Use: fix, minor, or major."
    exit 1
fi

case $type in
    fix)
        fix=$((fix + 1))
        ;;
    minor)
        minor=$((minor + 1))
        fix=0
        ;;
    major)
        major=$((major + 1))
        minor=0
        fix=0
        ;;
    *)
        echo "Invalid type: $type. Use: major, minor, or fix."
        exit 1
        ;;
esac

new_version="${major}.${minor}.${fix}"
echo "Updating version to $new_version"

# Update version in Cargo.toml
sed -i "s/^version = \".*\"/version = \"$new_version\"/" flowrs/Cargo.toml

# Step 5: Commit and push the changes
git add flowrs/Cargo.toml
git commit -m "Bump version to $new_version"
git push origin main

# Step 6: Ensure the main branch is up to date (redundant, but safe)
git fetch origin
if ! git diff --quiet origin/main; then
    echo "The main branch is not up to date after pushing changes."
    exit 1
fi

# Step 7: Create a new tag
git tag -a "$new_version" -m "Release $new_version"

# Step 8: Push the new tag
git push origin "$new_version"
echo "Release $new_version created and pushed successfully."
