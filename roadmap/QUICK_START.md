# redstr Quick Start Guide
## For Developers and AI Agents

**Last Updated:** November 22, 2025

---

## 🚀 Quick Start (5 Minutes)

### 1. Find a Task
```bash
# Open TASKS.md and look for:
# - Tasks with ⬜ Not Started
# - Tasks tagged [AGENT-FRIENDLY] (for AI agents)
# - Tasks matching your skills
```

### 2. Claim the Task
- Comment on task or create GitHub issue
- Update status: ⬜ → 🟡
- Assign yourself

### 3. Set Up
```bash
git clone https://github.com/arvid-berndtsson/redstr.git
cd redstr
cargo build
cargo test
```

### 4. Create Branch
```bash
git checkout -b task/{TASK-ID}-{short-description}
# Example: task/CF-001-cloudflare-turnstile
```

### 5. Work on Task
- Follow task requirements in `TASKS.md`
- Write code, tests, documentation
- Run `cargo test` and `cargo clippy`

### 6. Submit PR
- Push branch
- Create PR with `[TASK-ID]` in title
- Use PR template
- Update task status: 🟡 → ✅

---

## 📋 Task Status Symbols

- ⬜ **Not Started** - Available
- 🟡 **In Progress** - Being worked on
- ✅ **Completed** - Done
- 🔄 **Blocked** - Waiting
- ❌ **Cancelled** - Not needed

---

## 🎯 For AI Agents

### Finding Tasks
1. Search `TASKS.md` for `[AGENT-FRIENDLY]` tag
2. Check task has no assignee
3. Verify dependencies are met
4. Check complexity matches agent capabilities

### Task Format
Each task includes:
- **ID**: Unique identifier (e.g., `CF-001`)
- **Priority**: Critical/High/Medium/Low
- **Complexity**: Simple/Medium/Complex
- **Requirements**: What to build
- **Deliverables**: What to create
- **Acceptance Criteria**: How to verify
- **Function Signatures**: Code templates

### Creating PRs
```markdown
Title: [CF-001] Add Cloudflare Turnstile variation

Body: Use PR template from .github/PULL_REQUEST_TEMPLATE.md
```

---

## 🛠️ Common Tasks

### Adding a New Function

1. **Create function** in appropriate module:
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
       // Implementation using only std
   }
   ```

2. **Add tests**:
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;
       
       #[test]
       fn test_new_function() {
           assert_eq!(new_function("test"), "expected");
       }
   }
   ```

3. **Export** in [`crates/redstr/src/lib.rs`](../crates/)redstr/src/lib.rs):
   ```rust
   pub use transformations::cloudflare::new_function;
   ```

4. **Add to builder** (if applicable) in [`crates/redstr/src/builder.rs`](../crates/)redstr/src/builder.rs)

5. **Create example** in [`crates/redstr/examples/`](../crates/)redstr/examples/)

### Running Checks
```bash
cargo fmt          # Format code
cargo clippy       # Check for issues
cargo test         # Run tests
cargo doc --open   # Build and view docs
```

---

## ⚠️ Critical Rules

### Minimal Dependencies
- ✅ Core library: `std` only (no required dependencies)
- ✅ Optional features: Behind feature flags (e.g., serde)
- ✅ Dev-dependencies: OK for tests/benchmarks (e.g., cc-check)
- ❌ Never add required dependencies to core

### Code Quality
- ✅ All tests pass
- ✅ No clippy warnings
- ✅ Code formatted
- ✅ Documentation complete

### Task Workflow
- ✅ Update status in `TASKS.md`
- ✅ Include task ID in PR title
- ✅ Use PR template
- ✅ Check dependencies first

---

## 📚 Key Files

- `TASKS.md` - Complete task list
- `ROADMAP.md` - Strategic roadmap
- [`CONTRIBUTING.md`](CONTRIBUTING.md) - Full contribution guide
- [`README.md`](README.md) - Project overview

---

## 🆘 Getting Help

1. Check [`CONTRIBUTING.md`](CONTRIBUTING.md) for detailed guidelines
2. Look at existing code for examples
3. Check `TASKS.md` for task details
4. Create GitHub issue for questions

---

## ✅ Checklist Before PR

- [ ] Task ID in PR title: `[TASK-ID] Description`
- [ ] All tests pass: `cargo test`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Code formatted: `cargo fmt`
- [ ] Documentation updated
- [ ] Examples added (if applicable)
- [ ] No new required dependencies added
- [ ] Task status updated in `TASKS.md`

---

**Ready to contribute? Check `TASKS.md` for available tasks!** 🚀
