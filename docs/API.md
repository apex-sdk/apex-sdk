# Apex SDK Protocol API Reference - Complete Developer Documentation

Comprehensive API reference for Apex SDK Protocol v0.1.4 - Your guide to building cross-chain blockchain applications with Rust, Substrate, and EVM.

## Table of Contents

- [Core Types](#core-types)
- [ApexSDK](#apexsdk)
- [ApexSDKBuilder](#apexsdkbuilder)
- [Transaction Builder](#transaction-builder)
- [Addresses](#addresses)
- [Chains](#chains)
- [Error Types](#error-types)
- [Feature Flags](#feature-flags)

## Core Types

### `ApexSDK`

The main SDK instance providing unified access to blockchain operations.

```rust
pub struct ApexSDK {
    substrate_adapter: Option<SubstrateAdapter>,
    evm_adapter: Option<EvmAdapter>,
}
```

#### Methods

##### `builder() -> ApexSDKBuilder`

Creates a new builder for configuring the SDK.

```rust
let sdk = ApexSDK::builder()
    .with_substrate_endpoint("wss://...")
    .with_evm_endpoint("https://...")
    .build()
    .await?;
```

##### `substrate(&self) -> Result<&SubstrateAdapter>`

Returns a reference to the Substrate adapter if configured.

```rust
let substrate = sdk.substrate()?;
```

##### `evm(&self) -> Result<&EvmAdapter>`

Returns a reference to the EVM adapter if configured.

```rust
let evm = sdk.evm()?;
```

##### `is_chain_supported(&self, chain: &Chain) -> bool`

Checks if a specific chain is supported based on configured adapters.

```rust
if sdk.is_chain_supported(&Chain::Ethereum) {
    println!("Ethereum is supported!");
}
```

##### `get_transaction_status(&self, chain: &Chain, tx_hash: &str) -> Result<TransactionStatus>`

Queries the status of a transaction on a specific chain.

```rust
let status = sdk.get_transaction_status(
    &Chain::Ethereum,
    "0x123..."
).await?;
```

##### `transaction(&self) -> TransactionBuilder`

Creates a new transaction builder.

```rust
let tx = sdk.transaction()
    .from_evm_address("0x...")
    .to_evm_address("0x...")
    .amount(1000)
    .build()?;
```

##### `execute(&self, transaction: Transaction) -> Result<TransactionResult>`

Executes a transaction.

```rust
let result = sdk.execute(tx).await?;
```

---

## ApexSDKBuilder

Builder for constructing an `ApexSDK` instance with custom configuration.

```rust
pub struct ApexSDKBuilder {
    substrate_endpoint: Option<String>,
    evm_endpoint: Option<String>,
    timeout_seconds: Option<u64>,
}
```

#### Methods

##### `new() -> Self`

Creates a new builder instance.

```rust
let builder = ApexSDKBuilder::new();
```

##### `with_substrate_endpoint(self, url: impl Into<String>) -> Self`

Sets the Substrate WebSocket endpoint.

```rust
let builder = builder.with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws");
```

##### `with_evm_endpoint(self, url: impl Into<String>) -> Self`

Sets the EVM HTTP/WebSocket endpoint.

```rust
let builder = builder.with_evm_endpoint("https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY");
```

##### `with_timeout(self, seconds: u64) -> Self`

Sets the connection timeout in seconds.

```rust
let builder = builder.with_timeout(30);
```

##### `build(self) -> Result<ApexSDK>`

Builds the SDK instance. Returns an error if no adapters are configured.

```rust
let sdk = builder.build().await?;
```

---

## Transaction Builder

Builder for creating blockchain transactions.

```rust
pub struct TransactionBuilder {
    from: Option<Address>,
    to: Option<Address>,
    amount: Option<u128>,
    source_chain: Option<Chain>,
    data: Option<Vec<u8>>,
    gas_limit: Option<u64>,
}
```

#### Methods

##### `new() -> Self`

Creates a new transaction builder.

```rust
let builder = TransactionBuilder::new();
```

##### `from(self, address: Address) -> Self`

Sets the sender address.

```rust
let builder = builder.from(Address::evm("0x..."));
```

##### `from_substrate_account(self, address: impl Into<String>) -> Self`

Sets the sender as a Substrate address.

```rust
let builder = builder.from_substrate_account("5GrwvaEF...");
```

##### `from_evm_address(self, address: impl Into<String>) -> Self`

Sets the sender as an EVM address.

```rust
let builder = builder.from_evm_address("0x742d35Cc...");
```

##### `to(self, address: Address) -> Self`

Sets the recipient address.

```rust
let builder = builder.to(Address::evm("0x..."));
```

##### `to_substrate_account(self, address: impl Into<String>) -> Self`

Sets the recipient as a Substrate address.

```rust
let builder = builder.to_substrate_account("5GrwvaEF...");
```

##### `to_evm_address(self, address: impl Into<String>) -> Self`

Sets the recipient as an EVM address.

```rust
let builder = builder.to_evm_address("0x742d35Cc...");
```

##### `amount(self, amount: u128) -> Self`

Sets the transfer amount.

```rust
let builder = builder.amount(1_000_000_000_000_000_000u128); // 1 ETH
```

##### `on_chain(self, chain: Chain) -> Self`

Explicitly sets the source chain.

```rust
let builder = builder.on_chain(Chain::Ethereum);
```

##### `with_data(self, data: Vec<u8>) -> Self`

Sets transaction data/payload.

```rust
let builder = builder.with_data(b"Hello, World!".to_vec());
```

##### `with_gas_limit(self, limit: u64) -> Self`

Sets the gas limit (for EVM transactions).

```rust
let builder = builder.with_gas_limit(21000);
```

##### `build(self) -> Result<Transaction>`

Builds the transaction. Returns an error if required fields are missing.

```rust
let tx = builder.build()?;
```

---

## Transaction

Represents a blockchain transaction.

```rust
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub amount: u128,
    pub source_chain: Chain,
    pub destination_chain: Chain,
    pub data: Option<Vec<u8>>,
    pub gas_limit: Option<u64>,
}
```

#### Methods

##### `is_cross_chain(&self) -> bool`

Checks if this is a cross-chain transaction.

```rust
if tx.is_cross_chain() {
    println!("This is a cross-chain transaction");
}
```

##### `hash(&self) -> String`

Returns a hash of the transaction.

```rust
let hash = tx.hash();
```

---

## TransactionResult

Result of a transaction execution.

```rust
pub struct TransactionResult {
    pub source_tx_hash: String,
    pub destination_tx_hash: Option<String>,
    pub status: TransactionStatus,
    pub block_number: Option<u64>,
    pub gas_used: Option<u64>,
}
```

---

## Addresses

### `Address`

Represents a blockchain address.

```rust
pub enum Address {
    Substrate(String),
    Evm(String),
}
```

#### Methods

##### `substrate(addr: impl Into<String>) -> Self`

Creates a Substrate address.

```rust
let addr = Address::substrate("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
```

##### `evm(addr: impl Into<String>) -> Self`

Creates an EVM address.

```rust
let addr = Address::evm("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7");
```

##### `as_str(&self) -> &str`

Returns the address as a string.

```rust
let addr_str = address.as_str();
```

---

## Chains

### `Chain`

Supported blockchain networks.

```rust
pub enum Chain {
    Polkadot,
    Kusama,
    Ethereum,
    BinanceSmartChain,
    Polygon,
    Avalanche,
    Moonbeam,
    Astar,
}
```

#### Methods

##### `chain_type(&self) -> ChainType`

Returns the chain type.

```rust
let chain_type = Chain::Ethereum.chain_type();
// Returns ChainType::Evm
```

##### `name(&self) -> &str`

Returns the chain name.

```rust
let name = Chain::Polkadot.name();
// Returns "Polkadot"
```

### `ChainType`

Type of blockchain.

```rust
pub enum ChainType {
    Substrate,
    Evm,
    Hybrid,
}
```

---

## Transaction Status

### `TransactionStatus`

Status of a blockchain transaction.

```rust
pub enum TransactionStatus {
    Pending,
    Confirmed {
        block_number: u64,
        confirmations: u32,
    },
    Failed {
        error: String,
    },
    Unknown,
}
```

---

## Error Types

### `Error`

Main error type for Apex SDK.

```rust
pub enum Error {
    Config(String),
    Connection(String),
    Transaction(String),
    UnsupportedChain(String),
    InvalidAddress(String),
    Substrate(apex_sdk_substrate::Error),
    Evm(apex_sdk_evm::Error),
    Serialization(String),
    Other(String),
}
```

#### Error Variants

- **`Config`**: Configuration errors (e.g., no adapters configured)
- **`Connection`**: Network connection errors
- **`Transaction`**: Transaction building or execution errors
- **`UnsupportedChain`**: Requested chain is not supported
- **`InvalidAddress`**: Invalid address format
- **`Substrate`**: Substrate adapter error
- **`Evm`**: EVM adapter error
- **`Serialization`**: Serialization/deserialization error
- **`Other`**: Generic error

### `Result<T>`

Type alias for `std::result::Result<T, Error>`.

```rust
pub type Result<T> = std::result::Result<T, Error>;
```

---

## Adapters

### Substrate Adapter

Low-level adapter for Substrate chains.

```rust
pub struct SubstrateAdapter {
    endpoint: String,
    connected: bool,
}
```

#### Methods

##### `connect(endpoint: &str) -> Result<Self>`

Connects to a Substrate node.

```rust
let adapter = SubstrateAdapter::connect("wss://polkadot.api.onfinality.io/public-ws").await?;
```

##### `get_transaction_status(&self, tx_hash: &str) -> Result<TransactionStatus>`

Gets transaction status.

```rust
let status = adapter.get_transaction_status("0x123...").await?;
```

##### `validate_address(&self, address: &Address) -> bool`

Validates a Substrate address.

```rust
let valid = adapter.validate_address(&address);
```

##### `pallet(&self, name: &str) -> Result<PalletInfo>`

Gets information about a pallet.

```rust
let balances = adapter.pallet("Balances")?;
```

### EVM Adapter

Low-level adapter for EVM chains.

```rust
pub struct EvmAdapter {
    endpoint: String,
    connected: bool,
}
```

#### Methods

##### `connect(endpoint: &str) -> Result<Self>`

Connects to an EVM node.

```rust
let adapter = EvmAdapter::connect("https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY").await?;
```

##### `get_transaction_status(&self, tx_hash: &str) -> Result<TransactionStatus>`

Gets transaction status.

```rust
let status = adapter.get_transaction_status("0x123...").await?;
```

##### `validate_address(&self, address: &Address) -> bool`

Validates an EVM address.

```rust
let valid = adapter.validate_address(&address);
```

##### `contract(&self, address: &str) -> Result<ContractInfo>`

Gets information about a contract.

```rust
let contract = adapter.contract("0x...")?;
```

---

## Usage Examples

### Example 1: Initialize SDK

```rust
use apex_sdk::prelude::*;

let sdk = ApexSDK::builder()
    .with_substrate_endpoint("wss://polkadot.api.onfinality.io/public-ws")
    .with_evm_endpoint("https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY")
    .build()
    .await?;
```

### Example 2: Build and Execute Transaction

```rust
let tx = sdk
    .transaction()
    .from_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
    .to_evm_address("0x1234567890123456789012345678901234567890")
    .amount(1_000_000_000_000_000_000u128)
    .with_gas_limit(21000)
    .build()?;

let result = sdk.execute(tx).await?;
println!("TX Hash: {}", result.source_tx_hash);
```

### Example 3: Query Transaction Status

```rust
let status = sdk
    .get_transaction_status(&Chain::Ethereum, "0x123...")
    .await?;

match status {
    TransactionStatus::Confirmed { block_number, confirmations } => {
        println!("Confirmed at block {} ({} confirmations)",
                 block_number, confirmations);
    }
    _ => println!("Not confirmed yet"),
}
```

### Example 4: Cross-Chain Transfer

```rust
let tx = sdk
    .transaction()
    .from_substrate_account("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY")
    .to_evm_address("0x742d35Cc6634C0532925a3b844Bc9e7595f0bEb7")
    .amount(5_000_000_000_000)
    .build()?;

let result = sdk.execute(tx).await?;
println!("Source TX: {}", result.source_tx_hash);
if let Some(dest_tx) = result.destination_tx_hash {
    println!("Destination TX: {}", dest_tx);
}
```

---

## Feature Flags

Apex SDK provides optional feature flags to enable specific functionality:

### Core Features

#### `apex-sdk-core`

**`mocks`**
- Enables mock implementations for testing
- Includes tokio runtime for mock testing
- Useful for unit testing and integration testing

```toml
[dependencies]
apex-sdk-core = { version = "0.1.4", features = ["mocks"] }
```

### Substrate Features

#### `apex-sdk-substrate`

**`typed`**
- Base feature for typed metadata support
- Enables compile-time type safety for runtime interactions
- Required by all chain-specific typed features

**`typed-polkadot`**
- Typed metadata for Polkadot relay chain
- Includes `typed` feature automatically
- Provides compile-time verified Polkadot runtime types

**`typed-kusama`**
- Typed metadata for Kusama canary network
- Includes `typed` feature automatically
- Provides compile-time verified Kusama runtime types

**`typed-westend`**
- Typed metadata for Westend testnet
- Includes `typed` feature automatically
- Provides compile-time verified Westend runtime types

```toml
[dependencies]
# Enable typed metadata for Polkadot
apex-sdk-substrate = { version = "0.1.4", features = ["typed-polkadot"] }

# Enable multiple chains
apex-sdk-substrate = { version = "0.1.4", features = ["typed-polkadot", "typed-kusama"] }
```

### Default Configuration

By default, all crates use minimal dependencies. Enable features as needed for your use case:

```toml
[dependencies]
# Minimal configuration (default)
apex-sdk = "0.1.4"

# With testing support
apex-sdk-core = { version = "0.1.4", features = ["mocks"] }

# With typed Substrate metadata
apex-sdk-substrate = { version = "0.1.4", features = ["typed-polkadot"] }
```

### Usage Examples

#### Testing with Mocks

```rust
#[cfg(feature = "mocks")]
use apex_sdk_core::mocks::MockAdapter;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_mock() {
        let mock_adapter = MockAdapter::new();
        // Test your code with mock adapter
    }
}
```

#### Typed Metadata for Polkadot

```rust
#[cfg(feature = "typed-polkadot")]
use apex_sdk_substrate::polkadot;

async fn polkadot_example() -> Result<()> {
    let adapter = SubstrateAdapter::connect("wss://polkadot.api.onfinality.io/public-ws").await?;

    // Use typed metadata for compile-time safety
    #[cfg(feature = "typed-polkadot")]
    {
        let balance = adapter.typed_balance::<polkadot::Runtime>(&address).await?;
    }

    Ok(())
}
```


## Version Compatibility

- **Rust**: 1.85 or higher (MSRV)
- **Edition**: 2021
- **Dependencies**: See `Cargo.toml` for specific versions


## Further Reading

- [Quick Start Guide](QUICK_START.md)
- [Examples](examples/)
- [Contributing Guide](CONTRIBUTING.md)
- [Security Policy](SECURITY.md)
