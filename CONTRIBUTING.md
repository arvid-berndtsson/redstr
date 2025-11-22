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

### Zero Dependencies Principle

**CRITICAL:** The core library must use only Rust's standard library.

- ✅ Use `std` only for core functionality
- ✅ Optional features (like `serde`) behind feature flags
- ❌ Never add dependencies to core library
- ✅ Dev-dependencies (like `criterion`) are OK for benchmarks

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

**We enforce conventional commits using `cc-check`.**

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

Commit messages are automatically validated in CI. The format must be:

```
[TASK-ID] <type>(<scope>): <subject>
```

Where:
- `TASK-ID` is optional but recommended (e.g., `CF-001`)
- `type` is required (feat, fix, docs, etc.)
- `scope` is optional (module name)
- `subject` is required (brief description)

## 🔍 Code Review Process

1. **Automated Checks**
   - Tests must pass
   - Clippy must pass
   - Formatting must be correct

2. **Review Criteria**
   - Code follows style guide
   - Tests are comprehensive
   - Documentation is complete
   - Zero dependencies maintained
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
   // src/transformations/cloudflare.rs
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

3. Export in `src/lib.rs`:
   ```rust
   pub use transformations::cloudflare::new_function;
   ```

4. Add to `TransformBuilder` if applicable

5. Create example in `examples/`

### Creating a New Integration

1. Research integration points
2. Create `docs/{platform}_integration.md`
3. Create `integrations/{platform}/` directory
4. Implement integration
5. Add examples and documentation

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
