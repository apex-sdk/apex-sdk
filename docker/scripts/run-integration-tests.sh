#!/bin/bash
# Run integration tests with Docker test nodes

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo "Running Apex SDK Integration Tests"
echo "======================================"

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo -e "${RED}Docker is not running. Please start Docker first.${NC}"
    exit 1
fi

# Start test nodes
echo -e "${YELLOW}Starting test nodes...${NC}"
"$SCRIPT_DIR/start-nodes.sh"

# Set environment variables for integration tests
export INTEGRATION_TESTS=1
export EVM_RPC_URL="http://localhost:8545"
export SUBSTRATE_RPC_URL="ws://localhost:9944"

cd "$PROJECT_ROOT"

# Run tests
echo ""
echo -e "${YELLOW}Running integration tests...${NC}"
echo ""

# Run all tests including ignored ones (integration tests)
if cargo test --workspace -- --include-ignored; then
    echo ""
    echo -e "${GREEN}All integration tests passed!${NC}"
    TEST_RESULT=0
else
    echo ""
    echo -e "${RED}Some integration tests failed${NC}"
    TEST_RESULT=1
fi

# Stop test nodes
echo ""
echo -e "${YELLOW}Stopping test nodes...${NC}"
"$SCRIPT_DIR/stop-nodes.sh"

exit $TEST_RESULT
