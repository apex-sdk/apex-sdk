#!/bin/bash
# Stop all test nodes

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo "Stopping Apex SDK test nodes..."

cd "$PROJECT_ROOT"

# Stop and remove containers
docker compose down

echo "All test nodes stopped"
