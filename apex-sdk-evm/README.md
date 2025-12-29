# apex-sdk-evm

[![Crates.io](https://img.shields.io/crates/v/apex-sdk-evm)](https://crates.io/crates/apex-sdk-evm)
[![Documentation](https://docs.rs/apex-sdk-evm/badge.svg)](https://docs.rs/apex-sdk-evm)
[![Downloads](https://img.shields.io/crates/d/apex-sdk-evm)](https://crates.io/crates/apex-sdk-evm)
[![License](https://img.shields.io/crates/l/apex-sdk-evm)](LICENSE)
[![EVM](https://img.shields.io/badge/EVM-compatible-627EEA)](https://ethereum.org/)
[![Alloy](https://img.shields.io/badge/Alloy-powered-blue)](https://alloy.rs/)
[![Rust](https://img.shields.io/badge/rust-1.85%2B-orange.svg)](https://www.rust-lang.org/)

EVM blockchain adapter for the Apex SDK, providing seamless interaction with Ethereum and EVM-compatible chains.

> **Note**: This package has been fully migrated from ethers-rs to [Alloy](https://alloy.rs/). Examples in this README are being updated. For the latest API usage, see the [source code](src/) and [tests](tests/).

## Overview

`apex-sdk-evm` enables developers to interact with Ethereum and other EVM-compatible blockchains through a unified, type-safe Rust API. It supports HTTP and WebSocket connections, transaction building, smart contract interaction, and wallet management.

## Features

- **Multi-Chain Support**: Ethereum, Polygon, BSC, Arbitrum, Optimism, and other EVM chains
- **Modern Alloy Integration**: Built on [Alloy](https://alloy.rs/), the modern Ethereum library
- **Connection Management**: HTTP and WebSocket provider support with connection pooling
- **Wallet Integration**: BIP-39 mnemonic support, private key management, transaction signing
- **Transaction Building**: EIP-1559 and legacy transactions with automatic gas estimation
- **Message Signing**: EIP-191 (personal sign) and EIP-712 (typed data) support
- **Type Safety**: Compile-time guarantees with Alloy's strong typing
- **Caching Layer**: Intelligent caching for improved performance
- **Metrics**: Built-in monitoring and observability

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
apex-sdk-evm = "0.1.3"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

### Basic Connection

```rust
use apex_sdk_evm::EvmAdapter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Ethereum mainnet
    let adapter = EvmAdapter::connect("https://eth.llamarpc.com").await?;

    // Get latest block number
    let block_number = adapter.get_block_number().await?;
    println!("Latest block: {}", block_number);

    // Get chain ID
    let chain_id = adapter.get_chain_id().await?;
    println!("Chain ID: {}", chain_id);

    Ok(())
}
```

### Using WebSocket

```rust
use apex_sdk_evm::EvmAdapter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect via WebSocket
    let adapter = EvmAdapter::connect("wss://eth.llamarpc.com").await?;

    // Query blockchain data
    let block_number = adapter.get_block_number().await?;
    println!("Latest block: {}", block_number);

    Ok(())
}
```

## Wallet Management

### Creating and Managing Wallets

```rust
use apex_sdk_evm::wallet::Wallet;

// Generate a new wallet
let wallet = Wallet::generate();
println!("Address: {:?}", wallet.eth_address());
println!("Private key: {}", wallet.private_key_hex());

// Import from private key (hex string with or without 0x prefix)
let wallet = Wallet::from_private_key("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")?;

// Import from mnemonic
let wallet = Wallet::from_mnemonic("test test test test test test test test test test test junk")?;

// Set chain ID for transaction signing
let wallet = wallet.with_chain_id(1); // Ethereum mainnet
```

### Executing Transactions with Apex SDK

The recommended way to execute EVM transactions is through the unified Apex SDK interface:

```rust
use apex_sdk::prelude::*;
use apex_sdk_evm::wallet::Wallet;

#[tokio::main]
async fn main() -> Result<()> {
    // Create wallet from private key
    let wallet = Wallet::from_private_key("0x...")?
        .with_chain_id(11155111); // Sepolia testnet

    // Initialize SDK with wallet
    let sdk = ApexSDK::builder()
        .with_evm_endpoint("https://eth-sepolia.g.alchemy.com/v2/demo")
        .with_evm_wallet(wallet)
        .build()
        .await?;

    // Build and execute transaction
    let recipient = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";
    let amount = 10_000_000_000_000u128; // 0.00001 ETH in wei

    let tx = sdk
        .transaction()
        .from_evm_address(&wallet.address())
        .to_evm_address(recipient)
        .amount(amount)
        .build()?;

    let result = sdk.execute(tx).await?;

    println!("Transaction hash: {}", result.source_tx_hash);
    println!("Status: {:?}", result.status);

    Ok(())
}
```

See [`examples/evm-transfer/`](../examples/evm-transfer/) for a complete working example.

## Smart Contract Interaction

### Type-Safe Contract Calls with Alloy's `sol!` Macro

Apex SDK uses Alloy's `sol!` macro for compile-time safe contract interactions:

```rust
use alloy::sol;
use alloy_primitives::{Address as EthAddress, U256};
use apex_sdk::prelude::*;

// Define ERC20 interface using Alloy's sol! macro
sol! {
    #[sol(rpc)]
    interface IERC20 {
        function balanceOf(address account) external view returns (uint256);
        function totalSupply() external view returns (uint256);
        function transfer(address to, uint256 amount) external returns (bool);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize SDK
    let sdk = ApexSDK::builder()
        .with_evm_endpoint("https://eth-sepolia.g.alchemy.com/v2/demo")
        .build()
        .await?;

    let evm = sdk.evm()?;
    let provider = evm.provider();

    // Contract and account addresses
    let weth: EthAddress = "0xfFf9976782d46CC05630D1f6eBAb18b2324d6B14".parse()?;
    let account: EthAddress = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".parse()?;

    // Query balance (read-only, no gas)
    let balance_call = IERC20::balanceOfCall { account };
    let balance_data = balance_call.abi_encode();

    let result = provider
        .inner
        .call(&alloy::rpc::types::TransactionRequest::default()
            .to(weth)
            .input(balance_data.into()))
        .await?;

    let balance = IERC20::balanceOfCall::abi_decode_returns(&result, true)?._0;
    println!("Balance: {} WETH", balance);

    Ok(())
}
```

For a complete example including write transactions, see [`examples/evm-contract-call/`](../examples/evm-contract-call/).

### Key Advantages of `sol!` Macro

- **Compile-time safety**: Wrong types won't compile
- **Automatic ABI encoding/decoding**: No manual serialization
- **Type inference**: Rust knows the exact types of all parameters
- **No runtime errors**: All type checking happens at compile time

## Advanced Features

### Query Blockchain Data

```rust
use apex_sdk_evm::EvmAdapter;
use apex_sdk_core::ChainAdapter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let adapter = EvmAdapter::connect("https://eth.llamarpc.com").await?;

    // Get account balance
    let address = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045";
    let balance = adapter.get_balance(address).await?;
    println!("Balance: {} wei", balance);

    // Get balance in ETH format
    let balance_eth = adapter.get_balance_eth(address).await?;
    println!("Balance: {} ETH", balance_eth);

    // Get transaction status
    let tx_hash = "0x5c504ed432cb51138bcf09aa5e8a410dd4a1e204ef84bfed1be16dfba1b22060";
    let status = adapter.get_transaction_status(tx_hash).await?;
    println!("Transaction status: {:?}", status);

    // Get latest block number
    let block_num = adapter.get_block_number().await?;
    println!("Latest block: {}", block_num);

    // Get chain ID
    let chain_id = adapter.get_chain_id().await?;
    println!("Chain ID: {}", chain_id);

    Ok(())
}
```

### Message Signing

```rust
use apex_sdk_evm::wallet::Wallet;

let wallet = Wallet::from_private_key("0x...")?;

// Sign a message (EIP-191 personal sign)
let message = "Hello, Ethereum!";
let signature = wallet.sign_message(message).await?;
println!("Signature: {:?}", signature);

// Verify signature
let recovered_address = signature.recover_address_from_msg(message)?;
assert_eq!(recovered_address, wallet.eth_address());
```

## Supported Chains

The adapter works with any EVM-compatible blockchain:

```rust
// Ethereum Mainnet
let eth = EvmAdapter::new("https://eth.llamarpc.com");

// Polygon
let polygon = EvmAdapter::new("https://polygon-rpc.com");

// BSC
let bsc = EvmAdapter::new("https://bsc.publicnode.com");

// Arbitrum
let arbitrum = EvmAdapter::new("https://arb1.arbitrum.io/rpc");

// Optimism
let optimism = EvmAdapter::new("https://mainnet.optimism.io");

// Local development (Ganache, Hardhat)
let local = EvmAdapter::new("http://localhost:8545");
```

## Error Handling

Comprehensive error types for robust applications:

```rust
use apex_sdk_evm::{EvmError, Result};

match some_evm_operation().await {
    Err(EvmError::InsufficientFunds) => {
        println!("Not enough ETH for transaction");
    }
    Err(EvmError::GasTooLow) => {
        println!("Gas limit too low");
    }
    Err(EvmError::ContractError(reason)) => {
        println!("Contract reverted: {}", reason);
    }
    Err(EvmError::NetworkError(msg)) => {
        println!("Network error: {}", msg);
    }
    Ok(result) => {
        // Handle success
    }
}
```

## Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests

```bash
cargo test --features integration-tests
```

The integration tests require a running Ethereum node or testnet access.

## Examples

Complete working examples are available in the [examples](../examples) directory:

- **[EVM Transfer](../examples/evm-transfer)** - Execute real ETH transfers on Sepolia testnet with wallet signing
- **[Contract Interaction](../examples/evm-contract-call)** - Type-safe ERC-20 contract calls using Alloy's `sol!` macro
- [Account Manager](../examples/account-manager) - Multi-chain account management
- [Contract Orchestration](../examples/contract-orchestration) - Smart contract deployment across chains
- [Price Oracle](../examples/price-oracle) - Multi-chain price aggregation

All examples use the modern Alloy library and demonstrate real blockchain interactions.

## Configuration

### Environment Variables

```bash
# Provider URLs
ETHEREUM_RPC_URL="https://eth.llamarpc.com"
POLYGON_RPC_URL="https://polygon-rpc.com"

# Private keys (use with caution)
PRIVATE_KEY="0x..." # For testing only

# API keys
INFURA_API_KEY="your-infura-key"
ALCHEMY_API_KEY="your-alchemy-key"
```

### Configuration File

```toml
# config.toml
[evm]
default_network = "ethereum"
request_timeout = "30s"

[evm.networks.ethereum]
rpc_url = "https://eth.llamarpc.com"
chain_id = 1

[evm.networks.polygon]
rpc_url = "https://polygon-rpc.com" 
chain_id = 137

[evm.cache]
enabled = true
max_entries = 10000
ttl = "5m"

[evm.metrics]
enabled = true
prometheus_endpoint = "0.0.0.0:9090"
```

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../LICENSE) for details.

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

## Support

- [Documentation](https://docs.rs/apex-sdk-evm)
- [GitHub Issues](https://github.com/carbobit/apex-sdk/issues)
- [Examples](../examples)