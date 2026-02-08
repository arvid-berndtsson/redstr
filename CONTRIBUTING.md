# Contributing to redstr

Thank you for your interest in contributing to redstr! This document provides guidelines and instructions for contributing.

## 🎯 Getting Started

### For Human Developers

1. **Pick a Task**
   - Check `roadmap/TASKS.md` for available tasks
   - Look for tasks with no assignee (⬜ Not Started)
   - Choose based on your skills and interests
   - Check that dependencies are met

2. **Claim the Task**
   - Comment on the task in `roadmap/TASKS.md` or create an issue
   - Update status: ⬜ → 🟡 (In Progress)
   - Assign yourself if using GitHub

3. **Set Up Development**
   ```bash
   git clone https://github.com/arvid-berndtsson/redstr.git
   cd redstr
   cargo build
   cargo test
   ```

4. **Create Feature Branch**
   ```bash
   git checkout -b task/{TASK-ID}-{short-description}
   ```

5. **Make Changes**
   - Follow the task requirements
   - Write tests
   - Update documentation
   - Follow code style

6. **Submit PR**
   - Push your branch
   - Create PR with task ID in title
   - Use PR template
   - Update task status: 🟡 → ✅

### For AI Agents (Cursor/GitHub)

1. **Find Agent-Friendly Tasks**
   - Look for tasks tagged `[AGENT-FRIENDLY]` in `roadmap/TASKS.md`
   - Check task has no assignee
   - Verify dependencies are met

2. **Follow Task Template**
   - Read task description carefully
   - Follow function signatures provided
   - Implement all requirements
   - Create tests and documentation

3. **Create Atomic PRs**
   - One feature per PR
   - Include task ID in PR title
   - Use PR template
   - Ensure all checks pass

## 📋 Task Workflow

### Task Status

- ⬜ **Not Started** - Available for assignment
- 🟡 **In Progress** - Currently being worked on
- ✅ **Completed** - Done and merged
- 🔄 **Blocked** - Waiting on dependencies
- ❌ **Cancelled** - No longer needed

### Updating Task Status

Update status in `roadmap/TASKS.md` when:
- Starting: ⬜ → 🟡
- Completing: 🟡 → ✅
- Blocked: 🟡 → 🔄

**Note:** Update via PR, not directly in main branch.

## 🛠️ Development Guidelines

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Run clippy and fix warnings (`cargo clippy`)
- Use 4 spaces for indentation
- Keep line length < 100 characters
- Follow Rust naming conventions

### Pre-commit Hooks

A pre-commit hook is automatically installed that runs the following checks before each commit:

1. **Code Formatting** (`cargo fmt --check`)
   - Ensures code follows Rust standard formatting
   - If formatting fails, run `cargo fmt` to auto-format

2. **Clippy** (`cargo clippy -- -D warnings`)
   - Catches common mistakes and enforces idiomatic Rust
   - All warnings must be fixed before committing

3. **Compilation Check** (`cargo check`)
   - Verifies code compiles successfully
   - Catches compilation errors early

4. **Tests** (`cargo test`)
   - Runs all tests to ensure nothing is broken
   - All tests must pass before committing

**Note:** If you need to skip the pre-commit hook (e.g., for WIP commits), use `git commit --no-verify`. However, **never skip hooks for final commits** - all checks must pass before merging.

### Minimal Dependencies Principle

**CRITICAL:** The core library must not have required dependencies.

- ✅ Use `std` only for core functionality
- ✅ Optional features (like `serde`) behind feature flags are acceptable
- ❌ Never add required dependencies to core library
- ✅ Dev-dependencies (like `criterion`) are OK for testing/benchmarking

### Testing

- Write tests for all public functions
- Test edge cases (empty strings, Unicode, special chars)
- Run `cargo test` before submitting
- Aim for 100% test coverage

### Documentation

- Every public function needs doc comments (`///`)
- Include `# Examples` section
- Document security use cases
- Reference related functions

### Performance

- Use `String::with_capacity` when size is known
- Avoid unnecessary allocations
- Profile performance-critical paths
- Benchmark new functions

## 📝 Commit Messages

Conventional commit format is recommended for readability and release hygiene.

### Format

```
[TASK-ID] <type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks
- `perf`: Performance improvements

### Examples

```
[CF-001] feat(cloudflare): add Turnstile challenge variation

Implements cloudflare_turnstile_variation() for bot detection evasion.

- Add function to cloudflare.rs
- Add comprehensive tests
- Add documentation with examples

Closes #123
```

```
[PKG-001] feat(packaging): create Debian package structure

Adds debian/ directory with control, rules, and changelog files.

- Create debian/control
- Add debian/rules for building
- Add debian/changelog
```

```
fix(encoding): correct URL encoding for special characters

Fixes issue where certain Unicode characters were not properly encoded.

