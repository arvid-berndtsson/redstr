# Conventional Commits Enforcement

This document explains how conventional commits are enforced in the redstr project.

## Enforcement Mechanisms

### 1. CI/CD Validation (Primary Enforcement) ✅

**Location:** `.github/workflows/commit-check.yml`

The GitHub Actions workflow automatically validates:
- **PR titles** - Must follow conventional commits format
- **Commit messages** - All commits in a PR must follow the format

**When it runs:**
- On every PR (opened, updated, synchronized)
- On every push to `main` or `develop` branches

**What happens if validation fails:**
- The CI check fails
- The PR cannot be merged until the commit messages/PR title are fixed

**This is the most reliable enforcement mechanism** and works for both local development and Cursor Cloud.

### 2. Cursor Rules (Guidance for AI Agents) 🤖

**Location:** `.cursorrules`, `.cursor/agent-instructions.md`, `.cursor/SYSTEM_INSTRUCTIONS.md`

These files provide clear instructions to Cursor Cloud agents about:
- The required format
- Valid types
- Examples of correct and incorrect messages
- Enforcement warnings

**Note:** Cursor Cloud agents will see these rules, but they are guidance only. The actual enforcement happens in CI.

### 3. Git Hook (Local Enforcement) 🔧

**Location:** `scripts/install-commit-hook.sh`

A git `commit-msg` hook can be installed locally to validate commit messages before they're committed.

**To install:**
```bash
./scripts/install-commit-hook.sh
```

**What it does:**
- Validates commit message format before commit is created
- Prevents commits that don't follow the format
- Provides helpful error messages

**Note:** This only works locally. Cursor Cloud's remote environment may not support git hooks.

**To bypass (not recommended):**
```bash
git commit --no-verify
```

## Conventional Commits Format

### Required Format

```
[TASK-ID] <type>(<scope>): <subject>
```

**OR (without task ID):**

```
<type>(<scope>): <subject>
```

### Valid Types

- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation changes
- `style` - Code style/formatting
- `refactor` - Code refactoring
- `test` - Adding or updating tests
- `chore` - Maintenance tasks
- `perf` - Performance improvements

### Examples

✅ **Correct:**
- `[CF-001] feat(cloudflare): add Turnstile challenge variation`
- `fix(encoding): correct URL encoding for special characters`
- `docs(readme): update installation instructions`
- `test(builder): add TransformBuilder tests`

❌ **Incorrect:**
- `Update code`
- `Fix bug`
- `Add feature`
- `Changes`
- `WIP`

## For Cursor Cloud Users

**Important:** Cursor Cloud agents will:
1. See the rules in `.cursorrules` and agent instruction files
2. Be guided to use the correct format
3. Have their commits validated in CI (which will fail if format is wrong)

**The CI validation is the primary enforcement** - even if an agent makes a mistake, CI will catch it and prevent merging.

## Troubleshooting

### My commit was rejected by CI

1. Check the CI error message - it will show what's wrong
2. Amend your commit message:
   ```bash
   git commit --amend -m "feat(scope): correct message"
   ```
3. Force push (if on a feature branch):
   ```bash
   git push --force-with-lease
   ```

### My PR title was rejected

1. Edit the PR title on GitHub
2. Or update it via command line:
   ```bash
   gh pr edit --title "feat(scope): correct title"
   ```

## Best Practices

1. **Always check your commit message before committing**
2. **Use descriptive scopes** (e.g., `cloudflare`, `encoding`, `builder`)
3. **Include task ID when available** (e.g., `[CF-001]`)
4. **Use imperative mood** in the subject (e.g., "add" not "added")
5. **Keep subjects concise** but descriptive

## Summary

- ✅ **CI/CD** - Primary enforcement (works everywhere)
- 🤖 **Cursor Rules** - Guidance for AI agents
- 🔧 **Git Hook** - Optional local enforcement

The CI validation ensures that all commits and PRs follow the conventional commits format, regardless of how they were created.
