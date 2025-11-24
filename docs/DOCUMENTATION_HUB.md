# Apex SDK Protocol Documentation Hub

> Your complete guide to building cross-chain applications with Apex SDK

## Getting Started

**New to Apex SDK?** Start here:

| Guide | Time | Description |
|-------|------|-------------|
| [**Quick Start Guide**](QUICK_START.md) | 5 min | Get your first cross-chain app running |
| [**CLI Guide**](CLI_GUIDE.md) | 10 min | Master the command-line tools |
| [**API Reference**](API.md) | Reference | Complete API documentation |

## Core Documentation

### Development Guides

<details>
<summary><b>Basic Concepts</b></summary>

- [Quick Start](QUICK_START.md) - Get started in 5 minutes
- [Project Structure](#project-structure) - Understand the codebase
- [Configuration](CLI_GUIDE.md#configuration) - Configure your SDK
- [Error Handling](#error-handling) - Handle errors gracefully

</details>

<details>
<summary><b>Building Applications</b></summary>

- [Substrate Integration](#substrate) - Connect to Substrate chains
- [EVM Integration](#evm) - Connect to EVM chains
- [Cross-Chain Transactions](#cross-chain) - Bridge between ecosystems
- [Smart Contracts](#contracts) - Deploy and interact with contracts

</details>

<details>
<summary><b>Security & Best Practices</b></summary>

- [Security Guide](SECURITY.md) - Secure your application
- [Security Audit](SECURITY_AUDIT.md) - Review security audit results
- [Best Practices](#best-practices) - Production-ready code
- [Testing Framework](TESTING_FRAMEWORK.md) - Write comprehensive tests

</details>

---

## By Use Case

Find documentation specific to your needs:

### DeFi Applications
- [Token Swaps](#defi-swaps) - Build DEX integrations
- [Yield Farming](#defi-farming) - Aggregate yields across chains
- [Liquidity Pools](#defi-pools) - Manage cross-chain liquidity

### NFT Applications
- [NFT Minting](#nft-minting) - Create and mint NFTs
- [NFT Bridging](#nft-bridge) - Transfer NFTs across chains
- [NFT Marketplaces](#nft-marketplace) - Build marketplaces

### DAO & Governance
- [Voting Systems](#dao-voting) - Implement cross-chain voting
- [Treasury Management](#dao-treasury) - Manage multi-chain treasuries
- [Membership](#dao-membership) - Handle cross-chain memberships

### Infrastructure
- [Oracle Services](#oracle) - Build price oracles
- [Event Indexing](#indexing) - Index blockchain events
- [RPC Services](#rpc) - Set up RPC infrastructure

---

## Developer Tools

### CLI Commands
```bash
# Project Management
apex new <name>          # Create new project
apex build              # Build project
apex test               # Run tests

# Chain Operations
apex chain list         # List supported chains
apex chain info <name>  # Get chain information

# Documentation (Current Status)
apex account generate   # Not implemented - see alternatives
apex account import     # Not implemented - see alternatives
apex deploy             # Not implemented - see alternatives
```

> **Note:** Some CLI features are in development. See [UX Improvements Applied](../UX_IMPROVEMENTS_APPLIED.md) for current status and alternatives.

### Configuration Files
- `.apex/config.json` - Project configuration
- `Cargo.toml` - Dependencies and metadata
- `.github/workflows/` - CI/CD pipelines

---

## Examples & Templates

### Quick Start Examples

| Example | Chains | Description |
|---------|--------|-------------|
| [Account Manager](../examples/account-manager/) | Multi-chain | Manage accounts across ecosystems |
| [Price Oracle](../examples/price-oracle/) | Polkadot + ETH | Aggregate prices cross-chain |
| [Contract Orchestration](../examples/contract-orchestration/) | EVM | Deploy and manage contracts |
| [Parachain Assets](../examples/parachain-assets/) | Substrate | Work with parachain assets |

### Project Templates
```bash
# DeFi template
apex new my-defi --template defi

# NFT template
apex new my-nft --template nft

# Basic template (default)
apex new my-app --template basic
```

---

## API Reference

### Core Modules

| Module | Description | Documentation |
|--------|-------------|---------------|
| `apex_sdk` | Main SDK interface | [API Docs](API.md#apex-sdk) |
| `apex_sdk_substrate` | Substrate adapter | [API Docs](API.md#substrate-adapter) |
| `apex_sdk_evm` | EVM adapter | [API Docs](API.md#evm-adapter) |
| `apex_sdk_types` | Shared types | [API Docs](API.md#types) |
| `apex_sdk_core` | Core traits | [API Docs](API.md#core) |

### Key Types
- `ApexSDK` - Main SDK struct
- `Transaction` - Cross-chain transaction
- `Chain` - Blockchain enumeration
- `Address` - Multi-chain address type
- `TransactionResult` - Execution result

---

## Testing & Quality

### Testing Tools
- [Testing Framework](TESTING_FRAMEWORK.md) - Comprehensive test suite
- [Benchmarking](#benchmarking) - Performance testing
- [Integration Tests](#integration-tests) - End-to-end testing

### Quality Assurance
- **Type Safety** - Compile-time checks with Rust
- **Security Audits** - [View audit results](SECURITY_AUDIT.md)
- **CI/CD** - Automated testing on every commit
- **Code Coverage** - Minimum 80% coverage required

---

## Supported Blockchains

### Substrate Ecosystem
- **Polkadot** - Relay chain (Stable)
- **Kusama** - Canary network (Stable)
- **Westend** - Testnet (In Development)
- **Parachains** - Moonbeam, Astar, Acala (In Development)

### EVM Ecosystem
- **Ethereum** - Mainnet (Stable)
- **BSC** - Binance Smart Chain (Stable)
- **Polygon** - Matic Network (Stable)
- **Avalanche** - C-Chain (Stable)
- **Arbitrum** - L2 (Coming soon)
- **Optimism** - L2 (Coming soon)

---

## Community & Support

### Get Help
- [GitHub Discussions](https://github.com/kh3rld/apexsdk/discussions) - Ask questions
- [Issue Tracker](https://github.com/kh3rld/apexsdk/issues) - Report bugs
- [Security Reports](SECURITY.md) - Report vulnerabilities
- [Contributing Guide](CONTRIBUTING.md) - Contribute code

### Stay Updated
- [Star on GitHub](https://github.com/kh3rld/apexsdk)
- [Changelog](../CHANGELOG.md) - Latest changes
- [Roadmap](#roadmap) - Upcoming features

---

## What's Next?

See our [Development Roadmap](ROADMAP.md) for:
- Planned features
- Timeline and milestones
- How to contribute
- Community priorities

---

## Quick Reference

### Common Tasks

**Initialize a new project:**
```bash
apex new my-app && cd my-app
```

**Connect to chains:**
```rust
let sdk = ApexSDK::builder()
    .with_substrate(Chain::Polkadot, "wss://rpc.polkadot.io")
    .with_evm(Chain::Ethereum, "https://eth.llamarpc.com")
    .build().await?;
```

**Execute a transaction:**
```rust
let tx = sdk.transaction()
    .from_substrate_account("5GrwvaEF...")
    .to_evm_address("0x742d35Cc...")
    .amount(1000)
    .build()?;

let result = sdk.execute(tx).await?;
```

**Check balance:**
```rust
let balance = sdk.substrate()
    .get_balance("5GrwvaEF...")
    .await?;
```

---

## Additional Resources

### Technical Documentation
- [Typed Metadata](TYPED_METADATA.md) - Type-safe runtime interaction
- [Metadata Generation](METADATA_GENERATION.md) - Generate chain metadata
- [Ecosystem Integration](ECOSYSTEM_INTEGRATION.md) - Integrate with tools

### Research & Advanced Topics
- [Research Initiatives](RESEARCH_INITIATIVES.md) - Ongoing research
- [Architecture](#architecture) - System design
- [Performance](#performance) - Optimization guides

---

## Tips & Tricks

**Pro Tips:**
- Use typed metadata for production deployments
- Enable connection pooling for better performance
- Set appropriate timeouts for long-running operations
- Never commit private keys or mnemonics
- Write tests for all cross-chain interactions

**Common Pitfalls:**
- Not handling chain-specific errors
- Assuming synchronous operations
- Ignoring gas/fee estimation
- Using example addresses in production

---

## Quick Links

| Resource | Link |
|----------|------|
| **Main Repository** | [github.com/kh3rld/apexsdk](https://github.com/kh3rld/apexsdk) |
| **Issues** | [Report a bug](https://github.com/kh3rld/apexsdk/issues/new) |
| **Examples** | [Browse examples](../examples/) |
| **API Docs** | [API Reference](API.md) |
| **CLI Guide** | [Command-line tools](CLI_GUIDE.md) |

---

<div align="center">

**Built by the Apex SDK community**

[Get Started](QUICK_START.md) • [View on GitHub](https://github.com/kh3rld/apexsdk) • [Join Discussion](https://github.com/kh3rld/apexsdk/discussions)

</div>
