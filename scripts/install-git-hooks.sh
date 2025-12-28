#!/bin/bash
# Install git hooks for the apex-sdk project
# Run this script to set up pre-commit checks

set -e

REPO_ROOT=$(git rev-parse --show-toplevel 2>/dev/null || echo ".")
HOOKS_DIR="$REPO_ROOT/.git/hooks"

echo "📦 Installing git hooks for apex-sdk..."
echo ""

# Create pre-commit hook
cat > "$HOOKS_DIR/pre-commit" << 'EOF'
#!/bin/bash
# Pre-commit hook to run Clippy checks matching CI requirements
# This prevents CI failures by catching issues locally
#
# To bypass this hook (use sparingly!): git commit --no-verify

set -e

echo "🔍 Running pre-commit checks..."
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if we're in a rebase/merge
if [ -f .git/MERGE_HEAD ] || [ -f .git/REBASE_HEAD ]; then
    echo -e "${YELLOW}⚠️  Skipping pre-commit hook (merge/rebase in progress)${NC}"
    exit 0
fi

# Function to run check with timing
run_check() {
    local name=$1
    local cmd=$2

    echo -e "${YELLOW}▶ ${name}...${NC}"
    start=$(date +%s)

    if eval "$cmd" > /tmp/precommit-output 2>&1; then
        end=$(date +%s)
        duration=$((end - start))
        echo -e "${GREEN}✓ ${name} passed (${duration}s)${NC}"
        return 0
    else
        end=$(date +%s)
        duration=$((end - start))
        echo -e "${RED}✗ ${name} failed (${duration}s)${NC}"
        echo ""
        cat /tmp/precommit-output
        return 1
    fi
}

# Track if any checks fail
failed=0

# 1. Format check (fast)
if ! run_check "Format check" "cargo fmt --all -- --check"; then
    echo -e "${YELLOW}💡 Tip: Run 'cargo fmt' to fix formatting${NC}"
    echo ""
    failed=1
fi

# 2. Clippy check (matches CI exactly)
if ! run_check "Clippy lint" "cargo clippy --all-features --all-targets -- -D warnings"; then
    echo -e "${YELLOW}💡 Tip: Fix clippy warnings shown above${NC}"
    echo ""
    failed=1
fi

# 3. Quick compile check
if ! run_check "Build check" "cargo check --all-features"; then
    echo -e "${YELLOW}💡 Tip: Fix compilation errors shown above${NC}"
    echo ""
    failed=1
fi

# Cleanup
rm -f /tmp/precommit-output

# Summary
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ $failed -eq 0 ]; then
    echo -e "${GREEN}✓ All pre-commit checks passed!${NC}"
    echo -e "${GREEN}✓ Ready to commit${NC}"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    exit 0
else
    echo -e "${RED}✗ Pre-commit checks failed${NC}"
    echo ""
    echo "Fix the issues above, or use 'git commit --no-verify' to skip"
    echo "(but your CI will likely fail!)"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    exit 1
fi
EOF

chmod +x "$HOOKS_DIR/pre-commit"

echo "✅ Pre-commit hook installed successfully!"
echo ""
echo "The hook will run these checks before each commit:"
echo "  • Format check (cargo fmt)"
echo "  • Clippy lint (matches CI exactly)"
echo "  • Build check (cargo check)"
echo ""
echo "To bypass the hook (not recommended):"
echo "  git commit --no-verify"
echo ""
echo "To uninstall:"
echo "  rm .git/hooks/pre-commit"
