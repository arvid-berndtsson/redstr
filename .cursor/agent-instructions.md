# Cursor Cloud Agent Instructions

## Commit Messages

Write clear, descriptive commit messages that explain what changed and why.

### Guidelines

- Use imperative mood: "add", "fix", "update" (not "added", "fixed", "updated")
- Be specific and concise
- Include task ID if available (e.g., `[CF-001]`)
- Explain the "why" in the body if the change is non-obvious
- Reference related issues or PRs when applicable

### Examples of Good Commit Messages

- `[CF-001] Add Cloudflare Turnstile challenge variation`
- `Fix URL encoding for special characters`
- `Update README with installation instructions`
- `Add tests for TransformBuilder`
- `Refactor encoding module for better performance`

### Examples of Poor Commit Messages (Avoid)

- ❌ `Update code`
- ❌ `Fix bug`
- ❌ `Add feature`
- ❌ `Changes`
- ❌ `WIP`
- ❌ `Update README` (too vague - what was updated?)

### PR Titles

PR titles should be clear and descriptive, similar to commit messages.

- Include task ID if available (e.g., `[CF-001]`)
- Be specific about what the PR does
- Use imperative mood

### Examples of Good PR Titles

- `[CF-001] Add Cloudflare Turnstile challenge variation`
- `Fix URL encoding for special characters`
- `Update README with installation instructions`

### Before Committing

1. Review your commit message
2. Ensure it clearly describes what changed
3. Add task ID if available
4. Include additional context in the body if needed

### Before Creating PR

1. Review your PR title
2. Ensure it clearly describes what the PR does
3. Add task ID if available
4. Use the PR template to provide additional context
