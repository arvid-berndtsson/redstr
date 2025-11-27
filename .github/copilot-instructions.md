# GitHub Copilot Instructions for redstr

## 🚨 CRITICAL: Read the `.cursorrules` file

**ALL development rules, guidelines, and requirements are documented in the `.cursorrules` file at the root of this repository.**

This file contains comprehensive instructions that **MUST be followed strictly** by all AI agents, including:

- ✅ **Mandatory pre-commit checks**: `cargo fmt`, `cargo check`, `cargo clippy -- -D warnings`, `cargo test`
- ✅ **Conventional commits format**: All commits and PR titles MUST follow the format `[TASK-ID] <type>(<scope>): <subject>`
- ✅ **Zero dependencies principle**: The core library uses only Rust's standard library
- ✅ **Code quality standards**: Documentation, testing, and API design requirements
- ✅ **Security considerations**: Responsible use and ethical guidelines
- ✅ **Project-specific workflows**: Complete pre-commit checklist and development workflow

## Why This Matters

The `.cursorrules` file is currently **not being followed strictly**, which causes:
- CI failures due to incorrect commit message formats
- Code quality issues from skipped pre-commit checks
- Violations of the zero-dependency principle
- Missing or inadequate documentation and tests

## Quick Reference

Before making **any** changes:

1. **Read `.cursorrules` completely** - it contains all the rules
2. **Run pre-commit checks**:
   ```bash
   cargo fmt
   cargo check
   cargo clippy -- -D warnings
   cargo test
   ```
3. **Use conventional commits format**:
   - Format: `[TASK-ID] <type>(<scope>): <subject>`
   - Example: `[CF-001] feat(cloudflare): add Turnstile challenge variation`
   - Valid types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `perf`

## Enforcement

- CI will **FAIL** if rules from `.cursorrules` are not followed
- Commit messages are validated and will be rejected if incorrect
- Formatting, clippy warnings, and test failures will block merges
- The pre-commit hook automatically runs checks before each commit

## For AI Agents

When working on this repository:
- **Always** refer to `.cursorrules` as the source of truth
- **Never** skip the pre-commit checks listed above
- **Always** use conventional commits format for commits and PR titles
- **Follow** the zero-dependency principle strictly
- **Document** all public functions with examples
- **Test** all new code with comprehensive unit tests

---

**📖 Full documentation**: See [`.cursorrules`](../.cursorrules) in the repository root.
