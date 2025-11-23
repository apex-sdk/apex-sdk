# Apex SDK

[![CI](https://github.com/kherldhussein/apex-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/kherldhussein/apex-sdk/actions/workflows/ci.yml)
[![Security](https://github.com/kherldhussein/apex-sdk/actions/workflows/security.yml/badge.svg)](https://github.com/kherldhussein/apex-sdk/actions/workflows/security.yml/badge.svg)
[![Benchmarks](https://github.com/kherldhussein/apex-sdk/actions/workflows/benchmarks.yml/badge.svg)](https://github.com/kherldhussein/apex-sdk/actions/workflows/benchmarks.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.85%2B-orange.svg)](https://www.rust-lang.org/)

> **Unified Rust SDK for cross-chain development across Substrate and EVM ecosystems**

Apex SDK provides a compile-time safe, unified interface for building blockchain applications. Write once, deploy across Polkadot, Ethereum, and compatible chains with a single API.

**Key Features:** Type-safe · Cross-chain · Metadata-driven · Production-ready

---

## Overview

| Aspect | Details |
|--------|---------|
| **Purpose** | Unified SDK for Substrate and EVM blockchain development |
| **Language** | Rust 1.85+ |
| **Type Safety** | Compile-time guarantees via metadata-driven code generation |
| **Supported Chains** | Polkadot, Kusama, Ethereum, Polygon, BSC, Avalanche, [+more](./docs/SUPPORTED_CHAINS.md) |
| **Status** | Beta - Production ready for testing |

**[→ Read Full Objectives & Goals](./docs/PROJECT_OBJECTIVES.md)**

## Quick Start

```bash
# Install CLI
cargo install apex-sdk-cli

# Create new project
apex new my-app
cd my-app

# Build and run
cargo build
cargo test
```

**[→ Complete Setup Guide](./docs/INSTALLATION.md)**

## Installation

**Prerequisites:** Rust 1.85+, OpenSSL dev libraries

**Add to Cargo.toml:**
```toml
[dependencies]
apex-sdk = "0.1.3"
apex-sdk-substrate = "0.1.3"  # For Substrate chains
apex-sdk-evm = "0.1.3"        # For EVM chains
```

**CLI Installation:**
```bash
cargo install apex-sdk-cli
apex --version
```

**[→ Detailed Installation Instructions](./docs/INSTALLATION.md)** | **[→ Troubleshooting](./docs/TROUBLESHOOTING.md)**

## Supported Chains

**Substrate:** Polkadot · Kusama · Paseo · Westend · Moonbeam · Astar
**EVM:** Ethereum · Polygon · BSC · Avalanche · Arbitrum · Optimism
**Testnets:** Sepolia · Paseo · Westend

**[→ Complete Chain Support Matrix](./docs/SUPPORTED_CHAINS.md)** | **[→ Roadmap](./docs/ROADMAP.md)**

## Architecture & Dependencies

**Core Stack:** Rust · Substrate · EVM · WebAssembly
**Key Libraries:** subxt · ethers · tokio · serde

**[→ Full Technology Stack](./docs/DEPENDENCIES.md)** | **[→ Architecture Overview](./docs/ARCHITECTURE.md)**

## Documentation

| Resource | Link |
|----------|------|
| **Documentation Hub** | [Complete Guide](./docs/DOCUMENTATION_HUB.md) |
| **Quick Start** | [5-minute setup](./docs/QUICK_START.md) |
| **API Reference** | [Full API docs](./docs/API.md) |
| **CLI Guide** | [Command-line tools](./docs/CLI_GUIDE.md) |
| **Examples** | [Working code samples](./examples/) |

### Examples

- **[Account Manager](./examples/account-manager/)** - Multi-chain account management
- **[Price Oracle](./examples/price-oracle/)** - Cross-chain price aggregation
- **[Contract Orchestration](./examples/contract-orchestration/)** - Smart contract deployment
- **[Parachain Assets](./examples/parachain-assets/)** - Asset management

**[→ Browse All Examples](./examples/)**

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Quick Start:**
```bash
git clone https://github.com/kherldhussein/apex-sdk.git
cd apex-sdk
cargo test --all-features
```

**[→ Development Guide](./docs/DEVELOPMENT.md)** | **[→ Code of Conduct](./CODE_OF_CONDUCT.md)**

## Security

**Report vulnerabilities:** kherld@duck.com

**[→ Security Policy](./docs/SECURITY.md)** | **[→ Security Audit](./docs/SECURITY_AUDIT.md)**

## License

[Apache 2.0](LICENSE) © 2024 Apex SDK Contributors 