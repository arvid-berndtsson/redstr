#!/bin/bash
# Install git commit-msg hook for conventional commits validation
# This hook validates commit messages locally before they're committed

HOOK_FILE=".git/hooks/commit-msg"

cat > "$HOOK_FILE" << 'EOF'
#!/bin/bash
# Git commit-msg hook to validate conventional commits format

COMMIT_MSG_FILE=$1
COMMIT_MSG=$(cat "$COMMIT_MSG_FILE")
FIRST_LINE=$(echo "$COMMIT_MSG" | head -1)

# Pattern for conventional commits with optional task ID
PATTERN_WITH_TASK='^\[[A-Z]+-[0-9]{3}\][[:space:]]+[a-z]+\([^)]*\):[[:space:]]+.+'
PATTERN_WITHOUT_TASK='^[a-z]+\([^)]*\):[[:space:]]+.+'

if ! echo "$FIRST_LINE" | grep -qE "$PATTERN_WITH_TASK|$PATTERN_WITHOUT_TASK"; then
    echo "❌ Commit message does not follow conventional commits format!"
    echo ""
    echo "Expected format: [TASK-ID] <type>(<scope>): <subject>"
    echo "Or: <type>(<scope>): <subject>"
    echo ""
    echo "Valid types: feat, fix, docs, style, refactor, test, chore, perf"
    echo ""
    echo "Examples:"
    echo "  [CF-001] feat(cloudflare): add Turnstile challenge variation"
    echo "  fix(encoding): correct URL encoding for special characters"
    echo "  docs(readme): update installation instructions"
    echo ""
    echo "Your commit message was:"
    echo "  $FIRST_LINE"
    echo ""
    exit 1
fi

# Validate commit type
COMMIT_TYPE=$(echo "$FIRST_LINE" | sed -E 's/^\[[A-Z]+-[0-9]{3}\][[:space:]]+//; s/^([a-z]+)\(.*/\1/')
VALID_TYPES="feat fix docs style refactor test chore perf"

if ! echo "$VALID_TYPES" | grep -qw "$COMMIT_TYPE"; then
    echo "❌ Invalid commit type: $COMMIT_TYPE"
    echo ""
    echo "Valid types: feat, fix, docs, style, refactor, test, chore, perf"
    echo ""
    exit 1
fi

echo "✅ Commit message follows conventional commits format"
exit 0
EOF

chmod +x "$HOOK_FILE"
echo "✅ Git commit-msg hook installed successfully!"
echo ""
echo "The hook will now validate commit messages before they're committed."
echo "To bypass the hook (not recommended), use: git commit --no-verify"