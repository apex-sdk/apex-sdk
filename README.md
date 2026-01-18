# Apex SDK

[![Crates.io](https://img.shields.io/crates/v/apex-sdk.svg)](https://crates.io/crates/apex-sdk)
[![Documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://apexsdk.dev/)
[![CI](https://github.com/apex-sdk/apex-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/apex-sdk/apex-sdk/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

A unified Rust SDK for building cross-chain applications across Substrate and EVM ecosystems.

Apex SDK provides a single, type-safe API to interact with multiple blockchain protocols—enabling developers to build multichain applications without managing different client libraries, RPC interfaces, or type systems for each chain.

## Features

- **Unified API** — Single interface for Substrate and EVM chains
- **Type Safety** — Compile-time guarantees from blockchain metadata
- **Chain Abstraction** — Write once, deploy to Polkadot, Ethereum, and beyond
- **Native Performance** — Zero-cost abstractions built on Rust

## Quick Start

**Installation**

```toml
[dependencies]
apex-sdk = "0.1"
```

**CLI Tools**

```bash
cargo install apex-cli
apex new my-project
```


## Supported Ecosystems

- **Substrate** — Polkadot, Kusama, and any Substrate-based chain
- **EVM** — Ethereum, Polygon, BSC, Avalanche, Arbitrum, Optimism, and EVM-compatible networks
- **Hybrid** — Moonbeam, Astar (Substrate + EVM)

## Documentation

- **[Getting Started](./docs/QUICK_START.md)** — Build your first multichain app
- **[API Reference](./docs/API.md)** — Complete API documentation
- **[Examples](./examples/)** — Working code examples
- **[CLI Guide](./docs/CLI_GUIDE.md)** — Command-line tools

Visit [apexsdk.dev](https://apexsdk.dev) for full documentation.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines and [GOVERNANCE.md](GOVERNANCE.md) for project structure.

```bash
git clone https://github.com/apex-sdk/apex-sdk.git
cd apex-sdk
cargo test
```

## Community

- [Discord](https://discord.gg/zCDFsBaZJN) — Join the community
- [Discussions](https://github.com/apex-sdk/apex-sdk/discussions) — Ask questions
- [Issues](https://github.com/apex-sdk/apex-sdk/issues) — Report bugs

## Security

To report vulnerabilities, email **security@apexsdk.dev** or see [SECURITY.md](./SECURITY.md).

## License

Licensed under [Apache 2.0](LICENSE)

---

**Apex SDK** is maintained by [@apex-sdk/core-maintainers](https://github.com/orgs/apex-sdk/teams/core-maintainers)
