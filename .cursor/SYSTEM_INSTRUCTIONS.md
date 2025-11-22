# SYSTEM INSTRUCTIONS FOR CURSOR CLOUD AGENT

## MANDATORY: COMMIT MESSAGE FORMAT

**YOU MUST ALWAYS USE THIS FORMAT FOR COMMITS:**

```
[TASK-ID] <type>(<scope>): <subject>
```

**OR:**

```
<type>(<scope>): <subject>
```

**VALID TYPES ONLY:**
- feat
- fix
- docs
- style
- refactor
- test
- chore
- perf

**EXAMPLES:**
- ✅ `[CF-001] feat(cloudflare): add Turnstile challenge variation`
- ✅ `fix(encoding): correct URL encoding`
- ✅ `docs(readme): update installation`
- ❌ `Update code` (WRONG)
- ❌ `Fix bug` (WRONG)
- ❌ `Add feature` (WRONG)

## MANDATORY: PR TITLE FORMAT

**PR TITLES MUST USE THE SAME FORMAT AS COMMITS**

## ENFORCEMENT

- CI will FAIL if format is wrong
- Git hooks will BLOCK commits
- NO EXCEPTIONS

## BEFORE COMMITTING

1. Check your commit message matches the format
2. If not, REWRITE IT
3. Verify the type is valid

## BEFORE CREATING PR

1. Check your PR title matches the format
2. If not, REWRITE IT
