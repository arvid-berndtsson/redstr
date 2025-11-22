# Release Process

This document explains the release process for `redstr` and why we chose this approach.

## Current Approach

We use a **hybrid approach** combining:
1. **Custom release script** (`scripts/release.sh`) - for local releases
2. **CI validation** - ensures version matches tag before publishing
3. **GitHub Actions** - automated publishing to crates.io

## Why This Approach?

### ✅ Advantages

1. **Zero Runtime Dependencies**: Aligns with our core principle
   - No need to install `cargo-release` or other tools
   - Works out of the box for all contributors

2. **Simple and Transparent**: 
   - Easy to understand what's happening
   - Script is readable and maintainable
   - No "magic" - you can see exactly what it does

3. **CI Validation**: 
   - Catches mistakes before publishing
   - Clear error messages
   - Prevents version mismatches

4. **Flexible**:
   - Can be customized for project needs
   - Easy to add features (e.g., changelog generation)

### ⚠️ Trade-offs

1. **Maintenance**: Custom script needs to be maintained
2. **Features**: Less feature-rich than `cargo-release`
3. **Testing**: Need to test the script itself

## Alternative Approaches

### Option 1: `cargo-release` (Industry Standard)

**Pros:**
- Battle-tested, widely used
- Handles changelog generation
- Supports pre-release hooks
- Semantic versioning helpers

**Cons:**
- Requires installing a tool (`cargo install cargo-release`)
- Adds a dependency (even if just for releases)
- Less transparent (more "magic")

**Usage:**
```bash
cargo release patch  # 0.1.0 -> 0.1.1
cargo release minor  # 0.1.0 -> 0.2.0
cargo release major  # 0.1.0 -> 1.0.0
```

### Option 2: GitHub Actions Auto-bump

**Pros:**
- Fully automated
- No local tooling needed

**Cons:**
- Less control over release timing
- Harder to review changes before release
- More complex workflow

### Option 3: Current Approach (Recommended)

**Best for:**
- Projects with zero-dependency philosophy
- Teams that want transparency
- Simple release needs

## Release Workflow

### Using the Script (Recommended)

```bash
./scripts/release.sh 0.2.0
```

This will:
1. ✅ Validate version format
2. ✅ Check tag doesn't exist
3. ✅ Verify working directory is clean
4. ✅ Update `Cargo.toml`
5. ✅ Run `cargo check`
6. ✅ Commit the change
7. ✅ Create git tag
8. ✅ Optionally push to origin

### Manual Process

If you prefer manual control:

```bash
# 1. Update Cargo.toml version
vim Cargo.toml  # Change version = "0.2.0"

# 2. Commit
git add Cargo.toml
git commit -m "chore: bump version to 0.2.0"

# 3. Create and push tag
git tag -a v0.2.0 -m "Release 0.2.0"
git push origin main
git push origin v0.2.0
```

## CI Validation

The GitHub Actions workflow automatically:
- ✅ Validates version matches tag
- ✅ Fails with clear error if mismatch
- ✅ Publishes to crates.io if validation passes

This prevents the issue where a tag is created but `Cargo.toml` version doesn't match.

## Future Improvements

Potential enhancements:
- [ ] Add changelog generation
- [ ] Support semantic versioning helpers (patch/minor/major)
- [ ] Pre-release validation (tests, docs, etc.)
- [ ] GitHub release creation
- [ ] Automatic dependency updates

## Comparison Table

| Feature | Current | cargo-release | Auto-bump |
|---------|---------|---------------|-----------|
| Zero deps | ✅ | ❌ | ✅ |
| Transparency | ✅ | ⚠️ | ❌ |
| Changelog | ❌ | ✅ | ❌ |
| Semantic versioning | ❌ | ✅ | ❌ |
| CI validation | ✅ | ✅ | ✅ |
| Ease of use | ✅ | ✅ | ✅ |

## Conclusion

The current approach strikes a good balance between:
- **Simplicity**: Easy to understand and use
- **Reliability**: CI validation prevents mistakes
- **Philosophy**: Aligns with zero-dependency principle
- **Flexibility**: Can be extended as needed

For most Rust projects, `cargo-release` is the standard choice. However, for projects with strict dependency policies or those wanting maximum transparency, the current approach is a solid alternative.
