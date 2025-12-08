# Apex SDK CLI

[![Crates.io](https://img.shields.io/crates/v/apex-sdk-cli)](https://crates.io/crates/apex-sdk-cli)
[![Downloads](https://img.shields.io/crates/d/apex-sdk-cli)](https://crates.io/crates/apex-sdk-cli)
[![License](https://img.shields.io/crates/l/apex-sdk-cli)](../LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org/)

Command-line interface for Apex SDK - unified Rust SDK for Substrate & EVM blockchain development.

## Installation

```bash
cargo install apex-sdk-cli
```

## Quick Start

```bash
# Create a new project
apex new my-project --template defi

# Navigate and build
cd my-project
cargo build
```

## Commands

### Project Management

```bash
# Create new project with templates (default, defi, nft)
apex new <project-name> [--template <template>]

# Build project
apex build [--release]

# Run tests
apex test [--filter <pattern>]

# Run benchmarks
apex bench [--filter <pattern>]
```

### Account Management

```bash
# Generate new account
apex account generate --account-type <substrate|evm> --name <name>

# List accounts
apex account list

# Import from mnemonic
apex account import --name <name>

# Export mnemonic
apex account export --name <name>

# Get balance
apex account balance --name <name> --chain <chain>
```

### Configuration

```bash
# Initialize config interactively
apex config init

# Show current configuration
apex config show

# Set a value
apex config set <key> <value>

# Get a value
apex config get <key>

# Validate configuration
apex config validate

# Reset to defaults
apex config reset
```

### Chain Information

```bash
# List supported chains
apex chain list

# Get chain info
apex chain info <chain>

# Check chain health
apex chain health <chain>
```

### Contract Deployment

```bash
# Deploy contract
apex deploy <contract> --chain <chain> --endpoint <url> [--dry-run]
```

### Shell Completions

```bash
# Generate completions for your shell
apex completions <bash|zsh|fish|powershell|elvish>

# Example: Add to your .bashrc
apex completions bash >> ~/.bashrc
```

### Other

```bash
# Show version
apex version

# Show help
apex --help
```

## Supported Chains

**Substrate:** Polkadot, Kusama, Paseo, Westend, Moonbeam, Astar

**EVM:** Ethereum, Polygon, BSC, Avalanche, Arbitrum, Optimism, zkSync, Sepolia

## Configuration

Config file location: `~/.config/apex-sdk/config.json`

Default configuration includes:
- Network endpoints for all supported chains
- Default chain (Paseo testnet)
- UI preferences (colors, progress bars)
- Log level settings

## License

Apache-2.0
