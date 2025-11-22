# Cursor Configuration

This directory contains Cursor-specific configuration files for the redstr project.

## Files

- `agent-instructions.md` - Detailed instructions for Cursor Cloud Agent about commit messages and PR titles
- `config.json` - Structured configuration for Cursor Cloud Agent

## Important Rules

**ALL commits and PR titles MUST follow conventional commits format:**

```
[TASK-ID] <type>(<scope>): <subject>
```

Or:

```
<type>(<scope>): <subject>
```

Valid types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `perf`

See `agent-instructions.md` for complete details and examples.
