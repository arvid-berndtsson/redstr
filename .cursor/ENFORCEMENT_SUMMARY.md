# Conventional Commits Enforcement Summary

## What's Enforced

1. **Git Hook** (`.git/hooks/commit-msg`)
   - Blocks commits locally if they don't follow conventional commits format
   - Runs automatically on every commit attempt
   - Provides clear error messages

2. **CI Workflow** (`.github/workflows/commit-check.yml`)
   - Validates all commit messages in PRs
   - Validates PR titles
   - Fails the CI if format is incorrect
   - Uses `cc-check` tool for validation

3. **Documentation**
   - `.cursorrules` - Main rules file (read by Cursor)
   - `.cursor/agent-instructions.md` - Detailed instructions for AI agents
   - `.cursor/SYSTEM_INSTRUCTIONS.md` - System-level instructions
   - `.cursor/config.json` - Structured configuration

## Format Required

```
[TASK-ID] <type>(<scope>): <subject>
```

OR:

```
<type>(<scope>): <subject>
```

## Valid Types

- `feat` - New feature
- `fix` - Bug fix
- `docs` - Documentation
- `style` - Code style/formatting
- `refactor` - Code refactoring
- `test` - Tests
- `chore` - Maintenance
- `perf` - Performance

## Examples

✅ **CORRECT:**
- `[CF-001] feat(cloudflare): add Turnstile challenge variation`
- `fix(encoding): correct URL encoding for special characters`
- `docs(readme): update installation instructions`

❌ **INCORRECT:**
- `Update code`
- `Fix bug`
- `Add feature`
- `Changes`
- `WIP`

## How It Works

1. **Local (Git Hook):** When you try to commit, the hook validates the message
2. **CI (GitHub Actions):** When you push or create a PR, CI validates all commits and PR title
3. **Both will FAIL if format is incorrect**

## For Cursor Cloud Agent

- Read `.cursorrules` at the top (very prominent warnings)
- Read `.cursor/agent-instructions.md` for detailed guidance
- Read `.cursor/SYSTEM_INSTRUCTIONS.md` for system-level rules
- Check `.cursor/config.json` for structured config

**Remember: There are NO exceptions to this rule!**
