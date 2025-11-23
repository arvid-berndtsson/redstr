# Multi-Project Workflow Instructions

## Overview

This workspace may contain multiple related projects (e.g., `redstr`, `redstr-server`, `redstr-burp`, etc.). This document provides instructions for working with multiple projects simultaneously.

## Project Structure

### Current Projects
- **redstr** - Main library (located at `/workspace`)
- **redstr-server** - HTTP API server (may be in subdirectory or separate repo)
- **redstr-burp** - Burp Suite extension
- **redstr-caido** - Caido plugin
- **redstr-eviljinx** - EvilJinx integration
- **redstr-owasp-zap** - OWASP ZAP add-on

## Working with Multiple Projects

### 1. Discovering Projects

**Check for existing projects:**
```bash
# Find all Cargo.toml files (Rust projects)
find /workspace -name "Cargo.toml" -type f

# Find all directories that might be projects
ls -d /workspace/*/

# Check for git repositories
find /workspace -name ".git" -type d
```

### 2. Navigating Between Projects

**Always use absolute paths or cd explicitly:**
```bash
# Navigate to a project
cd /workspace/redstr-server

# Or if in a subdirectory
cd /workspace/projects/redstr-server

# Check current location
pwd
```

**When working with multiple projects:**
- Always verify your current directory before making changes
- Use absolute paths in file operations when switching between projects
- Keep track of which project you're working on

### 3. Working with Project-Specific Code

**Before making changes:**
1. Navigate to the project directory: `cd /workspace/redstr-server`
2. Verify you're in the right place: `pwd && ls -la`
3. Check the project structure: `cat Cargo.toml` or `read_file("Cargo.toml")`
4. Understand the project's dependencies and structure

**Making changes:**
- Use relative paths from the project root when in that project
- Use absolute paths when switching between projects
- Always verify file paths before editing

### 4. Git Operations for Multiple Projects

**Each project may have its own git repository:**

```bash
# Check if project has its own .git
cd /workspace/redstr-server
if [ -d ".git" ]; then
    echo "Has own git repo"
    git status
    git remote -v
else
    echo "Part of parent repo or separate setup needed"
fi
```

**Creating branches:**
- Use project-specific branch names: `feat/redstr-server-api-endpoints`
- Or include project in branch: `redstr-server/feat/api-endpoints`
- Always create branches from the correct project directory

**Commits:**
- Always commit from the project's root directory
- Use conventional commits format with project-specific scope:
  - `feat(server): add POST /transform endpoint`
  - `fix(burp): correct extension initialization`
  - `docs(caido): update plugin documentation`

### 5. Building and Testing Multiple Projects

**For each project:**
```bash
# Navigate to project
cd /workspace/redstr-server

# Build
cargo build

# Test
cargo test

# Format
cargo fmt

# Clippy
cargo clippy -- -D warnings
```

**When working with dependencies:**
- If `redstr-server` depends on `redstr`, ensure `redstr` is published or use path dependency
- Check `Cargo.toml` for dependency configuration
- Update dependencies as needed

### 6. Creating Pull Requests

**For each project:**
1. Navigate to project root: `cd /workspace/redstr-server`
2. Create feature branch: `git checkout -b feat/server-new-endpoint`
3. Make changes and commit (following conventional commits)
4. Push branch: `git push origin feat/server-new-endpoint`
5. PR title must follow conventional commits: `feat(server): add new endpoint`

**PR Title Format (per project):**
- `feat(server): add POST /transform endpoint` - for redstr-server
- `fix(burp): correct extension initialization` - for redstr-burp
- `docs(caido): update plugin documentation` - for redstr-caido

### 7. Project-Specific Rules

**Each project may have:**
- Different `Cargo.toml` configurations
- Different dependency requirements
- Different CI/CD workflows
- Different commit message conventions (though all should follow conventional commits)

**Always check:**
- `.cursorrules` in project directory (if exists)
- `CONTRIBUTING.md` in project directory
- `.github/workflows/` for CI requirements
- Project-specific README.md

### 8. Common Workflows

#### Adding a Feature to redstr-server

```bash
# 1. Navigate to project
cd /workspace/redstr-server

# 2. Verify structure
pwd
cat Cargo.toml

# 3. Create branch
git checkout -b feat/server-new-feature

# 4. Make changes
# ... edit files ...

# 5. Test
cargo test
cargo clippy -- -D warnings
cargo fmt

# 6. Commit (from project root)
git add .
git commit -m "feat(server): add new feature"

# 7. Push
git push origin feat/server-new-feature
```

#### Updating redstr and redstr-server Together

```bash
# 1. Update redstr first
cd /workspace
git checkout -b feat/redstr-new-transformation
# ... make changes ...
cargo test
git commit -m "feat(transformations): add new transformation"
git push origin feat/redstr-new-transformation

# 2. Update redstr-server to use new feature
cd /workspace/redstr-server
git checkout -b feat/server-use-new-transformation
# ... update code to use new transformation ...
cargo test
git commit -m "feat(server): use new transformation from redstr"
git push origin feat/server-use-new-transformation
```

### 9. Best Practices

1. **Always verify location**: Use `pwd` before making changes
2. **Use absolute paths**: When switching between projects, use `/workspace/project-name/`
3. **Project-specific commits**: Include project context in commit scope
4. **Test each project**: Run tests in the project you're modifying
5. **Check dependencies**: Ensure dependent projects still work after changes
6. **Document changes**: Update relevant README files for each project
7. **Follow project conventions**: Each project may have specific rules

### 10. Troubleshooting

**Problem: Can't find project**
```bash
# Search for it
find /workspace -name "Cargo.toml" -type f
find /workspace -type d -name "*server*"
```

**Problem: Wrong project directory**
```bash
# Always check current directory
pwd
ls -la
cat Cargo.toml  # Verify project name
```

**Problem: Git operations in wrong repo**
```bash
# Check which repo you're in
git remote -v
git status
```

**Problem: Dependencies not found**
```bash
# Check if parent project needs to be built first
cd /workspace
cargo build --release

# Or check if path dependency is needed
cd /workspace/redstr-server
cat Cargo.toml  # Check dependency configuration
```

## Quick Reference

### Commands for Multi-Project Work

```bash
# Find all projects
find /workspace -name "Cargo.toml" -type f

# Navigate to project
cd /workspace/redstr-server

# Verify location
pwd && cat Cargo.toml | grep "^name"

# Check git status
git status

# Build and test
cargo build && cargo test && cargo clippy -- -D warnings

# Format code
cargo fmt

# Commit (from project root)
git commit -m "feat(scope): description"

# Push branch
git push origin branch-name
```

### Commit Message Scopes by Project

- **redstr**: `encoding`, `cloudflare`, `builder`, `transformations`, `obfuscation`, etc.
- **redstr-server**: `server`, `api`, `endpoint`, `handler`, `middleware`
- **redstr-burp**: `burp`, `extension`, `tab`, `intruder`
- **redstr-caido**: `caido`, `plugin`, `workflow`
- **redstr-eviljinx**: `eviljinx`, `script`, `integration`
- **redstr-owasp-zap**: `zap`, `addon`, `scanner`

## Notes

- Always use conventional commits format for ALL projects
- Each project should be tested independently
- Dependencies between projects should be managed carefully
- PR titles must follow conventional commits format
- CI will validate commits and PR titles for each project
