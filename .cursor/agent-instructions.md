# Cursor Cloud Agent Instructions

## ⚠️ CRITICAL: COMMIT MESSAGES AND PR TITLES

**YOU MUST ALWAYS follow conventional commits format for ALL commits and PR titles.**

### Commit Message Format

**REQUIRED FORMAT:**
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
- `style` - Code style changes (formatting, etc.)
- `refactor` - Code refactoring
- `test` - Adding or updating tests
- `chore` - Maintenance tasks
- `perf` - Performance improvements

### Examples of CORRECT Commit Messages
- `[CF-001] feat(cloudflare): add Turnstile challenge variation`
- `fix(encoding): correct URL encoding for special characters`
- `docs(readme): update installation instructions`
- `test(builder): add tests for TransformBuilder`
- `chore(deps): update dependencies`

### Examples of INCORRECT Commit Messages (DO NOT USE)
- ❌ `Update code`
- ❌ `Fix bug`
- ❌ `Add feature`
- ❌ `Changes`
- ❌ `WIP`
- ❌ `Update README`
- ❌ `Fix issue`

### PR Title Format

**PR titles MUST follow the SAME format as commit messages:**
- `[CF-001] feat(cloudflare): add Turnstile challenge variation`
- `fix(encoding): correct URL encoding for special characters`
- `docs(readme): update installation instructions`

### Before Every Commit

1. **STOP** and check your commit message
2. Verify it matches: `[TASK-ID] <type>(<scope>): <subject>` OR `<type>(<scope>): <subject>`
3. Ensure the type is one of: feat, fix, docs, style, refactor, test, chore, perf
4. If it doesn't match, **REWRITE IT** before committing

### Before Creating Every PR

1. **STOP** and check your PR title
2. Verify it matches the conventional commits format
3. If it doesn't match, **REWRITE IT** before creating the PR

### Enforcement

- CI will **FAIL** if commits don't follow this format
- CI will **FAIL** if PR titles don't follow this format
- Git hooks will **BLOCK** commits that don't follow this format
- **There are NO exceptions to this rule**

### How to Write a Good Commit Message

1. Identify the type: What kind of change is this?
   - New feature? → `feat`
   - Bug fix? → `fix`
   - Documentation? → `docs`
   - Tests? → `test`
   - etc.

2. Identify the scope: What part of the codebase?
   - `encoding` - encoding functions
   - `cloudflare` - cloudflare functions
   - `builder` - TransformBuilder
   - `readme` - README.md
   - `ci` - CI/CD
   - etc.

3. Write a clear subject: What did you do?
   - Use imperative mood: "add", "fix", "update", not "added", "fixed", "updated"
   - Be specific but concise
   - Start with lowercase

4. Add task ID if available: `[CF-001]`

### Example Workflow

**Before committing:**
```
Current commit message: "Update README"
❌ This is WRONG!

Correct commit message: "docs(readme): update installation instructions"
✅ This is CORRECT!
```

**Before creating PR:**
```
Current PR title: "Add new feature"
❌ This is WRONG!

Correct PR title: "feat(cloudflare): add Turnstile challenge variation"
✅ This is CORRECT!
```