Fixes #456
```

### Validation

Commit format is not hard-enforced by CI. The format above is still
recommended to keep project history searchable and consistent.

## 🚀 Release Process

### Creating a Release

We use [`cargo-release`](https://github.com/crate-ci/cargo-release) to automate the release process and prevent version mismatches.

#### Prerequisites

Install `cargo-release` (one-time setup):
```bash
cargo install cargo-release
```

#### Recommended: Use cargo-release

The easiest way to create a release is using `cargo-release`:

**For semantic versioning:**
```bash
cargo release patch   # 0.1.0 -> 0.1.1
cargo release minor   # 0.1.0 -> 0.2.0
cargo release major   # 0.1.0 -> 1.0.0
```

**For specific version:**
```bash
cargo release 0.2.0
```

**Dry run (preview changes without committing):**
```bash
cargo release --dry-run patch
```

`cargo-release` will:
1. ✅ Update `Cargo.toml` version
2. ✅ Update `Cargo.lock` if needed
3. ✅ Create a git commit with the version change
4. ✅ Create a git tag `v<version>`
5. ✅ Optionally push to origin (use `--execute` flag)

**Full release workflow:**
```bash
# Preview what will happen
cargo release --dry-run minor

# Execute the release (updates version, commits, tags)
cargo release minor --execute

# Push the commit and tag
git push origin main
git push origin --tags
```

### CI Validation & Publishing

The GitHub Actions workflow automatically:
- ✅ Updates `Cargo.toml` version to match the tag version (if needed)
- ✅ Commits the version update back to the repository
- ✅ Publishes to crates.io with the correct version
- ✅ Uses Trusted Publishing (no API tokens needed)

**Workflow:** Push tag → CI auto-updates version → Commits changes → Auto-publishes to crates.io

**Note:** If you push a tag like `v0.2.1` and `Cargo.toml` has version `0.2.0`, the workflow will automatically update `Cargo.toml` to `0.2.1` and commit the change before publishing.


## 🔍 Code Review Process

1. **Automated Checks**
   - Tests must pass
   - Clippy must pass
   - Formatting must be correct

2. **Review Criteria**
   - Code follows style guide
   - Tests are comprehensive
   - Documentation is complete
   - No new required dependencies added
   - Performance considered

3. **Review Feedback**
   - Address all comments
   - Update code as needed
   - Re-request review when ready

## 🎯 Task Priorities

### Priority Levels

- **Critical**: Must have for next release
- **High**: Important feature
- **Medium**: Nice to have
- **Low**: Future consideration

### Complexity Levels

- **Simple**: 1-2 days
- **Medium**: 3-5 days
- **Complex**: 1-2 weeks

## 📚 Resources

### Documentation
- `roadmap/TASKS.md` - Complete task list
- `roadmap/ROADMAP.md` - Strategic roadmap
- `roadmap/QUICK_START.md` - Quick start guide
- `README.md` - Project overview
- [docs.rs/redstr](https://docs.rs/redstr) - API documentation

### Getting Help
- GitHub Discussions
- Discord/Slack (when available)
- Create an issue for questions

## 🚀 Quick Start Examples

### Adding a New Transformation Function

1. Create function in appropriate module:
   ```rust
   // crates/redstr/src/transformations/cloudflare.rs
   /// Brief description.
   ///
   /// # Examples
   /// ```
   /// use redstr::new_function;
   /// let result = new_function("input");
   /// ```
   pub fn new_function(input: &str) -> String {
       // Implementation
   }
   ```

2. Add tests:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_new_function() {
           // Tests
       }
   }
   ```

3. Export in `crates/redstr/src/lib.rs`:
   ```rust
   pub use transformations::cloudflare::new_function;
   ```

4. Add to `TransformBuilder` if applicable

5. Create example in `examples/`

### Creating a New Integration

**First, check where your integration should live!**

See **[Integration Guidelines](docs/INTEGRATION_GUIDELINES.md)** for guidance on:
- When to use a separate repository (e.g., Burp Suite, OWASP ZAP add-ons)
- When to contribute to this repository (examples, docs)
- How to structure your integration
- Tool-specific guidance

**For integrations in this repo:**
1. Research integration points
2. Create `docs/{platform}_integration.md`
3. Add examples to `examples/` directory
4. Update documentation

**For separate repo integrations:**
1. Follow the [Integration Guidelines](docs/INTEGRATION_GUIDELINES.md)
2. Reference redstr in your documentation
3. Let us know so we can feature it!

## 🤝 Community Guidelines

- Be respectful and inclusive
- Help others learn
- Share knowledge
- Follow code of conduct

## 📊 Progress Tracking

- Update `roadmap/TASKS.md` with progress
- Use GitHub Projects for visualization
- Report blockers early
- Celebrate completions!

---

**Thank you for contributing to redstr!** 🎉
