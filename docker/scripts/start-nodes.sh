#!/bin/bash
# Start all test nodes with Docker Compose

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo "Starting Apex SDK test nodes..."

cd "$PROJECT_ROOT"

# Build and start containers
docker compose up -d --build

echo "Waiting for nodes to be healthy..."

# Wait for EVM node
echo "Checking EVM node..."
timeout 60 bash -c 'until docker compose exec -T evm-node wget --spider -q http://localhost:8545 2>/dev/null; do sleep 2; done' || {
    echo "EVM node failed to start"
    docker compose logs evm-node
    exit 1
}
echo "EVM node is ready at http://localhost:8545"
# Wait for Substrate node
echo "Checking Substrate node..."
timeout 90 bash -c 'until curl -s http://localhost:9933/health >/dev/null 2>&1; do sleep 2; done' || {
    echo "Substrate node failed to start"
    docker compose logs substrate-node
    exit 1
}
echo "Substrate node is ready at ws://localhost:9944"
echo ""
echo "All test nodes are running!"
echo ""
echo "EVM Node:       http://localhost:8545"
echo "Substrate Node: ws://localhost:9944 (RPC: http://localhost:9933)"
echo ""
echo "To view logs:    docker compose logs -f"
echo "To stop nodes:   ./docker/scripts/stop-nodes.sh"
echo ""
