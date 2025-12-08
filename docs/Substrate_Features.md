# Apex SDK Substrate

## Overview

This guide covers the features implemented in Apex SDK Substrate, bringing production-grade functionality to Polkadot ecosystem development.

## Table of Contents

1. [Typed Metadata Support](#typed-metadata-support)
2. [XCM Cross-Chain Messaging](#xcm-cross-chain-messaging)
3. [Ink! Smart Contract Integration](#ink-smart-contract-integration)
4. [Custom Signers](#custom-signers)
5. [Storage Queries](#storage-queries)


## Typed Metadata Support

### Overview

Typed metadata provides compile-time type safety for Substrate interactions, catching errors during development instead of runtime.

### Benefits

- **Compile-time type checking**
- **Better IDE support** (autocomplete, type hints)
- **10x faster transactions** (reduced runtime overhead)
- **Refactoring safety**
- **Inline documentation**

### Usage

#### Enable Feature

```toml
[dependencies]
apex-sdk-substrate = { version = "0.1.3", features = ["typed-westend"] }
```

Available features:
- `typed` - Enable typed metadata support
- `typed-polkadot` - Polkadot-specific types
- `typed-kusama` - Kusama-specific types
- `typed-westend` - Westend-specific types

#### Generate Metadata

```bash
cd apex-sdk-substrate
./scripts/generate_metadata.sh westend
```

#### Use Typed API

```rust
#[cfg(feature = "typed-westend")]
use apex_sdk_substrate::metadata::westend;

#[cfg(feature = "typed-westend")]
let tx = westend::tx()
    .balances()
    .transfer_keep_alive(dest, amount);

// Compile-time type safety!
client.tx().sign_and_submit(&tx, &signer).await?;
```

### Documentation

See `METADATA_GENERATION.md` for comprehensive guide.


## XCM Cross-Chain Messaging

### Overview

XCM (Cross-Consensus Messaging) enables cross-chain asset transfers and communication within the Polkadot ecosystem.

### Features

- **Reserve transfers** (via reserve chain)
- **Teleport transfers** (burn and mint)
- **Multi-location addressing**
- **XCM v3/v4 support**
- **Parachain-to-parachain transfers**
- **Parachain-to-relay transfers**

### Usage

#### Basic Transfer to Relay Chain

```rust
use apex_sdk_substrate::{XcmExecutor, SubstrateAdapter};

// Connect to parachain
let adapter = SubstrateAdapter::connect("wss://parachain-rpc").await?;
let executor = XcmExecutor::new(adapter.client().clone());

// Transfer 1 DOT to relay chain
let beneficiary = [/* 32-byte account ID */];
let tx_hash = executor
    .transfer_to_relay(&wallet, beneficiary, 1_000_000_000_000)
    .await?;

println!("XCM transfer: {}", tx_hash);
```

#### Transfer to Another Parachain

```rust
// Transfer to parachain 2000
let tx_hash = executor
    .transfer_to_parachain(
        &wallet,
        2000, // Parachain ID
        beneficiary,
        1_000_000_000_000, // 1 DOT
    )
    .await?;
```

#### Advanced: Custom Multi-Location

```rust
use apex_sdk_substrate::{MultiLocation, Junction, XcmAsset};

// Create complex multi-location
let location = MultiLocation::new(
    1, // parents (relay chain)
    vec![
        Junction::Parachain(2000),
        Junction::AccountId32 {
            network: None,
            id: beneficiary,
        },
    ],
);

// Create assets to transfer
let assets = vec![XcmAsset::native(1_000_000_000_000)];

// Execute reserve transfer
let tx_hash = executor
    .reserve_transfer(&wallet, location, beneficiary, assets)
    .await?;
```

#### Teleport Transfer

```rust
// Teleport (requires mutual trust)
let tx_hash = executor
    .teleport(&wallet, location, beneficiary, assets)
    .await?;
```

#### Configure XCM Settings

```rust
use apex_sdk_substrate::{XcmConfig, XcmVersion, WeightLimit};

let config = XcmConfig {
    version: XcmVersion::V3,
    weight_limit: WeightLimit::Limited(5_000_000_000),
    fee_asset: None,
};

let executor = XcmExecutor::with_config(client, config);
```

### Multi-Location Helpers

```rust
// Relay chain
let relay = MultiLocation::parent();

// Specific parachain
let para = MultiLocation::parachain(2000);

// Account on current chain
let account = MultiLocation::account(account_id);

// Account on parachain
let para_account = MultiLocation::parachain_account(2000, account_id);
```

### Safety Notes

- **Reserve transfers**: Safer, use for untrusted destinations
- **Teleports**: Faster but requires trust, only between system chains
- **Weight limits**: Set conservatively to avoid failed transfers
- **Test on testnets first!** (Westend, Rococo)


## Ink! Smart Contract Integration

### Overview

Deploy and interact with ink! smart contracts on Substrate chains.

### Features

- **Deploy compiled Wasm contracts**
- **Call contract methods** (read and write)
- **Parse contract metadata**
- **Gas estimation**
- **Storage deposit handling**

### Usage

#### Deploy Contract

```rust
use apex_sdk_substrate::{ContractClient, GasLimit, parse_metadata};

// Load contract artifacts
let wasm_code = std::fs::read("contract.wasm")?;
let metadata_json = std::fs::read_to_string("metadata.json")?;
let metadata = parse_metadata(&metadata_json)?;

// Deploy
let contract = ContractClient::deploy(
    client,
    wasm_code,
    metadata,
    "new", // constructor name
    &[], // constructor args (SCALE-encoded)
    &wallet,
    None, // salt (None for random)
).await?;

println!("Contract deployed at: {:?}", contract.address());
```

#### Call Contract Method (Mutable)

```rust
// Encode arguments using parity-scale-codec
use parity_scale_codec::Encode;

let recipient = [/* account ID */];
let amount = 1000u128;
let args = (recipient, amount).encode();

// Call transfer method
let tx_hash = contract
    .call("transfer", &args, &wallet)
    .await?;

println!("Transfer tx: {}", tx_hash);
```

#### Read Contract State (Immutable)

```rust
let caller = wallet.account_id();
let result = contract
    .read("balance_of", &caller.encode(), &caller)
    .await?;

// Decode result
use parity_scale_codec::Decode;
let balance = u128::decode(&mut &result[..])?;
println!("Balance: {}", balance);
```

#### Custom Gas Limits

```rust
use apex_sdk_substrate::{GasLimit, StorageDepositLimit};

// Higher gas for complex operations
let gas = GasLimit::new(
    5_000_000_000_000, // ref_time
    10_485_760,        // proof_size
);

// With storage deposit limit
let storage = StorageDepositLimit::Limited(1_000_000_000_000);
```

#### Contract Call Builder

```rust
use apex_sdk_substrate::ContractCallBuilder;

let builder = ContractCallBuilder::new(contract_address, selector)
    .args(&encoded_args)
    .gas_limit(GasLimit::default_call())
    .storage_deposit(StorageDepositLimit::NoLimit)
    .value(1000); // payable

let call_data = builder.build_call_data();
```

### Contract Metadata Structure

```rust
// Automatically parsed from metadata.json
let metadata = contract.metadata().unwrap();

// Access constructors
for constructor in &metadata.spec.constructors {
    println!("Constructor: {}", constructor.label);
    println!("Selector: {:?}", constructor.selector);
}

// Access messages (methods)
for message in &metadata.spec.messages {
    println!("Method: {}", message.label);
    println!("Mutates: {}", message.mutates);
    println!("Payable: {}", message.payable);
}

// Access events
for event in &metadata.spec.events {
    println!("Event: {}", event.label);
}
```

### Best Practices

1. **Always test on testnet first**
2. **Set appropriate gas limits** (use estimation)
3. **Handle storage deposits** (can be refunded)
4. **Validate contract metadata** before deployment
5. **Use typed metadata** when possible for better safety

---

## Custom Signers

### Overview

Custom signer implementation replacing deprecated `PairSigner` from substrate-compat.

### Features

- **SR25519 signer**
- **ED25519 signer**
- **Enum wrapper** for both types
- **Proper type conversions** (subxt::utils types)

### Usage

#### SR25519 Signer

```rust
use apex_sdk_substrate::Sr25519Signer;
use sp_core::{Pair, sr25519};

let (pair, _) = sr25519::Pair::generate();
let signer = Sr25519Signer::new(pair);

// Use with subxt
let tx_hash = client
    .tx()
    .sign_and_submit_then_watch_default(&tx, &signer)
    .await?;
```

#### ED25519 Signer

```rust
use apex_sdk_substrate::Ed25519Signer;
use sp_core::{Pair, ed25519};

let (pair, _) = ed25519::Pair::generate();
let signer = Ed25519Signer::new(pair);
```

#### Apex Signer (Enum)

```rust
use apex_sdk_substrate::ApexSigner;

// Supports both key types
let signer = ApexSigner::from_sr25519(sr_pair);
// or
let signer = ApexSigner::from_ed25519(ed_pair);

// Works with both
client.tx().sign_and_submit(&tx, &signer).await?;
```

### Implementation Details

- Uses `subxt::utils::AccountId32` instead of `sp_runtime::AccountId32`
- Uses `subxt::utils::MultiSignature` for compatibility
- Properly converts signatures from `sp_core` to `subxt` types
- Implements `subxt::tx::Signer` trait correctly

---

## Storage Queries

### Overview

Real storage query implementation replacing placeholder functions.

### Features

- **Dynamic storage queries** using System.Account
- **Proper value decoding**
- **Balance queries** with chain state
- **Account info** (nonce, balances, etc.)

### Usage

#### Query Balance

```rust
use apex_sdk_substrate::SubstrateAdapter;

let adapter = SubstrateAdapter::connect("wss://westend-rpc.polkadot.io").await?;

// Get balance in Planck (smallest unit)
let balance = adapter.get_balance(address).await?;
println!("Balance: {} Planck", balance);

// Get formatted balance
let formatted = adapter.get_balance_formatted(address).await?;
println!("Balance: {}", formatted); // "1.234567890000 WND"
```

#### Query Account Info

```rust
let storage = adapter.storage();
let account_info = storage.get_account_info(address).await?;

println!("Nonce: {}", account_info.nonce);
println!("Free: {}", account_info.free);
println!("Reserved: {}", account_info.reserved);
println!("Frozen: {}", account_info.frozen);
```

#### Dynamic Storage Queries

```rust
// Query any storage item
let storage_query = subxt::dynamic::storage(
    "System",
    "Account",
    vec![subxt::dynamic::Value::from_bytes(&account_id)],
);

let result = adapter
    .client()
    .storage()
    .at_latest()
    .await?
    .fetch(&storage_query)
    .await?;

if let Some(data) = result {
    let decoded = data.to_value()?;
    // Access fields using At trait
    use subxt::dynamic::At;
    let free = decoded.at("data")?.at("free")?.as_u128()?;
}
```

### Implementation Details

- Uses `System.Account` storage map
- Decodes `DecodedValueThunk` to extract values
- Handles non-existent accounts (returns 0)
- Proper error handling for invalid addresses

---

## Performance Optimizations

### Connection Pooling

```rust
use apex_sdk_substrate::{ConnectionPool, PoolConfig};

let config = PoolConfig {
    max_connections: 10,
    min_idle: 2,
    connection_timeout: Duration::from_secs(30),
};

let pool = ConnectionPool::new(endpoint, config).await?;
let client = pool.get_connection().await?;
```

### Caching

```rust
use apex_sdk_substrate::{Cache, CacheConfig};

let cache_config = CacheConfig {
    capacity: 1000,
    ttl: Duration::from_secs(60),
};

let cache = Cache::new(cache_config);
```

### Metrics

```rust
let metrics = adapter.metrics();
println!("RPC calls: {}", metrics.total_rpc_calls);
println!("Transactions: {}", metrics.total_transactions);
println!("Success rate: {:.2}%", metrics.success_rate());
```

---

## Error Handling

All functions return `Result<T, Error>` with detailed error messages:

```rust
match adapter.get_balance(address).await {
    Ok(balance) => println!("Balance: {}", balance),
    Err(Error::Connection(msg)) => eprintln!("Connection error: {}", msg),
    Err(Error::Storage(msg)) => eprintln!("Storage error: {}", msg),
    Err(Error::Transaction(msg)) => eprintln!("Transaction error: {}", msg),
    Err(e) => eprintln!("Other error: {}", e),
}
```

---

## Testing

### Integration Tests

```bash
# Run Westend integration tests
cargo test --test westend_integration -- --ignored

# Run specific test
cargo test --test westend_integration test_query_balance_westend -- --ignored
```

### Unit Tests

```bash
# Run all unit tests
cargo test

# Run XCM tests
cargo test xcm::tests

# Run contract tests
cargo test contracts::tests
```

---

## Examples

See `/examples` directory for complete working examples:

- `basic-transfer/` - Simple balance transfers
- `xcm-transfer/` - Cross-chain XCM transfers
- `contract-deploy/` - ink! contract deployment
- `storage-queries/` - Chain state queries

---

## Support & Resources

- **Documentation**: https://docs.apexsdk.io
- **GitHub**: https://github.com/eurybits/apex-sdk
- **Discord**: https://discord.gg/zCDFsBaZJN
- **Issues**: https://github.com/eurybits/apex-sdk/issues


## Contributing

We welcome contributions! See `CONTRIBUTING.md` for guidelines.

## License

Apache 2.0 - See `LICENSE` file for details.
