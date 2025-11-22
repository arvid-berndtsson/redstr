#!/bin/bash
# Helper script to create a release with automatic version bumping
# Usage: ./scripts/release.sh <version>
# Example: ./scripts/release.sh 0.2.0

set -e

if [ $# -eq 0 ]; then
    echo "❌ Error: Version argument required"
    echo ""
    echo "Usage: $0 <version>"
    echo "Example: $0 0.2.0"
    echo ""
    echo "This script will:"
    echo "  1. Update Cargo.toml to the specified version"
    echo "  2. Commit the change"
    echo "  3. Create a git tag v<version>"
    echo "  4. Push the commit and tag"
    exit 1
fi

VERSION=$1
TAG="v${VERSION}"

# Validate version format (basic check)
if ! echo "$VERSION" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+$'; then
    echo "❌ Error: Invalid version format: $VERSION"
    echo "Expected format: X.Y.Z (e.g., 0.2.0)"
    exit 1
fi

# Check if tag already exists
if git rev-parse "$TAG" >/dev/null 2>&1; then
    echo "❌ Error: Tag $TAG already exists"
    exit 1
fi

# Check if working directory is clean
if ! git diff-index --quiet HEAD --; then
    echo "❌ Error: Working directory is not clean"
    echo "Please commit or stash your changes before creating a release"
    exit 1
fi

# Get current version
CURRENT_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
echo "Current version: $CURRENT_VERSION"
echo "New version: $VERSION"
echo "Tag: $TAG"
echo ""

# Confirm
read -p "Continue? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted"
    exit 1
fi

# Update Cargo.toml
echo "📝 Updating Cargo.toml..."
# Use cargo metadata to verify we're updating the right package
# More robust than sed - handles edge cases better
sed -i.bak "s/^version = \".*\"/version = \"$VERSION\"/" Cargo.toml
rm -f Cargo.toml.bak

# Verify the change using cargo metadata (more reliable)
NEW_VERSION=$(cargo metadata --format-version 1 --no-deps 2>/dev/null | \
  grep -o '"version":"[^"]*"' | head -1 | cut -d'"' -f4)

# Fallback to grep if cargo metadata fails
if [ -z "$NEW_VERSION" ]; then
    NEW_VERSION=$(grep '^version = ' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
fi

if [ "$NEW_VERSION" != "$VERSION" ]; then
    echo "❌ Error: Failed to update version in Cargo.toml"
    echo "   Expected: $VERSION"
    echo "   Got: $NEW_VERSION"
    exit 1
fi

# Run cargo check to ensure everything compiles
echo "🔍 Running cargo check..."
cargo check --quiet || {
    echo "❌ Error: cargo check failed"
    exit 1
}

# Commit the change
echo "📦 Committing version change..."
git add Cargo.toml
git commit -m "chore: bump version to $VERSION" || {
    echo "❌ Error: Failed to commit"
    exit 1
}

# Create tag
echo "🏷️  Creating tag $TAG..."
git tag -a "$TAG" -m "Release $VERSION" || {
    echo "❌ Error: Failed to create tag"
    exit 1
}

echo ""
echo "✅ Release prepared successfully!"
echo ""
echo "To publish the release, run:"
echo "  git push origin main"
echo "  git push origin $TAG"
echo ""
echo "Or push both at once:"
echo "  git push origin main $TAG"
echo ""
read -p "Push now? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "📤 Pushing to origin..."
    git push origin main
    git push origin "$TAG"
    echo ""
    echo "✅ Release pushed! The GitHub Actions workflow will publish to crates.io automatically."
fi
