# Docker Integration Test Infrastructure

This directory contains Docker configurations and scripts for running integration tests against local blockchain nodes.

## Overview

The integration test infrastructure provides:

- **EVM Test Node**: Hardhat network running in Docker
- **Substrate Test Node**: substrate-contracts-node running in Docker
- **Automated Scripts**: Helper scripts for managing test nodes
- **CI Integration**: GitHub Actions workflows for daily automated testing

## Quick Start

### Prerequisites

- Docker 20.10 or later
- Docker Compose v2.0 or later
- Rust 1.85 or later

### Start Test Nodes

```bash
# Start both EVM and Substrate nodes
./docker/scripts/start-nodes.sh

# Or use docker compose directly
docker compose up -d
```

### Run Integration Tests

```bash
# Run EVM integration tests
INTEGRATION_TESTS=1 cargo test --test evm_integration_test -- --include-ignored

# Run Substrate integration tests
INTEGRATION_TESTS=1 cargo test --test substrate_integration_test -- --include-ignored

# Run all integration tests with helper script
./docker/scripts/run-integration-tests.sh
```

### Stop Test Nodes

```bash
# Stop all nodes
./docker/scripts/stop-nodes.sh

# Or use docker compose directly
docker compose down
```

## Test Node Details

### EVM Node (Hardhat)

- **Image**: Node.js 22 Alpine with Hardhat
- **Port**: 8545 (HTTP RPC)
- **Chain ID**: 31337
- **Test Accounts**: 20 pre-funded accounts with 10,000 ETH each
- **Mnemonic**: `test test test test test test test test test test test junk`

**Test Account Addresses:**
- Account 0: `0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266`
- Account 1: `0x70997970C51812dc3A010C7d01b50e0d17dc79C8`
- Account 2: `0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC`
- ... (17 more accounts available)

### Substrate Node (contracts-node)

- **Image**: Ubuntu 22.04 with substrate-contracts-node v0.42.0
- **Ports**:
  - 9944 (WebSocket RPC)
  - 9933 (HTTP RPC)
- **Mode**: Development (`--dev` with `--tmp` for clean state)
- **Features**: Full smart contract support via pallet-contracts

## Directory Structure

```
docker/
├── README.md                           # This file
├── evm/
│   ├── Dockerfile                      # EVM node container
│   └── hardhat.config.js              # Hardhat configuration
├── substrate/
│   └── Dockerfile                      # Substrate node container
└── scripts/
    ├── start-nodes.sh                  # Start all test nodes
    ├── stop-nodes.sh                   # Stop all test nodes
    ├── wait-for-nodes.sh              # Wait for nodes to be ready
    └── run-integration-tests.sh       # Run complete test suite
```

## Integration Test Files

Located in the `tests/` directory at the project root:

- `tests/integration_helpers.rs` - Shared helper functions and utilities
- `tests/evm_integration_test.rs` - EVM-specific integration tests
- `tests/substrate_integration_test.rs` - Substrate-specific integration tests

## CI/CD Integration

### Integration Tests Workflow

Located at `.github/workflows/integration-tests.yml`

**Schedule**: Daily at 2:00 AM UTC

**Jobs**:
- `docker-integration-tests` - Tests against Docker nodes
- `network-integration-tests` - Tests against live networks
- `all-ignored-tests` - Comprehensive test run

**Triggers**:
- Daily scheduled run
- Manual workflow dispatch
- Push to main branch (when relevant files change)

### Daily Health Check Workflow

Located at `.github/workflows/daily-health-check.yml`

**Schedule**: Daily at 6:00 AM UTC

**Jobs**:
- Build health check
- Test suite health check
- Docker infrastructure health check
- Dependency health check
- Security audit
- Coverage analysis

## Environment Variables

### Test Control

- `INTEGRATION_TESTS=1` - Enable integration tests (required for `#[ignore]` tests)

### Node URLs (optional overrides)

- `EVM_RPC_URL` - Override EVM node URL (default: `http://localhost:8545`)
- `SUBSTRATE_RPC_URL` - Override Substrate node URL (default: `ws://localhost:9944`)

## Troubleshooting

### EVM Node Issues

**Container exits immediately:**
```bash
# Check logs
docker logs apex-evm-node

# Common issues:
# - Port 8545 already in use
# - Hardhat configuration syntax error
```

**Solution:**
```bash
# Check what's using port 8545
lsof -i :8545

# Rebuild container
docker compose up -d --build evm-node
```

### Substrate Node Issues

**Node shows as unhealthy:**
```bash
# Check health endpoint
curl http://localhost:9933/health

# Check logs
docker logs apex-substrate-node
```

**Common issues:**
- Node starting slowly (normal, can take 30-60 seconds)
- Health check timing too aggressive
- Port 9944 or 9933 already in use

### General Debugging

**View all container logs:**
```bash
docker compose logs

# Follow logs in real-time
docker compose logs -f

# View specific service
docker compose logs evm-node
docker compose logs substrate-node
```

**Check container status:**
```bash
docker compose ps

# Detailed inspect
docker inspect apex-evm-node
docker inspect apex-substrate-node
```

**Clean restart:**
```bash
# Stop and remove everything
docker compose down -v

# Rebuild and restart
docker compose up -d --build
```

## Performance Considerations

### Build Caching

Docker images use multi-stage builds and layer caching for faster rebuilds:

- EVM: Node modules are cached unless package.json changes
- Substrate: Binary download is cached in Docker layer

### Test Parallelization

Integration tests run with `--test-threads=1` to avoid conflicts when accessing shared nodes.

For faster local development:
```bash
# Run tests in parallel (may cause flakiness)
INTEGRATION_TESTS=1 cargo test --test evm_integration_test -- --include-ignored
```

## Development Tips

### Skip Integration Tests Locally

Integration tests are marked with `#[ignore]` and skip automatically unless `INTEGRATION_TESTS=1` is set:

```bash
# This will NOT run integration tests
cargo test

# This WILL run integration tests
INTEGRATION_TESTS=1 cargo test -- --include-ignored
```

### Quick Iteration

```bash
# Keep nodes running between test runs
docker compose up -d

# Run tests repeatedly
INTEGRATION_TESTS=1 cargo test --test evm_integration_test -- --include-ignored

# Stop when done
docker compose down
```

### Adding New Tests

1. Add test function to `tests/evm_integration_test.rs` or `tests/substrate_integration_test.rs`
2. Mark with `#[tokio::test]` and `#[ignore]`
3. Add `skip_if_not_integration!()` at the start
4. Use helper functions from `integration_helpers.rs`

Example:
```rust
#[tokio::test]
#[ignore]
async fn test_my_new_feature() {
    skip_if_not_integration!();
    wait_for_evm_node(30).await.expect("Node should be ready");

    let adapter = EvmAdapter::connect(&evm_rpc_url())
        .await
        .expect("Should connect");

    // Your test code here
}
```

## Security Notes

- Test nodes run in development mode with unsafe RPC methods enabled
- Never use test mnemonics or private keys in production
- Test accounts are publicly known and should only be used for testing
- Docker containers bind to localhost by default for security

## Contributing

When adding new Docker infrastructure:

1. Update relevant Dockerfiles
2. Add tests to verify the infrastructure works
3. Update this README with new configurations
4. Test locally before committing
5. Verify CI passes with changes

## Resources

- [Hardhat Documentation](https://hardhat.org/)
- [Substrate Contracts Node](https://github.com/paritytech/substrate-contracts-node)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)

## License

This infrastructure code is part of the Apex SDK project and follows the same Apache-2.0 license.
