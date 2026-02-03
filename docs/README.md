
# Apex SDK: System Chain Standard Library

[![Docs](https://img.shields.io/badge/docs-latest-brightgreen.svg)](https://apexsdk.dev/)
[![Discord](https://img.shields.io/discord/1234567890?label=discord)](https://discord.gg/zCDFsBaZJN)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](../LICENSE)
[![GitHub Stars](https://img.shields.io/github/stars/apex-sdk/apex-sdk?style=social)](https://github.com/apex-sdk/apex-sdk)


Welcome to Apex SDK—the canonical Rust SDK for Polkadot System Chains. Build next-generation dApps for Asset Hub and Revive (PolkaVM/Solidity) with type safety and performance.

> **New here?** Start with the [**Quick Start Guide**](QUICK_START.md) to build your first Asset Hub or Revive workflow in minutes!

## Quick Navigation

### For New Users
- [**Quick Start**](QUICK_START.md) - 5-minute setup guide
- [**Examples**](../examples/) - Working code samples
- [**CLI Guide**](CLI_GUIDE.md) - Command-line tools

### For Developers
- [**API Reference**](API.md) - Complete API documentation
- [**Testing Framework**](TESTING_FRAMEWORK.md) - Write comprehensive tests
- [**Security Guide**](SECURITY.md) - Security best practices
- [**Contributing**](CONTRIBUTING.md) - Join the community

### For Planning
- [**Roadmap**](ROADMAP.md) - Future development plans
- [**UX Improvements**](../UX_IMPROVEMENTS_APPLIED.md) - Recent enhancements
- [**Security Audit**](SECURITY_AUDIT.md) - Audit results

---


## What is Apex SDK?

Apex SDK is the official System Chain Standard Library for Polkadot Hub and Revive:

- **Asset Hub Primitives**: High-level Rust APIs for `pallet-assets` and `pallet-nfts` (see `AssetManager`, `NftManager`).
- **Revive (PolkaVM/Solidity) Support**: Native Rust client for `pallet-revive`—compile, deploy, and call Solidity contracts with zero JS dependencies.
- **Atomic Orchestrator**: Compose cross-pallet and cross-chain workflows in a single transaction.
- **Pre-bundled Metadata**: Out-of-the-box support for Paseo and Polkadot Asset Hubs with pre-generated `subxt` types.
- **CLI for Canonical Patterns**: Compile and deploy Solidity contracts, orchestrate Asset Hub/Revive flows, and access production-ready templates.

## Quick Example

```rust
use apex_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let sdk = ApexSDK::builder()
        .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
        .with_evm_endpoint("https://mainnet.infura.io/v3/YOUR_KEY")
        .build()
        .await?;

    let tx = sdk
        .transaction()
        .from_substrate_account("5GrwvaEF...")
        .to_evm_address("0x742d35Cc...")
        .amount(1000)
        .build()?;

    let result = sdk.execute(tx).await?;
    println!("Transaction: {:?}", result);

    Ok(())
}
```

## Supported Chains

### Substrate
- Polkadot
- Kusama

### EVM
- Ethereum
- Binance Smart Chain
- Polygon
- Avalanche

### Hybrid
- Moonbeam
- Astar

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
apex-sdk = "0.1.6"
tokio = { version = "1.35", features = ["full"] }
```

Or use the CLI:

```bash
cargo install apex-sdk-cli
apex new my-project
```

## Examples

Check out our comprehensive examples in the [`examples/`](../examples/) directory:

- [**Account Manager**](../examples/account-manager/) - Multi-chain account management
- [**Price Oracle**](../examples/price-oracle/) - Cross-chain price aggregation
- [**Contract Orchestration**](../examples/contract-orchestration/) - Smart contract deployment
- [**Parachain Assets**](../examples/parachain-assets/) - Parachain asset management

Each example includes:
- Complete working code
- Detailed README with explanations
- Step-by-step instructions

## Community

- **GitHub**: [apex-sdk/apex-sdk](https://github.com/apex-sdk/apex-sdk)
- **Issues**: [Report bugs](https://github.com/apex-sdk/apex-sdk/issues)
- **Discussions**: [Join the conversation](https://github.com/apex-sdk/apex-sdk/discussions)

## License

[Apache 2.0](https://github.com/apex-sdk/apex-sdk/blob/main/LICENSE)
