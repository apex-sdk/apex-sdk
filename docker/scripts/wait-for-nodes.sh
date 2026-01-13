#!/bin/bash
# Wait for test nodes to be ready

set -e

echo "Waiting for test nodes to be ready..."

# Wait for EVM node
echo -n "EVM node... "
timeout 60 bash -c 'until curl -s -X POST -H "Content-Type: application/json" --data "{\"jsonrpc\":\"2.0\",\"method\":\"eth_blockNumber\",\"params\":[],\"id\":1}" http://localhost:8545 >/dev/null 2>&1; do sleep 1; done' && echo "Ready" || {
    echo "Failed"
    exit 1
}

# Wait for Substrate node
echo -n "Substrate node... "
timeout 90 bash -c 'until curl -s http://localhost:9933/health >/dev/null 2>&1; do sleep 1; done' && echo "Ready" || {
    echo "Failed"
    exit 1
}

echo "All nodes are ready!"
