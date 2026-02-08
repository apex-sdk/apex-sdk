<div align="center">

![Apex SDK](./docs/assets/apex-banner.jpg)


</div>


# System Chain Standard Library for Polkadot Hub & Revive

**[Apex SDK](https://apexsdk.dev)** is the canonical Rust toolkit for Polkadot System Chains, specializing in high-performance, type-safe orchestration of Asset Hub primitives and the new pallet-revive (PolkaVM/Solidity) execution environment.


## System Chain Standard Features

- **Asset Hub Primitives**: High-level, type-safe Rust APIs for `pallet-assets` and `pallet-nfts` (see `AssetManager`, `NftManager`).
- **Revive (PolkaVM/Solidity) Support**: Native Rust client for `pallet-revive`—compile, deploy, and call Solidity contracts with zero JS dependencies.
- **Atomic Orchestrator**: Compose cross-pallet and cross-chain workflows (e.g., mint asset → deposit to contract) in a single transaction.
- **Pre-bundled Metadata**: Out-of-the-box support for Paseo and Polkadot Asset Hubs with pre-generated `subxt` types.
- **CLI for Canonical Patterns**: Compile and deploy Solidity contracts, orchestrate Asset Hub/Revive flows, and access production-ready templates.


## Installation & Usage

```toml
[dependencies]
apex-sdk = { version = "0.1", features = ["substrate", "revive"] }
```

```bash
cargo install apex-sdk-cli
apex compile MyContract.sol --target polkavm
apex deploy --asset-hub --revive
```

---


**Apex SDK** is the official System Chain Standard Library for Polkadot Hub and Revive. It enables developers to build the next generation of canonical dApps, combining native Asset Hub assets and PolkaVM smart contracts in atomic, type-safe workflows.

<div align="center">


**[Start Building](./docs/QUICK_START.md)** • **[Join Discord](https://discord.gg/zCDFsBaZJN)** • **[Read Docs](https://apexsdk.dev)**

</div>
