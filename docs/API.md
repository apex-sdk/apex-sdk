apex-sdk-substrate = { version = "0.1.6", features = ["typed-polkadot"] }
apex-sdk-evm = { version = "0.1.6" }
apex-sdk-core = { version = "0.1.6", features = ["mocks"] }

# API Reference: System Chain Standard

Comprehensive API reference for Apex SDK v0.1.6 — the canonical toolkit for Asset Hub and Revive (PolkaVM/Solidity).

## Core Modules

### AssetManager (Asset Hub)

```rust
use apex_sdk_substrate::{SubstrateAdapter, AssetManager};
let adapter = SubstrateAdapter::connect("wss://paseo-asset-hub-pub.dwellir.com").await?;
let assets = AssetManager::new(&adapter);
let admin = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".parse()?;
let payload = assets.create(1000, &admin, 1_000_000_000_000).await?;
```

#### Methods
- `create(id, admin, min_balance)`
- `mint(id, beneficiary, amount)`
- `burn(id, who, amount)`
- `transfer(id, target, amount)`
- `set_metadata(id, name, symbol, decimals)`

### NftManager (Asset Hub)

```rust
use apex_sdk_substrate::{SubstrateAdapter, NftManager};
let nfts = NftManager::new(&adapter);
let owner = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".parse()?;
let payload = nfts.create_collection(42, &owner, "My NFT Collection".to_string()).await?;
```

#### Methods
- `create_collection(collection_id, owner, metadata)`
- `mint(collection_id, item_id, owner, metadata)`
- `transfer(collection_id, item_id, to)`

### ReviveAdapter & ContractManager (Revive/PolkaVM)

```rust
use apex_sdk_revive::{ReviveAdapter, ContractManager};
let adapter = ReviveAdapter::connect("wss://paseo-asset-hub-pub.dwellir.com").await?;
let contracts = ContractManager::new(&adapter);
let code = std::fs::read("MyContract.polkavm")?;
let constructor_data = vec![];
let salt = [0u8; 32];
let address = contracts.deploy(code, constructor_data, salt).await?;
```

#### Methods
- `deploy(code, constructor_data, salt)`
- `Contract<T>::new(address)`

---

## Pre-bundled Metadata

Enable type-safe calls for Asset Hub and Polkadot:

```toml
apex-sdk-substrate = { version = "0.1.6", features = ["typed-westend", "typed-polkadot"] }
```

```toml
apex-sdk-core = { version = "0.1.6", features = ["mocks"] }
```
