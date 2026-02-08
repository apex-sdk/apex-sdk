
# Quick Start: System Chain Standard (Asset Hub & Revive)

## Installation

Add `apex-sdk` to your `Cargo.toml`:

```toml
[dependencies]
apex-sdk = "0.1.6"
```

## Asset Hub: AssetManager Example

```rust
use apex_sdk_substrate::{SubstrateAdapter, AssetManager};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let adapter = SubstrateAdapter::connect("wss://paseo-asset-hub-pub.dwellir.com").await?;
    let assets = AssetManager::new(&adapter);
    let admin = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".parse()?;
    let payload = assets.create(1000, &admin, 1_000_000_000_000).await?;
    println!("Asset create payload: {:?}", payload);
    Ok(())
}
```

## Asset Hub: NftManager Example

```rust
use apex_sdk_substrate::{SubstrateAdapter, NftManager};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let adapter = SubstrateAdapter::connect("wss://paseo-asset-hub-pub.dwellir.com").await?;
    let nfts = NftManager::new(&adapter);
    let owner = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".parse()?;
    let payload = nfts.create_collection(42, &owner, "My NFT Collection".to_string()).await?;
    println!("NFT collection payload: {:?}", payload);
    Ok(())
}
```

## Revive: Deploy & Call Solidity Contract

```rust
use apex_sdk_revive::{ReviveAdapter, ContractManager};
use apex_sdk_substrate::Wallet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let adapter = ReviveAdapter::connect("wss://paseo-asset-hub-pub.dwellir.com").await?;
    let wallet = Wallet::from_mnemonic("your mnemonic...", Default::default())?;
    let contracts = ContractManager::new(&adapter, wallet.to_subxt_signer());

    // 1. Deploy
    let code = std::fs::read("MyContract.polkavm")?;
    let constructor_data = vec![]; // ABI-encoded constructor args
    let salt = [0u8; 32];
    let value = 0;
    let address = contracts.deploy(code, constructor_data, salt, value, None).await?;
    println!("Deployed contract at: {}", address);

    // 2. Call (Transaction)
    let call_data = hex::decode("0x...")?;
    contracts.call(&address, call_data, 0, Some(100_000_000)).await?;

    // 3. Read (Dry-run/Static Call)
    let read_data = hex::decode("0x...")?;
    let result = contracts.read(&address, read_data, 0).await?;
    println!("Read result: {:?}", result);

    Ok(())
}
```

## CLI Patterns

```bash
# Compile Solidity to PolkaVM
apex compile MyContract.sol --target polkavm

# Deploy to Asset Hub/Revive
apex deploy --asset-hub --revive
```
