#!/bin/bash
# Run integration tests that are normally ignored in CI
# These tests require network access and may take longer to run

set -e

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║        Apex SDK Integration Test Runner               ║${NC}"
echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo ""

# Parse command line arguments
TEST_SUITE="${1:-all}"

run_test_suite() {
    local suite_name="$1"
    local command="$2"

    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Running: ${suite_name}${NC}"
    echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    if eval "$command"; then
        echo -e "${GREEN}✓ ${suite_name} passed${NC}"
        return 0
    else
        echo -e "${RED}✗ ${suite_name} failed${NC}"
        return 1
    fi
}

echo -e "${YELLOW}Test Suite: ${TEST_SUITE}${NC}"
echo ""

case "$TEST_SUITE" in
    evm)
        echo "Running EVM integration tests..."
        run_test_suite "EVM Integration Tests" \
            "cargo test -p apex-sdk-evm --test integration_test -- --ignored --test-threads=1"
        run_test_suite "EVM Library Tests" \
            "cargo test -p apex-sdk-evm --lib -- --ignored --test-threads=1"
        ;;

    substrate)
        echo "Running Substrate integration tests..."
        run_test_suite "Substrate Library Tests" \
            "cargo test -p apex-sdk-substrate --lib -- --ignored --test-threads=1"
        ;;

    westend)
        echo "Running Westend testnet integration tests..."
        run_test_suite "Westend Integration Tests" \
            "cargo test -p apex-sdk-substrate --test westend_integration -- --ignored --test-threads=1"
        ;;

    cli)
        echo "Running CLI integration tests..."
        run_test_suite "CLI Integration Tests" \
            "cargo test -p apex-sdk-cli -- --ignored --test-threads=1"
        ;;

    all)
        echo "Running ALL integration tests (this may take a while)..."
        echo ""

        FAILED=0

        run_test_suite "EVM Integration Tests" \
            "cargo test -p apex-sdk-evm --test integration_test -- --ignored --test-threads=1" || FAILED=$((FAILED + 1))

        run_test_suite "EVM Library Tests" \
            "cargo test -p apex-sdk-evm --lib -- --ignored --test-threads=1" || FAILED=$((FAILED + 1))

        run_test_suite "Substrate Library Tests" \
            "cargo test -p apex-sdk-substrate --lib -- --ignored --test-threads=1" || FAILED=$((FAILED + 1))

        run_test_suite "Westend Integration Tests" \
            "cargo test -p apex-sdk-substrate --test westend_integration -- --ignored --test-threads=1" || FAILED=$((FAILED + 1))

        run_test_suite "CLI Integration Tests" \
            "cargo test -p apex-sdk-cli -- --ignored --test-threads=1" || FAILED=$((FAILED + 1))

        echo ""
        echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        if [ $FAILED -eq 0 ]; then
            echo -e "${GREEN}✓ All integration test suites passed!${NC}"
        else
            echo -e "${RED}✗ $FAILED test suite(s) failed${NC}"
            echo -e "${YELLOW}Note: Failures may be due to network issues or endpoint rate limiting${NC}"
        fi
        echo -e "${YELLOW}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
        ;;

    quick)
        echo "Running a quick sample of integration tests..."
        run_test_suite "Quick Integration Sample" \
            "cargo test --all-features -- --ignored test_get_balance test_connect --test-threads=1"
        ;;

    *)
        echo -e "${RED}Unknown test suite: $TEST_SUITE${NC}"
        echo ""
        echo "Usage: $0 [test-suite]"
        echo ""
        echo "Available test suites:"
        echo "  all        - Run all integration tests (default)"
        echo "  evm        - Run EVM integration tests only"
        echo "  substrate  - Run Substrate integration tests only"
        echo "  westend    - Run Westend testnet tests only"
        echo "  cli        - Run CLI integration tests only"
        echo "  quick      - Run a quick sample of integration tests"
        echo ""
        echo "Examples:"
        echo "  $0              # Run all tests"
        echo "  $0 evm          # Run only EVM tests"
        echo "  $0 quick        # Run quick sample"
        exit 1
        ;;
esac

echo ""
echo -e "${BLUE}Integration test run complete${NC}"
