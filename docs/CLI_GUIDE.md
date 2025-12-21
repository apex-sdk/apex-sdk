# Apex SDK CLI Guide

The Apex SDK CLI is a powerful command-line tool for building, testing, and deploying blockchain applications across Substrate and EVM ecosystems.

## Table of Contents

- [Installation](#installation)
- [Getting Started](#getting-started)
- [Commands](#commands)
  - [Project Management](#project-management)
  - [Account Management](#account-management)
  - [Chain Operations](#chain-operations)
  - [Deployment](#deployment)
  - [Testing & Benchmarking](#testing--benchmarking)
- [Configuration](#configuration)
- [Examples](#examples)
- [Troubleshooting](#troubleshooting)

## Installation

### From Source

```bash
git clone https://github.com/carbobit/apex-sdk.git
cd apex-sdk/cli
cargo install --path .
```

### From Crates.io

```bash
cargo install apex-sdk-cli
```

### Verify Installation

```bash
apex version
```

## Getting Started

### Create Your First Project

```bash
# Create a new project with the default template
apex new my-project

# Create a DeFi project
apex new defi-app --template defi

# Create an NFT project
apex new nft-marketplace --template nft
```

### Initialize Configuration

```bash
# Initialize with default settings
apex init

# Initialize with interactive prompts
apex init --interactive
```

This creates a `.apex/config.json` file in your project directory with default settings:

```json
{
  "default_chain": "polkadot",
  "default_endpoint": "wss://polkadot.api.onfinality.io/public-ws",
  "accounts": []
}
```

## Commands

### Project Management

#### `apex new`

Create a new Apex SDK project from a template.

**Usage:**
```bash
apex new <PROJECT_NAME> [OPTIONS]
```

**Options:**
- `-t, --template <TEMPLATE>`: Project template (default, defi, nft) [default: default]

**Examples:**
```bash
# Basic project
apex new my-app

# DeFi application
apex new defi-protocol --template defi

# NFT marketplace
apex new nft-market --template nft
```

#### `apex build`

Build your project using Cargo.

**Usage:**
```bash
apex build [OPTIONS]
```

**Options:**
- `-r, --release`: Build in release mode with optimizations

**Examples:**
```bash
# Development build
apex build

# Production build
apex build --release
```

#### `apex test`

Run your project's test suite.

**Usage:**
```bash
apex test [OPTIONS]
```

**Options:**
- `-f, --filter <PATTERN>`: Run only tests matching the pattern

**Examples:**
```bash
# Run all tests
apex test

# Run specific tests
apex test --filter substrate
apex test --filter evm_integration
```

#### `apex bench`

Run performance benchmarks.

**Usage:**
```bash
apex bench [OPTIONS]
```

**Options:**
- `-f, --filter <PATTERN>`: Run only benchmarks matching the pattern

**Examples:**
```bash
# Run all benchmarks
apex bench

# Run specific benchmarks
apex bench --filter transaction
```

### Account Management

#### `apex account generate`

Generate a new blockchain account/wallet.

**Usage:**
```bash
apex account generate [OPTIONS]
```

**Options:**
- `-a, --account-type <TYPE>`: Account type (substrate, evm)

**Examples:**
```bash
# Generate Substrate account
apex account generate --account-type substrate

# Generate EVM account
apex account generate --account-type evm
```

**Output:**
```
 Generating new substrate account...
   Type: Substrate (SR25519)
   Address: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
   Mnemonic: [SECURE - Store this safely!]

  WARNING: Keep your keys secure and never share them!
```

#### `apex account import`

Import an account from a mnemonic phrase.

**Usage:**
```bash
apex account import <MNEMONIC> [OPTIONS]
```

**Options:**
- `-a, --account-type <TYPE>`: Account type (substrate, evm)

**Examples:**
```bash
apex account import "word1 word2 ... word12" --account-type substrate
apex account import "word1 word2 ... word12" --account-type evm
```

#### `apex account list`

List all managed accounts.

**Usage:**
```bash
apex account list
```

#### `apex account balance`

Check the balance of an account.

**Usage:**
```bash
apex account balance <ADDRESS> [OPTIONS]
```

**Options:**
- `-c, --chain <CHAIN>`: Chain name
- `-e, --endpoint <ENDPOINT>`: RPC endpoint URL

**Examples:**
```bash
# Check Polkadot balance
apex account balance 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \
  --chain polkadot \
  --endpoint wss://polkadot.api.onfinality.io/public-ws

# Check Ethereum balance
apex account balance 0x1234567890123456789012345678901234567890 \
  --chain ethereum \
  --endpoint https://mainnet.infura.io/v3/YOUR_KEY
```

### Chain Operations

#### `apex chain list`

List all supported blockchain networks.

**Usage:**
```bash
apex chain list
```

**Output:**
```
 Supported chains:

   Substrate-based:
     • polkadot    - Polkadot Relay Chain
     • kusama      - Kusama Relay Chain
     • moonbeam    - Moonbeam Parachain
     • astar       - Astar Parachain
     • acala       - Acala DeFi Hub
     • phala       - Phala Privacy Cloud
     • bifrost     - Bifrost Liquid Staking

   EVM-compatible:
     • ethereum    - Ethereum Mainnet
     • bsc         - Binance Smart Chain
     • polygon     - Polygon (Matic)
     • avalanche   - Avalanche C-Chain
     • arbitrum    - Arbitrum One (L2)
     • optimism    - Optimism (L2)
     • zksync      - zkSync Era (L2)
```

#### `apex chain info`

Get information about a specific chain.

**Usage:**
```bash
apex chain info <CHAIN> [OPTIONS]
```

**Options:**
- `-e, --endpoint <ENDPOINT>`: RPC endpoint URL

**Examples:**
```bash
apex chain info polkadot \
  --endpoint wss://polkadot.api.onfinality.io/public-ws

apex chain info ethereum \
  --endpoint https://mainnet.infura.io/v3/YOUR_KEY
```

#### `apex chain health`

Check the health status of a chain endpoint.

**Usage:**
```bash
apex chain health <ENDPOINT>
```

**Examples:**
```bash
apex chain health wss://polkadot.api.onfinality.io/public-ws
apex chain health https://mainnet.infura.io/v3/YOUR_KEY
```

### Deployment

#### `apex deploy`

Deploy a smart contract to a blockchain.

**Usage:**
```bash
apex deploy <CONTRACT> [OPTIONS]
```

**Options:**
- `-c, --chain <CHAIN>`: Target chain name
- `-e, --endpoint <ENDPOINT>`: RPC endpoint URL

**Examples:**
```bash
# Deploy to Polkadot
apex deploy ./contracts/my_contract.wasm \
  --chain polkadot \
  --endpoint wss://polkadot.api.onfinality.io/public-ws

# Deploy to Ethereum
apex deploy ./contracts/MyContract.sol \
  --chain ethereum \
  --endpoint https://mainnet.infura.io/v3/YOUR_KEY
```

### Testing & Benchmarking

See [Project Management](#project-management) section for `apex test` and `apex bench` commands.

## Configuration

### Configuration File

The CLI stores configuration in `.apex/config.json`:

```json
{
  "default_chain": "polkadot",
  "default_endpoint": "wss://polkadot.api.onfinality.io/public-ws",
  "accounts": [
    {
      "type": "substrate",
      "address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
      "name": "Alice"
    }
  ],
  "custom_endpoints": {
    "polkadot": [
      "wss://polkadot.api.onfinality.io/public-ws",
      "wss://rpc.polkadot.io"
    ],
    "ethereum": [
      "https://mainnet.infura.io/v3/YOUR_KEY"
    ]
  }
}
```

### Environment Variables

You can override configuration using environment variables:

- `APEX_DEFAULT_CHAIN`: Default blockchain to use
- `APEX_DEFAULT_ENDPOINT`: Default RPC endpoint
- `APEX_CONFIG_PATH`: Path to configuration file

**Example:**
```bash
export APEX_DEFAULT_CHAIN=ethereum
export APEX_DEFAULT_ENDPOINT=https://mainnet.infura.io/v3/YOUR_KEY
apex chain info ethereum
```

## Examples

### Complete Workflow: DeFi Application

```bash
# 1. Create a new DeFi project
apex new defi-protocol --template defi

# 2. Navigate to project
cd defi-protocol

# 3. Initialize configuration
apex init

# 4. Generate deployment account
apex account generate --account-type substrate

# 5. Build the project
apex build --release

# 6. Run tests
apex test

# 7. Run benchmarks
apex bench

# 8. Deploy to testnet
apex deploy ./target/release/defi_protocol.wasm \
  --chain westend \
  --endpoint wss://westend-rpc.polkadot.io
```

### Cross-Chain Asset Transfer

```bash
# 1. Check balance on Polkadot
apex account balance 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY \
  --chain polkadot \
  --endpoint wss://polkadot.api.onfinality.io/public-ws

# 2. Check health of destination chain
apex chain health wss://moonbeam.api.onfinality.io/public-ws

# 3. Get chain information
apex chain info moonbeam \
  --endpoint wss://moonbeam.api.onfinality.io/public-ws
```

### Multi-Chain Development

```bash
# Create project
apex new multi-chain-app

cd multi-chain-app

# List all supported chains
apex chain list

# Test on multiple chains
apex test --filter substrate
apex test --filter evm

# Build optimized release
apex build --release

# Run benchmarks
apex bench
```

## Troubleshooting

### Common Issues

#### Build Failures

**Problem:** Build fails with dependency errors

**Solution:**
```bash
# Clean and rebuild
cargo clean
apex build

# Update dependencies
cargo update
apex build --release
```

#### Connection Issues

**Problem:** Cannot connect to RPC endpoint

**Solution:**
```bash
# Check endpoint health first
apex chain health <ENDPOINT>

# Verify endpoint is correct
apex chain info <CHAIN> --endpoint <ENDPOINT>

# Try alternative endpoint
apex chain list  # Shows alternative endpoints
```

#### Account Generation Issues

**Problem:** Account generation fails

**Solution:**
```bash
# Ensure you have latest version
apex version

# Try with explicit account type
apex account generate --account-type substrate
```

### Getting Help

```bash
# General help
apex --help

# Command-specific help
apex new --help
apex deploy --help
apex account --help
```

### Verbose Output

For debugging, set the `RUST_LOG` environment variable:

```bash
# Info level
RUST_LOG=info apex build

# Debug level
RUST_LOG=debug apex deploy ./contract.wasm --chain polkadot --endpoint wss://...

# Trace level (very verbose)
RUST_LOG=trace apex test
```

## Advanced Features

### Custom Templates

You can create custom project templates by adding them to the `cli/templates` directory:

```bash
# Use custom template
apex new my-app --template custom
```

### Scripting and Automation

The CLI is designed to work well in scripts:

```bash
#!/bin/bash
set -e

# Automated deployment script
apex build --release
apex test
apex deploy ./target/release/app.wasm \
  --chain polkadot \
  --endpoint $POLKADOT_ENDPOINT

# Check deployment
apex chain info polkadot --endpoint $POLKADOT_ENDPOINT
```

### CI/CD Integration

```yaml
# .github/workflows/deploy.yml
name: Deploy

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install Apex CLI
        run: cargo install apex-sdk-cli
      - name: Build
        run: apex build --release
      - name: Test
        run: apex test
      - name: Deploy
        run: |
          apex deploy ./target/release/app.wasm \
            --chain polkadot \
            --endpoint ${{ secrets.POLKADOT_ENDPOINT }}
```

## Resources

- [Apex SDK Documentation](https://github.com/carbobit/apex-sdk)
- [API Reference](../API.md)
- [Examples](../examples/)
- [Contributing Guide](../CONTRIBUTING.md)
- [Security Policy](../SECURITY.md)

## Support

- GitHub Issues: https://github.com/carbobit/apex-sdk/issues
- Discord: [Join our community](https://discord.gg/zCDFsBaZJN)
- Email: support@apexsdk.io

## License

Apache-2.0 - See [LICENSE](../LICENSE) for details.
