# Pull Request

## ⚠️ IMPORTANT: PR Title Format

**PR titles MUST follow conventional commits format:**
- Format: `[TASK-ID] <type>(<scope>): <subject>`
- Examples:
  - `[CF-001] feat(cloudflare): add Turnstile challenge variation`
  - `fix(encoding): correct URL encoding for special characters`
  - `docs(readme): update installation instructions`
- **CI will fail if PR title doesn't follow this format**

## Task Information

**Task ID:** [e.g., CF-001]  
**Related Issue:** #[issue-number]  
**Phase:** [1/2/3/4]

## Description

[Brief description of changes]

## Changes

- [ ] Feature implementation
- [ ] Tests added
- [ ] Documentation updated
- [ ] Examples added
- [ ] Performance optimized

## Testing

- [ ] All tests pass (`cargo test`)
- [ ] New tests added for new features
- [ ] Examples run successfully
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Code formatted (`cargo fmt`)

## Checklist

- [ ] Code follows project style guide
- [ ] Zero dependencies maintained (core library)
- [ ] Documentation includes examples
- [ ] All public functions documented
- [ ] Tests cover edge cases
- [ ] Performance considered
- [ ] Security implications documented

## Breaking Changes

- [ ] No breaking changes
- [ ] Breaking changes documented

## Additional Notes

[Any additional context, screenshots, or notes]

---

**Reference:** See `TASKS.md` for task details.
