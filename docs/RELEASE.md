# Release Guide

This project uses [`cargo-release`](https://github.com/crate-ci/cargo-release) for automated releases.

## Quick Start

### Installation

```bash
cargo install cargo-release
```

### Basic Usage

**Semantic versioning (recommended):**
```bash
cargo release patch   # 0.1.0 -> 0.1.1
cargo release minor   # 0.1.0 -> 0.2.0
cargo release major   # 0.1.0 -> 1.0.0
```

**Specific version:**
```bash
cargo release 0.2.0
```

### Full Release Workflow

1. **Preview changes (dry run):**
   ```bash
   cargo release --dry-run minor
   ```

2. **Execute the release:**
   ```bash
   cargo release minor --execute
   ```
   
   This will:
   - Update `Cargo.toml` version
   - Update `Cargo.lock` if needed
   - Create a commit with the version change
   - Create a git tag `v<version>`

3. **Push to remote:**
   ```bash
   git push origin main
   git push origin --tags
   ```

4. **Automatic publishing:**
   - GitHub Actions will detect the tag push
   - CI will validate version matches tag
   - If validation passes, automatically publishes to crates.io

## CI Validation

The GitHub Actions workflow automatically validates that:
- The version in `Cargo.toml` matches the tag version
- If they don't match, the workflow fails with a clear error

This prevents the issue where a tag is created but `Cargo.toml` version doesn't match.

## Common Commands

```bash
# Preview what will happen
cargo release --dry-run patch

# Release patch version
cargo release patch --execute

# Release minor version
cargo release minor --execute

# Release major version
cargo release major --execute

# Release specific version
cargo release 0.2.0 --execute

# Skip git operations (just update files)
cargo release patch --no-publish --no-push --no-tag
```

## Troubleshooting

**Error: "cargo-release not found"**
```bash
cargo install cargo-release
```

**Error: "Working directory is not clean"**
```bash
# Commit or stash your changes first
git add .
git commit -m "your changes"
# or
git stash
```

**Error: "Tag already exists"**
```bash
# Delete the tag if it was created incorrectly
git tag -d v0.2.0
git push origin :refs/tags/v0.2.0
# Then run cargo-release again
```

## Why cargo-release?

- ✅ Industry standard tool for Rust releases
- ✅ Handles version bumps automatically
- ✅ Creates proper git commits and tags
- ✅ Supports semantic versioning
- ✅ Dry-run mode for safety
- ✅ Well-maintained and widely used

## Additional Resources

- [cargo-release documentation](https://github.com/crate-ci/cargo-release)
- [Semantic Versioning](https://semver.org/)
- [CONTRIBUTING.md](../CONTRIBUTING.md) - Full contribution guidelines
