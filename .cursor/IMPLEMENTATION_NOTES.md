# Implementation Notes: Conventional Commits Enforcement

## What Was Implemented

This implementation enforces conventional commits format for Cursor Cloud Agent through multiple layers:

### 1. Documentation Files

- **`.cursorrules`** - Enhanced with very prominent warnings at the top
  - Multiple warning sections
  - Clear examples of correct/incorrect formats
  - Explicit instructions for AI agents

- **`.cursor/agent-instructions.md`** - Detailed step-by-step instructions
  - Complete format specification
  - Examples of correct and incorrect messages
  - Workflow guidance

- **`.cursor/SYSTEM_INSTRUCTIONS.md`** - System-level instructions
  - Concise format reference
  - Quick validation checklist

- **`.cursor/config.json`** - Structured configuration
  - Machine-readable format
  - Rules and examples in JSON

### 2. Enforcement Mechanisms

- **Git Hook** (`.git/hooks/commit-msg`)
  - Validates commit messages locally
  - Blocks commits that don't follow format
  - Executable and active

- **CI Workflow** (`.github/workflows/commit-check.yml`)
  - Validates commit messages in PRs
  - Validates PR titles
  - Fails CI if format is incorrect

### 3. Updated Files

- `.cursorrules` - Added prominent warnings
- `.github/workflows/commit-check.yml` - Added PR title validation
- `.github/PULL_REQUEST_TEMPLATE.md` - Added PR title format reminder
- `CONTRIBUTING.md` - Added reference to Cursor-specific instructions

## How Cursor Cloud Agent Should Use This

1. **Primary Source:** `.cursorrules` (read automatically by Cursor)
   - Very prominent warnings at the top
   - Clear format specification
   - Examples provided

2. **Detailed Instructions:** `.cursor/agent-instructions.md`
   - Step-by-step guidance
   - Workflow examples
   - Common mistakes to avoid

3. **Quick Reference:** `.cursor/SYSTEM_INSTRUCTIONS.md`
   - Concise format reference
   - Quick validation checklist

4. **Structured Config:** `.cursor/config.json`
   - Machine-readable format
   - Can be parsed programmatically

## Testing the Enforcement

To test if the enforcement is working:

1. **Test Git Hook:**
   ```bash
   git commit -m "test commit"  # Should fail
   git commit -m "chore(test): test commit"  # Should succeed
   ```

2. **Test CI:**
   - Create a PR with incorrect title format
   - CI should fail with clear error message

## Troubleshooting

If Cursor Cloud Agent is still not following the format:

1. **Check if `.cursorrules` is being read:**
   - The file should be in the root directory
   - Cursor should automatically read it

2. **Verify git hook is active:**
   ```bash
   ls -la .git/hooks/commit-msg
   # Should show executable file
   ```

3. **Check CI workflow:**
   - Verify `.github/workflows/commit-check.yml` exists
   - Check that it runs on PR events

4. **Add more explicit instructions:**
   - Consider adding instructions in the task description
   - Add reminders in code comments if needed

## Additional Recommendations

1. **In Task Descriptions:** Always include a reminder about commit format
2. **In Code Comments:** Add reminders for AI agents when relevant
3. **In PR Templates:** Already updated with format reminder
4. **Monitor CI Failures:** Check if commits are being rejected and why

## Format Reference

**Required Format:**
```
[TASK-ID] <type>(<scope>): <subject>
```

**Valid Types:**
- feat, fix, docs, style, refactor, test, chore, perf

**Examples:**
- ✅ `[CF-001] feat(cloudflare): add Turnstile challenge variation`
- ✅ `fix(encoding): correct URL encoding`
- ❌ `Update code` (WRONG)
- ❌ `Fix bug` (WRONG)
